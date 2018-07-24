use std::fmt;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::collections::vec_deque::VecDeque;
trait HKT: 'static + Debug {
    type Kind: HKT;
}

#[derive(Debug)]
struct Empty {}
impl HKT for Empty {
    type Kind = Empty;
}

#[derive(Debug)]
struct IdKind {}
impl HKT for IdKind {
    type Kind = IdKind;
}
#[derive(Debug)]
struct OptionTKind {}
impl HKT for OptionTKind {
    type Kind = OptionTKind;
}
#[derive(Debug)]
struct OptionKind;
impl HKT for OptionKind {
    type Kind = OptionKind;
}

#[derive(Debug)]
struct EitherKind {}
impl HKT for EitherKind {
    type Kind = EitherKind;
}
use std::iter::Iterator;
enum Either<A,B> {
    Left(A),
    Right(B),
}

impl<A> Kind<OptionKind> for Option<A> {
    type HKT = OptionKind;
    type Lifted = Option<A>;
    type Unlifted = A;
}

trait Kind<F: HKT> {
    type HKT: HKT;
    type Lifted;
    type Unlifted;
}

struct Lifted<F: HKT, A: Kind<F>> {
    __kind: F,
    __type: PhantomData<*const A::Lifted>,
}

impl Kind<Empty> for i32 {
    type HKT = Empty;
    type Lifted = i32;
    type Unlifted = i32;
}

use std::future::Future;
use std::future;
use std::mem::PinMut;
use core::task::Context;
use core::task::Poll;
use std::task::TaskObj;

pub trait Functor<F_: HKT> {
    fn map<Fn_, A, B>(a: Lifted<F_, A>, f: Fn_) -> Lifted<F_, B>
        where
            A: Kind<F_>,
            B: Kind<F_>,
            Fn_: Fn(A) -> B;
}

impl Functor<OptionKind> for OptionKind {
    fn map<Fn_, A, B>(a: Lifted<OptionKind, A>, f: Fn_) -> Lifted<OptionKind, B> where
        A: Kind<OptionKind>,
        B: Kind<OptionKind>,
        Fn_: Fn(A) -> B {
    }
}

impl Kind<OptionKind> for i32 {
    type HKT = OptionKind;
    type Lifted = Self;
    type Unlifted = Self;
}
#[test]
fn test() {
    let f = Some(1);
    let l = Lifted::<OptionKind, i32>{__type: PhantomData, __kind: OptionKind};
    OptionKind::map(l, |i| i * 2);
}