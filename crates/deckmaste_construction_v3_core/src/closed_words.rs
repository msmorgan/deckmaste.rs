use super::{Declaration, Equation, FieldType, Part};
use std::collections::BTreeSet;
use syn::{Ident, LitStr, Token, parse::ParseStream};

pub(crate) struct Word {
    pub name: Ident,
    pub category: Ident,
    pub id: LitStr,
}

pub(crate) struct Class {
    pub name: Ident,
    pub categories: Vec<Ident>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Constraint {
    Word(Ident),
    Class(Ident),
}

pub(crate) fn parse_class(input: ParseStream<'_>, name: Ident) -> syn::Result<Class> {
    input.parse::<Token![:]>()?;
    let mut categories = vec![input.parse()?];
    while input.peek(Token![|]) {
        input.parse::<Token![|]>()?;
        categories.push(input.parse()?);
    }
    input.parse::<Token![;]>()?;
    Ok(Class { name, categories })
}

pub(crate) fn parse_word(input: ParseStream<'_>, name: Ident) -> syn::Result<Word> {
    input.parse::<Token![:]>()?;
    let category = input.parse()?;
    input.parse::<Token![=]>()?;
    let id = input.parse()?;
    input.parse::<Token![;]>()?;
    Ok(Word { name, category, id })
}

fn validate_declarations(declaration: &Declaration) -> syn::Result<()> {
    let mut names = BTreeSet::new();
    let mut ids = BTreeSet::new();
    for word in &declaration.words {
        if word.id.value().is_empty() {
            return Err(syn::Error::new(
                word.name.span(),
                "closed word requires nonempty id",
            ));
        }
        if !names.insert(word.name.to_string()) || !ids.insert(word.id.value()) {
            return Err(syn::Error::new(
                word.name.span(),
                "duplicate closed word or lexeme id",
            ));
        }
        match word.category.to_string().as_str() {
            "Preposition" | "Determinative" | "Pronoun" | "Coordinator" | "Subordinator"
            | "Adverb" | "Verb" => {}
            _ => {
                return Err(syn::Error::new(
                    word.category.span(),
                    "closed word requires a closed-class lexical Category",
                ));
            }
        }
    }
    for class in &declaration.lexical_classes {
        if !names.insert(class.name.to_string()) {
            return Err(syn::Error::new(
                class.name.span(),
                "duplicate lexical declaration",
            ));
        }
        for category in &class.categories {
            ron::from_str::<deckmaste_lexical_model::Category>(&category.to_string())
                .map_err(|_| syn::Error::new(category.span(), "unknown lexical Category"))?;
        }
    }
    Ok(())
}

pub(crate) fn elaborate(declaration: &mut Declaration) -> syn::Result<()> {
    validate_declarations(declaration)?;
    for construction in &mut declaration.constructions {
        for (form, requirements) in construction
            .forms
            .iter_mut()
            .zip(&mut construction.form_requirements)
        {
            for part in form {
                if let Part::Field(field, kind @ FieldType::LexicalWord(_)) = part {
                    let FieldType::LexicalWord(name) = kind else { unreachable!() };
                    let word = declaration
                        .words
                        .iter()
                        .find(|word| word.name == *name)
                        .ok_or_else(|| syn::Error::new(name.span(), "unknown closed word"))?;
                    requirements.push(Equation::RequireLexical(
                        field.clone(),
                        Constraint::Word(word.name.clone()),
                    ));
                    *kind = FieldType::Lexical(word.category.to_string());
                }
            }
        }
        for (equation, only_form) in construction.equations.iter().map(|e| (e, None)).chain(
            construction
                .form_requirements
                .iter()
                .enumerate()
                .flat_map(|(i, es)| es.iter().map(move |e| (e, Some(i)))),
        ) {
            let Equation::RequireLexical(field, target) = equation else { continue };
            let categories = match target {
                Constraint::Word(name) => vec![
                    &declaration
                        .words
                        .iter()
                        .find(|w| w.name == *name)
                        .ok_or_else(|| syn::Error::new(name.span(), "unknown closed word"))?
                        .category,
                ],
                Constraint::Class(name) => declaration
                    .lexical_classes
                    .iter()
                    .find(|c| c.name == *name)
                    .ok_or_else(|| syn::Error::new(name.span(), "unknown lexical class"))?
                    .categories
                    .iter()
                    .collect(),
            };
            for (index, form) in construction.forms.iter().enumerate() {
                if only_form.is_some_and(|only| only != index) {
                    continue;
                }
                let category = form
                    .iter()
                    .find_map(|part| match part {
                        Part::Field(
                            name,
                            FieldType::Lexical(c) | FieldType::OptionalLexical(c, _, _),
                        ) if name == field => Some(c),
                        _ => None,
                    })
                    .ok_or_else(|| {
                        syn::Error::new(
                            field.span(),
                            "word/class requirement requires a lexical field",
                        )
                    })?;
                if !categories.iter().any(|c| *c == category) {
                    return Err(syn::Error::new(
                        field.span(),
                        "lexical field Category outside word/class declaration",
                    ));
                }
            }
        }
    }
    Ok(())
}
