use std::marker::PhantomData;
use v2::lifted::*;
use v2::yieldable::Yieldable;

pub struct FMap<'d, K: HKT, B: 'd, F, D> {
    inner: D,
    f: F,
    __marker_b: PhantomData<&'d B>,
    __marker_k: (K),
}

impl<'l, K, B: 'l, F, L> FMap<'l, K, B, F, L>
where
    K: HKT,
{
    pub fn new(inner: L, f: F) -> Self {
        FMap {
            inner,
            f,
            __marker_k: K::marker(),
            __marker_b: PhantomData,
        }
    }

    pub fn call(&self, input: L::Item) -> B
    where
        L: Lifted<'l>,
        F: Fn(L::Item) -> B,
    {
        (self.f)(input)
    }

    pub fn inner(&self) -> &L {
        &self.inner
    }
}

pub trait Functor<'d, K: HKT>: HKT {
    #[inline]
    fn map<F, A, B, FA>(fa: FA, f: F) -> FMap<'d, K, B, F, FA>
    where
        F: Fn(FA::Item) -> B,
        FA: Lifted<'d, Kind = K, Item = A>;
}

pub trait FunctorExt<'d, K: Functor<'d, K> + Yieldable<K, A>, A>:
    Lifted<'d, Kind = K, Item = A>
{
    #[inline]
    fn map<F, B>(self, f: F) -> FMap<'d, K, B, F, Self>
    where
        Self: Sized,
        F: Fn(Self::Item) -> B;
}

impl<'d, K: Functor<'d, K> + Yieldable<K, A>, A, D> FunctorExt<'d, K, A> for D
where
    D: Lifted<'d, Kind = K, Item = A>,
{
    fn map<F, B>(self, f: F) -> FMap<'d, K, B, F, Self>
    where
        Self: Sized,
        F: Fn(Self::Item) -> B,
    {
        K::map(self, f)
    }
}
