use std::any::Any;
use std::marker::PhantomData;
use std::ops::Deref;
use v2::erased::Erased;

pub trait HKT {
    fn marker() -> Self;
}

pub trait Kinded<K: HKT, T> {
    type Kind = K;
}

pub struct Kind<K, A>
where
    K: HKT,
{
    kind: K,
    _marker: PhantomData<*const A>,
    data: Erased,
}

impl<K, A> Kind<K, A>
where
    K: HKT,
{
    pub fn new<T>(k: T) -> Kind<K, A>
    where
        T: Kinded<K, A>,
    {
        Kind {
            kind: K::marker(),
            _marker: PhantomData,
            data: Erased::erase(k),
        }
    }

    pub unsafe fn unwrap<T: Sized>(self) -> T {
        self.data.unerase()
    }
}

