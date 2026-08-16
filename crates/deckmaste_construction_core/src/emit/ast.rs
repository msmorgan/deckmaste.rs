use std::collections::HashMap;

use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;

use crate::ValidatedDeclarations;
use crate::model::Declaration;
use crate::model::FieldKind;
use crate::model::NonPublicVisibility;
use crate::plan::DeclarationKey;
use crate::plan::DeclarationKind;
use crate::plan::GeneratedItem;
use crate::plan::ItemKey;

pub(crate) fn emit(validated: &ValidatedDeclarations) -> syn::Result<Vec<GeneratedItem>> {
    let constructions = validated
        .raw()
        .declarations
        .iter()
        .filter_map(|declaration| match declaration {
            Declaration::Construction(construction) => Some(construction),
            _ => None,
        })
        .collect::<Vec<_>>();
    let records = validated.contributions().constructions();
    if constructions.len() != records.len() {
        return Err(internal_error(
            "validated construction inventory is inconsistent",
        ));
    }

    let mut categories = Vec::<CategoryItem>::new();
    let mut category_indices = HashMap::<String, usize>::new();
    for (construction, record) in constructions.iter().zip(records) {
        if construction.name != record.construction_id()
            || construction.element.name != record.element_type()
        {
            return Err(internal_error(
                "validated construction identity is inconsistent",
            ));
        }
        let category = record.category().to_owned();
        let index = *category_indices.entry(category.clone()).or_insert_with(|| {
            let index = categories.len();
            categories.push(CategoryItem {
                name: category,
                span: construction
                    .category
                    .segments
                    .last()
                    .map_or_else(Span::call_site, |segment| segment.ident.span()),
                variants: Vec::new(),
                origins: Vec::new(),
            });
            index
        });
        categories[index].variants.push((
            record.category_variant().to_owned(),
            record.element_type().to_owned(),
            construction.name.span(),
        ));
        categories[index].origins.push(DeclarationKey::new(
            DeclarationKind::Construction,
            record.construction_id(),
        ));
    }

    let mut items = categories
        .into_iter()
        .map(CategoryItem::finish)
        .collect::<Vec<_>>();
    for (construction, record) in constructions.into_iter().zip(records) {
        let ident = syn::Ident::new(record.element_type(), construction.element.name.span());
        let origins = vec![DeclarationKey::new(
            DeclarationKind::Construction,
            record.construction_id(),
        )];
        let tokens = emit_product(validated, construction, &ident)?;
        items.push(GeneratedItem::new(
            ItemKey::named_type(record.element_type()),
            tokens,
            origins,
        ));
    }
    Ok(items)
}

struct CategoryItem {
    name: String,
    span: Span,
    variants: Vec<(String, String, Span)>,
    origins: Vec<DeclarationKey>,
}

impl CategoryItem {
    fn finish(self) -> GeneratedItem {
        let ident = syn::Ident::new(&self.name, self.span);
        let variants = self.variants.into_iter().map(|(variant, element, span)| {
            let variant = syn::Ident::new(&variant, span);
            let element = syn::Ident::new(&element, span);
            quote! { #variant(#element) }
        });
        let tokens = quote! {
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub enum #ident {
                #(#variants),*
            }
        };
        GeneratedItem::new(ItemKey::named_type(self.name), tokens, self.origins)
    }
}

fn emit_product(
    validated: &ValidatedDeclarations,
    construction: &crate::Construction,
    ident: &syn::Ident,
) -> syn::Result<TokenStream> {
    if construction.element.fields.is_empty() {
        return Ok(quote! {
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct #ident;
        });
    }

    let construction_name = construction.name.to_string();
    let fields = construction
        .element
        .fields
        .iter()
        .map(|field| {
            let name = &field.name;
            let visibility = field_visibility(construction, field)?;
            let ty = match &field.kind {
                FieldKind::Category(path) | FieldKind::Lex(path) | FieldKind::Identity(path) => {
                    path
                }
            };
            let boxed = validated
                .boxed_fields()
                .contains(&(construction_name.clone(), field.name.to_string()));
            Ok(if boxed {
                quote! { #visibility #name: Box<#ty> }
            } else {
                quote! { #visibility #name: #ty }
            })
        })
        .collect::<syn::Result<Vec<_>>>()?;
    Ok(quote! {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct #ident {
            #(#fields),*
        }
    })
}

fn field_visibility(
    construction: &crate::Construction,
    field: &crate::Field,
) -> syn::Result<TokenStream> {
    let Some(checked) = &construction.checked else {
        return Ok(quote! { pub });
    };
    let visibility = checked
        .visibilities
        .iter()
        .find(|visibility| visibility.role == field.name)
        .ok_or_else(|| internal_error("validated checked visibility is incomplete"))?;
    Ok(match &visibility.visibility {
        NonPublicVisibility::Private(_) => TokenStream::new(),
        NonPublicVisibility::Restricted(visibility) => quote! { #visibility },
    })
}

fn internal_error(message: &str) -> syn::Error {
    syn::Error::new(Span::call_site(), message)
}

#[cfg(test)]
mod tests {
    use quote::ToTokens;
    use syn::Fields;
    use syn::Item;
    use syn::Type;
    use syn::Visibility;
    use syn::parse::Parser;

    use crate::DeclarationKind;
    use crate::ItemKey;
    use crate::test_support::representative_expansion;

    #[test]
    fn emits_exact_category_variants_products_visibility_and_boxing() {
        let expansion = representative_expansion();
        let items = expansion.items();

        let node = parse_named(items, "Node");
        let Item::Enum(node) = node else {
            panic!("Node is an enum");
        };
        assert_eq!(
            derive_names(&node.attrs),
            ["Debug", "Clone", "PartialEq", "Eq"]
        );
        assert_eq!(
            node.variants
                .iter()
                .map(|variant| {
                    let Fields::Unnamed(fields) = &variant.fields else {
                        panic!("category variants have one unnamed payload");
                    };
                    assert_eq!(fields.unnamed.len(), 1);
                    (variant.ident.to_string(), type_name(&fields.unnamed[0].ty))
                })
                .collect::<Vec<_>>(),
            [
                ("Leaf".into(), "Leaf".into()),
                ("Chain".into(), "Chain".into())
            ]
        );
        assert_eq!(
            named(items, "Node")
                .origins
                .iter()
                .map(|origin| (origin.kind(), origin.name()))
                .collect::<Vec<_>>(),
            [
                (DeclarationKind::Construction, "leaf"),
                (DeclarationKind::Construction, "chain"),
            ]
        );

        let chain = parse_named(items, "Chain");
        let Item::Struct(chain) = chain else {
            panic!("Chain is a struct");
        };
        assert_eq!(
            derive_names(&chain.attrs),
            ["Debug", "Clone", "PartialEq", "Eq"]
        );
        let Fields::Named(fields) = chain.fields else {
            panic!("Chain has named fields");
        };
        let fields = fields.named.into_iter().collect::<Vec<_>>();
        assert_eq!(fields.len(), 2);
        assert!(matches!(fields[0].vis, Visibility::Inherited));
        assert_eq!(boxed_inner_name(&fields[0].ty), Some("Node".to_owned()));
        let Visibility::Restricted(restricted) = &fields[1].vis else {
            panic!("checked word field is pub(crate)");
        };
        assert!(restricted.path.is_ident("crate"));
        assert_eq!(type_name(&fields[1].ty), "Words");

        let leaf = parse_named(items, "Leaf");
        let Item::Struct(leaf) = leaf else {
            panic!("Leaf is a struct");
        };
        let Fields::Named(fields) = leaf.fields else {
            panic!("Leaf has named fields");
        };
        assert!(matches!(fields.named[0].vis, Visibility::Public(_)));
        assert_eq!(type_name(&fields.named[0].ty), "Words");

        for item in items {
            let file = syn::parse2::<syn::File>(item.tokens.clone()).expect("item reparses");
            assert_eq!(file.items.len(), 1, "{:?}", item.key);
        }
    }

    #[test]
    fn public_full_golden_expansion_matches_exact_category_and_product_tables() {
        let expansion = crate::generate(crate::validate::tests::full_golden_tokens())
            .expect("the full golden fixture expands through the public API");

        const CATEGORIES: &[(&str, &[(&str, &str)])] = &[
            ("Ability", &[("Spell", "Spell"), ("Triggered", "Triggered")]),
            (
                "Sentence",
                &[
                    ("Imperative", "Imperative"),
                    ("Declarative", "Declarative"),
                    ("WithWhere", "WithWhere"),
                ],
            ),
            (
                "Clause",
                &[("Event", "EventClause"), ("Where", "WhereClause")],
            ),
            (
                "NounPhrase",
                &[
                    ("Pronoun", "PronounNp"),
                    ("Common", "Common"),
                    ("Demonstrative", "DemonstrativeNp"),
                    ("Target", "TargetNp"),
                    ("SelfReference", "SelfReferenceNp"),
                    ("Count", "CountNp"),
                ],
            ),
            (
                "VerbPhrase",
                &[
                    ("Destroy", "Destroy"),
                    ("Connive", "Connive"),
                    ("DealDamage", "DealDamage"),
                    ("GainLife", "GainLife"),
                ],
            ),
            (
                "Amount",
                &[("Number", "NumberAmount"), ("Variable", "VariableAmount")],
            ),
        ];

        for &(name, expected_variants) in CATEGORIES {
            let Item::Enum(item) = parse_named(expansion.items(), name) else {
                panic!("{name} is an enum");
            };
            assert_eq!(item.ident, name, "category item name");
            assert_public(&item.vis, name);
            assert_common_derives(&item.attrs, name);
            assert!(item.generics.params.is_empty(), "{name} has no generics");
            assert!(
                item.generics.where_clause.is_none(),
                "{name} has no where clause"
            );
            let actual = item
                .variants
                .iter()
                .map(|variant| {
                    assert!(
                        variant.attrs.is_empty(),
                        "{} has no attributes",
                        variant.ident
                    );
                    assert!(
                        variant.discriminant.is_none(),
                        "{} has no discriminant",
                        variant.ident
                    );
                    let Fields::Unnamed(fields) = &variant.fields else {
                        panic!("{} has one tuple payload", variant.ident);
                    };
                    assert_eq!(fields.unnamed.len(), 1, "{} payload count", variant.ident);
                    assert!(fields.unnamed[0].attrs.is_empty());
                    (
                        variant.ident.to_string(),
                        exact_simple_type(&fields.unnamed[0].ty),
                    )
                })
                .collect::<Vec<_>>();
            assert_eq!(
                actual,
                expected_variants
                    .iter()
                    .map(|&(variant, payload)| (variant.to_owned(), payload.to_owned()))
                    .collect::<Vec<_>>(),
                "{name} variant table"
            );
        }

        const PUB: FieldVisibility = FieldVisibility::Public;
        const PRIVATE: FieldVisibility = FieldVisibility::Private;
        const CRATE: FieldVisibility = FieldVisibility::Crate;
        const PRODUCTS: &[(&str, Option<&[(&str, &str, bool, FieldVisibility)]>)] = &[
            ("Spell", Some(&[("effect", "Sentence", false, PUB)])),
            (
                "Triggered",
                Some(&[
                    ("trigger", "TriggerWord", false, CRATE),
                    ("event", "Clause", false, CRATE),
                    ("effect", "Sentence", false, CRATE),
                ]),
            ),
            (
                "Imperative",
                Some(&[("predicate", "VerbPhrase", false, PUB)]),
            ),
            (
                "Declarative",
                Some(&[
                    ("subject", "NounPhrase", false, PUB),
                    ("predicate", "VerbPhrase", false, PUB),
                ]),
            ),
            (
                "WithWhere",
                Some(&[
                    ("body", "Sentence", true, PUB),
                    ("clause", "Clause", false, PUB),
                ]),
            ),
            (
                "EventClause",
                Some(&[
                    ("subject", "NounPhrase", false, PUB),
                    ("predicate", "VerbPhrase", false, PUB),
                ]),
            ),
            (
                "WhereClause",
                Some(&[
                    ("variable", "Variable", false, PUB),
                    ("value", "NounPhrase", false, PUB),
                ]),
            ),
            ("PronounNp", Some(&[("word", "Pronoun", false, PUB)])),
            (
                "Common",
                Some(&[
                    ("article", "Article", false, PUB),
                    ("head", "Noun", false, PUB),
                ]),
            ),
            (
                "DemonstrativeNp",
                Some(&[
                    ("word", "Demonstrative", false, PUB),
                    ("head", "Noun", false, PUB),
                ]),
            ),
            ("TargetNp", Some(&[("head", "Noun", false, PUB)])),
            (
                "SelfReferenceNp",
                Some(&[("spelling", "SelfReferenceSpelling", false, PRIVATE)]),
            ),
            (
                "CountNp",
                Some(&[
                    ("head", "Noun", false, PUB),
                    ("controller", "Pronoun", false, PUB),
                    ("threshold", "SignedNumber", false, PUB),
                ]),
            ),
            ("Destroy", Some(&[("object", "NounPhrase", false, PUB)])),
            ("Connive", None),
            (
                "DealDamage",
                Some(&[
                    ("amount", "Amount", false, PUB),
                    ("to", "NounPhrase", false, PUB),
                ]),
            ),
            ("GainLife", Some(&[("amount", "Amount", false, PUB)])),
            (
                "NumberAmount",
                Some(&[("number", "SignedNumber", false, PUB)]),
            ),
            (
                "VariableAmount",
                Some(&[("variable", "Variable", false, PUB)]),
            ),
        ];

        for &(name, expected_fields) in PRODUCTS {
            let Item::Struct(item) = parse_named(expansion.items(), name) else {
                panic!("{name} is a struct");
            };
            assert_eq!(item.ident, name, "product item name");
            assert_public(&item.vis, name);
            assert_common_derives(&item.attrs, name);
            assert!(item.generics.params.is_empty(), "{name} has no generics");
            assert!(
                item.generics.where_clause.is_none(),
                "{name} has no where clause"
            );
            match (expected_fields, &item.fields) {
                (None, Fields::Unit) => {}
                (None, _) => panic!("{name} is exactly a unit struct"),
                (Some(expected), Fields::Named(fields)) => {
                    let actual = fields
                        .named
                        .iter()
                        .map(|field| {
                            assert!(
                                field.attrs.is_empty(),
                                "{name}.{:?} has no attributes",
                                field.ident
                            );
                            let (field_type, boxed) = exact_field_type(&field.ty);
                            (
                                field.ident.as_ref().expect("named field").to_string(),
                                field_type,
                                boxed,
                                field_visibility(&field.vis),
                            )
                        })
                        .collect::<Vec<_>>();
                    assert_eq!(
                        actual,
                        expected
                            .iter()
                            .map(|&(field, ty, boxed, visibility)| {
                                (field.to_owned(), ty.to_owned(), boxed, visibility)
                            })
                            .collect::<Vec<_>>(),
                        "{name} field table"
                    );
                }
                (Some(_), _) => panic!("{name} has ordered named fields"),
            }
        }
    }

    fn named<'a>(items: &'a [crate::GeneratedItem], name: &str) -> &'a crate::GeneratedItem {
        items
            .iter()
            .find(|item| matches!(&item.key, ItemKey::Named { name: found, .. } if found == name))
            .unwrap_or_else(|| panic!("generated item `{name}` exists"))
    }

    fn parse_named(items: &[crate::GeneratedItem], name: &str) -> Item {
        let file = syn::parse2::<syn::File>(named(items, name).tokens.clone())
            .expect("generated item parses as a file");
        assert_eq!(file.items.len(), 1);
        file.items.into_iter().next().expect("one item")
    }

    fn derive_names(attrs: &[syn::Attribute]) -> Vec<String> {
        assert_eq!(attrs.len(), 1, "derive is the only item attribute");
        let derive = attrs
            .iter()
            .find(|attribute| attribute.path().is_ident("derive"))
            .expect("derive attribute");
        let syn::Meta::List(list) = &derive.meta else {
            panic!("derive is a list");
        };
        syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated
            .parse2(list.tokens.clone())
            .expect("derive paths parse")
            .iter()
            .map(|path| {
                assert_eq!(path.segments.len(), 1, "derive path is unqualified");
                path.segments[0].ident.to_string()
            })
            .collect()
    }

    fn assert_common_derives(attrs: &[syn::Attribute], name: &str) {
        assert_eq!(
            derive_names(attrs),
            ["Debug", "Clone", "PartialEq", "Eq"],
            "{name} derives"
        );
    }

    fn assert_public(visibility: &Visibility, name: &str) {
        assert!(
            matches!(visibility, Visibility::Public(_)),
            "{name} is public"
        );
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum FieldVisibility {
        Public,
        Private,
        Crate,
    }

    fn field_visibility(visibility: &Visibility) -> FieldVisibility {
        match visibility {
            Visibility::Public(_) => FieldVisibility::Public,
            Visibility::Inherited => FieldVisibility::Private,
            Visibility::Restricted(restricted)
                if restricted.in_token.is_none()
                    && restricted.path.leading_colon.is_none()
                    && restricted.path.is_ident("crate") =>
            {
                FieldVisibility::Crate
            }
            other => panic!("unexpected field visibility: {other:?}"),
        }
    }

    fn exact_field_type(ty: &Type) -> (String, bool) {
        let Type::Path(path) = ty else {
            panic!("field type is a path: {}", ty.to_token_stream());
        };
        assert!(path.qself.is_none(), "field type has no qualified self");
        if path.path.segments.len() == 1 && path.path.segments[0].ident == "Box" {
            let syn::PathArguments::AngleBracketed(arguments) = &path.path.segments[0].arguments
            else {
                panic!("Box has one type argument");
            };
            assert_eq!(arguments.args.len(), 1, "Box has one type argument");
            let syn::GenericArgument::Type(inner) = &arguments.args[0] else {
                panic!("Box argument is a type");
            };
            return (exact_simple_type(inner), true);
        }
        (exact_simple_type(ty), false)
    }

    fn exact_simple_type(ty: &Type) -> String {
        let Type::Path(path) = ty else {
            panic!("type is a simple path: {}", ty.to_token_stream());
        };
        assert!(path.qself.is_none(), "type has no qualified self");
        assert!(path.path.leading_colon.is_none(), "type is not absolute");
        assert_eq!(path.path.segments.len(), 1, "type path is unqualified");
        let segment = &path.path.segments[0];
        assert!(matches!(segment.arguments, syn::PathArguments::None));
        segment.ident.to_string()
    }

    fn type_name(ty: &Type) -> String {
        let Type::Path(path) = ty else {
            panic!("field type is a path: {}", ty.to_token_stream());
        };
        path.path
            .segments
            .last()
            .expect("type path has a segment")
            .ident
            .to_string()
    }

    fn boxed_inner_name(ty: &Type) -> Option<String> {
        let Type::Path(path) = ty else { return None };
        let segment = path.path.segments.last()?;
        if segment.ident != "Box" {
            return None;
        }
        let syn::PathArguments::AngleBracketed(arguments) = &segment.arguments else {
            return None;
        };
        let syn::GenericArgument::Type(inner) = arguments.args.first()? else {
            return None;
        };
        Some(type_name(inner))
    }
}
