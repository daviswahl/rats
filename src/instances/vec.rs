use conversions::*;
use functor::Functor;
use hkt::{Kind, Unkind};
use kinds::vec::VecK;

impl Functor<VecK> for VecK {
    fn map<F, A, B>(k: Kind<VecK, A>, f: F) -> Kind<VecK, B>
    where
        F: FnMut(A) -> B,
    {
        k.unkind().into_iter().map(f).collect::<Vec<B>>().into_kind()
    }
}
