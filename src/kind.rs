use data::id::Id;
use std::marker::PhantomData;

pub trait HKT: Sized + 'static + Send + Sync {}

#[derive(Clone, Debug, PartialEq)]
pub struct Empty;

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum Kind<K: HKT, A, B = Empty> {
    Vec(Vec<A>),
    Option(Option<A>),
    Id(Id<A>),
    Result(Result<A, B>),
    // Is this valid? also need to understand which pointer type to use here
    __MARKER(PhantomData<*const K>),
}

pub trait Reify<K: HKT, A, B = Empty> {
    type Out;
    fn reify(self) -> Self::Out;
}

pub trait IntoKind<K: HKT, A, B = Empty> {
    type Kind: HKT;
    fn into_kind(self) -> Kind<K, A, B>;
}
