use lifted::Lifted;
use std::marker::PhantomData;

pub struct Kleisli<F, A, B, Func> {
    run: Func,
    __marker: PhantomData<(F, A, B)>,
}

impl<F, A, B, Func> Kleisli<F, A, B, Func> where Func: Fn(A) -> Lifted<F, B> {}
