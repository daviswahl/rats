use std::any::Any;
use std::marker::PhantomData;
use std::ops::Deref;
use v2::erased::Erased;

trait HKT {
    fn marker() -> Self;
}

struct VecK;

impl HKT for VecK {
    fn marker() -> VecK {
        VecK
    }
}

trait Kinded<K: HKT, T> {
    type Kind = K;
}

struct Kind<K, A>
where
    K: HKT,
{
    kind: K,
    _marker: PhantomData<*const A>,
    data: Erased,
}

impl<K, A> Kind<K, A>
where
    K: HKT,
{
    fn new<T>(k: T) -> Kind<K, A>
    where
        T: Kinded<K, A>,
    {
        Kind {
            kind: K::marker(),
            _marker: PhantomData,
            data: Erased::erase(k),
        }
    }

    unsafe fn unwrap<T: Sized>(self) -> T {
        self.data.unerase()
    }
}

trait Functor<K>
where
    K: HKT,
{
    fn map<F, A, B>(k: Kind<K, A>, f: F) -> Kind<K, B>
    where
        F: FnMut(A) -> B;
}

impl<T> Kinded<VecK, T> for Vec<T> {}

trait IntoKind<K: HKT, T>
where
    Self: Kinded<K, T>,
{
    fn into_kind(self) -> Kind<K, T>;
}

trait FromKind<K: HKT, T> {
    type Out: Kinded<K, T>;
    fn from_kind(k: Kind<K, T>) -> Self::Out;
}

impl<T> FromKind<VecK, T> for Vec<T> {
    type Out = Vec<T>;
    fn from_kind(k: Kind<VecK, T>) -> Vec<T> {
        unsafe { k.unwrap::<Self>() }
    }
}

impl<T> IntoKind<VecK, T> for Vec<T> {
    fn into_kind(self) -> Kind<VecK, T> {
        Kind::new(self)
    }
}

impl Functor<VecK> for VecK {
    fn map<F, A, B>(k: Kind<VecK, A>, f: F) -> Kind<VecK, B>
    where
        F: FnMut(A) -> B,
    {
        unsafe {
            let k: Vec<A> = k.unwrap();
            k.into_iter().map(f).collect::<Vec<B>>().into_kind()
        }
    }
}

trait FunctorExt<K: HKT> {
    type Item;

    fn map<B, F>(self, f: F) -> Kind<K, B>
    where
        F: FnMut(Self::Item) -> B;
}

impl<K: HKT, T> FunctorExt<K> for Kind<K, T>
where
    K: Functor<K>,
{
    type Item = T;

    fn map<B, F>(self, f: F) -> Kind<K, B>
    where
        F: FnMut(Self::Item) -> B,
    {
        <K as Functor<K>>::map(self, f)
    }
}

trait IntoKinded<K: HKT, T> {
    type Out: Kinded<K, T>;
    fn into_kinded(self) -> Self::Out;
}

impl<T> IntoKinded<VecK, T> for Kind<VecK, T> {
    type Out = Vec<T>;
    fn into_kinded(self) -> Self::Out {
        <Self::Out as FromKind<VecK, T>>::from_kind(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn into_kind() {
        let kind = vec![1, 2, 3].into_kind();
        assert_eq!(Vec::from_kind(kind), vec![1, 2, 3]);
    }
    #[test]
    fn test_functor() {
        let kind = vec![1, 2, 3].into_kind();

        let result = kind.map(|i| i * 2);
        assert_eq!(result.into_kinded(), vec![2, 4, 6]);
    }

    use test::Bencher;

    #[bench]
    fn bench_vec_map_native(b: &mut Bencher) {
        b.iter(|| {
            vec![1, 2, 3]
                .into_iter()
                .map(|i| i * 2)
                .collect::<Vec<i32>>()
        });
    }

    #[bench]
    fn bench_vec_map_from_functor(b: &mut Bencher) {
        b.iter(|| vec![1, 2, 3].into_kind().map(|i| i * 2).into_kinded());
    }

    #[bench]
    fn bench_vec_map_native_2(b: &mut Bencher) {
        b.iter(|| {
            vec![1, 2, 3]
                .into_iter()
                .map(|i| i * 2)
                .map(|i| i * 2)
                .collect::<Vec<i32>>()
        });
    }

    use test::black_box;
    #[bench]
    fn bench_vec_map_from_functor_2(b: &mut Bencher) {
        b.iter(|| {
            vec![1, 2, 3]
                .into_kind()
                .map(|i| i * 2)
                .map(|i| i * 2)
                .into_kinded()
        });
    }

    #[bench]
    fn bench_vec_map_from_functor_amortized_strings(b: &mut Bencher) {
        b.iter(|| {
            let t = vec![1,2,3,4,5].into_kind().map(|outer| {
                let n = black_box(1000);
                let range: Vec<i64> = (0..n).collect();
                range.into_kind()
                    .map(|i| format!("{}{}", outer, i))
                    .into_kinded()
            });
            let result = t.into_kinded();
        });
    }

    #[bench]
    fn bench_vec_map_native_amortized_strings(b: &mut Bencher) {
        b.iter(|| {
            let t = vec![1,2,3,4,5].into_iter().map(|outer| {
                let n = black_box(1000);
                let range: Vec<i64> = (0..n).collect();
                range.into_iter()
                    .map(|i| format!("{}{}", outer, i))
                    .collect::<Vec<String>>()
            });
            let result = t.collect::<Vec<Vec<String>>>();
            println!("{:?}", result)
        });
    }

    #[bench]
    fn bench_vec_map_from_functor_amortized_ints(b: &mut Bencher) {
        b.iter(|| {
            let t = vec![1,2,3,4,5].into_kind().map(|outer| {
                let n = black_box(1000);
                let range: Vec<i64> = (0..n).collect();
                range.into_kind()
                    .map(|i| i * outer)
                    .into_kinded()
            });
            let result = t.into_kinded();
        });
    }

    #[bench]
    fn bench_vec_map_native_amortized_ints(b: &mut Bencher) {
        b.iter(|| {
            let t = vec![1,2,3,4,5].into_iter().map(|outer| {
                let n = black_box(1000);
                let range: Vec<i64> = (0..n).collect();
                range.into_iter()
                    .map(|i| i * outer)
                    .collect::<Vec<i64>>()
            });
            let result = t.collect::<Vec<Vec<i64>>>();
            println!("{:?}", result)
        });
    }
}
