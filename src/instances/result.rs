use function_k::FunctionK;
use functor::Functor;
use kind::{Empty, IntoKind, Kind, Reify, HKT};
use kinds::{OptionKind, ResultKind};
use applicative::Applicative;

impl <Z> Functor<ResultKind, Z> for ResultKind {
    fn map<'f_, Fn_, A, B>(a: Kind<'f_, ResultKind, A, Z>, f: Fn_) -> Kind<'f_, ResultKind, B, Z>
    where
        Fn_: FnOnce(A) -> B + 'f_,
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

trait ResultKindExt<'f_, A, B> {
    fn map_kind<G_>(self) -> Kind<'f_, G_, A, Empty>
    where
        G_: HKT,
        ResultKind: FunctionK<ResultKind, G_, B, ZOut = Empty>;
}

impl<'f_, A, B> ResultKindExt<'f_, A, B> for Kind<'f_, ResultKind, A, B> {
    fn map_kind<G: HKT>(self) -> Kind<'f_, G, A, Empty>
    where
        ResultKind: FunctionK<ResultKind, G, B, ZOut = Empty>,
    {
        ResultKind::map_kind(self)
    }
}

trait OkExt {
    fn ok<E>(&self) -> Kind<ResultKind, &Self, E>
    where
        Self: Sized;
}

trait ErrExt {
    fn err<E>(&self) -> Kind<ResultKind, E, &Self>
    where
        Self: Sized;
}

impl<A> OkExt for A
where
    A: Sized,
{
    fn ok<E>(&self) -> Kind<ResultKind, &A, E> {
        Ok::<&A, E>(self).into_kind()
    }
}


impl<E> ErrExt for E {
    fn err<A>(&self) -> Kind<ResultKind, A, &E> {
        Err::<A, &E>(self).into_kind()
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
        assert_eq!(r, Some(&4));

        let r = "woop".err::<i32>().map_kind::<OptionKind>().reify();
        assert_eq!(r, None)
    }
}
