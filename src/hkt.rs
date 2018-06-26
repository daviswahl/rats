use std::marker::PhantomData;
use erased::Erased;
use std::ops::Deref;

pub trait HKT {
    fn marker() -> Self;
}

pub trait Kinded<K: HKT, T> { }

#[must_use]
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
            data: unsafe { Erased::erase(k) },
        }
    }

    pub fn unkind(self) -> <Self as Unkind<K,T>>::Out where Self: Unkind<K,T> {
        <Self as Unkind<K,T>>::unkind(self)
    }

    pub fn unkind_ref(&self) -> &<Self as UnkindRef<K,T>>::Out where Self: UnkindRef<K,T> {
        unimplemented!()
    }

    pub unsafe fn unwrap<A: Kinded<K,T>>(self) -> A {
        self.data.unerase()
    }
}

impl<K: HKT,T> Deref for Kind<K,T> where Self: Unkind<K,T> {
    type Target = <Self as Unkind<K,T>>::Out;
    fn deref(&self) -> &<Self as Deref>::Target {
        unimplemented!()
    }
}

pub trait Unkind<K: HKT, T> {
    type Out: Kinded<K,T>;
    fn unkind(k: Kind<K,T>) -> Self::Out;
}

pub trait UnkindRef<K: HKT, T> {
    type Out: Kinded<K,T>;
    fn unkind_ref(&self) -> &Self::Out;
}
#[cfg(test)]
mod tests {
    use conversions::*;

    #[test]
    fn test_must_use() {
       vec![1,2,3].into_kind();
    }
}
