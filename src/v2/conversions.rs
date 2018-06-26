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