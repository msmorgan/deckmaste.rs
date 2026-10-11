use deckmaste_lexical::{LexicalReading, Lexicon};
use deckmaste_lexical_source::load_workspace;
use std::path::Path;

#[test]
fn doubled_core_verbs_retain_their_authored_past_participles() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let lexicon = Lexicon::new(load_workspace(&root).unwrap().lexemes).unwrap();
    for (owner, surface) in [
        ("core-verb:Control", "controlled"),
        ("core-verb:Flip", "flipped"),
        ("core-verb:Skip", "skipped"),
    ] {
        let analysis = lexicon.analyze(surface);
        let participle = analysis
            .matches
            .iter()
            .find_map(|entry| {
                let LexicalReading::Word(value) = &entry.reading else {
                    return None;
                };
                (value.lexeme.as_str() == owner
                    && value.form == deckmaste_lexical::WordForm::PastParticiple)
                    .then_some(&entry.reading)
            })
            .unwrap_or_else(|| panic!("missing past participle of {owner}"));
        assert_eq!(lexicon.realize(participle).unwrap(), surface);
    }
}
