use applicative::Applicative;
use functor::Functor;
use kind::Kind;

pub trait Traverse<'f_, F_>: Functor<'f_, F_>
where
    F_: Functor<'f_, F_>,
{
    /// (F<A>, λ) -> G<F<B>>
    /// where
    /// G: Applicative
    /// λ: Fn(A) -> G<B>
    fn traverse<Fn_, G_, A, B>(
        fa: Kind<'f_, F_, A>,
        fn_: Fn_,
    ) -> Kind<'f_, G_, Kind<'f_, F_, B>>
    where
        G_: Applicative<'f_, G_>,
        Fn_: Fn(A) -> Kind<'f_, G_, B>;
}

pub trait TraverseExt<'f_, F_: Traverse<'f_, F_>> {
    type A;

    /// (Self<Self::A>, λ) -> G<Self<Self::A>>
    /// where
    /// Self: F
    /// G: Applicative
    /// λ: Fn(Self::A) -> G<B>
    fn traverse<Fn_, G_, B>(self, fn_: Fn_) -> Kind<'f_, G_, Kind<'f_, F_, B>>
    where
        G_: Applicative<'f_, G_>,
        Fn_: Fn(Self::A) -> Kind<'f_, G_, B>;
}

impl<'f_, F_, A> TraverseExt<'f_, F_> for Kind<'f_, F_, A>
where
    F_: Traverse<'f_, F_>,
{
    /// (Self<A>, λ) -> G<Self<A>>
    /// where
    /// Self: F
    /// G: Applicative,
    /// λ: Fn(A) -> G<B>
    type A = A;
    fn traverse<Fn_, G_, B>(self, f: Fn_) -> Kind<'f_, G_, Kind<'f_, F_, B>>
    where
        G_: Applicative<'f_, G_>,
        Fn_: Fn(Self::A) -> Kind<'f_, G_, B>,
    {
        F_::traverse(self, f)
    }
}
