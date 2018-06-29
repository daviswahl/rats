#![feature(test)]

extern crate test;

pub mod function_k;
pub mod functor;
pub mod instances;
pub mod kind;
pub mod applicative;
pub mod kinds;
pub mod id;

#[cfg(test)]
mod tests {
    use function_k::KindFunctionKExt;
    use functor::KindFunctorExt;
    use kind::{IntoKind,Reify};
    use kinds::VecKind;
    use test::{black_box, Bencher};

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
        b.iter(|| vec![1, 2, 3].into_kind().map(|i| i * 2).reify());
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

    #[bench]
    fn bench_option_map_kind(b: &mut Bencher) {
        b.iter(|| {
            let n = black_box(1000);
            (0..n).into_iter().map(|i| {
                if i % 2 == 0 {
                    None.into_kind().map_kind::<VecKind>()
                } else {
                    Some(i).into_kind().map_kind::<VecKind>()
                }
            })
        })
    }

    fn native_convert(f: Option<i32>) -> Vec<i32> {
        match f {
            Some(i) => vec![i],
            None => vec![],
        }
    }
    #[bench]
    fn bench_option_to_vec_native(b: &mut Bencher) {
        b.iter(|| {
            let n = black_box(1000);
            (0..n).into_iter().map(|i| {
                if i % 2 == 0 {
                    native_convert(None)
                } else {
                    native_convert(Some(i))
                }
            })
        })
    }

}
