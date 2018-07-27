use functor::Functor;
use lifted::{Lifted, Nothing};

/// OptionT supports G but not when Lifted as the G parameter will be used as F in the Lifted enum.
pub struct OptionT<'f, F, A, Z = Nothing, G = Nothing>
where
    F: 'static,
    G: 'static,
    A: 'f,
    Z: 'f,
{
    pub value: Option<Lifted<'f, F, A, Z, G>>,
}

impl<'f, F, A, Z, G> OptionT<'f, F, A, Z, G> {
    pub fn map<Func, B>(self, func: Func) -> OptionT<'f, F, B, Z, G>
    where
        Func: Fn(A) -> B + 'f,
        F: Functor<'f, F, Z, G>,
    {
        OptionT {
            value: self.value.map(|inner| F::map(inner, func)),
        }
    }
}
