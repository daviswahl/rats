use functor::Functor;
use lifted::Lift;
use lifted::Lifted;
use lifted::Nothing;
use lifted::Unlift;
use monad::Monad;
use std::marker::PhantomData;
use std::rc::Rc;

pub trait Kleisli<'a, F, A, B, Z = Nothing, G = Nothing>
where
    F: 'static,
    B: 'a,
{
    fn run(&self, a: A) -> Lifted<'a, F, B, Z, G>;
}

pub trait KleisliExt<'a, F, A, B, Z = Nothing, G = Nothing>
where
    F: 'static,
    A: 'a,
    B: 'a,
    Self: Sized + Kleisli<'a, F, A, B, Z, G>,
{
    fn runlift(&self, a: A) -> <Lifted<'a, F, B, Z, G> as Unlift<F>>::Out
    where
        Lifted<'a, F, B, Z, G>: Unlift<F>,
    {
        Kleisli::<_, _, _, _, _>::run(self, a).unlift()
    }

    fn compose<Z2, K>(self, k: K) -> Compose<A, Self, K>
    where
        F: Monad<'a, F, Z, G>,
        K: Kleisli<'a, F, Z2, A, Z, G>,
    {
        Compose {
            k1: self,
            k2: k,
            marker: PhantomData,
        }
    }

    fn map<Z2, Func>(self, func: Func) -> Map<Z2, Self, Func>
    where
        Func: Fn(B) -> Z2,
    {
        Map {
            func: RcFn(Rc::new(func)),
            k: self,
            marker: PhantomData,
        }
    }
}

impl<'a, F, A, B, K, Z, G> KleisliExt<'a, F, A, B, Z, G> for K
where
    K: Kleisli<'a, F, A, B, Z, G>,
    F: 'static,
    A: 'a,
    B: 'a,
{}

pub struct Run<'a, F, A, B, Z = Nothing, G = Nothing>(Box<Fn(A) -> Lifted<'a, F, B, Z, G> + 'a>)
where
    F: 'static,
    G: 'static,
    A: 'a,
    B: 'a,
    Z: 'a;

impl<'a, F, A, B, Z, G> Kleisli<'a, F, A, B, Z, G> for Run<'a, F, A, B, Z, G>
where
    F: 'static,
    B: 'a,
    Z: 'a,
    G: 'static,
{
    fn run(&self, a: A) -> Lifted<'a, F, B, Z, G> {
        (self.0)(a)
    }
}

/// Compose
pub struct Compose<A, K1, K2> {
    k1: K1,
    k2: K2,
    marker: PhantomData<*const A>,
}

impl<'a, F, A, B, C, K1, K2, Z, G> Kleisli<'a, F, C, B, Z, G> for Compose<A, K1, K2>
where
    A: 'a,
    B: 'a,
    F: Monad<'a, F, Z, G>,
    K1: Kleisli<'a, F, A, B, Z, G>,
    K2: Kleisli<'a, F, C, A, Z, G>,
{
    fn run(&self, a: C) -> Lifted<'a, F, B, Z, G> {
        F::flat_map(self.k2.run(a), |a| self.k1.run(a))
    }
}

/// Map
/// RC may not be necessary here and could be a byproduct of a misunderstanding of lifetimes throughout
/// the whole library.
pub struct Map<B, K1, Func> {
    k: K1,
    func: RcFn<Func>,
    marker: PhantomData<*const B>,
}

impl<'a, F, A, B, C, K1, Func, Z, G> Kleisli<'a, F, A, C, Z, G> for Map<B, K1, Func>
where
    B: 'a,
    C: 'a,
    F: Functor<'a, F, Z, G> + 'static,
    K1: Kleisli<'a, F, A, B, Z, G> + 'a,
    RcFn<Func>: Fn(B) -> C + 'a,
{
    fn run(&self, a: A) -> Lifted<'a, F, C, Z, G> {
        F::map(self.k.run(a), self.func.clone())
    }
}

pub fn run<'a, F, A, B, Z, G>(
    run: impl Fn(A) -> Lifted<'a, F, B, Z, G> + 'a,
) -> impl Kleisli<'a, F, A, B, Z, G>
where
    F: 'static,
    G: 'static,
    B: 'a,
    Z: 'a,
{
    Run(Box::new(run))
}

pub fn lift<'a, F, A, B, Z, G, L>(run: impl Fn(A) -> L + 'a) -> impl Kleisli<'a, F, A, B, Z, G>
where
    L: Lift<'a, F, B, Z, G> + 'a,
    F: 'static,
    G: 'static,
    B: 'a,
    Z: 'a,
{
    self::run(move |a| run(a).lift())
}

pub struct RcFn<F>(Rc<F>);

impl<F> Clone for RcFn<F> {
    fn clone(&self) -> Self {
        RcFn(self.0.clone())
    }
}

impl<'a, A, B, F> Fn<(A,)> for RcFn<F>
where
    F: Fn(A) -> B + 'a,
{
    extern "rust-call" fn call(&self, args: (A,)) -> Self::Output {
        self.0.call(args)
    }
}

impl<'a, A, B, F> FnOnce<(A,)> for RcFn<F>
where
    F: Fn(A) -> B + 'a,
{
    type Output = B;

    extern "rust-call" fn call_once(self, args: (A,)) -> Self::Output {
        self.0.call(args)
    }
}

impl<'a, A, B, F> FnMut<(A,)> for RcFn<F>
where
    F: Fn(A) -> B + 'a,
{
    extern "rust-call" fn call_mut(&mut self, args: (A,)) -> Self::Output {
        self.0.call(args)
    }
}

///
impl<'a, F, A, B, Z, G> Fn<(A,)> for Kleisli<'a, F, A, B, Z, G>
where
    Lifted<'a, F, B, Z, G>: Unlift<F>,
    Self: Sized + 'a,
    A: 'a,
    G: 'static,
    B: 'a,
    Z: 'a,
    F: 'static,
{
    extern "rust-call" fn call(&self, args: (A,)) -> Self::Output {
        self.run(args.0).unlift()
    }
}

impl<'a, F, A, B, Z, G> FnOnce<(A,)> for Kleisli<'a, F, A, B, Z, G>
where
    Lifted<'a, F, B, Z, G>: Unlift<F>,
    Self: Sized + 'a,
    B: 'a,
    G: 'static,
    Z: 'a,
    F: 'static,
{
    type Output = <Lifted<'a, F, B, Z, G> as Unlift<F>>::Out;

    extern "rust-call" fn call_once(self, args: (A,)) -> Self::Output {
        self.run(args.0).unlift()
    }
}

impl<'a, F, A, B, Z, G> FnMut<(A,)> for Kleisli<'a, F, A, B, Z, G>
where
    Lifted<'a, F, B, Z, G>: Unlift<F>,
    Self: Sized,
    A: 'a,
    G: 'static,
    B: 'a,
    Z: 'a,
    F: 'static,
{
    extern "rust-call" fn call_mut(&mut self, args: (A,)) -> Self::Output {
        self.run(args.0).unlift()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use data::kleisli;

    #[test]
    fn test_compose_and_map() {
        let parse = kleisli::lift(|s: &str| s.parse::<i32>().map_err(|_| "parse error"));

        let reciprocal = kleisli::lift(|i: i32| {
            if i != 0 {
                Ok(1.0 / i as f32)
            } else {
                Err("divide by 0")
            }
        });

        let parse_and_recriprocal = reciprocal.compose(parse);

        assert_eq!(parse_and_recriprocal.runlift("123"), Ok(0.008130081));

        assert_eq!(parse_and_recriprocal.runlift("yospos"), Err("parse error"));

        let doubled = parse_and_recriprocal.map(|f| f * 2 as f32);
        assert_eq!(doubled.runlift("123"), Ok(0.016260162));
        assert_eq!(doubled.runlift("0"), Err("divide by 0"));
    }
}
