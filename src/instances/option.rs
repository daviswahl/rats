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
    use applicative::{ApplicativeExt, ApplicativeKindExt};
    use kind::HKT;
    #[test]
    fn test_option_pure() {
        let f = 5.point::<OptionKind>();
        assert_eq!(Some(5), f.reify());
    }

    #[test]
    fn test_option_product() {
        let a = 5.point::<OptionKind>();
        let b = "hello".point::<OptionKind>();
        let result = a.product(b);
        assert_eq!(result.reify(), Some((5, "hello")));
    }

    fn ap_map<K: HKT + Applicative<K>, F, A, B>(k: Kind<K, A>, f: F) -> Kind<K, B>
    where
        F: Fn(A) -> B,
    {
        <K as Applicative<K>>::map(k, f)
    }

    #[test]
    fn map_via_applicative() {
        let f = 5.point::<OptionKind>();
        let result = ap_map(f, |i| i * 2);
        assert_eq!(result.reify(), Some(10))
    }
}
