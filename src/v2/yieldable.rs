use std::ops::Generator;
use v2::lifted::Return;
use v2::lifted::HKT;
use v2::yielder::Yielder;

pub trait Yieldable<K: HKT, A> {
    type Collected;
    type Item;
    fn zero() -> Self::Collected;
    fn accumulate<'d>(r: Box<'d + Generator<Return = (), Yield = Self::Item>>) -> Self::Collected;
    fn point<'d>(i: Self::Item) -> Self::Collected;
    fn generate<'d, D: 'd + Yielder<'d, Input = Self::Item>>(
        c: Self::Collected,
        d: D,
    ) -> Return<'d, D::ChainOutput>
    where
        Self::Item: 'd;

    fn singleton<'d, D: 'd + Yielder<'d, Input = Self::Item>>(
        i: Self::Collected,
        D,
    ) -> Return<'d, D::ChainOutput>;

    fn handle_yield<'d>(r: Return<'d, Self::Item>) -> Self::Collected {
        match r {
            Return::Return(a) => Self::point(a),
            Return::Yield(gen) => Self::accumulate(gen),
            Return::None => Self::zero(),
        }
    }
}
