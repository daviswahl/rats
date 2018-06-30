use functor::Functor;
use futures::future;
use futures::future::Future;
use futures::FutureExt;
use kind::IntoKind;
use kind::Kind;
use kind::Reify;
use kinds::FutureKind;

impl<Z> Functor<FutureKind, Z> for FutureKind {
    fn map<'kind, F, A, B>(a: Kind<'kind, FutureKind, A, Z>, f: F) -> Kind<FutureKind, B, Z>
    where
        F: FnMut(A) -> B + 'kind,
    {
        let fut: Box<Future<Item = A, Error = Z>> = a.reify();
        let r = fut.map(f);
        let k = Kind::Future::<FutureKind, B, Z>(Box::new(r));
        k
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
