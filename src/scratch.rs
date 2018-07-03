

use kind::{Kind, Empty, HKT};
use kinds::{VecKind, OptionKind};
use std::iter::FromIterator;


trait FromUnkind<'f_, F, Z = Empty> {
    type Out;
    fn from_unkind(f: F) -> Self::Out;
}

impl<'f_, A> FromUnkind<'f_, Vec<A>> for Vec<A>
where
    A: FromUnkind<'f_, A> + 'f_,
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

impl<'f_, A> FromUnkind<'f_, Option<A>> for Option<A>
where
    A: FromUnkind<'f_, A> + 'f_,
{
    type Out = Kind<'f_, OptionKind, A::Out>;

    fn from_unkind(f: Option<A>) -> Self::Out {
        Kind::Option(f.map(|a| A::from_unkind(a)))
    }
}


trait Primitive {}

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

impl<'f_, A> FromUnkind<'f_, A> for A
where
    A: 'f_ + Primitive,
{
    type Out = A;
    fn from_unkind(a: A) -> Self::Out {
        a
    }
}


trait IntoKind<'f_, F_: HKT, G_: HKT, Z = Empty>: Sized {
    type Out;
    fn into_kind(self) -> Self::Out;
}

impl<'f_, 'g_, G_: HKT, A> IntoKind<'f_, VecKind, G_> for Vec<A>
where
    A: FromUnkind<'g_, G_, A>,
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

impl<'f_, A> IntoKind<'f_, Empty, Empty> for A
where
    A: FromUnkind<'f_, A>,
{
    type Out = <A as FromUnkind<'f_, A>>::Out;
    fn into_kind(self) -> Self::Out {
        <A as FromUnkind<'f_, A>>::from_unkind(self)
    }
}

#[test]
fn test_from_unkind() {
    let f = Some(vec![Some(1)]);
    let f = f.into_kind();
    assert_eq!(
        f,
        Kind::Option(Some(Kind::Vec(vec![Kind::Option(Some(1))])))
    );
}
