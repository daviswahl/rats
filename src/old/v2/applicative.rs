use std::marker::PhantomData;
use std::ops::Generator;
use std::ops::GeneratorState;
use v2::functor;
use v2::functor::{Functor, FunctorExt};
use v2::lifted::Combine;
use v2::lifted::*;
use v2::option::*;
use v2::yielder::YieldChain;
use v2::yielder::YieldHead;
use v2::yielder::Yielder;
use v2::{Eval, Head};

pub struct Ap<'l, K: HKT, B: 'l, FF: 'l, FA: 'l>
where
    FA: Lifted<'l>,
    FF: Lifted<'l>,
{
    pub k: K,
    pub b: PhantomData<&'l B>,
    pub combine: Combine<FA, FF>,
}

pub struct Product<K: HKT, Product, FA, FB> {
    k: K,
    p: PhantomData<*const Product>,
    pub combine: Combine<FA, FB>,
}

pub struct Map2<K: HKT, C, FA, FB, F>
where
{
    pub k: K,
    pub c: PhantomData<*const C>,
    pub combine: Combine<FA, FB>,
    pub f: F,
}

pub trait Applicative<'l, K>: Functor<'l, K>
where
    K: Functor<'l, K>,
{
    /// (F<A>, F<FnOnce(A) -> B>) -> F<B>
    fn ap<A: 'l, B: 'l, Fn_: 'l, FA, FF>(fa: FA, ffn: FF) -> Ap<'l, K, B, FF, FA>
    where
        FA: Lifted<'l, Kind = K, Item = A>,
        FF: Lifted<'l, Kind = K, Item = Fn_>,
        Fn_: FnOnce(A) -> B;

    fn product<A: 'l, B: 'l, FA: 'l, FB: 'l>(fa: FA, fb: FB) -> Product<K, (A, B), FA, FB>
    where
        FA: Lifted<'l, Kind = K, Item = A>,
        FB: Lifted<'l, Kind = K, Item = B>,
    {
        Product {
            k: K::marker(),
            p: PhantomData,
            combine: Combine::new(fa, fb),
        }
    }

    fn map2<A: 'l, B: 'l, C: 'l, FA: 'l, FB: 'l, F>(
        fa: FA,
        fb: FB,
        f: F,
    ) -> Map2<K, C, FA, FB, F>
    where
        FA: Lifted<'l, Kind = K, Item = A>,
        FB: Lifted<'l, Kind = K, Item = B>,
        F: Fn((A, B)) -> C,
    {
        Map2 {
            k: K::marker(),
            c: PhantomData,
            f: f,
            combine: Combine::new(fa, fb)
        }
    }
}

impl<'l, K: Functor<'l, K>> Applicative<'l, K> for K {
    fn ap<A: 'l, B: 'l, Fn_: 'l, FA, FF>(fa: FA, ff: FF) -> Ap<'l, K, B, FF, FA>
    where
        FA: Lifted<'l, Kind = K, Item = A>,
        FF: Lifted<'l, Kind = K, Item = Fn_>,
        Fn_: FnOnce(A) -> B,
    {
        Ap {
            k: K::marker(),
            b: PhantomData,
            combine: Combine::new(fa, ff),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::*;
    use v2::EvalF;

}
