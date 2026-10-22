//! The loader rejects declarations that would break the catalog contract.

use deckmaste_lexical_source_v4::AttestedShortenings;
use deckmaste_lexical_source_v4::Sources;
use deckmaste_lexical_source_v4::schema::AnchorWord;
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
        form_of: None,
        binding: None,
        frames,
        active_only: false,
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
fn only_a_verb_with_frames_restricts_them_to_active_voice() {
    let mut sources = Sources::load().unwrap();
    let mut auxiliary = verb("frobnicate", Vec::new());
    auxiliary.active_only = true;
    sources.closed_class.auxiliaries.push(auxiliary);
    rejects(&sources, "restricts them to active voice");
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
            distribution: deckmaste_lexical_source_v4::schema::NounDistribution::Ordinary,
            kind: deckmaste_lexical_source_v4::schema::NounKind::Ordinary,
            parameters: Vec::new(),
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

#[test]
fn a_card_scope_names_cards_and_an_introduced_name_is_none() {
    let card_names = "Dark Depths\nMarit Lage's Slumber\nCitadel Siege\n";
    let mut sources = Sources::load().unwrap();
    sources
        .introduced_names
        .retain(|name| name.name == "Marit Lage");
    sources.anchor_words.clear();
    let none = AttestedShortenings::default();
    sources.environment(card_names, &none).unwrap();

    let mut stray = sources.clone();
    stray.introduced_names[0]
        .cards
        .push("Not a Card".to_owned());
    let error = stray
        .environment(card_names, &none)
        .unwrap_err()
        .to_string();
    assert!(error.contains("which is not a card name"), "{error}");

    let mut stray = sources.clone();
    stray.anchor_words.push(AnchorWord {
        word: "Khans".to_owned(),
        cards: vec!["Not a Card".to_owned()],
    });
    let error = stray
        .environment(card_names, &none)
        .unwrap_err()
        .to_string();
    assert!(error.contains("which is not a card name"), "{error}");

    let error = sources
        .environment(&format!("{card_names}Marit Lage\n"), &none)
        .unwrap_err()
        .to_string();
    assert!(error.contains("is a card name"), "{error}");
}
