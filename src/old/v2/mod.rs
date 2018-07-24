use std::marker::PhantomData;

mod applicative;
mod functor;
mod instances;
mod lifted;

pub(crate) mod yieldable;
pub(crate) mod yielder;
use v2::applicative::*;
use v2::instances::*;
use v2::lifted::*;
use v2::yielder::*;

pub trait Eval<'d> {
    type Item;
    type Head: Lifted<'d, Item = Self::Item>;
    fn eval() -> Self::Head;
}

trait EvalF<'l, F> {
    type Head;
    fn evalf(f: F) -> Self::Head;
}

pub struct Nothing {}
impl HKT for Nothing {
    type Kind = Nothing;
    fn marker() -> Self {
        unreachable!()
    }
}

impl<'d> Lifted<'d> for Nothing {
    type Kind = Nothing;
    type Output = Nothing;
    type Input = Nothing;
    type YieldInput = Nothing;
    type Item = Nothing;
    type HeadInput = Nothing;

    fn run(&self, _input: <Self as Lifted>::HeadInput) -> <Self as Lifted>::Output {
        unreachable!()
    }

    fn run_inner(&self, _input: <Self as Lifted>::YieldInput) -> <Self as Lifted>::Output {
        unreachable!()
    }

    fn request_yield<D2: Yielder<'d, Input = Self::Item>>(
        &self,
        _input: <Self as Lifted>::HeadInput,
        _outer: D2,
    ) -> Return<'d, D2::ChainOutput> {
        unreachable!()
    }
}

#[allow(dead_code)]
pub struct Head<'d, K: HKT, A: 'd> {
    pub k: K,
    pub a: PhantomData<&'d A>,
}
