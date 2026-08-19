use deckmaste_catalogs::CatalogKind;

use crate::ast::CatalogIdentity;
use crate::ast::Noun;
use crate::ast::NounLexeme;
use crate::ast::Sign;
use crate::ast::SignedNumber;
use crate::constructions::Number;
use crate::context::ParseContext;

pub trait Render {
    fn render(&self, context: &ParseContext<'_>) -> String;
}

#[derive(Clone, Copy)]
enum CatalogCasing {
    Lowercase,
    Preserve,
}

trait CatalogKindCasing {
    fn casing(self) -> CatalogCasing;
}

pub(crate) struct Writer {
    output: String,
    capitalize_next: bool,
}

impl Writer {
    pub(crate) fn new() -> Self {
        Self {
            output: String::new(),
            capitalize_next: true,
        }
    }

    pub(crate) fn word(&mut self, word: &str) {
        if !self.output.is_empty() {
            self.output.push(' ');
        }
        if self.capitalize_next {
            let mut characters = word.chars();
            if let Some(first) = characters.next() {
                self.output.extend(first.to_uppercase());
                self.output.push_str(characters.as_str());
            }
            self.capitalize_next = false;
        } else {
            self.output.push_str(word);
        }
    }

    pub(crate) fn identity(&mut self, identity: &str) {
        if !self.output.is_empty() {
            self.output.push(' ');
        }
        self.output.push_str(identity);
        self.capitalize_next = false;
    }

    pub(crate) fn punctuation(&mut self, mark: char) {
        self.output.push(mark);
        self.capitalize_next = mark == '.';
    }

    pub(crate) fn finish(self) -> String {
        self.output
    }
}

pub(crate) fn render_noun(writer: &mut Writer, noun: &Noun, number: Number) {
    let (singular, casing) = match noun {
        Noun::Lexeme(NounLexeme::Player) => ("player", CatalogCasing::Lowercase),
        Noun::Catalog(CatalogIdentity { kind, spelling }) => (spelling.as_str(), kind.casing()),
    };
    let cased = match casing {
        CatalogCasing::Lowercase => singular.to_lowercase(),
        CatalogCasing::Preserve => singular.to_owned(),
    };
    let rendered = match number {
        Number::Singular => cased,
        Number::Plural => pluralize(noun, &cased),
    };
    writer.word(&rendered);
}

fn pluralize(noun: &Noun, singular: &str) -> String {
    match noun {
        Noun::Lexeme(NounLexeme::Player)
        | Noun::Catalog(CatalogIdentity {
            kind: _,
            spelling: _,
        }) => format!("{singular}s"),
    }
}

pub(crate) fn render_signed_number(writer: &mut Writer, number: &SignedNumber) {
    let SignedNumber { sign, magnitude } = number;
    let magnitude = magnitude.to_string();
    match sign {
        Sign::Positive => writer.word(&magnitude),
        Sign::Negative => writer.word(&format!("-{magnitude}")),
    }
}

impl CatalogKindCasing for CatalogKind {
    fn casing(self) -> CatalogCasing {
        match self {
            Self::CardTypes | Self::Supertypes => CatalogCasing::Lowercase,
            Self::AbilityWords
            | Self::ArtifactTypes
            | Self::BattleTypes
            | Self::CardNames
            | Self::CounterKindPhrases
            | Self::CreatureTypes
            | Self::EnchantmentTypes
            | Self::KeywordAbilities
            | Self::KeywordActions
            | Self::LandTypes
            | Self::PlaneswalkerTypes
            | Self::SpellTypes => CatalogCasing::Preserve,
        }
    }
}
