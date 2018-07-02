use applicative::Applicative;
use data::id::{Id, IdExt};
use functor::Functor;
use kind::{IntoKind, Kind, Reify};
use kinds::IdKind;

impl Applicative<IdKind> for IdKind {
    fn point<'kind, A>(a: A) -> Kind<'kind, IdKind, A> {
        Id(a).into_kind()
    }

    fn ap<'kind, A: 'kind, B: 'kind, F>(
        fa: Kind<'kind, IdKind, A>,
        ff: Kind<'kind, IdKind, F>,
    ) -> Kind<'kind, IdKind, B>
    where
        F: FnOnce(A) -> B,
    {
        let fa = fa.reify().take();
        let ff = ff.reify().take();
        ff(fa).id().into_kind()
    }
}

impl Functor<IdKind> for IdKind {
    fn map<'kind, F, A, B>(a: Kind<'kind, IdKind, A>, f: F) -> Kind<'kind, IdKind, B>
    where
        F: Fn(A) -> B + 'kind,
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
