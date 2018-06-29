use applicative::Applicative;
use functor::Functor;
use id::{Id, IdExt};
use kind::{IntoKind, Kind, Reify};
use kinds::IdKind;

impl Applicative<IdKind> for IdKind {
    fn point<A>(a: A) -> Kind<IdKind, A> {
        Id(a).into_kind()
    }

    fn ap<A, B, F>(fa: Kind<IdKind, A>, ff: Kind<IdKind, F>) -> Kind<IdKind, B>
    where
        F: FnOnce(A) -> B,
    {
        unimplemented!()
    }
}

impl Functor<IdKind> for IdKind {
    fn map<F, A, B>(a: Kind<IdKind, A>, f: F) -> Kind<IdKind, B>
    where
        F: Fn(A) -> B,
    {
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
