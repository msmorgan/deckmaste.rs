//! Data-only vocabulary shared by lexical analysis and English grammar.

use serde::Deserialize;
use serde::Serialize;

/// Stable identity of a declared lexeme.
pub type LexemeId = String;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Ord, PartialOrd, Serialize)]
pub enum Category {
    Noun,
    Verb,
    Adjective,
    Adverb,
    Determinative,
    Pronoun,
    Preposition,
    Coordinator,
    Subordinator,
    Numeral,
    /// An explicitly named inventory whose grammatical distribution is not yet
    /// mapped. It never implicitly acquires an ordinary part of speech.
    Catalog,
    Keyword,
    Symbol,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Ord, PartialOrd, Serialize)]
pub enum Number {
    Singular,
    Plural,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Ord, PartialOrd, Serialize)]
pub enum Person {
    First,
    Second,
    Third,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Ord, PartialOrd, Serialize)]
pub enum Tense {
    Present,
    Past,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Ord, PartialOrd, Serialize)]
pub enum Finiteness {
    Finite,
    Nonfinite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Ord, PartialOrd, Serialize)]
pub enum Case {
    Nominative,
    Accusative,
    Genitive,
}

/// One correlated lexical alternative.
///
/// A missing dimension is inapplicable, not a wildcard. Which combinations
/// are applicable is enforced by the lexical engine that owns morphology.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Ord, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FeatureBundle {
    pub number: Option<Number>,
    pub person: Option<Person>,
    pub tense: Option<Tense>,
    pub finiteness: Option<Finiteness>,
    pub case: Option<Case>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Ord, PartialOrd, Serialize)]
pub enum WordForm {
    Invariant,
    Singular,
    Plural,
    Plain,
    Present,
    Preterite,
    GerundParticiple,
    PastParticiple,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Countability {
    Count,
    Mass,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Relation {
    Subject,
    Object,
    Complement,
}

/// One grammatical position in a lexical frame.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FrameSlot {
    pub relation: Relation,
    pub category: String,
}

/// One ordered element of a lexical frame signature.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum FrameItem {
    Argument(FrameSlot),
    Marker {
        vocabulary: String,
        member: String,
    },
    Marked {
        vocabulary: String,
        member: String,
        slot: FrameSlot,
    },
    Optional(Box<FrameItem>),
    /// Legacy source data that has not yet been reconciled to a declared
    /// marker identity.
    Literal(String),
}

/// A lexical frame signature, independent of grammar productions and
/// admission.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Frame {
    pub kind: String,
    pub items: Vec<FrameItem>,
}

/// The selected capitalization of a grammatical lexical value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Ord, PartialOrd, Serialize)]
pub enum SurfaceCase {
    Declared,
    Initial,
}

/// An integer notation identity. Its parse and realization codecs belong to
/// `deckmaste_lexical`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Numeral {
    Cardinal,
    Ordinal,
    Arabic(bool),
    Roman,
}

/// A compact declared-word value carried by grammar leaves and realization.
/// Source positions and declaration provenance are intentionally absent.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Ord, PartialOrd, Serialize)]
pub struct LexicalValue {
    pub lexeme: LexemeId,
    pub form: WordForm,
    pub features: FeatureBundle,
    pub variant: usize,
    pub capitalization: SurfaceCase,
}

/// A compact lexical alternative, independent of its source occurrence.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Ord, PartialOrd, Serialize)]
pub enum LexicalReading {
    Word(LexicalValue),
    Numeral {
        value: i32,
        notation: Numeral,
        capitalization: SurfaceCase,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_retains_ordered_slots_and_licensed_categories() {
        let frame = Frame {
            kind: "Predicate".to_owned(),
            items: vec![
                FrameItem::Argument(FrameSlot {
                    relation: Relation::Object,
                    category: "NounPhrase".to_owned(),
                }),
                FrameItem::Marked {
                    vocabulary: "Preposition".to_owned(),
                    member: "To".to_owned(),
                    slot: FrameSlot {
                        relation: Relation::Complement,
                        category: "Destination".to_owned(),
                    },
                },
            ],
        };

        assert_eq!(
            frame.items,
            [
                FrameItem::Argument(FrameSlot {
                    relation: Relation::Object,
                    category: "NounPhrase".to_owned(),
                }),
                FrameItem::Marked {
                    vocabulary: "Preposition".to_owned(),
                    member: "To".to_owned(),
                    slot: FrameSlot {
                        relation: Relation::Complement,
                        category: "Destination".to_owned(),
                    },
                },
            ]
        );
    }

    #[test]
    fn lexical_values_keep_correlated_features_and_spelling_choices() {
        let value = LexicalReading::Word(LexicalValue {
            lexeme: "core-verb:Cast".to_owned(),
            form: WordForm::Preterite,
            features: FeatureBundle {
                number: Some(Number::Plural),
                person: Some(Person::Third),
                tense: Some(Tense::Past),
                finiteness: Some(Finiteness::Finite),
                case: None,
            },
            variant: 1,
            capitalization: SurfaceCase::Initial,
        });

        let LexicalReading::Word(value) = value else {
            panic!("constructed a declared-word value");
        };
        assert_eq!(value.lexeme, "core-verb:Cast");
        assert_eq!(value.features.tense, Some(Tense::Past));
        assert_eq!(value.variant, 1);
        assert_eq!(value.capitalization, SurfaceCase::Initial);
    }
}
