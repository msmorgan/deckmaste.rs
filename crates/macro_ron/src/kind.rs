//! The kind registry: which position types consult the macro namespace, and
//! the per-kind reader policy. Kinds are identified by the serde name of the
//! Rust type standing at the position.

use std::collections::HashMap;

use crate::Ident;

/// One macroable kind: the serde name of a type whose parse positions
/// consult the macro namespace, plus the reader policy at those positions.
#[derive(Debug, Clone)]
pub struct Kind {
    pub(crate) name: Ident,
    pub(crate) remembers: bool,
    pub(crate) literal: Option<&'static str>,
    pub(crate) embeds: bool,
}

impl Kind {
    /// A kind with the default policy: name-erasing (an expansion re-reads
    /// the body directly) and no literal sugar.
    #[must_use]
    pub fn new(name: &str) -> Self {
        Kind {
            name: name.into(),
            remembers: false,
            literal: None,
            embeds: false,
        }
    }

    /// Expanding a macro at a position of this kind remembers the invocation
    /// by wrapping the expansion in the kind's `Expanded(Expansion<Self>)`
    /// variant — the type must carry one (see [`Expansion`](crate::Expansion)).
    #[must_use]
    pub fn remembers_expansion(mut self) -> Self {
        self.remembers = true;
        self
    }

    /// Positions of this kind accept bare literal sugar: a digit-led value
    /// `N` reads as `<wrapper>(N)`. Reader sugar only — the type's own
    /// grammar stays strict.
    #[must_use]
    pub fn literal_wrapper(mut self, wrapper: &'static str) -> Self {
        self.literal = Some(wrapper);
        self
    }

    /// Positions of this kind embed another type untagged: when the leading
    /// identifier names neither one of this kind's own variants nor one of
    /// its macros, the value is re-presented to the kind's `Deserialize` via
    /// `visit_newtype_struct`, so its visitor can read the embedded type and
    /// wrap it. The kind owns *which* type it embeds (its
    /// `visit_newtype_struct` names that type's `Deserialize`, which
    /// re-enters the reader under the embedded type's own namespace —
    /// variants and macros alike).
    #[must_use]
    pub fn embeds_untagged(mut self) -> Self {
        self.embeds = true;
        self
    }
}

/// The registered kinds, keyed by position name. Built by the consumer and
/// handed to [`MacroSet::new`](crate::MacroSet::new).
#[derive(Debug, Clone, Default)]
pub struct KindSet {
    kinds: HashMap<Ident, Kind>,
}

impl KindSet {
    #[must_use]
    pub fn new() -> Self { Self::default() }

    /// Registers `kind`, replacing any previous registration of its name.
    pub fn add(&mut self, kind: Kind) { self.kinds.insert(kind.name, kind); }

    /// Whether a kind of this name is registered.
    #[must_use]
    pub fn contains(&self, name: &str) -> bool { self.kinds.contains_key(name) }

    /// How many kinds are registered.
    #[must_use]
    pub fn len(&self) -> usize { self.kinds.len() }

    #[must_use]
    pub fn is_empty(&self) -> bool { self.kinds.is_empty() }

    pub(crate) fn get(&self, name: &str) -> Option<&Kind> { self.kinds.get(name) }
}
