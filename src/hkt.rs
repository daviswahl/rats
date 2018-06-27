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
    #[test]
    fn test_macro() {
        //derive_hkt!(Option);

        //let f = Some(1).into_kind();
        //assert_eq!(f.reify(), Some(1))
    }
}
