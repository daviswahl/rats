pub trait Semigroup<A> {
    fn combine(x: A, y: A) -> A;
}
