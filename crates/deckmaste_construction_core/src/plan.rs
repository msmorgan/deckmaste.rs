use std::collections::HashSet;

use proc_macro2::TokenStream;

use crate::model::Declaration;
use crate::semantic::SemanticPlan;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NamedKind {
    Type,
    Trait,
    Function,
    Constant,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ItemKey {
    Named {
        kind: NamedKind,
        name: String,
    },
    Impl {
        trait_name: Option<String>,
        self_ty: String,
    },
}

impl ItemKey {
    pub(crate) fn named_type(name: impl Into<String>) -> Self {
        Self::Named {
            kind: NamedKind::Type,
            name: name.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourceDeclarationKind {
    Construction,
    AbstractProduct,
    AbstractSum,
    Vocab,
    Morphology,
    Lexeme,
    Codec,
    Identity,
    Root,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeclarationKey {
    kind: SourceDeclarationKind,
    name: String,
}

impl DeclarationKey {
    pub(crate) fn new(kind: SourceDeclarationKind, name: impl Into<String>) -> Self {
        Self {
            kind,
            name: name.into(),
        }
    }

    #[must_use]
    pub fn kind(&self) -> SourceDeclarationKind {
        self.kind
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn from_source(declaration: &Declaration) -> Self {
        match declaration {
            Declaration::Construction(value) => {
                Self::new(SourceDeclarationKind::Construction, value.name.to_string())
            }
            Declaration::AbstractProduct(value) => Self::new(
                SourceDeclarationKind::AbstractProduct,
                value.name.to_string(),
            ),
            Declaration::AbstractSum(value) => {
                Self::new(SourceDeclarationKind::AbstractSum, value.name.to_string())
            }
            Declaration::Vocab(value) => {
                Self::new(SourceDeclarationKind::Vocab, value.name.to_string())
            }
            Declaration::Morphology(value) => {
                Self::new(SourceDeclarationKind::Morphology, value.name.to_string())
            }
            Declaration::Lexeme(value) => {
                Self::new(SourceDeclarationKind::Lexeme, value.name.to_string())
            }
            Declaration::Codec(value) => {
                Self::new(SourceDeclarationKind::Codec, value.name.to_string())
            }
            Declaration::Identity(value) => {
                Self::new(SourceDeclarationKind::Identity, value.name.to_string())
            }
            Declaration::Root(value) => Self::new(
                SourceDeclarationKind::Root,
                crate::identifier::path_key(&value.category),
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GeneratedItem {
    pub key: ItemKey,
    pub tokens: TokenStream,
    pub origins: Vec<DeclarationKey>,
}

impl GeneratedItem {
    pub(crate) fn new(key: ItemKey, tokens: TokenStream, origins: Vec<DeclarationKey>) -> Self {
        Self {
            key,
            tokens,
            origins,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalKind {
    Vocab,
    Lexeme,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerminalVariantContribution {
    name: String,
    word: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerminalSurfaceContribution {
    member: String,
    feature: crate::macro_def::SurfaceFeature,
    surface: String,
}

impl TerminalSurfaceContribution {
    fn new(
        member: impl Into<String>,
        feature: crate::macro_def::SurfaceFeature,
        surface: impl Into<String>,
    ) -> Self {
        Self {
            member: member.into(),
            feature,
            surface: surface.into(),
        }
    }

    #[must_use]
    pub fn member(&self) -> &str {
        &self.member
    }

    #[must_use]
    pub fn feature(&self) -> crate::macro_def::SurfaceFeature {
        self.feature
    }

    #[must_use]
    pub fn surface(&self) -> &str {
        &self.surface
    }
}

impl TerminalVariantContribution {
    pub(crate) fn new(name: impl Into<String>, word: Option<String>) -> Self {
        Self {
            name: name.into(),
            word,
        }
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn word(&self) -> Option<&str> {
        self.word.as_deref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerminalContribution {
    origin: DeclarationKey,
    kind: TerminalKind,
    name: String,
    render_function: Option<String>,
    variants: Vec<TerminalVariantContribution>,
    verb_provider: bool,
    surfaces: Vec<TerminalSurfaceContribution>,
    expected_generated_item_keys: Vec<ItemKey>,
}

impl TerminalContribution {
    pub(crate) fn new(
        origin: DeclarationKey,
        kind: TerminalKind,
        name: impl Into<String>,
        render_function: Option<String>,
        variants: Vec<TerminalVariantContribution>,
        verb_provider: bool,
    ) -> Self {
        Self {
            origin,
            kind,
            name: name.into(),
            render_function,
            variants,
            verb_provider,
            surfaces: Vec::new(),
            expected_generated_item_keys: Vec::new(),
        }
    }

    #[must_use]
    pub fn origin(&self) -> &DeclarationKey {
        &self.origin
    }

    #[must_use]
    pub fn kind(&self) -> TerminalKind {
        self.kind
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn render_function(&self) -> Option<&str> {
        self.render_function.as_deref()
    }

    #[must_use]
    pub fn variants(&self) -> &[TerminalVariantContribution] {
        &self.variants
    }

    #[must_use]
    pub fn is_verb_provider(&self) -> bool {
        self.verb_provider
    }

    #[must_use]
    pub fn surfaces(&self) -> &[TerminalSurfaceContribution] {
        &self.surfaces
    }

    #[must_use]
    pub fn expected_generated_item_keys(&self) -> &[ItemKey] {
        &self.expected_generated_item_keys
    }

    fn seal_projection(
        &mut self,
        surfaces: Vec<TerminalSurfaceContribution>,
        expected_generated_item_keys: Vec<ItemKey>,
    ) {
        self.surfaces = surfaces;
        self.expected_generated_item_keys = expected_generated_item_keys;
    }
}

#[derive(Debug)]
pub struct EmissionPlan {
    items: Vec<GeneratedItem>,
    terminal_contributions: Vec<TerminalContribution>,
}

impl EmissionPlan {
    #[must_use]
    pub fn items(&self) -> &[GeneratedItem] {
        &self.items
    }

    #[must_use]
    pub fn terminal_contributions(&self) -> &[TerminalContribution] {
        &self.terminal_contributions
    }

    #[must_use]
    pub fn tokens(&self) -> TokenStream {
        self.items.iter().map(|item| item.tokens.clone()).collect()
    }
}

pub(crate) fn plan_emission(plan: &SemanticPlan) -> syn::Result<EmissionPlan> {
    let mut items = crate::emit::ast::emit(plan)?;
    items.extend(crate::emit::final_constituent::emit(plan)?);
    let (terminal_items, mut terminal_contributions) = crate::emit::terminal::emit(plan)?;
    seal_terminal_contribution_projection(plan, &mut terminal_contributions)?;
    items.extend(terminal_items);
    items.extend(crate::emit::runtime::emit(plan));
    items.extend(crate::emit::scanner::emit(plan));
    items.extend(crate::emit::render::emit(plan)?);
    items.extend(crate::emit::visit::emit(plan)?);
    items.extend(crate::emit::rules::emit(plan)?);
    items.extend(crate::emit::build::emit(plan)?);

    let mut keys = HashSet::new();
    for item in &items {
        if !keys.insert(item.key.clone()) {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("duplicate generated item key {:?}", item.key),
            ));
        }
        crate::format::validate_item(item).map_err(|error| {
            syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("generated item {:?} is invalid: {error}", item.key),
            )
        })?;
    }

    Ok(EmissionPlan {
        items,
        terminal_contributions,
    })
}

fn seal_terminal_contribution_projection(
    plan: &SemanticPlan,
    terminal_contributions: &mut [TerminalContribution],
) -> syn::Result<()> {
    for contribution in terminal_contributions {
        let (surfaces, expected_generated_item_keys) = plan
            .terminals()
            .iter()
            .find_map(|terminal| match terminal {
                crate::semantic::TerminalPlan::Lexeme(lexeme)
                    if lexeme.name() == contribution.name() =>
                {
                    Some((
                        lexeme
                            .surfaces()
                            .iter()
                            .map(|row| {
                                TerminalSurfaceContribution::new(
                                    row.member(),
                                    row.feature(),
                                    row.surface(),
                                )
                            })
                            .collect(),
                        terminal.expected_terminal_item_keys(),
                    ))
                }
                crate::semantic::TerminalPlan::Vocab(vocab)
                    if vocab.name() == contribution.name() =>
                {
                    Some((Vec::new(), terminal.expected_terminal_item_keys()))
                }
                _ => None,
            })
            .ok_or_else(|| {
                syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!(
                        "terminal contribution `{}` has no semantic projection",
                        contribution.name()
                    ),
                )
            })?;
        contribution.seal_projection(surfaces, expected_generated_item_keys);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::GeneratedItem;
    use crate::ItemKey;
    use crate::NamedKind;
    use crate::SourceDeclarationKind;
    use crate::semantic::SemanticPlan;
    use crate::test_support::representative_expansion;

    fn declaration_verb_plan_for(tail: &proc_macro2::TokenStream) -> SemanticPlan {
        crate::validate_declarations(
            crate::parse_declarations(crate::test_support::declaration_verb_tokens(tail))
                .expect("declaration_verb syntax parses"),
        )
        .expect("the exact declaration_verb recipe validates")
        .into_semantic()
    }

    fn sole_declaration_verb(
        plan: &SemanticPlan,
    ) -> (usize, &crate::semantic::DeclarationVerbPlan) {
        let recipes = plan.runtime_declaration_verbs().collect::<Vec<_>>();
        let [(terminal_index, recipe)] = recipes.as_slice() else {
            panic!("one declaration_verb recipe is sealed")
        };
        (*terminal_index, *recipe)
    }

    #[test]
    fn structural_planning_emits_abstract_rows_and_structural_construction_bnf() {
        let abstract_semantic = crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
                construction node: Node { element NodeValue {} form node = "node"; }
                abstract product Holder { value: Node, }
                abstract sum Choice { node: Node, }
                root Node { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect("abstract planning fixture parses"),
        )
        .expect("abstract planning fixture validates")
        .into_semantic();
        let emission = super::plan_emission(&abstract_semantic)
            .expect("abstract structural AST/runtime rows emit before BNF helpers");
        for name in ["Holder", "Choice", "BuildValue", "NonterminalCategory"] {
            assert!(
                emission.items().iter().any(|item| {
                    matches!(&item.key, crate::ItemKey::Named { name: actual, .. } if actual == name)
                }),
                "Task 3 emits {name}",
            );
        }

        let construction_semantic = crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
                construction node: Node { element NodeValue {} form node = "node"; }
                construction document: Document {
                    element DocumentValue { nodes: seq Node terminated by ".", }
                    require len(nodes) >= 1;
                    form document = nodes;
                }
                root Node { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect("structural construction fixture parses"),
        )
        .expect("structural construction fixture validates")
        .into_semantic();
        let emission = super::plan_emission(&construction_semantic)
            .expect("Task 4 emits structural construction fields");
        let rules = emission
            .items()
            .iter()
            .find(|item| matches!(&item.key, crate::ItemKey::Named { name, .. } if name == "RULES"))
            .expect("structural rules item")
            .tokens
            .to_string();
        assert!(
            rules.contains("DocumentValueNodesSequenceSingleton")
                && rules.contains("DocumentValueNodesSequenceRecursive"),
            "{rules}"
        );
    }

    #[test]
    fn explicit_sum_owned_category_reuses_construction_products_across_all_emitters() {
        let expansion = crate::generate(quote::quote! {
            abstract sum Choice { Left: LeftNode, Renamed: RightNode, }
            construction left: Choice {
                element LeftNode {}
                form left = "left";
            }
            construction right: Choice {
                element RightNode {}
                form right = "right";
            }
            root Choice { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("the explicit-sum-owned category generates");
        let source = expansion.tokens().to_string();

        assert_eq!(source.matches("pub enum Choice").count(), 1, "{source}");
        for expected in [
            "Choice :: Renamed (value)",
            "RuleId :: RightNodeConstruction",
            "RuleId :: ChoiceRenamed",
            "BuildValue :: RightNode",
            "fn render_right_node",
            "fn walk_choice",
            "fn visit_right_node",
        ] {
            assert!(source.contains(expected), "missing `{expected}`: {source}");
        }
        assert_eq!(source.matches("fn visit_right_node").count(), 1, "{source}");
    }

    #[test]
    fn structural_wrapped_terminal_roles_reach_the_deferred_emission_boundary() {
        let semantic = crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
                vocab Word { One = "one", }
                identity HandleSpelling {
                    generate context {
                        Full => card_name,
                        Abbreviated => abbreviated_card_name,
                        canonical_on_collision = Full;
                    }
                }
                construction wrapped: Root {
                    element Wrapped {
                        maybe: opt lex Word,
                        handles: seq identity HandleSpelling,
                    }
                    form wrapped = lex(maybe) identity(handles);
                }
                root Root { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect("wrapped terminal fixture parses"),
        )
        .expect("wrapped terminal fixture validates")
        .into_semantic();

        let emission =
            super::plan_emission(&semantic).expect("Task 4 emits wrapped structural fields");
        let rules = emission
            .items()
            .iter()
            .find(|item| matches!(&item.key, crate::ItemKey::Named { name, .. } if name == "RULES"))
            .expect("wrapped structural rules")
            .tokens
            .to_string();
        assert!(
            rules.contains("WrappedMaybeOptionalAbsent")
                && rules.contains("WrappedHandlesSequenceEmpty"),
            "{rules}"
        );
    }

    #[test]
    fn invariant_field_policy_discovers_context_and_seals_accessor_modes() {
        let semantic = crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
                vocab Mode { One = "one", Two = "two", }
                identity SelfReferenceSpelling {
                    generate context {
                        Full => card_name,
                        Abbreviated => abbreviated_card_name,
                        canonical_on_collision = Full;
                    }
                }
                construction first: Child {
                    element FirstChild {}
                    form first = "first";
                }
                construction second: Child {
                    element SecondChild {}
                    form second = "second";
                }
                construction only: Root {
                    element Only {
                        subject: Child,
                        mode: lex Mode,
                        spelling: identity SelfReferenceSpelling,
                    }
                    require subject is First;
                    require mode is One;
                    form only = subject lex(mode) identity(spelling);
                }
                root Root { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect("field-policy fixture parses"),
        )
        .expect("field-policy fixture validates")
        .into_semantic();
        let construction = semantic
            .constructions()
            .iter()
            .find(|construction| construction.construction_id() == "only")
            .expect("root construction is sealed");
        let invariant = construction.invariant();

        assert_eq!(
            invariant
                .constrained_fields()
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>(),
            ["subject", "mode"]
        );
        assert_eq!(
            invariant
                .context_identity_fields()
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>(),
            ["spelling"]
        );
        assert!(invariant.requires_context());
        assert_eq!(
            construction
                .fields()
                .iter()
                .map(|field| (
                    field.name_key(),
                    field.is_invariant_bearing(),
                    field.accessor_mode(),
                ))
                .collect::<Vec<_>>(),
            [
                (
                    "subject".to_owned(),
                    true,
                    Some(crate::semantic::AccessorMode::Borrow)
                ),
                (
                    "mode".to_owned(),
                    true,
                    Some(crate::semantic::AccessorMode::Copy)
                ),
                (
                    "spelling".to_owned(),
                    true,
                    Some(crate::semantic::AccessorMode::Copy)
                ),
            ]
        );
    }

    #[test]
    fn concord_class_constraint_field_policy_seals_target_and_source_accessors() {
        let semantic = crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
                vocab Mode { One = "one", Many = "many", }
                construction bare: Child {
                    element BareChild {}
                    derive concord_class = Values::Other;
                    form bare = "bare";
                }
                construction third: Child {
                    element ThirdChild {}
                    derive concord_class = Values::ThirdPersonSingular;
                    form third = "third";
                }
                construction constrained: Root {
                    element Constrained { mode: lex Mode, child: Child, }
                    derive child.concord_class = mode.concord_class;
                    derive mode.concord_class = match mode {
                        One => Values::Other,
                        Many => Values::ThirdPersonSingular,
                    };
                    form constrained = lex(mode) child;
                }
                root Root { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect("ConcordClass field-policy fixture parses"),
        )
        .expect("ConcordClass field-policy fixture validates")
        .into_semantic();
        let construction = semantic
            .constructions()
            .iter()
            .find(|construction| construction.construction_id() == "constrained")
            .expect("constrained construction is sealed");

        assert_eq!(
            construction
                .fields()
                .iter()
                .map(|field| (field.name_key(), field.accessor_mode()))
                .collect::<Vec<_>>(),
            [
                ("mode".to_owned(), Some(crate::semantic::AccessorMode::Copy),),
                (
                    "child".to_owned(),
                    Some(crate::semantic::AccessorMode::Borrow),
                ),
            ],
        );
    }

    #[test]
    fn invariant_plan_drives_ast_build_render_and_visit_projection() {
        let source = quote::quote! {
            vocab Mode { One = "one", Two = "two", }
            identity SelfReferenceSpelling {
                generate context {
                    Full => card_name,
                    Abbreviated => abbreviated_card_name,
                    canonical_on_collision = Full;
                }
            }
            construction only: Root {
                element Only { mode: lex Mode, spelling: identity SelfReferenceSpelling, }
                require mode is One;
                form only = lex(mode) identity(spelling);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        };
        let mut semantic = crate::validate_declarations(
            crate::parse_declarations(source).expect("mutation fixture parses"),
        )
        .expect("mutation fixture validates")
        .into_semantic();

        let ast = formatted(&crate::emit::ast::emit(&semantic).expect("sealed AST emits"));
        let build = formatted(&crate::emit::build::emit(&semantic).expect("sealed build emits"));
        let render = formatted(&crate::emit::render::emit(&semantic).expect("sealed render emits"));
        let visitor =
            formatted(&crate::emit::visit::emit(&semantic).expect("sealed visitor emits"));
        for fragment in [
            "pub struct Only { mode : Mode , spelling : SelfReferenceSpelling }",
            "pub const fn mode (& self) -> Mode",
            "pub const fn spelling (& self) -> SelfReferenceSpelling",
        ] {
            assert!(ast.contains(fragment), "missing `{fragment}`: {ast}");
        }
        assert!(build.contains("Leaf :: Mode (mode)"), "{build}");
        assert!(!build.contains("Leaf :: Mode (Mode :: One)"), "{build}");
        assert!(
            build.contains("Only :: try_new (* mode , * spelling , context)"),
            "{build}",
        );
        assert!(render.contains("Self :: Only (only)"), "{render}");
        assert!(
            render.contains("render_mode (writer , only . mode ())"),
            "{render}"
        );
        assert!(
            render.contains("only . spelling ()") && !render.contains("Only { mode , spelling }"),
            "{render}",
        );
        assert!(
            visitor.contains("walk_mode (visitor , only . mode ())"),
            "{visitor}"
        );
        assert!(
            visitor.contains("walk_self_reference_spelling (visitor , only . spelling ())")
                && !visitor.contains("let Only { mode , spelling }"),
            "{visitor}",
        );

        semantic.test_only_replace_invariant_member("only", "mode", "Two");
        semantic.test_only_remove_invariant_context_field("only", "spelling");
        let changed_ast =
            formatted(&crate::emit::ast::emit(&semantic).expect("mutated sealed AST emits"));
        let changed_build =
            formatted(&crate::emit::build::emit(&semantic).expect("mutated sealed build emits"));
        let changed_render =
            formatted(&crate::emit::render::emit(&semantic).expect("mutated sealed render emits"));
        let changed_visitor =
            formatted(&crate::emit::visit::emit(&semantic).expect("mutated sealed visitor emits"));
        assert!(
            changed_ast.contains("matches ! (mode , Mode :: Two)"),
            "{changed_ast}"
        );
        assert!(
            !changed_ast.contains("matches ! (mode , Mode :: One)"),
            "{changed_ast}"
        );
        assert!(
            changed_ast.contains("pub spelling : SelfReferenceSpelling")
                && !changed_ast.contains("spelling . valid_in (context)")
                && !changed_ast.contains("pub const fn spelling"),
            "{changed_ast}",
        );
        assert!(
            changed_build.contains("Only :: try_new (* mode , * spelling)")
                && !changed_build.contains("Only :: try_new (* mode , * spelling , context)"),
            "{changed_build}",
        );
        assert!(
            changed_render.contains("only . spelling")
                && !changed_render.contains("only . spelling ()"),
            "{changed_render}",
        );
        assert!(
            changed_visitor.contains("only . spelling")
                && !changed_visitor.contains("only . spelling ()"),
            "{changed_visitor}",
        );
        assert_ne!(ast, changed_ast);
        assert_ne!(build, changed_build);
        assert_ne!(render, changed_render);
        assert_ne!(visitor, changed_visitor);
    }

    #[test]
    fn generated_morphology_terminal_contribution_retains_sealed_projection() {
        let expansion = crate::generate(quote::quote! {
            morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
            lexeme VerbLexeme using EnglishVerb {
                Deal = "deal",
                Be = "be" { Other = "are", ThirdPersonSingular = "is", },
            }
            construction action: Ability {
                element Action {}
                derive concord_class = verb.concord_class;
                derive verb.concord_class = Values::Other;
                form action = verb(VerbLexeme::Deal);
            }
            root Ability { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("the sealed morphology emits");
        let contribution = expansion
            .terminal_contributions()
            .iter()
            .find(|row| row.name() == "VerbLexeme")
            .expect("the lexeme retains its terminal contribution");

        assert_eq!(
            contribution
                .surfaces()
                .iter()
                .map(|row| (row.member(), row.feature(), row.surface()))
                .collect::<Vec<_>>(),
            [
                ("Deal", crate::macro_def::SurfaceFeature::PLAIN, "deal"),
                (
                    "Deal",
                    crate::macro_def::SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT,
                    "deals",
                ),
                ("Be", crate::macro_def::SurfaceFeature::PLAIN, "are"),
                (
                    "Be",
                    crate::macro_def::SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT,
                    "is",
                ),
            ]
        );
        assert_eq!(
            contribution.expected_generated_item_keys(),
            [
                crate::ItemKey::named_type("VerbLexeme"),
                crate::ItemKey::Named {
                    kind: crate::NamedKind::Function,
                    name: "surface_for_verb_lexeme".into(),
                },
            ]
        );
    }

    #[test]
    fn generated_morphology_lexeme_changes_semantic_expected_item_inventory() {
        fn expected(tokens: proc_macro2::TokenStream) -> Vec<crate::ItemKey> {
            crate::validate_declarations(
                crate::parse_declarations(tokens).expect("inventory fixture parses"),
            )
            .expect("inventory fixture validates")
            .semantic()
            .terminals()
            .iter()
            .flat_map(crate::semantic::TerminalPlan::expected_terminal_item_keys)
            .collect()
        }

        let without_lexeme = expected(quote::quote! {
            morphology EnglishNoun { feature = Number; recipe = english_noun; }
            construction only: Root { element Only {} form only = "only"; }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        let with_lexeme = expected(quote::quote! {
            morphology EnglishNoun { feature = Number; recipe = english_noun; }
            lexeme NounLexeme using EnglishNoun { Object = "object", }
            construction only: Root { element Only {} form only = "only"; }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });

        assert_eq!(without_lexeme, []);
        assert_eq!(
            with_lexeme,
            [
                crate::ItemKey::named_type("NounLexeme"),
                crate::ItemKey::Named {
                    kind: crate::NamedKind::Function,
                    name: "surface_for_noun_lexeme".into(),
                },
            ],
        );
    }

    #[test]
    fn generated_morphology_terminal_projection_rejects_missing_semantic_match() {
        let semantic = crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
                morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
                lexeme VerbLexeme using EnglishVerb { Deal = "deal", }
                construction action: Ability {
                    element Action {}
                    derive concord_class = verb.concord_class;
                    derive verb.concord_class = Values::Other;
                    form action = verb(VerbLexeme::Deal);
                }
                root Ability { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect("mismatch fixture parses"),
        )
        .expect("mismatch fixture validates")
        .into_semantic();
        let mut contributions = vec![crate::TerminalContribution::new(
            crate::DeclarationKey::new(crate::SourceDeclarationKind::Lexeme, "MissingLexeme"),
            crate::TerminalKind::Lexeme,
            "MissingLexeme",
            None,
            Vec::new(),
            true,
        )];

        let error = super::seal_terminal_contribution_projection(&semantic, &mut contributions)
            .expect_err("a contribution without a semantic terminal must be rejected");

        assert!(
            error
                .to_string()
                .contains("terminal contribution `MissingLexeme` has no semantic projection"),
            "{error}"
        );
    }

    #[test]
    fn open_verb_atom_seals_a_typed_plan_and_generated_matcher() {
        let plan = crate::validate_declarations(
            crate::parse_declarations(crate::test_support::open_verb_tokens())
                .expect("open_verb syntax parses"),
        )
        .expect("open_verb declaration validates")
        .into_semantic();

        assert!(plan.snapshot().constructions.iter().any(|row| {
            row.0 == "destroy"
                && row.3
                    == [
                        "open_verb(KeywordAction, Destroy, Verb)".to_owned(),
                        "category(object: NounPhrase)".to_owned(),
                    ]
        }));

        let rules = formatted(&crate::emit::rules::emit(&plan).expect("open matcher emits"));
        for expected in [
            "Lexical :: Declaration (DeclarationMatcher",
            "DeclarationKind :: KeywordAction",
            "name : \"Destroy\"",
            "GrammarPosition :: Verb",
            "FeatureConstraint :: Any",
            "LexicalOwnerTemplate :: Declaration",
        ] {
            assert!(rules.contains(expected), "missing `{expected}`: {rules}");
        }
        let requirements = formatted(&crate::emit::runtime::emit(&plan));
        assert!(requirements.contains("REQUIRED_DECLARATIONS"));
        assert!(requirements.contains("name : \"Destroy\""));
    }

    #[test]
    fn declaration_noun_roles_share_one_aggregate_inventory_terminal() {
        let source: proc_macro2::TokenStream = quote::quote! {
            morphology EnglishNoun { feature = Number; recipe = english_noun; }
            lexeme NounLexeme using EnglishNoun { Player = "player", }
            codec Noun {
                generate declaration_noun {
                    closed = NounLexeme;
                    position = Noun;
                    kinds = [Type, Subtype(Creature), TurnPart];
                    feature = Number;
                }
            }
            construction modified: Phrase {
                element Modified {
                    modifier: lex Noun,
                    head: lex Noun,
                }
                derive modifier.number = Values::Singular;
                derive head.number = modifier.number;
                derive number = head.number;
                form modified = noun(modifier) noun(head);
            }
            root Phrase { punctuation = "."; eoi = true; standalone_render = true; }
        };
        let plan = crate::validate_declarations(
            crate::parse_declarations(source).expect("declaration_noun syntax parses"),
        )
        .expect("the exact closed recipe validates")
        .into_semantic();

        let terminals = plan
            .runtime_declaration_nouns()
            .map(|(index, terminal)| (index, terminal.codec_name()))
            .collect::<Vec<_>>();
        assert_eq!(terminals, [(1, "Noun")]);

        let construction = plan
            .constructions()
            .iter()
            .find(|construction| construction.construction_id() == "modified")
            .expect("the consumer construction is sealed");
        for role in ["modifier", "head"] {
            let field = construction.field(role).expect("the noun role is sealed");
            assert_eq!(field.terminal(), "Noun");
            assert!(matches!(
                plan.atom_terminal(field.terminal()),
                Ok(crate::semantic::AtomTerminal::DeclarationNoun {
                    terminal_index,
                    plan: terminal,
                }) if terminal_index == 1 && terminal.codec_name() == "Noun"
            ));
        }
    }

    #[test]
    fn declaration_verb_tail_is_a_normalized_semantic_frame_key() {
        use crate::macro_def::CustomTailAtom;
        use crate::macro_def::VerbFrameSet;
        use crate::semantic::{DeclarationVerbPlan, VerbFrameAtom, VerbFrameClass};

        let intransitive = VerbFrameSet::Intransitive;
        let transitive = VerbFrameSet::Transitive;
        let measure_complement = VerbFrameSet::MeasureComplement;

        let object_plan = declaration_verb_plan_for(&quote::quote! { ObjectNounPhrase });
        let (terminal_index, object) = sole_declaration_verb(&object_plan);
        assert_eq!(terminal_index, 1);
        assert_eq!(object.source_index(), 2);
        assert_eq!(object.codec_ident(), "TransitiveVerb");
        assert_eq!(
            object.declaration_value_ident(),
            "DeclarationTransitiveVerb"
        );
        assert_eq!(object.closed_lexeme().expect("closed branch"), "CoreVerb");
        assert_eq!(
            DeclarationVerbPlan::position(),
            crate::macro_def::GrammarPosition::Verb
        );
        assert_eq!(object.feature_axis(), crate::feature::Feature::ConcordClass);
        assert_eq!(object.frame_key().class(), VerbFrameClass::Predicate);
        assert_eq!(
            object.frame_key().atoms(),
            [VerbFrameAtom::ObjectNounPhrase]
        );
        assert!(object.frame_key().matches_frame_set(&transitive));
        assert!(!object.frame_key().matches_frame_set(&measure_complement));
        assert!(object.frame_key().matches_frame_set(&VerbFrameSet::Custom {
            frames: vec![vec![CustomTailAtom::ObjectNounPhrase]],
        }));

        let amount_plan = declaration_verb_plan_for(&quote::quote! { Amount });
        let (_, amount) = sole_declaration_verb(&amount_plan);
        assert_eq!(amount.frame_key().atoms(), [VerbFrameAtom::Amount]);
        assert!(amount.frame_key().matches_frame_set(&measure_complement));
        assert!(!amount.frame_key().matches_frame_set(&transitive));

        let predicative_plan = declaration_verb_plan_for(&quote::quote! { PredicativeComplement });
        let (_, predicative) = sole_declaration_verb(&predicative_plan);
        assert_eq!(
            predicative.frame_key().atoms(),
            [VerbFrameAtom::PredicativeComplement]
        );
        assert!(
            predicative
                .frame_key()
                .matches_frame_set(&VerbFrameSet::Custom {
                    frames: vec![vec![CustomTailAtom::PredicativeComplement]],
                })
        );

        let optional_predicative_plan =
            declaration_verb_plan_for(&quote::quote! { PredicativeComplement? });
        let (_, optional_predicative) = sole_declaration_verb(&optional_predicative_plan);
        assert_eq!(
            optional_predicative.frame_key().atoms(),
            [VerbFrameAtom::OptionalRole(
                "PredicativeComplement".to_owned()
            )]
        );
        assert!(
            !optional_predicative
                .frame_key()
                .matches_frame_set(&VerbFrameSet::Custom {
                    frames: vec![vec![CustomTailAtom::PredicativeComplement]],
                })
        );

        let rich_role_plan =
            declaration_verb_plan_for(&quote::quote! { ToDestination, PostState? });
        let (_, rich_role) = sole_declaration_verb(&rich_role_plan);
        assert_eq!(
            rich_role.frame_key().atoms(),
            [
                VerbFrameAtom::Role("ToDestination".to_owned()),
                VerbFrameAtom::OptionalRole("PostState".to_owned()),
            ]
        );

        let search_plan = declaration_verb_plan_for(&quote::quote! {
            location: ObjectNounPhrase,
            lex(Preposition::For),
            sought: ObjectNounPhrase
        });
        let (_, search) = sole_declaration_verb(&search_plan);
        assert_eq!(
            search.frame_key().atoms(),
            [
                VerbFrameAtom::ObjectNounPhrase,
                VerbFrameAtom::Lex("Preposition".to_owned(), "For".to_owned()),
                VerbFrameAtom::ObjectNounPhrase,
            ]
        );
        assert!(search.frame_key().matches_frame_set(&VerbFrameSet::Custom {
            frames: vec![vec![
                CustomTailAtom::ObjectNounPhrase,
                CustomTailAtom::Lex("Preposition".to_owned(), "For".to_owned()),
                CustomTailAtom::ObjectNounPhrase,
            ]],
        }));
        assert!(!search.frame_key().matches_frame_set(&transitive));

        let optional_preposition_plan =
            declaration_verb_plan_for(&quote::quote! { lex(Preposition::For)? });
        let (_, optional_preposition) = sole_declaration_verb(&optional_preposition_plan);
        assert_eq!(
            optional_preposition.frame_key().atoms(),
            [VerbFrameAtom::OptionalLex(
                "Preposition".to_owned(),
                "For".to_owned(),
            )]
        );

        let marked_role_plan = declaration_verb_plan_for(&quote::quote! {
            destination: marked(Preposition::For, ObjectNounPhrase)?
        });
        let (_, marked_role) = sole_declaration_verb(&marked_role_plan);
        assert_eq!(
            marked_role.frame_key().atoms(),
            [VerbFrameAtom::OptionalMarkedRole(
                "Preposition".to_owned(),
                "For".to_owned(),
                "ObjectNounPhrase".to_owned(),
            )]
        );

        let empty_plan = declaration_verb_plan_for(&quote::quote! {});
        let (_, empty) = sole_declaration_verb(&empty_plan);
        assert!(empty.frame_key().matches_frame_set(&intransitive));
        assert!(empty.frame_key().matches_frame_set(&VerbFrameSet::Custom {
            frames: vec![vec![], vec![CustomTailAtom::Amount]],
        }));
        assert!(!empty.frame_key().matches_frame_set(&transitive));

        let custom_plan = declaration_verb_plan_for(&quote::quote! { "with", ObjectNounPhrase });
        let (_, custom) = sole_declaration_verb(&custom_plan);
        assert_eq!(
            custom.frame_key().atoms(),
            [
                VerbFrameAtom::Literal("with".to_owned()),
                VerbFrameAtom::ObjectNounPhrase,
            ]
        );
        assert!(custom.frame_key().matches_frame_set(&VerbFrameSet::Custom {
            frames: vec![vec![
                CustomTailAtom::Literal("with".to_owned()),
                CustomTailAtom::ObjectNounPhrase,
            ]],
        }));
        assert!(
            !custom.frame_key().matches_frame_set(&VerbFrameSet::Custom {
                frames: vec![vec![
                    CustomTailAtom::Literal("with".to_owned()),
                    CustomTailAtom::Amount,
                ]],
            })
        );
        assert!(!custom.frame_key().matches_frame_set(&intransitive));
    }

    #[test]
    fn declaration_verb_frame_classes_are_sealed_and_semantic() {
        use crate::macro_def::VerbFrameSet;

        let plan = crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
                codec PredicateVerb {
                    generate declaration_verb {
                        position = Verb;
                        tail = [];
                        feature = ConcordClass;
                    }
                }
                codec AuxiliaryVerb {
                    generate declaration_verb {
                        class = Auxiliary;
                        position = Verb;
                        tail = [];
                        feature = ConcordClass;
                    }
                }
                codec ProVerb {
                    generate declaration_verb {
                        class = ProVerb;
                        position = Verb;
                        tail = [];
                        feature = ConcordClass;
                    }
                }
                construction only: Root {
                    element Only {}
                    form only = "only";
                }
                root Root { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect("sealed declaration_verb classes parse"),
        )
        .expect("identical tails in distinct classes validate")
        .into_semantic();
        let recipes = plan
            .runtime_declaration_verbs()
            .map(|(_, recipe)| recipe)
            .collect::<Vec<_>>();
        assert_eq!(recipes.len(), 3);
        assert_eq!(
            recipes[0].frame_key().class(),
            crate::semantic::VerbFrameClass::Predicate
        );
        assert_eq!(
            recipes[1].frame_key().class(),
            crate::semantic::VerbFrameClass::Auxiliary
        );
        assert_eq!(
            recipes[2].frame_key().class(),
            crate::semantic::VerbFrameClass::ProVerb
        );
        assert!(
            recipes[0]
                .frame_key()
                .matches_frame_set(&VerbFrameSet::Intransitive)
        );
        assert!(
            !recipes[1]
                .frame_key()
                .matches_frame_set(&VerbFrameSet::Intransitive)
        );
        assert!(
            !recipes[2]
                .frame_key()
                .matches_frame_set(&VerbFrameSet::Intransitive)
        );
    }

    #[test]
    fn sealed_open_identity_mutation_changes_emitters_without_changing_source() {
        let authored = crate::test_support::open_verb_tokens().to_string();
        let mut plan = crate::validate_declarations(
            crate::parse_declarations(crate::test_support::open_verb_tokens())
                .expect("open_verb syntax parses"),
        )
        .expect("open_verb declaration validates")
        .into_semantic();

        plan.test_only_replace_open_declaration_name("destroy", 0, "Connive");
        let rules = formatted(&crate::emit::rules::emit(&plan).expect("mutated rule emits"));
        let runtime = formatted(&crate::emit::runtime::emit(&plan));
        let render = formatted(&crate::emit::render::emit(&plan).expect("mutated render emits"));
        let visitor = formatted(&crate::emit::visit::emit(&plan).expect("mutated visitor emits"));
        assert!(rules.contains("name : \"Connive\""));
        assert!(runtime.contains("REQUIRED_DECLARATIONS"));
        assert!(runtime.contains("name : \"Connive\""));
        assert!(render.contains("\"Connive\""));
        assert!(visitor.contains("\"Connive\""));
        assert!(!rules.contains("name : \"Destroy\""));
        assert!(!runtime.contains("name : \"Destroy\""));
        assert_eq!(
            crate::test_support::open_verb_tokens().to_string(),
            authored
        );
    }

    #[test]
    fn semantic_plan_construction_emitters_share_one_mutated_fact() {
        let input = crate::test_support::representative_tokens().to_string();
        let mut plan = crate::test_support::representative_semantic_plan();
        plan.test_only_replace_planned_literal("first", 0, "changed");
        let ast = crate::emit::ast::emit(&plan).unwrap();
        let rules = crate::emit::rules::emit(&plan).unwrap();
        let build = crate::emit::build::emit(&plan).unwrap();
        assert!(formatted(&rules).contains("Literal (\"changed\")"));
        assert!(formatted(&build).contains("Leaf :: Literal (\"changed\")"));
        assert_eq!(named_types(&ast), expected_category_and_product_types());
        assert_eq!(
            crate::test_support::representative_tokens().to_string(),
            input
        );
    }

    #[test]
    fn semantic_plan_terminal_emitters_share_one_mutated_fact() {
        let input = crate::test_support::representative_tokens().to_string();
        let mut plan = crate::test_support::representative_semantic_plan();
        plan.test_only_replace_vocab_spelling("Words", "First", "changed");
        let terminal = crate::emit::terminal::emit(&plan).unwrap();
        assert_eq!(terminal_word(&terminal, "First"), "changed");
        assert!(formatted(&crate::emit::render::emit(&plan).unwrap()).contains("\"changed\""));

        let visitor_before = visitor_origins(&crate::emit::visit::emit(&plan).unwrap());
        let expected_visitor_after = visitor_before
            .iter()
            .map(
                |origin| {
                    if origin == "Words" { "ChangedWords".to_owned() } else { origin.clone() }
                },
            )
            .collect::<Vec<_>>();
        assert!(visitor_before.iter().any(|origin| origin == "Words"));
        plan.test_only_replace_vocab_name("Words", "ChangedWords");
        let visitor_after = visitor_origins(&crate::emit::visit::emit(&plan).unwrap());
        assert_eq!(visitor_after, expected_visitor_after);
        assert_eq!(
            crate::test_support::representative_tokens().to_string(),
            input
        );

        let binding_input = crate::test_support::synthetic_projection_tokens().to_string();
        let mut binding_plan = crate::validate_declarations(
            crate::parse_declarations(crate::test_support::synthetic_projection_tokens())
                .expect("synthetic fixture parses"),
        )
        .expect("synthetic fixture validates")
        .into_semantic();
        assert_eq!(
            report_binding_names(&binding_plan),
            ["Resource", "Marker", "Handle", "Pair", "Record"]
        );
        binding_plan.test_only_replace_binding_name("Resource", "ChangedResource");
        assert_eq!(
            report_binding_names(&binding_plan),
            ["ChangedResource", "Marker", "Handle", "Pair", "Record"]
        );
        assert_eq!(
            crate::test_support::synthetic_projection_tokens().to_string(),
            binding_input
        );

        let mut corrupt_plan = crate::validate_declarations(
            crate::parse_declarations(crate::test_support::synthetic_projection_tokens())
                .expect("synthetic fixture parses"),
        )
        .expect("synthetic fixture validates")
        .into_semantic();
        corrupt_plan.test_only_mismatch_binding_origin("Resource");
        assert_eq!(
            crate::generate_from_semantic(&corrupt_plan)
                .expect_err("generation propagates binding-origin mismatches")
                .to_string(),
            "sealed terminal binding `Resource` has a mismatched declaration kind"
        );
    }

    #[test]
    fn signed_decimal_plan_is_typed_and_shared_by_every_codec_emitter() {
        let source: proc_macro2::TokenStream = r#"
            codec SignedNumber {
                generate signed_decimal {
                    magnitude = u32;
                    sign_type = Sign { Positive = none, Negative = "-", };
                }
            }
            construction number: Amount {
                element NumberAmount { number: lex SignedNumber, }
                form number = lex(number);
            }
            root Amount { punctuation = "."; eoi = true; standalone_render = true; }
        "#
        .parse()
        .expect("signed-decimal fixture tokenizes");
        let authored = source.to_string();
        let mut plan = crate::validate_declarations(
            crate::parse_declarations(source.clone()).expect("signed-decimal fixture parses"),
        )
        .expect("signed-decimal fixture validates")
        .into_semantic();
        let codec = plan
            .terminals()
            .iter()
            .find_map(|terminal| match terminal {
                crate::semantic::TerminalPlan::SignedDecimal(codec) => Some(codec),
                _ => None,
            })
            .expect("typed signed-decimal plan exists");
        assert_eq!(codec.codec_name(), "SignedNumber");
        assert_eq!(codec.sign_type(), "Sign");
        assert_eq!(codec.positive_variant(), "Positive");
        assert_eq!(codec.negative_variant(), "Negative");
        assert_eq!(codec.magnitude(), crate::semantic::UnsignedPrimitive::U32);

        plan.test_only_replace_signed_decimal_sign_shape("Polarity", "Plus", "Minus");
        plan.test_only_replace_signed_decimal_codec_name("GeneratedNumber");
        let terminal = formatted(&crate::emit::terminal::emit(&plan).unwrap().0);
        let runtime = formatted(&crate::emit::runtime::emit(&plan));
        let scanner = formatted(&crate::emit::scanner::emit(&plan));
        let render = formatted(&crate::emit::render::emit(&plan).unwrap());
        let visitor = formatted(&crate::emit::visit::emit(&plan).unwrap());
        let rules = formatted(&crate::emit::rules::emit(&plan).unwrap());
        let build = formatted(&crate::emit::build::emit(&plan).unwrap());
        for output in [&terminal, &scanner, &render] {
            assert!(output.contains("Polarity"), "{output}");
            assert!(output.contains("Plus"), "{output}");
            assert!(output.contains("Minus"), "{output}");
            assert!(!output.contains("Sign :: Positive"), "{output}");
        }
        assert!(visitor.contains("Polarity"), "{visitor}");
        assert!(!visitor.contains("fn visit_sign ("), "{visitor}");
        for (phase, output) in [
            ("terminal", &terminal),
            ("runtime", &runtime),
            ("scanner", &scanner),
            ("rules", &rules),
            ("build", &build),
            ("visitor", &visitor),
        ] {
            assert!(output.contains("GeneratedNumber"), "{phase}: {output}");
            assert!(!output.contains("SignedNumber"), "{phase}: {output}");
        }
        assert!(
            rules.contains("stable_id : \"codec:GeneratedNumber\""),
            "{rules}"
        );
        assert!(!rules.contains("codec:SignedNumber"), "{rules}");
        assert_eq!(source.to_string(), authored);
    }

    #[test]
    fn unsigned_decimal_plan_routes_nonzero_magnitude_through_every_codec_emitter() {
        let source: proc_macro2::TokenStream = r#"
            codec ScalarNumber {
                generate unsigned_decimal { magnitude = u32; }
            }
            codec NonZeroScalarNumber {
                generate unsigned_decimal { magnitude = NonZeroU32; }
            }
            construction scalar: Amount {
                element ScalarAmount { number: lex ScalarNumber, }
                form scalar = lex(number);
            }
            construction nonzero: Amount {
                element NonZeroAmount { number: lex NonZeroScalarNumber, }
                form nonzero = lex(number);
            }
            root Amount { punctuation = "."; eoi = true; standalone_render = true; }
        "#
        .parse()
        .expect("unsigned-decimal fixture tokenizes");
        let authored = source.to_string();
        let plan = crate::validate_declarations(
            crate::parse_declarations(source.clone()).expect("unsigned-decimal fixture parses"),
        )
        .expect("unsigned-decimal fixture validates")
        .into_semantic();
        let codecs = plan
            .terminals()
            .iter()
            .filter_map(|terminal| match terminal {
                crate::semantic::TerminalPlan::UnsignedNumber(codec) => {
                    Some((codec.codec_name(), codec.magnitude()))
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(
            codecs,
            [
                ("ScalarNumber", crate::semantic::UnsignedPrimitive::U32),
                (
                    "NonZeroScalarNumber",
                    crate::semantic::UnsignedPrimitive::NonZeroU32,
                ),
            ]
        );

        let terminal = formatted(&crate::emit::terminal::emit(&plan).unwrap().0);
        assert!(terminal.contains("std :: num :: NonZeroU32"), "{terminal}");
        let rules = formatted(&crate::emit::rules::emit(&plan).unwrap());
        let expected_nonzero_rule_position = "LexicalTerminal { matcher : Lexical :: NonZeroScalarNumber , owner : LexicalOwnerTemplate :: Static { kind : LexicalProvenanceKind :: Codec , stable_id : \"codec:NonZeroScalarNumber\" , } , right_boundary : LexicalBoundary :: Separated , }";
        assert_eq!(
            rules.matches(expected_nonzero_rule_position).count(),
            1,
            "the nonzero rule must pair its exact matcher class with its owner: {rules}"
        );

        let outputs = [
            ("runtime", formatted(&crate::emit::runtime::emit(&plan))),
            ("scanner", formatted(&crate::emit::scanner::emit(&plan))),
            (
                "render",
                formatted(&crate::emit::render::emit(&plan).unwrap()),
            ),
            (
                "visitor",
                formatted(&crate::emit::visit::emit(&plan).unwrap()),
            ),
            ("rules", rules),
            (
                "build",
                formatted(&crate::emit::build::emit(&plan).unwrap()),
            ),
        ];
        for (phase, output) in outputs {
            assert!(output.contains("ScalarNumber"), "{phase}: {output}");
            assert!(output.contains("NonZeroScalarNumber"), "{phase}: {output}");
        }
        assert_eq!(source.to_string(), authored);
    }

    #[test]
    fn context_identity_plan_routes_each_typed_fact_to_its_consuming_emitters() {
        let source: proc_macro2::TokenStream = r#"
            identity SelfReferenceSpelling {
                generate context {
                    Full => card_name,
                    Abbreviated => abbreviated_card_name,
                    canonical_on_collision = Full;
                }
            }
            construction self_reference: NounPhrase {
                element SelfReferenceNp { spelling: identity SelfReferenceSpelling, }
                form self_reference = identity(spelling);
            }
            root NounPhrase { punctuation = "."; eoi = true; standalone_render = true; }
        "#
        .parse()
        .expect("context-identity fixture tokenizes");
        let authored = source.to_string();
        let mut plan = crate::validate_declarations(
            crate::parse_declarations(source.clone()).expect("context-identity fixture parses"),
        )
        .expect("context-identity fixture validates")
        .into_semantic();

        assert_eq!(
            plan.context_identity_snapshot(),
            [(
                "SelfReferenceSpelling".to_owned(),
                vec![
                    "Full=>card_name".to_owned(),
                    "Abbreviated=>abbreviated_card_name".to_owned(),
                ],
                "Full".to_owned(),
            )]
        );

        plan.test_only_replace_context_identity_shape(
            "SelfReferenceSpelling",
            "ReferenceMode",
            "Canonical",
            "Short",
        );
        let terminal_before_accessors = formatted(&crate::emit::terminal::emit(&plan).unwrap().0);
        let runtime_before_accessors = formatted(&crate::emit::runtime::emit(&plan));
        let scanner_before_accessors = formatted(&crate::emit::scanner::emit(&plan));
        let rules_before_accessors = formatted(&crate::emit::rules::emit(&plan).unwrap());
        let build_before_accessors = formatted(&crate::emit::build::emit(&plan).unwrap());
        let render_before_accessors = formatted(&crate::emit::render::emit(&plan).unwrap());
        let visitor_before_accessors = formatted(&crate::emit::visit::emit(&plan).unwrap());
        plan.test_only_replace_context_identity_accessors(
            "ReferenceMode",
            "abbreviated_card_name",
            "card_name",
        );
        let terminal = formatted(&crate::emit::terminal::emit(&plan).unwrap().0);
        let runtime = formatted(&crate::emit::runtime::emit(&plan));
        let scanner = formatted(&crate::emit::scanner::emit(&plan));
        let rules = formatted(&crate::emit::rules::emit(&plan).unwrap());
        let build = formatted(&crate::emit::build::emit(&plan).unwrap());
        let render = formatted(&crate::emit::render::emit(&plan).unwrap());
        let visitor = formatted(&crate::emit::visit::emit(&plan).unwrap());
        assert_ne!(terminal, terminal_before_accessors);
        assert_ne!(scanner, scanner_before_accessors);
        assert_eq!(runtime, runtime_before_accessors);
        assert_eq!(rules, rules_before_accessors);
        assert_eq!(build, build_before_accessors);
        assert_eq!(render, render_before_accessors);
        assert_eq!(visitor, visitor_before_accessors);
        for (phase, output) in [
            ("terminal", &terminal),
            ("runtime", &runtime),
            ("scanner", &scanner),
            ("rules", &rules),
            ("build", &build),
            ("visitor", &visitor),
        ] {
            assert!(output.contains("ReferenceMode"), "{phase}: {output}");
            assert!(
                !output.contains("SelfReferenceSpelling"),
                "{phase}: {output}"
            );
        }
        for output in [&terminal, &scanner, &visitor] {
            assert!(output.contains("Canonical"), "{output}");
            assert!(output.contains("Short"), "{output}");
        }
        assert!(
            terminal.contains("Self :: Canonical => context . abbreviated_card_name ()"),
            "{terminal}"
        );
        assert!(
            terminal.contains("Self :: Short => context . card_name ()"),
            "{terminal}"
        );
        assert!(
            scanner.contains(
                "(ReferenceMode :: Canonical , input . context . abbreviated_card_name ())"
            ),
            "{scanner}"
        );
        assert!(
            scanner.contains("(ReferenceMode :: Short , input . context . card_name ())"),
            "{scanner}"
        );
        assert!(render.contains(". surface (context)"), "{render}");
        assert!(!render.contains("SelfReferenceSpelling ::"), "{render}");
        assert!(
            rules.contains("Identity { declaration : \"ReferenceMode\" }"),
            "{rules}"
        );
        assert!(
            runtime.contains("identity:ReferenceMode/Canonical"),
            "{runtime}"
        );
        assert!(
            runtime.contains("identity:ReferenceMode/Short"),
            "{runtime}"
        );
        assert_eq!(source.to_string(), authored);
    }

    #[test]
    fn generated_vocab_scanner_is_exhaustive_and_owner_typed() {
        let mut plan = crate::validate_declarations(
            crate::parse_declarations(crate::test_support::representative_tokens())
                .expect("representative fixture parses"),
        )
        .expect("representative fixture validates")
        .into_semantic();
        plan.test_only_replace_vocab_spelling("Words", "First", "changed");

        let scanner = formatted(&crate::emit::scanner::emit(&plan));
        for expected in [
            "pub (crate) fn scan_lexical",
            "Lexical :: EndOfInput",
            "Lexical :: Literal",
            "Lexical :: Words",
            "(\"changed\" , Words :: First)",
            "Leaf :: Words (value)",
            "(Verbs :: Act , ConcordClass :: Other , Onset :: Vowel , \"act\")",
            "(Nouns :: Person , Number :: Singular , Onset :: Consonant , \"person\")",
            "Lexical :: Declaration (matcher)",
            "input . declaration_readings (matcher , terminal . right_boundary)",
        ] {
            assert!(
                scanner.contains(expected),
                "generated scanner is missing `{expected}`: {scanner}"
            );
        }
        assert!(
            !scanner.contains("\"first\""),
            "scanner reread authored source instead of the mutated semantic row: {scanner}"
        );
        assert!(
            !scanner.contains("scan_bound_terminal"),
            "closed lexemes must not use the binding seam: {scanner}"
        );

        let expansion = representative_expansion();
        let item = expansion
            .items()
            .iter()
            .find(|item| {
                matches!(
                    &item.key,
                    ItemKey::Named {
                        kind: NamedKind::Function,
                        name,
                    } if name == "scan_lexical"
                )
            })
            .expect("the planned expansion includes the generated scanner");
        assert_eq!(
            item.origins
                .iter()
                .map(|origin| (origin.kind(), origin.name()))
                .collect::<Vec<_>>(),
            [
                (SourceDeclarationKind::Vocab, "Words"),
                (SourceDeclarationKind::Lexeme, "Nouns"),
                (SourceDeclarationKind::Lexeme, "Verbs"),
                (SourceDeclarationKind::Codec, "SignedNumber"),
                (SourceDeclarationKind::Root, "Action"),
            ],
            "the generated scanner exposes exactly its sealed semantic authorities"
        );
    }

    #[test]
    fn generated_scanner_origins_cover_punctuation_constructions_and_runtime_bindings() {
        let representative = crate::test_support::representative_tokens();
        let expansion = crate::generate(quote::quote! {
            #representative
            construction separator: Node {
                element Separator {}
                form separator = "?";
            }
        })
        .expect("punctuation-bearing construction generates");
        let scanner_origins = |expansion: &crate::Expansion| {
            expansion
                .items()
                .iter()
                .find(|item| {
                    matches!(
                        &item.key,
                        ItemKey::Named {
                            kind: NamedKind::Function,
                            name,
                        } if name == "scan_lexical"
                    )
                })
                .expect("the expansion includes the generated scanner")
                .origins
                .iter()
                .map(|origin| (origin.kind(), origin.name().to_owned()))
                .collect::<Vec<_>>()
        };

        assert_eq!(
            scanner_origins(&expansion),
            [
                (SourceDeclarationKind::Vocab, "Words"),
                (SourceDeclarationKind::Lexeme, "Nouns"),
                (SourceDeclarationKind::Lexeme, "Verbs"),
                (SourceDeclarationKind::Codec, "SignedNumber"),
                (SourceDeclarationKind::Root, "Action"),
                (SourceDeclarationKind::Construction, "separator"),
            ]
            .map(|(kind, name)| (kind, name.to_owned()))
        );
        assert_eq!(
            scanner_origins(&crate::test_support::synthetic_projection_expansion()),
            [
                (SourceDeclarationKind::Vocab, "Mode"),
                (SourceDeclarationKind::Lexeme, "ObjectStem"),
                (SourceDeclarationKind::Lexeme, "ActionStem"),
                (SourceDeclarationKind::Codec, "Resource"),
                (SourceDeclarationKind::Codec, "Marker"),
                (SourceDeclarationKind::Identity, "Handle"),
                (SourceDeclarationKind::Codec, "Pair"),
                (SourceDeclarationKind::Root, "Document"),
            ]
            .map(|(kind, name)| (kind, name.to_owned())),
            "every scanner-owned binding contributes its source authority, while nonlexical Record does not"
        );
    }

    #[test]
    fn generated_vocab_scanner_rejects_empty_spelling_at_the_literal() {
        let error = crate::generate(quote::quote! {
            vocab Words { Empty = "", }
            construction leaf: Node {
                element WordLeaf { word: lex Words, }
                form leaf = lex(word);
            }
            root Node { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect_err("an empty vocab spelling cannot produce a bounded scanner match");

        assert_eq!(error.to_string(), "vocab spelling must not be empty");
    }

    #[test]
    fn generated_vocab_scanner_preserves_unicode_raw_names_and_initial_collisions() {
        let source = quote::quote! {
            vocab Terms {
                Unicode = "e\u{301}lan",
                r#RawMember = "raw",
                Running = "word",
                AlreadyInitial = "Word",
            }
            construction leaf: Node {
                element TermLeaf { term: lex Terms, }
                form leaf = lex(term);
            }
            root Node { punctuation = "."; eoi = true; standalone_render = true; }
        };
        let validated = crate::validate_declarations(
            crate::parse_declarations(source).expect("fixture parses"),
        )
        .expect("distinct running spellings remain valid when initial forms collide");
        let scanner_item = crate::emit::scanner::emit(validated.semantic())
            .into_iter()
            .next()
            .expect("scanner item");
        syn::parse2::<syn::ItemFn>(scanner_item.tokens.clone()).unwrap_or_else(|error| {
            panic!(
                "scanner with a raw member must reparse: {error}; tokens: {}",
                scanner_item.tokens
            )
        });
        let scanner = scanner_item.tokens.to_string();

        for expected in [
            "(\"e\\u{301}lan\" , Terms :: Unicode)",
            "(\"raw\" , Terms :: RawMember)",
            "(\"word\" , Terms :: Running)",
            "(\"Word\" , Terms :: AlreadyInitial)",
        ] {
            assert!(
                scanner.contains(expected),
                "missing `{expected}`: {scanner}"
            );
        }

        let duplicate = crate::generate(quote::quote! {
            vocab Terms { First = "same", Second = "same", }
            construction leaf: Node {
                element TermLeaf { term: lex Terms, }
                form leaf = lex(term);
            }
            root Node { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect_err("duplicate running spellings remain invalid");
        assert!(duplicate.to_string().contains("duplicate word `same`"));

        let unknown = crate::generate(quote::quote! {
            vocab Terms { Unknown = "élan", }
            construction leaf: Node {
                element TermLeaf { term: lex Terms, }
                form leaf = lex(term);
            }
            root Node { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect_err("an undecidable onset without an authored override is rejected");
        assert_eq!(
            unknown.to_string(),
            "vocab spelling has no bounded onset and no authored override"
        );
    }

    #[test]
    fn semantic_plan_binding_emitters_share_typed_build_and_value_facts() {
        let input = crate::test_support::synthetic_projection_tokens().to_string();
        let mut plan = crate::validate_declarations(
            crate::parse_declarations(crate::test_support::synthetic_projection_tokens())
                .expect("synthetic fixture parses"),
        )
        .expect("synthetic fixture validates")
        .into_semantic();

        plan.test_only_replace_binding_build_variant("Pair", "ChangedPair");
        let build = formatted(&crate::emit::build::emit(&plan).unwrap());
        assert!(build.contains("Leaf :: ChangedPair"));

        plan.test_only_replace_binding_value_type_name("Handle", "ChangedHandle");
        let render = formatted(&crate::emit::render::emit(&plan).unwrap());
        let visitor = formatted(&crate::emit::visit::emit(&plan).unwrap());
        assert!(render.contains("ChangedHandle :: Primary"));
        assert!(visitor.contains("ChangedHandle"));
        assert_eq!(
            crate::test_support::synthetic_projection_tokens().to_string(),
            input
        );
    }

    #[test]
    fn context_identity_generated_body_is_pinned() {
        let expansion = crate::generate(quote::quote! {
            identity SelfReferenceSpelling {
                generate context {
                    Full => card_name,
                    Abbreviated => abbreviated_card_name,
                    canonical_on_collision = Full;
                }
            }
            construction self_reference: NounPhrase {
                element SelfReferenceNp { spelling: identity SelfReferenceSpelling, }
                form self_reference = identity(spelling);
            }
            root NounPhrase { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("representative context identity generates");
        let actual = expansion
            .items()
            .iter()
            .filter(|item| {
                matches!(
                    &item.key,
                    ItemKey::Named {
                        kind: NamedKind::Type,
                        name,
                    } if name == "SelfReferenceSpelling"
                ) || matches!(
                    &item.key,
                    ItemKey::Impl {
                        trait_name: None,
                        self_ty,
                    } if self_ty == "SelfReferenceSpelling"
                )
            })
            .map(crate::format_generated_item)
            .collect::<syn::Result<Vec<_>>>()
            .expect("representative context identity items format")
            .join("");
        assert_eq!(
            actual,
            include_str!("../tests/golden/context-identity-expansion.txt")
        );
    }

    #[test]
    fn catalog_identity_emits_typed_provider_metadata() {
        let expansion = crate::generate(quote::quote! {
            vocab Supertype {
                Basic = "basic",
                Legendary = "legendary",
                Ongoing = "ongoing",
                Snow = "snow",
                World = "world",
            }
            identity CardName {
                generate catalog_identity { provider = CardNames; }
            }
            identity CardAlias {
                generate catalog_identity { provider = CardNames; }
            }
            construction named: NounPhrase {
                element NamedNp { name: identity CardName, }
                derive onset = name.onset;
                form vowel when name.onset is Vowel = "vowel" identity(name);
                form consonant otherwise = "consonant" identity(name);
            }
            root NounPhrase { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("representative catalog identity generates");

        assert_eq!(
            enum_variants(generated_item(&expansion, "Supertype")),
            ["Basic", "Legendary", "Ongoing", "Snow", "World"]
        );
        assert_eq!(
            enum_variants(generated_item(&expansion, "CatalogProvider")),
            ["CardNames"]
        );

        let card_name =
            syn::parse2::<syn::ItemStruct>(generated_item(&expansion, "CardName").tokens.clone())
                .expect("generated CardName is a typed struct");
        assert_eq!(
            card_name
                .fields
                .iter()
                .map(|field| {
                    let ty = &field.ty;
                    (
                        field.ident.as_ref().unwrap().to_string(),
                        quote::quote!(#ty).to_string(),
                    )
                })
                .collect::<Vec<_>>(),
            [
                ("provider".to_owned(), "CatalogProvider".to_owned()),
                (
                    "canonical_identity".to_owned(),
                    "std :: sync :: Arc < str >".to_owned(),
                ),
            ]
        );

        let named_item = |kind, name| {
            expansion
                .items()
                .iter()
                .find(|item| {
                    matches!(
                        &item.key,
                        ItemKey::Named {
                            kind: actual_kind,
                            name: actual_name,
                        } if *actual_kind == kind && actual_name == name
                    )
                })
                .unwrap_or_else(|| panic!("generated {kind:?} `{name}` exists"))
        };
        assert_eq!(
            named_item(NamedKind::Constant, "REQUIRED_CATALOG_PROVIDERS")
                .tokens
                .to_string(),
            "pub (crate) const REQUIRED_CATALOG_PROVIDERS : & [CatalogProvider] = & [CatalogProvider :: CardNames] ;"
        );

        assert!(
            expansion.items().iter().any(|item| {
                matches!(
                    &item.key,
                    ItemKey::Named {
                        kind: NamedKind::Function,
                        name,
                    } if name == "scan_lexical"
                )
            }),
            "catalog grammar emits its indexed scanner entry point"
        );
    }

    #[test]
    fn declaration_noun_generated_body_is_pinned() {
        let expansion = crate::generate(quote::quote! {
            morphology EnglishNoun { feature = Number; recipe = english_noun; }
            lexeme NounLexeme using EnglishNoun { Player = "player", }
            codec Noun {
                generate declaration_noun {
                    closed = NounLexeme;
                    position = Noun;
                    kinds = [Type, Subtype, TurnPart];
                    feature = Number;
                }
            }
            construction common: Phrase {
                element Common { head: lex Noun, }
                derive number = Values::Singular;
                form common = noun(head);
            }
            root Phrase { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("representative declaration noun generates");
        let actual = expansion
            .items()
            .iter()
            .filter(|item| {
                matches!(
                    &item.key,
                    ItemKey::Named {
                        kind: NamedKind::Type,
                        name,
                    } if matches!(name.as_str(), "DeclarationNoun" | "Noun")
                ) || matches!(
                    &item.key,
                    ItemKey::Impl {
                        trait_name: None,
                        self_ty,
                    } if self_ty == "DeclarationNoun"
                )
            })
            .map(crate::format_generated_item)
            .collect::<syn::Result<Vec<_>>>()
            .expect("representative declaration noun items format")
            .join("");
        assert_eq!(
            actual,
            include_str!("../tests/golden/declaration-noun-expansion.txt")
        );
    }

    fn declaration_verb_expansion() -> crate::Expansion {
        crate::generate(quote::quote! {
            morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
            lexeme CoreVerb using EnglishVerb { Act = "act", }
            vocab ObjectWord { Object = "object", }
            codec TransitiveVerb {
                generate declaration_verb {
                    closed = CoreVerb;
                    position = Verb;
                    tail = [ObjectNounPhrase];
                    feature = ConcordClass;
                }
            }
            construction transitive: VerbPhrase {
                element Transitive { head: lex TransitiveVerb, object: lex ObjectWord, }
                derive head.concord_class = Values::Other;
                form transitive = verb(head) lex(object);
            }
            root VerbPhrase { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("representative declaration verb generates")
    }

    #[test]
    fn declaration_verb_generated_body_is_pinned() {
        let expansion = declaration_verb_expansion();
        let actual = expansion
            .items()
            .iter()
            .filter(|item| {
                matches!(
                    &item.key,
                    ItemKey::Named {
                        kind: NamedKind::Type,
                        name,
                    } if matches!(name.as_str(), "DeclarationTransitiveVerb" | "TransitiveVerb")
                ) || matches!(
                    &item.key,
                    ItemKey::Impl {
                        trait_name: None,
                        self_ty,
                    } if self_ty == "DeclarationTransitiveVerb"
                )
            })
            .map(crate::format_generated_item)
            .collect::<syn::Result<Vec<_>>>()
            .expect("representative declaration verb items format")
            .join("");
        assert_eq!(
            actual,
            include_str!("../tests/golden/declaration-verb-expansion.txt")
        );
    }

    #[test]
    fn declaration_verb_expansion_carries_explicit_frame_classes() {
        let expansion = crate::generate(quote::quote! {
            codec AuxiliaryVerb {
                generate declaration_verb {
                    class = Auxiliary;
                    position = Verb;
                    tail = [];
                    feature = ConcordClass;
                }
            }
            codec ProVerb {
                generate declaration_verb {
                    class = ProVerb;
                    position = Verb;
                    tail = [];
                    feature = ConcordClass;
                }
            }
            construction auxiliary: Root {
                element AuxiliaryUse { head: lex AuxiliaryVerb, }
                derive head.concord_class = Values::Other;
                form auxiliary = verb(head);
            }
            construction pro_verb: Root {
                element ProVerbUse { head: lex ProVerb, }
                derive head.concord_class = Values::Other;
                form pro_verb = verb(head);
            }
            construction only: Root {
                element Only {}
                form only = "only";
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("explicit declaration verb classes generate");
        assert_eq!(
            enum_variants(generated_item(&expansion, "VerbFrameClass")),
            ["Predicate", "Auxiliary", "ProVerb"]
        );
        let emitted = expansion
            .items()
            .iter()
            .map(|item| item.tokens.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        assert!(emitted.contains("VerbFrameClass :: Auxiliary"), "{emitted}");
        assert!(emitted.contains("VerbFrameClass :: ProVerb"), "{emitted}");
    }

    #[test]
    fn generated_terminal_abi_is_derived_from_the_semantic_inventory() {
        let expansion = representative_expansion();

        assert_eq!(
            enum_variants(generated_item(&expansion, "ConcordClass")),
            ["Other", "ThirdPersonSingular"]
        );
        assert_eq!(
            enum_variants(generated_item(&expansion, "Number")),
            ["Singular", "Plural"]
        );
        assert_eq!(
            enum_variants(generated_item(&expansion, "CasePosition")),
            ["DocumentInitial", "SentenceInitial", "Continuation"]
        );
        assert_eq!(
            enum_variants(generated_item(&expansion, "PrefixPosition")),
            ["WordOwnedSpace", "SurfaceOwned", "None"]
        );
        assert_eq!(
            enum_variants(generated_item(&expansion, "StructuralTransition")),
            ["Preserve", "SentenceInitial", "Continuation"]
        );
        let scan_position =
            syn::parse2::<syn::File>(generated_item(&expansion, "ScanPosition").tokens.clone())
                .expect("ScanPosition parses");
        let syn::Item::Struct(scan_position) = &scan_position.items[0] else {
            panic!("ScanPosition is a struct")
        };
        assert_eq!(
            scan_position
                .fields
                .iter()
                .map(|field| field.ident.as_ref().unwrap().to_string())
                .collect::<Vec<_>>(),
            ["byte_offset", "case", "prefix"],
        );
        assert_eq!(
            enum_variants(generated_item(&expansion, "Lexical")),
            [
                "Literal",
                "EndOfInput",
                "Words",
                "Noun",
                "Verb",
                "SignedNumber",
                "Declaration",
            ]
        );
        assert_eq!(
            enum_variants(generated_item(&expansion, "Leaf")),
            [
                "Literal",
                "EndOfInput",
                "Words",
                "Noun",
                "Verb",
                "SignedNumber",
                "Declaration",
            ]
        );
        assert_eq!(
            enum_variants(generated_item(&expansion, "TerminalClass")),
            [
                "EndOfInput",
                "Words",
                "Noun",
                "VerbLexeme",
                "SignedNumber",
                "Declaration",
            ]
        );
        assert_eq!(
            enum_variants(generated_item(&expansion, "LexicalProvenanceKind")),
            ["FormLiteral", "Vocab", "Lexeme", "Codec", "Identity",]
        );

        let rules = expansion
            .items()
            .iter()
            .find(|item| {
                matches!(
                    &item.key,
                    ItemKey::Named {
                        kind: NamedKind::Constant,
                        name,
                    } if name == "RULES"
                )
            })
            .expect("generated rules constant")
            .tokens
            .to_string();
        assert!(
            rules.contains("Rule < Category , LexicalTerminal , RuleId >"),
            "generated rules carry owner-bearing terminals: {rules}"
        );
        for owner in [
            "Vocab { declaration : \"Words\" }",
            "Lexeme { declaration : \"Verbs\" , member : \"Act\" }",
        ] {
            assert!(rules.contains(owner), "missing owner `{owner}`: {rules}");
        }
        let root_adapter = generated_root_adapter(&expansion);
        for owner in [
            "stable_id : \"root:Action/punctuation\"",
            "owner : LexicalOwnerTemplate :: None",
        ] {
            assert!(
                root_adapter.contains(owner),
                "missing root adapter owner `{owner}`: {root_adapter}"
            );
        }

        for name in [
            "FeatureConstraint",
            "PrefixPosition",
            "StructuralTransition",
            "ScanPosition",
            "DeclarationMatcher",
            "DeclarationLeaf",
            "DeclarationClass",
            "LexicalTerminal",
            "LexicalOwnerTemplate",
            "LexicalOwnerIdentity",
            "LexicalOwner",
            "LexicalProvenanceKind",
        ] {
            generated_item(&expansion, name);
        }

        assert_eq!(
            generated_item(&expansion, "Lexical")
                .origins
                .iter()
                .map(|origin| (origin.kind(), origin.name()))
                .collect::<Vec<_>>(),
            [
                (SourceDeclarationKind::Vocab, "Words"),
                (SourceDeclarationKind::Lexeme, "Nouns"),
                (SourceDeclarationKind::Lexeme, "Verbs"),
                (SourceDeclarationKind::Codec, "SignedNumber"),
                (SourceDeclarationKind::Morphology, "EnglishNoun"),
                (SourceDeclarationKind::Morphology, "EnglishVerb"),
                (SourceDeclarationKind::Construction, "leaf"),
                (SourceDeclarationKind::Construction, "chain"),
                (SourceDeclarationKind::Construction, "action"),
                (SourceDeclarationKind::Root, "Action"),
            ]
        );
    }

    #[test]
    fn runtime_emitter_consumes_the_sealed_runtime_projection() {
        let input = crate::test_support::representative_tokens().to_string();
        let mut plan = crate::test_support::representative_semantic_plan();

        plan.test_only_remove_runtime_vocab("Words");
        let runtime = crate::emit::runtime::emit(&plan);
        assert_eq!(
            enum_variants(
                runtime
                    .iter()
                    .find(|item| item.key == ItemKey::named_type("Lexical"))
                    .expect("runtime lexical aggregate"),
            ),
            ["Literal", "EndOfInput", "Declaration"]
        );
        assert_eq!(
            crate::test_support::representative_tokens().to_string(),
            input
        );
    }

    #[test]
    fn sealed_construction_atoms_reject_count_and_kind_mismatches() {
        let source = crate::parse_declarations(crate::test_support::representative_tokens())
            .expect("representative fixture parses");
        let construction = source
            .declarations
            .iter()
            .find_map(|declaration| match declaration {
                crate::Declaration::Construction(construction) => Some(construction),
                crate::Declaration::AbstractProduct(_)
                | crate::Declaration::AbstractSum(_)
                | crate::Declaration::Vocab(_)
                | crate::Declaration::Morphology(_)
                | crate::Declaration::Lexeme(_)
                | crate::Declaration::Codec(_)
                | crate::Declaration::Identity(_)
                | crate::Declaration::Root(_) => None,
            })
            .expect("representative source has a construction");

        let count_error =
            SemanticPlan::test_only_seal_construction_atoms(&construction.forms[0], &[])
                .expect_err("a sealed atom list cannot silently truncate");
        assert!(count_error.to_string().contains("atom count"));

        let kind_error = SemanticPlan::test_only_seal_construction_atoms(
            &construction.forms[0],
            &[crate::validate::AtomContribution::Category {
                role: "ignored".to_owned(),
                category: "Node".to_owned(),
            }],
        )
        .expect_err("a sealed atom kind cannot panic");
        assert!(kind_error.to_string().contains("atom kind"));
    }

    #[test]
    fn sealed_construction_atoms_reject_every_name_mismatch() {
        let cases = [
            (
                parsed_form(&quote::quote! { child }),
                crate::validate::AtomContribution::Category {
                    role: "other".to_owned(),
                    category: "Cat".to_owned(),
                },
                "category atom role name",
            ),
            (
                parsed_form(&quote::quote! { lex(word) }),
                crate::validate::AtomContribution::Lex {
                    role: "other".to_owned(),
                    terminal: "Words".to_owned(),
                },
                "lex atom role name",
            ),
            (
                parsed_form(&quote::quote! { identity(owner) }),
                crate::validate::AtomContribution::Identity {
                    role: "other".to_owned(),
                    terminal: "Owner".to_owned(),
                },
                "identity atom role name",
            ),
            (
                parsed_form(&quote::quote! { noun(object) }),
                crate::validate::AtomContribution::Noun {
                    role: "other".to_owned(),
                    terminal: "Object".to_owned(),
                },
                "noun atom role name",
            ),
            (
                parsed_form(&quote::quote! { verb(Verbs::Act) }),
                crate::validate::AtomContribution::VerbFixed {
                    terminal: "OtherVerbs".to_owned(),
                    variant: "Act".to_owned(),
                },
                "fixed-verb terminal name",
            ),
            (
                parsed_form(&quote::quote! { verb(Verbs::Act) }),
                crate::validate::AtomContribution::VerbFixed {
                    terminal: "Verbs".to_owned(),
                    variant: "Other".to_owned(),
                },
                "fixed-verb variant name",
            ),
            (
                parsed_form(&quote::quote! { circumfix("[", child, "]") }),
                crate::validate::AtomContribution::Category {
                    role: "other".to_owned(),
                    category: "Cat".to_owned(),
                },
                "circumfix role name",
            ),
        ];

        for (form, resolved, expected) in cases {
            let error = SemanticPlan::test_only_seal_construction_atoms(&form, &[resolved])
                .expect_err("authored and resolved atom names must agree");
            assert!(error.to_string().contains(expected), "{error}");
        }
    }

    #[test]
    fn construction_emitters_do_not_fall_back_to_validated_raw_declarations() {
        for source in [
            include_str!("emit/ast.rs"),
            include_str!("emit/rules.rs"),
            include_str!("emit/build.rs"),
        ] {
            assert!(!source.contains("ValidatedDeclarations"));
            assert!(!source.contains(".raw()"));
        }
    }

    #[test]
    fn circumfix_emitters_contain_no_english_specific_production_branches() {
        for source in [
            include_str!("emit/build.rs"),
            include_str!("emit/render.rs"),
            include_str!("emit/rules.rs"),
            include_str!("emit/scanner.rs"),
            include_str!("emit/visit.rs"),
        ] {
            let production = source
                .rsplit_once("#[cfg(test)]\nmod tests")
                .map_or(source, |(production, _)| production);
            for forbidden in ["mana", "loyalty", "bracketed_value", "braced_values"] {
                assert!(
                    !production.to_ascii_lowercase().contains(forbidden),
                    "production circumfix lowering contains English-specific `{forbidden}`",
                );
            }
            for forbidden in ["\"[\"", "\"]\"", "\"{\"", "\"}\""] {
                assert!(
                    !production.contains(forbidden),
                    "production circumfix lowering contains fixed delimiter `{forbidden}`",
                );
            }
        }
    }

    #[test]
    fn binding_emitters_do_not_reclassify_authored_syntax() {
        for source in [
            include_str!("emit/build.rs"),
            include_str!("emit/render.rs"),
            include_str!("emit/visit.rs"),
        ] {
            let production = source
                .split_once("#[cfg(test)]")
                .map_or(source, |(production, _)| production);
            for forbidden in [
                "binding.pattern()",
                "binding.construct()",
                "build.pattern()",
                "build.construct()",
                "binding.value_type()",
                "syn::Pat::",
                "syn::Expr::",
                "syn::Type::",
                "binding_pattern",
                "lower_build_expr",
                "direct_bound_path",
                "simple_type_ident",
            ] {
                assert!(
                    !production.contains(forbidden),
                    "production emitter still reclassifies `{forbidden}`"
                );
            }
        }
    }

    fn parsed_form(atom: &proc_macro2::TokenStream) -> crate::Form {
        let source = crate::parse_declarations(quote::quote! {
            construction only: Cat {
                element Only {}
                form only = #atom;
            }
        })
        .expect("single-atom construction parses");
        source
            .declarations
            .into_iter()
            .find_map(|declaration| match declaration {
                crate::Declaration::Construction(construction) => {
                    construction.forms.into_iter().next()
                }
                crate::Declaration::AbstractProduct(_)
                | crate::Declaration::AbstractSum(_)
                | crate::Declaration::Vocab(_)
                | crate::Declaration::Morphology(_)
                | crate::Declaration::Lexeme(_)
                | crate::Declaration::Codec(_)
                | crate::Declaration::Identity(_)
                | crate::Declaration::Root(_) => None,
            })
            .expect("fixture has one construction")
    }

    fn formatted(items: &[GeneratedItem]) -> String {
        items
            .iter()
            .map(|item| item.tokens.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn terminal_word(
        emitted: &(Vec<GeneratedItem>, Vec<crate::TerminalContribution>),
        variant_name: &str,
    ) -> String {
        emitted
            .1
            .iter()
            .flat_map(crate::TerminalContribution::variants)
            .find(|variant| variant.name() == variant_name)
            .and_then(crate::TerminalVariantContribution::word)
            .expect("terminal variant has a word")
            .to_owned()
    }

    fn visitor_origins(items: &[GeneratedItem]) -> Vec<String> {
        items
            .iter()
            .flat_map(|item| item.origins.iter().map(crate::DeclarationKey::name))
            .map(str::to_owned)
            .collect()
    }

    fn report_binding_names(plan: &SemanticPlan) -> Vec<String> {
        crate::report::escape_hatch_report(plan)
            .expect("sealed report facts are consistent")
            .terminal_bindings()
            .iter()
            .map(|binding| binding.name().to_owned())
            .collect()
    }

    fn named_types(items: &[GeneratedItem]) -> Vec<String> {
        items
            .iter()
            .filter_map(|item| match &item.key {
                ItemKey::Named {
                    kind: NamedKind::Type,
                    name,
                } => Some(name.clone()),
                ItemKey::Named {
                    kind: NamedKind::Trait | NamedKind::Function | NamedKind::Constant,
                    ..
                }
                | ItemKey::Impl { .. } => None,
            })
            .collect()
    }

    fn generated_item<'a>(expansion: &'a crate::Expansion, name: &str) -> &'a GeneratedItem {
        expansion
            .items()
            .iter()
            .find(|item| {
                matches!(
                    &item.key,
                    ItemKey::Named {
                        kind: NamedKind::Type,
                        name: candidate,
                    } if candidate == name
                )
            })
            .unwrap_or_else(|| panic!("generated type `{name}` exists"))
    }

    fn generated_root_adapter(expansion: &crate::Expansion) -> String {
        expansion
            .items()
            .iter()
            .find(|item| {
                matches!(
                    &item.key,
                    ItemKey::Impl {
                        trait_name: None,
                        self_ty,
                    } if self_ty == "Category"
                ) && item.tokens.to_string().contains("root_adapter")
            })
            .expect("generated root adapter")
            .tokens
            .to_string()
    }

    fn enum_variants(item: &GeneratedItem) -> Vec<String> {
        let file = syn::parse2::<syn::File>(item.tokens.clone()).expect("generated item parses");
        let syn::Item::Enum(item) = &file.items[0] else {
            panic!("generated item is an enum")
        };
        item.variants
            .iter()
            .map(|variant| variant.ident.to_string())
            .collect()
    }

    fn expected_category_and_product_types() -> Vec<String> {
        ["Node", "First"].into_iter().map(str::to_owned).collect()
    }

    fn assert_representative_concord_class_match(expansion: &crate::Expansion) {
        let concord_class_match = expansion
            .items()
            .iter()
            .find(|item| {
                matches!(
                    &item.key,
                    ItemKey::Named {
                        kind: NamedKind::Function,
                        name,
                    } if name == "concord_class_matches_for_action"
                )
            })
            .expect("the contextual Action ConcordClass matcher is planned");
        assert_eq!(
            concord_class_match
                .origins
                .iter()
                .map(|origin| (origin.kind(), origin.name()))
                .collect::<Vec<_>>(),
            [(SourceDeclarationKind::Construction, "action")],
        );
    }

    #[test]
    #[allow(
        clippy::too_many_lines,
        reason = "the phase-order assertion pins one complete representative prefix"
    )]
    fn plan_is_unique_repeatable_and_phase_ordered_with_exact_origins() {
        let first = representative_expansion();
        let second = representative_expansion();

        let keys = first
            .items()
            .iter()
            .map(|item| &item.key)
            .collect::<Vec<_>>();
        assert_eq!(
            &keys[..17],
            &[
                &ItemKey::Named {
                    kind: NamedKind::Type,
                    name: "Node".into()
                },
                &ItemKey::Named {
                    kind: NamedKind::Type,
                    name: "Action".into()
                },
                &ItemKey::Named {
                    kind: NamedKind::Type,
                    name: "WordLeaf".into()
                },
                &ItemKey::Named {
                    kind: NamedKind::Type,
                    name: "Chain".into()
                },
                &ItemKey::Named {
                    kind: NamedKind::Type,
                    name: "ActionElement".into()
                },
                &ItemKey::Named {
                    kind: NamedKind::Trait,
                    name: "RightmostLeaf".into()
                },
                &ItemKey::Named {
                    kind: NamedKind::Trait,
                    name: "RightmostLeafCategory".into()
                },
                &ItemKey::Named {
                    kind: NamedKind::Function,
                    name: "rightmost_leaf_is".into()
                },
                &ItemKey::Impl {
                    trait_name: Some("RightmostLeafCategory".into()),
                    self_ty: "Node".into()
                },
                &ItemKey::Impl {
                    trait_name: Some("RightmostLeaf".into()),
                    self_ty: "Node".into()
                },
                &ItemKey::Impl {
                    trait_name: Some("RightmostLeafCategory".into()),
                    self_ty: "Action".into()
                },
                &ItemKey::Impl {
                    trait_name: Some("RightmostLeaf".into()),
                    self_ty: "Action".into()
                },
                &ItemKey::Named {
                    kind: NamedKind::Type,
                    name: "Words".into()
                },
                &ItemKey::Named {
                    kind: NamedKind::Type,
                    name: "Nouns".into()
                },
                &ItemKey::Named {
                    kind: NamedKind::Function,
                    name: "surface_for_nouns".into()
                },
                &ItemKey::Named {
                    kind: NamedKind::Type,
                    name: "Verbs".into()
                },
                &ItemKey::Named {
                    kind: NamedKind::Function,
                    name: "surface_for_verbs".into()
                },
            ]
        );
        assert_eq!(keys.len(), 125);
        assert_representative_concord_class_match(&first);
        assert!(keys.contains(&&ItemKey::Named {
            kind: NamedKind::Trait,
            name: "GeneratedRoot".into(),
        }));
        assert!(keys.contains(&&ItemKey::Named {
            kind: NamedKind::Trait,
            name: "GeneratedParseRoot".into(),
        }));
        assert!(keys.contains(&&ItemKey::Impl {
            trait_name: Some("GeneratedRoot".into()),
            self_ty: "Action".into(),
        }));
        assert!(keys.contains(&&ItemKey::Impl {
            trait_name: Some("GeneratedParseRoot".into()),
            self_ty: "Action".into(),
        }));
        assert!(keys.contains(&&ItemKey::Impl {
            trait_name: None,
            self_ty: "Category".into(),
        }));
        assert!(keys.iter().any(|key| {
            matches!(
                key,
                ItemKey::Named {
                    kind: NamedKind::Function,
                    name,
                } if name == "scan_lexical"
            )
        }));
        assert_eq!(
            keys.len(),
            keys.iter().copied().collect::<HashSet<_>>().len()
        );
        assert_eq!(
            first
                .items()
                .iter()
                .map(|item| &item.key)
                .collect::<Vec<_>>(),
            second
                .items()
                .iter()
                .map(|item| &item.key)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            first
                .items()
                .iter()
                .take(17)
                .map(|item| item
                    .origins
                    .iter()
                    .map(super::DeclarationKey::name)
                    .collect::<Vec<_>>())
                .collect::<Vec<_>>(),
            [
                vec!["leaf", "chain"],
                vec!["action"],
                vec!["leaf"],
                vec!["chain"],
                vec!["action"],
                vec!["leaf", "chain", "action"],
                vec!["leaf", "chain", "action"],
                vec!["leaf", "chain", "action"],
                vec!["leaf"],
                vec!["leaf"],
                vec!["action"],
                vec!["action"],
                vec!["Words"],
                vec!["Nouns"],
                vec!["Nouns"],
                vec!["Verbs"],
                vec!["Verbs"],
            ]
        );
        assert!(first.items()[..5].iter().all(|item| {
            item.origins
                .iter()
                .all(|origin| origin.kind() == SourceDeclarationKind::Construction)
        }));

        let formatted = crate::format_expansion(&first).expect("each planned item formats");
        syn::parse_file(&formatted).expect("formatted expansion reparses");
        assert_eq!(
            formatted,
            crate::format_expansion(&second).expect("repeat formats")
        );
    }

    #[test]
    fn representative_generated_body_uses_each_planned_item_exactly_once() {
        let expansion = representative_expansion();
        let flattened = syn::parse2::<syn::File>(expansion.tokens())
            .expect("flattened expansion is valid Rust");
        let individual = expansion
            .items()
            .iter()
            .map(|item| {
                let file = syn::parse2::<syn::File>(item.tokens.clone())
                    .expect("planned item is valid Rust");
                assert_eq!(file.items.len(), 1);
                file.items.into_iter().next().expect("one planned item")
            })
            .collect::<Vec<_>>();
        assert_eq!(flattened.items.len(), expansion.items().len());
        assert_eq!(flattened.items, individual);
        assert_eq!(
            prettyplease::unparse(&flattened),
            expansion
                .items()
                .iter()
                .map(|item| crate::format_generated_item(item).expect("planned item formats"))
                .collect::<String>()
        );
    }
}
