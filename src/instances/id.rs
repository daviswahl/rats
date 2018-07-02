use applicative::Applicative;
use data::id::{Id, IdExt};
use functor::Functor;
use kind::{IntoKind, Kind, Reify};
use kinds::IdKind;

impl <'f_> Applicative<'f_, IdKind> for IdKind {
    fn point<A>(a: A) -> Kind<'f_, IdKind, A> {
        Id(a).into_kind()
    }

    fn ap<A: 'f_, B: 'f_, FnAb>(
        fa: Kind<'f_, IdKind, A>,
        ff: Kind<'f_, IdKind, FnAb>,
    ) -> Kind<'f_, IdKind, B>
    where
        FnAb: FnOnce(A) -> B,
    {
        let fa = fa.reify().take();
        let ff = ff.reify().take();
        ff(fa).id().into_kind()
    }
}

impl <'f_> Functor<'f_, IdKind> for IdKind {
    fn map<F, A, B>(a: Kind<'f_, IdKind, A>, f: F) -> Kind<'f_, IdKind, B>
    where
        F: Fn(A) -> B + 'f_,
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
