use std::marker;
use kind;
use kind::{Kind};

#[derive(Debug, Clone)]
pub struct Context<K: Kind, I> {
    kind: K,
    _marker: marker::PhantomData<*const I>,
}
impl<K: Kind, I> Context<K, I> {
    pub fn from<C>(c: C) -> Self
    where
        C: IntoContext<Kind = K, Item = I> + EraseType<C>,
    {
        let new = Context {
            kind: <K as Kind>::new(<C as EraseType<C>>::erase(c)),
            _marker: marker::PhantomData,
        };

        println!("new context");
        new
    }
}

pub unsafe trait UnsafePointerCast {
    unsafe fn unsafe_cast<T>(self) -> T;
}

unsafe impl<K: Kind, T> UnsafePointerCast for Context<K, T> {
    unsafe fn unsafe_cast<G>(self) -> G {
        self.kind.read::<G>()
    }
}

pub trait ExtractKind<C: UnsafePointerCast + FromContext> {
    fn extract(c: C) -> C::Out;
}

impl<C: UnsafePointerCast + FromContext> ExtractKind<C> for C {
    fn extract(c: C) -> C::Out {
        println!("unsafe pointer cast for c");
        unsafe { c.unsafe_cast() }
    }
}

pub trait ExtractExt<C: UnsafePointerCast + FromContext> {
    fn extract(self) -> C::Out;
}

impl<C: UnsafePointerCast + FromContext> ExtractExt<C> for C {
    fn extract(self) -> <C as FromContext>::Out {
        println!("extracting");
        let result = unsafe { self.unsafe_cast() };
        println!("extracting successful");
        result
    }
}

pub trait EraseType<C>{
    fn erase(t: C) -> Box<[u8]>;
}

impl<C> EraseType<C> for C {
    fn erase(t: C) -> Box<[u8]> {
        unsafe { kind::any_as_u8_box_slice(t) }
    }
}

pub trait IntoContextExt<C: IntoContext + EraseType<C>> {
    fn into_context(self) -> Context<C::Kind, C::Item>;
}

impl<C: IntoContext + EraseType<C>> IntoContextExt<C> for C {
    fn into_context(self) -> Context<<C as IntoContext>::Kind, <C as IntoContext>::Item> {
        Context::from(self)
    }
}

pub trait IntoContext {
    type Kind: Kind;
    type Item;
}

pub trait FromContext {
    type Out: IntoContext;
}

#[cfg(test)]
mod test {
    use super::*;
    use kinds;

    #[test]
    fn extractor_test() {
        let k: Context<kinds::Vec, i32> = vec![1,2,3].into_context();
        println!("got k: {:?}", k);
        let result = k.extract();
        println!("extracted from k: {:?}", result);
        assert_eq!(result, vec![1, 2, 3])
    }

    #[test]
    fn clone_test() {
        let k: Context<kinds::Vec, i32> = Context::from(vec![1, 2, 3]);
        assert_eq!(k.clone().extract(), vec![1, 2, 3]);
        assert_eq!(k.extract(), vec![1, 2, 3]);
    }
}