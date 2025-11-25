use jacklang::lexer::Lexer;
use std::fs;
use std::io::Error;
use std::path::Path;

fn main() -> Result<(), Error> {
    let path = Path::new("examples/testing.jack");
    let src = fs::read_to_string(path)?;

    let mut lexer = Lexer::new(src);
    lexer.scan();

    Ok(())
}
