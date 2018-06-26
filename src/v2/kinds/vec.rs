use v2::hkt::*;
use v2::conversions::*;

pub struct VecK;

impl HKT for VecK {
    fn marker() -> VecK {
        VecK
    }
}

impl<T> IntoKinded<VecK, T> for Kind<VecK, T> {
    type Out = Vec<T>;
    fn into_kinded(self) -> Self::Out {
        <Self::Out as FromKind<VecK, T>>::from_kind(self)
    }
}

impl<T> FromKind<VecK, T> for Vec<T> {
    type Out = Vec<T>;
    fn from_kind(k: Kind<VecK, T>) -> Vec<T> {
        unsafe { k.unwrap::<Self>() }
    }
}

impl<T> IntoKind<VecK, T> for Vec<T> {
    fn into_kind(self) -> Kind<VecK, T> {
        Kind::new(self)
    }
}

impl<T> Kinded<VecK, T> for Vec<T> {}

