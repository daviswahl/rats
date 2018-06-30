use functor::Functor;
use futures::future::Future;
use futures::FutureExt;
use kind::IntoKind;
use kind::Kind;
use kind::Reify;
use kinds::FutureKind;
//impl<Z> Functor<FutureKind, Z> for FutureKind {
//    default fn map<F, A, B>(a: Kind<FutureKind, A, Z>, f: F) -> Kind<FutureKind, B, Z>
//    where
//        F: FnMut(A) -> B,
//    {
//        let fut: Box<Future<Item = A, Error = Z>> = a.reify();
//        let r = fut.map(f);
//        let k = Kind::Future::<FutureKind, B, Z>(Box::new(r));
//        unimplemented!()
//    }
//}
