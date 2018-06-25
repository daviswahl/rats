use std::ptr;
use std::mem;
use std::slice;
use std::fmt::Debug;

pub(crate) unsafe fn any_as_u8_box_slice<T: Sized>(p: T) -> Box<[u8]> {
    slice::from_raw_parts((&p as *const _) as *const u8, ::std::mem::size_of::<T>()).into()
}

pub trait Kind: Clone+Debug {
    type Kind: Kind;

    fn new<T>(t: T) -> Self {
        Self::from_boxed_slice(unsafe { any_as_u8_box_slice(t) })
    }

    fn from_boxed_slice(b: Box<[u8]>) -> Self;

    fn buf(self) -> Box<[u8]>;

    unsafe fn read<T>(self) -> T {
        mem::transmute_copy(&self.buf().as_ptr())
    }
}
