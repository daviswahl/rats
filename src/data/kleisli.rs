use lifted::Lifted;
use std::marker::PhantomData;

pub struct Kleisli<'a, F: 'a, A: 'a, B: 'a, Func> {
    run: Func,
    __marker: PhantomData<&'a (F, A, B)>,
}

impl<'a, F, A, B, Func> Kleisli<'a, F, A, B, Func>
where
    Func: Fn(A) -> Lifted<'a, F, B>,
{
}
