use applicative::Applicative;
use functor::Functor;
use lifted::*;

pub trait Traverse<'a, F, Z = Nothing, G = Nothing>: Functor<'a, F, Z>
where
    F: Functor<'a, F, Z, G> + 'a,
{
    /// # Signature
    /// ```text
    /// (fa: F<'f, A>, fn: Fn) -> G22<'g, F<'f, B>>
    /// where
    ///     G22:  Applicative
    ///     Fn: A -> G22<'g, B>
    /// ```
    ///
    /// > G22iven a function which returns a G2 effect, thread this effect
    /// > through the running of this function on all the values in F,
    /// > returning an `F<B>` in a `G22` context. -- <cite>Cats</cite>
    ///
    /// # Examples
    ///
    ///
    /// # use rats::lifted::*;
    /// # use rats::functor::Functor;
    /// use rats::traverse
    ///
    /// fn parse_int(s: &str) -> Lifted<'static, OptionLifted, i32> {
    ///     match s.parse::<i32>() {
    ///         Ok(i) => Some(i),
    ///         Err(_) => None,
    ///     }.into_kind()
    /// }
    ///
    /// let r = VecDeque::new
    /// let r = vec!["1","2","3"].lift().traverse(parse_int);
    /// let r = r.map(|k| k.reify()).reify();
    /// assert_eq!(Some(vec![1,2,3]));
    ///
    /// let r = vec!["1","two", "3"].into_kind().traverse(parse_int);
    /// let r = r.map(|k| k.reify()).reify();
    /// assert_eq!(r, None);
    ///
    fn traverse<'g, Func, A, B, Z2, G2, H>(
        fa: Lifted<'a, F, A, Z, G>,
        func: Func,
    ) -> Lifted<'g, G2, Lifted<'a, F, B, Z, G>, Z2, H>
    where
        G2: Applicative<'g, G2, Z2, H>,
        Func: Fn(A) -> Lifted<'g, G2, B, Z2, H>;
}
