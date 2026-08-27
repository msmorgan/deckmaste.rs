use std::path::Path;

use deckmaste_english_v2::ast::CatalogProvider;
use deckmaste_english_v2::context::ParseContext;
use deckmaste_english_v2::environment::CatalogProviderRow;
use deckmaste_english_v2::environment::CatalogProviderRows;
use deckmaste_english_v2::environment::ParserEnvironment;
use deckmaste_english_v2::parser::Parser;
use macro_ron::v2::Onset;

fn parser() -> Parser {
    let declarations = macro_ron::v2::read_builtin_v2(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin_v2"),
    )
    .expect("builtin declarations load");
    let cards = CatalogProviderRows::new(
        CatalogProvider::CardNames,
        [CatalogProviderRow::new(
            "context-card",
            "Context Card",
            Onset::Consonant,
        )],
    );
    let environment = ParserEnvironment::try_from_parts(declarations, [cards])
        .expect("parser environment freezes");
    Parser::new(environment).expect("required declarations are present")
}

fn context() -> ParseContext<'static> {
    ParseContext::new("Context Card", false, Onset::Consonant).expect("context is valid")
}

fn generated_item(name: &str) -> proc_macro2::TokenStream {
    let invocation = deckmaste_construction_core::invocation_from_source(include_str!(
        "../src/constructions.rs"
    ))
    .expect("the grammar source has one constructions invocation");
    let expansion = deckmaste_construction_core::generate(invocation.tokens)
        .expect("the zeroable determiner declaration validates and emits");
    expansion
        .items()
        .iter()
        .find(|item| {
            matches!(
                &item.key,
                deckmaste_construction_core::ItemKey::Named {
                    name: item_name,
                    ..
                } if item_name == name
            )
        })
        .unwrap_or_else(|| panic!("generated item {name:?} is present"))
        .tokens
        .clone()
}

fn has_exact_path(tokens: &str, prefix: &str, name: &str) -> bool {
    tokens.split(prefix).skip(1).any(|tail| {
        let Some(rest) = tail.trim_start().strip_prefix(name) else {
            return false;
        };
        !rest
            .chars()
            .next()
            .is_some_and(|character| character.is_alphanumeric() || character == '_')
    })
}

#[test]
fn zeroable_determiner_emits_required_ast_and_direct_owner_branches() {
    let determiner = generated_item("Determiner");
    let syn::Item::Enum(determiner) = syn::parse2(determiner).expect("Determiner item parses")
    else {
        panic!("Determiner is an enum")
    };
    assert_eq!(determiner.variants.len(), 2);
    let zero = determiner
        .variants
        .iter()
        .find(|variant| variant.ident == "Zero")
        .expect("zeroable field emits the Zero value");
    assert!(matches!(zero.fields, syn::Fields::Unit));
    let headed = determiner
        .variants
        .iter()
        .find(|variant| variant.ident == "Headed")
        .expect("zeroable field emits the Headed value");
    let syn::Fields::Unnamed(headed_fields) = &headed.fields else {
        panic!("Headed carries the consumed Determinative")
    };
    assert_eq!(headed_fields.unnamed.len(), 1);
    let syn::Type::Path(headed_type) = &headed_fields.unnamed[0].ty else {
        panic!("Headed stores a named Determinative type")
    };
    assert!(headed_type.qself.is_none());
    assert_eq!(
        headed_type.path.segments.last().unwrap().ident,
        "Determinative"
    );

    let determined = generated_item("DeterminedNominal");
    let syn::Item::Struct(determined) =
        syn::parse2(determined).expect("DeterminedNominal item parses")
    else {
        panic!("DeterminedNominal is a struct")
    };
    let syn::Fields::Named(fields) = determined.fields else {
        panic!("DeterminedNominal has named fields")
    };
    let det = fields
        .named
        .iter()
        .find(|field| field.ident.as_ref().is_some_and(|name| name == "det"))
        .expect("DeterminedNominal retains its determiner field");
    let syn::Type::Path(det_type) = &det.ty else {
        panic!("the AST stores a concrete Determiner, not a wrapper")
    };
    assert!(det_type.qself.is_none());
    assert_eq!(det_type.path.segments.len(), 1);
    assert_eq!(det_type.path.segments[0].ident, "Determiner");

    let categories = generated_item("Category").to_string();
    assert!(
        !has_exact_path(&categories, "enum Category {", "Determiner"),
        "zeroability must not introduce a public Determiner grammar category: {categories}"
    );

    let rules = generated_item("RULES").to_string();
    let absent = rules
        .split("Rule {")
        .find(|row| row.contains("RuleId :: UnqualifiedReferenceDeterminedNominalDetAbsent"))
        .expect("the zeroable absent owner branch is emitted");
    assert!(
        absent.contains("rhs : & [N (Category :: Nominal)]"),
        "the absent owner branch claims only its nominal bytes: {absent}"
    );
    assert!(
        !absent.contains("Category :: Determinative"),
        "the absent owner branch must not claim determinative bytes: {absent}"
    );
    let present = rules
        .split("Rule {")
        .find(|row| row.contains("RuleId :: UnqualifiedReferenceDeterminedNominalDetPresent"))
        .expect("the zeroable present owner branch is emitted");
    assert!(
        present.contains("N (Category :: Determinative)")
            && present.contains("N (Category :: Nominal)"),
        "the present owner branch directly consumes determinative then nominal: {present}"
    );
    assert!(
        !rules.contains("DeterminedNominalDetOptional"),
        "zeroability must not emit a nullable optional helper: {rules}"
    );
    assert!(
        !has_exact_path(&rules, "Category ::", "Determiner")
            && !has_exact_path(&rules, "Lexical ::", "Determiner"),
        "zeroability must not add a nullable Determiner category or synthetic terminal"
    );
}

#[test]
fn checked_zeroable_determiner_reaches_the_emitted_build_guard() {
    let build = generated_item("build_checked").to_string();
    for branch in [
        "UnqualifiedReferenceDeterminedNominalDetAbsent",
        "UnqualifiedReferenceDeterminedNominalDetPresent",
    ] {
        let guard = build
            .split("RuleId ::")
            .find(|arm| arm.starts_with(&format!(" {branch}")))
            .unwrap_or_else(|| panic!("the {branch} build arm is emitted"));
        let guard_offset = guard
            .find("determiner_licenses_nominal")
            .expect("the checked-by callback is emitted in the build arm");
        assert!(
            guard[..guard_offset].contains("if"),
            "the checked-by callback is a build guard rather than an unobserved helper: {guard}"
        );
        assert!(
            guard[guard_offset..].contains("nominal_onset"),
            "the checked-by callback receives its validated nominal.onset companion: {guard}"
        );
        assert!(
            guard.contains("Determiner :: Headed") && guard.contains("Determiner :: Zero"),
            "the build arm converts direct branches into the required AST value: {guard}"
        );
    }
}

#[test]
fn zeroable_determiner_branches_have_no_synthetic_ownership_claims() {
    let parser = parser();
    let context = context();
    for text in [
        "Destroy creatures.",
        "Destroy target creature.",
        "Destroy an artifact.",
    ] {
        let analysis = parser.analyze(text, &context);
        assert!(
            analysis.selected().is_some(),
            "{text:?} selects a zeroable determiner branch: {analysis:?}"
        );
        let ownership = analysis
            .ownership()
            .expect("a selected reference document owns its bytes");
        assert!(ownership.failures().is_empty(), "{text:?}: {ownership:?}");
        let summary = ownership.summary();
        assert!(summary.covered(), "{text:?}: {ownership:?}");
        assert_eq!(summary.synthetic_claims(), 0, "{text:?}: {ownership:?}");
    }
}
