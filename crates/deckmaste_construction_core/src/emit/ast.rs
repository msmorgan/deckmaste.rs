use std::collections::HashMap;

use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;

use crate::identifier::emitted_ident;
use crate::plan::DeclarationKey;
use crate::plan::DeclarationKind;
use crate::plan::GeneratedItem;
use crate::plan::ItemKey;
use crate::semantic::ConstructionPlan;
use crate::semantic::FieldVisibilityPlan;
use crate::semantic::SemanticPlan;

pub(crate) fn emit(plan: &SemanticPlan) -> syn::Result<Vec<GeneratedItem>> {
    let mut categories = Vec::<CategoryItem>::new();
    let mut category_indices = HashMap::<String, usize>::new();
    for construction in plan.constructions() {
        let category = construction.category().to_owned();
        let index = *category_indices.entry(category.clone()).or_insert_with(|| {
            let index = categories.len();
            categories.push(CategoryItem {
                name: category,
                span: construction.category_span(),
                variants: Vec::new(),
                origins: Vec::new(),
            });
            index
        });
        categories[index].variants.push((
            construction.category_variant().to_owned(),
            construction.element_type().to_owned(),
            construction.origin_span(),
        ));
        categories[index].origins.push(DeclarationKey::new(
            DeclarationKind::Construction,
            construction.construction_id(),
        ));
    }

    let mut items = categories
        .into_iter()
        .map(CategoryItem::finish)
        .collect::<Vec<_>>();
    for construction in plan.constructions() {
        let ident = emitted_ident(construction.element_type(), construction.origin_span());
        let origins = vec![DeclarationKey::new(
            DeclarationKind::Construction,
            construction.construction_id(),
        )];
        let tokens = emit_product(plan, construction, &ident)?;
        items.push(GeneratedItem::new(
            ItemKey::named_type(construction.element_type()),
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
        let ident = emitted_ident(&self.name, self.span);
        let variants = self.variants.into_iter().map(|(variant, element, span)| {
            let variant = emitted_ident(&variant, span);
            let element = emitted_ident(&element, span);
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
    plan: &SemanticPlan,
    construction: &ConstructionPlan,
    ident: &syn::Ident,
) -> syn::Result<TokenStream> {
    if construction.fields().is_empty() {
        return Ok(quote! {
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct #ident;
        });
    }

    let construction_name = construction.construction_id();
    let fields = construction
        .fields()
        .iter()
        .map(|field| {
            let name = field.name();
            let visibility = field_visibility(field.visibility());
            let ty = field.value_type();
            let boxed = plan
                .boxed_fields()
                .contains(&(construction_name.to_owned(), field.name_key()));
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

fn field_visibility(visibility: &FieldVisibilityPlan) -> TokenStream {
    match visibility {
        FieldVisibilityPlan::Public => quote! { pub },
        FieldVisibilityPlan::Private => TokenStream::new(),
        FieldVisibilityPlan::Restricted(visibility) => quote! { #visibility },
    }
}

#[cfg(test)]
mod tests {
    #![allow(
        clippy::items_after_statements,
        clippy::match_wildcard_for_single_variants,
        clippy::too_many_lines,
        clippy::type_complexity,
        reason = "literal full-surface structural oracles are intentionally table-dense"
    )]
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
                ("Leaf".into(), "WordLeaf".into()),
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

        let leaf = parse_named(items, "WordLeaf");
        let Item::Struct(leaf) = leaf else {
            panic!("WordLeaf is a struct");
        };
        let Fields::Named(fields) = leaf.fields else {
            panic!("WordLeaf has named fields");
        };
        assert!(matches!(fields.named[0].vis, Visibility::Public(_)));
        assert_eq!(type_name(&fields.named[0].ty), "Words");

        for item in items {
            let file = syn::parse2::<syn::File>(item.tokens.clone()).expect("item reparses");
            assert_eq!(file.items.len(), 1, "{:?}", item.key);
        }
    }

    #[test]
    fn synthetic_projection_has_exact_category_products_visibility_and_boxing() {
        let expansion = crate::test_support::synthetic_projection_expansion();

        const CATEGORIES: &[(&str, &[(&str, &str)])] = &[
            ("Expr", &[("Leaf", "LeafNode"), ("Nested", "NestedNode")]),
            (
                "Predicate",
                &[("Action", "ActionNode"), ("Idle", "IdleNode")],
            ),
            ("Tag", &[("Solo", "SoloTag")]),
            ("Document", &[("Document", "DocumentNode")]),
        ];
        for &(name, variants) in CATEGORIES {
            let Item::Enum(item) = parse_named(expansion.items(), name) else {
                panic!("{name} is a category enum");
            };
            assert_public(&item.vis, name);
            assert_common_derives(&item.attrs, name);
            assert_eq!(
                item.variants
                    .iter()
                    .map(|variant| {
                        let Fields::Unnamed(fields) = &variant.fields else {
                            panic!("{} has one tuple payload", variant.ident);
                        };
                        (
                            variant.ident.to_string(),
                            exact_simple_type(&fields.unnamed[0].ty),
                        )
                    })
                    .collect::<Vec<_>>(),
                variants
                    .iter()
                    .map(|&(variant, ty)| (variant.to_owned(), ty.to_owned()))
                    .collect::<Vec<_>>(),
            );
        }

        const PUB: FieldVisibility = FieldVisibility::Public;
        const PRIVATE: FieldVisibility = FieldVisibility::Private;
        const CRATE: FieldVisibility = FieldVisibility::Crate;
        const PRODUCTS: &[(&str, Option<&[(&str, &str, bool, FieldVisibility)]>)] = &[
            (
                "LeafNode",
                Some(&[
                    ("mode", "Mode", false, PUB),
                    ("resource", "Resource", false, PUB),
                ]),
            ),
            (
                "NestedNode",
                Some(&[
                    ("next", "Expr", true, PRIVATE),
                    ("marker", "Marker", false, CRATE),
                ]),
            ),
            ("ActionNode", None),
            ("IdleNode", None),
            ("SoloTag", Some(&[("mode", "Mode", false, PUB)])),
            (
                "DocumentNode",
                Some(&[
                    ("subject", "Expr", false, CRATE),
                    ("predicate", "Predicate", false, CRATE),
                    ("handle", "Handle", false, PRIVATE),
                    ("pair", "Pair", false, PRIVATE),
                ]),
            ),
        ];
        for &(name, expected_fields) in PRODUCTS {
            let Item::Struct(item) = parse_named(expansion.items(), name) else {
                panic!("{name} is a product struct");
            };
            assert_public(&item.vis, name);
            assert_common_derives(&item.attrs, name);
            match expected_fields {
                None => assert!(matches!(item.fields, Fields::Unit)),
                Some(expected) => {
                    let Fields::Named(fields) = item.fields else {
                        panic!("{name} has named fields");
                    };
                    assert_eq!(
                        fields
                            .named
                            .iter()
                            .map(|field| {
                                let (ty, boxed) = exact_field_type(&field.ty);
                                (
                                    field.ident.as_ref().unwrap().to_string(),
                                    ty,
                                    boxed,
                                    field_visibility(&field.vis),
                                )
                            })
                            .collect::<Vec<_>>(),
                        expected
                            .iter()
                            .map(|&(field, ty, boxed, visibility)| {
                                (field.to_owned(), ty.to_owned(), boxed, visibility)
                            })
                            .collect::<Vec<_>>(),
                    );
                }
            }
        }

        assert_eq!(
            expansion
                .items()
                .iter()
                .filter_map(|item| {
                    if let ItemKey::Named {
                        kind: crate::NamedKind::Type,
                        name,
                    } = &item.key
                    {
                        Some(name.as_str())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>(),
            [
                "Expr",
                "Predicate",
                "Tag",
                "Document",
                "LeafNode",
                "NestedNode",
                "ActionNode",
                "IdleNode",
                "SoloTag",
                "DocumentNode",
                "Mode",
                "ObjectStem",
                "ActionStem",
                "Agreement",
                "Number",
                "FeatureConstraint",
                "CasePosition",
                "ScanPosition",
                "DeclarationClass",
                "DeclarationMatcher",
                "DeclarationLeaf",
                "Lexical",
                "Leaf",
                "TerminalClass",
                "LexicalTerminal",
                "LexicalProvenanceKind",
                "LexicalOwnerTemplate",
                "LexicalOwnerIdentity",
                "LexicalOwner",
                "Category",
                "Construction",
                "RuleId",
            ],
        );
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
