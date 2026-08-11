//! The construction declaration IR: plain data types describing a group of
//! constructions, produced by [`crate::parse::parse_group`] and consumed by
//! [`crate::validate`]. All fields are `pub`; this module has no invariants
//! of its own — validation lives in `validate.rs`.

#[derive(Debug, Clone)]
pub struct Spanned<T> {
    pub value: T,
    pub span: proc_macro2::Span,
}

impl<T> Spanned<T> {
    pub fn call_site(value: T) -> Self {
        Self {
            value,
            span: proc_macro2::Span::call_site(),
        }
    }
}

// `proc_macro2::Span` has no `PartialEq`/`Eq` impl, so these can't be
// derived. Span is provenance metadata, not data, so equality is defined
// over `value` alone — this is what lets `FieldKind`/`SelectionPromise`
// derive `PartialEq, Eq` per the model contract.
impl<T: PartialEq> PartialEq for Spanned<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: Eq> Eq for Spanned<T> {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupDeclaration {
    pub name: Spanned<String>,
    pub backend: ConstructionBackend,
    pub constructions: Vec<ConstructionDeclaration>,
    pub elements: Vec<ElementDeclaration>,
    /// Reusable Rust record layouts addressed by construction-level lens
    /// applications. These are rebuild schemas, not semantic chart elements.
    pub lenses: Vec<LensDeclaration>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ConstructionBackend {
    #[default]
    Chart,
    Ability,
}

impl GroupDeclaration {
    /// Returns the declaration-driven inverse-dispatch family, if this group
    /// has one. Adapted families comprise the complete group and dispatch by
    /// whole-value guards. Lens families may coexist with helper
    /// constructions and dispatch their common owner through exact generated
    /// `parts_*` matches.
    #[must_use]
    pub(crate) fn inverse_dispatch_family(&self) -> Option<InverseDispatchFamily<'_>> {
        inverse_dispatch_family_from(&self.constructions.iter().collect::<Vec<_>>(), None)
    }

    /// Returns every declaration-selected inverse family, grouped by chart
    /// category as well as bound Rust target. A group may intentionally bind
    /// the same Rust type through distinct semantic categories; keeping those
    /// categories separate prevents a language-side selector from becoming a
    /// second authority.
    #[must_use]
    pub(crate) fn inverse_dispatch_families(&self) -> Vec<InverseDispatchFamily<'_>> {
        let mut categories = Vec::<&Spanned<String>>::new();
        for construction in &self.constructions {
            if !categories
                .iter()
                .any(|category| category.value == construction.category.value)
            {
                categories.push(&construction.category);
            }
        }
        categories
            .into_iter()
            .filter_map(|category| {
                let members = self
                    .constructions
                    .iter()
                    .filter(|construction| construction.category.value == category.value)
                    .collect::<Vec<_>>();
                inverse_dispatch_family_from(&members, Some(category))
            })
            .collect()
    }

    /// Returns inverse families whose bound Rust target spans more than one
    /// declared category. These compose the category dispatchers into the
    /// target-level entry point needed by consumers that hold only the Rust
    /// value, without asking the language layer to rediscover its category.
    #[must_use]
    pub(crate) fn inverse_target_dispatch_families(&self) -> Vec<InverseDispatchFamily<'_>> {
        let mut targets = Vec::<&Spanned<String>>::new();
        for construction in &self.constructions {
            let AstShape::Bind { path, .. } = &construction.ast else {
                continue;
            };
            if !targets.iter().any(|target| target.value == path.value) {
                targets.push(path);
            }
        }
        targets
            .into_iter()
            .filter_map(|target| {
                let members = self
                    .constructions
                    .iter()
                    .filter(|construction| {
                        matches!(
                            &construction.ast,
                            AstShape::Bind { path, .. } if path.value == target.value
                        )
                    })
                    .collect::<Vec<_>>();
                let first_category = &members.first()?.category.value;
                members
                    .iter()
                    .any(|construction| construction.category.value != *first_category)
                    .then(|| inverse_dispatch_family_from(&members, None))?
            })
            .collect()
    }
}

fn inverse_dispatch_family_from<'a>(
    constructions: &[&'a ConstructionDeclaration],
    category: Option<&'a Spanned<String>>,
) -> Option<InverseDispatchFamily<'a>> {
    let first = *constructions.first()?;
    if let AstShape::Bind {
        path: first_path, ..
    } = &first.ast
        && constructions
            .iter()
            .all(|construction| guarded_adapter_for_target(construction, &first_path.value))
    {
        return Some(InverseDispatchFamily {
            target: first_path,
            category,
            kind: InverseDispatchKind::ValueGuard,
        });
    }

    let lensed = constructions
        .iter()
        .copied()
        .filter(|construction| construction.lens.is_some())
        .collect::<Vec<_>>();
    let first = lensed.first()?;
    let AstShape::Bind {
        path: first_path, ..
    } = &first.ast
    else {
        return None;
    };
    lensed
        .iter()
        .all(|construction| {
            matches!(
                &construction.ast,
                AstShape::Bind { path, .. } if path.value == first_path.value
            )
        })
        .then_some(InverseDispatchFamily {
            target: first_path,
            category,
            kind: if constructions
                .iter()
                .any(|construction| guarded_adapter_for_target(construction, &first_path.value))
            {
                InverseDispatchKind::Mixed
            } else {
                InverseDispatchKind::LensParts
            },
        })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum InverseDispatchKind {
    ValueGuard,
    LensParts,
    Mixed,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct InverseDispatchFamily<'a> {
    pub target: &'a Spanned<String>,
    pub category: Option<&'a Spanned<String>>,
    pub kind: InverseDispatchKind,
}

impl InverseDispatchFamily<'_> {
    #[must_use]
    pub(crate) fn contains(self, construction: &ConstructionDeclaration) -> bool {
        if self
            .category
            .is_some_and(|category| category.value != construction.category.value)
        {
            return false;
        }
        let AstShape::Bind { path, .. } = &construction.ast else {
            return false;
        };
        if path.value != self.target.value {
            return false;
        }
        match self.kind {
            InverseDispatchKind::ValueGuard => construction.bind_adapter.is_some(),
            InverseDispatchKind::LensParts => construction.lens.is_some(),
            InverseDispatchKind::Mixed => {
                construction.lens.is_some()
                    || guarded_adapter_for_target(construction, &self.target.value)
            }
        }
    }
}

fn guarded_adapter_for_target(construction: &ConstructionDeclaration, target: &str) -> bool {
    matches!(&construction.ast, AstShape::Bind { path, .. } if path.value == target)
        && construction.bind_adapter.is_some()
        && !construction.forms.is_empty()
        && construction.forms.iter().all(|form| {
            form.inverse_guard.is_some() || (form.value_guard.is_some() && !form.fallback)
        })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstructionDeclaration {
    pub id: Spanned<String>,
    pub category: Spanned<String>,
    pub internal: bool,
    pub ast: AstShape,
    /// Optional semantic adapter for a bind whose declared construction
    /// fields do not mirror the target's stored Rust fields one-for-one.
    pub bind_adapter: Option<BindAdapter>,
    /// A reversible projection/rebuild over the bound owner. Kept outside
    /// `AstShape::fields` because lens ownership is not another semantic hole.
    pub lens: Option<LensApplication>,
    /// Optional semantic projection label. The label remains declaration
    /// metadata; generated forward projection crosses the typed
    /// `From<Owner> for Category` boundary instead of assuming that the
    /// category exposes an enum-shaped constructor with this spelling.
    pub projection: Option<Spanned<String>>,
    /// Optional borrowed inverse for a projected owner:
    /// `fn(&Category) -> Option<&Owner>`. This lets construction projection
    /// start from the sealed category value without exposing its storage.
    pub projection_inverse: Option<Spanned<String>>,
    pub constraints: Vec<Constraint>,
    /// Authored provenance for the semantic fact that makes this
    /// construction's selected chart edge meaningful. The source is kept in
    /// declaration metadata so consumers never need a parallel ID table.
    pub evidence: Option<EvidenceDeclaration>,
    pub witnesses: Vec<WitnessDeclaration>,
    pub forms: Vec<FormDeclaration>,
    pub dominance: Vec<DominanceEdge>,
    pub selection: SelectionPromise,
    /// Emit structural `Serialize` for an owned semantic output. Bind-mode
    /// constructions have no generated type and therefore cannot opt in.
    pub serialize: bool,
    /// Emit validating `Deserialize` for an owned semantic output. The raw
    /// structural payload is always routed through the generated checked
    /// constructor; this is independent from [`Self::serialize`].
    pub deserialize: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceDeclaration {
    pub kind: EvidenceKind,
    pub label: Spanned<String>,
    pub source: EvidenceSource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceKind {
    Guard,
    Feature,
    Role,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvidenceSource {
    /// The selected field value evaluated against the authored `require`
    /// predicate naming this path.
    Requirement(FieldPath),
    /// A feature on the completed construction output.
    Output(FieldPath),
    /// A feature on one selected construction input field.
    Field(FieldPath),
    /// The declaration's selected chart category.
    Category,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LensDeclaration {
    pub name: Spanned<String>,
    pub owner_type: Spanned<String>,
    /// Optional owner-local capability used when the target's fields are
    /// private. The constructor and destructurer carry the complete lens
    /// record in declaration order, so generated code never needs field
    /// visibility.
    pub adapter: Option<BindAdapter>,
    pub fields: Vec<LensFieldDeclaration>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LensFieldDeclaration {
    pub name: Spanned<String>,
    pub kind: LensFieldKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LensFieldKind {
    Value { value_type: Spanned<String> },
    Optional { value_type: Spanned<String> },
    Vector { element_type: Spanned<String> },
}

impl LensFieldKind {
    #[must_use]
    pub fn value_type(&self) -> &Spanned<String> {
        match self {
            Self::Value { value_type } | Self::Optional { value_type } => value_type,
            Self::Vector { element_type } => element_type,
        }
    }

    #[must_use]
    pub const fn is_defaultable(&self) -> bool {
        matches!(self, Self::Optional { .. } | Self::Vector { .. })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LensApplication {
    pub owner: Spanned<String>,
    /// Semantic field carrying the residual owner for an extension. `None`
    /// means the construction rebuilds from focused fields plus empty
    /// optional/vector fields.
    pub source: Option<Spanned<String>>,
    pub edits: Vec<LensEdit>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LensEdit {
    pub target: FieldPath,
    pub value: Spanned<String>,
    pub kind: LensEditKind,
    /// Typed owner-relative conversion between a semantic field and the
    /// lens owner's focused field or ordered-slice element.
    pub adapter: Option<BindAdapter>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LensEditKind {
    Focus,
    Prepend,
    Append,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BindAdapter {
    pub constructor: Spanned<String>,
    pub destructurer: Spanned<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstShape {
    Bind {
        path: Spanned<String>,
        fields: Vec<FieldBinding>,
    },
    Own {
        name: Spanned<String>,
        fields: Vec<FieldBinding>,
    },
}

impl AstShape {
    #[must_use]
    pub fn fields(&self) -> &[FieldBinding] {
        match self {
            Self::Bind { fields, .. } | Self::Own { fields, .. } => fields,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldBinding {
    pub field: Spanned<String>,
    pub kind: FieldKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldKind {
    /// A typed lexical identity supplied by a language-local provider. The
    /// compiler preserves both names as metadata and keeps the concrete Rust
    /// value intact through builders and linearization.
    Identity {
        value_type: Spanned<String>,
        provider: Spanned<String>,
    },
    Subtree {
        category: Spanned<String>,
        boxed: bool,
    },
    Scalar {
        codec: Spanned<String>,
    },
    /// A typed scalar whose stored Rust value differs from the language-local
    /// surface codec that scans and linearizes it.
    TypedScalar {
        value_type: Spanned<String>,
        codec: Spanned<String>,
    },
    /// A scalar observed on the surface but omitted from the bound semantic
    /// element. Sequence linearization asks the visitor to derive it from the
    /// member index and sequence length.
    SurfaceScalar {
        codec: Spanned<String>,
    },
    Sequence {
        element: Spanned<String>,
    },
    /// `opt <kind>`. Wraps exactly one non-Optional kind — the parser
    /// rejects `opt opt`, so nesting is unrepresentable in parsed input.
    Optional {
        inner: Box<FieldKind>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElementDeclaration {
    pub name: Spanned<String>,
    pub bind_path: Option<Spanned<String>>,
    pub fields: Vec<FieldBinding>,
    pub variants: Vec<ElementVariantDeclaration>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElementVariantDeclaration {
    pub name: Spanned<String>,
    pub payload: FieldKind,
}

#[derive(Debug, Clone)]
pub struct FieldPath {
    pub segments: Vec<Spanned<String>>,
    pub span: proc_macro2::Span,
}

impl FieldPath {
    #[must_use]
    pub fn call_site(dotted: &str) -> Self {
        Self {
            segments: dotted
                .split('.')
                .map(|s| Spanned::call_site(s.to_owned()))
                .collect(),
            span: proc_macro2::Span::call_site(),
        }
    }

    /// The path as the author dotted it — the only rendering diagnostics
    /// and abstraction keys may use.
    #[must_use]
    pub fn dotted(&self) -> String {
        let names: Vec<&str> = self.segments.iter().map(|s| s.value.as_str()).collect();
        names.join(".")
    }
}

// Span is provenance, not data (same contract as `Spanned<T>`), and
// `proc_macro2::Span` has no `PartialEq` — so equality is segments-only.
impl PartialEq for FieldPath {
    fn eq(&self, other: &Self) -> bool {
        self.segments == other.segments
    }
}

impl Eq for FieldPath {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Constraint {
    Require(Spanned<Predicate>),
    /// A predicate enforced by parse recognition but not by the typed AST
    /// builder. This keeps contextual surface admission from narrowing the
    /// construction's representable value domain.
    Recognize(Spanned<Predicate>),
    DeriveFeature {
        target: FieldPath,
        /// A concrete feature type makes the combinator a typed callback
        /// selected and dispatched by generated code. Untyped combinators
        /// retain the portable metadata-only vocabulary.
        feature_type: Option<Spanned<String>>,
        combinator: Spanned<String>,
        args: Vec<FieldPath>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Predicate {
    LenAtLeast {
        path: FieldPath,
        min: u32,
    },
    LenIs {
        path: FieldPath,
        len: u32,
    },
    In {
        path: FieldPath,
        allowed: Vec<String>,
    },
    IsSome {
        path: FieldPath,
    },
    IsNone {
        path: FieldPath,
    },
    All(Vec<Predicate>),
    Any(Vec<Predicate>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WitnessDeclaration {
    pub name: Spanned<String>,
    pub class: WitnessClass,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WitnessClass {
    Stored {
        path: FieldPath,
    },
    Derived {
        combinator: Spanned<String>,
        args: Vec<FieldPath>,
    },
    Free {
        ty: Spanned<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormDeclaration {
    pub name: Spanned<String>,
    pub ordinal: Spanned<u16>,
    pub surface: Vec<SurfaceAtom>,
    pub guard: Option<Spanned<Predicate>>,
    /// A named predicate over the complete bound/owned semantic value.
    pub value_guard: Option<Spanned<String>>,
    /// A whole-value predicate used only when selecting a construction from
    /// a mixed inverse family. Forward parse lowering deliberately ignores
    /// it, so a flattened recursive owner can identify its outermost edit
    /// without narrowing the admitted child owner.
    pub inverse_guard: Option<Spanned<String>>,
    /// An unguarded canonical fallback. It remains selectable by explicit
    /// ordinal even when a guarded form is canonical for the same value.
    pub fallback: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SurfaceAtom {
    Literal(Spanned<String>),
    Hole(FieldPath),
    Lexeme(FieldPath),
    Identity(FieldPath),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DominanceEdge {
    pub winner: Spanned<String>,
    pub loser: Spanned<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectionPromise {
    Packed,
    Unique,
}

pub const KNOWN_COMBINATORS: &[&str] = &[
    "from_first",
    "fixed",
    "complete_sentence",
    "complete_noun_phrase_coordination",
    "shared_determiner_coordination",
    "quantity_exact",
    "quantity_at_least",
    "quantity_or",
    "quantity_bound",
    "quantity_x",
    "quantity_plural",
    "quantity_plural_count",
    "quantity_mass",
];

/// `snake_case` to `PascalCase`. Shared between `validate.rs`'s EC006
/// generated-name collision check and `emit.rs`'s struct-name rendering —
/// they must agree on exactly the same transform, or a collision the
/// checker finds could differ from the one the emitter actually produces
/// (or vice versa), so the algorithm lives once.
#[must_use]
pub fn pascal_case(snake: &str) -> String {
    let mut out = String::new();
    for part in snake.split('_') {
        let mut chars = part.chars();
        if let Some(first) = chars.next() {
            out.extend(first.to_uppercase());
            out.push_str(chars.as_str());
        }
    }
    out
}

/// `PascalCase` (including ordinary acronym boundaries) to `snake_case`.
/// Shared by emitted bound-enum builder names and their collision checks.
#[must_use]
pub fn snake_case(pascal: &str) -> String {
    let chars: Vec<char> = pascal.chars().collect();
    let mut out = String::new();
    for (index, &ch) in chars.iter().enumerate() {
        if ch.is_uppercase() {
            let previous_is_lower_or_digit =
                index > 0 && (chars[index - 1].is_lowercase() || chars[index - 1].is_ascii_digit());
            let next_is_lower = chars.get(index + 1).is_some_and(|next| next.is_lowercase());
            if index > 0 && (previous_is_lower_or_digit || next_is_lower) {
                out.push('_');
            }
            out.extend(ch.to_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}

/// Returns the generated inverse dispatcher name for a bound Rust target.
///
/// Bare identifiers retain their established readable suffix. Qualified or
/// generic paths are encoded byte-for-byte so every accepted `syn::Path`
/// spelling becomes a valid identifier without collapsing distinct module
/// paths onto the same terminal type name. Validation and emission must call
/// this one function so collision checks cover the exact emitted name.
#[must_use]
pub(crate) fn inverse_target_dispatcher_name(group: &str, target: &str) -> String {
    let bare_identifier = target.bytes().enumerate().all(|(index, byte)| {
        byte == b'_' || byte.is_ascii_alphabetic() || (index > 0 && byte.is_ascii_digit())
    });
    let target = if bare_identifier {
        snake_case(target)
    } else {
        use std::fmt::Write as _;

        let mut encoded = String::with_capacity("path_".len() + target.len() * 2);
        encoded.push_str("path_");
        for byte in target.bytes() {
            write!(encoded, "{byte:02x}").expect("writing to a String cannot fail");
        }
        encoded
    };
    format!("linearize_{group}_{target}_with")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_path_equality_ignores_spans() {
        let a = FieldPath::call_site("members.last.comma");
        let mut b = FieldPath::call_site("members.last.comma");
        b.span = proc_macro2::Span::mixed_site();
        b.segments[0].span = proc_macro2::Span::mixed_site();
        assert_eq!(a, b);
        assert_ne!(a, FieldPath::call_site("members.comma"));
        assert_eq!(a.dotted(), "members.last.comma");
    }

    #[test]
    fn pascal_variant_names_become_stable_builder_suffixes() {
        assert_eq!(snake_case("EventClause"), "event_clause");
        assert_eq!(snake_case("URLValue"), "url_value");
        assert_eq!(snake_case("PowerToughness"), "power_toughness");
    }
}
