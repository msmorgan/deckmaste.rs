use std::collections::BTreeMap;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FormDeclaration {
    pub form: crate::WordForm,
    pub features: crate::FeatureBundle,
    /// None requests the default; Some replaces it with exactly these
    /// alternatives. An unavailable slot is omitted from the lexeme's
    /// forms.
    pub surfaces: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LexicalProperties {
    pub countability: Vec<crate::Countability>,
    /// Declared distribution features retained by name for the grammar adapter.
    pub features: BTreeMap<String, String>,
    pub frames: Vec<crate::Frame>,
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
    pub id: crate::LexemeId,
    pub lemma: String,
    pub category: crate::Category,
    pub properties: LexicalProperties,
    pub forms: Vec<FormDeclaration>,
    pub source: Source,
    pub capitalization: Capitalization,
    pub binding: Binding,
}

impl Lexeme {
    #[must_use]
    pub fn invariant(
        id: impl Into<crate::LexemeId>,
        lemma: impl Into<String>,
        category: crate::Category,
        source: Source,
    ) -> Self {
        Self {
            id: id.into(),
            lemma: lemma.into(),
            category,
            properties: LexicalProperties::default(),
            forms: vec![FormDeclaration {
                form: crate::WordForm::Invariant,
                features: crate::FeatureBundle::default(),
                surfaces: None,
            }],
            source,
            capitalization: Capitalization::Initial,
            binding: Binding::Free,
        }
    }

    #[must_use]
    pub fn noun(
        id: impl Into<crate::LexemeId>,
        lemma: impl Into<String>,
        countability: Vec<crate::Countability>,
        source: Source,
    ) -> Self {
        let mut lexeme = Self::invariant(id, lemma, crate::Category::Noun, source);
        lexeme.properties.countability = countability;
        lexeme.forms = [crate::WordForm::Singular, crate::WordForm::Plural]
            .into_iter()
            .filter(|form| {
                *form != crate::WordForm::Plural
                    || lexeme
                        .properties
                        .countability
                        .contains(&crate::Countability::Count)
            })
            .map(|form| FormDeclaration {
                form,
                features: crate::FeatureBundle {
                    number: Some(if form == crate::WordForm::Singular {
                        crate::Number::Singular
                    } else {
                        crate::Number::Plural
                    }),
                    ..crate::FeatureBundle::default()
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
    pub fn verb(id: impl Into<crate::LexemeId>, lemma: impl Into<String>, source: Source) -> Self {
        let mut lexeme = Self::invariant(id, lemma, crate::Category::Verb, source);
        lexeme.forms = [
            crate::WordForm::Plain,
            crate::WordForm::GerundParticiple,
            crate::WordForm::PastParticiple,
        ]
        .into_iter()
        .map(|form| FormDeclaration {
            form,
            features: crate::FeatureBundle {
                finiteness: Some(crate::Finiteness::Nonfinite),
                ..crate::FeatureBundle::default()
            },
            surfaces: None,
        })
        .collect();
        for (form, tense) in [
            (crate::WordForm::Present, crate::Tense::Present),
            (crate::WordForm::Preterite, crate::Tense::Past),
        ] {
            for number in [crate::Number::Singular, crate::Number::Plural] {
                for person in [
                    crate::Person::First,
                    crate::Person::Second,
                    crate::Person::Third,
                ] {
                    lexeme.forms.push(FormDeclaration {
                        form,
                        features: crate::FeatureBundle {
                            number: Some(number),
                            person: Some(person),
                            tense: Some(tense),
                            finiteness: Some(crate::Finiteness::Finite),
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
