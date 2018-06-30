use functor::Functor;
use kind::{IntoKind, Kind, Reify};
use kinds::ResultKind;

impl<Z> Functor<ResultKind, Z> for ResultKind {
    fn map<F, A, B>(a: Kind<ResultKind, A, Z>, f: F) -> Kind<ResultKind, B, Z>
    where
        F: FnMut(A) -> B,
    {
        a.reify().map(f).into_kind()
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
}
