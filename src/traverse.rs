use applicative::Applicative;
use functor::Functor;
use kind::{Kind, HKT};

pub trait Traverse<K: HKT>: Functor<K> {
    fn traverse<'kind, F, G, A, B>(
        fa: Kind<'kind, K, A>,
        f: F,
    ) -> Kind<'kind, G, Kind<'kind, K, B>>
    where
        G: Applicative<G>,
        F: Fn(A) -> Kind<'kind, G, B>;
}

pub trait TraverseExt<'kind, K: Traverse<K>> {
    type Item;
    fn traverse<F, G, B>(self, f: F) -> Kind<'kind, G, Kind<'kind, K, B>>
    where
        G: Applicative<G>,
        F: Fn(Self::Item) -> Kind<'kind, G, B>;
}

impl<'kind, K, T> TraverseExt<'kind, K> for Kind<'kind, K, T>
where
    K: Traverse<K>,
{
    type Item = T;
    fn traverse<F, G, B>(self, f: F) -> Kind<'kind, G, Kind<'kind, K, B>>
    where
        G: Applicative<G>,
        F: Fn(Self::Item) -> Kind<'kind, G, B>,
    {
        K::traverse(self, f)
    }
}
