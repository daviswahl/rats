use hkt::*;
pub trait IntoKind<K: HKT, T>
where
    Self: Kinded<K, T>,
{
    fn into_kind(self) -> Kind<K, T>;
}

pub trait FromKind<K: HKT, T> {
    type Out: Kinded<K, T>;
    fn from_kind(k: Kind<K, T>) -> Self::Out;
}

pub trait IntoKinded<K: HKT, T> {
    type Out: Kinded<K, T>;
    fn into_kinded(self) -> Self::Out;
}

#[cfg(test)]
mod tests {
    use super::*;
    use functor::FunctorExt;

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
}
