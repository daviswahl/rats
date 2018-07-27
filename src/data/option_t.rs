use functor::Functor;
use lifted::{Lifted, Nothing};

pub struct OptionT<'f, F, A, Z = Nothing>
where
    F: 'static,
    A: 'f,
    Z: 'f,
{
    pub value: Option<Lifted<'f, F, A, Z>>,
}

impl<'f, F, A, Z> OptionT<'f, F, A, Z> {
    pub fn map<Func, B>(self, func: Func) -> OptionT<'f, F, B, Z>
    where
        Func: Fn(A) -> B + 'f,
        F: Functor<'f, F, Z>,
    {
        OptionT {
            value: self.value.map(|inner| F::map(inner, func)),
        }
    }
}
