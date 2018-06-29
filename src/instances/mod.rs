pub mod vec;

use functor::Functor;
use kind::{OptionKind,Kind, Reify, IntoKind};
use applicative::{Applicative};

impl Functor<OptionKind> for OptionKind {
    fn map<F,A,B>(a: Kind<OptionKind, A>, f: F) -> Kind<OptionKind, B>
    where
    F: FnOnce(A) -> B {
        a.reify().map(f).into_kind()
    }
}

impl Applicative<OptionKind> for OptionKind {
    fn point<A>(a: A) -> Kind<OptionKind, A> {
        Some(a).into_kind()
    }
}


#[cfg(test)]
mod tests {

use kind::HKT;
    use super::*;
    use applicative::ApplicativeExt;
    #[test]
    fn test_option_pure() {
        let f = 5.point::<OptionKind>();
        assert_eq!(Some(5), f.reify());
    }

}
