use applicative::Applicative;
use functor::Functor;
use kind::{Kind, HKT};

pub trait Traverse<F_: HKT>: Functor<F_> {
    fn traverse<'f_, FnAGb, G_, A, B>(
        fa: Kind<'f_, F_, A>,
        f: FnAGb,
    ) -> Kind<'f_, G_, Kind<'f_, F_, B>>
    where
        G_: Applicative<G_>,
        FnAGb: Fn(A) -> Kind<'f_, G_, B>;
}

pub trait TraverseExt<'f_, F_: Traverse<F_>> {
    type A;
    fn traverse<FnAGb, G_, B>(self, f: FnAGb) -> Kind<'f_, G_, Kind<'f_, F_, B>>
    where
        G_: Applicative<G_>,
        FnAGb: Fn(Self::A) -> Kind<'f_, G_, B>;
}

impl<'f_, F_, A> TraverseExt<'f_, F_> for Kind<'f_, F_, A>
where
    F_: Traverse<F_>,
{
    type A = A;
    fn traverse<F, G_, B>(self, f: F) -> Kind<'f_, G_, Kind<'f_, F_, B>>
    where
        G_: Applicative<G_>,
        F: Fn(Self::A) -> Kind<'f_, G_, B>,
    {
        F_::traverse(self, f)
    }
}
