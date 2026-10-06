use deckmaste_lexical_model::Frame;
use deckmaste_lexical_model::FrameItem;
use deckmaste_lexical_model::FrameSlot;
use deckmaste_lexical_model::Relation;
use syn::Ident;
use syn::LitStr;
use syn::Token;
use syn::Visibility;
use syn::braced;
use syn::bracketed;
use syn::ext::IdentExt;
use syn::parenthesized;
use syn::parse::Parse;
use syn::parse::ParseStream;

pub(crate) struct Declaration {
    pub visibility: Visibility,
    pub module: Ident,
    pub categories: Vec<Category>,
    pub features: Vec<Feature>,
    pub frames: Vec<(Ident, Frame)>,
    pub frame_categories: Vec<(Ident, Ident)>,
    pub tables: Vec<FeatureTable>,
    pub frame_marker_licences: Vec<(Ident, Ident, Ident)>,
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
    pub default: Option<Ident>,
    pub set: bool,
}

pub(crate) struct FeatureTable {
    pub name: Ident,
    pub inputs: Vec<Ident>,
    pub output: Ident,
    pub rows: Vec<(Vec<Ident>, Ident)>,
    pub operation: Option<(Ident, Option<(Ident, Ident)>)>,
}

#[derive(Clone)]
pub(crate) struct Construction {
    pub parameters: Vec<Ident>,
    pub defaults: Vec<Option<Ident>>,
    pub rows: Vec<Vec<Ident>>,
    pub shared: bool,
    pub bindings: Vec<(Ident, Ident)>,
    pub uses: Vec<PolicyUse>,
    pub cost: Option<u64>,
    pub name: Ident,
    pub category: Ident,
    pub forms: Vec<Vec<Part>>,
    pub equations: Vec<Equation>,
    pub boundary: Option<Ident>,
    pub onset: Option<Ident>,
    pub segments: Option<Segments>,
}

#[derive(Clone)]
pub(crate) enum Segments {
    Build(Ident),
    Share(Ident, Ident),
    Discharge(Ident, Ident),
}

impl Segments {
    fn fields_mut(&mut self) -> Vec<&mut Ident> {
        match self {
            Self::Build(_) => vec![],
            Self::Share(left, right) | Self::Discharge(left, right) => vec![left, right],
        }
    }
}

#[derive(Clone)]
pub(crate) struct PolicyUse {
    pub name: Ident,
    pub arguments: Vec<Ident>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum FieldType {
    Lexical(String),
    SelectedFrame(String, String),
    One(String),
    Optional(String),
    Repeated(String, String),
}

#[derive(Clone)]
pub(crate) enum Part {
    Literal(LitStr),
    Field(Ident, FieldType),
}

#[derive(Clone)]
pub(crate) struct Reference {
    pub field: Ident,
    pub feature: Ident,
}

#[derive(Clone)]
pub(crate) enum Equation {
    Export(Ident, Reference),
    ExportConstant(Ident, Ident),
    ExportTable(Ident, Ident, Vec<Reference>),
    Agree(Reference, Reference),
    Require(Reference, Ident),
    RequireTable(Ident, Vec<Reference>, Ident),
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
            frame_categories: vec![],
            tables: vec![],
            frame_marker_licences: vec![],
            constructions: vec![],
            capitalization: None,
        };
        let mut schemas = Vec::new();
        let mut policies = Vec::new();
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
                    let values = names(&values)?;
                    let set = body.peek(Ident) && body.fork().parse::<Ident>()? == "set";
                    if set {
                        body.parse::<Ident>()?;
                        body.parse::<Token![;]>()?;
                    }
                    let default = if body.peek(Ident) && body.fork().parse::<Ident>()? == "default"
                    {
                        body.parse::<Ident>()?;
                        let value = body.parse()?;
                        body.parse::<Token![;]>()?;
                        Some(value)
                    } else {
                        None
                    };
                    result.features.push(Feature {
                        name,
                        values,
                        default,
                        set,
                    });
                }
                "frame_marker_licence" => {
                    let pos;
                    parenthesized!(pos in body);
                    let pos = pos.parse()?;
                    body.parse::<Token![=]>()?;
                    result
                        .frame_marker_licences
                        .push((name, pos, body.parse()?));
                    body.parse::<Token![;]>()?;
                }
                "frame_category" => {
                    body.parse::<Token![=]>()?;
                    result.frame_categories.push((name, body.parse()?));
                    body.parse::<Token![;]>()?;
                }
                "frame" => {
                    body.parse::<Token![=]>()?;
                    let frame = parse_frame(&body)?;
                    result.frames.push((name, frame));
                    body.parse::<Token![;]>()?;
                }
                "table" => result.tables.push(parse_table(&body, name)?),
                "policy" => {
                    policies.push(parse_construction(&body, name, true, true)?);
                }
                "schema" => {
                    schemas.push(parse_construction(&body, name, true, false)?);
                }
                "instance" => {
                    let mut instance = parse_construction(&body, name, false, false)?;
                    instance.shared = true;
                    result.constructions.push(instance);
                }
                "construction" => {
                    result
                        .constructions
                        .push(parse_construction(&body, name, false, false)?);
                }
                _ => {
                    return Err(syn::Error::new(
                        keyword.span(),
                        "expected capitalization, category, feature, frame, table, construction, schema, instance or policy",
                    ));
                }
            }
        }
        result.constructions = expand_rows(
            std::mem::take(&mut result.constructions),
            &result.categories,
            &policies,
        )?;
        validate_policies(&policies)?;
        apply_policies(&mut schemas, &mut result.constructions, &policies)?;
        validate_schemas(&schemas)?;
        instantiate_schemas(&mut result.constructions, &schemas)?;
        Ok(result)
    }
}

fn parse_table(input: ParseStream<'_>, name: Ident) -> syn::Result<FeatureTable> {
    let inputs;
    parenthesized!(inputs in input);
    let inputs = names(&inputs)?;
    input.parse::<Token![->]>()?;
    let output = input.parse()?;
    if input.peek(Ident) {
        let operation: Ident = input.parse()?;
        let parameters = if operation == "contains" {
            let member = input.parse()?;
            input.parse::<Token![=>]>()?;
            Some((member, input.parse()?))
        } else {
            None
        };
        input.parse::<Token![;]>()?;
        return Ok(FeatureTable {
            name,
            inputs,
            output,
            rows: vec![],
            operation: Some((operation, parameters)),
        });
    }
    let rows;
    braced!(rows in input);
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
    Ok(FeatureTable {
        name,
        inputs,
        output,
        rows: values,
        operation: None,
    })
}

fn expand_rows(
    constructions: Vec<Construction>,
    categories: &[Category],
    policies: &[Construction],
) -> syn::Result<Vec<Construction>> {
    let mut expanded = Vec::new();
    for construction in constructions {
        if construction.parameters.is_empty() {
            expanded.push(construction);
            continue;
        }
        if !construction.shared || construction.rows.is_empty() {
            return Err(syn::Error::new(
                construction.name.span(),
                "category rows require a schema instance and at least one row",
            ));
        }
        let unique: std::collections::BTreeSet<_> = construction
            .parameters
            .iter()
            .map(ToString::to_string)
            .collect();
        if unique.len() != construction.parameters.len() {
            return Err(syn::Error::new(
                construction.name.span(),
                "duplicate category parameter",
            ));
        }
        let category_roles = row_category_roles(&construction)?;
        validate_row_defaults(&construction, &category_roles, categories, policies)?;
        for row in &construction.rows {
            expanded.push(row_instance(&construction, row, &category_roles)?);
        }
    }
    Ok(expanded)
}

fn row_category_roles(construction: &Construction) -> syn::Result<Vec<bool>> {
    let mut category_roles = Vec::new();
    for (index, parameter) in construction.parameters.iter().enumerate() {
        let category_role = index == 0
            || construction
                .bindings
                .iter()
                .any(|(_, value)| value == parameter);
        let policy_role = construction
            .uses
            .iter()
            .any(|value| value.name == *parameter);
        category_roles.push(category_role);
        if category_role == policy_role {
            return Err(syn::Error::new(
                parameter.span(),
                "row parameter must have exactly one category or policy role",
            ));
        }
    }
    Ok(category_roles)
}

fn validate_row_defaults(
    construction: &Construction,
    category_roles: &[bool],
    categories: &[Category],
    policies: &[Construction],
) -> syn::Result<()> {
    for (index, default) in construction.defaults.iter().enumerate() {
        let Some(default) = default else {
            continue;
        };
        if default == "Self" {
            if index == 0 {
                return Err(syn::Error::new(
                    default.span(),
                    "result category cannot be Self",
                ));
            }
            if !category_roles[index] {
                return Err(syn::Error::new(
                    default.span(),
                    "Self row alias requires a category column",
                ));
            }
        } else if category_roles[index] {
            if !categories.iter().any(|category| category.name == *default) {
                return Err(syn::Error::new(
                    default.span(),
                    "default row category must be declared",
                ));
            }
        } else {
            let policy = policies
                .iter()
                .find(|policy| policy.name == *default)
                .ok_or_else(|| syn::Error::new(default.span(), "unknown policy default"))?;
            if construction
                .uses
                .iter()
                .filter(|application| application.name == construction.parameters[index])
                .any(|application| application.arguments.len() != policy.parameters.len())
            {
                return Err(syn::Error::new(
                    default.span(),
                    "policy default field argument arity must match parameters",
                ));
            }
        }
    }
    Ok(())
}

fn row_instance(
    construction: &Construction,
    row: &[Ident],
    category_roles: &[bool],
) -> syn::Result<Construction> {
    let required = construction
        .defaults
        .iter()
        .position(Option::is_some)
        .unwrap_or(construction.parameters.len());
    if row.len() < required || row.len() > construction.parameters.len() {
        return Err(syn::Error::new(
            construction.name.span(),
            "category row arity must match parameters",
        ));
    }
    let mut row = row.to_vec();
    for index in row.len()..construction.parameters.len() {
        row.push(
            construction.defaults[index]
                .clone()
                .expect("validated trailing default"),
        );
    }
    if row[0] == "Self" {
        return Err(syn::Error::new(
            row[0].span(),
            "result category cannot be Self",
        ));
    }
    for index in 1..row.len() {
        if row[index] == "Self" {
            if !category_roles[index] {
                return Err(syn::Error::new(
                    row[index].span(),
                    "Self row alias requires a category column",
                ));
            }
            row[index] = row[0].clone();
        }
    }
    let mut instance = construction.clone();
    instance.category = row[0].clone();
    for (_, category) in &mut instance.bindings {
        if let Some(index) = construction
            .parameters
            .iter()
            .position(|parameter| parameter == category)
        {
            *category = row[index].clone();
        }
    }
    for policy in &mut instance.uses {
        if let Some(index) = construction
            .parameters
            .iter()
            .position(|parameter| *parameter == policy.name)
        {
            policy.name = row[index].clone();
        }
    }
    instance.parameters.clear();
    instance.defaults.clear();
    instance.rows.clear();
    Ok(instance)
}

fn validate_policies(policies: &[Construction]) -> syn::Result<()> {
    let mut policy_names = std::collections::BTreeSet::new();
    for policy in policies {
        if !policy_names.insert(policy.name.to_string()) {
            return Err(syn::Error::new(policy.name.span(), "duplicate policy"));
        }
        let unique: std::collections::BTreeSet<_> =
            policy.parameters.iter().map(ToString::to_string).collect();
        if unique.len() != policy.parameters.len() {
            return Err(syn::Error::new(
                policy.name.span(),
                "duplicate policy field parameter",
            ));
        }
        if !policy.forms.is_empty()
            || !policy.bindings.is_empty()
            || !policy.uses.is_empty()
            || policy.cost.is_some()
            || policy.boundary.is_some()
            || policy.onset.is_some()
        {
            return Err(syn::Error::new(
                policy.name.span(),
                "policy admits only feature equations",
            ));
        }
    }
    Ok(())
}

fn apply_policies(
    schemas: &mut [Construction],
    constructions: &mut [Construction],
    policies: &[Construction],
) -> syn::Result<()> {
    let schema_fields: std::collections::BTreeMap<_, _> = schemas
        .iter()
        .map(|schema| (schema.name.to_string(), declared_fields(&schema.forms)))
        .collect();
    for owner in schemas.iter_mut().chain(constructions.iter_mut()) {
        let fields = if owner.shared {
            schema_fields
                .get(&owner.name.to_string())
                .cloned()
                .unwrap_or_default()
        } else {
            declared_fields(&owner.forms)
        };
        for application in &owner.uses {
            let policy = policies
                .iter()
                .find(|policy| policy.name == application.name)
                .ok_or_else(|| syn::Error::new(application.name.span(), "unknown policy"))?;
            if policy.parameters.len() != application.arguments.len() {
                return Err(syn::Error::new(
                    application.name.span(),
                    "policy field argument arity must match parameters",
                ));
            }
            for argument in &application.arguments {
                if !fields.contains(&argument.to_string()) {
                    return Err(syn::Error::new(
                        argument.span(),
                        "policy argument must name a caller field",
                    ));
                }
            }
            let mut equations = policy.equations.clone();
            for equation in &mut equations {
                for reference in equation.references_mut() {
                    if let Some(index) = policy
                        .parameters
                        .iter()
                        .position(|parameter| *parameter == reference.field)
                    {
                        reference.field = application.arguments[index].clone();
                    }
                    if !fields.contains(&reference.field.to_string()) {
                        return Err(syn::Error::new(
                            reference.field.span(),
                            "policy reference must name a caller field",
                        ));
                    }
                }
            }
            if let Some(mut segments) = policy.segments.clone() {
                for field in segments.fields_mut() {
                    if let Some(index) = policy
                        .parameters
                        .iter()
                        .position(|parameter| parameter == field)
                    {
                        *field = application.arguments[index].clone();
                    }
                    if !fields.contains(&field.to_string()) {
                        return Err(syn::Error::new(
                            field.span(),
                            "segment relation must name a caller field",
                        ));
                    }
                }
                if owner.segments.replace(segments).is_some() {
                    return Err(syn::Error::new(
                        owner.name.span(),
                        "duplicate segment relation",
                    ));
                }
            }
            owner.equations.extend(equations);
        }
    }
    Ok(())
}

fn validate_schemas(schemas: &[Construction]) -> syn::Result<()> {
    let mut schema_names = std::collections::BTreeSet::new();
    for schema in schemas {
        if !schema_names.insert(schema.name.to_string()) {
            return Err(syn::Error::new(schema.name.span(), "duplicate schema"));
        }
        if schema
            .forms
            .iter()
            .flatten()
            .any(|part| matches!(part, Part::Field(name, _) if name == "category"))
        {
            return Err(syn::Error::new(
                schema.name.span(),
                "schema field category is reserved for result identity",
            ));
        }
        if !schema.bindings.is_empty() {
            return Err(syn::Error::new(
                schema.name.span(),
                "schema bindings belong to instances",
            ));
        }
    }
    Ok(())
}

fn instantiate_schemas(
    constructions: &mut [Construction],
    schemas: &[Construction],
) -> syn::Result<()> {
    for instance in constructions {
        if !instance.shared {
            if !instance.bindings.is_empty() {
                return Err(syn::Error::new(
                    instance.name.span(),
                    "bindings belong to schema instances",
                ));
            }
            continue;
        }
        let schema = schemas
            .iter()
            .find(|schema| schema.name == instance.name)
            .ok_or_else(|| syn::Error::new(instance.name.span(), "unknown schema"))?;
        if !instance.forms.is_empty()
            || instance.cost.is_some()
            || instance.boundary.is_some()
            || instance.onset.is_some()
        {
            return Err(syn::Error::new(
                instance.name.span(),
                "instance surface and cost are owned by schema",
            ));
        }
        let mut equations = schema.equations.clone();
        equations.append(&mut instance.equations);
        instance.equations = equations;
        instance.forms = schema.forms.clone();
        instance.cost = schema.cost;
        instance.boundary = schema.boundary.clone();
        instance.onset = schema.onset.clone();
        if let Some(segments) = &schema.segments
            && instance.segments.replace(segments.clone()).is_some()
        {
            return Err(syn::Error::new(
                instance.name.span(),
                "duplicate segment relation",
            ));
        }
        let mut used = std::collections::BTreeSet::new();
        for form in &mut instance.forms {
            for part in form {
                let Part::Field(name, ty) = part else {
                    continue;
                };
                let category = match ty {
                    FieldType::One(category)
                    | FieldType::Optional(category)
                    | FieldType::Repeated(category, _) => category,
                    FieldType::Lexical(_) | FieldType::SelectedFrame(_, _) => continue,
                };
                if category != "node" {
                    continue;
                }
                let matches: Vec<_> = instance
                    .bindings
                    .iter()
                    .filter(|(field, _)| field == name)
                    .collect();
                if matches.len() != 1 {
                    return Err(syn::Error::new(
                        name.span(),
                        "generic field needs exactly one category binding",
                    ));
                }
                *category = matches[0].1.to_string();
                used.insert(name.to_string());
            }
        }
        if instance
            .bindings
            .iter()
            .any(|(field, _)| !used.contains(&field.to_string()))
        {
            return Err(syn::Error::new(
                instance.name.span(),
                "binding must name a generic node field",
            ));
        }
    }
    Ok(())
}

fn parse_frame(input: ParseStream<'_>) -> syn::Result<Frame> {
    if input.peek(LitStr) {
        let text: LitStr = input.parse()?;
        return ron::from_str(&text.value()).map_err(|error| {
            syn::Error::new(text.span(), format!("lexical frame signature: {error}"))
        });
    }
    let kind: Ident = input.parse()?;
    let items;
    parenthesized!(items in input);
    Ok(Frame {
        kind: kind.to_string(),
        items: items
            .parse_terminated(parse_frame_item, Token![,])?
            .into_iter()
            .collect(),
    })
}

fn frame_name(input: ParseStream<'_>) -> syn::Result<String> {
    if input.peek(LitStr) {
        Ok(input.parse::<LitStr>()?.value())
    } else {
        Ok(input.parse::<Ident>()?.to_string())
    }
}

fn parse_frame_slot(input: ParseStream<'_>) -> syn::Result<FrameSlot> {
    let relation: Ident = input.parse()?;
    let category;
    parenthesized!(category in input);
    let result = frame_slot(&relation, &category)?;
    if !category.is_empty() {
        return Err(category.error("frame relation requires exactly one category"));
    }
    Ok(result)
}

fn frame_slot(relation: &Ident, category: ParseStream<'_>) -> syn::Result<FrameSlot> {
    let relation = match relation.to_string().as_str() {
        "Subject" => Relation::Subject,
        "Object" => Relation::Object,
        "Complement" => Relation::Complement,
        _ => {
            return Err(syn::Error::new(
                relation.span(),
                "expected frame relation Subject, Object or Complement",
            ));
        }
    };
    Ok(FrameSlot {
        relation,
        category: frame_name(category)?,
    })
}

fn parse_frame_item(input: ParseStream<'_>) -> syn::Result<FrameItem> {
    let kind: Ident = input.parse()?;
    let arguments;
    parenthesized!(arguments in input);
    let item = match kind.to_string().as_str() {
        "Subject" | "Object" | "Complement" => FrameItem::Argument(frame_slot(&kind, &arguments)?),
        "Optional" => FrameItem::Optional(Box::new(parse_frame_item(&arguments)?)),
        "Literal" => FrameItem::Literal(arguments.parse::<LitStr>()?.value()),
        "Marked" => {
            let vocabulary = frame_name(&arguments)?;
            arguments.parse::<Token![,]>()?;
            let member = frame_name(&arguments)?;
            arguments.parse::<Token![,]>()?;
            let slot = parse_frame_slot(&arguments)?;
            FrameItem::Marked {
                vocabulary,
                member,
                slot,
            }
        }
        "Marker" => {
            let vocabulary = frame_name(&arguments)?;
            arguments.parse::<Token![,]>()?;
            FrameItem::Marker {
                vocabulary,
                member: frame_name(&arguments)?,
            }
        }
        _ => FrameItem::Marker {
            vocabulary: kind.to_string(),
            member: frame_name(&arguments)?,
        },
    };
    if !arguments.is_empty() {
        return Err(arguments.error("unexpected frame item arguments"));
    }
    Ok(item)
}

fn parse_construction(
    body: ParseStream<'_>,
    name: Ident,
    schema: bool,
    policy: bool,
) -> syn::Result<Construction> {
    let mut parameters = Vec::new();
    let mut defaults = Vec::new();
    let mut rows = Vec::new();
    let mut bindings = Vec::new();
    if policy && body.peek(Token![<]) {
        body.parse::<Token![<]>()?;
        loop {
            parameters.push(body.parse::<Ident>()?);
            defaults.push(None);
            if body.peek(Token![>]) {
                break;
            }
            body.parse::<Token![,]>()?;
        }
        body.parse::<Token![>]>()?;
    }
    let category = if schema {
        name.clone()
    } else {
        if body.peek(Token![<]) {
            body.parse::<Token![<]>()?;
            loop {
                let parameter = body.parse::<Ident>()?;
                parameters.push(parameter.clone());
                if body.peek(Token![:]) {
                    body.parse::<Token![:]>()?;
                    let fields = if body.peek(syn::token::Paren) {
                        let fields;
                        parenthesized!(fields in body);
                        let fields = names(&fields)?;
                        if fields.is_empty() {
                            return Err(syn::Error::new(
                                parameter.span(),
                                "category binding needs a field target",
                            ));
                        }
                        fields
                    } else {
                        vec![body.parse::<Ident>()?]
                    };
                    bindings.extend(fields.into_iter().map(|field| (field, parameter.clone())));
                }
                let default = if body.peek(Token![=]) {
                    body.parse::<Token![=]>()?;
                    Some(body.call(Ident::parse_any)?)
                } else {
                    None
                };
                if default.is_none() && defaults.iter().any(Option::is_some) {
                    return Err(syn::Error::new(
                        name.span(),
                        "only trailing row parameters may have defaults",
                    ));
                }
                defaults.push(default);
                if body.peek(Token![>]) {
                    break;
                }
                body.parse::<Token![,]>()?;
            }
            body.parse::<Token![>]>()?;
        }
        body.parse::<Token![:]>()?;
        if parameters.is_empty() {
            body.parse()?
        } else {
            let row_list;
            bracketed!(row_list in body);
            while !row_list.is_empty() {
                let row;
                parenthesized!(row in row_list);
                rows.push(
                    row.parse_terminated(Ident::parse_any, Token![,])?
                        .into_iter()
                        .collect(),
                );
                if row_list.is_empty() {
                    break;
                }
                row_list.parse::<Token![,]>()?;
            }
            parameters[0].clone()
        }
    };
    let contents;
    braced!(contents in body);
    let mut construction = Construction {
        parameters,
        defaults,
        rows,
        shared: false,
        bindings,
        uses: vec![],
        cost: None,
        name,
        category,
        forms: vec![],
        equations: vec![],
        boundary: None,
        onset: None,
        segments: None,
    };
    while !contents.is_empty() {
        let directive: Ident = contents.call(Ident::parse_any)?;
        parse_directive(&contents, &directive, &mut construction)?;
        contents.parse::<Token![;]>()?;
    }
    Ok(construction)
}

fn parse_directive(
    contents: ParseStream<'_>,
    directive: &Ident,
    construction: &mut Construction,
) -> syn::Result<()> {
    match directive.to_string().as_str() {
        "segment" | "share_segments" | "discharge_segments" => {
            let left = contents.parse()?;
            let relation = if directive == "segment" {
                Segments::Build(left)
            } else {
                contents.parse::<Token![,]>()?;
                let right = contents.parse()?;
                if directive == "share_segments" {
                    Segments::Share(left, right)
                } else {
                    Segments::Discharge(left, right)
                }
            };
            if construction.segments.replace(relation).is_some() {
                return Err(syn::Error::new(
                    directive.span(),
                    "duplicate segment relation",
                ));
            }
        }
        "use" => {
            let name = contents.parse()?;
            let arguments = if contents.peek(syn::token::Paren) {
                let arguments;
                parenthesized!(arguments in contents);
                names(&arguments)?
            } else {
                Vec::new()
            };
            construction.uses.push(PolicyUse { name, arguments });
        }
        "bind" => {
            let mut fields = vec![contents.parse::<Ident>()?];
            while contents.peek(Token![,]) {
                contents.parse::<Token![,]>()?;
                fields.push(contents.parse()?);
            }
            contents.parse::<Token![=]>()?;
            let category: Ident = contents.parse()?;
            construction
                .bindings
                .extend(fields.into_iter().map(|field| (field, category.clone())));
        }
        "cost" => {
            let value: syn::LitInt = contents.parse()?;
            let value = value.base10_parse::<u64>()?;
            if construction.cost.replace(value).is_some() {
                return Err(syn::Error::new(
                    directive.span(),
                    "duplicate construction cost",
                ));
            }
        }
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
                Equation::Export(feature, reference(contents)?)
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
            let left = reference(contents)?;
            contents.parse::<Token![=]>()?;
            construction
                .equations
                .push(Equation::Agree(left, reference(contents)?));
        }
        "require" => {
            let equation = if contents.peek2(syn::token::Paren) {
                let table = contents.parse()?;
                let arguments;
                parenthesized!(arguments in contents);
                let arguments = arguments.parse_terminated(reference, Token![,])?;
                contents.parse::<Token![=]>()?;
                Equation::RequireTable(table, arguments.into_iter().collect(), contents.parse()?)
            } else {
                let left = reference(contents)?;
                contents.parse::<Token![=]>()?;
                Equation::Require(left, contents.parse()?)
            };
            construction.equations.push(equation);
        }
        _ => {
            return Err(syn::Error::new(
                directive.span(),
                format!(
                    "construction {}: unknown generated obligation; expected form, export, agree, require, boundary, onset or cost",
                    construction.name
                ),
            ));
        }
    }
    Ok(())
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
                    "selected_frame" => {
                        arguments.parse::<Token![,]>()?;
                        let kind: Ident = arguments.parse()?;
                        FieldType::SelectedFrame(category.to_string(), kind.to_string())
                    }
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

fn declared_fields(forms: &[Vec<Part>]) -> std::collections::BTreeSet<String> {
    forms
        .iter()
        .flatten()
        .filter_map(|part| match part {
            Part::Field(name, _) => Some(name.to_string()),
            Part::Literal(_) => None,
        })
        .collect()
}

impl Equation {
    fn references_mut(&mut self) -> Vec<&mut Reference> {
        match self {
            Self::Export(_, reference) | Self::Require(reference, _) => vec![reference],
            Self::ExportTable(_, _, arguments) | Self::RequireTable(_, arguments, _) => {
                arguments.iter_mut().collect()
            }
            Self::Agree(left, right) => vec![left, right],
            Self::ExportConstant(_, _) => vec![],
        }
    }
}
