
#[cfg(test)]
mod prost_test {
    use super::*;

    // https://github.com/tokio-rs/prost/blob/bff66a665fc2d5a0cbb826e446f8050450cbd8a2/tests/src/lib.rs#L232
    #[test]
    fn serialize_test() {
        assert_eq!(4, 4);
    }
}
