use lifted::{Lifted, Nothing, HKT};

pub trait Functor<'a, F: HKT + 'a, Z = Nothing, G = Nothing>: HKT {
    fn map<Func: 'a, A, B>(fa: Lifted<'a, F, A, Z, G>, func: Func) -> Lifted<'a, F, B, Z, G>
    where
        Func: Fn(&A) -> B;
}
