use applicative::Applicative;
use data::id::{Id, IdExt};
use functor::Functor;
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
        let fa = fa.reify().take();
        let ff = ff.reify().take();
        ff(fa).id().into_kind()
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
    use applicative::{ApplicativeKindExt, Point};
    use functor::KindFunctorExt;
    #[test]
    fn test_applicative() {
        let r = 5.point::<IdKind>();
        assert_eq!(r, Kind::Id::<IdKind, i32>(Id(5)));

        assert_eq!(r.ap(Id(|i| i * 2).into_kind()), 10.id().into_kind())
    }

    #[test]
    fn test_functor() {
        let r = 5.point::<IdKind>();
        let r = r.map(|i| i * 2);
        assert_eq!(r.reify(), Id(10))
    }
}
