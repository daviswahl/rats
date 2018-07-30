use functor::Functor;
use instances::future::FutureKind;
use instances::option::OptionKind;
use instances::vec_deque::VecDequeKind;
use lifted::Lift;
use lifted::Unlift;
use lifted::HKT;
use lifted::{Lifted, Nothing};
use std::collections::VecDeque;

/// OptionT supports G but not when Lifted as the G parameter will be used as F in the Lifted enum.
pub struct OptionT<'f, F, A, Z = Nothing>
where
    F: 'static,
    A: 'f,
    Z: 'f,
{
    pub value: Lifted<'f, F, A, Z, OptionKind>,
}

impl<'f, F, A, Z> OptionT<'f, F, A, Z> where {
    pub fn map<Func, B>(self, func: &'f Func) -> OptionT<'f, F, B, Z>
    where
        Func: Fn(A) -> B + 'f,
        F: Functor<'f, F, Z, OptionKind>,
    {
        OptionT {
            value: F::map(self.value, func),
        }
    }
}

pub struct ;
impl HKT for OptionTVecDeque {}

impl<'f> Functor<'f, VecDequeKind, Nothing, OptionKind> for OptionTVecDeque {
    fn map<Func, A, B>(
        fa: Lifted<'f, VecDequeKind, A, Nothing, OptionKind>,
        func: Func,
    ) -> Lifted<'f, VecDequeKind, B, Nothing, OptionKind>
    where
        Func: Fn(A) -> B + 'f,
    {
        unimplemented!()
    }
}

trait Lift2<'f, F, A, Z, G> {
    fn lift_2(self) -> Lifted<'f, F, A, Z, G>;
}

impl<'f, A> Lift2<'f, VecDequeKind, A, Nothing, OptionKind> for VecDeque<Option<A>> {
    fn lift_2(self) -> Lifted<'f, VecDequeKind, A, Nothing, OptionKind>
    {

    }
}

impl<'f, A> Unlift<VecDequeKind> for Lifted<'f, VecDequeKind, A, Nothing, OptionKind> {
    type Out = VecDeque<Option<A>>;

    fn unlift(self) -> <Self as Unlift<VecDequeKind>>::Out {
        unimplemented!()
    }
}
