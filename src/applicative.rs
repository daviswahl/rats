use functor::Functor;
use lifted::*;

pub trait Applicative<'a, F, Z = Nothing, G = Nothing>: Functor<'a, F, Z, G>
where
    F: Functor<'a, F, Z, G> + 'a,
{
    fn ap<A, B, Func>(
        ff: Lifted<'a, F, Func, Z, G>,
        fa: Lifted<'a, F, A, Z, G>,
    ) -> Lifted<'a, F, B, Z, G>
    where
        Func: FnOnce(A) -> B + 'a;

    fn point<A>(a: A) -> Lifted<'a, F, A, Z, G>;

    fn map<A, B, Func>(fa: Lifted<'a, F, A, Z, G>, func: Func) -> Lifted<'a, F, B, Z, G>
    where
        Func: Fn(A) -> B + 'a,
    {
        Self::ap(Self::point(func), fa)
    }

    /// (F<A>, F<B>) -> F<(A,B)>
    fn product<A, B>(
        fa: Lifted<'a, F, A, Z, G>,
        fb: Lifted<'a, F, B, Z, G>,
    ) -> Lifted<'a, F, (A, B), Z, G> {
        let fab = |a| |b| (a, b);
        let fab = F::map(fa, fab);
        Self::ap(fab, fb)
    }

    /// (F<A>, F<B>, Fn((A,B)) -> C) -> F<C>
    fn map2<Func, A, B, C>(
        fa: Lifted<'a, F, A, Z, G>,
        fb: Lifted<'a, F, B, Z, G>,
        fn_: Func,
    ) -> Lifted<'a, F, C, Z, G>
    where
        Func: Fn((A, B)) -> C + 'a,
    {
        F::map(Self::product(fa, fb), fn_)
    }
}
