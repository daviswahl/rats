use functor::Functor;
use futures::Future;
use futures::FutureExt;
use lifted::*;

pub struct FutureKind;

impl HKT for FutureKind {}
impl<'f> HKT for &'f FutureKind {}

impl<'f, A, B, F> Lift<'f, FutureKind, A, B> for F
where
    F: Future<Item = A, Error = B> + 'f,
{
    fn lift(self) -> Lifted<'f, FutureKind, A, B> {
        Lifted::Future(Box::new(self))
    }
}

impl<'f, A, B> Unlift<FutureKind> for Lifted<'f, FutureKind, A, B> {
    type Out = Box<Future<Item = A, Error = B> + 'f>;
    fn unlift(self) -> <Self as Unlift<FutureKind>>::Out {
        match self {
            Lifted::Future(f) => f,
            _ => unreachable!(),
        }
    }
}

impl<'f, Z> Functor<'f, FutureKind, Z> for FutureKind {
    fn map<Func, A, B>(fa: Lifted<'f, FutureKind, A, Z>, func: Func) -> Lifted<'f, FutureKind, B, Z>
    where
        Func: FnOnce(A) -> B + 'f,
    {
        fa.unlift().map(|f| func(f)).lift()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;
    use futures::future;

    #[test]
    fn test_lift() {
        let f = future::ok::<i32, &str>(1).lift();
        let f = FutureKind::map(f, |i| i * 2).unlift();
        assert_eq!(block_on(f).unwrap(), 2)
    }

    use super::*;
    use test::*;

    #[bench]
    fn bench_functor_map(b: &mut Bencher) {
        b.iter(|| {
            for i in 0..10000 {
                let f = future::ok::<&str, &str>("foo").lift();
                black_box(
                    block_on(FutureKind::map(f, |s| "foo".to_string() + "foo").unlift()).unwrap(),
                );
            }
        })
    }

    #[bench]
    fn bench_native_map(b: &mut Bencher) {
        b.iter(|| {
            for i in 0..10000 {
                let f = future::ok::<String, &str>("foo".to_owned());
                black_box(block_on(f.map(|i| i + "foo")).unwrap());
            }
        })
    }
}
