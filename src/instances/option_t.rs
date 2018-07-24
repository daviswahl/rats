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
    use std::marker::PhantomData;

    #[test]
    fn test_lift() {
        let f = OptionT {
            value: Some(vec![1, 2, 3].into_iter().lift()),
        }.lift();
    }

    #[test]
    fn test_functor() {
        let f = OptionT {
            value: Some(vec![1, 2, 3].into_iter().lift()),
        }.lift();

        let result = OptionTKind::map(f, |i| i * 2);
        assert_eq!(
            result
                .unlift()
                .value
                .map(|i| i.unlift().collect::<Vec<i32>>()),
            Some(vec![2, 4, 6])
        )
    }
}
