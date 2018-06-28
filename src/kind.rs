use hkt::*;
use std::marker::PhantomData;

#[allow(dead_code)]
pub enum Kind<K: HKT, A> {
    Vec(Vec<A>),
    Option(Option<A>),
    __MARKER(PhantomData<*const K>),
}

pub trait Reify<K: HKT, A> {
    type Type;
    fn reify(self) -> Self::Type;
    fn new(t: Self::Type) -> Kind<K, A>;
}

pub trait Kinded<K: HKT, T> {
    type Kind: HKT;
    type Out;
    fn into_kind(self) -> Kind<K, T>;
}

impl<T> Kinded<VecKind, T> for Vec<T> {
    type Kind = VecKind;
    type Out = Vec<T>;

    fn into_kind(self) -> Kind<VecKind, T> {
        Kind::new(self)
    }
}

#[allow(unreachable_patterns)]
impl<T> Reify<VecKind, T> for Kind<VecKind, T> {
    type Type = Vec<T>;
    fn reify(self) -> Vec<T> {
        match self {
            Kind::Vec(t) => t,
            _ => unreachable!(),
        }
    }

    fn new(t: Vec<T>) -> Kind<VecKind, T> {
        Kind::Vec::<VecKind, T>(t)
    }
}

#[allow(unreachable_patterns)]
impl<T> Reify<OptionKind, T> for Kind<OptionKind, T> {
    type Type = Option<T>;

    fn reify(self) -> Self::Type {
        match self {
            Kind::Option(t) => t,
            _ => unreachable!(),
        }
    }

    fn new(t: Self::Type) -> Kind<OptionKind, T> {
        Kind::Option(t)
    }
}

pub struct VecKind;
impl HKT for VecKind {}

pub struct OptionKind;
impl HKT for OptionKind {}

#[cfg(tests)]
mod tests {
    #[test]
    fn tests() {
        let r = vec![1, 2, 3].into_kind();
        assert_eq!(vec![1, 2, 3], r.extract());

        let r = Some(1).into_kind();
        assert_eq!(Some(1), r.reify());

        let r = Ok("yes").into_kind();
        assert_eq!(Ok("yes"), r.reify())
    }
}
