use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use anyhow::Context as _;
use anyhow::bail;
use deckmaste_construction_core::Declaration;
use deckmaste_construction_core::Declarations;
use deckmaste_construction_core::Expansion;
use deckmaste_construction_core::Field;
use deckmaste_construction_core::GeneratedCodecRecipe;
use serde::Deserialize;
use serde::Serialize;
use syn::visit::Visit as _;

/// No word-naming `checked by` guard is grandfathered.
const GRANDFATHERED_FORBIDDEN: [&str; 0] = [];

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
        let grandfathered = BTreeSet::from(GRANDFATHERED_FORBIDDEN);
        let unlicensed = self
            .rows
            .iter()
            .filter(|row| row.kind == LicensingCheckerKind::ForbiddenLexicalIdentity)
            .map(|row| row.identity.as_str())
            .filter(|identity| !grandfathered.contains(identity))
            .collect::<Vec<_>>();
        if unlicensed.is_empty() {
            return Ok(());
        }
        bail!(
            "English-v2 licensing checker gate rejected forbidden lexical-identity checkers: {unlicensed:?}; a checker must read a declared feature or a structural predicate, never a lexical identity. Grandfathered exceptions: {GRANDFATHERED_FORBIDDEN:?}",
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
    let expansion = deckmaste_construction_core::generate(invocation.tokens.clone())
        .map_err(anyhow::Error::new)
        .context("generating English-v2 construction declaration")?;
    from_parts(source, invocation.tokens, &expansion)
}

/// The census for a caller that already generated the expansion, so `report`
/// does not expand the declaration a second time.
pub(super) fn from_expansion(
    source: &str,
    expansion: &Expansion,
) -> anyhow::Result<LicensingCheckerCensus> {
    let invocation = deckmaste_construction_core::invocation_from_source(source)
        .map_err(anyhow::Error::new)
        .context("extracting English-v2 construction declaration")?;
    from_parts(source, invocation.tokens, expansion)
}

fn from_parts(
    source: &str,
    tokens: proc_macro2::TokenStream,
    expansion: &Expansion,
) -> anyhow::Result<LicensingCheckerCensus> {
    let declarations = deckmaste_construction_core::parse_declarations(tokens)
        .map_err(anyhow::Error::new)
        .context("parsing English-v2 construction declaration")?;
    let lexical_members = lexical_members(&declarations);
    let callbacks = checker_callbacks(&declarations);
    let syntax = syn::parse_file(source).context("parsing English-v2 Rust source")?;
    let mut bodies = function_bodies(&syntax)?;

    for item in expansion.items() {
        let Ok(syn::Item::Fn(function)) = syn::parse2::<syn::Item>(item.tokens.clone()) else {
            continue;
        };
        insert_function_body(&mut bodies, &function)?;
    }

    let mut rows = Vec::with_capacity(callbacks.len());
    for (identity, function_name) in callbacks {
        if !bodies.contains_key(&function_name) {
            bail!("locating body for English-v2 licensing checker `{identity}`");
        }
        rows.push(LicensingChecker {
            identity,
            kind: classify_checker(&function_name, &bodies, &lexical_members),
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
            Declaration::Codec(binding) => {
                let Some(GeneratedCodecRecipe::DeclarationDeterminative(source)) =
                    &binding.generated
                else {
                    continue;
                };
                let owner = format!("{}Lemma", binding.name);
                for slot in &source.closed_slots {
                    for member in &slot.members {
                        members.insert((owner.clone(), member.lemma.to_string()));
                    }
                }
            }
            _ => {}
        }
    }
    members
}

/// A locally defined function body, with the fact that decides whether the
/// classifier follows a call into it: only `-> bool` helpers continue a
/// checker's own predicate logic. A helper returning a feature value is the
/// declared-feature layer itself, so the walk stops there.
struct CheckerBody {
    block: syn::Block,
    returns_bool: bool,
}

fn returns_bool(signature: &syn::Signature) -> bool {
    let syn::ReturnType::Type(_, kind) = &signature.output else {
        return false;
    };
    let syn::Type::Path(path) = kind.as_ref() else {
        return false;
    };
    path.qself.is_none() && path.path.is_ident("bool")
}

fn function_bodies(file: &syn::File) -> anyhow::Result<BTreeMap<String, CheckerBody>> {
    let mut bodies = BTreeMap::new();
    for item in &file.items {
        if let syn::Item::Fn(function) = item {
            insert_function_body(&mut bodies, function)?;
        }
    }
    Ok(bodies)
}

fn insert_function_body(
    bodies: &mut BTreeMap<String, CheckerBody>,
    function: &syn::ItemFn,
) -> anyhow::Result<()> {
    let name = function.sig.ident.to_string();
    let body = CheckerBody {
        block: (*function.block).clone(),
        returns_bool: returns_bool(&function.sig),
    };
    if bodies.insert(name.clone(), body).is_some() {
        bail!("duplicate English-v2 checker function body `{name}`");
    }
    Ok(())
}

/// Classifies a checker over the transitive closure of the bodies it calls, so
/// a comparison moved one call away from the checked field is still counted.
fn classify_checker(
    entry: &str,
    bodies: &BTreeMap<String, CheckerBody>,
    lexical_members: &BTreeSet<(String, String)>,
) -> LicensingCheckerKind {
    let mut visited = BTreeSet::new();
    let mut pending = vec![entry.to_owned()];
    let mut reads_declared_feature = false;
    let mut compares_lexical_identity = false;
    while let Some(name) = pending.pop() {
        if !visited.insert(name.clone()) {
            continue;
        }
        let Some(body) = bodies.get(&name) else {
            continue;
        };
        let mut classifier = BodyClassifier::new(lexical_members);
        classifier.visit_block(&body.block);
        reads_declared_feature |= classifier.reads_declared_feature;
        compares_lexical_identity |= classifier.compares_lexical_identity;
        pending.extend(
            classifier
                .callees
                .into_iter()
                .filter(|callee| bodies.get(callee).is_some_and(|body| body.returns_bool)),
        );
    }
    kind_for(compares_lexical_identity, reads_declared_feature)
}

#[cfg(test)]
fn classify_body(
    body: &syn::Block,
    lexical_members: &BTreeSet<(String, String)>,
) -> LicensingCheckerKind {
    let mut classifier = BodyClassifier::new(lexical_members);
    classifier.visit_block(body);
    kind_for(
        classifier.compares_lexical_identity,
        classifier.reads_declared_feature,
    )
}

const fn kind_for(
    compares_lexical_identity: bool,
    reads_declared_feature: bool,
) -> LicensingCheckerKind {
    if compares_lexical_identity {
        LicensingCheckerKind::ForbiddenLexicalIdentity
    } else if reads_declared_feature {
        LicensingCheckerKind::DeclaredLicenseFeature
    } else {
        LicensingCheckerKind::StructuralPredicate
    }
}

struct BodyClassifier<'a> {
    lexical_members: &'a BTreeSet<(String, String)>,
    reads_declared_feature: bool,
    compares_lexical_identity: bool,
    callees: BTreeSet<String>,
}

impl<'a> BodyClassifier<'a> {
    const fn new(lexical_members: &'a BTreeSet<(String, String)>) -> Self {
        Self {
            lexical_members,
            reads_declared_feature: false,
            compares_lexical_identity: false,
            callees: BTreeSet::new(),
        }
    }
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

    fn visit_expr_call(&mut self, call: &'ast syn::ExprCall) {
        if let syn::Expr::Path(path) = call.func.as_ref()
            && let Some(segment) = path.path.segments.last()
        {
            self.callees.insert(segment.ident.to_string());
        }
        syn::visit::visit_expr_call(self, call);
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
        for window in trees.windows(2) {
            let [
                proc_macro2::TokenTree::Ident(callee),
                proc_macro2::TokenTree::Group(arguments),
            ] = window
            else {
                continue;
            };
            if arguments.delimiter() == proc_macro2::Delimiter::Parenthesis {
                self.callees.insert(callee.to_string());
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
    let snake = to_snake_case(identifier);
    deckmaste_construction_core::feature_keys().any(|feature| {
        snake == feature
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
    fn production_census_has_no_word_naming_checker() {
        let census = from_source(PRODUCTION_SOURCE).expect("production census builds");
        let forbidden = census
            .rows()
            .iter()
            .filter(|row| row.kind() == LicensingCheckerKind::ForbiddenLexicalIdentity)
            .map(LicensingChecker::identity)
            .collect::<Vec<_>>();

        assert!(forbidden.is_empty());
        assert_eq!(
            census.permitted_total() + census.forbidden_total(),
            census.rows().len()
        );
        census
            .enforce_forbidden_policy()
            .expect("production has no word-naming checker");
    }

    #[test]
    fn classifier_follows_a_predicate_helper_into_its_lexical_comparison() {
        let lexical_members = BTreeSet::from([("Word".to_owned(), "Named".to_owned())]);
        let bodies = BTreeMap::from([
            (
                "entry".to_owned(),
                CheckerBody {
                    block: syn::parse_quote!({ number == Number::Plural && helper(value) }),
                    returns_bool: true,
                },
            ),
            (
                "helper".to_owned(),
                CheckerBody {
                    block: syn::parse_quote!({ matches!(value, Word::Named) }),
                    returns_bool: true,
                },
            ),
        ]);

        assert_eq!(
            classify_checker("entry", &bodies, &lexical_members),
            LicensingCheckerKind::ForbiddenLexicalIdentity,
        );
    }

    // A helper that returns a declared feature value rather than `bool` is the
    // feature layer, not the checker's own logic, so the walk stops at it.
    #[test]
    fn classifier_stops_at_a_feature_returning_helper() {
        let lexical_members = BTreeSet::from([("Word".to_owned(), "Named".to_owned())]);
        let bodies = BTreeMap::from([
            (
                "entry".to_owned(),
                CheckerBody {
                    block: syn::parse_quote!({ properness(value) == Properness::Proper }),
                    returns_bool: true,
                },
            ),
            (
                "properness".to_owned(),
                CheckerBody {
                    block: syn::parse_quote!({
                        match value {
                            Word::Named => Properness::Proper,
                            _ => Properness::Common,
                        }
                    }),
                    returns_bool: false,
                },
            ),
        ]);

        assert_eq!(
            classify_checker("entry", &bodies, &lexical_members),
            LicensingCheckerKind::DeclaredLicenseFeature,
        );
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
    fn classifier_recognizes_every_published_feature_key() {
        for feature in deckmaste_construction_core::feature_keys() {
            assert!(is_declared_feature_identifier(feature));
        }
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
