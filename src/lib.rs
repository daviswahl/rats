#![feature(test)]

extern crate test;

pub mod function_k;
pub mod functor;
pub mod hkt;
pub mod instances;
pub mod kind;
pub mod kinds;
pub mod scratch;

#[cfg(test)]
mod tests {
    use functor::KindFunctorExt;
    use kind::KindExt;
    use kind::Kinded;
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
    fn bench_vec_map_from_functor_1(b: &mut Bencher) {
        b.iter(|| vec![1, 2, 3].into_kind().map(|i| i * 2).reify());
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
                .reify()
        });
    }

    #[bench]
    fn bench_vec_map_from_functor_amortized_strings(b: &mut Bencher) {
        b.iter(|| {
            let t = vec![1, 2, 3, 4, 5].into_kind().map(|outer| {
                let n = black_box(1000);
                let range: Vec<i64> = (0..n).collect();
                range.into_kind().map(|i| format!("{}{}", outer, i)).reify()
            });
            let result = t.reify();
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
                range.into_kind().map(|i| i * outer).reify()
            });
            let result = t.reify();
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
