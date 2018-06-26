use serde;
use serde::de;
use v1::context::{Context, ExtractExt, IntoContextExt};
use v1::kind::Kind;
use v1::kinds;

trait Functor<K>
where
    K: Kind,
{
    fn map<A, B, F>(k: Context<K, A>, f: F) -> Context<K, B>
    where
        F: FnMut(A) -> B,
        A: de::DeserializeOwned,
        B: de::DeserializeOwned + serde::Serialize;
}

trait KindFunctorExt<K: Kind> {
    type Item;

    fn map<B, F>(self, f: F) -> Context<K, B>
    where
        F: FnMut(Self::Item) -> B,
        B: de::DeserializeOwned + serde::Serialize;
}

impl<K: Kind + Functor<K>, T: de::DeserializeOwned> KindFunctorExt<K> for Context<K, T> {
    type Item = T;

    fn map<B, F>(self, f: F) -> Context<K, B>
    where
        F: FnMut(Self::Item) -> B,
        B: de::DeserializeOwned + serde::Serialize,
    {
        <K as Functor<K>>::map::<T, B, F>(self, f)
    }
}

impl Functor<kinds::Vec> for kinds::Vec {
    fn map<A, B, F>(m: Context<kinds::Vec, A>, f: F) -> Context<kinds::Vec, B>
    where
        F: FnMut(A) -> B,
        A: de::DeserializeOwned,
        B: serde::Serialize + de::DeserializeOwned,
    {
        m.extract()
            .into_iter()
            .map(f)
            .collect::<Vec<B>>()
            .into_context()
    }
}

impl Functor<kinds::Option> for kinds::Option {
    fn map<A, B, F>(k: Context<kinds::Option, A>, f: F) -> Context<kinds::Option, B>
    where
        F: FnMut(A) -> B,
        A: de::DeserializeOwned,
        B: de::DeserializeOwned + serde::Serialize,
    {
        k.extract().map(f).into_context()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_functor_test() {
        let k: Context<kinds::Vec, i32> = Context::from(vec![1, 2, 3]);
        let result1 = k.map(|i| i * 2).map(|i| format!("{}", i));
        assert_eq!(result1.clone().extract(), vec!["2", "4", "6"]);
    }

    #[test]
    fn option_funtor_test() {
        let opt: Context<kinds::Option, i32> = Context::from(Some(1));
        let opt = opt.map(|i| i * 2);
        assert_eq!(opt.extract(), Some(2));
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
        b.iter(|| Context::from(vec![1, 2, 3]).map(|i| i * 2).extract());
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

    #[bench]
    fn bench_vec_map_from_functor_2(b: &mut Bencher) {
        b.iter(|| {
            Context::from(vec![1, 2, 3])
                .map(|i| i * 2)
                .map(|i| i * 2)
                .extract()
        });
    }
}
