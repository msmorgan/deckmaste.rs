use std::path::Path;
use std::sync::LazyLock;

use deckmaste_lexical::{
    Capitalization, Category, Countability, Lexeme, LexemeId, LexicalReading, Lexicon, SourceKind,
    SurfaceCase, SurfaceStructure, WordForm,
};

static LEXICON: LazyLock<Lexicon> = LazyLock::new(|| {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    Lexicon::new(crate::load_workspace(&root).unwrap().lexemes).unwrap()
});

fn entry(owner: &str) -> &'static Lexeme {
    &LEXICON.lexemes()[&LexemeId::new(owner)]
}

#[test]
fn native_nouns_names_and_adjectives_retain_their_declared_classes() {
    assert_eq!(
        entry("lexeme:CommonNoun/Quality").properties.countability,
        vec![Countability::Count]
    );
    assert_eq!(
        entry("lexeme:CommonNoun/Bidding").properties.countability,
        vec![Countability::Mass]
    );
    assert_eq!(
        entry("lexeme:CommonNoun/Chapter").properties.features["NounPremodifier"],
        "Yes"
    );
    let name = entry("lexeme:Name/KomasCoil");
    assert_eq!(name.category, Category::Catalog);
    assert_eq!(name.properties.features["IdentityUse"], "Name");
    assert_eq!(name.capitalization, Capitalization::Exact);
    assert_eq!(name.surface_structure, SurfaceStructure::Opaque);
    for owner in [
        "vocab:AttributiveAdjective/DoubleFaced",
        "vocab:AttributiveAdjective/Recent",
    ] {
        let adjective = entry(owner);
        assert_eq!(adjective.category, Category::Adjective);
        assert_eq!(
            adjective.properties.features["ModifierLicense"],
            "Unrestricted"
        );
        assert_eq!(
            adjective.properties.features["HomographLicense"],
            "Unlicensed"
        );
    }
    assert_eq!(
        entry("vocab:AttributiveAdjective/Fewest")
            .properties
            .features["QuantitativeAdjectiveComplementUse"],
        "Yes"
    );
}

#[test]
fn position_licences_follow_the_attested_adverbs_and_prepositions() {
    let simultaneous = &entry("vocab:Adverb/Simultaneously").properties.features;
    assert_eq!(simultaneous["VPFinalAdjunct"], "Yes");
    assert_eq!(simultaneous["ClauseInitialAdjunct"], "Yes");
    assert_eq!(
        simultaneous["CentralAdjunctLicence"],
        "Preverbal|Imperative"
    );
    for owner in ["vocab:Adverb/Yet", "vocab:Adverb/Wrong"] {
        let features = &entry(owner).properties.features;
        assert_eq!(features["VPFinalAdjunct"], "Yes");
        assert_eq!(features["ClauseInitialAdjunct"], "No");
        assert_eq!(features["CentralAdjunctLicence"], "None");
    }
    for owner in ["vocab:Preposition/Above", "vocab:Preposition/Beneath"] {
        let features = &entry(owner).properties.features;
        assert_eq!(features["PrepositionComplement"], "NounPhrase");
        assert_eq!(
            features["PrepositionFunctionLicence"],
            "Modifier|PredicativeComplement"
        );
        assert_eq!(features["LocativeUse"], "Yes");
    }
}

#[test]
fn verb_frames_and_inflections_preserve_their_declared_uses() {
    let deal = entry("core-verb:Deal")
        .properties
        .frames
        .iter()
        .find(|frame| frame.items.len() == 2)
        .unwrap();
    assert_eq!(
        entry("core-verb:Give").properties.frames,
        vec![deal.clone()]
    );
    assert_eq!(
        entry("core-verb:Lock").properties.features["PastParticipialPremodifier"],
        "Yes"
    );
    let bidding: Vec<_> = LEXICON
        .values()
        .filter(|value| {
            value.lexeme == LexemeId::new("core-verb:Bid")
                && value.form == WordForm::GerundParticiple
                && value.capitalization == SurfaceCase::Declared
        })
        .map(|value| {
            LEXICON
                .realize(&LexicalReading::Word(value.clone()))
                .unwrap()
        })
        .collect();
    assert_eq!(bidding, vec!["bidding"]);
}

#[test]
fn plugin_owned_spelling_and_plural_keep_their_owners() {
    let adjective = entry("lexeme:keyword_ability/plot/adjective");
    assert_eq!(adjective.lemma, "plotted");
    assert_eq!(adjective.category, Category::Adjective);
    assert_eq!(adjective.source.kind, SourceKind::Keyword);
    assert_eq!(adjective.source.owner, "lexeme:keyword_ability/plot");
    let jace = entry("lexeme:planeswalker_subtype/jace");
    let plurals: Vec<_> = LEXICON
        .values()
        .filter(|value| {
            value.lexeme == jace.id
                && value.form == WordForm::Plural
                && value.capitalization == SurfaceCase::Declared
        })
        .map(|value| {
            LEXICON
                .realize(&LexicalReading::Word(value.clone()))
                .unwrap()
        })
        .collect();
    assert_eq!(plurals, vec!["Jaces"]);
}
