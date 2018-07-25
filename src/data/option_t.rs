use functor::Functor;
use lifted::Unlift;
use lifted::{Lifted, Nothing};

pub struct OptionT<'a, F, A, Z = Nothing>
where
    F: 'a,
    A: 'a,
    Z: 'a,
{
    pub value: Option<Lifted<'a, F, A, Z>>,
}

impl<'a, F, A, Z> OptionT<'a, F, A, Z> {
    pub fn map<Func, B>(self, func: Func) -> OptionT<'a, F, B, Z>
    where
        Func: Fn(&A) -> B + 'a,
        F: Functor<'a, F, Z>,
    {
        OptionT {
            value: self.value.map(|inner| F::map(inner, func)),
        }
    }
}
