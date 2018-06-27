
pub trait HKT {}

pub trait Kinded<K: HKT> {
    type Item;
    type Out;
}

pub struct Kind<K: HKT, T: Kinded<K>> {
    t: T::Out,
    k: K,
}

trait KindExt<T> {
    fn extract(self) -> T;
}

impl<T> KindExt<Vec<T>> for Kind<VecKind, T> {
    fn extract(self) -> Vec<T> {
        self.t
    }
}
pub struct VecKind;
impl HKT for VecKind{}

impl<T> Kinded<VecKind> for T {
    type Item = T;
    type Out = Vec<T>;
}

trait Functor<K: HKT> {
    fn map<A, B , F>(a: Kind<K,A>, f: F) -> Kind<K,B> where
    A: Kinded<K, Item=A>,
    B: Kinded<K, Item=B>,
    F: FnMut(A) -> B;
}

impl<T> Functor<VecKind> for T where T: Kinded<VecKind> {
    fn map<A, B, F>(a: Kind<VecKind, A>, f: F) -> Kind<VecKind, B> where
        A: Kinded<VecKind, Item=A>,
        B: Kinded<VecKind, Item=B>,
        F: FnMut(A) -> B {
        let f = a.extract();
        let result = f.into_iter().map(f).collect::<Vec<B>>();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let kind: Kind<VecKind, i32> = Kind { k: VecKind, t: vec![1,2,3]};
        //let r: Kind<VecKind, i32> = Functor::<VecKind>::map(kind, |i| i*2);


    }
}