use std::ptr;
use std::mem;
use std::slice;
use std::fmt::Debug;
use bincode::{serialize, deserialize};
use serde::{Serialize, Deserialize};

pub trait Kind: Clone+Debug {
    type Kind: Kind;

    fn from_boxed_slice(b: Vec<u8>) -> Self;

    fn buf(self) -> Vec<u8>;
}
