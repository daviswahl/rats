use std::ptr;

/// Erased "erases" the type of a value by boxing it into the heap and then
/// casting the returned pointer into a NonNull<()>. The caller is responsible for tracking the type of the
/// erased value, so that it can be properly unerased.
///
/// I believe Erased will leak memory if the caller does not consume or unerase the pointer.
///
/// I believe it is desirable and possible to implement drop for this struct, but it's a bit difficult because
/// we have know way of knowing the type of the erased value from the drop callsite. I think we can
/// maybe get around this by tracking the size and alignment of the type?
pub struct Erased {
    ptr: ptr::NonNull<()>,
    /// We need to store how to drop this. We do so upon creation.
    drop_impl: Option<Box<Fn(&mut Erased)>>,
}

impl Erased {
    // I think this is safe, but will leak memory if the caller doesn't unerase or consume self.
    // But I guess leaking memory is not technically "unsafe".
    pub fn erase<T: Sized>(t: T) -> Erased {
        let ptr = unsafe { ptr::NonNull::new_unchecked(Box::into_raw(Box::new(t))) };
        Erased {
            ptr: ptr.cast(),
            drop_impl: Some(Box::new(|s: &mut Erased| unsafe {
                ptr::drop_in_place(s.ptr.cast::<T>().as_ptr());
            })),
        }
    }

    // unsafe because it is the callers responsibility to ask for the correct type.
    pub unsafe fn reify<T: Sized>(mut self) -> T {
        self.drop_impl.take();
        *Box::from_raw(self.ptr.cast().as_ptr())
    }

    pub unsafe fn reify_as_ref<T: Sized>(&self) -> &T {
        &*self.ptr.cast().as_ptr()
    }

    pub unsafe fn reify_as_mut_ref<T: Sized>(&mut self) -> &mut T {
        &mut *self.ptr.cast().as_ptr()
    }
}

impl Drop for Erased {
    fn drop(&mut self) {
        if let Some(func) = self.drop_impl.take() { func(self) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fmt::Debug;
    fn test_erase<T: Clone + PartialEq + Debug>(t: T) {
        let copy = t.clone();
        let erased = Erased::erase(t);
        erased.ptr.cast::<T>();
        unsafe {
            assert_eq!(copy, erased.reify());
        }
    }
    #[test]
    #[allow(unused_variables)]
    fn drop_test() {
        {
            Erased::erase(vec![1, 2, 3]);
        }
        {
            let f = Erased::erase(vec![1, 2, 3]);
            unsafe { f.reify::<Vec<i32>>() };
        }
    }

    #[test]
    fn erasure_1() {
        test_erase(vec![1, 2, 3]);
        test_erase(Some(1));
    }
}
