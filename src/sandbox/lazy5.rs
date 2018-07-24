use super::{OptionKind, HKT};
use std::marker::PhantomData;
use sandbox::IdKind;

pub struct Nothing {}

impl Delay for Nothing {
    type Input = Nothing;
    type Return = Nothing;
    type Inner = Nothing;

    fn run(&self, _input: <Self as Delay>::Input) -> <Self as Delay>::Return {
        unreachable!()
    }
}

#[allow(dead_code)]
pub struct Head<A>(PhantomData<*const A>);

impl<A> Delay for Head<Option<A>> {
    type Return = Option<A>;
    type Input = Option<A>;
    type Inner = Nothing;

    fn run(&self, input: <Self as Delay>::Input) -> <Self as Delay>::Return {
        input
    }
}

pub struct DelayMap<K: HKT, F, Inner> {
    k: K,
    func: F,
    inner: Inner,
}

/// LazyMap
impl<F, I, A, B> Delay for DelayMap<OptionKind, F, I>
where
    I: Delay<Return = Option<A>>,
    F: Fn(A) -> B,
{
    type Return = Option<B>;
    type Inner = I;
    type Input = I::Input;

    fn run(&self, input: <Self as Delay>::Input) -> <Self as Delay>::Return {
        match self.inner.run(input) {
            Some(i) => Some((self.func)(i)),
            None => None,
        }
    }
}

/// LazyNode
trait Delay {
    type Input;
    type Return;
    type Inner: Delay;

    fn run(&self, input: Self::Input) -> Self::Return;
}

/// Lazy
#[allow(dead_code)]
pub trait Lifted<K: HKT, A, L>: Delay {}


impl<A, L: Delay<Return = Option<A>>> Delay for LiftedKind<OptionKind, A, L> {
    type Input = L::Input;
    type Return = Option<A>;
    type Inner = L;

    fn run(&self, input: <Self as Delay>::Input) -> <Self as Delay>::Return {
        self.inner.run(input)
    }
}

struct LiftedKind<K :, A, Inner>
where
    Inner: Delay,
{
    k: K,
    a: PhantomData<*const A>,
    inner: Inner,
}


impl<F_: HKT, A, Inner: Delay> Lifted<F_, A, Inner> for
LiftedKind<F_,A,Inner> where LiftedKind<F_, A, Inner>: Delay {

}

/// Functor
trait Functor<F_: HKT>: HKT {
    fn map<F, A, B, FA, L>(fa: FA, f: F) -> LiftedKind<F_, B, DelayMap<F_, F, FA>>
    where
        F: Fn(A) -> B,
        FA: Lifted<F_, A, L>,
        L: Delay,
        DelayMap<F_, F, FA>: Delay;
}

impl Functor<OptionKind> for OptionKind {
    fn map<F, A, B, FA, L>(fa: FA, f: F) -> Box<Delay<Input=>>
    where
        F: Fn(A) -> B,
        FA: Lifted<OptionKind, A, L>,
        L: Delay,
        DelayMap<OptionKind, F, FA>: Delay,
    {
        LiftedKind {
            k: OptionKind,
            inner: DelayMap {
                k: OptionKind,
                inner: fa,
                func: f
            },
            a: PhantomData,
        }
    }
}

#[test]
fn delay_test() {

    let l = LiftedKind {
        k: OptionKind,
        a: PhantomData::<*const i32>,
        inner: Head::<Option<i32>>(PhantomData),
    };
    let l = OptionKind::map(l, |i| i * 2);
    let l = OptionKind::map(l, |s| format!("{:?}", s));
    assert_eq!(l.run(Some(1)), Some("2".to_owned()));
    assert_eq!(l.run(Some(2)), Some("4".to_owned()));


    //let r = Lazy::delay(|i| i.map(|i| i * 2), f);
    //let f = r.run(Some(1));
    //assert_eq!(f, Some(2));
}


