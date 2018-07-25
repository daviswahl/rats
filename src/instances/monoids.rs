use monoid::Monoid;

impl Monoid<i32> for i32 {
    fn empty() -> i32 {
        0
    }
}
