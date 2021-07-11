use std::io::{Error, Read, Write};

pub fn upcase(
    _input: &mut impl Read,
    _output: &mut impl Write
) -> Result<(), Error> {
  Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_upcased_input_to_output() {
	let mut output: Vec<u8> = Vec::new();

	upcase(&mut "Hello, world!\n".as_bytes(), &mut output).unwrap();
	assert_eq!(&output, b"HELLO, WORLD!\n");
    }
}
