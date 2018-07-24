use std::marker::PhantomData;
use std::ops::Generator;
use std::ops::GeneratorState;
use v2::functor::FMap;
use v2::functor::Functor;
use v2::lifted::Lifted;
use v2::lifted::Return;
use v2::lifted::VecKind;
use v2::yieldable::Yieldable;
use v2::yielder::YieldChain;
use v2::yielder::Yielder;

use v2::Eval;
use v2::Head;

impl<'d, A: 'd> Eval<'d> for Vec<A> {
    type Item = A;
    type Head = Head<'d, VecKind, A>;

    fn eval() -> <Self as Eval<'d>>::Head {
        Head {
            k: VecKind,
            a: PhantomData,
        }
    }
}

impl<A> Yieldable<VecKind, A> for VecKind {
    type Collected = Vec<A>;
    type Item = A;

    fn zero() -> <Self as Yieldable<VecKind, A>>::Collected {
        vec![]
    }

    fn accumulate<'d>(
        mut r: Box<'d + Generator<Yield = A, Return = ()>>,
    ) -> <Self as Yieldable<VecKind, A>>::Collected {
        let mut buf = vec![];
        loop {
            match unsafe { r.resume() } {
                GeneratorState::Yielded(y) => buf.push(y),
                GeneratorState::Complete(_) => break,
            }
        }
        buf
    }

    fn point<'d>(
        i: <Self as Yieldable<VecKind, A>>::Item,
    ) -> <Self as Yieldable<VecKind, A>>::Collected {
        vec![i]
    }

    fn generate<'d, D: 'd + Yielder<'d, Input = Self::Item>>(
        mut c: Vec<A>,
        d: D,
    ) -> Return<'d, D::ChainOutput>
    where
        Self::Item: 'd,
    {
        if c.len() < 2 {
            match c.pop() {
                Some(a) => return Return::Return(d.run(a)),
                None => return Return::None,
            }
        }

        Return::Yield(Box::new(move || {
            for i in c {
                yield d.run(i)
            }
            return ();
        }))
    }

    fn singleton<'d, D: 'd + Yielder<'d, Input = Self::Item>>(
        mut i: <Self as Yieldable<VecKind, A>>::Collected,
        d: D,
    ) -> Return<'d, <D as Yielder<'d>>::ChainOutput> {
        match i.pop() {
            Some(i) => Return::Return(d.run(i)),
            None => Return::None,
        }
    }
}

impl<'d, D, F, B: 'd> Lifted<'d> for FMap<'d, VecKind, B, F, D>
where
    D: 'd + Lifted<'d>,
    F: Fn(D::Item) -> B,
{
    type Kind = VecKind;

    type Output = Vec<B>;
    type Input = D::Output;
    type YieldInput = D::Item;
    type Item = B;
    type HeadInput = D::HeadInput;

    #[inline]
    fn run_inner(&'d self, input: <Self as Lifted<'d>>::YieldInput) -> <Self as Lifted<'d>>::Item {
        self.call(input)
    }

    #[inline]
    fn request_yield<D2: 'd + Yielder<'d, Input = Self::Item>>(
        &'d self,
        input: <Self as Lifted<'d>>::HeadInput,
        outer: D2,
    ) -> Return<'d, D2::ChainOutput> {
        let chain = YieldChain::new(self, outer);
        self.inner().request_yield(input, chain)
    }
}

impl<'d, A: 'd> Lifted<'d> for Head<'d, VecKind, A> {
    type Kind = VecKind;
    type Output = Vec<A>;
    type Input = Vec<A>;
    type YieldInput = A;
    type Item = A;
    type HeadInput = Vec<A>;

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
    ) -> Return<D2::ChainOutput>
    where
        D2: 'd + Yielder<'d, Input = Self::Item>,
    {
        VecKind::generate(input, outer)
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
    fn map<F, A, B, FA>(fa: FA, f: F) -> FMap<'d, VecKind, B, F, FA>
    where
        F: Fn(FA::Item) -> B,
        FA: Lifted<'d, Kind = VecKind, Item = A>,
    {
        FMap::new(fa, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::*;
    use v2::functor::FunctorExt;

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
    fn bench_vec_map_native(b: &mut Bencher) {
        b.iter(|| {
            let range = (1..100).collect::<Vec<i32>>();
            black_box(
                range
                    .into_iter()
                    .map(|i| i * 2)
                    .map(|i| i * 2)
                    .map(|i| format!("{:?}", i))
                    .collect::<Vec<String>>(),
            )
        })
    }

    #[bench]
    fn bench_vec_map_from_functor(b: &mut Bencher) {
        let h = Vec::<i32>::eval()
            .map(|i| i * 2)
            .map(|i| i * 2)
            .map(|i| format!("{:?}", i));

        b.iter(|| {
            let range = (1..100).collect::<Vec<i32>>();
            black_box(h.run(range))
        })
    }
}
