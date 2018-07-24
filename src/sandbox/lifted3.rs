//use v1.data::id::Id;
//use futures::future::Future;
use std::marker::PhantomData;
trait HKT: 'static {}

struct Empty {}
impl HKT for Empty {}

struct IdKind {}
impl HKT for IdKind {}
struct OptionTKind {}
impl HKT for OptionTKind {}
struct OptionKind {}
impl HKT for OptionKind {}

struct EitherKKind {}
impl HKT for EitherKKind {}
struct EitherKind {}
impl HKT for EitherKind {}
use std::iter::Iterator;
enum Either<A,B> {
    Left(A),
    Right(B),
}


//enum EitherK<'f_, Left, Right, A, B = Empty> where Left: HKT, Right: HKT, A: 'f_, B: 'f_ {
//   Left(&'f_ Lifted<'f_, Left,A, B>),
//   Right(&'f_ Lifted<'f_, Right, A, B>),
//    __LEFT(PhantomData<*const Left>),
//    __RIGHT(PhantomData<*const Right>)
//}
//
//impl<'f_, Left, Right, A, B> EitherK<'f_, Left, Right, A, B> where Left: HKT, Right: HKT, A: 'f_, B: 'f_ {
//    fn right<LeftK: HKT>(l: &'f_ Lifted<'f_, Right, A, B>) -> EitherK<'f_, LeftK, Right, A, B> {
//        EitherK::Right(l)
//    }
//}

struct NoKind {}
impl HKT for NoKind {}

impl Kind for i32 {
    type Kind = NoKind;
    type Lifted = i32;
    type Item = i32;
}

impl<A> Kind for Vec<A> where A: Kind {
    type Kind = VecKind;
    type Lifted = Lifted<VecKind, A>;
    type Item = A;
}

struct VecKind;
impl HKT for VecKind {}

trait Kind {
    type Kind: HKT;
    type Lifted;
    type Item;
}

impl Kind for Empty {
    type Kind = Empty;
    type Lifted = Empty;
    type Item = Empty;
}


impl<A,B> Kind for Either<A,B> where A: Kind, B: Kind{
    type Kind = EitherKind;
    type Lifted = Lifted<EitherKind, A, B>;
    type Item = ();
}

enum Lifted<F: HKT, A, B = Empty> where A: Kind, B: Kind {
    Either(Either<A::Lifted, B::Lifted>),
    Vec(Vec<A::Lifted>),
    __marker(PhantomData<*const F>)
}


type KindOp<F: HKT, A, B, Args> = Fn(Args) -> Lifted<F, A, B>;

struct LazyKind<K: HKT, A> {
    lifted: Lifted<K, A>,
    __marker: K,
}

trait Functor<F: HKT> {
    fn map<A, B, Func>(fa: LazyKind<F, A>, func: Func) -> LazyKind<F,B> {
    }
}

impl<K: HKT, A> LazyKind<K,A> {
    fn apply<Args, B, F: HKT, H: HKT>(self, t: ) -> LazyKind<H, B> {

    }
}


#[test]
fn test() {
    let r: Lifted<VecKind, i32> = Lifted::Vec(vec![1,2,3]);

    let r = VecKind::fmap::<i32, i32, _>(r, |i| i * 2);
    let mut r = match r {
        Lifted::Vec(v) => v,
        _ => unimplemented!()
    };
    //r.into_iter().map(|i| i * 2).collect::<Vec<i32>>();


}

impl<F_: HKT, A, B> Kind for Lifted<F_, A, B> where A: Kind, B: Kind {
    type Kind = F_;
    type Lifted = Self;
    type Item = Self;
}

