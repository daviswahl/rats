use either::Either;
use functor::Functor;
use lifted::{Lifted, Nothing};

pub struct EitherT<'f, F, A, Z, G = Nothing>
where
    F: 'static,
    G: 'static,
    A: 'f,
    Z: 'f,
{
    pub value: Either<Lifted<'f, F, A, Nothing, G>, Lifted<'f, F, Z, Nothing, G>>,
}

impl<'f, F, A, Z, G> EitherT<'f, F, A, Z, G> {
    pub fn map<Func, B>(self, func: Func) -> EitherT<'f, F, B, Z, G>
    where
        Func: Fn(A) -> B + 'f,
        F: Functor<'f, F, Z, G>,
    {
        EitherT {
            value: self.value.map(|inner| F::map(inner, func)),
        }
    }
}
