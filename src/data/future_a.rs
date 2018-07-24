use core::task::Context;
use core::task::Poll;
use std::future::Future;
use std::marker::PhantomData;
use std::marker::Unpin;
use std::mem::PinMut;

pub struct FutureA<F> {
    inner: F,
}

impl<F, A> Future for FutureA<F>
where
    F: Future<Output = A> + Unpin,
{
    type Output = F::Output;

    fn poll(mut self: PinMut<Self>, cx: &mut Context) -> Poll<<Self as Future>::Output> {
        F::poll(PinMut::new(&mut self.inner), cx)
    }
}
