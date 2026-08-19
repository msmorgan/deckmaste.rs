use std::fs;
use std::path::Path;

use deckmaste_english_v2::ast::NounLexeme;
use deckmaste_english_v2::catalogs::ParserCatalogs;
use deckmaste_english_v2::context::ParseContext;
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
use macro_ron::v2::SurfaceFeature;
use macro_ron::v2::read_builtin_v2;
use macro_ron::v2::read_str;
use syn::visit::Visit;

fn declaration(path: &str, source: &str) -> NormalizedDeclaration {
    read_str(path, source).expect("synthetic normalized declaration is valid")
}

fn builtin_rows() -> Vec<NormalizedDeclaration> {
    read_builtin_v2(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin_v2"))
        .expect("integrated builtin-v2 declarations load")
}

fn production_environment() -> ParserEnvironment {
    let environment = ParserEnvironment::try_from_declarations(builtin_rows())
        .expect("builtin-v2 declarations freeze");
    ParserCatalogs::load(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/gen/catalogs"))
        .expect("canonical catalogs load")
        .attach_to(environment)
}

fn parser() -> Parser {
    Parser::new(production_environment()).expect("required declarations are present")
}

fn context() -> ParseContext<'static> {
    ParseContext::new("Context Card").expect("context is valid")
}

#[test]
fn parser_build_rejects_missing_wrong_recipe_and_missing_agreement_surface() {
    let missing = Parser::new(ParserEnvironment::try_from_declarations([]).unwrap())
        .expect_err("static declarations must exist before parsing");
    assert!(matches!(
        missing,
        ParserBuildError::MissingDeclaration { kind: DeclarationKind::KeywordAction, ref name }
            if name == "Destroy" || name == "Connive"
    ));

    let wrong_recipe = ParserEnvironment::try_from_declarations([
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

    let missing_feature = ParserEnvironment::try_from_declarations([
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
fn builtin_open_verbs_parse_and_render_both_agreements_exactly() {
    let parser = parser();
    let context = context();
    for text in [
        "Destroy target creature.",
        "That creature destroys target creature.",
        "Connive.",
        "It connives.",
    ] {
        let ability = parser.parse(text, &context).unwrap_or_else(|error| {
            panic!("builtin declaration surface must parse `{text}`: {error}")
        });
        assert_eq!(ability.render(&context, parser.environment()), text);
    }

    for text in ["It destroy target creature.", "That creature connive."] {
        assert!(
            parser.parse(text, &context).is_err(),
            "wrong agreement parsed: {text}"
        );
    }
}

#[test]
fn category_homonym_does_not_replace_the_requested_action_identity() {
    let mut rows = builtin_rows();
    rows.push(declaration(
        "/synthetic/abilities/Destroy.ron",
        r#"KeywordAbility(name:"Destroy",spelling:"destroy",grammar:Verb(bare:"destroy",valence:Transitive))"#,
    ));
    let environment = ParserEnvironment::try_from_declarations(rows).unwrap();
    assert_eq!(
        environment.readings(GrammarPosition::Verb, "destroy").len(),
        2,
        "the environment must retain both category-safe identities"
    );
    let environment = ParserCatalogs::load(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/gen/catalogs"),
    )
    .unwrap()
    .attach_to(environment);
    let parser = Parser::new(environment).unwrap();
    let trace = parser.trace(
        "Destroy target creature.",
        &context(),
        TraceLimits::new(usize::MAX),
    );
    let declaration_tokens = trace
        .tokens()
        .items()
        .iter()
        .filter(|token| token.terminal_name_v1().contains("Declaration"))
        .collect::<Vec<_>>();
    assert_eq!(declaration_tokens.len(), 1);
    assert!(
        declaration_tokens[0]
            .value_label_v1()
            .contains("KeywordAction")
    );
    assert!(
        !declaration_tokens[0]
            .value_label_v1()
            .contains("KeywordAbility")
    );

    let ability_only = ParserEnvironment::try_from_declarations([declaration(
        "/synthetic/abilities/Destroy.ron",
        r#"KeywordAbility(name:"Destroy",spelling:"destroy",grammar:Verb(bare:"destroy",valence:Transitive))"#,
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

    fn visit_noun_lexeme(&mut self, noun: NounLexeme) {
        self.events.push(format!("noun:{noun:?}"));
    }
}

#[test]
fn visitor_observes_owned_declaration_identity_in_form_order() {
    let parser = parser();
    let ability = parser
        .parse("Destroy target player.", &context())
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
fn destroy_and_connive_have_no_closed_member_or_handwritten_spelling_authority() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let constructions = fs::read_to_string(root.join("constructions.rs")).unwrap();
    let features = fs::read_to_string(root.join("features.rs")).unwrap();
    let scanner = fs::read_to_string(root.join("parser/scan.rs")).unwrap();

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
        ["Deal", "Gain", "Control", "Be"]
    );

    let feature_literals = production_string_literals(&features);
    let scanner_literals = production_string_literals(&scanner);
    for forbidden in ["destroy", "destroys", "connive", "connives"] {
        assert!(
            !feature_literals.iter().any(|literal| literal == forbidden),
            "handwritten inflection remains: {forbidden}"
        );
        assert!(
            !scanner_literals.iter().any(|literal| literal == forbidden),
            "handwritten scanner spelling remains: {forbidden}"
        );
    }
}
