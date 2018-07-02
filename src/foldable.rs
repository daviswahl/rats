use kind::{Kind, HKT};
pub trait Foldable<F_: HKT> {
    fn fold_right<Fn_, A, B>(fa: Kind<F_, A>, b: B, fn_: Fn_) -> B
    where
        Fn_: Fn((A, B)) -> B;
}
