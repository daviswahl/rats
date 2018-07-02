



#[cfg(test)]
mod tests {
    use kind::HKT;
    use kind::Kind;
    use kind::{IntoKind, Reify, ReifyKind, AnyKind};

    #[derive(Clone, Debug, PartialEq)]
    pub struct CustomKind;
    impl HKT for CustomKind {
        type Kind = CustomKind;
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct Custom<T>(Vec<T>);

    impl<A: 'static> IntoKind<'static, CustomKind, A> for Custom<A> {
        type Kind = CustomKind;
        fn into_kind(self) -> Kind<'static, CustomKind, A> {
            Kind::Any::<CustomKind, A>(Box::new(self))
        }
    }

    impl<A: 'static> AnyKind<A> for Custom<A> {
        type Out = Custom<A>;
        type Kind = CustomKind;
    }

    #[allow(unreachable_patterns)]
    impl<A> ReifyKind<'static, CustomKind, A> for CustomKind {
        fn reify(k: Kind<'static, CustomKind, A>) -> Self::Out {
            match k {
                Kind::Any(t) => *t.downcast().unwrap(),
                _ => unreachable!(),
            }
        }
        type Out = Custom<A>;
    }

    #[test]
    fn test() {
        let f: Kind<'static, CustomKind, i32> = Custom(vec![1, 2, 3]).into_kind();
        assert_eq!(f.reify(), Custom(vec![1, 2, 3]));
    }
}
