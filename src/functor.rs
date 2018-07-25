use lifted::{Lifted, Nothing, HKT};

pub trait Functor<'f, F: HKT + 'f, Z = Nothing, G = Nothing>: HKT {
    fn map<Func, A, B>(fa: Lifted<'f, F, A, Z, G>, func: Func) -> Lifted<'f, F, B, Z, G>
    where
        Func: Fn(A) -> B + 'f;
}
