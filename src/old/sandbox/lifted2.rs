//use old.v1.data::id::Id;
//use futures::future::Future;
use std::any::Any;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::marker::{PhantomData, Send, Sync};

pub trait HKT: 'static + Debug {
    type Kind: HKT;
}

//pub trait AnyKind<A, B = Empty>: 'static {
//    type Out;
//    type Kind: HKT;
//}

#[derive(Clone, Debug, PartialEq)]
pub struct Empty {}
impl HKT for Empty {
    type Kind = Empty;
}

type OptionT<'f_, 'g_, F_, A>= Kind<'f_, OptionTKind, Kind<'f_, F_, Kind<'g_, OptionKind, A>>>;

//pub enum EitherK<'f_, F_: HKT, G_: HKT, A, B = Empty>
//where F_: 'static,
//      G_: 'static,
//      A: 'f_,
//      B: 'f_
//{
//   A(Kind<'f_, F_, A, B>),
//   B(Kind<'f_, G_, A, B>)
//}

#[derive(Debug)]
struct Meta<'f_,F_: HKT, G_: HKT, A: 'f_, B: 'f_> {
    inner: &'f_ Kind<'f_, G_, A, B>,
    __marker: PhantomData<*const F_>,
}

type Kind2<'f_, F_, G_, A, B = Empty> = Kind<'f_, F_, A, B, G_>;

type Kind<'f_, F_: HKT, A, B = Empty, G_: HKT = Empty> = Lifted<'f_, F_, G_, A, B>;
#[allow(dead_code)]
pub enum Lifted<'f_, F_, G_, A, B>
where
    F_: HKT,
    G_: HKT,
    A: 'f_,
    B: 'f_,
{
    Vec(Vec<A>),
    VecRef(&'f_ Vec<A>),
    Option(Option<A>),
    //Id(Id<A>),
    //EitherK(&'f_ EitherK<'f_, F_, G_, A, B>),
    Result(Result<A, B>),
   // Future(&'f_ Future<Item = A, Error = B>),
    Any(&'f_ Any),
    Meta(Meta<'f_, F_, G_, A, B>),
    // Is this valid? also need to understand which pointer type to use here
    __MARKER_F(PhantomData<*mut F_>),
}

impl<'f_, F_: HKT, A, B> Kind<'f_, F_, A, B> {
    fn lift_kind<G_: HKT>(&'f_ self) -> Kind<'f_, G_, A, B, F_> {
        Lifted::Meta(Meta {
            inner: self,
            __marker: PhantomData,
        })
    }
}

impl<'f_, F_, A, B> Debug for Kind<'f_, F_, A, B>
where F_: HKT, A: Debug, B: Debug
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
           Lifted::Vec(v) => write!(f, "Kind[{}, {:?}]", "VecKind", v),
            Lifted::Option(o) => write!(f, "Kind[{}, {:?}]", "OptionKind", o),
            Lifted::Meta(k) => write!(f, "Kind[{}, {:?}]", "Kind", k),
           _ => unimplemented!("formatter"),
        }
    }
}
//impl<'f_, 'g_, F_, A, B> Clone for Kind<'f_, F_, A, B>
//where
//    F_: HKT,
//    A: Clone,
//    B: Clone,
//{
//    fn clone(&self) -> Self {
//        match self {
//            Kind::Vec(ref v) => Kind::Vec(v.clone()),
//            Kind::VecRef(v) => Kind::VecRef(v.clone()),
//            Kind::Option(ref o) => Kind::Option(o.clone()),
//            Kind::Id(ref id) => Kind::Id(id.clone()),
//            Kind::Future(_) => unimplemented!(),
//            Kind::EitherK(ref k) => Kind::EitherK(k.clone()),
//            Kind::OptionT(ref k) => unimplemented!(),
//            Kind::Result(ref res) => Kind::Result(res.clone()),
//            Kind::Any(_) => unimplemented!(),
//            Kind::__MARKER(ref old.v1.data) => Kind::__MARKER(old.v1.data.clone()),
//            Kind::__MARKER2(ref old.v1.data) => Kind::__MARKER2(old.v1.data.clone()),
//        }
//    }
//}


/// Vec Defs
///
///
#[derive(Clone, Debug, PartialEq)]
pub struct VecKind {}
impl HKT for VecKind {
    type Kind = VecKind;
}
#[derive(Clone, Debug, PartialEq)]
pub struct OptionTKind {}
impl HKT for OptionTKind {
    type Kind = OptionTKind;
}

impl HKT for &'static VecKind {
    type Kind = &'static VecKind;
}

pub trait Functor<'f_,F_: HKT, Z = Empty>: HKT {
    /// (F<(A,)>, Fn(A) -> B) -> F<B,>
    fn map<Fn_, A, B>(a: Kind<'f_, F_, A, Z>, f: Fn_) -> Kind<'f_,  F_, B, Z>
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

impl <'f_> Functor<'f_, OptionTKind> for OptionTKind {
    fn map<Fn_, A, B>(a: Kind<OptionTKind, A, Empty>, f: Fn_) -> Kind<OptionTKind, B> where
        Fn_: Fn(A) -> B + 'f_ {
        unimplemented!()
    }
}
/// Option Defs
#[derive(Clone, Debug, PartialEq)]
pub struct OptionKind;
impl HKT for OptionKind {
    type Kind = OptionKind;
}

#[test]
fn nested_kind_test() {

}


#[test]
fn reification_test() {
    let k = vec![Some(1)].lift_2();

    let k = k.lift_kind::<OptionTKind>();

    println!("{:?}", k);

   // let f = VecKind::map(f, |i| i * 2);
    //assert_eq!(f.unlift(), vec![2, 4, 6]);

    //assert_eq!(f.unlift_2(), Some(1))

   // let f = vec![1,2,3];

   // let r = &f;
    //let g: () = r.lift().map(|i| i *2);

}

pub trait Lift<'f_, F_: HKT> {
    type Kind: HKT;
    type Lifted;
    fn lift(self) -> Self::Lifted;
}

impl<'f_, A: 'f_> Lift<'f_, &'static VecKind> for &'f_ Vec<A> {
    type Kind = &'static VecKind;
    type Lifted = Kind<'f_, &'static VecKind, A>;

    fn lift(self) -> Self::Lifted {
        Lifted::VecRef(self)
    }
}

pub trait Unlift<'f_, F_: HKT> {
    type Kind: HKT;
    type Unlifted;
    fn unlift(self) -> Self::Unlifted;
}

impl<'f_, A: 'f_> Lift<'f_, VecKind> for Vec<A> {
    type Kind = VecKind;
    type Lifted = Kind<'f_, VecKind, A>;
    fn lift(self) -> Self::Lifted {
        Lifted::Vec(self)
    }
}

impl<'f_, A: 'f_> Lift<'f_, OptionKind> for Option<A> {
    type Kind = OptionKind;
    type Lifted = Kind<'f_, OptionKind, A>;

    fn lift(self) -> Self::Lifted {
        Lifted::Option(self)
    }
}

impl<'f_,A: 'f_> Unlift<'f_, OptionKind> for Kind<'f_, OptionKind, A> {
    type Kind = OptionKind;
    type Unlifted = Option<A>;
    fn unlift(self) -> Self::Unlifted {
        match self {
            Lifted::Option(v) => v,
            _ => unreachable!(),

        }
    }
}

impl<'f_, A: 'f_> Unlift<'f_,  VecKind> for Kind<'f_, VecKind, A> {
    type Kind = VecKind;
    type Unlifted = Vec<A>;
    fn unlift(self) -> Self::Unlifted {
        match self {
            Lifted::Vec(v) => v,
            _ => unreachable!(),

        }
    }
}

impl<'f_, 'g_, A: 'f_> Unlift<'f_, &'static VecKind> for Kind<'f_, &'static VecKind, A> {
    type Kind = &'static VecKind;
    type Unlifted = &'f_ Vec<A>;
    fn unlift(self) -> Self::Unlifted {
        match self {
            Lifted::VecRef(v) => v,
            _ => unreachable!(),

        }
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


