use data::id::Id;
use kind::{IntoKind, Kind, Reify, ReifyRef, HKT};

#[derive(Clone, Debug, PartialEq)]
pub struct VecKind;
impl HKT for VecKind {}

#[derive(Clone, Debug, PartialEq)]
pub struct OptionKind;
impl HKT for OptionKind {}

#[derive(Clone, Debug, PartialEq)]
pub struct IdKind;
impl HKT for IdKind {}

#[derive(Clone, Debug, PartialEq)]
pub struct ResultKind;
impl HKT for ResultKind {}

#[derive(Clone, Debug, PartialEq)]
pub struct FutureKind;
impl HKT for FutureKind {}

#[derive(Clone, Debug, PartialEq)]
pub struct OptionTKind;
impl HKT for OptionTKind {}

impl<'kind, A: 'kind> IntoKind<'kind, VecKind, A> for Vec<A> {
    type Kind = VecKind;
    fn into_kind(self) -> Kind<'kind, VecKind, A> {
        Kind::Vec::<VecKind, A>(self)
    }
}

impl<'kind, A: 'kind> IntoKind<'kind, OptionKind, A> for Option<A> {
    type Kind = OptionKind;
    fn into_kind(self) -> Kind<'kind, OptionKind, A> {
        Kind::Option::<OptionKind, A>(self)
    }
}

impl<'kind, A: 'kind> IntoKind<'kind, IdKind, A> for Id<A> {
    type Kind = IdKind;
    fn into_kind(self) -> Kind<'kind, IdKind, A> {
        Kind::Id::<IdKind, A>(self)
    }
}

impl<'kind, A: 'kind, B: 'kind> IntoKind<'kind, ResultKind, A, B> for Result<A, B> {
    type Kind = ResultKind;
    fn into_kind(self) -> Kind<'kind, ResultKind, A, B> {
        Kind::Result::<ResultKind, A, B>(self)
    }
}

use futures::future::Future;
impl<'kind, A: 'kind, B: 'kind, F_> IntoKind<'kind, FutureKind, A, B> for F_
where
    F_: Future<
        Item = A,
        Error = B,
    >,
    F_: 'static,
{
    type Kind = FutureKind;
    fn into_kind(self) -> Kind<'kind, FutureKind, A, B> {
        Kind::Future::<FutureKind, A, B>(Box::new(self))
    }
}

#[allow(unreachable_patterns)]
impl<'kind, T> Reify<VecKind, T> for Kind<'kind, VecKind, T> {
    type Out = Vec<T>;
    fn reify(self) -> Vec<T> {
        match self {
            Kind::Vec(t) => t,
            _ => unreachable!(),
        }
    }
}

#[allow(unreachable_patterns)]
impl<'kind, T> ReifyRef<VecKind, T> for Kind<'kind, VecKind, T> {
    type Out = Vec<T>;
    fn reify_as_ref(&self) -> &Vec<T> {
        match *self {
            Kind::Vec(ref t) => t,
            _ => unreachable!(),
        }
    }
}

#[allow(unreachable_patterns)]
impl<'kind, T> Reify<OptionKind, T> for Kind<'kind, OptionKind, T> {
    type Out = Option<T>;

    fn reify(self) -> Self::Out {
        match self {
            Kind::Option(t) => t,
            _ => unreachable!(),
        }
    }
}

#[allow(unreachable_patterns)]
impl<'kind, T> ReifyRef<OptionKind, T> for Kind<'kind, OptionKind, T> {
    type Out = Option<T>;

    fn reify_as_ref(&self) -> &Self::Out {
        match *self {
            Kind::Option(ref t) => t,
            _ => unreachable!(),
        }
    }
}

#[allow(unreachable_patterns)]
impl<'kind, T> Reify<IdKind, T> for Kind<'kind, IdKind, T> {
    type Out = Id<T>;
    fn reify(self) -> Id<T> {
        match self {
            Kind::Id(t) => t,
            _ => unreachable!(),
        }
    }
}

#[allow(unreachable_patterns)]
impl<'kind, T> ReifyRef<IdKind, T> for Kind<'kind, IdKind, T> {
    type Out = Id<T>;
    fn reify_as_ref(&self) -> &Id<T> {
        match *self {
            Kind::Id(ref t) => t,
            _ => unreachable!(),
        }
    }
}

#[allow(unreachable_patterns)]
impl<'kind, A, B> Reify<ResultKind, A, B> for Kind<'kind, ResultKind, A, B> {
    type Out = Result<A, B>;
    fn reify(self) -> Result<A, B> {
        match self {
            Kind::Result(t) => t,
            _ => unreachable!(),
        }
    }
}

#[allow(unreachable_patterns)]
impl<'kind, A: 'kind, B: 'kind> Reify<FutureKind, A, B> for Kind<'kind, FutureKind, A, B> {
    type Out = Box<Future<Item = A, Error = B> + 'kind>;
    fn reify(self) -> Box<Future<Item = A, Error = B> + 'kind> {
        match self {
            Kind::Future(t) => t,
            _ => unreachable!(),
        }
    }
}
use std::fmt;
use std::fmt::{Debug, Formatter};
impl<'kind, K, A, B> Debug for Kind<'kind, K, A, B>
where
    K: HKT,
    A: Debug,
    B: Debug,
    Self: ReifyRef<K, A, B>,
    <Self as ReifyRef<K, A, B>>::Out: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Kind<'kind,{:?}>", self.reify_as_ref())
    }
}

impl<'kind, K, A, B> PartialEq for Kind<'kind, K, A, B>
where
    K: HKT,
    A: PartialEq,
    B: PartialEq,
    Self: ReifyRef<K, A, B>,
    <Self as ReifyRef<K, A, B>>::Out: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.reify_as_ref() == other.reify_as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tests() {
        let r = vec![1, 2, 3].into_kind();
        assert_eq!(vec![1, 2, 3], r.reify());

        let r = Some(1).into_kind();
        assert_eq!(Some(1), r.reify());

        let r = Id(1).into_kind();
        assert_eq!(Id(1), r.reify());

        let r = Ok::<i32, &str>(1).into_kind();
        assert_eq!(Ok(1), r.reify());

        let r = Err::<i32, &str>("woops").into_kind();
        assert_eq!(Err("woops"), r.reify())
    }
}
