use function_k::FunctionK;
use functor::Functor;
use kind::{Empty, IntoKind, Kind, Reify, HKT};
use kinds::{OptionKind, ResultKind};

impl<Z> Functor<ResultKind, Z> for ResultKind {
    fn map<F, A, B>(a: Kind<ResultKind, A, Z>, f: F) -> Kind<ResultKind, B, Z>
    where
        F: FnMut(A) -> B,
    {
        a.reify().map(f).into_kind()
    }
}

impl<Z> FunctionK<ResultKind, OptionKind, Z> for ResultKind {
    type ZOut = Empty;
    fn map_kind<A>(fa: Kind<ResultKind, A, Z>) -> Kind<OptionKind, A, Empty> {
        match fa.reify() {
            Ok(t) => Some(t),
            Err(_) => None,
        }.into_kind()
    }
}

trait ResultKindExt<A, B> {
    fn map_kind<G: HKT>(self) -> Kind<G, A, Empty>
    where
        ResultKind: FunctionK<ResultKind, G, B, ZOut = Empty>;
}

impl<A, B> ResultKindExt<A, B> for Kind<ResultKind, A, B> {
    fn map_kind<G: HKT>(self) -> Kind<G, A, Empty>
    where
        ResultKind: FunctionK<ResultKind, G, B, ZOut = Empty>,
    {
        ResultKind::map_kind(self)
    }
}

trait ResultExt {
    fn ok<A>(self) -> Kind<ResultKind, Self, A>
    where
        Self: Sized;
    fn err<A>(self) -> Kind<ResultKind, A, Self>
    where
        Self: Sized;
}

impl<T> ResultExt for T
where
    T: Sized,
{
    fn ok<A>(self) -> Kind<ResultKind, T, A> {
        Ok::<T, A>(self).into_kind()
    }

    fn err<A>(self) -> Kind<ResultKind, A, T> {
        Err::<A, T>(self).into_kind()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use functor::KindFunctorExt;
    #[test]
    fn test_result_functor() {
        let r = Ok::<i32, &str>(4).into_kind().map(|i| i * 2).reify();
        assert_eq!(r, Ok(8))
    }

    #[test]
    fn test_result_function_k() {
        let r = 4.ok::<&str>().map_kind::<OptionKind>().reify();
        assert_eq!(r, Some(4));

        let r = "woop".err::<i32>().map_kind::<OptionKind>().reify();
        assert_eq!(r, None)
    }
}
