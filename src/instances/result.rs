use function_k::FunctionK;
use functor::Functor;
use kind::{Empty, IntoKind, Kind, Reify, HKT};
use kinds::{OptionKind, ResultKind};

impl<Z> Functor<ResultKind, Z> for ResultKind {
    fn map<'kind, F, A, B>(a: Kind<'kind, ResultKind, A, Z>, f: F) -> Kind<'kind, ResultKind, B, Z>
    where
        F: FnMut(A) -> B + 'kind,
    {
        a.reify().map(f).into_kind()
    }
}

impl<Z> FunctionK<ResultKind, OptionKind, Z> for ResultKind {
    type ZOut = Empty;
    fn map_kind<'kind, A>(fa: Kind<'kind, ResultKind, A, Z>) -> Kind<'kind, OptionKind, A, Empty> {
        match fa.reify() {
            Ok(t) => Some(t),
            Err(_) => None,
        }.into_kind()
    }
}

trait ResultKindExt<'kind, A, B> {
    fn map_kind<G: HKT>(self) -> Kind<'kind, G, A, Empty>
    where
        ResultKind: FunctionK<ResultKind, G, B, ZOut = Empty>;
}

impl<'kind, A, B> ResultKindExt<'kind, A, B> for Kind<'kind, ResultKind, A, B> {
    fn map_kind<G: HKT>(self) -> Kind<'kind, G, A, Empty>
    where
        ResultKind: FunctionK<ResultKind, G, B, ZOut = Empty>,
    {
        ResultKind::map_kind(self)
    }
}

trait ResultExt {
    fn ok<'kind, A>(self) -> Kind<'kind, ResultKind, Self, A>
    where
        Self: Sized;
    fn err<'kind, A>(self) -> Kind<'kind, ResultKind, A, Self>
    where
        Self: Sized;
}

impl<T> ResultExt for T
where
    T: Sized,
{
    fn ok<'kind, A>(self) -> Kind<'kind, ResultKind, T, A> {
        Ok::<T, A>(self).into_kind()
    }

    fn err<'kind, A>(self) -> Kind<'kind, ResultKind, A, T> {
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
