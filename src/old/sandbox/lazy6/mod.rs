use super::{Empty, OptionKind, HKT};
use std::marker::PhantomData;
use std::ops::Generator;
use std::ops::GeneratorState;
mod nodes;
mod more_nodes;
use self::nodes::*;
use self::more_nodes::*;
use std::fmt::Debug;

/// Lifted
/// Functor
trait Functor<F_: HKT> {
    fn map<F, A: Debug, B,
        LA>(fa: Lowered<F_, A, LA>, f: F)
            -> LazyOp<F, Lowered<OptionKind, A, LA>>
        where
            F: Fn(A) -> B,
            LA: LazyNode;
}

impl Functor<OptionKind> for OptionKind {
    fn map<F, A: Debug, B, LA>(fa: Lowered<OptionKind, A, LA>, f: F) -> LazyOp<F, Lowered<OptionKind, A, LA>> where
        F: Fn(A) -> B,
        LA: LazyNode {
        LazyOp {
            back: Some(fa),
            op: f,
        }
    }
}

pub trait TypeName: Debug {
    const TYPE_NAME: &'static str;
}

impl TypeName for i32 {
    const TYPE_NAME: &'static str = "i32";
}


trait OpChain {
    type Input;
    type Output;
    type ChainOutput;
    type Next: OpChain<Input=Self::Output>;
    type Op;

    fn call(&self, input: Self::Input) -> Self::ChainOutput;
}

pub struct Compiled<Op, Args, C: OpChain> {
    op: Op,
    inner: Option<C>,
    args: PhantomData<*const Args>,
}


impl<A,Op, Inner> OpChain for Compiled<Op, A, Inner>
    where Inner: OpChain, Op: Fn(A) -> Inner::Input {
    type Input = A;
    type Output = Inner::Input;
    type ChainOutput = Inner::ChainOutput;
    type Next = Inner;
    type Op = Op;

    fn call(&self, input: <Self as OpChain>::Input) -> <Self as OpChain>::ChainOutput {
        unimplemented!()
    }
}

fn compile<I: LazyNode>(i: I) where I::Op: Fn(I::Input) -> I::Return {
    let tail = i.back();
    compile_rec(i.compile::<Nothing>(None), tail);
}

fn compile_rec<Head: OpChain, Tail: LazyNode>(hd: Head, tl: Option<Tail>)
    -> Compiled<Tail::Op, Tail::Input, Head>{

}

#[test]
fn delay_test() {

    let a = Lowered { k: OptionKind, a: PhantomData::<*const i32>, back: LazyHead::<Option<i32>>(PhantomData)};
    //   let a = LazyOp {
    //       op: |o: Option<i32>| o.map(|i| i * 2),
    //       next: a,
    //       a: PhantomData::<*const Option<i32>>
    //   };

    let a = OptionKind::map(a, |i: i32| i * 2);
    let a = Lifted{k: OptionKind, back: a};


    //let result = a.run(Some(1));

    //assert_eq!(Some(2), result);

    //let r = Lazy::delay(|i| i.map(|i| i * 2), f);
    //let f = r.run(Some(1));
    //assert_eq!(f, Some(2));
}