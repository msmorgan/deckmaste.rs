use proc_macro2::Span;
use proc_macro2::TokenStream;
use syn::Expr;
use syn::Ident;
use syn::LitStr;
use syn::Pat;
use syn::Path;
use syn::Visibility;

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
    pub checked: Option<Checked>,
    pub requirements: Vec<RequireExprSource>,
    pub equations: Vec<FeatureEquation>,
    pub form: Form,
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
}

#[derive(Debug)]
pub enum FieldKind {
    Category(Path),
    Lex(Path),
    Identity(Path),
}

#[derive(Debug)]
pub struct Checked {
    pub visibilities: Vec<CheckedVisibility>,
    pub accessors: Vec<CheckedAccessor>,
    pub constructor: ConstructorBinding,
}

#[derive(Debug)]
pub struct CheckedAccessor {
    pub role: Ident,
    pub method: Ident,
}

#[derive(Debug)]
pub struct CheckedVisibility {
    pub role: Ident,
    pub visibility: NonPublicVisibility,
}

#[derive(Debug)]
pub enum NonPublicVisibility {
    Private(Span),
    Restricted(Visibility),
}

#[derive(Debug)]
pub struct ConstructorBinding {
    pub path: Path,
    pub arguments: Vec<ConstructorArgument>,
}

#[derive(Debug)]
pub enum ConstructorArgument {
    Role(Ident),
    Context(Span),
    VecRole { span: Span, role: Ident },
}

#[derive(Debug)]
pub enum RequireExprSource {
    In {
        subject: RequireSubjectSource,
        members: Vec<Ident>,
    },
    All(Vec<RequireExprSource>),
    Any(Vec<RequireExprSource>),
}

impl RequireExprSource {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Feature {
    Agreement,
    Number,
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
    pub atoms: Vec<FormAtom>,
}

#[derive(Debug)]
pub enum FormAtom {
    Literal(LitStr),
    Role(Ident),
    Lex(Ident),
    Identity(Ident),
    Verb(VerbOperand),
    OpenVerb(OpenDeclarationAtom),
    Noun(Ident),
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
    pub variants: Vec<VocabVariant>,
}

#[derive(Debug)]
pub struct VocabVariant {
    pub name: Ident,
    pub word: LitStr,
}

#[derive(Debug)]
pub struct Lexeme {
    pub name: Ident,
    pub morphology: Ident,
    pub members: Vec<LexemeMember>,
}

#[derive(Debug)]
pub struct LexemeMember {
    pub name: Ident,
    pub lemma: LitStr,
    pub overrides: Vec<LexemeOverride>,
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
    DeclarationNoun(DeclarationNounSource),
    Unsupported { name: Ident },
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
pub struct GeneratedIdentSlot {
    pub slot: Ident,
    pub value: Ident,
}

#[derive(Debug)]
pub struct DeclarationNounKindsSource {
    pub slot: Ident,
    pub kinds: Vec<Ident>,
}

#[derive(Debug)]
pub enum GeneratedIdentityRecipe {
    Context(ContextIdentitySource),
    Unsupported { name: Ident },
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
pub struct UnsignedPrimitiveSource {
    pub slot: Ident,
    pub primitive: Ident,
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
    pub punctuation: LitStr,
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
