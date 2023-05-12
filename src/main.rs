use std::io;

fn main() -> io::Result<()> {
    upcase::upcase(&mut io::stdin(), &mut io::stdout())
}
