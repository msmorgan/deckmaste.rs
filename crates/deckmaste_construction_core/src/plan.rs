use std::collections::HashSet;

use proc_macro2::TokenStream;

use crate::ValidatedDeclarations;

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

pub(crate) fn plan_emission(validated: &ValidatedDeclarations) -> syn::Result<EmissionPlan> {
    let mut items = crate::emit::ast::emit(validated)?;
    let (terminal_items, terminal_contributions) = crate::emit::terminal::emit(validated)?;
    items.extend(terminal_items);
    items.extend(crate::emit::render::emit(validated)?);
    items.extend(crate::emit::visit::emit(validated)?);

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
    use crate::ItemKey;
    use crate::NamedKind;
    use crate::test_support::representative_expansion;

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
        assert_eq!(keys.len(), 20);
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
