use hkt::{Kinded, HKT};
use kind::Kind;

pub trait KindedExt<K: HKT, T>
where
    Self: Kinded<K, T>,
{
    fn into_kind(self) -> Kind<K, T>;
}

impl<K: HKT, T, Knd> KindedExt<K, T> for Knd
where
    Knd: Kinded<K, T>,
{
    fn into_kind(self) -> Kind<K, T> {
        Kind::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use functor::FunctorExt;

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
