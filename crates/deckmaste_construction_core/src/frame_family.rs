use crate::semantic::{ConstructionPlan, TerminalPlan};
use std::collections::HashSet;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Ident, Path, Token, braced, bracketed, parenthesized};

#[derive(Debug)]
pub struct FrameFamilySource {
    pub name: Ident,
    pub categories: Vec<Ident>,
    pub roles: Vec<(Ident, Ident, Ident)>,
    pub coordinators: Vec<Path>,
}

impl Parse for FrameFamilySource {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let _: Ident = input.parse()?;
        let name = input.parse()?;
        let body;
        braced!(body in input);
        let field: Ident = body.parse()?;
        if field != "categories" {
            return Err(syn::Error::new_spanned(field, "expected categories"));
        }
        body.parse::<Token![:]>()?;
        let categories;
        bracketed!(categories in body);
        let categories = Punctuated::<Ident, Token![,]>::parse_terminated(&categories)?
            .into_iter()
            .collect();
        body.parse::<Token![,]>()?;
        let field: Ident = body.parse()?;
        if field != "roles" {
            return Err(syn::Error::new_spanned(field, "expected roles"));
        }
        body.parse::<Token![:]>()?;
        let role_body;
        bracketed!(role_body in body);
        let mut roles = Vec::new();
        while !role_body.is_empty() {
            let role = role_body.parse()?;
            role_body.parse::<Token![=]>()?;
            let relation = role_body.parse()?;
            let argument;
            parenthesized!(argument in role_body);
            let category = argument.parse()?;
            if !argument.is_empty() {
                return Err(argument.error("expected one category"));
            }
            roles.push((role, relation, category));
            if role_body.is_empty() {
                break;
            }
            role_body.parse::<Token![,]>()?;
        }
        body.parse::<Token![,]>()?;
        let field: Ident = body.parse()?;
        if field != "coordinators" {
            return Err(syn::Error::new_spanned(field, "expected coordinators"));
        }
        body.parse::<Token![:]>()?;
        let coordinator_body;
        bracketed!(coordinator_body in body);
        let coordinators = Punctuated::<Path, Token![,]>::parse_terminated(&coordinator_body)?
            .into_iter()
            .collect();
        if body.peek(Token![,]) {
            body.parse::<Token![,]>()?;
        }
        if !body.is_empty() {
            return Err(body.error("unexpected frame family field"));
        }
        Ok(Self {
            name,
            categories,
            roles,
            coordinators,
        })
    }
}

#[derive(Debug)]
pub(crate) struct FrameFamilyPlan {
    pub(crate) name: Ident,
    pub(crate) categories: Vec<Ident>,
    pub(crate) roles: Vec<(Ident, Ident, Ident)>,
    pub(crate) coordinators: Vec<(Ident, Ident)>,
}

pub(crate) fn seal(
    source: &crate::model::Declarations,
    constructions: &[ConstructionPlan],
    terminals: &[TerminalPlan],
) -> syn::Result<Vec<FrameFamilyPlan>> {
    let mut names = HashSet::new();
    source
        .frame_families
        .iter()
        .map(|family| {
            if !names.insert(crate::identifier::snake_case(&family.name.to_string())) {
                return Err(syn::Error::new_spanned(
                    &family.name,
                    "duplicate frame family",
                ));
            }
            let mut categories = HashSet::new();
            for category in &family.categories {
                if !categories.insert(category.to_string()) {
                    return Err(syn::Error::new_spanned(
                        category,
                        "duplicate frame category",
                    ));
                }
                if !constructions.iter().any(|c| category == c.category()) {
                    return Err(syn::Error::new_spanned(category, "unknown frame category"));
                }
            }
            let mut roles = HashSet::new();
            for (role, relation, category) in &family.roles {
                if !roles.insert(role.to_string()) {
                    return Err(syn::Error::new_spanned(role, "duplicate frame role"));
                }
                if !matches!(
                    relation.to_string().as_str(),
                    "Subject" | "Object" | "Complement"
                ) {
                    return Err(syn::Error::new_spanned(
                        relation,
                        "unknown grammatical relation",
                    ));
                }
                if !categories.contains(&category.to_string()) {
                    return Err(syn::Error::new_spanned(
                        category,
                        "role refers to an undeclared frame category",
                    ));
                }
            }
            let mut coordinators = Vec::new();
            for path in &family.coordinators {
                if path.leading_colon.is_some() || path.segments.len() != 2 {
                    return Err(syn::Error::new_spanned(
                        path,
                        "coordinator requires Vocabulary::Member",
                    ));
                }
                let vocabulary = &path.segments[0].ident;
                let member = &path.segments[1].ident;
                if !terminals.iter().any(|terminal| match terminal {
                    TerminalPlan::Vocab(vocab) => {
                        vocab.name_ident() == vocabulary
                            && vocab.variants().iter().any(|v| v.name() == member)
                    }
                    _ => false,
                }) {
                    return Err(syn::Error::new_spanned(
                        path,
                        "unknown coordinator lexical reference",
                    ));
                }
                let pair = (vocabulary.clone(), member.clone());
                if coordinators.contains(&pair) {
                    return Err(syn::Error::new_spanned(path, "duplicate coordinator"));
                }
                coordinators.push(pair);
            }
            Ok(FrameFamilyPlan {
                name: family.name.clone(),
                categories: family.categories.clone(),
                roles: family.roles.clone(),
                coordinators,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use quote::quote;

    #[test]
    fn frame_family_rejects_unknown_categories_relations_and_coordinators() {
        let cases = [
            (
                quote! { categories: [Unknown], roles: [], coordinators: [Join::And] },
                "unknown frame category",
            ),
            (
                quote! { categories: [Phrase], roles: [Legacy = Agent(Phrase)], coordinators: [Join::And] },
                "unknown grammatical relation",
            ),
            (
                quote! { categories: [Phrase], roles: [Legacy = Object(Unknown)], coordinators: [Join::And] },
                "role refers to an undeclared frame category",
            ),
            (
                quote! { categories: [Phrase], roles: [], coordinators: [Join::Unknown] },
                "unknown coordinator lexical reference",
            ),
        ];
        for (body, expected) in cases {
            let error = crate::generate(quote! {
                vocab Join { And = "and", }
                construction phrase: Phrase { element PhraseValue {} form phrase = "object"; }
                frame_family VerbPhrase { #body }
            })
            .expect_err("an unresolved family must fail before code generation");
            assert_eq!(error.to_string(), expected);
        }
    }
}
