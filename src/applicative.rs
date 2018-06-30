use functor::Functor;
use kind::{Kind, HKT};
pub trait Applicative<K: HKT>: Functor<K> {
    fn ap<'kind, A: 'kind, B: 'kind, F>(
        fa: Kind<'kind, K, A>,
        ff: Kind<'kind, K, F>,
    ) -> Kind<'kind, K, B>
    where
        F: FnOnce(A) -> B;
    fn point<'kind, T: 'kind>(value: T) -> Kind<'kind, K, T>;

    fn product<'kind, A: 'kind, B: 'kind>(
        fa: Kind<'kind, K, A>,
        fb: Kind<'kind, K, B>,
    ) -> Kind<'kind, K, (A, B)> {
        let f1 = |a| |b| (a, b);
        let t = Self::map(fa, f1);
        Self::ap(fb, t)
    }

    fn map2<'kind, F, A, B, Z>(
        fa: Kind<'kind, K, A>,
        fb: Kind<'kind, K, B>,
        f: F,
    ) -> Kind<'kind, K, Z>
    where
        F: Fn((A, B)) -> Z + 'kind,
    {
        Self::map(Self::product(fa, fb), f)
    }
}

pub trait ApplicativeKindExt<'kind, K: Applicative<K>> {
    type Item;
    fn product<B>(self, Kind<'kind, K, B>) -> Kind<'kind, K, (Self::Item, B)>;

    fn ap<B, F>(self, ff: Kind<'kind, K, F>) -> Kind<'kind, K, B>
    where
        F: FnOnce(Self::Item) -> B;
}

impl<'kind, K, T> ApplicativeKindExt<'kind, K> for Kind<'kind, K, T>
where
    K: Applicative<K>,
{
    type Item = T;
    fn product<B>(self, fb: Kind<'kind, K, B>) -> Kind<'kind, K, (Self::Item, B)> {
        K::product(self, fb)
    }

    fn ap<B, F>(self, ff: Kind<'kind, K, F>) -> Kind<'kind, K, B>
    where
        F: FnOnce(Self::Item) -> B,
    {
        K::ap(self, ff)
    }
}

pub trait Point<'kind> {
    type Out;
    fn point<F>(self) -> Kind<'kind, F, Self::Out>
    where
        F: HKT + Applicative<F>;
}

impl<'kind, T: 'kind> Point<'kind> for T {
    type Out = T;
    fn point<F: HKT + Applicative<F>>(self) -> Kind<'kind, F, T> {
        F::point::<T>(self)
    }
}
