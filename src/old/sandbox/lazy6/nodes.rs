use super::{Empty, OptionKind, HKT, Compiled, OpChain};
use std::marker::PhantomData;
use std::fmt::{Formatter, Error};

use super::TypeName;


pub enum Lazy<A> {
    Return(A),
    Yield,
    Await,
}

impl<A: TypeName> Debug for Lazy<A> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Lazy::Return(r) => write!(f, "Lazy::Return[Return={:?}]", A::TYPE_NAME),
            Lazy::Yield => write!(f, "Lazy::Yield[Return={:?}]", A::TYPE_NAME),
            Lazy::Await => write!(f, "Lazy::Await[Return={:?}]", A::TYPE_NAME)
        }
    }
}


type GenBox<'a, B> = Box<Generator<Yield=B, Return=()>+'a>;

pub trait LazyNode: TypeName {
    type Back: LazyNode<HeadInput=Self::HeadInput, Return=Self::Input, Yield=Self::YieldInput>;
    type Head: LazyNode;
    type Input: Debug;
    type HeadInput: Debug;
    type Yield: Debug;
    type Return: Debug;
    type YieldInput: Debug;

    type Op;

    fn back(&self) -> Option<Self::Back>;

    fn run(&self, input: Self::Input) -> Lazy<Self::Return>;

    fn run_head(&self, input: Self::HeadInput) -> Self::Return;

    fn do_yield<'a, 'b, L2: LazyNode<YieldInput=Self::Yield>>(&'a self, input: Self::Input, forward: &'b L2) -> GenBox<'b, L2::Return> where Self::Yield: 'b;

    fn accept_yield(&self, input: Self::YieldInput) -> Self::Return;

    fn compile<C: OpChain>(self, next: Option<C>) -> Compiled<Self::Op, Self::Input, C>;
}


/// lazy op

pub struct LazyOp<F,L> {
    pub back: Option<L>,
    pub op: F,
}

impl<F,L> Debug for LazyOp<F,L> where L: LazyNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "LazyOp[Next={:?}]", self.next)
    }
}

impl<F,L: LazyNode> TypeName for LazyOp<F,L> {
    const TYPE_NAME: &'static str = "LazyNode";
}

impl<F,L,B> LazyNode for LazyOp<F,L> where L: LazyNode, F: Fn(L::Yield) -> B, B: TypeName {
    type Input = L::Return;
    type Head = L::Head;
    type Return = B;
    type HeadInput = L::HeadInput;
    type Back = L;
    type Yield = B;

    fn back(&self) -> Option<Self::Back> {
        self.back.take()
    }


    fn run_head(&self, input: <Self as LazyNode>::HeadInput) -> <Self as LazyNode>::Return {
        unimplemented!()
    }

    fn run<'a>(&self, input: <Self as LazyNode>::Input) -> Lazy<<Self as LazyNode>::Return> {
        Lazy::Await
    }

    type YieldInput = L::Yield;

    fn accept_yield(&self, input: <Self as LazyNode>::YieldInput) -> <Self as LazyNode>::Return {
        unimplemented!()
    }

    fn do_yield<'a, 'b,
        L2: LazyNode<YieldInput=Self::Yield>>(&'a self,
                                              input:
                                              <Self as LazyNode>::Input, forward: &L2)
        -> Box<Generator<Yield=<L2 as LazyNode>::Return, Return=()>> where Self::Yield: 'b {
        unimplemented!()
    }

    type Op = ();

    fn compile<C: OpChain>(self, next:
    Option<C>) -> Compiled<<Self as LazyNode>::Op, <Self as LazyNode>::Input, C> {
        unimplemented!()
    }
}

use std::fmt;
use std::fmt::Debug;
use std::ops::Generator;


/// Lifted
#[derive(Debug)]
pub struct Lowered<K: HKT, A, LA> {
    pub k: K,
    pub a: PhantomData<* const A>,
    pub back: Option<LA>,
}


impl<A: Debug, LA: LazyNode, K: HKT+Debug> TypeName for Lowered<K,A, LA> {
    const TYPE_NAME: &'static str = "Lifted";
}

impl<A: Debug, LA> LazyNode for Lowered<OptionKind, A, LA> where LA: LazyNode<Return=Option<A>> {

    type Input = LA::Return;
    type Return = Option<A>;
    type HeadInput = LA::HeadInput;
    type Back = LA;

    fn back(&self) -> Option<<Self as LazyNode>::Back> {
        self.back.take()
    }

    fn run<'a>(&self, input: <Self as LazyNode>::Input) -> Lazy<<Self as LazyNode>::Return> {
        Lazy::Yield
    }

    fn run_head(&self, input: <Self as LazyNode>::HeadInput) -> <Self as LazyNode>::Return {
        unimplemented!()
    }

    type Yield = A;

    fn do_yield<'a, 'b,
        L2: LazyNode<YieldInput=Self::Yield>>(&'a self,
                                              input: <Self as LazyNode>::Input, forward: &'b L2)
        -> GenBox<'b, L2::Return> where Self::Yield: 'b {
        Box::new(move || {
            match input {
                Some(i) => {
       //             yield forward.accept_yield(i);
                }
                None => return ()

        }})
    }

    type YieldInput = LA::Yield;

    fn accept_yield(&self, input: <Self as LazyNode>::YieldInput) -> <Self as LazyNode>::Return {
        unimplemented!()
    }

    type Op = ();


    type Head = LA::Head;

    fn compile<C: OpChain>(self, next: Option<C>) -> Compiled<<Self as LazyNode>::Op,
        <Self as LazyNode>::Input, C> {
        unimplemented!()
    }
}





#[derive(Debug)]
pub struct Lifted<K:HKT, LA> {
    pub k: K,
    pub back: Option<LA>,
}

impl<LA: LazyNode, K: HKT+Debug> TypeName for Lifted<K,LA> {
    const TYPE_NAME: &'static str = "Lifted";
}

impl<A: Debug, LA> LazyNode for Lifted<OptionKind, LA> where LA: LazyNode<Return=A> {

    type Input = LA::Return;
    type Return = Option<LA::Return>;
    type HeadInput = LA::HeadInput;
    type Back = LA;

    fn back(&self) -> Option<<Self as LazyNode>::Back> {
        self.back.take()
    }

    fn run<'a>(&self, input: <Self as LazyNode>::Input) -> Lazy<<Self as LazyNode>::Return> {
        Lazy::Yield
    }

    fn run_head(&self, input: <Self as LazyNode>::HeadInput) -> <Self as LazyNode>::Return {
        unimplemented!()
    }

    type Yield = A;

    fn do_yield<'a, 'b,
        L2: LazyNode<YieldInput=Self::Yield>>(&'a self,
                                              input: <Self as LazyNode>::Input,
                                              forward: &'b L2) -> GenBox<'b, L2::Return> where
        Self::Yield: 'b {
        unimplemented!()
    }

    type YieldInput = LA::Yield;

    fn accept_yield(&self, input: <Self as LazyNode>::YieldInput) -> <Self as LazyNode>::Return {
        unimplemented!()
    }

    type Op = ();


    type Head = LA::Head;

    fn compile<C: OpChain>(self, next: Option<C>) -> Compiled<<Self as LazyNode>::Op,
        <Self as LazyNode>::Input, C> {
        unimplemented!()
    }
}

