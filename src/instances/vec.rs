use functor::Functor;
use kind::Kind;
use kind::Reify;
use kind::VecKind;

impl Functor<VecKind> for VecKind {
    fn map<F, A, B>(a: Kind<VecKind, A>, f: F) -> Kind<VecKind, F::Output>
    where
        F: Fn(A) -> B,
    {
        Kind::new(a.reify().into_iter().map(f).collect::<Vec<B>>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use functor::KindFunctorExt;
    use kind::Kinded;

    #[test]
    fn test_vec_map_from_functor_1() {
        let result = vec![1, 2, 3].into_kind().map(|i| i * 2).reify();
        assert_eq!(result, vec![2, 4, 6]);
    }
}
