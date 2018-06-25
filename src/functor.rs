use kind::{Kind};
use context::{Context, ExtractExt, ExtractKind, FromContext, IntoContext, IntoContextExt};
use kinds;

trait Functor<K>
where
    K: Kind,
{
    fn map<A, B, F>(k: Context<K, A>, f: F) -> Context<K, B>
    where
        F: FnMut(A) -> B;
}

trait KindFunctorExt<K: Kind> {
    type Item;

    fn map<B, F>(self, f: F) -> Context<K, B>
    where
        F: FnMut(Self::Item) -> B;
}

impl<K: Kind + Functor<K>, T> KindFunctorExt<K> for Context<K, T> {
    type Item = T;

    fn map<B, F>(self, f: F) -> Context<K, B>
    where
        F: FnMut(Self::Item) -> B,
    {
        <K as Functor<K>>::map::<T, B, F>(self, f)
    }
}

impl Functor<kinds::Vec> for kinds::Vec {
    fn map<A, B, F>(m: Context<kinds::Vec, A>, f: F) -> Context<kinds::Vec, B>
    where
        F: FnMut(A) -> B,
    {
        m.extract().into_iter().map(f).collect::<Vec<B>>().into_context()
    }
}

impl Functor<kinds::Option> for kinds::Option {
    fn map<A, B, F>(k: Context<kinds::Option, A>, f: F) -> Context<kinds::Option, B>
    where
        F: FnMut(A) -> B,
    {
        k.extract().map(f).into_context()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn vec_functor_test() {
        let k: Context<kinds::Vec, i32> = Context::from(vec![1, 2, 3]);
        let result1 = k.map(|i| i * 2).map(|i| format!("{}", i));
        assert_eq!(result1.clone().extract(), vec!["2", "4", "6"]);
    }

    #[test]
    fn option_funtor_test() {
        let opt: Context<kinds::Option, i32> = Context::from(Some(1));
        //let result3 = opt.map(|i| i * 2);
        println!("opt: {:?}", opt);
        let result = opt.extract();
        println!("extracted {:?}", result);
        assert_eq!(result, Some(2));
    }
}
