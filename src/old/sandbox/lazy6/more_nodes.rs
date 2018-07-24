
use super::{LazyNode, Lazy, Compiled, OpChain};
use std::ops::Generator;
use std::marker::PhantomData;
use std::fmt::Debug;
use super::TypeName;
#[derive(Debug)]
pub struct Nothing {}


impl TypeName for Nothing {
    const TYPE_NAME: &'static str = "Nothing";
}
impl OpChain for Nothing {
    type Input = Nothing;
    type Output = Nothing;
    type ChainOutput = Nothing;
    type Next = Self;
    type Op = Nothing;

    fn call(&self, input: <Self as OpChain>::Input) -> <Self as OpChain>::ChainOutput {
        unimplemented!()
    }
}
impl LazyNode for Nothing {
    type Back = Nothing;
    type Head = Nothing;
    type Input = Nothing;
    type HeadInput = Nothing;
    type Yield = Nothing;
    type Return = Nothing;
    type YieldInput = Nothing;
    type Op = ();

    fn back(&self) -> Option<<Self as LazyNode>::Back> {
        None
    }

    fn run(&self, input: <Self as LazyNode>::Input) -> Lazy<<Self as LazyNode>::Return> {
        unimplemented!()
    }

    fn run_head(&self, input: <Self as LazyNode>::HeadInput) -> <Self as LazyNode>::Return {
        unimplemented!()
    }

    fn do_yield<'a, 'b, L2: LazyNode<YieldInput=Self::Yield>>(&'a self,
                                                              input: <Self as LazyNode>::Input,
                                                              forward: &L2) ->
                                                     Box<Generator<Yield=<L2 as LazyNode>::Return,
                                                         Return=()>> where Self::Yield: 'b {
        unimplemented!()
    }

    fn accept_yield(&self, input: <Self as LazyNode>::YieldInput) -> <Self as LazyNode>::Return {
        unimplemented!()
    }

    fn compile<C: OpChain>(self, next: Option<C>) -> Compiled<<Self as LazyNode>::Op,
        <Self as LazyNode>::Input, C> {
        unimplemented!()
    }
}

/// lazy head
#[derive(Debug)]
pub struct LazyHead<A: Debug>(pub PhantomData<*const A>);

impl<A: TypeName> TypeName for LazyHead<A> {
    const TYPE_NAME: &'static str = A::TYPE_NAME;
}

impl<A: Debug> TypeName for Option<A> {
    const TYPE_NAME: &'static str = "Option";
}
impl<A: Debug> LazyNode for LazyHead<Option<A>> {
    type Input = Option<A>;
    type Return = Option<A>;

    type HeadInput = Self::Input;
    type Back = Self;

    fn back(&self) -> Option<<Self as LazyNode>::Back> {
        None
    }

    fn run<'a>(&self, input: <Self as LazyNode>::Input) -> Lazy<<Self as LazyNode>::Return> {
        Lazy::Yield
    }

    fn run_head(&self, input: <Self as LazyNode>::HeadInput) -> <Self as LazyNode>::Return {
        input
    }

    type Yield = A;


    type YieldInput = A;


    fn accept_yield(&self, input: <Self as LazyNode>::YieldInput) -> <Self as LazyNode>::Return {
        unimplemented!()
    }

    fn do_yield<'a, 'b, L2: LazyNode<YieldInput=Self::Yield>>(&'a self, input:
    <Self as LazyNode>::Input, forward: &L2) -> Box<Generator<Yield=<L2 as LazyNode>::Return,
        Return=()>> where Self::Yield: 'b {
        unimplemented!()
    }

    type Op = ();


    type Head = Self;

    fn compile<C: OpChain>(self, next: Option<C>) -> Compiled<<Self as LazyNode>::Op,
        <Self as LazyNode>::Input, C> {
        unimplemented!()
    }
}