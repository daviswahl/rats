use applicative::Applicative;
use functor::Functor;
use lifted::Lift;
use lifted::Unlift;
use lifted::{Lifted, HKT};
use monad::Monad;

pub struct OptionKind;

impl HKT for OptionKind {}

impl<'a, A, B, G> Unlift<OptionKind> for Lifted<'a, OptionKind, A, B, G> {
    type Out = Option<A>;

    fn unlift(self) -> <Self as Unlift<OptionKind>>::Out {
        match self {
            Lifted::Option(o) => o,
            _ => unreachable!(),
        }
    }
}

impl<'a, A> Lift<'a, OptionKind, A> for Option<A> {
    fn lift(self) -> Lifted<'a, OptionKind, A> {
        Lifted::Option(self)
    }
}

// Functor
impl<'a> Functor<'a, OptionKind> for OptionKind {
    fn map<Func, A, B>(fa: Lifted<'a, OptionKind, A>, func: Func) -> Lifted<'a, OptionKind, B>
    where
        Func: Fn(A) -> B,
    {
        match fa.unlift() {
            Some(a) => Some(func(a)),
            None => None,
        }.lift()
    }
}

// Applicative
impl<'a> Applicative<'a, OptionKind> for OptionKind {
    fn ap<A, B, Func>(
        ff: Lifted<'a, OptionKind, Func>,
        fa: Lifted<'a, OptionKind, A>,
    ) -> Lifted<'a, OptionKind, B>
    where
        Func: FnOnce(A) -> B + 'a,
    {
        let ff = ff.unlift();
        let fa = fa.unlift();
        ff.and_then(|f| fa.map(|a| f(a))).lift()
    }

    fn point<A>(a: A) -> Lifted<'a, OptionKind, A> {
        Some(a).lift()
    }
}

// Monad
impl<'a> Monad<'a, OptionKind> for OptionKind {
    fn flat_map<A, B, Func>(fa: Lifted<'a, OptionKind, A>, func: Func) -> Lifted<'a, OptionKind, B>
    where
        Func: Fn(A) -> Lifted<'a, OptionKind, B>,
    {
        match fa.unlift() {
            Some(t) => func(t),
            None => None.lift(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::*;

    #[test]
    fn test_lift_unlift() {
        let foo = "foo".to_owned();
        let o = Some(&foo).lift();

        let r = <OptionKind as Monad<_, _, _>>::map(o, |i| i.split_at(1));
        r.unlift();
    }

    #[bench]
    fn bench_functor_map(b: &mut Bencher) {
        b.iter(|| {
            for i in 0..10000 {
                black_box(
                    <OptionKind as Functor<_, _, _>>::map(Some(i).lift(), |i| i * 2).unlift(),
                );
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
