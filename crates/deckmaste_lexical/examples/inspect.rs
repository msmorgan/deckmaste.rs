//! Inspect exported lexical data without linking the construction
//! grammar/compiler.

use std::path::PathBuf;

use clap::Parser;
use deckmaste_lexical::Lexeme;
use deckmaste_lexical::Lexicon;

#[derive(Parser)]
struct Args {
    /// RON declarations exported by `cargo xtask lexical --export`.
    declarations: PathBuf,
    text: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let declarations: Vec<Lexeme> = ron::from_str(&std::fs::read_to_string(args.declarations)?)?;
    let lexicon = Lexicon::new(declarations)?;
    let analyzed = lexicon.analyze(&args.text);
    println!("{}", serde_json::to_string_pretty(&analyzed.matches)?);
    for word in analyzed.unknown_words() {
        if let Some(bytes) = analyzed.byte_range(word.start, word.end) {
            eprintln!("unknown {:?} at {bytes:?}", &args.text[bytes.clone()]);
        }
    }
    Ok(())
}
