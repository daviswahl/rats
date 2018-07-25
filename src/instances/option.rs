use functor::Functor;
use lifted::Lift;
use lifted::Unlift;
use lifted::{Lifted, Nothing, HKT};

pub struct OptionKind;

impl HKT for OptionKind {}
impl<'a> HKT for &'a OptionKind {}
impl<'a> HKT for &'a mut OptionKind {}

impl<'a> Functor<'a, OptionKind> for OptionKind {
    fn map<Func, A, B>(fa: Lifted<OptionKind, A>, func: Func) -> Lifted<OptionKind, B>
    where
        Func: Fn(&A) -> B,
    {
        match fa.unlift() {
            Some(a) => Some(func(&a)),
            None => None,
        }.lift()
    }
}

impl<'a, A, B, G> Unlift<OptionKind> for Lifted<'a, OptionKind, A, B, G> {
    type Out = Option<A>;

    fn unlift(self) -> <Self as Unlift<OptionKind>>::Out {
        match self {
            Lifted::Option(o) => o,
            _ => unreachable!(),
        }
    }
}

impl<'a, A, B, G> Unlift<&'a mut OptionKind> for Lifted<'a, &'a mut OptionKind, A, B, G> {
    type Out = Option<&'a mut A>;

    fn unlift(self) -> <Self as Unlift<&'a mut OptionKind>>::Out {
        match self {
            Lifted::OptionMut(o) => o,
            _ => unreachable!(),
        }
    }
}

impl<'a, A> Lift<'a, OptionKind, A> for Option<A> {
    fn lift(self) -> Lifted<'a, OptionKind, A> {
        Lifted::Option(self)
    }
}

impl<'a, A> Lift<'a, &'a mut OptionKind, A> for Option<&'a mut A> {
    fn lift(self) -> Lifted<'a, &'a mut OptionKind, A> {
        Lifted::OptionMut(self)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use test::*;

    #[test]
    fn test_lift_unlift() {
        let mut foo = "foo".to_owned();
        let o = Some(&foo).lift();

        let r = OptionKind::map(o, |i| i.split_at(1));
        r.unlift();
    }

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
