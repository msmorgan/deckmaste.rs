use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use anyhow::Context as _;
use anyhow::bail;
use deckmaste_construction_core::Declaration;
use deckmaste_construction_core::Declarations;
use deckmaste_construction_core::Field;
use serde::Deserialize;
use serde::Serialize;
use syn::visit::Visit as _;

const GRANDFATHERED_FORBIDDEN: [&str; 2] = ["noun_is_way", "singular_demonstrative_is_this"];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum LicensingCheckerKind {
    DeclaredLicenseFeature,
    StructuralPredicate,
    ForbiddenLexicalIdentity,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub(super) struct LicensingChecker {
    identity: String,
    kind: LicensingCheckerKind,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
pub(super) struct LicensingCheckerCensus {
    rows: Vec<LicensingChecker>,
    permitted_total: usize,
    forbidden_total: usize,
}

impl LicensingCheckerCensus {
    pub(super) const fn permitted_total(&self) -> usize {
        self.permitted_total
    }

    pub(super) const fn forbidden_total(&self) -> usize {
        self.forbidden_total
    }

    pub(super) fn rows(&self) -> &[LicensingChecker] {
        &self.rows
    }

    pub(super) fn enforce_forbidden_policy(&self) -> anyhow::Result<()> {
        let forbidden = self
            .rows
            .iter()
            .filter(|row| row.kind == LicensingCheckerKind::ForbiddenLexicalIdentity)
            .map(|row| row.identity.as_str())
            .collect::<Vec<_>>();
        if forbidden.is_empty() || forbidden == GRANDFATHERED_FORBIDDEN {
            return Ok(());
        }
        bail!(
            "English-v2 licensing checker gate rejected forbidden lexical-identity checkers: {forbidden:?}; only the temporary grandfathered pair {GRANDFATHERED_FORBIDDEN:?} is permitted until its retirement ticket lands",
        )
    }
}

impl LicensingChecker {
    pub(super) fn identity(&self) -> &str {
        &self.identity
    }

    pub(super) const fn kind(&self) -> LicensingCheckerKind {
        self.kind
    }
}

pub(super) fn from_path(path: &Path) -> anyhow::Result<LicensingCheckerCensus> {
    let source = fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    from_source(&source)
}

pub(super) fn from_source(source: &str) -> anyhow::Result<LicensingCheckerCensus> {
    let invocation = deckmaste_construction_core::invocation_from_source(source)
        .map_err(anyhow::Error::new)
        .context("extracting English-v2 construction declaration")?;
    let declarations = deckmaste_construction_core::parse_declarations(invocation.tokens)
        .map_err(anyhow::Error::new)
        .context("parsing English-v2 construction declaration")?;
    let lexical_members = lexical_members(&declarations);
    let callbacks = checker_callbacks(&declarations);
    let syntax = syn::parse_file(source).context("parsing English-v2 Rust source")?;
    let mut bodies = function_bodies(&syntax)?;

    let expansion = deckmaste_construction_core::generate(
        deckmaste_construction_core::invocation_from_source(source)
            .map_err(anyhow::Error::new)?
            .tokens,
    )
    .map_err(anyhow::Error::new)
    .context("generating English-v2 construction declaration")?;
    for item in expansion.items() {
        let Ok(syn::Item::Fn(function)) = syn::parse2::<syn::Item>(item.tokens.clone()) else {
            continue;
        };
        insert_function_body(&mut bodies, &function)?;
    }

    let mut rows = Vec::with_capacity(callbacks.len());
    for (identity, function_name) in callbacks {
        let body = bodies.get(&function_name).with_context(|| {
            format!("locating body for English-v2 licensing checker `{identity}`")
        })?;
        rows.push(LicensingChecker {
            identity,
            kind: classify_body(body, &lexical_members),
        });
    }
    rows.sort_by(|left, right| left.identity.cmp(&right.identity));
    let forbidden_total = rows
        .iter()
        .filter(|row| row.kind == LicensingCheckerKind::ForbiddenLexicalIdentity)
        .count();
    Ok(LicensingCheckerCensus {
        permitted_total: rows.len() - forbidden_total,
        forbidden_total,
        rows,
    })
}

fn checker_callbacks(declarations: &Declarations) -> BTreeMap<String, String> {
    let mut callbacks = BTreeMap::new();
    for declaration in &declarations.declarations {
        let fields = match declaration {
            Declaration::Construction(construction) => construction.element.fields.as_slice(),
            Declaration::AbstractProduct(product) => product.fields.as_slice(),
            _ => continue,
        };
        collect_callbacks(fields, &mut callbacks);
    }
    callbacks
}

fn collect_callbacks(fields: &[Field], callbacks: &mut BTreeMap<String, String>) {
    for field in fields {
        let Some(check) = &field.check else { continue };
        let identity = compact_tokens(&check.function);
        let function_name = check
            .function
            .segments
            .last()
            .expect("checked callback paths are nonempty")
            .ident
            .to_string();
        callbacks.insert(identity, function_name);
    }
}

fn lexical_members(declarations: &Declarations) -> BTreeSet<(String, String)> {
    let mut members = BTreeSet::new();
    for declaration in &declarations.declarations {
        match declaration {
            Declaration::Vocab(vocab) => {
                for variant in &vocab.variants {
                    members.insert((vocab.name.to_string(), variant.name.to_string()));
                }
            }
            Declaration::Lexeme(lexeme) => {
                for member in &lexeme.members {
                    members.insert((lexeme.name.to_string(), member.name.to_string()));
                }
            }
            _ => {}
        }
    }
    members
}

fn function_bodies(file: &syn::File) -> anyhow::Result<BTreeMap<String, syn::Block>> {
    let mut bodies = BTreeMap::new();
    for item in &file.items {
        if let syn::Item::Fn(function) = item {
            insert_function_body(&mut bodies, function)?;
        }
    }
    Ok(bodies)
}

fn insert_function_body(
    bodies: &mut BTreeMap<String, syn::Block>,
    function: &syn::ItemFn,
) -> anyhow::Result<()> {
    let name = function.sig.ident.to_string();
    if bodies
        .insert(name.clone(), (*function.block).clone())
        .is_some()
    {
        bail!("duplicate English-v2 checker function body `{name}`");
    }
    Ok(())
}

fn classify_body(
    body: &syn::Block,
    lexical_members: &BTreeSet<(String, String)>,
) -> LicensingCheckerKind {
    let mut classifier = BodyClassifier {
        lexical_members,
        reads_declared_feature: false,
        compares_lexical_identity: false,
    };
    classifier.visit_block(body);
    if classifier.compares_lexical_identity {
        LicensingCheckerKind::ForbiddenLexicalIdentity
    } else if classifier.reads_declared_feature {
        LicensingCheckerKind::DeclaredLicenseFeature
    } else {
        LicensingCheckerKind::StructuralPredicate
    }
}

struct BodyClassifier<'a> {
    lexical_members: &'a BTreeSet<(String, String)>,
    reads_declared_feature: bool,
    compares_lexical_identity: bool,
}

impl<'ast> syn::visit::Visit<'ast> for BodyClassifier<'_> {
    fn visit_path(&mut self, path: &'ast syn::Path) {
        for pair in path.segments.iter().collect::<Vec<_>>().windows(2) {
            let key = (pair[0].ident.to_string(), pair[1].ident.to_string());
            if self.lexical_members.contains(&key) {
                self.compares_lexical_identity = true;
            }
        }
        for segment in &path.segments {
            self.reads_declared_feature |=
                is_declared_feature_identifier(&segment.ident.to_string());
        }
        syn::visit::visit_path(self, path);
    }

    fn visit_expr_method_call(&mut self, call: &'ast syn::ExprMethodCall) {
        self.reads_declared_feature |= is_declared_feature_identifier(&call.method.to_string());
        syn::visit::visit_expr_method_call(self, call);
    }

    fn visit_member(&mut self, member: &'ast syn::Member) {
        if let syn::Member::Named(name) = member {
            self.reads_declared_feature |= is_declared_feature_identifier(&name.to_string());
        }
        syn::visit::visit_member(self, member);
    }

    fn visit_macro(&mut self, expression: &'ast syn::Macro) {
        self.inspect_macro_tokens(expression.tokens.clone());
        syn::visit::visit_macro(self, expression);
    }
}

impl BodyClassifier<'_> {
    fn inspect_macro_tokens(&mut self, tokens: proc_macro2::TokenStream) {
        let trees = tokens.into_iter().collect::<Vec<_>>();
        for tree in &trees {
            match tree {
                proc_macro2::TokenTree::Ident(identifier) => {
                    self.reads_declared_feature |=
                        is_declared_feature_identifier(&identifier.to_string());
                }
                proc_macro2::TokenTree::Group(group) => {
                    self.inspect_macro_tokens(group.stream());
                }
                proc_macro2::TokenTree::Punct(_) | proc_macro2::TokenTree::Literal(_) => {}
            }
        }
        for window in trees.windows(4) {
            let [
                proc_macro2::TokenTree::Ident(owner),
                proc_macro2::TokenTree::Punct(first_colon),
                proc_macro2::TokenTree::Punct(second_colon),
                proc_macro2::TokenTree::Ident(member),
            ] = window
            else {
                continue;
            };
            if first_colon.as_char() == ':'
                && second_colon.as_char() == ':'
                && self
                    .lexical_members
                    .contains(&(owner.to_string(), member.to_string()))
            {
                self.compares_lexical_identity = true;
            }
        }
    }
}

fn is_declared_feature_identifier(identifier: &str) -> bool {
    const FEATURE_NAMES: [&str; 21] = [
        "agreement",
        "bare_locative_complement",
        "bare_locative_license",
        "cardinality",
        "compoundability",
        "countability",
        "determiner_number",
        "fused_head_license",
        "homograph_license",
        "locative_temporal_license",
        "modifier_license",
        "nominal_form",
        "nominal_license",
        "number",
        "onset",
        "participle",
        "possessive_ending",
        "preposition_attachment",
        "preposition_complement_kind",
        "properness",
        "relationality",
    ];
    let snake = to_snake_case(identifier);
    FEATURE_NAMES.iter().any(|feature| {
        snake == *feature
            || snake.starts_with(&format!("{feature}_"))
            || snake.ends_with(&format!("_{feature}"))
    }) || snake == "license"
        || snake.ends_with("_license")
        || snake.ends_with("_licensed")
        || snake.ends_with("_licenses")
}

fn to_snake_case(identifier: &str) -> String {
    let mut snake = String::with_capacity(identifier.len());
    for (index, character) in identifier.chars().enumerate() {
        if character.is_ascii_uppercase() {
            if index != 0 {
                snake.push('_');
            }
            snake.push(character.to_ascii_lowercase());
        } else {
            snake.push(character);
        }
    }
    snake
}

fn compact_tokens(tokens: &impl quote::ToTokens) -> String {
    tokens.to_token_stream().to_string().replace(' ', "")
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRODUCTION_SOURCE: &str =
        include_str!("../../../deckmaste_english_v2/src/constructions.rs");

    #[test]
    fn production_census_names_only_the_grandfathered_forbidden_pair() {
        let census = from_source(PRODUCTION_SOURCE).expect("production census builds");
        let forbidden = census
            .rows()
            .iter()
            .filter(|row| row.kind() == LicensingCheckerKind::ForbiddenLexicalIdentity)
            .map(LicensingChecker::identity)
            .collect::<Vec<_>>();

        assert_eq!(forbidden, GRANDFATHERED_FORBIDDEN);
        assert_eq!(
            census.permitted_total() + census.forbidden_total(),
            census.rows().len()
        );
        census
            .enforce_forbidden_policy()
            .expect("the grandfathered pair remains accepted until retirement");
    }

    #[test]
    fn classifier_reads_bodies_instead_of_checker_names() {
        let lexical_members = BTreeSet::from([("Word".to_owned(), "Named".to_owned())]);
        let forbidden_name_with_feature_body: syn::Block =
            syn::parse_quote!({ license == License::Allowed });
        let structural_name_with_lexical_body: syn::Block =
            syn::parse_quote!({ matches!(value, Word::Named) });

        assert_eq!(
            classify_body(&forbidden_name_with_feature_body, &lexical_members),
            LicensingCheckerKind::DeclaredLicenseFeature,
        );
        assert_eq!(
            classify_body(&structural_name_with_lexical_body, &lexical_members),
            LicensingCheckerKind::ForbiddenLexicalIdentity,
        );
    }

    #[test]
    fn forbidden_policy_rejects_any_added_identity() {
        let census = LicensingCheckerCensus {
            rows: vec![LicensingChecker {
                identity: "new_checker".to_owned(),
                kind: LicensingCheckerKind::ForbiddenLexicalIdentity,
            }],
            permitted_total: 0,
            forbidden_total: 1,
        };

        let error = census
            .enforce_forbidden_policy()
            .expect_err("new forbidden checker must fail the gate");
        assert!(error.to_string().contains("new_checker"));
    }
}
