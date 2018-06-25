use std::ptr;
use std::mem;
use std::slice;
use std::fmt::Debug;
use bincode::{serialize, deserialize};
use serde::{Serialize, Deserialize};

pub trait Kind<U> {
    type C;
    type T;
}

impl<U,T> Kind<U> for Vec<T> {
    type C = Vec<T>;
    type T = ();
}

trait Context<K: Kind> {
    type Item;
    type Item2 = ();
}

impl<K: Kind,T> Context<K> for Vec<T> {
    type Item = T;
}

trait Functor<K> where K: Kind {
    fn map<F, A, B, Out>(k: K) -> Out<K> where F: FnMut(A) -> B, Out: Kind<K, Item=B>;
}
