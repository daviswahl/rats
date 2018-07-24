use kind::{Empty, Kind, HKT};
use kinds::{OptionKind, VecKind};


trait Lifted<K: HKT, A> {
    type Unlifted;
    type Lifted;

    fn unlift_vec(self) -> Self::Unlifted where K: HKT<Kind=VecKind>;
    fn lift_vec(v: Vec<A>) -> Option<Self::Lifted> where K: HKT<Kind=VecKind>, Self: Sized;

    fn unlift_option(self) -> Option<Option<A>> where K: HKT<Kind=OptionKind>;
    fn lift_option(o: Option<A>) -> Option<Self> where K: HKT<Kind=OptionKind>, Self: Sized;
}

impl<A> Lifted<OptionKind,A> for Option<A> {
    type Lifted = Option<A>;
    type Unlifted = A;

    fn unlift_option(self) -> Option<Option<A>> where {
        Some(self)
    }

    fn lift_option(l: Option<A>) -> Option<Self> {
        Some(l)
    }

    fn unlift_vec(self) -> Self::Unlifted where {
        None
    }

    fn lift_vec(v: Vec<A>) -> Option<Self> {
        None
    }
}

impl<A> Lifted<VecKind,A> for Vec<A> {
    type Lifted = Vec<A>;
    type Unlifted = A;

    fn unlift_vec(self) ->  Self::Unlifted where {
        Some(self)
    }

    fn lift_vec(v: Vec<A>) -> Option<Self> {
        Some(v)
    }
    fn unlift_option(self) -> Option<Option<A>> {
        None
    }

    fn lift_option(o: Option<A>) -> Option<Self> {
        None
    }
}

impl<A>Lifted<Empty, A> for A {
    default type Lifted = ();
    default type Unlifted = ();

    default fn unlift_vec(self) -> Option<Vec<A>> {
        None
    }

    default fn lift_vec(v: Vec<A>) -> Option<Self> {
        None
    }

    default fn unlift_option(self) -> Option<Option<A>> {
        None
    }

    default fn lift_option(o: Option<A>) -> Option<Self> {
        None
    }
}

trait Functor<K: HKT> {
    fn map2<A,B,F>(a: A::Lifted, f: F) -> B::Lifted where
    A: Lifted<K, A>,
    B: Lifted<K, B>,
    F: Fn(A::Unlifted) -> B::Unlifted;
}

impl Functor<VecKind> for VecKind {
    fn map2<A, B, F>(a: A::Lifted, f: F) -> B::Lifted where
        A: Lifted<VecKind, A>,
        B: Lifted<VecKind, B>,
        F: FnMut(A::Unlifted) -> B::Unlifted {
        A::Lifted::<VecKind, A>::unlift_vec(a)
        //B::lift_vec(r)
        //let f: Vec<A>= A::unlift_vec(a).unwrap(); //.into_iter().map(f).collect()

    }
}

impl Functor<OptionKind> for OptionKind {
    fn map2<A, B, F>(a: A::Lifted, f: F) -> B::Lifted where
        A: Lifted<OptionKind, A>,
        B: Lifted<OptionKind, B>,
        F: FnMut(A::Unlifted) -> B::Unlifted {
 //       let r = <A as Lifted<VecKind, A>>::unlift_vec(a).into_iter().map(f).collect::<Vec<B>>();
 //       <B as Lifted<VecKind, B>>::lift_vec(r)
    }
}
#[test]
fn test_lifter() {
    //let v = VecKind::lift_vec(vec![1,2,3]);

    let r = VecKind::map2(Some(1), |i: i32| i);
    VecKind::unlift_vec(r);

}
