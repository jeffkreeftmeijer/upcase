#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_utf8_bytes_eq {
        ($left:expr, $right:expr) => {
            if ($left != $right) {
                panic!(
                    "assertion failed: `(left == right)`\n  left: `{:?}`\n right: `{:?}`",
                    String::from_utf8_lossy($left),
                    String::from_utf8_lossy($right),
                )
            }
        };
    }

    #[test]
    fn writes_upcased_input_to_output() {
        let mut output: Vec<u8> = Vec::new();

        upcase(&mut "uppercase!\n".as_bytes(), &mut output).unwrap();
        assert_utf8_bytes_eq!(&output, b"UPPERCASE!\n");
    }
}
