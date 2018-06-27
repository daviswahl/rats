use conversions::*;
use functor::Functor;
use kind::Kind;
use kinds::vec::VecK;

impl Functor<VecK> for VecK {
    fn map<F, A, B>(k: Kind<VecK, A>, f: F) -> Kind<VecK, B>
    where
        F: FnMut(A) -> B,
    {
        k.reify().into_iter().map(f).collect::<Vec<B>>().into_kind()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use functor::FunctorExt;
    #[test]
    fn test_vec_map_from_functor_1() {
        let result = vec![1, 2, 3].into_kind().map(|i| i * 2).reify();
        assert_eq!(result, vec![2, 4, 6]);
    }
}
