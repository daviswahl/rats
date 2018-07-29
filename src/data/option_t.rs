use data::kleisli::RcFn;
use functor::Functor;
use instances::option::OptionKind;
use lifted::{Lifted, Nothing};
use std::marker::PhantomData;
use std::rc::Rc;

pub mod t {
    use instances::option::OptionKind;
    use lifted::{Lifted, Nothing};
    pub trait OptionT<'f, F, A, Z = Nothing, G = Nothing>
    where
        F: 'static,
        A: 'f,
        Z: 'f,
    {
        fn value(self) -> Lifted<'f, F, Lifted<'f, OptionKind, A>, Z, G>;
    }
}

pub trait OptionTExt<'f, F, A, Z = Nothing, G = Nothing>
where
    F: 'static,
    G: 'static,
    A: 'f,
    Z: 'f,
    Self: Sized + t::OptionT<'f, F, A, Z, G>,
{
    fn map<Func, B>(self, func: Func) -> Map<Self, Func, A>
    where
        Func: Fn(A) -> B;
}

impl<'f, F, A, Z, G, O> OptionTExt<'f, F, A, Z, G> for O
where
    F: 'static,
    G: 'static,
    A: 'f,
    Z: 'f,
    O: t::OptionT<'f, F, A, Z, G>,
{
    fn map<Func, B>(self, func: Func) -> Map<Self, Func, A> {
        Map {
            inner: self,
            func: RcFn(Rc::new(func)),
            a: PhantomData,
        }
    }
}

pub struct OptionT<'f, F, A, Z = Nothing, G = Nothing>(
    pub Lifted<'f, F, Lifted<'f, OptionKind, A>, Z, G>,
)
where
    G: 'static,
    F: 'static,
    A: 'f,
    Z: 'f;

impl<'f, F, A, Z, G> t::OptionT<'f, F, A, Z, G> for OptionT<'f, F, A, Z, G> {
    fn value(self) -> Lifted<'f, F, Lifted<'f, OptionKind, A>, Z, G> {
        self.0
    }
}

pub struct Map<OptT, Func, A> {
    inner: OptT,
    // Really don't think RC is necessary here
    func: RcFn<Func>,
    a: PhantomData<*const A>,
}

impl<'f, F, A, Z, G, B, Func, OptT> t::OptionT<'f, F, B, Z, G> for Map<OptT, Func, A>
where
    A: 'f,
    Z: 'f,
    B: 'f,
    Func: Fn(A) -> B + 'f,
    OptT: t::OptionT<'f, F, A, Z, G> + 'f,
    F: Functor<'f, F, Z, G>,
{
    fn value(self) -> Lifted<'f, F, Lifted<'f, OptionKind, B>, Z, G> {
        let (inner, func) = (self.inner, self.func);
        F::map(inner.value(), move |l| OptionKind::map(l, func.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::t::OptionT as opt;
    use super::*;
    use futures::executor::block_on;
    use futures::future::lazy;
    use instances::future::FutureKind;
    use lifted::{Lift, Unlift};
    use std::collections::VecDeque;

    #[test]
    fn test_map() {
        let mut v = VecDeque::new();
        v.push_back(Some(1).lift());
        v.push_back(Some(2).lift());

        let opt = OptionT(v.lift());
        let opt = opt.map(|i| i * 2);
        let result = opt
            .value()
            .unlift()
            .into_iter()
            .map(|i| i.unlift())
            .collect::<VecDeque<Option<i32>>>();

        let mut expected = VecDeque::new();
        expected.push_back(Some(2));
        expected.push_back(Some(4));
        assert_eq!(result, expected)
    }
}
//    pub fn map<Func, B>(self, func: Func) -> OptionT<'f, F, B, Z, G>
//    where
//        Func: Fn(A) -> B + 'f,
//        F: Functor<'f, F, Z, G>,
//    {
//        OptionT {
//            value: self.value.map(|inner| F::map(inner, func)),
//        }
//    }
//}
