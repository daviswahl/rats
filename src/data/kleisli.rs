use lifted::Lifted;
use monad::Monad;
use std::boxed::FnBox;
use std::marker::PhantomData;

pub trait KleisliT<'a, F: 'static, A, B: 'a> {
    fn run(&self, a: A) -> Lifted<'a, F, B>;
}

trait KleisliExt<'a, F, A, B> {
    fn compose<Z, K>(self, k: K) -> Compose<'a, F, A, B, Z, Self, K>
    where
        F: Monad<'a, F>,
        Self: Sized + KleisliT<'a, F, A, B>,
        K: KleisliT<'a, F, Z, A>;
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

pub struct Compose<'a, F: 'static, A: 'a, B: 'a, Z: 'a, K1, K2>
where
    K1: KleisliT<'a, F, A, B>,
    K2: KleisliT<'a, F, Z, A>,
{
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

fn compose<'a, F: 'static, A: 'a, B: 'a, Z: 'a>(
    k1: impl KleisliT<'a, F, A, B>,
    k2: impl KleisliT<'a, F, Z, A>,
) -> impl KleisliT<'a, F, Z, B>
where
    F: Monad<'a, F>,
{
    Compose {
        k1,
        k2,
        marker: PhantomData,
    }
}

fn apply<'a, F: 'static, A, B: 'a>(
    run: impl Fn(A) -> Lifted<'a, F, B> + 'a,
) -> impl KleisliT<'a, F, A, B> {
    Kleisli(Box::new(run))
}

#[cfg(test)]
mod tests {
    use super::*;
    use data::kleisli;
    use instances::option::OptionKind;
    use lifted::*;

    #[test]
    fn test_compose() {
        let parse = kleisli::apply(|s: String| {
            match s.parse::<i32>() {
                Ok(i) => Some(i),
                Err(_) => None,
            }.lift()
        });

        let reciprocal =
            kleisli::apply(|i: i32| if i != 0 { Some(1.0 / i as f32) } else { None }.lift());

        let parse_and_recriprocal = reciprocal.compose(parse);

        assert_eq!(
            parse_and_recriprocal.run("123".to_owned()).unlift(),
            Some(0.008130081)
        );

        assert_eq!(
            parse_and_recriprocal.run("yospos".to_owned()).unlift(),
            None
        )
    }
}
