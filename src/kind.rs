use erased::Erased;
use hkt::*;
use std::marker::PhantomData;
#[must_use]
#[allow(dead_code)]
pub struct Kind<K, T>
where
    K: HKT,
{
    kind: K,
    _marker: PhantomData<*const T>,
    data: Erased,
}

impl<K, T> Kind<K, T>
where
    K: HKT,
{
    pub fn new<A>(k: A) -> Kind<K, T>
    where
        A: Kinded<K, T>,
    {
        Kind {
            kind: K::marker(),
            _marker: PhantomData,
            data: Erased::erase(k),
        }
    }

    pub fn reify(self) -> <Self as Reify<K, T>>::Out
    where
        Self: Reify<K, T>,
    {
        unsafe { self.data.reify() }
    }

    pub fn reify_as_ref(&self) -> &<Self as Reify<K, T>>::Out
    where
        Self: Reify<K, T>,
    {
        unsafe { self.data.reify_as_ref() }
    }

    pub fn reify_as_mut_ref(&mut self) -> &mut <Self as Reify<K, T>>::Out
    where
        Self: Reify<K, T>,
    {
        unsafe { self.data.reify_as_mut_ref() }
    }
}
