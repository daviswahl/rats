use std::marker::PhantomData;
use v2::erased::Erased;

pub trait HKT {
    fn marker() -> Self;
}

pub trait Kinded<K: HKT, T> {
    type Kind = K;
}

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

    pub unsafe fn unwrap<A: Kinded<K,T>>(self) -> A {
        self.data.unerase()
    }
}

pub trait Unkind<K: HKT, T> {
    type Out: Kinded<K,T>;
    fn unkind(k: Kind<K,T>) -> Self::Out;
}

#[cfg(test)]
mod tests {
    use v2::conversions::*;

    #[test]
    fn test_must_use() {
       vec![1,2,3].into_kind();
    }
}
