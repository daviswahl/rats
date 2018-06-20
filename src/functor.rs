struct Context<C, T>
where C: Container<Kind=C,Item=T> {
    inner: C,
}

trait Container {
    type Kind;
    type Item;
}

use std;
struct Functor;

impl<C> FunctorTypeClass<C> for Functor
where C: Container<Kind=C> {
    fn fmap<C2, F, B>(ctx: Context<C, <C as Container>::Item>, f: F) -> Context<C2, B> where F: Fn(C::Item) -> B,
                                                                                             C2: Container<Kind=C2, Item=B> {

    }
}

trait FunctorTypeClass<C> where C: Container<Kind=C> {
    fn fmap<C2, F, B>(ctx: Context<C,C::Item>, f: F) -> Context<C2, B>
        where F: Fn(C::Item) -> B,
        C2: Container<Kind=C2,Item=B>;
}

impl<T> Container for Vec<T> {
    type Kind = Vec<T>;
    type Item = T;
}


#[cfg(test)]
mod test {
    use super::*;
    use std::vec::Vec;

    #[test]
    fn functor_test() {
        let ctx = Context{inner: vec![1,2,3]};
        assert_eq!(ctx.inner, vec![1,2,3]);
        let f = Functor::fmap(ctx, |b| "foo");
        assert!(f.inner == vec![2,4,6])
    }
}
