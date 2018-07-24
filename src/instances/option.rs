use functor::Functor;
use lifted::{Lifted, Nothing, HKT};
use lifted::Unlift;

struct OptionKind;

impl HKT for OptionKind {}

impl Functor<OptionKind> for OptionKind {
    fn map<Func, A, B>(
        fa: Lifted<OptionKind, A, Self::B, Self::G, Self::Func>,
        func: Func,
    ) -> Lifted<OptionKind, A, Self::B, Self::G, Self::Func>
    where
        Func: Fn(A) -> B,
    {
        fa.unlift()
    }
}

impl<A> Unlift for Lifted<OptionKind,A> {
    type Out = Option<A>;

    fn unlift(&self) -> <Self as Unlift<F>>::Out {
        
    }
}