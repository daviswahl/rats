use kind::Empty;
use kind::Kind;
use kind::HKT;
use kind::{EmptyType, IntoKind, Reify};
use kinds::{OptionKind, VecKind};

pub trait FunctionK<F: HKT, G: HKT, Z = Empty>: HKT {
    type ZOut;
    fn map_kind<A>(fa: Kind<F, A, Z>) -> Kind<G, A, Self::ZOut>;
}

impl FunctionK<OptionKind, VecKind> for OptionKind {
    type ZOut = Empty;
    fn map_kind<A>(
        fa: Kind<OptionKind, A, Empty>,
    ) -> Kind<VecKind, A, Self::ZOut> {
        let t = match fa.reify() {
            Some(t) => vec![t],
            None => vec![],
        };
        t.into_kind()
    }
}

pub trait KindFunctionKExt<'kind, K, A, Z = Empty>
where
    K: HKT,
{
    type ZOut = Z;
    fn map_kind<G: HKT>(self) -> Kind<'kind, G, A, Self::ZOut>
    where
        K: FunctionK<K, G, Z, ZOut = Self::ZOut>;
}

impl<'kind, K, A, Z> KindFunctionKExt<'kind, K, A, Z> for Kind<'kind, K, A, Z>
where
    K: HKT,
    Z: EmptyType,
{
    type ZOut = Empty;
    default fn map_kind<G: HKT>(self) -> Kind<'kind, G, A, Empty>
    where
        K: FunctionK<K, G, Z, ZOut = Self::ZOut>,
    {
        K::map_kind(self)
    }
}

#[test]
fn test() {
    let s = Some(1).into_kind();
    let r = s.map_kind::<VecKind>();
    assert_eq!(r.reify(), vec![1]);
}
