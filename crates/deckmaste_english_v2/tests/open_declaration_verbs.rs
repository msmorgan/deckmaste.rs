use std::fs;
use std::path::Path;

use deckmaste_english_v2::ast::CatalogProvider;
use deckmaste_english_v2::ast::CommonNoun;
use deckmaste_english_v2::context::ParseContext;
use deckmaste_english_v2::environment::CatalogProviderRow;
use deckmaste_english_v2::environment::CatalogProviderRows;
use deckmaste_english_v2::environment::DeclarationId;
use deckmaste_english_v2::environment::ParserEnvironment;
use deckmaste_english_v2::parser::Parser;
use deckmaste_english_v2::parser::ParserBuildError;
use deckmaste_english_v2::parser::TraceLimits;
use deckmaste_english_v2::render::Render;
use deckmaste_english_v2::visit::Visitor;
use deckmaste_english_v2::visit::walk_ability;
use macro_ron::v2::DeclarationKind;
use macro_ron::v2::GrammarPosition;
use macro_ron::v2::NormalizedDeclaration;
use macro_ron::v2::Onset;
use macro_ron::v2::SurfaceFeature;
use macro_ron::v2::read_str;
use syn::visit::Visit;

fn declaration(path: &str, source: &str) -> NormalizedDeclaration {
    read_str(path, source).expect("synthetic normalized declaration is valid")
}

fn synthetic_verb_rows() -> Vec<NormalizedDeclaration> {
    vec![
        declaration(
            "/synthetic/actions/Destroy.ron",
            r#"KeywordAction(name:"Destroy",spelling:"frindle",grammar:Verb(bare:"frindle",third_person:"frondles",valence:Transitive))"#,
        ),
        declaration(
            "/synthetic/actions/Connive.ron",
            r#"KeywordAction(name:"Connive",spelling:"zorble",grammar:Verb(bare:"zorble",third_person:"zurbles",valence:Intransitive))"#,
        ),
    ]
}

fn synthetic_environment() -> ParserEnvironment {
    environment_from(synthetic_verb_rows()).expect("synthetic verb declarations freeze")
}

fn environment_from(
    declarations: impl IntoIterator<Item = NormalizedDeclaration>,
) -> Result<ParserEnvironment, deckmaste_english_v2::environment::ParserEnvironmentError> {
    ParserEnvironment::try_from_parts(
        declarations,
        [CatalogProviderRows::new(
            CatalogProvider::CardNames,
            [CatalogProviderRow::new(
                "context-card",
                "Context Card",
                Onset::Consonant,
            )],
        )],
    )
}

fn parser() -> Parser {
    Parser::new(synthetic_environment()).expect("required declarations are present")
}

fn context() -> ParseContext<'static> {
    ParseContext::new("Context Card").expect("context is valid")
}

#[test]
fn parser_build_rejects_missing_wrong_recipe_and_missing_agreement_surface() {
    let missing = Parser::new(environment_from([]).unwrap())
        .expect_err("static declarations must exist before parsing");
    assert!(matches!(
        missing,
        ParserBuildError::MissingDeclaration { kind: DeclarationKind::KeywordAction, ref name }
            if name == "Destroy" || name == "Connive"
    ));

    let wrong_recipe = environment_from([
        declaration(
            "/synthetic/actions/Destroy.ron",
            r#"KeywordAction(name:"Destroy",spelling:"destroy",grammar:FixedTerm(surface:"destroy"))"#,
        ),
        declaration(
            "/synthetic/actions/Connive.ron",
            r#"KeywordAction(name:"Connive",spelling:"connive",grammar:Verb(bare:"connive",valence:Intransitive))"#,
        ),
    ])
    .unwrap();
    assert!(matches!(
        Parser::new(wrong_recipe).expect_err("fixed-term Destroy is not a verb"),
        ParserBuildError::WrongGrammarPosition {
            expected: GrammarPosition::Verb,
            actual: Some(GrammarPosition::FixedTerm),
            ..
        }
    ));

    let missing_feature = environment_from([
        declaration(
            "/synthetic/actions/Destroy.ron",
            r#"KeywordAction(name:"Destroy",spelling:"destroy",grammar:Verb(bare:"destroy",third_person:Unavailable,valence:Transitive))"#,
        ),
        declaration(
            "/synthetic/actions/Connive.ron",
            r#"KeywordAction(name:"Connive",spelling:"connive",grammar:Verb(bare:"connive",valence:Intransitive))"#,
        ),
    ])
    .unwrap();
    assert!(matches!(
        Parser::new(missing_feature).expect_err("both agreement surfaces are required"),
        ParserBuildError::MissingSurfaceFeature {
            feature: SurfaceFeature::ThirdPersonSingular,
            ..
        }
    ));
}

#[test]
fn open_declaration_synthetic_verbs_parse_and_render_both_agreements_exactly() {
    let parser = parser();
    let context = context();
    for text in [
        "Frindle target player.",
        "That player frondles target player.",
        "Zorble.",
        "It zurbles.",
    ] {
        let ability = parser.parse(text, &context).unwrap_or_else(|error| {
            panic!("synthetic declaration surface must parse `{text}`: {error}")
        });
        assert_eq!(ability.render(&context, parser.environment()), text);
    }

    for text in ["It frindle target player.", "That player zorble."] {
        assert!(
            parser.parse(text, &context).is_err(),
            "wrong agreement parsed: {text}"
        );
    }
}

#[test]
fn category_homonym_does_not_replace_the_requested_action_identity() {
    let mut rows = synthetic_verb_rows();
    rows.push(declaration(
        "/synthetic/abilities/Destroy.ron",
        r#"KeywordAbility(name:"Destroy",spelling:"frindle",grammar:Verb(bare:"frindle",third_person:"frondles",valence:Transitive))"#,
    ));
    let environment = environment_from(rows).unwrap();
    assert_eq!(
        environment.readings(GrammarPosition::Verb, "frindle").len(),
        2,
        "the environment must retain both category-safe identities"
    );
    let parser = Parser::new(environment).unwrap();
    let trace = parser.trace(
        "Frindle target player.",
        &context(),
        TraceLimits::new(usize::MAX),
    );
    let declaration_matches = trace
        .scanner_matches()
        .items()
        .iter()
        .filter(|scanner_match| scanner_match.terminal_name_v1().starts_with("Declaration("))
        .collect::<Vec<_>>();
    assert_eq!(declaration_matches.len(), 1);
    assert!(
        declaration_matches[0]
            .value_label_v1()
            .contains("KeywordAction")
    );
    assert!(
        !declaration_matches[0]
            .value_label_v1()
            .contains("KeywordAbility")
    );

    let ability_only = environment_from([declaration(
        "/synthetic/abilities/Destroy.ron",
        r#"KeywordAbility(name:"Destroy",spelling:"frindle",grammar:Verb(bare:"frindle",third_person:"frondles",valence:Transitive))"#,
    )])
    .unwrap();
    assert!(matches!(
        Parser::new(ability_only).expect_err("another category cannot satisfy the request"),
        ParserBuildError::MissingDeclaration {
            kind: DeclarationKind::KeywordAction,
            ..
        }
    ));
}

#[derive(Default)]
struct IdentityVisitor {
    events: Vec<String>,
}

impl Visitor for IdentityVisitor {
    fn visit_declaration(&mut self, id: &DeclarationId) {
        self.events
            .push(format!("declaration:{:?}/{}", id.kind(), id.name()));
    }

    fn visit_common_noun(&mut self, noun: CommonNoun) {
        self.events.push(format!("noun:{noun:?}"));
    }
}

#[test]
fn visitor_observes_owned_declaration_identity_in_form_order() {
    let parser = parser();
    let ability = parser
        .parse("Frindle target player.", &context())
        .expect("open declaration form parses");
    let mut visitor = IdentityVisitor::default();
    visitor.visit_ability(&ability);
    assert_eq!(
        visitor.events,
        ["declaration:KeywordAction/Destroy", "noun:Player"]
    );

    let mut explicit = IdentityVisitor::default();
    walk_ability(&mut explicit, &ability);
    assert_eq!(explicit.events, visitor.events);
}

fn item_attributes(item: &syn::Item) -> &[syn::Attribute] {
    match item {
        syn::Item::Const(item) => &item.attrs,
        syn::Item::Enum(item) => &item.attrs,
        syn::Item::ExternCrate(item) => &item.attrs,
        syn::Item::Fn(item) => &item.attrs,
        syn::Item::ForeignMod(item) => &item.attrs,
        syn::Item::Impl(item) => &item.attrs,
        syn::Item::Macro(item) => &item.attrs,
        syn::Item::Mod(item) => &item.attrs,
        syn::Item::Static(item) => &item.attrs,
        syn::Item::Struct(item) => &item.attrs,
        syn::Item::Trait(item) => &item.attrs,
        syn::Item::TraitAlias(item) => &item.attrs,
        syn::Item::Type(item) => &item.attrs,
        syn::Item::Union(item) => &item.attrs,
        syn::Item::Use(item) => &item.attrs,
        syn::Item::Verbatim(_) => &[],
        _ => panic!("unrecognized Rust item in production source census"),
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ProductionCfg {
    Enabled,
    Disabled,
    Unknown,
}

fn production_cfg(meta: &syn::Meta) -> ProductionCfg {
    match meta {
        syn::Meta::Path(path) if path.is_ident("test") => ProductionCfg::Disabled,
        syn::Meta::Path(_) | syn::Meta::NameValue(_) => ProductionCfg::Unknown,
        syn::Meta::List(list) => {
            let parser = syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated;
            let predicates = syn::parse::Parser::parse2(parser, list.tokens.clone())
                .expect("production cfg predicate parses");
            if list.path.is_ident("not") {
                assert_eq!(predicates.len(), 1, "cfg(not(...)) has one predicate");
                match production_cfg(&predicates[0]) {
                    ProductionCfg::Enabled => ProductionCfg::Disabled,
                    ProductionCfg::Disabled => ProductionCfg::Enabled,
                    ProductionCfg::Unknown => ProductionCfg::Unknown,
                }
            } else if list.path.is_ident("all") {
                if predicates
                    .iter()
                    .any(|predicate| production_cfg(predicate) == ProductionCfg::Disabled)
                {
                    ProductionCfg::Disabled
                } else if predicates
                    .iter()
                    .all(|predicate| production_cfg(predicate) == ProductionCfg::Enabled)
                {
                    ProductionCfg::Enabled
                } else {
                    ProductionCfg::Unknown
                }
            } else if list.path.is_ident("any") {
                if predicates
                    .iter()
                    .any(|predicate| production_cfg(predicate) == ProductionCfg::Enabled)
                {
                    ProductionCfg::Enabled
                } else if predicates
                    .iter()
                    .all(|predicate| production_cfg(predicate) == ProductionCfg::Disabled)
                {
                    ProductionCfg::Disabled
                } else {
                    ProductionCfg::Unknown
                }
            } else {
                ProductionCfg::Unknown
            }
        }
    }
}

fn is_cfg_test(attribute: &syn::Attribute) -> bool {
    if !attribute.path().is_ident("cfg") {
        return false;
    }
    let syn::Meta::List(list) = &attribute.meta else {
        panic!("cfg attribute has a predicate")
    };
    let predicate =
        syn::parse2::<syn::Meta>(list.tokens.clone()).expect("production cfg attribute parses");
    production_cfg(&predicate) == ProductionCfg::Disabled
}

fn expression_attributes(expression: &syn::Expr) -> &[syn::Attribute] {
    match expression {
        syn::Expr::Array(expression) => &expression.attrs,
        syn::Expr::Assign(expression) => &expression.attrs,
        syn::Expr::Async(expression) => &expression.attrs,
        syn::Expr::Await(expression) => &expression.attrs,
        syn::Expr::Binary(expression) => &expression.attrs,
        syn::Expr::Block(expression) => &expression.attrs,
        syn::Expr::Break(expression) => &expression.attrs,
        syn::Expr::Call(expression) => &expression.attrs,
        syn::Expr::Cast(expression) => &expression.attrs,
        syn::Expr::Closure(expression) => &expression.attrs,
        syn::Expr::Const(expression) => &expression.attrs,
        syn::Expr::Continue(expression) => &expression.attrs,
        syn::Expr::Field(expression) => &expression.attrs,
        syn::Expr::ForLoop(expression) => &expression.attrs,
        syn::Expr::Group(expression) => &expression.attrs,
        syn::Expr::If(expression) => &expression.attrs,
        syn::Expr::Index(expression) => &expression.attrs,
        syn::Expr::Infer(expression) => &expression.attrs,
        syn::Expr::Let(expression) => &expression.attrs,
        syn::Expr::Lit(expression) => &expression.attrs,
        syn::Expr::Loop(expression) => &expression.attrs,
        syn::Expr::Macro(expression) => &expression.attrs,
        syn::Expr::Match(expression) => &expression.attrs,
        syn::Expr::MethodCall(expression) => &expression.attrs,
        syn::Expr::Paren(expression) => &expression.attrs,
        syn::Expr::Path(expression) => &expression.attrs,
        syn::Expr::Range(expression) => &expression.attrs,
        syn::Expr::RawAddr(expression) => &expression.attrs,
        syn::Expr::Reference(expression) => &expression.attrs,
        syn::Expr::Repeat(expression) => &expression.attrs,
        syn::Expr::Return(expression) => &expression.attrs,
        syn::Expr::Struct(expression) => &expression.attrs,
        syn::Expr::Try(expression) => &expression.attrs,
        syn::Expr::TryBlock(expression) => &expression.attrs,
        syn::Expr::Tuple(expression) => &expression.attrs,
        syn::Expr::Unary(expression) => &expression.attrs,
        syn::Expr::Unsafe(expression) => &expression.attrs,
        syn::Expr::While(expression) => &expression.attrs,
        syn::Expr::Yield(expression) => &expression.attrs,
        syn::Expr::Verbatim(_) => &[],
        _ => panic!("unrecognized Rust expression in production source census"),
    }
}

#[derive(Default)]
struct StringLiteralCensus(Vec<String>);

fn visit_macro_string_literals(tokens: proc_macro2::TokenStream, literals: &mut Vec<String>) {
    for token in tokens {
        match token {
            proc_macro2::TokenTree::Group(group) => {
                visit_macro_string_literals(group.stream(), literals);
            }
            proc_macro2::TokenTree::Literal(literal) => {
                if let Ok(literal) = syn::parse_str::<syn::LitStr>(&literal.to_string()) {
                    literals.push(literal.value());
                }
            }
            proc_macro2::TokenTree::Ident(_) | proc_macro2::TokenTree::Punct(_) => {}
        }
    }
}

impl<'ast> Visit<'ast> for StringLiteralCensus {
    fn visit_item(&mut self, item: &'ast syn::Item) {
        if !item_attributes(item).iter().any(is_cfg_test) {
            syn::visit::visit_item(self, item);
        }
    }

    fn visit_expr(&mut self, expression: &'ast syn::Expr) {
        if !expression_attributes(expression).iter().any(is_cfg_test) {
            syn::visit::visit_expr(self, expression);
        }
    }

    fn visit_arm(&mut self, arm: &'ast syn::Arm) {
        if !arm.attrs.iter().any(is_cfg_test) {
            syn::visit::visit_arm(self, arm);
        }
    }

    fn visit_field_value(&mut self, field: &'ast syn::FieldValue) {
        if !field.attrs.iter().any(is_cfg_test) {
            syn::visit::visit_field_value(self, field);
        }
    }

    fn visit_impl_item(&mut self, item: &'ast syn::ImplItem) {
        let attributes: &[syn::Attribute] = match item {
            syn::ImplItem::Const(item) => &item.attrs,
            syn::ImplItem::Fn(item) => &item.attrs,
            syn::ImplItem::Type(item) => &item.attrs,
            syn::ImplItem::Macro(item) => &item.attrs,
            syn::ImplItem::Verbatim(_) => &[],
            _ => panic!("unrecognized impl item in production source census"),
        };
        if !attributes.iter().any(is_cfg_test) {
            syn::visit::visit_impl_item(self, item);
        }
    }

    fn visit_trait_item(&mut self, item: &'ast syn::TraitItem) {
        let attributes: &[syn::Attribute] = match item {
            syn::TraitItem::Const(item) => &item.attrs,
            syn::TraitItem::Fn(item) => &item.attrs,
            syn::TraitItem::Type(item) => &item.attrs,
            syn::TraitItem::Macro(item) => &item.attrs,
            syn::TraitItem::Verbatim(_) => &[],
            _ => panic!("unrecognized trait item in production source census"),
        };
        if !attributes.iter().any(is_cfg_test) {
            syn::visit::visit_trait_item(self, item);
        }
    }

    fn visit_foreign_item(&mut self, item: &'ast syn::ForeignItem) {
        let attributes: &[syn::Attribute] = match item {
            syn::ForeignItem::Fn(item) => &item.attrs,
            syn::ForeignItem::Static(item) => &item.attrs,
            syn::ForeignItem::Type(item) => &item.attrs,
            syn::ForeignItem::Macro(item) => &item.attrs,
            syn::ForeignItem::Verbatim(_) => &[],
            _ => panic!("unrecognized foreign item in production source census"),
        };
        if !attributes.iter().any(is_cfg_test) {
            syn::visit::visit_foreign_item(self, item);
        }
    }

    fn visit_variant(&mut self, variant: &'ast syn::Variant) {
        if !variant.attrs.iter().any(is_cfg_test) {
            syn::visit::visit_variant(self, variant);
        }
    }

    fn visit_field(&mut self, field: &'ast syn::Field) {
        if !field.attrs.iter().any(is_cfg_test) {
            syn::visit::visit_field(self, field);
        }
    }

    fn visit_generic_param(&mut self, parameter: &'ast syn::GenericParam) {
        let attributes = match parameter {
            syn::GenericParam::Lifetime(parameter) => &parameter.attrs,
            syn::GenericParam::Type(parameter) => &parameter.attrs,
            syn::GenericParam::Const(parameter) => &parameter.attrs,
        };
        if !attributes.iter().any(is_cfg_test) {
            syn::visit::visit_generic_param(self, parameter);
        }
    }

    fn visit_local(&mut self, local: &'ast syn::Local) {
        if !local.attrs.iter().any(is_cfg_test) {
            syn::visit::visit_local(self, local);
        }
    }

    fn visit_stmt_macro(&mut self, statement: &'ast syn::StmtMacro) {
        if !statement.attrs.iter().any(is_cfg_test) {
            syn::visit::visit_stmt_macro(self, statement);
        }
    }

    fn visit_macro(&mut self, macro_: &'ast syn::Macro) {
        visit_macro_string_literals(macro_.tokens.clone(), &mut self.0);
    }

    fn visit_lit_str(&mut self, literal: &'ast syn::LitStr) {
        self.0.push(literal.value());
    }
}

fn production_string_literals(source: &str) -> Vec<String> {
    let file = syn::parse_file(source).expect("production Rust source parses");
    let mut census = StringLiteralCensus::default();
    census.visit_file(&file);
    census.0
}

#[test]
fn source_census_excludes_nested_cfg_test_and_keeps_later_production() {
    let source = r#"
        fn before() { consume("deal"); }
        #[cfg(test)]
        fn test_helper() { consume("ignored"); }
        fn after() {
            #[cfg(test)]
            { consume("connive"); }
            #[cfg(not(test))]
            { consume("control"); }
            consume("destroys");
        }
        struct Fixture;
        impl Fixture {
            #[cfg(test)]
            fn test_only() { consume("destroy"); }
            #[cfg(not(test))]
            fn production() { consume("gain"); }
        }
        fn arms(value: bool) {
            match value {
                #[cfg(test)]
                true => consume("connives"),
                false => consume("be"),
            }
        }
    "#;
    assert_eq!(
        production_string_literals(source),
        ["deal", "control", "destroys", "gain", "be"]
    );
}

#[test]
fn source_census_visits_production_macro_token_groups() {
    let source = r##"
        fn macro_literals(candidate: &str) {
            matches!(candidate, "destroy" | nested!("destroys", [r#"connive"#, 7, 'x']));
            #[cfg(test)]
            matches!(candidate, "connives");
            #[cfg(not(test))]
            production_macro!({ "control" }, ["gain"]);
        }
    "##;
    assert_eq!(
        production_string_literals(source),
        ["destroy", "destroys", "connive", "control", "gain"]
    );
}

#[test]
fn generated_morphology_is_the_only_closed_spelling_authority() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let constructions = fs::read_to_string(root.join("constructions.rs")).unwrap();
    let scanner = fs::read_to_string(root.join("parser/scan.rs")).unwrap();
    let library = fs::read_to_string(root.join("lib.rs")).unwrap();

    assert!(
        !root.join("features.rs").exists(),
        "handwritten morphology module remains"
    );
    assert!(
        !library.contains("mod features"),
        "handwritten morphology module remains wired"
    );

    let invocation = deckmaste_construction_core::invocation_from_source(&constructions)
        .expect("production construction invocation is authentic");
    let expansion = deckmaste_construction_core::generate(invocation.tokens)
        .expect("production construction inventory compiles");
    let verb_lexeme = expansion
        .terminal_contributions()
        .iter()
        .find(|terminal| terminal.name() == "VerbLexeme")
        .expect("closed verb lexeme provider exists");
    assert!(verb_lexeme.is_verb_provider());
    assert_eq!(
        verb_lexeme
            .variants()
            .iter()
            .map(deckmaste_construction_core::TerminalVariantContribution::name)
            .collect::<Vec<_>>(),
        ["Deal", "Gain", "Control", "Own", "Be"]
    );
    assert_eq!(
        verb_lexeme
            .surfaces()
            .iter()
            .map(|row| (row.member(), row.feature(), row.surface()))
            .collect::<Vec<_>>(),
        [
            ("Deal", SurfaceFeature::Bare, "deal"),
            ("Deal", SurfaceFeature::ThirdPersonSingular, "deals"),
            ("Gain", SurfaceFeature::Bare, "gain"),
            ("Gain", SurfaceFeature::ThirdPersonSingular, "gains"),
            ("Control", SurfaceFeature::Bare, "control"),
            ("Control", SurfaceFeature::ThirdPersonSingular, "controls"),
            ("Own", SurfaceFeature::Bare, "own"),
            ("Own", SurfaceFeature::ThirdPersonSingular, "owns"),
            ("Be", SurfaceFeature::Bare, "are"),
            ("Be", SurfaceFeature::ThirdPersonSingular, "is"),
        ]
    );
    let noun_lexeme = expansion
        .terminal_contributions()
        .iter()
        .find(|terminal| terminal.name() == "CommonNoun")
        .expect("closed noun lexeme provider exists");
    assert_eq!(
        noun_lexeme
            .surfaces()
            .iter()
            .map(|row| (row.member(), row.feature(), row.surface()))
            .collect::<Vec<_>>(),
        [
            ("Card", SurfaceFeature::Singular, "card"),
            ("Card", SurfaceFeature::Plural, "cards"),
            ("Controller", SurfaceFeature::Singular, "controller"),
            ("Controller", SurfaceFeature::Plural, "controllers"),
            ("Opponent", SurfaceFeature::Singular, "opponent"),
            ("Opponent", SurfaceFeature::Plural, "opponents"),
            ("Owner", SurfaceFeature::Singular, "owner"),
            ("Owner", SurfaceFeature::Plural, "owners"),
            ("Permanent", SurfaceFeature::Singular, "permanent"),
            ("Permanent", SurfaceFeature::Plural, "permanents"),
            ("Player", SurfaceFeature::Singular, "player"),
            ("Player", SurfaceFeature::Plural, "players"),
            ("Source", SurfaceFeature::Singular, "source"),
            ("Source", SurfaceFeature::Plural, "sources"),
            ("Spell", SurfaceFeature::Singular, "spell"),
            ("Spell", SurfaceFeature::Plural, "spells"),
            ("Target", SurfaceFeature::Singular, "target"),
            ("Target", SurfaceFeature::Plural, "targets"),
            ("Token", SurfaceFeature::Singular, "token"),
            ("Token", SurfaceFeature::Plural, "tokens"),
        ]
    );
    let generated = expansion.tokens().to_string();
    for owner in [
        "lexeme:VerbLexeme/Deal/bare",
        "lexeme:VerbLexeme/Deal/third_person_singular",
        "lexeme:VerbLexeme/Be/bare",
        "lexeme:VerbLexeme/Be/third_person_singular",
        "lexeme:CommonNoun/Card/singular",
        "lexeme:CommonNoun/Card/plural",
        "lexeme:CommonNoun/Controller/singular",
        "lexeme:CommonNoun/Controller/plural",
        "lexeme:CommonNoun/Opponent/singular",
        "lexeme:CommonNoun/Opponent/plural",
        "lexeme:CommonNoun/Owner/singular",
        "lexeme:CommonNoun/Owner/plural",
        "lexeme:CommonNoun/Permanent/singular",
        "lexeme:CommonNoun/Permanent/plural",
        "lexeme:CommonNoun/Player/singular",
        "lexeme:CommonNoun/Player/plural",
        "lexeme:CommonNoun/Source/singular",
        "lexeme:CommonNoun/Source/plural",
        "lexeme:CommonNoun/Spell/singular",
        "lexeme:CommonNoun/Spell/plural",
        "lexeme:CommonNoun/Target/singular",
        "lexeme:CommonNoun/Target/plural",
        "lexeme:CommonNoun/Token/singular",
        "lexeme:CommonNoun/Token/plural",
    ] {
        assert!(
            generated.contains(owner),
            "generated scan/render provenance lacks `{owner}`"
        );
    }

    let scanner_literals = production_string_literals(&scanner);
    for forbidden in ["inflect", "scan_verb", "scan_bound_terminal"] {
        assert!(
            !scanner.contains(forbidden),
            "handwritten scanner/morphology seam remains: {forbidden}"
        );
    }
    for forbidden in [
        "deal", "deals", "gain", "gains", "control", "controls", "are", "is",
    ] {
        assert!(
            !scanner_literals.iter().any(|literal| literal == forbidden),
            "source-string surface mirror remains: {forbidden}"
        );
    }
}
