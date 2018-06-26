use std::mem;
use std::ptr;
use std::{alloc, alloc::Alloc};

pub struct Erased {
    ptr: ptr::NonNull<u8>,
    size: usize,
    align: usize,
}

///
impl Erased {
    // unsafe because caller must deal with memory: I believe the compiler will drop
    // the value automatically, but it will try to drop it with the wrong type, which is a problem.
    pub unsafe fn erase<T: Sized>(t: T) -> Erased {
        let layout = alloc::Layout::for_value(&t);
        let align = layout.align();
        let size = layout.size();

        let ptr = alloc::Global.alloc(layout).unwrap().cast::<T>();
        ptr::write(ptr.as_ptr(), t);

        // Capture size and alignment in hopes that we'll be able to use this information to properly
        // drop
        Erased {
            ptr: ptr.cast::<u8>(),
            size,
            align,
        }
    }

    // unsafe because caller needs to get the type right.
    pub unsafe fn unerase<T: Sized>(self) -> T {
        let ptr = self.ptr.cast::<T>();
        let result = ptr::read(ptr.as_ptr());
        alloc::Global.dealloc(ptr.cast::<u8>(), alloc::Layout::for_value(&result));
        result
    }

    // caller must drop, unsafe because they need to get the type right.
    pub unsafe fn drop<T: Sized>(self) {
        let ptr = self.ptr.cast::<T>();
        let result = ptr::read(ptr.as_ptr());
        alloc::Global.dealloc(ptr.cast::<u8>(), alloc::Layout::for_value(&result));
    }
}

// I don't think this is correct, we don't know the size and alignment of T so ptr::read can't be
// right.
impl Drop for Erased {
    fn drop(&mut self) {
        println!(
            "dropping: size: {}, align: {}, ptr: {:#?}",
            self.size,
            self.align,
            unsafe { ptr::read(self.ptr.as_ptr()) }
        );
    }
}

// TODO: Figure out how to implement drop without knowing the type
#[cfg(test)]
mod tests {
    use super::*;

    use std::fmt::Debug;
    fn test_erase<T: Clone + PartialEq + Debug>(t: T) {
        let copy = t.clone();
        let erased = unsafe { Erased::erase(t) };
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
