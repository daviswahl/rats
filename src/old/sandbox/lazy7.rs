use super::{Empty, OptionKind, HKT};
use std::marker::PhantomData;
use std::ops::Generator;
use std::ops::GeneratorState;

struct Nothing {}

type GenBox<'a, B> = Box<Generator<Yield = B, Return = ()> + 'a>;

trait ReverseOp {
    type Back: ReverseOp<Output = Self::Input>;
    type ChainInput;
    type Input;
    type Output;

    fn back(&mut self) -> Option<Self::Back>;

    fn op(self) -> fn(Self::Input) -> Self::Output;
}

struct LazyOp<F, Next, A> {
    op: F,
    next: Next,
    a: PhantomData<*const A>,
}

struct LazyReverseOp<F, Back> {
    op: F,
    back: Option<Back>,
}

impl<F, Back, B> ReverseOp for LazyReverseOp<F, Back>
where
    Back: ReverseOp,
    F: Fn(Back::Output) -> B,
{
    type Back = Back;
    type ChainInput = Back::ChainInput;
    type Input = Back::Output;
    type Output = B;

    fn back(&mut self) -> Option<<Self as ReverseOp>::Back> {
        self.back.take()
    }

    fn op(self) -> <Self as ReverseOp>::Op {
        self.op
    }
}

struct ReverseHead<A>(PhantomData<*const A>);

fn identity<A>(a: A) -> A {
    a
}

impl<A> ReverseOp for ReverseHead<A> {
    type Back = Self;
    type ChainInput = A;
    type Input = A;
    type Output = A;
    type Op = fn(A) -> A;

    fn back(&mut self) -> Option<<Self as ReverseOp>::Back> {
        None
    }

    fn op(self) -> <Self as ReverseOp>::Op {
        identity
    }
}

fn invert<R, A>(mut n: R) -> impl Fn(A) -> R::Output
where
    R: ReverseOp<ChainInput = A, Input = A>,
    R::Op: Fn(A) -> R::Output,
    R::Back: ReverseOp<Input=A>,
    <R::Back as ReverseOp>::Op: Fn(A) -> <R::Back as ReverseOp>::Output
{
    let back = n.back();
    let result = match back {
        Some(back) => invert_rec(back, n),
        None => unimplemented!(),
    };

    n.op()
}

fn invert_rec<Back, Next>(back: Back, mut next: Next) -> impl Fn(Back::Input) -> Back::Output
where
    Back: ReverseOp<Output=Next::Input>,
    Next: ReverseOp,
{
        let back = next.back();
        let func = match back {
            Some(back) => invert_rec(back, next),
            None => unimplemented!(),
        };
        func
}

#[test]
fn test() {
    let op = LazyReverseOp {
        op: |i: i32| i * 2,
        back: Some(ReverseHead(PhantomData::<*const i32>)),
    };

    let mut op = LazyReverseOp {
        op: |i: i32| i * 2,
        back: Some(op),
    };

    invert(op);
}
