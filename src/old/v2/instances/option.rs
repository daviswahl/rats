use std::marker::PhantomData;
use std::ops::Generator;
use std::ops::GeneratorState;
use v2::applicative::Ap;
use v2::applicative::Product;
use v2::functor::FMap;
use v2::functor::Functor;
use v2::lifted::Lifted;
use v2::lifted::OptionKind;
use v2::lifted::Return;
use v2::yieldable::Yieldable;
use v2::yielder::YieldChain;
use v2::yielder::YieldHead;
use v2::yielder::Yielder;
use v2::Eval;
use v2::EvalF;
use v2::Head;
use v2::applicative::Map2;

impl<'d, A> Lifted<'d> for Head<'d, OptionKind, A> {
    type Kind = OptionKind;
    type Output = Option<A>;
    type Input = Option<A>;
    type YieldInput = A;
    type Item = A;
    type HeadInput = Option<A>;

    #[inline]
    fn run_inner(&self, input: <Self as Lifted<'d>>::YieldInput) -> <Self as Lifted<'d>>::Item {
        input
    }

    fn request_yield<D2: 'd + Yielder<'d, Input = Self::Item>>(
        &'d self,
        input: <Self as Lifted<'d>>::HeadInput,
        outer: D2,
    ) -> Return<<D2 as Yielder>::ChainOutput> {
        match input {
            Some(a) => Return::Return(outer.run(a)),
            None => Return::None,
        }
    }
}

impl<A> Yieldable<OptionKind, A> for OptionKind {
    type Collected = Option<A>;
    type Item = A;

    fn zero() -> <Self as Yieldable<OptionKind, A>>::Collected {
        None
    }

    fn accumulate<'d>(
        mut r: Box<'d + Generator<Yield = A, Return = ()>>,
    ) -> <Self as Yieldable<OptionKind, A>>::Collected {
        match unsafe { r.resume() } {
            GeneratorState::Yielded(y) => Some(y),
            GeneratorState::Complete(_) => None,
        }
    }

    fn point<'d>(
        i: <Self as Yieldable<OptionKind, A>>::Item,
    ) -> <Self as Yieldable<OptionKind, A>>::Collected {
        Some(i)
    }

    fn generate<'d, D: 'd + Yielder<'d, Input = Self::Item>>(
        mut c: Option<A>,
        d: D,
    ) -> Return<'d, D::ChainOutput>
    where
        Self::Item: 'd,
    {
        match c {
            Some(i) => Return::Return(d.run(i)),
            None => Return::None,
        }
    }

    fn singleton<'d, D: 'd + Yielder<'d, Input = Self::Item>>(
        i: <Self as Yieldable<OptionKind, A>>::Collected,
        d: D,
    ) -> Return<'d, <D as Yielder<'d>>::ChainOutput> {
        match i {
            Some(i) => Return::Return(d.run(i)),
            None => Return::None,
        }
    }
}

impl<'d, A: 'd> Eval<'d> for Option<A> {
    type Item = A;
    type Head = Head<'d, OptionKind, A>;

    fn eval() -> <Self as Eval<'d>>::Head {
        Head {
            k: OptionKind,
            a: PhantomData,
        }
    }
}

impl<'d> Functor<'d, OptionKind> for OptionKind {
    #[inline]
    fn map<F, A, B, FA>(fa: FA, f: F) -> FMap<'d, OptionKind, B, F, FA>
    where
        F: Fn(FA::Item) -> B,
        FA: Lifted<'d, Kind = OptionKind, Item = A>,
    {
        FMap::new(fa, f)
    }
}

/// Ap

impl<'l, A: 'l, B: 'l, F: 'l> EvalF<'l, F> for Option<A>
where
    F: Fn(A) -> B,
{
    type Head = Head<'l, OptionKind, F>;

    fn evalf(f: F) -> <Self as EvalF<'l, F>>::Head {
        Head {
            k: OptionKind,
            a: PhantomData,
        }
    }
}

impl<'l, A: 'l, B: 'l, F, FF, FA> Lifted<'l> for Ap<'l, OptionKind, B, FF, FA>
where
    FA: Lifted<'l, Kind = OptionKind, Item = A>,
    FF: Lifted<'l, Kind = OptionKind, Item = F>,
    F: Fn(A) -> B,
{
    type Kind = OptionKind;
    type Output = Option<B>;
    type Input = Option<A>;
    type YieldInput = (FA::Item, FF::Item);
    type Item = B;
    type HeadInput = (FA::HeadInput, FF::HeadInput);

    fn run(&'l self, input: <Self as Lifted<'l>>::HeadInput) -> <Self as Lifted<'l>>::Output
    where
        Self: 'l,
    {
        let y = self.request_yield::<YieldHead<B>>(input, YieldHead::new());
        match y {
            Return::Return(a) => Some(a),
            Return::Yield(mut gen) => match unsafe { gen.resume() } {
                GeneratorState::Yielded(y) => Some(y),
                GeneratorState::Complete(_) => None,
            },
            Return::None => None,
        }
    }

    fn run_inner(&'l self, input: <Self as Lifted<'l>>::YieldInput) -> <Self as Lifted<'l>>::Item {
        (input.1)(input.0)
    }

    fn request_yield<D2: 'l + Yielder<'l, Input = Self::Item>>(
        &'l self,
        input: <Self as Lifted<'l>>::HeadInput,
        outer: D2,
    ) -> Return<<D2 as Yielder>::ChainOutput> {
        let chain = YieldChain::new(self, outer);
        let result = self.combine.request_yield(input, chain);
        result
    }
}

impl<'l, A, B, FA, FB> Lifted<'l> for Product<OptionKind, (A, B), FA, FB>
where
    FA: Lifted<'l, Kind = OptionKind, Item = A>,
    FB: Lifted<'l, Kind = OptionKind, Item = B>,
{
    type Kind = OptionKind;
    type Output = Option<(A, B)>;
    type Input = Option<A>;
    type YieldInput = (FA::Item, FB::Item);
    type Item = (A, B);
    type HeadInput = (FA::HeadInput, FB::HeadInput);

    fn run_inner(&'l self, input: <Self as Lifted<'l>>::YieldInput) -> <Self as Lifted<'l>>::Item {
        input
    }

    fn request_yield<D2: 'l + Yielder<'l, Input = Self::Item>>(
        &'l self,
        input: <Self as Lifted<'l>>::HeadInput,
        outer: D2,
    ) -> Return<<D2 as Yielder>::ChainOutput> {
        self.combine.request_yield(input, YieldChain::new(self, outer))
    }
}

impl<'l, A: 'l, B: 'l, C: 'l, FA, FB, F> Lifted<'l> for Map2<OptionKind, C, FA, FB, F>
    where
        FA: Lifted<'l, Kind = OptionKind, Item = A>,
        FB: Lifted<'l, Kind = OptionKind, Item = B>,
        F: Fn((A,B)) -> C
{
    type Kind = OptionKind;
    type Output = Option<(A, B)>;
    type Input = Option<A>;
    type YieldInput = (FA::Item, FB::Item);
    type Item = C;
    type HeadInput = (FA::HeadInput, FB::HeadInput);

    fn run_inner(&'l self, input: <Self as Lifted<'l>>::YieldInput) -> <Self as Lifted<'l>>::Item {
        (self.f)(input)
    }

    fn request_yield<D2: 'l + Yielder<'l, Input = Self::Item>>(
        &'l self,
        input: <Self as Lifted<'l>>::HeadInput,
        outer: D2,
    ) -> Return<<D2 as Yielder>::ChainOutput> {
        self.combine.request_yield(input, YieldChain::new(self, outer))
    }
}

impl<'d, D, F, B: 'd> Lifted<'d> for FMap<'d, OptionKind, B, F, D>
where
    D: Lifted<'d>,
    F: Fn(D::Item) -> B,
{
    type Kind = OptionKind;

    type Output = Option<B>;
    type Input = D::Output;
    type YieldInput = D::Item;
    type Item = B;
    type HeadInput = D::HeadInput;

    #[inline]
    fn run(&'d self, input: <Self as Lifted<'d>>::HeadInput) -> <Self as Lifted<'d>>::Output {
        let y = self.request_yield::<YieldHead<B>>(input, YieldHead::new());
        match y {
            Return::Return(a) => Some(a),
            Return::Yield(mut gen) => match unsafe { gen.resume() } {
                GeneratorState::Yielded(y) => Some(y),
                GeneratorState::Complete(_) => None,
            },
            Return::None => None,
        }
    }

    #[inline]
    fn run_inner(&'d self, input: <Self as Lifted<'d>>::YieldInput) -> <Self as Lifted<'d>>::Item {
        self.call(input)
    }

    fn request_yield<D2: 'd + Yielder<'d, Input = Self::Item>>(
        &'d self,
        input: <Self as Lifted<'d>>::HeadInput,
        outer: D2,
    ) -> Return<<D2 as Yielder>::ChainOutput> {
        let chain = YieldChain::new(self, outer);
        self.inner().request_yield(input, chain)
    }
}

#[cfg(test)]
mod tests {

    fn needs_applicative<'l, K: Applicative<'l, K>, A: 'l, B: 'l, C: 'l, F>(a: A, b: B, f: F) -> C
        where F: Fn((A,B)) -> C,
        Head<'l, K, A>: Lifted<'l, Kind=K, Item=A,>,
        Head<'l, K, B>: Lifted<'l, Kind=K, Item=B>, {
        let fa = K::eval::<A>();
        let fb = K::eval::<B>();
        K::map2(fa, fb, f).run(a, b)
    }

    use super::*;
    use test::*;
    use v2::applicative::*;
    use v2::functor::FunctorExt;
    #[test]
    fn test_option() {
        let h = Head {
            k: OptionKind,
            a: PhantomData::<&'static i32>,
        };
        let r = OptionKind::map(h, |i| i * 2);
        let r = OptionKind::map(r, |i| i * 2);
        let result = r.run(Some(1));
        assert_eq!(result, Some(4))
    }

    #[bench]
    fn bench_option_functor_map(b: &mut Bencher) {
        let h = Head {
            k: OptionKind,
            a: PhantomData::<&'static i32>,
        };
        let r = OptionKind::map(h, |i| i * 2);
        let r = OptionKind::map(r, |i| i * 2);

        b.iter(|| {
            let options = (0..1000).collect::<Vec<i32>>();
            for i in options {
                black_box(r.run(Some(i)));
            }
        })
    }

    #[bench]
    fn bench_option_map_nativew(b: &mut Bencher) {
        b.iter(|| {
            let options = (0..1000).collect::<Vec<i32>>();
            for i in options {
                black_box(Some(i).map(|i| i * 2).map(|i| i * 2));
            }
        })
    }

    #[test]
    fn test_ap() {
        let f = |i| i * 2;
        let fa = Option::<i32>::eval();
        let ff = Option::evalf(f);
        let r = OptionKind::ap(fa, ff);
        r.run((Some(1), Some(f)));
    }

    #[test]
    fn test_product() {
        let fa = Option::<i32>::eval();
        let fb = Option::<&'static str>::eval();
        let r = OptionKind::product(fa, fb);
        let r = r.map(|i| ((i.0 * 2), i.1));
        assert_eq!(r.run((Some(1), Some("foo"))), Some((2, "foo")))
    }

    #[test]
    fn test_map2() {
        let fa = Option::<i32>::eval();
        let fb = Option::<&'static str>::eval();
        let r = OptionKind::map2(fa, fb, |(a,b)| (a*2, b));
        assert_eq!(r.run((Some(1), Some("foo"))), Some((2, "foo")));
    }

    #[bench]
    fn bench_option_ap(b: &mut Bencher) {
        let f = |i| i * 2;
        let fa = Option::<i32>::eval();
        let ff = Option::evalf(f);
        let r = OptionKind::ap(fa, ff);

        b.iter(|| {
            let options = (0..1000).collect::<Vec<i32>>();
            for i in options {
                black_box(r.run((Some(i), Some(f))));
            }
        })
    }

    #[bench]
    fn bench_option_ap_native(b: &mut Bencher) {
        b.iter(|| {
            let options = (0..1000).collect::<Vec<i32>>();
            let f = Some(|i| i * 2);
            for i in options {
                black_box(Some(i).and_then(|i| f.map(|func| func(i))));
            }
        })
    }
}
