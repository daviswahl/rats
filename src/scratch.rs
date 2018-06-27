use std::marker::PhantomData;
pub trait HKT {}

pub enum Kind<K: HKT, T> {
   Vec {
       t: Vec<T>,
       _marker: PhantomData<*const K>,
   },
    Option {
        t: Option<T>,
        _marker: PhantomData<*const K>,
    }
}

trait KindExtractor<K: HKT,T> {
    type Type;
    fn extract(self) -> Self::Type;
    fn new(t: Self::Type) -> Kind<K,T>;
}

#[allow(unreachable_patterns)]
impl<T> KindExtractor<VecKind, T> for Kind<VecKind, T> {
    type Type = Vec<T>;
    fn extract(self) -> Vec<T> {
        match self  {
           Kind::Vec{t,..} => t,
            _ => unreachable!()
        }
    }

    fn new(t: Vec<T>) -> Kind<VecKind, T> {
        Kind::Vec::<VecKind, T> {
            _marker: PhantomData,
            t
        }
    }
}

#[allow(unreachable_patterns)]
impl<T> KindExtractor<OptionKind, T> for Kind<OptionKind, T>{
    type Type = Option<T>;

    fn extract(self) -> Self::Type {
        match self {
            Kind::Option{t,..} => t,
            _ => unreachable!(),
        }
    }

    fn new(t: Self::Type) -> Kind<OptionKind, T> {
        Kind::Option {
            _marker: PhantomData,
            t
        }
    }
}
pub struct VecKind {}
impl HKT for VecKind{}
pub struct OptionKind {}
impl HKT for OptionKind {}

trait Functor<K: HKT> {
   fn map<A,B, F>(a: Kind<K, A>, f: F) -> Kind<K,B>
    where F: Fn(A) -> B;
}

impl Functor<VecKind> for VecKind {
    fn map<A, B, F>(a: Kind<VecKind, A>, f: F) -> Kind<VecKind, F::Output> where F: Fn(A) -> B {
        let v = a.extract().into_iter().map(f).collect::<Vec<B>>();
        Kind::new(v)
    }
}

trait KindFunctorExt<K: HKT> where K: Functor<K> {
    type Item;
    fn map<B,F>(self, f: F) -> Kind<K,B> where
    F: Fn(Self::Item) -> B;
}

impl<K, T> KindFunctorExt<K> for Kind<K, T> where K: HKT+Functor<K> {
    type Item = T;

    fn map<B, F>(self, f: F) -> Kind<K, B> where
        F: Fn(Self::Item) -> B {
        K::map(self, f)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test() {
        let k = Kind::<VecKind, i32>::new(vec![1,2,3]);
        let result = k.map(|i| i *2);
        assert_eq!(result.extract(), vec![2,4,6]);

        let k = Kind::<OptionKind, i32>::new(Some(1));
    }
    #[bench]
    fn bench_vec_map_from_functor_1(b: &mut Bencher) {
        b.iter(|| Kind::<VecKind, i32>::new(vec![1, 2, 3]).map(|i| i * 2).extract());
    }
}