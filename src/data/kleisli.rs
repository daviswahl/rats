use kind::{Kind, HKT};
use std::marker::PhantomData;

struct Run<'kind, F, A: 'kind, B: 'kind, Func>
where
    F: HKT,
    Func: Fn(A) -> Kind<'kind, F, B>,
{
    func: Func,
    a: PhantomData<*const A>,
    b: PhantomData<*const B>,
}
impl<'kind, F, A: 'kind, B: 'kind, Func> Run<'kind, F, A, B, Func>
where
    F: HKT,
    Func: Fn(A) -> Kind<'kind, F, B>,
{
    fn new(f: Func) -> Run<'kind, F, A, B, Func> {
        Run {
            func: f,
            a: PhantomData,
            b: PhantomData,
        }
    }
}
struct Kleisli<'kind, F: HKT, A: 'kind, B: 'kind, Func>
where
    Func: Fn(A) -> Kind<'kind, F, B>,
{
    run: Run<'kind, F, A, B, Func>,
}
