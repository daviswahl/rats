use kind;
use kind::{Kind};
use context::{Context, IntoContext, FromContext, IntoContextExt};
use std::mem;
use std::vec;

#[derive(Debug)]
pub struct Vec(Box<[u8]>);

use std::ptr;
impl kind::Kind for Vec {
    type Kind = Vec;

    fn from_boxed_slice(b: Box<[u8]>) -> Self {
        Vec(b)
    }

    fn buf(self) -> Box<[u8]> {
        self.0
    }
}

impl Clone for Vec {
    fn clone(&self) -> Self {
        Vec(self.0.clone())
    }
}
impl<T> IntoContext for vec::Vec<T> {
    type Kind = Vec;
    type Item = T;
}

#[derive(Debug, Clone)]
pub struct Option(Box<[u8]>);

impl kind::Kind for Option {
    type Kind = Option;

    fn from_boxed_slice(b: Box<[u8]>) -> Self {
        Option(b)
    }

    fn buf(self) -> Box<[u8]> {
        self.0
    }
}

use std::option;
impl<T> IntoContext for option::Option<T> {
    type Kind = Option;
    type Item = T;
}

impl<T> FromContext for Context<Vec, T> {
    type Out = vec::Vec<T>;
}

impl<T> FromContext for Context<Option, T> {
    type Out = option::Option<T>;
}
