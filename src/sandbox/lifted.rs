use data::id::Id;
use futures::future::Future;
use std::any::Any;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::marker::{PhantomData, Send, Sync};

pub trait HKT {
    type Kind: HKT;
}

pub trait AnyKind<A, B = Empty>: 'static {
    type Out;
    type Kind: HKT;
}



#[derive(Clone, Debug, PartialEq)]
pub struct Empty {}
impl HKT for Empty {
    type Kind = Empty;
}

#[allow(dead_code)]
pub enum Kind<'f_, F_: HKT, A, B = Empty>
where
    F_: HKT,
    A: 'f_,
    B: 'f_,
{
    Vec(Vec<A>),
    VecRef(&'f_ Vec<A>),
    Option(Option<A>),
    Id(Id<A>),
    Result(Result<A, B>),
    Future(&'f_ Future<Item = A, Error = B>),
    Any(&'f_ Any),
    // Is this valid? also need to understand which pointer type to use here
    __MARKER(PhantomData<*const F_>),
}

impl<'f_, F_, A, B> Clone for Kind<'f_, F_, A, B>
where
    F_: HKT,
    A: Clone,
    B: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Kind::Vec(ref v) => Kind::Vec(v.clone()),
            Kind::VecRef(v) => Kind::VecRef(v.clone()),
            Kind::Option(ref o) => Kind::Option(o.clone()),
            Kind::Id(ref id) => Kind::Id(id.clone()),
            Kind::Future(_) => unimplemented!(),
            Kind::Result(ref res) => Kind::Result(res.clone()),
            Kind::Any(_) => unimplemented!(),
            Kind::__MARKER(ref data) => Kind::__MARKER(data.clone()),
        }
    }
}


/// Vec Defs
///
///
#[derive(Clone, Debug, PartialEq)]
pub struct VecKind;
impl HKT for VecKind {
    type Kind = VecKind;
}

impl<'f_> HKT for &'f_ VecKind {
    type Kind = &'f_ VecKind;
}
pub trait Functor<'f_, F_: HKT, Z = Empty>: HKT {
    /// (F<(A,)>, Fn(A) -> B) -> F<B,>
    fn map<Fn_, A, B>(a: Kind<'f_, F_, A, Z>, f: Fn_) -> Kind<'f_, F_, B, Z>
        where
            Fn_: Fn(A) -> B + 'f_;
}

impl<'f_> Functor<'f_, VecKind> for VecKind {
    fn map<F, A, B>(a: Kind<'f_, VecKind, A>, f: F) -> Kind<'f_, VecKind, B>
        where
            F: FnMut(A) -> B + 'f_,
    {
        a.unlift().into_iter().map(f).collect::<Vec<B>>().lift()
    }
}

//impl<'f_> Functor<'f_, &'f_ VecKind> for &'f_ VecKind {
//    fn map<F, A, B>(a: Kind<'f_, &'f_ VecKind, A>, f: F) -> Kind<'f_, &'f_ VecKind, B>
//        where
//            F: FnMut(&A) -> B + 'f_,
//    {
//        a.unlift().into_iter().map(f).collect::<Vec<B>>().lift()
//    }
//}

/// Option Defs
#[derive(Clone, Debug, PartialEq)]
pub struct OptionKind;
impl HKT for OptionKind {
    type Kind = OptionKind;
}


#[test]
fn reification_test() {
    let f = vec![Some(1), Some(2), Some(3)].lift_2();
    let f = VecKind::map(f, |i| i.unlift().map(|i| i * 2).lift());
    //assert_eq!(f.unlift(), vec![2, 4, 6]);

    let f = vec![Some(1)].lift_2();
    //assert_eq!(f.unlift_2(), Some(1))

    let f = vec![1,2,3];

    let r = &f;
    //let g: () = r.lift().map(|i| i *2);

}

pub trait Lift<'f_, F_: HKT> {
    type Kind: HKT;
    type Lifted;
    fn lift(self) -> Self::Lifted;
}

impl<'f_, A> Lift<'f_, &'f_ VecKind> for &'f_ Vec<A> {
    type Kind = &'f_ VecKind;
    type Lifted = Kind<'f_, &'f_ VecKind, A>;

    fn lift(self) -> Self::Lifted {
        Kind::VecRef(self)
    }
}

pub trait Lift2<'f_, 'g_, F_, G_>
where
    F_: HKT,
    G_: HKT,
{
    type Lifted2;
    fn lift_2(self) -> Self::Lifted2;
}

impl<'f_, 'g_, G_, A> Lift2<'f_, 'g_, VecKind, G_> for Vec<A>
where
    G_: HKT + 'f_,
    A: Lift<'g_, G_>,
    A::Lifted: 'f_,
    A: 'f_,
    'g_: 'f_
{
    type Lifted2 = Kind<'f_, VecKind, A::Lifted>;

    fn lift_2(self) -> Self::Lifted2 {
        let mut tmp = Vec::with_capacity(self.len());
        for k in self.into_iter() {
            tmp.push(A::lift(k))
        }
        Vec::lift(tmp)
    }
}

pub trait Unlift<'f_, F_: HKT> {
    type Kind: HKT;
    type Unlifted;
    fn unlift(self) -> Self::Unlifted;
}

pub trait Unlift2<'f_, 'g_, F_: HKT, G_: HKT> {
    type Kind: HKT;
    type Unlifted;
    fn unlift_2(self) -> Self::Unlifted;
}

impl<'f_, 'g_, G_: HKT, A> Unlift2<'f_,'g_, VecKind,G_> for Kind<'f_, VecKind, A>
where A: Unlift<'g_, G_> {
    type Kind = VecKind;
    type Unlifted = Vec<A::Unlifted>;

    fn unlift_2(self) -> Self::Unlifted {
        self.unlift().into_iter().map(|f| f.unlift()).collect()
    }
}

impl<'f_, 'g_, G_: HKT, A> Unlift2<'f_,'g_, OptionKind, G_> for Kind<'f_, OptionKind, A>
    where A: Unlift<'g_, G_> {
    type Kind = VecKind;
    type Unlifted = Vec<A::Unlifted>;

    fn unlift_2(self) -> Self::Unlifted {
        self.unlift().into_iter().map(|f| f.unlift()).collect()
    }
}

impl<'f_, A: 'f_> Lift<'f_, VecKind> for Vec<A> {
    type Kind = VecKind;
    type Lifted = Kind<'f_, VecKind, A>;
    fn lift(self) -> Self::Lifted {
        Kind::Vec(self)
    }
}

impl<'f_, A: 'f_> Lift<'f_, OptionKind> for Option<A> {
    type Kind = OptionKind;
    type Lifted = Kind<'f_, OptionKind, A>;

    fn lift(self) -> Self::Lifted {
        Kind::Option(self)
    }
}


impl<'f_, A: 'f_> Unlift<'f_, OptionKind> for Kind<'f_, OptionKind, A> {
    type Kind = OptionKind;
    type Unlifted = Option<A>;
    fn unlift(self) -> Self::Unlifted {
        match self {
            Kind::Option(v) => v,
            _ => unreachable!(),

        }
    }
}

impl<'f_, A: 'f_> Unlift<'f_, VecKind> for Kind<'f_, VecKind, A> {
    type Kind = VecKind;
    type Unlifted = Vec<A>;
    fn unlift(self) -> Self::Unlifted {
        match self {
            Kind::Vec(v) => v,
            _ => unreachable!(),

        }
    }
}

impl<'f_, A: 'f_> Unlift<'f_, &'f_ VecKind> for Kind<'f_, &'f_ VecKind, A> {
    type Kind = &'f_ VecKind;
    type Unlifted = &'f_ Vec<A>;
    fn unlift(self) -> Self::Unlifted {
        match self {
            Kind::VecRef(v) => v,
            _ => unreachable!(),

        }
    }
}