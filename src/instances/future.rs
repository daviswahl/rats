use functor::Functor;
use futures::Future;
use futures::FutureExt;
use lifted::*;

pub struct FutureKind;

impl HKT for FutureKind {}
impl<'a, A, B, F> Lift<'a, FutureKind, A, B> for F
where
    F: Future<Item = A, Error = B> + 'a,
{
    fn lift(self) -> Lifted<'a, FutureKind, A, B> {
        Lifted::Future(Box::new(self))
    }
}

impl<'a, A, B> Unlift<FutureKind> for Lifted<'a, FutureKind, A, B> {
    type Out = Box<Future<Item = A, Error = B> + 'a>;
    fn unlift(self) -> <Self as Unlift<FutureKind>>::Out {
        match self {
            Lifted::Future(f) => f,
            _ => unreachable!(),
        }
    }
}

impl<'a, Z> Functor<'a, FutureKind, Z> for FutureKind {
    fn map<Func, A, B>(fa: Lifted<'a, FutureKind, A, Z>, func: Func) -> Lifted<'a, FutureKind, B, Z>
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
        let f = future::ok::<i32, &str>(1).lift();
        let f = FutureKind::map(f, |i| i * 2).unlift();
        assert_eq!(block_on(f).unwrap(), 2)
    }

    #[bench]
    fn bench_functor_map(b: &mut Bencher) {
        b.iter(|| {
            for _ in 0..10000 {
                let f = future::ok::<&str, &str>("foo").lift();
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
                let f = future::ok::<String, &str>("foo".to_owned());
                black_box(block_on(f.map(|i| i + "foo")).unwrap());
            }
        })
    }
}
