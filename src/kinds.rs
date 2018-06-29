use kind::{HKT, IntoKind, Reify, Kind};
use id::Id;

#[derive(Debug, PartialEq)]
pub struct VecKind;
impl HKT for VecKind {}

#[derive(Debug, PartialEq)]
pub struct OptionKind;
impl HKT for OptionKind {}

#[derive(Debug, PartialEq)]
pub struct IdKind;
impl HKT for IdKind {}


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
    }
}