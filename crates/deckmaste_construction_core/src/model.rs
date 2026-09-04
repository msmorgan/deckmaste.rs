use proc_macro2::Span;
use proc_macro2::TokenStream;
use syn::Expr;
use syn::Ident;
use syn::LitInt;
use syn::LitStr;
use syn::Pat;
use syn::Path;

#[derive(Debug)]
pub struct Declarations {
    pub declarations: Vec<Declaration>,
}

impl Declarations {
    #[allow(
        dead_code,
        reason = "semantic rows resolve only through their owning semantic plan"
    )]
    pub(crate) fn source_at(&self, index: usize) -> &Declaration {
        &self.declarations[index]
    }
}

#[derive(Debug)]
pub enum Declaration {
    Construction(Construction),
    AbstractProduct(AbstractProduct),
    AbstractSum(AbstractSum),
    Vocab(Vocab),
    Morphology(Morphology),
    Lexeme(Lexeme),
    Codec(TerminalBinding),
    Identity(TerminalBinding),
    Root(Root),
}

#[derive(Debug)]
pub struct Morphology {
    pub name: Ident,
    pub feature: Feature,
    pub recipe: Ident,
}

#[derive(Debug)]
pub struct Construction {
    pub name: Ident,
    pub category: Path,
    pub element: Element,
    pub requirements: Vec<RequireExprSource>,
    pub equations: Vec<FeatureEquation>,
    pub forms: Vec<Form>,
}

#[derive(Debug)]
pub struct AbstractProduct {
    pub name: Ident,
    pub fields: Vec<Field>,
    pub requirements: Vec<RequireExprSource>,
}

#[derive(Debug)]
pub struct AbstractSum {
    pub name: Ident,
    pub alternatives: Vec<AbstractAlternative>,
}

#[derive(Debug)]
pub struct AbstractAlternative {
    pub name: Ident,
    pub value_type: Path,
}

#[derive(Debug)]
pub struct Element {
    pub name: Ident,
    pub fields: Vec<Field>,
}

#[derive(Debug)]
pub struct Field {
    pub name: Ident,
    pub kind: FieldKind,
    pub check: Option<FieldCheck>,
}

#[derive(Debug)]
pub enum FieldKind {
    Category(Path),
    Lex(Path),
    Identity(Path),
    Zeroable {
        value_type: Path,
        item: Box<FieldKind>,
    },
    Optional(Box<FieldKind>),
    Sequence {
        item: Box<FieldKind>,
        surface: SequenceSurfaceSource,
    },
}

#[derive(Debug)]
pub struct FieldCheck {
    pub function: Path,
    pub arguments: Vec<FeatureSlot>,
}

#[derive(Debug)]
pub struct SequenceSurfaceSource {
    pub separator: Option<SeparatorSource>,
    pub terminator: Option<FixedSurfaceSource>,
}

#[derive(Debug)]
pub enum SeparatorSource {
    Uniform(FixedSurfaceSource),
    Positional(Vec<PositionalSeparatorSource>),
}

#[derive(Debug)]
pub struct PositionalSeparatorSource {
    pub class: Ident,
    pub surface: FixedSurfaceSource,
}

#[derive(Debug)]
pub struct FixedSurfaceSource {
    pub atoms: Vec<FixedSurfaceAtomSource>,
    pub transition: SurfaceCaseTransition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurfaceCaseTransition {
    Preserve,
    SentenceInitial,
    Continuation,
}

#[derive(Debug)]
pub enum FixedSurfaceAtomSource {
    Literal(LitStr),
    Lex(Path),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LengthComparison {
    Equal,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

#[derive(Debug)]
pub enum RequireExprSource {
    OptionalPresence {
        role: Ident,
        present: bool,
    },
    Length {
        owner: Option<Path>,
        role: Ident,
        comparison: LengthComparison,
        value: LitInt,
    },
    In {
        subject: RequireSubjectSource,
        members: Vec<Ident>,
    },
    All(Vec<RequireExprSource>),
    Any(Vec<RequireExprSource>),
}

impl RequireExprSource {
    #[cfg(test)]
    pub(crate) fn as_role_refinement(&self) -> Option<(&Ident, &Ident)> {
        let Self::In {
            subject: RequireSubjectSource::Role(role),
            members,
        } = self
        else {
            return None;
        };
        let [variant] = members.as_slice() else {
            return None;
        };
        Some((role, variant))
    }
}

#[cfg(test)]
mod tests {
    use super::RequireExprSource;

    #[test]
    fn optional_presence_is_not_a_role_refinement() {
        let expression = RequireExprSource::OptionalPresence {
            role: syn::parse_str("optional").expect("test role parses"),
            present: true,
        };
        assert!(expression.as_role_refinement().is_none());
    }
}

#[derive(Debug)]
pub enum RequireSubjectSource {
    Role(Ident),
    RoleFeature { role: Ident, feature: Feature },
    ConstructionFeature(Feature),
}

#[derive(Debug)]
pub struct FeatureEquation {
    pub target: FeaturePlace,
    pub value: FeatureValue,
}

#[derive(Debug)]
pub enum FeaturePlace {
    Construction(Feature),
    Role { field: Ident, feature: Feature },
}

#[derive(Debug)]
pub struct FeatureSlot {
    pub role: Ident,
    pub feature: Feature,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum Feature {
    ConcordClass,
    BareLocativeComplement,
    BareLocativeLicense,
    Cardinality,
    Compoundability,
    Countability,
    HomographLicense,
    MannerAnaphorClass,
    ModifierLicense,
    DeterminerNumber,
    FusedHeadLicense,
    PrepositionComplementKind,
    LocativeTemporalLicense,
    NominalForm,
    NominalLicense,
    Number,
    Onset,
    Participle,
    PossessiveEnding,
    PrepositionAttachment,
    Properness,
    Relationality,
}

#[derive(Debug)]
pub enum FeatureValue {
    Constant(Path),
    FromRole(FeatureSlot),
    Match {
        role: Ident,
        arms: Vec<FeatureMatchArm>,
    },
}

#[derive(Debug)]
pub struct FeatureMatchArm {
    pub variant: Ident,
    pub value: Path,
}

#[derive(Debug)]
pub struct Form {
    pub name: Ident,
    pub guard: FormGuardSource,
    pub atoms: Vec<FormAtom>,
}

#[derive(Debug)]
pub enum FormGuardSource {
    Unguarded,
    When(RequireExprSource),
    Otherwise,
}

#[derive(Debug)]
pub enum FormAtom {
    Literal(LitStr),
    /// A literal whose surface deliberately repeats a vocabulary surface it
    /// does not own. The licence is emitted as form-surface metadata and
    /// changes neither parsing nor rendering.
    LicensedLiteral(LitStr),
    SentenceInitial(LitStr),
    StructuralLiteral(LitStr),
    Role(Ident),
    Lex(Ident),
    /// A fixed vocabulary member written `Vocabulary::Member`. It carries no
    /// role and the vocabulary, not the form, owns the rendered surface.
    FixedLex(syn::Path),
    /// A vocabulary marker and its following category role, kept as one atom
    /// so an optional role cannot admit the marker or complement alone.
    Marked(MarkedAtom),
    Identity(Ident),
    Verb(VerbOperand),
    OpenVerb(OpenDeclarationAtom),
    Noun(Ident),
    Bound(BoundAtom),
    Circumfix(CircumfixAtom),
}

#[derive(Debug)]
pub struct MarkedAtom {
    pub marker: syn::Path,
    pub role: Ident,
}

#[derive(Debug)]
pub struct BoundAtom {
    pub direction: BoundDirection,
    pub affix: LitStr,
    pub value: Box<FormAtom>,
}

#[derive(Debug)]
pub struct CircumfixAtom {
    pub prefix: LitStr,
    pub role: Ident,
    pub suffix: LitStr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoundDirection {
    Prefix,
    Suffix,
}

#[derive(Debug)]
pub struct OpenDeclarationAtom {
    pub kind: Ident,
    pub name: LitStr,
}

#[derive(Debug)]
pub enum VerbOperand {
    Fixed(Path),
    Projected(Ident),
}

#[derive(Debug)]
pub struct Vocab {
    pub name: Ident,
    pub feature_defaults: Vec<LexemeFeatureDefault>,
    pub variants: Vec<VocabVariant>,
}

#[derive(Debug)]
pub struct VocabVariant {
    pub name: Ident,
    pub word: LitStr,
    pub feature_overrides: Vec<LexemeFeatureOverride>,
}

#[derive(Debug)]
pub struct Lexeme {
    pub name: Ident,
    pub morphology: Ident,
    pub feature_defaults: Vec<LexemeFeatureDefault>,
    pub members: Vec<LexemeMember>,
}

#[derive(Debug)]
pub struct LexemeFeatureDefault {
    pub feature: Feature,
    pub value: Ident,
}

#[derive(Debug)]
pub struct LexemeMember {
    pub name: Ident,
    pub lemma: LitStr,
    pub overrides: Vec<LexemeOverride>,
    pub feature_overrides: Vec<LexemeFeatureOverride>,
}

#[derive(Debug)]
pub struct LexemeFeatureOverride {
    pub feature: Feature,
    pub value: Ident,
}

#[derive(Debug)]
pub struct LexemeOverride {
    pub feature: Ident,
    pub surface: LitStr,
}

#[derive(Debug)]
pub struct TerminalBinding {
    pub name: Ident,
    pub kind: TerminalBindingKind,
    pub generated: Option<GeneratedCodecRecipe>,
    pub generated_identity: Option<GeneratedIdentityRecipe>,
    pub codec_atom: Option<CodecAtomClass>,
    pub value_type: syn::Type,
    pub lexical_variant: Option<Path>,
    pub render: Option<RenderBinding>,
    pub build: Option<BuildLeaf>,
    pub traversal: Traversal,
}

#[derive(Debug)]
pub enum GeneratedCodecRecipe {
    SignedDecimal(SignedDecimalSource),
    EnglishCardinal(UnsignedNumberSource),
    UnsignedDecimal(UnsignedNumberSource),
    DeclarationNoun(DeclarationNounSource),
    DeclarationDeterminative(DeclarationDeterminativeSource),
    DeclarationTerm(DeclarationTermSource),
    DeclarationVerb(DeclarationVerbSource),
    Unsupported { name: Ident },
}

#[derive(Debug)]
pub struct DeclarationTermSource {
    pub recipe: Ident,
    pub position_slots: Vec<GeneratedIdentSlot>,
    pub kind_slots: Vec<DeclarationVerbKindsSource>,
    pub param_slots: Vec<DeclarationVerbKindsSource>,
    pub param_policy_slots: Vec<GeneratedIdentSlot>,
    pub feature_slots: Vec<GeneratedIdentSlot>,
}

#[derive(Debug)]
pub struct DeclarationNounSource {
    pub recipe: Ident,
    pub closed_slots: Vec<GeneratedIdentSlot>,
    pub position_slots: Vec<GeneratedIdentSlot>,
    pub kind_slots: Vec<DeclarationNounKindsSource>,
    pub feature_slots: Vec<GeneratedIdentSlot>,
}

#[derive(Debug)]
pub struct DeclarationDeterminativeSource {
    pub recipe: Ident,
    pub closed_slots: Vec<DeclarationDeterminativeClosedSource>,
}

#[derive(Debug)]
pub struct DeclarationDeterminativeClosedSource {
    pub slot: Ident,
    pub members: Vec<DeclarationDeterminativeMemberSource>,
}

#[derive(Debug)]
pub struct DeclarationDeterminativeMemberSource {
    pub lemma: Ident,
    pub number_license_slots: Vec<GeneratedIdentSlot>,
    pub fused_head_license_slots: Vec<GeneratedIdentSlot>,
    pub nominal_license_slots: Vec<GeneratedIdentSlot>,
    pub realization_slots: Vec<DeclarationDeterminativeRealizationsSource>,
}

#[derive(Debug)]
pub struct DeclarationDeterminativeRealizationsSource {
    pub slot: Ident,
    pub realizations: Vec<DeclarationDeterminativeRealizationSource>,
}

#[derive(Debug)]
pub struct DeclarationDeterminativeRealizationSource {
    pub surface_slots: Vec<LitStr>,
    pub phrase_number_slots: Vec<GeneratedIdentSlot>,
    pub following_onset_slots: Vec<GeneratedIdentSlot>,
}

#[derive(Debug)]
pub struct GeneratedIdentSlot {
    pub slot: Ident,
    pub value: Ident,
}

#[derive(Debug)]
pub struct DeclarationNounKindsSource {
    pub slot: Ident,
    pub kinds: Vec<DeclarationNounKindSource>,
}

#[derive(Debug)]
pub struct DeclarationNounKindSource {
    pub kind: Ident,
    pub subtype_family: Option<Ident>,
}

#[derive(Debug)]
pub struct DeclarationVerbSource {
    pub recipe: Ident,
    pub closed_slots: Vec<GeneratedIdentSlot>,
    pub class_slots: Vec<GeneratedIdentSlot>,
    pub position_slots: Vec<GeneratedIdentSlot>,
    pub tail_slots: Vec<DeclarationVerbTailSource>,
    pub feature_slots: Vec<GeneratedIdentSlot>,
}

#[derive(Debug)]
pub struct DeclarationVerbKindsSource {
    pub slot: Ident,
    pub kinds: Vec<Ident>,
}

#[derive(Debug)]
pub struct DeclarationVerbTailSource {
    pub slot: Ident,
    pub atoms: Vec<DeclarationVerbTailAtomSource>,
}

#[derive(Debug)]
pub struct DeclarationVerbTailAtomSource {
    pub label: Option<Ident>,
    pub optional: bool,
    pub kind: DeclarationVerbTailAtomKindSource,
}

#[derive(Debug)]
pub enum DeclarationVerbTailAtomKindSource {
    Literal(LitStr),
    Lex(syn::Path),
    Marked {
        marker: syn::Path,
        role: Ident,
    },
    Amount(Ident),
    ObjectNounPhrase(Ident),
    PredicativeComplement(Ident),
    /// A compiler-side grammar role.  These atoms are deliberately separate
    /// from macro-RON's small plugin-facing `CustomTailAtom` vocabulary.
    Role(Ident),
}

#[derive(Debug)]
pub enum GeneratedIdentityRecipe {
    Context(ContextIdentitySource),
    Catalog(CatalogIdentitySource),
    Unsupported { name: Ident },
}

#[derive(Debug)]
pub struct CatalogIdentitySource {
    pub recipe: Ident,
    pub provider_slots: Vec<GeneratedIdentSlot>,
}

#[derive(Debug)]
pub struct ContextIdentitySource {
    pub recipe: Ident,
    pub arms: Vec<ContextIdentitySourceArm>,
    pub canonical_slots: Vec<ContextIdentityCanonicalSource>,
}

#[derive(Debug)]
pub struct ContextIdentitySourceArm {
    pub variant: Ident,
    pub accessor: Ident,
}

#[derive(Debug)]
pub struct ContextIdentityCanonicalSource {
    pub slot: Ident,
    pub arm: Ident,
}

#[derive(Debug)]
pub struct SignedDecimalSource {
    pub recipe: Ident,
    pub magnitude_slots: Vec<UnsignedPrimitiveSource>,
    pub sign_type_slots: Vec<SignedDecimalSignTypeSource>,
}

#[derive(Debug)]
pub enum UnsignedPrimitiveSource {
    U32 { slot: Ident, primitive: Ident },
    NonZeroU32 { slot: Ident, primitive: Ident },
    Unsupported { slot: Ident, primitive: Ident },
}

impl UnsignedPrimitiveSource {
    pub(crate) fn slot(&self) -> &Ident {
        match self {
            Self::U32 { slot, .. }
            | Self::NonZeroU32 { slot, .. }
            | Self::Unsupported { slot, .. } => slot,
        }
    }

    pub(crate) fn primitive(&self) -> &Ident {
        match self {
            Self::U32 { primitive, .. }
            | Self::NonZeroU32 { primitive, .. }
            | Self::Unsupported { primitive, .. } => primitive,
        }
    }
}

#[derive(Debug)]
pub struct UnsignedNumberSource {
    pub recipe: Ident,
    pub magnitude_slots: Vec<UnsignedPrimitiveSource>,
}

#[derive(Debug)]
pub struct SignedDecimalSignTypeSource {
    pub slot: Ident,
    pub name: Ident,
    pub roles: Vec<SignedDecimalSignRoleSource>,
}

#[derive(Debug)]
pub struct SignedDecimalSignRoleSource {
    pub variant: Ident,
    pub spelling: SignedDecimalSignSpelling,
}

#[derive(Debug)]
pub enum SignedDecimalSignSpelling {
    None(Span),
    Literal(LitStr),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalBindingKind {
    Codec,
    Identity,
}

#[derive(Debug)]
pub enum RenderBinding {
    Runtime(Path),
    ContextIdentity(Vec<ContextIdentityArm>),
}

#[derive(Debug)]
pub struct ContextIdentityArm {
    pub variant: Ident,
    pub accessor: Ident,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodecAtomClass {
    Lex,
    Noun,
}

#[derive(Debug)]
pub struct BuildLeaf {
    pub pattern: Pat,
    pub construct: Expr,
}

#[derive(Debug)]
pub struct Traversal {
    pub parts: Vec<TraversalPart>,
    pub visit_order: Vec<Ident>,
    pub callback_mode: Option<VisitMode>,
    pub argument: Option<Ident>,
    pub variants: Vec<Ident>,
    pub fields: Vec<TraversalField>,
    pub calls: Vec<TraversalCall>,
    pub branches: Vec<TraversalBranch>,
    pub leaf_callbacks: Vec<LeafCallback>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisitMode {
    Copy,
    Borrowed,
}

#[derive(Debug)]
pub struct TraversalCall {
    pub callback: Path,
    pub mode: VisitMode,
    pub value: Expr,
}

#[derive(Debug)]
pub struct TraversalField {
    pub name: Ident,
    pub value_type: syn::Type,
}

#[derive(Debug)]
pub struct TraversalBranch {
    pub value: Expr,
    pub arms: Vec<TraversalBranchArm>,
}

#[derive(Debug)]
pub struct TraversalBranchArm {
    pub variant: Path,
    pub binding: Ident,
    pub value_type: syn::Type,
    pub call: TraversalCall,
}

#[derive(Debug)]
pub struct LeafCallback {
    pub name: Ident,
    pub value_type: syn::Type,
    pub mode: VisitMode,
}

#[derive(Debug)]
pub struct TraversalPart {
    pub name: Ident,
    pub kind: TraversalKind,
    pub value: Expr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraversalKind {
    Scalar,
    Identity,
    Subtree,
}

#[derive(Debug)]
pub struct Root {
    pub category: Path,
    pub punctuation: Option<LitStr>,
    pub eoi: bool,
    pub standalone_render: bool,
}

#[derive(Debug)]
pub struct Invocation {
    pub tokens: TokenStream,
    pub span: Span,
}

#[derive(Debug)]
pub struct Expansion {
    pub(crate) plan: crate::plan::EmissionPlan,
    pub(crate) escape_hatches: crate::report::EscapeHatchReport,
}

impl Expansion {
    #[must_use]
    pub fn plan(&self) -> &crate::plan::EmissionPlan {
        &self.plan
    }

    #[must_use]
    pub fn items(&self) -> &[crate::plan::GeneratedItem] {
        self.plan.items()
    }

    #[must_use]
    pub fn terminal_contributions(&self) -> &[crate::plan::TerminalContribution] {
        self.plan.terminal_contributions()
    }

    #[must_use]
    pub fn escape_hatches(&self) -> &crate::report::EscapeHatchReport {
        &self.escape_hatches
    }

    #[must_use]
    pub fn tokens(&self) -> TokenStream {
        self.plan.tokens()
    }
}
