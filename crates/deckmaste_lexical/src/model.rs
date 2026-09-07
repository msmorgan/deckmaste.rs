use std::collections::BTreeMap;

use serde::Deserialize;
use serde::Serialize;

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

/// Each bundle is one alternative. Missing dimensions are inapplicable, not
/// wildcards.
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

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FormDeclaration {
    pub form: WordForm,
    pub features: FeatureBundle,
    /// None requests the default; Some replaces it with exactly these
    /// alternatives. An unavailable slot is omitted from the lexeme's
    /// forms.
    pub surfaces: Option<Vec<String>>,
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

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum FrameItem {
    Argument {
        relation: Relation,
        category: String,
    },
    Marker {
        vocabulary: String,
        member: String,
    },
    Optional(Box<FrameItem>),
    Sequence(Vec<FrameItem>),
    Literal(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Frame {
    pub kind: String,
    pub items: Vec<FrameItem>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LexicalProperties {
    pub countability: Vec<Countability>,
    /// Declared distribution features retained by name for the grammar adapter.
    pub features: BTreeMap<String, String>,
    pub frames: Vec<Frame>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Ord, PartialOrd, Serialize)]
pub enum SourceKind {
    Core,
    Plugin,
    Catalog,
    Keyword,
    Symbol,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Source {
    pub kind: SourceKind,
    pub path: String,
    /// Original inventory/member or declaration identity, independent of
    /// spelling.
    pub owner: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Capitalization {
    Exact,
    Initial,
}

/// Bound entries relax only the declared word edge, never whitespace matching.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Binding {
    Free,
    Prefix,
    Suffix,
    Bound,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Lexeme {
    pub id: String,
    pub lemma: String,
    pub category: Category,
    pub properties: LexicalProperties,
    pub forms: Vec<FormDeclaration>,
    pub source: Source,
    pub capitalization: Capitalization,
    pub binding: Binding,
}

impl Lexeme {
    #[must_use]
    pub fn invariant(
        id: impl Into<String>,
        lemma: impl Into<String>,
        category: Category,
        source: Source,
    ) -> Self {
        Self {
            id: id.into(),
            lemma: lemma.into(),
            category,
            properties: LexicalProperties::default(),
            forms: vec![FormDeclaration {
                form: WordForm::Invariant,
                features: FeatureBundle::default(),
                surfaces: None,
            }],
            source,
            capitalization: Capitalization::Initial,
            binding: Binding::Free,
        }
    }

    #[must_use]
    pub fn noun(
        id: impl Into<String>,
        lemma: impl Into<String>,
        countability: Vec<Countability>,
        source: Source,
    ) -> Self {
        let mut lexeme = Self::invariant(id, lemma, Category::Noun, source);
        lexeme.properties.countability = countability;
        lexeme.forms = [WordForm::Singular, WordForm::Plural]
            .into_iter()
            .filter(|form| {
                *form != WordForm::Plural
                    || lexeme
                        .properties
                        .countability
                        .contains(&Countability::Count)
            })
            .map(|form| FormDeclaration {
                form,
                features: FeatureBundle {
                    number: Some(if form == WordForm::Singular {
                        Number::Singular
                    } else {
                        Number::Plural
                    }),
                    ..FeatureBundle::default()
                },
                surfaces: None,
            })
            .collect();
        lexeme
    }

    /// Declares a regular verb's complete finite bundles and nonfinite forms.
    /// Callers replace irregular slots or remove unavailable slots before
    /// indexing.
    #[must_use]
    pub fn verb(id: impl Into<String>, lemma: impl Into<String>, source: Source) -> Self {
        let mut lexeme = Self::invariant(id, lemma, Category::Verb, source);
        lexeme.forms = [
            WordForm::Plain,
            WordForm::GerundParticiple,
            WordForm::PastParticiple,
        ]
        .into_iter()
        .map(|form| FormDeclaration {
            form,
            features: FeatureBundle {
                finiteness: Some(Finiteness::Nonfinite),
                ..FeatureBundle::default()
            },
            surfaces: None,
        })
        .collect();
        for (form, tense) in [
            (WordForm::Present, Tense::Present),
            (WordForm::Preterite, Tense::Past),
        ] {
            for number in [Number::Singular, Number::Plural] {
                for person in [Person::First, Person::Second, Person::Third] {
                    lexeme.forms.push(FormDeclaration {
                        form,
                        features: FeatureBundle {
                            number: Some(number),
                            person: Some(person),
                            tense: Some(tense),
                            finiteness: Some(Finiteness::Finite),
                            case: None,
                        },
                        surfaces: None,
                    });
                }
            }
        }
        lexeme
    }
}
