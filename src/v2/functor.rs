use v2::hkt::{Kind, HKT};
pub trait Functor<K>
where
    K: HKT,
{
    fn map<F, A, B>(k: Kind<K, A>, f: F) -> Kind<K, B>
    where
        F: FnMut(A) -> B;
}

pub trait FunctorExt<K: HKT> {
    type Item;

    fn map<B, F>(self, f: F) -> Kind<K, B>
    where
        F: FnMut(Self::Item) -> B;
}

impl<K: HKT, T> FunctorExt<K> for Kind<K, T>
where
    K: Functor<K>,
{
    type Item = T;

    fn map<B, F>(self, f: F) -> Kind<K, B>
    where
        F: FnMut(Self::Item) -> B,
    {
        <K as Functor<K>>::map(self, f)
    }
}
