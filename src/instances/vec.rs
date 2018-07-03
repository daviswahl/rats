use applicative::Applicative;
use applicative::Point;
use foldable::Foldable;
use functor::Functor;
use kind::IntoKind;
use kind::Kind;
use kind::Reify;
use kinds::VecKind;
use kind::Empty;
use traverse::Traverse;




impl<'f_> Functor<'f_, VecKind> for VecKind {
    fn map<F, A, B>(a: Kind<'f_, VecKind, A>, f: F) -> Kind<'f_, VecKind, B>
    where
        F: FnMut(A) -> B + 'f_,
    {
        a.reify().into_iter().map(f).collect::<Vec<B>>().into_kind()
    }
}

impl Foldable<VecKind> for VecKind {
    // this is almost certainly wrong.
    fn fold_right<Fn_, A, B>(fa: Kind<VecKind, A>, b: B, fn_: Fn_) -> B
    where
        Fn_: Fn((A, B)) -> B,
    {
        let mut b = b;
        for a in fa.reify() {
            b = fn_((a, b));
        }
        b
    }
}

impl<'f_> Traverse<'f_, VecKind> for VecKind {
    fn traverse<'g_, Fn_, G_, A, B, Z2>(
        fa: Kind<'f_, VecKind, A>,
        fn_: Fn_,
    ) -> Kind<'g_, G_, Kind<'f_, VecKind, B>, Z2>
    where
        G_: Applicative<'g_, G_, Z2>,
        Fn_: Fn(A) -> Kind<'g_, G_, B, Z2>,
    {
        let acc = vec![].into_kind().point::<G_>();
        VecKind::fold_right(fa, acc, |(a, acc)| {
            G_::map2(fn_(a), acc, |(a, b)| {
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
    use traverse::{TraverseExt, Traverse};

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
            .map(|f| f.into_kind())
            .traverse(identity);
        assert_eq!(list.map(|i| i.reify()).reify(), None)
    }
}
