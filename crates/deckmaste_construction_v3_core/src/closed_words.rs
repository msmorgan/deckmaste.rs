use super::{Declaration, Equation, FieldType, Part, Reference};
use std::collections::BTreeSet;
use syn::{
    Ident, LitStr, Token, braced, bracketed,
    parse::{Parse, ParseStream},
};

pub(crate) struct Word {
    pub name: Ident,
    pub category: Ident,
    pub id: LitStr,
    pub lemma: LitStr,
    pub classes: Vec<Ident>,
    pub properties: Vec<(LitStr, LitStr)>,
}

pub(crate) struct Class {
    pub name: Ident,
    pub categories: Vec<Ident>,
}

pub(crate) fn check_name(name: &Ident, class: bool) -> Ident {
    Ident::new(
        &format!("__{}_{name}", if class { "class" } else { "word" }),
        name.span(),
    )
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
    let body;
    braced!(body in input);
    let mut id = None;
    let mut lemma = None;
    let mut classes = vec![];
    let mut properties = vec![];
    let mut fields = BTreeSet::new();
    while !body.is_empty() {
        let key: Ident = body.parse()?;
        if !fields.insert(key.to_string()) {
            return Err(syn::Error::new(
                key.span(),
                "duplicate closed-word property",
            ));
        }
        match key.to_string().as_str() {
            "id" => id = Some(body.parse()?),
            "lemma" => lemma = Some(body.parse()?),
            "classes" => {
                let members;
                bracketed!(members in body);
                classes = members
                    .parse_terminated(Ident::parse, Token![,])?
                    .into_iter()
                    .collect();
            }
            "properties" => {
                let entries;
                braced!(entries in body);
                while !entries.is_empty() {
                    let key = entries.parse()?;
                    entries.parse::<Token![:]>()?;
                    properties.push((key, entries.parse()?));
                    if entries.is_empty() {
                        break;
                    }
                    entries.parse::<Token![,]>()?;
                }
            }
            _ => {
                return Err(syn::Error::new(
                    key.span(),
                    "expected id, lemma, classes or properties",
                ));
            }
        }
        body.parse::<Token![;]>()?;
    }
    Ok(Word {
        id: id.ok_or_else(|| syn::Error::new(name.span(), "closed word requires id"))?,
        lemma: lemma.ok_or_else(|| syn::Error::new(name.span(), "closed word requires lemma"))?,
        name,
        category,
        classes,
        properties,
    })
}

fn validate_declarations(declaration: &Declaration) -> syn::Result<()> {
    let mut names = BTreeSet::new();
    let mut ids = BTreeSet::new();
    for word in &declaration.words {
        if word.id.value().is_empty() || word.lemma.value().is_empty() {
            return Err(syn::Error::new(
                word.name.span(),
                "closed word requires nonempty id and lemma",
            ));
        }
        let mut properties = BTreeSet::new();
        for (key, _) in &word.properties {
            if !properties.insert(key.value()) {
                return Err(syn::Error::new(key.span(), "duplicate lexical property"));
            }
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
        for name in &word.classes {
            let class = declaration
                .lexical_classes
                .iter()
                .find(|class| class.name == *name)
                .ok_or_else(|| syn::Error::new(name.span(), "unknown lexical class"))?;
            if !class.categories.contains(&word.category) {
                return Err(syn::Error::new(
                    name.span(),
                    "word Category outside lexical class",
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
    let checks: Vec<_> = declaration
        .words
        .iter()
        .map(|w| (check_name(&w.name, false), vec![w.category.clone()]))
        .chain(
            declaration
                .lexical_classes
                .iter()
                .map(|c| (check_name(&c.name, true), c.categories.clone())),
        )
        .collect();
    for construction in &mut declaration.constructions {
        for (form, requirements) in construction
            .forms
            .iter_mut()
            .zip(&mut construction.form_requirements)
        {
            for part in form {
                if let Part::Field(field, FieldType::Lexical(category)) = part
                    && let Some(name) = category.strip_prefix('@')
                {
                    let word = declaration
                        .words
                        .iter()
                        .find(|word| word.name == name)
                        .ok_or_else(|| {
                            syn::Error::new(field.span(), format!("unknown closed word {name}"))
                        })?;
                    *category = word.category.to_string();
                    requirements.push(Equation::Require(
                        Reference {
                            field: field.clone(),
                            feature: check_name(&word.name, false),
                            last: false,
                        },
                        Ident::new("Yes", field.span()),
                    ));
                }
            }
        }
        for (equation, only_form) in construction
            .equations
            .iter_mut()
            .map(|equation| (equation, None))
            .chain(
                construction
                    .form_requirements
                    .iter_mut()
                    .enumerate()
                    .flat_map(|(index, equations)| {
                        equations
                            .iter_mut()
                            .map(move |equation| (equation, Some(index)))
                    }),
            )
        {
            for reference in equation.references_mut() {
                let name = reference.feature.to_string();
                if !name.starts_with("__word_") && !name.starts_with("__class_") {
                    continue;
                }
                let (_, categories) = checks
                    .iter()
                    .find(|(name, _)| *name == reference.feature)
                    .ok_or_else(|| {
                        syn::Error::new(
                            reference.feature.span(),
                            "unknown closed word or lexical class",
                        )
                    })?;
                for (index, form) in construction.forms.iter().enumerate() {
                    if only_form.is_some_and(|only| only != index) {
                        continue;
                    }
                    let category = form
                        .iter()
                        .find_map(|part| match part {
                            Part::Field(
                                field,
                                FieldType::Lexical(category)
                                | FieldType::OptionalLexical(category, _, _),
                            ) if *field == reference.field => Some(category),
                            _ => None,
                        })
                        .ok_or_else(|| {
                            syn::Error::new(
                                reference.field.span(),
                                "word/class requirement requires a lexical field",
                            )
                        })?;
                    if !categories.iter().any(|c| c == category) {
                        return Err(syn::Error::new(
                            reference.field.span(),
                            "lexical field Category outside word/class declaration",
                        ));
                    }
                }
            }
        }
    }
    Ok(())
}
