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
    /// The literal wrapper's binder name, when the wrapper is a struct
    /// variant: the splice is `Lit(value: 3)` rather than `Lit(3)`.
    pub(crate) literal_binder: Option<&'static str>,
    pub(crate) embeds: bool,
    /// The kind's dispatch set (`SupportsMacros::ALL_VARIANTS`), supplied by
    /// the derive. Empty for a hand-built `Kind`, which only costs the checks
    /// that consult it their precision.
    pub(crate) variants: &'static [&'static str],
    /// The kind's own directly-tagged variant names
    /// (`SupportsMacros::OWN_VARIANTS`, a subset of `variants`), supplied
    /// by the derive. Empty for a hand-built `Kind` and for a kind with no
    /// `flatten`/`embed` tails, where it equals `variants` anyway. Lets a
    /// consumer distinguish a variant this kind itself declares from one it
    /// only inherits transitively through a flattened/embedded payload.
    pub(crate) own_variants: &'static [&'static str],
    /// The kind's per-variant signature lookup
    /// (`SupportsMacros::ALL_SIGNATURES`), supplied by the derive. Empty for
    /// a hand-built `Kind`, which only costs the checks that consult it
    /// their precision.
    pub(crate) signatures: &'static [(&'static str, VariantSignature)],
    /// The variants of this kind that keep native candidacy under a
    /// restricted read — see [`Kind::natively_spellable`]. Empty by default:
    /// restriction suppresses every variant of a registered kind.
    pub(crate) natively_spellable: Vec<&'static str>,
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
            literal_binder: None,
            embeds: false,
            variants: &[],
            own_variants: &[],
            signatures: &[],
            natively_spellable: Vec::new(),
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

    /// This kind's own directly-tagged variant names — a subset of
    /// [`variants()`](Self::variants) that excludes anything inherited
    /// transitively through a `flatten`/`embed` tail. Empty for a hand-built
    /// `Kind` and for struct kinds, which have no variant dispatch.
    #[must_use]
    pub fn own_variants(&self) -> &'static [&'static str] {
        self.own_variants
    }

    /// The kind's own dispatch set. The derive passes
    /// [`SupportsMacros::OWN_VARIANTS`](crate::SupportsMacros::OWN_VARIANTS);
    /// a consumer that needs to tell "this kind's own name" from "a name it
    /// only inherits" — e.g. the embed-untagged recursion hazard — consults
    /// it.
    #[must_use]
    pub fn with_own_variants(mut self, variants: &'static [&'static str]) -> Self {
        self.own_variants = variants;
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

    /// The binder the literal wrapper's one field carries, when the wrapper
    /// is a struct variant (`Amount::Lit { value }`): the reader splices
    /// `Lit(value: 3)`, the spelling that variant reads.
    #[must_use]
    pub fn literal_binder(mut self, binder: &'static str) -> Self {
        self.literal_binder = Some(binder);
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

    /// Variants of this kind that keep native candidacy even in a restricted
    /// read: a consumer's own carve-outs — structural grammar, or closed
    /// atoms at a kind that is otherwise macroable. Suppression is a
    /// per-ROW question, not a per-kind one; without this it would be
    /// per-kind, and a carve-out row at a registered kind would be banned
    /// with no macro able to cover it.
    ///
    /// Which rows those are is consumer policy, exactly like which types are
    /// kinds. `macro_ron` only honors the list.
    #[must_use]
    /// EXTENDS rather than replaces: a carve-out assembled from more than one
    /// source (deckmaste builds this from two separate whitelists) must not
    /// depend on which call came last.
    pub fn natively_spellable(mut self, variants: impl IntoIterator<Item = &'static str>) -> Self {
        self.natively_spellable.extend(variants);
        self
    }

    /// Whether `variant` keeps native candidacy at this kind under a
    /// restricted read — the query counterpart of
    /// [`natively_spellable`](Self::natively_spellable).
    #[must_use]
    pub fn is_natively_spellable(&self, variant: &str) -> bool {
        self.natively_spellable.contains(&variant)
    }

    /// Whether positions of this kind take the embed-untagged fallthrough
    /// (set by [`embeds_untagged`](Self::embeds_untagged)) — the query
    /// counterpart of that builder. A consumer outside `macro_ron` (the
    /// author-surface coverage gate) needs this to identify embed hosts
    /// without hand-listing them: an identifier that is one of this kind's
    /// own variants stays safe to self-reference there, but one only
    /// inherited transitively through the embed recurses (see
    /// `deckmaste_semantics::authoring`'s embed-host ruling).
    #[must_use]
    pub fn embeds(&self) -> bool {
        self.embeds
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
