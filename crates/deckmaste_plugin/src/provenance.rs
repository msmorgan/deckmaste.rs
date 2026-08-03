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
//! A miss is not an error, but it is never silent: the caller renders the
//! unrecovered core value as a visible `[unrendered: …]` marker rather than
//! dropping it or inventing prose, which is why every lookup returns `Option`
//! and nothing here panics.

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

    /// Records every ability of a token, and everything nested inside them.
    ///
    /// A created token's abilities reach an object verbatim
    /// (`Cards::push_token` copies them into a synthesized card def), and the
    /// `CardId` it mints sits past the end of the companion table — so this
    /// index is the ONLY provenance a token permanent has.
    pub fn insert_token(&mut self, token: &deckmaste_authoring::Token) {
        let mut top = Vec::new();
        token.abilities.push_abilities(&mut top);
        for ability in top {
            self.insert_ability(ability);
        }
    }

    /// Records every predefined token's abilities ([CR#111.10]).
    ///
    /// `TokenSpec::Named(Treasure)` resolves to an owned `Token` built from the
    /// rules definition rather than from any plugin file, so the walker cannot
    /// reach it from a card (see `AbilitySubterms for TokenSpec`). The set is
    /// closed and cheap, so index all of it once.
    pub fn insert_predefined_tokens(&mut self) {
        for predefined in deckmaste_authoring::PredefinedToken::ALL {
            self.insert_token(&predefined.token());
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
    fn insert_ability(&mut self, authored: &deckmaste_authoring::Ability) {
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
        self.subtypes.insert(authored.name, authored.clone());
    }

    /// The type-def twin of [`insert_subtype`](Self::insert_subtype).
    pub fn insert_type_def(&mut self, authored: &deckmaste_authoring::TypeDef) {
        for property in authored.confers.iter() {
            if let Some(conferred) = property.conferred_ability() {
                self.insert_ability_owned(conferred);
            }
        }
        self.type_defs.insert(authored.name, authored.clone());
    }

    /// Records the abilities a counter kind confers.
    ///
    /// A keyword counter ([CR#122.1b]) reaches layer 6 by a DIFFERENT path than
    /// subtype conferral: the layer pass folds the counter registry's
    /// `Property::Continuous(This, GainAbility(a))` into a continuous effect
    /// and pushes `a` verbatim ([CR#613.1f]), so the key is the BARE payload —
    /// no `Innate` wrapper, unlike [`insert_subtype`](Self::insert_subtype).
    /// Indexing the `conferred_ability` form here would key the map on a value
    /// the engine never produces.
    pub fn insert_counter(&mut self, authored: &deckmaste_authoring::Counter) {
        let mut top = Vec::new();
        authored.confers.push_abilities(&mut top);
        for ability in top {
            self.insert_ability(ability);
        }
    }

    /// Folds `other` in.
    ///
    /// The name-keyed registries take `other`'s entry on a collision, so
    /// folding plugins in load order (canon, then builtin, then the corpus)
    /// resolves a redefined subtype/type to the SAME plugin the engine's own
    /// last-plugin-wins registries resolve it to.
    ///
    /// The ability map keeps this index's entry instead. It is not a registry
    /// and carries no plugin precedence: any preimage of a lowered ability is
    /// semantically exact (`docs/decisions/authoring-spelling-lowering.md` §9),
    /// so the choice is a SPELLING preference, and the same first-insert-wins
    /// rule that keeps the outermost (template-carrying) form within one
    /// insertion keeps it across a fold.
    pub fn extend(&mut self, other: &Self) {
        for (k, v) in &other.abilities {
            self.abilities.entry(k.clone()).or_insert_with(|| v.clone());
        }
        for (k, v) in &other.subtypes {
            self.subtypes.insert(*k, v.clone());
        }
        for (k, v) in &other.type_defs {
            self.type_defs.insert(*k, v.clone());
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
