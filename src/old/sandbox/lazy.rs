use super::{Empty, OptionKind, HKT};
use std::marker::PhantomData;

struct Nothing {}

struct Head<A>(PhantomData<*const A>);
impl<A> LazyNode for Head<Option<A>> {
    type Input = Option<A>;
    type Item = Option<A>;

    fn run_inner(self, input: <Self as LazyNode>::Input) -> <Self as LazyNode>::Item {
        input
    }
}
/// LazyMap
struct LazyMap<I: LazyNode, A, B, F> {
    inner: I,
    func: F,
    a: PhantomData<*const A>,
    b: PhantomData<*const B>
}


struct Lower<A,I> {
   inner: I,
    a: PhantomData<*const A>
}

impl<A,I> LazyNode for Lower<Option<A>,I> where I: LazyNode<Item=Option<A>> {
    type Input = I::Input;
    type Item = A;

    fn run_inner(self, input: <Self as LazyNode>::Input) -> <Self as LazyNode>::Item {
        let f = self.inner.run_inner(input);
        
    }
}
fn lift_fn<I: LazyNode, A, B, F>(inner: I, func: F) -> LazyMap<I, A,B, F> where
    F: Fn(A) -> B {
    LazyMap {
        inner,
        func,
        a: PhantomData,
        b: PhantomData,

    }
}

impl<I: LazyNode<Item=A>, A, B, F> LazyNode for LazyMap<I, A, B, F> where F: Fn(A) -> B{
    type Input = I::Input;
    type Item =  B;

    fn run_inner(self, input: <Self as LazyNode>::Input) -> <Self as LazyNode>::Item {
        (self.func)(self.inner.run_inner(input))
    }
}

/// Kind
trait Lifted {
    type Kind;
    type Lifted;
    type Unlifted;
}

impl<A> Lifted for Option<A> {
    type Kind = OptionKind;
    type Lifted = Option<A>;
    type Unlifted = A;
}

/// LazyNode
trait LazyNode {
    type Input;
    type Item;
    fn run_inner(self, input: Self::Input) -> Self::Item;
}

/// Lazy
struct Lazy<K: HKT, A, L: LazyNode> {
    k: PhantomData<*const K>,
    a: PhantomData<*const A>,
    inner: L,
}

impl<A, Inner: LazyNode> LazyNode for Lazy<OptionKind, A, Inner> {
    type Input = Inner::Input;
    type Item = Inner::Item;

    fn run_inner(self, input: <Self as LazyNode>::Input) -> <Self as LazyNode>::Item {
        self.inner.run_inner(input)
    }
}

/// Functor
trait Functor<F_: HKT> {
    fn map<FA,F, A, B>(fa: FA, f: F) -> LazyMap<FA,A,B,F>
    where
        FA: LazyNode<Item=A>,
        F: Fn(A) -> B,

        A: Lifted<Kind = F_>,
        B: Lifted<Kind = F_>;
}


impl Functor<OptionKind> for OptionKind {
    fn map<FA,F, A, B>(fa: FA, f: F) -> LazyMap<FA,A,B,F> where
        FA: LazyNode<Item=A>,
        F: Fn(A) -> B,
        A: Lifted<Kind=OptionKind>,
        B: Lifted<Kind=OptionKind> {
        lift_fn(fa, f)
    }
}

impl Lifted for i32 {
    type Kind = OptionKind;
    type Lifted = Option<i32>;
    type Unlifted = i32;
}
impl Lifted for String {
    type Kind = OptionKind;
    type Lifted = Option<String>;
    type Unlifted = String;
}
#[test]
fn delay_test() {
    let f = Lazy::<OptionKind, i32, Head<Option<i32>>> {
        k: PhantomData,
        a: PhantomData,
        inner: Head(PhantomData),
    };

    let f = Lower{inner: f, a: PhantomData};
    //let lifted = lift_fn::<_, Option<String>, Option<i32>, _>(f, do_thing, |o: Option<i32>| o.map(|s| format!("{:?}", s)));
    //assert_eq!(lifted.run(Some(1)), Some("1".to_owned()));
    let r = OptionKind::map(f, |i: i32| i * 2) ;
    r.run_inner(Some(1));



    //let r = Lazy::delay(|i| i.map(|i| i * 2), f);
    //let f = r.run(Some(1));
    //assert_eq!(f, Some(2));
}
