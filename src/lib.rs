pub mod functor;
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn functor() {
        assert_eq!("foo", "bar")
    }
}