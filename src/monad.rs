use applicative::Applicative;
use kind::{Kind, Empty};
use identity;

pub trait Monad<'f_, F_, Z = Empty>: Applicative<'f_, F_>
where
    F_: Applicative<'f_, F_>,
{
    /// (F<A>, Fn) -> F<B>
    /// where
    /// Fn: Fn(A) -> F<B>
    fn flat_map<A, B, Fn_>(fa: Kind<'f_, F_, A>, fn_: Fn_) -> Kind<'f_, F_, B>
    where
        A: 'f_,
        B: 'f_,
        Fn_: Fn(A) -> Kind<'f_, F_, B>;

    /// (F<A>, Fn) -> F<B>
    /// where
    /// Fn: Fn(A) -> F<B>
    fn map<A, B, Fn_>(fa: Kind<'f_, F_, A>, fn_: Fn_) -> Kind<'f_, F_, B>
    where
        A: 'f_,
        B: 'f_,
        Fn_: Fn(A) -> B {
        Self::flat_map(fa, |a| F_::point(fn_(a)))
    }

    /// (F<F<A>> -> F<A>
    fn flatten<A>(fa: Kind<'f_, F_, Kind<'f_, F_, A>>) -> Kind<'f_, F_, A> {
       Self::flat_map(fa, identity)
    }
}

