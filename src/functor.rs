use kind::Kind;
use kind::HKT;

pub trait Functor<K: HKT>: HKT {
    fn map<F, A, B>(a: Kind<K, A>, f: F) -> Kind<K, B>
    where
        F: Fn(A) -> B;
}

pub trait KindFunctorExt<K>
where
    K: Functor<K>,
{
    type Item;
    fn map<B, F>(self, f: F) -> Kind<K, B>
    where
        F: Fn(Self::Item) -> B;
}

impl<K, T> KindFunctorExt<K> for Kind<K, T>
where
    K: Functor<K>,
{
    type Item = T;

    fn map<B, F>(self, f: F) -> Kind<K, B>
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
        F: HKT + Functor<F>,
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
