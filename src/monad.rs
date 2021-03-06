use applicative::Applicative;
use identity;
use lifted::*;

pub trait Monad<'a, F, Z = Nothing, G = Nothing>: Applicative<'a, F, Z, G>
where
    F: Applicative<'a, F, Z, G> + 'a,
{
    /// (F<A>, Fn) -> F<B>
    /// where
    /// Fn: Fn(A) -> F<B>
    fn flat_map<A, B, Func>(fa: Lifted<'a, F, A, Z, G>, func: Func) -> Lifted<'a, F, B, Z, G>
    where
        Func: Fn(A) -> Lifted<'a, F, B, Z, G> + 'a;

    /// (F<A>, Fn) -> F<B>
    /// where
    /// Fn: Fn(A) -> F<B>
    fn map<A, B, Func>(fa: Lifted<'a, F, A, Z, G>, func: Func) -> Lifted<'a, F, B, Z, G>
    where
        A: 'a,
        B: 'a,
        Func: Fn(A) -> B + 'a,
    {
        Self::flat_map(fa, move |a| F::point(func(a)))
    }

    /// (F<F<A>> -> F<A>
    fn flatten<A>(fa: Lifted<'a, F, Lifted<'a, F, A, Z, G>, Z, G>) -> Lifted<'a, F, A, Z, G> {
        Self::flat_map(fa, &identity)
    }
}
