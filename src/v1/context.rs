use bincode::{deserialize, serialize};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::marker;
use v1::kind::Kind;
use v1::kinds;

#[derive(Debug, Clone)]
pub struct Context<K: Kind, I: DeserializeOwned> {
    kind: K,
    _marker: marker::PhantomData<*const I>,
}

impl<K: Kind, I: DeserializeOwned> Context<K, I> {
    pub fn from<C>(c: C) -> Self
    where
        C: IntoContext<Kind = K, Item = I> + EraseType<C>,
    {
        Context {
            kind: <K as Kind>::from_boxed_slice(<C as EraseType<C>>::erase(c)),
            _marker: marker::PhantomData,
        }
    }
}

pub unsafe trait UnsafePointerCast {
    unsafe fn unsafe_cast<T>(self) -> T
    where
        T: DeserializeOwned;
}

unsafe impl<K: Kind, T: DeserializeOwned> UnsafePointerCast for Context<K, T> {
    unsafe fn unsafe_cast<G>(self) -> G
    where
        G: DeserializeOwned,
    {
        deserialize(self.kind.buf().as_slice()).unwrap()
    }
}

pub trait ExtractKind<C>
where
    C: UnsafePointerCast + FromContext,
    C::Out: NewType,
    C::Out: DeserializeOwned,
{
    fn extract(c: C) -> <C::Out as NewType>::Type;
}

impl<C> ExtractKind<C> for C
where
    C: UnsafePointerCast + FromContext,
    C::Out: NewType,
    C::Out: DeserializeOwned,
{
    fn extract(c: C) -> <C::Out as NewType>::Type {
        unsafe { c.unsafe_cast::<C::Out>() }.get()
    }
}

pub trait ExtractExt<C>
where
    C: UnsafePointerCast + FromContext + ExtractKind<C>,
    <C as FromContext>::Out: DeserializeOwned,
{
    fn extract(self) -> <<C as FromContext>::Out as NewType>::Type;
}

impl<C> ExtractExt<C> for C
where
    C: UnsafePointerCast + FromContext + ExtractKind<C>,
    <C as FromContext>::Out: DeserializeOwned,
{
    fn extract(self) -> <<C as FromContext>::Out as NewType>::Type {
        <C as ExtractKind<C>>::extract(self)
    }
}

pub trait EraseType<C> {
    fn erase(t: C) -> Vec<u8>;
}

impl<C: Serialize> EraseType<C> for C {
    fn erase(t: C) -> Vec<u8> {
        serialize(&t).unwrap()
    }
}

pub trait IntoContextExt<C: IntoContext + EraseType<C>>
where
    C::Item: DeserializeOwned,
{
    fn into_context(self) -> Context<C::Kind, C::Item>;
}

impl<C: IntoContext + EraseType<C> + Serialize> IntoContextExt<C> for C
where
    C::Item: DeserializeOwned,
{
    fn into_context(self) -> Context<<C as IntoContext>::Kind, <C as IntoContext>::Item> {
        Context::from(self)
    }
}

pub trait NewType {
    type Type;
    fn get(self) -> Self::Type;
}

pub trait IntoContext {
    type Kind: Kind;
    type Item;
}

pub trait FromContext {
    type Out: NewType;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn extractor_test() {
        let k: Context<kinds::Vec, i32> = vec![1, 2, 3].into_context();
        println!("got k: {:?}", k);
        let result = k.extract();
        println!("extracted from k: {:?}", result);
        assert_eq!(result, vec![1, 2, 3])
    }

    #[test]
    fn clone_test() {
        let k: Context<kinds::Vec, i32> = Context::from(vec![1, 2, 3]);
        assert_eq!(k.clone().extract(), vec![1, 2, 3]);
        assert_eq!(k.extract(), vec![1, 2, 3]);
    }
}
