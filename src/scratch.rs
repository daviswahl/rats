

use kind::{Kind, Empty, HKT};
use kinds::{VecKind,OptionKind};
use std::iter::FromIterator;


trait FromUnkind<'f_, F, Z = Empty> {
    type Out;
   fn from_unkind(f: F) -> Self::Out;
}

impl<'f_, A> FromUnkind<'f_, Vec<A>> for Vec<A>
where A: FromUnkind<'f_, A> + 'f_
{
    type Out = Kind<'f_, VecKind, A::Out>;
    fn from_unkind(f: Vec<A>) -> Self::Out {
        let mut v = vec![];
        for a in f {
           v.push(A::from_unkind(a));
        }
        Kind::Vec(v)
    }
}

impl <'f_, A> FromUnkind<'f_, Option<A>> for Option<A>
    where A: FromUnkind<'f_, A> + 'f_
{
    type Out = Kind<'f_, OptionKind, A::Out>;

    fn from_unkind(f: Option<A>) -> Self::Out {
       Kind::Option(f.map(|a| A::from_unkind(a)))
    }
}


trait Unkind {}
impl Unkind for i32 {}

impl<'f_, A> FromUnkind<'f_, A> for A where A: 'f_ + Unkind{
    type  Out = A;
    fn from_unkind(a: A) -> Self::Out {
       a
    }
}


trait IntoKind<'f_, F_: HKT, G_: HKT, Z = Empty>: Sized{
    type Out;
    fn into_kind(self) -> Self::Out;
}

impl<'f_, 'g_, G_: HKT, A> IntoKind<'f_, VecKind, G_> for Vec<A>
where A: FromUnkind<'g_, G_, A>,
      Vec<A>: FromUnkind<'f_, Vec<A>>,
{
    type Out = <Vec<A> as FromUnkind<'f_, Vec<A>>>::Out;
    fn into_kind(self) -> Self::Out {
        <Vec<A> as FromUnkind<'f_, Vec<A>>>::from_unkind(self)
    }
}

impl<'f_, 'g_, G_: HKT, A> IntoKind<'f_, OptionKind, G_> for Option<A>
where A: FromUnkind<'g_, G_, A>,
      Option<A>: FromUnkind<'f_, Option<A>>,
{
    type Out = <Option<A> as FromUnkind<'f_, Option<A>>>::Out;
    fn into_kind(self) -> Self::Out {
        <Option<A> as FromUnkind<'f_, Option<A>>>::from_unkind(self)
    }
}

impl<'f_,A> IntoKind<'f_, Empty, Empty> for A
where A: FromUnkind<'f_, A>,
{
    type Out = <A as FromUnkind<'f_, A>>::Out;
    fn into_kind(self) -> Self::Out {
        <A as FromUnkind<'f_, A>>::from_unkind(self)
    }
}
#[test]
fn test_from_unkind() {
    let f = vec![Some(Some(vec![1])), Some(Some(vec![2])), Some(Some(vec![3]))];
    f.into_kind();
}