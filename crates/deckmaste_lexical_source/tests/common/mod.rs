use std::path::Path;
use std::sync::LazyLock;

use deckmaste_lexical::Lexicon;
use deckmaste_lexical_source::LexicalSources;

pub static SOURCES: LazyLock<LexicalSources> = LazyLock::new(|| {
    deckmaste_lexical_source::load_workspace(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."))
        .unwrap()
});

pub static LEXICON: LazyLock<Lexicon> =
    LazyLock::new(|| Lexicon::new(SOURCES.lexemes.clone()).unwrap());
