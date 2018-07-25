use semigroup::Semigroup;

impl Semigroup<i32> for i32 {
    fn combine(x: i32, y: i32) -> i32 {
        x + y
    }
}
