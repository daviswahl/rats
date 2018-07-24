use std::ops::Generator;
use v2::yieldable::Yieldable;
use v2::yielder::YieldHead;
use v2::yielder::Yielder;
use v2::Eval;
use v2::Head;
use std::marker::PhantomData;

pub trait HKT {
    type Kind: HKT;
    fn marker() -> Self;
    fn eval<'l, A>() -> Head<'l, Self, A> where Self: Sized, Head<'l, Self, A>: Lifted<'l> {
        Head {
            k: Self::marker(),
            a: PhantomData
        }
    }
}

pub struct OptionKind;

impl HKT for OptionKind {
    type Kind = OptionKind;
    fn marker() -> Self {
        OptionKind
    }
}

pub struct VecKind;
impl HKT for VecKind {
    type Kind = VecKind;
    fn marker() -> Self {
        VecKind
    }
}

pub enum Return<'l, A> {
    Return(A),
    Yield(Box<'l + Generator<Return = (), Yield = A>>),
    None,
}
pub trait Lifted<'l> {
    type Kind: HKT;
    type Output;
    type Input;
    type YieldInput;
    type Item;
    type HeadInput;

    #[inline]
    fn run(&'l self, input: Self::HeadInput) -> Self::Output
    where
        Self: 'l,
        Self::Kind: Yieldable<Self::Kind, Self::Item, Item = Self::Item, Collected = Self::Output>,
    {
        <Self::Kind as Yieldable<Self::Kind, Self::Item>>::handle_yield(
            self.request_yield(input, YieldHead::new()),
        )
    }

    #[inline]
    fn run_inner(&'l self, input: Self::YieldInput) -> Self::Item;

    #[inline]
    fn request_yield<D2: 'l + Yielder<'l, Input = Self::Item>>(
        &'l self,
        input: Self::HeadInput,
        outer: D2,
    ) -> Return<'l, D2::ChainOutput>;
}

impl<
        'l,
        K: HKT + Yieldable<K, L1::Item> + Yieldable<K, L2::Item>,
        L1: Lifted<'l, Kind = K>,
        L2: Lifted<'l, Kind = K>,
    > Lifted<'l> for Combine<L1, L2>
{
    type Kind = K;
    type Output = (L1::Output, L2::Output);
    type Input = (L1::Output, L2::Output);
    type YieldInput = (L1::Item, L2::Item);
    type Item = (L1::Item, L2::Item);
    type HeadInput = (L1::HeadInput, L2::HeadInput);

    fn run_inner(&'l self, input: <Self as Lifted<'l>>::YieldInput) -> <Self as Lifted<'l>>::Item {
        input
    }

    fn request_yield<D2: 'l + Yielder<'l, Input = Self::Item>>(
        &'l self,
        input: <Self as Lifted<'l>>::HeadInput,
        outer: D2,
    ) -> Return<<D2 as Yielder<'l>>::ChainOutput> {
        match self.l1.request_yield(input.0, YieldHead::new()) {
            Return::Return(o) => match self.l2.request_yield(input.1, YieldHead::new()) {
                Return::Return(o2) => Return::Return(outer.run((o, o2))),
                _ => unimplemented!(),
            },

            _ => unimplemented!(),
        }
    }
}

pub struct Combine<L1, L2> {
    l1: L1,
    l2: L2,
}

impl<'l, L1: 'l + Lifted<'l>, L2: 'l + Lifted<'l>> Combine<L1, L2> {
    pub fn new(l1: L1, l2: L2) -> Self {
        Combine { l1, l2 }
    }
}
