use kind::Empty;
use kind::Kind;
use kind::HKT;
use kind::{IntoKind, Reify};
use kinds::{OptionKind, VecKind};

pub trait FunctionK<F_: HKT, G_: HKT, Z = Empty>: HKT {
    type ZOut;
    fn map_kind<A>(fa: Kind<F_, A, Z>) -> Kind<G_, A, Self::ZOut>;
}

impl FunctionK<OptionKind, VecKind> for OptionKind {
    type ZOut = Empty;
    fn map_kind<A>(fa: Kind<OptionKind, A, Empty>) -> Kind<VecKind, A, Self::ZOut> {
        let t = match fa.reify() {
            Some(t) => vec![t],
            None => vec![],
        };
        t.into_kind()
    }
}

pub trait KindFunctionKExt<'f_, F_, A, Z = Empty>
where
    F_: HKT,
{
    type ZOut = Z;
    fn map_kind<G_: HKT>(self) -> Kind<'f_, G_, A, Self::ZOut>
    where
        F_: FunctionK<F_, G_, Z, ZOut = Self::ZOut>;
}

impl<'f_, F_, A, Z> KindFunctionKExt<'f_, F_, A, Z> for Kind<'f_, F_, A, Z>
where
    F_: HKT,
{
    type ZOut = Empty;
    default fn map_kind<G_: HKT>(self) -> Kind<'f_, G_, A, Empty>
    where
        F_: FunctionK<F_, G_, Z, ZOut = Self::ZOut>,
    {
        F_::map_kind(self)
    }
}

#[test]
fn test() {
    let s = Some(1).into_kind();
    let r = s.map_kind::<VecKind>();
    assert_eq!(r.reify(), vec![1]);
}
