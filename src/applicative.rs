use functor::Functor;
use kind::{Kind, HKT};
pub trait Applicative<K: HKT>: Functor<K> {
    fn ap<A, B, F>(fa: Kind<K, A>, ff: Kind<K, F>) -> Kind<K, B>
    where
        F: FnOnce(A) -> B;

    fn product<A, B>(fa: Kind<K, A>, fb: Kind<K, B>) -> Kind<K, (A, B)> {
        let f1 = |a| |b| (a, b);
        let t = <Self as Applicative<K>>::map(fa, f1);
        Self::ap(fb, t)
    }

    fn point<T>(value: T) -> Kind<K, T>;
    fn map<F, A, B>(fa: Kind<K, A>, f: F) -> Kind<K, B>
    where
        F: Fn(A) -> B,
    {
        Self::ap(fa, Self::point(f))
    }
}

pub trait ApplicativeExt {
    type Out;
    fn point<F>(self) -> Kind<F, Self::Out>
    where
        F: HKT + Applicative<F>;
}

pub trait ApplicativeKindExt<K: HKT + Applicative<K>> {
    type Item;
    fn product<B>(self, Kind<K, B>) -> Kind<K, (Self::Item, B)>;

    fn ap<B, F>(self, ff: Kind<K, F>) -> Kind<K, B>
    where
        F: FnOnce(Self::Item) -> B;
}

impl<K, T> ApplicativeKindExt<K> for Kind<K, T>
where
    K: HKT + Applicative<K>,
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

impl<T> ApplicativeExt for T {
    type Out = T;
    fn point<F: HKT + Applicative<F>>(self) -> Kind<F, T> {
        F::point::<T>(self)
    }
}
