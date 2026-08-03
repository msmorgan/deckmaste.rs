//! Authored provenance, recovered from a lowered value.
//!
//! Lowering erases macro-invocation provenance, so a core value carries no
//! template and no invocation name
//! (`docs/decisions/authoring-spelling-lowering.md` §12). This index is the
//! replacement channel: it recovers the authored term a core value compiled
//! from, which is what the renderer needs to produce invocation prose.
//!
//! Keyed by VALUE, not by identity. A layer-6 grant pushes a verbatim clone of
//! its `GainAbility` payload ([CR#613.1f]), so a granted ability is `Eq` to the
//! authored one it came from; layer-3 text-changing is a documented empty slot,
//! so nothing synthesizes an ability that was never authored.
//!
//! A miss is not an error. The renderer degrades to its visible `[unrendered]`
//! marker rather than inventing prose, which is why every lookup returns
//! `Option` and nothing here panics.

use std::collections::HashMap;

use deckmaste_authoring::AbilitySubterms as _;
use deckmaste_core::Ident;
use deckmaste_lowering::Lower as _;

/// The authored term behind a lowered value.
#[derive(Debug, Clone, Default)]
pub struct ProvenanceIndex {
    abilities: HashMap<deckmaste_core::Ability, deckmaste_authoring::Ability>,
    subtypes: HashMap<Ident, deckmaste_authoring::Subtype>,
    type_defs: HashMap<Ident, deckmaste_authoring::TypeDef>,
}

impl ProvenanceIndex {
    /// Records every ability of every face, and everything nested inside them.
    pub fn insert_card(&mut self, card: &deckmaste_authoring::Card) {
        let mut top = Vec::new();
        card.push_abilities(&mut top);
        for ability in top {
            self.insert_ability(ability);
        }
    }

    /// Records every ability of a token.
    pub fn insert_token(&mut self, token: &deckmaste_authoring::Token) {
        for ability in token.abilities.iter() {
            self.insert_ability(ability);
        }
    }

    /// Records `authored` and, recursively, every ability nested inside it.
    ///
    /// Pre-order, first-insert-wins. `Innate` and `Expanded` are look-through
    /// wrappers whose payload lowers to the SAME core ability the wrapper does
    /// once the `Expansion` arms erase, and the wrapper is the half carrying
    /// the invocation template. Inserting outermost-first and never overwriting
    /// keeps the spelling that renders best; reversing the order silently
    /// degrades prose without failing any type check.
    pub fn insert_ability(&mut self, authored: &deckmaste_authoring::Ability) {
        self.insert_ability_owned(authored.clone());
    }

    fn insert_ability_owned(&mut self, authored: deckmaste_authoring::Ability) {
        let nested: Vec<deckmaste_authoring::Ability> =
            authored.nested_abilities().into_iter().cloned().collect();
        self.abilities
            .entry(authored.clone().lower())
            .or_insert(authored);
        for child in nested {
            self.insert_ability_owned(child);
        }
    }

    /// Records a registry subtype and the abilities it confers.
    ///
    /// The conferral family builds `Ability` values at layer time out of
    /// `confers` payloads, so those abilities appear on no card and have to be
    /// indexed from the registry side. `conferred_ability` is the same
    /// projection the engine applies, `Innate` wrapper and all — indexing the
    /// raw payload instead would key the map on a value the engine never
    /// produces.
    pub fn insert_subtype(&mut self, authored: &deckmaste_authoring::Subtype) {
        for property in authored.confers.iter() {
            if let Some(conferred) = property.conferred_ability() {
                self.insert_ability_owned(conferred);
            }
        }
        self.subtypes
            .entry(authored.name)
            .or_insert_with(|| authored.clone());
    }

    /// The type-def twin of [`insert_subtype`](Self::insert_subtype).
    pub fn insert_type_def(&mut self, authored: &deckmaste_authoring::TypeDef) {
        for property in authored.confers.iter() {
            if let Some(conferred) = property.conferred_ability() {
                self.insert_ability_owned(conferred);
            }
        }
        self.type_defs
            .entry(authored.name)
            .or_insert_with(|| authored.clone());
    }

    /// Folds `other` in, keeping this index's entry on any collision.
    pub fn extend(&mut self, other: &Self) {
        for (k, v) in &other.abilities {
            self.abilities.entry(k.clone()).or_insert_with(|| v.clone());
        }
        for (k, v) in &other.subtypes {
            self.subtypes.entry(*k).or_insert_with(|| v.clone());
        }
        for (k, v) in &other.type_defs {
            self.type_defs.entry(*k).or_insert_with(|| v.clone());
        }
    }

    /// The authored ability a core ability compiled from, if it was indexed.
    #[must_use]
    pub fn ability(&self, core: &deckmaste_core::Ability) -> Option<&deckmaste_authoring::Ability> {
        self.abilities.get(core)
    }

    /// Keyed by printed name: `deckmaste_core::Subtype` compares by name alone
    /// (`types`/`confers` ride along), so the name IS the identity here.
    #[must_use]
    pub fn subtype(&self, core: &deckmaste_core::Subtype) -> Option<&deckmaste_authoring::Subtype> {
        self.subtypes.get(&core.name)
    }

    /// The type-def twin of [`subtype`](Self::subtype).
    #[must_use]
    pub fn type_def(
        &self,
        core: &deckmaste_core::TypeDef,
    ) -> Option<&deckmaste_authoring::TypeDef> {
        self.type_defs.get(&core.name)
    }

    /// How many distinct abilities are indexed. Diagnostics and tests only.
    #[must_use]
    pub fn ability_count(&self) -> usize {
        self.abilities.len()
    }
}

/// `Supertype` is a closed five-variant enum on both sides, so raising one is
/// total and needs no index.
#[must_use]
pub fn raise_supertype(core: deckmaste_core::Supertype) -> deckmaste_authoring::Supertype {
    match core {
        deckmaste_core::Supertype::Basic => deckmaste_authoring::Supertype::Basic,
        deckmaste_core::Supertype::Legendary => deckmaste_authoring::Supertype::Legendary,
        deckmaste_core::Supertype::Ongoing => deckmaste_authoring::Supertype::Ongoing,
        deckmaste_core::Supertype::Snow => deckmaste_authoring::Supertype::Snow,
        deckmaste_core::Supertype::World => deckmaste_authoring::Supertype::World,
    }
}
