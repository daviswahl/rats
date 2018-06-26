use erased::Erased;
use std::marker::PhantomData;

pub trait HKT {
    fn marker() -> Self;
}

pub trait Kinded<K: HKT, T> {}

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

    pub fn unkind(self) -> <Self as Unkind<K, T>>::Out
    where
        Self: Unkind<K, T>,
    {
        <Self as Unkind<K, T>>::unkind(self)
    }

    pub fn unkind_ref(&self) -> &<Self as UnkindRef<K, T>>::Out
    where
        Self: UnkindRef<K, T>,
    {
        <Self as UnkindRef<K, T>>::unkind_ref(self)
    }

    pub unsafe fn unwrap<A: Kinded<K, T>>(self) -> A {
        self.data.unerase()
    }

    pub unsafe fn unwrap_ref<A: Kinded<K, T>>(&self) -> &A {
        (&self.data).unerase_ref()
    }
}

pub trait Unkind<K: HKT, T> {
    type Out: Kinded<K, T>;
    fn unkind(k: Kind<K, T>) -> Self::Out;
}

pub trait UnkindRef<K: HKT, T> {
    type Out: Kinded<K, T>;
    fn unkind_ref(&self) -> &Self::Out;
}

#[cfg(test)]
mod tests {
    use conversions::*;

    #[test]
    fn test_must_use() {
        vec![1, 2, 3].into_kind();
    }

    #[test]
    fn test_unkind_ref() {
        let vec = vec![1, 2, 3];
        let v = vec.clone().into_kind();
        //let r = v.unkind_ref();
        //assert_eq!(r, &vec);
    }
}
