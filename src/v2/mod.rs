pub mod hkt;

pub mod erased;

pub mod conversions;
pub mod functor;

pub mod instances;
pub mod kinds;

#[cfg(test)]
mod tests {
    use test::Bencher;
    use v2::conversions::*;
    use v2::functor::{Functor, FunctorExt};
    use v2::instances::vec::*;
    use v2::kinds::vec::VecK;

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
            let t = vec![1, 2, 3, 4, 5].into_kind().map(|outer| {
                let n = black_box(1000);
                let range: Vec<i64> = (0..n).collect();
                range
                    .into_kind()
                    .map(|i| format!("{}{}", outer, i))
                    .into_kinded()
            });
            let result = t.into_kinded();
            result
        });
    }

    #[bench]
    fn bench_vec_map_native_amortized_strings(b: &mut Bencher) {
        b.iter(|| {
            let t = vec![1, 2, 3, 4, 5].into_iter().map(|outer| {
                let n = black_box(1000);
                let range: Vec<i64> = (0..n).collect();
                range
                    .into_iter()
                    .map(|i| format!("{}{}", outer, i))
                    .collect::<Vec<String>>()
            });
            let result = t.collect::<Vec<Vec<String>>>();
            result
        });
    }

    #[bench]
    fn bench_vec_map_from_functor_amortized_ints(b: &mut Bencher) {
        b.iter(|| {
            let t = vec![1, 2, 3, 4, 5].into_kind().map(|outer| {
                let n = black_box(1000);
                let range: Vec<i64> = (0..n).collect();
                range.into_kind().map(|i| i * outer).into_kinded()
            });
            let result = t.into_kinded();
            result
        });
    }

    #[bench]
    fn bench_vec_map_native_amortized_ints(b: &mut Bencher) {
        b.iter(|| {
            let t = vec![1, 2, 3, 4, 5].into_iter().map(|outer| {
                let n = black_box(1000);
                let range: Vec<i64> = (0..n).collect();
                range.into_iter().map(|i| i * outer).collect::<Vec<i64>>()
            });
            let result = t.collect::<Vec<Vec<i64>>>();
            result
        });
    }
}
