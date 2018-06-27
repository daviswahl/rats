use hkt::HKT;
use kind::Kind;

pub trait Functor<K: HKT> {
    fn map<A,B, F>(a: Kind<K, A>, f: F) -> Kind<K,B>
        where F: Fn(A) -> B;
}

pub trait KindFunctorExt<K: HKT> where K: Functor<K> {
    type Item;
    fn map<B,F>(self, f: F) -> Kind<K,B> where
        F: Fn(Self::Item) -> B;
}

impl<K, T> KindFunctorExt<K> for Kind<K, T> where K: HKT+Functor<K> {
    type Item = T;

    fn map<B, F>(self, f: F) -> Kind<K, B> where
        F: Fn(Self::Item) -> B {
        K::map(self, f)
    }
}
