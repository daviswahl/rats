use functor::Functor;
use lifted::{Lifted, Nothing};

pub struct OptionT<'f, F, A, Z = Nothing>
where
    F: 'static,
    A: 'f,
    Z: 'f,
{
    pub value: Lifted<'f, F, Option<A>, Option<Z>>,
}

//impl<'f, F, A, Z, G> OptionT<'f, F, A, Z, G> {
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
