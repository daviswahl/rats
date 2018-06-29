use kind::Kind;
use kind::HKT;
use kind::{IntoKind, Reify};
use kind::{OptionKind, VecKind};

pub trait FunctionK<F: HKT, G: HKT> {
    fn map_kind<A>(fa: Kind<F, A>) -> Kind<G, A>;
}

impl FunctionK<OptionKind, VecKind> for OptionKind {
    fn map_kind<A>(fa: Kind<OptionKind, A>) -> Kind<VecKind, A> {
        let t = match fa.reify() {
            Some(t) => vec![t],
            None => vec![],
        };
        t.into_kind()
    }
}

pub trait KindFunctionKExt<K, A>
where
    K: HKT,
{
    fn map_kind<G: HKT>(self) -> Kind<G, A>
    where
        K: FunctionK<K, G>;
}

impl<K, A> KindFunctionKExt<K, A> for Kind<K, A>
where
    K: HKT,
{
    fn map_kind<G: HKT>(self) -> Kind<G, A>
    where
        K: FunctionK<K, G>,
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
