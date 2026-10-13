use super::Category;
use super::Construction;
use super::Declaration;
use super::Equation;
use super::FeatureTable;
use super::FieldType;
use super::InterfaceFeature;
use super::Part;
use super::Reference;
use super::TablePattern;
use super::TableResult;
use super::parse_directive;
use super::reference;
use std::collections::{BTreeMap, BTreeSet};
use syn::Ident;
use syn::Token;
use syn::braced;
use syn::parenthesized;
use syn::parse::ParseStream;

#[derive(Clone)]
pub(crate) enum Combiner {
    All,
    Any,
    Same,
    Value(Ident),
    Copy(Reference),
    Table(Ident, Vec<Reference>),
}

pub(super) fn parse_combiner(input: ParseStream<'_>) -> syn::Result<Combiner> {
    if input.peek2(Token![.]) {
        return Ok(Combiner::Copy(reference(input)?));
    }
    let name: Ident = input.parse()?;
    if input.peek(syn::token::Paren) {
        let args;
        parenthesized!(args in input);
        return Ok(Combiner::Table(
            name,
            args.parse_terminated(reference, Token![,])?
                .into_iter()
                .collect(),
        ));
    }
    Ok(match name.to_string().as_str() {
        "all" => Combiner::All,
        "any" => Combiner::Any,
        "same" => Combiner::Same,
        _ => Combiner::Value(name),
    })
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Role {
    Binary,
    End,
    Continuation,
    Complete,
}

struct Template {
    name: Ident,
    role: Role,
    correlative: bool,
}

impl Template {
    fn helper(&self) -> bool {
        matches!(self.role, Role::End | Role::Continuation)
    }
    fn tail(&self) -> bool {
        matches!(self.role, Role::Continuation | Role::Complete)
    }
    fn matches(&self, context: &Ident) -> bool {
        match context.to_string().as_str() {
            "series" => self.helper(),
            "complete" => !self.helper(),
            "serial" => self.role != Role::Binary,
            "binary" => self.role == Role::Binary,
            "ordinary" => !self.correlative,
            "correlative" => self.correlative,
            _ => self.name == *context,
        }
    }
}

pub(super) struct Forms {
    name: Ident,
    templates: Vec<Template>,
}

pub(super) fn parse_forms(input: ParseStream<'_>, name: Ident) -> syn::Result<Forms> {
    let body;
    braced!(body in input);
    let mut templates = Vec::new();
    let mut seen = BTreeSet::new();
    while !body.is_empty() {
        let kind: Ident = body.parse()?;
        let mut list = vec![body.parse::<Ident>()?];
        while body.peek(Token![,]) {
            body.parse::<Token![,]>()?;
            list.push(body.parse()?);
        }
        body.parse::<Token![;]>()?;
        let roles: &[Role] = match (kind.to_string().as_str(), list.len()) {
            ("binary" | "correlative", 1) => &[Role::Binary],
            ("series", 3) => &[Role::End, Role::Continuation, Role::Complete],
            ("correlative_series", 2) => &[Role::End, Role::Continuation],
            ("correlative", 2) => &[Role::Binary, Role::Complete],
            _ => {
                return Err(syn::Error::new(
                    kind.span(),
                    "expected binary (one schema), series (end, continuation, complete), correlative_series (end, continuation), or correlative (binary, optional complete)",
                ));
            }
        };
        for (name, role) in list.into_iter().zip(roles) {
            if !seen.insert(name.to_string()) {
                return Err(syn::Error::new(
                    name.span(),
                    "duplicate coordination template",
                ));
            }
            templates.push(Template {
                name,
                role: *role,
                correlative: kind.to_string().starts_with("correlative"),
            });
        }
    }
    Ok(Forms { name, templates })
}

struct Scope {
    contexts: Vec<Ident>,
    combiners: Vec<(Ident, Option<Combiner>)>,
    requirements: Vec<Equation>,
}

pub(super) struct Coordinable {
    category: Ident,
    profile: Ident,
    forms: Vec<Ident>,
    series: Option<Ident>,
    correlative_series: Option<Ident>,
    scopes: Vec<Scope>,
}

fn empty_instance(name: Ident, category: Ident) -> Construction {
    Construction {
        parameters: vec![],
        defaults: vec![],
        rows: vec![],
        shared: true,
        bindings: vec![],
        uses: vec![],
        cost: None,
        name,
        category,
        forms: vec![],
        form_requirements: vec![],
        equations: vec![],
        boundary: None,
        onset: None,
        segments: None,
        head: None,
    }
}

fn scope_directive(
    input: ParseStream<'_>,
    directive: &Ident,
    scope: &mut Scope,
) -> syn::Result<()> {
    match directive.to_string().as_str() {
        "combine" | "omit" => {
            let feature: Ident = input.parse()?;
            if scope.combiners.iter().any(|(name, _)| *name == feature) {
                return Err(syn::Error::new(
                    feature.span(),
                    "duplicate combiner override",
                ));
            }
            let combiner = if directive == "combine" {
                input.parse::<Token![=]>()?;
                Some(parse_combiner(input)?)
            } else {
                None
            };
            scope.combiners.push((feature, combiner));
        }
        "require" | "agree" => {
            let mut holder = empty_instance(directive.clone(), directive.clone());
            parse_directive(input, directive, &mut holder)?;
            scope.requirements.extend(holder.equations);
        }
        _ => {
            return Err(syn::Error::new(
                directive.span(),
                "expected combine, omit, require or agree",
            ));
        }
    }
    input.parse::<Token![;]>()?;
    Ok(())
}

pub(super) fn parse_coordinable(
    input: ParseStream<'_>,
    category: Ident,
) -> syn::Result<Coordinable> {
    let using: Ident = input.parse()?;
    if using != "using" {
        return Err(syn::Error::new(
            using.span(),
            "expected using coordination_forms",
        ));
    }
    let profile = input.parse()?;
    let body;
    braced!(body in input);
    let mut result = Coordinable {
        category,
        profile,
        forms: vec![],
        series: None,
        correlative_series: None,
        scopes: vec![],
    };
    let mut global = Scope {
        contexts: vec![],
        combiners: vec![],
        requirements: vec![],
    };
    while !body.is_empty() {
        let directive: Ident = body.parse()?;
        match directive.to_string().as_str() {
            "forms" => {
                if !result.forms.is_empty() {
                    return Err(syn::Error::new(
                        directive.span(),
                        "duplicate forms selection",
                    ));
                }
                result.forms.push(body.parse()?);
                while body.peek(Token![,]) {
                    body.parse::<Token![,]>()?;
                    result.forms.push(body.parse()?);
                }
                body.parse::<Token![;]>()?;
            }
            "series" | "correlative_series" => {
                let slot = if directive == "series" {
                    &mut result.series
                } else {
                    &mut result.correlative_series
                };
                if slot.replace(body.parse()?).is_some() {
                    return Err(syn::Error::new(
                        directive.span(),
                        "duplicate series category",
                    ));
                }
                body.parse::<Token![;]>()?;
            }
            "on" => {
                let mut contexts = vec![body.parse()?];
                while body.peek(Token![,]) {
                    body.parse::<Token![,]>()?;
                    contexts.push(body.parse()?);
                }
                let statements;
                braced!(statements in body);
                let mut scope = Scope {
                    contexts,
                    combiners: vec![],
                    requirements: vec![],
                };
                while !statements.is_empty() {
                    let keyword = statements.parse()?;
                    scope_directive(&statements, &keyword, &mut scope)?;
                }
                result.scopes.push(scope);
            }
            _ => scope_directive(&body, &directive, &mut global)?,
        }
    }
    result.scopes.insert(0, global);
    Ok(result)
}

fn reference_to(field: &str, feature: &Ident) -> Reference {
    Reference {
        field: Ident::new(field, feature.span()),
        feature: feature.clone(),
        last: false,
    }
}

fn equations(
    feature: &Ident,
    combiner: &Combiner,
    declaration: &mut Declaration,
) -> syn::Result<Vec<Equation>> {
    let left = reference_to("left", feature);
    let right = reference_to("right", feature);
    Ok(match combiner {
        Combiner::Same => vec![
            Equation::Agree(left.clone(), right),
            Equation::Export(feature.clone(), left),
        ],
        Combiner::Value(value) => vec![Equation::ExportConstant(feature.clone(), value.clone())],
        Combiner::Copy(source) => vec![Equation::Export(feature.clone(), source.clone())],
        Combiner::Table(table, args) => vec![Equation::ExportTable(
            feature.clone(),
            table.clone(),
            args.clone(),
        )],
        Combiner::All | Combiner::Any => {
            let domain = declaration.features.iter().find(|f| f.name == *feature);
            if !domain.is_some_and(|f| {
                !f.set
                    && f.values.len() == 2
                    && f.values.iter().any(|v| v == "Yes")
                    && f.values.iter().any(|v| v == "No")
            }) {
                return Err(syn::Error::new(
                    feature.span(),
                    "all/any requires a Yes/No feature domain",
                ));
            }
            let all = matches!(combiner, Combiner::All);
            let name = Ident::new(
                &format!(
                    "coordination_combine_{}_{}",
                    if all { "all" } else { "any" },
                    feature
                ),
                feature.span(),
            );
            if !declaration.tables.iter().any(|t| t.name == name) {
                let exact = Ident::new(if all { "Yes" } else { "No" }, feature.span());
                let fallback = Ident::new(if all { "No" } else { "Yes" }, feature.span());
                declaration.tables.push(FeatureTable {
                    name: name.clone(),
                    inputs: vec![feature.clone(), feature.clone()],
                    output: Some(feature.clone()),
                    rows: vec![
                        (
                            vec![
                                TablePattern::Exact(exact.clone()),
                                TablePattern::Exact(exact.clone()),
                            ],
                            TableResult::Feature(exact),
                        ),
                        (
                            vec![TablePattern::Any, TablePattern::Any],
                            TableResult::Feature(fallback),
                        ),
                    ],
                    operation: None,
                });
            }
            vec![Equation::ExportTable(
                feature.clone(),
                name,
                vec![left, right],
            )]
        }
    })
}

fn export_feature(equation: &Equation) -> Option<&Ident> {
    match equation {
        Equation::Export(f, _)
        | Equation::ExportConstant(f, _)
        | Equation::ExportTable(f, _, _) => Some(f),
        _ => None,
    }
}

fn select<'a>(item: &Coordinable, profile: &'a Forms) -> syn::Result<Vec<&'a Template>> {
    let mut selected = BTreeSet::new();
    for name in &item.forms {
        let matching: Vec<_> = profile
            .templates
            .iter()
            .filter(|t| match name.to_string().as_str() {
                "binary" => !t.correlative && t.role == Role::Binary,
                "serial" => !t.correlative && t.role != Role::Binary,
                _ => t.name == *name && !t.helper(),
            })
            .collect();
        if matching.is_empty() {
            return Err(syn::Error::new(
                name.span(),
                "unknown coordination form selection",
            ));
        }
        for t in matching {
            if !selected.insert(t.name.to_string()) {
                return Err(syn::Error::new(
                    name.span(),
                    "duplicate coordination form selection",
                ));
            }
        }
    }
    if selected.is_empty() {
        return Err(syn::Error::new(
            item.category.span(),
            "coordinable requires forms",
        ));
    }
    if profile.templates.iter().any(|t| {
        t.correlative && t.role == Role::Complete && selected.contains(&t.name.to_string())
    }) {
        for role in [Role::End, Role::Continuation] {
            let helpers: Vec<_> = profile
                .templates
                .iter()
                .filter(|t| t.correlative && t.role == role)
                .collect();
            if helpers.len() != 1 {
                return Err(syn::Error::new(
                    profile.name.span(),
                    "correlative serial forms require one end and continuation template",
                ));
            }
            selected.insert(helpers[0].name.to_string());
        }
    }
    let templates: Vec<_> = profile
        .templates
        .iter()
        .filter(|t| selected.contains(&t.name.to_string()))
        .collect();
    for scope in &item.scopes {
        for context in &scope.contexts {
            if !templates.iter().any(|t| t.matches(context)) {
                return Err(syn::Error::new(
                    context.span(),
                    "coordination context matches no selected form",
                ));
            }
        }
    }
    Ok(templates)
}

type Combinations = Vec<(Ident, Combiner)>;

fn contract(
    item: &Coordinable,
    template: &Template,
    base: &Category,
) -> syn::Result<(Combinations, Vec<Equation>)> {
    let mut values: BTreeMap<String, (Ident, Option<Combiner>, usize)> = base
        .features
        .iter()
        .map(|f| (f.name.to_string(), (f.name.clone(), f.combiner.clone(), 0)))
        .collect();
    let mut requirements = vec![];
    let mut assigned = BTreeSet::new();
    for scope in &item.scopes {
        if !scope.contexts.is_empty() && !scope.contexts.iter().any(|c| template.matches(c)) {
            continue;
        }
        let rank = if scope.contexts.is_empty() {
            1
        } else if scope.contexts.contains(&template.name) {
            3
        } else {
            2
        };
        for (feature, combiner) in &scope.combiners {
            if !assigned.insert((feature.to_string(), rank)) {
                return Err(syn::Error::new(
                    feature.span(),
                    "conflicting coordination overrides at the same specificity",
                ));
            }
            if combiner.is_none() && !template.helper() {
                return Err(syn::Error::new(
                    feature.span(),
                    "omit applies only to series summaries",
                ));
            }
            if !values.contains_key(&feature.to_string()) && combiner.is_none() {
                return Err(syn::Error::new(
                    feature.span(),
                    "cannot omit unknown coordination feature",
                ));
            }
            if values
                .get(&feature.to_string())
                .is_none_or(|(_, _, old)| *old < rank)
            {
                values.insert(
                    feature.to_string(),
                    (feature.clone(), combiner.clone(), rank),
                );
            }
        }
        requirements.extend(scope.requirements.clone());
    }
    let mut result = vec![];
    for (_, (feature, combiner, rank)) in values {
        match combiner {
            Some(c) => result.push((feature, c)),
            None if rank == 0 => {
                return Err(syn::Error::new(
                    feature.span(),
                    "generated coordination requires a combiner for every interface entry",
                ));
            }
            None => (),
        }
    }
    Ok((result, requirements))
}

pub(super) fn elaborate(
    declaration: &mut Declaration,
    schemas: &[Construction],
    profiles: &[Forms],
    items: &[Coordinable],
) -> syn::Result<()> {
    let mut names = BTreeSet::new();
    for profile in profiles {
        if !names.insert(profile.name.to_string()) {
            return Err(syn::Error::new(
                profile.name.span(),
                "duplicate coordination_forms",
            ));
        }
        for template in &profile.templates {
            if !schemas.iter().any(|s| s.name == template.name) {
                return Err(syn::Error::new(
                    template.name.span(),
                    "unknown coordination schema",
                ));
            }
        }
    }
    // These names are reserved so an authored table cannot replace strict combination.
    if let Some(table) = declaration
        .tables
        .iter()
        .find(|t| t.name.to_string().starts_with("coordination_combine_"))
    {
        return Err(syn::Error::new(
            table.name.span(),
            "reserved generated combiner table name",
        ));
    }
    let mut categories = BTreeSet::new();
    for item in items {
        if !categories.insert(item.category.to_string()) {
            return Err(syn::Error::new(
                item.category.span(),
                "duplicate coordinable category",
            ));
        }
        let base = declaration
            .categories
            .iter()
            .find(|c| c.name == item.category)
            .cloned()
            .ok_or_else(|| syn::Error::new(item.category.span(), "unknown coordinable category"))?;
        let profile = profiles
            .iter()
            .find(|p| p.name == item.profile)
            .ok_or_else(|| syn::Error::new(item.profile.span(), "unknown coordination_forms"))?;
        elaborate_category(declaration, schemas, item, profile, &base)?;
    }
    Ok(())
}

fn elaborate_category(
    declaration: &mut Declaration,
    schemas: &[Construction],
    item: &Coordinable,
    profile: &Forms,
    base: &Category,
) -> syn::Result<()> {
    let selected = select(item, profile)?;
    let mut generated = vec![];
    let mut helpers = HelperInterfaces::new();
    for template in &selected {
        let schema = schemas
            .iter()
            .find(|s| s.name == template.name)
            .expect("validated schema");
        let helper = if template.correlative { &item.correlative_series } else { &item.series };
        if template.role != Role::Binary && helper.is_none() {
            return Err(syn::Error::new(
                item.category.span(),
                "serial forms require a named series category",
            ));
        }
        let result = if template.helper() {
            helper.as_ref().expect("checked helper")
        } else {
            &item.category
        };
        let mut instance = empty_instance(template.name.clone(), result.clone());
        let (combiners, requirements) = contract(item, template, base)?;
        for (feature, combiner) in combiners {
            instance
                .equations
                .extend(equations(&feature, &combiner, declaration)?);
        }
        instance.equations.extend(requirements);
        for equation in &mut instance.equations {
            for r in equation.references_mut() {
                r.field = match r.field.to_string().as_str() {
                    "left" => r.field.clone(),
                    "right" => Ident::new(
                        if template.tail() { "rest" } else { "right" },
                        r.field.span(),
                    ),
                    "source" => Ident::new(
                        if template.tail() { "rest" } else { "coordinator" },
                        r.field.span(),
                    ),
                    _ => {
                        return Err(syn::Error::new(
                            r.field.span(),
                            "combiner references use left, right or source",
                        ));
                    }
                };
            }
        }
        for form in &schema.forms {
            for part in form {
                if let Part::Field(field, FieldType::One(ty)) = part
                    && ty == "node"
                {
                    let category = match field.to_string().as_str() {
                        "left" => &item.category,
                        "right" if !template.tail() => &item.category,
                        "rest" if template.tail() => helper.as_ref().expect("checked helper"),
                        _ => {
                            return Err(syn::Error::new(
                                field.span(),
                                "coordination template node fields must be left and right, or left and rest",
                            ));
                        }
                    };
                    if !instance.bindings.iter().any(|(f, _)| f == field) {
                        instance.bindings.push((field.clone(), category.clone()));
                    }
                }
            }
        }
        if template.helper() {
            let mut features = BTreeMap::new();
            for e in instance
                .equations
                .iter()
                .chain(&schema.equations)
                .chain(schema.form_requirements.iter().flatten())
            {
                if let Some(f) = export_feature(e) {
                    features.insert(f.to_string(), f.clone());
                }
            }
            if let Some((_, previous)) = helpers.get(&result.to_string())
                && previous.keys().ne(features.keys())
            {
                return Err(syn::Error::new(
                    result.span(),
                    "series end and continuation must expose the same summary features",
                ));
            }
            helpers.insert(result.to_string(), (result.clone(), features));
        }
        generated.push((*template, instance));
    }
    carry_summaries(
        declaration,
        schemas,
        item,
        base,
        &mut generated,
        &mut helpers,
    )?;
    for (_, (name, features)) in helpers {
        if declaration.categories.iter().any(|c| c.name == name) {
            return Err(syn::Error::new(
                name.span(),
                "generated series category already declared",
            ));
        }
        declaration.categories.push(Category {
            name,
            features: features
                .into_values()
                .map(|name| InterfaceFeature {
                    name,
                    propagation: None,
                    combiner: None,
                })
                .collect(),
        });
    }
    declaration
        .constructions
        .extend(generated.into_iter().map(|(_, instance)| instance));
    Ok(())
}

type HelperInterfaces = BTreeMap<String, (Ident, BTreeMap<String, Ident>)>;

fn carry_summaries(
    declaration: &mut Declaration,
    schemas: &[Construction],
    item: &Coordinable,
    base: &Category,
    generated: &mut [(&Template, Construction)],
    helpers: &mut HelperInterfaces,
) -> syn::Result<()> {
    // A coordinator feature read through a tail must be carried by its series.
    for correlative in [false, true] {
        let mut carried = BTreeMap::new();
        for (template, _) in generated.iter() {
            if template.correlative != correlative || !template.tail() {
                continue;
            }
            let (combiners, requirements) = contract(item, template, base)?;
            let mut eqs = requirements;
            for (feature, combiner) in combiners {
                eqs.extend(equations(&feature, &combiner, declaration)?);
            }
            for e in &mut eqs {
                for r in e.references_mut() {
                    if r.field == "source" {
                        carried.insert(r.feature.to_string(), r.feature.clone());
                    }
                }
            }
        }
        for (template, instance) in generated.iter_mut() {
            if template.correlative != correlative || !template.helper() {
                continue;
            }
            let (_, features) = helpers
                .get_mut(&instance.category.to_string())
                .expect("generated helper");
            let schema = schemas
                .iter()
                .find(|s| s.name == template.name)
                .expect("validated schema");
            for (name, feature) in &carried {
                if !instance
                    .equations
                    .iter()
                    .chain(&schema.equations)
                    .any(|e| export_feature(e) == Some(feature))
                {
                    instance.equations.push(Equation::Export(
                        feature.clone(),
                        reference_to(
                            if template.tail() { "rest" } else { "coordinator" },
                            feature,
                        ),
                    ));
                }
                features.insert(name.clone(), feature.clone());
            }
        }
    }

    Ok(())
}
