use functor::Functor;
use futures::FutureExt;
use kind::Kind;
use kind::Reify;
use kinds::FutureKind;
use futures::future;
use applicative::Applicative;
use kind::IntoKind;

type FutureK<'f_, A, B> = Kind<'f_, FutureKind, A, B>;

impl<'f_, Z> Functor<'f_, FutureKind, Z> for FutureKind {
    fn map<Fn_, A, B>(fa: FutureK<'f_, A, Z>, fn_: Fn_) -> FutureK<B, Z>
    where
        Fn_: FnOnce(A) -> B + 'f_,
    {
        Kind::Future::<FutureKind, B, Z>(Box::new(fa.reify().map(fn_)))
    }
}

impl<'f_, Z> Applicative<'f_, FutureKind, Z> for FutureKind {
    fn ap<A: 'f_, B: 'f_, Fn_>(
        fa: FutureK<'f_, A, Z>,
        ff: FutureK<'f_, Fn_, Z>,
    ) -> FutureK<'f_, B, Z>
    where
        Fn_: FnOnce(A) -> B,
    {
        let fa = fa.reify();
        let ff = ff.reify();
        let fb = fa.and_then(|fa| ff.map(|ff| ff(fa)));

        Kind::Future(Box::new(fb))
    }

    fn point<A>(a: A) -> FutureK<'f_, A, Z> {
        Box::new(future::ok(a)).into_kind()
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
