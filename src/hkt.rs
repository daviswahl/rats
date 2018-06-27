use std::marker::PhantomData;

pub trait HKT {
    fn marker() -> Self;
}

pub trait Kinded<K: HKT, T> {}

pub trait Reify<K: HKT, T> {
    type Out: Kinded<K, T>;
}

//macro_rules! derive_hkt {
//    ($t:ident) => {
//        impl HKT for $tK {
//            fn marker() -> $tK {
//                $tK
//            }
//        }
//
//        impl<T> Kinded<$tK, T> for $t<T> {}
//
//        impl<T> Reify<$tK, T> for ::kind::Kind<$tK, T> {
//            type Out = $t<T>;
//        }
//    }
//}

#[cfg(test)]
mod tests {
    use super::*;
    use conversions::*;

    #[test]
    fn test_kinded() {
        let f = vec![1, 2, 3];
    }
    #[test]
    fn test_must_use() {
        vec![1, 2, 3].into_kind();
    }

    #[test]
    fn test_unkind_ref() {
        let vec = vec![1, 2, 3];
        let v = vec.clone().into_kind();
        //let r = v.unkind_ref();
        //assert_eq!(r, &vec);
    }

    #[test]
    fn test_macro() {
        //derive_hkt!(Option);

        //let f = Some(1).into_kind();
        //assert_eq!(f.reify(), Some(1))
    }
}
