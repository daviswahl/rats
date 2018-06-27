use hkt::{Kinded, HKT};
use kind::Kind;

#[cfg(test)]
mod tests {
    use super::*;
    use functor::KindFunctorExt;
    use kind::Kinded;
    use kind::KindExt;

    #[test]
    fn into_kind() {
        let kind = vec![1, 2, 3].into_kind();
        assert_eq!(kind.reify(), vec![1, 2, 3]);
    }

    #[test]
    fn test_functor() {
        let kind = vec![1, 2, 3].into_kind();
        let result = kind.map(|i| i * 2);
        assert_eq!(result.reify(), vec![2, 4, 6]);
    }
}
