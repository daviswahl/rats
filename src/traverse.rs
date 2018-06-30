use applicative::Applicative;
use functor::Functor;
use kind::{Kind, HKT};

pub trait Traverse<K: HKT>: Functor<K> {
    fn traverse<F, G, A, B>(fa: Kind<K, A>, f: F) -> Kind<G, Kind<K, B>>
    where
        G: Applicative<G>,
        F: Fn(A) -> Kind<G, B>;
}

pub trait TraverseExt<K: Traverse<K>> {
    type Item;
    fn traverse<F, G, B>(self, f: F) -> Kind<G, Kind<K, B>>
    where
        G: Applicative<G>,
        F: Fn(Self::Item) -> Kind<G, B>;
}

impl<K, T> TraverseExt<K> for Kind<K, T>
where
    K: Traverse<K>,
{
    type Item = T;
    fn traverse<F, G, B>(self, f: F) -> Kind<G, Kind<K, B>>
    where
        G: Applicative<G>,
        F: Fn(Self::Item) -> Kind<G, B>,
    {
        K::traverse(self, f)
    }
}
