#![allow(
    dead_code,
    reason = "each integration test uses its own subset of this support module"
)]

use std::collections::BTreeSet;
use std::path::Path;
use std::sync::LazyLock;

use deckmaste_english_v3::Leaf;
use deckmaste_english_v3::Materializer;
use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Error;
use deckmaste_english_v3::grammar::Grammar;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::State;
use deckmaste_english_v3::grammar::Summary;
use deckmaste_english_v3::grammar::Value;
use deckmaste_english_v3::parse;
use deckmaste_lexical::Lexicon;

pub static LEXICON: LazyLock<Lexicon> = LazyLock::new(|| {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    Lexicon::new(
        deckmaste_lexical_source::load_workspace(&root)
            .unwrap()
            .lexemes,
    )
    .unwrap()
});

pub fn lexicon() -> &'static Lexicon {
    &LEXICON
}

pub fn readings(text: &str, category: Category) -> BTreeSet<Reading> {
    readings_with_lexicon(lexicon(), text, category)
}

pub fn readings_with_lexicon(
    lexicon: &Lexicon,
    text: &str,
    category: Category,
) -> BTreeSet<Reading> {
    let environment = deckmaste_english_v3::grammar::GrammarEnvironment::new(lexicon);
    let grammar = environment.grammar();
    let input = lexicon.analyze(text);
    let forest = parse(grammar, lexicon, &input, &category).unwrap();
    let mut values = BTreeSet::new();
    for value in grammar.readings(&forest) {
        let value = value.unwrap();
        assert_eq!(grammar.realize(&value, lexicon).unwrap(), text);
        assert!(values.insert(value), "duplicate Reading for {text:?}");
    }
    values
}

pub fn ability_readings(text: &str) -> BTreeSet<Reading> {
    readings(text, Category::Ability)
}

pub fn noun_phrase_readings(text: &str) -> BTreeSet<Reading> {
    readings(text, Category::NounPhrase)
}

/// Require one admitted Reading containing every requested Category at its
/// exact source span. Each surface must occur exactly once in the text; use
/// `assert_constituent_occurrences` to choose among repeated surfaces. Separate
/// calls can witness alternative analyses of the same text.
pub fn assert_constituents(text: &str, category: Category, expected: &[(Category, &str)]) {
    let expected: Vec<_> = expected
        .iter()
        .map(|&(category, surface)| (category, surface, None))
        .collect();
    assert_expected_constituents(text, category, &expected);
}

/// Choose each exact constituent surface's zero-based occurrence in the source.
/// Occurrences include overlapping matches and are counted at UTF-8 boundaries.
/// Every requested span must belong to the same admitted Reading.
pub fn assert_constituent_occurrences(
    text: &str,
    category: Category,
    expected: &[(Category, &str, usize)],
) {
    let expected: Vec<_> = expected
        .iter()
        .map(|&(category, surface, occurrence)| (category, surface, Some(occurrence)))
        .collect();
    assert_expected_constituents(text, category, &expected);
}

fn assert_expected_constituents(
    text: &str,
    category: Category,
    expected: &[(Category, &str, Option<usize>)],
) {
    let expected: BTreeSet<_> = expected
        .iter()
        .map(|&(category, surface, occurrence)| {
            let starts: Vec<_> = text
                .char_indices()
                .filter_map(|(start, _)| text[start..].starts_with(surface).then_some(start))
                .collect();
            assert!(
                !surface.is_empty() && !starts.is_empty(),
                "expected constituent surface {surface:?} is not a nonempty substring of {text:?}"
            );
            assert!(
                occurrence.is_some() || starts.len() == 1,
                "surface {surface:?} occurs {} times in {text:?}; choose an occurrence explicitly",
                starts.len()
            );
            let occurrence = occurrence.unwrap_or(0);
            let start = *starts.get(occurrence).unwrap_or_else(|| {
                panic!(
                    "surface {surface:?} has {} occurrences; zero-based occurrence {occurrence} is out of range",
                    starts.len()
                )
            });
            (category, start, start + surface.len())
        })
        .collect();
    let observed = constituent_positions(text, category);
    if observed.iter().any(|nodes| expected.is_subset(nodes)) {
        return;
    }
    panic!(
        "no single Reading of {text:?} at {category:?} contains all constituents {expected:?}; \
         observed constituent byte spans by Reading: {observed:?}"
    );
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct PositionedValue {
    value: Value,
    extent: Option<(usize, usize)>,
    constituents: BTreeSet<(Category, usize, usize)>,
}

struct PositionedMaterializer<'a> {
    grammar: &'a Grammar,
}

// Delegate all grammatical construction to the generated materializer. Carry
// parser leaf coordinates through each build instead of locating rendered node
// strings, which cannot distinguish identical words at different positions.
impl Materializer<Summary, State> for PositionedMaterializer<'_> {
    type Reading = PositionedValue;
    type Error = Error;

    fn leaf(&mut self, leaf: &Leaf, summary: Option<&Summary>) -> Result<PositionedValue, Error> {
        let (start, end) = match leaf {
            Leaf::Lexical { occurrence, .. } => (occurrence.start, occurrence.end),
            Leaf::Literal { start, end, .. } => (*start, *end),
        };
        Ok(PositionedValue {
            value: self.grammar.leaf(leaf, summary)?,
            extent: Some((start, end)),
            constituents: BTreeSet::new(),
        })
    }

    fn build(
        &mut self,
        production: usize,
        summary: &Summary,
        state: &State,
        children: Vec<PositionedValue>,
    ) -> Result<PositionedValue, Error> {
        let mut extent: Option<(usize, usize)> = None;
        let mut constituents = BTreeSet::new();
        let mut values = Vec::new();
        for child in children {
            if let Some((start, end)) = child.extent {
                extent = Some(extent.map_or((start, end), |(left, right)| {
                    (left.min(start), right.max(end))
                }));
            }
            constituents.extend(child.constituents);
            values.push(child.value);
        }
        let value = self.grammar.build(production, summary, state, values)?;
        if let (Value::Reading(node), Some((start, end))) = (&value, extent) {
            constituents.insert((node.category(), start, end));
        }
        Ok(PositionedValue {
            value,
            extent,
            constituents,
        })
    }
}

pub fn constituent_positions(
    text: &str,
    category: Category,
) -> Vec<BTreeSet<(Category, usize, usize)>> {
    let environment = deckmaste_english_v3::grammar::GrammarEnvironment::new(lexicon());
    let grammar = environment.grammar();
    let input = lexicon().analyze(text);
    let forest = parse(grammar, lexicon(), &input, &category).unwrap();
    // The parser's coordinates count Unicode scalars; assertion spans count
    // bytes.
    let bytes: Vec<_> = text
        .char_indices()
        .map(|(start, _)| start)
        .chain(std::iter::once(text.len()))
        .collect();
    forest
        .readings(PositionedMaterializer { grammar })
        .map(|value| {
            let value = value.unwrap();
            let Value::Reading(reading) = value.value else {
                panic!("a root must materialize a Reading");
            };
            assert_eq!(grammar.realize(&reading, lexicon()).unwrap(), text);
            value
                .constituents
                .into_iter()
                .map(|(category, start, end)| (category, bytes[start], bytes[end]))
                .collect()
        })
        .collect()
}
