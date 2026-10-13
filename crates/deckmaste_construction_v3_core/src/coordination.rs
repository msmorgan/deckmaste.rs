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
    InferredTable(Ident),
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
        "always" => Combiner::Value(input.parse()?),
        _ => Combiner::InferredTable(name),
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
    kind: String,
}

impl Template {
    fn helper(&self) -> bool {
        matches!(self.role, Role::End | Role::Continuation)
    }
    fn tail(&self) -> bool {
        matches!(self.role, Role::Continuation | Role::Complete)
    }
    fn matches(&self, context: &Context) -> bool {
        if context.exact {
            return self.name == context.name;
        }
        match context.name.to_string().as_str() {
            "helper" => self.helper(),
            "result" => !self.helper(),
            "overt" => !self.tail(),
            "ordinary_overt" => !self.correlative && !self.tail(),
            "correlative" => self.correlative,
            _ => context.name == self.kind,
        }
    }
}

pub(super) struct Forms {
    name: Ident,
    templates: Vec<Template>,
    scopes: Vec<Scope>,
}

pub(super) fn parse_forms(input: ParseStream<'_>, name: Ident) -> syn::Result<Forms> {
    let body;
    braced!(body in input);
    let mut templates = Vec::new();
    let mut scopes = Vec::new();
    let mut seen = BTreeSet::new();
    while !body.is_empty() {
        let kind: Ident = body.parse()?;
        if kind == "on" {
            scopes.push(parse_scope(&body)?);
            continue;
        }
        let mut list = vec![body.parse::<Ident>()?];
        while body.peek(Token![,]) {
            body.parse::<Token![,]>()?;
            list.push(body.parse()?);
        }
        body.parse::<Token![;]>()?;
        let roles: &[Role] = match (kind.to_string().as_str(), list.len()) {
            ("binary" | "both" | "either" | "neither", 1) => &[Role::Binary],
            ("serial", 3) => &[Role::End, Role::Continuation, Role::Complete],
            ("correlative_helper", 2) => &[Role::End, Role::Continuation],
            ("both" | "either" | "neither", 2) => &[Role::Binary, Role::Complete],
            _ => {
                return Err(syn::Error::new(
                    kind.span(),
                    "expected binary (one schema), serial (end, continuation, result), correlative_helper (end, continuation), or both/either/neither (binary, optional serial result)",
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
                correlative: !matches!(kind.to_string().as_str(), "binary" | "serial"),
                kind: kind.to_string(),
            });
        }
    }
    Ok(Forms {
        name,
        templates,
        scopes,
    })
}

struct Context {
    name: Ident,
    exact: bool,
}

fn context(input: ParseStream<'_>) -> syn::Result<Context> {
    let name: Ident = input.parse()?;
    if name == "schema" {
        let inner;
        parenthesized!(inner in input);
        let name = inner.parse()?;
        if !inner.is_empty() {
            return Err(inner.error("expected one schema name"));
        }
        Ok(Context { name, exact: true })
    } else {
        Ok(Context { name, exact: false })
    }
}

fn parse_scope(input: ParseStream<'_>) -> syn::Result<Scope> {
    let mut contexts = vec![context(input)?];
    while input.peek(Token![,]) {
        input.parse::<Token![,]>()?;
        contexts.push(context(input)?);
    }
    let statements;
    braced!(statements in input);
    let mut scope = Scope {
        contexts,
        combiners: vec![],
        requirements: vec![],
    };
    while !statements.is_empty() {
        let keyword = statements.parse()?;
        scope_directive(&statements, &keyword, &mut scope)?;
    }
    Ok(scope)
}

struct Scope {
    contexts: Vec<Context>,
    combiners: Vec<(Ident, Option<Combiner>)>,
    requirements: Vec<Equation>,
}

pub(super) struct Coordinable {
    category: Ident,
    member: Ident,
    profile: Ident,
    forms: Vec<Context>,
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
    let mut using: Ident = input.parse()?;
    let member = if using == "of" {
        let member = input.parse()?;
        using = input.parse()?;
        member
    } else {
        category.clone()
    };
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
        member,
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
                result.forms.push(context(&body)?);
                while body.peek(Token![,]) {
                    body.parse::<Token![,]>()?;
                    result.forms.push(context(&body)?);
                }
                body.parse::<Token![;]>()?;
            }
            "helper" | "correlative_helper" => {
                let slot = if directive == "helper" {
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
            "on" => result.scopes.push(parse_scope(&body)?),
            "combine" => {
                return Err(syn::Error::new(
                    directive.span(),
                    "combine overrides belong in an on context",
                ));
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
        Combiner::InferredTable(name) => {
            let table = declaration
                .tables
                .iter()
                .find(|t| t.name == *name)
                .ok_or_else(|| syn::Error::new(name.span(), "unknown feature table"))?;
            let inputs: Vec<_> = table.inputs.iter().map(ToString::to_string).collect();
            let f = feature.to_string();
            let args = if inputs == [f.clone(), f.clone()] {
                vec![left, right]
            } else if inputs == ["CoordinationKind".to_owned(), f.clone(), f] {
                vec![reference_to("coordinator", &table.inputs[0]), left, right]
            } else {
                return Err(syn::Error::new(
                    name.span(),
                    "table shape needs explicit combiner arguments",
                ));
            };
            vec![Equation::ExportTable(feature.clone(), name.clone(), args)]
        }
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
            .filter(|t| t.matches(name) && (!name.exact || !t.helper()))
            .collect();
        if matching.is_empty() {
            return Err(syn::Error::new(
                name.name.span(),
                "unknown coordination form selection",
            ));
        }
        for t in matching {
            if !selected.insert(t.name.to_string()) {
                return Err(syn::Error::new(
                    name.name.span(),
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
                    context.name.span(),
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
    declaration: &Declaration,
    profile: &Forms,
) -> syn::Result<(Combinations, Vec<Equation>)> {
    let mut values: BTreeMap<String, (Ident, Option<Combiner>, usize)> = base
        .features
        .iter()
        .map(|f| {
            let combiner = f.combiner.clone().or_else(|| {
                declaration
                    .features
                    .iter()
                    .find(|d| d.name == f.name)
                    .and_then(|d| d.combiner.clone())
            });
            (f.name.to_string(), (f.name.clone(), combiner, 0))
        })
        .collect();
    if let Some((feature, _, _)) = values.values().find(|(_, combiner, _)| combiner.is_none()) {
        return Err(syn::Error::new(
            feature.span(),
            "generated coordination requires a combiner for every interface entry",
        ));
    }
    let mut requirements = vec![];
    let mut assigned = BTreeSet::new();
    for scope in profile.scopes.iter().chain(&item.scopes) {
        if !scope.contexts.is_empty() && !scope.contexts.iter().any(|c| template.matches(c)) {
            continue;
        }
        let rank = if scope.contexts.is_empty() {
            1
        } else if scope
            .contexts
            .iter()
            .any(|c| c.exact && c.name == template.name)
        {
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
        for scope in &profile.scopes {
            if !scope.combiners.is_empty() {
                return Err(syn::Error::new(
                    profile.name.span(),
                    "template contexts contain requirements, not combiner overrides",
                ));
            }
            for context in &scope.contexts {
                if !profile.templates.iter().any(|t| t.matches(context)) {
                    return Err(syn::Error::new(
                        context.name.span(),
                        "coordination context matches no template",
                    ));
                }
            }
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
        if !declaration.categories.iter().any(|c| c.name == item.member) {
            return Err(syn::Error::new(
                item.member.span(),
                "unknown coordination member category",
            ));
        }
        let profile = profiles
            .iter()
            .find(|p| p.name == item.profile)
            .ok_or_else(|| syn::Error::new(item.profile.span(), "unknown coordination_forms"))?;
        elaborate_category(declaration, schemas, item, profile, &base)?;
    }
    for category in &declaration.categories {
        if !categories.contains(&category.name.to_string())
            && category.features.iter().any(|f| f.combiner.is_some())
        {
            return Err(syn::Error::new(
                category.name.span(),
                "combine annotation requires a coordinable block",
            ));
        }
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
    let mut carried = [BTreeMap::new(), BTreeMap::new()];
    let ordinary_helper = item
        .series
        .clone()
        .unwrap_or_else(|| Ident::new(&format!("{}Series", item.category), item.category.span()));
    let correlative_helper = item.correlative_series.clone().unwrap_or_else(|| {
        Ident::new(
            &format!("Correlative{}Series", item.category),
            item.category.span(),
        )
    });
    for template in &selected {
        let schema = schemas
            .iter()
            .find(|s| s.name == template.name)
            .expect("validated schema");
        let helper = if template.correlative { &correlative_helper } else { &ordinary_helper };
        let result = if template.helper() { helper } else { &item.category };
        let mut instance = empty_instance(template.name.clone(), result.clone());
        let (combiners, requirements) = contract(item, template, base, declaration, profile)?;
        for (feature, combiner) in combiners {
            instance
                .equations
                .extend(equations(&feature, &combiner, declaration)?);
        }
        instance.equations.extend(requirements);
        for equation in &mut instance.equations {
            for r in equation.references_mut() {
                if template.tail() && r.field == "coordinator" {
                    carried[usize::from(template.correlative)]
                        .insert(r.feature.to_string(), r.feature.clone());
                }
                r.field = match r.field.to_string().as_str() {
                    "left" => r.field.clone(),
                    "right" => Ident::new(
                        if template.tail() { "rest" } else { "right" },
                        r.field.span(),
                    ),
                    "coordinator" => Ident::new(
                        if template.tail() { "rest" } else { "coordinator" },
                        r.field.span(),
                    ),
                    _ => {
                        return Err(syn::Error::new(
                            r.field.span(),
                            "combiner references use left, right or coordinator",
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
                        "left" => &item.member,
                        "right" if !template.tail() => &item.member,
                        "rest" if template.tail() => helper,
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
    carry_summaries(schemas, &carried, &mut generated, &mut helpers);
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
    schemas: &[Construction],
    carried: &[BTreeMap<String, Ident>; 2],
    generated: &mut [(&Template, Construction)],
    helpers: &mut HelperInterfaces,
) {
    // A coordinator feature read through a tail must be carried by its helper.
    for correlative in [false, true] {
        let carried = &carried[usize::from(correlative)];
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
            for (name, feature) in carried {
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
}
