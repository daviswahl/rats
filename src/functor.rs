use lifted::{Lifted, Nothing, HKT};

pub trait Functor<'a, F, Z = Nothing, G = Nothing>: HKT
where
    F: HKT,
{
    fn map<Func, A, B>(fa: Lifted<'a, F, A, Z, G>, func: Func) -> Lifted<'a, F, B, Z, G>
    where
        Func: Fn(A) -> B + 'a;
}
