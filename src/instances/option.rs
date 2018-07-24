use functor::Functor;
use lifted::Lift;
use lifted::Unlift;
use lifted::{Lifted, Nothing, HKT};

pub struct OptionKind;

impl HKT for OptionKind {}

impl Functor<OptionKind> for OptionKind {
    type B = Nothing;
    type G = Nothing;

    fn map<Func, A, B>(fa: Lifted<OptionKind, A>, func: Func) -> Lifted<OptionKind, B>
    where
        Func: Fn(A) -> B,
    {
        match fa.unlift() {
            Some(a) => Some(func(a)),
            None => None,
        }.lift()
    }
}

impl<A, B, G> Unlift<OptionKind> for Lifted<OptionKind, A, B, G> {
    type Out = Option<A>;

    fn unlift(self) -> <Self as Unlift<OptionKind>>::Out {
        match self {
            Lifted::Option(o) => o,
            _ => unreachable!(),
        }
    }
}

impl<A> Lift<OptionKind, A> for Option<A> {
    fn lift(self) -> Lifted<OptionKind, A> {
        Lifted::Option(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::*;

    #[bench]
    fn bench_functor_map(b: &mut Bencher) {
        b.iter(|| {
            for i in 0..10000 {
                black_box(OptionKind::map(Some(i).lift(), |i| i * 2).unlift());
            }
        })
    }

    #[bench]
    fn bench_native_map(b: &mut Bencher) {
        b.iter(|| {
            for i in 0..10000 {
                black_box(Some(i).map(|i| i * 2));
            }
        })
    }
}
