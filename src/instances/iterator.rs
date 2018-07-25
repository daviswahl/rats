use functor::Functor;
use lifted::{Lift, Lifted, Nothing, Unlift, HKT};

pub struct IteratorKind;

impl HKT for IteratorKind {}

impl<'a, A: 'a, I> Lift<'a, IteratorKind, A> for I
where
    I: Iterator<Item = A> + 'a,
{
    fn lift(self) -> Lifted<'a, IteratorKind, A> {
        Lifted::Iterator(Box::new(self))
    }
}

impl<'a, A: 'a> Unlift<IteratorKind> for Lifted<'a, IteratorKind, A> {
    type Out = Box<Iterator<Item = A> + 'a>;

    fn unlift(self) -> <Self as Unlift<IteratorKind>>::Out {
        match self {
            Lifted::Iterator(i) => i,
            _ => unreachable!(),
        }
    }
}

impl<'a> Functor<'a, IteratorKind> for IteratorKind {
    fn map<Func, A, B>(fa: Lifted<'a, IteratorKind, A>, func: Func) -> Lifted<'a, IteratorKind, B>
    where
        Func: Fn(A) -> B + 'a,
    {
        fa.unlift().map(move |a| func(a)).lift()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::*;

    #[test]
    fn test_iterator() {
        let v = vec![1, 2, 3, 4].into_iter().lift();
        IteratorKind::map(v, |i| i * 2).unlift();
    }

    #[bench]
    fn bench_functor_map_string(b: &mut Bencher) {
        b.iter(|| {
            let v = (0..1000).collect::<Vec<i32>>().into_iter().lift();
            black_box(
                IteratorKind::map(IteratorKind::map(v, |i| i * 2), |i| format!("{:?}", i))
                    .unlift()
                    .collect::<Vec<String>>(),
            );
        })
    }

    #[bench]
    fn bench_native_map_string(b: &mut Bencher) {
        b.iter(|| {
            let v = (0..1000).collect::<Vec<i32>>().into_iter();
            black_box(
                v.map(|i| i * 2)
                    .map(|i| format!("{:?}", i))
                    .collect::<Vec<String>>(),
            );
        })
    }

    #[bench]
    fn bench_functor_map_int(b: &mut Bencher) {
        b.iter(|| {
            let v = (0..1000).collect::<Vec<i32>>().into_iter().lift();
            black_box(
                IteratorKind::map(IteratorKind::map(v, |i| i * 2), |i| i * 10)
                    .unlift()
                    .collect::<Vec<i32>>(),
            );
        })
    }

    #[bench]
    fn bench_native_map_int(b: &mut Bencher) {
        b.iter(|| {
            let v = (0..1000).collect::<Vec<i32>>().into_iter();
            black_box(v.map(|i| i * 2).map(|i| i * 10).collect::<Vec<i32>>());
        })
    }
}
