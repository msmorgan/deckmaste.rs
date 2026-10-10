use std::collections::BTreeMap;
use std::collections::BTreeSet;

use deckmaste_lexical_model::Frame;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::Ident;
use syn::Visibility;

use crate::parse::Declaration;
use crate::parse::Equation;
use crate::parse::FieldType;
use crate::parse::Part;
use crate::parse::Reference;

pub(crate) struct Ir {
    pub visibility: Visibility,
    pub module: Ident,
    pub positional_capitalization: bool,
    pub features: Vec<Domain>,
    pub frames: Vec<(Ident, Frame)>,
    pub frame_categories: Vec<(Ident, usize)>,
    pub frame_category_requirements: Vec<(Ident, usize, TokenStream)>,
    pub tables: Vec<FeatureTable>,
    pub frame_marker_licences: Vec<(String, String, usize, TokenStream, TokenStream)>,
    pub categories: Vec<Ident>,
    pub public_categories: usize,
    pub constructors: Vec<Constructor>,
    pub rules: Vec<Rule>,
}

pub(crate) struct Domain {
    pub name: String,
    pub values: Vec<String>,
    pub kind: DomainKind,
}

pub(crate) enum DomainKind {
    Builtin(String),
    Countability,
    Frame,
    NumeralKind,
    NumeralSize,
    NumeralSign,
    Framing,
    Onset,
    TerminalPunctuation,
    Custom { default: Option<usize> },
    Set,
}

pub(crate) struct FeatureTable {
    pub name: Ident,
    pub inputs: Vec<usize>,
    pub output: Option<usize>,
    pub rows: Vec<(Vec<TokenStream>, TokenStream)>,
    pub operation: Option<TokenStream>,
    pub literals: Vec<String>,
}

#[derive(Clone)]
pub(crate) struct TableExport {
    pub feature: usize,
    pub table: usize,
    pub registers: Vec<usize>,
}

#[derive(Clone)]
pub(crate) struct TableRequirement {
    pub table: usize,
    pub registers: Vec<usize>,
    pub expected: TokenStream,
}

pub(crate) struct Constructor {
    pub shared: bool,
    pub cost: u64,
    pub name: Ident,
    pub category: usize,
    pub fields: Vec<(Ident, FieldType)>,
    pub forms: Vec<Form>,
}

pub(crate) struct Form {
    pub index: usize,
    pub category: usize,
    pub fields: Vec<(Ident, FieldType)>,
    pub pieces: Vec<Piece>,
    pub rule: usize,
    pub rules: Vec<usize>,
    pub repeat_features: Vec<Vec<(usize, TokenStream)>>,
}

#[derive(Clone, PartialEq, Eq)]
pub(crate) enum Piece {
    Literal(String),
    Field(usize),
    SelectedLiteral {
        table: usize,
        arguments: Vec<(usize, usize)>,
    },
}

pub(crate) struct Rule {
    pub category: usize,
    pub symbols: Vec<Symbol>,
    pub checks: Vec<Vec<(usize, usize)>>,
    pub initial: Vec<Option<TokenStream>>,
    pub exports: Vec<(usize, usize)>,
    pub table_exports: Vec<TableExport>,
    pub table_requirements: Vec<TableRequirement>,
    pub release: Vec<Vec<usize>>,
    pub segments: Option<Segments>,
    pub build: Build,
    pub boundary: Option<Ident>,
    pub onset: Option<Ident>,
}

pub(crate) enum Segments {
    Build(String),
    Share(usize, usize),
    Discharge(usize, usize),
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Symbol {
    Literal(String),
    Lexical(String),
    Nonterminal(usize),
}

pub(crate) enum Build {
    Construction {
        constructor: usize,
        form: usize,
        slots: Vec<usize>,
    },
    Absent,
    Present,
    Empty,
    Singleton,
    Append,
    Sequence,
}

fn builtin_domains() -> Vec<Domain> {
    [
        ("number", "Number", &["Singular", "Plural"][..]),
        ("person", "Person", &["First", "Second", "Third"][..]),
        ("tense", "Tense", &["Present", "Past"][..]),
        ("finiteness", "Finiteness", &["Finite", "Nonfinite"][..]),
        (
            "case",
            "Case",
            &["Nominative", "Accusative", "Genitive"][..],
        ),
        (
            "form",
            "WordForm",
            &[
                "Invariant",
                "Singular",
                "Plural",
                "Plain",
                "Present",
                "Preterite",
                "Irrealis",
                "GerundParticiple",
                "PastParticiple",
            ][..],
        ),
    ]
    .into_iter()
    .map(|(name, ty, values)| Domain {
        name: name.into(),
        kind: DomainKind::Builtin(ty.into()),
        values: values.iter().map(|v| (*v).into()).collect(),
    })
    .chain([
        Domain {
            name: "countability".into(),
            values: vec!["Count".into(), "Mass".into()],
            kind: DomainKind::Countability,
        },
        Domain {
            name: "frame".into(),
            values: vec![],
            kind: DomainKind::Frame,
        },
        Domain {
            name: "numeral_kind".into(),
            values: ["Cardinal", "Ordinal", "Arabic", "GroupedArabic", "Roman"]
                .map(str::to_owned)
                .into(),
            kind: DomainKind::NumeralKind,
        },
        Domain {
            name: "numeral_size".into(),
            values: vec!["Small".into(), "Large".into()],
            kind: DomainKind::NumeralSize,
        },
        Domain {
            name: "framing".into(),
            values: vec!["Unframed".into(), "Framed".into()],
            kind: DomainKind::Framing,
        },
        Domain {
            name: "numeral_sign".into(),
            values: vec!["Nonnegative".into(), "Negative".into()],
            kind: DomainKind::NumeralSign,
        },
        Domain {
            name: "onset".into(),
            values: vec!["Consonant".into(), "Vowel".into()],
            kind: DomainKind::Onset,
        },
        Domain {
            name: "article_onset".into(),
            values: vec!["Consonant".into(), "Vowel".into()],
            kind: DomainKind::Onset,
        },
        Domain {
            name: "terminal_punctuation".into(),
            values: vec!["None".into(), "FullStop".into(), "QuotedFullStop".into()],
            kind: DomainKind::TerminalPunctuation,
        },
    ])
    .collect()
}

fn error(name: &Ident, obligation: &str) -> syn::Error {
    syn::Error::new(name.span(), format!("declaration {name}: {obligation}"))
}

fn reserve(names: &mut BTreeSet<String>, name: &Ident) -> syn::Result<()> {
    if name.to_string().starts_with("__") || !names.insert(name.to_string()) {
        return Err(error(
            name,
            "unique generated identity required (the __ prefix is reserved)",
        ));
    }
    Ok(())
}

fn validate_features(declared: &[crate::parse::Feature]) -> syn::Result<Vec<Domain>> {
    let mut features = builtin_domains();
    let mut feature_names: BTreeSet<_> = features.iter().map(|f| f.name.clone()).collect();
    for feature in declared {
        reserve(&mut feature_names, &feature.name)?;
        if feature.values.is_empty() {
            return Err(error(
                &feature.name,
                "a feature requires a finite nonempty domain",
            ));
        }
        let mut values = BTreeSet::new();
        for value in &feature.values {
            reserve(&mut values, value)?;
        }
        let default = feature
            .default
            .as_ref()
            .map(|default| {
                feature
                    .values
                    .iter()
                    .position(|value| value == default)
                    .ok_or_else(|| {
                        error(default, &format!("not a value of feature {}", feature.name))
                    })
            })
            .transpose()?;
        features.push(Domain {
            name: feature.name.to_string(),
            values: feature.values.iter().map(ToString::to_string).collect(),
            kind: if feature.set {
                if feature.values.len() > usize::BITS as usize || default.is_some() {
                    return Err(error(
                        &feature.name,
                        "set features require at most usize::BITS members and no scalar default",
                    ));
                }
                DomainKind::Set
            } else {
                DomainKind::Custom { default }
            },
        });
    }
    Ok(features)
}

fn validate_frames(declared: &[(Ident, Frame)]) -> syn::Result<Vec<(Ident, Frame)>> {
    let mut frames = vec![];
    let mut frame_names = BTreeSet::new();
    for (name, frame) in declared {
        reserve(&mut frame_names, name)?;
        if frames.iter().any(|(_, previous)| previous == frame) {
            return Err(error(
                name,
                "duplicate frame signature would introduce a false grammatical choice",
            ));
        }
        frames.push((name.clone(), frame.clone()));
    }
    Ok(frames)
}

fn validate_frame_category_requirements(
    ir: &mut Ir,
    interfaces: &[BTreeSet<usize>],
    requirements: Vec<(Ident, Ident, Ident)>,
) -> syn::Result<()> {
    let mut constrained = BTreeSet::new();
    for (alias, feature, expected) in requirements {
        let target = ir
            .frame_categories
            .iter()
            .find(|(name, _)| name == &alias)
            .unwrap()
            .1;
        let index = feature_index(&ir.features, &feature)?;
        if !interfaces[target].contains(&index) {
            return Err(error(
                &feature,
                "frame category requirement needs an exported Category feature",
            ));
        }
        if !constrained.insert((alias.to_string(), index)) {
            return Err(error(&feature, "duplicate frame category requirement"));
        }
        let expected = value(ir, index, &expected)?;
        ir.frame_category_requirements
            .push((alias, index, expected));
    }
    Ok(())
}

pub(crate) fn validate(declaration: Declaration) -> syn::Result<Ir> {
    let mut features = validate_features(&declaration.features)?;
    let frames = validate_frames(&declaration.frames)?;
    features[7].values = frames.iter().map(|(name, _)| name.to_string()).collect();
    let mut names = BTreeSet::new();
    let mut categories = vec![];
    let mut interfaces = vec![];
    for category in &declaration.categories {
        reserve(&mut names, &category.name)?;
        let mut interface = BTreeSet::new();
        for name in &category.features {
            let index = feature_index(&features, name)?;
            if !interface.insert(index) {
                return Err(error(name, "duplicate Category feature"));
            }
        }
        categories.push(category.name.clone());
        interfaces.push(interface);
    }
    if categories.is_empty() {
        return Err(error(
            &declaration.module,
            "at least one Category is required",
        ));
    }
    if declaration.constructions.is_empty() {
        return Err(error(
            &declaration.module,
            "at least one Construction is required",
        ));
    }
    let mut frame_categories = Vec::new();
    let mut source_categories = BTreeSet::new();
    for (source, target) in &declaration.frame_categories {
        reserve(&mut source_categories, source)?;
        let target = categories
            .iter()
            .position(|name| name == target)
            .ok_or_else(|| error(target, "unknown frame category projection"))?;
        frame_categories.push((source.clone(), target));
    }
    let public_categories = categories.len();
    let category_index = |name: &str| categories.iter().position(|n| n == name);
    let mut constructors = vec![];
    names.clear();
    let mut shared_categories = BTreeSet::new();
    for construction in &declaration.constructions {
        if construction.shared {
            if declaration
                .constructions
                .iter()
                .any(|other| other.name == construction.name && !other.shared)
            {
                return Err(error(
                    &construction.name,
                    "schema conflicts with plain construction",
                ));
            }
            if !shared_categories.insert((
                construction.name.to_string(),
                construction.category.to_string(),
            )) {
                return Err(error(
                    &construction.name,
                    "schema requires unique result category per instance",
                ));
            }
        } else {
            reserve(&mut names, &construction.name)?;
        }
        let category = category_index(&construction.category.to_string())
            .ok_or_else(|| error(&construction.name, "unknown result Category"))?;
        let Some(first) = construction.forms.first() else {
            return Err(error(
                &construction.name,
                "realization requires at least one form",
            ));
        };
        let fields = collect_fields(first, &construction.name)?;
        for form in &construction.forms {
            let other = collect_fields(form, &construction.name)?;
            if fields.len() != other.len()
                || fields.iter().any(|(name, ty)| {
                    !other
                        .iter()
                        .any(|(n, t)| name == n && same_field_storage(ty, t))
                })
            {
                return Err(error(
                    &construction.name,
                    "parse/materialize/realize obligation: every form must retain the same named fields and cardinalities",
                ));
            }
        }
        for form in &construction.forms {
            for (name, ty) in collect_fields(form, &construction.name)? {
                match &ty {
                    FieldType::SelectedFrame(_, _) => {}
                    FieldType::Lexical(category) => {
                        ron::from_str::<deckmaste_lexical_model::Category>(category)
                            .map_err(|_| error(&name, "unknown lexical Category"))?;
                    }
                    FieldType::One(category)
                    | FieldType::Optional(category)
                    | FieldType::Repeated(category, _) => {
                        if category_index(category).is_none() {
                            return Err(error(&name, "unknown constituent Category"));
                        }
                    }
                }
            }
        }
        constructors.push(Constructor {
            shared: construction.shared,
            cost: construction.cost.unwrap_or(1),
            name: construction.name.clone(),
            category,
            fields,
            forms: vec![],
        });
    }
    let mut ir = Ir {
        visibility: declaration.visibility,
        module: declaration.module,
        positional_capitalization: declaration.capitalization.is_some(),
        features,
        frames,
        frame_categories,
        frame_category_requirements: vec![],
        tables: vec![],
        frame_marker_licences: vec![],
        categories,
        public_categories,
        constructors,
        rules: vec![],
    };
    validate_frame_category_requirements(
        &mut ir,
        &interfaces,
        declaration.frame_category_requirements,
    )?;
    validate_tables(&mut ir, declaration.tables)?;
    validate_marker_licences(&mut ir, declaration.frame_marker_licences)?;
    normalize(&mut ir, &declaration.constructions, &interfaces)?;
    check_recursion(&ir)?;
    merge_schemas(&mut ir)?;
    Ok(ir)
}

fn validate_marker_licences(ir: &mut Ir, licences: Vec<(Ident, Ident, Ident)>) -> syn::Result<()> {
    for (kind, pos, table_name) in licences {
        let table = ir
            .tables
            .iter()
            .find(|table| table.name == table_name)
            .ok_or_else(|| error(&table_name, "unknown frame marker licence table"))?;
        if table.inputs.len() != 1 {
            return Err(error(&table_name, "frame marker licence must be unary"));
        }
        let operation = table.operation.clone().ok_or_else(|| {
            error(
                &table_name,
                "frame marker licence requires a set containment table",
            )
        })?;
        let expected = value(
            ir,
            table.output.ok_or_else(|| {
                error(&table_name, "frame marker licence requires a feature table")
            })?,
            &Ident::new("Yes", table_name.span()),
        )?;
        ir.frame_marker_licences.push((
            kind.to_string(),
            pos.to_string(),
            table.inputs[0],
            operation,
            expected,
        ));
    }
    Ok(())
}

fn validate_tables(ir: &mut Ir, tables: Vec<crate::parse::FeatureTable>) -> syn::Result<()> {
    let mut table_names = BTreeSet::new();
    for table in tables {
        reserve(&mut table_names, &table.name)?;
        if table.inputs.is_empty() || (table.rows.is_empty() && table.operation.is_none()) {
            return Err(error(
                &table.name,
                "a feature table requires inputs and rows",
            ));
        }
        let inputs: Vec<_> = table
            .inputs
            .iter()
            .map(|name| feature_index(&ir.features, name))
            .collect::<syn::Result<_>>()?;
        let output = table
            .output
            .as_ref()
            .map(|name| feature_index(&ir.features, name))
            .transpose()?;
        let operation = if let Some((name, parameters)) = table.operation {
            let feature = *inputs
                .first()
                .ok_or_else(|| error(&name, "set operation requires an input feature"))?;
            if !matches!(ir.features[feature].kind, DomainKind::Set) {
                return Err(error(&name, "set operation requires a set feature"));
            }
            match (name.to_string().as_str(), parameters) {
                ("contains", Some((member, result))) if inputs.len() == 1 => {
                    let index = ir.features[feature]
                        .values
                        .iter()
                        .position(|value| value == &member.to_string())
                        .ok_or_else(|| error(&member, "unknown set member"))?;
                    let mask = 1usize << index;
                    let result = value(ir, output.unwrap(), &result)?;
                    Some(quote!(TableOperation::Contains(#feature, #mask, #result)))
                }
                ("intersection", None)
                    if inputs.len() == 2 && inputs[1] == feature && output == Some(feature) =>
                {
                    Some(quote!(TableOperation::Intersection(#feature)))
                }
                _ => return Err(error(&name, "invalid set table operation or signature")),
            }
        } else {
            None
        };
        let mut seen = BTreeSet::new();
        let mut rows = vec![];
        let mut literals = vec![];
        for (arguments, result) in table.rows {
            if arguments.len() != inputs.len() {
                return Err(error(&table.name, "feature table row has wrong arity"));
            }
            if !seen.insert(
                arguments
                    .iter()
                    .map(|pattern| match pattern {
                        crate::parse::TablePattern::Exact(name) => name.to_string(),
                        crate::parse::TablePattern::Contains(name) => format!("contains {name}"),
                        crate::parse::TablePattern::Any => "_".into(),
                    })
                    .collect::<Vec<_>>(),
            ) {
                return Err(error(&table.name, "duplicate feature table input tuple"));
            }
            let arguments = inputs
                .iter()
                .zip(arguments.iter())
                .map(|(&domain, pattern)| match pattern {
                    crate::parse::TablePattern::Exact(name) => {
                        let value = value(ir, domain, name)?;
                        Ok(quote!(FeaturePattern::Exact(#value)))
                    }
                    crate::parse::TablePattern::Contains(name) => {
                        if !matches!(ir.features[domain].kind, DomainKind::Set) {
                            return Err(error(name, "membership pattern requires a set feature"));
                        }
                        let value = value(ir, domain, name)?;
                        Ok(quote!(FeaturePattern::Contains(#value)))
                    }
                    crate::parse::TablePattern::Any => Ok(quote!(FeaturePattern::Any)),
                })
                .collect::<syn::Result<_>>()?;
            let result = match result {
                crate::parse::TableResult::Feature(result) => value(ir, output.unwrap(), &result)?,
                crate::parse::TableResult::Literal(result) => {
                    let text = result.value();
                    literals.push(text.clone());
                    quote!(FeatureValue::SelectedLiteral(#text))
                }
            };
            rows.push((arguments, result));
        }
        ir.tables.push(FeatureTable {
            name: table.name,
            inputs,
            output,
            rows,
            operation,
            literals,
        });
    }
    Ok(())
}

fn same_field_storage(left: &FieldType, right: &FieldType) -> bool {
    left == right
        || matches!(
            (left, right),
            (FieldType::One(_), FieldType::One(_)) | (FieldType::Lexical(_), FieldType::Lexical(_))
        )
}

fn normalize(
    ir: &mut Ir,
    constructions: &[crate::parse::Construction],
    interfaces: &[BTreeSet<usize>],
) -> syn::Result<()> {
    for (index, construction) in constructions.iter().enumerate() {
        if let Some(boundary) = &construction.boundary
            && boundary != "Initial"
            && boundary != "Interior"
        {
            return Err(error(
                boundary,
                "surface boundary must be Initial or Interior",
            ));
        }
        if let Some(onset) = &construction.onset
            && onset != "Consonant"
            && onset != "Vowel"
        {
            return Err(error(onset, "surface onset must be Consonant or Vowel"));
        }
        let repeat_features = repeated_projections(ir, index, interfaces, construction)?;
        let field_symbols = normalize_fields(ir, index, &repeat_features);
        let mut canonical_requirements = vec![];
        for (parts, requirements) in construction
            .forms
            .iter()
            .zip(&construction.form_requirements)
        {
            // Field names and storage cardinalities belong to the Construction;
            // child Categories and reachable features belong to its selected
            // form.
            let form_fields = collect_fields(parts, &construction.name)?;
            let fields = ir.constructors[index]
                .fields
                .iter()
                .map(|(name, _)| {
                    form_fields
                        .iter()
                        .find(|(candidate, _)| candidate == name)
                        .unwrap()
                        .clone()
                })
                .collect();
            let constructor_fields = std::mem::replace(&mut ir.constructors[index].fields, fields);
            let mut obligations = construction.equations.clone();
            obligations.extend(requirements.iter().cloned());
            validate_selected_frame_surface(ir, interfaces, &construction.name, parts)?;
            let pieces = normalize_form(ir, index, interfaces, parts)?;
            let canonical = ir.constructors[index].forms.iter().position(|f| {
                f.pieces == pieces
                    && f.fields == ir.constructors[index].fields
                    && canonical_requirements[f.index] == *requirements
            });
            let form_index = canonical.unwrap_or(ir.constructors[index].forms.len());
            let mut form_rules = vec![];
            for (expanded, selected_requirements) in expand_selected_literals(ir, parts)? {
                let mut row_obligations = obligations.clone();
                row_obligations.extend(selected_requirements);
                let plan = equations(ir, index, interfaces, &row_obligations)?;
                let scan_pieces = normalize_form(ir, index, interfaces, &expanded)?;
                let mut symbols = vec![];
                let mut slots = vec![0; ir.constructors[index].fields.len()];
                let mut checks = vec![];
                for piece in &scan_pieces {
                    match piece {
                        Piece::Literal(text) => {
                            symbols.push(Symbol::Literal(text.clone()));
                            checks.push(vec![]);
                        }
                        Piece::SelectedLiteral { .. } => unreachable!(),
                        Piece::Field(field) => {
                            slots[*field] = symbols.len();
                            // Optional/repeated projections remain shared
                            // across forms.
                            // Single constituent and lexical Categories belong
                            // to the selected form.
                            symbols.push(match &ir.constructors[index].fields[*field].1 {
                                FieldType::Lexical(category) => Symbol::Lexical(category.clone()),
                                FieldType::One(category) => Symbol::Nonterminal(
                                    ir.categories
                                        .iter()
                                        .position(|name| name == category)
                                        .unwrap(),
                                ),
                                _ => match &field_symbols[*field] {
                                    Symbol::Lexical(c) => Symbol::Lexical(c.clone()),
                                    Symbol::Nonterminal(c) => Symbol::Nonterminal(*c),
                                    Symbol::Literal(_) => unreachable!(),
                                },
                            });
                            checks.push(
                                plan.references
                                    .iter()
                                    .filter(|((f, _), _)| f == field)
                                    .map(|((_, feature), reg)| (*feature, *reg))
                                    .collect(),
                            );
                        }
                    }
                }
                let mut release = vec![vec![]; symbols.len()];
                for register in 0..plan.initial.len() {
                    if !plan.exports.iter().any(|(_, r)| *r == register)
                        && !plan
                            .table_exports
                            .iter()
                            .any(|export| export.registers.contains(&register))
                        && !plan
                            .table_requirements
                            .iter()
                            .any(|requirement| requirement.registers.contains(&register))
                        && let Some(last) = checks
                            .iter()
                            .rposition(|c| c.iter().any(|(_, r)| *r == register))
                    {
                        release[last].push(register);
                    }
                }
                let segments = segment_relation(construction, &ir.constructors[index], &slots)?;
                let rule = ir.rules.len();
                ir.rules.push(Rule {
                    category: ir.constructors[index].category,
                    symbols,
                    checks,
                    initial: plan.initial.clone(),
                    boundary: construction.boundary.clone(),
                    onset: construction.onset.clone(),
                    exports: plan.exports.clone(),
                    table_exports: plan.table_exports.clone(),
                    table_requirements: plan.table_requirements.clone(),
                    release,
                    segments,
                    build: Build::Construction {
                        constructor: index,
                        form: form_index,
                        slots,
                    },
                });
                form_rules.push(rule);
            }
            if canonical.is_none() {
                canonical_requirements.push(requirements.clone());
                let category = ir.constructors[index].category;
                let fields = ir.constructors[index].fields.clone();
                ir.constructors[index].forms.push(Form {
                    pieces,
                    rule: form_rules[0],
                    rules: form_rules,
                    index: form_index,
                    category,
                    fields,
                    repeat_features: repeat_features.clone(),
                });
            }
            ir.constructors[index].fields = constructor_fields;
        }
    }
    Ok(())
}

fn validate_selected_frame_surface(
    ir: &Ir,
    interfaces: &[BTreeSet<usize>],
    owner: &Ident,
    parts: &[Part],
) -> syn::Result<()> {
    if parts.iter().any(|p| {
        matches!(
            p,
            crate::parse::Part::Field(_, FieldType::SelectedFrame(_, _))
        )
    }) {
        let valid = match parts {
            [
                Part::Field(head, selector),
                Part::Field(_, FieldType::SelectedFrame(selected, _)),
            ] if head == selected => match selector {
                FieldType::Lexical(category) => category == "Verb",
                FieldType::One(category) => ir
                    .categories
                    .iter()
                    .position(|name| name == category)
                    .is_some_and(|index| interfaces[index].contains(&7)),
                _ => false,
            },
            _ => false,
        };
        if !valid {
            return Err(error(
                owner,
                "selected_frame requires exactly a lexical Verb or node exporting frame, followed by selected_frame(head)",
            ));
        }
    }
    Ok(())
}

fn segment_relation(
    construction: &crate::parse::Construction,
    constructor: &Constructor,
    slots: &[usize],
) -> syn::Result<Option<Segments>> {
    let segment_field = |name: &Ident, lexical: bool| -> syn::Result<usize> {
        let field = constructor
            .fields
            .iter()
            .position(|(candidate, _)| candidate == name)
            .ok_or_else(|| error(name, "segment relation must name a field"))?;
        let valid = if lexical {
            matches!(&constructor.fields[field].1, FieldType::Lexical(category) if category == "Verb")
        } else {
            matches!(&constructor.fields[field].1, FieldType::One(_))
        };
        if !valid {
            return Err(error(
                name,
                "segment relation requires a lexical Verb head and single constituent fields",
            ));
        }
        Ok(slots[field])
    };
    let segments = match &construction.segments {
        Some(crate::parse::Segments::Build(kind)) => {
            if constructor
                .fields
                .iter()
                .any(|(_, ty)| !matches!(ty, FieldType::One(_) | FieldType::Lexical(_)))
            {
                return Err(error(
                    kind,
                    "segment fields must be single arguments or lexical markers",
                ));
            }
            Some(Segments::Build(kind.to_string()))
        }
        Some(crate::parse::Segments::Share(left, right)) => Some(Segments::Share(
            segment_field(left, false)?,
            segment_field(right, false)?,
        )),
        Some(crate::parse::Segments::Discharge(head, tail)) => Some(Segments::Discharge(
            segment_field(head, true)?,
            segment_field(tail, false)?,
        )),
        None => None,
    };
    if matches!(segments, Some(Segments::Share(left, right) | Segments::Discharge(left, right)) if left >= right)
    {
        return Err(error(
            &construction.name,
            "segment fields must be distinct and ordered",
        ));
    }
    Ok(segments)
}

fn collect_fields(parts: &[Part], owner: &Ident) -> syn::Result<Vec<(Ident, FieldType)>> {
    let mut result = vec![];
    let mut names = BTreeSet::from(["form".into()]);
    for part in parts {
        if let Part::Field(name, ty) = part {
            if !names.insert(name.to_string()) {
                return Err(error(
                    owner,
                    "each lexical/structural field must occur exactly once; `form` is reserved",
                ));
            }
            result.push((name.clone(), ty.clone()));
        }
    }
    Ok(result)
}

fn expand_selected_literals(
    ir: &Ir,
    parts: &[Part],
) -> syn::Result<Vec<(Vec<Part>, Vec<Equation>)>> {
    let mut alternatives = vec![(vec![], vec![])];
    for part in parts {
        if let Part::SelectedLiteral(name, arguments) = part {
            let table = ir
                .tables
                .iter()
                .find(|table| table.name == *name)
                .ok_or_else(|| error(name, "unknown separator table"))?;
            if table.output.is_some() {
                return Err(error(name, "selected literal requires a separator table"));
            }
            let texts: BTreeSet<_> = table.literals.iter().collect();
            alternatives = alternatives
                .into_iter()
                .flat_map(|(pieces, requirements)| {
                    texts.iter().map(move |text| {
                        let mut pieces = pieces.clone();
                        pieces.push(Part::Literal(syn::LitStr::new(text, name.span())));
                        let mut requirements = requirements.clone();
                        requirements.push(Equation::RequireSelectedLiteral(
                            name.clone(),
                            arguments.clone(),
                            (*text).clone(),
                        ));
                        (pieces, requirements)
                    })
                })
                .collect();
        } else {
            for (pieces, _) in &mut alternatives {
                pieces.push(part.clone());
            }
        }
    }
    Ok(alternatives)
}

fn normalize_form(
    ir: &Ir,
    constructor: usize,
    interfaces: &[BTreeSet<usize>],
    parts: &[Part],
) -> syn::Result<Vec<Piece>> {
    let construction = &ir.constructors[constructor];
    let fields = &construction.fields;
    let mut pieces = vec![];
    for part in parts {
        match part {
            Part::Literal(text) if !text.value().is_empty() => {
                if let Some(Piece::Literal(previous)) = pieces.last_mut() {
                    previous.push_str(&text.value());
                } else {
                    pieces.push(Piece::Literal(text.value()));
                }
            }
            Part::Literal(_) => {}
            Part::Field(name, _) => pieces.push(Piece::Field(
                fields.iter().position(|(n, _)| n == name).unwrap(),
            )),
            Part::SelectedLiteral(name, arguments) => {
                let table = ir
                    .tables
                    .iter()
                    .position(|table| table.name == *name)
                    .ok_or_else(|| error(name, "unknown separator table"))?;
                let arguments = arguments
                    .iter()
                    .map(|reference| resolve_reference(ir, construction, interfaces, reference))
                    .collect::<syn::Result<Vec<_>>>()?;
                pieces.push(Piece::SelectedLiteral { table, arguments });
            }
        }
    }
    Ok(pieces)
}

fn feature_index(features: &[Domain], name: &Ident) -> syn::Result<usize> {
    features
        .iter()
        .position(|f| *name == f.name)
        .ok_or_else(|| error(name, "unknown feature in admission obligation"))
}

fn value(ir: &Ir, feature: usize, name: &Ident) -> syn::Result<TokenStream> {
    let domain = &ir.features[feature];
    let index = domain
        .values
        .iter()
        .position(|v| v == &name.to_string())
        .ok_or_else(|| error(name, &format!("not a value of feature {}", domain.name)))?;
    Ok(match &domain.kind {
        DomainKind::Builtin(ty) => {
            let ty = format_ident!("{ty}");
            quote!(FeatureValue::#ty(::deckmaste_lexical::#ty::#name))
        }
        DomainKind::Countability => {
            let is_count = index == 0;
            quote!(FeatureValue::Countability(#is_count))
        }
        DomainKind::Frame => quote!(FeatureValue::Frame(#index)),
        DomainKind::NumeralKind => quote!(FeatureValue::NumeralKind(#index)),
        DomainKind::NumeralSize => {
            let large = index == 1;
            quote!(FeatureValue::NumeralSize(#large))
        }
        DomainKind::NumeralSign => {
            let negative = index == 1;
            quote!(FeatureValue::NumeralSign(#negative))
        }
        DomainKind::Framing => {
            let framed = index == 1;
            quote!(FeatureValue::Framing(#framed))
        }
        DomainKind::Onset => {
            let value = format_ident!("{}", domain.values[index]);
            quote!(FeatureValue::Onset(::deckmaste_lexical::Onset::#value))
        }
        DomainKind::TerminalPunctuation => {
            quote!(FeatureValue::TerminalPunctuation(TerminalPunctuation::#name))
        }
        DomainKind::Set => {
            let mask = 1usize << index;
            quote!(FeatureValue::Set(#feature, #mask))
        }
        DomainKind::Custom { .. } => quote!(FeatureValue::Custom(#feature, #index)),
    })
}

struct EquationPlan {
    references: BTreeMap<(usize, usize), usize>,
    initial: Vec<Option<TokenStream>>,
    exports: Vec<(usize, usize)>,
    table_exports: Vec<TableExport>,
    table_requirements: Vec<TableRequirement>,
}

fn resolve_reference(
    ir: &Ir,
    construction: &Constructor,
    interfaces: &[BTreeSet<usize>],
    reference: &Reference,
) -> syn::Result<(usize, usize)> {
    let field = construction
        .fields
        .iter()
        .position(|(name, _)| name == &reference.field)
        .ok_or_else(|| {
            error(
                &construction.name,
                &format!("admission cannot reach field {}", reference.field),
            )
        })?;
    let feature = feature_index(&ir.features, &reference.feature)?;
    if reference.last && !matches!(construction.fields[field].1, FieldType::Repeated(_, _)) {
        return Err(error(
            &reference.field,
            "last requires a repeated constituent field",
        ));
    }
    match &construction.fields[field].1 {
        FieldType::Lexical(_) => {}
        FieldType::One(category) | FieldType::Repeated(category, _)
            if reference.last || matches!(construction.fields[field].1, FieldType::One(_)) =>
        {
            let category = ir
                .categories
                .iter()
                .position(|name| name == category)
                .unwrap();
            if !interfaces[category].contains(&feature)
                && !matches!(
                    ir.features[feature].name.as_str(),
                    "onset" | "terminal_punctuation"
                )
            {
                return Err(error(
                    &construction.name,
                    &format!(
                        "admission cannot reach {}.{} through its Category interface",
                        reference.field, reference.feature
                    ),
                ));
            }
            if reference.last
                && !matches!(
                    ir.features[feature].kind,
                    DomainKind::Custom { default: Some(_) }
                )
            {
                return Err(error(
                    &reference.feature,
                    "last requires a declared scalar feature default for the empty repeat",
                ));
            }
        }
        _ => {
            return Err(error(
                &construction.name,
                "optional/repeated fields have no single feature value; constrain their element Construction",
            ));
        }
    }
    Ok((field, feature))
}

fn same_domain(ir: &Ir, left: usize, right: usize) -> bool {
    left == right
        || matches!(
            (&ir.features[left].kind, &ir.features[right].kind),
            (DomainKind::Onset, DomainKind::Onset)
        )
}

fn equation_references(
    equations: &[Equation],
    resolve: impl Fn(&Reference) -> syn::Result<(usize, usize)>,
) -> syn::Result<Vec<(usize, usize)>> {
    let mut refs = BTreeSet::new();
    for equation in equations {
        match equation {
            Equation::Export(_, r) | Equation::Require(r, _) => {
                refs.insert(resolve(r)?);
            }
            Equation::Agree(l, r) => {
                refs.insert(resolve(l)?);
                refs.insert(resolve(r)?);
            }
            Equation::ExportConstant(_, _) => {}
            Equation::ExportTable(_, _, arguments)
            | Equation::RequireTable(_, arguments, _)
            | Equation::RequireSelectedLiteral(_, arguments, _) => {
                for argument in arguments {
                    refs.insert(resolve(argument)?);
                }
            }
        }
    }
    Ok(refs.into_iter().collect())
}

fn agreement_groups(
    ir: &Ir,
    construction: &Ident,
    refs: &[(usize, usize)],
    equations: &[Equation],
    position: impl Fn(&Reference) -> syn::Result<usize>,
) -> syn::Result<Vec<usize>> {
    let mut groups: Vec<_> = (0..refs.len()).collect();
    for equation in equations {
        if let Equation::Agree(l, r) = equation {
            let left = position(l)?;
            let right = position(r)?;
            if !same_domain(ir, refs[left].1, refs[right].1) {
                return Err(error(
                    construction,
                    "agreement requires the same feature domain",
                ));
            }
            let from = groups[right];
            let to = groups[left];
            for group in &mut groups {
                if *group == from {
                    *group = to;
                }
            }
        }
    }
    Ok(groups)
}

fn equations(
    ir: &Ir,
    constructor: usize,
    interfaces: &[BTreeSet<usize>],
    equations: &[Equation],
) -> syn::Result<EquationPlan> {
    let construction = &ir.constructors[constructor];
    let resolve =
        |reference: &Reference| resolve_reference(ir, construction, interfaces, reference);
    let refs = equation_references(equations, resolve)?;
    let position = |r: &Reference| -> syn::Result<usize> {
        let resolved = resolve(r)?;
        refs.iter()
            .position(|v| *v == resolved)
            .ok_or_else(|| error(&construction.name, "unresolved admission reference"))
    };
    let groups = agreement_groups(ir, &construction.name, &refs, equations, position)?;
    let mut plan = EquationPlan {
        references: refs.iter().copied().zip(groups.iter().copied()).collect(),
        initial: vec![None; refs.len()],
        exports: vec![],
        table_exports: vec![],
        table_requirements: vec![],
    };
    let table_call = |table_name: &Ident, arguments: &[Reference]| -> syn::Result<_> {
        let table = ir
            .tables
            .iter()
            .position(|table| table.name == *table_name)
            .ok_or_else(|| error(table_name, "unknown feature table"))?;
        let declaration = &ir.tables[table];
        if arguments.len() != declaration.inputs.len() {
            return Err(error(table_name, "feature table call has wrong arity"));
        }
        let mut registers = vec![];
        for (argument, domain) in arguments.iter().zip(&declaration.inputs) {
            let source = position(argument)?;
            if refs[source].1 != *domain {
                return Err(error(table_name, "feature table argument has wrong domain"));
            }
            registers.push(groups[source]);
        }
        Ok((table, registers))
    };
    let mut exported = BTreeSet::new();
    for equation in equations {
        match equation {
            Equation::Export(name, reference) => {
                let feature = feature_index(&ir.features, name)?;
                let source = position(reference)?;
                if feature != refs[source].1
                    || !interfaces[construction.category].contains(&feature)
                    || !exported.insert(feature)
                {
                    return Err(error(
                        &construction.name,
                        "export must supply each declared Category feature exactly once from the same domain",
                    ));
                }
                plan.exports.push((feature, groups[source]));
            }
            Equation::Require(reference, constant) => {
                let source = position(reference)?;
                let constant = value(ir, refs[source].1, constant)?;
                let register = groups[source];
                if plan.initial[register]
                    .as_ref()
                    .is_some_and(|previous| previous.to_string() != constant.to_string())
                {
                    return Err(error(
                        &construction.name,
                        "contradictory admission requirements",
                    ));
                }
                plan.initial[register] = Some(constant);
            }
            Equation::ExportConstant(name, constant) => {
                let feature = feature_index(&ir.features, name)?;
                if !interfaces[construction.category].contains(&feature)
                    || !exported.insert(feature)
                {
                    return Err(error(
                        &construction.name,
                        "export must supply each declared Category feature exactly once from the same domain",
                    ));
                }
                let constant = value(ir, feature, constant)?;
                let register = plan.initial.len();
                plan.initial.push(Some(constant));
                plan.exports.push((feature, register));
            }
            Equation::ExportTable(name, table_name, arguments) => {
                let feature = feature_index(&ir.features, name)?;
                let (table, registers) = table_call(table_name, arguments)?;
                let declaration = &ir.tables[table];
                if declaration.output != Some(feature)
                    || !interfaces[construction.category].contains(&feature)
                    || !exported.insert(feature)
                {
                    return Err(error(
                        &construction.name,
                        "export must supply each declared Category feature exactly once from the same domain",
                    ));
                }
                plan.table_exports.push(TableExport {
                    feature,
                    table,
                    registers,
                });
            }
            Equation::RequireTable(table_name, arguments, constant) => {
                let (table, registers) = table_call(table_name, arguments)?;
                let expected = value(
                    ir,
                    ir.tables[table]
                        .output
                        .ok_or_else(|| error(table_name, "separator is not a feature table"))?,
                    constant,
                )?;
                plan.table_requirements.push(TableRequirement {
                    table,
                    registers,
                    expected,
                });
            }
            Equation::RequireSelectedLiteral(table_name, arguments, text) => {
                let (table, registers) = table_call(table_name, arguments)?;
                plan.table_requirements.push(TableRequirement {
                    table,
                    registers,
                    expected: quote!(FeatureValue::SelectedLiteral(#text)),
                });
            }
            Equation::Agree(_, _) => {}
        }
    }
    if exported != interfaces[construction.category] {
        return Err(error(
            &construction.name,
            "required Category feature cannot reach admission: missing export",
        ));
    }
    Ok(plan)
}

fn helper(ir: &mut Ir, category: usize, symbols: Vec<Symbol>, build: Build) {
    let size = symbols.len();
    ir.rules.push(Rule {
        category,
        symbols,
        checks: vec![vec![]; size],
        initial: vec![],
        boundary: None,
        onset: None,
        exports: vec![],
        table_exports: vec![],
        table_requirements: vec![],
        release: vec![vec![]; size],
        segments: None,
        build,
    });
}

fn normalize_fields(
    ir: &mut Ir,
    constructor: usize,
    repeat_features: &[Vec<(usize, TokenStream)>],
) -> Vec<Symbol> {
    ir.constructors[constructor]
        .fields
        .clone()
        .into_iter()
        .enumerate()
        .map(|(field, (_, ty))| normalize_field(ir, &ty, &repeat_features[field]))
        .collect()
}

fn repeated_projections(
    ir: &Ir,
    constructor: usize,
    interfaces: &[BTreeSet<usize>],
    construction: &crate::parse::Construction,
) -> syn::Result<Vec<Vec<(usize, TokenStream)>>> {
    let mut features = vec![BTreeSet::new(); ir.constructors[constructor].fields.len()];
    let mut add = |reference: &Reference| -> syn::Result<()> {
        if reference.last {
            let (field, feature) =
                resolve_reference(ir, &ir.constructors[constructor], interfaces, reference)?;
            features[field].insert(feature);
        }
        Ok(())
    };
    for equation in construction
        .equations
        .iter()
        .chain(construction.form_requirements.iter().flatten())
    {
        for reference in equation.clone().references_mut() {
            add(reference)?;
        }
    }
    for part in construction.forms.iter().flatten() {
        if let Part::SelectedLiteral(_, arguments) = part {
            for reference in arguments {
                add(reference)?;
            }
        }
    }
    features
        .into_iter()
        .map(|features| {
            features
                .into_iter()
                .map(|feature| {
                    let DomainKind::Custom {
                        default: Some(default),
                    } = ir.features[feature].kind
                    else {
                        unreachable!()
                    };
                    let name = format_ident!("{}", ir.features[feature].values[default]);
                    Ok((feature, value(ir, feature, &name)?))
                })
                .collect()
        })
        .collect()
}

fn projected_helper(
    ir: &mut Ir,
    category: usize,
    symbols: Vec<Symbol>,
    build: Build,
    features: &[(usize, TokenStream)],
    source: Option<usize>,
) {
    helper(ir, category, symbols, build);
    let rule = ir.rules.last_mut().unwrap();
    for (register, (feature, default)) in features.iter().enumerate() {
        rule.initial.push(source.is_none().then(|| default.clone()));
        rule.exports.push((*feature, register));
        if let Some(dot) = source {
            rule.checks[dot].push((*feature, register));
        }
    }
}

fn normalize_field(ir: &mut Ir, ty: &FieldType, features: &[(usize, TokenStream)]) -> Symbol {
    if matches!(ty, FieldType::SelectedFrame(_, _)) {
        let category = ir.categories.len();
        ir.categories
            .push(format_ident!("__SelectedFrame{category}"));
        return Symbol::Nonterminal(category);
    }

    if let FieldType::Lexical(category) = ty {
        return Symbol::Lexical(category.clone());
    }
    let name = match ty {
        FieldType::One(c) | FieldType::Optional(c) | FieldType::Repeated(c, _) => c,
        FieldType::Lexical(_) | FieldType::SelectedFrame(_, _) => unreachable!(),
    };
    let child = ir.categories.iter().position(|n| n == name).unwrap();
    if matches!(ty, FieldType::One(_)) {
        return Symbol::Nonterminal(child);
    }
    let category = ir.categories.len();
    ir.categories.push(format_ident!("__Constituent{category}"));
    match ty {
        FieldType::Optional(_) => {
            helper(ir, category, vec![], Build::Absent);
            helper(
                ir,
                category,
                vec![Symbol::Nonterminal(child)],
                Build::Present,
            );
        }
        FieldType::Repeated(_, separator) => {
            let nonempty = ir.categories.len();
            ir.categories.push(format_ident!("__Constituent{nonempty}"));
            projected_helper(ir, category, vec![], Build::Empty, features, None);
            projected_helper(
                ir,
                category,
                vec![Symbol::Nonterminal(nonempty)],
                Build::Sequence,
                features,
                Some(0),
            );
            projected_helper(
                ir,
                nonempty,
                vec![Symbol::Nonterminal(child)],
                Build::Singleton,
                features,
                Some(0),
            );
            projected_helper(
                ir,
                nonempty,
                vec![
                    Symbol::Nonterminal(nonempty),
                    Symbol::Literal(separator.clone()),
                    Symbol::Nonterminal(child),
                ],
                Build::Append,
                features,
                Some(2),
            );
        }
        _ => unreachable!(),
    }
    Symbol::Nonterminal(category)
}

fn check_recursion(ir: &Ir) -> syn::Result<()> {
    let mut nullable = vec![false; ir.categories.len()];
    loop {
        let previous = nullable.clone();
        for rule in &ir.rules {
            if rule.symbols.iter().all(|s| is_nullable(s, &previous)) {
                nullable[rule.category] = true;
            }
        }
        if previous == nullable {
            break;
        }
    }
    let mut reaches = vec![vec![false; ir.categories.len()]; ir.categories.len()];
    for rule in &ir.rules {
        for (index, symbol) in rule.symbols.iter().enumerate() {
            if let Symbol::Nonterminal(child) = symbol
                && rule
                    .symbols
                    .iter()
                    .enumerate()
                    .all(|(i, s)| i == index || is_nullable(s, &nullable))
            {
                reaches[rule.category][*child] = true;
            }
        }
    }
    for via in 0..reaches.len() {
        for from in 0..reaches.len() {
            for to in 0..reaches.len() {
                reaches[from][to] |= reaches[from][via] && reaches[via][to];
            }
        }
    }
    if let Some(cycle) = (0..reaches.len()).find(|&i| reaches[i][i]) {
        return Err(error(
            &ir.categories[cycle],
            "packing obligation: recursion can revisit a constituent without consuming surface; cyclic Reading enumeration is unsafe",
        ));
    }
    Ok(())
}

fn is_nullable(symbol: &Symbol, nullable: &[bool]) -> bool {
    match symbol {
        Symbol::Literal(text) => text.is_empty(),
        Symbol::Lexical(_) => false,
        Symbol::Nonterminal(category) => nullable[*category],
    }
}

fn merge_schemas(ir: &mut Ir) -> syn::Result<()> {
    let mut merged: Vec<Constructor> = Vec::new();
    let mut remap = Vec::new();
    for constructor in std::mem::take(&mut ir.constructors) {
        if constructor.shared
            && let Some(index) = merged
                .iter()
                .position(|existing| existing.shared && existing.name == constructor.name)
        {
            let existing = &mut merged[index];
            if existing.fields.len() != constructor.fields.len()
                || existing
                    .fields
                    .iter()
                    .zip(&constructor.fields)
                    .any(|((a, ta), (b, tb))| {
                        a != b || std::mem::discriminant(ta) != std::mem::discriminant(tb)
                    })
            {
                return Err(error(
                    &constructor.name,
                    "schema instance fields differ in shape",
                ));
            }
            existing.forms.extend(constructor.forms);
            remap.push(index);
            continue;
        }
        remap.push(merged.len());
        merged.push(constructor);
    }
    for rule in &mut ir.rules {
        if let Build::Construction { constructor, .. } = &mut rule.build {
            *constructor = remap[*constructor];
        }
    }
    ir.constructors = merged;
    Ok(())
}
