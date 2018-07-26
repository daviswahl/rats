use applicative::Applicative;
use functor::Functor;
use lifted::Lift;
use lifted::Lifted;
use lifted::Unlift;
use lifted::HKT;
use monad::Monad;

pub struct ResultKind;

impl HKT for ResultKind {}

/// Lift
impl<'a, A, B> Lift<'a, ResultKind, A, B> for Result<A, B> {
    fn lift(self) -> Lifted<'a, ResultKind, A, B> {
        Lifted::Result(self)
    }
}

/// Unlift
impl<'a, A, B> Unlift<ResultKind> for Lifted<'a, ResultKind, A, B> {
    type Out = Result<A, B>;

    fn unlift(self) -> <Self as Unlift<ResultKind>>::Out {
        match self {
            Lifted::Result(r) => r,
            _ => unreachable!(),
        }
    }
}

/// Functor
impl<'a, Z> Functor<'a, ResultKind, Z> for ResultKind {
    fn map<Func, A, B>(fa: Lifted<'a, ResultKind, A, Z>, func: Func) -> Lifted<'a, ResultKind, B, Z>
    where
        Func: Fn(A) -> B + 'a,
    {
        match fa.unlift() {
            Err(e) => Err(e),
            Ok(a) => Ok(func(a)),
        }.lift()
    }
}

/// Applicative
impl<'a, Z> Applicative<'a, ResultKind, Z> for ResultKind {
    fn ap<A, B, Func>(
        ff: Lifted<'a, ResultKind, Func, Z>,
        fa: Lifted<'a, ResultKind, A, Z>,
    ) -> Lifted<'a, ResultKind, B, Z>
    where
        Func: FnOnce(A) -> B + 'a,
    {
        let ff = ff.unlift();
        let fa = fa.unlift();
        match ff {
            Ok(ff) => match fa {
                Ok(fa) => Ok(ff(fa)),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }.lift()
    }

    fn point<A>(a: A) -> Lifted<'a, ResultKind, A, Z> {
        Ok(a).lift()
    }
}

/// Monad

impl<'a, Z> Monad<'a, ResultKind, Z> for ResultKind {
    fn flat_map<A, B, Func>(
        fa: Lifted<'a, ResultKind, A, Z>,
        func: Func,
    ) -> Lifted<'a, ResultKind, B, Z>
    where
        Func: Fn(A) -> Lifted<'a, ResultKind, B, Z>,
    {
        match fa.unlift() {
            Ok(a) => func(a),
            Err(e) => Err(e).lift(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use monad::Monad;
    use test::*;

    #[test]
    fn test_ap() {
        let ff = Ok::<_, &str>(|i| i * 2).lift();
        let fa = Ok::<_, &str>(1).lift();

        let result = ResultKind::ap(ff, fa).unlift();
        assert_eq!(Ok(2), result)
    }
    #[test]
    fn test_lift_unlift() {
        let mut foo = "foo".to_owned();
        let o: Lifted<_, _, &str> = Ok(&foo).lift();
    }

    #[bench]
    fn bench_functor_map(b: &mut Bencher) {
        b.iter(|| {
            for i in 0..10000 {
                black_box(
                    <ResultKind as Functor<_, &str, _>>::map(Ok(i).lift(), |i| i * 2).unlift(),
                );
            }
        })
    }

    #[bench]
    fn bench_native_map(b: &mut Bencher) {
        b.iter(|| {
            for i in 0..10000 {
                black_box(Ok::<_, &str>(i).map(|i| i * 2));
            }
        })
    }
}
