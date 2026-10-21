//! The parser-engine obligations of the engine contract
//! (`docs/decisions/english-lexical-analysis.md` §§1–2), each over a small
//! hand grammar: lexical alternatives reach the chart unchosen, packing keeps
//! exactly the distinctions a parent can see, Gaps close only at their
//! governor, unary cycles hide nothing, and both round-trip laws hold for
//! every declared lexical value and for small complete phrase families.

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use deckmaste_lexical::AnalyzedText;
use deckmaste_lexical::Category;
use deckmaste_lexical::Countability;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::FormDeclaration;
use deckmaste_lexical::Lexeme;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Lexicon;
use deckmaste_lexical::Number;
use deckmaste_lexical::Person;
use deckmaste_lexical::Source;
use deckmaste_lexical::SourceKind;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::SurfaceStructure;
use deckmaste_lexical::WordForm;
use deckmaste_syntax::AdmissionError;
use deckmaste_syntax::CompiledGrammar;
use deckmaste_syntax::FrameCatalog;
use deckmaste_syntax::Gap;
use deckmaste_syntax::Grammar;
use deckmaste_syntax::LoadError;
use deckmaste_syntax::Reading;
use deckmaste_syntax::ReadingNode;
use dpsi::Ident;

fn core() -> Source {
    Source {
        kind: SourceKind::Core,
        path: "tests/contract.rs".to_owned(),
        owner: "test".to_owned(),
    }
}

fn noun(lemma: &str) -> Lexeme {
    Lexeme::noun(
        format!("n:{lemma}"),
        lemma,
        vec![Countability::Count],
        core(),
    )
}

/// A count noun whose plural is spelled like its singular.
fn zero_plural(lemma: &str) -> Lexeme {
    let mut lexeme = noun(lemma);
    for form in &mut lexeme.forms {
        if form.form == WordForm::Plural {
            form.surfaces = Some(vec![lemma.to_owned()]);
        }
    }
    lexeme
}

fn word(id: &str, lemma: &str, category: Category) -> Lexeme {
    Lexeme::invariant(id, lemma, category, core())
}

/// A personal pronoun with one invariant form per (number, person) bundle.
fn pronoun(id: &str, lemma: &str, bundles: &[(Number, Person)]) -> Lexeme {
    let mut lexeme = word(id, lemma, Category::Pronoun);
    lexeme.forms = bundles
        .iter()
        .map(|&(number, person)| FormDeclaration {
            form: WordForm::Invariant,
            features: FeatureBundle {
                number: Some(number),
                person: Some(person),
                ..FeatureBundle::default()
            },
            surfaces: None,
        })
        .collect();
    lexeme
}

fn bind(grammar: &str, lexicon: &Lexicon, frames: &str) -> Result<CompiledGrammar, LoadError> {
    CompiledGrammar::bind(
        &Grammar::from_ron_str(grammar)?,
        lexicon,
        &FrameCatalog::from_ron_str(frames)?,
    )
}

const NO_FRAMES: &str = "(frames: {})";

/// Every Reading of an analyzed text, after checking Admission and both
/// round-trip laws on each: it realizes as the text, the forest realizes as
/// the text, and it is a Reading of the parse of its realization.
fn checked(
    grammar: &CompiledGrammar,
    lexicon: &Lexicon,
    analyzed: &AnalyzedText,
) -> BTreeSet<Reading> {
    let parse = grammar.parse_analyzed(analyzed);
    let text = analyzed.text();
    let readings: BTreeSet<Reading> = parse.readings().into_iter().collect();
    if !readings.is_empty() {
        assert_eq!(parse.verify_realization(lexicon), Ok(()), "{text}");
    }
    for reading in &readings {
        assert_eq!(grammar.admit(reading, lexicon), Ok(()), "{reading:#?}");
        let surface = grammar.realize(reading, lexicon).unwrap();
        assert_eq!(surface, text);
        assert!(
            grammar.parse(lexicon, &surface).contains(reading),
            "{reading:#?}"
        );
    }
    readings
}

fn readings(grammar: &CompiledGrammar, lexicon: &Lexicon, text: &str) -> BTreeSet<Reading> {
    checked(grammar, lexicon, &lexicon.analyze(text))
}

fn node(
    production: &str,
    category: &str,
    features: &[(&str, &str)],
    children: Vec<Reading>,
) -> Reading {
    Reading::Node(ReadingNode {
        production: Ident::new(production),
        category: Ident::new(category),
        features: features
            .iter()
            .map(|&(f, v)| (Ident::new(f), Ident::new(v)))
            .collect(),
        gap: Gap::None,
        selected_frame: None,
        children,
    })
}

fn leaf(
    lexeme: &str,
    form: WordForm,
    features: FeatureBundle,
    capitalization: SurfaceCase,
) -> Reading {
    Reading::Leaf(LexicalReading::Word(LexicalValue {
        lexeme: lexeme.into(),
        form,
        features,
        variant: 0,
        capitalization,
    }))
}

fn number(n: Number) -> FeatureBundle {
    FeatureBundle {
        number: Some(n),
        ..FeatureBundle::default()
    }
}

fn number_name(n: Number) -> &'static str {
    match n {
        Number::Singular => "Singular",
        Number::Plural => "Plural",
    }
}

/// The lexemes of the leaves of a Reading, in order.
fn lexemes(reading: &Reading) -> Vec<String> {
    match reading {
        Reading::Leaf(LexicalReading::Word(value)) => vec![value.lexeme.as_str().to_owned()],
        Reading::Leaf(_) => vec!["<numeral>".to_owned()],
        Reading::Node(node) => node.children.iter().flat_map(lexemes).collect(),
    }
}

/// One word of a lexical Category, then a full stop.
const WORD: &str = r#"(
    root: "Sentence",
    categories: { "Sentence": (), "Word": () },
    productions: [
        (name: "Sentence", lhs: "Sentence", rhs: [Cat("word", "Word"), Punct('.')]),
        (name: "Noun", lhs: "Word", rhs: [Lex("word", Noun)]),
        (name: "Verb", lhs: "Word", rhs: [Lex("word", Verb)]),
        (name: "Adjective", lhs: "Word", rhs: [Lex("word", Adjective)]),
        (name: "Determinative", lhs: "Word", rhs: [Lex("word", Determinative)]),
        (name: "Numeral", lhs: "Word", rhs: [Lex("word", Numeral)]),
    ],
)"#;

/// The Reading of `Sentence` over one leaf.
fn word_reading(production: &str, value: LexicalReading) -> Reading {
    node(
        "Sentence",
        "Sentence",
        &[],
        vec![node(production, "Word", &[], vec![Reading::Leaf(value)])],
    )
}

fn production_for(category: Category) -> &'static str {
    match category {
        Category::Noun => "Noun",
        Category::Verb => "Verb",
        Category::Adjective => "Adjective",
        Category::Determinative => "Determinative",
        Category::Numeral => "Numeral",
        other => panic!("WORD has no production for {other:?}"),
    }
}

/// Obligations 1 and 11: the chart never chooses among homographs. Every
/// analysis of the text reaches every Production accepting its Category, so
/// the Readings are exactly one per analysis.
#[test]
fn every_homograph_reaches_every_production_accepting_it() {
    let mut cast = Lexeme::verb("v:cast", "cast", core());
    for form in &mut cast.forms {
        if matches!(form.form, WordForm::Preterite | WordForm::PastParticiple) {
            form.surfaces = Some(vec!["cast".to_owned()]);
        }
    }
    let lexicon = Lexicon::new([
        cast,
        noun("cast"),
        noun("one"),
        word("d:one", "one", Category::Determinative),
        word("a:light", "light", Category::Adjective),
        word("a:light-weight", "light", Category::Adjective),
    ])
    .unwrap();
    let grammar = bind(WORD, &lexicon, NO_FRAMES).unwrap();
    for (text, analyses) in [("Cast.", 14), ("One.", 3), ("Light.", 2)] {
        let analyzed = lexicon.analyze(text);
        let word_end = text.chars().count() - 1;
        let expected: BTreeSet<Reading> = analyzed
            .matches
            .iter()
            .filter(|m| m.start == 0 && m.end == word_end)
            .map(|m| {
                let category = match &m.reading {
                    LexicalReading::Word(value) => lexicon.lexemes()[&value.lexeme].category,
                    LexicalReading::Numeral { .. } => Category::Numeral,
                    LexicalReading::FlavorWord { .. } => unreachable!(),
                };
                word_reading(production_for(category), m.reading.clone())
            })
            .collect();
        // Every form of the verb spelled `cast` (plain, five present, six
        // preterite, past participle) and the noun; noun, determinative and
        // numeral `one`; the two adjectives spelled `light`.
        assert_eq!(expected.len(), analyses, "{text}");
        assert_eq!(readings(&grammar, &lexicon, text), expected, "{text}");
    }
}

/// Obligation 1: two Derivations of one tree are one Reading, while
/// `derivation_count` counts both. Production ids are unique, so the
/// duplicate is a lexical alternative supplied twice.
#[test]
fn duplicate_derivations_of_one_tree_are_one_reading() {
    let lexicon = Lexicon::new([noun("card")]).unwrap();
    let grammar = bind(WORD, &lexicon, NO_FRAMES).unwrap();
    let mut analyzed = lexicon.analyze("Card.");
    assert_eq!(analyzed.matches.len(), 1);
    analyzed.matches.push(analyzed.matches[0].clone());
    let parse = grammar.parse_analyzed(&analyzed);
    assert_eq!(parse.derivation_count(), 2);
    assert_eq!(
        checked(&grammar, &lexicon, &analyzed),
        BTreeSet::from([word_reading("Noun", analyzed.matches[0].reading.clone())])
    );
    // Two Productions cannot derive one tree: a tree names its Production.
    let twice = WORD.replace(
        "(name: \"Verb\", lhs: \"Word\"",
        "(name: \"Noun\", lhs: \"Word\"",
    );
    match bind(&twice, &lexicon, NO_FRAMES) {
        Err(LoadError::Invalid(message)) => assert!(message.contains("duplicate production")),
        other => panic!("{other:?}"),
    }
}

/// Obligation 2: a multiword lexeme is one leaf beside the separate-word
/// analysis of the same text, and only its exact separator spells it.
#[test]
fn multiword_lexeme_is_atomic_and_needs_its_separator() {
    let mut first_strike = word("n:first-strike", "first strike", Category::Noun);
    first_strike.surface_structure = SurfaceStructure::Multiword;
    first_strike.forms[0].form = WordForm::Singular;
    first_strike.forms[0].features = number(Number::Singular);
    let words = [
        word("a:first", "first", Category::Adjective),
        noun("strike"),
    ];
    let lexicon = Lexicon::new([first_strike].into_iter().chain(words.clone())).unwrap();
    let source = r#"(
            root: "Sentence",
            categories: { "Sentence": (), "Nominal": () },
            productions: [
                (name: "Sentence", lhs: "Sentence", rhs: [Cat("head", "Nominal"), Punct('.')]),
                (name: "Noun", lhs: "Nominal", rhs: [Lex("head", Noun)]),
                (name: "Premodified", lhs: "Nominal", rhs: [Lex("modifier", Adjective), Cat("head", "Nominal")]),
            ],
        )"#;
    let grammar = bind(source, &lexicon, NO_FRAMES).unwrap();
    let found: BTreeSet<Vec<String>> = readings(&grammar, &lexicon, "First strike.")
        .iter()
        .map(lexemes)
        .collect();
    assert_eq!(found, BTreeSet::from([vec!["n:first-strike".to_owned()],]));
    // Admission rejects the separate-word Reading, whose leaves are values of
    // this lexicon too: its words rebuild the multiword noun.
    let separate = Lexicon::new(words).unwrap();
    let decomposed = bind(source, &separate, NO_FRAMES)
        .unwrap()
        .parse(&separate, "First strike.")
        .readings();
    assert_eq!(decomposed.len(), 1);
    assert!(matches!(
        grammar.admit(&decomposed[0], &lexicon),
        Err(AdmissionError::Rejected { production, .. }) if production == "lexical atomicity"
    ));
    // Neither lexical nor constructional realization inserts two spaces.
    // Reject the source rather than admitting a law-one failure.
    let spaced = grammar.parse(&lexicon, "First  strike.");
    assert_eq!(spaced.readings(), [] as [Reading; 0]);
    assert_eq!(spaced.verify_realization(&lexicon), Ok(()));
}

/// A catalog's multiword lexeme's words stand apart only where they straddle
/// a constituent boundary, each in a child that reaches past the lexeme, as
/// in *[big first] [strike damage]*; never where they rebuild it, alone or
/// with a dependent of one part, as in *first [strike damage]*. A core
/// multiword lexeme's never do (`docs/decisions/english-lexical-analysis.md`
/// §2).
#[test]
fn multiword_lexeme_words_straddle_but_never_rebuild_it() {
    let mut first_strike = word("n:first-strike", "first strike", Category::Noun);
    first_strike.surface_structure = SurfaceStructure::Multiword;
    first_strike.forms[0].form = WordForm::Singular;
    first_strike.forms[0].features = number(Number::Singular);
    let core_first_strike = first_strike.clone();
    first_strike.source.kind = SourceKind::Catalog;
    let others = [
        word("a:big", "big", Category::Adjective),
        word("a:first", "first", Category::Adjective),
        noun("strike"),
        noun("damage"),
    ];
    let lexicon = Lexicon::new([first_strike].into_iter().chain(others.clone())).unwrap();
    let source = r#"(
            root: "Sentence",
            categories: { "Sentence": (), "Nominal": (), "Adjectives": () },
            productions: [
                (name: "Sentence", lhs: "Sentence", rhs: [Cat("head", "Nominal"), Punct('.')]),
                (name: "Split", lhs: "Sentence", rhs: [Cat("modifier", "Adjectives"), Cat("head", "Nominal"), Punct('.')]),
                (name: "Adjectives", lhs: "Adjectives", rhs: [Lex("first", Adjective), Lex("last", Adjective)]),
                (name: "Noun", lhs: "Nominal", rhs: [Lex("head", Noun)]),
                (name: "Premodified", lhs: "Nominal", rhs: [Lex("modifier", Adjective), Cat("head", "Nominal")]),
                (name: "Compound", lhs: "Nominal", rhs: [Lex("modifier", Noun), Cat("head", "Nominal")]),
            ],
        )"#;
    let grammar = bind(source, &lexicon, NO_FRAMES).unwrap();
    let found = |text: &str| -> BTreeSet<Vec<String>> {
        readings(&grammar, &lexicon, text)
            .iter()
            .map(lexemes)
            .collect()
    };
    let words = |ids: &[&str]| ids.iter().map(|&id| id.to_owned()).collect::<Vec<_>>();
    assert_eq!(
        found("Big first strike damage."),
        BTreeSet::from([
            words(&["a:big", "n:first-strike", "n:damage"]),
            words(&["a:big", "a:first", "n:strike", "n:damage"]),
        ])
    );
    // *First* is a sister of *strike damage*, which rebuilds the noun with a
    // dependent of *strike*.
    assert_eq!(
        found("First strike damage."),
        BTreeSet::from([words(&["n:first-strike", "n:damage"])])
    );
    // *First* and *strike* as the whole of `Premodified` rebuild it.
    assert_eq!(
        found("First strike."),
        BTreeSet::from([words(&["n:first-strike"])])
    );
    let core = Lexicon::new([core_first_strike].into_iter().chain(others)).unwrap();
    let grammar = bind(source, &core, NO_FRAMES).unwrap();
    let found: BTreeSet<Vec<String>> = readings(&grammar, &core, "Big first strike damage.")
        .iter()
        .map(lexemes)
        .collect();
    assert_eq!(
        found,
        BTreeSet::from([words(&["a:big", "n:first-strike", "n:damage"])])
    );
}

/// Obligations 3 and 4, discharged by construction: no Category is nullable.
/// A Production with an empty right-hand side is a load error, the engine
/// has no literal symbol (punctuation is one character), and every lexical
/// alternative spans at least one character. So there is no empty-span
/// Derivation to complete, predict, duplicate or keep as a leaf.
#[test]
fn no_category_is_nullable() {
    let lexicon =
        Lexicon::new([noun("one"), word("d:one", "one", Category::Determinative)]).unwrap();
    let empty = WORD.replace("rhs: [Lex(\"word\", Numeral)]", "rhs: []");
    match bind(&empty, &lexicon, NO_FRAMES) {
        Err(LoadError::Invalid(message)) => assert!(message.contains("empty right-hand side")),
        other => panic!("{other:?}"),
    }
    let grammar = bind(WORD, &lexicon, NO_FRAMES).unwrap();
    let parse = grammar.parse(&lexicon, "");
    assert_eq!(parse.derivation_count(), 0);
    assert_eq!(parse.forest.metrics().completed_nodes, 0);
    for text in ["", " ", "One one hundred.", "one,  one"] {
        assert!(
            lexicon
                .analyze(text)
                .matches
                .iter()
                .all(|m| m.end > m.start),
            "{text}"
        );
    }
}

/// A list of words: `Seq -> Item | Seq Item`, where `Item` carries Number
/// and `Seq` carries nothing.
const LIST: &str = r#"(
    root: "Sentence",
    features: { "Number": ["Singular", "Plural"] },
    categories: { "Sentence": (), "Seq": (), "Item": (features: ["Number"]) },
    productions: [
        (name: "Sentence", lhs: "Sentence", rhs: [Cat("list", "Seq"), Punct('.')]),
        (name: "One", lhs: "Seq", rhs: [Cat("item", "Item")]),
        (name: "More", lhs: "Seq", rhs: [Cat("list", "Seq"), Cat("item", "Item")]),
        (name: "Noun", lhs: "Item", rhs: [Lex("head", Noun)]),
        (name: "Adjective", lhs: "Item", rhs: [Lex("head", Adjective)]),
    ],
)"#;

fn repeated(word: &str, n: usize) -> String {
    let mut text = vec![word; n].join(" ");
    text.push('.');
    text
}

/// Obligations 5 and 6: Derivations with equal summaries pack, so the chart
/// does not grow with the number of homographs, and the chart grows
/// linearly with the input.
#[test]
fn homograph_choices_do_not_multiply_the_chart() {
    let homographs = |k: usize| {
        Lexicon::new((0..k).map(|i| word(&format!("a:x{i}"), "x", Category::Adjective))).unwrap()
    };
    let lexicons = [homographs(1), homographs(3)];
    let mut items = Vec::new();
    for n in [4, 8, 16] {
        let text = repeated("x", n);
        let [one, three] = [0, 1].map(|i| {
            let grammar = bind(LIST, &lexicons[i], NO_FRAMES).unwrap();
            let parse = grammar.parse(&lexicons[i], &text);
            (parse.forest.metrics(), parse.derivation_count())
        });
        assert_eq!(one.1, 1);
        assert_eq!(three.1, 3u128.pow(u32::try_from(n).unwrap()));
        assert_eq!(three.0.leaves, 3 * one.0.leaves);
        assert_eq!(three.0.chart_items, one.0.chart_items, "n = {n}");
        assert_eq!(three.0.completed_nodes, one.0.completed_nodes, "n = {n}");
        items.push(one.0.chart_items);
    }
    assert_eq!(items[2] - items[1], 2 * (items[1] - items[0]), "{items:?}");
    let grammar = bind(LIST, &lexicons[1], NO_FRAMES).unwrap();
    assert_eq!(readings(&grammar, &lexicons[1], &repeated("x", 2)).len(), 9);
}

/// Obligations 6 and 10: Constituents with distinct summaries stay distinct,
/// a lexical choice the parent does not inspect still yields distinct
/// Readings, and the incomplete prefixes over those Constituents are shared.
/// (`sheep` is singular or plural; `goat` only singular. Countability is a
/// property of a v4 lexeme, not of a lexical alternative, so count versus
/// mass use is not a choice the chart sees.)
#[test]
fn distinct_summaries_stay_distinct_under_shared_prefixes() {
    let lexicon = Lexicon::new([zero_plural("sheep"), noun("goat")]).unwrap();
    let grammar = bind(LIST, &lexicon, NO_FRAMES).unwrap();
    let numbers: BTreeSet<Vec<String>> = readings(&grammar, &lexicon, "Sheep.")
        .iter()
        .map(|reading| {
            let seq = reading.node().unwrap().children[0].node().unwrap();
            let item = seq.children[0].node().unwrap();
            item.features.values().map(ToString::to_string).collect()
        })
        .collect();
    assert_eq!(
        numbers,
        BTreeSet::from([vec!["Plural".to_owned()], vec!["Singular".to_owned()]])
    );
    for n in [4, 8, 16] {
        let sheep = grammar.parse(&lexicon, &repeated("sheep", n));
        let goats = grammar.parse(&lexicon, &repeated("goat", n));
        assert_eq!(
            sheep.derivation_count(),
            2u128.pow(u32::try_from(n).unwrap())
        );
        assert_eq!(goats.derivation_count(), 1);
        let (s, g) = (sheep.forest.metrics(), goats.forest.metrics());
        // One more `Item` per word, and one more completed `Item` item per
        // word; every `Seq` prefix is shared by both.
        assert_eq!(s.completed_nodes, g.completed_nodes + n, "n = {n}");
        assert_eq!(s.chart_items, g.chart_items + n, "n = {n}");
    }
}

/// Obligation 7: agreement is checked on each child's joint alternative,
/// never on the union of its alternatives, by the chart and by Admission.
#[test]
fn agreement_keeps_joint_alternatives_correlated() {
    let lexicon = Lexicon::new([
        Lexeme::verb("v:see", "see", core()),
        pronoun("pr:it", "it", &[(Number::Singular, Person::Third)]),
        pronoun("pr:they", "they", &[(Number::Plural, Person::Third)]),
        // Singular first or plural third: never singular third.
        pronoun(
            "pr:x",
            "x",
            &[
                (Number::Singular, Person::First),
                (Number::Plural, Person::Third),
            ],
        ),
    ])
    .unwrap();
    let grammar = bind(
        r#"(
            root: "Sentence",
            features: {
                "Number": ["Singular", "Plural"],
                "Person": ["First", "Second", "Third"],
                "VerbForm": ["Plain", "Finite", "GerundParticiple", "PastParticiple"],
            },
            categories: {
                "Sentence": (),
                "Subject": (features: ["Number", "Person"]),
                "VerbPhrase": (features: ["Number", "Person", "VerbForm"]),
            },
            productions: [
                (name: "Clause", lhs: "Sentence", rhs: [Cat("subject", "Subject"), Cat("head", "VerbPhrase"), Punct('.')],
                    equations: [
                        Agree("subject.Number", "head.Number"),
                        Agree("subject.Person", "head.Person"),
                        Require("head.VerbForm", "Finite"),
                    ]),
                (name: "Pronoun", lhs: "Subject", rhs: [Lex("head", Pronoun)]),
                (name: "Verb", lhs: "VerbPhrase", rhs: [Lex("head", Verb)]),
            ],
        )"#,
        &lexicon,
        NO_FRAMES,
    )
    .unwrap();
    let agreement = |text: &str| -> BTreeSet<(String, String)> {
        readings(&grammar, &lexicon, text)
            .iter()
            .map(|reading| {
                let subject = reading.node().unwrap().children[0].node().unwrap();
                (
                    subject.features["Number"].to_string(),
                    subject.features["Person"].to_string(),
                )
            })
            .collect()
    };
    let pair = |n: &str, p: &str| (n.to_owned(), p.to_owned());
    assert_eq!(
        agreement("It sees."),
        BTreeSet::from([pair("Singular", "Third")])
    );
    assert_eq!(
        agreement("They see."),
        BTreeSet::from([pair("Plural", "Third")])
    );
    // `see` is singular (first, second) or plural (any person), so the union
    // of its alternatives holds both Singular and Third; no one alternative
    // does.
    assert_eq!(agreement("It see."), BTreeSet::new());
    assert_eq!(
        agreement("X see."),
        BTreeSet::from([pair("Singular", "First"), pair("Plural", "Third")])
    );
    // Neither of `x`'s alternatives is singular third.
    assert_eq!(agreement("X sees."), BTreeSet::new());

    // Admission: build `X sees.` with a crossed bundle for `x`, and `It see.`
    // pairing two licensed leaves that disagree.
    let built = |subject: FeatureBundle, pronoun: &str, verb: FeatureBundle| {
        let feature = |b: &FeatureBundle| {
            vec![
                (
                    "Number",
                    if b.number == Some(Number::Singular) { "Singular" } else { "Plural" },
                ),
                (
                    "Person",
                    if b.person == Some(Person::First) { "First" } else { "Third" },
                ),
            ]
        };
        let mut verb_features = feature(&verb);
        verb_features.push(("VerbForm", "Finite"));
        node(
            "Clause",
            "Sentence",
            &[],
            vec![
                node(
                    "Pronoun",
                    "Subject",
                    &feature(&subject),
                    vec![leaf(
                        pronoun,
                        WordForm::Invariant,
                        subject,
                        SurfaceCase::Initial,
                    )],
                ),
                node(
                    "Verb",
                    "VerbPhrase",
                    &verb_features,
                    vec![leaf(
                        "v:see",
                        WordForm::Present,
                        verb,
                        SurfaceCase::Declared,
                    )],
                ),
            ],
        )
    };
    let bundle = |n: Number, p: Person| FeatureBundle {
        number: Some(n),
        person: Some(p),
        ..FeatureBundle::default()
    };
    let finite = |n: Number, p: Person| FeatureBundle {
        finiteness: Some(deckmaste_lexical::Finiteness::Finite),
        tense: Some(deckmaste_lexical::Tense::Present),
        ..bundle(n, p)
    };
    let sg3 = (Number::Singular, Person::Third);
    let pl3 = (Number::Plural, Person::Third);
    let it_sees = built(bundle(sg3.0, sg3.1), "pr:it", finite(sg3.0, sg3.1));
    assert_eq!(grammar.admit(&it_sees, &lexicon), Ok(()));
    let crossed = built(bundle(sg3.0, sg3.1), "pr:x", finite(sg3.0, sg3.1));
    assert!(grammar.admit(&crossed, &lexicon).is_err());
    let disagreeing = built(bundle(sg3.0, sg3.1), "pr:it", finite(pl3.0, pl3.1));
    assert!(grammar.admit(&disagreeing, &lexicon).is_err());
}

/// Imperatives whose objects take gapped relative clauses.
const RELATIVE: &str = r#"(
    root: "Sentence",
    features: {
        "Number": ["Singular", "Plural"],
        "Person": ["First", "Second", "Third"],
        "VerbForm": ["Plain", "Finite", "GerundParticiple", "PastParticiple"],
    },
    categories: {
        "Sentence": (),
        "Clause": (gap: true),
        "VerbPhrase": (features: ["Number", "Person", "VerbForm"], gap: true),
        "NounPhrase": (),
        "Nominal": (),
    },
    closed_words: { "The": "d:the", "You": "pr:you" },
    slot_categories: { NP: "NounPhrase" },
    productions: [
        (name: "Imperative", lhs: "Sentence", rhs: [Cat("clause", "VerbPhrase"), Punct('.')],
            equations: [Require("clause.VerbForm", "Plain")]),
        (name: "Declarative", lhs: "Clause", rhs: [Word("subject", "You"), Cat("head", "VerbPhrase")],
            equations: [
                Require("head.VerbForm", "Finite"),
                Agree("subject.Number", "head.Number"),
                Agree("subject.Person", "head.Person"),
            ], transparent: ["head"]),
        (name: "Determined", lhs: "NounPhrase", rhs: [Word("det", "The"), Cat("head", "Nominal")]),
        (name: "Noun", lhs: "Nominal", rhs: [Lex("head", Noun)]),
        (name: "Relative", lhs: "Nominal", rhs: [Cat("head", "Nominal"), Cat("modifier", "Clause")],
            equations: [Require("modifier.Gap", "NP")]),
    ],
    schemas: [Valence(head: Verb, phrase: "VerbPhrase")],
)"#;

const SEE_BOTH: &str = r#"(frames: { "v:see": [
    (family: Monotransitive, slots: [(relation: Object, fill: NP)]),
    (family: Intransitive, slots: []),
] })"#;

const SEE_TRANSITIVE: &str =
    r#"(frames: { "v:see": [(family: Monotransitive, slots: [(relation: Object, fill: NP)])] })"#;

/// Obligation 8: a Gap opened by a descendant passes up through transparent
/// children, survives packing beside the gapless Constituent of the same
/// span, and is discharged only at the governing parent; a Constituent with
/// an open Gap is never a root.
#[test]
fn gap_survives_packing_until_its_governor() {
    let lexicon = Lexicon::new([
        Lexeme::verb("v:see", "see", core()),
        noun("card"),
        word("d:the", "the", Category::Determinative),
        pronoun("pr:you", "you", &[(Number::Plural, Person::Second)]),
    ])
    .unwrap();
    let gapped = |frames: &str| bind(RELATIVE, &lexicon, frames).unwrap();
    let grammar = gapped(SEE_BOTH);
    let text = "See the card you see.";
    let found = readings(&grammar, &lexicon, text);
    assert_eq!(found.len(), 1, "{found:#?}");
    let (realized, constituents) = grammar
        .constituents(found.first().unwrap(), &lexicon)
        .unwrap();
    let gap = |category: &str, span: &str| {
        constituents
            .iter()
            .find(|c| c.category == category && &realized[c.span.clone()] == span)
            .map(|c| c.gap)
    };
    assert_eq!(gap("VerbPhrase", "see"), Some(Gap::NP));
    assert_eq!(gap("Clause", "you see"), Some(Gap::NP));
    assert_eq!(gap("Nominal", "card you see"), Some(Gap::None));
    assert_eq!(gap("VerbPhrase", "See the card you see"), Some(Gap::None));

    // `you see` is a Clause with a Gap and one without; only the gapless one
    // is a root.
    let clause = |text: &str| {
        let analyzed = lexicon.analyze(text);
        let parse = grammar.parse_category(&analyzed, "Clause").unwrap();
        let gaps: Vec<Gap> = parse
            .readings()
            .iter()
            .map(|r| r.node().unwrap().gap)
            .collect();
        gaps
    };
    assert_eq!(clause("you see"), [Gap::None]);
    assert_eq!(clause("you see the card"), [Gap::None]);
    // Without the intransitive frame, `you see` and `See` have only a gapped
    // analysis, which is built (it is the relative clause above) but is
    // never a root and never fills a gapless slot.
    let grammar = gapped(SEE_TRANSITIVE);
    assert_eq!(readings(&grammar, &lexicon, text).len(), 1);
    let analyzed = lexicon.analyze("you see");
    let parse = grammar.parse_category(&analyzed, "Clause").unwrap();
    assert!(parse.forest.metrics().completed_nodes > 0);
    assert_eq!(parse.derivation_count(), 0);
    assert_eq!(readings(&grammar, &lexicon, "See."), BTreeSet::new());
}

/// Obligation 9: a unary cycle hides no finite sibling Derivation. `A` and
/// `B` each derive the word and each other; every tree in which no
/// Constituent contains itself is a Reading, whichever Constituent the
/// forest reaches first.
/// A Production introduces a Gap with `Require("$.Gap", ..)` (a subject
/// Gap), and a transparent child with a required Gap raises exactly that Gap
/// to its mother.
#[test]
fn authored_productions_introduce_and_raise_gaps() {
    let lexicon = Lexicon::new([
        Lexeme::verb("v:see", "see", core()),
        noun("card"),
        word("d:the", "the", Category::Determinative),
        pronoun("pr:you", "you", &[(Number::Plural, Person::Second)]),
    ])
    .unwrap();
    let grammar = RELATIVE.replace(
        r#""Nominal": (),"#,
        r#""Nominal": (), "Raised": (gap: true),"#,
    )
    .replace(
        "    ],\n    schemas:",
        r#"        (name: "SubjectGap", lhs: "Clause", rhs: [Cat("head", "VerbPhrase")],
            equations: [Require("head.VerbForm", "Finite"), Require("$.Gap", "NP")]),
        (name: "Raise", lhs: "Raised", rhs: [Cat("inner", "Clause")],
            equations: [Require("inner.Gap", "NP")], transparent: ["inner"]),
        (name: "RaisedRelative", lhs: "Nominal", rhs: [Cat("head", "Nominal"), Cat("modifier", "Raised")],
            equations: [Require("modifier.Gap", "NP")]),
    ],
    schemas:"#,
    );
    let grammar = bind(&grammar, &lexicon, SEE_BOTH).unwrap();
    let gaps = |text: &str, category: &str| -> Vec<Gap> {
        let mut found = Vec::new();
        for reading in readings(&grammar, &lexicon, text) {
            let (realized, constituents) = grammar.constituents(&reading, &lexicon).unwrap();
            found.extend(
                constituents
                    .iter()
                    .filter(|c| c.category == category && &realized[c.span.clone()] == "you see")
                    .map(|c| c.gap),
            );
        }
        found
    };
    // The subject Gap: *the card [_ sees]*, bound directly or after `Raised`
    // raises it.
    let subject = readings(&grammar, &lexicon, "See the card sees.");
    assert_eq!(subject.len(), 2, "{subject:#?}");
    // The object Gap of *you see* is raised through `Raised` to the relative.
    assert!(gaps("See the card you see.", "Raised").contains(&Gap::NP));
    // A gapless Clause has nothing to raise.
    let analyzed = lexicon.analyze("you see the card");
    let raised = grammar.parse_category(&analyzed, "Raised").unwrap();
    assert_eq!(raised.derivation_count(), 0);
}

#[test]
fn unary_cycle_hides_no_finite_sibling() {
    let lexicon = Lexicon::new([noun("card")]).unwrap();
    let grammar = bind(
        r#"(
            root: "Sentence",
            categories: { "Sentence": (), "A": (), "B": () },
            productions: [
                (name: "SA", lhs: "Sentence", rhs: [Cat("head", "A"), Punct('.')]),
                (name: "SB", lhs: "Sentence", rhs: [Cat("head", "B"), Punct('.')]),
                (name: "AB", lhs: "A", rhs: [Cat("head", "B")]),
                (name: "BA", lhs: "B", rhs: [Cat("head", "A")]),
                (name: "AWord", lhs: "A", rhs: [Lex("head", Noun)]),
                (name: "BWord", lhs: "B", rhs: [Lex("head", Noun)]),
            ],
        )"#,
        &lexicon,
        NO_FRAMES,
    )
    .unwrap();
    let card = || {
        leaf(
            "n:card",
            WordForm::Singular,
            number(Number::Singular),
            SurfaceCase::Initial,
        )
    };
    let a_word = || node("AWord", "A", &[], vec![card()]);
    let b_word = || node("BWord", "B", &[], vec![card()]);
    let expected = BTreeSet::from([
        node("SA", "Sentence", &[], vec![a_word()]),
        node(
            "SA",
            "Sentence",
            &[],
            vec![node("AB", "A", &[], vec![b_word()])],
        ),
        node("SB", "Sentence", &[], vec![b_word()]),
        node(
            "SB",
            "Sentence",
            &[],
            vec![node("BA", "B", &[], vec![a_word()])],
        ),
    ]);
    let parse = grammar.parse(&lexicon, "Card.");
    assert_eq!(parse.derivation_count(), 4);
    assert_eq!(readings(&grammar, &lexicon, "Card."), expected);
    assert_eq!(parse.best_readings(4).len(), 4);
}

/// Whether a Reading has `value` as a leaf.
fn uses(reading: &Reading, value: &LexicalReading) -> bool {
    match reading {
        Reading::Leaf(leaf) => leaf == value,
        Reading::Node(node) => node.children.iter().any(|c| uses(c, value)),
    }
}

/// Obligation 12: recognition consumes exactly the supplied lexical
/// analysis. Removing one alternative removes exactly the Readings that use
/// it.
#[test]
fn removing_an_alternative_removes_exactly_its_readings() {
    let lexicon = Lexicon::new([
        Lexeme::verb("v:see", "see", core()),
        zero_plural("sheep"),
        noun("one"),
        word("d:one", "one", Category::Determinative),
    ])
    .unwrap();
    let grammar = bind(
        r#"(
            root: "Sentence",
            features: { "Number": ["Singular", "Plural"], "VerbForm": ["Plain", "Finite", "GerundParticiple", "PastParticiple"] },
            categories: { "Sentence": (), "VerbPhrase": (features: ["VerbForm"]), "NounPhrase": (features: ["Number"]) },
            productions: [
                (name: "Imperative", lhs: "Sentence", rhs: [Cat("clause", "VerbPhrase"), Punct('.')]),
                (name: "Transitive", lhs: "VerbPhrase", rhs: [Lex("head", Verb), Cat("object", "NounPhrase")]),
                (name: "Bare", lhs: "NounPhrase", rhs: [Lex("head", Noun)]),
                (name: "Determined", lhs: "NounPhrase", rhs: [Lex("det", Determinative), Lex("head", Noun)]),
                (name: "Counted", lhs: "NounPhrase", rhs: [Lex("det", Numeral), Lex("head", Noun)]),
            ],
        )"#,
        &lexicon,
        NO_FRAMES,
    )
    .unwrap();
    let analyzed = lexicon.analyze("See one sheep.");
    let all = checked(&grammar, &lexicon, &analyzed);
    assert!(all.len() > 4, "{all:#?}");
    for removed in 0..analyzed.matches.len() {
        let mut fewer = analyzed.clone();
        let gone = fewer.matches.remove(removed);
        // Leaves carry no position, and no value occurs twice in this text.
        assert_eq!(
            analyzed
                .matches
                .iter()
                .filter(|m| m.reading == gone.reading)
                .count(),
            1
        );
        let expected: BTreeSet<Reading> = all
            .iter()
            .filter(|r| !uses(r, &gone.reading))
            .cloned()
            .collect();
        assert_eq!(checked(&grammar, &lexicon, &fewer), expected, "{gone:?}");
    }
}

/// Obligation 13: both round-trip laws hold for every declared lexical
/// value, spelling variants, capitalization and non-ASCII surfaces
/// included. Each value is built as a Reading without parsing; the parse
/// of its realization has exactly that Reading.
#[test]
fn every_declared_lexical_value_round_trips() {
    let mut burn = Lexeme::verb("v:burn", "burn", core());
    for form in &mut burn.forms {
        if matches!(form.form, WordForm::Preterite | WordForm::PastParticiple) {
            form.surfaces = Some(vec!["burned".to_owned(), "burnt".to_owned()]);
        }
    }
    let mut cafe = noun("café");
    for form in &mut cafe.forms {
        if form.form == WordForm::Plural {
            form.surfaces = Some(vec!["cafés".to_owned(), "caféses".to_owned()]);
        }
    }
    let mut aether = word("n:Æther", "Æther", Category::Noun);
    aether.forms[0].form = WordForm::Singular;
    aether.forms[0].features = number(Number::Singular);
    let lexicon = Lexicon::new([
        burn,
        cafe,
        aether,
        noun("élan"),
        word("a:naïve", "naïve", Category::Adjective),
        word("d:each", "each", Category::Determinative),
    ])
    .unwrap();
    let grammar = bind(WORD, &lexicon, NO_FRAMES).unwrap();
    let values: Vec<LexicalValue> = lexicon.values().cloned().collect();
    assert!(values.iter().any(|v| v.variant == 1));
    assert!(
        values
            .iter()
            .any(|v| v.capitalization == SurfaceCase::Initial)
    );
    let family = values
        .into_iter()
        .map(|value| {
            let category = lexicon.lexemes()[&value.lexeme].category;
            word_reading(production_for(category), LexicalReading::Word(value))
        })
        .collect();
    // Each spelling in both capitalizations: `burn` 22 (plain, gerund, six
    // present, past participle and six preterite in two variants each),
    // `café` 3, `élan` 2, `naïve` 1, `each` 1; `Æther` has only its
    // declared capitalization.
    assert_eq!(assert_family(&grammar, &lexicon, "Sentence", family), 59);
}

/// Imperatives with nominal and predicate attachment of Preposition Phrases.
const PHRASES: &str = r#"(
    root: "Sentence",
    features: { "Number": ["Singular", "Plural"], "VerbForm": ["Plain", "Finite", "GerundParticiple", "PastParticiple"] },
    categories: {
        "Sentence": (),
        "VerbPhrase": (features: ["VerbForm"]),
        "NounPhrase": (features: ["Number"]),
        "Nominal": (features: ["Number"]),
        "PrepositionPhrase": (),
    },
    closed_words: { "The": "d:the", "This": "d:this", "These": "d:these" },
    productions: [
        (name: "Imperative", lhs: "Sentence", rhs: [Cat("clause", "VerbPhrase"), Punct('.')],
            equations: [Require("clause.VerbForm", "Plain")]),
        (name: "Transitive", lhs: "VerbPhrase", rhs: [Lex("head", Verb), Cat("object", "NounPhrase")]),
        (name: "PredicateAdjunct", lhs: "VerbPhrase", rhs: [Cat("head", "VerbPhrase"), Cat("adjunct", "PrepositionPhrase")]),
        (name: "Determined", lhs: "NounPhrase", rhs: [Word("det", "The"), Cat("head", "Nominal")]),
        (name: "DeterminedSingular", lhs: "NounPhrase", rhs: [Word("det", "This"), Cat("head", "Nominal")],
            equations: [Require("head.Number", "Singular")]),
        (name: "DeterminedPlural", lhs: "NounPhrase", rhs: [Word("det", "These"), Cat("head", "Nominal")],
            equations: [Require("head.Number", "Plural")]),
        (name: "Noun", lhs: "Nominal", rhs: [Lex("head", Noun)]),
        (name: "Premodified", lhs: "Nominal", rhs: [Lex("modifier", Adjective), Cat("head", "Nominal")]),
        (name: "Postmodifier", lhs: "Nominal", rhs: [Cat("head", "Nominal"), Cat("modifier", "PrepositionPhrase")]),
        (name: "PrepositionPhrase", lhs: "PrepositionPhrase", rhs: [Lex("head", Preposition), Cat("object", "NounPhrase")]),
    ],
)"#;

/// Groups independently built Readings by their realization and checks
/// that the parse of each realization has exactly that group as Readings.
fn assert_family(
    grammar: &CompiledGrammar,
    lexicon: &Lexicon,
    category: &str,
    family: Vec<Reading>,
) -> usize {
    let size = family.len();
    let mut groups: BTreeMap<String, BTreeSet<Reading>> = BTreeMap::new();
    for built in family {
        assert_eq!(
            grammar.admit_category(&built, lexicon, category),
            Ok(()),
            "{built:#?}"
        );
        let text = grammar.realize(&built, lexicon).unwrap();
        assert!(groups.entry(text).or_default().insert(built), "built twice");
    }
    for (text, group) in groups {
        let analyzed = lexicon.analyze(&text);
        let parse = grammar.parse_category(&analyzed, category).unwrap();
        assert_eq!(parse.verify_realization(lexicon), Ok(()), "{text}");
        let parsed: BTreeSet<Reading> = parse.readings().into_iter().collect();
        assert_eq!(parsed, group, "{text}");
    }
    size
}

/// Obligation 13: both round-trip laws hold for every member of small
/// complete phrase families: every noun phrase of the toy lexicon, every
/// Preposition Phrase over every preposition, and every imperative with a
/// Preposition Phrase at either of its two attachment sites.
#[test]
fn complete_phrase_families_round_trip() {
    let lexicon = Lexicon::new([
        Lexeme::verb("v:see", "see", core()),
        noun("card"),
        zero_plural("sheep"),
        word("a:red", "red", Category::Adjective),
        word("d:the", "the", Category::Determinative),
        word("d:this", "this", Category::Determinative),
        word("d:these", "these", Category::Determinative),
        word("p:on", "on", Category::Preposition),
        word("p:with", "with", Category::Preposition),
    ])
    .unwrap();
    let grammar = bind(PHRASES, &lexicon, NO_FRAMES).unwrap();
    let invariant =
        |id: &str, case: SurfaceCase| leaf(id, WordForm::Invariant, FeatureBundle::default(), case);
    let declared = SurfaceCase::Declared;

    let mut nominals = Vec::new();
    for noun in ["n:card", "n:sheep"] {
        for n in [Number::Singular, Number::Plural] {
            let form = if n == Number::Singular { WordForm::Singular } else { WordForm::Plural };
            let features = [("Number", number_name(n))];
            let bare = node(
                "Noun",
                "Nominal",
                &features,
                vec![leaf(noun, form, number(n), declared)],
            );
            nominals.push((n, bare.clone()));
            nominals.push((
                n,
                node(
                    "Premodified",
                    "Nominal",
                    &features,
                    vec![invariant("a:red", declared), bare],
                ),
            ));
        }
    }
    let mut noun_phrases = Vec::new();
    for (n, nominal) in &nominals {
        let features = [("Number", number_name(*n))];
        let (production, det) = match n {
            Number::Singular => ("DeterminedSingular", "d:this"),
            Number::Plural => ("DeterminedPlural", "d:these"),
        };
        for (production, det) in [("Determined", "d:the"), (production, det)] {
            noun_phrases.push((
                *n,
                node(
                    production,
                    "NounPhrase",
                    &features,
                    vec![invariant(det, declared), nominal.clone()],
                ),
            ));
        }
    }
    assert_eq!(
        assert_family(
            &grammar,
            &lexicon,
            "NounPhrase",
            noun_phrases.iter().map(|(_, r)| r.clone()).collect()
        ),
        16
    );

    let mut preposition_phrases = Vec::new();
    for preposition in ["p:on", "p:with"] {
        for (_, np) in &noun_phrases {
            preposition_phrases.push(node(
                "PrepositionPhrase",
                "PrepositionPhrase",
                &[],
                vec![invariant(preposition, declared), np.clone()],
            ));
        }
    }
    assert_eq!(
        assert_family(
            &grammar,
            &lexicon,
            "PrepositionPhrase",
            preposition_phrases.clone()
        ),
        32
    );

    // Objects are determiner and noun, so each sentence has exactly two
    // attachment sites: the verb phrase and the object's nominal. (A
    // premodified object would add a third tree, the premodifier over the
    // postmodified nominal.)
    let see = leaf(
        "v:see",
        WordForm::Plain,
        FeatureBundle::default(),
        SurfaceCase::Initial,
    );
    let plain = [("VerbForm", "Plain")];
    let mut imperatives = Vec::new();
    for (n, np) in noun_phrases.iter().filter(|(_, np)| lexemes(np).len() == 2) {
        let Reading::Node(object) = np else { unreachable!() };
        for pp in &preposition_phrases {
            let transitive = |object: Reading| {
                node(
                    "Transitive",
                    "VerbPhrase",
                    &plain,
                    vec![see.clone(), object],
                )
            };
            imperatives.push(node(
                "Imperative",
                "Sentence",
                &[],
                vec![node(
                    "PredicateAdjunct",
                    "VerbPhrase",
                    &plain,
                    vec![transitive(np.clone()), pp.clone()],
                )],
            ));
            let mut postmodified = object.clone();
            let nominal = postmodified.children.pop().unwrap();
            postmodified.children.push(node(
                "Postmodifier",
                "Nominal",
                &[("Number", number_name(*n))],
                vec![nominal, pp.clone()],
            ));
            imperatives.push(node(
                "Imperative",
                "Sentence",
                &[],
                vec![transitive(Reading::Node(postmodified))],
            ));
        }
    }
    assert_eq!(
        assert_family(&grammar, &lexicon, "Sentence", imperatives),
        512
    );
}

#[test]
fn numeral_sign_survives_phrase_projection_and_controls_admission() {
    let lexicon = Lexicon::new([]).unwrap();
    let grammar = bind(
        r#"(
            root: "Root",
            features: { "Sign": ["Negative", "Zero", "Positive"] },
            categories: { "Root": (), "Amount": (features: ["Sign"]) },
            productions: [
                (name: "Nonnegative", lhs: "Root", rhs: [Cat("head", "Amount")],
                    equations: [Exclude("head.Sign", "Negative")]),
                (name: "Numeric", lhs: "Amount", rhs: [Numeral("head", Arabic(false))]),
            ],
        )"#,
        &lexicon,
        NO_FRAMES,
    )
    .unwrap();
    for (value, sign) in [(-2, "Negative"), (0, "Zero"), (3, "Positive")] {
        let reading = node(
            "Nonnegative",
            "Root",
            &[],
            vec![node(
                "Numeric",
                "Amount",
                &[("Sign", sign)],
                vec![Reading::Leaf(LexicalReading::Numeral {
                    value,
                    notation: deckmaste_lexical::Numeral::Arabic(false),
                    capitalization: SurfaceCase::Declared,
                })],
            )],
        );
        if value < 0 {
            assert!(grammar.admit(&reading, &lexicon).is_err());
            assert!(readings(&grammar, &lexicon, &value.to_string()).is_empty());
        } else {
            let surface = grammar.realize(&reading, &lexicon).unwrap();
            assert_eq!(surface, value.to_string());
            assert_eq!(
                readings(&grammar, &lexicon, &surface),
                BTreeSet::from([reading])
            );
        }
    }
}

/// Round-trip law 2 for a Category its Readings report under another
/// Category's name: the parse of the realization is made for the same
/// Category, not for the one whose name the Readings carry.
#[test]
fn reparse_keeps_a_category_reported_under_another_name() {
    let lexicon = Lexicon::new([
        word("a:first", "first", Category::Adjective),
        noun("strike"),
    ])
    .unwrap();
    let grammar = bind(
        r#"(
            root: "Nominal",
            categories: { "Nominal": (), "Premodified": (category: Some("Nominal")) },
            productions: [
                (name: "Noun", lhs: "Nominal", rhs: [Lex("head", Noun)]),
                (name: "Premodified", lhs: "Premodified", rhs: [Lex("modifier", Adjective), Cat("head", "Nominal")]),
            ],
        )"#,
        &lexicon,
        NO_FRAMES,
    )
    .unwrap();
    let analyzed = lexicon.analyze("first strike");
    let parse = grammar.parse_category(&analyzed, "Premodified").unwrap();
    let readings = parse.readings();
    assert_eq!(readings.len(), 1);
    let reading = &readings[0];
    assert_eq!(parse.category(), "Nominal");
    assert_eq!(reading.node().unwrap().category, "Nominal");
    assert!(parse.reparse(&analyzed).contains(reading));
    let named = grammar.parse_category(&analyzed, parse.category()).unwrap();
    assert!(!named.contains(reading));
}
