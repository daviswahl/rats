use erased::Erased;
use std::marker::PhantomData;

pub trait HKT {
    fn marker() -> Self;
}

pub trait Kinded<K: HKT, T> {}

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
            data: Erased::erase(k)
        }
    }

    pub fn reify(self) -> <Self as Reify<K, T>>::Out
    where
        Self: Reify<K, T>,
    {
        unsafe {
            self.data.reify()
        }
    }

    pub fn reify_as_ref(&self) -> &<Self as Reify<K, T>>::Out
    where
        Self: Reify<K, T>,
    {
        unsafe {
            self.data.reify_as_ref()
        }
    }

    pub fn reify_as_mut_ref(&mut self) -> &mut <Self as Reify<K,T>>::Out
    where Self: Reify<K,T>
    {
        unsafe { self.data.reify_as_mut_ref() }
    }
}

pub trait Reify<K: HKT, T> {
    type Out: Kinded<K, T>;
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
