use lifted::Lifted;
use lifted::Nothing;
use lifted::HKT;
use monoid::Monoid;

pub trait Foldable<F_, Z = Nothing, G = Nothing>: HKT
where
    F_: HKT,
{
    fn fold_left<A, B, Func>(fa: Lifted<F_, A, Z, G>, acc: B, f: Func) -> B
    where
        Func: Fn(B, A) -> B;
    fn fold_right<A, B, Func>(fa: Lifted<F_, A, Z, G>, acc: B, f: &Func) -> B
    where
        Func: Fn(B, A) -> B;

    fn fold_m<A>(fa: Lifted<F_, A, Z, G>) -> A
    where
        A: Monoid<A>,
    {
        Self::fold_left(fa, A::empty(), A::combine)
    }
}
