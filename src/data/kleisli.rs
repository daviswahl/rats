use lifted::Lifted;
use monad::Monad;
use std::boxed::FnBox;
use std::marker::PhantomData;

// Needing an FnMut here seems wrong but I think the correct fix is tied up
// in doing lifetimes properly.
pub struct Kleisli<'a, F: 'static, A, B: 'a> {
    run: Box<FnBox(A) -> Lifted<'a, F, B> + 'a>,
}

impl<'a, F, A, B> FnOnce<(A,)> for Kleisli<'a, F, A, B> {
    type Output = Lifted<'a, F, B>;

    extern "rust-call" fn call_once(self, args: (A,)) -> Self::Output {
        self.run.call_once(args)
    }
}

impl<'a, F: 'static, A, B: 'a> Kleisli<'a, F, A, B> {
    pub fn new<Func>(func: Func) -> Self
    where
        Func: FnMut(A) -> Lifted<'a, F, B> + 'a,
    {
        Kleisli {
            run: Box::new(func),
        }
    }
}

impl<'a, F, A, B> Kleisli<'a, F, A, B> where {
    fn compose<Z: 'a>(self, mut k: Kleisli<'a, F, Z, A>) -> Kleisli<'a, F, Z, B>
    where
        F: Monad<'a, F>,
    {
        Kleisli {
            run: Box::new((move |z| F::flat_map(k(z), self))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use instances::option::OptionKind;
    use lifted::*;

    #[test]
    fn test_compose() {
        let parse = Kleisli::new(|s: String| {
            match s.parse::<i32>() {
                Ok(i) => Some(i),
                Err(_) => None,
            }.lift()
        });

        let reciprocal =
            Kleisli::new(|i: i32| if i != 0 { Some(1.0 / i as f32) } else { None }.lift());

        let parse_and_recriprocal = reciprocal.compose(parse);

        assert_eq!(
            parse_and_recriprocal("123".to_owned()).unlift(),
            Some(0.008130081)
        )
    }
}
