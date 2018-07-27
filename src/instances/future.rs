use functor::Functor;
use futures::future::{Future, FutureObj};
use futures::FutureExt;
use lifted::*;

pub struct FutureKind;

impl HKT for FutureKind {}
impl<'a, A, B, G, F> Lift<'a, FutureKind, A, B, G> for F
where
    F: Future<Output = A>,
    FutureObj<'a, A>: From<Box<F>>,
{
    fn lift(self) -> Lifted<'a, FutureKind, A, B, G> {
        Lifted::Future(Box::new(self).into())
    }
}

impl<'a, A, B, G> Unlift<FutureKind> for Lifted<'a, FutureKind, A, B, G> {
    type Out = FutureObj<'a, A>;
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
        fa.unlift().map(func).lift()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;
    use futures::future;
    use test::*;

    #[test]
    fn test_lift() {
        let f = future::lazy(|_| 1).lift();
        let f = FutureKind::map(f, |i| i * 2).unlift();
        assert_eq!(block_on(f).unwrap(), 2)
    }

    #[bench]
    fn bench_functor_map(b: &mut Bencher) {
        b.iter(|| {
            for _ in 0..10000 {
                let f = future::lazy(|_| ()).lift();
                black_box(
                    block_on(FutureKind::map(f, |s| "foo".to_string() + s).unlift()).unwrap(),
                );
            }
        })
    }

    #[bench]
    fn bench_native_map(b: &mut Bencher) {
        b.iter(|| {
            for _ in 0..10000 {
                let f = future::lazy(|_| "foo").lift();
                black_box(block_on(f.map(|i| i + "foo")).unwrap());
            }
        })
    }
}
