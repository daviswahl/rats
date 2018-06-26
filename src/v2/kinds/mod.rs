macro_rules! derive_hkt {
    ($t:ident) => {
        impl<T, U> HKT<U> for $t<T> {
            type C = T;
            type T = $t<U>;
        }
    }
}

pub mod vec;