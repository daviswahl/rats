use applicative::Applicative;
use kind::{Kind, Empty};
use identity;

pub trait Monad<'f_, F_, Z = Empty>: Applicative<'f_, F_, Z>
where
    F_: Applicative<'f_, F_, Z>,
{
    /// (F<A>, Fn) -> F<B>
    /// where
    /// Fn: Fn(A) -> F<B>
    fn flat_map<A, B, Fn_>(fa: Kind<'f_, F_, A, Z>, fn_: Fn_) -> Kind<'f_, F_, B, Z>
    where
        A: 'f_,
        B: 'f_,
        Fn_: Fn(A) -> Kind<'f_, F_, B, Z>;

    /// (F<A>, Fn) -> F<B>
    /// where
    /// Fn: Fn(A) -> F<B>
    fn map<A, B, Fn_>(fa: Kind<'f_, F_, A, Z>, fn_: Fn_) -> Kind<'f_, F_, B, Z>
    where
        A: 'f_,
        B: 'f_,
        Fn_: Fn(A) -> B,
    {
        Self::flat_map(fa, |a| F_::point(fn_(a)))
    }

    /// (F<F<A>> -> F<A>
    fn flatten<A>(fa: Kind<'f_, F_, Kind<'f_, F_, A, Z>, Z>) -> Kind<'f_, F_, A, Z> {
        Self::flat_map(fa, identity)
    }
}


pub trait MonadExt<'f_, F_, Z>
where
    F_: Monad<'f_, F_, Z>,
{
    type A;
    type Z = Z;

    fn flat_map<B, Fn_>(self, f: Fn_) -> Kind<'f_, F_, B, Self::Z>
    where
        Fn_: Fn(Self::A) -> Kind<'f_, F_, B, Self::Z> + 'f_;

    // TODO: may not be able to implement this in the same trait?
    // Not sure how to represent that self is Kind<'f_, F_, Kind<...
    // fn flatten<A>(self) -> Kind<'f_, F_, B, Self::Z>;
}

impl<'f_, F_, A, Z> MonadExt<'f_, F_, Z> for Kind<'f_, F_, A, Z>
where
    F_: Monad<'f_, F_, Z>,
{
    type A = A;
    type Z = Z;

    fn flat_map<B, Fn_>(self, f: Fn_) -> Kind<'f_, F_, B, Self::Z>
    where
        Fn_: Fn(Self::A) -> Kind<'f_, F_, B, Self::Z> + 'f_,
    {
        F_::flat_map(self, f)
    }
}
