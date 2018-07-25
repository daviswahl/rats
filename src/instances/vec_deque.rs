use functor::Functor;
use lifted::*;
use std::collections::VecDeque;

struct VecDequeKind;
impl HKT for VecDequeKind {}

impl<'a, A> Lift<'a, VecDequeKind, A> for VecDeque<A> {
    fn lift(self) -> Lifted<'a, VecDequeKind, A> {
        Lifted::VecDeque(self)
    }
}

impl<'a, A> Unlift<VecDequeKind> for Lifted<'a, VecDequeKind, A> {
    type Out = VecDeque<A>;

    fn unlift(self) -> <Self as Unlift<VecDequeKind>>::Out {
        match self {
            Lifted::VecDeque(a) => a,
            _ => unimplemented!(),
        }
    }
}

impl<'a> Functor<'a, VecDequeKind> for VecDequeKind {
    fn map<Func: 'a, A, B>(
        fa: Lifted<'a, VecDequeKind, A, Nothing, Nothing>,
        func: Func,
    ) -> Lifted<'a, VecDequeKind, B, Nothing, Nothing>
    where
        Func: Fn(&A) -> B,
    {
        fa.unlift()
            .into_iter()
            .map(|f| func(&f))
            .collect::<VecDeque<B>>()
            .lift()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lift() {
        let mut v = VecDeque::new();
        v.push_front("foo");
        v.push_front("bar");

        v.lift().unlift();
    }
}
