use applicative::Applicative;
use functor::Functor;
use futures::future::{Future, LocalFutureObj};
use futures::FutureExt;
use lifted::*;

pub struct FutureKind;

impl HKT for FutureKind {}
impl<'a, A, B, G, F> Lift<'a, FutureKind, A, B, G> for F
where
    F: Future<Output = A> + 'a,
{
    fn lift(self) -> Lifted<'a, FutureKind, A, B, G> {
        Lifted::Future(LocalFutureObj::new(Box::new(self)))
    }
}

impl<'a, A, B, G> Unlift<FutureKind> for Lifted<'a, FutureKind, A, B, G> {
    type Out = LocalFutureObj<'a, A>;

    fn unlift(self) -> <Self as Unlift<FutureKind>>::Out {
        match self {
            Lifted::Future(f) => f,
            _ => unreachable!(),
        }
    }
}

impl<'a, Z, G> Functor<'a, FutureKind, Z, G> for FutureKind {
    fn map<Func, A, B>(
        fa: Lifted<'a, FutureKind, A, Z, G>,
        func: Func,
    ) -> Lifted<'a, FutureKind, B, Z, G>
    where
        Func: FnOnce(A) -> B + 'a,
    {
        Lifted::Future(LocalFutureObj::new(Box::new(fa.unlift().map(func))))
    }
}

impl<'a, Z, G> Applicative<'a, FutureKind, Z, G> for FutureKind {
    fn ap<A, B, Func>(
        ff: Lifted<'a, FutureKind, Func, Z, G>,
        fa: Lifted<'a, FutureKind, A, Z, G>,
    ) -> Lifted<'a, FutureKind, B, Z, G>
    where
        Func: FnOnce(A) -> B + 'a,
    {
        let ff = ff.unlift();
        let fa = fa.unlift();
        ff.map(|ff| fa.map(|a| ff(a))).flatten().lift()
    }

    fn point<A>(a: A) -> Lifted<'a, FutureKind, A, Z, G> {
        use futures::future;
        future::lazy(|_| a).lift()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use data::kleisli;
    use data::kleisli::KleisliExt;
    use futures::executor::block_on;
    use futures::future;
    use test::*;

    #[test]
    fn test_kleisli_map() {
        let k = kleisli::lift::<FutureKind, i32, i32, Nothing, Nothing, _, _>(|i: i32| {
            future::lazy(move |_| i)
        });
        let k = k.map(|i| i * 2);
        assert_eq!(block_on(k.runlift(15)), 30);
    }

    #[test]
    fn test_applicative() {
        let fut: Lifted<FutureKind, &str, Nothing, Nothing> = future::lazy(|_| "yospos").lift();
        let fut2: Lifted<FutureKind, &str, Nothing, Nothing> = future::lazy(|_| "bictch").lift();

        assert_eq!(
            block_on(FutureKind::product(fut, fut2).unlift()),
            ("yospos", "bictch")
        )
    }

    #[test]
    fn test_lift() {
        // TODO: Fix type annotation
        let f: Lifted<FutureKind, i32, Nothing, Nothing> = future::lazy(|_| 1).lift();
        let f = <FutureKind as Functor<_>>::map(f, |i| i * 2).unlift();
        assert_eq!(block_on(f), 2)
    }

    #[bench]
    fn bench_functor_map(b: &mut Bencher) {
        b.iter(|| {
            for _ in 0..10000 {
                let f: Lifted<_, _, Nothing, Nothing> = future::lazy(|_| "foo").lift();
                black_box(block_on(
                    <FutureKind as Functor<_>>::map(f, |s| "foo".to_string() + s).unlift(),
                ));
            }
        })
    }

    #[bench]
    fn bench_native_map(b: &mut Bencher) {
        b.iter(|| {
            for _ in 0..10000 {
                let f = future::lazy(|_| "foo");
                black_box(block_on(f.map(|i| "foo".to_string() + i)));
            }
        })
    }
}
