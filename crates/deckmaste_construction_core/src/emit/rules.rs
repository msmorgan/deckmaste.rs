use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;

use crate::feature::Feature;
use crate::feature::FeatureExpr;
use crate::feature::FeaturePlace;
use crate::feature::FeatureValue;
use crate::identifier::RULE_CATEGORY_TYPE;
use crate::identifier::RULE_CONSTRUCTION_TYPE;
use crate::identifier::RULE_ID_COUNT;
use crate::identifier::RULE_ID_INDEX;
use crate::identifier::RULE_ID_TYPE;
use crate::identifier::RULES_CONSTANT;
use crate::identifier::emitted_ident;
use crate::model::TerminalBindingKind;
use crate::plan::DeclarationKey;
use crate::plan::DeclarationKind;
use crate::plan::GeneratedItem;
use crate::plan::ItemKey;
use crate::plan::NamedKind;
use crate::semantic::AtomPlan;
use crate::semantic::AtomTerminal;
use crate::semantic::ConstructionPlan;
use crate::semantic::EdgeClass;
use crate::semantic::FixedSurfaceAtomPlan;
use crate::semantic::FixedSurfacePlan;
use crate::semantic::SemanticPlan;
use crate::semantic::SeparatorPlan;
use crate::semantic::StructuralFieldKindPlan;
use crate::semantic::StructuralFieldPlan;
use crate::semantic::ValueKindPlan;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PositionalOwnerState {
    Empty,
    Singleton,
    Pair,
    ThreePlus,
}

impl PositionalOwnerState {
    pub(super) const fn spelling(self) -> &'static str {
        match self {
            Self::Empty => "positional_empty",
            Self::Singleton => "positional_singleton",
            Self::Pair => "positional_pair",
            Self::ThreePlus => "positional_three_plus",
        }
    }

    fn suffix(self) -> &'static str {
        match self {
            Self::Empty => "Empty",
            Self::Singleton => "Singleton",
            Self::Pair => "Pair",
            Self::ThreePlus => "ThreePlus",
        }
    }
}

#[derive(Debug, Clone)]
pub(super) enum RuleSymbolPlan {
    Authored {
        construction_index: usize,
        atom_index: usize,
    },
    Value(ValueKindPlan),
    Helper(String),
    Surface(FixedSurfaceAtomPlan),
}

impl RuleSymbolPlan {
    #[cfg(test)]
    fn test_label(&self) -> String {
        match self {
            Self::Authored { .. } => "authored".to_owned(),
            Self::Value(value) => format!("value:{}", value_name(value)),
            Self::Helper(category) => format!("helper:{category}"),
            Self::Surface(FixedSurfaceAtomPlan::Literal(value)) => {
                format!("literal:{value}")
            }
            Self::Surface(FixedSurfaceAtomPlan::Lex { terminal, variant }) => {
                format!("lex:{terminal}::{variant}")
            }
        }
    }
}

#[derive(Debug, Clone)]
pub(super) enum RuleBuildPlan {
    Construction {
        index: usize,
        positional: Vec<(String, PositionalOwnerState)>,
    },
    Product {
        index: usize,
        positional: Vec<(String, PositionalOwnerState)>,
    },
    Sum {
        sum_index: usize,
        alternative_index: usize,
    },
    Optional {
        owner: StructuralOwner,
        field_index: usize,
        present: bool,
    },
    Sequence {
        owner: StructuralOwner,
        field_index: usize,
        state: SequenceBuildState,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum StructuralOwner {
    Construction(usize),
    Product(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SequenceBuildState {
    Empty,
    Singleton,
    Recursive,
    Last,
    Middle,
}

#[derive(Debug, Clone)]
pub(super) struct RuleRowPlan {
    pub(super) id: String,
    pub(super) lhs: String,
    pub(super) owner: String,
    pub(super) role: Option<String>,
    pub(super) state: String,
    pub(super) public_construction: Option<String>,
    pub(super) rhs: Vec<RuleSymbolPlan>,
    pub(super) build: RuleBuildPlan,
}

pub(crate) fn emit(plan: &SemanticPlan) -> syn::Result<Vec<GeneratedItem>> {
    let category_type = ident(RULE_CATEGORY_TYPE);
    let construction_type = ident(RULE_CONSTRUCTION_TYPE);
    let rule_id_type = ident(RULE_ID_TYPE);
    let rule_id_count = ident(RULE_ID_COUNT);
    let rule_id_public_construction = ident("public_construction");
    let rule_id_index = ident(RULE_ID_INDEX);
    let rules_constant = ident(RULES_CONSTANT);
    let constructions = plan.constructions();
    let origins = construction_origins(constructions);
    let categories = category_names(plan);
    let lowered = lowered_rows(plan)?;
    let construction_ids = constructions
        .iter()
        .map(|construction| ident(construction.rule_id()))
        .collect::<Vec<_>>();
    let rule_ids = lowered.iter().map(|row| ident(&row.id)).collect::<Vec<_>>();
    let count = syn::LitInt::new(&lowered.len().to_string(), Span::call_site());
    let construction_matches = lowered.iter().zip(&rule_ids).map(|(row, rule_id)| {
        row.public_construction.as_ref().map_or_else(
            || quote! { Self::#rule_id => None },
            |construction| {
                let construction = ident(construction);
                quote! { Self::#rule_id => Some(Construction::#construction) }
            },
        )
    });
    let owner_matches = lowered.iter().zip(&rule_ids).map(|(row, rule_id)| {
        let owner = syn::LitStr::new(&row.owner, Span::call_site());
        quote! { Self::#rule_id => #owner }
    });
    let role_matches = lowered.iter().zip(&rule_ids).map(|(row, rule_id)| {
        let role = row.role.as_ref().map_or_else(
            || quote! { None },
            |role| {
                let role = syn::LitStr::new(role, Span::call_site());
                quote! { Some(#role) }
            },
        );
        quote! { Self::#rule_id => #role }
    });
    let state_matches = lowered.iter().zip(&rule_ids).map(|(row, rule_id)| {
        let state = syn::LitStr::new(&row.state, Span::call_site());
        quote! { Self::#rule_id => #state }
    });
    let rows = lowered
        .iter()
        .zip(&rule_ids)
        .map(|(row, rule_id)| emit_rule(plan, row, rule_id))
        .collect::<syn::Result<Vec<_>>>()?;

    let mut rule_origins = origins.clone();
    rule_origins.extend(
        plan.roots()
            .iter()
            .filter(|root| root.is_parse_entry())
            .map(|root| DeclarationKey::new(DeclarationKind::Root, root.category())),
    );
    Ok(vec![
        GeneratedItem::new(
            ItemKey::named_type(RULE_CATEGORY_TYPE),
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub(crate) enum #category_type { #(#categories),* }
            },
            category_origins(plan),
        ),
        GeneratedItem::new(
            ItemKey::named_type(RULE_CONSTRUCTION_TYPE),
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub(crate) enum #construction_type { #(#construction_ids),* }
            },
            origins.clone(),
        ),
        GeneratedItem::new(
            ItemKey::named_type(RULE_ID_TYPE),
            quote! {
                #[repr(usize)]
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub(crate) enum #rule_id_type { #(#rule_ids),* }
            },
            origins.clone(),
        ),
        GeneratedItem::new(
            ItemKey::Impl {
                trait_name: None,
                self_ty: RULE_ID_TYPE.to_owned(),
            },
            quote! {
                impl RuleId {
                    #[cfg(test)]
                    pub(crate) const #rule_id_count: usize = #count;
                    pub(crate) const fn #rule_id_public_construction(self) -> Option<Construction> {
                        match self { #(#construction_matches,)* }
                    }
                    pub(crate) const fn owner(self) -> &'static str {
                        match self { #(#owner_matches,)* }
                    }
                    pub(crate) const fn role(self) -> Option<&'static str> {
                        match self { #(#role_matches,)* }
                    }
                    pub(crate) const fn state(self) -> &'static str {
                        match self { #(#state_matches,)* }
                    }
                    pub(crate) const fn #rule_id_index(self) -> usize { self as usize }
                }
            },
            origins.clone(),
        ),
        GeneratedItem::new(
            ItemKey::Named {
                kind: NamedKind::Constant,
                name: RULES_CONSTANT.to_owned(),
            },
            quote! { pub(crate) const #rules_constant: &[Rule<Category, LexicalTerminal, RuleId>] = &[#(#rows),*]; },
            rule_origins,
        ),
    ])
}

fn emit_rule(
    plan: &SemanticPlan,
    row: &RuleRowPlan,
    rule_id: &syn::Ident,
) -> syn::Result<TokenStream> {
    let lhs = ident(&row.lhs);
    let mut rhs = row
        .rhs
        .iter()
        .map(|symbol| emit_rule_symbol(plan, symbol))
        .collect::<syn::Result<Vec<_>>>()?;
    if let Some(root) = plan.parse_root(&row.lhs) {
        if !root.punctuation().is_empty() {
            let punctuation = syn::LitStr::new(root.punctuation(), Span::call_site());
            let stable_id = syn::LitStr::new(
                &format!("root:{}/punctuation", root.category()),
                Span::call_site(),
            );
            rhs.push(lexical_terminal(
                &quote! { Lexical::Literal(#punctuation) },
                &quote! {
                    LexicalOwnerTemplate::Static {
                        kind: LexicalProvenanceKind::FormLiteral,
                        stable_id: #stable_id,
                    }
                },
            ));
        }
        rhs.push(lexical_terminal(
            &quote! { Lexical::EndOfInput },
            &quote! { LexicalOwnerTemplate::None },
        ));
    }
    Ok(quote! { Rule { id: RuleId::#rule_id, lhs: Category::#lhs, rhs: &[#(#rhs),*] } })
}

fn emit_rule_symbol(plan: &SemanticPlan, symbol: &RuleSymbolPlan) -> syn::Result<TokenStream> {
    match symbol {
        RuleSymbolPlan::Authored {
            construction_index,
            atom_index,
        } => {
            let construction = &plan.constructions()[*construction_index];
            emit_position(
                plan,
                construction,
                *atom_index,
                &construction.atoms()[*atom_index],
            )
        }
        RuleSymbolPlan::Value(value) => emit_value_position(plan, value),
        RuleSymbolPlan::Helper(category) => {
            let category = ident(category);
            Ok(quote! { N(Category::#category) })
        }
        RuleSymbolPlan::Surface(FixedSurfaceAtomPlan::Literal(value)) => {
            let value = syn::LitStr::new(value, Span::call_site());
            Ok(lexical_terminal(
                &quote! { Lexical::Literal(#value) },
                &quote! { LexicalOwnerTemplate::None },
            ))
        }
        RuleSymbolPlan::Surface(FixedSurfaceAtomPlan::Lex { terminal, .. }) => {
            let lexical = lexical_variant(plan, terminal)?;
            Ok(lexical_terminal(
                &lexical,
                &quote! { LexicalOwnerTemplate::None },
            ))
        }
    }
}

fn emit_value_position(plan: &SemanticPlan, value: &ValueKindPlan) -> syn::Result<TokenStream> {
    match value {
        ValueKindPlan::Category(name) | ValueKindPlan::Product(name) | ValueKindPlan::Sum(name) => {
            let name = ident(name);
            Ok(quote! { N(Category::#name) })
        }
        ValueKindPlan::Lex(name) | ValueKindPlan::Identity(name) => {
            let lexical = lexical_variant(plan, name)?;
            let owner = owner_template(plan, name)?;
            Ok(lexical_terminal(&lexical, &owner))
        }
    }
}

pub(super) fn lowered_rows(plan: &SemanticPlan) -> syn::Result<Vec<RuleRowPlan>> {
    let mut rows = Vec::new();
    for (index, construction) in plan.constructions().iter().enumerate() {
        rows.extend(lower_construction_rows(index, construction)?);
        rows.extend(lower_helper_rows(
            StructuralOwner::Construction(index),
            construction.element_type(),
            construction
                .fields()
                .iter()
                .enumerate()
                .filter_map(|(index, field)| field.structural_plan().map(|field| (index, field))),
        )?);
    }
    for (index, product) in plan.products().iter().enumerate() {
        rows.extend(lower_product_rows(index, product)?);
        rows.extend(lower_helper_rows(
            StructuralOwner::Product(index),
            product.name(),
            product.fields().iter().enumerate(),
        )?);
    }
    for (sum_index, sum) in plan.sums().iter().enumerate() {
        for (alternative_index, alternative) in sum.alternatives().iter().enumerate() {
            rows.push(RuleRowPlan {
                id: format!(
                    "{}{}",
                    crate::identifier::pascal_case(sum.name()),
                    crate::identifier::pascal_case(alternative.name()),
                ),
                lhs: sum.name().to_owned(),
                owner: sum.name().to_owned(),
                role: Some(alternative.name().to_owned()),
                state: "sum_alternative".to_owned(),
                public_construction: None,
                rhs: vec![RuleSymbolPlan::Value(alternative.value().clone())],
                build: RuleBuildPlan::Sum {
                    sum_index,
                    alternative_index,
                },
            });
        }
    }
    Ok(rows)
}

fn lower_construction_rows(
    construction_index: usize,
    construction: &ConstructionPlan,
) -> syn::Result<Vec<RuleRowPlan>> {
    let mut variants = vec![(
        Vec::<(String, PositionalOwnerState)>::new(),
        Vec::<RuleSymbolPlan>::new(),
    )];
    for (atom_index, atom) in construction.atoms().iter().enumerate() {
        let structural = atom_role(atom).and_then(|role| {
            construction
                .fields()
                .iter()
                .find(|field| field.name_key() == role)
                .and_then(crate::semantic::ConstructionFieldPlan::structural_plan)
        });
        let Some(field) = structural else {
            for (_, rhs) in &mut variants {
                rhs.push(RuleSymbolPlan::Authored {
                    construction_index,
                    atom_index,
                });
            }
            continue;
        };
        let field_variants = owner_field_variants(construction.element_type(), field)?;
        variants = combine_owner_variants(variants, field, &field_variants);
    }
    variants
        .into_iter()
        .map(|(positional, rhs)| {
            let (id, role, state) = public_variant_metadata(
                construction.rule_id(),
                construction.element_type(),
                construction.fields().iter().filter_map(|field| {
                    field
                        .structural_plan()
                        .map(|structural| (field.name_key(), structural))
                }),
                &positional,
            )?;
            Ok(RuleRowPlan {
                id,
                lhs: construction.category().to_owned(),
                owner: construction.element_type().to_owned(),
                role,
                state,
                public_construction: Some(construction.rule_id().to_owned()),
                rhs,
                build: RuleBuildPlan::Construction {
                    index: construction_index,
                    positional,
                },
            })
        })
        .collect()
}

fn lower_product_rows(
    product_index: usize,
    product: &crate::semantic::ProductPlan,
) -> syn::Result<Vec<RuleRowPlan>> {
    let mut variants = vec![(
        Vec::<(String, PositionalOwnerState)>::new(),
        Vec::<RuleSymbolPlan>::new(),
    )];
    for field in product.fields() {
        let field_variants = owner_field_variants(product.name(), field)?;
        variants = combine_owner_variants(variants, field, &field_variants);
    }
    variants
        .into_iter()
        .map(|(positional, rhs)| {
            let base = format!("{}Product", crate::identifier::pascal_case(product.name()));
            let (id, role, state) = public_variant_metadata(
                &base,
                product.name(),
                product
                    .fields()
                    .iter()
                    .map(|field| (field.name().to_owned(), field)),
                &positional,
            )?;
            Ok(RuleRowPlan {
                id,
                lhs: product.name().to_owned(),
                owner: product.name().to_owned(),
                role,
                state,
                public_construction: None,
                rhs,
                build: RuleBuildPlan::Product {
                    index: product_index,
                    positional,
                },
            })
        })
        .collect()
}

fn public_variant_metadata<'a>(
    base: &str,
    _owner: &str,
    fields: impl Iterator<Item = (String, &'a StructuralFieldPlan)>,
    positional: &[(String, PositionalOwnerState)],
) -> syn::Result<(String, Option<String>, String)> {
    if positional.is_empty() {
        return Ok((base.to_owned(), None, "public".to_owned()));
    }
    if positional.len() == 1 {
        let (role, state) = &positional[0];
        let field = fields
            .into_iter()
            .find(|(candidate, _)| candidate == role)
            .map(|(_, field)| field)
            .ok_or_else(|| internal("positional owner state has no structural field"))?;
        let aggregate = field
            .helper_names()
            .ok_or_else(|| internal("positional sequence has no helper inventory"))?
            .all()[0];
        return Ok((
            format!("{aggregate}{}", state.suffix()),
            Some(role.clone()),
            state.spelling().to_owned(),
        ));
    }
    let suffix = positional
        .iter()
        .map(|(role, state)| format!("{}{}", crate::identifier::pascal_case(role), state.suffix()))
        .collect::<String>();
    Ok((
        format!("{base}{suffix}"),
        None,
        "positional_product".to_owned(),
    ))
}

fn combine_owner_variants(
    current: Vec<(Vec<(String, PositionalOwnerState)>, Vec<RuleSymbolPlan>)>,
    field: &StructuralFieldPlan,
    field_variants: &[(Option<PositionalOwnerState>, Vec<RuleSymbolPlan>)],
) -> Vec<(Vec<(String, PositionalOwnerState)>, Vec<RuleSymbolPlan>)> {
    current
        .into_iter()
        .flat_map(|(states, rhs)| {
            field_variants.iter().map(move |(state, field_rhs)| {
                let mut states = states.clone();
                if let Some(state) = state {
                    states.push((field.name().to_owned(), *state));
                }
                let mut combined = rhs.clone();
                combined.extend(field_rhs.iter().cloned());
                (states, combined)
            })
        })
        .collect()
}

fn owner_field_variants(
    owner: &str,
    field: &StructuralFieldPlan,
) -> syn::Result<Vec<(Option<PositionalOwnerState>, Vec<RuleSymbolPlan>)>> {
    match field.kind() {
        StructuralFieldKindPlan::Required(value) => {
            Ok(vec![(None, vec![RuleSymbolPlan::Value(value.clone())])])
        }
        StructuralFieldKindPlan::Optional(_) => Ok(vec![(
            None,
            vec![RuleSymbolPlan::Helper(helper_category(owner, field)?)],
        )]),
        StructuralFieldKindPlan::Sequence {
            item,
            bounds,
            surface,
        } => match surface.separator() {
            Some(SeparatorPlan::Positional(rows)) => {
                let mut variants = Vec::new();
                if bounds.allows(0) {
                    variants.push((Some(PositionalOwnerState::Empty), Vec::new()));
                }
                if bounds.allows(1) {
                    variants.push((
                        Some(PositionalOwnerState::Singleton),
                        item_with_terminator(item, surface.terminator()),
                    ));
                }
                if bounds.allows(2) {
                    let mut rhs = item_with_terminator(item, surface.terminator());
                    rhs.extend(surface_symbols(positional_surface(rows, EdgeClass::Pair)?));
                    rhs.extend(item_with_terminator(item, surface.terminator()));
                    variants.push((Some(PositionalOwnerState::Pair), rhs));
                }
                if bounds.allows_at_least(3) {
                    let mut rhs = item_with_terminator(item, surface.terminator());
                    rhs.extend(surface_symbols(positional_surface(rows, EdgeClass::First)?));
                    rhs.push(RuleSymbolPlan::Helper(helper_category(owner, field)?));
                    variants.push((Some(PositionalOwnerState::ThreePlus), rhs));
                }
                Ok(variants)
            }
            Some(SeparatorPlan::Uniform(_)) | None => Ok(vec![(
                None,
                vec![RuleSymbolPlan::Helper(helper_category(owner, field)?)],
            )]),
        },
    }
}

fn lower_helper_rows<'a>(
    owner_key: StructuralOwner,
    owner: &str,
    fields: impl Iterator<Item = (usize, &'a StructuralFieldPlan)>,
) -> syn::Result<Vec<RuleRowPlan>> {
    let mut rows = Vec::new();
    for (field_index, field) in fields {
        if matches!(field.kind(), StructuralFieldKindPlan::Required(_)) {
            continue;
        }
        let aggregate = field.helper_names().map_or_else(
            || {
                format!(
                    "{}{}Optional",
                    crate::identifier::pascal_case(owner),
                    crate::identifier::pascal_case(field.name()),
                )
            },
            |names| names.all()[0].to_owned(),
        );
        let category = helper_category(owner, field)?;
        let role = Some(field.name().to_owned());
        match field.kind() {
            StructuralFieldKindPlan::Required(_) => {}
            StructuralFieldKindPlan::Optional(value) => {
                for (suffix, state, present, rhs) in [
                    ("Absent", "optional_absent", false, Vec::new()),
                    (
                        "Present",
                        "optional_present",
                        true,
                        vec![RuleSymbolPlan::Value(value.clone())],
                    ),
                ] {
                    rows.push(RuleRowPlan {
                        id: format!("{aggregate}{suffix}"),
                        lhs: category.clone(),
                        owner: owner.to_owned(),
                        role: role.clone(),
                        state: state.to_owned(),
                        public_construction: None,
                        rhs,
                        build: RuleBuildPlan::Optional {
                            owner: owner_key,
                            field_index,
                            present,
                        },
                    });
                }
            }
            StructuralFieldKindPlan::Sequence { item, surface, .. } => match surface.separator() {
                Some(SeparatorPlan::Positional(positional)) => {
                    let mut last = item_with_terminator(item, surface.terminator());
                    last.extend(surface_symbols(positional_surface(
                        positional,
                        EdgeClass::Last,
                    )?));
                    last.extend(item_with_terminator(item, surface.terminator()));
                    rows.push(sequence_helper_row(
                        owner_key,
                        field_index,
                        owner,
                        field.name(),
                        &aggregate,
                        &category,
                        "Last",
                        "positional_last",
                        SequenceBuildState::Last,
                        last,
                    ));
                    if positional
                        .iter()
                        .any(|row| row.class() == EdgeClass::Middle)
                    {
                        let mut middle = item_with_terminator(item, surface.terminator());
                        middle.extend(surface_symbols(positional_surface(
                            positional,
                            EdgeClass::Middle,
                        )?));
                        middle.push(RuleSymbolPlan::Helper(category.clone()));
                        rows.push(sequence_helper_row(
                            owner_key,
                            field_index,
                            owner,
                            field.name(),
                            &aggregate,
                            &category,
                            "Middle",
                            "positional_middle",
                            SequenceBuildState::Middle,
                            middle,
                        ));
                    }
                }
                uniform => {
                    rows.push(sequence_helper_row(
                        owner_key,
                        field_index,
                        owner,
                        field.name(),
                        &aggregate,
                        &category,
                        "Empty",
                        "sequence_empty",
                        SequenceBuildState::Empty,
                        Vec::new(),
                    ));
                    rows.push(sequence_helper_row(
                        owner_key,
                        field_index,
                        owner,
                        field.name(),
                        &aggregate,
                        &category,
                        "Singleton",
                        "sequence_singleton",
                        SequenceBuildState::Singleton,
                        item_with_terminator(item, surface.terminator()),
                    ));
                    let mut recursive = item_with_terminator(item, surface.terminator());
                    if let Some(SeparatorPlan::Uniform(separator)) = uniform {
                        recursive.extend(surface_symbols(separator));
                    }
                    recursive.push(RuleSymbolPlan::Helper(category.clone()));
                    rows.push(sequence_helper_row(
                        owner_key,
                        field_index,
                        owner,
                        field.name(),
                        &aggregate,
                        &category,
                        "Recursive",
                        "sequence_recursive",
                        SequenceBuildState::Recursive,
                        recursive,
                    ));
                }
            },
        }
    }
    Ok(rows)
}

#[allow(clippy::too_many_arguments)]
fn sequence_helper_row(
    owner_key: StructuralOwner,
    field_index: usize,
    owner: &str,
    role: &str,
    aggregate: &str,
    category: &str,
    suffix: &str,
    state: &str,
    build_state: SequenceBuildState,
    rhs: Vec<RuleSymbolPlan>,
) -> RuleRowPlan {
    RuleRowPlan {
        id: format!("{aggregate}{suffix}"),
        lhs: category.to_owned(),
        owner: owner.to_owned(),
        role: Some(role.to_owned()),
        state: state.to_owned(),
        public_construction: None,
        rhs,
        build: RuleBuildPlan::Sequence {
            owner: owner_key,
            field_index,
            state: build_state,
        },
    }
}

fn item_with_terminator(
    item: &ValueKindPlan,
    terminator: Option<&FixedSurfacePlan>,
) -> Vec<RuleSymbolPlan> {
    let mut symbols = vec![RuleSymbolPlan::Value(item.clone())];
    if let Some(terminator) = terminator {
        symbols.extend(surface_symbols(terminator));
    }
    symbols
}

fn surface_symbols(surface: &FixedSurfacePlan) -> Vec<RuleSymbolPlan> {
    surface
        .atoms()
        .iter()
        .cloned()
        .map(RuleSymbolPlan::Surface)
        .collect()
}

fn positional_surface(
    rows: &[crate::semantic::PositionalSeparatorPlan],
    class: EdgeClass,
) -> syn::Result<&FixedSurfacePlan> {
    rows.iter()
        .find(|row| row.class() == class)
        .map(crate::semantic::PositionalSeparatorPlan::surface)
        .ok_or_else(|| internal("reachable positional class has no sealed surface"))
}

fn helper_category(owner: &str, field: &StructuralFieldPlan) -> syn::Result<String> {
    if let Some(names) = field.helper_names() {
        return Ok(names.all()[1].to_owned());
    }
    if matches!(field.kind(), StructuralFieldKindPlan::Optional(_)) {
        return Ok(format!(
            "{}{}OptionalCategory",
            crate::identifier::pascal_case(owner),
            crate::identifier::pascal_case(field.name()),
        ));
    }
    Err(internal(
        "structural cardinality field has no helper inventory",
    ))
}

fn atom_role(atom: &AtomPlan) -> Option<&str> {
    match atom {
        AtomPlan::Category { role, .. }
        | AtomPlan::Lex { role, .. }
        | AtomPlan::Identity { role, .. }
        | AtomPlan::Noun { role, .. } => Some(role),
        AtomPlan::Literal(_) | AtomPlan::VerbFixed { .. } | AtomPlan::OpenDeclaration(_) => None,
    }
}

#[cfg(test)]
fn value_name(value: &ValueKindPlan) -> &str {
    match value {
        ValueKindPlan::Category(name)
        | ValueKindPlan::Lex(name)
        | ValueKindPlan::Identity(name)
        | ValueKindPlan::Product(name)
        | ValueKindPlan::Sum(name) => name,
    }
}

fn emit_position(
    plan: &SemanticPlan,
    construction: &ConstructionPlan,
    atom_index: usize,
    atom: &AtomPlan,
) -> syn::Result<TokenStream> {
    if let Some(role) = atom_role(atom)
        && let Some(structural) = construction
            .fields()
            .iter()
            .find(|field| field.name_key() == role)
            .and_then(crate::semantic::ConstructionFieldPlan::structural_plan)
    {
        match structural.kind() {
            StructuralFieldKindPlan::Optional(_) => {
                let category = ident(&helper_category(construction.element_type(), structural)?);
                return Ok(quote! { N(Category::#category) });
            }
            StructuralFieldKindPlan::Sequence { surface, .. } => {
                if matches!(surface.separator(), Some(SeparatorPlan::Positional(_))) {
                    return Err(internal(
                        "positional sequence atom reached the unexpanded public rule",
                    ));
                }
                let category = ident(&helper_category(construction.element_type(), structural)?);
                return Ok(quote! { N(Category::#category) });
            }
            StructuralFieldKindPlan::Required(_) => {}
        }
    }
    match atom {
        AtomPlan::Literal(literal) => {
            let literal = syn::LitStr::new(literal, Span::call_site());
            let stable_id = syn::LitStr::new(
                &format!(
                    "form:{}/{}/{}",
                    construction.construction_id(),
                    construction.form(),
                    atom_index
                ),
                Span::call_site(),
            );
            Ok(lexical_terminal(
                &quote! { Lexical::Literal(#literal) },
                &quote! {
                    LexicalOwnerTemplate::Static {
                        kind: LexicalProvenanceKind::FormLiteral,
                        stable_id: #stable_id,
                    }
                },
            ))
        }
        AtomPlan::Category { category, .. } => {
            let category = ident(category);
            Ok(quote! { N(Category::#category) })
        }
        AtomPlan::Lex { terminal, .. } | AtomPlan::Identity { terminal, .. } => {
            let lexical = lexical_variant(plan, terminal)?;
            let owner = owner_template(plan, terminal)?;
            Ok(lexical_terminal(&lexical, &owner))
        }
        AtomPlan::Noun { terminal, .. } => {
            let lexical = lexical_variant(plan, terminal)?;
            let number = noun_number(plan, construction)?;
            let owner = owner_template(plan, terminal)?;
            Ok(lexical_terminal(&quote! { #lexical(#number) }, &owner))
        }
        AtomPlan::VerbFixed {
            terminal, variant, ..
        } => {
            let terminal = ident(terminal);
            let variant = ident(variant);
            let agreement = closed_verb_feature(plan, construction)?;
            let declaration = syn::LitStr::new(&terminal.to_string(), Span::call_site());
            let member = syn::LitStr::new(&variant.to_string(), Span::call_site());
            Ok(lexical_terminal(
                &quote! { Lexical::Verb(#terminal::#variant, #agreement) },
                &quote! {
                    LexicalOwnerTemplate::Lexeme { declaration: #declaration, member: #member }
                },
            ))
        }
        AtomPlan::OpenDeclaration(open) => {
            let kind = crate::emit::declaration_kind(open.kind());
            let name = syn::LitStr::new(open.name(), Span::call_site());
            let position = crate::emit::grammar_position(open.position());
            let feature = open_verb_feature(plan, construction)?;
            Ok(lexical_terminal(
                &quote! {
                    Lexical::Declaration(DeclarationMatcher {
                        kind: #kind,
                        name: #name,
                        position: #position,
                        feature: #feature,
                    })
                },
                &quote! { LexicalOwnerTemplate::Declaration { kind: #kind, name: #name } },
            ))
        }
    }
}

fn closed_verb_feature(
    plan: &SemanticPlan,
    construction: &ConstructionPlan,
) -> syn::Result<TokenStream> {
    let target = FeaturePlace::Role {
        field: syn::Ident::new("verb", construction.origin_span()),
        feature: Feature::Agreement,
    };
    match plan.feature_resolution(construction.construction_id(), &target) {
        Some(crate::feature::FeatureResolution::Known(value)) => match value {
            FeatureValue::Bare => Ok(quote! {
                FeatureConstraint::Exact(Agreement::Bare)
            }),
            FeatureValue::ThirdPersonSingular => Ok(quote! {
                FeatureConstraint::Exact(Agreement::ThirdPersonSingular)
            }),
            FeatureValue::Singular | FeatureValue::Plural => {
                Err(internal("closed verb agreement has a number value"))
            }
        },
        Some(
            crate::feature::FeatureResolution::External
            | crate::feature::FeatureResolution::Runtime,
        ) => Ok(quote! { FeatureConstraint::Any }),
        None => Err(internal("closed verb has no sealed agreement resolution")),
    }
}

pub(crate) fn open_verb_feature(
    plan: &SemanticPlan,
    construction: &ConstructionPlan,
) -> syn::Result<TokenStream> {
    let target = FeaturePlace::Role {
        field: syn::Ident::new("verb", construction.origin_span()),
        feature: Feature::Agreement,
    };
    match plan.feature_resolution(construction.construction_id(), &target) {
        Some(crate::feature::FeatureResolution::Known(value)) => match value {
            FeatureValue::Bare => Ok(quote! {
                FeatureConstraint::Exact(::macro_ron::v2::SurfaceFeature::Bare)
            }),
            FeatureValue::ThirdPersonSingular => Ok(quote! {
                FeatureConstraint::Exact(::macro_ron::v2::SurfaceFeature::ThirdPersonSingular)
            }),
            FeatureValue::Singular | FeatureValue::Plural => {
                Err(internal("open verb agreement has a number value"))
            }
        },
        Some(
            crate::feature::FeatureResolution::External
            | crate::feature::FeatureResolution::Runtime,
        ) => Ok(quote! { FeatureConstraint::Any }),
        None => Err(internal("open verb has no sealed agreement resolution")),
    }
}

fn lexical_terminal(matcher: &TokenStream, owner: &TokenStream) -> TokenStream {
    quote! {
        L(LexicalTerminal { matcher: #matcher, owner: #owner })
    }
}

fn owner_template(plan: &SemanticPlan, terminal: &str) -> syn::Result<TokenStream> {
    let declaration = syn::LitStr::new(terminal, Span::call_site());
    match plan.atom_terminal(terminal)? {
        AtomTerminal::Vocab(_) => Ok(quote! {
            LexicalOwnerTemplate::Vocab { declaration: #declaration }
        }),
        AtomTerminal::Binding(binding) => {
            let (kind, prefix) = match binding.kind() {
                TerminalBindingKind::Codec => (quote! { LexicalProvenanceKind::Codec }, "codec"),
                TerminalBindingKind::Identity => {
                    (quote! { LexicalProvenanceKind::Identity }, "identity")
                }
            };
            let stable_id = syn::LitStr::new(&format!("{prefix}:{terminal}"), Span::call_site());
            Ok(quote! {
                LexicalOwnerTemplate::Static { kind: #kind, stable_id: #stable_id }
            })
        }
        AtomTerminal::ContextIdentity(_) => Ok(quote! {
            LexicalOwnerTemplate::Identity { declaration: #declaration }
        }),
        AtomTerminal::SignedDecimal(_) => {
            let stable_id = syn::LitStr::new(&format!("codec:{terminal}"), Span::call_site());
            Ok(quote! {
                LexicalOwnerTemplate::Static {
                    kind: LexicalProvenanceKind::Codec,
                    stable_id: #stable_id,
                }
            })
        }
        AtomTerminal::DeclarationNoun(codec) => {
            debug_assert_eq!(codec.position(), ::macro_ron::v2::GrammarPosition::Noun);
            Ok(quote! { LexicalOwnerTemplate::DeclarationNoun })
        }
    }
}

fn lexical_variant(plan: &SemanticPlan, name: &str) -> syn::Result<TokenStream> {
    match plan.atom_terminal(name)? {
        AtomTerminal::Vocab(vocab) => {
            let name = ident(vocab.name());
            Ok(quote! { Lexical::#name })
        }
        AtomTerminal::Binding(binding) => {
            if binding.codec_atom() == Some(crate::model::CodecAtomClass::Noun) {
                return Ok(quote! { Lexical::Noun });
            }
            let path = binding
                .lexical_variant()
                .ok_or_else(|| internal("atom-capable terminal binding has no lexical variant"))?;
            Ok(quote! { #path })
        }
        AtomTerminal::ContextIdentity(identity) => {
            let variant = identity.aggregate_ident();
            Ok(quote! { Lexical::#variant })
        }
        AtomTerminal::SignedDecimal(codec) => {
            let variant = codec.codec_ident();
            Ok(quote! { Lexical::#variant })
        }
        AtomTerminal::DeclarationNoun(_) => Ok(quote! { Lexical::Noun }),
    }
}

fn noun_number(plan: &SemanticPlan, construction: &ConstructionPlan) -> syn::Result<TokenStream> {
    let equation = plan
        .feature_equations(construction.construction_id())
        .iter()
        .find(|equation| equation.target() == &FeaturePlace::Construction(Feature::Number))
        .ok_or_else(|| internal("noun atom has no validated construction number"))?;
    match equation.value() {
        FeatureExpr::Constant(value) => match value.value() {
            FeatureValue::Singular => Ok(quote! { FeatureConstraint::Exact(Number::Singular) }),
            FeatureValue::Plural => Ok(quote! { FeatureConstraint::Exact(Number::Plural) }),
            FeatureValue::Bare | FeatureValue::ThirdPersonSingular => {
                Err(internal("noun number has an agreement value"))
            }
        },
        FeatureExpr::MatchVocab { .. } | FeatureExpr::FromRole { .. } => {
            Ok(quote! { FeatureConstraint::Any })
        }
    }
}

fn construction_origins(constructions: &[ConstructionPlan]) -> Vec<DeclarationKey> {
    constructions
        .iter()
        .map(|construction| {
            DeclarationKey::new(
                DeclarationKind::Construction,
                construction.construction_id(),
            )
        })
        .collect()
}

fn category_names(plan: &SemanticPlan) -> Vec<syn::Ident> {
    super::semantic_types(plan)
        .into_iter()
        .map(|item| ident(item.name))
        .chain(
            super::structural_carriers(plan)
                .into_iter()
                .map(|carrier| ident(&carrier.category_variant())),
        )
        .collect()
}

fn category_origins(plan: &SemanticPlan) -> Vec<DeclarationKey> {
    plan.declaration_keys()
        .iter()
        .filter(|origin| {
            matches!(
                origin.kind(),
                DeclarationKind::Construction
                    | DeclarationKind::AbstractProduct
                    | DeclarationKind::AbstractSum
            )
        })
        .cloned()
        .collect()
}

fn ident(name: &str) -> syn::Ident {
    emitted_ident(name, Span::call_site())
}
fn internal(message: &str) -> syn::Error {
    syn::Error::new(Span::call_site(), message)
}

#[cfg(test)]
mod tests {
    use quote::quote;

    fn structural_semantic_plan() -> crate::semantic::SemanticPlan {
        crate::validate_declarations(
            crate::parse_declarations(quote! {
                vocab Marker { Alpha = "alpha", Beta = "beta", }
                construction item: Item {
                    element ItemValue { marker: lex Marker, }
                    form item = lex(marker);
                }
                construction uniform: Uniform {
                    element UniformValue {
                        maybe: opt Item,
                        items: seq Item separated by "<S>" terminated by "<T>",
                    }
                    require len(items) >= 1;
                    form uniform = maybe items;
                }
                construction positional: Positional {
                    element PositionalValue {
                        items: seq Item separated by position {
                            pair = "<P>";
                            first = "<F>";
                            middle = "<M>";
                            last = "<L>";
                        } terminated by "<T>",
                    }
                    require len(items) >= 1;
                    form positional = items;
                }
                root Item { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect("structural rule fixture parses"),
        )
        .expect("structural rule fixture validates")
        .into_semantic()
    }

    #[test]
    fn structural_optional_and_uniform_rows_have_exact_metadata_and_surface_order() {
        let plan = structural_semantic_plan();
        let rows = super::lowered_rows(&plan).expect("structural rows lower");
        let rows = rows
            .iter()
            .filter(|row| {
                row.owner == "UniformValue"
                    && matches!(row.role.as_deref(), Some("maybe" | "items"))
            })
            .map(|row| {
                (
                    row.id.as_str(),
                    row.lhs.as_str(),
                    row.owner.as_str(),
                    row.role.as_deref(),
                    row.state.as_str(),
                    row.public_construction.as_deref(),
                    row.rhs
                        .iter()
                        .map(super::RuleSymbolPlan::test_label)
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>();

        assert_eq!(
            rows,
            [
                (
                    "UniformValueMaybeOptionalAbsent",
                    "UniformValueMaybeOptionalCategory",
                    "UniformValue",
                    Some("maybe"),
                    "optional_absent",
                    None,
                    vec![],
                ),
                (
                    "UniformValueMaybeOptionalPresent",
                    "UniformValueMaybeOptionalCategory",
                    "UniformValue",
                    Some("maybe"),
                    "optional_present",
                    None,
                    vec!["value:Item".to_owned()],
                ),
                (
                    "UniformValueItemsSequenceEmpty",
                    "UniformValueItemsSequenceCategory",
                    "UniformValue",
                    Some("items"),
                    "sequence_empty",
                    None,
                    vec![],
                ),
                (
                    "UniformValueItemsSequenceSingleton",
                    "UniformValueItemsSequenceCategory",
                    "UniformValue",
                    Some("items"),
                    "sequence_singleton",
                    None,
                    vec!["value:Item".to_owned(), "literal:<T>".to_owned()],
                ),
                (
                    "UniformValueItemsSequenceRecursive",
                    "UniformValueItemsSequenceCategory",
                    "UniformValue",
                    Some("items"),
                    "sequence_recursive",
                    None,
                    vec![
                        "value:Item".to_owned(),
                        "literal:<T>".to_owned(),
                        "literal:<S>".to_owned(),
                        "helper:UniformValueItemsSequenceCategory".to_owned(),
                    ],
                ),
            ],
            "the recursive row authenticates item, terminator, separator, tail order",
        );
    }

    #[test]
    fn structural_positional_rows_distinguish_owner_entry_families_and_tail_states() {
        let plan = structural_semantic_plan();
        let rows = super::lowered_rows(&plan).expect("positional rows lower");
        let rows = rows
            .iter()
            .filter(|row| row.owner == "PositionalValue")
            .map(|row| {
                (
                    row.id.as_str(),
                    row.lhs.as_str(),
                    row.role.as_deref(),
                    row.state.as_str(),
                    row.public_construction.as_deref(),
                    row.rhs
                        .iter()
                        .map(super::RuleSymbolPlan::test_label)
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>();

        assert_eq!(
            rows,
            [
                (
                    "PositionalValueItemsSequenceSingleton",
                    "Positional",
                    Some("items"),
                    "positional_singleton",
                    Some("PositionalPositional"),
                    vec!["value:Item".to_owned(), "literal:<T>".to_owned()],
                ),
                (
                    "PositionalValueItemsSequencePair",
                    "Positional",
                    Some("items"),
                    "positional_pair",
                    Some("PositionalPositional"),
                    vec![
                        "value:Item".to_owned(),
                        "literal:<T>".to_owned(),
                        "literal:<P>".to_owned(),
                        "value:Item".to_owned(),
                        "literal:<T>".to_owned(),
                    ],
                ),
                (
                    "PositionalValueItemsSequenceThreePlus",
                    "Positional",
                    Some("items"),
                    "positional_three_plus",
                    Some("PositionalPositional"),
                    vec![
                        "value:Item".to_owned(),
                        "literal:<T>".to_owned(),
                        "literal:<F>".to_owned(),
                        "helper:PositionalValueItemsSequenceCategory".to_owned(),
                    ],
                ),
                (
                    "PositionalValueItemsSequenceLast",
                    "PositionalValueItemsSequenceCategory",
                    Some("items"),
                    "positional_last",
                    None,
                    vec![
                        "value:Item".to_owned(),
                        "literal:<T>".to_owned(),
                        "literal:<L>".to_owned(),
                        "value:Item".to_owned(),
                        "literal:<T>".to_owned(),
                    ],
                ),
                (
                    "PositionalValueItemsSequenceMiddle",
                    "PositionalValueItemsSequenceCategory",
                    Some("items"),
                    "positional_middle",
                    None,
                    vec![
                        "value:Item".to_owned(),
                        "literal:<T>".to_owned(),
                        "literal:<M>".to_owned(),
                        "helper:PositionalValueItemsSequenceCategory".to_owned(),
                    ],
                ),
            ],
        );
    }

    #[test]
    fn generated_morphology_rules_keep_lexeme_identity_for_feature_instantiation() {
        let validated = crate::validate_declarations(
            crate::parse_declarations(crate::test_support::generated_morphology_tokens()).unwrap(),
        )
        .unwrap();
        let generated =
            super::emit(validated.semantic()).expect("generated morphology rules lower");
        let rules = generated.last().expect("rules item").tokens.to_string();
        for owner in [
            "LexicalOwnerTemplate :: Lexeme { declaration : \"VerbLexeme\" , member : \"InventedLemma\" }",
            "LexicalOwnerTemplate :: Lexeme { declaration : \"VerbLexeme\" , member : \"Be\" }",
        ] {
            assert!(
                rules.contains(owner),
                "missing feature-bearing owner template `{owner}`: {rules}"
            );
        }
        assert!(
            !rules.contains("stable_id : \"lexeme:VerbLexeme/"),
            "rule froze a featureless lexeme owner: {rules}"
        );
    }

    #[test]
    fn generated_morphology_rules_seal_exact_and_runtime_verb_constraints() {
        let runtime = crate::test_support::generated_morphology_expansion()
            .items()
            .iter()
            .find(|item| matches!(&item.key, crate::ItemKey::Named { name, .. } if name == "RULES"))
            .expect("generated morphology rules")
            .tokens
            .to_string();
        assert!(
            runtime.contains(
                "Lexical :: Verb (VerbLexeme :: InventedLemma , FeatureConstraint :: Any)"
            ),
            "runtime agreement did not lower to Any: {runtime}"
        );

        let exact = crate::test_support::representative_expansion()
            .items()
            .iter()
            .find(|item| matches!(&item.key, crate::ItemKey::Named { name, .. } if name == "RULES"))
            .expect("representative rules")
            .tokens
            .to_string();
        assert!(
            exact.contains(
                "Lexical :: Verb (Verbs :: Act , FeatureConstraint :: Exact (Agreement :: Bare))"
            ),
            "known agreement did not lower to Exact: {exact}"
        );
    }

    #[test]
    fn role_derived_noun_requests_either_number() {
        let validated = crate::validate_declarations(
            crate::parse_declarations(crate::test_support::role_derived_noun_tokens()).unwrap(),
        )
        .unwrap();
        let generated = super::emit(validated.semantic()).expect("role-derived noun rules lower");
        let rules = generated.last().expect("rules item").tokens.to_string();
        assert!(
            rules.contains("Lexical :: Noun (FeatureConstraint :: Any)"),
            "role-derived noun must let the scanner return either number: {rules}"
        );
    }

    #[test]
    fn vocab_matched_number_requests_either_for_every_noun() {
        let validated = crate::validate_declarations(
            crate::parse_declarations(
                crate::test_support::vocab_matched_number_with_two_nouns_tokens(),
            )
            .unwrap(),
        )
        .unwrap();
        let generated =
            super::emit(validated.semantic()).expect("two dynamic nouns lower to rules");
        let rules = generated.last().expect("rules item").tokens.to_string();
        assert_eq!(
            rules
                .matches("Lexical :: Noun (FeatureConstraint :: Any)")
                .count(),
            2,
            "each noun scanner is independently unconstrained until build: {rules}"
        );
    }

    fn expected_synthetic_projection_rules() -> syn::Item {
        syn::parse_quote! {
            pub(crate) const RULES: &[Rule<Category, LexicalTerminal, RuleId>] = &[
                Rule {
                    id: RuleId::ExprLeaf,
                    lhs: Category::Expr,
                    rhs: &[
                        L(LexicalTerminal {
                            matcher: Lexical::Mode,
                            owner: LexicalOwnerTemplate::Vocab { declaration: "Mode" },
                        }),
                        L(LexicalTerminal {
                            matcher: Lexical::Noun(FeatureConstraint::Any),
                            owner: LexicalOwnerTemplate::Static {
                                kind: LexicalProvenanceKind::Codec,
                                stable_id: "codec:Resource",
                            },
                        }),
                    ],
                },
                Rule {
                    id: RuleId::ExprNested,
                    lhs: Category::Expr,
                    rhs: &[
                        L(LexicalTerminal {
                            matcher: Lexical::Literal("nest"),
                            owner: LexicalOwnerTemplate::Static {
                                kind: LexicalProvenanceKind::FormLiteral,
                                stable_id: "form:nested/nested/0",
                            },
                        }),
                        N(Category::Expr),
                        L(LexicalTerminal {
                            matcher: Lexical::Marker,
                            owner: LexicalOwnerTemplate::Static {
                                kind: LexicalProvenanceKind::Codec,
                                stable_id: "codec:Marker",
                            },
                        }),
                    ],
                },
                Rule {
                    id: RuleId::PredicateAction,
                    lhs: Category::Predicate,
                    rhs: &[L(LexicalTerminal {
                        matcher: Lexical::Verb(
                            ActionStem::Activate,
                            FeatureConstraint::Any,
                        ),
                        owner: LexicalOwnerTemplate::Lexeme {
                            declaration: "ActionStem",
                            member: "Activate",
                        },
                    })],
                },
                Rule {
                    id: RuleId::PredicateIdle,
                    lhs: Category::Predicate,
                    rhs: &[L(LexicalTerminal {
                        matcher: Lexical::Literal("idle"),
                        owner: LexicalOwnerTemplate::Static {
                            kind: LexicalProvenanceKind::FormLiteral,
                            stable_id: "form:idle/idle/0",
                        },
                    })],
                },
                Rule {
                    id: RuleId::TagSolo,
                    lhs: Category::Tag,
                    rhs: &[L(LexicalTerminal {
                        matcher: Lexical::Mode,
                        owner: LexicalOwnerTemplate::Vocab { declaration: "Mode" },
                    })],
                },
                Rule {
                    id: RuleId::DocumentDocument,
                    lhs: Category::Document,
                    rhs: &[
                        N(Category::Expr),
                        N(Category::Predicate),
                        L(LexicalTerminal {
                            matcher: Lexical::Handle,
                            owner: LexicalOwnerTemplate::Static {
                                kind: LexicalProvenanceKind::Identity,
                                stable_id: "identity:Handle",
                            },
                        }),
                        L(LexicalTerminal {
                            matcher: Lexical::Pair,
                            owner: LexicalOwnerTemplate::Static {
                                kind: LexicalProvenanceKind::Codec,
                                stable_id: "codec:Pair",
                            },
                        }),
                        L(LexicalTerminal {
                            matcher: Lexical::Literal("!"),
                            owner: LexicalOwnerTemplate::Static {
                                kind: LexicalProvenanceKind::FormLiteral,
                                stable_id: "root:Document/punctuation",
                            },
                        }),
                        L(LexicalTerminal {
                            matcher: Lexical::EndOfInput,
                            owner: LexicalOwnerTemplate::None,
                        }),
                    ],
                },
            ];
        }
    }

    #[test]
    fn synthetic_projection_rules_have_exact_ids_rows_and_ownership() {
        let validated = crate::validate_declarations(
            crate::parse_declarations(crate::test_support::synthetic_projection_tokens()).unwrap(),
        )
        .unwrap();
        let generated = super::emit(validated.semantic()).unwrap();
        assert_eq!(generated.len(), 5);
        let actual = generated
            .iter()
            .map(|item| syn::parse2::<syn::Item>(item.tokens.clone()).unwrap())
            .collect::<Vec<_>>();

        let enum_variants = |item: &syn::Item| {
            let syn::Item::Enum(item) = item else {
                panic!("expected enum");
            };
            item.variants
                .iter()
                .map(|variant| variant.ident.to_string())
                .collect::<Vec<_>>()
        };
        assert_eq!(
            enum_variants(&actual[0]),
            ["Expr", "Predicate", "Tag", "Document"]
        );
        let ids = [
            "ExprLeaf",
            "ExprNested",
            "PredicateAction",
            "PredicateIdle",
            "TagSolo",
            "DocumentDocument",
        ];
        assert_eq!(enum_variants(&actual[1]), ids);
        assert_eq!(enum_variants(&actual[2]), ids);

        let normalize = |item: syn::Item| {
            prettyplease::unparse(&syn::File {
                shebang: None,
                attrs: Vec::new(),
                items: vec![item],
            })
        };
        assert_eq!(
            normalize(actual[4].clone()),
            normalize(expected_synthetic_projection_rules())
        );

        let construction_origins = ["leaf", "nested", "action", "idle", "solo", "document"]
            .map(|name| (crate::DeclarationKind::Construction, name));
        for item in &generated[..4] {
            assert_eq!(
                item.origins
                    .iter()
                    .map(|origin| (origin.kind(), origin.name()))
                    .collect::<Vec<_>>(),
                construction_origins,
            );
        }
        assert_eq!(
            generated[4]
                .origins
                .iter()
                .map(|origin| (origin.kind(), origin.name()))
                .collect::<Vec<_>>(),
            construction_origins
                .into_iter()
                .chain([(crate::DeclarationKind::Root, "Document")])
                .collect::<Vec<_>>(),
        );
    }
    #[test]
    fn root_eoi_and_number_equations_control_rule_projection() {
        let expansion = crate::generate(quote! {
            codec Object {
                atom = noun;
                value_type = Object;
                lexical = Lexical::Object;
                render = render_object;
                build { pattern = BuildValue::Object(object); construct = object; }
                traversal { callback = borrowed; argument = object; call visitor::visit_object(borrowed(object)); }
            }
            construction one: Phrase {
                element One { object: lex Object, }
                derive number = Values::Singular;
                form one = noun(object);
            }
            root Phrase { punctuation = "!"; eoi = true; standalone_render = true; }
            root Other { punctuation = "."; eoi = false; standalone_render = false; }
        });
        assert!(
            expansion.is_err(),
            "undeclared render-only roots remain invalid"
        );

        let validated = crate::validate_declarations(
            crate::parse_declarations(quote! {
                codec Object {
                    atom = noun;
                    value_type = Object;
                    lexical = Lexical::Object;
                    render = render_object;
                    build { pattern = BuildValue::Object(object); construct = object; }
                    traversal { callback = borrowed; argument = object; call visitor::visit_object(borrowed(object)); }
                }
                construction one: Phrase {
                    element One { object: lex Object, }
                    derive number = Values::Singular;
                    form one = noun(object);
                }
                root Phrase { punctuation = "!"; eoi = true; standalone_render = true; }
            }).unwrap(),
        ).unwrap();
        let items = super::emit(validated.semantic()).unwrap();
        let rules = items.last().unwrap().tokens.to_string();
        assert!(rules.contains("FeatureConstraint :: Exact (Number :: Singular)"));
        assert!(rules.contains("Lexical :: Literal (\"!\")"));
        assert!(rules.contains("Lexical :: EndOfInput"));
    }
}
