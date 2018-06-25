use std::fmt::Debug;

pub trait Kind: Clone + Debug {
    type Kind: Kind;

    fn from_boxed_slice(b: Vec<u8>) -> Self;

    fn buf(self) -> Vec<u8>;
}
