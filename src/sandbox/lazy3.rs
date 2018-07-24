use std::marker::PhantomData;

use super::{HKT,OptionKind, IdKind, Empty};
struct Nothing {}


trait Lazy {
    type Head: Lazy;
    type Output;
}

trait LazyKind<K, A>: Lazy+Sized {
    type Lifted;
    type Unlifted;

    fn map<F, B: LazyKind<K,B>>(self, f: F) -> B where F: Fn(A::Unlifted) -> B::Unlifted, A: LazyKind<K,A>, K: HKT;
}

struct LazyOption<A>{
    a: PhantomData<*const A>,
}

impl<A> Lazy for LazyOption<A> {
    type Head = Self;
    type Output = <Self as LazyKind<OptionKind,A>>::Lifted;
}

impl<A> LazyKind<OptionKind, A> for LazyOption<A> {
    type Lifted = Option<A>;
    type Unlifted = A;

    fn map<F, B>(self, f: F) -> B {
        unimplemented!("in optionkind")
    }
}

struct Lifted<K, A: LazyKind<K,A>> {
    k: PhantomData<*const K>,
    inner: A,
}

impl<A: LazyKind<OptionKind,A>> Lifted<OptionKind, A> {
    fn map<F,B>(self, f: F) -> Lifted<OptionKind, B> where B: LazyKind<OptionKind, B>, F: Fn(A) -> B {
        unimplemented!()
    }
}


trait Functor<K: HKT> {
   fn map<F, A, B>(fa: Lifted<K, A>, f: F) -> Lifted<K,B> where A: LazyKind<K, A>, B: LazyKind<K,B>, F: Fn(A) -> B;
}

impl Functor<OptionKind> for OptionKind {
    fn map<F, A, B>(fa: Lifted<OptionKind, A>, f: F) -> Lifted<OptionKind, B> where A: LazyKind<OptionKind, A>, B: LazyKind<OptionKind, B>, F: Fn(A) -> B {
        fa.map(f)
    }
}

#[test]
fn test() {
    let f= LazyOption::<i32>{a: PhantomData};
    let lifted = Lifted{k: PhantomData,  inner: f};
    Functor::<OptionKind>::map(lifted, |i| i * 2);
}