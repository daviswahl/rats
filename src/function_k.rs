use hkt::HKT;
use kind::Kind;
use kind::{KindExt, Kinded};
use kind::{OptionKind, VecKind};

trait FunctionK<F: HKT, G: HKT> {
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

trait FunctionKExt<K, A>
where
    K: HKT,
{
    fn map_kind<G: HKT>(self) -> Kind<G, A>
    where
        K: FunctionK<K, G>;
}

impl<K, A> FunctionKExt<K, A> for Kind<K, A>
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
    let s: Kind<OptionKind, i32> = Kind::new(Some(1));
    let r = s.map_kind::<VecKind>();
    assert_eq!(r.reify(), vec![1]);
}
