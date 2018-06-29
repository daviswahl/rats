use std::marker::PhantomData;

pub trait HKT {}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Kind<K: HKT, A> {
    Vec(Vec<A>),
    Option(Option<A>),
    // Is this valid? also need to understand which pointer type to use here
    __MARKER(PhantomData<*const K>),
}

pub trait Reify<K: HKT, A> {
    type Out;
    fn reify(self) -> Self::Out;
}

pub trait IntoKind<K: HKT, T> {
    type Kind: HKT;
    fn into_kind(self) -> Kind<K, T>;
}

//macro_rules! derive_hkt {
//    ($t:ident) => {
//        impl HKT for $tK {
//            fn marker() -> $tK {
//                $tK
//            }
//        }
//
//        impl<T> Kinded<$tK, T> for $t<T> {}
//
//        impl<T> Reify<$tK, T> for ::kind::Kind<$tK, T> {
//            type Out = $t<T>;
//        }
//    }
//}

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

#[derive(Debug, PartialEq)]
pub struct VecKind;
impl HKT for VecKind {}

#[derive(Debug, PartialEq)]
pub struct OptionKind;
impl HKT for OptionKind {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tests() {
        let r = vec![1, 2, 3].into_kind();
        assert_eq!(vec![1, 2, 3], r.reify());

        let r = Some(1).into_kind();
        assert_eq!(Some(1), r.reify());
    }
}
