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
    pub features: Vec<Domain>,
    pub frames: Vec<(Ident, Frame)>,
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
    Custom,
}

pub(crate) struct Constructor {
    pub name: Ident,
    pub category: usize,
    pub fields: Vec<(Ident, FieldType)>,
    pub forms: Vec<Form>,
}

pub(crate) struct Form {
    pub pieces: Vec<Piece>,
    pub rule: usize,
}

#[derive(Clone, PartialEq, Eq)]
pub(crate) enum Piece {
    Literal(String),
    Field(usize),
}

pub(crate) struct Rule {
    pub category: usize,
    pub symbols: Vec<Symbol>,
    pub checks: Vec<Vec<(usize, usize)>>,
    pub initial: Vec<Option<TokenStream>>,
    pub exports: Vec<(usize, usize)>,
    pub release: Vec<Vec<usize>>,
    pub build: Build,
}

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

pub(crate) fn validate(declaration: Declaration) -> syn::Result<Ir> {
    let mut features = builtin_domains();
    let mut feature_names: BTreeSet<_> = features.iter().map(|f| f.name.clone()).collect();
    for feature in &declaration.features {
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
        features.push(Domain {
            name: feature.name.to_string(),
            values: feature.values.iter().map(ToString::to_string).collect(),
            kind: DomainKind::Custom,
        });
    }
    let mut frames = vec![];
    let mut frame_names = BTreeSet::new();
    for (name, text) in &declaration.frames {
        reserve(&mut frame_names, name)?;
        let frame: Frame = ron::from_str(&text.value())
            .map_err(|e| error(name, &format!("lexical frame signature: {e}")))?;
        if frames.iter().any(|(_, previous)| previous == &frame) {
            return Err(error(
                name,
                "duplicate frame signature would introduce a false grammatical choice",
            ));
        }
        frames.push((name.clone(), frame));
    }
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
    let public_categories = categories.len();
    let category_index = |name: &str| categories.iter().position(|n| n == name);
    let mut constructors = vec![];
    names.clear();
    for construction in &declaration.constructions {
        reserve(&mut names, &construction.name)?;
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
                || fields
                    .iter()
                    .any(|(name, ty)| !other.iter().any(|(n, t)| name == n && ty == t))
            {
                return Err(error(
                    &construction.name,
                    "parse/materialize/realize obligation: every form must retain the same named fields and cardinalities",
                ));
            }
        }
        for (name, ty) in &fields {
            match ty {
                FieldType::Lexical(category) => {
                    ron::from_str::<deckmaste_lexical_model::Category>(category)
                        .map_err(|_| error(name, "unknown lexical Category"))?;
                }
                FieldType::One(category)
                | FieldType::Optional(category)
                | FieldType::Repeated(category, _) => {
                    if category_index(category).is_none() {
                        return Err(error(name, "unknown constituent Category"));
                    }
                }
            }
        }
        constructors.push(Constructor {
            name: construction.name.clone(),
            category,
            fields,
            forms: vec![],
        });
    }
    let mut ir = Ir {
        visibility: declaration.visibility,
        module: declaration.module,
        features,
        frames,
        categories,
        public_categories,
        constructors,
        rules: vec![],
    };
    normalize(&mut ir, &declaration.constructions, &interfaces)?;
    check_recursion(&ir)?;
    Ok(ir)
}

fn normalize(
    ir: &mut Ir,
    constructions: &[crate::parse::Construction],
    interfaces: &[BTreeSet<usize>],
) -> syn::Result<()> {
    for (index, construction) in constructions.iter().enumerate() {
        let plan = equations(ir, index, interfaces, &construction.equations)?;
        let mut field_symbols = vec![];
        for (_, ty) in ir.constructors[index].fields.clone() {
            field_symbols.push(normalize_field(ir, &ty));
        }
        for parts in &construction.forms {
            let pieces = normalize_form(parts, &ir.constructors[index].fields);
            let canonical = ir.constructors[index]
                .forms
                .iter()
                .position(|f| f.pieces == pieces);
            let form_index = canonical.unwrap_or(ir.constructors[index].forms.len());
            let mut symbols = vec![];
            let mut slots = vec![0; ir.constructors[index].fields.len()];
            let mut checks = vec![];
            for piece in &pieces {
                match piece {
                    Piece::Literal(text) => {
                        symbols.push(Symbol::Literal(text.clone()));
                        checks.push(vec![]);
                    }
                    Piece::Field(field) => {
                        slots[*field] = symbols.len();
                        symbols.push(match &field_symbols[*field] {
                            Symbol::Lexical(c) => Symbol::Lexical(c.clone()),
                            Symbol::Nonterminal(c) => Symbol::Nonterminal(*c),
                            Symbol::Literal(_) => unreachable!(),
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
                    && let Some(last) = checks
                        .iter()
                        .rposition(|c| c.iter().any(|(_, r)| *r == register))
                {
                    release[last].push(register);
                }
            }
            let rule = ir.rules.len();
            ir.rules.push(Rule {
                category: ir.constructors[index].category,
                symbols,
                checks,
                initial: plan.initial.clone(),
                exports: plan.exports.clone(),
                release,
                build: Build::Construction {
                    constructor: index,
                    form: form_index,
                    slots,
                },
            });
            if canonical.is_none() {
                ir.constructors[index].forms.push(Form { pieces, rule });
            }
        }
    }
    Ok(())
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

fn normalize_form(parts: &[Part], fields: &[(Ident, FieldType)]) -> Vec<Piece> {
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
        }
    }
    pieces
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
        DomainKind::Custom => quote!(FeatureValue::Custom(#feature, #index)),
    })
}

struct EquationPlan {
    references: BTreeMap<(usize, usize), usize>,
    initial: Vec<Option<TokenStream>>,
    exports: Vec<(usize, usize)>,
}

fn equations(
    ir: &Ir,
    constructor: usize,
    interfaces: &[BTreeSet<usize>],
    equations: &[Equation],
) -> syn::Result<EquationPlan> {
    let construction = &ir.constructors[constructor];
    let resolve = |reference: &Reference| -> syn::Result<(usize, usize)> {
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
        match &construction.fields[field].1 {
            FieldType::Lexical(_) => {}
            FieldType::One(category) => {
                let category = ir
                    .categories
                    .iter()
                    .position(|name| name == category)
                    .unwrap();
                if !interfaces[category].contains(&feature) {
                    return Err(error(
                        &construction.name,
                        &format!(
                            "admission cannot reach {}.{} through its Category interface",
                            reference.field, reference.feature
                        ),
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
    };
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
        }
    }
    let refs: Vec<_> = refs.into_iter().collect();
    let position = |r: &Reference| -> syn::Result<usize> {
        let resolved = resolve(r)?;
        refs.iter()
            .position(|v| *v == resolved)
            .ok_or_else(|| error(&construction.name, "unresolved admission reference"))
    };
    let mut groups: Vec<_> = (0..refs.len()).collect();
    for equation in equations {
        if let Equation::Agree(l, r) = equation {
            let left = position(l)?;
            let right = position(r)?;
            if refs[left].1 != refs[right].1 {
                return Err(error(
                    &construction.name,
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
    let mut plan = EquationPlan {
        references: refs.iter().copied().zip(groups.iter().copied()).collect(),
        initial: vec![None; refs.len()],
        exports: vec![],
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
        exports: vec![],
        release: vec![vec![]; size],
        build,
    });
}

fn normalize_field(ir: &mut Ir, ty: &FieldType) -> Symbol {
    if let FieldType::Lexical(category) = ty {
        return Symbol::Lexical(category.clone());
    }
    let name = match ty {
        FieldType::One(c) | FieldType::Optional(c) | FieldType::Repeated(c, _) => c,
        FieldType::Lexical(_) => unreachable!(),
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
            helper(ir, category, vec![], Build::Empty);
            helper(
                ir,
                category,
                vec![Symbol::Nonterminal(nonempty)],
                Build::Sequence,
            );
            helper(
                ir,
                nonempty,
                vec![Symbol::Nonterminal(child)],
                Build::Singleton,
            );
            helper(
                ir,
                nonempty,
                vec![
                    Symbol::Nonterminal(nonempty),
                    Symbol::Literal(separator.clone()),
                    Symbol::Nonterminal(child),
                ],
                Build::Append,
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
