use functor::Functor;
use instances::option::OptionKind;
use lifted::{Lifted, Nothing};
use std::marker::PhantomData;

pub mod t {
    use instances::option::OptionKind;
    use lifted::{Lifted, Nothing};
    pub trait OptionT<'f, F, A, Z = Nothing, G = Nothing>
    where
        F: 'static,
        A: 'f,
        Z: 'f,
    {
        fn value(self) -> Lifted<'f, OptionKind, Lifted<'f, F, A, Z, G>>;
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
    fn map<Func, B>(self, func: Func) -> Map<B, Self, Func>;
}

impl<'f, F, A, Z, G, O> OptionTExt<'f, F, A, Z, G> for O
where
    F: 'static,
    G: 'static,
    A: 'f,
    Z: 'f,
    O: t::OptionT<'f, F, A, Z, G>,
{
    fn map<Func, B>(self, func: Func) -> Map<B, Self, Func> {
        unimplemented!()
    }
}

pub struct OptionT<'f, F, A, Z = Nothing, G = Nothing>(
    pub Lifted<'f, OptionKind, Lifted<'f, F, A, Z, G>>,
)
where
    G: 'static,
    F: 'static,
    A: 'f,
    Z: 'f;

impl<'f, F, A, Z, G> t::OptionT<'f, F, A, Z, G> for OptionT<'f, F, A, Z, G> {
    fn value(self) -> Lifted<'f, OptionKind, Lifted<'f, F, A, Z, G>> {
        self.0
    }
}

pub struct Map<B, Func, OptT> {
    inner: OptT,
    func: Func,
    b: PhantomData<*const B>,
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
