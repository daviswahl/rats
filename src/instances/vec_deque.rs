use applicative::Applicative;
use foldable::Foldable;
use functor::Functor;
use lifted::*;
use std::collections::VecDeque;
use traverse::Traverse;

pub struct VecDequeKind;
impl HKT for VecDequeKind {}

impl<'a, A> Lift<'a, VecDequeKind, A> for VecDeque<A> {
    fn lift(self) -> Lifted<'a, VecDequeKind, A> {
        Lifted::VecDeque(self)
    }
}

impl<'a, A> Unlift<VecDequeKind> for Lifted<'a, VecDequeKind, A> {
    type Out = VecDeque<A>;

    fn unlift(self) -> <Self as Unlift<VecDequeKind>>::Out {
        match self {
            Lifted::VecDeque(a) => a,
            _ => unimplemented!(),
        }
    }
}

impl<'a> Functor<'a, VecDequeKind> for VecDequeKind {
    fn map<Func: 'a, A, B>(
        fa: Lifted<'a, VecDequeKind, A, Nothing, Nothing>,
        func: Func,
    ) -> Lifted<'a, VecDequeKind, B, Nothing, Nothing>
    where
        Func: Fn(A) -> B,
    {
        fa.unlift()
            .into_iter()
            .map(|f| func(f))
            .collect::<VecDeque<B>>()
            .lift()
    }
}

impl<'a> Foldable<VecDequeKind> for VecDequeKind {
    fn fold_left<A, B, Func>(fa: Lifted<VecDequeKind, A>, acc: B, func: Func) -> B
    where
        Func: Fn(B, A) -> B,
    {
        let mut tail = fa.unlift();
        let mut acc = acc;
        while let Some(head) = tail.pop_front() {
            acc = func(acc, head)
        }
        acc
    }

    fn fold_right<A, B, Func>(fa: Lifted<VecDequeKind, A>, acc: B, func: &Func) -> B
    where
        Func: Fn(B, A) -> B,
    {
        let mut acc = acc;
        let mut tail = fa.unlift();
        if let Some(head) = tail.pop_front() {
            acc = func(Self::fold_right(tail.lift(), acc, func), head)
        }
        acc
    }
}

impl<'a> Traverse<'a, VecDequeKind> for VecDequeKind {
    fn traverse<'g, Func, A, B, Z2, G2, H>(
        fa: Lifted<VecDequeKind, A>,
        func: Func,
    ) -> Lifted<'g, G2, Lifted<'a, VecDequeKind, B>, Z2, H>
    where
        G2: Applicative<'g, G2, Z2, H>,
        Func: Fn(A) -> Lifted<'g, G2, B, Z2, H>,
    {
        let acc = G2::point(VecDeque::new().lift());
        VecDequeKind::fold_right(fa, acc, &|acc, a| {
            G2::map2(func(a), acc, |(a, b)| {
                // need to add a direct push method
                let mut v = b.unlift();
                v.push_back(a);
                v.lift()
            })
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lift() {
        let mut v = VecDeque::new();
        v.push_front("foo");
        v.push_front("bar");

        v.lift().unlift();
    }

    #[test]
    fn fold_left() {
        let mut v = VecDeque::new();
        v.push_back(1);
        v.push_back(2);
        v.push_back(3);

        let result = VecDequeKind::fold_left(v.lift(), 0, |acc, i| acc - i);
        assert_eq!(result, -6);
    }

    #[test]
    fn fold_right() {
        let mut v = VecDeque::new();
        v.push_back(1);
        v.push_back(2);
        v.push_back(3);

        let result = VecDequeKind::fold_right(v.lift(), 0, &|acc, i| acc - i);
        assert_eq!(result, -6);
    }

    #[test]
    fn fold_m() {
        use monoid;
        let mut v = VecDeque::new();
        v.push_back(1);
        v.push_back(2);
        v.push_back(3);

        let result = VecDequeKind::fold_m(v.lift());
        assert_eq!(result, 6)
    }

    //#[test]
    fn blows_stack() {
        let mut vc = VecDeque::new();
        for i in 0..10000 {
            vc.push_back(i)
        }

        use std::panic;
        let result =
            panic::catch_unwind(|| VecDequeKind::fold_right(vc.lift(), 0, &|acc, i| acc + i));
    }
}
