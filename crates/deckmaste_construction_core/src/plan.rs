use std::collections::HashSet;

use proc_macro2::TokenStream;

#[cfg(test)]
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

    #[cfg(test)]
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
    fn semantic_plan_construction_emitters_share_one_mutated_fact() {
        let mut plan = crate::test_support::representative_semantic_plan();
        plan.test_only_replace_planned_literal("first", 0, "changed");
        assert_eq!(authored_first_literal(&plan), "first");
        let ast = crate::emit::ast::emit(&plan).unwrap();
        let rules = crate::emit::rules::emit(&plan).unwrap();
        let build = crate::emit::build::emit(&plan).unwrap();
        assert!(formatted(&rules).contains("Literal (\"changed\")"));
        assert!(formatted(&build).contains("Leaf :: Literal (\"changed\")"));
        assert_eq!(named_types(&ast), expected_category_and_product_types());
    }

    #[test]
    fn semantic_plan_terminal_emitters_share_one_mutated_fact() {
        let input = crate::test_support::representative_tokens().to_string();
        let mut plan = crate::test_support::representative_semantic_plan();
        plan.test_only_replace_vocab_spelling("Words", "First", "changed");
        let terminal = crate::emit::terminal::emit(&plan).unwrap();
        assert_eq!(terminal_word(&terminal, "First"), "changed");
        assert!(formatted(&crate::emit::render::emit(&plan).unwrap()).contains("\"changed\""));
        assert!(
            visitor_origins(&crate::emit::visit::emit(&plan).unwrap())
                .contains(&"Words".to_owned())
        );
        assert!(
            crate::report::escape_hatch_report(&plan)
                .unwrap()
                .terminal_bindings()
                .is_empty()
        );
        assert_eq!(
            crate::test_support::representative_tokens().to_string(),
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
    fn sealed_plan_source_mismatches_are_emission_errors() {
        let mut plan = crate::test_support::representative_semantic_plan();
        plan.test_only_mispoint_construction_source("first");
        let error = crate::emit::build::emit(&plan).expect_err("bad sealed plan is fallible");
        assert!(error.to_string().contains("construction source"));
    }

    #[test]
    fn sealed_construction_atoms_reject_count_and_kind_mismatches() {
        let plan = crate::test_support::representative_semantic_plan();
        let crate::Declaration::Construction(construction) = &plan.source().declarations[0] else {
            panic!("representative source begins with its construction");
        };

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

    fn expected_category_and_product_types() -> Vec<String> {
        ["Node", "First"].into_iter().map(str::to_owned).collect()
    }

    fn authored_first_literal(plan: &SemanticPlan) -> String {
        let crate::Declaration::Construction(construction) = &plan.source().declarations[0] else {
            panic!("representative source begins with its construction");
        };
        let crate::FormAtom::Literal(literal) = &construction.form.atoms[0] else {
            panic!("representative construction begins with its literal");
        };
        literal.value()
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
                    name: "Leaf".into()
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
        assert_eq!(keys.len(), 26);
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
