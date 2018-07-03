use applicative::Applicative;
use functor::Functor;
use kind::{Kind,IntoKind, Empty};

pub trait Traverse<'f_, F_, Z = Empty>: Functor<'f_, F_, Z>
where
    F_: Functor<'f_, F_, Z>,
{
    /// Given a function which returns a G effect, thread this effect
    /// through the running of this function on all the values in F,
    /// returning an F<B> in a G context. -- Cats
    ///
    /// # Examples
    ///
    /// ```
    /// # use rats::kind::{IntoKind, Reify, Kind, ReifyRef};
    /// # use rats::kinds::OptionKind;
    /// # use rats::functor::KindFunctorExt;
    /// use rats::traverse::TraverseExt;
    ///
    /// fn parse_int(s: &str) -> Kind<'static, OptionKind, i32> {
    ///     match s.parse::<i32>() {
    ///         Ok(i) => Some(i),
    ///         Err(_) => None,
    ///     }.into_kind()
    /// }
    ///
    /// let r = vec!["1","2","3"].into_kind().traverse(parse_int);
    /// let r = r.map(|k| k.reify()).reify();
    /// assert_eq!(r, Some(vec![1,2,3]));
    ///
    /// let r = vec!["1","two", "3"].into_kind().traverse(parse_int);
    /// let r = r.map(|k| k.reify()).reify();
    /// assert_eq!(r, None);
    /// ```
    ///
    /// (F<A>, Fn) -> G<F<B>>
    /// where
    /// G: Applicative
    /// Fn: Fn(A) -> G<B>
    fn traverse<'g_, Fn_, G_, A, B, Z2>(fa: Kind<'f_, F_, A, Z>, fn_: Fn_) -> Kind<'g_, G_, Kind<'f_, F_, B, Z>, Z2>
    where
        G_: Applicative<'g_, G_, Z2>,
        Fn_: Fn(A) -> Kind<'g_, G_, B, Z>;
}

pub trait TraverseExt<'f_, F_, Z> where F_: Traverse<'f_, F_, Z> {
    type A;

    /// (Self<Self::A>, λ) -> G<Self<Self::A>>
    /// where
    /// Self: F
    /// G: Applicative
    /// λ: Fn(Self::A) -> G<B>
    fn traverse<Fn_, G_, B, Z2>(self, fn_: Fn_) -> Kind<'f_, G_, Kind<'f_, F_, B, Z>, Z2>
    where
        G_: Applicative<'f_, G_, Z2>,
        Fn_: Fn(Self::A) -> Kind<'f_, G_, B, Z>;
}

impl<'f_, F_, A, Z> TraverseExt<'f_, F_, Z> for A
where
    F_: Traverse<'f_, F_, Z>,
    A: IntoKind<'f_, F_, A, Z, Kind=F_> + 'f_,
    Z: 'f_
{
    /// (Self<A>, λ) -> G<Self<A>>
    /// where
    /// Self: F
    /// G: Applicative,
    /// λ: Fn(A) -> G<B>
    type A = A;
    fn traverse<Fn_, G_, B, Z2>(self, f: Fn_) -> Kind<'f_, G_, Kind<'f_, F_, B, Z>, Z2>
    where
        G_: Applicative<'f_, G_, Z2>,
        Fn_: Fn(Self::A) -> Kind<'f_, G_, B, Z>,
    {
        F_::traverse(self.into_kind(), f)
    }
}

mod tests {
    use kind::{Kind,IntoKind, Reify, ReifyRef};
    use kinds::OptionKind;
    use traverse::TraverseExt;
    fn doc_test() {
        fn parse_int<'a>(s: &str) -> Kind<'a, OptionKind, i32> {
           match s.parse::<i32>() {
               Ok(e) => Some(e)     ,
               Err(_) => None
            }.into_kind()
        }
    }
}