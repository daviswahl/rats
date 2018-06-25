use context::{Context, FromContext, IntoContext, NewType};
use kind;
use serde::de::DeserializeOwned;
use std::vec;

#[derive(Debug, Clone)]
pub struct Vec(vec::Vec<u8>);

impl kind::Kind for Vec {
    type Kind = Vec;

    fn from_boxed_slice(b: vec::Vec<u8>) -> Self {
        Vec(b)
    }

    fn buf(self) -> vec::Vec<u8> {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct Option(vec::Vec<u8>);

impl kind::Kind for Option {
    type Kind = Option;

    fn from_boxed_slice(b: vec::Vec<u8>) -> Self {
        Option(b)
    }

    fn buf(self) -> vec::Vec<u8> {
        self.0
    }
}

use std::option;

impl<T> IntoContext for option::Option<T> {
    type Kind = Option;
    type Item = T;
}

impl<T> IntoContext for vec::Vec<T> {
    type Kind = Vec;
    type Item = T;
}

#[derive(Serialize, Deserialize)]
pub struct NewVec<T>(vec::Vec<T>);

impl<T> NewType for NewVec<T> {
    type Type = vec::Vec<T>;

    fn get(self) -> <Self as NewType>::Type {
        self.0
    }
}

impl<T: DeserializeOwned> FromContext for Context<Vec, T> {
    type Out = NewVec<T>;
}

#[derive(Serialize, Deserialize)]
pub struct NewOption<T>(option::Option<T>);

impl<T> NewType for NewOption<T> {
    type Type = option::Option<T>;

    fn get(self) -> <Self as NewType>::Type {
        self.0
    }
}
impl<T: DeserializeOwned> FromContext for Context<Option, T> {
    type Out = NewOption<T>;
}
