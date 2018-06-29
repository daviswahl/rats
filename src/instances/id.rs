use id::{Id,IdExt};
use kinds::IdKind;
use kind::{Kind,IntoKind, Reify};
use functor::Functor;
use applicative::Applicative;

impl Applicative<IdKind> for IdKind {
    fn point<A>(a: A) -> Kind<IdKind, A> {
        Id(a).into_kind()
    }
}

impl Functor<IdKind> for IdKind {
    fn map<F,A,B>(a: Kind<IdKind, A>, f: F) -> Kind<IdKind, B>
    where F: Fn(A) -> B {
        f(a.reify().take()).id().into_kind()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use applicative::ApplicativeExt;
    use functor::KindFunctorExt;
    #[test]
    fn test_applicative() {
        let r = 5_i32.point::<IdKind>();
        assert_eq!(r, Kind::Id::<IdKind, i32>(Id(5)))
    }

    #[test]
    fn test_functor() {
        let r = 5.point::<IdKind>();
        let r = r.map(|i| i * 2);
        assert_eq!(r.reify(), Id(10))
    }
}