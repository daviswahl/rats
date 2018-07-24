#[derive(Clone, Debug, PartialEq)]
pub struct Id<T>(pub T);

impl<T> Id<T> {
    pub fn take(self) -> T {
        self.0
    }
}

pub trait IdExt {
    type Out;
    fn id(self) -> Id<Self::Out>;
}

impl<T> IdExt for T {
    type Out = T;
    fn id(self) -> Id<Self::Out> {
        Id(self)
    }
}
