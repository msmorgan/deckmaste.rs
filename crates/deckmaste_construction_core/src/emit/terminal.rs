use quote::quote;

use crate::ValidatedDeclarations;
use crate::model::Declaration;
use crate::plan::DeclarationKey;
use crate::plan::DeclarationKind;
use crate::plan::GeneratedItem;
use crate::plan::ItemKey;
use crate::plan::TerminalContribution;
use crate::plan::TerminalKind;
use crate::plan::TerminalVariantContribution;

pub(crate) fn emit(
    validated: &ValidatedDeclarations,
) -> syn::Result<(Vec<GeneratedItem>, Vec<TerminalContribution>)> {
    let mut items = Vec::new();
    let mut contributions = Vec::new();

    for declaration in &validated.raw().declarations {
        match declaration {
            Declaration::Vocab(vocab) => {
                let name = vocab.name.to_string();
                let origin = DeclarationKey::new(DeclarationKind::Vocab, &name);
                let variants = vocab
                    .variants
                    .iter()
                    .map(|variant| {
                        TerminalVariantContribution::new(
                            variant.name.to_string(),
                            Some(variant.word.value()),
                        )
                    })
                    .collect::<Vec<_>>();
                let variant_idents = vocab.variants.iter().map(|variant| &variant.name);
                let ident = &vocab.name;
                let tokens = quote! {
                    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
                    pub enum #ident {
                        #(#variant_idents),*
                    }
                };
                items.push(GeneratedItem::new(
                    ItemKey::named_type(&name),
                    tokens,
                    vec![origin.clone()],
                ));
                contributions.push(TerminalContribution::new(
                    origin,
                    TerminalKind::Vocab,
                    &name,
                    Some(format!("render_{}", snake_case(&name))),
                    variants,
                    false,
                ));
            }
            Declaration::Lexeme(lexeme) => {
                let name = lexeme.name.to_string();
                let terminal = validated
                    .contributions()
                    .terminals()
                    .iter()
                    .find(|terminal| terminal.name() == name)
                    .ok_or_else(|| {
                        syn::Error::new(
                            lexeme.name.span(),
                            "validated terminal inventory is inconsistent",
                        )
                    })?;
                let verb_provider = terminal.supports_verb_atom();
                let origin = DeclarationKey::new(DeclarationKind::Lexeme, &name);
                let variants = lexeme
                    .variants
                    .iter()
                    .map(|variant| TerminalVariantContribution::new(variant.to_string(), None))
                    .collect::<Vec<_>>();
                let variant_idents = &lexeme.variants;
                let ident = &lexeme.name;
                let tokens = if verb_provider {
                    quote! {
                        #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                        pub enum #ident {
                            #(#variant_idents),*
                        }
                    }
                } else {
                    quote! {
                        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
                        pub enum #ident {
                            #(#variant_idents),*
                        }
                    }
                };
                items.push(GeneratedItem::new(
                    ItemKey::named_type(&name),
                    tokens,
                    vec![origin.clone()],
                ));
                contributions.push(TerminalContribution::new(
                    origin,
                    TerminalKind::Lexeme,
                    &name,
                    None,
                    variants,
                    verb_provider,
                ));
            }
            Declaration::Construction(_)
            | Declaration::Codec(_)
            | Declaration::Identity(_)
            | Declaration::Root(_) => {}
        }
    }
    Ok((items, contributions))
}

fn snake_case(name: &str) -> String {
    let characters = name.chars().collect::<Vec<_>>();
    let mut result = String::new();
    for (index, character) in characters.iter().copied().enumerate() {
        if character.is_uppercase() {
            let previous_is_lower = index > 0 && characters[index - 1].is_lowercase();
            let acronym_boundary = index > 0
                && characters[index - 1].is_uppercase()
                && characters
                    .get(index + 1)
                    .is_some_and(|next| next.is_lowercase());
            if (previous_is_lower || acronym_boundary) && !result.ends_with('_') {
                result.push('_');
            }
            result.extend(character.to_lowercase());
        } else {
            result.push(character);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    #![allow(
        clippy::items_after_statements,
        clippy::too_many_lines,
        clippy::type_complexity,
        reason = "literal full-surface structural oracles are intentionally table-dense"
    )]
    use syn::Item;
    use syn::parse::Parser;

    use crate::ItemKey;
    use crate::TerminalKind;
    use crate::test_support::representative_expansion;

    #[test]
    fn emits_vocab_and_lexeme_enums_with_resolved_derives_and_word_data() {
        let expansion = representative_expansion();

        assert_enum(&expansion, "Words", &["First", "Second"], false);
        assert_enum(&expansion, "Nouns", &["Person"], false);
        assert_enum(&expansion, "Verbs", &["Act"], true);

        let contributions = expansion.terminal_contributions();
        assert_eq!(contributions.len(), 3);
        assert_eq!(contributions[0].kind(), TerminalKind::Vocab);
        assert_eq!(contributions[0].name(), "Words");
        assert_eq!(contributions[0].render_function(), Some("render_words"));
        assert_eq!(
            contributions[0]
                .variants()
                .iter()
                .map(|variant| (variant.name(), variant.word()))
                .collect::<Vec<_>>(),
            [("First", Some("first")), ("Second", Some("second"))]
        );
        assert_eq!(contributions[1].kind(), TerminalKind::Lexeme);
        assert_eq!(contributions[1].render_function(), None);
        assert!(!contributions[1].is_verb_provider());
        assert!(contributions[2].is_verb_provider());
    }

    #[test]
    fn synthetic_projection_has_exact_terminal_tables_and_capabilities() {
        let expansion = crate::test_support::synthetic_projection_expansion();

        assert_enum(&expansion, "Mode", &["Solo", "Group"], false);
        assert_enum(&expansion, "ObjectStem", &["Widget"], false);
        assert_enum(&expansion, "ActionStem", &["Activate"], true);

        let contributions = expansion.terminal_contributions();
        assert_eq!(contributions.len(), 3);
        assert_eq!(
            contributions
                .iter()
                .map(|terminal| (
                    terminal.name(),
                    terminal.kind(),
                    terminal.render_function(),
                    terminal.is_verb_provider(),
                ))
                .collect::<Vec<_>>(),
            [
                ("Mode", TerminalKind::Vocab, Some("render_mode"), false),
                ("ObjectStem", TerminalKind::Lexeme, None, false),
                ("ActionStem", TerminalKind::Lexeme, None, true),
            ]
        );
        assert_eq!(
            contributions[0]
                .variants()
                .iter()
                .map(|variant| (variant.name(), variant.word()))
                .collect::<Vec<_>>(),
            [("Solo", Some("solo")), ("Group", Some("group"))],
        );

        let emitted = expansion
            .items()
            .iter()
            .filter_map(|item| match &item.key {
                ItemKey::Named {
                    kind: crate::NamedKind::Type,
                    name,
                } if ["Mode", "ObjectStem", "ActionStem"].contains(&name.as_str()) => {
                    Some(name.as_str())
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(emitted, ["Mode", "ObjectStem", "ActionStem"]);
    }
    fn assert_enum(expansion: &crate::Expansion, name: &str, variants: &[&str], ordered: bool) {
        let item = expansion
            .items()
            .iter()
            .find(|item| matches!(&item.key, ItemKey::Named { name: found, .. } if found == name))
            .unwrap_or_else(|| panic!("terminal enum `{name}` exists"));
        let file = syn::parse2::<syn::File>(item.tokens.clone()).expect("terminal item reparses");
        let Item::Enum(item) = &file.items[0] else {
            panic!("terminal output is an enum");
        };
        assert_eq!(
            item.variants
                .iter()
                .map(|variant| variant.ident.to_string())
                .collect::<Vec<_>>(),
            variants
        );
        let derive = item
            .attrs
            .iter()
            .find(|attribute| attribute.path().is_ident("derive"))
            .expect("derive attribute");
        let syn::Meta::List(list) = &derive.meta else {
            panic!("derive is a list");
        };
        let paths = syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated
            .parse2(list.tokens.clone())
            .expect("derive paths parse");
        let mut expected = vec!["Debug", "Clone", "Copy", "PartialEq", "Eq"];
        if ordered {
            expected.extend(["Ord", "PartialOrd"]);
        }
        assert_eq!(
            paths
                .iter()
                .map(|path| path
                    .segments
                    .last()
                    .expect("derive path has a segment")
                    .ident
                    .to_string())
                .collect::<Vec<_>>(),
            expected
        );
    }
}
