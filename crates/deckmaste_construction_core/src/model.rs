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

#[derive(Debug)]
pub enum Declaration {
    Construction(Construction),
    Vocab(Vocab),
    Lexeme(Lexeme),
    Codec(TerminalBinding),
    Identity(TerminalBinding),
    Root(Root),
}

#[derive(Debug)]
pub struct Construction {
    pub name: Ident,
    pub category: Path,
    pub element: Element,
    pub checked: Option<Checked>,
    pub requirements: Vec<RoleRefinement>,
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
    pub constructor: ConstructorBinding,
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
pub struct RoleRefinement {
    pub role: Ident,
    pub variant: Ident,
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
    Noun(Ident),
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
    pub variants: Vec<Ident>,
}

#[derive(Debug)]
pub struct TerminalBinding {
    pub name: Ident,
    pub codec_atom: Option<CodecAtomClass>,
    pub value_type: syn::Type,
    pub lexical_variant: Path,
    pub render: Path,
    pub build: BuildLeaf,
    pub traversal: Traversal,
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
    pub fn tokens(&self) -> TokenStream {
        self.plan.tokens()
    }
}
