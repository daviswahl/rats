use kind::{Kind, HKT};
pub trait Foldable<F: HKT> {
    fn fold_right<Func, A, B>(fa: Kind<F, A>, acc: B, f: Func) -> B
    where
        Func: Fn((A, B)) -> B;
}
//override def foldRight[B](z: B)(f: (A, B) => B): B = this match {
//  case Nil => z
//  case x :: xs => f(x, xs.foldRight(z)(f))
//}
