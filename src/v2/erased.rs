use std::mem;
use std::ptr;
use std::{alloc, alloc::Alloc};

pub struct Erased {
    ptr: ptr::NonNull<u8>,
    size: usize,
    align: usize,
}

impl Erased {
    pub fn erase<T: Sized>(t: T) -> Erased {
        unsafe {
            let layout = alloc::Layout::for_value(&t);
            let align = layout.align();
            let size = layout.size();

            let ptr = alloc::Global.alloc(layout).unwrap().cast::<T>();
            ptr::write(ptr.cast::<T>().as_ptr(), t);
            Erased {
                ptr: ptr.cast::<u8>(),
                size,
                align,
            }
        }
    }

    pub unsafe fn unerase<T: Sized>(self) -> T {
        unsafe {
            let ptr = self.ptr.cast::<T>();
            let result = ptr::read(ptr.as_ptr());
            alloc::Global.dealloc(ptr.cast::<u8>(), alloc::Layout::for_value(&result));
            result
        }
    }

    pub unsafe fn drop<T: Sized>(self) {
        let ptr = self.ptr.cast::<T>();
        let result = ptr::read(ptr.as_ptr());
        alloc::Global.dealloc(ptr.cast::<u8>(), alloc::Layout::for_value(&result));
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    use std::fmt::Debug;
    fn test_erase<T: Clone + PartialEq + Debug>(t: T) {
        let copy = t.clone();
        let erased = Erased::erase(t);
        unsafe {
            assert_eq!(copy, erased.unerase::<T>());
        }
    }
    #[test]
    fn erasure_1() {
        test_erase(vec![1, 2, 3]);
        test_erase(Some(1));
    }
}
