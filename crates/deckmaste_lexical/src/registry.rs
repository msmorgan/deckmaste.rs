use crate::{Category, Lexeme};

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

/// Grammar-owned declarations supplied by callers to a source adapter.
#[derive(Debug, Clone, Default)]
pub struct LexicalRegistry {
    pub words: Vec<Lexeme>,
    pub classes: Vec<LexicalClass>,
    pub properties: Vec<PropertyDomain>,
}
