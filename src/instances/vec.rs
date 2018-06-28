use functor::Functor;
use kind::IntoKind;
use kind::Kind;
use kind::Reify;
use kind::VecKind;

impl Functor<VecKind> for VecKind {
    fn map<F, A, B>(a: Kind<VecKind, A>, f: F) -> Kind<VecKind, B>
    where
        F: FnMut(A) -> B,
    {
        a.reify().into_iter().map(f).collect::<Vec<B>>().into_kind()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use functor::KindFunctorExt;
    use kind::IntoKind;

    #[test]
    fn test_vec_map_from_functor_1() {
        let result = vec![1, 2, 3].into_kind().map(|i| i * 2).reify();
        assert_eq!(result, vec![2, 4, 6]);
    }
    struct Foo<'a> {
        b: &'a str,
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

    use std::ops::Add;
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
}
