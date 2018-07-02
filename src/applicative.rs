use functor::Functor;
use kind::{Kind, HKT};
pub trait Applicative<F_: HKT>: Functor<F_> {
    fn ap<'f_, A: 'f_, B: 'f_, FnAb>(
        fa: Kind<'f_, F_, A>,
        ff: Kind<'f_, F_, FnAb>,
    ) -> Kind<'f_, F_, B>
    where
        FnAb: FnOnce(A) -> B;
    fn point<'f_, A: 'f_>(value: A) -> Kind<'f_, F_, A>;

    fn product<'f_, A: 'f_, B: 'f_>(
        fa: Kind<'f_, F_, A>,
        fb: Kind<'f_, F_, B>,
    ) -> Kind<'f_, F_, (A, B)> {
        let fab = |a| |b| (a, b);
        let fab = Self::map(fa, fab);
        Self::ap(fb, fab)
    }

    fn map2<'f_, FnAbz, A, B, Z>(
        fa: Kind<'f_, F_, A>,
        fb: Kind<'f_, F_, B>,
        fn_abz: FnAbz,
    ) -> Kind<'f_, F_, Z>
    where
        FnAbz: Fn((A, B)) -> Z + 'f_,
    {
        Self::map(Self::product(fa, fb), fn_abz)
    }
}

pub trait ApplicativeKindExt<'_f, F_: Applicative<F_>> {
    type Item;
    fn product<B>(self, fb: Kind<'_f, F_, B>) -> Kind<'_f, F_, (Self::Item, B)>;

    fn ap<B, FnAb>(self, ff: Kind<'_f, F_, FnAb>) -> Kind<'_f, F_, B>
    where
        FnAb: FnOnce(Self::Item) -> B;
}

impl<'f_, F_, A> ApplicativeKindExt<'f_, F_> for Kind<'f_, F_, A>
where
    F_: Applicative<F_>,
{
    type Item = A;
    fn product<B>(self, fb: Kind<'f_, F_, B>) -> Kind<'f_, F_, (Self::Item, B)> {
        F_::product(self, fb)
    }

    fn ap<B, FnAb>(self, ff: Kind<'f_, F_, FnAb>) -> Kind<'f_, F_, B>
    where
        FnAb: FnOnce(Self::Item) -> B,
    {
        F_::ap(self, ff)
    }
}

pub trait Point<'f_> {
    type Out;
    fn point<F_>(self) -> Kind<'f_, F_, Self::Out>
    where
        F_: HKT + Applicative<F_>;
}

impl<'f_, A: 'f_> Point<'f_> for A {
    type Out = A;
    fn point<F_: HKT + Applicative<F_>>(self) -> Kind<'f_, F_, A> {
        F_::point::<A>(self)
    }
}
