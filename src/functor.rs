use lifted::{Lifted, Nothing, HKT};

pub trait Functor<'a, F: HKT + 'a, Z = Nothing, G = Nothing>: HKT {
    fn map<Func: 'a, A, B>(fa: Lifted<'a, F, A, Z, G>, func: Func) -> Lifted<'a, F, B, Z, G>
    where
        Func: Fn(A) -> B;
}

pub trait Functor2<'a, F: HKT + 'a, A, Z = Nothing, G = Nothing> {
    fn fmap<Func: 'a, B>(self, func: Func) -> Lifted<'a, F, B, Z, G>
    where
        Func: Fn(A) -> B;
}
