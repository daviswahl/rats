use functor::Functor;
use kind::Kind;
pub trait Applicative<F_>: Functor<F_>
where
    F_: Functor<F_>,
{
    fn ap<'f_, A: 'f_, B: 'f_, Fn_>(
        fa: Kind<'f_, F_, A>,
        ffn: Kind<'f_, F_, Fn_>,
    ) -> Kind<'f_, F_, B>
    where
        Fn_: FnOnce(A) -> B;
    fn point<'f_, A: 'f_>(value: A) -> Kind<'f_, F_, A>;

    fn product<'f_, A: 'f_, B: 'f_>(
        fa: Kind<'f_, F_, A>,
        fb: Kind<'f_, F_, B>,
    ) -> Kind<'f_, F_, (A, B)> {
        let fab = |a| |b| (a, b);
        let fab = Self::map(fa, fab);
        Self::ap(fb, fab)
    }

    fn map2<'f_, Fn_, A, B, Z>(
        fa: Kind<'f_, F_, A>,
        fb: Kind<'f_, F_, B>,
        fn_: Fn_,
    ) -> Kind<'f_, F_, Z>
    where
        Fn_: Fn((A, B)) -> Z + 'f_,
    {
        F_::map(Self::product(fa, fb), fn_)
    }
}

pub trait ApplicativeKindExt<'f_, F_: Applicative<F_>> {
    type A;
    fn product<B>(self, fb: Kind<'f_, F_, B>) -> Kind<'f_, F_, (Self::A, B)>;

    fn ap<B, Fn_>(self, ffn: Kind<'f_, F_, Fn_>) -> Kind<'f_, F_, B>
    where
        Fn_: FnOnce(Self::A) -> B;
}

impl<'f_, F_, A> ApplicativeKindExt<'f_, F_> for Kind<'f_, F_, A>
where
    F_: Applicative<F_>,
{
    type A = A;
    fn product<B>(self, fb: Kind<'f_, F_, B>) -> Kind<'f_, F_, (Self::A, B)> {
        F_::product(self, fb)
    }

    fn ap<B, Fn_>(self, ffn: Kind<'f_, F_, Fn_>) -> Kind<'f_, F_, B>
    where
        Fn_: FnOnce(Self::A) -> B,
    {
        F_::ap(self, ffn)
    }
}

pub trait Point<'f_> {
    type Out;
    fn point<F_>(self) -> Kind<'f_, F_, Self::Out>
    where
        F_: Applicative<F_>;
}

impl<'f_, A: 'f_> Point<'f_> for A {
    type Out = A;
    fn point<F_: Applicative<F_>>(self) -> Kind<'f_, F_, A> {
        F_::point::<A>(self)
    }
}
