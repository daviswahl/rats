use functor::Functor;
use kind::{Kind, HKT};
pub trait Applicative<K: HKT>: Functor<K> {
    fn ap<A, B, F>(fa: Kind<K, A>, ff: Kind<K, F>) -> Kind<K, B>
    where
        F: FnOnce(A) -> B;
    fn point<T>(value: T) -> Kind<K, T>;

    fn product<A, B>(fa: Kind<K, A>, fb: Kind<K, B>) -> Kind<K, (A, B)> {
        let f1 = |a| |b| (a, b);
        let t = Self::map(fa, f1);
        Self::ap(fb, t)
    }

    fn map2<F, A, B, Z>(fa: Kind<K, A>, fb: Kind<K, B>, f: F) -> Kind<K, Z>
    where
        F: Fn((A, B)) -> Z,
    {
        Self::map(Self::product(fa, fb), f)
    }
}

pub trait ApplicativeKindExt<K: Applicative<K>> {
    type Item;
    fn product<B>(self, Kind<K, B>) -> Kind<K, (Self::Item, B)>;

    fn ap<B, F>(self, ff: Kind<K, F>) -> Kind<K, B>
    where
        F: FnOnce(Self::Item) -> B;
}

impl<K, T> ApplicativeKindExt<K> for Kind<K, T>
where
    K: Applicative<K>,
{
    type Item = T;
    fn product<B>(self, fb: Kind<K, B>) -> Kind<K, (Self::Item, B)> {
        K::product(self, fb)
    }

    fn ap<B, F>(self, ff: Kind<K, F>) -> Kind<K, B>
    where
        F: FnOnce(Self::Item) -> B,
    {
        K::ap(self, ff)
    }
}

pub trait Point {
    type Out;
    fn point<F>(self) -> Kind<F, Self::Out>
    where
        F: HKT + Applicative<F>;
}

impl<T> Point for T {
    type Out = T;
    fn point<F: HKT + Applicative<F>>(self) -> Kind<F, T> {
        F::point::<T>(self)
    }
}
