pub mod vec;
use functor::{Functor,KindFunctorExt};
use kind::{OptionKind,Kind, Reify, IntoKind};

impl Functor<OptionKind> for OptionKind {
    fn map<F,A,B>(a: Kind<OptionKind, A>, f: F) -> Kind<OptionKind, B>
    where
    F: FnOnce(A) -> B {
        a.reify().map(f).into_kind()
    }
}