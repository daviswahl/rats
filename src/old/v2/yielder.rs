use std::marker::PhantomData;
use v2::lifted::*;
pub struct YieldChain<'d, D: 'd + Lifted<'d>, Y: Yielder<'d>> {
    d: &'d D,
    next: Y,
}

impl<'d, D: 'd + Lifted<'d>, Y: Yielder<'d>> YieldChain<'d, D, Y> {
    pub fn new(d: &'d D, y: Y) -> Self {
        YieldChain { d, next: y }
    }
}

pub trait Yielder<'d> {
    type Input;
    type Output;
    type ChainOutput: 'd;
    type Next: Yielder<'d, Input = Self::Output>;
    #[inline]
    fn run(&self, input: Self::Input) -> Self::ChainOutput;
}

impl<'d, D: Lifted<'d>, Next> Yielder<'d> for YieldChain<'d, D, Next>
where
    Next: Yielder<'d, Input = D::Item>,
{
    type Input = D::YieldInput;
    type Output = D::Item;
    type ChainOutput = Next::ChainOutput;
    type Next = Next;

    #[inline]
    fn run(&self, input: <Self as Yielder<'d>>::Input) -> <Self as Yielder<'d>>::ChainOutput {
        self.next.run(self.d.run_inner(input))
    }
}

pub struct YieldHead<A>(PhantomData<*const A>);
impl<'d, A: 'd> Yielder<'d> for YieldHead<A> {
    type Input = A;
    type Output = A;
    type ChainOutput = A;
    type Next = Self;

    #[inline]
    fn run(&self, input: <Self as Yielder<'d>>::Input) -> <Self as Yielder<'d>>::ChainOutput {
        input
    }
}

impl<A> YieldHead<A> {
    pub fn new() -> YieldHead<A> {
        YieldHead(PhantomData)
    }
}
