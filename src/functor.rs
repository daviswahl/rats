use kind::Empty;
use kind::EmptyType;
use kind::Kind;
use kind::HKT;

pub trait Functor<K: HKT, Z = Empty>: HKT {
    fn map<F, A, B>(a: Kind<K, A, Z>, f: F) -> Kind<K, B, Z>
    where
        F: Fn(A) -> B;
}

pub trait KindFunctorExt<K, Z = Empty>
where
    K: Functor<K, Z>,
{
    type Item;
    type Z = Z;
    fn map<B, F>(self, f: F) -> Kind<K, B, Self::Z>
    where
        F: Fn(Self::Item) -> B;
}

impl<K, T, Z> KindFunctorExt<K, Z> for Kind<K, T, Z>
where
    K: Functor<K, Z>,
{
    type Item = T;
    type Z = Z;

    fn map<B, F>(self, f: F) -> Kind<K, B, Self::Z>
    where
        F: Fn(Self::Item) -> B,
    {
        K::map(self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kind::IntoKind;

    fn convert_to_string<F>(fa: Kind<F, i32>) -> Kind<F, String>
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
