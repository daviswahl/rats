use kind::Empty;
use kind::EmptyType;
use kind::Kind;
use kind::HKT;

pub trait Functor<K: HKT, Z = Empty>: HKT {
    fn map<'kind, F, A, B>(a: Kind<'kind, K, A, Z>, f: F) -> Kind<'kind, K, B, Z>
    where
        F: Fn(A) -> B + 'kind;
}

pub trait KindFunctorExt<'kind, K, Z = Empty>
where
    K: Functor<K, Z>,
{
    type Item;
    type Z = Z;
    fn map<B, F>(self, f: F) -> Kind<'kind, K, B, Self::Z>
    where
        F: Fn(Self::Item) -> B + 'kind;
}

impl<'kind, K, T, Z> KindFunctorExt<'kind, K, Z> for Kind<'kind, K, T, Z>
where
    K: Functor<K, Z>,
{
    type Item = T;
    type Z = Z;

    fn map<B, F>(self, f: F) -> Kind<'kind, K, B, Self::Z>
    where
        F: Fn(Self::Item) -> B + 'kind,
    {
        K::map(self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kind::IntoKind;

    fn convert_to_string<'kind, F>(fa: Kind<'kind, F, i32>) -> Kind<'kind, F, String>
    where
        F: Functor<F>,
    {
        fa.map(|i| format!("{} wow monads!!!!", i))
    }

    #[test]
    fn test_consuming_a_functor() {
        println!("{:?}", convert_to_string(Some(1).into_kind()));
        println!("{:?}", convert_to_string(vec![1, 2, 3].into_kind()));

        // println!("{:?}", direct_convert_to_string(Some(1)));
        // println!("{:?}", direct_convert_to_string(vec![1,2,3]))
    }
}
