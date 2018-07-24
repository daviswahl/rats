use kind::{Empty, Kind, HKT};
use kinds::{OptionKind, VecKind};

pub trait Primitive {}

macro_rules! primitive {
    ( $( $element:ty ) , + ) => {
    $(
            impl Primitive for $element {}
            impl<'a> Primitive for &'a $element {}
            impl<'a> Primitive for &'a mut $element {}
    )+ };
}
primitive!(i32, i64, str);

macro_rules! array_primitives {
    ($($N:expr)+) => {
        $ (
            impl<T> Primitive for [T; $N] {}
        ) +
    };
}

array_primitives! {
     0  1  2  3  4  5  6  7  8  9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27 28 29
    30 31 32
}

struct PrimitiveKind {}
impl HKT for PrimitiveKind {
    type Kind = PrimitiveKind;
}

pub trait Unlift<K: HKT, A, F_: HKT = Empty> {
    type Out;
    type Lifted;
    fn unlift(self) -> Self::Out;
}

impl<A> Unlift<PrimitiveKind, A> for A
where
    A: Primitive,
{
    type Out = A;

    fn unlift(self) -> Self::Out {
        self
    }
    type Lifted = A;
}

impl<'f_, A, F_: HKT> Unlift<VecKind, A, F_> for Kind<'f_, VecKind, A>
where
    A: Unlift<F_, A>,
{

    type Out = Vec<A::Out>;
    fn unlift(self) -> Self::Out {
        match self {
            Kind::Vec(v) => v.into_iter().map(|k| k.unlift()).collect(),
            _ => unreachable!(),
        }
    }
    type Lifted = Kind<'f_, VecKind, A::Out>;
}

//impl<'f_, A> Unlift for Kind<'f_, OptionKind, A>
//where
//    A: Unlift,
//{
//    type Out = Option<A::Out>;
//    fn unlift(self) -> Self::Out {
//        match self {
//            Kind::Option(o) => o.map(|k| k.unlift()),
//            _ => unreachable!(),
//        }
//    }
//}
//
//use v1.kinds::ResultKind;
//impl<'f_, A, Z> Unlift for Kind<'f_, ResultKind, A, Z>
//where
//    Z: Unlift,
//    A: Unlift,
//{
//    type Out = Result<A::Out, Z::Out>;
//    fn unlift(self) -> Self::Out {
//        match self {
//            Kind::Result(r) => r.map(|k| k.unlift()).map_err(|k| k.unlift()),
//            _ => unreachable!(),
//        }
//    }
//}
pub trait Lift2<'f_> {
    type Out;
    fn lift2(self) -> Self::Out;
}

impl<'f_, A: 'f_> Lift2<'f_> for Vec<A>
where
    A: Lift2<'f_>,
{
    type Out = Kind<'f_, VecKind, A::Out>;
    fn lift2(self) -> Self::Out {
        Kind::Vec(self.into_iter().map(|k| k.lift2()).collect())
    }
}

//impl<'f_, A: 'f_> Lift2<'f_> for Option<A>
//where
//    A: Lift2<'f_>,
//{
//    type Out = Kind<'f_, OptionKind, A::Out>;
//    fn lift2(self) -> Self::Out {
//        Kind::Option(self.map(|k| k.lift2()))
//    }
//}
//
//impl<'f_, A: 'f_, E: 'f_> Lift2<'f_> for Result<A, E>
//where
//    A: Lift2<'f_>,
//    E: Lift2<'f_>,
//{
//    type Out = Kind<'f_, ResultKind, A::Out, E::Out>;
//    fn lift2(self) -> Self::Out {
//        Kind::Result(self.map(|k| k.lift2()).map_err(|e| e.lift2()))
//    }
//}

impl<'f_, A> Lift2<'f_> for A
where
    A: Primitive,
{
    type Out = A;
    fn lift2(self) -> A {
        self
    }
}


pub trait Functor<'f_, F_: HKT, Z = Empty>: HKT {
    /// (F<(A,)>, Fn(A) -> B) -> F<B,>
    fn map<Fn_, A, B>(a: Kind<'f_, F_, A, Z>, f: Fn_) -> Kind<'f_, F_, B::Out, Z>
        where
            Fn_: Fn(A::Out) -> B + 'f_,
            B: Lift2<'f_> + 'f_,
            A: Unlift<VecKind, A>;
}

pub mod lifted {
    use kind::Kind;
    use super::VecKind;
    pub type Vec<'f_,A> = Kind<'f_, VecKind, A>;
}

impl<'f_> Functor<'f_, VecKind> for VecKind {
    fn map<Fn_, A, B>(a: lifted::Vec<'f_, A>, f: Fn_) -> Kind<'f_, VecKind, B::Out> where
        A: Unlift<VecKind, A>,
        B: Lift2<'f_> + 'f_,
        Fn_: FnMut(A::Out) -> B + 'f_ {
        let mut f = f;
        a.unlift().into_iter().map(|a| f(a)).collect::<Vec<B>>().lift2()
    }
}

#[test]
fn test_lifted_ext() {}

use functor::KindFunctorExt;
#[test]
fn test_from_unkind() {
    //let r = Err::<&str, i32>(1);
    let expected = vec![vec![1,2,3]];
    let actual = expected.clone().lift2().map(|f| f).unlift();
    assert_eq!(actual, expected);
}
