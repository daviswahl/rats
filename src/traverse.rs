use applicative::Applicative;
use functor::Functor;
use kind::Kind;

pub trait Traverse<F_>: Functor<F_> where F_: Functor<F_> {
    /// (F<A>, Fn(A) -> G<B>) -> G<F<B>>
    /// where G: Applicative, F: Functor
    fn traverse<'f_, Fn_, G_, A, B>(
        fa: Kind<'f_, F_, A>,
        fn_: Fn_,
    ) -> Kind<'f_, G_, Kind<'f_, F_, B>>
    where
        G_: Applicative<G_>,
        Fn_: Fn(A) -> Kind<'f_, G_, B>;
}

pub trait TraverseExt<'f_, F_: Traverse<F_>> {
    type A;
    fn traverse<Fn_, G_, B>(self, fn_: Fn_) -> Kind<'f_, G_, Kind<'f_, F_, B>>
    where
        G_: Applicative<G_>,
        Fn_: Fn(Self::A) -> Kind<'f_, G_, B>;
}

impl<'f_, F_, A> TraverseExt<'f_, F_> for Kind<'f_, F_, A>
where
    F_: Traverse<F_>,
{
    type A = A;
    fn traverse<Fn_, G_, B>(self, f: Fn_) -> Kind<'f_, G_, Kind<'f_, F_, B>>
    where
        G_: Applicative<G_>,
        Fn_: Fn(Self::A) -> Kind<'f_, G_, B>,
    {
        F_::traverse(self, f)
    }
}
