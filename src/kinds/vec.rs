use hkt::*;
use kind::Kind;

pub struct VecK;

impl HKT for VecK {
    fn marker() -> VecK {
        VecK
    }
}

impl<T> Kinded<VecK, T> for Vec<T> {}

impl<T> Reify<VecK, T> for Kind<VecK, T> {
    type Out = Vec<T>;
}
