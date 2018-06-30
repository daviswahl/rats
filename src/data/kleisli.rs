use kind::{Kind, HKT};
use std::marker::PhantomData;

struct Run<F, A, B, Func>
where
    F: HKT,
    Func: Fn(A) -> Kind<F, B>,
{
    func: Func,
    a: PhantomData<*const A>,
    b: PhantomData<*const B>,
}
impl<F, A, B, Func> Run<F, A, B, Func>
where
    F: HKT,
    Func: Fn(A) -> Kind<F, B>,
{
    fn new(f: Func) -> Run<F, A, B, Func> {
        Run {
            func: f,
            a: PhantomData,
            b: PhantomData,
        }
    }
}
struct Kleisli<F: HKT, A, B, Func>
where
    Func: Fn(A) -> Kind<F, B>,
{
    run: Run<F, A, B, Func>,
}
