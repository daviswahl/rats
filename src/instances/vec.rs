use applicative::Applicative;
use applicative::Point;
use foldable::Foldable;
use functor::Functor;
use kind::IntoKind;
use kind::Kind;
use kind::Reify;
use kinds::VecKind;
use traverse::Traverse;

impl Functor<VecKind> for VecKind {
    fn map<F, A, B>(a: Kind<VecKind, A>, f: F) -> Kind<VecKind, B>
    where
        F: FnMut(A) -> B,
    {
        a.reify().into_iter().map(f).collect::<Vec<B>>().into_kind()
    }
}

impl Foldable<VecKind> for VecKind {
    // this is almost certainly wrong.
    fn fold_right<Func, A, B>(fa: Kind<VecKind, A>, acc: B, f: Func) -> B
    where
        Func: Fn((A, B)) -> B,
    {
        let mut accum = acc;
        let mut iter = fa.reify().into_iter();
        while let Some(x) = iter.next() {
            accum = f((x, accum));
        }
        accum
    }
}

impl Traverse<VecKind> for VecKind {
    fn traverse<F, G, A, B>(fa: Kind<VecKind, A>, f: F) -> Kind<G, Kind<VecKind, B>>
    where
        G: Applicative<G>,
        F: Fn(A) -> Kind<G, B>,
    {
        let acc = vec![].into_kind().point::<G>();
        VecKind::fold_right(fa, acc, |(a, acc)| {
            G::map2(f(a), acc, |(a, b)| {
                // need to add a direct push method
                let mut v = b.reify();
                v.push(a);
                v.into_kind()
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use functor::KindFunctorExt;
    use identity;
    use kind::IntoKind;
    use traverse::TraverseExt;

    #[test]
    fn test_vec_map_from_functor_1() {
        let result = vec![1, 2, 3].into_kind().map(|i| i * 2).reify();
        assert_eq!(result, vec![2, 4, 6]);
    }
    struct Foo<'a> {
        b: &'a str,
    }

    #[test]
    fn test_fold_right() {
        let k = vec![1, 2, 3].into_kind();
        let result = VecKind::fold_right(k, 0, |(i, acc)| i + acc);
        assert_eq!(6, result);
    }
    #[test]
    fn test_with_refs() {
        let foo = "foo".to_owned();
        let bar = "bar".to_owned();
        let batz = "batz".to_owned();
        let strings: Vec<Foo> = vec![foo.as_ref(), bar.as_ref(), batz.as_ref()]
            .into_iter()
            .map(|s| Foo { b: s })
            .collect();

        let result = strings.into_kind().map(|i| i.b).reify();
        assert_eq!(result, vec!["foo", "bar", "batz"]);
    }

    use std::string::String;
    #[test]
    fn test_with_mut_refs() {
        let mut foo = "foo".to_owned();
        let mut bar = "bar".to_owned();
        let mut batz = "batz".to_owned();
        {
            let strings = vec![&mut foo, &mut bar, &mut batz];
            strings
                .into_kind()
                .map(|s: &mut String| {
                    s.push_str("butts");
                    s
                })
                .reify();
        }
        assert_eq!(foo, "foobutts")
    }

    #[test]
    fn test_traverse() {
        let list = vec![Some(1), Some(2), None]
            .into_kind()
            .map(|k| k.into_kind())
            .traverse(identity);
        assert_eq!(list.map(|i| i.reify()).reify(), None)
    }
}
