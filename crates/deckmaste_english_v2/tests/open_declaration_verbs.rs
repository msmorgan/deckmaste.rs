use std::fs;
use std::path::Path;

use deckmaste_english_v2::ast::CatalogProvider;
use deckmaste_english_v2::ast::CommonNoun;
use deckmaste_english_v2::ast::DeclarationTransitiveVerb;
use deckmaste_english_v2::context::ParseContext;
use deckmaste_english_v2::environment::CatalogProviderRow;
use deckmaste_english_v2::environment::CatalogProviderRows;
use deckmaste_english_v2::environment::DeclarationId;
use deckmaste_english_v2::environment::ParserEnvironment;
use deckmaste_english_v2::parser::Parser;
use deckmaste_english_v2::render::Render;
use deckmaste_english_v2::visit::Visitor;
use deckmaste_english_v2::visit::walk_ability;
use macro_ron::v2::CustomTailAtom;
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
    ParseContext::new("Context Card", false, macro_ron::v2::Onset::Consonant)
        .expect("context is valid")
}

#[test]
fn environment_declaration_verb_readings_use_literal_exact_frame_membership() {
    let environment = environment_from([
        declaration(
            "/synthetic/actions/FirstAct.ron",
            r#"KeywordAction(name:"FirstAct",spelling:"act",grammar:Verb(bare:"act",valence:Transitive))"#,
        ),
        declaration(
            "/synthetic/actions/SecondAct.ron",
            r#"KeywordAction(name:"SecondAct",spelling:"act",grammar:Verb(bare:"act",valence:Custom(shapes:[[ObjectNounPhrase]])))"#,
        ),
        declaration(
            "/synthetic/actions/Rest.ron",
            r#"KeywordAction(name:"Rest",spelling:"rest",grammar:Verb(bare:"rest",valence:Intransitive))"#,
        ),
        declaration(
            "/synthetic/actions/Count.ron",
            r#"KeywordAction(name:"Count",spelling:"count",grammar:Verb(bare:"count",valence:Numerative))"#,
        ),
        declaration(
            "/synthetic/actions/Shape.ron",
            r#"KeywordAction(name:"Shape",spelling:"shape",grammar:Verb(bare:"shape",valence:Custom(shapes:[[],[Amount]])))"#,
        ),
        declaration(
            "/synthetic/actions/Cross.ron",
            r#"KeywordAction(name:"Cross",spelling:"cross",grammar:Verb(bare:"cross",third_person:"crosses",valence:Custom(shapes:[[ObjectNounPhrase,Amount]])))"#,
        ),
        declaration(
            "/synthetic/abilities/WrongKind.ron",
            r#"KeywordAbility(name:"WrongKind",spelling:"act",grammar:Verb(bare:"act",valence:Transitive))"#,
        ),
        declaration(
            "/synthetic/actions/WrongPosition.ron",
            r#"KeywordAction(name:"WrongPosition",spelling:"act",grammar:FixedTerm(surface:"act"))"#,
        ),
    ])
    .expect("synthetic declaration frame matrix freezes");
    let names = |surface, feature, frame: &[CustomTailAtom]| {
        environment
            .declaration_verb_readings(&[DeclarationKind::KeywordAction], surface, feature, frame)
            .into_iter()
            .map(|reading| reading.id().name().to_owned())
            .collect::<Vec<_>>()
    };

    assert_eq!(
        names(
            "act",
            SurfaceFeature::Bare,
            &[CustomTailAtom::ObjectNounPhrase]
        ),
        ["FirstAct", "SecondAct"]
    );
    assert_eq!(names("rest", SurfaceFeature::Bare, &[]), ["Rest"]);
    assert_eq!(
        names("count", SurfaceFeature::Bare, &[CustomTailAtom::Amount]),
        ["Count"]
    );
    assert_eq!(names("shape", SurfaceFeature::Bare, &[]), ["Shape"]);
    assert_eq!(
        names("shape", SurfaceFeature::Bare, &[CustomTailAtom::Amount]),
        ["Shape"]
    );
    assert!(
        names(
            "shape",
            SurfaceFeature::Bare,
            &[CustomTailAtom::ObjectNounPhrase]
        )
        .is_empty()
    );
    assert!(
        names(
            "cross",
            SurfaceFeature::Bare,
            &[CustomTailAtom::ObjectNounPhrase]
        )
        .is_empty()
    );
}

#[test]
fn environment_declaration_verb_readings_filter_kind_position_surface_and_agreement() {
    let environment = environment_from([
        declaration(
            "/synthetic/actions/Right.ron",
            r#"KeywordAction(name:"Right",spelling:"echo",grammar:Verb(bare:"echo",third_person:"echoes",valence:Transitive))"#,
        ),
        declaration(
            "/synthetic/abilities/WrongKind.ron",
            r#"KeywordAbility(name:"WrongKind",spelling:"echo",grammar:Verb(bare:"echo",third_person:"echoes",valence:Transitive))"#,
        ),
        declaration(
            "/synthetic/actions/WrongPosition.ron",
            r#"KeywordAction(name:"WrongPosition",spelling:"echo",grammar:FixedTerm(surface:"echo"))"#,
        ),
        declaration(
            "/synthetic/actions/MissingThird.ron",
            r#"KeywordAction(name:"MissingThird",spelling:"wane",grammar:Verb(bare:"wane",third_person:Unavailable,valence:Transitive))"#,
        ),
    ])
    .expect("synthetic declaration filters freeze");
    let names = |kinds: &[DeclarationKind], surface, feature| {
        environment
            .declaration_verb_readings(kinds, surface, feature, &[CustomTailAtom::ObjectNounPhrase])
            .into_iter()
            .map(|reading| reading.id().name().to_owned())
            .collect::<Vec<_>>()
    };

    assert_eq!(
        names(
            &[DeclarationKind::KeywordAction],
            "echo",
            SurfaceFeature::Bare
        ),
        ["Right"]
    );
    assert_eq!(
        names(
            &[DeclarationKind::KeywordAction],
            "echoes",
            SurfaceFeature::ThirdPersonSingular
        ),
        ["Right"]
    );
    assert!(
        names(
            &[DeclarationKind::KeywordAction],
            "echo",
            SurfaceFeature::ThirdPersonSingular
        )
        .is_empty()
    );
    assert!(
        names(
            &[DeclarationKind::KeywordAction],
            "wane",
            SurfaceFeature::ThirdPersonSingular
        )
        .is_empty()
    );
    assert_eq!(
        names(
            &[DeclarationKind::KeywordAbility],
            "echo",
            SurfaceFeature::Bare
        ),
        ["WrongKind"]
    );
    assert!(
        names(
            &[DeclarationKind::KeywordAction],
            "missing",
            SurfaceFeature::Bare
        )
        .is_empty()
    );
}

#[test]
fn parser_build_has_no_fixed_keyword_requirements_and_constructors_fail_closed() {
    Parser::new(environment_from([]).unwrap())
        .expect("the shared frames have no fixed keyword-name requirements");

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
    let wrong_id = DeclarationId::new(DeclarationKind::KeywordAction, "Destroy");
    assert!(DeclarationTransitiveVerb::new(&wrong_recipe, wrong_id).is_none());
    Parser::new(wrong_recipe).expect("wrong recipes do not become fixed parser requirements");

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
    let missing_id = DeclarationId::new(DeclarationKind::KeywordAction, "Destroy");
    assert!(DeclarationTransitiveVerb::new(&missing_feature, missing_id).is_none());
    Parser::new(missing_feature)
        .expect("missing agreement surfaces do not become fixed parser requirements");
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
    let ability = parser
        .parse("Frindle target player.", &context())
        .expect("only the requested declaration category reaches the shared frame");
    let mut visitor = IdentityVisitor::default();
    visitor.visit_ability(&ability);
    assert_eq!(
        visitor.events.first().map(String::as_str),
        Some("declaration:KeywordAction/Destroy")
    );

    let ability_only = environment_from([declaration(
        "/synthetic/abilities/Destroy.ron",
        r#"KeywordAbility(name:"Destroy",spelling:"frindle",grammar:Verb(bare:"frindle",third_person:"frondles",valence:Transitive))"#,
    )])
    .unwrap();
    let parser = Parser::new(ability_only)
        .expect("another declaration category is not a fixed parser requirement");
    assert!(parser.parse("Frindle target player.", &context()).is_err());
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
#[allow(
    clippy::too_many_lines,
    reason = "the literal closed morphology authority is intentionally exhaustive"
)]
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
        [
            "May", "Can", "Cant", "Must", "Didnt", "Would", "Add", "Choose", "Deal", "Draw",
            "Enter", "Gain", "Get", "Lose", "Pay", "Prevent", "Put", "Remove", "Roll", "Have",
            "Look", "Leave", "Control", "Own", "Return", "Cause", "Become", "Cost", "Do", "Be"
        ]
    );
    assert_eq!(
        verb_lexeme
            .surfaces()
            .iter()
            .map(|row| (row.member(), row.feature(), row.surface()))
            .collect::<Vec<_>>(),
        [
            ("May", SurfaceFeature::Bare, "may"),
            ("May", SurfaceFeature::ThirdPersonSingular, "may"),
            ("Can", SurfaceFeature::Bare, "can"),
            ("Can", SurfaceFeature::ThirdPersonSingular, "can"),
            ("Cant", SurfaceFeature::Bare, "can't"),
            ("Cant", SurfaceFeature::ThirdPersonSingular, "can't"),
            ("Must", SurfaceFeature::Bare, "must"),
            ("Must", SurfaceFeature::ThirdPersonSingular, "must"),
            ("Didnt", SurfaceFeature::Bare, "didn't"),
            ("Didnt", SurfaceFeature::ThirdPersonSingular, "didn't"),
            ("Would", SurfaceFeature::Bare, "would"),
            ("Would", SurfaceFeature::ThirdPersonSingular, "would"),
            ("Add", SurfaceFeature::Bare, "add"),
            ("Add", SurfaceFeature::ThirdPersonSingular, "adds"),
            ("Choose", SurfaceFeature::Bare, "choose"),
            ("Choose", SurfaceFeature::ThirdPersonSingular, "chooses"),
            ("Deal", SurfaceFeature::Bare, "deal"),
            ("Deal", SurfaceFeature::ThirdPersonSingular, "deals"),
            ("Draw", SurfaceFeature::Bare, "draw"),
            ("Draw", SurfaceFeature::ThirdPersonSingular, "draws"),
            ("Enter", SurfaceFeature::Bare, "enter"),
            ("Enter", SurfaceFeature::ThirdPersonSingular, "enters"),
            ("Gain", SurfaceFeature::Bare, "gain"),
            ("Gain", SurfaceFeature::ThirdPersonSingular, "gains"),
            ("Get", SurfaceFeature::Bare, "get"),
            ("Get", SurfaceFeature::ThirdPersonSingular, "gets"),
            ("Lose", SurfaceFeature::Bare, "lose"),
            ("Lose", SurfaceFeature::ThirdPersonSingular, "loses"),
            ("Pay", SurfaceFeature::Bare, "pay"),
            ("Pay", SurfaceFeature::ThirdPersonSingular, "pays"),
            ("Prevent", SurfaceFeature::Bare, "prevent"),
            ("Prevent", SurfaceFeature::ThirdPersonSingular, "prevents"),
            ("Put", SurfaceFeature::Bare, "put"),
            ("Put", SurfaceFeature::ThirdPersonSingular, "puts"),
            ("Remove", SurfaceFeature::Bare, "remove"),
            ("Remove", SurfaceFeature::ThirdPersonSingular, "removes"),
            ("Roll", SurfaceFeature::Bare, "roll"),
            ("Roll", SurfaceFeature::ThirdPersonSingular, "rolls"),
            ("Have", SurfaceFeature::Bare, "have"),
            ("Have", SurfaceFeature::ThirdPersonSingular, "has"),
            ("Look", SurfaceFeature::Bare, "look"),
            ("Look", SurfaceFeature::ThirdPersonSingular, "looks"),
            ("Leave", SurfaceFeature::Bare, "leave"),
            ("Leave", SurfaceFeature::ThirdPersonSingular, "leaves"),
            ("Control", SurfaceFeature::Bare, "control"),
            ("Control", SurfaceFeature::ThirdPersonSingular, "controls"),
            ("Own", SurfaceFeature::Bare, "own"),
            ("Own", SurfaceFeature::ThirdPersonSingular, "owns"),
            ("Return", SurfaceFeature::Bare, "return"),
            ("Return", SurfaceFeature::ThirdPersonSingular, "returns"),
            ("Cause", SurfaceFeature::Bare, "cause"),
            ("Cause", SurfaceFeature::ThirdPersonSingular, "causes"),
            ("Become", SurfaceFeature::Bare, "become"),
            ("Become", SurfaceFeature::ThirdPersonSingular, "becomes"),
            ("Cost", SurfaceFeature::Bare, "cost"),
            ("Cost", SurfaceFeature::ThirdPersonSingular, "costs"),
            ("Do", SurfaceFeature::Bare, "do"),
            ("Do", SurfaceFeature::ThirdPersonSingular, "does"),
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
            ("Ability", SurfaceFeature::Singular, "ability"),
            ("Ability", SurfaceFeature::Plural, "abilities"),
            ("Attacker", SurfaceFeature::Singular, "attacker"),
            ("Attacker", SurfaceFeature::Plural, "attackers"),
            ("Battlefield", SurfaceFeature::Singular, "battlefield"),
            ("Battlefield", SurfaceFeature::Plural, "battlefields"),
            ("Blocker", SurfaceFeature::Singular, "blocker"),
            ("Blocker", SurfaceFeature::Plural, "blockers"),
            ("Card", SurfaceFeature::Singular, "card"),
            ("Card", SurfaceFeature::Plural, "cards"),
            ("Choice", SurfaceFeature::Singular, "choice"),
            ("Choice", SurfaceFeature::Plural, "choices"),
            ("Coin", SurfaceFeature::Singular, "coin"),
            ("Coin", SurfaceFeature::Plural, "coins"),
            ("Color", SurfaceFeature::Singular, "color"),
            ("Color", SurfaceFeature::Plural, "colors"),
            ("Combat", SurfaceFeature::Singular, "combat"),
            ("Combat", SurfaceFeature::Plural, "combats"),
            ("Copy", SurfaceFeature::Singular, "copy"),
            ("Copy", SurfaceFeature::Plural, "copies"),
            ("Counter", SurfaceFeature::Singular, "counter"),
            ("Counter", SurfaceFeature::Plural, "counters"),
            ("Damage", SurfaceFeature::Singular, "damage"),
            ("Damage", SurfaceFeature::Plural, "damages"),
            ("Death", SurfaceFeature::Singular, "death"),
            ("Death", SurfaceFeature::Plural, "deaths"),
            ("Draw", SurfaceFeature::Singular, "draw"),
            ("Draw", SurfaceFeature::Plural, "draws"),
            ("Exile", SurfaceFeature::Singular, "exile"),
            ("Exile", SurfaceFeature::Plural, "exiles"),
            ("Graveyard", SurfaceFeature::Singular, "graveyard"),
            ("Graveyard", SurfaceFeature::Plural, "graveyards"),
            ("Hand", SurfaceFeature::Singular, "hand"),
            ("Hand", SurfaceFeature::Plural, "hands"),
            ("Library", SurfaceFeature::Singular, "library"),
            ("Library", SurfaceFeature::Plural, "libraries"),
            ("Name", SurfaceFeature::Singular, "name"),
            ("Name", SurfaceFeature::Plural, "names"),
            ("Number", SurfaceFeature::Singular, "number"),
            ("Number", SurfaceFeature::Plural, "numbers"),
            ("Controller", SurfaceFeature::Singular, "controller"),
            ("Controller", SurfaceFeature::Plural, "controllers"),
            ("Opponent", SurfaceFeature::Singular, "opponent"),
            ("Opponent", SurfaceFeature::Plural, "opponents"),
            ("Owner", SurfaceFeature::Singular, "owner"),
            ("Owner", SurfaceFeature::Plural, "owners"),
            ("Permanent", SurfaceFeature::Singular, "permanent"),
            ("Permanent", SurfaceFeature::Plural, "permanents"),
            ("Phase", SurfaceFeature::Singular, "phase"),
            ("Phase", SurfaceFeature::Plural, "phases"),
            ("Player", SurfaceFeature::Singular, "player"),
            ("Player", SurfaceFeature::Plural, "players"),
            ("Power", SurfaceFeature::Singular, "power"),
            ("Power", SurfaceFeature::Plural, "powers"),
            ("Rest", SurfaceFeature::Singular, "rest"),
            ("Rest", SurfaceFeature::Plural, "rests"),
            ("Source", SurfaceFeature::Singular, "source"),
            ("Source", SurfaceFeature::Plural, "sources"),
            ("Size", SurfaceFeature::Singular, "size"),
            ("Size", SurfaceFeature::Plural, "sizes"),
            ("Spell", SurfaceFeature::Singular, "spell"),
            ("Spell", SurfaceFeature::Plural, "spells"),
            ("Stack", SurfaceFeature::Singular, "stack"),
            ("Stack", SurfaceFeature::Plural, "stacks"),
            ("Step", SurfaceFeature::Singular, "step"),
            ("Step", SurfaceFeature::Plural, "steps"),
            ("Tax", SurfaceFeature::Singular, "tax"),
            ("Tax", SurfaceFeature::Plural, "taxes"),
            ("Token", SurfaceFeature::Singular, "token"),
            ("Token", SurfaceFeature::Plural, "tokens"),
            ("Toughness", SurfaceFeature::Singular, "toughness"),
            ("Toughness", SurfaceFeature::Plural, "toughnesses"),
            ("Turn", SurfaceFeature::Singular, "turn"),
            ("Turn", SurfaceFeature::Plural, "turns"),
            ("Type", SurfaceFeature::Singular, "type"),
            ("Type", SurfaceFeature::Plural, "types"),
            ("Die", SurfaceFeature::Singular, "die"),
            ("Die", SurfaceFeature::Plural, "dice"),
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
        "lexeme:CommonNoun/Token/singular",
        "lexeme:CommonNoun/Token/plural",
        "lexeme:CommonNoun/Hand/singular",
        "lexeme:CommonNoun/Hand/plural",
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

#[test]
fn task11_suffix_edge_overrides_generate_only_correct_english_surfaces() {
    let source =
        fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/constructions.rs"))
            .expect("production constructions are readable");
    let invocation = deckmaste_construction_core::invocation_from_source(&source)
        .expect("production construction invocation is authentic");
    let expansion = deckmaste_construction_core::generate(invocation.tokens)
        .expect("production construction inventory compiles");
    let surfaces = |terminal_name: &str| {
        expansion
            .terminal_contributions()
            .iter()
            .find(|terminal| terminal.name() == terminal_name)
            .unwrap_or_else(|| panic!("{terminal_name} terminal contribution exists"))
            .surfaces()
            .iter()
            .map(|row| (row.member(), row.feature(), row.surface()))
            .collect::<Vec<_>>()
    };
    let verbs = surfaces("VerbLexeme");
    let nouns = surfaces("CommonNoun");

    for expected in [
        ("May", SurfaceFeature::Bare, "may"),
        ("May", SurfaceFeature::ThirdPersonSingular, "may"),
        ("Do", SurfaceFeature::Bare, "do"),
        ("Do", SurfaceFeature::ThirdPersonSingular, "does"),
    ] {
        assert!(
            verbs.contains(&expected),
            "missing verb surface {expected:?}"
        );
    }
    for expected in [
        ("Copy", SurfaceFeature::Plural, "copies"),
        ("Library", SurfaceFeature::Plural, "libraries"),
        ("Tax", SurfaceFeature::Plural, "taxes"),
        ("Toughness", SurfaceFeature::Plural, "toughnesses"),
    ] {
        assert!(
            nouns.contains(&expected),
            "missing noun surface {expected:?}"
        );
    }
    for malformed in ["mays", "dos"] {
        assert!(
            verbs.iter().all(|(_, _, surface)| *surface != malformed),
            "generated verb inventory retained malformed {malformed:?}",
        );
    }
    for malformed in ["copys", "librarys", "taxs", "toughnesss"] {
        assert!(
            nouns.iter().all(|(_, _, surface)| *surface != malformed),
            "generated noun inventory retained malformed {malformed:?}",
        );
    }

    let parser = parser();
    let context = context();
    for text in ["It does.", "Toughnesses gain 1 life."] {
        let parsed = parser.parse(text, &context).unwrap_or_else(|error| {
            panic!("correct irregular surface must parse {text:?}: {error}")
        });
        assert_eq!(parsed.render(&context, parser.environment()), text);
    }
    for text in ["It dos.", "Toughnesss gain 1 life."] {
        assert!(
            parser.parse(text, &context).is_err(),
            "malformed generated surface must reject {text:?}",
        );
    }
}
