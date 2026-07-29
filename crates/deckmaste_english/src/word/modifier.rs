use std::collections::HashMap;
use std::sync::OnceLock;

use super::Tense;
use super::Verb;
use super::VerbSlot;
use super::Vocab;
use super::Vocabulary;
use crate::catalog::CatalogAtom;
use crate::syntax::ComparativeWord;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum ColorWord {
    White,
    Blue,
    Black,
    Red,
    Green,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum CardOrientation {
    FaceUp,
    FaceDown,
}

impl CardOrientation {
    #[must_use]
    pub const fn spelling(self) -> &'static str {
        match self {
            Self::FaceUp => "face up",
            Self::FaceDown => "face down",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
pub enum Adjective {
    Word(Vocab),
    Color(ColorWord),
    CardOrientation(CardOrientation),
    Participle(Tense, Verb),
    Catalog(CatalogAtom),
    Ordinal(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum InitialSound {
    Consonant,
    Vowel,
}

/// The initial sound of a surface string, read off its first character.
/// Shared by every site that derives an onset from spelling rather than from
/// a lexical entry's overridden `initial_sound`.
#[must_use]
pub fn surface_initial_sound(surface: &str) -> InitialSound {
    surface
        .chars()
        .next()
        .map_or(InitialSound::Consonant, character_initial_sound)
}

pub(super) fn character_initial_sound(character: char) -> InitialSound {
    if matches!(character.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u') {
        InitialSound::Vowel
    } else {
        InitialSound::Consonant
    }
}

/// The comparison capability of an adjective, recorded as vocabulary metadata
/// rather than matched by spelling in grammar control flow. Every member takes
/// a `… than X` complement (the parser marks it comparison-pending);
/// `OrComparative` members additionally head an `N or <word>` quantity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub(crate) enum AdjectiveComparison {
    /// `less`/`fewer`/`greater`/`more` — heads `N or <word>` and `<word> than
    /// X`.
    OrComparative(ComparativeWord),
    /// `other` — heads `other than X` only, never a quantity bound.
    ThanOnly,
}

/// Which comparison class produced a comparison-pending adjective. Derived
/// from the per-`Vocab` comparison metadata, never from a spelling match.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub(crate) enum AdjectiveComparisonClass {
    OrComparative,
    ThanOnly,
}

impl AdjectiveComparison {
    pub(crate) const fn class(self) -> AdjectiveComparisonClass {
        match self {
            Self::OrComparative(_) => AdjectiveComparisonClass::OrComparative,
            Self::ThanOnly => AdjectiveComparisonClass::ThanOnly,
        }
    }
}

/// Surface → comparative-word index, derived from the per-[`Vocab`] comparison
/// metadata so the fact lives in one place. Only `OrComparative` adjectives
/// (`less`/`fewer`/`greater`/`more`) can head an `N or <word>` quantity;
/// `other` (`ThanOnly`) is deliberately absent.
fn comparison_lexicon() -> &'static HashMap<&'static str, ComparativeWord> {
    static LEXICON: OnceLock<HashMap<&'static str, ComparativeWord>> = OnceLock::new();
    LEXICON.get_or_init(|| {
        Vocab::ALL
            .iter()
            .filter_map(|&vocab| match vocab.comparison() {
                Some(AdjectiveComparison::OrComparative(word)) => Some((vocab.spelling(), word)),
                Some(AdjectiveComparison::ThanOnly) | None => None,
            })
            .collect()
    })
}

/// The comparative word a surface form names when it heads an `N or <word>`
/// quantity, or `None` if the form is not a quantity-heading comparative.
pub(crate) fn comparative_word(surface: &str) -> Option<ComparativeWord> {
    comparison_lexicon().get(surface).copied()
}

impl ColorWord {
    /// The five colors, in the fixed order rules text lists them.
    pub const ALL: [Self; 5] = [Self::White, Self::Blue, Self::Black, Self::Red, Self::Green];

    #[must_use]
    pub const fn spelling(self) -> &'static str {
        match self {
            Self::White => "white",
            Self::Blue => "blue",
            Self::Black => "black",
            Self::Red => "red",
            Self::Green => "green",
        }
    }

    /// The color a lowercase surface names, or `None`. Case-sensitive: color
    /// words are always lowercase in the value-nominal argument position.
    #[must_use]
    pub fn from_surface(surface: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|color| color.spelling() == surface)
    }
}

impl Vocabulary {
    #[must_use]
    pub fn initial_sound(self, vocab: Vocab) -> InitialSound {
        let definition = vocab.definition();
        definition
            .initial_sound
            .unwrap_or_else(|| surface_initial_sound(definition.spelling))
    }

    #[must_use]
    pub fn render_adjective(self, adjective: &Adjective) -> Option<String> {
        match adjective {
            Adjective::Word(vocab) => vocab
                .definition()
                .adjective
                .then(|| vocab.spelling().to_owned()),
            Adjective::Color(color) => Some(color.spelling().to_owned()),
            Adjective::CardOrientation(orientation) => Some(orientation.spelling().to_owned()),
            Adjective::Participle(Tense::Present, verb) => {
                self.render_verb_identity(verb, VerbSlot::PresentParticiple)
            }
            Adjective::Participle(Tense::Past, verb) => {
                self.render_verb_identity(verb, VerbSlot::PastParticiple)
            }
            Adjective::Catalog(atom) => Some(atom.render_adjective()),
            Adjective::Ordinal(value) => Some(crate::numeral::Numeral::Ordinal.format(*value)),
        }
    }
}
