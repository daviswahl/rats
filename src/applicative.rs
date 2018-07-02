use functor::Functor;
use kind::Kind;
use kind::Empty;
pub trait Applicative<F_, Z = Empty>: Functor<F_, Z>
where
    F_: Functor<F_, Z>,
{
    /// (F<A>, F<FnOnce(A) -> B>) -> F<B>
    fn ap<'f_, A: 'f_, B: 'f_, Fn_>(
        fa: Kind<'f_, F_, A, Z>,
        ffn: Kind<'f_, F_, Fn_, Z>,
    ) -> Kind<'f_, F_, B, Z>
    where
        Fn_: FnOnce(A) -> B;

    /// A -> F<A>
    fn point<'f_, A: 'f_>(a: A) -> Kind<'f_, F_, A, Z>;

    /// (F<A>, F<B>) -> F<(A,B)>
    fn product<'f_, A: 'f_, B: 'f_>(
        fa: Kind<'f_, F_, A, Z>,
        fb: Kind<'f_, F_, B, Z>,
    ) -> Kind<'f_, F_, (A, B), Z> {
        let fab = |a| |b| (a, b);
        let fab = F_::map(fa, fab);
        Self::ap(fb, fab)
    }

    /// (F<A>, F<B>, Fn((A,B)) -> C) -> F<C>
    fn map2<'f_, Fn_, A, B, C>(
        fa: Kind<'f_, F_, A, Z>,
        fb: Kind<'f_, F_, B, Z>,
        fn_: Fn_,
    ) -> Kind<'f_, F_, C, Z>
    where
        Fn_: Fn((A, B)) -> C + 'f_,
    {
        F_::map(Self::product(fa, fb), fn_)
    }
}

pub trait ApplicativeKindExt<'f_, F_, Z = Empty> where F_: Applicative<F_, Z> {
    type A;
    /// (Self, F<B>) -> F<(Self::A,B)>
    fn product<B>(self, fb: Kind<'f_, F_, B, Z>) -> Kind<'f_, F_, (Self::A, B), Z>;

    /// (Self, F<Fn(Self::A) -> B>) -> F<B>
    fn ap<B, Fn_>(self, ffn: Kind<'f_, F_, Fn_, Z>) -> Kind<'f_, F_, B, Z>
    where
        Fn_: FnOnce(Self::A) -> B;
}

impl<'f_, F_, A, Z> ApplicativeKindExt<'f_, F_, Z> for Kind<'f_, F_, A, Z>
where
    F_: Applicative<F_, Z>,
{
    type A = A;
    /// (Self<A>, F<B>) -> F<(A,B)> where Self: F_
    fn product<B>(self, fb: Kind<'f_, F_, B, Z>) -> Kind<'f_, F_, (Self::A, B), Z> {
        F_::product(self, fb)
    }

    /// (Self<A>, F<Fn(A) -> B>) -> F<B> where Self: F
    fn ap<B, Fn_>(self, ffn: Kind<'f_, F_, Fn_, Z>) -> Kind<'f_, F_, B, Z>
    where
        Fn_: FnOnce(Self::A) -> B,
    {
        F_::ap(self, ffn)
    }
}

pub trait Point<'f_> {
    type Out;
    /// Self -> F<Self> where F: Applicative
    fn point<F_>(self) -> Kind<'f_, F_, Self::Out>
    where
        F_: Applicative<F_>;
}

impl<'f_, A: 'f_> Point<'f_> for A {
    type Out = A;
    /// A -> F<B>
    fn point<F_: Applicative<F_>>(self) -> Kind<'f_, F_, A> {
        F_::point::<A>(self)
    }
}
