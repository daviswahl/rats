use functor::Functor;
use futures::FutureExt;
use kind::Kind;
use kind::Reify;
use kinds::FutureKind;

impl<Z> Functor<FutureKind, Z> for FutureKind {
    fn map<'f_, FnAb, A, B>(fa: Kind<'f_, FutureKind, A, Z>, fn_ab: FnAb) -> Kind<FutureKind, B, Z>
    where
        FnAb: FnOnce(A) -> B + 'f_,
    {
        Kind::Future::<FutureKind, B, Z>(Box::new(fa.reify().map(fn_ab)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kind::IntoKind;

    use futures::future;
    use functor::KindFunctorExt;
    use futures::executor::ThreadPool;
    use futures::future::FutureResult;
    #[test]
    fn future_functor_test() {
        let a: FutureResult<i32, &str> = future::ok(1);
        let result = a.into_kind().map(|i| i * 2).reify();
        let result = ThreadPool::new().unwrap().run(result).unwrap();
        assert_eq!(2, result)
    }
}
