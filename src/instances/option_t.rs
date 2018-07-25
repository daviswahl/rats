use data::option_t;
use data::option_t::OptionT;
use functor::Functor;
use lifted::{Lift, Lifted, Nothing, Unlift, HKT};

struct OptionTKind;
impl HKT for OptionTKind {}

impl<'a, Z, G> Functor<'a, OptionTKind, Z, G> for OptionTKind
where
    G: Functor<'a, G, Z> + 'a,
{
    fn map<Func, A, B>(
        fa: Lifted<'a, OptionTKind, A, Z, G>,
        func: Func,
    ) -> Lifted<'a, OptionTKind, B, Z, G>
    where
        Func: Fn(A) -> B + 'a,
    {
        fa.unlift().map(func).lift()
    }
}

impl<'a, A, Z, G> Lift<'a, OptionTKind, A, Z, G> for OptionT<'a, G, A, Z> {
    fn lift(self) -> Lifted<'a, OptionTKind, A, Z, G> {
        Lifted::OptionT(Box::new(self))
    }
}

impl<'a, G, A, Z> Unlift<OptionTKind> for Lifted<'a, OptionTKind, A, Z, G> {
    type Out = OptionT<'a, G, A, Z>;

    fn unlift(self) -> <Self as Unlift<OptionTKind>>::Out {
        match self {
            Lifted::OptionT(opt) => *opt,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;
    use std::marker::PhantomData;

    #[test]
    fn test_lift() {
        let mut v = VecDeque::new();
        v.push_back(1);
        v.push_back(2);
        v.push_back(3);
        let f = OptionT {
            value: Some(v.lift()),
        }.lift();
    }

    #[test]
    fn test_functor() {
        let mut v = VecDeque::new();
        v.push_back(1);
        v.push_back(2);
        v.push_back(3);
        let f = OptionT {
            value: Some(v.lift()),
        }.lift();

        let result = OptionTKind::map(f, |i| i * 2)
            .unlift()
            .value
            .map(|i| i.unlift());

        let mut e = VecDeque::new();
        e.push_back(2);
        e.push_back(4);
        e.push_back(6);
        assert_eq!(result, Some(e))
    }
}
