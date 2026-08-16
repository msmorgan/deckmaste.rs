use deckmaste_catalogs::CatalogKind;

use crate::catalogs::ParserCatalogs;
pub use crate::constructions::Ability;
pub use crate::constructions::Amount;
pub use crate::constructions::Article;
pub use crate::constructions::Clause;
pub use crate::constructions::Common;
pub use crate::constructions::Connive;
pub use crate::constructions::CountNp;
pub use crate::constructions::DealDamage;
pub use crate::constructions::Declarative;
pub use crate::constructions::Demonstrative;
pub use crate::constructions::DemonstrativeNp;
pub use crate::constructions::Destroy;
pub use crate::constructions::EventClause;
pub use crate::constructions::GainLife;
pub use crate::constructions::Imperative;
pub use crate::constructions::NounLexeme;
pub use crate::constructions::NounPhrase;
pub use crate::constructions::NumberAmount;
pub use crate::constructions::Pronoun;
pub use crate::constructions::PronounNp;
pub use crate::constructions::SelfReferenceNp;
pub use crate::constructions::Sentence;
pub use crate::constructions::Spell;
pub use crate::constructions::TargetNp;
pub use crate::constructions::TriggerWord;
pub use crate::constructions::Triggered;
pub use crate::constructions::Variable;
pub use crate::constructions::VariableAmount;
pub use crate::constructions::VerbLexeme;
pub use crate::constructions::VerbPhrase;
pub use crate::constructions::WhereClause;
pub use crate::constructions::WithWhere;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sign {
    Positive,
    Negative,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelfReferenceSpelling {
    Full,
    Abbreviated,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Noun {
    Lexeme(NounLexeme),
    Catalog(CatalogIdentity),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignedNumber {
    pub sign: Sign,
    pub magnitude: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogIdentity {
    pub(crate) kind: CatalogKind,
    pub(crate) spelling: String,
}

impl CatalogIdentity {
    #[must_use]
    pub fn new(
        catalogs: &ParserCatalogs,
        kind: CatalogKind,
        spelling: impl Into<String>,
    ) -> Option<Self> {
        let spelling = spelling.into();
        catalogs
            .set()
            .get(kind)
            .contains(&spelling)
            .then_some(Self { kind, spelling })
    }

    #[must_use]
    pub const fn kind(&self) -> CatalogKind {
        self.kind
    }

    #[must_use]
    pub fn spelling(&self) -> &str {
        &self.spelling
    }
}
