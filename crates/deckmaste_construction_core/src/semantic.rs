use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;

use proc_macro2::Span;
use syn::spanned::Spanned;

use crate::feature;
use crate::feature::Feature;
use crate::feature::FeatureExpr;
use crate::feature::FeaturePlace;
use crate::feature::FeatureValue;
use crate::identifier::CHECKED_CONSTRUCTOR;
use crate::identifier::INVARIANT_CONSTRUCTOR;
use crate::identifier::key as identifier_key;
use crate::identifier::pascal_case;
use crate::identifier::path_key;
use crate::identifier::snake_case;
use crate::model::Declaration;
use crate::model::Declarations;
use crate::model::Form;
use crate::model::FormAtom;
use crate::model::SurfaceCaseTransition;
use crate::model::VerbOperand;
use crate::plan::DeclarationKey;
use crate::plan::SourceDeclarationKind;
use crate::validate::AtomContribution;
use crate::validate::CategoryRenderCapability;

/// The one sealed semantic authority produced after validation succeeds.
#[derive(Debug)]
pub(crate) struct SemanticPlan {
    declaration_keys: Vec<DeclarationKey>,
    constructions: Vec<ConstructionPlan>,
    #[allow(dead_code, reason = "Task 3 consumes sealed abstract product rows")]
    products: Vec<ProductPlan>,
    #[allow(dead_code, reason = "Task 3 consumes sealed abstract sum rows")]
    sums: Vec<SumPlan>,
    #[allow(
        dead_code,
        reason = "Tasks 3 through 5 consume sealed nullability facts"
    )]
    nullable_types: HashSet<String>,
    #[allow(
        dead_code,
        reason = "sealed morphology rows are consumed by later generation phases"
    )]
    morphologies: Vec<MorphologyPlan>,
    terminals: Vec<TerminalPlan>,
    runtime: RuntimeEmissionPlan,
    roots: Vec<RootPlan>,
    features: FeaturePlan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LengthBounds {
    min: usize,
    max: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum EdgeClass {
    Pair,
    First,
    Middle,
    Last,
}

#[derive(Debug)]
#[allow(
    dead_code,
    reason = "Tasks 3 through 5 consume the sealed product plan"
)]
pub(crate) struct ProductPlan {
    source_index: usize,
    name: String,
    fields: Vec<StructuralFieldPlan>,
    bounds_by_role: HashMap<String, LengthBounds>,
    nullable: bool,
}

#[derive(Debug)]
#[allow(dead_code, reason = "Tasks 3 through 5 consume the sealed sum plan")]
pub(crate) struct SumPlan {
    source_index: usize,
    name: String,
    alternatives: Vec<SumAlternativePlan>,
    nullable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ValueKindPlan {
    Category(String),
    Lex(String),
    Identity(String),
    Product(String),
    Sum(String),
}

#[derive(Debug)]
#[allow(
    dead_code,
    reason = "Tasks 3 through 5 consume sealed structural fields"
)]
pub(crate) struct StructuralFieldPlan {
    name: String,
    span: Span,
    kind: StructuralFieldKindPlan,
    recursive: bool,
    helper_names: Option<StructuralHelperNames>,
}

#[derive(Debug, Clone)]
pub(crate) enum StructuralFieldKindPlan {
    Required(ValueKindPlan),
    /// A construction-owned derived value whose absent and present surface
    /// branches are emitted directly by its owner.  It is deliberately not an
    /// `Optional`: the derived value is required AST data, not a nullable
    /// grammar category.
    Zeroable(ValueKindPlan),
    Optional(ValueKindPlan),
    Sequence {
        item: ValueKindPlan,
        bounds: LengthBounds,
        surface: SequenceSurfacePlan,
    },
}

#[derive(Debug)]
#[allow(
    dead_code,
    reason = "Tasks 3 through 5 consume sealed sum alternatives"
)]
pub(crate) struct SumAlternativePlan {
    name: String,
    value: ValueKindPlan,
    recursive: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct SequenceSurfacePlan {
    separator: Option<SeparatorPlan>,
    terminator: Option<FixedSurfacePlan>,
}

#[derive(Debug, Clone)]
#[allow(
    dead_code,
    reason = "Tasks 3 through 5 consume sealed separator surfaces"
)]
pub(crate) enum SeparatorPlan {
    Uniform(FixedSurfacePlan),
    Positional(Vec<PositionalSeparatorPlan>),
}

#[derive(Debug, Clone)]
#[allow(
    dead_code,
    reason = "Tasks 3 through 5 consume positional separator rows"
)]
pub(crate) struct PositionalSeparatorPlan {
    class: EdgeClass,
    surface: FixedSurfacePlan,
}

#[derive(Debug, Clone)]
pub(crate) struct FixedSurfacePlan {
    atoms: Vec<FixedSurfaceAtomPlan>,
    transition: SurfaceCaseTransition,
}

#[derive(Debug, Clone)]
#[allow(dead_code, reason = "Tasks 3 through 5 consume fixed-surface atoms")]
pub(crate) enum FixedSurfaceAtomPlan {
    Literal(String),
    Lex { terminal: String, variant: String },
}

#[derive(Debug, Clone)]
#[allow(
    dead_code,
    reason = "Tasks 3 through 5 consume the helper-name inventory"
)]
pub(crate) struct StructuralHelperNames {
    aggregate: String,
    category: String,
    rule: String,
    builder: String,
    renderer: String,
    walker: String,
}

#[derive(Debug, Default)]
pub(crate) struct StructuralSemantics {
    pub(crate) products: Vec<ProductPlan>,
    pub(crate) sums: Vec<SumPlan>,
    pub(crate) nullable_types: HashSet<String>,
    pub(crate) construction_fields: HashMap<(String, String), StructuralFieldPlan>,
    pub(crate) construction_nullability: HashMap<String, bool>,
    pub(crate) boxed_fields: HashSet<(String, String)>,
}

impl LengthBounds {
    pub(crate) const fn new(min: usize, max: Option<usize>) -> Self {
        Self { min, max }
    }

    pub(crate) const fn min(self) -> usize {
        self.min
    }

    pub(crate) const fn max(self) -> Option<usize> {
        self.max
    }

    pub(crate) fn allows(self, length: usize) -> bool {
        length >= self.min && self.max.is_none_or(|max| length <= max)
    }

    pub(crate) fn allows_at_least(self, length: usize) -> bool {
        self.max.is_none_or(|max| max >= length)
    }
}

#[allow(
    dead_code,
    reason = "Tasks 3 through 5 consume the sealed product accessors"
)]
impl ProductPlan {
    pub(crate) fn new(
        source_index: usize,
        name: String,
        fields: Vec<StructuralFieldPlan>,
        bounds_by_role: HashMap<String, LengthBounds>,
        nullable: bool,
    ) -> Self {
        Self {
            source_index,
            name,
            fields,
            bounds_by_role,
            nullable,
        }
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) const fn source_index(&self) -> usize {
        self.source_index
    }

    pub(crate) fn fields(&self) -> &[StructuralFieldPlan] {
        &self.fields
    }

    pub(crate) fn bounds(&self, role: &str) -> Option<LengthBounds> {
        self.bounds_by_role.get(role).copied()
    }

    pub(crate) const fn is_nullable(&self) -> bool {
        self.nullable
    }

    pub(crate) fn requires_constructor(&self) -> bool {
        self.fields
            .iter()
            .any(StructuralFieldPlan::emits_generated_accessor)
    }
}

#[allow(
    dead_code,
    reason = "Tasks 3 through 5 consume the sealed sum accessors"
)]
impl SumPlan {
    pub(crate) fn new(
        source_index: usize,
        name: String,
        alternatives: Vec<SumAlternativePlan>,
        nullable: bool,
    ) -> Self {
        Self {
            source_index,
            name,
            alternatives,
            nullable,
        }
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) const fn source_index(&self) -> usize {
        self.source_index
    }

    pub(crate) fn alternatives(&self) -> &[SumAlternativePlan] {
        &self.alternatives
    }

    pub(crate) const fn is_nullable(&self) -> bool {
        self.nullable
    }
}

#[allow(
    dead_code,
    reason = "Tasks 3 through 5 consume sealed structural field accessors"
)]
impl StructuralFieldPlan {
    pub(crate) fn new(
        name: String,
        span: Span,
        kind: StructuralFieldKindPlan,
        recursive: bool,
        helper_names: Option<StructuralHelperNames>,
    ) -> Self {
        Self {
            name,
            span,
            kind,
            recursive,
            helper_names,
        }
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) const fn span(&self) -> Span {
        self.span
    }

    pub(crate) fn kind(&self) -> &StructuralFieldKindPlan {
        &self.kind
    }

    pub(crate) fn emits_generated_accessor(&self) -> bool {
        matches!(
            self.kind(),
            StructuralFieldKindPlan::Sequence { bounds, .. }
                if bounds.min() != 0 || bounds.max().is_some()
        )
    }

    pub(crate) const fn is_recursive(&self) -> bool {
        self.recursive
    }

    pub(crate) fn helper_names(&self) -> Option<&StructuralHelperNames> {
        self.helper_names.as_ref()
    }
}

impl StructuralFieldKindPlan {
    pub(crate) fn value(&self) -> &ValueKindPlan {
        match self {
            Self::Required(value) | Self::Zeroable(value) | Self::Optional(value) => value,
            Self::Sequence { item, .. } => item,
        }
    }
}

#[allow(
    dead_code,
    reason = "Tasks 3 through 5 consume sealed alternative accessors"
)]
impl SumAlternativePlan {
    pub(crate) fn new(name: String, value: ValueKindPlan) -> Self {
        Self {
            name,
            value,
            recursive: false,
        }
    }

    pub(crate) const fn with_recursive(mut self, recursive: bool) -> Self {
        self.recursive = recursive;
        self
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn value(&self) -> &ValueKindPlan {
        &self.value
    }

    pub(crate) const fn is_recursive(&self) -> bool {
        self.recursive
    }
}

impl SequenceSurfacePlan {
    pub(crate) fn new(
        separator: Option<SeparatorPlan>,
        terminator: Option<FixedSurfacePlan>,
    ) -> Self {
        Self {
            separator,
            terminator,
        }
    }

    pub(crate) fn separator(&self) -> Option<&SeparatorPlan> {
        self.separator.as_ref()
    }

    pub(crate) fn terminator(&self) -> Option<&FixedSurfacePlan> {
        self.terminator.as_ref()
    }
}

#[allow(
    dead_code,
    reason = "Tasks 3 through 5 consume positional separator accessors"
)]
impl PositionalSeparatorPlan {
    pub(crate) fn new(class: EdgeClass, surface: FixedSurfacePlan) -> Self {
        Self { class, surface }
    }

    pub(crate) const fn class(&self) -> EdgeClass {
        self.class
    }

    pub(crate) fn surface(&self) -> &FixedSurfacePlan {
        &self.surface
    }
}

#[allow(
    dead_code,
    reason = "Tasks 3 through 5 consume fixed-surface accessors"
)]
impl FixedSurfacePlan {
    pub(crate) fn new(atoms: Vec<FixedSurfaceAtomPlan>, transition: SurfaceCaseTransition) -> Self {
        Self { atoms, transition }
    }

    pub(crate) fn atoms(&self) -> &[FixedSurfaceAtomPlan] {
        &self.atoms
    }

    pub(crate) const fn transition(&self) -> SurfaceCaseTransition {
        self.transition
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.atoms.iter().all(|atom| match atom {
            FixedSurfaceAtomPlan::Literal(value) => value.is_empty(),
            FixedSurfaceAtomPlan::Lex { .. } => false,
        })
    }
}

#[allow(dead_code, reason = "Tasks 3 through 5 consume helper-name accessors")]
impl StructuralHelperNames {
    pub(crate) fn new(
        aggregate: String,
        category: String,
        rule: String,
        builder: String,
        renderer: String,
        walker: String,
    ) -> Self {
        Self {
            aggregate,
            category,
            rule,
            builder,
            renderer,
            walker,
        }
    }

    pub(crate) fn all(&self) -> [&str; 6] {
        [
            &self.aggregate,
            &self.category,
            &self.rule,
            &self.builder,
            &self.renderer,
            &self.walker,
        ]
    }
}

#[derive(Debug)]
#[allow(
    clippy::large_enum_variant,
    dead_code,
    reason = "typed terminal rows deliberately own their complete lowering payloads"
)]
pub(crate) struct ConstructionPlan {
    source_index: usize,
    origin_span: Span,
    category_span: Span,
    construction_id: String,
    category: String,
    category_variant: String,
    element_type: String,
    rule_id: String,
    render_arm: String,
    visitor_method: String,
    walker: String,
    nullable: bool,
    fields: Vec<ConstructionFieldPlan>,
    invariant: InvariantPlan,
    forms: Vec<FormPlan>,
}

#[derive(Debug)]
pub(crate) struct FormPlan {
    origin_span: Span,
    name: String,
    rule_id: String,
    guard: FormGuardPlan,
    atoms: Vec<AtomPlan>,
    licensed_literal_indexes: Vec<usize>,
    nullable: bool,
}

#[derive(Debug)]
pub(crate) enum FormGuardPlan {
    Unguarded,
    Predicate(FinitePredicatePlan),
    Otherwise { guarded_form_indexes: Vec<usize> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FinitePredicatePlan {
    domains: Vec<FiniteDomainPlan>,
    accepting: Vec<FiniteAssignmentPlan>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FiniteDomainPlan {
    role: String,
    kind: FiniteDomainKindPlan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum FiniteDomainKindPlan {
    Vocab {
        terminal: String,
        variants: Vec<String>,
    },
    OptionalVocab {
        terminal: String,
        variants: Vec<String>,
    },
    OptionalPresence,
    Feature {
        feature: Feature,
        values: Vec<FeatureValue>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FiniteAssignmentPlan {
    values: Vec<FiniteValuePlan>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum FiniteValuePlan {
    Vocab(String),
    OptionalVocab(Option<String>),
    OptionalPresence(bool),
    Feature(FeatureValue),
}

#[derive(Debug)]
pub(crate) struct ConstructionFieldPlan {
    name: syn::Ident,
    kind: ConstructionFieldKind,
    terminal: String,
    value_type: syn::Path,
    structural: Option<StructuralFieldPlan>,
    invariant_bearing: bool,
    accessor_mode: Option<AccessorMode>,
    zeroable: bool,
    mobile: bool,
    mobile_scope_sibling: Option<String>,
    field_check: Option<(syn::Path, Vec<FieldCheckArgumentPlan>)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum FieldCheckArgumentPlan {
    Feature { role: String, feature: Feature },
    VerbFrameRolePrepositions { role: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ConstructionFieldKind {
    Category,
    Lex,
    Identity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AccessorMode {
    Copy,
    Borrow,
}

#[derive(Debug, Clone)]
pub(crate) struct InvariantPlan {
    alternatives: Vec<PredicateConjunctionPlan>,
    constrained_fields: Vec<syn::Ident>,
    context_identity_fields: Vec<syn::Ident>,
    category_feature_reads: Vec<(String, Feature)>,
    requires_context: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct PredicateConjunctionPlan {
    atoms: Vec<PredicateAtomPlan>,
}

#[derive(Debug, Clone)]
pub(crate) struct PredicateAtomPlan {
    subject: PredicateSubjectPlan,
    allowed: Vec<PredicateMemberPlan>,
}

#[derive(Debug, Clone)]
pub(crate) enum PredicateSubjectPlan {
    CategoryRole {
        role: syn::Ident,
        category: String,
    },
    VocabRole {
        role: syn::Ident,
        terminal: String,
        optional: bool,
    },
    OptionalPresenceRole {
        role: syn::Ident,
    },
    RoleFeature {
        role: syn::Ident,
        feature: feature::Feature,
        optional: bool,
    },
    ConstructionFeature(feature::Feature),
}

#[derive(Debug, Clone)]
pub(crate) enum PredicateMemberPlan {
    Variant(syn::Ident),
    Presence(bool),
    Feature(feature::Spanned<feature::FeatureValue>),
}

#[derive(Debug, Clone)]
pub(crate) enum AtomPlan {
    Literal(String),
    SentenceInitialLiteral(String),
    StructuralLiteral(String),
    Category {
        role: String,
        category: String,
    },
    Lex {
        role: String,
        terminal: String,
    },
    LexFixed {
        terminal: String,
        variant: String,
        path: syn::Path,
    },
    Marked {
        role: String,
        category: String,
        terminal: String,
        variant: String,
        path: syn::Path,
    },
    Identity {
        role: String,
        terminal: String,
    },
    Noun {
        role: String,
        terminal: String,
    },
    VerbFixed {
        terminal: String,
        variant: String,
        path: syn::Path,
    },
    OpenDeclaration(OpenDeclarationAtomPlan),
    Bound {
        direction: BoundDirectionPlan,
        affix: String,
        value: Box<AtomPlan>,
    },
    Circumfix {
        prefix: String,
        value: Box<AtomPlan>,
        suffix: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BoundDirectionPlan {
    Prefix,
    Suffix,
}

#[derive(Debug, Clone)]
pub(crate) struct OpenDeclarationAtomPlan {
    kind: crate::macro_def::DeclarationKind,
    name: String,
    position: crate::macro_def::GrammarPosition,
}

#[derive(Debug)]
#[allow(
    clippy::large_enum_variant,
    dead_code,
    reason = "typed terminal rows deliberately own their complete lowering payloads"
)]
pub(crate) enum TerminalPlan {
    Vocab(VocabPlan),
    Lexeme(LexemePlan),
    Binding(BindingPlan),
    ContextIdentity(ContextIdentityPlan),
    CatalogIdentity(CatalogIdentityPlan),
    SignedDecimal(SignedDecimalPlan),
    UnsignedNumber(UnsignedNumberPlan),
    DeclarationDeterminative(DeclarationDeterminativePlan),
    DeclarationNoun(DeclarationNounPlan),
    DeclarationTerm(DeclarationTermPlan),
}

impl TerminalPlan {
    fn provides_onset(&self) -> bool {
        matches!(
            self,
            Self::Vocab(_)
                | Self::Lexeme(_)
                | Self::ContextIdentity(_)
                | Self::CatalogIdentity(_)
                | Self::DeclarationDeterminative(_)
                | Self::DeclarationNoun(_)
                | Self::DeclarationTerm(_)
                | Self::Binding(BindingPlan {
                    declaration_verb: Some(_),
                    ..
                })
        )
    }

    fn provides_possessive_ending(&self) -> bool {
        match self {
            Self::Lexeme(lexeme) => {
                lexeme.morphology.recipe() == crate::morphology::MorphologyRecipe::EnglishNoun
            }
            Self::Vocab(_)
            | Self::ContextIdentity(_)
            | Self::CatalogIdentity(_)
            | Self::DeclarationDeterminative(_)
            | Self::DeclarationNoun(_)
            | Self::DeclarationTerm(_) => true,
            Self::Binding(_) | Self::SignedDecimal(_) | Self::UnsignedNumber(_) => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DeclarationKindFamily {
    Type,
    TurnPart,
    Subtype,
    SubtypeFamily(crate::macro_def::SubtypeCategory),
}

#[derive(Debug)]
pub(crate) struct DeclarationNounPlan {
    source_index: usize,
    origin: DeclarationKey,
    codec_ident: syn::Ident,
    declaration_value_ident: syn::Ident,
    closed_lexeme: Option<syn::Ident>,
    position: crate::macro_def::GrammarPosition,
    kinds: Vec<DeclarationKindFamily>,
    feature_axis: Feature,
}

#[derive(Debug)]
pub(crate) struct DeclarationDeterminativePlan {
    source_index: usize,
    origin: DeclarationKey,
    codec_ident: syn::Ident,
    lemma_ident: syn::Ident,
    closed: Vec<ClosedDeterminativePlan>,
}

#[derive(Debug)]
pub(crate) struct ClosedDeterminativePlan {
    lemma: syn::Ident,
    number_license: crate::macro_def::DeterminativeNumberLicense,
    fused_head_license: crate::macro_def::DeterminativeFusedHeadLicense,
    nominal_license: crate::macro_def::DeterminativeNominalLicense,
    bare_duration_license: crate::macro_def::DeterminativeBareDurationLicense,
    realizations: Vec<DeterminativeRealizationPlan>,
}

#[derive(Debug)]
pub(crate) struct DeterminativeRealizationPlan {
    surface: String,
    phrase_number: Option<crate::macro_def::DeterminativePhraseNumber>,
    following_onset: Option<crate::macro_def::Onset>,
}

#[derive(Debug)]
pub(crate) struct DeclarationTermPlan {
    source_index: usize,
    origin: DeclarationKey,
    codec_ident: syn::Ident,
    position: crate::macro_def::GrammarPosition,
    kinds: Vec<crate::macro_def::DeclarationKind>,
    params: Option<Vec<String>>,
    feature: crate::macro_def::SurfaceFeature,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum VerbFrameAtom {
    Literal(String),
    Lex(String, String),
    OptionalLex(String, String),
    MarkedRole(String, String, String),
    OptionalMarkedRole(String, String, String),
    Amount,
    ObjectNounPhrase,
    PredicativeComplement,
    FrameComplementPair,
    Role(String),
    OptionalRole(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum VerbFrameClass {
    Predicate,
    Auxiliary,
    ProVerb,
}

/// Compiler compatibility key for matching a realized Lexical Verb Phrase
/// against a declared Verb Frame.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct VerbFrameKey {
    class: VerbFrameClass,
    atoms: Vec<VerbFrameAtom>,
}

impl VerbFrameKey {
    pub(crate) fn atoms(&self) -> &[VerbFrameAtom] {
        &self.atoms
    }

    pub(crate) const fn class(&self) -> VerbFrameClass {
        self.class
    }

    #[cfg(test)]
    pub(crate) fn matches_frame_set(&self, frame_set: &crate::macro_def::VerbFrameSet) -> bool {
        use crate::macro_def::VerbFrameSet;

        if self.class != VerbFrameClass::Predicate {
            return false;
        }
        match frame_set {
            VerbFrameSet::Intransitive => self.atoms.is_empty(),
            VerbFrameSet::Transitive => self.atoms == [VerbFrameAtom::ObjectNounPhrase],
            VerbFrameSet::MeasureComplement => self.atoms == [VerbFrameAtom::Amount],
            VerbFrameSet::Custom { frames } => frames.iter().any(|frame| {
                frame.len() == self.atoms.len()
                    && frame.iter().zip(&self.atoms).all(|(source, planned)| {
                        match (source, planned) {
                            (
                                crate::macro_def::CustomTailAtom::Literal(source),
                                VerbFrameAtom::Literal(planned),
                            ) => source == planned,
                            (
                                crate::macro_def::CustomTailAtom::Lex(
                                    source_terminal,
                                    source_variant,
                                ),
                                VerbFrameAtom::Lex(planned_terminal, planned_variant),
                            ) => {
                                source_terminal == planned_terminal
                                    && source_variant == planned_variant
                            }
                            (crate::macro_def::CustomTailAtom::Amount, VerbFrameAtom::Amount)
                            | (
                                crate::macro_def::CustomTailAtom::ObjectNounPhrase,
                                VerbFrameAtom::ObjectNounPhrase,
                            )
                            | (
                                crate::macro_def::CustomTailAtom::PredicativeComplement,
                                VerbFrameAtom::PredicativeComplement,
                            ) => true,
                            // Rich construction roles are core-inventory
                            // frames. Plugin Custom tails intentionally keep
                            // their closed atom vocabulary.
                            _ => false,
                        }
                    })
            }),
        }
    }
}

#[derive(Debug)]
pub(crate) struct DeclarationVerbPlan {
    source_index: usize,
    origin: DeclarationKey,
    codec_ident: syn::Ident,
    declaration_value_ident: syn::Ident,
    closed_lexeme: Option<syn::Ident>,
    frame_key: VerbFrameKey,
    feature_axis: Feature,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum UnsignedPrimitive {
    U32,
    NonZeroU32,
}

impl UnsignedPrimitive {
    fn from_source(source: &crate::model::UnsignedPrimitiveSource) -> Self {
        match source {
            crate::model::UnsignedPrimitiveSource::U32 { .. } => Self::U32,
            crate::model::UnsignedPrimitiveSource::NonZeroU32 { .. } => Self::NonZeroU32,
            crate::model::UnsignedPrimitiveSource::Unsupported { .. } => {
                unreachable!("validated unsigned primitive is supported")
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct SignedDecimalPlan {
    source_index: usize,
    origin: DeclarationKey,
    origin_span: Span,
    codec_name: String,
    codec_ident: syn::Ident,
    sign_type: syn::Ident,
    positive_variant: syn::Ident,
    negative_variant: syn::Ident,
    magnitude: UnsignedPrimitive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum UnsignedNumberKind {
    EnglishCardinal,
    UnsignedDecimal,
}

#[derive(Debug)]
pub(crate) struct UnsignedNumberPlan {
    source_index: usize,
    origin: DeclarationKey,
    codec_name: String,
    codec_ident: syn::Ident,
    magnitude: UnsignedPrimitive,
    kind: UnsignedNumberKind,
}

#[derive(Debug)]
pub(crate) struct ContextIdentityPlan {
    source_index: usize,
    origin: DeclarationKey,
    origin_span: Span,
    name: String,
    ident: syn::Ident,
    aggregate_ident: syn::Ident,
    arms: Vec<ContextIdentityArmPlan>,
    canonical: syn::Ident,
}

#[derive(Debug)]
pub(crate) struct CatalogIdentityPlan {
    source_index: usize,
    origin: DeclarationKey,
    name: String,
    ident: syn::Ident,
    provider: syn::Ident,
}

/// The sealed terminal-capability projection consumed by runtime emission.
#[derive(Debug, Default)]
struct RuntimeEmissionPlan {
    vocab_indices: Vec<usize>,
    noun_lexeme_index: Option<usize>,
    verb_lexeme_index: Option<usize>,
    noun_binding_index: Option<usize>,
    direct_binding_indices: Vec<usize>,
    opaque_binding_indices: Vec<usize>,
    context_identity_indices: Vec<usize>,
    catalog_identity_indices: Vec<usize>,
    signed_decimal_index: Option<usize>,
    unsigned_number_indices: Vec<usize>,
    declaration_determinative_indices: Vec<usize>,
    declaration_noun_indices: Vec<usize>,
    declaration_term_indices: Vec<usize>,
    declaration_verb_indices: Vec<usize>,
    punctuation_literals: Vec<String>,
    scanner_origin_indices: Vec<usize>,
}

impl RuntimeEmissionPlan {
    fn seal(
        terminals: &[TerminalPlan],
        constructions: &[ConstructionPlan],
        roots: &[RootPlan],
    ) -> syn::Result<Self> {
        let mut plan = Self::default();
        for (index, terminal) in terminals.iter().enumerate() {
            match terminal {
                TerminalPlan::Vocab(_) => plan.vocab_indices.push(index),
                TerminalPlan::Lexeme(lexeme) if lexeme.is_verb_provider() => {
                    if plan.verb_lexeme_index.replace(index).is_some() {
                        return Err(syn::Error::new(
                            lexeme.name_ident().span(),
                            "sealed runtime inventory has multiple verb lexeme providers",
                        ));
                    }
                }
                TerminalPlan::Lexeme(lexeme)
                    if lexeme.morphology().recipe()
                        == crate::morphology::MorphologyRecipe::EnglishNoun =>
                {
                    if plan.noun_lexeme_index.replace(index).is_some() {
                        return Err(syn::Error::new(
                            lexeme.name_ident().span(),
                            "sealed runtime inventory has multiple noun lexeme providers",
                        ));
                    }
                }
                TerminalPlan::Binding(binding) if binding.declaration_verb().is_some() => {
                    plan.declaration_verb_indices.push(index);
                }
                TerminalPlan::Binding(binding)
                    if binding.codec_atom() == Some(crate::model::CodecAtomClass::Noun)
                        && binding.lexical_variant().is_some() =>
                {
                    if plan.noun_binding_index.replace(index).is_some() {
                        return Err(syn::Error::new(
                            binding.origin_span(),
                            "sealed runtime inventory has multiple noun terminal bindings",
                        ));
                    }
                }
                TerminalPlan::Binding(binding) if binding.lexical_variant().is_some() => {
                    if binding.build().is_some_and(|build| {
                        build.slots().len() == 1 && build.construct_is_direct_slot()
                    }) {
                        plan.direct_binding_indices.push(index);
                    } else {
                        plan.opaque_binding_indices.push(index);
                    }
                }
                TerminalPlan::Lexeme(_) | TerminalPlan::Binding(_) => {}
                TerminalPlan::ContextIdentity(_) => {
                    plan.context_identity_indices.push(index);
                }
                TerminalPlan::CatalogIdentity(_) => {
                    plan.catalog_identity_indices.push(index);
                }
                TerminalPlan::SignedDecimal(codec) => {
                    if plan.signed_decimal_index.replace(index).is_some() {
                        return Err(syn::Error::new(
                            codec.origin_span(),
                            "sealed runtime inventory has multiple signed_decimal codecs",
                        ));
                    }
                }
                TerminalPlan::UnsignedNumber(_) => {
                    plan.unsigned_number_indices.push(index);
                }
                TerminalPlan::DeclarationDeterminative(_) => {
                    plan.declaration_determinative_indices.push(index);
                }
                TerminalPlan::DeclarationNoun(_) => {
                    plan.declaration_noun_indices.push(index);
                }
                TerminalPlan::DeclarationTerm(_) => {
                    plan.declaration_term_indices.push(index);
                }
            }
        }
        plan.punctuation_literals = roots
            .iter()
            .map(|root| root.punctuation.clone())
            .filter(|punctuation| !punctuation.is_empty())
            .chain(constructions.iter().flat_map(|construction| {
                construction
                    .forms
                    .iter()
                    .flat_map(FormPlan::atoms)
                    .flat_map(|atom| {
                        let mut literals = Vec::new();
                        if let AtomPlan::Bound { affix, .. } = atom
                            && is_punctuation_literal(affix)
                        {
                            literals.push(affix.clone());
                        }
                        if let AtomPlan::Circumfix { prefix, suffix, .. } = atom {
                            literals.extend(
                                [prefix, suffix]
                                    .into_iter()
                                    .filter(|surface| is_punctuation_literal(surface))
                                    .cloned(),
                            );
                        }
                        let atom = atom.value_atom();
                        if let AtomPlan::Literal(literal) = atom
                            && is_punctuation_literal(literal)
                        {
                            literals.push(literal.clone());
                        }
                        literals
                    })
            }))
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        let mut scanner_origin_indices = plan
            .vocab_indices
            .iter()
            .chain(plan.noun_lexeme_index.iter())
            .chain(plan.verb_lexeme_index.iter())
            .chain(plan.noun_binding_index.iter())
            .chain(&plan.direct_binding_indices)
            .chain(&plan.opaque_binding_indices)
            .chain(&plan.context_identity_indices)
            .chain(&plan.catalog_identity_indices)
            .chain(plan.signed_decimal_index.iter())
            .chain(&plan.unsigned_number_indices)
            .chain(&plan.declaration_determinative_indices)
            .chain(&plan.declaration_noun_indices)
            .chain(&plan.declaration_term_indices)
            .chain(&plan.declaration_verb_indices)
            .map(|&index| terminals[index].source_index())
            .collect::<BTreeSet<_>>();
        scanner_origin_indices.extend(roots.iter().map(RootPlan::source_index));
        scanner_origin_indices.extend(
            constructions
                .iter()
                .filter(|construction| {
                    construction.forms.iter().flat_map(FormPlan::atoms).any(
                        |atom| matches!(atom, AtomPlan::Literal(literal) if is_punctuation_literal(literal)),
                    )
                })
                .map(ConstructionPlan::source_index),
        );
        plan.scanner_origin_indices = scanner_origin_indices.into_iter().collect();
        Ok(plan)
    }
}

fn is_punctuation_literal(literal: &str) -> bool {
    literal.chars().count() == 1
        && literal
            .chars()
            .all(|character| character.is_ascii_punctuation())
}

pub(crate) enum AtomTerminal<'a> {
    Vocab(&'a VocabPlan),
    Lexeme(&'a LexemePlan),
    Binding(&'a BindingPlan),
    ContextIdentity(&'a ContextIdentityPlan),
    CatalogIdentity {
        terminal_index: usize,
        plan: &'a CatalogIdentityPlan,
    },
    SignedDecimal(&'a SignedDecimalPlan),
    UnsignedNumber(&'a UnsignedNumberPlan),
    DeclarationNoun {
        terminal_index: usize,
        plan: &'a DeclarationNounPlan,
    },
    DeclarationDeterminative {
        terminal_index: usize,
        plan: &'a DeclarationDeterminativePlan,
    },
    DeclarationTerm {
        terminal_index: usize,
        plan: &'a DeclarationTermPlan,
    },
    DeclarationVerb {
        terminal_index: usize,
        plan: &'a DeclarationVerbPlan,
    },
}

#[derive(Debug)]
#[allow(
    dead_code,
    reason = "sealed rows are the next generation phase's semantic input"
)]
pub(crate) struct VocabPlan {
    source_index: usize,
    name: syn::Ident,
    name_key: String,
    variants: Vec<VocabVariantPlan>,
    features: Vec<LexemeFeaturePlan>,
}

#[derive(Debug)]
pub(crate) struct VocabVariantPlan {
    name: syn::Ident,
    word: syn::LitStr,
    onset: crate::macro_def::Onset,
}

#[derive(Debug, Clone)]
#[allow(
    dead_code,
    reason = "sealed morphology rows are consumed by later generation phases"
)]
pub(crate) struct MorphologyPlan {
    name: syn::Ident,
    name_key: String,
    feature: crate::Feature,
    recipe: crate::morphology::MorphologyRecipe,
}

#[derive(Debug)]
pub(crate) struct LexemePlan {
    source_index: usize,
    name: syn::Ident,
    name_key: String,
    variants: Vec<syn::Ident>,
    morphology: MorphologyPlan,
    surfaces: Vec<LexemeSurfacePlan>,
    irregulars: Vec<LexemeIrregularPlan>,
    features: Vec<LexemeFeaturePlan>,
    verb_provider: bool,
}

#[derive(Debug)]
pub(crate) struct LexemeFeaturePlan {
    feature: Feature,
    members: Vec<(String, FeatureValue)>,
}

#[derive(Debug)]
pub(crate) struct LexemeSurfacePlan {
    span: Span,
    member: String,
    feature: crate::macro_def::SurfaceFeature,
    surface: String,
    onset: crate::macro_def::Onset,
}

#[derive(Debug)]
#[allow(
    dead_code,
    reason = "sealed irregular rows are consumed by later generation phases"
)]
pub(crate) struct LexemeIrregularPlan {
    member: String,
    overrides: Vec<LexemeOverridePlan>,
}

#[derive(Debug)]
#[allow(
    dead_code,
    reason = "sealed irregular rows are consumed by later generation phases"
)]
pub(crate) struct LexemeOverridePlan {
    feature: crate::macro_def::SurfaceFeature,
    surface: String,
}

#[derive(Debug)]
#[allow(
    dead_code,
    reason = "sealed rows are the next generation phase's semantic input"
)]
pub(crate) struct BindingPlan {
    source_index: usize,
    origin: DeclarationKey,
    origin_span: Span,
    name: syn::Ident,
    name_key: String,
    kind: crate::model::TerminalBindingKind,
    codec_atom: Option<crate::model::CodecAtomClass>,
    value_type_name: syn::Ident,
    lexical_variant: Option<syn::Path>,
    render: Option<BindingRenderPlan>,
    build: Option<BindingBuildPlan>,
    traversal: BindingTraversalPlan,
    stored_spelling: bool,
    declaration_verb: Option<DeclarationVerbPlan>,
}

#[derive(Debug)]
pub(crate) enum BindingRenderPlan {
    Runtime(syn::Path),
    ContextIdentity(Vec<ContextIdentityArmPlan>),
}

#[derive(Debug)]
pub(crate) struct ContextIdentityArmPlan {
    variant: syn::Ident,
    accessor: syn::Ident,
}

#[derive(Debug)]
pub(crate) struct BindingBuildPlan {
    variant: syn::Ident,
    slots: Vec<syn::Ident>,
    construct: BindingBuildExprPlan,
    construct_is_direct_slot: bool,
}

#[derive(Debug)]
pub(crate) enum BindingBuildExprPlan {
    Slot(String),
    Field {
        base: Box<BindingBuildExprPlan>,
        member: syn::Member,
    },
    Call {
        function: syn::Path,
        arguments: Vec<BindingBuildExprPlan>,
    },
    Parenthesized(Box<BindingBuildExprPlan>),
}

#[derive(Debug)]
pub(crate) struct BindingTraversalPlan {
    mode: crate::model::VisitMode,
    argument: String,
    fields: Vec<TraversalFieldPlan>,
    recipe: BindingTraversalRecipe,
    leaf_callbacks: Vec<LeafCallbackPlan>,
}

#[derive(Debug)]
pub(crate) enum BindingTraversalRecipe {
    Branches(Vec<TraversalBranchPlan>),
    Variants(Vec<syn::Ident>),
    Calls(Vec<TraversalCallPlan>),
}

#[derive(Debug)]
pub(crate) struct TraversalFieldPlan {
    name: syn::Ident,
}

#[derive(Debug)]
pub(crate) struct TraversalCallPlan {
    callback: syn::Path,
    mode: crate::model::VisitMode,
    value: TraversalValuePlan,
}

#[derive(Debug)]
pub(crate) struct TraversalBranchPlan {
    value: TraversalValuePlan,
    arms: Vec<TraversalBranchArmPlan>,
}

#[derive(Debug)]
pub(crate) enum TraversalValuePlan {
    Root(String),
    Field {
        base: Box<TraversalValuePlan>,
        member: syn::Member,
    },
    Parenthesized(Box<TraversalValuePlan>),
}

#[derive(Debug)]
pub(crate) struct TraversalBranchArmPlan {
    variant: syn::Path,
    binding: syn::Ident,
    call: TraversalCallPlan,
}

#[derive(Debug)]
pub(crate) struct LeafCallbackPlan {
    name: syn::Ident,
    value_type: syn::Type,
    mode: crate::model::VisitMode,
}

#[derive(Debug)]
#[allow(
    dead_code,
    reason = "sealed rows are the next generation phase's semantic input"
)]
pub(crate) struct RootPlan {
    source_index: usize,
    category: String,
    punctuation: String,
    parse_entry: bool,
    render_entry: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ConcordClassAuthorityPlan {
    Contextual,
    Exact,
    SequenceConstraint { role: String, target: String },
    ValueConstraint { role: String, target: String },
}

#[derive(Debug)]
pub(crate) struct FeaturePlan {
    boxed_fields: HashSet<(String, String)>,
    dynamic_numbers: HashSet<String>,
    category_reads: HashMap<String, HashSet<Feature>>,
    equations: HashMap<String, Vec<feature::FeatureEquation>>,
    resolutions: HashMap<String, HashMap<feature::FeaturePlace, feature::FeatureResolution>>,
    category_render: HashMap<String, CategoryRenderCapability>,
    sequence_features: HashMap<(String, String), Vec<Feature>>,
    concord_class_carry_sums: HashSet<String>,
    cardinality_carry_categories: HashSet<String>,
    number_carry_categories: HashSet<String>,
    onset_carry_categories: HashSet<String>,
    possessive_ending_carry_categories: HashSet<String>,
}

fn seal_terminals(
    source: &Declarations,
    morphology_by_name: &HashMap<String, MorphologyPlan>,
    verb_lexeme_provider: Option<&str>,
) -> syn::Result<Vec<TerminalPlan>> {
    source
        .declarations
        .iter()
        .enumerate()
        .filter_map(|(source_index, declaration)| match declaration {
            Declaration::Vocab(vocab) => {
                Some(VocabPlan::from_source(source_index, vocab).map(TerminalPlan::Vocab))
            }
            Declaration::Lexeme(lexeme) => Some(
                LexemePlan::from_source(
                    source_index,
                    lexeme,
                    morphology_by_name
                        .get(&identifier_key(&lexeme.morphology))
                        .expect("validated lexeme morphology")
                        .clone(),
                    verb_lexeme_provider
                        .is_some_and(|provider| provider == identifier_key(&lexeme.name)),
                )
                .map(TerminalPlan::Lexeme),
            ),
            Declaration::Codec(binding) if binding.generated.is_some() => Some(Ok(
                match binding.generated.as_ref().expect("generated recipe exists") {
                    crate::model::GeneratedCodecRecipe::SignedDecimal(_) => {
                        TerminalPlan::SignedDecimal(SignedDecimalPlan::from_source(
                            source_index,
                            binding,
                        ))
                    }
                    crate::model::GeneratedCodecRecipe::DeclarationNoun(_) => {
                        TerminalPlan::DeclarationNoun(DeclarationNounPlan::from_source(
                            source_index,
                            binding,
                        ))
                    }
                    crate::model::GeneratedCodecRecipe::DeclarationDeterminative(_) => {
                        TerminalPlan::DeclarationDeterminative(
                            DeclarationDeterminativePlan::from_source(source_index, binding),
                        )
                    }
                    crate::model::GeneratedCodecRecipe::DeclarationTerm(_) => {
                        TerminalPlan::DeclarationTerm(DeclarationTermPlan::from_source(
                            source_index,
                            binding,
                        ))
                    }
                    crate::model::GeneratedCodecRecipe::DeclarationVerb(_) => {
                        TerminalPlan::Binding(BindingPlan::from_declaration_verb(
                            source_index,
                            binding,
                        ))
                    }
                    crate::model::GeneratedCodecRecipe::EnglishCardinal(_)
                    | crate::model::GeneratedCodecRecipe::UnsignedDecimal(_) => {
                        TerminalPlan::UnsignedNumber(UnsignedNumberPlan::from_source(
                            source_index,
                            binding,
                        ))
                    }
                    crate::model::GeneratedCodecRecipe::Unsupported { .. } => {
                        unreachable!("validated generated codec recipe is supported")
                    }
                },
            )),
            Declaration::Identity(binding) if binding.generated_identity.is_some() => Some(Ok(
                match binding
                    .generated_identity
                    .as_ref()
                    .expect("generated recipe")
                {
                    crate::model::GeneratedIdentityRecipe::Context(_) => {
                        TerminalPlan::ContextIdentity(ContextIdentityPlan::from_source(
                            source_index,
                            binding,
                        ))
                    }
                    crate::model::GeneratedIdentityRecipe::Catalog(_) => {
                        TerminalPlan::CatalogIdentity(CatalogIdentityPlan::from_source(
                            source_index,
                            binding,
                        ))
                    }
                    crate::model::GeneratedIdentityRecipe::Unsupported { .. } => {
                        unreachable!("validated generated identity recipe is supported")
                    }
                },
            )),
            Declaration::Codec(binding) | Declaration::Identity(binding) => {
                Some(BindingPlan::from_source(source_index, binding).map(TerminalPlan::Binding))
            }
            Declaration::Construction(_)
            | Declaration::AbstractProduct(_)
            | Declaration::AbstractSum(_)
            | Declaration::Morphology(_)
            | Declaration::Root(_) => None,
        })
        .collect()
}

impl SemanticPlan {
    #[allow(
        clippy::too_many_arguments,
        clippy::too_many_lines,
        reason = "validation seals its independent facts together into one complete plan"
    )]
    pub(crate) fn new(
        source: &Declarations,
        structural: StructuralSemantics,
        boxed_fields: HashSet<(String, String)>,
        dynamic_numbers: HashSet<String>,
        mut category_reads: HashMap<String, HashSet<Feature>>,
        equations: HashMap<String, Vec<feature::FeatureEquation>>,
        resolutions: HashMap<String, HashMap<feature::FeaturePlace, feature::FeatureResolution>>,
        category_render: HashMap<String, CategoryRenderCapability>,
        sequence_features: HashMap<(String, String), Vec<Feature>>,
        concord_class_carry_sums: HashSet<String>,
        mut atoms_by_construction: HashMap<String, (Span, Vec<AtomContribution>)>,
        mut invariants_by_construction: HashMap<String, (Span, InvariantPlan)>,
        verb_lexeme_provider: Option<&str>,
    ) -> syn::Result<Self> {
        let StructuralSemantics {
            products,
            sums,
            nullable_types,
            mut construction_fields,
            mut construction_nullability,
            boxed_fields: _,
        } = structural;
        let declaration_keys = source
            .declarations
            .iter()
            .map(DeclarationKey::from_source)
            .collect();
        let morphologies = source
            .declarations
            .iter()
            .filter_map(|declaration| {
                let Declaration::Morphology(morphology) = declaration else {
                    return None;
                };
                Some(MorphologyPlan::from_source(morphology))
            })
            .collect::<syn::Result<Vec<_>>>()?;
        let morphology_by_name = morphologies
            .iter()
            .map(|morphology| (identifier_key(morphology.name_ident()), morphology.clone()))
            .collect::<HashMap<_, _>>();
        let field_policy_terminals =
            seal_terminals(source, &morphology_by_name, verb_lexeme_provider)?;
        let mut constructions =
            source
                .declarations
                .iter()
                .enumerate()
                .filter_map(|(source_index, declaration)| match declaration {
                    Declaration::Construction(construction) => Some((source_index, construction)),
                    Declaration::AbstractProduct(_)
                    | Declaration::AbstractSum(_)
                    | Declaration::Vocab(_)
                    | Declaration::Morphology(_)
                    | Declaration::Lexeme(_)
                    | Declaration::Codec(_)
                    | Declaration::Identity(_)
                    | Declaration::Root(_) => None,
                })
                .map(|(source_index, construction)| {
                    let construction_id = identifier_key(&construction.name);
                    let (_, atoms) = atoms_by_construction.remove(&construction_id).ok_or_else(|| {
                    syn::Error::new(
                        construction.name.span(),
                        format!(
                            "sealed semantic plan is missing construction `{construction_id}`"
                        ),
                    )
                })?;
                    let (_, invariant) = invariants_by_construction
                        .remove(&construction_id)
                        .ok_or_else(|| {
                            syn::Error::new(
                                construction.name.span(),
                                format!(
                                    "sealed semantic plan is missing invariant '{construction_id}'"
                                ),
                            )
                        })?;
                    let nullable = construction_nullability
                        .remove(&construction_id)
                        .ok_or_else(|| {
                            syn::Error::new(
                                construction.name.span(),
                                format!(
                                    "sealed semantic plan is missing construction nullability '{construction_id}'"
                                ),
                            )
                        })?;
                    ConstructionPlan::from_source(
                        source_index,
                        construction,
                        &atoms,
                        invariant,
                        nullable,
                        &mut construction_fields,
                        &field_policy_terminals,
                        &nullable_types,
                    )
                })
                .collect::<syn::Result<Vec<_>>>()?;
        if let Some(((construction, role), _)) = construction_fields.into_iter().next() {
            return Err(syn::Error::new(
                Span::call_site(),
                format!(
                    "sealed semantic plan has surplus structural field `{construction}.{role}`"
                ),
            ));
        }
        if let Some((construction, _)) = construction_nullability.into_iter().next() {
            return Err(syn::Error::new(
                Span::call_site(),
                format!(
                    "sealed semantic plan has surplus construction nullability '{construction}'"
                ),
            ));
        }
        if let Some((name, (span, _))) = atoms_by_construction.into_iter().next() {
            return Err(syn::Error::new(
                span,
                format!("sealed semantic plan has surplus construction `{name}`"),
            ));
        }
        if let Some((name, (span, _))) = invariants_by_construction.into_iter().next() {
            return Err(syn::Error::new(
                span,
                format!("sealed semantic plan has surplus invariant '{name}'"),
            ));
        }
        seal_invariant_field_policy(
            &mut constructions,
            &equations,
            &resolutions,
            &category_render,
            &concord_class_carry_sums,
            &field_policy_terminals,
        )?;
        validate_generated_associated_names(&constructions, &products)?;
        seal_invariant_category_feature_reads(&constructions, &mut category_reads);
        seal_form_guard_category_feature_reads(&constructions, &equations, &mut category_reads)?;
        let checked_field_category_reads = checked_field_category_feature_reads(&constructions)?;
        let number_carry_categories =
            number_carry_categories(&constructions, &equations, &checked_field_category_reads);
        let cardinality_carry_categories = cardinality_carry_categories(&constructions, &equations);
        let onset_carry_categories = onset_carry_categories(&constructions, &equations);
        let possessive_ending_carry_categories =
            possessive_ending_carry_categories(&constructions, &equations);

        let terminals = field_policy_terminals;
        validate_onset_provider_capabilities(&constructions, &terminals, &equations)?;
        validate_possessive_ending_provider_capabilities(&constructions, &terminals, &equations)?;
        let roots: Vec<RootPlan> = source
            .declarations
            .iter()
            .enumerate()
            .filter_map(|(source_index, declaration)| match declaration {
                Declaration::Root(root) => Some(RootPlan {
                    source_index,
                    category: crate::identifier::path_key(&root.category),
                    punctuation: root
                        .punctuation
                        .as_ref()
                        .map_or_else(String::new, syn::LitStr::value),
                    parse_entry: root.eoi,
                    render_entry: root.standalone_render,
                }),
                Declaration::Construction(_)
                | Declaration::AbstractProduct(_)
                | Declaration::AbstractSum(_)
                | Declaration::Vocab(_)
                | Declaration::Morphology(_)
                | Declaration::Lexeme(_)
                | Declaration::Codec(_)
                | Declaration::Identity(_) => None,
            })
            .collect();
        let runtime = RuntimeEmissionPlan::seal(&terminals, &constructions, &roots)?;

        Ok(Self {
            declaration_keys,
            constructions,
            products,
            sums,
            nullable_types,
            morphologies,
            terminals,
            runtime,
            roots,
            features: FeaturePlan {
                boxed_fields,
                dynamic_numbers,
                category_reads,
                equations,
                resolutions,
                category_render,
                sequence_features,
                concord_class_carry_sums,
                cardinality_carry_categories,
                number_carry_categories,
                onset_carry_categories,
                possessive_ending_carry_categories,
            },
        })
    }

    pub(crate) fn declaration_count(&self) -> usize {
        self.declaration_keys.len()
    }

    pub(crate) fn declaration_keys(&self) -> &[DeclarationKey] {
        &self.declaration_keys
    }

    #[allow(
        dead_code,
        reason = "sealed rows are the next generation phase's semantic input"
    )]
    pub(crate) fn constructions(&self) -> &[ConstructionPlan] {
        &self.constructions
    }

    pub(crate) fn products(&self) -> &[ProductPlan] {
        &self.products
    }

    pub(crate) fn sums(&self) -> &[SumPlan] {
        &self.sums
    }

    pub(crate) fn explicit_sum_owns_construction_category(&self, category: &str) -> bool {
        self.sums.iter().any(|sum| sum.name() == category)
            && self
                .constructions
                .iter()
                .any(|construction| construction.category() == category)
    }

    #[allow(
        dead_code,
        reason = "Tasks 3 through 5 consume sealed nullability facts"
    )]
    pub(crate) fn is_nullable(&self, name: &str) -> bool {
        self.nullable_types.contains(name)
    }

    pub(crate) fn required_open_declarations(
        &self,
    ) -> impl Iterator<Item = (&ConstructionPlan, &OpenDeclarationAtomPlan)> {
        self.constructions.iter().flat_map(|construction| {
            construction
                .forms
                .iter()
                .flat_map(FormPlan::atoms)
                .filter_map(move |atom| match atom {
                    AtomPlan::OpenDeclaration(open) => Some((construction, open)),
                    _ => None,
                })
        })
    }

    pub(crate) fn has_open_declarations(&self) -> bool {
        self.required_open_declarations().next().is_some()
    }

    pub(crate) fn needs_parser_environment(&self) -> bool {
        self.has_open_declarations()
            || !self.runtime.declaration_noun_indices.is_empty()
            || !self.runtime.declaration_term_indices.is_empty()
            || !self.runtime.declaration_verb_indices.is_empty()
            || !self.runtime.catalog_identity_indices.is_empty()
    }

    #[allow(
        dead_code,
        reason = "sealed rows are the next generation phase's semantic input"
    )]
    pub(crate) fn terminals(&self) -> &[TerminalPlan] {
        &self.terminals
    }

    pub(crate) fn terminal_provides_onset(&self, name: &str) -> bool {
        self.terminals
            .iter()
            .find(|terminal| terminal.plan_name() == name)
            .is_some_and(TerminalPlan::provides_onset)
    }

    #[allow(
        dead_code,
        reason = "sealed morphology rows are consumed by later generation phases"
    )]
    pub(crate) fn morphologies(&self) -> &[MorphologyPlan] {
        &self.morphologies
    }

    pub(crate) fn runtime_vocabs(&self) -> impl Iterator<Item = &VocabPlan> {
        self.runtime
            .vocab_indices
            .iter()
            .map(|&index| match &self.terminals[index] {
                TerminalPlan::Vocab(vocab) => vocab,
                TerminalPlan::Lexeme(_)
                | TerminalPlan::Binding(_)
                | TerminalPlan::ContextIdentity(_)
                | TerminalPlan::CatalogIdentity(_)
                | TerminalPlan::SignedDecimal(_)
                | TerminalPlan::UnsignedNumber(_)
                | TerminalPlan::DeclarationDeterminative(_)
                | TerminalPlan::DeclarationNoun(_)
                | TerminalPlan::DeclarationTerm(_) => {
                    unreachable!("sealed runtime vocab index changed terminal kind")
                }
            })
    }

    pub(crate) fn runtime_punctuation_literals(&self) -> &[String] {
        &self.runtime.punctuation_literals
    }

    pub(crate) fn runtime_scanner_origins(&self) -> Vec<DeclarationKey> {
        self.runtime
            .scanner_origin_indices
            .iter()
            .map(|&index| self.declaration_keys[index].clone())
            .collect()
    }

    pub(crate) fn runtime_noun_lexeme(&self) -> Option<&LexemePlan> {
        self.runtime_lexeme(self.runtime.noun_lexeme_index)
    }

    pub(crate) fn runtime_verb_lexeme(&self) -> Option<&LexemePlan> {
        self.runtime_lexeme(self.runtime.verb_lexeme_index)
    }

    pub(crate) fn lexeme(&self, name: &str) -> Option<&LexemePlan> {
        self.terminals.iter().find_map(|terminal| match terminal {
            TerminalPlan::Lexeme(lexeme) if lexeme.name() == name => Some(lexeme),
            _ => None,
        })
    }

    pub(crate) fn terminal_has_feature(&self, name: &str, feature: Feature) -> bool {
        self.terminals.iter().any(|terminal| match terminal {
            TerminalPlan::Vocab(vocab) => {
                vocab.name() == name && vocab.feature_members(feature).is_some()
            }
            TerminalPlan::Lexeme(lexeme) => {
                lexeme.name() == name && lexeme.feature_members(feature).is_some()
            }
            TerminalPlan::DeclarationNoun(codec) => {
                codec.codec_name() == name
                    && codec
                        .closed_lexeme()
                        .and_then(|closed| self.lexeme(&closed.to_string()))
                        .is_some_and(|closed| closed.feature_members(feature).is_some())
            }
            TerminalPlan::DeclarationDeterminative(codec) => {
                codec.codec_name() == name
                    && matches!(
                        feature,
                        Feature::BareDurationLicense | Feature::FusedHeadLicense
                    )
            }
            TerminalPlan::Binding(_)
            | TerminalPlan::ContextIdentity(_)
            | TerminalPlan::CatalogIdentity(_)
            | TerminalPlan::SignedDecimal(_)
            | TerminalPlan::UnsignedNumber(_)
            | TerminalPlan::DeclarationTerm(_) => false,
        })
    }

    pub(crate) fn runtime_noun_binding(&self) -> Option<&BindingPlan> {
        self.runtime.noun_binding_index.map(|index| {
            let TerminalPlan::Binding(binding) = &self.terminals[index] else {
                unreachable!("sealed runtime noun-binding index changed terminal kind")
            };
            binding
        })
    }

    pub(crate) fn runtime_declaration_nouns(
        &self,
    ) -> impl Iterator<Item = (usize, &DeclarationNounPlan)> {
        self.runtime.declaration_noun_indices.iter().map(|&index| {
            let TerminalPlan::DeclarationNoun(codec) = &self.terminals[index] else {
                unreachable!("sealed runtime declaration-noun index changed terminal kind")
            };
            (index, codec)
        })
    }

    pub(crate) fn runtime_aggregate_noun(&self) -> Option<&DeclarationNounPlan> {
        let noun_lexeme = self.runtime_noun_lexeme()?.name();
        self.runtime_declaration_nouns()
            .map(|(_, codec)| codec)
            .find(|codec| {
                codec
                    .closed_lexeme()
                    .is_some_and(|closed| closed == noun_lexeme)
            })
    }

    pub(crate) fn runtime_declaration_noun_for(
        &self,
        value_type: &str,
    ) -> Option<(usize, &DeclarationNounPlan)> {
        self.runtime_declaration_nouns()
            .find(|(_, codec)| codec.codec_name() == value_type)
    }

    pub(crate) fn runtime_declaration_determinatives(
        &self,
    ) -> impl Iterator<Item = (usize, &DeclarationDeterminativePlan)> {
        self.runtime
            .declaration_determinative_indices
            .iter()
            .map(|&index| {
                let TerminalPlan::DeclarationDeterminative(codec) = &self.terminals[index] else {
                    unreachable!(
                        "sealed runtime declaration-determinative index changed terminal kind"
                    )
                };
                (index, codec)
            })
    }

    pub(crate) fn runtime_declaration_determinative_for(
        &self,
        value_type: &str,
    ) -> Option<(usize, &DeclarationDeterminativePlan)> {
        self.runtime_declaration_determinatives()
            .find(|(_, codec)| codec.codec_name() == value_type)
    }

    pub(crate) fn runtime_declaration_terms(
        &self,
    ) -> impl Iterator<Item = (usize, &DeclarationTermPlan)> {
        self.runtime.declaration_term_indices.iter().map(|&index| {
            let TerminalPlan::DeclarationTerm(codec) = &self.terminals[index] else {
                unreachable!("sealed runtime declaration-term index changed terminal kind")
            };
            (index, codec)
        })
    }

    pub(crate) fn runtime_declaration_verbs(
        &self,
    ) -> impl Iterator<Item = (usize, &DeclarationVerbPlan)> {
        self.runtime.declaration_verb_indices.iter().map(|&index| {
            let TerminalPlan::Binding(binding) = &self.terminals[index] else {
                unreachable!("sealed runtime declaration-verb index changed terminal kind")
            };
            let plan = binding
                .declaration_verb()
                .expect("sealed runtime declaration-verb binding retains its recipe");
            (index, plan)
        })
    }

    #[cfg(test)]
    pub(crate) fn runtime_declaration_verb_for(
        &self,
        value_type: &str,
    ) -> Option<(usize, &DeclarationVerbPlan)> {
        self.runtime_declaration_verbs()
            .find(|(_, codec)| codec.codec_name() == value_type)
    }

    pub(crate) fn runtime_direct_bindings(&self) -> impl Iterator<Item = &BindingPlan> {
        self.runtime_bindings(&self.runtime.direct_binding_indices)
    }

    pub(crate) fn runtime_opaque_bindings(&self) -> impl Iterator<Item = &BindingPlan> {
        self.runtime_bindings(&self.runtime.opaque_binding_indices)
    }

    pub(crate) fn runtime_context_identities(&self) -> impl Iterator<Item = &ContextIdentityPlan> {
        self.runtime.context_identity_indices.iter().map(|&index| {
            let TerminalPlan::ContextIdentity(identity) = &self.terminals[index] else {
                unreachable!("sealed runtime context-identity index changed terminal kind")
            };
            identity
        })
    }

    pub(crate) fn runtime_catalog_identities(
        &self,
    ) -> impl Iterator<Item = (usize, &CatalogIdentityPlan)> {
        self.runtime.catalog_identity_indices.iter().map(|&index| {
            let TerminalPlan::CatalogIdentity(identity) = &self.terminals[index] else {
                unreachable!("sealed runtime catalog-identity index changed terminal kind")
            };
            (index, identity)
        })
    }

    pub(crate) fn runtime_signed_decimal(&self) -> Option<&SignedDecimalPlan> {
        self.runtime.signed_decimal_index.map(|index| {
            let TerminalPlan::SignedDecimal(codec) = &self.terminals[index] else {
                unreachable!("sealed runtime signed-decimal index changed terminal kind")
            };
            codec
        })
    }

    pub(crate) fn runtime_unsigned_numbers(&self) -> impl Iterator<Item = &UnsignedNumberPlan> {
        self.runtime.unsigned_number_indices.iter().map(|&index| {
            let TerminalPlan::UnsignedNumber(codec) = &self.terminals[index] else {
                unreachable!("sealed runtime unsigned-number index changed terminal kind")
            };
            codec
        })
    }

    fn runtime_lexeme(&self, index: Option<usize>) -> Option<&LexemePlan> {
        index.map(|index| {
            let TerminalPlan::Lexeme(lexeme) = &self.terminals[index] else {
                unreachable!("sealed runtime lexeme index changed terminal kind")
            };
            lexeme
        })
    }

    fn runtime_bindings<'a>(
        &'a self,
        indices: &'a [usize],
    ) -> impl Iterator<Item = &'a BindingPlan> + 'a {
        indices.iter().map(|&index| {
            let TerminalPlan::Binding(binding) = &self.terminals[index] else {
                unreachable!("sealed runtime binding index changed terminal kind")
            };
            binding
        })
    }

    #[allow(
        dead_code,
        reason = "sealed rows are the next generation phase's semantic input"
    )]
    pub(crate) fn roots(&self) -> &[RootPlan] {
        &self.roots
    }

    pub(crate) fn feature_equations(&self, construction: &str) -> &[feature::FeatureEquation] {
        self.features
            .equations
            .get(construction)
            .map_or(&[], Vec::as_slice)
    }

    pub(crate) fn feature_resolution(
        &self,
        construction: &str,
        place: &feature::FeaturePlace,
    ) -> Option<feature::FeatureResolution> {
        self.features
            .resolutions
            .get(construction)
            .and_then(|resolutions| resolutions.get(place))
            .copied()
    }

    pub(crate) fn category_render_capability(&self, category: &str) -> CategoryRenderCapability {
        self.features
            .category_render
            .get(category)
            .copied()
            .unwrap_or_default()
    }

    pub(crate) fn category_reads_feature(&self, category: &str, feature: Feature) -> bool {
        self.features
            .category_reads
            .get(category)
            .is_some_and(|features| features.contains(&feature))
    }

    pub(crate) fn category_carries_concord_class(&self, category: &str) -> bool {
        self.category_render_capability(category)
            .carries_concord_class()
            || self.features.concord_class_carry_sums.contains(category)
            || self.carries_feature(category, Feature::ConcordClass)
    }

    pub(crate) fn sum_carries_concord_class(&self, sum: &str) -> bool {
        self.features.concord_class_carry_sums.contains(sum)
    }

    pub(crate) fn carries_feature(&self, value: &str, feature: Feature) -> bool {
        fn carries(
            plan: &SemanticPlan,
            value: &str,
            feature: Feature,
            visiting: &mut HashSet<String>,
        ) -> bool {
            if !visiting.insert(value.to_owned()) {
                return false;
            }
            let direct = plan
                .constructions
                .iter()
                .filter(|construction| construction.category() == value)
                .collect::<Vec<_>>();
            let result = if !direct.is_empty() {
                direct.iter().all(|construction| plan.feature_equations(construction.construction_id()).iter().any(|equation| matches!(equation.target(), crate::feature::FeaturePlace::Construction(found) if *found == feature)))
            } else if let Some(sum) = plan.sums.iter().find(|sum| sum.name() == value) {
                !sum.alternatives().is_empty()
                    && sum
                        .alternatives()
                        .iter()
                        .all(|alternative| match alternative.value() {
                            ValueKindPlan::Category(category) | ValueKindPlan::Sum(category) => {
                                carries(plan, category, feature, visiting)
                            }
                            ValueKindPlan::Product(product) => plan
                                .constructions
                                .iter()
                                .find(|construction| construction.element_type() == product)
                                .is_some_and(|construction| {
                                    plan.feature_equations(construction.construction_id())
                                        .iter()
                                        .any(|equation| {
                                            matches!(equation.target(), crate::feature::FeaturePlace::Construction(found) if *found == feature)
                                        })
                                }),
                            ValueKindPlan::Lex(_)
                            | ValueKindPlan::Identity(_) => false,
                        })
            } else {
                false
            };
            visiting.remove(value);
            result
        }
        carries(self, value, feature, &mut HashSet::new())
    }

    pub(crate) fn sum_requires_external_concord_class(&self, sum: &str) -> bool {
        fn requires(plan: &SemanticPlan, sum: &str, visiting: &mut HashSet<String>) -> bool {
            if !visiting.insert(sum.to_owned()) {
                return false;
            }
            let result = plan
                .sums
                .iter()
                .find(|candidate| candidate.name() == sum)
                .is_some_and(|sum| {
                    sum.alternatives()
                        .iter()
                        .any(|alternative| match alternative.value() {
                            ValueKindPlan::Category(category) => {
                                plan.category_requires_external_concord_class(category)
                            }
                            ValueKindPlan::Sum(nested) => requires(plan, nested, visiting),
                            ValueKindPlan::Product(_)
                            | ValueKindPlan::Lex(_)
                            | ValueKindPlan::Identity(_) => false,
                        })
                });
            visiting.remove(sum);
            result
        }

        requires(self, sum, &mut HashSet::new())
    }

    pub(crate) fn sequence_features(&self, owner: &str, role: &str) -> &[Feature] {
        self.features
            .sequence_features
            .get(&(owner.to_owned(), role.to_owned()))
            .map(Vec::as_slice)
            .unwrap_or_default()
    }

    pub(crate) fn category_requires_external_concord_class(&self, category: &str) -> bool {
        self.category_render_capability(category)
            .requires_external_concord_class()
    }

    pub(crate) fn construction_concord_class_authority(
        &self,
        construction: &ConstructionPlan,
    ) -> ConcordClassAuthorityPlan {
        self.construction_concord_class_authority_inner(construction)
    }

    fn construction_concord_class_authority_inner(
        &self,
        construction: &ConstructionPlan,
    ) -> ConcordClassAuthorityPlan {
        let place = feature::FeaturePlace::Construction(Feature::ConcordClass);
        if self.feature_resolution(construction.construction_id(), &place)
            == Some(feature::FeatureResolution::External)
        {
            return ConcordClassAuthorityPlan::Contextual;
        }
        let Some(equation) = self
            .feature_equations(construction.construction_id())
            .iter()
            .find(|equation| equation.target() == &place)
        else {
            return ConcordClassAuthorityPlan::Contextual;
        };
        let feature::FeatureExpr::FromRole {
            role,
            feature: Feature::ConcordClass,
        } = equation.value()
        else {
            return ConcordClassAuthorityPlan::Exact;
        };
        let role = identifier_key(role);
        let Ok(field) = construction.field(&role) else {
            return ConcordClassAuthorityPlan::Contextual;
        };
        let (sequence, value) = match field.structural_kind() {
            Some(StructuralFieldKindPlan::Sequence { item, .. }) => (true, item),
            Some(StructuralFieldKindPlan::Required(value)) => (false, value),
            Some(StructuralFieldKindPlan::Zeroable(_) | StructuralFieldKindPlan::Optional(_)) => {
                return ConcordClassAuthorityPlan::Exact;
            }
            None if field.kind() == ConstructionFieldKind::Category => {
                let terminal = field.terminal();
                let value = if self.sum_carries_concord_class(terminal) {
                    ValueKindPlan::Sum(terminal.to_owned())
                } else {
                    ValueKindPlan::Category(terminal.to_owned())
                };
                return self.concord_class_authority_for_value(&role, false, &value);
            }
            None => return ConcordClassAuthorityPlan::Exact,
        };
        self.concord_class_authority_for_value(&role, sequence, value)
    }

    fn concord_class_authority_for_value(
        &self,
        role: &str,
        sequence: bool,
        value: &ValueKindPlan,
    ) -> ConcordClassAuthorityPlan {
        let target = match value {
            ValueKindPlan::Sum(sum) if self.sum_carries_concord_class(sum) => Some(sum.clone()),
            ValueKindPlan::Category(category) if self.category_carries_concord_class(category) => {
                Some(category.clone())
            }
            ValueKindPlan::Category(_)
            | ValueKindPlan::Sum(_)
            | ValueKindPlan::Product(_)
            | ValueKindPlan::Lex(_)
            | ValueKindPlan::Identity(_) => None,
        };
        if target
            .as_deref()
            .is_some_and(|category| self.category_requires_external_concord_class(category))
        {
            return ConcordClassAuthorityPlan::Contextual;
        }
        match (sequence, target) {
            (true, Some(target)) => ConcordClassAuthorityPlan::SequenceConstraint {
                role: role.to_owned(),
                target,
            },
            (false, Some(target)) => ConcordClassAuthorityPlan::ValueConstraint {
                role: role.to_owned(),
                target,
            },
            (_, None) => ConcordClassAuthorityPlan::Exact,
        }
    }

    pub(crate) fn construction_requires_checked_ast(
        &self,
        construction: &ConstructionPlan,
    ) -> bool {
        construction.requires_constructor()
            || self
                .feature_equations(construction.construction_id())
                .iter()
                .any(|equation| {
                    let feature::FeaturePlace::Role { field, feature } = equation.target() else {
                        return false;
                    };
                    construction
                        .field(&identifier_key(field))
                        .is_ok_and(|field| {
                            if field.kind() != ConstructionFieldKind::Category {
                                return false;
                            }
                            match feature {
                                Feature::ConcordClass => {
                                    self.sum_carries_concord_class(field.terminal())
                                        || self.category_carries_concord_class(field.terminal())
                                }
                                Feature::Number => self.category_carries_number(field.terminal()),
                                _ => false,
                            }
                        })
                })
    }

    pub(crate) fn boxed_fields(&self) -> &HashSet<(String, String)> {
        &self.features.boxed_fields
    }

    pub(crate) fn dynamic_number_constructions(&self) -> &HashSet<String> {
        &self.features.dynamic_numbers
    }

    pub(crate) fn category_carries_number(&self, category: &str) -> bool {
        self.features.number_carry_categories.contains(category)
    }

    pub(crate) fn category_carries_cardinality(&self, category: &str) -> bool {
        self.features
            .cardinality_carry_categories
            .contains(category)
    }

    pub(crate) fn category_carries_onset(&self, category: &str) -> bool {
        self.features.onset_carry_categories.contains(category)
    }

    pub(crate) fn category_carries_possessive_ending(&self, category: &str) -> bool {
        self.features
            .possessive_ending_carry_categories
            .contains(category)
    }

    #[cfg(test)]
    pub(crate) fn test_only_replace_invariant_member(
        &mut self,
        construction_id: &str,
        role: &str,
        member: &str,
    ) {
        let construction = self
            .constructions
            .iter_mut()
            .find(|construction| construction.construction_id == construction_id)
            .expect("test construction is present");
        let atom = construction
            .invariant
            .alternatives
            .iter_mut()
            .flat_map(|alternative| &mut alternative.atoms)
            .find(|atom| {
                atom.subject
                    .role()
                    .is_some_and(|candidate| identifier_key(candidate) == role)
            })
            .expect("test invariant role is present");
        let PredicateMemberPlan::Variant(existing) = &mut atom.allowed[0] else {
            panic!("test invariant member is a variant");
        };
        *existing = syn::Ident::new(member, existing.span());
    }

    #[cfg(test)]
    pub(crate) fn test_only_remove_invariant_context_field(
        &mut self,
        construction_id: &str,
        field: &str,
    ) {
        let terminals = &self.terminals;
        let construction = self
            .constructions
            .iter_mut()
            .find(|construction| construction.construction_id == construction_id)
            .expect("test construction is present");
        let before = construction.invariant.context_identity_fields.len();
        construction
            .invariant
            .context_identity_fields
            .retain(|candidate| identifier_key(candidate) != field);
        assert_eq!(
            construction.invariant.context_identity_fields.len() + 1,
            before,
            "test context-identity field is present",
        );
        construction.invariant.requires_context =
            !construction.invariant.context_identity_fields.is_empty();
        construction
            .invariant
            .apply_field_policy(&mut construction.fields, terminals)
            .expect("mutated test field policy remains sealable");
    }

    #[cfg(test)]
    pub(crate) fn test_only_replace_planned_literal(
        &mut self,
        construction_id: &str,
        atom_index: usize,
        literal: &str,
    ) {
        let construction = self
            .constructions
            .iter_mut()
            .find(|row| row.construction_id == construction_id)
            .expect("test construction is present");
        construction.forms[0].atoms[atom_index] = AtomPlan::Literal(literal.to_owned());
    }

    #[cfg(test)]
    pub(crate) fn test_only_replace_open_declaration_name(
        &mut self,
        construction_id: &str,
        atom_index: usize,
        name: &str,
    ) {
        let construction = self
            .constructions
            .iter_mut()
            .find(|row| row.construction_id == construction_id)
            .expect("test construction is present");
        let AtomPlan::OpenDeclaration(open) = &mut construction.forms[0].atoms[atom_index] else {
            panic!("test atom is an open declaration")
        };
        open.name = name.to_owned();
    }

    #[cfg(test)]
    pub(crate) fn test_only_replace_vocab_spelling(
        &mut self,
        vocabulary: &str,
        variant: &str,
        spelling: &str,
    ) {
        let vocab = self
            .terminals
            .iter_mut()
            .find_map(|terminal| match terminal {
                TerminalPlan::Vocab(vocab) if vocab.name() == vocabulary => Some(vocab),
                TerminalPlan::Vocab(_)
                | TerminalPlan::Lexeme(_)
                | TerminalPlan::Binding(_)
                | TerminalPlan::ContextIdentity(_)
                | TerminalPlan::CatalogIdentity(_)
                | TerminalPlan::SignedDecimal(_)
                | TerminalPlan::UnsignedNumber(_)
                | TerminalPlan::DeclarationDeterminative(_)
                | TerminalPlan::DeclarationNoun(_)
                | TerminalPlan::DeclarationTerm(_) => None,
            })
            .expect("test vocabulary is present");
        let word = vocab
            .variants
            .iter_mut()
            .find(|candidate| identifier_key(&candidate.name) == variant)
            .expect("test vocabulary variant is present");
        word.word = syn::LitStr::new(spelling, word.word.span());
    }

    #[cfg(test)]
    pub(crate) fn test_only_replace_vocab_name(&mut self, old: &str, new: &str) {
        let vocab = self
            .terminals
            .iter_mut()
            .find_map(|terminal| match terminal {
                TerminalPlan::Vocab(vocab) if vocab.name() == old => Some(vocab),
                TerminalPlan::Vocab(_)
                | TerminalPlan::Lexeme(_)
                | TerminalPlan::Binding(_)
                | TerminalPlan::ContextIdentity(_)
                | TerminalPlan::CatalogIdentity(_)
                | TerminalPlan::SignedDecimal(_)
                | TerminalPlan::UnsignedNumber(_)
                | TerminalPlan::DeclarationDeterminative(_)
                | TerminalPlan::DeclarationNoun(_)
                | TerminalPlan::DeclarationTerm(_) => None,
            })
            .expect("test vocabulary is present");
        vocab.name = syn::Ident::new(new, vocab.name.span());
        vocab.name_key = new.to_owned();
    }

    #[cfg(test)]
    pub(crate) fn test_only_replace_binding_name(&mut self, old: &str, new: &str) {
        let binding = self
            .terminals
            .iter_mut()
            .find_map(|terminal| match terminal {
                TerminalPlan::Binding(binding) if binding.name() == old => Some(binding),
                TerminalPlan::Vocab(_)
                | TerminalPlan::Lexeme(_)
                | TerminalPlan::Binding(_)
                | TerminalPlan::ContextIdentity(_)
                | TerminalPlan::CatalogIdentity(_)
                | TerminalPlan::SignedDecimal(_)
                | TerminalPlan::UnsignedNumber(_)
                | TerminalPlan::DeclarationDeterminative(_)
                | TerminalPlan::DeclarationNoun(_)
                | TerminalPlan::DeclarationTerm(_) => None,
            })
            .expect("test binding is present");
        binding.name = syn::Ident::new(new, binding.name.span());
        binding.name_key = new.to_owned();
        binding.origin = DeclarationKey::new(binding.origin.kind(), new);
    }

    #[cfg(test)]
    pub(crate) fn test_only_replace_signed_decimal_sign_shape(
        &mut self,
        sign_type: &str,
        positive: &str,
        negative: &str,
    ) {
        let codec = self
            .terminals
            .iter_mut()
            .find_map(|terminal| match terminal {
                TerminalPlan::SignedDecimal(codec) => Some(codec),
                TerminalPlan::Vocab(_)
                | TerminalPlan::Lexeme(_)
                | TerminalPlan::Binding(_)
                | TerminalPlan::ContextIdentity(_)
                | TerminalPlan::CatalogIdentity(_)
                | TerminalPlan::UnsignedNumber(_)
                | TerminalPlan::DeclarationDeterminative(_)
                | TerminalPlan::DeclarationNoun(_)
                | TerminalPlan::DeclarationTerm(_) => None,
            })
            .expect("test signed_decimal codec is present");
        codec.sign_type = syn::Ident::new(sign_type, codec.sign_type.span());
        codec.positive_variant = syn::Ident::new(positive, codec.positive_variant.span());
        codec.negative_variant = syn::Ident::new(negative, codec.negative_variant.span());
    }

    #[cfg(test)]
    pub(crate) fn test_only_replace_signed_decimal_codec_name(&mut self, new: &str) {
        let codec = self
            .terminals
            .iter_mut()
            .find_map(|terminal| match terminal {
                TerminalPlan::SignedDecimal(codec) => Some(codec),
                TerminalPlan::Vocab(_)
                | TerminalPlan::Lexeme(_)
                | TerminalPlan::Binding(_)
                | TerminalPlan::ContextIdentity(_)
                | TerminalPlan::CatalogIdentity(_)
                | TerminalPlan::UnsignedNumber(_)
                | TerminalPlan::DeclarationDeterminative(_)
                | TerminalPlan::DeclarationNoun(_)
                | TerminalPlan::DeclarationTerm(_) => None,
            })
            .expect("test signed_decimal codec is present");
        let old = codec.codec_name.clone();
        codec.codec_name = new.to_owned();
        codec.codec_ident = syn::Ident::new(new, codec.codec_ident.span());
        codec.origin = DeclarationKey::new(SourceDeclarationKind::Codec, new);

        for construction in &mut self.constructions {
            for field in &mut construction.fields {
                if field.terminal == old {
                    field.terminal = new.to_owned();
                    field
                        .value_type
                        .segments
                        .last_mut()
                        .expect("validated test field type has a segment")
                        .ident = syn::Ident::new(new, field.value_type.span());
                }
            }
            for atom in construction
                .forms
                .iter_mut()
                .flat_map(|form| &mut form.atoms)
            {
                if let AtomPlan::Lex { terminal, .. } = atom
                    && *terminal == old
                {
                    *terminal = new.to_owned();
                }
            }
        }
    }

    #[cfg(test)]
    pub(crate) fn context_identity_snapshot(&self) -> Vec<(String, Vec<String>, String)> {
        self.terminals
            .iter()
            .filter_map(|terminal| match terminal {
                TerminalPlan::ContextIdentity(identity) => Some((
                    identity.name.clone(),
                    identity
                        .arms
                        .iter()
                        .map(|arm| format!("{}=>{}", arm.variant, arm.accessor))
                        .collect(),
                    identity.canonical.to_string(),
                )),
                TerminalPlan::Vocab(_)
                | TerminalPlan::Lexeme(_)
                | TerminalPlan::Binding(_)
                | TerminalPlan::CatalogIdentity(_)
                | TerminalPlan::SignedDecimal(_)
                | TerminalPlan::UnsignedNumber(_)
                | TerminalPlan::DeclarationDeterminative(_)
                | TerminalPlan::DeclarationNoun(_)
                | TerminalPlan::DeclarationTerm(_) => None,
            })
            .collect()
    }

    #[cfg(test)]
    pub(crate) fn test_only_replace_context_identity_shape(
        &mut self,
        old: &str,
        new: &str,
        canonical: &str,
        alternate: &str,
    ) {
        let identity = self
            .terminals
            .iter_mut()
            .find_map(|terminal| match terminal {
                TerminalPlan::ContextIdentity(identity) if identity.name() == old => Some(identity),
                TerminalPlan::Vocab(_)
                | TerminalPlan::Lexeme(_)
                | TerminalPlan::Binding(_)
                | TerminalPlan::ContextIdentity(_)
                | TerminalPlan::CatalogIdentity(_)
                | TerminalPlan::SignedDecimal(_)
                | TerminalPlan::UnsignedNumber(_)
                | TerminalPlan::DeclarationDeterminative(_)
                | TerminalPlan::DeclarationNoun(_)
                | TerminalPlan::DeclarationTerm(_) => None,
            })
            .expect("test context identity is present");
        identity.name = new.to_owned();
        identity.ident = syn::Ident::new(new, identity.ident.span());
        identity.aggregate_ident = syn::Ident::new(new, identity.aggregate_ident.span());
        identity.origin = DeclarationKey::new(SourceDeclarationKind::Identity, new);
        identity.arms[0].variant = syn::Ident::new(canonical, identity.arms[0].variant.span());
        identity.arms[1].variant = syn::Ident::new(alternate, identity.arms[1].variant.span());
        identity.canonical = syn::Ident::new(canonical, identity.canonical.span());

        for construction in &mut self.constructions {
            for field in &mut construction.fields {
                if field.terminal == old {
                    field.terminal = new.to_owned();
                    field
                        .value_type
                        .segments
                        .last_mut()
                        .expect("validated test field type has a segment")
                        .ident = syn::Ident::new(new, field.value_type.span());
                }
            }
            for atom in construction
                .forms
                .iter_mut()
                .flat_map(|form| &mut form.atoms)
            {
                if let AtomPlan::Identity { terminal, .. } = atom
                    && *terminal == old
                {
                    *terminal = new.to_owned();
                }
            }
        }
    }

    #[cfg(test)]
    pub(crate) fn test_only_replace_context_identity_accessors(
        &mut self,
        name: &str,
        canonical_accessor: &str,
        alternate_accessor: &str,
    ) {
        let identity = self
            .terminals
            .iter_mut()
            .find_map(|terminal| match terminal {
                TerminalPlan::ContextIdentity(identity) if identity.name() == name => {
                    Some(identity)
                }
                TerminalPlan::Vocab(_)
                | TerminalPlan::Lexeme(_)
                | TerminalPlan::Binding(_)
                | TerminalPlan::ContextIdentity(_)
                | TerminalPlan::CatalogIdentity(_)
                | TerminalPlan::SignedDecimal(_)
                | TerminalPlan::UnsignedNumber(_)
                | TerminalPlan::DeclarationDeterminative(_)
                | TerminalPlan::DeclarationNoun(_)
                | TerminalPlan::DeclarationTerm(_) => None,
            })
            .expect("test context identity is present");
        identity.arms[0].accessor =
            syn::Ident::new(canonical_accessor, identity.arms[0].accessor.span());
        identity.arms[1].accessor =
            syn::Ident::new(alternate_accessor, identity.arms[1].accessor.span());
    }

    #[cfg(test)]
    pub(crate) fn test_only_replace_binding_build_variant(&mut self, name: &str, variant: &str) {
        let binding = self
            .terminals
            .iter_mut()
            .find_map(|terminal| match terminal {
                TerminalPlan::Binding(binding) if binding.name() == name => Some(binding),
                TerminalPlan::Vocab(_)
                | TerminalPlan::Lexeme(_)
                | TerminalPlan::Binding(_)
                | TerminalPlan::ContextIdentity(_)
                | TerminalPlan::CatalogIdentity(_)
                | TerminalPlan::SignedDecimal(_)
                | TerminalPlan::UnsignedNumber(_)
                | TerminalPlan::DeclarationDeterminative(_)
                | TerminalPlan::DeclarationNoun(_)
                | TerminalPlan::DeclarationTerm(_) => None,
            })
            .expect("test binding is present");
        let build = binding
            .build
            .as_mut()
            .expect("test binding build is present");
        build.variant = syn::Ident::new(variant, build.variant.span());
    }

    #[cfg(test)]
    pub(crate) fn test_only_replace_binding_value_type_name(&mut self, name: &str, value: &str) {
        let binding = self
            .terminals
            .iter_mut()
            .find_map(|terminal| match terminal {
                TerminalPlan::Binding(binding) if binding.name() == name => Some(binding),
                TerminalPlan::Vocab(_)
                | TerminalPlan::Lexeme(_)
                | TerminalPlan::Binding(_)
                | TerminalPlan::ContextIdentity(_)
                | TerminalPlan::CatalogIdentity(_)
                | TerminalPlan::SignedDecimal(_)
                | TerminalPlan::UnsignedNumber(_)
                | TerminalPlan::DeclarationDeterminative(_)
                | TerminalPlan::DeclarationNoun(_)
                | TerminalPlan::DeclarationTerm(_) => None,
            })
            .expect("test binding is present");
        binding.value_type_name = syn::Ident::new(value, binding.value_type_name.span());
    }

    #[cfg(test)]
    pub(crate) fn test_only_mismatch_binding_origin(&mut self, name: &str) {
        let binding = self
            .terminals
            .iter_mut()
            .find_map(|terminal| match terminal {
                TerminalPlan::Binding(binding) if binding.name() == name => Some(binding),
                TerminalPlan::Vocab(_)
                | TerminalPlan::Lexeme(_)
                | TerminalPlan::Binding(_)
                | TerminalPlan::ContextIdentity(_)
                | TerminalPlan::CatalogIdentity(_)
                | TerminalPlan::SignedDecimal(_)
                | TerminalPlan::UnsignedNumber(_)
                | TerminalPlan::DeclarationDeterminative(_)
                | TerminalPlan::DeclarationNoun(_)
                | TerminalPlan::DeclarationTerm(_) => None,
            })
            .expect("test binding is present");
        binding.origin = DeclarationKey::new(SourceDeclarationKind::Identity, binding.name());
    }

    #[cfg(test)]
    pub(crate) fn test_only_remove_runtime_vocab(&mut self, name: &str) {
        let index = self
            .terminals
            .iter()
            .position(
                |terminal| matches!(terminal, TerminalPlan::Vocab(vocab) if vocab.name() == name),
            )
            .expect("test runtime vocabulary is present");
        self.runtime
            .vocab_indices
            .retain(|candidate| *candidate != index);
    }

    #[cfg(test)]
    pub(crate) fn test_only_seal_construction_atoms(
        form: &Form,
        atoms: &[AtomContribution],
    ) -> syn::Result<Vec<AtomPlan>> {
        seal_atoms(form, atoms)
    }

    pub(crate) fn parse_root(&self, category: &str) -> Option<&RootPlan> {
        self.roots
            .iter()
            .find(|root| root.parse_entry && root.category == category)
    }

    pub(crate) fn atom_terminal(&self, name: &str) -> syn::Result<AtomTerminal<'_>> {
        for (terminal_index, terminal) in self.terminals.iter().enumerate() {
            match terminal {
                TerminalPlan::Vocab(row) if row.name() == name => {
                    return Ok(AtomTerminal::Vocab(row));
                }
                TerminalPlan::Lexeme(row) if row.name() == name => {
                    return Ok(AtomTerminal::Lexeme(row));
                }
                TerminalPlan::Binding(row) if row.name() == name => {
                    if let Some(plan) = row.declaration_verb() {
                        return Ok(AtomTerminal::DeclarationVerb {
                            terminal_index,
                            plan,
                        });
                    }
                    return Ok(AtomTerminal::Binding(row));
                }
                TerminalPlan::ContextIdentity(row) if row.name() == name => {
                    return Ok(AtomTerminal::ContextIdentity(row));
                }
                TerminalPlan::CatalogIdentity(row) if row.name() == name => {
                    return Ok(AtomTerminal::CatalogIdentity {
                        terminal_index,
                        plan: row,
                    });
                }
                TerminalPlan::SignedDecimal(row) if row.codec_name() == name => {
                    return Ok(AtomTerminal::SignedDecimal(row));
                }
                TerminalPlan::UnsignedNumber(row) if row.codec_name() == name => {
                    return Ok(AtomTerminal::UnsignedNumber(row));
                }
                TerminalPlan::DeclarationNoun(row) if row.codec_name() == name => {
                    return Ok(AtomTerminal::DeclarationNoun {
                        terminal_index,
                        plan: row,
                    });
                }
                TerminalPlan::DeclarationDeterminative(row) if row.codec_name() == name => {
                    return Ok(AtomTerminal::DeclarationDeterminative {
                        terminal_index,
                        plan: row,
                    });
                }
                TerminalPlan::DeclarationTerm(row) if row.codec_name() == name => {
                    return Ok(AtomTerminal::DeclarationTerm {
                        terminal_index,
                        plan: row,
                    });
                }
                TerminalPlan::Vocab(_)
                | TerminalPlan::Lexeme(_)
                | TerminalPlan::Binding(_)
                | TerminalPlan::ContextIdentity(_)
                | TerminalPlan::CatalogIdentity(_)
                | TerminalPlan::SignedDecimal(_)
                | TerminalPlan::UnsignedNumber(_)
                | TerminalPlan::DeclarationDeterminative(_)
                | TerminalPlan::DeclarationNoun(_)
                | TerminalPlan::DeclarationTerm(_) => {}
            }
        }
        Err(sealed_error("resolved atom terminal"))
    }

    #[cfg(test)]
    pub(crate) fn snapshot(&self) -> SemanticSnapshot {
        let declaration_keys = self
            .declaration_keys
            .iter()
            .map(|key| (key.kind(), key.name().to_owned()))
            .collect();
        let constructions = self
            .constructions
            .iter()
            .map(|plan| {
                (
                    plan.construction_id.clone(),
                    plan.element_type.clone(),
                    plan.forms[0].name.clone(),
                    plan.forms[0].atoms.iter().map(AtomPlan::snapshot).collect(),
                )
            })
            .collect();
        let terminals = self.terminals.iter().map(TerminalPlan::snapshot).collect();
        let roots = self
            .roots
            .iter()
            .map(|root| (root.category.clone(), root.parse_entry, root.render_entry))
            .collect();
        let feature_equations = self
            .constructions
            .iter()
            .filter_map(|construction| {
                let equations = self.feature_equations(&construction.construction_id);
                (!equations.is_empty()).then(|| {
                    (
                        construction.construction_id.clone(),
                        equations
                            .iter()
                            .map(feature::FeatureEquation::snapshot)
                            .collect::<Vec<_>>()
                            .join("; "),
                    )
                })
            })
            .collect();
        let mut boxed_fields = self.boxed_fields().iter().cloned().collect::<Vec<_>>();
        boxed_fields.sort();
        let mut dynamic_number_constructions = self
            .dynamic_number_constructions()
            .iter()
            .cloned()
            .collect::<Vec<_>>();
        dynamic_number_constructions.sort();
        SemanticSnapshot {
            declaration_keys,
            constructions,
            terminals,
            roots,
            feature_equations,
            boxed_fields,
            dynamic_number_constructions,
        }
    }

    #[cfg(test)]
    pub(crate) fn feature_resolutions_snapshot(&self) -> Vec<(String, Vec<(String, String)>)> {
        self.constructions
            .iter()
            .map(|construction| {
                let mut resolutions = self
                    .features
                    .resolutions
                    .get(&construction.construction_id)
                    .into_iter()
                    .flat_map(|resolutions| resolutions.iter())
                    .map(|(place, resolution)| (place.snapshot(), resolution.snapshot()))
                    .collect::<Vec<_>>();
                resolutions.sort();
                (construction.construction_id.clone(), resolutions)
            })
            .collect()
    }

    #[cfg(test)]
    pub(crate) fn category_render_capabilities_snapshot(&self) -> Vec<(String, bool, bool, bool)> {
        let mut capabilities = self
            .features
            .category_render
            .iter()
            .map(|(category, capability)| {
                (
                    category.clone(),
                    capability.carries_concord_class(),
                    capability.requires_external_concord_class(),
                    capability.requires_context(),
                )
            })
            .collect::<Vec<_>>();
        capabilities.sort();
        capabilities
    }
}

fn seal_invariant_category_feature_reads(
    constructions: &[ConstructionPlan],
    category_reads: &mut HashMap<String, HashSet<Feature>>,
) {
    for construction in constructions {
        for (category, feature) in construction.invariant().category_feature_reads() {
            category_reads
                .entry(category.clone())
                .or_default()
                .insert(*feature);
        }
    }
}

fn seal_form_guard_category_feature_reads(
    constructions: &[ConstructionPlan],
    equations: &HashMap<String, Vec<feature::FeatureEquation>>,
    category_reads: &mut HashMap<String, HashSet<Feature>>,
) -> syn::Result<()> {
    for construction in constructions {
        for domain in construction
            .forms()
            .iter()
            .filter_map(|form| form.guard().predicate())
            .flat_map(FinitePredicatePlan::domains)
        {
            let FiniteDomainKindPlan::Feature { feature, .. } = domain.kind() else {
                continue;
            };
            if domain.role().starts_with('@') {
                collect_form_guard_category_read(
                    construction,
                    equations
                        .get(construction.construction_id())
                        .map_or(&[], Vec::as_slice),
                    &FeaturePlace::Construction(*feature),
                    category_reads,
                    &mut HashSet::new(),
                )?;
            } else {
                let field = construction.field(domain.role())?;
                if field.kind() == ConstructionFieldKind::Category {
                    category_reads
                        .entry(field.terminal().to_owned())
                        .or_default()
                        .insert(*feature);
                }
            }
        }
    }
    Ok(())
}

fn checked_field_category_feature_reads(
    constructions: &[ConstructionPlan],
) -> syn::Result<HashMap<String, HashSet<Feature>>> {
    let mut reads = HashMap::<String, HashSet<Feature>>::new();
    for construction in constructions {
        for field in construction.fields() {
            let Some((_, arguments)) = field.field_check() else {
                continue;
            };
            for argument in arguments {
                if let FieldCheckArgumentPlan::Feature { role, feature } = argument {
                    let source = construction.field(role)?;
                    if source.kind() == ConstructionFieldKind::Category {
                        reads
                            .entry(source.terminal().to_owned())
                            .or_default()
                            .insert(*feature);
                    }
                }
            }
        }
    }
    Ok(reads)
}

fn collect_form_guard_category_read(
    construction: &ConstructionPlan,
    equations: &[feature::FeatureEquation],
    place: &FeaturePlace,
    category_reads: &mut HashMap<String, HashSet<Feature>>,
    visiting: &mut HashSet<FeaturePlace>,
) -> syn::Result<()> {
    if !visiting.insert(place.clone()) {
        return Err(sealed_error("form guard feature dependency cycle"));
    }
    match equations
        .iter()
        .find(|equation| equation.target() == place)
        .map(feature::FeatureEquation::value)
    {
        Some(FeatureExpr::FromRole { role, feature }) => {
            collect_form_guard_category_read(
                construction,
                equations,
                &FeaturePlace::Role {
                    field: role.clone(),
                    feature: *feature,
                },
                category_reads,
                visiting,
            )?;
        }
        Some(FeatureExpr::Constant(_) | FeatureExpr::MatchVocab { .. }) => {}
        None => {
            let FeaturePlace::Role { field, feature } = place else {
                return Err(sealed_error("form guard construction feature dependency"));
            };
            let field = construction.field(&identifier_key(field))?;
            if field.kind() == ConstructionFieldKind::Category {
                category_reads
                    .entry(field.terminal().to_owned())
                    .or_default()
                    .insert(*feature);
            }
        }
    }
    visiting.remove(place);
    Ok(())
}

#[derive(Default)]
struct InvariantFeatureDependencies {
    fields: HashSet<String>,
    category_reads: HashSet<(String, Feature)>,
}

fn validate_generated_associated_names(
    constructions: &[ConstructionPlan],
    products: &[ProductPlan],
) -> syn::Result<()> {
    let mut errors = None;
    for construction in constructions {
        if !construction.requires_constructor() {
            continue;
        }
        validate_constructor_associated_names(
            "construction",
            construction.element_type(),
            construction
                .fields()
                .iter()
                .filter(|field| field.emits_generated_accessor())
                .map(|field| (field.name_key(), field.name().span())),
            &mut errors,
        );
    }
    for product in products
        .iter()
        .filter(|product| product.requires_constructor())
    {
        validate_constructor_associated_names(
            "structural",
            product.name(),
            product
                .fields()
                .iter()
                .filter(|field| field.emits_generated_accessor())
                .map(|field| (field.name().to_owned(), field.span())),
            &mut errors,
        );
    }
    errors.map_or(Ok(()), Err)
}

fn validate_constructor_associated_names(
    surface: &str,
    owner: &str,
    accessors: impl IntoIterator<Item = (String, Span)>,
    errors: &mut Option<syn::Error>,
) {
    for (name, span) in accessors {
        if !matches!(name.as_str(), INVARIANT_CONSTRUCTOR | CHECKED_CONSTRUCTOR) {
            continue;
        }
        combine_errors(
            errors,
            syn::Error::new(
                span,
                format!(
                    "generated {surface} associated item `{name}` for `{owner}` collides with generated {surface} accessor `{owner}.{name}`"
                ),
            ),
        );
    }
}

fn combine_errors(errors: &mut Option<syn::Error>, error: syn::Error) {
    if let Some(errors) = errors {
        errors.combine(error);
    } else {
        *errors = Some(error);
    }
}

fn seal_invariant_field_policy(
    constructions: &mut [ConstructionPlan],
    equations: &HashMap<String, Vec<feature::FeatureEquation>>,
    resolutions: &HashMap<String, HashMap<feature::FeaturePlace, feature::FeatureResolution>>,
    category_render: &HashMap<String, crate::validate::CategoryRenderCapability>,
    concord_class_carry_sums: &HashSet<String>,
    terminals: &[TerminalPlan],
) -> syn::Result<()> {
    for construction in constructions {
        let construction_equations = equations
            .get(construction.construction_id())
            .map_or(&[][..], Vec::as_slice);
        let construction_resolutions = resolutions.get(construction.construction_id());
        let dependencies = invariant_feature_dependencies(
            construction,
            construction_equations,
            construction_resolutions,
            category_render,
            concord_class_carry_sums,
        )?;
        construction.invariant.seal_field_policy(
            &mut construction.fields,
            terminals,
            dependencies,
        )?;
    }
    Ok(())
}

fn invariant_feature_dependencies(
    construction: &ConstructionPlan,
    equations: &[feature::FeatureEquation],
    resolutions: Option<&HashMap<feature::FeaturePlace, feature::FeatureResolution>>,
    category_render: &HashMap<String, crate::validate::CategoryRenderCapability>,
    concord_class_carry_sums: &HashSet<String>,
) -> syn::Result<InvariantFeatureDependencies> {
    let mut dependencies = InvariantFeatureDependencies::default();
    for subject in construction
        .invariant()
        .alternatives()
        .iter()
        .flat_map(PredicateConjunctionPlan::atoms)
        .map(PredicateAtomPlan::subject)
    {
        let place = match subject {
            PredicateSubjectPlan::RoleFeature { role, feature, .. } => {
                feature::FeaturePlace::Role {
                    field: role.clone(),
                    feature: *feature,
                }
            }
            PredicateSubjectPlan::ConstructionFeature(feature) => {
                feature::FeaturePlace::Construction(*feature)
            }
            PredicateSubjectPlan::CategoryRole { .. }
            | PredicateSubjectPlan::VocabRole { .. }
            | PredicateSubjectPlan::OptionalPresenceRole { .. } => continue,
        };
        collect_invariant_feature_dependencies(
            construction,
            equations,
            resolutions,
            &place,
            &mut HashSet::new(),
            &mut dependencies,
        )?;
    }
    for equation in equations {
        let feature::FeaturePlace::Role { field, feature } = equation.target() else {
            continue;
        };
        let Ok(constrained) = construction.field(&identifier_key(field)) else {
            continue;
        };
        if constrained.kind() != ConstructionFieldKind::Category {
            continue;
        }
        let carries_feature = match feature {
            Feature::ConcordClass => {
                concord_class_carry_sums.contains(constrained.terminal())
                    || category_render
                        .get(constrained.terminal())
                        .is_some_and(|capability| capability.carries_concord_class())
            }
            Feature::Number => true,
            _ => false,
        };
        if !carries_feature {
            continue;
        }
        dependencies.fields.insert(constrained.name_key());
        if *feature == Feature::Number {
            dependencies
                .category_reads
                .insert((constrained.terminal().to_owned(), Feature::Number));
        }
        collect_invariant_feature_dependencies(
            construction,
            equations,
            resolutions,
            equation.target(),
            &mut HashSet::new(),
            &mut dependencies,
        )?;
    }
    Ok(dependencies)
}

fn collect_invariant_feature_dependencies(
    construction: &ConstructionPlan,
    equations: &[feature::FeatureEquation],
    resolutions: Option<&HashMap<feature::FeaturePlace, feature::FeatureResolution>>,
    place: &feature::FeaturePlace,
    visiting: &mut HashSet<feature::FeaturePlace>,
    dependencies: &mut InvariantFeatureDependencies,
) -> syn::Result<()> {
    if resolutions
        .and_then(|rows| rows.get(place))
        .is_some_and(|resolution| matches!(resolution, feature::FeatureResolution::Known(_)))
    {
        return Ok(());
    }
    if !visiting.insert(place.clone()) {
        return Err(sealed_error("invariant feature dependency cycle"));
    }
    let equation = equations.iter().find(|equation| equation.target() == place);
    match equation.map(feature::FeatureEquation::value) {
        Some(FeatureExpr::Constant(_)) => {}
        Some(FeatureExpr::MatchVocab { role, .. }) => {
            let field = construction.field(&identifier_key(role))?;
            if field.kind() != ConstructionFieldKind::Lex {
                return Err(sealed_error("invariant MatchVocab dependency field kind"));
            }
            dependencies.fields.insert(field.name_key());
        }
        Some(FeatureExpr::FromRole { role, feature }) => {
            collect_invariant_feature_dependencies(
                construction,
                equations,
                resolutions,
                &feature::FeaturePlace::Role {
                    field: role.clone(),
                    feature: *feature,
                },
                visiting,
                dependencies,
            )?;
        }
        None => {
            let feature::FeaturePlace::Role { field, feature } = place else {
                return Err(sealed_error("invariant construction feature dependency"));
            };
            let field = construction.field(&identifier_key(field))?;
            if field.kind() == ConstructionFieldKind::Lex {
                dependencies.fields.insert(field.name_key());
                visiting.remove(place);
                return Ok(());
            }
            if field.kind() != ConstructionFieldKind::Category {
                return Err(sealed_error("invariant role feature dependency"));
            }
            dependencies.fields.insert(field.name_key());
            dependencies
                .category_reads
                .insert((field.terminal().to_owned(), *feature));
        }
    }
    visiting.remove(place);
    Ok(())
}

impl ConstructionPlan {
    #[expect(
        clippy::too_many_arguments,
        reason = "construction sealing consumes the already-separated validated fact inventories"
    )]
    fn from_source(
        source_index: usize,
        source: &crate::Construction,
        resolved_atoms: &[AtomContribution],
        invariant: InvariantPlan,
        nullable: bool,
        structural_fields: &mut HashMap<(String, String), StructuralFieldPlan>,
        terminals: &[TerminalPlan],
        nullable_types: &HashSet<String>,
    ) -> syn::Result<Self> {
        let construction_id = identifier_key(&source.name);
        let category = path_key(&source.category);
        let category_variant = pascal_case(&construction_id);
        let element_type = identifier_key(&source.element.name);
        let rule_id = format!("{}{category_variant}", pascal_case(&category));
        let fields = source
            .element
            .fields
            .iter()
            .map(|field| {
                let (kind, terminal, value_type, zeroable) =
                    if let crate::model::FieldKind::Zeroable {
                        value_type, item, ..
                    } = &field.kind
                    {
                        (
                            ConstructionFieldKind::Category,
                            match item.as_ref() {
                                crate::model::FieldKind::Category(path) => path_key(path),
                                _ => unreachable!("zeroable field items are categories"),
                            },
                            value_type.clone(),
                            true,
                        )
                    } else {
                        let leaf = structural_field_source_leaf(&field.kind);
                        let (kind, terminal, value_type) = match leaf {
                            crate::model::FieldKind::Category(path) => (
                                ConstructionFieldKind::Category,
                                path_key(path),
                                path.clone(),
                            ),
                            crate::model::FieldKind::Lex(path) => {
                                (ConstructionFieldKind::Lex, path_key(path), path.clone())
                            }
                            crate::model::FieldKind::Identity(path) => (
                                ConstructionFieldKind::Identity,
                                path_key(path),
                                path.clone(),
                            ),
                            crate::model::FieldKind::Optional(_)
                            | crate::model::FieldKind::Zeroable { .. }
                            | crate::model::FieldKind::Sequence { .. } => unreachable!(
                                "the parser rejects nested structural cardinality before semantic lowering"
                            ),
                        };
                        (kind, terminal, value_type, false)
                    };
                let role = identifier_key(&field.name);
                Ok(ConstructionFieldPlan {
                    name: field.name.clone(),
                    kind,
                    terminal,
                    value_type,
                    structural: structural_fields.remove(&(construction_id.clone(), role)),
                    invariant_bearing: false,
                    accessor_mode: None,
                    zeroable,
                    mobile: field.mobile,
                    mobile_scope_sibling: field
                        .mobile_scope_sibling
                        .as_ref()
                        .map(identifier_key),
                    field_check: field.check.as_ref().map(|check| {
                        (
                            check.function.clone(),
                            check
                                .arguments
                                .iter()
                                .map(|argument| match argument {
                                    crate::model::FieldCheckArgument::Feature(argument) => {
                                        FieldCheckArgumentPlan::Feature {
                                            role: identifier_key(&argument.role),
                                            feature: Feature::from(argument.feature),
                                        }
                                    }
                                    crate::model::FieldCheckArgument::VerbFrameRolePrepositions {
                                        role,
                                    } => FieldCheckArgumentPlan::VerbFrameRolePrepositions {
                                        role: identifier_key(role),
                                    },
                                })
                                .collect(),
                        )
                    }),
                })
            })
            .collect::<syn::Result<Vec<_>>>()?;
        let forms = seal_forms(
            source,
            &fields,
            resolved_atoms,
            &rule_id,
            terminals,
            nullable_types,
        )?;
        if nullable != forms.iter().any(FormPlan::is_nullable) {
            return Err(syn::Error::new(
                source.name.span(),
                "sealed construction form nullability is inconsistent",
            ));
        }
        Ok(Self {
            source_index,
            origin_span: source.name.span(),
            category_span: source
                .category
                .segments
                .last()
                .map_or_else(Span::call_site, |segment| segment.ident.span()),
            construction_id,
            category,
            category_variant,
            element_type: element_type.clone(),
            rule_id: rule_id.clone(),
            render_arm: element_type.clone(),
            visitor_method: format!("visit_{}", snake_case(&element_type)),
            walker: format!("walk_{}", snake_case(&element_type)),
            nullable,
            fields,
            invariant,
            forms,
        })
    }

    #[allow(
        dead_code,
        reason = "future emitters retain source order through this index"
    )]
    pub(crate) fn source_index(&self) -> usize {
        self.source_index
    }

    pub(crate) fn construction_id(&self) -> &str {
        &self.construction_id
    }

    pub(crate) fn category(&self) -> &str {
        &self.category
    }

    #[cfg(test)]
    pub(crate) fn form(&self) -> &str {
        self.forms
            .first()
            .expect("sealed construction has at least one form")
            .name()
    }

    pub(crate) fn category_variant(&self) -> &str {
        &self.category_variant
    }

    pub(crate) fn element_type(&self) -> &str {
        &self.element_type
    }

    pub(crate) fn rule_id(&self) -> &str {
        &self.rule_id
    }

    pub(crate) fn forms(&self) -> &[FormPlan] {
        &self.forms
    }

    pub(crate) fn origin_span(&self) -> Span {
        self.origin_span
    }

    pub(crate) fn category_span(&self) -> Span {
        self.category_span
    }

    pub(crate) fn fields(&self) -> &[ConstructionFieldPlan] {
        &self.fields
    }

    pub(crate) fn has_mobile_role(&self) -> bool {
        self.fields.iter().any(ConstructionFieldPlan::is_mobile)
    }

    pub(crate) fn requires_constructor(&self) -> bool {
        self.invariant.requires_constructor()
            || self
                .fields
                .iter()
                .any(ConstructionFieldPlan::emits_generated_accessor)
            || self.has_mobile_role()
            || self.fields.iter().any(|field| {
                matches!(
                    field.structural_kind(),
                    Some(StructuralFieldKindPlan::Sequence { bounds, .. })
                        if bounds.min() != 0 || bounds.max().is_some()
                )
            })
    }

    #[allow(
        dead_code,
        reason = "Tasks 3 through 5 consume construction nullability"
    )]
    pub(crate) const fn is_nullable(&self) -> bool {
        self.nullable
    }

    pub(crate) fn field(&self, role: &str) -> syn::Result<&ConstructionFieldPlan> {
        self.fields
            .iter()
            .find(|field| field.name_key() == role)
            .ok_or_else(|| syn::Error::new(self.origin_span, "sealed construction field is absent"))
    }

    #[allow(dead_code, reason = "later invariant emitters consume the sealed plan")]
    pub(crate) fn invariant(&self) -> &InvariantPlan {
        &self.invariant
    }
}

impl FormPlan {
    pub(crate) const fn origin_span(&self) -> Span {
        self.origin_span
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn rule_id(&self) -> &str {
        &self.rule_id
    }

    pub(crate) fn guard(&self) -> &FormGuardPlan {
        &self.guard
    }

    pub(crate) fn atoms(&self) -> &[AtomPlan] {
        &self.atoms
    }

    pub(crate) fn literal_is_licensed(&self, atom_index: usize) -> bool {
        self.licensed_literal_indexes.contains(&atom_index)
    }

    pub(crate) const fn is_nullable(&self) -> bool {
        self.nullable
    }
}

impl FormGuardPlan {
    pub(crate) fn guarded_form_indexes(&self) -> Option<&[usize]> {
        match self {
            Self::Otherwise {
                guarded_form_indexes,
            } => Some(guarded_form_indexes),
            Self::Unguarded | Self::Predicate(_) => None,
        }
    }

    pub(crate) fn predicate(&self) -> Option<&FinitePredicatePlan> {
        match self {
            Self::Predicate(predicate) => Some(predicate),
            Self::Unguarded | Self::Otherwise { .. } => None,
        }
    }

    #[cfg(test)]
    fn test_accepting_witnesses(&self, forms: &[FormPlan]) -> Vec<String> {
        let domains = combined_form_domains(forms);
        enumerate_assignments(&domains)
            .into_iter()
            .filter(|assignment| form_guard_accepts(self, forms, &domains, assignment))
            .map(|assignment| assignment_witness(&domains, &assignment))
            .collect()
    }
}

impl FinitePredicatePlan {
    pub(crate) fn domains(&self) -> &[FiniteDomainPlan] {
        &self.domains
    }

    pub(crate) fn accepting(&self) -> &[FiniteAssignmentPlan] {
        &self.accepting
    }

    fn accepts(&self, domains: &[FiniteDomainPlan], assignment: &FiniteAssignmentPlan) -> bool {
        self.accepting.iter().any(|accepted| {
            self.domains
                .iter()
                .zip(&accepted.values)
                .all(|(domain, value)| {
                    domains
                        .iter()
                        .position(|candidate| candidate == domain)
                        .is_some_and(|index| assignment.values.get(index) == Some(value))
                })
        })
    }
}

impl FiniteDomainPlan {
    pub(crate) fn role(&self) -> &str {
        &self.role
    }

    pub(crate) fn kind(&self) -> &FiniteDomainKindPlan {
        &self.kind
    }
}

impl FiniteAssignmentPlan {
    pub(crate) fn values(&self) -> &[FiniteValuePlan] {
        &self.values
    }
}

fn seal_forms(
    construction: &crate::Construction,
    fields: &[ConstructionFieldPlan],
    resolved_atoms: &[AtomContribution],
    base_rule_id: &str,
    terminals: &[TerminalPlan],
    nullable_types: &HashSet<String>,
) -> syn::Result<Vec<FormPlan>> {
    let mut atom_offset = 0;
    let mut guarded_indexes = Vec::new();
    let mut forms = Vec::with_capacity(construction.forms.len());
    for (form_index, source) in construction.forms.iter().enumerate() {
        let atom_end = atom_offset + source.atoms.len();
        let atoms = seal_atoms(
            source,
            resolved_atoms.get(atom_offset..atom_end).ok_or_else(|| {
                syn::Error::new(
                    source.name.span(),
                    "sealed construction atom inventory is truncated",
                )
            })?,
        )?;
        atom_offset = atom_end;
        let guard = match &source.guard {
            crate::model::FormGuardSource::Unguarded => FormGuardPlan::Unguarded,
            crate::model::FormGuardSource::When(predicate) => {
                guarded_indexes.push(form_index);
                FormGuardPlan::Predicate(seal_finite_predicate(
                    predicate,
                    fields,
                    terminals,
                    source.name.span(),
                )?)
            }
            crate::model::FormGuardSource::Otherwise => FormGuardPlan::Otherwise {
                guarded_form_indexes: guarded_indexes.clone(),
            },
        };
        let name = identifier_key(&source.name);
        let rule_id = if construction.forms.len() == 1 {
            base_rule_id.to_owned()
        } else {
            format!("{base_rule_id}{}", pascal_case(&name))
        };
        let nullable = atoms
            .iter()
            .all(|atom| form_atom_is_nullable(atom, fields, nullable_types));
        let licensed_literal_indexes = source
            .atoms
            .iter()
            .enumerate()
            .filter_map(|(index, atom)| {
                matches!(atom, FormAtom::LicensedLiteral(_)).then_some(index)
            })
            .collect();
        forms.push(FormPlan {
            origin_span: source.name.span(),
            name,
            rule_id,
            guard,
            atoms,
            licensed_literal_indexes,
            nullable,
        });
    }
    if atom_offset != resolved_atoms.len() {
        return Err(syn::Error::new(
            construction.name.span(),
            "sealed construction atom inventory has surplus rows",
        ));
    }
    reject_guard_overlaps(&forms, fields)?;
    reject_unreachable_fallback(&forms, fields, construction)?;
    Ok(forms)
}

fn form_atom_is_nullable(
    atom: &AtomPlan,
    fields: &[ConstructionFieldPlan],
    nullable_types: &HashSet<String>,
) -> bool {
    match atom {
        AtomPlan::Literal(value)
        | AtomPlan::SentenceInitialLiteral(value)
        | AtomPlan::StructuralLiteral(value) => value.is_empty(),
        AtomPlan::Category { role, category } => fields
            .iter()
            .find(|field| field.name_key() == *role)
            .and_then(ConstructionFieldPlan::structural_kind)
            .map_or_else(
                || nullable_types.contains(category),
                |kind| structural_kind_is_nullable(kind, nullable_types),
            ),
        AtomPlan::Lex { role, .. }
        | AtomPlan::Marked { role, .. }
        | AtomPlan::Identity { role, .. }
        | AtomPlan::Noun { role, .. } => fields
            .iter()
            .find(|field| field.name_key() == *role)
            .and_then(ConstructionFieldPlan::structural_kind)
            .is_some_and(|kind| structural_kind_is_nullable(kind, nullable_types)),
        AtomPlan::LexFixed { .. }
        | AtomPlan::VerbFixed { .. }
        | AtomPlan::OpenDeclaration(_)
        | AtomPlan::Bound { .. }
        | AtomPlan::Circumfix { .. } => false,
    }
}

fn structural_kind_is_nullable(
    kind: &StructuralFieldKindPlan,
    nullable_types: &HashSet<String>,
) -> bool {
    match kind {
        StructuralFieldKindPlan::Zeroable(_) | StructuralFieldKindPlan::Optional(_) => true,
        StructuralFieldKindPlan::Sequence { bounds, .. } => bounds.min() == 0,
        StructuralFieldKindPlan::Required(value) => match value {
            ValueKindPlan::Category(name)
            | ValueKindPlan::Product(name)
            | ValueKindPlan::Sum(name) => nullable_types.contains(name),
            ValueKindPlan::Lex(_) | ValueKindPlan::Identity(_) => false,
        },
    }
}

fn seal_finite_predicate(
    source: &crate::model::RequireExprSource,
    fields: &[ConstructionFieldPlan],
    terminals: &[TerminalPlan],
    form_span: Span,
) -> syn::Result<FinitePredicatePlan> {
    let referenced = guard_referenced_roles(source);
    let mut domains = fields
        .iter()
        .filter(|field| referenced.contains(&field.name_key()))
        .map(|field| finite_domain(field, source, terminals))
        .collect::<syn::Result<Vec<_>>>()?;
    domains.extend(
        guard_referenced_construction_features(source)
            .into_iter()
            .map(|feature| FiniteDomainPlan {
                role: construction_feature_domain_key(feature),
                kind: FiniteDomainKindPlan::Feature {
                    feature,
                    values: feature.domain().to_vec(),
                },
            }),
    );
    let accepting = enumerate_assignments(&domains)
        .into_iter()
        .filter(|assignment| evaluate_guard(source, &domains, assignment))
        .collect::<Vec<_>>();
    if accepting.is_empty() {
        return Err(syn::Error::new(form_span, "form guard is unsatisfiable"));
    }
    Ok(FinitePredicatePlan { domains, accepting })
}

fn construction_feature_domain_key(feature: Feature) -> String {
    format!("@{}", feature.key())
}

fn guard_referenced_construction_features(
    source: &crate::model::RequireExprSource,
) -> Vec<Feature> {
    fn collect(source: &crate::model::RequireExprSource, features: &mut Vec<Feature>) {
        match source {
            crate::model::RequireExprSource::In {
                subject: crate::model::RequireSubjectSource::ConstructionFeature(feature),
                ..
            } => {
                let feature = Feature::from(*feature);
                if !features.contains(&feature) {
                    features.push(feature);
                }
            }
            crate::model::RequireExprSource::All(operands)
            | crate::model::RequireExprSource::Any(operands) => {
                for operand in operands {
                    collect(operand, features);
                }
            }
            crate::model::RequireExprSource::OptionalPresence { .. }
            | crate::model::RequireExprSource::In { .. }
            | crate::model::RequireExprSource::Length { .. } => {}
        }
    }
    let mut features = Vec::new();
    collect(source, &mut features);
    features
}

fn guard_referenced_roles(source: &crate::model::RequireExprSource) -> HashSet<String> {
    fn collect(source: &crate::model::RequireExprSource, roles: &mut HashSet<String>) {
        match source {
            crate::model::RequireExprSource::OptionalPresence { role, .. }
            | crate::model::RequireExprSource::In {
                subject: crate::model::RequireSubjectSource::Role(role),
                ..
            }
            | crate::model::RequireExprSource::In {
                subject: crate::model::RequireSubjectSource::RoleFeature { role, .. },
                ..
            } => {
                roles.insert(identifier_key(role));
            }
            crate::model::RequireExprSource::All(operands)
            | crate::model::RequireExprSource::Any(operands) => {
                for operand in operands {
                    collect(operand, roles);
                }
            }
            crate::model::RequireExprSource::In { .. }
            | crate::model::RequireExprSource::Length { .. } => {}
        }
    }
    let mut roles = HashSet::new();
    collect(source, &mut roles);
    roles
}

fn finite_domain(
    field: &ConstructionFieldPlan,
    predicate: &crate::model::RequireExprSource,
    terminals: &[TerminalPlan],
) -> syn::Result<FiniteDomainPlan> {
    let role = field.name_key();
    let optional = predicate_uses_optional_presence(predicate, &role);
    let membership = predicate_uses_membership(predicate, &role);
    let feature = predicate_feature(predicate, &role);
    let structurally_optional = matches!(
        field.structural_kind(),
        Some(StructuralFieldKindPlan::Zeroable(_) | StructuralFieldKindPlan::Optional(_))
    );
    let vocab = terminals.iter().find_map(|terminal| match terminal {
        TerminalPlan::Vocab(vocab) if vocab.name() == field.terminal() => Some(vocab),
        _ => None,
    });
    let kind = if let Some(feature) = feature {
        let feature = Feature::from(feature);
        FiniteDomainKindPlan::Feature {
            feature,
            values: feature.domain().to_vec(),
        }
    } else if structurally_optional
        && (membership || optional)
        && let Some(vocab) = vocab
    {
        let variants = vocab
            .variants()
            .iter()
            .map(|variant| identifier_key(variant.name()))
            .collect::<Vec<_>>();
        if membership {
            validate_predicate_members(predicate, &role, &variants)?;
        }
        FiniteDomainKindPlan::OptionalVocab {
            terminal: field.terminal().to_owned(),
            variants,
        }
    } else if membership {
        let vocab = vocab.ok_or_else(|| {
            syn::Error::new(
                field.name().span(),
                "form guard membership requires a vocab role",
            )
        })?;
        let variants = vocab
            .variants()
            .iter()
            .map(|variant| identifier_key(variant.name()))
            .collect::<Vec<_>>();
        validate_predicate_members(predicate, &role, &variants)?;
        FiniteDomainKindPlan::Vocab {
            terminal: field.terminal().to_owned(),
            variants,
        }
    } else if optional {
        FiniteDomainKindPlan::OptionalPresence
    } else {
        return Err(syn::Error::new(
            field.name().span(),
            "form guard role has no finite predicate",
        ));
    };
    Ok(FiniteDomainPlan { role, kind })
}

fn predicate_feature(
    source: &crate::model::RequireExprSource,
    role: &str,
) -> Option<crate::model::Feature> {
    match source {
        crate::model::RequireExprSource::In {
            subject:
                crate::model::RequireSubjectSource::RoleFeature {
                    role: candidate,
                    feature,
                },
            ..
        } if identifier_key(candidate) == role => Some(*feature),
        crate::model::RequireExprSource::All(operands)
        | crate::model::RequireExprSource::Any(operands) => operands
            .iter()
            .find_map(|operand| predicate_feature(operand, role)),
        crate::model::RequireExprSource::OptionalPresence { .. }
        | crate::model::RequireExprSource::In { .. }
        | crate::model::RequireExprSource::Length { .. } => None,
    }
}

fn predicate_uses_optional_presence(source: &crate::model::RequireExprSource, role: &str) -> bool {
    match source {
        crate::model::RequireExprSource::OptionalPresence {
            role: candidate, ..
        } => identifier_key(candidate) == role,
        crate::model::RequireExprSource::All(operands)
        | crate::model::RequireExprSource::Any(operands) => operands
            .iter()
            .any(|operand| predicate_uses_optional_presence(operand, role)),
        crate::model::RequireExprSource::In { .. }
        | crate::model::RequireExprSource::Length { .. } => false,
    }
}

fn predicate_uses_membership(source: &crate::model::RequireExprSource, role: &str) -> bool {
    match source {
        crate::model::RequireExprSource::In {
            subject: crate::model::RequireSubjectSource::Role(candidate),
            ..
        } => identifier_key(candidate) == role,
        crate::model::RequireExprSource::All(operands)
        | crate::model::RequireExprSource::Any(operands) => operands
            .iter()
            .any(|operand| predicate_uses_membership(operand, role)),
        crate::model::RequireExprSource::OptionalPresence { .. }
        | crate::model::RequireExprSource::In { .. }
        | crate::model::RequireExprSource::Length { .. } => false,
    }
}

fn validate_predicate_members(
    source: &crate::model::RequireExprSource,
    role: &str,
    variants: &[String],
) -> syn::Result<()> {
    match source {
        crate::model::RequireExprSource::In {
            subject: crate::model::RequireSubjectSource::Role(candidate),
            members,
        } if identifier_key(candidate) == role => {
            for member in members {
                let name = identifier_key(member);
                if !variants.contains(&name) {
                    return Err(syn::Error::new(
                        member.span(),
                        format!("unknown member `{name}` in form guard"),
                    ));
                }
            }
        }
        crate::model::RequireExprSource::All(operands)
        | crate::model::RequireExprSource::Any(operands) => {
            for operand in operands {
                validate_predicate_members(operand, role, variants)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn enumerate_assignments(domains: &[FiniteDomainPlan]) -> Vec<FiniteAssignmentPlan> {
    let mut assignments = vec![FiniteAssignmentPlan { values: Vec::new() }];
    for domain in domains {
        let values = match &domain.kind {
            FiniteDomainKindPlan::Vocab { variants, .. } => variants
                .iter()
                .cloned()
                .map(FiniteValuePlan::Vocab)
                .collect::<Vec<_>>(),
            FiniteDomainKindPlan::OptionalVocab { variants, .. } => variants
                .iter()
                .cloned()
                .map(|variant| FiniteValuePlan::OptionalVocab(Some(variant)))
                .chain(std::iter::once(FiniteValuePlan::OptionalVocab(None)))
                .collect::<Vec<_>>(),
            FiniteDomainKindPlan::OptionalPresence => vec![
                FiniteValuePlan::OptionalPresence(false),
                FiniteValuePlan::OptionalPresence(true),
            ],
            FiniteDomainKindPlan::Feature { values, .. } => values
                .iter()
                .copied()
                .map(FiniteValuePlan::Feature)
                .collect(),
        };
        assignments = assignments
            .into_iter()
            .flat_map(|assignment| {
                values.iter().cloned().map(move |value| {
                    let mut assignment = assignment.clone();
                    assignment.values.push(value);
                    assignment
                })
            })
            .collect();
    }
    assignments
}

fn evaluate_guard(
    source: &crate::model::RequireExprSource,
    domains: &[FiniteDomainPlan],
    assignment: &FiniteAssignmentPlan,
) -> bool {
    match source {
        crate::model::RequireExprSource::OptionalPresence { role, present } => {
            match assignment_value(domains, assignment, &identifier_key(role)) {
                Some(FiniteValuePlan::OptionalPresence(actual)) => actual == present,
                Some(FiniteValuePlan::OptionalVocab(value)) => value.is_some() == *present,
                _ => false,
            }
        }
        crate::model::RequireExprSource::In {
            subject: crate::model::RequireSubjectSource::Role(role),
            members,
        } => match assignment_value(domains, assignment, &identifier_key(role)) {
            Some(FiniteValuePlan::Vocab(value)) => members
                .iter()
                .any(|member| identifier_key(member) == *value),
            Some(FiniteValuePlan::OptionalVocab(Some(value))) => members
                .iter()
                .any(|member| identifier_key(member) == *value),
            _ => false,
        },
        crate::model::RequireExprSource::In {
            subject: crate::model::RequireSubjectSource::RoleFeature { role, feature },
            members,
        } => match assignment_value(domains, assignment, &identifier_key(role)) {
            Some(FiniteValuePlan::Feature(value)) => {
                let feature = Feature::from(*feature);
                members
                    .iter()
                    .any(|member| feature.member(member).ok() == Some(*value))
            }
            _ => false,
        },
        crate::model::RequireExprSource::In {
            subject: crate::model::RequireSubjectSource::ConstructionFeature(feature),
            members,
        } => {
            let feature = Feature::from(*feature);
            let key = construction_feature_domain_key(feature);
            match assignment_value(domains, assignment, &key) {
                Some(FiniteValuePlan::Feature(value)) => members
                    .iter()
                    .any(|member| feature.member(member).ok() == Some(*value)),
                _ => false,
            }
        }
        crate::model::RequireExprSource::All(operands) => operands
            .iter()
            .all(|operand| evaluate_guard(operand, domains, assignment)),
        crate::model::RequireExprSource::Any(operands) => operands
            .iter()
            .any(|operand| evaluate_guard(operand, domains, assignment)),
        crate::model::RequireExprSource::Length { .. } => false,
    }
}

fn assignment_value<'a>(
    domains: &[FiniteDomainPlan],
    assignment: &'a FiniteAssignmentPlan,
    role: &str,
) -> Option<&'a FiniteValuePlan> {
    domains
        .iter()
        .position(|domain| domain.role == role)
        .and_then(|index| assignment.values.get(index))
}

fn combined_form_domains(forms: &[FormPlan]) -> Vec<FiniteDomainPlan> {
    let mut domains = Vec::new();
    for form in forms {
        let Some(predicate) = form.guard.predicate() else { continue };
        for domain in &predicate.domains {
            if !domains.iter().any(|candidate| candidate == domain) {
                domains.push(domain.clone());
            }
        }
    }
    domains
}

fn form_guard_accepts(
    guard: &FormGuardPlan,
    forms: &[FormPlan],
    domains: &[FiniteDomainPlan],
    assignment: &FiniteAssignmentPlan,
) -> bool {
    match guard {
        FormGuardPlan::Unguarded => true,
        FormGuardPlan::Predicate(predicate) => predicate.accepts(domains, assignment),
        FormGuardPlan::Otherwise {
            guarded_form_indexes,
        } => guarded_form_indexes.iter().all(|index| {
            forms[*index]
                .guard
                .predicate()
                .is_some_and(|predicate| !predicate.accepts(domains, assignment))
        }),
    }
}

fn reject_guard_overlaps(forms: &[FormPlan], fields: &[ConstructionFieldPlan]) -> syn::Result<()> {
    let domains = ordered_combined_form_domains(forms, fields);
    let assignments = enumerate_assignments(&domains);
    let guarded = forms
        .iter()
        .enumerate()
        .filter(|(_, form)| matches!(form.guard, FormGuardPlan::Predicate(_)))
        .collect::<Vec<_>>();
    for (left_offset, (left_index, left)) in guarded.iter().enumerate() {
        for (right_index, right) in guarded.iter().skip(left_offset + 1) {
            if let Some(witness) = assignments.iter().find(|assignment| {
                form_guard_accepts(&left.guard, forms, &domains, assignment)
                    && form_guard_accepts(&right.guard, forms, &domains, assignment)
            }) {
                return Err(syn::Error::new(
                    fields
                        .first()
                        .map_or(Span::call_site(), |field| field.name().span()),
                    format!(
                        "form guards `{}` and `{}` overlap at {}",
                        forms[*left_index].name,
                        forms[*right_index].name,
                        assignment_witness(&domains, witness),
                    ),
                ));
            }
        }
    }
    Ok(())
}

fn reject_unreachable_fallback(
    forms: &[FormPlan],
    fields: &[ConstructionFieldPlan],
    construction: &crate::Construction,
) -> syn::Result<()> {
    let domains = ordered_combined_form_domains(forms, fields);
    let assignments = enumerate_assignments(&domains);
    for (index, form) in forms.iter().enumerate() {
        if matches!(form.guard, FormGuardPlan::Otherwise { .. })
            && !assignments
                .iter()
                .any(|assignment| form_guard_accepts(&form.guard, forms, &domains, assignment))
        {
            let span = construction
                .forms
                .get(index)
                .map_or_else(Span::call_site, |source| source.name.span());
            return Err(syn::Error::new(
                span,
                format!(
                    "fallback form `{}` is unreachable because prior guards exhaust its finite domain",
                    form.name,
                ),
            ));
        }
    }
    Ok(())
}

fn ordered_combined_form_domains(
    forms: &[FormPlan],
    fields: &[ConstructionFieldPlan],
) -> Vec<FiniteDomainPlan> {
    let mut domains = combined_form_domains(forms);
    domains.sort_by_key(|domain| {
        fields
            .iter()
            .position(|field| field.name_key() == domain.role)
            .unwrap_or(usize::MAX)
    });
    domains
}

fn assignment_witness(domains: &[FiniteDomainPlan], assignment: &FiniteAssignmentPlan) -> String {
    domains
        .iter()
        .zip(&assignment.values)
        .map(|(domain, value)| match value {
            FiniteValuePlan::Vocab(value) | FiniteValuePlan::OptionalVocab(Some(value)) => {
                format!("{}={value}", domain.role)
            }
            FiniteValuePlan::OptionalVocab(None) | FiniteValuePlan::OptionalPresence(false) => {
                format!("{}=absent", domain.role)
            }
            FiniteValuePlan::OptionalPresence(true) => format!("{}=present", domain.role),
            FiniteValuePlan::Feature(value) => {
                let feature = match domain.kind {
                    FiniteDomainKindPlan::Feature { feature, .. } => feature,
                    FiniteDomainKindPlan::Vocab { .. }
                    | FiniteDomainKindPlan::OptionalVocab { .. }
                    | FiniteDomainKindPlan::OptionalPresence => {
                        unreachable!("feature assignment has a feature domain")
                    }
                };
                format!("{}.{}={}", domain.role, feature.key(), value.key())
            }
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn structural_field_source_leaf(kind: &crate::model::FieldKind) -> &crate::model::FieldKind {
    match kind {
        crate::model::FieldKind::Optional(value) => structural_field_source_leaf(value),
        crate::model::FieldKind::Sequence { item, .. }
        | crate::model::FieldKind::Zeroable { item, .. } => structural_field_source_leaf(item),
        crate::model::FieldKind::Category(_)
        | crate::model::FieldKind::Lex(_)
        | crate::model::FieldKind::Identity(_) => kind,
    }
}

impl ConstructionFieldPlan {
    pub(crate) fn name(&self) -> &syn::Ident {
        &self.name
    }

    pub(crate) fn name_key(&self) -> String {
        identifier_key(&self.name)
    }

    pub(crate) fn kind(&self) -> ConstructionFieldKind {
        self.kind
    }

    pub(crate) fn terminal(&self) -> &str {
        &self.terminal
    }

    pub(crate) fn value_type(&self) -> &syn::Path {
        &self.value_type
    }

    pub(crate) const fn is_zeroable(&self) -> bool {
        self.zeroable
    }

    pub(crate) const fn is_mobile(&self) -> bool {
        self.mobile
    }

    #[allow(
        dead_code,
        reason = "the semantic plan carries the declared scope sibling for its consumers"
    )]
    pub(crate) fn mobile_scope_sibling(&self) -> Option<&str> {
        self.mobile_scope_sibling.as_deref()
    }

    pub(crate) fn is_optional(&self) -> bool {
        matches!(
            self.structural_kind(),
            Some(StructuralFieldKindPlan::Optional(_))
        )
    }

    pub(crate) fn field_check(&self) -> Option<(&syn::Path, &[FieldCheckArgumentPlan])> {
        self.field_check
            .as_ref()
            .map(|(function, arguments)| (function, arguments.as_slice()))
    }

    #[allow(
        dead_code,
        reason = "Tasks 3 through 5 consume structural construction fields"
    )]
    pub(crate) fn structural_kind(&self) -> Option<&StructuralFieldKindPlan> {
        self.structural.as_ref().map(StructuralFieldPlan::kind)
    }

    #[allow(
        dead_code,
        reason = "Tasks 3 through 5 consume complete construction structural facts"
    )]
    pub(crate) fn structural_plan(&self) -> Option<&StructuralFieldPlan> {
        self.structural.as_ref()
    }

    #[allow(
        dead_code,
        reason = "later invariant emitters consume the sealed field policy"
    )]
    pub(crate) fn is_invariant_bearing(&self) -> bool {
        self.invariant_bearing
    }

    #[allow(
        dead_code,
        reason = "later invariant emitters consume the sealed field policy"
    )]
    pub(crate) fn accessor_mode(&self) -> Option<AccessorMode> {
        self.accessor_mode
    }

    fn emits_generated_accessor(&self) -> bool {
        self.accessor_mode.is_some()
            || self
                .structural
                .as_ref()
                .is_some_and(StructuralFieldPlan::emits_generated_accessor)
    }
}

impl InvariantPlan {
    pub(crate) fn from_alternatives(alternatives: Vec<PredicateConjunctionPlan>) -> Self {
        Self {
            alternatives,
            constrained_fields: Vec::new(),
            context_identity_fields: Vec::new(),
            category_feature_reads: Vec::new(),
            requires_context: false,
        }
    }

    pub(crate) fn alternatives(&self) -> &[PredicateConjunctionPlan] {
        &self.alternatives
    }

    #[allow(
        dead_code,
        reason = "later invariant emitters consume the sealed field policy"
    )]
    pub(crate) fn constrained_fields(&self) -> &[syn::Ident] {
        &self.constrained_fields
    }

    #[allow(
        dead_code,
        reason = "later invariant emitters consume the sealed field policy"
    )]
    pub(crate) fn context_identity_fields(&self) -> &[syn::Ident] {
        &self.context_identity_fields
    }

    pub(crate) fn category_feature_reads(&self) -> &[(String, Feature)] {
        &self.category_feature_reads
    }

    #[allow(
        dead_code,
        reason = "later invariant emitters consume the sealed field policy"
    )]
    pub(crate) fn requires_context(&self) -> bool {
        self.requires_context
    }

    pub(crate) fn requires_constructor(&self) -> bool {
        self.requires_context || self.requires_predicate()
    }

    pub(crate) fn requires_predicate(&self) -> bool {
        !matches!(
            self.alternatives.as_slice(),
            [alternative] if alternative.atoms.is_empty()
        )
    }

    pub(crate) fn diagnostic_identity(&self) -> String {
        let alternatives = self
            .alternatives
            .iter()
            .map(PredicateConjunctionPlan::diagnostic_identity)
            .collect::<Vec<_>>();
        match alternatives.as_slice() {
            [identity] => identity.clone(),
            identities => format!("any({})", identities.join(", ")),
        }
    }

    pub(crate) fn constant_fold_unit(
        &mut self,
        construction: &syn::Ident,
        resolutions: Option<&HashMap<feature::FeaturePlace, feature::FeatureResolution>>,
    ) -> syn::Result<()> {
        for alternative in &self.alternatives {
            let mut satisfied = true;
            for atom in &alternative.atoms {
                let PredicateSubjectPlan::ConstructionFeature(feature) = atom.subject else {
                    return Err(syn::Error::new(
                        construction.span(),
                        format!(
                            "unit invariant for construction `{construction}` is not compile-time resolvable"
                        ),
                    ));
                };
                let place = feature::FeaturePlace::Construction(feature);
                let Some(feature::FeatureResolution::Known(value)) =
                    resolutions.and_then(|resolutions| resolutions.get(&place))
                else {
                    return Err(syn::Error::new(
                        construction.span(),
                        format!(
                            "unit invariant for construction `{construction}` is not compile-time resolvable"
                        ),
                    ));
                };
                if !atom.allowed.iter().any(
                    |member| matches!(member, PredicateMemberPlan::Feature(member) if member.value() == value),
                ) {
                    satisfied = false;
                    break;
                }
            }
            if satisfied {
                self.alternatives = vec![PredicateConjunctionPlan::new(Vec::new())];
                return Ok(());
            }
        }
        Err(syn::Error::new(
            construction.span(),
            format!("unit invariant for construction `{construction}` is unsatisfiable"),
        ))
    }

    fn seal_field_policy(
        &mut self,
        fields: &mut [ConstructionFieldPlan],
        terminals: &[TerminalPlan],
        dependencies: InvariantFeatureDependencies,
    ) -> syn::Result<()> {
        self.constrained_fields = fields
            .iter()
            .filter(|field| {
                dependencies.fields.contains(&field.name_key())
                    || self.alternatives.iter().any(|alternative| {
                        alternative.atoms.iter().any(|atom| {
                            atom.subject
                                .role()
                                .is_some_and(|role| crate::identifier::same(role, &field.name))
                        })
                    })
            })
            .map(|field| field.name.clone())
            .collect();
        self.category_feature_reads = dependencies.category_reads.into_iter().collect();
        self.category_feature_reads.sort_by(|left, right| {
            left.0
                .cmp(&right.0)
                .then_with(|| left.1.key().cmp(right.1.key()))
        });
        self.context_identity_fields = fields
            .iter()
            .filter(|field| {
                field.kind == ConstructionFieldKind::Identity
                    && terminals.iter().any(|terminal| {
                        matches!(terminal, TerminalPlan::ContextIdentity(identity) if identity.name() == field.terminal)
                    })
            })
            .map(|field| field.name.clone())
            .collect();
        self.requires_context = !self.context_identity_fields.is_empty();
        self.apply_field_policy(fields, terminals)
    }

    fn apply_field_policy(
        &self,
        fields: &mut [ConstructionFieldPlan],
        terminals: &[TerminalPlan],
    ) -> syn::Result<()> {
        for field in fields {
            let bearing = self
                .constrained_fields
                .iter()
                .chain(&self.context_identity_fields)
                .any(|name| crate::identifier::same(name, &field.name));
            field.invariant_bearing = bearing;
            field.accessor_mode = bearing
                .then(|| accessor_mode(field, terminals))
                .transpose()?;
        }
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn snapshot(&self) -> String {
        if matches!(self.alternatives.as_slice(), [alternative] if alternative.atoms.is_empty()) {
            return "TRUE".to_owned();
        }
        self.alternatives
            .iter()
            .map(|alternative| {
                format!(
                    "({})",
                    alternative
                        .atoms
                        .iter()
                        .map(PredicateAtomPlan::snapshot)
                        .collect::<Vec<_>>()
                        .join(" AND ")
                )
            })
            .collect::<Vec<_>>()
            .join("\nOR\n")
    }
}

impl PredicateConjunctionPlan {
    pub(crate) fn new(atoms: Vec<PredicateAtomPlan>) -> Self {
        Self { atoms }
    }

    pub(crate) fn atoms(&self) -> &[PredicateAtomPlan] {
        &self.atoms
    }

    fn diagnostic_identity(&self) -> String {
        let atoms = self
            .atoms
            .iter()
            .map(PredicateAtomPlan::diagnostic_identity)
            .collect::<Vec<_>>();
        match atoms.as_slice() {
            [identity] => identity.clone(),
            identities => format!("all({})", identities.join(", ")),
        }
    }
}

impl PredicateAtomPlan {
    pub(crate) fn new(subject: PredicateSubjectPlan, allowed: Vec<PredicateMemberPlan>) -> Self {
        Self { subject, allowed }
    }

    pub(crate) fn subject(&self) -> &PredicateSubjectPlan {
        &self.subject
    }

    pub(crate) fn allowed(&self) -> &[PredicateMemberPlan] {
        &self.allowed
    }

    pub(crate) fn semantic_key(&self) -> String {
        format!(
            "{}={}",
            self.subject.semantic_key(),
            self.allowed
                .iter()
                .map(PredicateMemberPlan::semantic_key)
                .collect::<Vec<_>>()
                .join("|")
        )
    }

    fn diagnostic_identity(&self) -> String {
        let subject = self.subject.diagnostic_identity();
        let members = self
            .allowed
            .iter()
            .map(PredicateMemberPlan::semantic_key)
            .collect::<Vec<_>>();
        match members.as_slice() {
            [member] => format!("{subject} is {member}"),
            members => format!("{subject} in [{}]", members.join(", ")),
        }
    }

    #[cfg(test)]
    fn snapshot(&self) -> String {
        format!(
            "{} in [{}]",
            self.subject.snapshot(),
            self.allowed
                .iter()
                .map(PredicateMemberPlan::semantic_key)
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl PredicateSubjectPlan {
    pub(crate) fn role(&self) -> Option<&syn::Ident> {
        match self {
            Self::CategoryRole { role, .. }
            | Self::VocabRole { role, .. }
            | Self::OptionalPresenceRole { role }
            | Self::RoleFeature { role, .. } => Some(role),
            Self::ConstructionFeature(_) => None,
        }
    }

    pub(crate) fn semantic_key(&self) -> String {
        match self {
            Self::CategoryRole { role, category } => {
                format!("category:{}:{category}", identifier_key(role))
            }
            Self::VocabRole {
                role,
                terminal,
                optional,
            } => {
                format!("vocab:{}:{terminal}:{optional}", identifier_key(role))
            }
            Self::OptionalPresenceRole { role } => {
                format!("optional-presence:{}", identifier_key(role))
            }
            Self::RoleFeature { role, feature, .. } => {
                format!("feature:{}.{}", identifier_key(role), feature.key())
            }
            Self::ConstructionFeature(feature) => format!("feature:{}", feature.key()),
        }
    }

    fn diagnostic_identity(&self) -> String {
        match self {
            Self::CategoryRole { role, .. }
            | Self::VocabRole { role, .. }
            | Self::OptionalPresenceRole { role } => identifier_key(role),
            Self::RoleFeature { role, feature, .. } => {
                format!("{}.{}", identifier_key(role), feature.key())
            }
            Self::ConstructionFeature(feature) => feature.key().to_owned(),
        }
    }

    #[cfg(test)]
    fn snapshot(&self) -> String {
        match self {
            Self::CategoryRole { role, .. }
            | Self::VocabRole { role, .. }
            | Self::OptionalPresenceRole { role } => role.to_string(),
            Self::RoleFeature { role, feature, .. } => format!("{role}.{}", feature.key()),
            Self::ConstructionFeature(feature) => feature.key().to_owned(),
        }
    }
}

impl PredicateMemberPlan {
    pub(crate) fn semantic_key(&self) -> String {
        match self {
            Self::Variant(member) => identifier_key(member),
            Self::Presence(true) => "present".to_owned(),
            Self::Presence(false) => "absent".to_owned(),
            Self::Feature(member) => member.value().key().to_owned(),
        }
    }
}

fn accessor_mode(
    field: &ConstructionFieldPlan,
    terminals: &[TerminalPlan],
) -> syn::Result<AccessorMode> {
    if field.kind == ConstructionFieldKind::Category {
        return Ok(AccessorMode::Borrow);
    }
    terminals
        .iter()
        .find(|terminal| terminal.plan_name() == field.terminal)
        .map(|terminal| match terminal {
            TerminalPlan::Vocab(_) | TerminalPlan::Lexeme(_) | TerminalPlan::ContextIdentity(_) => {
                AccessorMode::Copy
            }
            TerminalPlan::Binding(_)
            | TerminalPlan::CatalogIdentity(_)
            | TerminalPlan::SignedDecimal(_)
            | TerminalPlan::UnsignedNumber(_)
            | TerminalPlan::DeclarationDeterminative(_)
            | TerminalPlan::DeclarationNoun(_)
            | TerminalPlan::DeclarationTerm(_) => AccessorMode::Borrow,
        })
        .ok_or_else(|| sealed_error("invariant field terminal"))
}

impl AtomPlan {
    pub(crate) fn value_atom(&self) -> &Self {
        match self {
            Self::Bound { value, .. } | Self::Circumfix { value, .. } => value.value_atom(),
            atom => atom,
        }
    }

    fn from_source(source: &FormAtom, resolved: &AtomContribution) -> syn::Result<Self> {
        match (source, resolved) {
            (
                FormAtom::Literal(literal) | FormAtom::LicensedLiteral(literal),
                AtomContribution::Literal,
            ) => Ok(Self::Literal(literal.value())),
            (FormAtom::SentenceInitial(literal), AtomContribution::Literal) => {
                Ok(Self::SentenceInitialLiteral(literal.value()))
            }
            (FormAtom::StructuralLiteral(literal), AtomContribution::Literal) => {
                Ok(Self::StructuralLiteral(literal.value()))
            }
            (FormAtom::Role(authored), AtomContribution::Category { role, category }) => {
                ensure_atom_name(authored, role, "category atom role name")?;
                Ok(Self::Category {
                    role: role.clone(),
                    category: category.clone(),
                })
            }
            (FormAtom::Lex(authored), AtomContribution::Lex { role, terminal }) => {
                ensure_atom_name(authored, role, "lex atom role name")?;
                Ok(Self::Lex {
                    role: role.clone(),
                    terminal: terminal.clone(),
                })
            }
            (FormAtom::FixedLex(path), AtomContribution::LexFixed { terminal, variant }) => {
                Ok(Self::LexFixed {
                    terminal: terminal.clone(),
                    variant: variant.clone(),
                    path: path.clone(),
                })
            }
            (
                FormAtom::Marked(authored),
                AtomContribution::Marked {
                    role,
                    category,
                    terminal,
                    variant,
                },
            ) => {
                ensure_atom_name(&authored.role, role, "marked atom role name")?;
                Ok(Self::Marked {
                    role: role.clone(),
                    category: category.clone(),
                    terminal: terminal.clone(),
                    variant: variant.clone(),
                    path: authored.marker.clone(),
                })
            }
            (FormAtom::Identity(authored), AtomContribution::Identity { role, terminal }) => {
                ensure_atom_name(authored, role, "identity atom role name")?;
                Ok(Self::Identity {
                    role: role.clone(),
                    terminal: terminal.clone(),
                })
            }
            (FormAtom::Noun(authored), AtomContribution::Noun { role, terminal }) => {
                ensure_atom_name(authored, role, "noun atom role name")?;
                Ok(Self::Noun {
                    role: role.clone(),
                    terminal: terminal.clone(),
                })
            }
            (
                FormAtom::Verb(VerbOperand::Fixed(path)),
                AtomContribution::VerbFixed { terminal, variant },
            ) => {
                let mut segments = path.segments.iter().rev();
                let authored_variant = segments.next().ok_or_else(|| {
                    syn::Error::new(path.span(), "sealed fixed-verb path is empty")
                })?;
                let authored_terminal = segments.next().ok_or_else(|| {
                    syn::Error::new(path.span(), "sealed fixed-verb path has no terminal name")
                })?;
                if identifier_key(&authored_terminal.ident) != *terminal {
                    return Err(syn::Error::new(
                        authored_terminal.ident.span(),
                        "sealed fixed-verb terminal name is inconsistent",
                    ));
                }
                if identifier_key(&authored_variant.ident) != *variant {
                    return Err(syn::Error::new(
                        authored_variant.ident.span(),
                        "sealed fixed-verb variant name is inconsistent",
                    ));
                }
                Ok(Self::VerbFixed {
                    terminal: terminal.clone(),
                    variant: variant.clone(),
                    path: path.clone(),
                })
            }
            (
                FormAtom::Verb(VerbOperand::Projected(authored)),
                AtomContribution::VerbProjected { role, terminal },
            ) => {
                ensure_atom_name(authored, role, "projected verb role name")?;
                Ok(Self::Lex {
                    role: role.clone(),
                    terminal: terminal.clone(),
                })
            }
            (FormAtom::OpenVerb(authored), AtomContribution::OpenDeclaration { kind, name }) => {
                if authored.name.value() != *name {
                    return Err(syn::Error::new(
                        authored.name.span(),
                        "sealed open declaration name is inconsistent",
                    ));
                }
                Ok(Self::OpenDeclaration(OpenDeclarationAtomPlan {
                    kind: *kind,
                    name: name.clone(),
                    position: crate::macro_def::GrammarPosition::Verb,
                }))
            }
            (FormAtom::Bound(authored), resolved) => {
                let direction = match authored.direction {
                    crate::model::BoundDirection::Prefix => BoundDirectionPlan::Prefix,
                    crate::model::BoundDirection::Suffix => BoundDirectionPlan::Suffix,
                };
                Ok(Self::Bound {
                    direction,
                    affix: authored.affix.value(),
                    value: Box::new(Self::from_source(&authored.value, resolved)?),
                })
            }
            (FormAtom::Circumfix(authored), AtomContribution::Category { role, category }) => {
                ensure_atom_name(&authored.role, role, "circumfix role name")?;
                Ok(Self::Circumfix {
                    prefix: authored.prefix.value(),
                    value: Box::new(Self::Category {
                        role: role.clone(),
                        category: category.clone(),
                    }),
                    suffix: authored.suffix.value(),
                })
            }
            _ => Err(syn::Error::new(
                form_atom_span(source),
                "sealed construction atom kind is inconsistent",
            )),
        }
    }

    #[cfg(test)]
    fn snapshot(&self) -> String {
        match self {
            Self::Literal(_) => "literal".to_owned(),
            Self::SentenceInitialLiteral(_) => "sentence_initial(literal)".to_owned(),
            Self::StructuralLiteral(_) => "structural(literal)".to_owned(),
            Self::Category { role, category } => format!("category({role}: {category})"),
            Self::Lex { role, .. } => format!("lex({role})"),
            Self::LexFixed {
                terminal, variant, ..
            } => format!("lex({terminal}::{variant})"),
            Self::Marked {
                role,
                terminal,
                variant,
                ..
            } => format!("marked({terminal}::{variant}, {role})"),
            Self::Identity { role, .. } => format!("identity({role})"),
            Self::Noun { role, .. } => format!("noun({role})"),
            Self::VerbFixed {
                terminal, variant, ..
            } => format!("verb({terminal}::{variant})"),
            Self::OpenDeclaration(open) => format!(
                "open_verb({:?}, {}, {:?})",
                open.kind, open.name, open.position
            ),
            Self::Bound {
                direction,
                affix,
                value,
            } => format!("{direction:?}({affix:?}, {})", value.snapshot()),
            Self::Circumfix {
                prefix,
                value,
                suffix,
            } => format!("circumfix({prefix:?}, {}, {suffix:?})", value.snapshot()),
        }
    }
}

impl OpenDeclarationAtomPlan {
    pub(crate) fn kind(&self) -> crate::macro_def::DeclarationKind {
        self.kind
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn position(&self) -> crate::macro_def::GrammarPosition {
        self.position
    }
}

fn ensure_atom_name(authored: &syn::Ident, resolved: &str, fact: &str) -> syn::Result<()> {
    if identifier_key(authored) == resolved {
        Ok(())
    } else {
        Err(syn::Error::new(
            authored.span(),
            format!("sealed construction {fact} is inconsistent"),
        ))
    }
}

fn seal_atoms(form: &Form, resolved: &[AtomContribution]) -> syn::Result<Vec<AtomPlan>> {
    if form.atoms.len() != resolved.len() {
        return Err(syn::Error::new(
            form.name.span(),
            "sealed construction atom count is inconsistent",
        ));
    }
    form.atoms
        .iter()
        .zip(resolved)
        .map(|(source, resolved)| AtomPlan::from_source(source, resolved))
        .collect()
}

fn form_atom_span(atom: &FormAtom) -> Span {
    match atom {
        FormAtom::Literal(literal)
        | FormAtom::LicensedLiteral(literal)
        | FormAtom::SentenceInitial(literal)
        | FormAtom::StructuralLiteral(literal) => literal.span(),
        FormAtom::Role(role)
        | FormAtom::Lex(role)
        | FormAtom::Identity(role)
        | FormAtom::Noun(role)
        | FormAtom::Verb(VerbOperand::Projected(role)) => role.span(),
        FormAtom::Verb(VerbOperand::Fixed(path)) | FormAtom::FixedLex(path) => path.span(),
        FormAtom::Marked(marked) => marked.marker.span(),
        FormAtom::OpenVerb(open) => open.name.span(),
        FormAtom::Bound(bound) => bound.affix.span(),
        FormAtom::Circumfix(circumfix) => circumfix.prefix.span(),
    }
}

fn number_carry_categories(
    constructions: &[ConstructionPlan],
    equations: &HashMap<String, Vec<feature::FeatureEquation>>,
    checked_field_category_reads: &HashMap<String, HashSet<Feature>>,
) -> HashSet<String> {
    let mut carried = checked_field_category_reads
        .iter()
        .filter_map(|(category, features)| {
            features
                .contains(&Feature::Number)
                .then_some(category.clone())
        })
        .collect::<HashSet<_>>();
    for construction in constructions {
        for equation in equations
            .get(&construction.construction_id)
            .into_iter()
            .flatten()
        {
            let FeaturePlace::Role {
                field,
                feature: Feature::Number,
            } = equation.target()
            else {
                continue;
            };
            let Some(field) = construction
                .fields
                .iter()
                .find(|candidate| candidate.name_key() == identifier_key(field))
            else {
                continue;
            };
            if field.kind() == ConstructionFieldKind::Category {
                carried.insert(field.terminal().to_owned());
            }
            if let Some(StructuralFieldKindPlan::Sequence {
                item: ValueKindPlan::Category(category),
                ..
            }) = field.structural_plan().map(StructuralFieldPlan::kind)
            {
                carried.insert(category.clone());
            }
        }
    }
    loop {
        let before = carried.len();
        for construction in constructions {
            let output_is_needed = construction
                .forms
                .iter()
                .flat_map(FormPlan::atoms)
                .any(|atom| matches!(atom, AtomPlan::Noun { .. }))
                || construction.forms.iter().any(|form| {
                    form.guard().predicate().is_some_and(|predicate| {
                        predicate.domains().iter().any(|domain| {
                            domain.role() == construction_feature_domain_key(Feature::Number)
                        })
                    })
                })
                || carried.contains(&construction.category)
                || checked_field_category_reads
                    .get(&construction.category)
                    .is_some_and(|reads| reads.contains(&Feature::Number));
            if !output_is_needed {
                continue;
            }
            let Some(FeatureExpr::FromRole {
                role,
                feature: Feature::Number,
            }) = equations
                .get(&construction.construction_id)
                .into_iter()
                .flatten()
                .find(|equation| equation.target() == &FeaturePlace::Construction(Feature::Number))
                .map(feature::FeatureEquation::value)
            else {
                continue;
            };
            if let Some(category) = construction
                .forms
                .iter()
                .flat_map(FormPlan::atoms)
                .find_map(|atom| match atom.value_atom() {
                    AtomPlan::Category {
                        role: found,
                        category,
                    } if found == &identifier_key(role) => Some(category.clone()),
                    AtomPlan::Literal(_)
                    | AtomPlan::SentenceInitialLiteral(_)
                    | AtomPlan::StructuralLiteral(_)
                    | AtomPlan::Category { .. }
                    | AtomPlan::Lex { .. }
                    | AtomPlan::LexFixed { .. }
                    | AtomPlan::Marked { .. }
                    | AtomPlan::Identity { .. }
                    | AtomPlan::Noun { .. }
                    | AtomPlan::VerbFixed { .. }
                    | AtomPlan::OpenDeclaration(_)
                    | AtomPlan::Bound { .. }
                    | AtomPlan::Circumfix { .. } => None,
                })
            {
                carried.insert(category);
            }
        }
        if carried.len() == before {
            return carried;
        }
    }
}

fn onset_carry_categories(
    constructions: &[ConstructionPlan],
    equations: &HashMap<String, Vec<feature::FeatureEquation>>,
) -> HashSet<String> {
    let mut members = HashMap::<String, Vec<&ConstructionPlan>>::new();
    for construction in constructions {
        members
            .entry(construction.category.clone())
            .or_default()
            .push(construction);
    }
    members
        .into_iter()
        .filter_map(|(category, constructions)| {
            constructions
                .iter()
                .all(|construction| {
                    equations
                        .get(&construction.construction_id)
                        .into_iter()
                        .flatten()
                        .any(|equation| {
                            equation.target() == &FeaturePlace::Construction(Feature::Onset)
                        })
                })
                .then_some(category)
        })
        .collect()
}

fn cardinality_carry_categories(
    constructions: &[ConstructionPlan],
    equations: &HashMap<String, Vec<feature::FeatureEquation>>,
) -> HashSet<String> {
    let mut members = HashMap::<String, Vec<&ConstructionPlan>>::new();
    for construction in constructions {
        members
            .entry(construction.category.clone())
            .or_default()
            .push(construction);
    }
    members
        .into_iter()
        .filter_map(|(category, constructions)| {
            constructions
                .iter()
                .all(|construction| {
                    equations
                        .get(&construction.construction_id)
                        .into_iter()
                        .flatten()
                        .any(|equation| {
                            equation.target() == &FeaturePlace::Construction(Feature::Cardinality)
                        })
                })
                .then_some(category)
        })
        .collect()
}

fn possessive_ending_carry_categories(
    constructions: &[ConstructionPlan],
    equations: &HashMap<String, Vec<feature::FeatureEquation>>,
) -> HashSet<String> {
    let mut members = HashMap::<String, Vec<&ConstructionPlan>>::new();
    for construction in constructions {
        members
            .entry(construction.category.clone())
            .or_default()
            .push(construction);
    }
    members
        .into_iter()
        .filter_map(|(category, constructions)| {
            constructions
                .iter()
                .all(|construction| {
                    equations
                        .get(&construction.construction_id)
                        .into_iter()
                        .flatten()
                        .any(|equation| {
                            equation.target()
                                == &FeaturePlace::Construction(Feature::PossessiveEnding)
                        })
                })
                .then_some(category)
        })
        .collect()
}

fn validate_onset_provider_capabilities(
    constructions: &[ConstructionPlan],
    terminals: &[TerminalPlan],
    equations: &HashMap<String, Vec<feature::FeatureEquation>>,
) -> syn::Result<()> {
    let provides_onset = |name: &str| {
        terminals
            .iter()
            .find(|terminal| terminal.plan_name() == name)
            .is_some_and(TerminalPlan::provides_onset)
    };
    let mut errors: Option<syn::Error> = None;
    for construction in constructions {
        let mut roles = construction
            .forms()
            .iter()
            .filter_map(|form| form.guard().predicate())
            .flat_map(FinitePredicatePlan::domains)
            .filter_map(|domain| {
                matches!(
                    domain.kind(),
                    FiniteDomainKindPlan::Feature {
                        feature: Feature::Onset,
                        ..
                    }
                )
                .then_some(domain.role())
            })
            .map(str::to_owned)
            .collect::<HashSet<_>>();
        roles.extend(
            equations
                .get(construction.construction_id())
                .into_iter()
                .flatten()
                .filter_map(|equation| match equation.value() {
                    feature::FeatureExpr::FromRole {
                        role,
                        feature: Feature::Onset,
                    } => Some(identifier_key(role)),
                    feature::FeatureExpr::Constant(_)
                    | feature::FeatureExpr::FromRole { .. }
                    | feature::FeatureExpr::MatchVocab { .. } => None,
                }),
        );
        for role in roles {
            if role == "verb" {
                let mut sealed_provider = None;
                for form in construction.forms() {
                    let providers = form
                        .atoms()
                        .iter()
                        .filter_map(|atom| match atom.value_atom() {
                            AtomPlan::VerbFixed {
                                terminal, variant, ..
                            } => Some(format!("closed:{terminal}:{variant}")),
                            AtomPlan::OpenDeclaration(open) => {
                                Some(format!("open:{:?}:{}", open.kind(), open.name()))
                            }
                            AtomPlan::Literal(_)
                            | AtomPlan::SentenceInitialLiteral(_)
                            | AtomPlan::StructuralLiteral(_)
                            | AtomPlan::Category { .. }
                            | AtomPlan::Lex { .. }
                            | AtomPlan::LexFixed { .. }
                            | AtomPlan::Marked { .. }
                            | AtomPlan::Identity { .. }
                            | AtomPlan::Noun { .. }
                            | AtomPlan::Bound { .. }
                            | AtomPlan::Circumfix { .. } => None,
                        })
                        .collect::<Vec<_>>();
                    let [provider] = providers.as_slice() else {
                        let error = syn::Error::new(
                            form.origin_span(),
                            "verb onset requires exactly one sealed terminal provider in every form",
                        );
                        if let Some(errors) = &mut errors {
                            errors.combine(error);
                        } else {
                            errors = Some(error);
                        }
                        continue;
                    };
                    if sealed_provider
                        .as_ref()
                        .is_some_and(|sealed| sealed != provider)
                    {
                        let error = syn::Error::new(
                            form.origin_span(),
                            "verb onset requires the same sealed terminal provider in every form",
                        );
                        if let Some(errors) = &mut errors {
                            errors.combine(error);
                        } else {
                            errors = Some(error);
                        }
                    } else {
                        sealed_provider.get_or_insert_with(|| provider.clone());
                    }
                }
                continue;
            }
            let Some(field) = construction
                .fields()
                .iter()
                .find(|field| field.name_key() == role)
            else {
                continue;
            };
            if field.kind() != ConstructionFieldKind::Category && !provides_onset(field.terminal())
            {
                let error = syn::Error::new(
                    field.name().span(),
                    format!("terminal `{}` does not provide onset", field.terminal()),
                );
                if let Some(errors) = &mut errors {
                    errors.combine(error);
                } else {
                    errors = Some(error);
                }
            }
        }
    }
    errors.map_or(Ok(()), Err)
}

fn validate_possessive_ending_provider_capabilities(
    constructions: &[ConstructionPlan],
    terminals: &[TerminalPlan],
    equations: &HashMap<String, Vec<feature::FeatureEquation>>,
) -> syn::Result<()> {
    let provides_ending = |name: &str| {
        terminals
            .iter()
            .find(|terminal| terminal.plan_name() == name)
            .is_some_and(TerminalPlan::provides_possessive_ending)
    };
    let mut errors: Option<syn::Error> = None;
    for construction in constructions {
        let mut roles = construction
            .forms()
            .iter()
            .filter_map(|form| form.guard().predicate())
            .flat_map(FinitePredicatePlan::domains)
            .filter_map(|domain| {
                matches!(
                    domain.kind(),
                    FiniteDomainKindPlan::Feature {
                        feature: Feature::PossessiveEnding,
                        ..
                    }
                )
                .then_some(domain.role())
            })
            .filter(|role| !role.starts_with('@'))
            .map(str::to_owned)
            .collect::<HashSet<_>>();
        roles.extend(
            equations
                .get(construction.construction_id())
                .into_iter()
                .flatten()
                .filter_map(|equation| match equation.value() {
                    feature::FeatureExpr::FromRole {
                        role,
                        feature: Feature::PossessiveEnding,
                    } => Some(identifier_key(role)),
                    feature::FeatureExpr::Constant(_)
                    | feature::FeatureExpr::FromRole { .. }
                    | feature::FeatureExpr::MatchVocab { .. } => None,
                }),
        );
        for role in roles {
            let Some(field) = construction
                .fields()
                .iter()
                .find(|field| field.name_key() == role)
            else {
                continue;
            };
            if field.kind() != ConstructionFieldKind::Category && !provides_ending(field.terminal())
            {
                let error = syn::Error::new(
                    field.name().span(),
                    format!(
                        "terminal `{}` does not provide possessive ending",
                        field.terminal()
                    ),
                );
                if let Some(errors) = &mut errors {
                    errors.combine(error);
                } else {
                    errors = Some(error);
                }
            }
        }
    }
    errors.map_or(Ok(()), Err)
}

fn sealed_error(fact: &str) -> syn::Error {
    syn::Error::new(
        proc_macro2::Span::call_site(),
        format!("sealed semantic plan has an inconsistent {fact}"),
    )
}

impl TerminalPlan {
    fn source_index(&self) -> usize {
        match self {
            Self::Vocab(plan) => plan.source_index(),
            Self::Lexeme(plan) => plan.source_index(),
            Self::Binding(plan) => plan.source_index(),
            Self::ContextIdentity(plan) => plan.source_index(),
            Self::CatalogIdentity(plan) => plan.source_index(),
            Self::SignedDecimal(plan) => plan.source_index(),
            Self::UnsignedNumber(plan) => plan.source_index(),
            Self::DeclarationDeterminative(plan) => plan.source_index(),
            Self::DeclarationNoun(plan) => plan.source_index(),
            Self::DeclarationTerm(plan) => plan.source_index(),
        }
    }

    fn plan_name(&self) -> &str {
        match self {
            Self::Vocab(plan) => plan.name(),
            Self::Lexeme(plan) => plan.name(),
            Self::Binding(plan) => plan.name(),
            Self::ContextIdentity(plan) => plan.name(),
            Self::CatalogIdentity(plan) => plan.name(),
            Self::SignedDecimal(plan) => plan.codec_name(),
            Self::UnsignedNumber(plan) => plan.codec_name(),
            Self::DeclarationDeterminative(plan) => plan.codec_name(),
            Self::DeclarationNoun(plan) => plan.codec_name(),
            Self::DeclarationTerm(plan) => plan.codec_name(),
        }
    }

    pub(crate) fn expected_terminal_item_keys(&self) -> Vec<crate::ItemKey> {
        match self {
            Self::Vocab(vocab) => vec![crate::ItemKey::named_type(vocab.name())],
            Self::Lexeme(lexeme) => vec![
                crate::ItemKey::named_type(lexeme.name()),
                crate::ItemKey::Named {
                    kind: crate::NamedKind::Function,
                    name: crate::identifier::lexeme_surface_helper(lexeme.name()),
                },
            ],
            Self::Binding(_)
            | Self::ContextIdentity(_)
            | Self::CatalogIdentity(_)
            | Self::SignedDecimal(_)
            | Self::UnsignedNumber(_)
            | Self::DeclarationDeterminative(_)
            | Self::DeclarationNoun(_)
            | Self::DeclarationTerm(_) => Vec::new(),
        }
    }
}

#[cfg(test)]
impl TerminalPlan {
    pub(crate) fn name(&self) -> &str {
        match self {
            Self::Vocab(plan) => plan.name(),
            Self::Lexeme(plan) => plan.name(),
            Self::Binding(plan) => plan.name(),
            Self::ContextIdentity(plan) => plan.name(),
            Self::CatalogIdentity(plan) => plan.name(),
            Self::SignedDecimal(plan) => plan.codec_name(),
            Self::UnsignedNumber(plan) => plan.codec_name(),
            Self::DeclarationDeterminative(plan) => plan.codec_name(),
            Self::DeclarationNoun(plan) => plan.codec_name(),
            Self::DeclarationTerm(plan) => plan.codec_name(),
        }
    }

    #[cfg(test)]
    fn snapshot(&self) -> (String, String, Vec<String>) {
        let kind = match self {
            Self::Vocab(_) => "vocab",
            Self::Lexeme(_) => "lexeme",
            Self::Binding(binding)
                if binding.kind() == crate::model::TerminalBindingKind::Identity =>
            {
                "identity"
            }
            Self::ContextIdentity(_) | Self::CatalogIdentity(_) => "identity",
            Self::Binding(_)
            | Self::SignedDecimal(_)
            | Self::UnsignedNumber(_)
            | Self::DeclarationDeterminative(_)
            | Self::DeclarationNoun(_)
            | Self::DeclarationTerm(_) => "codec",
        };
        let mut capabilities = Vec::new();
        if self.supports_lex_atom() {
            capabilities.push("lex".to_owned());
        }
        if self.supports_identity_atom() {
            capabilities.push("identity".to_owned());
        }
        if self.supports_noun_atom() {
            capabilities.push("noun".to_owned());
        }
        if self.supports_verb_atom() {
            capabilities.push("verb".to_owned());
        }
        if self.has_direct_render() {
            capabilities.push("render".to_owned());
        }
        if self.has_direct_build() {
            capabilities.push("build".to_owned());
        }
        capabilities.push("traversal".to_owned());
        (self.name().to_owned(), kind.to_owned(), capabilities)
    }

    pub(crate) fn supports_lex_atom(&self) -> bool {
        matches!(
            self,
            Self::Vocab(_) | Self::SignedDecimal(_) | Self::UnsignedNumber(_)
        ) || matches!(self, Self::Binding(binding) if binding.codec_atom() == Some(crate::model::CodecAtomClass::Lex))
    }

    pub(crate) fn supports_identity_atom(&self) -> bool {
        matches!(self, Self::ContextIdentity(_) | Self::CatalogIdentity(_))
            || matches!(self, Self::Binding(binding) if binding.kind() == crate::model::TerminalBindingKind::Identity)
    }

    pub(crate) fn supports_noun_atom(&self) -> bool {
        matches!(self, Self::DeclarationNoun(_))
            || matches!(self, Self::Binding(binding) if binding.codec_atom() == Some(crate::model::CodecAtomClass::Noun))
    }

    pub(crate) fn supports_verb_atom(&self) -> bool {
        matches!(self, Self::Lexeme(lexeme) if lexeme.is_verb_provider())
            || matches!(self, Self::Binding(binding) if binding.declaration_verb().is_some())
    }

    pub(crate) fn has_direct_render(&self) -> bool {
        matches!(
            self,
            Self::Vocab(_)
                | Self::ContextIdentity(_)
                | Self::CatalogIdentity(_)
                | Self::SignedDecimal(_)
                | Self::UnsignedNumber(_)
                | Self::DeclarationNoun(_)
        ) || matches!(self, Self::Binding(binding) if binding.render().is_some())
    }

    pub(crate) fn has_direct_build(&self) -> bool {
        matches!(
            self,
            Self::Vocab(_)
                | Self::ContextIdentity(_)
                | Self::CatalogIdentity(_)
                | Self::SignedDecimal(_)
                | Self::UnsignedNumber(_)
                | Self::DeclarationNoun(_)
        ) || matches!(self, Self::Binding(binding) if binding.build().is_some())
    }

    #[allow(
        clippy::unused_self,
        reason = "every closed terminal row owns a traversal recipe"
    )]
    pub(crate) fn has_traversal(&self) -> bool {
        true
    }
}

impl VocabPlan {
    fn from_source(source_index: usize, source: &crate::Vocab) -> syn::Result<Self> {
        let mut features = Vec::new();
        let feature_kinds = source
            .feature_defaults
            .iter()
            .map(|row| row.feature)
            .chain(
                source
                    .variants
                    .iter()
                    .flat_map(|variant| variant.feature_overrides.iter().map(|row| row.feature)),
            )
            .collect::<std::collections::BTreeSet<_>>();
        for feature_kind in feature_kinds {
            let feature = Feature::from(feature_kind);
            let default_value = source
                .feature_defaults
                .iter()
                .find(|row| row.feature == feature_kind)
                .map(|row| feature.member(&row.value))
                .transpose()?;
            let members = source
                .variants
                .iter()
                .map(|variant| {
                    let value = variant
                        .feature_overrides
                        .iter()
                        .find(|override_| override_.feature == feature_kind)
                        .map(|override_| feature.member(&override_.value))
                        .transpose()?
                        .or(default_value)
                        .ok_or_else(|| {
                            syn::Error::new(
                                variant.name.span(),
                                format!(
                                    "vocab feature `{}` has no value for member `{}`",
                                    feature.key(),
                                    variant.name
                                ),
                            )
                        })?;
                    Ok((identifier_key(&variant.name), value))
                })
                .collect::<syn::Result<Vec<_>>>()?;
            features.push(LexemeFeaturePlan { feature, members });
        }
        Ok(Self {
            source_index,
            name: source.name.clone(),
            name_key: identifier_key(&source.name),
            variants: source
                .variants
                .iter()
                .map(|variant| {
                    let onset =
                        crate::macro_def::normalize_surface_onset(&variant.word.value(), None)
                            .ok_or_else(|| {
                                syn::Error::new(
                                    variant.word.span(),
                                    "vocab spelling has no bounded onset and no authored override",
                                )
                            })?;
                    Ok(VocabVariantPlan {
                        name: variant.name.clone(),
                        word: variant.word.clone(),
                        onset,
                    })
                })
                .collect::<syn::Result<Vec<_>>>()?,
            features,
        })
    }

    pub(crate) fn name(&self) -> &str {
        &self.name_key
    }

    pub(crate) fn name_ident(&self) -> &syn::Ident {
        &self.name
    }

    pub(crate) fn variants(&self) -> &[VocabVariantPlan] {
        &self.variants
    }

    pub(crate) fn feature_members(&self, feature: Feature) -> Option<&[(String, FeatureValue)]> {
        self.features
            .iter()
            .find(|row| row.feature == feature)
            .map(|row| row.members.as_slice())
    }

    #[allow(
        dead_code,
        reason = "future emitters retain source order through this index"
    )]
    pub(crate) fn source_index(&self) -> usize {
        self.source_index
    }
}

#[allow(
    dead_code,
    reason = "sealed morphology rows are consumed by later generation phases"
)]
impl MorphologyPlan {
    fn from_source(source: &crate::Morphology) -> syn::Result<Self> {
        let recipe =
            crate::morphology::MorphologyRecipe::from_ident(&source.recipe).ok_or_else(|| {
                syn::Error::new(
                    source.recipe.span(),
                    format!("unknown morphology recipe `{}`", source.recipe),
                )
            })?;
        if recipe.feature() != source.feature {
            return Err(syn::Error::new(
                source.name.span(),
                format!(
                    "morphology `{}` feature axis does not match recipe",
                    source.name
                ),
            ));
        }
        Ok(Self {
            name: source.name.clone(),
            name_key: identifier_key(&source.name),
            feature: source.feature,
            recipe,
        })
    }

    pub(crate) fn name(&self) -> &str {
        &self.name_key
    }

    pub(crate) fn name_ident(&self) -> &syn::Ident {
        &self.name
    }

    pub(crate) fn feature(&self) -> crate::Feature {
        self.feature
    }

    pub(crate) fn recipe(&self) -> crate::morphology::MorphologyRecipe {
        self.recipe
    }
}

impl LexemePlan {
    fn from_source(
        source_index: usize,
        source: &crate::Lexeme,
        morphology: MorphologyPlan,
        verb_provider: bool,
    ) -> syn::Result<Self> {
        let mut surfaces = Vec::new();
        let mut irregulars = Vec::new();
        let mut features = Vec::new();
        let feature_kinds = source
            .feature_defaults
            .iter()
            .map(|row| row.feature)
            .chain(
                source
                    .members
                    .iter()
                    .flat_map(|member| member.feature_overrides.iter().map(|row| row.feature)),
            )
            .collect::<std::collections::BTreeSet<_>>();
        for feature_kind in feature_kinds {
            let feature = Feature::from(feature_kind);
            let default_value = source
                .feature_defaults
                .iter()
                .find(|row| row.feature == feature_kind)
                .map(|row| feature.member(&row.value))
                .transpose()?;
            let members = source
                .members
                .iter()
                .map(|member| {
                    let value = member
                        .feature_overrides
                        .iter()
                        .find(|override_| override_.feature == feature_kind)
                        .map(|override_| feature.member(&override_.value))
                        .transpose()?
                        .or(default_value)
                        .ok_or_else(|| {
                            syn::Error::new(
                                member.name.span(),
                                format!(
                                    "lexeme feature `{}` has no value for member `{}`",
                                    feature.key(),
                                    member.name
                                ),
                            )
                        })?;
                    Ok((identifier_key(&member.name), value))
                })
                .collect::<syn::Result<Vec<_>>>()?;
            features.push(LexemeFeaturePlan { feature, members });
        }
        for member in &source.members {
            for &feature in morphology.recipe().features() {
                let surface = member
                    .overrides
                    .iter()
                    .find_map(|row| {
                        (morphology.recipe().feature_from_ident(&row.feature) == Some(feature))
                            .then(|| row.surface.value())
                    })
                    .map_or_else(
                        || {
                            crate::morphology::derive_surface(
                                morphology.recipe(),
                                &member.lemma.value(),
                                feature,
                            )
                        },
                        Ok,
                    )?;
                let onset =
                    crate::macro_def::normalize_surface_onset(&surface, None).ok_or_else(|| {
                        syn::Error::new(member.name.span(), "lexeme surface has unknown onset")
                    })?;
                surfaces.push(LexemeSurfacePlan {
                    span: member.name.span(),
                    member: identifier_key(&member.name),
                    feature,
                    surface,
                    onset,
                });
            }
            if !member.overrides.is_empty() {
                irregulars.push(LexemeIrregularPlan {
                    member: identifier_key(&member.name),
                    overrides: member
                        .overrides
                        .iter()
                        .map(|row| LexemeOverridePlan {
                            feature: morphology
                                .recipe()
                                .feature_from_ident(&row.feature)
                                .expect("validated override feature"),
                            surface: row.surface.value(),
                        })
                        .collect(),
                });
            }
        }
        let expected_members = source
            .members
            .iter()
            .map(|member| {
                (
                    identifier_key(&member.name),
                    member.name.span(),
                    morphology.recipe().features(),
                )
            })
            .collect::<Vec<_>>();
        let expected_members = expected_members
            .iter()
            .map(|(member, span, features)| (member.as_str(), *span, *features))
            .collect::<Vec<_>>();
        let realized_rows = surfaces
            .iter()
            .map(|row| (row.span, row.member(), row.feature(), row.surface()))
            .collect::<Vec<_>>();
        validate_complete_surface_rows(&expected_members, &realized_rows)?;
        Ok(Self {
            source_index,
            name: source.name.clone(),
            name_key: identifier_key(&source.name),
            variants: source
                .members
                .iter()
                .map(|member| member.name.clone())
                .collect(),
            morphology,
            surfaces,
            irregulars,
            features,
            verb_provider,
        })
    }

    pub(crate) fn name(&self) -> &str {
        &self.name_key
    }

    pub(crate) fn name_ident(&self) -> &syn::Ident {
        &self.name
    }

    pub(crate) fn variants(&self) -> &[syn::Ident] {
        &self.variants
    }

    pub(crate) fn feature_members(&self, feature: Feature) -> Option<&[(String, FeatureValue)]> {
        self.features
            .iter()
            .find(|row| row.feature == feature)
            .map(|row| row.members.as_slice())
    }

    pub(crate) fn is_verb_provider(&self) -> bool {
        self.verb_provider
    }

    #[allow(
        dead_code,
        reason = "sealed morphology rows are consumed by later generation phases"
    )]
    pub(crate) fn morphology(&self) -> &MorphologyPlan {
        &self.morphology
    }

    pub(crate) fn surfaces(&self) -> &[LexemeSurfacePlan] {
        &self.surfaces
    }

    #[allow(
        dead_code,
        reason = "sealed irregular rows are consumed by later generation phases"
    )]
    pub(crate) fn irregulars(&self) -> &[LexemeIrregularPlan] {
        &self.irregulars
    }
    #[allow(
        dead_code,
        reason = "future emitters retain source order through this index"
    )]
    pub(crate) fn source_index(&self) -> usize {
        self.source_index
    }
}

fn validate_complete_surface_rows(
    expected_members: &[(&str, Span, &[crate::macro_def::SurfaceFeature])],
    rows: &[(Span, &str, crate::macro_def::SurfaceFeature, &str)],
) -> syn::Result<()> {
    let mut errors: Option<syn::Error> = None;
    let mut exact_rows = HashSet::new();
    let mut covered_features = HashSet::new();
    for &(span, member, feature, surface) in rows {
        if !exact_rows.insert((member, feature, surface)) {
            let feature = morphology_feature_name(feature);
            let error = syn::Error::new(
                span,
                format!("duplicate exact morphology row `({member}, {feature}, {surface})`"),
            );
            if let Some(errors) = &mut errors {
                errors.combine(error);
            } else {
                errors = Some(error);
            }
        }
        covered_features.insert((member, feature));
    }
    for &(member, span, features) in expected_members {
        for &feature in features {
            if !covered_features.contains(&(member, feature)) {
                let feature = morphology_feature_name(feature);
                let error = syn::Error::new(
                    span,
                    format!("missing realized surface row for `{member}` feature `{feature}`"),
                );
                if let Some(errors) = &mut errors {
                    errors.combine(error);
                } else {
                    errors = Some(error);
                }
            }
        }
    }
    errors.map_or(Ok(()), Err)
}

fn morphology_feature_name(feature: crate::macro_def::SurfaceFeature) -> &'static str {
    use crate::macro_def::InflectionalForm;
    use crate::macro_def::SurfaceFeature;

    match feature {
        SurfaceFeature::Inflectional(InflectionalForm::Plain) => "Plain",
        SurfaceFeature::Inflectional(InflectionalForm::ThirdPersonSingularPresent) => {
            "ThirdPersonSingularPresent"
        }
        SurfaceFeature::Inflectional(InflectionalForm::Preterite) => "Preterite",
        SurfaceFeature::Inflectional(InflectionalForm::GerundParticiple) => "GerundParticiple",
        SurfaceFeature::Inflectional(InflectionalForm::PastParticiple) => "PastParticiple",
        SurfaceFeature::Singular => "Singular",
        SurfaceFeature::Plural => "Plural",
        SurfaceFeature::Fixed => "Fixed",
        SurfaceFeature::BlockLabel => "BlockLabel",
    }
}

impl LexemeSurfacePlan {
    pub(crate) fn member(&self) -> &str {
        &self.member
    }

    pub(crate) fn feature(&self) -> crate::macro_def::SurfaceFeature {
        self.feature
    }

    pub(crate) fn surface(&self) -> &str {
        &self.surface
    }

    pub(crate) fn onset(&self) -> crate::macro_def::Onset {
        self.onset
    }
}

#[allow(
    dead_code,
    reason = "sealed irregular rows are consumed by later generation phases"
)]
impl LexemeIrregularPlan {
    pub(crate) fn member(&self) -> &str {
        &self.member
    }

    pub(crate) fn overrides(&self) -> &[LexemeOverridePlan] {
        &self.overrides
    }
}

#[allow(
    dead_code,
    reason = "sealed irregular rows are consumed by later generation phases"
)]
impl LexemeOverridePlan {
    pub(crate) fn feature(&self) -> crate::macro_def::SurfaceFeature {
        self.feature
    }

    pub(crate) fn surface(&self) -> &str {
        &self.surface
    }
}

impl ContextIdentityPlan {
    fn from_source(source_index: usize, source: &crate::TerminalBinding) -> Self {
        let Some(crate::model::GeneratedIdentityRecipe::Context(recipe)) =
            &source.generated_identity
        else {
            unreachable!("validated generated identity has the context recipe")
        };
        let canonical = recipe
            .canonical_slots
            .first()
            .expect("validated context identity has one canonical arm")
            .arm
            .clone();
        let name = identifier_key(&source.name);
        let aggregate_name = name.strip_suffix("Spelling").unwrap_or(&name).to_owned();
        Self {
            source_index,
            origin: DeclarationKey::new(SourceDeclarationKind::Identity, &name),
            origin_span: source.name.span(),
            name,
            ident: source.name.clone(),
            aggregate_ident: syn::Ident::new(&aggregate_name, source.name.span()),
            arms: recipe
                .arms
                .iter()
                .map(|arm| ContextIdentityArmPlan {
                    variant: arm.variant.clone(),
                    accessor: arm.accessor.clone(),
                })
                .collect(),
            canonical,
        }
    }

    pub(crate) fn source_index(&self) -> usize {
        self.source_index
    }

    pub(crate) fn origin(&self) -> &DeclarationKey {
        &self.origin
    }

    pub(crate) fn origin_span(&self) -> Span {
        self.origin_span
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn ident(&self) -> &syn::Ident {
        &self.ident
    }

    pub(crate) fn aggregate_ident(&self) -> &syn::Ident {
        &self.aggregate_ident
    }

    pub(crate) fn arms(&self) -> &[ContextIdentityArmPlan] {
        &self.arms
    }

    pub(crate) fn canonical(&self) -> &syn::Ident {
        &self.canonical
    }
}

impl CatalogIdentityPlan {
    fn from_source(source_index: usize, source: &crate::TerminalBinding) -> Self {
        let Some(crate::model::GeneratedIdentityRecipe::Catalog(recipe)) =
            &source.generated_identity
        else {
            unreachable!("validated generated identity has the catalog recipe")
        };
        let provider = recipe
            .provider_slots
            .first()
            .expect("validated catalog identity has one provider")
            .value
            .clone();
        let name = identifier_key(&source.name);
        Self {
            source_index,
            origin: DeclarationKey::new(SourceDeclarationKind::Identity, &name),
            name,
            ident: source.name.clone(),
            provider,
        }
    }

    pub(crate) const fn source_index(&self) -> usize {
        self.source_index
    }

    pub(crate) fn origin(&self) -> &DeclarationKey {
        &self.origin
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn ident(&self) -> &syn::Ident {
        &self.ident
    }

    pub(crate) fn provider(&self) -> &syn::Ident {
        &self.provider
    }
}

impl SignedDecimalPlan {
    fn from_source(source_index: usize, source: &crate::TerminalBinding) -> Self {
        let Some(crate::model::GeneratedCodecRecipe::SignedDecimal(recipe)) = &source.generated
        else {
            unreachable!("validated generated codec has the signed_decimal recipe")
        };
        let sign = recipe
            .sign_type_slots
            .first()
            .expect("validated signed_decimal has one sign_type");
        let positive_variant = sign
            .roles
            .iter()
            .find(|role| identifier_key(&role.variant) == "Positive")
            .expect("validated signed_decimal has a positive role")
            .variant
            .clone();
        let negative_variant = sign
            .roles
            .iter()
            .find(|role| identifier_key(&role.variant) == "Negative")
            .expect("validated signed_decimal has a negative role")
            .variant
            .clone();
        Self {
            source_index,
            origin: DeclarationKey::new(SourceDeclarationKind::Codec, identifier_key(&source.name)),
            origin_span: source.name.span(),
            codec_name: identifier_key(&source.name),
            codec_ident: source.name.clone(),
            sign_type: sign.name.clone(),
            positive_variant,
            negative_variant,
            magnitude: UnsignedPrimitive::from_source(
                recipe
                    .magnitude_slots
                    .first()
                    .expect("validated signed_decimal has one magnitude"),
            ),
        }
    }

    pub(crate) fn source_index(&self) -> usize {
        self.source_index
    }

    pub(crate) fn origin(&self) -> &DeclarationKey {
        &self.origin
    }

    pub(crate) fn origin_span(&self) -> Span {
        self.origin_span
    }

    pub(crate) fn codec_name(&self) -> &str {
        &self.codec_name
    }

    pub(crate) fn codec_ident(&self) -> &syn::Ident {
        &self.codec_ident
    }

    pub(crate) fn sign_type(&self) -> &syn::Ident {
        &self.sign_type
    }

    pub(crate) fn positive_variant(&self) -> &syn::Ident {
        &self.positive_variant
    }

    pub(crate) fn negative_variant(&self) -> &syn::Ident {
        &self.negative_variant
    }

    pub(crate) fn magnitude(&self) -> UnsignedPrimitive {
        self.magnitude
    }
}

impl UnsignedNumberPlan {
    fn from_source(source_index: usize, source: &crate::TerminalBinding) -> Self {
        let (recipe, kind) = match source
            .generated
            .as_ref()
            .expect("generated unsigned numeral recipe exists")
        {
            crate::model::GeneratedCodecRecipe::EnglishCardinal(recipe) => {
                (recipe, UnsignedNumberKind::EnglishCardinal)
            }
            crate::model::GeneratedCodecRecipe::UnsignedDecimal(recipe) => {
                (recipe, UnsignedNumberKind::UnsignedDecimal)
            }
            crate::model::GeneratedCodecRecipe::SignedDecimal(_)
            | crate::model::GeneratedCodecRecipe::DeclarationDeterminative(_)
            | crate::model::GeneratedCodecRecipe::DeclarationNoun(_)
            | crate::model::GeneratedCodecRecipe::DeclarationTerm(_)
            | crate::model::GeneratedCodecRecipe::DeclarationVerb(_)
            | crate::model::GeneratedCodecRecipe::Unsupported { .. } => {
                unreachable!("validated unsigned numeral has its closed recipe")
            }
        };
        debug_assert_eq!(recipe.magnitude_slots.len(), 1);
        Self {
            source_index,
            origin: DeclarationKey::new(SourceDeclarationKind::Codec, identifier_key(&source.name)),
            codec_name: identifier_key(&source.name),
            codec_ident: source.name.clone(),
            magnitude: UnsignedPrimitive::from_source(
                recipe
                    .magnitude_slots
                    .first()
                    .expect("validated unsigned numeral has one magnitude"),
            ),
            kind,
        }
    }

    pub(crate) fn source_index(&self) -> usize {
        self.source_index
    }

    pub(crate) fn origin(&self) -> &DeclarationKey {
        &self.origin
    }

    pub(crate) fn codec_name(&self) -> &str {
        &self.codec_name
    }

    pub(crate) fn codec_ident(&self) -> &syn::Ident {
        &self.codec_ident
    }

    pub(crate) fn magnitude(&self) -> UnsignedPrimitive {
        self.magnitude
    }

    pub(crate) fn kind(&self) -> UnsignedNumberKind {
        self.kind
    }
}

impl DeclarationNounPlan {
    fn from_source(source_index: usize, source: &crate::TerminalBinding) -> Self {
        let Some(crate::model::GeneratedCodecRecipe::DeclarationNoun(recipe)) = &source.generated
        else {
            unreachable!("validated generated codec has the declaration_noun recipe")
        };
        let closed_lexeme = recipe.closed_slots.first().map(|slot| slot.value.clone());
        let kinds = recipe
            .kind_slots
            .first()
            .expect("validated declaration_noun has one kind set")
            .kinds
            .iter()
            .map(|kind| {
                match (
                    identifier_key(&kind.kind).as_str(),
                    kind.subtype_family.as_ref().map(identifier_key),
                ) {
                    ("Type", None) => DeclarationKindFamily::Type,
                    ("TurnPart", None) => DeclarationKindFamily::TurnPart,
                    ("Subtype", None) => DeclarationKindFamily::Subtype,
                    ("Subtype", Some(family)) => DeclarationKindFamily::SubtypeFamily(match family
                        .as_str()
                    {
                        "Artifact" => crate::macro_def::SubtypeCategory::Artifact,
                        "Battle" => crate::macro_def::SubtypeCategory::Battle,
                        "Creature" => crate::macro_def::SubtypeCategory::Creature,
                        "Enchantment" => crate::macro_def::SubtypeCategory::Enchantment,
                        "Land" => crate::macro_def::SubtypeCategory::Land,
                        "Planeswalker" => crate::macro_def::SubtypeCategory::Planeswalker,
                        "Spell" => crate::macro_def::SubtypeCategory::Spell,
                        _ => unreachable!("validated declaration_noun subtype family is closed"),
                    }),
                    _ => unreachable!("validated declaration_noun kind is closed"),
                }
            })
            .collect();
        Self {
            source_index,
            origin: DeclarationKey::new(SourceDeclarationKind::Codec, identifier_key(&source.name)),
            codec_ident: source.name.clone(),
            declaration_value_ident: syn::Ident::new(
                &format!("Declaration{}", identifier_key(&source.name)),
                source.name.span(),
            ),
            closed_lexeme,
            position: crate::macro_def::GrammarPosition::Noun,
            kinds,
            feature_axis: Feature::Number,
        }
    }

    pub(crate) fn source_index(&self) -> usize {
        self.source_index
    }

    pub(crate) fn origin(&self) -> &DeclarationKey {
        &self.origin
    }

    pub(crate) fn codec_name(&self) -> &str {
        self.origin.name()
    }

    pub(crate) fn codec_ident(&self) -> &syn::Ident {
        &self.codec_ident
    }

    pub(crate) fn declaration_value_ident(&self) -> &syn::Ident {
        &self.declaration_value_ident
    }

    pub(crate) fn closed_lexeme(&self) -> Option<&syn::Ident> {
        self.closed_lexeme.as_ref()
    }

    pub(crate) fn position(&self) -> crate::macro_def::GrammarPosition {
        self.position
    }

    pub(crate) fn kinds(&self) -> &[DeclarationKindFamily] {
        &self.kinds
    }

    pub(crate) fn feature_axis(&self) -> Feature {
        self.feature_axis
    }
}

impl DeclarationDeterminativePlan {
    fn from_source(source_index: usize, source: &crate::TerminalBinding) -> Self {
        let Some(crate::model::GeneratedCodecRecipe::DeclarationDeterminative(recipe)) =
            &source.generated
        else {
            unreachable!("validated generated codec has the declaration_determinative recipe")
        };
        let closed = recipe
            .closed_slots
            .first()
            .into_iter()
            .flat_map(|slot| &slot.members)
            .map(|member| {
                let number_license = match identifier_key(&member.number_license_slots[0].value)
                    .as_str()
                {
                    "SingularOnly" => crate::macro_def::DeterminativeNumberLicense::SingularOnly,
                    "PluralOnly" => crate::macro_def::DeterminativeNumberLicense::PluralOnly,
                    "Both" => crate::macro_def::DeterminativeNumberLicense::Both,
                    _ => unreachable!("validated determiner number license is closed"),
                };
                let nominal_license = match identifier_key(&member.nominal_license_slots[0].value)
                    .as_str()
                {
                    "CountNominal" => crate::macro_def::DeterminativeNominalLicense::CountNominal,
                    "BareSingularNoun" => {
                        crate::macro_def::DeterminativeNominalLicense::BareSingularNoun
                    }
                    "AnyNominal" => crate::macro_def::DeterminativeNominalLicense::AnyNominal,
                    "MassOrPluralCount" => {
                        crate::macro_def::DeterminativeNominalLicense::MassOrPluralCount
                    }
                    _ => unreachable!("validated nominal license is closed"),
                };
                let fused_head_license =
                    match identifier_key(&member.fused_head_license_slots[0].value).as_str() {
                        "NominalOnly" => {
                            crate::macro_def::DeterminativeFusedHeadLicense::NominalOnly
                        }
                        "PartitiveOnly" => {
                            crate::macro_def::DeterminativeFusedHeadLicense::PartitiveOnly
                        }
                        "FusedHead" => crate::macro_def::DeterminativeFusedHeadLicense::FusedHead,
                        "PluralPredeterminer" => {
                            crate::macro_def::DeterminativeFusedHeadLicense::PluralPredeterminer
                        }
                        _ => unreachable!("validated fused-head license is closed"),
                    };
                let bare_duration_license =
                    match identifier_key(&member.bare_duration_license_slots[0].value).as_str() {
                        "BareDurationLicensed" => {
                            crate::macro_def::DeterminativeBareDurationLicense::BareDurationLicensed
                        }
                        "MarkerRequired" => {
                            crate::macro_def::DeterminativeBareDurationLicense::MarkerRequired
                        }
                        _ => unreachable!("validated bare-duration license is closed"),
                    };
                let realizations = member.realization_slots[0]
                    .realizations
                    .iter()
                    .map(|row| {
                        let phrase_number = row.phrase_number_slots.first().map(|slot| {
                            match identifier_key(&slot.value).as_str() {
                                "Singular" => crate::macro_def::DeterminativePhraseNumber::Singular,
                                "Plural" => crate::macro_def::DeterminativePhraseNumber::Plural,
                                _ => {
                                    unreachable!("validated determinative phrase number is closed")
                                }
                            }
                        });
                        let following_onset = row.following_onset_slots.first().map(|slot| {
                            match identifier_key(&slot.value).as_str() {
                                "Consonant" => crate::macro_def::Onset::Consonant,
                                "Vowel" => crate::macro_def::Onset::Vowel,
                                _ => unreachable!(
                                    "validated determinative following onset is closed"
                                ),
                            }
                        });
                        DeterminativeRealizationPlan {
                            surface: row.surface_slots[0].value(),
                            phrase_number,
                            following_onset,
                        }
                    })
                    .collect();
                ClosedDeterminativePlan {
                    lemma: member.lemma.clone(),
                    number_license,
                    fused_head_license,
                    nominal_license,
                    bare_duration_license,
                    realizations,
                }
            })
            .collect();
        Self {
            source_index,
            origin: DeclarationKey::new(SourceDeclarationKind::Codec, identifier_key(&source.name)),
            codec_ident: source.name.clone(),
            lemma_ident: syn::Ident::new(
                &format!("{}Lemma", identifier_key(&source.name)),
                source.name.span(),
            ),
            closed,
        }
    }

    pub(crate) fn source_index(&self) -> usize {
        self.source_index
    }
    pub(crate) fn origin(&self) -> &DeclarationKey {
        &self.origin
    }
    pub(crate) fn codec_name(&self) -> &str {
        self.origin.name()
    }
    pub(crate) fn codec_ident(&self) -> &syn::Ident {
        &self.codec_ident
    }
    pub(crate) fn lemma_ident(&self) -> &syn::Ident {
        &self.lemma_ident
    }
    pub(crate) fn closed(&self) -> &[ClosedDeterminativePlan] {
        &self.closed
    }
}

impl ClosedDeterminativePlan {
    pub(crate) fn lemma(&self) -> &syn::Ident {
        &self.lemma
    }
    pub(crate) fn number_license(&self) -> crate::macro_def::DeterminativeNumberLicense {
        self.number_license
    }
    pub(crate) fn fused_head_license(&self) -> crate::macro_def::DeterminativeFusedHeadLicense {
        self.fused_head_license
    }
    pub(crate) fn nominal_license(&self) -> crate::macro_def::DeterminativeNominalLicense {
        self.nominal_license
    }
    pub(crate) fn bare_duration_license(
        &self,
    ) -> crate::macro_def::DeterminativeBareDurationLicense {
        self.bare_duration_license
    }
    pub(crate) fn realizations(&self) -> &[DeterminativeRealizationPlan] {
        &self.realizations
    }
}

impl DeterminativeRealizationPlan {
    pub(crate) fn surface(&self) -> &str {
        &self.surface
    }
    pub(crate) fn phrase_number(&self) -> Option<crate::macro_def::DeterminativePhraseNumber> {
        self.phrase_number
    }
    pub(crate) fn following_onset(&self) -> Option<crate::macro_def::Onset> {
        self.following_onset
    }
}

impl DeclarationTermPlan {
    fn from_source(source_index: usize, source: &crate::TerminalBinding) -> Self {
        let Some(crate::model::GeneratedCodecRecipe::DeclarationTerm(recipe)) = &source.generated
        else {
            unreachable!("validated generated codec has the declaration_term recipe")
        };
        let position = match identifier_key(
            &recipe
                .position_slots
                .first()
                .expect("validated declaration_term has one position")
                .value,
        )
        .as_str()
        {
            "FixedTerm" => crate::macro_def::GrammarPosition::FixedTerm,
            "FixedKeyword" => crate::macro_def::GrammarPosition::FixedKeyword,
            _ => unreachable!("validated declaration_term position is closed"),
        };
        let kinds = recipe
            .kind_slots
            .first()
            .expect("validated declaration_term has one kind set")
            .kinds
            .iter()
            .map(|kind| match identifier_key(kind).as_str() {
                "KeywordAbility" => crate::macro_def::DeclarationKind::KeywordAbility,
                "AbilityWord" => crate::macro_def::DeclarationKind::AbilityWord,
                "FlavorWord" => crate::macro_def::DeclarationKind::FlavorWord,
                "CounterKind" => crate::macro_def::DeclarationKind::CounterKind,
                "Designation" => crate::macro_def::DeclarationKind::Designation,
                _ => unreachable!("validated declaration_term kind is closed"),
            })
            .collect();
        let params = if recipe.param_policy_slots.is_empty() {
            Some(
                recipe
                    .param_slots
                    .first()
                    .map(|slot| slot.kinds.iter().map(identifier_key).collect())
                    .unwrap_or_default(),
            )
        } else {
            None
        };
        let feature =
            recipe
                .feature_slots
                .first()
                .map_or(
                    crate::macro_def::SurfaceFeature::Fixed,
                    |slot| match identifier_key(&slot.value).as_str() {
                        "Fixed" => crate::macro_def::SurfaceFeature::Fixed,
                        "Participle" => crate::macro_def::SurfaceFeature::PAST_PARTICIPLE,
                        "BlockLabel" => crate::macro_def::SurfaceFeature::BlockLabel,
                        _ => unreachable!("validated declaration_term feature is closed"),
                    },
                );
        Self {
            source_index,
            origin: DeclarationKey::new(SourceDeclarationKind::Codec, identifier_key(&source.name)),
            codec_ident: source.name.clone(),
            position,
            kinds,
            params,
            feature,
        }
    }

    pub(crate) fn source_index(&self) -> usize {
        self.source_index
    }

    pub(crate) fn origin(&self) -> &DeclarationKey {
        &self.origin
    }

    pub(crate) fn codec_name(&self) -> &str {
        self.origin.name()
    }

    pub(crate) fn codec_ident(&self) -> &syn::Ident {
        &self.codec_ident
    }

    pub(crate) fn position(&self) -> crate::macro_def::GrammarPosition {
        self.position
    }

    pub(crate) fn kinds(&self) -> &[crate::macro_def::DeclarationKind] {
        &self.kinds
    }

    pub(crate) fn params(&self) -> Option<&[String]> {
        self.params.as_deref()
    }

    pub(crate) fn feature(&self) -> crate::macro_def::SurfaceFeature {
        self.feature
    }
}

impl DeclarationVerbPlan {
    fn from_source(source_index: usize, source: &crate::TerminalBinding) -> Self {
        let Some(crate::model::GeneratedCodecRecipe::DeclarationVerb(recipe)) = &source.generated
        else {
            unreachable!("validated generated codec has the declaration_verb recipe")
        };
        let frame_key = VerbFrameKey {
            class: recipe
                .class_slots
                .first()
                .map_or(VerbFrameClass::Predicate, |slot| {
                    match identifier_key(&slot.value).as_str() {
                        "Predicate" => VerbFrameClass::Predicate,
                        "Auxiliary" => VerbFrameClass::Auxiliary,
                        "ProVerb" => VerbFrameClass::ProVerb,
                        _ => unreachable!("validated declaration_verb class is sealed"),
                    }
                }),
            atoms: recipe
                .tail_slots
                .first()
                .expect("validated declaration_verb has one tail")
                .atoms
                .iter()
                .map(|atom| match &atom.kind {
                    crate::model::DeclarationVerbTailAtomKindSource::Literal(literal) => {
                        VerbFrameAtom::Literal(literal.value())
                    }
                    crate::model::DeclarationVerbTailAtomKindSource::Lex(path) => {
                        let segments = path.segments.iter().collect::<Vec<_>>();
                        let terminal = identifier_key(&segments[0].ident);
                        let variant = identifier_key(&segments[1].ident);
                        if atom.optional {
                            VerbFrameAtom::OptionalLex(terminal, variant)
                        } else {
                            VerbFrameAtom::Lex(terminal, variant)
                        }
                    }
                    crate::model::DeclarationVerbTailAtomKindSource::Marked { marker, role } => {
                        let segments = marker.segments.iter().collect::<Vec<_>>();
                        let terminal = identifier_key(&segments[0].ident);
                        let variant = identifier_key(&segments[1].ident);
                        let role = identifier_key(role);
                        if atom.optional {
                            VerbFrameAtom::OptionalMarkedRole(terminal, variant, role)
                        } else {
                            VerbFrameAtom::MarkedRole(terminal, variant, role)
                        }
                    }
                    crate::model::DeclarationVerbTailAtomKindSource::Amount(ident) => {
                        if atom.optional {
                            VerbFrameAtom::OptionalRole(identifier_key(ident))
                        } else {
                            VerbFrameAtom::Amount
                        }
                    }
                    crate::model::DeclarationVerbTailAtomKindSource::ObjectNounPhrase(ident) => {
                        if atom.optional {
                            VerbFrameAtom::OptionalRole(identifier_key(ident))
                        } else {
                            VerbFrameAtom::ObjectNounPhrase
                        }
                    }
                    crate::model::DeclarationVerbTailAtomKindSource::PredicativeComplement(
                        ident,
                    ) => {
                        if atom.optional {
                            VerbFrameAtom::OptionalRole(identifier_key(ident))
                        } else {
                            VerbFrameAtom::PredicativeComplement
                        }
                    }
                    crate::model::DeclarationVerbTailAtomKindSource::FrameComplementPair(_) => {
                        VerbFrameAtom::FrameComplementPair
                    }
                    crate::model::DeclarationVerbTailAtomKindSource::Role(role) => {
                        if atom.optional {
                            VerbFrameAtom::OptionalRole(identifier_key(role))
                        } else {
                            VerbFrameAtom::Role(identifier_key(role))
                        }
                    }
                })
                .collect(),
        };
        Self {
            source_index,
            origin: DeclarationKey::new(SourceDeclarationKind::Codec, identifier_key(&source.name)),
            codec_ident: source.name.clone(),
            declaration_value_ident: syn::Ident::new(
                &format!("Declaration{}", identifier_key(&source.name)),
                source.name.span(),
            ),
            closed_lexeme: recipe.closed_slots.first().map(|slot| slot.value.clone()),
            frame_key,
            feature_axis: match identifier_key(
                &recipe
                    .feature_slots
                    .first()
                    .expect("validated declaration_verb has one feature")
                    .value,
            )
            .as_str()
            {
                "ConcordClass" => Feature::ConcordClass,
                "Participle" => Feature::Participle,
                _ => unreachable!("validated declaration_verb feature is closed"),
            },
        }
    }

    pub(crate) fn source_index(&self) -> usize {
        self.source_index
    }

    pub(crate) fn origin(&self) -> &DeclarationKey {
        &self.origin
    }

    pub(crate) fn codec_name(&self) -> &str {
        self.origin.name()
    }

    pub(crate) fn codec_ident(&self) -> &syn::Ident {
        &self.codec_ident
    }

    pub(crate) fn declaration_value_ident(&self) -> &syn::Ident {
        &self.declaration_value_ident
    }

    pub(crate) fn closed_lexeme(&self) -> Option<&syn::Ident> {
        self.closed_lexeme.as_ref()
    }

    #[cfg(test)]
    pub(crate) const fn position() -> crate::macro_def::GrammarPosition {
        crate::macro_def::GrammarPosition::Verb
    }

    pub(crate) fn frame_key(&self) -> &VerbFrameKey {
        &self.frame_key
    }

    pub(crate) fn feature_axis(&self) -> Feature {
        self.feature_axis
    }
}

impl BindingPlan {
    fn from_source(source_index: usize, source: &crate::TerminalBinding) -> syn::Result<Self> {
        let render = source.render.as_ref().map(|render| match render {
            crate::model::RenderBinding::Runtime(path) => BindingRenderPlan::Runtime(path.clone()),
            crate::model::RenderBinding::ContextIdentity(arms) => {
                BindingRenderPlan::ContextIdentity(
                    arms.iter()
                        .map(|arm| ContextIdentityArmPlan {
                            variant: arm.variant.clone(),
                            accessor: arm.accessor.clone(),
                        })
                        .collect(),
                )
            }
        });
        let build = source
            .build
            .as_ref()
            .map(BindingBuildPlan::from_source)
            .transpose()?;
        let value_type_name = binding_value_type_name(&source.value_type)?;
        let mode = source.traversal.callback_mode.ok_or_else(|| {
            syn::Error::new(
                source.name.span(),
                "sealed terminal traversal mode is absent",
            )
        })?;
        let argument = source.traversal.argument.as_ref().map_or_else(
            || default_leaf_argument(&identifier_key(&source.name)),
            identifier_key,
        );
        let fields = source
            .traversal
            .fields
            .iter()
            .map(|field| TraversalFieldPlan {
                name: field.name.clone(),
            })
            .collect();
        let recipe = if !source.traversal.branches.is_empty() {
            BindingTraversalRecipe::Branches(
                source
                    .traversal
                    .branches
                    .iter()
                    .map(TraversalBranchPlan::from_source)
                    .collect::<syn::Result<Vec<_>>>()?,
            )
        } else if !source.traversal.variants.is_empty() {
            BindingTraversalRecipe::Variants(source.traversal.variants.clone())
        } else {
            BindingTraversalRecipe::Calls(
                source
                    .traversal
                    .calls
                    .iter()
                    .map(TraversalCallPlan::from_source)
                    .collect::<syn::Result<Vec<_>>>()?,
            )
        };
        let leaf_callbacks = source
            .traversal
            .leaf_callbacks
            .iter()
            .map(|leaf| LeafCallbackPlan {
                name: leaf.name.clone(),
                value_type: leaf.value_type.clone(),
                mode: leaf.mode,
            })
            .collect();
        Ok(Self {
            source_index,
            origin: DeclarationKey::new(
                match source.kind {
                    crate::model::TerminalBindingKind::Codec => SourceDeclarationKind::Codec,
                    crate::model::TerminalBindingKind::Identity => SourceDeclarationKind::Identity,
                },
                identifier_key(&source.name),
            ),
            origin_span: source.name.span(),
            name: source.name.clone(),
            name_key: identifier_key(&source.name),
            kind: source.kind,
            codec_atom: source.codec_atom,
            value_type_name,
            lexical_variant: source.lexical_variant.clone(),
            stored_spelling: matches!(&render, Some(BindingRenderPlan::ContextIdentity(arms)) if arms.len() >= 2),
            render,
            build,
            traversal: BindingTraversalPlan {
                mode,
                argument,
                fields,
                recipe,
                leaf_callbacks,
            },
            declaration_verb: None,
        })
    }

    fn from_declaration_verb(source_index: usize, source: &crate::TerminalBinding) -> Self {
        Self {
            source_index,
            origin: DeclarationKey::new(SourceDeclarationKind::Codec, identifier_key(&source.name)),
            origin_span: source.name.span(),
            name: source.name.clone(),
            name_key: identifier_key(&source.name),
            kind: source.kind,
            codec_atom: source.codec_atom,
            value_type_name: source.name.clone(),
            lexical_variant: source.lexical_variant.clone(),
            render: None,
            build: None,
            traversal: BindingTraversalPlan {
                mode: crate::model::VisitMode::Borrowed,
                argument: default_leaf_argument(&identifier_key(&source.name)),
                fields: Vec::new(),
                recipe: BindingTraversalRecipe::Calls(Vec::new()),
                leaf_callbacks: Vec::new(),
            },
            stored_spelling: false,
            declaration_verb: Some(DeclarationVerbPlan::from_source(source_index, source)),
        }
    }

    pub(crate) fn name(&self) -> &str {
        &self.name_key
    }

    pub(crate) fn kind(&self) -> crate::model::TerminalBindingKind {
        self.kind
    }

    pub(crate) fn codec_atom(&self) -> Option<crate::model::CodecAtomClass> {
        self.codec_atom
    }

    pub(crate) fn value_type_name(&self) -> &syn::Ident {
        &self.value_type_name
    }

    pub(crate) fn lexical_variant(&self) -> Option<&syn::Path> {
        self.lexical_variant.as_ref()
    }

    pub(crate) fn render(&self) -> Option<&BindingRenderPlan> {
        self.render.as_ref()
    }

    pub(crate) fn build(&self) -> Option<&BindingBuildPlan> {
        self.build.as_ref()
    }

    pub(crate) fn traversal(&self) -> &BindingTraversalPlan {
        &self.traversal
    }

    pub(crate) fn has_stored_spelling(&self) -> bool {
        self.stored_spelling
    }

    pub(crate) fn declaration_verb(&self) -> Option<&DeclarationVerbPlan> {
        self.declaration_verb.as_ref()
    }

    pub(crate) fn origin(&self) -> &DeclarationKey {
        &self.origin
    }

    pub(crate) fn origin_span(&self) -> Span {
        self.origin_span
    }
    #[allow(
        dead_code,
        reason = "future emitters retain source order through this index"
    )]
    pub(crate) fn source_index(&self) -> usize {
        self.source_index
    }
}

impl VocabVariantPlan {
    pub(crate) fn name(&self) -> &syn::Ident {
        &self.name
    }

    pub(crate) fn word(&self) -> &syn::LitStr {
        &self.word
    }

    pub(crate) const fn onset(&self) -> crate::macro_def::Onset {
        self.onset
    }
}

impl BindingBuildPlan {
    fn from_source(source: &crate::model::BuildLeaf) -> syn::Result<Self> {
        let syn::Pat::TupleStruct(pattern) = &source.pattern else {
            return Err(syn::Error::new_spanned(
                &source.pattern,
                "sealed binding build pattern kind is inconsistent",
            ));
        };
        let variant = pattern
            .path
            .segments
            .last()
            .ok_or_else(|| {
                syn::Error::new_spanned(
                    &source.pattern,
                    "sealed binding build pattern path is empty",
                )
            })?
            .ident
            .clone();
        let slots = pattern
            .elems
            .iter()
            .map(|slot| match slot {
                syn::Pat::Ident(slot) => Ok(slot.ident.clone()),
                _ => Err(syn::Error::new_spanned(
                    slot,
                    "sealed binding build pattern slot kind is inconsistent",
                )),
            })
            .collect::<syn::Result<Vec<_>>>()?;
        let construct = BindingBuildExprPlan::from_source(&source.construct, true)?;
        let construct_is_direct_slot = construct.is_direct_slot();
        Ok(Self {
            variant,
            slots,
            construct,
            construct_is_direct_slot,
        })
    }

    pub(crate) fn variant(&self) -> &syn::Ident {
        &self.variant
    }

    pub(crate) fn slots(&self) -> &[syn::Ident] {
        &self.slots
    }

    pub(crate) fn recipe(&self) -> &BindingBuildExprPlan {
        &self.construct
    }

    pub(crate) fn construct_is_direct_slot(&self) -> bool {
        self.construct_is_direct_slot
    }
}

impl BindingBuildExprPlan {
    fn from_source(source: &syn::Expr, allow_calls: bool) -> syn::Result<Self> {
        match source {
            syn::Expr::Path(path) if path.qself.is_none() && path.path.segments.len() == 1 => {
                Ok(Self::Slot(identifier_key(&path.path.segments[0].ident)))
            }
            syn::Expr::Field(field) => Ok(Self::Field {
                base: Box::new(Self::from_source(&field.base, false)?),
                member: field.member.clone(),
            }),
            syn::Expr::Call(call) if allow_calls => {
                let syn::Expr::Path(function) = &*call.func else {
                    return Err(syn::Error::new_spanned(
                        &call.func,
                        "sealed binding build call target kind is inconsistent",
                    ));
                };
                if function.qself.is_some() || function.path.segments.len() <= 1 {
                    return Err(syn::Error::new_spanned(
                        &call.func,
                        "sealed binding build call target path is inconsistent",
                    ));
                }
                Ok(Self::Call {
                    function: function.path.clone(),
                    arguments: call
                        .args
                        .iter()
                        .map(|argument| Self::from_source(argument, true))
                        .collect::<syn::Result<Vec<_>>>()?,
                })
            }
            syn::Expr::Paren(paren) => Ok(Self::Parenthesized(Box::new(Self::from_source(
                &paren.expr,
                allow_calls,
            )?))),
            _ => Err(syn::Error::new_spanned(
                source,
                "sealed binding build expression kind is inconsistent",
            )),
        }
    }

    fn is_direct_slot(&self) -> bool {
        match self {
            Self::Slot(_) => true,
            Self::Parenthesized(inner) => inner.is_direct_slot(),
            Self::Field { .. } | Self::Call { .. } => false,
        }
    }
}

impl ContextIdentityArmPlan {
    pub(crate) fn variant(&self) -> &syn::Ident {
        &self.variant
    }

    pub(crate) fn accessor(&self) -> &syn::Ident {
        &self.accessor
    }
}

impl BindingTraversalPlan {
    pub(crate) fn mode(&self) -> crate::model::VisitMode {
        self.mode
    }

    pub(crate) fn argument(&self) -> &str {
        &self.argument
    }

    pub(crate) fn fields(&self) -> &[TraversalFieldPlan] {
        &self.fields
    }

    pub(crate) fn recipe(&self) -> &BindingTraversalRecipe {
        &self.recipe
    }

    pub(crate) fn leaf_callbacks(&self) -> &[LeafCallbackPlan] {
        &self.leaf_callbacks
    }
}

impl TraversalCallPlan {
    fn from_source(source: &crate::model::TraversalCall) -> syn::Result<Self> {
        Ok(Self {
            callback: source.callback.clone(),
            mode: source.mode,
            value: TraversalValuePlan::from_source(&source.value)?,
        })
    }

    pub(crate) fn callback(&self) -> &syn::Path {
        &self.callback
    }

    pub(crate) fn mode(&self) -> crate::model::VisitMode {
        self.mode
    }

    pub(crate) fn value(&self) -> &TraversalValuePlan {
        &self.value
    }

    pub(crate) fn mentions(&self, name: &str) -> bool {
        self.value.mentions(name)
    }
}

impl TraversalFieldPlan {
    pub(crate) fn name(&self) -> &syn::Ident {
        &self.name
    }
}

impl TraversalBranchPlan {
    fn from_source(source: &crate::model::TraversalBranch) -> syn::Result<Self> {
        Ok(Self {
            value: TraversalValuePlan::from_source(&source.value)?,
            arms: source
                .arms
                .iter()
                .map(TraversalBranchArmPlan::from_source)
                .collect::<syn::Result<Vec<_>>>()?,
        })
    }

    pub(crate) fn value(&self) -> &TraversalValuePlan {
        &self.value
    }

    pub(crate) fn arms(&self) -> &[TraversalBranchArmPlan] {
        &self.arms
    }
}

impl TraversalBranchArmPlan {
    fn from_source(source: &crate::model::TraversalBranchArm) -> syn::Result<Self> {
        Ok(Self {
            variant: source.variant.clone(),
            binding: source.binding.clone(),
            call: TraversalCallPlan::from_source(&source.call)?,
        })
    }

    pub(crate) fn variant(&self) -> &syn::Path {
        &self.variant
    }

    pub(crate) fn binding(&self) -> &syn::Ident {
        &self.binding
    }

    pub(crate) fn call(&self) -> &TraversalCallPlan {
        &self.call
    }
}

impl TraversalValuePlan {
    fn from_source(source: &syn::Expr) -> syn::Result<Self> {
        match source {
            syn::Expr::Path(path)
                if path.qself.is_none()
                    && path.path.leading_colon.is_none()
                    && path.path.segments.len() == 1 =>
            {
                let root =
                    path.path.segments.first().ok_or_else(|| {
                        syn::Error::new(source.span(), "traversal root is absent")
                    })?;
                Ok(Self::Root(identifier_key(&root.ident)))
            }
            syn::Expr::Field(field) => Ok(Self::Field {
                base: Box::new(Self::from_source(&field.base)?),
                member: field.member.clone(),
            }),
            syn::Expr::Paren(paren) => Ok(Self::Parenthesized(Box::new(Self::from_source(
                &paren.expr,
            )?))),
            _ => Err(syn::Error::new(
                source.span(),
                "sealed traversal value is outside the validated field-path grammar",
            )),
        }
    }

    pub(crate) fn mentions(&self, name: &str) -> bool {
        match self {
            Self::Root(root) => root == name,
            Self::Field { base, .. } | Self::Parenthesized(base) => base.mentions(name),
        }
    }
}

impl LeafCallbackPlan {
    pub(crate) fn name(&self) -> &syn::Ident {
        &self.name
    }

    pub(crate) fn value_type(&self) -> &syn::Type {
        &self.value_type
    }

    pub(crate) fn mode(&self) -> crate::model::VisitMode {
        self.mode
    }
}

fn binding_value_type_name(value_type: &syn::Type) -> syn::Result<syn::Ident> {
    let syn::Type::Path(value_type) = value_type else {
        return Err(syn::Error::new_spanned(
            value_type,
            "sealed binding value type kind is inconsistent",
        ));
    };
    value_type
        .path
        .segments
        .last()
        .map(|segment| segment.ident.clone())
        .ok_or_else(|| {
            syn::Error::new_spanned(value_type, "sealed binding value type path is empty")
        })
}

fn default_leaf_argument(name: &str) -> String {
    let snake = snake_case(name);
    if snake.ends_with("_word") {
        "word".to_owned()
    } else if snake.ends_with("_spelling") {
        "spelling".to_owned()
    } else if let Some(prefix) = snake.strip_suffix("_lexeme") {
        prefix.to_owned()
    } else if snake.ends_with("_identity") {
        "identity".to_owned()
    } else if snake.ends_with("_number") {
        "number".to_owned()
    } else {
        snake
    }
}

impl RootPlan {
    #[allow(
        dead_code,
        reason = "future emitters retain source order through this index"
    )]
    pub(crate) fn source_index(&self) -> usize {
        self.source_index
    }

    pub(crate) fn category(&self) -> &str {
        &self.category
    }

    pub(crate) fn is_parse_entry(&self) -> bool {
        self.parse_entry
    }

    pub(crate) fn is_render_entry(&self) -> bool {
        self.render_entry
    }

    pub(crate) fn punctuation(&self) -> &str {
        &self.punctuation
    }
}

#[cfg(test)]
mod tests {
    use super::EdgeClass;
    use super::FixedSurfaceAtomPlan;
    use super::FixedSurfacePlan;
    use super::LengthBounds;
    use super::PositionalSeparatorPlan;
    use super::ProductPlan;
    use super::SeparatorPlan;
    use super::StructuralFieldKindPlan;
    use super::ValueKindPlan;

    fn guarded_form_result(tokens: proc_macro2::TokenStream) -> syn::Result<super::SemanticPlan> {
        crate::validate_declarations(crate::parse_declarations(tokens)?)
            .map(crate::ValidatedDeclarations::into_semantic)
    }

    #[test]
    fn guarded_forms_seal_disjoint_domains_and_fallback_complement() {
        let semantic = guarded_form_result(quote::quote! {
            vocab Word { That = "that", Those = "those", Other = "other", }
            construction demonstrative: NounPhrase {
                element Demonstrative { word: lex Word, }
                form that when word in [That] = lex(word);
                form those when word in [Those] = lex(word);
                form fallback otherwise = lex(word);
            }
            root NounPhrase { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("disjoint guarded forms seal");
        let construction = &semantic.constructions()[0];

        assert_eq!(construction.construction_id(), "demonstrative");
        assert_eq!(construction.category_variant(), "Demonstrative");
        assert_eq!(construction.forms().len(), 3);
        assert_eq!(construction.forms()[0].name(), "that");
        assert_eq!(
            construction.forms()[0].rule_id(),
            "NounPhraseDemonstrativeThat"
        );
        assert_eq!(construction.forms()[1].name(), "those");
        assert_eq!(
            construction.forms()[1].rule_id(),
            "NounPhraseDemonstrativeThose"
        );
        assert_eq!(construction.forms()[2].name(), "fallback");
        assert_eq!(
            construction.forms()[2].rule_id(),
            "NounPhraseDemonstrativeFallback"
        );
        assert_eq!(
            construction
                .forms()
                .iter()
                .map(|form| form.guard().test_accepting_witnesses(construction.forms()))
                .collect::<Vec<_>>(),
            [
                vec!["word=That".to_owned()],
                vec!["word=Those".to_owned()],
                vec!["word=Other".to_owned()],
            ]
        );
    }

    #[test]
    fn circumfix_plan_seals_two_boundaries_around_one_delegated_payload() {
        let semantic = guarded_form_result(quote::quote! {
            construction item: Item {
                element ItemValue {}
                form item = "item";
            }
            construction singular: Root {
                element Singular { value: Item, }
                form singular = circumfix("[", value, "]");
            }
            construction sequence: Root {
                element Sequence { values: seq Item separated by "}{", }
                require len(values) >= 1;
                form sequence = circumfix("{", values, "}");
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("circumfix semantic fixture seals");

        let singular = &semantic.constructions()[1].forms()[0].atoms()[0];
        assert!(matches!(
            singular,
            super::AtomPlan::Circumfix {
                prefix,
                value,
                suffix,
            } if prefix == "["
                && suffix == "]"
                && matches!(value.as_ref(), super::AtomPlan::Category { role, category } if role == "value" && category == "Item")
        ));
        let sequence = &semantic.constructions()[2];
        assert!(matches!(
            &sequence.forms()[0].atoms()[0],
            super::AtomPlan::Circumfix {
                prefix,
                value,
                suffix,
            } if prefix == "{"
                && suffix == "}"
                && matches!(value.as_ref(), super::AtomPlan::Category { role, category } if role == "values" && category == "Item")
        ));
        assert!(matches!(
            sequence.fields()[0]
                .structural_plan()
                .expect("sequence payload retains its structural authority")
                .kind(),
            StructuralFieldKindPlan::Sequence {
                item: ValueKindPlan::Category(item),
                bounds,
                surface,
            } if item == "Item"
                && *bounds == LengthBounds::new(1, None)
                && matches!(surface.separator(), Some(SeparatorPlan::Uniform(separator)) if matches!(separator.atoms(), [FixedSurfaceAtomPlan::Literal(value)] if value == "}{"))
        ));
    }

    #[test]
    fn guarded_forms_reject_overlap_with_the_first_deterministic_witness() {
        let error = guarded_form_result(quote::quote! {
            vocab Word { That = "that", Those = "those", }
            construction demonstrative: NounPhrase {
                element Demonstrative { word: lex Word, }
                form broad when word in [That, Those] = lex(word);
                form narrow when word is Those = lex(word);
                form fallback otherwise = lex(word);
            }
            root NounPhrase { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect_err("overlapping forms do not seal")
        .into_compile_error()
        .to_string();

        assert!(error.contains("broad"), "{error}");
        assert!(error.contains("narrow"), "{error}");
        assert!(error.contains("word=Those"), "{error}");
    }

    #[test]
    fn guarded_forms_reject_an_unreachable_fallback() {
        let error = guarded_form_result(quote::quote! {
            vocab Word { That = "that", Those = "those", }
            construction demonstrative: NounPhrase {
                element Demonstrative { word: lex Word, }
                form that when word is That = lex(word);
                form those when word is Those = lex(word);
                form fallback otherwise = lex(word);
            }
            root NounPhrase { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect_err("a fallback outside an exhaustive prior partition is unreachable")
        .to_string();

        assert_eq!(
            error,
            "fallback form `fallback` is unreachable because prior guards exhaust its finite domain",
        );
    }

    #[test]
    fn guarded_forms_reject_unsatisfiable_unknown_and_wrong_kind_predicates() {
        let cases = [
            (
                quote::quote! {
                    vocab Word { That = "that", Those = "those", }
                    construction demonstrative: NounPhrase {
                        element Demonstrative { word: lex Word, }
                        form impossible when all(word is That, word is Those) = lex(word);
                        form fallback otherwise = lex(word);
                    }
                    root NounPhrase { punctuation = "."; eoi = true; standalone_render = true; }
                },
                "unsatisfiable",
            ),
            (
                quote::quote! {
                    vocab Word { That = "that", }
                    construction demonstrative: NounPhrase {
                        element Demonstrative { word: lex Word, }
                        form selected when missing is That = lex(word);
                        form fallback otherwise = lex(word);
                    }
                    root NounPhrase { punctuation = "."; eoi = true; standalone_render = true; }
                },
                "unknown form-guard role `missing`",
            ),
            (
                quote::quote! {
                    vocab Word { That = "that", }
                    construction demonstrative: NounPhrase {
                        element Demonstrative { word: lex Word, }
                        form selected when word is Missing = lex(word);
                        form fallback otherwise = lex(word);
                    }
                    root NounPhrase { punctuation = "."; eoi = true; standalone_render = true; }
                },
                "unknown member `Missing`",
            ),
            (
                quote::quote! {
                    construction child: Child { element ChildNode {} form child = "child"; }
                    construction parent: Parent {
                        element ParentNode { child: Child, }
                        form selected when child is Child = child;
                        form fallback otherwise = child;
                    }
                    root Parent { punctuation = "."; eoi = true; standalone_render = true; }
                },
                "form guard membership requires a vocab role",
            ),
        ];

        for (tokens, expected) in cases {
            let error = guarded_form_result(tokens)
                .expect_err("invalid guard does not seal")
                .into_compile_error()
                .to_string();
            assert!(error.contains(expected), "expected `{expected}` in {error}");
        }
    }

    #[test]
    fn optional_presence_guards_seal_both_assignments() {
        let semantic = guarded_form_result(quote::quote! {
            vocab Word { That = "that", Those = "those", }
            construction optional: NounPhrase {
                element OptionalWord { word: opt lex Word, }
                form present when word.is_some() = lex(word);
                form absent otherwise = lex(word);
            }
            root NounPhrase { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("optional-presence partition seals");
        let construction = &semantic.constructions()[0];

        assert_eq!(
            construction
                .forms()
                .iter()
                .map(|form| form.guard().test_accepting_witnesses(construction.forms()))
                .collect::<Vec<_>>(),
            [
                vec!["word=That".to_owned(), "word=Those".to_owned()],
                vec!["word=absent".to_owned()],
            ]
        );
    }

    #[test]
    fn optional_vocab_guards_partition_absence_and_every_vocab_member() {
        let semantic = guarded_form_result(quote::quote! {
            vocab Word { That = "that", Those = "those", }
            construction optional: NounPhrase {
                element OptionalWord { word: opt lex Word, }
                form that when word is That = lex(word);
                form fallback otherwise = lex(word);
            }
            root NounPhrase { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("optional vocab membership remains a finite guarded domain");
        let construction = &semantic.constructions()[0];

        assert_eq!(
            construction
                .forms()
                .iter()
                .map(|form| form.guard().test_accepting_witnesses(construction.forms()))
                .collect::<Vec<_>>(),
            [
                vec!["word=That".to_owned()],
                vec!["word=Those".to_owned(), "word=absent".to_owned()],
            ]
        );
    }

    #[test]
    #[allow(
        clippy::too_many_lines,
        reason = "one fixture asserts the mutually recursive structural plan as a whole"
    )]
    fn structural_plan_resolves_bounds_surfaces_nullability_and_recursive_edges() {
        let semantic = crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
                construction node: Node {
                    element NodeValue {}
                    form node = "node";
                }
                construction document: Document {
                    element DocumentValue { nodes: seq Node terminated by ".", }
                    require len(nodes) >= 1;
                    form document = nodes;
                }
                vocab Word { One = "one", }
                construction emptyable: Emptyable {
                    element EmptyableValue { maybe: opt lex Word, }
                    form emptyable = lex(maybe);
                }
                construction chain: Chain {
                    element ChainValue { rest: seq Chain terminated by ".", }
                    require len(rest) = 1;
                    form chain = rest;
                }
                abstract sum Choice { node: Node, }
                abstract sum RecursiveChoice { branch: RecursiveBranch, }
                abstract product Holder {
                    maybe: opt Node,
                    items: seq Choice
                        separated by position {
                            pair = " and ";
                            first = ", ";
                            middle = ", ";
                            last = ", and ";
                        }
                        terminated by ".",
                }
                require len(Holder.items) >= 2;
                require len(Holder.items) <= 4;
                abstract product Tree { children: seq Tree terminated by ".", }
                require len(Tree.children) = 1;
                abstract product RecursiveBranch {
                    choices: seq RecursiveChoice terminated by ".",
                }
                require len(RecursiveBranch.choices) = 1;
                root Node { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect("structural semantic fixture parses"),
        )
        .expect("structural semantic fixture validates")
        .into_semantic();

        assert_eq!(
            semantic
                .products()
                .iter()
                .map(ProductPlan::name)
                .collect::<Vec<_>>(),
            ["Holder", "Tree", "RecursiveBranch"]
        );
        let holder = &semantic.products()[0];
        assert_eq!(holder.bounds("items"), Some(LengthBounds::new(2, Some(4))));
        assert!(!holder.is_nullable());
        assert!(matches!(
            holder.fields()[0].kind(),
            StructuralFieldKindPlan::Optional(ValueKindPlan::Category(name)) if name == "Node"
        ));
        let StructuralFieldKindPlan::Sequence {
            item,
            bounds,
            surface,
        } = holder.fields()[1].kind()
        else {
            panic!("Holder.items is a sealed sequence");
        };
        assert_eq!(item, &ValueKindPlan::Sum("Choice".to_owned()));
        assert_eq!(*bounds, LengthBounds::new(2, Some(4)));
        let Some(SeparatorPlan::Positional(rows)) = surface.separator() else {
            panic!("Holder.items has a positional separator table");
        };
        assert_eq!(
            rows.iter()
                .map(PositionalSeparatorPlan::class)
                .collect::<Vec<_>>(),
            [
                EdgeClass::Pair,
                EdgeClass::First,
                EdgeClass::Middle,
                EdgeClass::Last
            ]
        );
        assert!(rows.iter().all(|row| !row.surface().is_empty()));
        assert!(matches!(
            surface.terminator().map(FixedSurfacePlan::atoms),
            Some([FixedSurfaceAtomPlan::Literal(mark)]) if mark == "."
        ));
        assert_eq!(
            holder.fields()[1]
                .helper_names()
                .expect("sequence helper inventory")
                .all(),
            [
                "HolderItemsSequence",
                "HolderItemsSequenceCategory",
                "HolderItemsSequenceRule",
                "build_holder_items_sequence",
                "render_holder_items_sequence",
                "walk_holder_items_sequence",
            ]
        );
        assert_eq!(semantic.sums()[0].name(), "Choice");
        assert!(!semantic.sums()[0].is_nullable());
        assert_eq!(semantic.sums()[0].alternatives()[0].name(), "node");
        assert_eq!(
            semantic.sums()[0].alternatives()[0].value(),
            &ValueKindPlan::Category("Node".to_owned())
        );
        assert!(semantic.sums()[1].alternatives()[0].is_recursive());
        assert!(semantic.products()[1].fields()[0].is_recursive());
        assert!(!semantic.is_nullable("Holder"));
        assert!(!semantic.is_nullable("Choice"));
        let document = &semantic.constructions()[1];
        assert!(!document.is_nullable());
        let document_nodes = document.fields()[0]
            .structural_plan()
            .expect("construction structural field facts are sealed");
        assert!(matches!(
            document_nodes.kind(),
            StructuralFieldKindPlan::Sequence {
                item: ValueKindPlan::Category(name),
                bounds,
                ..
            } if name == "Node" && *bounds == LengthBounds::new(1, None)
        ));
        assert_eq!(
            document_nodes
                .helper_names()
                .expect("construction sequence helper inventory")
                .all(),
            [
                "DocumentValueNodesSequence",
                "DocumentValueNodesSequenceCategory",
                "DocumentValueNodesSequenceRule",
                "build_document_value_nodes_sequence",
                "render_document_value_nodes_sequence",
                "walk_document_value_nodes_sequence",
            ]
        );
        assert!(semantic.constructions()[2].is_nullable());
        let chain_rest = semantic.constructions()[3].fields()[0]
            .structural_plan()
            .expect("recursive construction field facts are sealed");
        assert!(chain_rest.is_recursive());
        assert_eq!(
            chain_rest
                .helper_names()
                .expect("recursive construction helper inventory")
                .all(),
            [
                "ChainValueRestSequence",
                "ChainValueRestSequenceCategory",
                "ChainValueRestSequenceRule",
                "build_chain_value_rest_sequence",
                "render_chain_value_rest_sequence",
                "walk_chain_value_rest_sequence",
            ]
        );
    }

    #[test]
    fn invariant_normalization_has_stable_dnf_order_and_removes_duplicate_alternatives() {
        let semantic = crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
                vocab Mode { One = "one", Two = "two", Three = "three", }
                construction predicate: Predicate {
                    element PredicateNode { mode: lex Mode, }
                    require any(
                        all(mode is One, concord_class is Other),
                        all(mode is Two, concord_class is ThirdPersonSingular),
                        all(concord_class is Other, mode is One)
                    );
                    derive concord_class = mode.concord_class;
                    derive mode.concord_class = match mode {
                        One => Values::Other,
                        Two => Values::ThirdPersonSingular,
                        Three => Values::Other,
                    };
                    form predicate = lex(mode);
                }
                root Predicate { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect("predicate fixture parses"),
        )
        .expect("predicate fixture validates")
        .into_semantic();
        let invariant = semantic.constructions()[0].invariant();

        assert_eq!(
            invariant.snapshot(),
            "(mode in [One] AND concord_class in [Other])\nOR\n(mode in [Two] AND concord_class in [ThirdPersonSingular])"
        );
    }

    #[test]
    fn derived_onset_forms_are_a_finite_disjoint_exhaustive_partition() {
        let semantic = crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
                morphology EnglishNoun { feature = Number; recipe = english_noun; }
                lexeme NounLexeme using EnglishNoun {
                    Player = "player",
                    Artifact = "artifact",
                }
                codec Noun {
                    generate declaration_noun {
                        closed = NounLexeme;
                        position = Noun;
                        kinds = [Type, Subtype];
                        feature = Number;
                    }
                }
                construction common: Phrase {
                    element Common { head: lex Noun, }
                    derive number = Values::Singular;
                    derive onset = head.onset;
                    form an when head.onset is Vowel = "an" noun(head);
                    form a otherwise = "a" noun(head);
                }
                root Phrase { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect("Onset feature and feature guard parse"),
        )
        .expect("lexical Onset provider and finite guard validate")
        .into_semantic();
        let forms = semantic.constructions()[0].forms();
        assert_eq!(
            forms[0].guard().test_accepting_witnesses(forms),
            ["head.onset=Vowel"]
        );
        assert_eq!(
            forms[1].guard().test_accepting_witnesses(forms),
            ["head.onset=Consonant"]
        );
    }

    #[test]
    fn invariant_normalization_keeps_satisfiable_mixed_dnf_alternatives() {
        let semantic = crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
                vocab Mode { One = "one", Two = "two", }
                construction predicate: Predicate {
                    element PredicateNode { mode: lex Mode, }
                    require all(any(mode is One, mode is Two), mode is One);
                    form predicate = lex(mode);
                }
                root Predicate { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect("mixed-DNF fixture parses"),
        )
        .expect("a contradictory Cartesian pair does not reject a surviving alternative")
        .into_semantic();

        assert_eq!(
            semantic.constructions()[0].invariant().snapshot(),
            "(mode in [One])"
        );
    }

    #[test]
    fn invariant_resolution_covers_category_vocab_role_and_construction_features() {
        let semantic = crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
                vocab Mode { One = "one", Two = "two", Three = "three", }
                construction first: Child {
                    element FirstChild {}
                    derive number = Values::Singular;
                    form first = "first";
                }
                construction second: Child {
                    element SecondChild {}
                    derive number = Values::Plural;
                    form second = "second";
                }
                construction parent: Parent {
                    element ParentNode { subject: Child, mode: lex Mode, }
                    require subject is First;
                    require mode in [One, Two];
                    require subject.number is Singular;
                    require number is Singular;
                    derive number = subject.number;
                    form parent = subject lex(mode);
                }
                root Parent { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect("finite-domain fixture parses"),
        )
        .expect("finite-domain fixture validates")
        .into_semantic();
        let invariant = semantic
            .constructions()
            .iter()
            .find(|construction| construction.construction_id() == "parent")
            .expect("parent construction is sealed")
            .invariant();

        assert_eq!(
            invariant.snapshot(),
            "(subject in [First] AND mode in [One, Two] AND subject.number in [Singular] AND number in [Singular])"
        );
        assert_eq!(
            invariant
                .constrained_fields()
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>(),
            ["subject", "mode"]
        );
    }

    #[test]
    fn invariant_resolution_accepts_a_recursive_feature_chain() {
        let semantic = crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
                morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
                lexeme Verbs using EnglishVerb { Act = "act", }
                vocab Mode { One = "one", Two = "two", }
                construction recursive: Root {
                    element RecursiveNode { mode: lex Mode, }
                    require concord_class is Other;
                    derive concord_class = verb.concord_class;
                    derive verb.concord_class = mode.concord_class;
                    derive mode.concord_class = match mode {
                        One => Values::Other,
                        Two => Values::ThirdPersonSingular,
                    };
                    form recursive = lex(mode) verb(Verbs::Act);
                }
                root Root { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect("recursive feature-chain fixture parses"),
        )
        .expect("an acyclic multi-hop feature chain is constructible")
        .into_semantic();
        let construction = &semantic.constructions()[0];

        assert_eq!(
            construction.invariant().snapshot(),
            "(concord_class in [Other])"
        );
        assert_eq!(
            construction
                .invariant()
                .constrained_fields()
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>(),
            ["mode"]
        );
    }

    #[test]
    fn invariant_resolution_follows_construction_a_b_and_stored_mode_chain() {
        let semantic = crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
                vocab Mode { One = "one", Two = "two", }
                construction feature_bare: FeatureChild {
                    element FeatureBare {}
                    derive concord_class = Values::Other;
                    form feature_bare = "feature bare";
                }
                construction feature_third: FeatureChild {
                    element FeatureThird {}
                    derive concord_class = Values::ThirdPersonSingular;
                    form feature_third = "feature third";
                }
                construction recursive: Root {
                    element RecursiveNode {
                        a: FeatureChild,
                        b: FeatureChild,
                        mode: lex Mode,
                    }
                    require concord_class is Other;
                    derive concord_class = a.concord_class;
                    derive a.concord_class = b.concord_class;
                    derive b.concord_class = mode.concord_class;
                    derive mode.concord_class = match mode {
                        One => Values::Other,
                        Two => Values::ThirdPersonSingular,
                    };
                    form recursive = a b lex(mode);
                }
                root Root { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect("construction/a/b/mode feature-chain fixture parses"),
        )
        .expect("the sealed resolver follows every acyclic feature hop")
        .into_semantic();
        let construction = semantic
            .constructions()
            .iter()
            .find(|construction| construction.construction_id() == "recursive")
            .expect("recursive construction is sealed");

        assert_eq!(
            construction.invariant().snapshot(),
            "(concord_class in [Other])"
        );
        assert_eq!(
            construction
                .invariant()
                .constrained_fields()
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>(),
            ["a", "b", "mode"]
        );
    }

    #[test]
    fn multiple_require_clauses_are_an_implicit_all_and_tautologies_disappear() {
        let semantic = crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
                vocab Mode { One = "one", Two = "two", }
                construction only: Root {
                    element Only { mode: lex Mode, }
                    require mode in [One, Two];
                    require concord_class in [Other, ThirdPersonSingular];
                    derive concord_class = Values::Other;
                    form only = lex(mode);
                }
                root Root { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect("implicit-all fixture parses"),
        )
        .expect("implicit-all fixture validates")
        .into_semantic();

        assert_eq!(semantic.constructions()[0].invariant().snapshot(), "TRUE");
    }

    #[test]
    fn satisfied_unit_invariant_is_constant_folded_before_emission() {
        let semantic = crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
                construction satisfied: Root {
                    element SatisfiedUnit {}
                    require concord_class is Other;
                    derive concord_class = Values::Other;
                    form satisfied = "satisfied";
                }
                root Root { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect("satisfied unit fixture parses"),
        )
        .expect("a statically true unit invariant validates")
        .into_semantic();
        let invariant = semantic.constructions()[0].invariant();

        assert_eq!(invariant.snapshot(), "TRUE");
        assert!(!invariant.requires_constructor());
    }

    #[test]
    fn false_unit_invariant_is_rejected_before_emission() {
        let error = crate::generate(quote::quote! {
            construction impossible: Root {
                element ImpossibleUnit {}
                require concord_class is Other;
                derive concord_class = Values::ThirdPersonSingular;
                form impossible = "impossible";
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect_err("a false unit invariant must not reach direct unit construction")
        .to_string();

        assert!(
            error.contains("unit invariant for construction `impossible` is unsatisfiable"),
            "{error}"
        );
    }
}

#[cfg(test)]
mod morphology_tests {
    use proc_macro2::Span;

    use crate::macro_def::SurfaceFeature;

    #[test]
    fn generated_morphology_surface_boundary_rejects_missing_feature_coverage() {
        let member_span = Span::mixed_site();
        let expected = [(
            "Deal",
            member_span,
            &[
                SurfaceFeature::PLAIN,
                SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT,
            ][..],
        )];
        let rows = [(member_span, "Deal", SurfaceFeature::PLAIN, "deal")];

        let error = super::validate_complete_surface_rows(&expected, &rows)
            .expect_err("a missing recipe feature must be rejected");

        assert!(
            error.to_string().contains(
                "missing realized surface row for `Deal` feature `ThirdPersonSingularPresent`"
            ),
            "{error}"
        );
        assert_eq!(format!("{:?}", error.span()), format!("{member_span:?}"));
    }

    #[test]
    fn generated_morphology_surface_boundary_rejects_duplicate_exact_row() {
        let member_span = Span::call_site();
        let duplicate_span = Span::mixed_site();
        let expected = [(
            "Deal",
            member_span,
            &[
                SurfaceFeature::PLAIN,
                SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT,
            ][..],
        )];
        let rows = [
            (member_span, "Deal", SurfaceFeature::PLAIN, "deal"),
            (duplicate_span, "Deal", SurfaceFeature::PLAIN, "deal"),
            (
                member_span,
                "Deal",
                SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT,
                "deals",
            ),
        ];

        let error = super::validate_complete_surface_rows(&expected, &rows)
            .expect_err("an exact duplicate realized row must be rejected");

        assert!(
            error
                .to_string()
                .contains("duplicate exact morphology row `(Deal, Plain, deal)`"),
            "{error}"
        );
        assert_eq!(format!("{:?}", error.span()), format!("{duplicate_span:?}"));
    }
}

#[cfg(test)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SemanticSnapshot {
    pub declaration_keys: Vec<(SourceDeclarationKind, String)>,
    pub constructions: Vec<(String, String, String, Vec<String>)>,
    pub terminals: Vec<(String, String, Vec<String>)>,
    pub roots: Vec<(String, bool, bool)>,
    pub feature_equations: Vec<(String, String)>,
    pub boxed_fields: Vec<(String, String)>,
    pub dynamic_number_constructions: Vec<String>,
}
