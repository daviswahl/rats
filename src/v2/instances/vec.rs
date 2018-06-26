use v2::conversions::*;
use v2::functor::Functor;
use v2::hkt::{Kind, Unkind};
use v2::kinds::vec::VecK;

impl Functor<VecK> for VecK {
    fn map<F, A, B>(k: Kind<VecK, A>, f: F) -> Kind<VecK, B>
    where
        F: FnMut(A) -> B,
    {
        unsafe {
            k.unkind().into_iter().map(f).collect::<Vec<B>>().into_kind()
        }
    }
}
