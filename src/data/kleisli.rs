use functor::Functor;
use lifted::Lifted;
use monad::Monad;
use std::boxed::FnBox;
use std::marker::PhantomData;
use std::rc::Rc;

pub trait KleisliT<'a, F: 'static, A, B: 'a> {
    fn run(&self, a: A) -> Lifted<'a, F, B>;
}

pub trait KleisliExt<'a, F, A, B>
where
    F: 'static,
    B: 'a,
    Self: Sized + KleisliT<'a, F, A, B>,
{
    fn compose<Z, K>(self, k: K) -> Compose<'a, F, A, B, Z, Self, K>
    where
        F: Monad<'a, F>,
        K: KleisliT<'a, F, Z, A>;

    fn map<Z, Func>(self, func: Func) -> Map<Z, Self, Func>
    where
        Func: Fn(B) -> Z,
    {
        Map {
            func: RcFn(Rc::new(func)),
            k: self,
            marker: PhantomData,
        }
    }
}

impl<'a, F: 'static, A, B: 'a, K> KleisliExt<'a, F, A, B> for K
where
    K: KleisliT<'a, F, A, B>,
{
    fn compose<Z, K2>(self, k: K2) -> Compose<'a, F, A, B, Z, K, K2>
    where
        F: Monad<'a, F>,
        Self: Sized + KleisliT<'a, F, A, B>,
        K2: KleisliT<'a, F, Z, A>,
    {
        Compose {
            k1: self,
            k2: k,
            marker: PhantomData,
        }
    }
}

pub struct Kleisli<'a, F: 'static, A: 'a, B: 'a>(Box<Fn(A) -> Lifted<'a, F, B> + 'a>);

impl<'a, F: 'static, A, B: 'a> KleisliT<'a, F, A, B> for Kleisli<'a, F, A, B> {
    fn run(&self, a: A) -> Lifted<'a, F, B> {
        (self.0)(a)
    }
}

/// Compose
pub struct Compose<'a, F: 'static, A: 'a, B: 'a, Z: 'a, K1, K2> {
    k1: K1,
    k2: K2,
    marker: PhantomData<&'a (F, A, B, Z)>,
}

impl<'a, F, A, B, Z, K1, K2> KleisliT<'a, F, Z, B> for Compose<'a, F, A, B, Z, K1, K2>
where
    F: Monad<'a, F>,
    K1: KleisliT<'a, F, A, B>,
    K2: KleisliT<'a, F, Z, A>,
{
    fn run(&self, a: Z) -> Lifted<'a, F, B> {
        F::flat_map(self.k2.run(a), |a| self.k1.run(a))
    }
}

/// Map
pub struct Map<B, K1, Func> {
    k: K1,
    func: RcFn<Func>,
    marker: PhantomData<*const B>,
}

impl<'a, F, A, B, Z, K1, Func> KleisliT<'a, F, A, Z> for Map<B, K1, Func>
where
    B: 'a,
    Z: 'a,
    F: Functor<'a, F> + 'static,
    K1: KleisliT<'a, F, A, B> + 'a,
    RcFn<Func>: Fn(B) -> Z + 'a,
{
    fn run(&self, a: A) -> Lifted<'a, F, Z> {
        F::map(self.k.run(a), self.func.clone())
    }
}

fn apply<'a, F: 'static, A, B: 'a>(
    run: impl Fn(A) -> Lifted<'a, F, B> + 'a,
) -> impl KleisliT<'a, F, A, B> {
    Kleisli(Box::new(run))
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
#[cfg(test)]
mod tests {
    use super::*;
    use data::kleisli;
    use instances::option::OptionKind;
    use lifted::*;

    #[test]
    fn test_compose() {
        let parse = kleisli::apply(|s: &str| {
            match s.parse::<i32>() {
                Ok(i) => Some(i),
                Err(_) => None,
            }.lift()
        });

        let reciprocal =
            kleisli::apply(|i: i32| if i != 0 { Some(1.0 / i as f32) } else { None }.lift());

        let parse_and_recriprocal = reciprocal.compose(parse);

        assert_eq!(parse_and_recriprocal.run("123").unlift(), Some(0.008130081));

        assert_eq!(parse_and_recriprocal.run("yospos").unlift(), None);

        let doubled = parse_and_recriprocal.map(|f| f * 2 as f32);
        assert_eq!(doubled.run("123").unlift(), Some(0.016260162))
    }
}
