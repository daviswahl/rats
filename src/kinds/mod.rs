use data::id::Id;
use futures::future::Future;
use kind::{IntoKind, Kind, Reify, ReifyRef, HKT};
use std::fmt;
use std::fmt::{Debug, Formatter};

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

impl<'f_, A: 'f_> IntoKind<'f_, VecKind, A> for Vec<A> {
    type Kind = VecKind;
    fn into_kind(self) -> Kind<'f_, VecKind, A> {
        Kind::Vec::<VecKind, A>(self)
    }
}

impl<'f_, A: 'f_> IntoKind<'f_, OptionKind, A> for Option<A> {
    type Kind = OptionKind;
    fn into_kind(self) -> Kind<'f_, OptionKind, A> {
        Kind::Option::<OptionKind, A>(self)
    }
}

impl<'f_, A: 'f_> IntoKind<'f_, IdKind, A> for Id<A> {
    type Kind = IdKind;
    fn into_kind(self) -> Kind<'f_, IdKind, A> {
        Kind::Id::<IdKind, A>(self)
    }
}

impl<'f_, A: 'f_, B: 'f_> IntoKind<'f_, ResultKind, A, B> for Result<A, B> {
    type Kind = ResultKind;
    fn into_kind(self) -> Kind<'f_, ResultKind, A, B> {
        Kind::Result::<ResultKind, A, B>(self)
    }
}

impl<'f_, A: 'f_, B: 'f_, F_: 'f_> IntoKind<'f_, FutureKind, A, B> for F_
    where
        F_: Future<Item = A, Error = B>,
{
    default type Kind = FutureKind;
    default fn into_kind(self) -> Kind<'f_, FutureKind, A, B> {
        Kind::Future::<FutureKind, A, B>(Box::new(self))
    }
}

impl<'f_, A: 'f_, B: 'f_>  IntoKind<'f_, FutureKind, A, B> for Box<Future<Item=A, Error=B>>
    where
{
    type Kind = FutureKind;
    fn into_kind(self) -> Kind<'f_, FutureKind, A, B> {
        Kind::Future::<FutureKind, A, B>(self)
    }
}

#[allow(unreachable_patterns)]
impl<'f_, A> Reify<VecKind, A> for Kind<'f_, VecKind, A> {
    type Out = Vec<A>;
    fn reify(self) -> Vec<A> {
        match self {
            Kind::Vec(t) => t,
            _ => unreachable!(),
        }
    }
}

#[allow(unreachable_patterns)]
impl<'f_, A> ReifyRef<VecKind, A> for Kind<'f_, VecKind, A> {
    type Out = Vec<A>;
    fn reify_as_ref(&self) -> &Vec<A> {
        match *self {
            Kind::Vec(ref t) => t,
            _ => unreachable!(),
        }
    }
}

#[allow(unreachable_patterns)]
impl<'f_, A> Reify<OptionKind, A> for Kind<'f_, OptionKind, A> {
    type Out = Option<A>;

    fn reify(self) -> Self::Out {
        match self {
            Kind::Option(t) => t,
            _ => unreachable!(),
        }
    }
}

#[allow(unreachable_patterns)]
impl<'f_, A> ReifyRef<OptionKind, A> for Kind<'f_, OptionKind, A> {
    type Out = Option<A>;

    fn reify_as_ref(&self) -> &Self::Out {
        match *self {
            Kind::Option(ref t) => t,
            _ => unreachable!(),
        }
    }
}

#[allow(unreachable_patterns)]
impl<'f_, A> Reify<IdKind, A> for Kind<'f_, IdKind, A> {
    type Out = Id<A>;
    fn reify(self) -> Id<A> {
        match self {
            Kind::Id(t) => t,
            _ => unreachable!(),
        }
    }
}

#[allow(unreachable_patterns)]
impl<'f_, A> ReifyRef<IdKind, A> for Kind<'f_, IdKind, A> {
    type Out = Id<A>;
    fn reify_as_ref(&self) -> &Id<A> {
        match *self {
            Kind::Id(ref t) => t,
            _ => unreachable!(),
        }
    }
}

#[allow(unreachable_patterns)]
impl<'f_, A, B> Reify<ResultKind, A, B> for Kind<'f_, ResultKind, A, B> {
    type Out = Result<A, B>;
    fn reify(self) -> Result<A, B> {
        match self {
            Kind::Result(t) => t,
            _ => unreachable!(),
        }
    }
}

#[allow(unreachable_patterns)]
impl<'f_, A: 'f_, B: 'f_> Reify<FutureKind, A, B> for Kind<'f_, FutureKind, A, B> {
    type Out = Box<Future<Item = A, Error = B> + 'f_>;
    fn reify(self) -> Box<Future<Item = A, Error = B> + 'f_> {
        match self {
            Kind::Future(t) => t,
            _ => unreachable!(),
        }
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
