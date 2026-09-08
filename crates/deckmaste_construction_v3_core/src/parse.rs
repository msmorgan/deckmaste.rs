use syn::Ident;
use syn::LitStr;
use syn::Token;
use syn::Visibility;
use syn::braced;
use syn::bracketed;
use syn::parenthesized;
use syn::parse::Parse;
use syn::parse::ParseStream;

pub(crate) struct Declaration {
    pub visibility: Visibility,
    pub module: Ident,
    pub categories: Vec<Category>,
    pub features: Vec<Feature>,
    pub frames: Vec<(Ident, LitStr)>,
    pub tables: Vec<FeatureTable>,
    pub constructions: Vec<Construction>,
    pub capitalization: Option<Ident>,
}

pub(crate) struct Category {
    pub name: Ident,
    pub features: Vec<Ident>,
}

pub(crate) struct Feature {
    pub name: Ident,
    pub values: Vec<Ident>,
}

pub(crate) struct FeatureTable {
    pub name: Ident,
    pub inputs: Vec<Ident>,
    pub output: Ident,
    pub rows: Vec<(Vec<Ident>, Ident)>,
}

pub(crate) struct Construction {
    pub name: Ident,
    pub category: Ident,
    pub forms: Vec<Vec<Part>>,
    pub equations: Vec<Equation>,
    pub boundary: Option<Ident>,
    pub onset: Option<Ident>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum FieldType {
    Lexical(String),
    One(String),
    Optional(String),
    Repeated(String, String),
}

#[derive(Clone)]
pub(crate) enum Part {
    Literal(LitStr),
    Field(Ident, FieldType),
}

pub(crate) struct Reference {
    pub field: Ident,
    pub feature: Ident,
}

pub(crate) enum Equation {
    Export(Ident, Reference),
    ExportConstant(Ident, Ident),
    ExportTable(Ident, Ident, Vec<Reference>),
    Agree(Reference, Reference),
    Require(Reference, Ident),
}

fn names(input: ParseStream<'_>) -> syn::Result<Vec<Ident>> {
    input
        .parse_terminated(Ident::parse, Token![,])
        .map(|v| v.into_iter().collect())
}

fn reference(input: ParseStream<'_>) -> syn::Result<Reference> {
    let field = input.parse()?;
    input.parse::<Token![.]>()?;
    Ok(Reference {
        field,
        feature: input.parse()?,
    })
}

impl Parse for Declaration {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let visibility = input.parse()?;
        input.parse::<Token![mod]>()?;
        let module = input.parse()?;
        let body;
        braced!(body in input);
        let mut result = Self {
            visibility,
            module,
            categories: vec![],
            features: vec![],
            frames: vec![],
            tables: vec![],
            constructions: vec![],
            capitalization: None,
        };
        while !body.is_empty() {
            let keyword: Ident = body.parse()?;
            let name: Ident = body.parse()?;
            match keyword.to_string().as_str() {
                "capitalization" => {
                    if name != "Positional" || result.capitalization.replace(name.clone()).is_some()
                    {
                        return Err(syn::Error::new(
                            name.span(),
                            "declare capitalization Positional once",
                        ));
                    }
                    body.parse::<Token![;]>()?;
                }
                "category" => {
                    let features;
                    parenthesized!(features in body);
                    result.categories.push(Category {
                        name,
                        features: names(&features)?,
                    });
                    body.parse::<Token![;]>()?;
                }
                "feature" => {
                    let values;
                    braced!(values in body);
                    result.features.push(Feature {
                        name,
                        values: names(&values)?,
                    });
                }
                "frame" => {
                    body.parse::<Token![=]>()?;
                    result.frames.push((name, body.parse()?));
                    body.parse::<Token![;]>()?;
                }
                "table" => {
                    let inputs;
                    parenthesized!(inputs in body);
                    let inputs = names(&inputs)?;
                    body.parse::<Token![->]>()?;
                    let output = body.parse()?;
                    let rows;
                    braced!(rows in body);
                    let mut values = vec![];
                    while !rows.is_empty() {
                        let arguments;
                        parenthesized!(arguments in rows);
                        let arguments = names(&arguments)?;
                        rows.parse::<Token![=>]>()?;
                        values.push((arguments, rows.parse()?));
                        if rows.is_empty() {
                            break;
                        }
                        rows.parse::<Token![,]>()?;
                    }
                    result.tables.push(FeatureTable {
                        name,
                        inputs,
                        output,
                        rows: values,
                    });
                }
                "construction" => {
                    result.constructions.push(parse_construction(&body, name)?);
                }
                _ => {
                    return Err(syn::Error::new(
                        keyword.span(),
                        "expected capitalization, category, feature, frame, table or construction",
                    ));
                }
            }
        }
        Ok(result)
    }
}

fn parse_construction(body: ParseStream<'_>, name: Ident) -> syn::Result<Construction> {
    body.parse::<Token![:]>()?;
    let category = body.parse()?;
    let contents;
    braced!(contents in body);
    let mut construction = Construction {
        name,
        category,
        forms: vec![],
        equations: vec![],
        boundary: None,
        onset: None,
    };
    while !contents.is_empty() {
        let directive: Ident = contents.parse()?;
        match directive.to_string().as_str() {
            "boundary" | "onset" => {
                let value: Ident = contents.parse()?;
                let slot = if directive == "boundary" {
                    &mut construction.boundary
                } else {
                    &mut construction.onset
                };
                if slot.replace(value).is_some() {
                    return Err(syn::Error::new(
                        directive.span(),
                        "duplicate surface obligation",
                    ));
                }
            }
            "form" => {
                let form;
                bracketed!(form in contents);
                let parts = parse_form(&form)?;
                construction.forms.push(parts);
            }
            "export" => {
                let feature = contents.parse()?;
                contents.parse::<Token![=]>()?;
                let equation = if contents.peek2(Token![.]) {
                    Equation::Export(feature, reference(&contents)?)
                } else if contents.peek2(syn::token::Paren) {
                    let table = contents.parse()?;
                    let arguments;
                    parenthesized!(arguments in contents);
                    let arguments = arguments.parse_terminated(reference, Token![,])?;
                    Equation::ExportTable(feature, table, arguments.into_iter().collect())
                } else {
                    Equation::ExportConstant(feature, contents.parse()?)
                };
                construction.equations.push(equation);
            }
            "agree" => {
                let left = reference(&contents)?;
                contents.parse::<Token![=]>()?;
                construction
                    .equations
                    .push(Equation::Agree(left, reference(&contents)?));
            }
            "require" => {
                let left = reference(&contents)?;
                contents.parse::<Token![=]>()?;
                construction
                    .equations
                    .push(Equation::Require(left, contents.parse()?));
            }
            _ => {
                return Err(syn::Error::new(
                    directive.span(),
                    format!(
                        "construction {}: unknown generated obligation; expected form, export, agree, require, boundary or onset",
                        construction.name
                    ),
                ));
            }
        }
        contents.parse::<Token![;]>()?;
    }
    Ok(construction)
}

fn parse_form(form: ParseStream<'_>) -> syn::Result<Vec<Part>> {
    let mut parts = vec![];
    while !form.is_empty() {
        if form.peek(LitStr) {
            parts.push(Part::Literal(form.parse()?));
        } else {
            let field = form.parse()?;
            form.parse::<Token![:]>()?;
            let ty: Ident = form.parse()?;
            let ty = if form.peek(syn::token::Paren) {
                let arguments;
                parenthesized!(arguments in form);
                let category: Ident = arguments.parse()?;
                let ty = match ty.to_string().as_str() {
                    "lexical" => FieldType::Lexical(category.to_string()),
                    "optional" => FieldType::Optional(category.to_string()),
                    "repeat" => {
                        arguments.parse::<Token![,]>()?;
                        let separator: LitStr = arguments.parse()?;
                        FieldType::Repeated(category.to_string(), separator.value())
                    }
                    _ => {
                        return Err(syn::Error::new(
                            ty.span(),
                            "expected lexical, optional or repeat",
                        ));
                    }
                };
                if !arguments.is_empty() {
                    return Err(arguments.error("unexpected field arguments"));
                }
                ty
            } else {
                FieldType::One(ty.to_string())
            };
            parts.push(Part::Field(field, ty));
        }
        if form.is_empty() {
            break;
        }
        form.parse::<Token![,]>()?;
    }
    Ok(parts)
}
