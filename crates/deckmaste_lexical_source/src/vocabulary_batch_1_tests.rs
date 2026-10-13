use std::collections::BTreeSet;
use std::path::Path;
use std::sync::LazyLock;

use deckmaste_lexical::{
    Capitalization, Category, Countability, Finiteness, Lexeme, LexemeId, LexicalReading, Lexicon,
    Number, Person, SourceKind, SurfaceCase, SurfaceStructure, Tense, WordForm,
};

static LEXICON: LazyLock<Lexicon> = LazyLock::new(|| {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    Lexicon::new(
        crate::load_workspace(&root, &deckmaste_english_v3::grammar::lexical_registry())
            .unwrap()
            .lexemes,
    )
    .unwrap()
});

fn entry(owner: &str) -> &'static Lexeme {
    &LEXICON.lexemes()[&LexemeId::new(owner)]
}

fn spellings(owner: &str, form: WordForm) -> BTreeSet<String> {
    LEXICON
        .values()
        .filter(|v| {
            v.lexeme == LexemeId::new(owner)
                && v.form == form
                && v.capitalization == SurfaceCase::Declared
        })
        .map(|v| LEXICON.realize(&LexicalReading::Word(v.clone())).unwrap())
        .collect()
}

#[test]
fn nouns_preserve_countability_and_compound_boundaries() {
    for (owner, countability, structure, singular, plural) in [
        (
            "lexeme:CommonNoun/Pile",
            Countability::Count,
            SurfaceStructure::Word,
            "pile",
            Some("piles"),
        ),
        (
            "lexeme:CommonNoun/Text",
            Countability::Mass,
            SurfaceStructure::Word,
            "text",
            None,
        ),
        (
            "lexeme:CommonNoun/LifeTotal",
            Countability::Count,
            SurfaceStructure::Multiword,
            "life total",
            Some("life totals"),
        ),
        (
            "lexeme:CommonNoun/ManaSymbol",
            Countability::Count,
            SurfaceStructure::Multiword,
            "mana symbol",
            Some("mana symbols"),
        ),
        (
            "lexeme:CommonNoun/ColorIdentity",
            Countability::Count,
            SurfaceStructure::Multiword,
            "color identity",
            Some("color identities"),
        ),
        (
            "lexeme:CommonNoun/ColorWord",
            Countability::Count,
            SurfaceStructure::Multiword,
            "color word",
            Some("color words"),
        ),
    ] {
        let noun = entry(owner);
        assert_eq!(noun.category, Category::Noun);
        assert_eq!(noun.properties.countability, vec![countability]);
        assert_eq!(noun.surface_structure, structure);
        assert_eq!(
            spellings(owner, WordForm::Singular),
            BTreeSet::from([singular.into()])
        );
        assert_eq!(
            spellings(owner, WordForm::Plural),
            plural.into_iter().map(String::from).collect()
        );
        assert_eq!(noun.source.kind, SourceKind::Core);
        assert_eq!(noun.source.owner, owner);
    }
    assert_eq!(
        entry("lexeme:CommonNoun/Activation").properties.features["NounPremodifier"],
        "Yes"
    );
    assert_eq!(
        entry("lexeme:CommonNoun/LifeTotal").properties.features["ScalarDenotation"],
        "Yes"
    );
}

#[test]
fn ordinary_adjectives_have_invariant_forms() {
    for word in [
        "Starting",
        "Opening",
        "Excess",
        "Unspent",
        "Unblocked",
        "Odd",
        "Even",
        "Highest",
        "Lowest",
        "Colored",
    ] {
        let class = if ["Starting", "Opening", "Excess", "Highest", "Lowest"].contains(&word) {
            "AttributiveAdjective"
        } else {
            "Adjective"
        };
        let owner = format!("vocab:{class}/{word}");
        let adjective = entry(&owner);
        assert_eq!(adjective.category, Category::Adjective);
        assert_eq!(
            adjective.forms.iter().map(|f| f.form).collect::<Vec<_>>(),
            vec![WordForm::Invariant]
        );
        assert_eq!(
            spellings(&owner, WordForm::Invariant),
            BTreeSet::from([word.to_lowercase()])
        );
    }
}

#[test]
fn central_adverbs_keep_their_declared_positions() {
    for (word, position) in [("Secretly", "Imperative"), ("Fully", "Preverbal")] {
        let adverb = entry(&format!("vocab:Adverb/{word}"));
        assert_eq!(adverb.category, Category::Adverb);
        assert_eq!(
            adverb.properties.features["CentralAdjunctLicence"],
            position
        );
        assert_eq!(adverb.properties.features["VPFinalAdjunct"], "No");
        assert_eq!(adverb.properties.features["ClauseInitialAdjunct"], "No");
    }
}

#[test]
fn verbs_reuse_existing_complementation_and_derive_inflections() {
    let cases: &[(&str, &[&str])] = &[
        ("Declare", &["Draw"]),
        ("Take", &["Draw"]),
        ("Begin", &["Draw"]),
        ("Complete", &["Draw"]),
        ("Note", &["Draw"]),
        ("Follow", &["Draw"]),
        ("Switch", &["Draw"]),
        ("Move", &["Put"]),
        ("Separate", &["Put"]),
    ];
    for (verb, models) in cases {
        let actual = entry(&format!("core-verb:{verb}"));
        assert_eq!(actual.properties.frames.len(), models.len());
        for (frame, model) in actual.properties.frames.iter().zip(*models) {
            let reference = entry(&format!("core-verb:{model}"));
            let intended = reference
                .properties
                .frames
                .iter()
                .find(|f| f.items == frame.items)
                .unwrap();
            assert_eq!(frame, intended);
        }
    }
    let finite_begin = entry("core-verb:BeginIntransitive");
    assert_eq!(
        finite_begin.properties.frames,
        vec![entry("core-verb:Enter").properties.frames[0].clone()]
    );
    assert_eq!(finite_begin.forms.len(), 1);
    assert_eq!(finite_begin.forms[0].form, WordForm::Present);
    assert_eq!(
        finite_begin.forms[0].features,
        deckmaste_lexical::FeatureBundle {
            number: Some(Number::Singular),
            person: Some(Person::Third),
            tense: Some(Tense::Present),
            finiteness: Some(Finiteness::Finite),
            case: None,
        }
    );
    assert_eq!(
        spellings("core-verb:BeginIntransitive", WordForm::Present),
        BTreeSet::from(["begins".into()])
    );
    assert_eq!(
        spellings("core-verb:BeginIntransitive", WordForm::GerundParticiple),
        BTreeSet::new()
    );
    assert_eq!(
        spellings("core-verb:Switch", WordForm::Present),
        BTreeSet::from(["switch".into(), "switches".into()])
    );
    for (verb, preterite, participle, gerund) in [
        ("Take", "took", "taken", "taking"),
        ("Begin", "began", "begun", "beginning"),
        ("Move", "moved", "moved", "moving"),
        ("Switch", "switched", "switched", "switching"),
    ] {
        let owner = format!("core-verb:{verb}");
        for (form, text) in [
            (WordForm::Preterite, preterite),
            (WordForm::PastParticiple, participle),
            (WordForm::GerundParticiple, gerund),
        ] {
            assert_eq!(spellings(&owner, form), BTreeSet::from([text.into()]));
        }
    }
}

#[test]
fn negative_have_has_only_finite_present_concord_slots() {
    let auxiliary = entry("core-verb:HaveNegative");
    assert_eq!(auxiliary.category, Category::Verb);
    assert_eq!(auxiliary.forms.len(), 6);
    assert_eq!(
        auxiliary.properties.frames,
        entry("core-verb:Have")
            .properties
            .frames
            .iter()
            .filter(|f| f.kind == "Auxiliary")
            .cloned()
            .collect::<Vec<_>>()
    );
    assert_eq!(auxiliary.properties.features["AuxiliaryUse"], "Yes");
    for slot in &auxiliary.forms {
        assert_eq!(slot.form, WordForm::Present);
        assert_eq!(slot.features.tense, Some(Tense::Present));
        assert_eq!(slot.features.finiteness, Some(Finiteness::Finite));
        let third_singular = slot.features.number == Some(Number::Singular)
            && slot.features.person == Some(Person::Third);
        assert_eq!(
            slot.surfaces,
            Some(vec![
                if third_singular { "hasn't" } else { "haven't" }.into()
            ])
        );
    }
    assert_eq!(
        spellings("core-verb:HaveNegative", WordForm::Plain),
        BTreeSet::new()
    );
}

#[test]
fn keyword_adjectives_keep_the_keyword_and_plugin_provenance() {
    for (keyword, adjective) in [
        ("bargain", "bargained"),
        ("suspend", "suspended"),
        ("warp", "warped"),
    ] {
        let owner = format!("lexeme:keyword_ability/{keyword}");
        let original = entry(&owner);
        let derived = entry(&format!("{owner}/adjective"));
        assert_eq!(original.category, Category::Keyword);
        assert_eq!(derived.category, Category::Adjective);
        assert_eq!(derived.source, original.source);
        assert_eq!(derived.source.kind, SourceKind::Keyword);
        assert_eq!(derived.lemma, adjective);
        assert_eq!(derived.properties.features["BareSingularUse"], "No");
        assert_eq!(
            spellings(&format!("{owner}/adjective"), WordForm::Invariant),
            BTreeSet::from([adjective.into()])
        );
    }
}

#[test]
fn cadet_is_an_exact_token_name() {
    let name = entry("lexeme:Name/Cadet");
    assert_eq!(name.category, Category::Catalog);
    assert_eq!(name.capitalization, Capitalization::Exact);
    assert_eq!(name.surface_structure, SurfaceStructure::Opaque);
    assert_eq!(name.properties.features["IdentityUse"], "Name");
    assert_eq!(
        spellings("lexeme:Name/Cadet", WordForm::Invariant),
        BTreeSet::from(["Cadet".into()])
    );
}
