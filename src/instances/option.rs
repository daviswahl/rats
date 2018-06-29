use applicative::Applicative;
use functor::Functor;
use kind::{IntoKind, Kind, Reify};
use kinds::OptionKind;

impl Functor<OptionKind> for OptionKind {
    fn map<F, A, B>(a: Kind<OptionKind, A>, f: F) -> Kind<OptionKind, B>
    where
        F: FnOnce(A) -> B,
    {
        a.reify().map(f).into_kind()
    }
}

type OptionK<A> = Kind<OptionKind, A>;
impl Applicative<OptionKind> for OptionKind {
    fn ap<A, B, F>(fa: OptionK<A>, ff: OptionK<F>) -> OptionK<B>
    where
        F: FnOnce(A) -> B,
    {
        let fa = fa.reify();
        let ff = ff.reify();
        fa.and_then(|fa| ff.map(|ff| ff(fa))).into_kind()
    }

    fn point<A>(a: A) -> Kind<OptionKind, A> {
        Some(a).into_kind()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use applicative::{ApplicativeKindExt, Point};
    use kind::HKT;
    #[test]
    fn test_option_pure() {
        let f = 5.point::<OptionKind>();
        assert_eq!(Some(5), f.reify());
    }

    fn show_off_kind_tupler<K, A, B>(a: Kind<K, A>, b: Kind<K, B>) -> Kind<K, (A, B)>
    where
        K: HKT + Applicative<K>,
    {
        a.product(b)
    }

    #[test]
    fn test_kind_tupler() {
        // type annotations are not necessary here, they're just for the reader
        let a: Kind<OptionKind, i32> = 5.point::<OptionKind>();
        let b: Kind<OptionKind, &str> = "rats".point::<OptionKind>();
        let k: Option<(i32, &str)> = show_off_kind_tupler(a, b).reify();
        assert_eq!(k, Some((5, "rats")));

        use kinds::IdKind;
        let a: Kind<IdKind, i32> = 5.point::<IdKind>();
        let b: Kind<IdKind, &str> = "rats".point::<IdKind>();
        let k: (i32, &str) = show_off_kind_tupler(a, b).reify().take();
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
