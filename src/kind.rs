use data::id::Id;
use futures::future::Future;
use std::marker::PhantomData;

pub trait HKT: Sized + 'static {}

#[derive(Clone, Debug, PartialEq)]
pub struct Empty;

pub trait EmptyType {}
impl EmptyType for Empty {}

#[allow(dead_code)]
pub enum Kind<'f_, F_: HKT, A: 'f_, B: 'f_ = Empty> {
    Vec(Vec<A>),
    Option(Option<A>),
    Id(Id<A>),
    Result(Result<A, B>),
    Future(Box<Future<Item = A, Error = B> + 'f_>),
    // Is this valid? also need to understand which pointer type to use here
    __MARKER(PhantomData<*const F_>),
}

pub trait Reify<F_: HKT, A, B = Empty> {
    type Out;
    fn reify(self) -> Self::Out;
}

pub trait ReifyRef<F_: HKT, A, B = Empty> {
    type Out;
    fn reify_as_ref(&self) -> &Self::Out;
}

pub trait IntoKind<'kind, F_: HKT, A: 'kind, B: 'kind = Empty> {
    type Kind: HKT;
    fn into_kind(self) -> Kind<'kind, F_, A, B>;
}

pub trait AsKind<F_: HKT, A, B = Empty> {
    type Kind: HKT;
    fn as_kind(&self) -> &Kind<F_, A, B>;
}
