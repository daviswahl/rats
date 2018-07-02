use kind::HKT;
use kind::Kind;
use kind::{IntoKind, Reify};
use std::any;

#[derive(Clone, Debug, PartialEq)]
pub struct CustomKind;
impl HKT for CustomKind {}

#[derive(Clone, Debug, PartialEq)]
pub struct Custom<T>(Vec<T>);

impl<A: 'static> IntoKind<'static, CustomKind, A> for Custom<A> {
    type Kind = CustomKind;
    fn into_kind(self) -> Kind<'static, CustomKind, A> {
        Kind::Any::<CustomKind, A>(Box::new(self))
    }
}

#[allow(unreachable_patterns)]
impl<'f_, A> Reify<CustomKind, A> for Kind<'f_, CustomKind, A>
where A: 'static {

    type Out = Custom<A>;
    fn reify(self) -> Custom<A> {
        match self {
            Kind::Any(t) => {
                *t.downcast().unwrap()
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test() {
        let f = Custom(vec![1,2,3]).into_kind();
        assert_eq!(f.reify(), Custom(vec![1,2,3]));
    }
}