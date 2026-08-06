//! The kind registry: which position types consult the macro namespace, and
//! the per-kind reader policy. Kinds are identified by the serde name of the
//! Rust type standing at the position.

use std::collections::HashMap;

use crate::Ident;
use crate::VariantSignature;

/// One macroable kind: the serde name of a type whose parse positions
/// consult the macro namespace, plus the reader policy at those positions.
#[derive(Debug, Clone)]
pub struct Kind {
    pub(crate) name: Ident,
    pub(crate) remembers: bool,
    pub(crate) literal: Option<&'static str>,
    pub(crate) embeds: bool,
    /// The kind's dispatch set (`SupportsMacros::ALL_VARIANTS`), supplied by
    /// the derive. Empty for a hand-built `Kind`, which only costs the checks
    /// that consult it their precision.
    pub(crate) variants: &'static [&'static str],
    /// The kind's per-variant signature lookup
    /// (`SupportsMacros::ALL_SIGNATURES`), supplied by the derive. Empty for
    /// a hand-built `Kind`, which only costs the checks that consult it
    /// their precision.
    pub(crate) signatures: &'static [(&'static str, VariantSignature)],
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
            variants: &[],
            signatures: &[],
        }
    }

    /// Whether an expansion at this kind is wrapped in invocation provenance
    /// (the kind carries an `Expanded(Expansion<Self>)` variant).
    #[must_use]
    pub fn remembers_invocation(&self) -> bool {
        self.remembers
    }

    /// This kind's position name (the type's serde name).
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// This kind's dispatch set — every identifier that reads natively here.
    /// Empty for a hand-built `Kind` and for struct kinds, which have no
    /// variant dispatch.
    #[must_use]
    pub fn variants(&self) -> &'static [&'static str] {
        self.variants
    }

    /// The kind's dispatch set. The derive passes
    /// [`SupportsMacros::ALL_VARIANTS`](crate::SupportsMacros::ALL_VARIANTS);
    /// checks that must distinguish a native variant from a macro name — the
    /// cycle check, and the restricted-read ban — consult it.
    #[must_use]
    pub fn with_variants(mut self, variants: &'static [&'static str]) -> Self {
        self.variants = variants;
        self
    }

    /// This kind's per-variant signature lookup — every variant name in
    /// [`variants()`](Self::variants) mapped to its authored shape. Empty for
    /// a hand-built `Kind` and for struct kinds, which have no variant
    /// dispatch.
    #[must_use]
    pub fn signatures(&self) -> &'static [(&'static str, VariantSignature)] {
        self.signatures
    }

    /// The kind's per-variant signature lookup. The derive passes
    /// [`SupportsMacros::ALL_SIGNATURES`](crate::SupportsMacros::ALL_SIGNATURES);
    /// scaffolding an identity macro consults it for each variant's arity.
    #[must_use]
    pub fn with_signatures(
        mut self,
        signatures: &'static [(&'static str, VariantSignature)],
    ) -> Self {
        self.signatures = signatures;
        self
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
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers `kind`, replacing any previous registration of its name.
    pub fn add(&mut self, kind: Kind) {
        self.kinds.insert(kind.name, kind);
    }

    /// Whether a kind of this name is registered.
    #[must_use]
    pub fn contains(&self, name: &str) -> bool {
        self.kinds.contains_key(name)
    }

    /// How many kinds are registered.
    #[must_use]
    pub fn len(&self) -> usize {
        self.kinds.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.kinds.is_empty()
    }

    /// The registered kind of this name, if any.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&Kind> {
        self.kinds.get(name)
    }

    /// Every registered kind, in unspecified order. The author-surface
    /// reachability inventory walks this.
    pub fn iter(&self) -> impl Iterator<Item = &Kind> {
        self.kinds.values()
    }
}
