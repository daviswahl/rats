use super::{Empty, OptionKind, IdKind, HKT};

struct Id<A>(A);

use std::marker::PhantomData;



struct LiftedFn<F, K> {
    f: F,
    u: K,
}

impl<F> LiftedFn<F, OptionKind> {
    fn call_lifted<A,B>(&self, fa: Option<A>) -> Option<B> where F: Fn(A) -> B {
       fa.map(&self.f)
    }
}

impl<F> LiftedFn<F, IdKind> {
    fn call_lifted<A, B>(&self, fa: Id<A>) -> Id<B> where F: Fn(A) -> B {
        Id((self.f)(fa.0))
    }
}


#[test]
fn test() {

    let f = LiftedFn{f: |a: i32| a * 2, u: OptionKind};
    assert_eq!(Some(2), f.call_lifted(Some(1)));
}
