use std::collections::BTreeSet;
use std::convert::Infallible;

use deckmaste_english_v3::Grammar;
use deckmaste_english_v3::Leaf;
use deckmaste_english_v3::LexicalFeatures;
use deckmaste_english_v3::MaterializeError;
use deckmaste_english_v3::Materializer;
use deckmaste_english_v3::Production;
use deckmaste_english_v3::Symbol;
use deckmaste_english_v3::parse;
use deckmaste_lexical::Binding;
use deckmaste_lexical::Category;
use deckmaste_lexical::Countability;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::FormDeclaration;
use deckmaste_lexical::Frame;
use deckmaste_lexical::FrameItem;
use deckmaste_lexical::FrameSlot;
use deckmaste_lexical::Lexeme;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::Lexicon;
use deckmaste_lexical::Number;
use deckmaste_lexical::Person;
use deckmaste_lexical::Relation;
use deckmaste_lexical::Source;
use deckmaste_lexical::SourceKind;
use deckmaste_lexical::WordForm;

#[derive(Debug, Clone, Default, PartialEq, Eq, Ord, PartialOrd)]
struct Summary {
    features: FeatureBundle,
    count_use: Option<bool>,
    frame: Vec<String>,
    extraction: Option<String>,
    sharing: Option<String>,
    recoverability: Option<String>,
}

#[derive(Clone)]
enum Action {
    Erase,
    Forward,
    Agree,
    Require(Summary),
    Open(Summary),
    Local(Summary),
}

struct Fixture {
    rules: Vec<Production<&'static str>>,
    actions: Vec<Action>,
    names: Vec<&'static str>,
    split_countability: bool,
}

impl Fixture {
    fn new(
        rows: Vec<(
            &'static str,
            Vec<Symbol<&'static str>>,
            Action,
            &'static str,
        )>,
    ) -> Self {
        let mut result = Self {
            rules: Vec::new(),
            actions: Vec::new(),
            names: Vec::new(),
            split_countability: false,
        };
        for (category, symbols, action, name) in rows {
            result.rules.push(Production { category, symbols });
            result.actions.push(action);
            result.names.push(name);
        }
        result
    }
}

impl Grammar for Fixture {
    type Category = &'static str;
    type Summary = Summary;
    type State = Option<Summary>;

    fn productions(&self) -> &[Production<Self::Category>] {
        &self.rules
    }

    fn lexical(&self, features: LexicalFeatures<'_>) -> Vec<Summary> {
        match features {
            LexicalFeatures::Word {
                features,
                properties,
                ..
            } => {
                let summary = Summary {
                    features: features.clone(),
                    count_use: None,
                    frame: properties
                        .frames
                        .iter()
                        .flat_map(|frame| frame.items.iter())
                        .filter_map(|item| {
                            if let FrameItem::Argument(slot) = item {
                                Some(slot.category.clone())
                            } else {
                                None
                            }
                        })
                        .collect(),
                    extraction: properties.features.get("extraction").cloned(),
                    sharing: properties.features.get("sharing").cloned(),
                    recoverability: properties.features.get("recoverability").cloned(),
                };
                if self.split_countability && !properties.countability.is_empty() {
                    properties
                        .countability
                        .iter()
                        .map(|use_| Summary {
                            count_use: Some(*use_ == Countability::Count),
                            ..summary.clone()
                        })
                        .collect()
                } else {
                    vec![summary]
                }
            }
            LexicalFeatures::Numeral { .. } => vec![Summary::default()],
        }
    }

    fn begin(&self, production: usize) -> Vec<Self::State> {
        vec![match &self.actions[production] {
            Action::Open(summary) | Action::Local(summary) => Some(summary.clone()),
            _ => None,
        }]
    }

    fn advance(
        &self,
        production: usize,
        _: usize,
        state: &Self::State,
        child: Option<&Summary>,
    ) -> Vec<Self::State> {
        let Some(child) = child else { return vec![state.clone()] };
        match &self.actions[production] {
            Action::Erase | Action::Open(_) | Action::Local(_) => vec![state.clone()],
            Action::Forward => vec![Some(child.clone())],
            Action::Agree
                if state
                    .as_ref()
                    .is_none_or(|previous| previous.features == child.features) =>
            {
                vec![Some(child.clone())]
            }
            Action::Require(wanted) if child == wanted => vec![None],
            Action::Agree | Action::Require(_) => vec![],
        }
    }

    fn complete(&self, production: usize, state: &Self::State) -> Option<Summary> {
        Some(if matches!(self.actions[production], Action::Local(_)) {
            Summary::default()
        } else {
            state.clone().unwrap_or_default()
        })
    }

    fn root(&self, _: &Self::Category, summary: &Summary) -> bool {
        summary.frame.is_empty()
            && summary.extraction.is_none()
            && summary.sharing.is_none()
            && summary.recoverability.is_none()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
enum Reading {
    Word(LexicalReading, Summary),
    Literal(String),
    Construction(&'static str, Summary, Option<Summary>, Vec<Reading>),
}

impl Materializer<Summary, Option<Summary>> for &Fixture {
    type Reading = Reading;
    type Error = Infallible;

    fn leaf(&mut self, leaf: &Leaf, summary: Option<&Summary>) -> Result<Reading, Infallible> {
        Ok(match leaf {
            Leaf::Lexical {
                occurrence,
                provenance,
            } => {
                if let LexicalReading::Word(value) = &occurrence.reading {
                    assert_eq!(provenance.as_ref().unwrap().owner, value.lexeme);
                }
                Reading::Word(occurrence.reading.clone(), summary.unwrap().clone())
            }
            Leaf::Literal { text, start, end } => {
                assert_eq!(text.chars().count(), end - start);
                Reading::Literal(text.clone())
            }
        })
    }

    fn build(
        &mut self,
        production: usize,
        summary: &Summary,
        state: &Option<Summary>,
        children: Vec<Reading>,
    ) -> Result<Reading, Infallible> {
        Ok(Reading::Construction(
            self.names[production],
            summary.clone(),
            state.clone(),
            children,
        ))
    }
}

fn source(id: &str) -> Source {
    Source {
        kind: SourceKind::Core,
        path: "fixture".into(),
        owner: id.into(),
    }
}

fn word(id: &str, text: &str) -> Lexeme {
    Lexeme::invariant(id, text, Category::Adjective, source(id))
}

fn terminal() -> Symbol<&'static str> {
    Symbol::Lexical(Category::Adjective)
}
fn nt(category: &'static str) -> Symbol<&'static str> {
    Symbol::Nonterminal(category)
}
fn lit(text: &str) -> Symbol<&'static str> {
    Symbol::Literal(text.into())
}

fn realize(reading: &Reading, lexicon: &Lexicon) -> String {
    match reading {
        Reading::Word(value, _) => lexicon.realize(value).unwrap(),
        Reading::Literal(text) => text.clone(),
        Reading::Construction(_, _, _, children) => children
            .iter()
            .map(|child| realize(child, lexicon))
            .collect(),
    }
}

fn census(fixture: &Fixture, lexicon: &Lexicon, text: &str, label: &str) -> BTreeSet<Reading> {
    let input = lexicon.analyze(text);
    let forest = parse(fixture, lexicon, &input, &"S").unwrap();
    let mut readings = forest.readings(fixture);
    assert_eq!(readings.metrics().builds, 0);
    let mut result = BTreeSet::new();
    for reading in readings.by_ref() {
        let reading = reading.unwrap();
        assert_eq!(realize(&reading, lexicon), text);
        assert!(result.insert(reading));
    }
    println!("{label}: {:?}; {:?}", forest.metrics(), readings.metrics());
    result
}

#[test]
fn homographs_are_readings_and_duplicate_productions_are_not() {
    let lexicon = Lexicon::new([word("first", "light"), word("second", "light")]).unwrap();
    let fixture = Fixture::new(vec![
        ("S", vec![terminal()], Action::Erase, "adjective"),
        ("S", vec![terminal()], Action::Erase, "adjective"),
    ]);
    let readings = census(&fixture, &lexicon, "Light", "homographs");
    let expected = lexicon
        .analyze("Light")
        .matches
        .into_iter()
        .map(|occurrence| {
            Reading::Construction(
                "adjective",
                Summary::default(),
                None,
                vec![Reading::Word(occurrence.reading, Summary::default())],
            )
        })
        .collect();
    assert_eq!(readings, expected);
    assert_eq!(readings.len(), 2);
    let forest = parse(&fixture, &lexicon, &lexicon.analyze("Light"), &"S").unwrap();
    assert_eq!(forest.roots().len(), 1);
    let mut materialized = forest.readings(&fixture);
    assert_eq!(materialized.by_ref().count(), 2);
    assert_eq!(materialized.metrics().derivations, 4);
    assert_eq!(materialized.metrics().duplicates, 2);
}

#[test]
fn multiword_and_bound_overlaps_keep_distinct_leaf_structure() {
    let mut suffix = word("bound", "walk");
    suffix.binding = Binding::Suffix;
    let mut prefix = word("prefix", "island");
    prefix.binding = Binding::Prefix;
    let lexicon = Lexicon::new([
        word("whole", "first strike"),
        word("first", "first"),
        word("strike", "strike"),
        word("compound", "islandwalk"),
        prefix,
        suffix,
    ])
    .unwrap();
    let fixture = Fixture::new(vec![
        ("S", vec![terminal()], Action::Erase, "whole"),
        (
            "S",
            vec![terminal(), lit(" "), terminal()],
            Action::Erase,
            "separate",
        ),
        ("S", vec![terminal(), terminal()], Action::Erase, "bound"),
    ]);
    assert_eq!(
        census(&fixture, &lexicon, "first strike", "multiword").len(),
        2
    );
    assert_eq!(census(&fixture, &lexicon, "islandwalk", "bound").len(), 2);
    assert!(census(&fixture, &lexicon, "first  strike", "exact separators").is_empty());
}

#[test]
fn nullable_completion_handles_both_discovery_orders_and_late_growth() {
    let lexicon = Lexicon::new([]).unwrap();
    // Direct A completes before B -> C -> D has added A's second family.
    let fixture = Fixture::new(vec![
        ("S", vec![nt("A"), nt("A")], Action::Erase, "pair"),
        ("A", vec![], Action::Erase, "empty"),
        ("A", vec![nt("B")], Action::Erase, "late"),
        ("B", vec![nt("C")], Action::Erase, "b"),
        ("C", vec![nt("D")], Action::Erase, "c"),
        ("D", vec![], Action::Erase, "d"),
    ]);
    let readings = census(&fixture, &lexicon, "", "nullable late growth");
    let empty = Reading::Construction("empty", Summary::default(), None, vec![]);
    let d = Reading::Construction("d", Summary::default(), None, vec![]);
    let c = Reading::Construction("c", Summary::default(), None, vec![d]);
    let b = Reading::Construction("b", Summary::default(), None, vec![c]);
    let late = Reading::Construction("late", Summary::default(), None, vec![b]);
    let expected = [
        vec![empty.clone(), empty.clone()],
        vec![empty.clone(), late.clone()],
        vec![late.clone(), empty],
        vec![late.clone(), late],
    ]
    .into_iter()
    .map(|children| Reading::Construction("pair", Summary::default(), None, children))
    .collect();
    assert_eq!(readings, expected);
    let forest = parse(&fixture, &lexicon, &lexicon.analyze(""), &"S").unwrap();
    assert_eq!(forest.metrics().completion_work, 5);
}

#[test]
fn ambiguous_attachment_has_catalan_many_readings_in_a_small_forest() {
    let lexicon = Lexicon::new([word("leaf", "x")]).unwrap();
    let fixture = Fixture::new(vec![
        (
            "S",
            vec![nt("S"), lit(" "), nt("S")],
            Action::Erase,
            "attach",
        ),
        ("S", vec![terminal()], Action::Erase, "leaf"),
    ]);
    for (length, expected) in [(3, 2), (5, 14), (7, 132)] {
        let input = vec!["x"; length].join(" ");
        assert_eq!(
            census(&fixture, &lexicon, &input, &format!("attachment {length}")).len(),
            expected
        );
    }
}

#[test]
fn partial_histories_do_not_enumerate_homograph_products() {
    let lexicon = Lexicon::new([word("first", "x"), word("second", "x")]).unwrap();
    for length in [4, 16, 64] {
        let mut symbols = Vec::new();
        for index in 0..length {
            if index != 0 {
                symbols.push(lit(" "));
            }
            symbols.push(nt("A"));
        }
        let fixture = Fixture::new(vec![
            ("S", symbols, Action::Erase, "sequence"),
            ("A", vec![terminal()], Action::Erase, "word"),
        ]);
        let input = lexicon.analyze(&vec!["x"; length].join(" "));
        let forest = parse(&fixture, &lexicon, &input, &"S").unwrap();
        assert_eq!(forest.metrics().items, 4 * length);
        assert_eq!(forest.metrics().completion_work, length);
        assert_eq!(forest.metrics().families, 6 * length + 1);
        let mut readings = forest.readings(&fixture);
        assert_eq!(readings.metrics().builds, 0);
        let first = readings.next().unwrap().unwrap();
        let second = readings.next().unwrap().unwrap();
        assert_ne!(first, second);
        assert_eq!(realize(&first, &lexicon), input.text());
        assert_eq!(readings.metrics().derivations, 2);
        assert_eq!(readings.metrics().builds, length + 3);
        println!(
            "homograph product {length}: {:?}; {:?}",
            forest.metrics(),
            readings.metrics()
        );
    }
}

#[test]
fn incomplete_prefixes_pack_products_of_distinct_completed_summaries() {
    let lexicon = Lexicon::new([pronoun(
        "ambiguous",
        "x",
        &[
            agreement(Number::Singular, Person::First),
            agreement(Number::Plural, Person::Third),
        ],
    )])
    .unwrap();
    for length in [4, 16, 64] {
        let mut symbols = Vec::new();
        for index in 0..length {
            if index != 0 {
                symbols.push(lit(" "));
            }
            symbols.push(nt("A"));
        }
        // A has two completed nodes at each position. S treats those summaries
        // alike, so its prefixes must share histories referencing different
        // child nodes, not merely benefit from packing inside a single child.
        let fixture = Fixture::new(vec![
            ("S", symbols, Action::Erase, "sequence"),
            (
                "A",
                vec![Symbol::Lexical(Category::Pronoun)],
                Action::Forward,
                "pronoun",
            ),
        ]);
        let input = lexicon.analyze(&vec!["x"; length].join(" "));
        let forest = parse(&fixture, &lexicon, &input, &"S").unwrap();
        assert_eq!(forest.metrics().items, 5 * length);
        assert_eq!(forest.metrics().completed_nodes, 2 * length + 1);
        assert_eq!(forest.metrics().completion_work, 2 * length);
        assert_eq!(forest.metrics().families, 8 * length + 1);
        let mut readings = forest.readings(&fixture);
        assert_eq!(readings.metrics().builds, 0);
        let first = readings.next().unwrap().unwrap();
        let second = readings.next().unwrap().unwrap();
        assert_ne!(first, second);
        assert_eq!(realize(&first, &lexicon), input.text());
        assert_eq!(readings.metrics().derivations, 2);
        assert!(readings.metrics().builds <= 2 * (length + 1));
        println!(
            "distinct-summary prefix {length}: {:?}; {:?}",
            forest.metrics(),
            readings.metrics()
        );
        if length == 4 {
            assert_eq!(forest.readings(&fixture).count(), 16);
        }
    }
}

fn agreement(number: Number, person: Person) -> Summary {
    Summary {
        features: FeatureBundle {
            number: Some(number),
            person: Some(person),
            ..FeatureBundle::default()
        },
        ..Summary::default()
    }
}

fn pronoun(id: &str, surface: &str, alternatives: &[Summary]) -> Lexeme {
    let mut result = Lexeme::invariant(id, surface, Category::Pronoun, source(id));
    result.forms = alternatives
        .iter()
        .map(|summary| FormDeclaration {
            form: WordForm::Invariant,
            features: summary.features.clone(),
            surfaces: None,
        })
        .collect();
    result
}

#[test]
fn agreement_keeps_correlated_alternatives_and_rejects_crossed_features() {
    let first_singular = agreement(Number::Singular, Person::First);
    let third_plural = agreement(Number::Plural, Person::Third);
    let crossed = agreement(Number::Singular, Person::Third);
    let lexicon = Lexicon::new([
        pronoun("left", "a", &[first_singular.clone(), third_plural.clone()]),
        pronoun(
            "valid",
            "b",
            &[first_singular.clone(), third_plural.clone()],
        ),
        pronoun("crossed", "c", std::slice::from_ref(&crossed)),
    ])
    .unwrap();
    let fixture = Fixture::new(vec![
        (
            "S",
            vec![nt("A"), lit(" "), nt("A")],
            Action::Agree,
            "agree",
        ),
        (
            "A",
            vec![Symbol::Lexical(Category::Pronoun)],
            Action::Forward,
            "pronoun",
        ),
    ]);
    let valid = census(&fixture, &lexicon, "a b", "agreement correlated");
    assert_eq!(valid.len(), 2);
    let root_features: BTreeSet<_> = valid
        .iter()
        .map(|reading| {
            let Reading::Construction(_, summary, _, children) = reading else {
                panic!("root")
            };
            for child in children {
                if let Reading::Construction(_, child_summary, _, _) = child {
                    assert_eq!(child_summary, summary);
                }
            }
            summary.clone()
        })
        .collect();
    assert_eq!(
        root_features,
        BTreeSet::from([first_singular, third_plural])
    );
    assert!(census(&fixture, &lexicon, "a c", "crossed agreement exclusion").is_empty());
    let parent = Fixture::new(vec![
        ("S", vec![nt("A")], Action::Require(crossed), "require"),
        (
            "A",
            vec![Symbol::Lexical(Category::Pronoun)],
            Action::Forward,
            "pronoun",
        ),
    ]);
    assert!(census(&parent, &lexicon, "a", "correlated parent exclusion").is_empty());
}

#[test]
fn open_dependencies_survive_packing_until_the_governing_parent() {
    let requirements = [
        agreement(Number::Singular, Person::Third),
        Summary {
            frame: vec!["NP".into()],
            ..Summary::default()
        },
        Summary {
            extraction: Some("NP".into()),
            ..Summary::default()
        },
        Summary {
            sharing: Some("NP".into()),
            ..Summary::default()
        },
        Summary {
            recoverability: Some("antecedent".into()),
            ..Summary::default()
        },
    ];
    let lexicon = Lexicon::new([]).unwrap();
    for required in requirements {
        let fixture = Fixture::new(vec![
            (
                "S",
                vec![nt("A")],
                Action::Require(required.clone()),
                "governor",
            ),
            ("A", vec![nt("B")], Action::Forward, "delay"),
            ("B", vec![], Action::Open(required.clone()), "licensed"),
            ("B", vec![], Action::Open(Summary::default()), "other"),
        ]);
        let readings = census(&fixture, &lexicon, "", "open dependency");
        let b = Reading::Construction("licensed", required.clone(), Some(required.clone()), vec![]);
        let a = Reading::Construction("delay", required.clone(), Some(required.clone()), vec![b]);
        assert_eq!(
            readings,
            BTreeSet::from([Reading::Construction(
                "governor",
                Summary::default(),
                None,
                vec![a]
            )])
        );
        let direct = parse(&fixture, &lexicon, &lexicon.analyze(""), &"B").unwrap();
        assert_eq!(
            direct.roots().len(),
            if required.features == FeatureBundle::default() { 1 } else { 2 }
        );
    }
}

#[test]
fn lexical_frame_is_carried_to_its_parent() {
    let mut framed = word("framed", "x");
    framed.properties.frames.push(Frame {
        kind: "selected".into(),
        items: vec![FrameItem::Argument(FrameSlot {
            relation: Relation::Complement,
            category: "NP".into(),
        })],
    });
    let lexicon = Lexicon::new([framed, word("unframed", "x")]).unwrap();
    let fixture = Fixture::new(vec![
        (
            "S",
            vec![nt("A")],
            Action::Require(Summary {
                frame: vec!["NP".into()],
                ..Summary::default()
            }),
            "frame",
        ),
        ("A", vec![terminal()], Action::Forward, "head"),
    ]);
    let readings = census(&fixture, &lexicon, "x", "lexical frame");
    assert_eq!(readings.len(), 1);
    let expected_word = lexicon.analyze("x").matches.into_iter().find(|found| {
        matches!(&found.reading, LexicalReading::Word(value) if value.lexeme == "framed")
    }).unwrap().reading;
    let summary = Summary {
        frame: vec!["NP".into()],
        ..Summary::default()
    };
    let head = Reading::Construction(
        "head",
        summary.clone(),
        Some(summary.clone()),
        vec![Reading::Word(expected_word, summary)],
    );
    assert_eq!(
        readings,
        BTreeSet::from([Reading::Construction(
            "frame",
            Summary::default(),
            None,
            vec![head]
        )])
    );
}

#[test]
fn cycles_are_reported_without_losing_finite_siblings() {
    let fixture = Fixture::new(vec![
        ("S", vec![nt("S")], Action::Erase, "cycle"),
        ("S", vec![], Action::Erase, "empty"),
    ]);
    let lexicon = Lexicon::new([]).unwrap();
    let forest = parse(&fixture, &lexicon, &lexicon.analyze(""), &"S").unwrap();
    let mut readings = forest.readings(&fixture);
    assert_eq!(
        readings.next(),
        Some(Ok(Reading::Construction(
            "empty",
            Summary::default(),
            None,
            vec![]
        )))
    );
    assert_eq!(readings.next(), Some(Err(MaterializeError::Cycle)));
    assert_eq!(readings.next(), None);
    assert_eq!(readings.metrics().cyclic_derivations, 1);
    let unproductive = Fixture::new(vec![("S", vec![nt("S")], Action::Erase, "cycle")]);
    assert!(
        parse(&unproductive, &lexicon, &lexicon.analyze(""), &"S")
            .unwrap()
            .roots()
            .is_empty()
    );
}

#[test]
fn independent_values_preserve_features_variants_and_unicode_surface() {
    let mut noun = Lexeme::noun("cafe", "café", vec![Countability::Count], source("cafe"));
    noun.forms[1].surfaces = Some(vec!["cafés".into(), "caféses".into()]);
    let lexicon = Lexicon::new([noun]).unwrap();
    let fixture = Fixture::new(vec![(
        "S",
        vec![Symbol::Lexical(Category::Noun), lit("!")],
        Action::Forward,
        "nominal",
    )]);
    for value in lexicon.values() {
        let summary = Summary {
            features: value.features.clone(),
            ..Summary::default()
        };
        let independent = Reading::Construction(
            "nominal",
            summary.clone(),
            Some(summary.clone()),
            vec![
                Reading::Word(LexicalReading::Word(value.clone()), summary),
                Reading::Literal("!".into()),
            ],
        );
        let surface = realize(&independent, &lexicon);
        assert_eq!(
            census(&fixture, &lexicon, &surface, "independent value"),
            BTreeSet::from([independent])
        );
    }
}

#[test]
fn local_features_survive_equal_future_admissibility_summaries() {
    let singular = agreement(Number::Singular, Person::Third);
    let plural = agreement(Number::Plural, Person::Third);
    let fixture = Fixture::new(vec![
        ("S", vec![], Action::Local(singular.clone()), "implicit"),
        ("S", vec![], Action::Local(plural.clone()), "implicit"),
    ]);
    let lexicon = Lexicon::new([]).unwrap();
    let forest = parse(&fixture, &lexicon, &lexicon.analyze(""), &"S").unwrap();
    assert_eq!(forest.roots().len(), 1);
    assert_eq!(
        census(&fixture, &lexicon, "", "local features"),
        BTreeSet::from([
            Reading::Construction("implicit", Summary::default(), Some(singular), vec![]),
            Reading::Construction("implicit", Summary::default(), Some(plural), vec![]),
        ])
    );
}

#[test]
fn lexical_use_choices_survive_when_the_parent_forgets_them() {
    let lexicon = Lexicon::new([Lexeme::noun(
        "material",
        "material",
        vec![Countability::Count, Countability::Mass],
        source("material"),
    )])
    .unwrap();
    let mut fixture = Fixture::new(vec![(
        "S",
        vec![Symbol::Lexical(Category::Noun)],
        Action::Erase,
        "nominal",
    )]);
    fixture.split_countability = true;
    let values = census(&fixture, &lexicon, "material", "lexical uses");
    assert_eq!(values.len(), 2);
    let occurrence = lexicon.analyze("material").matches.remove(0);
    let expected = [false, true]
        .into_iter()
        .map(|count_use| {
            let summary = Summary {
                features: FeatureBundle {
                    number: Some(Number::Singular),
                    ..FeatureBundle::default()
                },
                count_use: Some(count_use),
                ..Summary::default()
            };
            Reading::Construction(
                "nominal",
                Summary::default(),
                None,
                vec![Reading::Word(occurrence.reading.clone(), summary)],
            )
        })
        .collect();
    assert_eq!(values, expected);
}

#[test]
fn duplicate_empty_derivations_are_one_reading_and_empty_literals_are_leaves() {
    let fixture = Fixture::new(vec![
        ("S", vec![lit("")], Action::Erase, "empty literal"),
        ("S", vec![lit("")], Action::Erase, "empty literal"),
    ]);
    let lexicon = Lexicon::new([]).unwrap();
    assert_eq!(
        census(&fixture, &lexicon, "", "duplicate empty literal"),
        BTreeSet::from([Reading::Construction(
            "empty literal",
            Summary::default(),
            None,
            vec![Reading::Literal(String::new())]
        ),])
    );
}

#[test]
fn real_lexical_homographs_are_not_selected_by_the_chart() {
    let mut cast = Lexeme::verb("cast", "cast", source("cast"));
    for slot in &mut cast.forms {
        if matches!(slot.form, WordForm::Preterite | WordForm::PastParticiple) {
            slot.surfaces = Some(vec!["cast".into()]);
        }
    }
    let lexicon = Lexicon::new([
        cast,
        Lexeme::noun(
            "counter_noun",
            "counter",
            vec![Countability::Count],
            source("counter_noun"),
        ),
        Lexeme::verb("counter_verb", "counter", source("counter_verb")),
        Lexeme::noun(
            "one_noun",
            "one",
            vec![Countability::Count],
            source("one_noun"),
        ),
        Lexeme::invariant(
            "one_determinative",
            "one",
            Category::Determinative,
            source("one_determinative"),
        ),
    ])
    .unwrap();
    let fixture = Fixture::new(vec![
        (
            "S",
            vec![Symbol::Lexical(Category::Noun)],
            Action::Forward,
            "noun",
        ),
        (
            "S",
            vec![Symbol::Lexical(Category::Verb)],
            Action::Forward,
            "verb",
        ),
        (
            "S",
            vec![Symbol::Lexical(Category::Determinative)],
            Action::Forward,
            "determinative",
        ),
        (
            "S",
            vec![Symbol::Lexical(Category::Numeral)],
            Action::Forward,
            "numeral",
        ),
    ]);
    for (text, expected) in [("cast", 13), ("counters", 2), ("one", 3)] {
        assert_eq!(census(&fixture, &lexicon, text, text).len(), expected);
    }
}

struct Ownership<'a>(&'a Fixture);

impl Materializer<Summary, Option<Summary>> for Ownership<'_> {
    type Reading = (Reading, usize, usize);
    type Error = Infallible;

    fn leaf(
        &mut self,
        leaf: &Leaf,
        summary: Option<&Summary>,
    ) -> Result<Self::Reading, Infallible> {
        let reading = Materializer::leaf(&mut self.0, leaf, summary)?;
        let (start, end) = match leaf {
            Leaf::Lexical { occurrence, .. } => (occurrence.start, occurrence.end),
            Leaf::Literal { start, end, .. } => (*start, *end),
        };
        Ok((reading, start, end))
    }

    fn build(
        &mut self,
        production: usize,
        summary: &Summary,
        state: &Option<Summary>,
        children: Vec<Self::Reading>,
    ) -> Result<Self::Reading, Infallible> {
        for pair in children.windows(2) {
            assert_eq!(
                pair[0].2, pair[1].1,
                "every scalar belongs to exactly one adjacent leaf"
            );
        }
        let start = children.first().unwrap().1;
        let end = children.last().unwrap().2;
        let values = children
            .into_iter()
            .map(|(reading, _, _)| reading)
            .collect();
        Ok((
            Materializer::build(&mut self.0, production, summary, state, values)?,
            start,
            end,
        ))
    }
}

#[test]
fn all_attachment_readings_have_exact_leaf_ownership_and_traversal_order() {
    let fixture = Fixture::new(vec![
        (
            "S",
            vec![nt("S"), lit(" \n"), nt("S")],
            Action::Erase,
            "attach",
        ),
        ("S", vec![terminal()], Action::Erase, "word"),
    ]);
    let lexicon = Lexicon::new([word("unicode", "é")]).unwrap();
    let input = lexicon.analyze("É \né \né");
    let forest = parse(&fixture, &lexicon, &input, &"S").unwrap();
    let values: Vec<_> = forest
        .readings(Ownership(&fixture))
        .map(Result::unwrap)
        .collect();
    assert_eq!(values.len(), 2);
    for (reading, start, end) in values {
        assert_eq!((start, end), (0, input.tokens.len()));
        assert_eq!(realize(&reading, &lexicon), input.text());
    }
}

#[test]
fn malformed_occurrences_report_input_errors() {
    use deckmaste_english_v3::ParseError;
    let fixture = Fixture::new(vec![("S", vec![terminal()], Action::Erase, "word")]);
    let lexicon = Lexicon::new([word("word", "x")]).unwrap();
    let mut input = lexicon.analyze("x");
    input.matches[0].end = 2;
    assert_eq!(
        parse(&fixture, &lexicon, &input, &"S").err(),
        Some(ParseError::InvalidOccurrence(0))
    );
    input.matches[0].end = 0;
    assert_eq!(
        parse(&fixture, &lexicon, &input, &"S").err(),
        Some(ParseError::InvalidOccurrence(0))
    );
    input.matches[0].end = 1;
    let LexicalReading::Word(value) = &mut input.matches[0].reading else {
        panic!("word")
    };
    value.variant = 100;
    assert_eq!(
        parse(&fixture, &lexicon, &input, &"S").err(),
        Some(ParseError::UndeclaredOccurrence(0))
    );
    input.tokens[0] = 'y';
    assert_eq!(
        parse(&fixture, &lexicon, &input, &"S").err(),
        Some(ParseError::InvalidTokens)
    );
}

#[test]
fn materializer_errors_are_visible_as_internal_failures() {
    struct Broken;
    impl Materializer<Summary, Option<Summary>> for Broken {
        type Reading = ();
        type Error = &'static str;
        fn leaf(&mut self, _: &Leaf, _: Option<&Summary>) -> Result<(), Self::Error> {
            Ok(())
        }
        fn build(
            &mut self,
            _: usize,
            _: &Summary,
            _: &Option<Summary>,
            _: Vec<()>,
        ) -> Result<(), Self::Error> {
            Err("broken construction")
        }
    }
    let fixture = Fixture::new(vec![("S", vec![], Action::Erase, "empty")]);
    let lexicon = Lexicon::new([]).unwrap();
    let forest = parse(&fixture, &lexicon, &lexicon.analyze(""), &"S").unwrap();
    assert_eq!(
        forest.readings(Broken).next(),
        Some(Err(MaterializeError::Build("broken construction")))
    );
}
