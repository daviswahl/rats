use kind::Empty;
use kind::Kind;
use kind::HKT;

pub trait Functor<'f_, F_: HKT, Z = Empty>: HKT {
    /// (F<(A,)>, Fn(A) -> B) -> F<B,>
    fn map<Fn_, A, B>(a: Kind<'f_, F_, A, Z>, f: Fn_) -> Kind<'f_, F_, B, Z>
    where
        Fn_: Fn(A) -> B + 'f_;
}

pub trait KindFunctorExt<'f_, F_, Z>
where
    F_: Functor<'f_, F_, Z>,
{
    type A;
    type Z = Z;
    fn map<B, Fn_>(self, f: Fn_) -> Kind<'f_, F_, B, Self::Z>
    where
        Fn_: Fn(Self::A) -> B + 'f_;
}

impl<'f_, F_, A, Z> KindFunctorExt<'f_, F_, Z> for Kind<'f_, F_, A, Z>
where
    F_: Functor<'f_, F_, Z>,
{
    type A = A;
    type Z = Z;

    fn map<B, Fn_>(self, f: Fn_) -> Kind<'f_, F_, B, Self::Z>
    where
        Fn_: Fn(Self::A) -> B + 'f_,
    {
        F_::map(self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kind::IntoKind;

    fn convert_to_string<'f_, F_>(fa: Kind<'f_, F_, i32>) -> Kind<'f_, F_, String>
    where
        F_: Functor<'f_, F_>,
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
