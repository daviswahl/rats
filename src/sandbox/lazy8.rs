use std::marker::PhantomData;

use super::{OptionKind, HKT};
use std::ops::Generator;
use std::ops::GeneratorState;

pub struct VecKind;
impl HKT for VecKind {
    type Kind = VecKind;
}

impl VecKind {
    fn eval<'d, A>() -> Head<'d, VecKind, A> {
        Head {
            k: VecKind,
            a: PhantomData,
        }
    }
}

trait Eval<'d, A> {
    type Head: Lifted<'d, Item = A>;
    fn eval() -> Self::Head;
}

impl<'d, A: 'd> Eval<'d, A> for Vec<A> {
    type Head = Head<'d, VecKind, A>;

    fn eval() -> <Self as Eval<'d, A>>::Head {
        Head {
            k: VecKind,
            a: PhantomData,
        }
    }
}

pub struct Nothing {}
impl HKT for Nothing {
    type Kind = Nothing;
}
impl<'d> Lifted<'d> for Nothing {
    type Kind = Nothing;
    type Output = Nothing;
    type Input = Nothing;
    type YieldInput = Nothing;
    type Item = Nothing;
    type HeadInput = Nothing;
    type Inner = Nothing;

    fn run(&self, _input: <Self as Lifted>::HeadInput) -> <Self as Lifted>::Output {
        unreachable!()
    }

    fn run_inner(&self, _input: <Self as Lifted>::YieldInput) -> <Self as Lifted>::Output {
        unreachable!()
    }

    fn request_yield<D2: Yield<'d, Input = Self::Item>>(
        &self,
        _input: <Self as Lifted>::HeadInput,
        _outer: D2,
    ) -> Box<Generator<Yield = <D2 as Yield<'d>>::ChainOutput, Return = ()>> {
        unreachable!()
    }
}

pub trait Lifted<'d> {
    type Kind: HKT;
    type Output;
    type Input;
    type YieldInput;
    type Item;
    type HeadInput;

    type Inner: Lifted<
        'd,
        Output = Self::Input,
        Item = Self::YieldInput,
        HeadInput = Self::HeadInput,
    >;

    #[inline]
    fn run(&'d self, input: Self::HeadInput) -> Self::Output
    where
        Self: 'd;

    #[inline]
    fn run_inner(&'d self, input: Self::YieldInput) -> Self::Item;

    #[inline]
    fn request_yield<D2: 'd + Yield<'d, Input = Self::Item>>(
        &'d self,
        input: Self::HeadInput,
        outer: D2,
    ) -> Box<Generator<Return = (), Yield = D2::ChainOutput> + 'd>;
}

pub struct functor<'d, K: HKT, B: 'd, F, D: Lifted<'d>> {
    inner: D,
    f: F,
    __marker_b: PhantomData<&'d B>,
    __marker_k: (K),
}

pub struct YieldChain<'d, D: 'd + Lifted<'d>, Y: Yield<'d>> {
    d: &'d D,
    next: Y,
}

pub trait Yield<'d> {
    type Input;
    type Output;
    type ChainOutput: 'd;
    type Next: Yield<'d, Input = Self::Output>;
    #[inline]
    fn run(&self, input: Self::Input) -> Self::ChainOutput;
}

impl<'d, D: Lifted<'d>, Next> Yield<'d> for YieldChain<'d, D, Next>
where
    Next: Yield<'d, Input = D::Item>,
{
    type Input = D::YieldInput;
    type Output = D::Item;
    type ChainOutput = Next::ChainOutput;
    type Next = Next;

    #[inline]
    fn run(&self, input: <Self as Yield<'d>>::Input) -> <Self as Yield<'d>>::ChainOutput {
        self.next.run(self.d.run_inner(input))
    }
}

pub struct YieldHead<A>(PhantomData<*const A>);
impl<'d, A: 'd> Yield<'d> for YieldHead<A> {
    type Input = A;
    type Output = A;
    type ChainOutput = A;
    type Next = Self;

    #[inline]
    fn run(&self, input: <Self as Yield<'d>>::Input) -> <Self as Yield<'d>>::ChainOutput {
        input
    }
}

impl<'d, D, F, B: 'd> Lifted<'d> for functor<'d, OptionKind, B, F, D>
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
    type Inner = D;

    #[inline]
    fn run(&'d self, input: <Self as Lifted<'d>>::HeadInput) -> <Self as Lifted<'d>>::Output {
        let mut y = self.request_yield::<YieldHead<B>>(input, YieldHead(PhantomData));

        match unsafe { y.resume() } {
            GeneratorState::Yielded(y) => Some(y),
            GeneratorState::Complete(_) => None,
        }
    }

    #[inline]
    fn run_inner(&'d self, input: <Self as Lifted<'d>>::YieldInput) -> <Self as Lifted<'d>>::Item {
        (self.f)(input)
    }

    #[inline]
    fn request_yield<D2: 'd + Yield<'d, Input = Self::Item>>(
        &'d self,
        input: <Self as Lifted<'d>>::HeadInput,
        outer: D2,
    ) -> Box<Generator<Yield = <D2 as Yield<'d>>::ChainOutput, Return = ()> + 'd> {
        let chain = YieldChain {
            d: self,
            next: outer,
        };
        self.inner.request_yield(input, chain)
    }
}

impl<'d, D, F, B: 'd> Lifted<'d> for functor<'d, VecKind, B, F, D>
where
    D: Lifted<'d>,
    F: Fn(D::Item) -> B,
{
    type Kind = VecKind;

    type Output = Vec<B>;
    type Input = D::Output;
    type YieldInput = D::Item;
    type Item = B;
    type HeadInput = D::HeadInput;
    type Inner = D;

    #[inline]
    fn run(&'d self, input: <Self as Lifted<'d>>::HeadInput) -> <Self as Lifted<'d>>::Output {
        let mut y = self.request_yield::<YieldHead<B>>(input, YieldHead(PhantomData));
        let mut buf = vec![];
        loop {
            match unsafe { y.resume() } {
                GeneratorState::Yielded(y) => buf.push(y),
                GeneratorState::Complete(_) => break,
            }
        }
        buf
    }

    #[inline]
    fn run_inner(&'d self, input: <Self as Lifted<'d>>::YieldInput) -> <Self as Lifted<'d>>::Item {
        (self.f)(input)
    }

    #[inline]
    fn request_yield<D2: 'd + Yield<'d, Input = Self::Item>>(
        &'d self,
        input: <Self as Lifted<'d>>::HeadInput,
        outer: D2,
    ) -> Box<Generator<Yield = <D2 as Yield<'d>>::ChainOutput, Return = ()> + 'd> {
        let chain = YieldChain {
            d: self,
            next: outer,
        };
        self.inner.request_yield(input, chain)
    }
}

struct Head<'d, K: HKT, A: 'd> {
    k: K,
    a: PhantomData<&'d A>,
}

impl<'d, A> Lifted<'d> for Head<'d, OptionKind, A> {
    type Kind = OptionKind;
    type Output = Option<A>;
    type Input = Option<A>;
    type YieldInput = A;
    type Item = A;
    type HeadInput = Option<A>;
    type Inner = Self;

    #[inline]
    fn run(&self, input: <Self as Lifted<'d>>::HeadInput) -> <Self as Lifted<'d>>::Output {
        input
    }

    #[inline]
    fn run_inner(&self, input: <Self as Lifted<'d>>::YieldInput) -> <Self as Lifted<'d>>::Item {
        input
    }

    #[inline]
    fn request_yield<D2: 'd + Yield<'d, Input = Self::Item>>(
        &self,
        input: <Self as Lifted<'d>>::HeadInput,
        outer: D2,
    ) -> Box<Generator<Yield = <D2 as Yield<'d>>::ChainOutput, Return = ()> + 'd> {
        Box::new(move || match input {
            Some(a) => yield outer.run(a),
            None => return (),
        })
    }
}

impl<'d, A: 'd> Lifted<'d> for Head<'d, VecKind, A> {
    type Kind = VecKind;
    type Output = Vec<A>;
    type Input = Vec<A>;
    type YieldInput = A;
    type Item = A;
    type HeadInput = Vec<A>;
    type Inner = Self;

    #[inline]
    fn run(&'d self, input: <Self as Lifted<'d>>::HeadInput) -> <Self as Lifted<'d>>::Output {
        input
    }

    #[inline]
    fn run_inner(&'d self, input: <Self as Lifted<'d>>::YieldInput) -> <Self as Lifted<'d>>::Item {
        input
    }

    #[inline]
    fn request_yield<D2>(
        &'d self,
        input: <Self as Lifted<'d>>::HeadInput,
        outer: D2,
    ) -> Box<Generator<Yield = <D2 as Yield<'d>>::ChainOutput, Return = ()> + 'd>
    where D2: 'd + Yield<'d, Input = Self::Item>
    {
        Box::new(move || {
            for i in input.into_iter() {
                yield outer.run(i)
            }
            return ();
        })
    }
}

pub trait Functor<'d, K: HKT>: HKT {
    #[inline]
    fn map<F, A, B, FA>(fa: FA, f: F) -> functor<'d, K, B, F, FA>
    where
        F: Fn(FA::Item) -> B,
        FA: Lifted<'d, Kind = K, Item = A>;
}

impl<'d> Functor<'d, OptionKind> for OptionKind {
    #[inline]
    fn map<F, A, B, FA>(fa: FA, f: F) -> functor<'d, OptionKind, B, F, FA>
    where
        F: Fn(FA::Item) -> B,
        FA: Lifted<'d, Kind = OptionKind, Item = A>,
    {
        functor {
            inner: fa,
            f: f,
            __marker_k: OptionKind,
            __marker_b: PhantomData,
        }
    }
}

impl<'d> Functor<'d, VecKind> for VecKind {
    /// If you use your imagination, you can pretend that translates to:
    /// (F<A>, Fn(A) -> B) -> F<B>
    /// While it's pretty ugly, the type equation above is enforced.
    /// There's also some room for improvement here.
    ///
    /// In this encoding, "FA" represents our concrete HKT, Vec<A>.
    ///
    /// The return type is the ugliest bit: basically, the only way to build a deferred chain
    /// of polymorphic operations on the stack is by building a nested structure like so:
    /// Map {
    ///   op: closure@src...,
    ///   inner: Map {
    ///       op: closure@src...,
    ///       inner: Value(Some(1))
    ///
    /// The outermost map executes it's closure with the value of the output of it's inner Map, which
    /// executes it's closure with the output of _it's_ inner Map, and so forth.
    ///
    /// So anyhow, we basically have to pass our function and our FA into the output of our functor,
    /// so I'm basically just pretending this is boilerplate that you can ignore. However it's still
    /// an important and complicated part of the public api so it's really not ideal. OH well.
    #[inline]
    fn map<F, A, B, FA>(fa: FA, f: F) -> functor<'d, VecKind, B, F, FA>
    where
        F: Fn(FA::Item) -> B,
        FA: Lifted<'d, Kind = VecKind, Item = A>,
    {
        functor {
            inner: fa,
            f: f,
            __marker_k: VecKind,
            __marker_b: PhantomData,
        }
    }
}

trait FunctorExt<'d, K: Functor<'d, K>, A>: Lifted<'d, Kind = K, Item = A> {
    #[inline]
    fn map<F, B>(self, f: F) -> functor<'d, K, B, F, Self>
    where
        Self: Sized,
        F: Fn(Self::Item) -> B;
}

impl<'d, K: Functor<'d, K>, A, D> FunctorExt<'d, K, A> for D
where
    D: Lifted<'d, Kind = K, Item = A>,
{
    #[inline]
    fn map<F, B>(self, f: F) -> functor<'d, K, B, F, Self>
    where
        Self: Sized,
        F: Fn(Self::Item) -> B,
    {
        K::map(self, f)
    }
}

fn uses_functor<'d, K, FA, F: Fn(i32) -> i32>(fa: FA, f: F) -> functor<'d, K, i32, F, FA>
where
    FA: Lifted<'d, Kind = K, Item = i32>,
    K: Functor<'d, K>,
{
    K::map(fa, f)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::*;

    #[test]
    fn test_vec_functor() {
        let e = Vec::<i32>::eval()
            .map(|i| i * 2)
            .map(|i| i * 2)
            .map(|i| format!{"{:?}", i});

        // operation is constructed on the stack, so, typeof e is:
        // `functor<'_, VecKind, String,
        //         functor<'_, VecKind, i32, [closure@src/sandbox/lazy8.rs:377:18: 377:27],
        //                functor<'_, VecKind, i32, [closure@src/sandbox/lazy8.rs:376:18: 376:27],
        //                       Head<'_, VecKind, i32>>>>`
        assert_eq!(
            e.run(vec![1, 2, 3]),
            vec!["4".to_owned(), "8".to_owned(), "12".to_owned()]
        );
        assert_eq!(
            e.run(vec![3, 2, 1]),
            vec!["12".to_owned(), "8".to_owned(), "4".to_owned()]
        )
    }
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

    #[test]
    fn test_vec() {
        let h = Head {
            k: VecKind,
            a: PhantomData::<&i32>,
        };
        let r = VecKind::map(h, |i| i * 2);
        let r = VecKind::map(r, |i| i * 2);
        let result = r.run(vec![1]);
        assert_eq!(result, vec![4])
    }

    #[bench]
    fn bench_option_map_native(b: &mut Bencher) {
        b.iter(|| {
            let r = Some(1)
                .map(|i| i * 2)
                .map(|i| i * 2)
                .map(|i| format!("{:?}", i));
            assert_eq!(r, Some("4".to_owned()))
        })
    }

    #[bench]
    fn bench_option_map_from_functor(b: &mut Bencher) {
        let h = Head {
            k: OptionKind,
            a: PhantomData::<&i32>,
        };
        let r = OptionKind::map(h, |i| i * 2);
        let r = OptionKind::map(r, |i| i * 2);
        let r = OptionKind::map(r, |i| format!("{:?}", i));
        b.iter(|| {
            let result = r.run(Some(1));
            assert_eq!(result, Some("4".to_owned()))
        })
    }

    #[bench]
    fn bench_vec_map_native(b: &mut Bencher) {
        b.iter(|| {
            let range = (1..10000).collect::<Vec<i32>>();
            black_box(range
                .into_iter()
                .map(|i| i * 2)
                .map(|i| i * 2)
                .map(|i| format!("{:?}", i))
                .collect::<Vec<String>>())
        })
    }

    #[bench]
    fn bench_vec_map_from_functor(b: &mut Bencher) {
        let h = Vec::<i32>::eval()
            .map(|i| i * 2)
            .map(|i| i * 2)
            .map(|i| format!("{:?}", i));

        b.iter(|| {
            let range = (1..10000).collect::<Vec<i32>>();
            black_box(h.run(range))
        })
    }
}
