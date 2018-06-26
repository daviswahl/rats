use std::ptr;

pub(crate) struct Erased(ptr::NonNull<()>);

///
impl Erased {
    // unsafe because caller must deal with memory.
    pub unsafe fn erase<T: Sized>(t: T) -> Erased {
        let data = Box::new(t);
        let ptr = ptr::NonNull::new_unchecked(Box::into_raw(data));
        Erased(ptr.cast())
    }

    // unsafe because caller needs to get the type right.
    pub unsafe fn unerase<T: Sized>(self) -> T {
        *Box::from_raw(self.0.cast().as_ptr())
    }

    pub unsafe fn unerase_ref<T: Sized>(&self) -> &T {
        unimplemented!() // not sure how to do this
    }
}

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
