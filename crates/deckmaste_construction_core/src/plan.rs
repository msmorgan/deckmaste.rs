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
pub enum DeclarationKind {
    Construction,
    Vocab,
    Lexeme,
    Codec,
    Identity,
    Root,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeclarationKey {
    kind: DeclarationKind,
    name: String,
}

impl DeclarationKey {
    pub(crate) fn new(kind: DeclarationKind, name: impl Into<String>) -> Self {
        Self {
            kind,
            name: name.into(),
        }
    }

    #[must_use]
    pub fn kind(&self) -> DeclarationKind {
        self.kind
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn from_source(declaration: &Declaration) -> Self {
        match declaration {
            Declaration::Construction(value) => {
                Self::new(DeclarationKind::Construction, value.name.to_string())
            }
            Declaration::Vocab(value) => Self::new(DeclarationKind::Vocab, value.name.to_string()),
            Declaration::Lexeme(value) => {
                Self::new(DeclarationKind::Lexeme, value.name.to_string())
            }
            Declaration::Codec(value) => Self::new(DeclarationKind::Codec, value.name.to_string()),
            Declaration::Identity(value) => {
                Self::new(DeclarationKind::Identity, value.name.to_string())
            }
            Declaration::Root(value) => Self::new(
                DeclarationKind::Root,
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
    let (terminal_items, terminal_contributions) = crate::emit::terminal::emit(plan)?;
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

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::DeclarationKind;
    use crate::GeneratedItem;
    use crate::ItemKey;
    use crate::NamedKind;
    use crate::semantic::SemanticPlan;
    use crate::test_support::representative_expansion;

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
        let terminal = formatted(&crate::emit::terminal::emit(&plan).unwrap().0);
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
        assert!(rules.contains("codec:SignedNumber"));
        assert!(build.contains("Leaf :: SignedNumber"));
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
            "Lexical :: Declaration (matcher)",
            "input . declaration_readings (matcher)",
            "scan_bound_terminal (input , terminal)",
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
                (DeclarationKind::Vocab, "Words"),
                (DeclarationKind::Lexeme, "Nouns"),
                (DeclarationKind::Lexeme, "Verbs"),
                (DeclarationKind::Codec, "SignedNumber"),
                (DeclarationKind::Root, "Action"),
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
                (DeclarationKind::Vocab, "Words"),
                (DeclarationKind::Lexeme, "Nouns"),
                (DeclarationKind::Lexeme, "Verbs"),
                (DeclarationKind::Codec, "SignedNumber"),
                (DeclarationKind::Root, "Action"),
                (DeclarationKind::Construction, "separator"),
            ]
            .map(|(kind, name)| (kind, name.to_owned()))
        );
        assert_eq!(
            scanner_origins(&crate::test_support::synthetic_projection_expansion()),
            [
                (DeclarationKind::Vocab, "Mode"),
                (DeclarationKind::Lexeme, "ObjectStem"),
                (DeclarationKind::Lexeme, "ActionStem"),
                (DeclarationKind::Codec, "Resource"),
                (DeclarationKind::Codec, "Marker"),
                (DeclarationKind::Identity, "Handle"),
                (DeclarationKind::Codec, "Pair"),
                (DeclarationKind::Root, "Document"),
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
                Unicode = "élan",
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
            "(\"élan\" , Terms :: Unicode)",
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
    fn representative_generated_body_is_pinned() {
        let actual = crate::format_expansion(&representative_expansion())
            .expect("representative expansion formats");
        assert_eq!(
            actual,
            include_str!("../tests/golden/representative-expansion.txt")
        );
    }

    #[test]
    fn generated_terminal_abi_is_derived_from_the_semantic_inventory() {
        let expansion = representative_expansion();

        assert_eq!(
            enum_variants(generated_item(&expansion, "Agreement")),
            ["Bare", "ThirdPersonSingular"]
        );
        assert_eq!(
            enum_variants(generated_item(&expansion, "Number")),
            ["Singular", "Plural"]
        );
        assert_eq!(
            enum_variants(generated_item(&expansion, "CasePosition")),
            ["DocumentInitial", "Continuation"]
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
            [
                "FormLiteral",
                "Vocab",
                "Lexeme",
                "Codec",
                "Identity",
                "Declaration",
            ]
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
            "Lexeme { declaration : \"Verbs\"",
            "member : \"Act\"",
            "stable_id : \"root:Action/punctuation\"",
            "owner : LexicalOwnerTemplate :: None",
        ] {
            assert!(rules.contains(owner), "missing owner `{owner}`: {rules}");
        }

        for name in [
            "FeatureConstraint",
            "ScanPosition",
            "DeclarationMatcher",
            "DeclarationLeaf",
            "DeclarationClass",
            "LexicalTerminal",
            "LexicalOwnerTemplate",
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
                (DeclarationKind::Vocab, "Words"),
                (DeclarationKind::Lexeme, "Nouns"),
                (DeclarationKind::Lexeme, "Verbs"),
                (DeclarationKind::Codec, "SignedNumber"),
                (DeclarationKind::Construction, "leaf"),
                (DeclarationKind::Construction, "chain"),
                (DeclarationKind::Construction, "action"),
                (DeclarationKind::Root, "Action"),
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
                crate::Declaration::Vocab(_)
                | crate::Declaration::Lexeme(_)
                | crate::Declaration::Codec(_)
                | crate::Declaration::Identity(_)
                | crate::Declaration::Root(_) => None,
            })
            .expect("representative source has a construction");

        let count_error = SemanticPlan::test_only_seal_construction_atoms(&construction.form, &[])
            .expect_err("a sealed atom list cannot silently truncate");
        assert!(count_error.to_string().contains("atom count"));

        let kind_error = SemanticPlan::test_only_seal_construction_atoms(
            &construction.form,
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
                crate::Declaration::Construction(construction) => Some(construction.form),
                crate::Declaration::Vocab(_)
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

    #[test]
    fn plan_is_unique_repeatable_and_phase_ordered_with_exact_origins() {
        let first = representative_expansion();
        let second = representative_expansion();

        let keys = first
            .items()
            .iter()
            .map(|item| &item.key)
            .collect::<Vec<_>>();
        assert_eq!(
            &keys[..8],
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
                    kind: NamedKind::Type,
                    name: "Words".into()
                },
                &ItemKey::Named {
                    kind: NamedKind::Type,
                    name: "Nouns".into()
                },
                &ItemKey::Named {
                    kind: NamedKind::Type,
                    name: "Verbs".into()
                },
            ]
        );
        assert_eq!(keys.len(), 54);
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
                .take(8)
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
                vec!["Words"],
                vec!["Nouns"],
                vec!["Verbs"],
            ]
        );
        assert!(first.items()[..5].iter().all(|item| {
            item.origins
                .iter()
                .all(|origin| origin.kind() == DeclarationKind::Construction)
        }));

        let formatted = crate::format_expansion(&first).expect("each planned item formats");
        syn::parse_file(&formatted).expect("formatted expansion reparses");
        assert_eq!(
            formatted,
            crate::format_expansion(&second).expect("repeat formats")
        );
    }

    #[test]
    fn public_flattened_tokens_contain_each_planned_item_exactly_once() {
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
