use lifted::{Lifted, Nothing};
pub struct OptionT<'a, F, A, B = Nothing, G = Nothing>
where
    F: 'a,
    A: 'a,
    B: 'a,
    G: 'a,
{
    value: Box<Lifted<'a, F, A, B, G>>,
}
