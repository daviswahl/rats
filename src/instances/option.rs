use applicative::Applicative;
use functor::Functor;
use kind::{IntoKind, Kind, Reify};
use kinds::OptionKind;

impl Functor<OptionKind> for OptionKind {
    /// (Option<A>, Fn(A) -> B) -> Option<B>
    fn map<'f_, Fn_, A, B>(a: Kind<'f_, OptionKind, A>, f: Fn_) -> Kind<'f_, OptionKind, B>
    where
        Fn_: FnOnce(A) -> B + 'f_,
    {
        a.reify().map(f).into_kind()
    }
}

type OptionK<'f_, A> = Kind<'f_, OptionKind, A>;
impl Applicative<OptionKind> for OptionKind {
    fn ap<'f_, A, B, Fn_>(fa: OptionK<A>, ff: OptionK<Fn_>) -> OptionK<'f_, B>
    where
        Fn_: FnOnce(A) -> B,
    {
        let fa = fa.reify();
        let ff = ff.reify();
        fa.and_then(|fa| ff.map(|ff| ff(fa))).into_kind()
    }

    fn point<'f_, A>(a: A) -> Kind<'f_, OptionKind, A> {
        Some(a).into_kind()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use applicative::{ApplicativeKindExt, Point};
    #[test]
    fn test_option_pure() {
        let f = 5.point::<OptionKind>();
        assert_eq!(Some(5), f.reify());
    }

    fn show_off_kind_tupler<'f_, F_, A, B>(
        a: Kind<'f_, F_, A>,
        b: Kind<'f_, F_, B>,
    ) -> Kind<'f_, F_, (A, B)>
    where
        F_: Applicative<F_>,
    {
        a.product(b)
    }

    #[test]
    fn test_kind_tupler() {
        // type annotations are not necessary here, they're just for the reader
        let a = 5.point::<OptionKind>();
        let b = "rats".point::<OptionKind>();
        let ab = show_off_kind_tupler(a, b).reify();
        assert_eq!(ab, Some((5, "rats")));

        use kinds::IdKind;
        let a = 5.point::<IdKind>();
        let b = "rats".point::<IdKind>();
        let k = show_off_kind_tupler(a, b).reify().take();
        assert_eq!(k, (5, "rats"))
    }

    #[test]
    fn test_option_product() {
        let a = 5.point::<OptionKind>();
        let b = "hello".point::<OptionKind>();
        let result = a.product(b);
        assert_eq!(result.reify(), Some((5, "hello")));
    }
}
