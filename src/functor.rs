use lifted::{Lifted, Nothing, HKT};

pub trait Functor<F: HKT> {
    type B = Nothing;
    type G = Nothing;
    type Func = Nothing;

    fn map<Func, A, B>(
        fa: Lifted<F, A, Self::B, Self::G, Self::Func>,
        func: Func,
    ) -> Lifted<F, A, Self::B, Self::G, Self::Func>
    where
        Func: Fn(A) -> B;
}
