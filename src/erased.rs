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
pub(crate) struct Erased(ptr::NonNull<()>);

impl Erased {
    // I think this is safe, but will leak memory if the caller doesn't unerase or consume self.
    // But I guess leaking memory is not technically "unsafe".
    pub fn erase<T: Sized>(t: T) -> Erased {
        let ptr = unsafe {
            ptr::NonNull::new_unchecked(Box::into_raw(Box::new(t)))
        };
        Erased(ptr.cast())
    }

    // unsafe because it is the callers responsibility to ask for the correct type.
    pub unsafe fn reify<T: Sized>(self) -> T {
        *Box::from_raw(self.0.cast().as_ptr())
    }

    // TODO: not sure how to get this right.
    pub unsafe fn reify_as_ref<T: Sized>(&self) -> &T {
        unimplemented!()
    }

    // TODO: not sure how to get this right.
    pub unsafe fn reify_as_mut_ref<T: Sized>(&mut self) -> &mut T {
        unimplemented!()
    }

    // unsafe because it is the callers responsibility to ask for the correct type.
    pub unsafe fn drop<T: Sized>(self) {
        self.reify::<T>();
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
            assert_eq!(copy, erased.reify());
        }
    }
    #[test]
    fn erasure_1() {
        test_erase(vec![1, 2, 3]);
        test_erase(Some(1));
    }
}
