use lifted::{Lifted, Nothing, HKT};

pub trait Functor<F: HKT> {
    type B;
    type G;

    fn map<Func, A, B>(
        fa: Lifted<F, A, Self::B, Self::G>,
        func: Func,
    ) -> Lifted<F, B, Self::B, Self::G>
    where
        Func: Fn(A) -> B;
}
