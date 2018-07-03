use data::id::Id;
use futures::future::Future;
use std::marker::{PhantomData, Send, Sync};
use std::any::Any;
use std::fmt;
use std::fmt::{Debug, Formatter};

pub trait HKT: Sync + Send + Sized + 'static {
    type Kind: HKT;
}

pub trait AnyKind<A, B = Empty>: 'static {
    type Out;
    type Kind: HKT;
}

#[derive(Clone, Debug, PartialEq)]
pub struct Empty {}
impl HKT for Empty {
    type Kind = Empty;
}

#[allow(dead_code)]
pub enum Kind<'f_, F_: HKT, A, B = Empty>
where
    F_: HKT,
    A: 'f_,
    B: 'f_,
{
    Vec(Vec<A>),
    Option(Option<A>),
    Id(Id<A>),
    Result(Result<A, B>),
    Future(Box<Future<Item = A, Error = B> + 'f_>),
    Any(Box<Any + 'static>),
    // Is this valid? also need to understand which pointer type to use here
    __MARKER(PhantomData<*const F_>),
}

pub trait ReifyKind<'f_, F_: HKT, A, B = Empty> {
    type Out;
    fn reify(fa: Kind<'f_, F_, A, B>) -> Self::Out;
}

pub trait ReifyRef<F_: HKT, A, B = Empty> {
    type Out;
    fn reify_as_ref(&self) -> &Self::Out;
}

pub trait IntoKind<'f_, F_: HKT, A: 'f_, B: 'f_ = Empty> {
    type Kind: HKT;
    fn into_kind(self) -> Kind<'f_, F_, A, B>;
}

pub trait AsKind<F_: HKT, A, B = Empty> {
    type Kind: HKT;
    fn as_kind(&self) -> Kind<F_, &A, &B>;
}

pub trait IntoAnyKind<F_: HKT, A: 'static, B: 'static = Empty> {
    type Kind: HKT;
    fn into_kind(self) -> Kind<'static, F_, A, B>;
}

pub trait Reify<'f_, F_: HKT, A, B = Empty>
where
    F_: ReifyKind<'f_, F_, A, B>,
{
    fn reify(self) -> F_::Out;
}

impl<'f_, F_, A, B> Reify<'f_, F_, A, B> for Kind<'f_, F_, A, B>
where
    F_: HKT + ReifyKind<'f_, F_, A, B>,
{
    fn reify(self) -> F_::Out {
        F_::reify(self)
    }
}

impl<'f_, F_, A, B> Debug for Kind<'f_, F_, A, B>
where
    F_: HKT,
    A: Debug,
    B: Debug,
    Self: ReifyRef<F_, A, B>,
    <Self as ReifyRef<F_, A, B>>::Out: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Kind<'f_,{:?}>", self.reify_as_ref())
    }
}

impl<'f_, F_, A, B> PartialEq for Kind<'f_, F_, A, B>
where
    F_: HKT,
    A: PartialEq,
    B: PartialEq,
    Self: ReifyRef<F_, A, B>,
    <Self as ReifyRef<F_, A, B>>::Out: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.reify_as_ref() == other.reify_as_ref()
    }
}

impl<'f_, F_, A, B> Clone for Kind<'f_, F_, A, B>
where
    F_: HKT,
    A: Clone,
    B: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Kind::Vec(ref v) => Kind::Vec(v.clone()),
            Kind::Option(ref o) => Kind::Option(o.clone()),
            Kind::Id(ref id) => Kind::Id(id.clone()),
            Kind::Future(ref future) => unimplemented!(),
            Kind::Result(ref res) => Kind::Result(res.clone()),
            Kind::Any(ref any) => unimplemented!(),
            Kind::__MARKER(ref data) => Kind::__MARKER(data.clone()),
        }
    }
}

impl<'f_, F_, A, B> IntoKind<'f_, F_, A, B> for Kind<'f_, F_, A, B>
where
    F_: HKT,
{
    type Kind = F_;

    fn into_kind(self) -> Kind<'f_, F_, A, B> {
        self
    }
}
