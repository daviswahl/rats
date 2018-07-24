use lifted::{Lifted, Nothing};
pub struct OptionT<F, A, B = Nothing, G = Nothing> {
    value: Box<Lifted<F, A, B, G>>,
}
