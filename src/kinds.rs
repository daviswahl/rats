use data::id::Id;
use kind::{IntoKind, Kind, Reify, HKT};

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

impl<T> IntoKind<VecKind, T> for Vec<T> {
    type Kind = VecKind;
    fn into_kind(self) -> Kind<VecKind, T> {
        Kind::Vec::<VecKind, T>(self)
    }
}

impl<T> IntoKind<OptionKind, T> for Option<T> {
    type Kind = OptionKind;
    fn into_kind(self) -> Kind<OptionKind, T> {
        Kind::Option::<OptionKind, T>(self)
    }
}

impl<T> IntoKind<IdKind, T> for Id<T> {
    type Kind = IdKind;
    fn into_kind(self) -> Kind<IdKind, T> {
        Kind::Id::<IdKind, T>(self)
    }
}

impl<A, B> IntoKind<ResultKind, A, B> for Result<A, B> {
    type Kind = ResultKind;
    fn into_kind(self) -> Kind<ResultKind, A, B> {
        Kind::Result::<ResultKind, A, B>(self)
    }
}

#[allow(unreachable_patterns)]
impl<T> Reify<VecKind, T> for Kind<VecKind, T> {
    type Out = Vec<T>;
    fn reify(self) -> Vec<T> {
        match self {
            Kind::Vec(t) => t,
            _ => unreachable!(),
        }
    }
}

#[allow(unreachable_patterns)]
impl<T> Reify<OptionKind, T> for Kind<OptionKind, T> {
    type Out = Option<T>;

    fn reify(self) -> Self::Out {
        match self {
            Kind::Option(t) => t,
            _ => unreachable!(),
        }
    }
}

#[allow(unreachable_patterns)]
impl<T> Reify<IdKind, T> for Kind<IdKind, T> {
    type Out = Id<T>;
    fn reify(self) -> Id<T> {
        match self {
            Kind::Id(t) => t,
            _ => unreachable!(),
        }
    }
}

#[allow(unreachable_patterns)]
impl<A, B> Reify<ResultKind, A, B> for Kind<ResultKind, A, B> {
    type Out = Result<A, B>;
    fn reify(self) -> Result<A, B> {
        match self {
            Kind::Result(t) => t,
            _ => unreachable!(),
        }
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
