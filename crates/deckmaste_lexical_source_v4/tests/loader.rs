//! The loader rejects declarations that would break the catalog contract.

use deckmaste_lexical_source_v4::Sources;
use deckmaste_lexical_source_v4::schema::FrameSpec;
use deckmaste_lexical_source_v4::schema::NounDecl;
use deckmaste_lexical_source_v4::schema::VerbEntry;

fn verb(lemma: &str, frames: Vec<FrameSpec>) -> VerbEntry {
    VerbEntry {
        lemma: lemma.to_owned(),
        third_singular: None,
        past: None,
        preterite: None,
        past_participle: None,
        gerund: None,
        irrealis: None,
        forms: Vec::new(),
        finite_only: false,
        negative: false,
        binding: None,
        frames,
    }
}

fn rejects(sources: &Sources, reason: &str) {
    let error = sources.vocabulary().expect_err(reason).to_string();
    assert!(error.contains(reason), "{error}");
}

#[test]
fn a_catalog_word_is_never_core_vocabulary() {
    let mut sources = Sources::load().unwrap();
    let action = sources.keyword_actions[0].verbs[0].clone();
    sources.verbs.push(action);
    rejects(&sources, "must not also be core vocabulary");

    let mut sources = Sources::load().unwrap();
    let card_type = sources.types.card_types[0].clone();
    sources.nouns.push(card_type);
    rejects(&sources, "must not also be core vocabulary");
}

#[test]
fn a_verb_has_a_frame() {
    let mut sources = Sources::load().unwrap();
    sources.verbs.push(verb("frobnicate", Vec::new()));
    rejects(&sources, "at least one Subcategorization Frame");
}

#[test]
fn a_frame_names_declared_prepositions() {
    let mut sources = Sources::load().unwrap();
    sources.verbs.push(verb(
        "frobnicate",
        vec![FrameSpec::Prepositional(vec!["amidst-ish".to_owned()])],
    ));
    rejects(&sources, "not a declared preposition");
}

#[test]
fn a_frame_belongs_to_its_head_category() {
    let mut sources = Sources::load().unwrap();
    sources.nouns.push(NounDecl::Entry(
        deckmaste_lexical_source_v4::schema::NounEntry {
            lemma: "frobnication".to_owned(),
            countability: vec![deckmaste_lexical::Countability::Count],
            plural: None,
            frames: vec![FrameSpec::Monotransitive],
        },
    ));
    rejects(&sources, "is not a frame of a Noun");
}

#[test]
fn an_identity_is_declared_once() {
    let mut sources = Sources::load().unwrap();
    let noun = sources.nouns[0].clone();
    sources.nouns.push(noun);
    rejects(&sources, "is declared twice");
}
