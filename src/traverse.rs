use applicative::Applicative;
use functor::Functor;
use kind::{Kind, HKT};

pub trait Traverse<F_: HKT>: Functor<F_> {
    fn traverse<'kind, F, G_, A, B>(
        fa: Kind<'kind, F_, A>,
        f: F,
    ) -> Kind<'kind, G_, Kind<'kind, F_, B>>
    where
        G_: Applicative<G_>,
        F: Fn(A) -> Kind<'kind, G_, B>;
}

pub trait TraverseExt<'kind, F_: Traverse<F_>> {
    type Item;
    fn traverse<Func, G_, B>(self, f: Func) -> Kind<'kind, G_, Kind<'kind, F_, B>>
    where
        G_: Applicative<G_>,
        Func: Fn(Self::Item) -> Kind<'kind, G_, B>;
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
