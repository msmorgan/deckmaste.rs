//! A Valence schema's `internalised` equations constrain the Internalised Complement
//! of every derived passive: with them, *reduced by 2* is only the `PP[by]`
//! extent its frame selects, and *reduced by Zombies* keeps its Internalised
//! Complement (CGEL,
//! Ch. 8 §5.2, p. 692; Ch. 16 §10.1.1, p. 1428).

use deckmaste_lexical::Category;
use deckmaste_lexical::Countability;
use deckmaste_lexical::Lexeme;
use deckmaste_lexical::Lexicon;
use deckmaste_lexical::Source;
use deckmaste_lexical::SourceKind;
use deckmaste_syntax::CompiledGrammar;
use deckmaste_syntax::FrameCatalog;
use deckmaste_syntax::Grammar;
use deckmaste_syntax::Reading;
use deckmaste_syntax::frames::FrameFamily;

fn lexicon() -> Lexicon {
    let source = |kind| Source {
        kind,
        path: "tests/internalised_complement.rs".to_owned(),
        owner: "test".to_owned(),
    };
    Lexicon::new([
        Lexeme::verb("v:reduce", "reduce", source(SourceKind::Core)),
        Lexeme::noun(
            "n:Zombie",
            "Zombie",
            vec![Countability::Count],
            source(SourceKind::Catalog),
        ),
        Lexeme::invariant(
            "b:by",
            "by",
            Category::Preposition,
            source(SourceKind::Core),
        ),
    ])
    .unwrap()
}

const FRAMES: &str = r#"(
    frames: {
        "v:reduce": [
            (family: Monotransitive, slots: [(relation: Object, fill: NP)]),
            (family: TransitiveWithPP, slots: [
                (relation: Object, fill: NP),
                (relation: Complement, fill: PP(["b:by"])),
            ]),
        ],
    },
    prepositions: {
        "b:by": [
            (complement: NP, hosts: ComplementOnly),
            (complement: Amount, hosts: ComplementOnly),
        ],
    },
)"#;

/// The grammar, with `internalised` spliced into its verb Valence schema.
fn grammar(lexicon: &Lexicon, internalised: &str) -> Result<CompiledGrammar, deckmaste_syntax::LoadError> {
    let source = format!(
        r#"(
    root: "Sentence",
    features: {{
        "VerbForm": ["Plain", "Finite", "GerundParticiple", "PastParticiple"],
        "Voice": ["Active", "Passive"],
        "ComplementKind": ["NP", "Amount"],
    }},
    categories: {{
        "Sentence": (),
        "VerbPhrase": (features: ["VerbForm", "Voice"], gap: true),
        "NounPhrase": (),
        "Nominal": (),
        "AmountPhrase": (),
        "PrepositionPhrase": (features: ["ComplementKind"]),
    }},
    closed_words: {{ "By": "b:by" }},
    slot_categories: {{
        NP: "NounPhrase",
        PP: "PrepositionPhrase",
        Nominal: "Nominal",
        Amount: "AmountPhrase",
    }},
    productions: [
        (name: "Participial", lhs: "Sentence", rhs: [Cat("clause", "VerbPhrase"), Punct('.')],
            equations: [Require("clause.VerbForm", "PastParticiple"), Require("clause.Voice", "Passive")]),
        (name: "Numeral", lhs: "AmountPhrase", rhs: [Lex("head", Numeral)]),
        (name: "Bare", lhs: "NounPhrase", rhs: [Cat("head", "Nominal")]),
    ],
    schemas: [
        Valence(head: Verb, phrase: "VerbPhrase", passive: Some("By"), internalised: [{internalised}]),
        Valence(head: Preposition, phrase: "PrepositionPhrase"),
        Valence(head: Noun, phrase: "Nominal"),
    ],
)"#
    );
    CompiledGrammar::bind(
        &Grammar::from_ron_str(&source).unwrap(),
        lexicon,
        &FrameCatalog::from_ron_str(FRAMES).unwrap(),
    )
}

fn families(internalised: &str, text: &str) -> Vec<Option<FrameFamily>> {
    let lexicon = lexicon();
    let grammar = grammar(&lexicon, internalised).unwrap();
    let readings = grammar.parse(&lexicon, text).readings();
    for reading in &readings {
        assert_eq!(grammar.admit(reading, &lexicon), Ok(()));
        assert_eq!(grammar.realize(reading, &lexicon).unwrap(), text);
    }
    let mut families: Vec<_> = readings.iter().map(family).collect();
    families.sort_by_key(|f| format!("{f:?}"));
    families
}

fn family(reading: &Reading) -> Option<FrameFamily> {
    let clause = reading.node()?.children.first()?.node()?;
    clause.selected_frame.as_ref().map(|frame| frame.family)
}

const BY_NP: &str = r#"Require("internalised.ComplementKind", "NP")"#;

#[test]
fn an_unconstrained_internalised_complement_doubles_a_selected_by_extent() {
    assert_eq!(
        families("", "Reduced by 2."),
        [Some(FrameFamily::Monotransitive), Some(FrameFamily::TransitiveWithPP)]
    );
}

#[test]
fn an_amount_is_only_the_selected_by_extent() {
    assert_eq!(
        families(BY_NP, "Reduced by 2."),
        [Some(FrameFamily::TransitiveWithPP)]
    );
}

#[test]
fn a_noun_phrase_stays_an_internalised_complement() {
    assert!(families(BY_NP, "Reduced by Zombies.").contains(&Some(FrameFamily::Monotransitive)));
}

#[test]
fn an_internalised_equation_names_a_carried_feature() {
    let lexicon = lexicon();
    assert!(grammar(&lexicon, r#"Require("internalised.VerbForm", "Plain")"#).is_err());
}
