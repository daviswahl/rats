use kind::{HKT,Kind};
pub trait Applicative<K: HKT> {
    fn point<T>(value: T) -> Kind<K,T>;
}

pub trait ApplicativeExt {
    type Out;
    fn point<F>(self) -> Kind<F, Self::Out> where F: HKT+Applicative<F>;
}

impl<T> ApplicativeExt for T {
    type Out = T;
    fn point<F: HKT+Applicative<F>>(self) -> Kind<F, T> {
        F::point::<T>(self)
    }
}
