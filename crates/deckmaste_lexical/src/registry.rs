use crate::{Category, LexemeId};

/// A grammar's finite lexical-property domain, checked when sources are loaded.
#[derive(Debug, Clone)]
pub struct PropertyDomain {
    pub name: String,
    pub values: Vec<String>,
    pub set: bool,
}

/// A lexical class may intersect more than one Part of Speech.
#[derive(Debug, Clone)]
pub struct LexicalClass {
    pub name: String,
    pub categories: Vec<Category>,
}

/// A grammar name bound to an existing core closed-class Lexeme.
#[derive(Debug, Clone)]
pub struct ClosedWord {
    pub name: String,
    pub id: LexemeId,
    pub category: Category,
}

/// Grammar-owned references and domains supplied to a source adapter.
#[derive(Debug, Clone, Default)]
pub struct LexicalRegistry {
    pub words: Vec<ClosedWord>,
    pub classes: Vec<LexicalClass>,
    pub properties: Vec<PropertyDomain>,
}
