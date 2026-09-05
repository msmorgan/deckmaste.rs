use std::collections::HashMap;

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
use crate::plan::GeneratedItem;
use crate::plan::ItemKey;
use crate::plan::NamedKind;
use crate::plan::SourceDeclarationKind;
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
pub(super) enum SequenceOwnerState {
    UniformEmpty,
    UniformNonEmpty,
    PositionalEmpty,
    PositionalSingleton,
    PositionalPair,
    PositionalThreePlus,
    PositionalMinimumPlus(usize),
}

impl SequenceOwnerState {
    pub(super) fn spelling(self) -> String {
        match self {
            Self::UniformEmpty => "sequence_empty".to_owned(),
            Self::UniformNonEmpty => "sequence_non_empty".to_owned(),
            Self::PositionalEmpty => "positional_empty".to_owned(),
            Self::PositionalSingleton => "positional_singleton".to_owned(),
            Self::PositionalPair => "positional_pair".to_owned(),
            Self::PositionalThreePlus => "positional_three_plus".to_owned(),
            Self::PositionalMinimumPlus(length) => {
                format!("positional_minimum_{length}_plus")
            }
        }
    }

    fn suffix(self) -> String {
        match self {
            Self::UniformEmpty | Self::PositionalEmpty => "Empty".to_owned(),
            Self::UniformNonEmpty => "NonEmpty".to_owned(),
            Self::PositionalSingleton => "Singleton".to_owned(),
            Self::PositionalPair => "Pair".to_owned(),
            Self::PositionalThreePlus => "ThreePlus".to_owned(),
            Self::PositionalMinimumPlus(length) => format!("Minimum{length}Plus"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum OwnerFieldBuildState {
    ZeroableAbsent,
    ZeroablePresent,
    Sequence(SequenceOwnerState),
}

impl OwnerFieldBuildState {
    fn suffix(self) -> String {
        match self {
            Self::ZeroableAbsent => "Absent".to_owned(),
            Self::ZeroablePresent => "Present".to_owned(),
            Self::Sequence(state) => state.suffix(),
        }
    }

    fn spelling(self) -> String {
        match self {
            Self::ZeroableAbsent => "zeroable_absent".to_owned(),
            Self::ZeroablePresent => "zeroable_present".to_owned(),
            Self::Sequence(state) => state.spelling(),
        }
    }
}

#[derive(Debug, Clone)]
pub(super) enum RuleSymbolPlan {
    Authored {
        construction_index: usize,
        form_index: usize,
        atom_index: usize,
    },
    MarkedMarker {
        construction_index: usize,
        form_index: usize,
        atom_index: usize,
    },
    BoundAffix {
        construction_index: usize,
        form_index: usize,
        atom_index: usize,
    },
    CircumfixAffix {
        construction_index: usize,
        form_index: usize,
        atom_index: usize,
        side: CircumfixSide,
    },
    Value(ValueKindPlan),
    AdjacentValue(ValueKindPlan),
    Helper(String),
    Surface(StructuralSurfaceSymbolPlan),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CircumfixSide {
    Prefix,
    Suffix,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum StructuralSurfacePolicy {
    SeparatorUniform,
    SeparatorPositional(EdgeClass),
    Terminator,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum StructuralTransitionPlan {
    Preserve,
    SentenceInitial,
    Continuation,
}

#[derive(Debug, Clone)]
pub(super) struct StructuralSurfaceSymbolPlan {
    pub(super) atom: FixedSurfaceAtomPlan,
    pub(super) stable_id: String,
    pub(super) transition: StructuralTransitionPlan,
}

impl RuleSymbolPlan {
    #[cfg(test)]
    fn test_label(&self) -> String {
        match self {
            Self::Authored { .. } => "authored".to_owned(),
            Self::MarkedMarker { .. } => "marked-marker".to_owned(),
            Self::BoundAffix { .. } => "bound-affix".to_owned(),
            Self::CircumfixAffix { side, .. } => format!("circumfix-{side:?}"),
            Self::Value(value) => format!("value:{}", value_name(value)),
            Self::AdjacentValue(value) => format!("adjacent-value:{}", value_name(value)),
            Self::Helper(category) => format!("helper:{category}"),
            Self::Surface(StructuralSurfaceSymbolPlan {
                atom: FixedSurfaceAtomPlan::Literal(value),
                ..
            }) => {
                format!("literal:{value}")
            }
            Self::Surface(StructuralSurfaceSymbolPlan {
                atom: FixedSurfaceAtomPlan::Lex { terminal, variant },
                ..
            }) => {
                format!("lex:{terminal}::{variant}")
            }
        }
    }
}

#[derive(Debug, Clone)]
pub(super) enum RuleBuildPlan {
    Construction {
        index: usize,
        owner_states: Vec<OwnerFieldBuildPlan>,
    },
    Product {
        index: usize,
        owner_states: Vec<OwnerFieldBuildPlan>,
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
        helper_category: Option<String>,
    },
}

#[derive(Debug, Clone)]
pub(super) struct OwnerFieldBuildPlan {
    pub(super) role: String,
    pub(super) state: OwnerFieldBuildState,
    pub(super) rhs_start: usize,
    pub(super) rhs_end: usize,
    pub(super) helper_category: Option<String>,
}

type HelperCategoryNames<'a> =
    HashMap<(&'a str, &'a str, super::StructuralHelperCategoryState), &'a str>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum StructuralOwner {
    Construction(usize),
    Product(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SequenceBuildState {
    Singleton,
    Exact(usize),
    Recursive,
    Last,
    PositionalExactTail(usize),
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
    pub(super) form_index: Option<usize>,
    pub(super) form_name: Option<String>,
    pub(super) rhs: Vec<RuleSymbolPlan>,
    pub(super) build: RuleBuildPlan,
}

#[expect(
    clippy::too_many_lines,
    reason = "generated rule metadata and its exhaustive associated methods form one authority"
)]
pub(crate) fn emit(plan: &SemanticPlan) -> syn::Result<Vec<GeneratedItem>> {
    let category_type = ident(RULE_CATEGORY_TYPE);
    let construction_type = ident(RULE_CONSTRUCTION_TYPE);
    let rule_id_type = ident(RULE_ID_TYPE);
    let rule_id_count = ident(RULE_ID_COUNT);
    let rule_id_public_construction = ident("public_construction");
    let rule_id_form_name = ident("form_name");
    let rule_id_metric_name = ident("metric_name");
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
    let construction_name_matches =
        constructions
            .iter()
            .zip(&construction_ids)
            .map(|(construction, construction_id)| {
                let name = syn::LitStr::new(construction.rule_id(), Span::call_site());
                quote! { Self::#construction_id => #name }
            });
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
    let metric_name_matches = lowered.iter().zip(&rule_ids).map(|(row, rule_id)| {
        let name = syn::LitStr::new(
            row.public_construction.as_deref().unwrap_or(&row.owner),
            Span::call_site(),
        );
        quote! { Self::#rule_id => #name }
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
    let form_name_matches = lowered.iter().zip(&rule_ids).map(|(row, rule_id)| {
        let value = row.form_name.as_ref().map_or_else(
            || quote! { None },
            |name| {
                let name = syn::LitStr::new(name, Span::call_site());
                quote! { Some(#name) }
            },
        );
        quote! { Self::#rule_id => #value }
    });
    let rows = lowered
        .iter()
        .zip(&rule_ids)
        .map(|(row, rule_id)| emit_rule(plan, row, rule_id))
        .collect::<syn::Result<Vec<_>>>()?;
    let root_adapter = emit_root_adapter(plan);

    let mut rule_origins = origins.clone();
    rule_origins.extend(
        plan.roots()
            .iter()
            .filter(|root| root.is_parse_entry())
            .map(|root| DeclarationKey::new(SourceDeclarationKind::Root, root.category())),
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
            ItemKey::Impl {
                trait_name: None,
                self_ty: RULE_CONSTRUCTION_TYPE.to_owned(),
            },
            quote! {
                impl Construction {
                    pub(crate) const fn name(self) -> &'static str {
                        match self { #(#construction_name_matches,)* }
                    }
                }
            },
            origins.clone(),
        ),
        root_adapter,
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
                    pub(crate) const fn #rule_id_form_name(self) -> Option<&'static str> {
                        match self { #(#form_name_matches,)* }
                    }
                    #[cfg(feature = "parser-metrics")]
                    pub(crate) const fn #rule_id_metric_name(self) -> &'static str {
                        match self { #(#metric_name_matches,)* }
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

fn emit_root_adapter(plan: &SemanticPlan) -> GeneratedItem {
    let root_adapters = plan
        .roots()
        .iter()
        .map(|root| {
            let category = ident(root.category());
            let eoi = plan.parse_root(root.category()).is_some();
            let mut terminals = Vec::new();
            if !root.punctuation().is_empty() {
                let punctuation = syn::LitStr::new(root.punctuation(), Span::call_site());
                let stable_id = syn::LitStr::new(
                    &format!("root:{}/punctuation", root.category()),
                    Span::call_site(),
                );
                terminals.push(quote! {
                    LexicalTerminal {
                        matcher: Lexical::Literal(#punctuation),
                        owner:
                        LexicalOwnerTemplate::Static {
                            kind: LexicalProvenanceKind::FormLiteral,
                            stable_id: #stable_id,
                        },
                        right_boundary: LexicalBoundary::LeftAdjacent,
                    }
                });
            }
            if eoi {
                terminals.push(quote! {
                    LexicalTerminal {
                        matcher: Lexical::EndOfInput,
                        owner: LexicalOwnerTemplate::None,
                        right_boundary: LexicalBoundary::Separated,
                    }
                });
            }
            (category, eoi, terminals)
        })
        .collect::<Vec<_>>();
    let adapter_arms = root_adapters.iter().map(|(category, eoi, terminals)| {
        quote! { (Self::#category, #eoi) => &[#(#terminals),*] }
    });
    let mut root_rule_arms = Vec::new();
    for (category, eoi, terminals) in &root_adapters {
        let adapter = terminals.iter().map(|terminal| quote! { L(#terminal) });
        root_rule_arms.push(quote! {
            (Self::#category, #eoi) => &[
                RulePosition::Nonterminal(Self::#category),
                #(#adapter),*
            ]
        });
    }
    let eoi_arms = plan.roots().iter().map(|root| {
        let category = ident(root.category());
        let eoi = root.is_parse_entry();
        quote! { Self::#category => Some(#eoi) }
    });
    GeneratedItem::new(
        ItemKey::Impl {
            trait_name: None,
            self_ty: RULE_CATEGORY_TYPE.to_owned(),
        },
        quote! {
            impl Category {
                pub(crate) const fn root_adapter(self, eoi: bool) -> &'static [LexicalTerminal] {
                    match (self, eoi) {
                        #(#adapter_arms,)*
                        _ => &[],
                    }
                }

                pub(crate) const fn root_rule_rhs(
                    self,
                    eoi: bool,
                ) -> &'static [RulePosition<Category, LexicalTerminal>] {
                    match (self, eoi) {
                        #(#root_rule_arms,)*
                        _ => &[],
                    }
                }

                pub(crate) const fn root_eoi(self) -> Option<bool> {
                    match self {
                        #(#eoi_arms,)*
                        _ => None,
                    }
                }
            }
        },
        plan.roots()
            .iter()
            .map(|root| DeclarationKey::new(SourceDeclarationKind::Root, root.category()))
            .collect(),
    )
}

fn emit_rule(
    plan: &SemanticPlan,
    row: &RuleRowPlan,
    rule_id: &syn::Ident,
) -> syn::Result<TokenStream> {
    let lhs = ident(&row.lhs);
    let rhs = row
        .rhs
        .iter()
        .map(|symbol| emit_rule_symbol(plan, symbol))
        .collect::<syn::Result<Vec<_>>>()?;
    Ok(quote! { Rule { id: RuleId::#rule_id, lhs: Category::#lhs, rhs: &[#(#rhs),*] } })
}

fn emit_rule_symbol(plan: &SemanticPlan, symbol: &RuleSymbolPlan) -> syn::Result<TokenStream> {
    match symbol {
        RuleSymbolPlan::Authored {
            construction_index,
            form_index,
            atom_index,
        } => {
            let construction = &plan.constructions()[*construction_index];
            emit_position(
                plan,
                construction,
                construction.forms()[*form_index].name(),
                *atom_index,
                &construction.forms()[*form_index].atoms()[*atom_index],
            )
        }
        RuleSymbolPlan::MarkedMarker {
            construction_index,
            form_index,
            atom_index,
        } => {
            let construction = &plan.constructions()[*construction_index];
            let AtomPlan::Marked {
                terminal,
                variant,
                path,
                ..
            } = &construction.forms()[*form_index].atoms()[*atom_index]
            else {
                return Err(internal("marked-marker symbol does not name a marked atom"));
            };
            let marker = AtomPlan::LexFixed {
                terminal: terminal.clone(),
                variant: variant.clone(),
                path: path.clone(),
            };
            emit_position(
                plan,
                construction,
                construction.forms()[*form_index].name(),
                *atom_index,
                &marker,
            )
        }
        RuleSymbolPlan::BoundAffix {
            construction_index,
            form_index,
            atom_index,
        } => emit_bound_affix_position(
            &plan.constructions()[*construction_index],
            *form_index,
            *atom_index,
        ),
        RuleSymbolPlan::CircumfixAffix {
            construction_index,
            form_index,
            atom_index,
            side,
        } => emit_circumfix_affix_position(
            &plan.constructions()[*construction_index],
            *form_index,
            *atom_index,
            *side,
        ),
        RuleSymbolPlan::Value(value) => emit_value_position(plan, value, false),
        RuleSymbolPlan::AdjacentValue(value) => emit_value_position(plan, value, true),
        RuleSymbolPlan::Helper(category) => {
            let category = ident(category);
            Ok(quote! { N(Category::#category) })
        }
        RuleSymbolPlan::Surface(StructuralSurfaceSymbolPlan {
            atom: FixedSurfaceAtomPlan::Literal(value),
            stable_id,
            transition,
        }) => {
            let value = syn::LitStr::new(value, Span::call_site());
            let stable_id = syn::LitStr::new(stable_id, Span::call_site());
            let transition = emit_structural_transition(*transition);
            Ok(lexical_terminal(
                &quote! { Lexical::Literal(#value) },
                &quote! {
                    LexicalOwnerTemplate::Structural {
                        stable_id: #stable_id,
                        transition: #transition,
                    }
                },
            ))
        }
        RuleSymbolPlan::Surface(StructuralSurfaceSymbolPlan {
            atom: FixedSurfaceAtomPlan::Lex { terminal, .. },
            stable_id,
            transition,
        }) => {
            let lexical = lexical_variant(plan, terminal)?;
            let stable_id = syn::LitStr::new(stable_id, Span::call_site());
            let transition = emit_structural_transition(*transition);
            Ok(lexical_terminal(
                &lexical,
                &quote! {
                    LexicalOwnerTemplate::Structural {
                        stable_id: #stable_id,
                        transition: #transition,
                    }
                },
            ))
        }
    }
}

fn emit_value_position(
    plan: &SemanticPlan,
    value: &ValueKindPlan,
    right_adjacent: bool,
) -> syn::Result<TokenStream> {
    match value {
        ValueKindPlan::Category(name) | ValueKindPlan::Product(name) | ValueKindPlan::Sum(name) => {
            let name = ident(name);
            Ok(if right_adjacent {
                quote! { RulePosition::AdjacentNonterminal(Category::#name) }
            } else {
                quote! { N(Category::#name) }
            })
        }
        ValueKindPlan::Lex(name) | ValueKindPlan::Identity(name) => {
            let lexical = lexical_variant(plan, name)?;
            let owner = owner_template(plan, name)?;
            Ok(if right_adjacent {
                lexical_terminal_with_boundary(
                    &lexical,
                    &owner,
                    &quote! { LexicalBoundary::Adjacent },
                )
            } else {
                lexical_terminal(&lexical, &owner)
            })
        }
    }
}

pub(super) fn lowered_rows(plan: &SemanticPlan) -> syn::Result<Vec<RuleRowPlan>> {
    let mut rows = Vec::new();
    let helper_categories = super::structural_helper_categories(plan);
    let helper_category_names = helper_categories
        .iter()
        .map(|category| {
            (
                (category.owner, category.field.name(), category.state),
                category.name.as_str(),
            )
        })
        .collect::<HelperCategoryNames<'_>>();
    for (index, construction) in plan.constructions().iter().enumerate() {
        let marked_roles = construction
            .forms()
            .iter()
            .enumerate()
            .flat_map(|(form_index, form)| {
                form.atoms()
                    .iter()
                    .enumerate()
                    .filter_map(move |(atom_index, atom)| match atom {
                        AtomPlan::Marked { role, .. } => Some((
                            role.clone(),
                            RuleSymbolPlan::MarkedMarker {
                                construction_index: index,
                                form_index,
                                atom_index,
                            },
                        )),
                        _ => None,
                    })
            })
            .collect::<HashMap<_, _>>();
        rows.extend(lower_construction_rows(
            index,
            construction,
            plan.explicit_sum_owns_construction_category(construction.category()),
        )?);
        rows.extend(lower_helper_rows(
            StructuralOwner::Construction(index),
            construction.element_type(),
            construction
                .fields()
                .iter()
                .enumerate()
                .filter_map(|(index, field)| field.structural_plan().map(|field| (index, field))),
            &marked_roles,
            &helper_category_names,
        )?);
    }
    for (index, product) in plan.products().iter().enumerate() {
        rows.extend(lower_product_rows(index, product)?);
        rows.extend(lower_helper_rows(
            StructuralOwner::Product(index),
            product.name(),
            product.fields().iter().enumerate(),
            &HashMap::new(),
            &helper_category_names,
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
                form_index: None,
                form_name: None,
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

fn structural_field_for_atom<'a>(
    construction: &'a ConstructionPlan,
    atom: &AtomPlan,
) -> syn::Result<Option<&'a StructuralFieldPlan>> {
    let structural = atom_role(atom).and_then(|role| {
        construction
            .fields()
            .iter()
            .find(|field| field.name_key() == role)
            .and_then(crate::semantic::ConstructionFieldPlan::structural_plan)
    });
    if structural.is_some() && matches!(atom, AtomPlan::Bound { .. }) {
        return Err(internal("bound atom reached structural role lowering"));
    }
    Ok(structural)
}

fn lower_construction_rows(
    construction_index: usize,
    construction: &ConstructionPlan,
    explicit_sum_owned: bool,
) -> syn::Result<Vec<RuleRowPlan>> {
    let mut rows = Vec::new();
    for (form_index, form) in construction.forms().iter().enumerate() {
        let mut variants = vec![(
            Vec::<OwnerFieldBuildPlan>::new(),
            Vec::<RuleSymbolPlan>::new(),
        )];
        for (atom_index, atom) in form.atoms().iter().enumerate() {
            let structural = structural_field_for_atom(construction, atom)?;
            let Some(field) = structural else {
                for (_, rhs) in &mut variants {
                    if let AtomPlan::Marked { category, .. } = atom {
                        rhs.extend([
                            RuleSymbolPlan::MarkedMarker {
                                construction_index,
                                form_index,
                                atom_index,
                            },
                            RuleSymbolPlan::Value(ValueKindPlan::Category(category.clone())),
                        ]);
                        continue;
                    }
                    let authored = RuleSymbolPlan::Authored {
                        construction_index,
                        form_index,
                        atom_index,
                    };
                    let affix = RuleSymbolPlan::BoundAffix {
                        construction_index,
                        form_index,
                        atom_index,
                    };
                    let circumfix = |side| RuleSymbolPlan::CircumfixAffix {
                        construction_index,
                        form_index,
                        atom_index,
                        side,
                    };
                    match atom {
                        AtomPlan::Bound {
                            direction: crate::semantic::BoundDirectionPlan::Prefix,
                            ..
                        } => rhs.extend([affix, authored]),
                        AtomPlan::Bound {
                            direction: crate::semantic::BoundDirectionPlan::Suffix,
                            ..
                        } => rhs.extend([authored, affix]),
                        AtomPlan::Circumfix { .. } => rhs.extend([
                            circumfix(CircumfixSide::Prefix),
                            authored,
                            circumfix(CircumfixSide::Suffix),
                        ]),
                        _ => rhs.push(authored),
                    }
                }
                continue;
            };
            let field_variants = owner_field_variants(construction.element_type(), field)?;
            if matches!(atom, AtomPlan::Marked { .. })
                && !matches!(field.kind(), StructuralFieldKindPlan::Optional(_))
            {
                variants = combine_marked_owner_variants(
                    variants,
                    field,
                    &field_variants,
                    &RuleSymbolPlan::MarkedMarker {
                        construction_index,
                        form_index,
                        atom_index,
                    },
                );
            } else if matches!(atom, AtomPlan::Circumfix { .. }) {
                for (_, rhs) in &mut variants {
                    rhs.push(RuleSymbolPlan::CircumfixAffix {
                        construction_index,
                        form_index,
                        atom_index,
                        side: CircumfixSide::Prefix,
                    });
                }
                variants = combine_owner_variants(variants, field, &field_variants);
                for (_, rhs) in &mut variants {
                    rhs.push(RuleSymbolPlan::CircumfixAffix {
                        construction_index,
                        form_index,
                        atom_index,
                        side: CircumfixSide::Suffix,
                    });
                }
            } else {
                variants = combine_owner_variants(variants, field, &field_variants);
            }
        }
        rows.extend(
            variants
                .into_iter()
                .map(|(owner_states, rhs)| {
                    let base_rule_id = if explicit_sum_owned {
                        let base = format!(
                            "{}Construction",
                            crate::identifier::pascal_case(construction.element_type())
                        );
                        if construction.forms().len() == 1 {
                            base
                        } else {
                            format!("{base}{}", crate::identifier::pascal_case(form.name()))
                        }
                    } else {
                        form.rule_id().to_owned()
                    };
                    let (id, role, state) = public_variant_metadata(
                        &base_rule_id,
                        construction.element_type(),
                        construction.fields().iter().filter_map(|field| {
                            field
                                .structural_plan()
                                .map(|structural| (field.name_key(), structural))
                        }),
                        &owner_states,
                    )?;
                    Ok(RuleRowPlan {
                        id,
                        lhs: if explicit_sum_owned {
                            construction.element_type().to_owned()
                        } else {
                            construction.category().to_owned()
                        },
                        owner: construction.element_type().to_owned(),
                        role,
                        state,
                        public_construction: Some(construction.rule_id().to_owned()),
                        form_index: Some(form_index),
                        form_name: (construction.forms().len() > 1).then(|| form.name().to_owned()),
                        rhs,
                        build: RuleBuildPlan::Construction {
                            index: construction_index,
                            owner_states,
                        },
                    })
                })
                .collect::<syn::Result<Vec<_>>>()?,
        );
    }
    Ok(rows)
}

fn combine_marked_owner_variants(
    current: Vec<(Vec<OwnerFieldBuildPlan>, Vec<RuleSymbolPlan>)>,
    field: &StructuralFieldPlan,
    field_variants: &[(Option<OwnerFieldBuildState>, Vec<RuleSymbolPlan>)],
    marker: &RuleSymbolPlan,
) -> Vec<(Vec<OwnerFieldBuildPlan>, Vec<RuleSymbolPlan>)> {
    current
        .into_iter()
        .flat_map(|(states, rhs)| {
            let marker = marker.clone();
            field_variants.iter().map(move |(state, field_rhs)| {
                let mut states = states.clone();
                let mut combined = rhs.clone();
                if !matches!(state, Some(OwnerFieldBuildState::ZeroableAbsent)) {
                    combined.push(marker.clone());
                }
                let rhs_start = combined.len();
                combined.extend(field_rhs.iter().cloned());
                if let Some(state) = state {
                    let helper_category = field_rhs.iter().find_map(|symbol| match symbol {
                        RuleSymbolPlan::Helper(category) => Some(category.clone()),
                        _ => None,
                    });
                    states.push(OwnerFieldBuildPlan {
                        role: field.name().to_owned(),
                        state: *state,
                        rhs_start,
                        rhs_end: combined.len(),
                        helper_category,
                    });
                }
                (states, combined)
            })
        })
        .collect()
}

fn lower_product_rows(
    product_index: usize,
    product: &crate::semantic::ProductPlan,
) -> syn::Result<Vec<RuleRowPlan>> {
    let mut variants = vec![(
        Vec::<OwnerFieldBuildPlan>::new(),
        Vec::<RuleSymbolPlan>::new(),
    )];
    for field in product.fields() {
        let field_variants = owner_field_variants(product.name(), field)?;
        variants = combine_owner_variants(variants, field, &field_variants);
    }
    variants
        .into_iter()
        .map(|(owner_states, rhs)| {
            let base = format!("{}Product", crate::identifier::pascal_case(product.name()));
            let (id, role, state) = public_variant_metadata(
                &base,
                product.name(),
                product
                    .fields()
                    .iter()
                    .map(|field| (field.name().to_owned(), field)),
                &owner_states,
            )?;
            Ok(RuleRowPlan {
                id,
                lhs: product.name().to_owned(),
                owner: product.name().to_owned(),
                role,
                state,
                public_construction: None,
                form_index: None,
                form_name: None,
                rhs,
                build: RuleBuildPlan::Product {
                    index: product_index,
                    owner_states,
                },
            })
        })
        .collect()
}

fn public_variant_metadata<'a>(
    base: &str,
    _owner: &str,
    fields: impl Iterator<Item = (String, &'a StructuralFieldPlan)>,
    owner_states: &[OwnerFieldBuildPlan],
) -> syn::Result<(String, Option<String>, String)> {
    if owner_states.is_empty() {
        return Ok((base.to_owned(), None, "public".to_owned()));
    }
    if owner_states.len() == 1 {
        let OwnerFieldBuildPlan { role, state, .. } = &owner_states[0];
        let field = fields
            .into_iter()
            .find(|(candidate, _)| candidate == role)
            .map(|(_, field)| field)
            .ok_or_else(|| internal("owner state has no structural field"))?;
        let aggregate = match state {
            OwnerFieldBuildState::ZeroableAbsent | OwnerFieldBuildState::ZeroablePresent => {
                format!("{base}{}", crate::identifier::pascal_case(role))
            }
            OwnerFieldBuildState::Sequence(_) => field
                .helper_names()
                .ok_or_else(|| internal("positional sequence has no helper inventory"))?
                .all()[0]
                .to_owned(),
        };
        return Ok((
            format!("{aggregate}{}", state.suffix()),
            Some(role.clone()),
            state.spelling(),
        ));
    }
    let mut suffix = String::new();
    for field in owner_states {
        suffix.push_str(&crate::identifier::pascal_case(&field.role));
        suffix.push_str(&field.state.suffix());
    }
    Ok((
        format!("{base}{suffix}"),
        None,
        "structural_product".to_owned(),
    ))
}

fn combine_owner_variants(
    current: Vec<(Vec<OwnerFieldBuildPlan>, Vec<RuleSymbolPlan>)>,
    field: &StructuralFieldPlan,
    field_variants: &[(Option<OwnerFieldBuildState>, Vec<RuleSymbolPlan>)],
) -> Vec<(Vec<OwnerFieldBuildPlan>, Vec<RuleSymbolPlan>)> {
    current
        .into_iter()
        .flat_map(|(states, rhs)| {
            field_variants.iter().map(move |(state, field_rhs)| {
                let mut states = states.clone();
                let mut combined = rhs.clone();
                let rhs_start = combined.len();
                combined.extend(field_rhs.iter().cloned());
                if let Some(state) = state {
                    let helper_category = field_rhs.iter().find_map(|symbol| match symbol {
                        RuleSymbolPlan::Helper(category) => Some(category.clone()),
                        _ => None,
                    });
                    states.push(OwnerFieldBuildPlan {
                        role: field.name().to_owned(),
                        state: *state,
                        rhs_start,
                        rhs_end: combined.len(),
                        helper_category,
                    });
                }
                (states, combined)
            })
        })
        .collect()
}

fn owner_field_variants(
    owner: &str,
    field: &StructuralFieldPlan,
) -> syn::Result<Vec<(Option<OwnerFieldBuildState>, Vec<RuleSymbolPlan>)>> {
    match field.kind() {
        StructuralFieldKindPlan::Required(value) => {
            Ok(vec![(None, vec![RuleSymbolPlan::Value(value.clone())])])
        }
        StructuralFieldKindPlan::Zeroable(value) => Ok(vec![
            (Some(OwnerFieldBuildState::ZeroableAbsent), Vec::new()),
            (
                Some(OwnerFieldBuildState::ZeroablePresent),
                vec![RuleSymbolPlan::Value(value.clone())],
            ),
        ]),
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
                    variants.push((
                        Some(OwnerFieldBuildState::Sequence(
                            SequenceOwnerState::PositionalEmpty,
                        )),
                        Vec::new(),
                    ));
                }
                if bounds.allows(1) {
                    variants.push((
                        Some(OwnerFieldBuildState::Sequence(
                            SequenceOwnerState::PositionalSingleton,
                        )),
                        item_with_terminator(item, surface.terminator(), owner, field.name()),
                    ));
                }
                if bounds.allows(2) {
                    variants.push((
                        Some(OwnerFieldBuildState::Sequence(
                            SequenceOwnerState::PositionalPair,
                        )),
                        exact_positional_rhs(item, surface, rows, 2, owner, field.name())?,
                    ));
                }
                let minimum = bounds.min().max(3);
                if bounds.max().is_none_or(|maximum| maximum >= minimum) {
                    let state = if minimum == 3 {
                        SequenceOwnerState::PositionalThreePlus
                    } else {
                        SequenceOwnerState::PositionalMinimumPlus(minimum)
                    };
                    let mut rhs =
                        item_with_terminator(item, surface.terminator(), owner, field.name());
                    extend_surface_symbols(
                        &mut rhs,
                        positional_surface(rows, EdgeClass::First)?,
                        owner,
                        field.name(),
                        StructuralSurfacePolicy::SeparatorPositional(EdgeClass::First),
                    );
                    rhs.push(RuleSymbolPlan::Helper(helper_category(owner, field)?));
                    variants.push((Some(OwnerFieldBuildState::Sequence(state)), rhs));
                }
                Ok(variants)
            }
            Some(SeparatorPlan::Uniform(_)) | None => {
                if bounds.max().is_none() {
                    if bounds.min() == 0 {
                        return Ok(vec![
                            (
                                Some(OwnerFieldBuildState::Sequence(
                                    SequenceOwnerState::UniformEmpty,
                                )),
                                Vec::new(),
                            ),
                            (
                                Some(OwnerFieldBuildState::Sequence(
                                    SequenceOwnerState::UniformNonEmpty,
                                )),
                                vec![RuleSymbolPlan::Helper(helper_category(owner, field)?)],
                            ),
                        ]);
                    }
                    return Ok(vec![(
                        None,
                        vec![RuleSymbolPlan::Helper(helper_category(owner, field)?)],
                    )]);
                }

                let mut variants = Vec::new();
                if bounds.allows(0) {
                    variants.push((
                        Some(OwnerFieldBuildState::Sequence(
                            SequenceOwnerState::UniformEmpty,
                        )),
                        Vec::new(),
                    ));
                }
                if bounds
                    .max()
                    .is_some_and(|maximum| maximum >= bounds.min().max(1))
                {
                    variants.push((
                        Some(OwnerFieldBuildState::Sequence(
                            SequenceOwnerState::UniformNonEmpty,
                        )),
                        vec![RuleSymbolPlan::Helper(helper_category(owner, field)?)],
                    ));
                }
                Ok(variants)
            }
        },
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "helper lowering exhaustively separates positional and uniform reachable states"
)]
fn lower_helper_rows<'a>(
    owner_key: StructuralOwner,
    owner: &str,
    fields: impl Iterator<Item = (usize, &'a StructuralFieldPlan)>,
    marked_roles: &HashMap<String, RuleSymbolPlan>,
    helper_categories: &HelperCategoryNames<'_>,
) -> syn::Result<Vec<RuleRowPlan>> {
    let mut rows = Vec::new();
    for (field_index, field) in fields {
        if matches!(
            field.kind(),
            StructuralFieldKindPlan::Required(_) | StructuralFieldKindPlan::Zeroable(_)
        ) {
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
            StructuralFieldKindPlan::Required(_) | StructuralFieldKindPlan::Zeroable(_) => {}
            StructuralFieldKindPlan::Optional(value) => {
                for (suffix, state, present, rhs) in [
                    ("Absent", "optional_absent", false, Vec::new()),
                    (
                        "Present",
                        "optional_present",
                        true,
                        marked_roles
                            .get(field.name())
                            .cloned()
                            .into_iter()
                            .chain(std::iter::once(RuleSymbolPlan::Value(value.clone())))
                            .collect(),
                    ),
                ] {
                    rows.push(RuleRowPlan {
                        id: format!("{aggregate}{suffix}"),
                        lhs: category.clone(),
                        owner: owner.to_owned(),
                        role: role.clone(),
                        state: state.to_owned(),
                        public_construction: None,
                        form_index: None,
                        form_name: None,
                        rhs,
                        build: RuleBuildPlan::Optional {
                            owner: owner_key,
                            field_index,
                            present,
                        },
                    });
                }
            }
            StructuralFieldKindPlan::Sequence {
                item,
                bounds,
                surface,
            } => match surface.separator() {
                Some(SeparatorPlan::Positional(positional)) => {
                    if let Some(maximum) = bounds.max() {
                        for position in 2..maximum {
                            let lhs = counted_helper_category(
                                helper_categories,
                                owner,
                                field.name(),
                                super::StructuralHelperCategoryState::PositionalCount(position),
                            )?;
                            let total = position + 1;
                            if bounds.allows(total) {
                                let mut last = item_with_terminator(
                                    item,
                                    surface.terminator(),
                                    owner,
                                    field.name(),
                                );
                                extend_surface_symbols(
                                    &mut last,
                                    positional_surface(positional, EdgeClass::Last)?,
                                    owner,
                                    field.name(),
                                    StructuralSurfacePolicy::SeparatorPositional(EdgeClass::Last),
                                );
                                last.extend(item_with_terminator(
                                    item,
                                    surface.terminator(),
                                    owner,
                                    field.name(),
                                ));
                                rows.push(sequence_helper_row(
                                    owner_key,
                                    field_index,
                                    owner,
                                    field.name(),
                                    &aggregate,
                                    &lhs,
                                    &format!("Count{position}Last"),
                                    &format!("positional_count_{position}_last"),
                                    SequenceBuildState::Last,
                                    last,
                                ));
                            }
                            if total < maximum {
                                let next = counted_helper_category(
                                    helper_categories,
                                    owner,
                                    field.name(),
                                    super::StructuralHelperCategoryState::PositionalCount(
                                        position + 1,
                                    ),
                                )?;
                                let mut middle = item_with_terminator(
                                    item,
                                    surface.terminator(),
                                    owner,
                                    field.name(),
                                );
                                extend_surface_symbols(
                                    &mut middle,
                                    positional_surface(positional, EdgeClass::Middle)?,
                                    owner,
                                    field.name(),
                                    StructuralSurfacePolicy::SeparatorPositional(EdgeClass::Middle),
                                );
                                middle.push(RuleSymbolPlan::Helper(next));
                                rows.push(sequence_helper_row(
                                    owner_key,
                                    field_index,
                                    owner,
                                    field.name(),
                                    &aggregate,
                                    &lhs,
                                    &format!("Count{position}Middle"),
                                    &format!("positional_count_{position}_middle"),
                                    SequenceBuildState::Middle,
                                    middle,
                                ));
                            }
                        }
                    } else if bounds.allows_at_least(3) {
                        let tail_length = bounds.min().max(3) - 1;
                        let (suffix, state, build_state) = if tail_length == 2 {
                            (
                                "Last".to_owned(),
                                "positional_last".to_owned(),
                                SequenceBuildState::Last,
                            )
                        } else {
                            (
                                format!("TailLength{tail_length}"),
                                format!("positional_tail_length_{tail_length}"),
                                SequenceBuildState::PositionalExactTail(tail_length),
                            )
                        };
                        rows.push(sequence_helper_row(
                            owner_key,
                            field_index,
                            owner,
                            field.name(),
                            &aggregate,
                            &category,
                            &suffix,
                            &state,
                            build_state,
                            exact_positional_tail_rhs(
                                item,
                                surface,
                                positional,
                                tail_length,
                                owner,
                                field.name(),
                            )?,
                        ));
                        let mut middle =
                            item_with_terminator(item, surface.terminator(), owner, field.name());
                        extend_surface_symbols(
                            &mut middle,
                            positional_surface(positional, EdgeClass::Middle)?,
                            owner,
                            field.name(),
                            StructuralSurfacePolicy::SeparatorPositional(EdgeClass::Middle),
                        );
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
                    if let Some(maximum) = bounds.max() {
                        for count in 1..=maximum {
                            let lhs = counted_helper_category(
                                helper_categories,
                                owner,
                                field.name(),
                                super::StructuralHelperCategoryState::UniformCount(count),
                            )?;
                            if bounds.allows(count) {
                                rows.push(sequence_helper_row(
                                    owner_key,
                                    field_index,
                                    owner,
                                    field.name(),
                                    &aggregate,
                                    &lhs,
                                    &format!("Count{count}Final"),
                                    &format!("sequence_count_{count}_final"),
                                    SequenceBuildState::Singleton,
                                    item_with_terminator(
                                        item,
                                        surface.terminator(),
                                        owner,
                                        field.name(),
                                    ),
                                ));
                            }
                            if count < maximum {
                                let next = counted_helper_category(
                                    helper_categories,
                                    owner,
                                    field.name(),
                                    super::StructuralHelperCategoryState::UniformCount(count + 1),
                                )?;
                                let mut recursive = item_with_terminator(
                                    item,
                                    surface.terminator(),
                                    owner,
                                    field.name(),
                                );
                                if let Some(SeparatorPlan::Uniform(separator)) = uniform {
                                    extend_surface_symbols(
                                        &mut recursive,
                                        separator,
                                        owner,
                                        field.name(),
                                        StructuralSurfacePolicy::SeparatorUniform,
                                    );
                                }
                                recursive.push(RuleSymbolPlan::Helper(next));
                                rows.push(sequence_helper_row(
                                    owner_key,
                                    field_index,
                                    owner,
                                    field.name(),
                                    &aggregate,
                                    &lhs,
                                    &format!("Count{count}Continue"),
                                    &format!("sequence_count_{count}_continue"),
                                    SequenceBuildState::Recursive,
                                    recursive,
                                ));
                            }
                        }
                    } else {
                        let base_length = bounds.min().max(1);
                        let (suffix, state, build_state) = if base_length == 1 {
                            (
                                "Singleton".to_owned(),
                                "sequence_singleton".to_owned(),
                                SequenceBuildState::Singleton,
                            )
                        } else {
                            (
                                format!("Length{base_length}"),
                                format!("sequence_length_{base_length}"),
                                SequenceBuildState::Exact(base_length),
                            )
                        };
                        rows.push(sequence_helper_row(
                            owner_key,
                            field_index,
                            owner,
                            field.name(),
                            &aggregate,
                            &category,
                            &suffix,
                            &state,
                            build_state,
                            exact_uniform_rhs(
                                item,
                                surface,
                                uniform,
                                base_length,
                                owner,
                                field.name(),
                            ),
                        ));
                        let mut recursive =
                            item_with_terminator(item, surface.terminator(), owner, field.name());
                        if let Some(SeparatorPlan::Uniform(separator)) = uniform {
                            extend_surface_symbols(
                                &mut recursive,
                                separator,
                                owner,
                                field.name(),
                                StructuralSurfacePolicy::SeparatorUniform,
                            );
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
                }
            },
        }
    }
    Ok(rows)
}

#[allow(
    clippy::too_many_arguments,
    reason = "one helper row carries its complete diagnostic and build identity"
)]
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
    let helper_category = rhs.iter().find_map(|symbol| match symbol {
        RuleSymbolPlan::Helper(category) => Some(category.clone()),
        _ => None,
    });
    RuleRowPlan {
        id: format!("{aggregate}{suffix}"),
        lhs: category.to_owned(),
        owner: owner.to_owned(),
        role: Some(role.to_owned()),
        state: state.to_owned(),
        public_construction: None,
        form_index: None,
        form_name: None,
        rhs,
        build: RuleBuildPlan::Sequence {
            owner: owner_key,
            field_index,
            state: build_state,
            helper_category,
        },
    }
}

fn exact_uniform_rhs(
    item: &ValueKindPlan,
    surface: &crate::semantic::SequenceSurfacePlan,
    separator: Option<&SeparatorPlan>,
    length: usize,
    owner: &str,
    role: &str,
) -> Vec<RuleSymbolPlan> {
    let mut rhs = Vec::new();
    for index in 0..length {
        if index > 0
            && let Some(SeparatorPlan::Uniform(separator)) = separator
        {
            extend_surface_symbols(
                &mut rhs,
                separator,
                owner,
                role,
                StructuralSurfacePolicy::SeparatorUniform,
            );
        }
        rhs.extend(item_with_terminator(
            item,
            surface.terminator(),
            owner,
            role,
        ));
    }
    rhs
}

fn exact_positional_rhs(
    item: &ValueKindPlan,
    surface: &crate::semantic::SequenceSurfacePlan,
    rows: &[crate::semantic::PositionalSeparatorPlan],
    length: usize,
    owner: &str,
    role: &str,
) -> syn::Result<Vec<RuleSymbolPlan>> {
    let mut rhs = Vec::new();
    for index in 0..length {
        if index > 0 {
            let class = match length {
                2 => EdgeClass::Pair,
                _ if index == 1 => EdgeClass::First,
                _ if index + 1 == length => EdgeClass::Last,
                _ => EdgeClass::Middle,
            };
            extend_surface_symbols(
                &mut rhs,
                positional_surface(rows, class)?,
                owner,
                role,
                StructuralSurfacePolicy::SeparatorPositional(class),
            );
        }
        rhs.extend(item_with_terminator(
            item,
            surface.terminator(),
            owner,
            role,
        ));
    }
    Ok(rhs)
}

fn exact_positional_tail_rhs(
    item: &ValueKindPlan,
    surface: &crate::semantic::SequenceSurfacePlan,
    rows: &[crate::semantic::PositionalSeparatorPlan],
    length: usize,
    owner: &str,
    role: &str,
) -> syn::Result<Vec<RuleSymbolPlan>> {
    let mut rhs = Vec::new();
    for index in 0..length {
        if index > 0 {
            let class = if index + 1 == length { EdgeClass::Last } else { EdgeClass::Middle };
            extend_surface_symbols(
                &mut rhs,
                positional_surface(rows, class)?,
                owner,
                role,
                StructuralSurfacePolicy::SeparatorPositional(class),
            );
        }
        rhs.extend(item_with_terminator(
            item,
            surface.terminator(),
            owner,
            role,
        ));
    }
    Ok(rhs)
}

fn item_with_terminator(
    item: &ValueKindPlan,
    terminator: Option<&FixedSurfacePlan>,
    owner: &str,
    role: &str,
) -> Vec<RuleSymbolPlan> {
    let mut symbols = vec![RuleSymbolPlan::Value(item.clone())];
    if let Some(terminator) = terminator {
        extend_surface_symbols(
            &mut symbols,
            terminator,
            owner,
            role,
            StructuralSurfacePolicy::Terminator,
        );
    }
    symbols
}

fn extend_surface_symbols(
    symbols: &mut Vec<RuleSymbolPlan>,
    surface: &FixedSurfacePlan,
    owner: &str,
    role: &str,
    policy: StructuralSurfacePolicy,
) {
    let adjacent_value = surface_starts_left_adjacent(surface)
        .then(|| match symbols.last() {
            Some(RuleSymbolPlan::Value(value)) => Some(value.clone()),
            _ => None,
        })
        .flatten();
    if let Some(value) = adjacent_value {
        *symbols.last_mut().expect("last value is present") = RuleSymbolPlan::AdjacentValue(value);
    }
    symbols.extend(surface_symbols(surface, owner, role, policy));
}

fn surface_starts_left_adjacent(surface: &FixedSurfacePlan) -> bool {
    surface.atoms().iter().find_map(|atom| match atom {
        FixedSurfaceAtomPlan::Literal(value) if value.is_empty() => None,
        FixedSurfaceAtomPlan::Literal(value) => Some(
            value
                .chars()
                .next()
                .is_some_and(|character| !character.is_whitespace()),
        ),
        FixedSurfaceAtomPlan::Lex { .. } => Some(false),
    }) == Some(true)
}

fn surface_symbols(
    surface: &FixedSurfacePlan,
    owner: &str,
    role: &str,
    policy: StructuralSurfacePolicy,
) -> Vec<RuleSymbolPlan> {
    surface
        .atoms()
        .iter()
        .cloned()
        .enumerate()
        .map(|(atom_index, atom)| {
            RuleSymbolPlan::Surface(StructuralSurfaceSymbolPlan {
                atom,
                stable_id: structural_surface_stable_id(owner, role, policy, atom_index),
                transition: structural_surface_transition(surface, policy, atom_index),
            })
        })
        .collect()
}

pub(super) fn fixed_surface_terminates_sentence(surface: &FixedSurfacePlan) -> bool {
    surface.atoms().iter().rev().find_map(|atom| match atom {
        FixedSurfaceAtomPlan::Literal(value) => {
            let significant = value.trim_end_matches(char::is_whitespace);
            (!significant.is_empty()).then(|| significant.ends_with('.'))
        }
        FixedSurfaceAtomPlan::Lex { .. } => Some(false),
    }) == Some(true)
}

pub(super) fn structural_surface_transition(
    surface: &FixedSurfacePlan,
    policy: StructuralSurfacePolicy,
    atom_index: usize,
) -> StructuralTransitionPlan {
    use crate::model::SurfaceCaseTransition;

    let final_atom = atom_index.checked_add(1) == Some(surface.atoms().len());
    if !final_atom {
        return StructuralTransitionPlan::Preserve;
    }
    match surface.transition() {
        SurfaceCaseTransition::Continuation => StructuralTransitionPlan::Continuation,
        SurfaceCaseTransition::SentenceInitial => StructuralTransitionPlan::SentenceInitial,
        SurfaceCaseTransition::Preserve
            if policy == StructuralSurfacePolicy::Terminator
                && fixed_surface_terminates_sentence(surface) =>
        {
            StructuralTransitionPlan::SentenceInitial
        }
        SurfaceCaseTransition::Preserve => StructuralTransitionPlan::Preserve,
    }
}

pub(super) fn emit_structural_transition(transition: StructuralTransitionPlan) -> TokenStream {
    match transition {
        StructuralTransitionPlan::Preserve => quote! { StructuralTransition::Preserve },
        StructuralTransitionPlan::SentenceInitial => {
            quote! { StructuralTransition::SentenceInitial }
        }
        StructuralTransitionPlan::Continuation => quote! { StructuralTransition::Continuation },
    }
}

pub(super) fn structural_surface_stable_id(
    owner: &str,
    role: &str,
    policy: StructuralSurfacePolicy,
    atom_index: usize,
) -> String {
    let policy = match policy {
        StructuralSurfacePolicy::SeparatorUniform => "separator/uniform",
        StructuralSurfacePolicy::SeparatorPositional(EdgeClass::Pair) => "separator/pair",
        StructuralSurfacePolicy::SeparatorPositional(EdgeClass::First) => "separator/first",
        StructuralSurfacePolicy::SeparatorPositional(EdgeClass::Middle) => "separator/middle",
        StructuralSurfacePolicy::SeparatorPositional(EdgeClass::Last) => "separator/last",
        StructuralSurfacePolicy::Terminator => "terminator",
    };
    format!("structural:{owner}/{role}/{policy}/{atom_index}")
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

fn counted_helper_category(
    categories: &HelperCategoryNames<'_>,
    owner: &str,
    role: &str,
    state: super::StructuralHelperCategoryState,
) -> syn::Result<String> {
    categories
        .get(&(owner, role, state))
        .map(|name| (*name).to_owned())
        .ok_or_else(|| internal("counted sequence row has no sealed helper category"))
}

fn atom_role(atom: &AtomPlan) -> Option<&str> {
    match atom.value_atom() {
        AtomPlan::Category { role, .. }
        | AtomPlan::Lex { role, .. }
        | AtomPlan::Marked { role, .. }
        | AtomPlan::Identity { role, .. }
        | AtomPlan::Noun { role, .. } => Some(role),
        AtomPlan::Literal(_)
        | AtomPlan::SentenceInitialLiteral(_)
        | AtomPlan::StructuralLiteral(_)
        | AtomPlan::LexFixed { .. }
        | AtomPlan::VerbFixed { .. }
        | AtomPlan::OpenDeclaration(_)
        | AtomPlan::Bound { .. }
        | AtomPlan::Circumfix { .. } => None,
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

fn form_literal_owner(
    stable_id: &syn::LitStr,
    transition: Option<StructuralTransitionPlan>,
) -> TokenStream {
    if let Some(transition) = transition {
        let transition = emit_structural_transition(transition);
        quote! {
            LexicalOwnerTemplate::TransitionedStatic {
                kind: LexicalProvenanceKind::FormLiteral,
                stable_id: #stable_id,
                transition: #transition,
            }
        }
    } else {
        quote! {
            LexicalOwnerTemplate::Static {
                kind: LexicalProvenanceKind::FormLiteral,
                stable_id: #stable_id,
            }
        }
    }
}

fn form_literal_transition(atom: &AtomPlan) -> Option<StructuralTransitionPlan> {
    match atom {
        AtomPlan::Literal(_) => None,
        AtomPlan::SentenceInitialLiteral(_) => Some(StructuralTransitionPlan::SentenceInitial),
        AtomPlan::StructuralLiteral(_) => Some(StructuralTransitionPlan::Preserve),
        _ => unreachable!("form literal branch contains only form literals"),
    }
}

fn lex_position(
    plan: &SemanticPlan,
    construction: &ConstructionPlan,
    role: &str,
    terminal: &str,
    right_boundary: &TokenStream,
) -> syn::Result<TokenStream> {
    if let AtomTerminal::DeclarationVerb {
        terminal_index,
        plan: codec,
    } = plan.atom_terminal(terminal)?
    {
        let matcher = match codec.feature_axis() {
            Feature::ConcordClass => {
                let concord_class = declaration_verb_feature(plan, construction, role)?;
                quote! { Lexical::DeclarationVerb(#terminal_index, #concord_class) }
            }
            Feature::Participle => quote! { Lexical::DeclarationParticiple(#terminal_index) },
            _ => unreachable!("validated declaration verb feature axis is closed"),
        };
        return Ok(lexical_terminal_with_boundary(
            &matcher,
            &quote! { LexicalOwnerTemplate::DeclarationVerb(#terminal_index) },
            right_boundary,
        ));
    }
    let lexical = lexical_variant(plan, terminal)?;
    let owner = owner_template(plan, terminal)?;
    Ok(lexical_terminal_with_boundary(
        &lexical,
        &owner,
        right_boundary,
    ))
}

fn emit_position(
    plan: &SemanticPlan,
    construction: &ConstructionPlan,
    form_name: &str,
    atom_index: usize,
    atom: &AtomPlan,
) -> syn::Result<TokenStream> {
    let adjacent_value = matches!(
        atom,
        AtomPlan::Bound {
            direction: crate::semantic::BoundDirectionPlan::Suffix,
            ..
        } | AtomPlan::Circumfix { .. }
    );
    let right_boundary = if adjacent_value {
        quote! { LexicalBoundary::Adjacent }
    } else {
        quote! { LexicalBoundary::Separated }
    };
    let atom = atom.value_atom();
    if let Some(role) = atom_role(atom)
        && let Some(structural) = construction
            .fields()
            .iter()
            .find(|field| field.name_key() == role)
            .and_then(crate::semantic::ConstructionFieldPlan::structural_plan)
    {
        match structural.kind() {
            StructuralFieldKindPlan::Zeroable(_) => {
                return Err(internal("zeroable atom reached the unexpanded public rule"));
            }
            StructuralFieldKindPlan::Optional(_) => {
                let category = ident(&helper_category(construction.element_type(), structural)?);
                return Ok(if adjacent_value {
                    quote! { RulePosition::AdjacentNonterminal(Category::#category) }
                } else {
                    quote! { N(Category::#category) }
                });
            }
            StructuralFieldKindPlan::Sequence { surface, .. } => {
                if matches!(surface.separator(), Some(SeparatorPlan::Positional(_))) {
                    return Err(internal(
                        "positional sequence atom reached the unexpanded public rule",
                    ));
                }
                let category = ident(&helper_category(construction.element_type(), structural)?);
                return Ok(if adjacent_value {
                    quote! { RulePosition::AdjacentNonterminal(Category::#category) }
                } else {
                    quote! { N(Category::#category) }
                });
            }
            StructuralFieldKindPlan::Required(_) => {}
        }
    }
    match atom {
        AtomPlan::Literal(literal)
        | AtomPlan::SentenceInitialLiteral(literal)
        | AtomPlan::StructuralLiteral(literal) => {
            let literal = syn::LitStr::new(literal, Span::call_site());
            let stable_id = syn::LitStr::new(
                &format!(
                    "form:{}/{}/{}",
                    construction.construction_id(),
                    form_name,
                    atom_index
                ),
                Span::call_site(),
            );
            let owner = form_literal_owner(&stable_id, form_literal_transition(atom));
            Ok(lexical_terminal_with_boundary(
                &quote! { Lexical::Literal(#literal) },
                &owner,
                &right_boundary,
            ))
        }
        AtomPlan::Category { category, .. } => {
            let category = ident(category);
            if adjacent_value {
                Ok(quote! { RulePosition::AdjacentNonterminal(Category::#category) })
            } else {
                Ok(quote! { N(Category::#category) })
            }
        }
        AtomPlan::Lex { role, terminal } => {
            lex_position(plan, construction, role, terminal, &right_boundary)
        }
        AtomPlan::Marked { terminal, .. }
        | AtomPlan::LexFixed { terminal, .. }
        | AtomPlan::Identity { terminal, .. } => {
            let lexical = lexical_variant(plan, terminal)?;
            let owner = owner_template(plan, terminal)?;
            Ok(lexical_terminal_with_boundary(
                &lexical,
                &owner,
                &right_boundary,
            ))
        }
        AtomPlan::Noun { role, terminal } => {
            let number = noun_number(plan, construction, role)?;
            let owner = owner_template(plan, terminal)?;
            let matcher = if let AtomTerminal::DeclarationNoun { terminal_index, .. } =
                plan.atom_terminal(terminal)?
            {
                quote! { Lexical::DeclarationNoun(#terminal_index, #number) }
            } else {
                let lexical = lexical_variant(plan, terminal)?;
                quote! { #lexical(#number) }
            };
            Ok(lexical_terminal_with_boundary(
                &matcher,
                &owner,
                &right_boundary,
            ))
        }
        AtomPlan::VerbFixed {
            terminal, variant, ..
        } => {
            let terminal = ident(terminal);
            let variant = ident(variant);
            let concord_class = closed_verb_feature(plan, construction)?;
            let declaration = syn::LitStr::new(&terminal.to_string(), Span::call_site());
            let member = syn::LitStr::new(&variant.to_string(), Span::call_site());
            Ok(lexical_terminal_with_boundary(
                &quote! { Lexical::Verb(#terminal::#variant, #concord_class) },
                &quote! {
                    LexicalOwnerTemplate::Lexeme { declaration: #declaration, member: #member }
                },
                &right_boundary,
            ))
        }
        AtomPlan::OpenDeclaration(open) => {
            let kind = crate::emit::declaration_kind(open.kind());
            let name = syn::LitStr::new(open.name(), Span::call_site());
            let position = crate::emit::grammar_position(open.position());
            let feature = open_verb_feature(plan, construction)?;
            Ok(lexical_terminal_with_boundary(
                &quote! {
                    Lexical::Declaration(DeclarationMatcher {
                        kind: #kind,
                        name: #name,
                        position: #position,
                        feature: #feature,
                    })
                },
                &quote! { LexicalOwnerTemplate::Declaration { kind: #kind, name: #name } },
                &right_boundary,
            ))
        }
        AtomPlan::Bound { .. } | AtomPlan::Circumfix { .. } => {
            unreachable!("value_atom removes form wrappers")
        }
    }
}

fn emit_bound_affix_position(
    construction: &ConstructionPlan,
    form_index: usize,
    atom_index: usize,
) -> syn::Result<TokenStream> {
    let form = &construction.forms()[form_index];
    let AtomPlan::Bound {
        direction, affix, ..
    } = &form.atoms()[atom_index]
    else {
        return Err(internal(
            "bound-affix rule symbol references an ordinary atom",
        ));
    };
    let literal = syn::LitStr::new(affix, Span::call_site());
    let stable_id = syn::LitStr::new(
        &format!(
            "form:{}/{}/{atom_index}/affix",
            construction.construction_id(),
            form.name(),
        ),
        Span::call_site(),
    );
    let boundary = match direction {
        crate::semantic::BoundDirectionPlan::Prefix => quote! { LexicalBoundary::Adjacent },
        crate::semantic::BoundDirectionPlan::Suffix => quote! { LexicalBoundary::LeftAdjacent },
    };
    Ok(lexical_terminal_with_boundary(
        &quote! { Lexical::Literal(#literal) },
        &quote! {
            LexicalOwnerTemplate::Static {
                kind: LexicalProvenanceKind::FormLiteral,
                stable_id: #stable_id,
            }
        },
        &boundary,
    ))
}

fn emit_circumfix_affix_position(
    construction: &ConstructionPlan,
    form_index: usize,
    atom_index: usize,
    side: CircumfixSide,
) -> syn::Result<TokenStream> {
    let form = &construction.forms()[form_index];
    let AtomPlan::Circumfix { prefix, suffix, .. } = &form.atoms()[atom_index] else {
        return Err(internal(
            "circumfix-affix rule symbol references an ordinary atom",
        ));
    };
    let (surface, side_name, boundary) = match side {
        CircumfixSide::Prefix => (prefix, "prefix", quote! { LexicalBoundary::Adjacent }),
        CircumfixSide::Suffix => (suffix, "suffix", quote! { LexicalBoundary::LeftAdjacent }),
    };
    let literal = syn::LitStr::new(surface, Span::call_site());
    let stable_id = syn::LitStr::new(
        &format!(
            "form:{}/{}/{atom_index}/{side_name}",
            construction.construction_id(),
            form.name(),
        ),
        Span::call_site(),
    );
    Ok(lexical_terminal_with_boundary(
        &quote! { Lexical::Literal(#literal) },
        &quote! {
            LexicalOwnerTemplate::Static {
                kind: LexicalProvenanceKind::FormLiteral,
                stable_id: #stable_id,
            }
        },
        &boundary,
    ))
}

fn closed_verb_feature(
    plan: &SemanticPlan,
    construction: &ConstructionPlan,
) -> syn::Result<TokenStream> {
    let target = FeaturePlace::Role {
        field: syn::Ident::new("verb", construction.origin_span()),
        feature: Feature::ConcordClass,
    };
    match plan.feature_resolution(construction.construction_id(), &target) {
        Some(crate::feature::FeatureResolution::Known(value)) => match value {
            FeatureValue::ConcordOther => Ok(quote! {
                FeatureConstraint::Exact(ConcordClass::Other)
            }),
            FeatureValue::ThirdPersonSingular => Ok(quote! {
                FeatureConstraint::Exact(ConcordClass::ThirdPersonSingular)
            }),
            _ => Err(internal(
                "closed verb concord_class has a non-concord_class value",
            )),
        },
        Some(
            crate::feature::FeatureResolution::External
            | crate::feature::FeatureResolution::Runtime,
        ) => Ok(quote! { FeatureConstraint::Any }),
        None => Err(internal(
            "closed verb has no sealed concord_class resolution",
        )),
    }
}

fn declaration_verb_feature(
    plan: &SemanticPlan,
    construction: &ConstructionPlan,
    role: &str,
) -> syn::Result<TokenStream> {
    let target = FeaturePlace::Role {
        field: syn::Ident::new(role, construction.origin_span()),
        feature: Feature::ConcordClass,
    };
    match plan.feature_resolution(construction.construction_id(), &target) {
        Some(crate::feature::FeatureResolution::Known(value)) => match value {
            FeatureValue::ConcordOther => {
                Ok(quote! { FeatureConstraint::Exact(ConcordClass::Other) })
            }
            FeatureValue::ThirdPersonSingular => {
                Ok(quote! { FeatureConstraint::Exact(ConcordClass::ThirdPersonSingular) })
            }
            _ => Err(internal(
                "declaration verb concord_class has a non-concord_class value",
            )),
        },
        Some(
            crate::feature::FeatureResolution::External
            | crate::feature::FeatureResolution::Runtime,
        ) => Ok(quote! { FeatureConstraint::Any }),
        None => Err(internal(
            "declaration verb has no sealed ConcordClass resolution",
        )),
    }
}

pub(crate) fn open_verb_feature(
    plan: &SemanticPlan,
    construction: &ConstructionPlan,
) -> syn::Result<TokenStream> {
    let target = FeaturePlace::Role {
        field: syn::Ident::new("verb", construction.origin_span()),
        feature: Feature::ConcordClass,
    };
    match plan.feature_resolution(construction.construction_id(), &target) {
        Some(crate::feature::FeatureResolution::Known(value)) => match value {
            FeatureValue::ConcordOther => Ok(quote! {
                FeatureConstraint::Exact(::deckmaste_construction_core::macro_def::SurfaceFeature::PLAIN)
            }),
            FeatureValue::ThirdPersonSingular => Ok(quote! {
                FeatureConstraint::Exact(::deckmaste_construction_core::macro_def::SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT)
            }),
            _ => Err(internal(
                "open verb concord_class has a non-concord_class value",
            )),
        },
        Some(
            crate::feature::FeatureResolution::External
            | crate::feature::FeatureResolution::Runtime,
        ) => Ok(quote! { FeatureConstraint::Any }),
        None => Err(internal("open verb has no sealed concord_class resolution")),
    }
}

fn lexical_terminal(matcher: &TokenStream, owner: &TokenStream) -> TokenStream {
    lexical_terminal_with_boundary(matcher, owner, &quote! { LexicalBoundary::Separated })
}

fn lexical_terminal_with_boundary(
    matcher: &TokenStream,
    owner: &TokenStream,
    right_boundary: &TokenStream,
) -> TokenStream {
    quote! {
        L(LexicalTerminal {
            matcher: #matcher,
            owner: #owner,
            right_boundary: #right_boundary,
        })
    }
}

fn owner_template(plan: &SemanticPlan, terminal: &str) -> syn::Result<TokenStream> {
    let declaration = syn::LitStr::new(terminal, Span::call_site());
    match plan.atom_terminal(terminal)? {
        AtomTerminal::Vocab(_) => Ok(quote! {
            LexicalOwnerTemplate::Vocab { declaration: #declaration }
        }),
        AtomTerminal::Lexeme(_) => Ok(quote! { LexicalOwnerTemplate::NounLexeme }),
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
        AtomTerminal::CatalogIdentity { terminal_index, .. } => {
            Ok(quote! { LexicalOwnerTemplate::CatalogIdentity(#terminal_index) })
        }
        AtomTerminal::SignedDecimal(_) | AtomTerminal::UnsignedNumber(_) => {
            let stable_id = syn::LitStr::new(&format!("codec:{terminal}"), Span::call_site());
            Ok(quote! {
                LexicalOwnerTemplate::Static {
                    kind: LexicalProvenanceKind::Codec,
                    stable_id: #stable_id,
                }
            })
        }
        AtomTerminal::DeclarationNoun {
            terminal_index,
            plan,
        } => {
            debug_assert_eq!(plan.position(), crate::macro_def::GrammarPosition::Noun);
            Ok(quote! { LexicalOwnerTemplate::DeclarationNoun(#terminal_index) })
        }
        AtomTerminal::DeclarationDeterminative { terminal_index, .. } => {
            Ok(quote! { LexicalOwnerTemplate::DeclarationDeterminative(#terminal_index) })
        }
        AtomTerminal::DeclarationTerm { terminal_index, .. } => {
            Ok(quote! { LexicalOwnerTemplate::DeclarationTerm(#terminal_index) })
        }
        AtomTerminal::DeclarationVerb { terminal_index, .. } => {
            Ok(quote! { LexicalOwnerTemplate::DeclarationVerb(#terminal_index) })
        }
    }
}

fn lexical_variant(plan: &SemanticPlan, name: &str) -> syn::Result<TokenStream> {
    match plan.atom_terminal(name)? {
        AtomTerminal::Vocab(vocab) => {
            let name = ident(vocab.name());
            Ok(quote! { Lexical::#name })
        }
        AtomTerminal::Lexeme(_) => Ok(quote! { Lexical::Noun }),
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
        AtomTerminal::CatalogIdentity { terminal_index, .. } => {
            Ok(quote! { Lexical::CatalogIdentity(#terminal_index) })
        }
        AtomTerminal::SignedDecimal(codec) => {
            let variant = codec.codec_ident();
            Ok(quote! { Lexical::#variant })
        }
        AtomTerminal::UnsignedNumber(codec) => {
            let variant = codec.codec_ident();
            Ok(quote! { Lexical::#variant })
        }
        AtomTerminal::DeclarationNoun { .. } => Err(internal(
            "declaration noun lexical matcher requires its sealed index",
        )),
        AtomTerminal::DeclarationDeterminative { terminal_index, .. } => {
            Ok(quote! { Lexical::DeclarationDeterminative(#terminal_index) })
        }
        AtomTerminal::DeclarationTerm { terminal_index, .. } => {
            Ok(quote! { Lexical::DeclarationTerm(#terminal_index) })
        }
        AtomTerminal::DeclarationVerb { .. } => Err(internal(
            "declaration verb lexical matcher requires its sealed index and ConcordClass",
        )),
    }
}

fn noun_number(
    plan: &SemanticPlan,
    construction: &ConstructionPlan,
    role: &str,
) -> syn::Result<TokenStream> {
    let equations = plan.feature_equations(construction.construction_id());
    let role_target = FeaturePlace::Role {
        field: syn::Ident::new(role, construction.origin_span()),
        feature: Feature::Number,
    };
    let target = if equations
        .iter()
        .any(|equation| equation.target() == &role_target)
    {
        role_target
    } else {
        FeaturePlace::Construction(Feature::Number)
    };
    let equation = equations
        .iter()
        .find(|equation| equation.target() == &target)
        .ok_or_else(|| internal("noun atom has no validated construction number"))?;
    match equation.value() {
        FeatureExpr::Constant(value) => match value.value() {
            FeatureValue::Singular => Ok(quote! { FeatureConstraint::Exact(Number::Singular) }),
            FeatureValue::Plural => Ok(quote! { FeatureConstraint::Exact(Number::Plural) }),
            _ => Err(internal("noun number has a non-number value")),
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
                SourceDeclarationKind::Construction,
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
            super::structural_helper_categories(plan)
                .into_iter()
                .map(|category| ident(&category.name)),
        )
        .collect()
}

fn category_origins(plan: &SemanticPlan) -> Vec<DeclarationKey> {
    plan.declaration_keys()
        .iter()
        .filter(|origin| {
            matches!(
                origin.kind(),
                SourceDeclarationKind::Construction
                    | SourceDeclarationKind::AbstractProduct
                    | SourceDeclarationKind::AbstractSum
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
    use std::collections::HashSet;

    use quote::quote;

    fn guarded_forms_plan() -> crate::semantic::SemanticPlan {
        crate::validate_declarations(
            crate::parse_declarations(quote! {
                vocab Word { That = "that", Those = "those", Other = "other", }
                construction demonstrative: NounPhrase {
                    element Demonstrative { word: lex Word, }
                    form that when word is That = "that" lex(word);
                    form those when word is Those = "those" lex(word);
                    form fallback otherwise = "other" lex(word);
                }
                root NounPhrase { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect("guarded emitter fixture parses"),
        )
        .expect("guarded emitter fixture validates")
        .into_semantic()
    }

    #[test]
    fn guarded_forms_lower_distinct_authored_rules_with_shared_construction_authority() {
        let plan = guarded_forms_plan();
        let rows = super::lowered_rows(&plan).expect("guarded rows lower");
        let authored = rows
            .iter()
            .filter(|row| row.public_construction.is_some())
            .map(|row| {
                (
                    row.id.as_str(),
                    row.public_construction.as_deref(),
                    row.form_name.as_deref(),
                    row.rhs
                        .iter()
                        .map(super::RuleSymbolPlan::test_label)
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(
            authored,
            [
                (
                    "NounPhraseDemonstrativeThat",
                    Some("NounPhraseDemonstrative"),
                    Some("that"),
                    vec!["authored".to_owned(), "authored".to_owned()],
                ),
                (
                    "NounPhraseDemonstrativeThose",
                    Some("NounPhraseDemonstrative"),
                    Some("those"),
                    vec!["authored".to_owned(), "authored".to_owned()],
                ),
                (
                    "NounPhraseDemonstrativeFallback",
                    Some("NounPhraseDemonstrative"),
                    Some("fallback"),
                    vec!["authored".to_owned(), "authored".to_owned()],
                ),
            ]
        );

        let source = super::emit(&plan)
            .expect("guarded rules emit")
            .into_iter()
            .map(|item| item.tokens.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        assert!(source.contains("const fn name"), "{source}");
        assert!(source.contains("const fn form_name"), "{source}");
    }

    #[test]
    fn mobile_role_metadata_contributes_no_rule_position_specificity_tier_or_helper_rule() {
        let plan = crate::validate_declarations(
            crate::parse_declarations(quote! {
                construction child: Child {
                    element ChildNode {}
                    form child = "child";
                }
                construction plain: PlainRoot {
                    element PlainHost { tail: Child, }
                    form plain = tail;
                }
                construction mobile: MobileRoot {
                    element MobileHost { tail: mobile Child, }
                    form mobile = tail;
                }
                root PlainRoot { punctuation = "."; eoi = true; standalone_render = true; }
                root MobileRoot { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect("mobile rule fixture parses"),
        )
        .expect("mobile rule fixture validates")
        .into_semantic();
        let rows = super::lowered_rows(&plan).expect("mobile rule fixture lowers");
        let public_rhs = |construction| {
            rows.iter()
                .find(|row| row.public_construction.as_deref() == Some(construction))
                .expect("construction has one public rule")
                .rhs
                .iter()
                .map(super::RuleSymbolPlan::test_label)
                .collect::<Vec<_>>()
        };
        assert_eq!(public_rhs("PlainRootPlain"), ["authored"]);
        assert_eq!(public_rhs("MobileRootMobile"), ["authored"]);
        assert_eq!(
            rows.iter()
                .filter(|row| row.public_construction.is_some())
                .count(),
            3,
        );
        assert!(
            rows.iter()
                .all(|row| { !row.id.contains("Admissible") && !row.owner.contains("Admissible") }),
            "the derived slot has no helper rule",
        );
    }

    #[derive(Debug, Clone, Copy)]
    struct LoweringSize {
        rows: usize,
        symbols: usize,
        names: usize,
    }

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
    fn bound_structural_roles_trip_the_defensive_lowering_invariant() {
        let plan = structural_semantic_plan();
        let construction = plan
            .constructions()
            .iter()
            .find(|construction| construction.element_type() == "UniformValue")
            .expect("fixture has a structural construction");
        let atom = crate::semantic::AtomPlan::Bound {
            direction: crate::semantic::BoundDirectionPlan::Prefix,
            affix: "non".to_owned(),
            value: Box::new(crate::semantic::AtomPlan::Category {
                role: "maybe".to_owned(),
                category: "Item".to_owned(),
            }),
        };

        let error = super::structural_field_for_atom(construction, &atom)
            .expect_err("validated bound atoms must never enter structural lowering")
            .to_string();
        assert!(
            error.contains("bound atom reached structural role lowering"),
            "{error}"
        );
    }

    #[test]
    fn circumfix_rows_wrap_but_do_not_replace_structural_sequence_authority() {
        let plan = crate::validate_declarations(
            crate::parse_declarations(quote! {
                construction item: Item {
                    element ItemValue {}
                    form item = "item";
                }
                construction singular: Root {
                    element Singular { value: Item, }
                    form singular = circumfix("[", value, "]");
                }
                construction sequence: Root {
                    element Sequence { values: seq Item separated by "}{", }
                    require len(values) >= 1;
                    form sequence = circumfix("{", values, "}");
                }
                root Root { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect("circumfix rule fixture parses"),
        )
        .expect("circumfix rule fixture validates")
        .into_semantic();
        let rows = super::lowered_rows(&plan).expect("circumfix rows lower");
        let labels = |id: &str| {
            rows.iter()
                .find(|row| row.id == id)
                .unwrap_or_else(|| panic!("missing rule row {id}"))
                .rhs
                .iter()
                .map(super::RuleSymbolPlan::test_label)
                .collect::<Vec<_>>()
        };

        assert_eq!(
            labels("RootSingular"),
            ["circumfix-Prefix", "authored", "circumfix-Suffix"],
        );
        assert_eq!(
            labels("RootSequence"),
            [
                "circumfix-Prefix",
                "helper:SequenceValuesSequenceCategory",
                "circumfix-Suffix",
            ],
        );
        assert_eq!(labels("SequenceValuesSequenceSingleton"), ["value:Item"],);
        assert_eq!(
            labels("SequenceValuesSequenceRecursive"),
            [
                "adjacent-value:Item",
                "literal:}{",
                "helper:SequenceValuesSequenceCategory",
            ],
        );
        let recursive = rows
            .iter()
            .find(|row| row.id == "SequenceValuesSequenceRecursive")
            .expect("recursive helper exists");
        assert!(matches!(
            &recursive.rhs[1],
            super::RuleSymbolPlan::Surface(super::StructuralSurfaceSymbolPlan {
                stable_id,
                ..
            }) if stable_id == "structural:Sequence/values/separator/uniform/0"
        ));
    }

    fn bounded_sequence_semantic_plan() -> crate::semantic::SemanticPlan {
        crate::validate_declarations(
            crate::parse_declarations(quote! {
                vocab Marker { Alpha = "alpha", Beta = "beta", Gamma = "gamma", Delta = "delta", }
                construction item: Item {
                    element ItemValue { marker: lex Marker, }
                    form item = lex(marker);
                }
                construction exact_pair: ExactPair {
                    element ExactPairValue {
                        items: seq Item separated by position { pair = "<P>"; } terminated by "<T>",
                    }
                    require len(items) = 2;
                    form exact_pair = items;
                }
                construction bounded_uniform: BoundedUniform {
                    element BoundedUniformValue {
                        items: seq Item separated by "<S>" terminated by "<T>",
                    }
                    require len(items) >= 2;
                    require len(items) <= 4;
                    form bounded_uniform = items;
                }
                construction bounded_positional: BoundedPositional {
                    element BoundedPositionalValue {
                        items: seq Item separated by position {
                            pair = "<P>";
                            first = "<F>";
                            middle = "<M>";
                            last = "<L>";
                        } terminated by "<T>",
                    }
                    require len(items) >= 2;
                    require len(items) <= 4;
                    form bounded_positional = items;
                }
                construction zero_uniform: ZeroUniform {
                    element ZeroUniformValue {
                        items: seq Item separated by "<S>" terminated by "<T>",
                    }
                    form zero_uniform = items;
                }
                root Item { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect("bounded sequence rule fixture parses"),
        )
        .expect("bounded sequence rule fixture validates")
        .into_semantic()
    }

    #[test]
    fn authored_sequence_separator_spacing_controls_preceding_lexical_adjacency() {
        let plan = crate::validate_declarations(
            crate::parse_declarations(quote! {
                vocab Digit { One = "one", }
                construction magnitude: Magnitude {
                    element MagnitudeValue { digit: lex Digit, }
                    form magnitude = lex(digit);
                }
                construction slash_pair: SlashPair {
                    element SlashPairValue {
                        magnitudes: seq Magnitude separated by "/",
                    }
                    require len(magnitudes) = 2;
                    form slash_pair = magnitudes;
                }
                construction spaced_pair: SpacedPair {
                    element SpacedPairValue {
                        magnitudes: seq Magnitude separated by " / ",
                    }
                    require len(magnitudes) = 2;
                    form spaced_pair = magnitudes;
                }
                root SlashPair { eoi = true; standalone_render = true; }
            })
            .expect("separator-adjacency fixture parses"),
        )
        .expect("separator-adjacency fixture validates")
        .into_semantic();
        let rows = super::lowered_rows(&plan).expect("separator-adjacency rows lower");
        let labels = |owner| {
            rows.iter()
                .find(|row| row.owner == owner && row.rhs.len() == 3)
                .expect("exact pair helper has three symbols")
                .rhs
                .iter()
                .map(super::RuleSymbolPlan::test_label)
                .collect::<Vec<_>>()
        };
        assert_eq!(
            labels("SlashPairValue"),
            [
                "adjacent-value:Magnitude",
                "literal:/",
                "helper:SlashPairValueMagnitudesSequenceCount2Category",
            ]
        );
        assert_eq!(
            labels("SpacedPairValue"),
            [
                "value:Magnitude",
                "literal: / ",
                "helper:SpacedPairValueMagnitudesSequenceCount2Category",
            ]
        );
    }

    fn finite_lowering_size(maximum: usize) -> LoweringSize {
        let maximum = syn::LitInt::new(&maximum.to_string(), proc_macro2::Span::call_site());
        let plan = crate::validate_declarations(
            crate::parse_declarations(quote! {
                construction item: Item {
                    element ItemValue {}
                    form item = "item";
                }
                construction finite_uniform: FiniteUniform {
                    element FiniteUniformValue {
                        items: seq Item separated by "<S>" terminated by "<T>",
                    }
                    require len(items) <= #maximum;
                    form finite_uniform = items;
                }
                construction finite_positional: FinitePositional {
                    element FinitePositionalValue {
                        items: seq Item separated by position {
                            pair = "<P>";
                            first = "<F>";
                            middle = "<M>";
                            last = "<L>";
                        } terminated by "<T>",
                    }
                    require len(items) <= #maximum;
                    form finite_positional = items;
                }
                root Item { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect("finite complexity fixture parses"),
        )
        .expect("finite complexity fixture validates")
        .into_semantic();
        let rows = super::lowered_rows(&plan).expect("finite complexity rows lower");
        let rows = rows
            .iter()
            .filter(|row| {
                matches!(
                    row.owner.as_str(),
                    "FiniteUniformValue" | "FinitePositionalValue"
                )
            })
            .collect::<Vec<_>>();
        let symbols = rows.iter().map(|row| row.rhs.len()).sum();
        let names = rows
            .iter()
            .flat_map(|row| [row.id.as_str(), row.lhs.as_str()])
            .collect::<HashSet<_>>()
            .len();
        LoweringSize {
            rows: rows.len(),
            symbols,
            names,
        }
    }

    #[test]
    fn structural_finite_lowering_rows_symbols_and_names_grow_linearly() {
        let small = finite_lowering_size(32);
        let large = finite_lowering_size(64);

        assert!(
            large.rows <= small.rows * 2 + 8,
            "finite row growth must be linear: N=32 {small:?}, N=64 {large:?}",
        );
        assert!(
            large.symbols <= small.symbols * 2 + 64,
            "finite symbol growth must be linear: N=32 {small:?}, N=64 {large:?}",
        );
        assert!(
            large.symbols <= large.rows * 8,
            "every counted row must have a constant-size RHS: N=64 {large:?}",
        );
        assert!(
            large.names <= small.names * 2 + 8,
            "finite generated-name growth must be linear: N=32 {small:?}, N=64 {large:?}",
        );
    }

    #[test]
    #[expect(
        clippy::too_many_lines,
        reason = "one exact assertion authenticates every bounded row and symbol order"
    )]
    fn bounded_sequences_emit_only_reachable_exact_cardinality_rows() {
        let plan = bounded_sequence_semantic_plan();
        let rows = super::lowered_rows(&plan).expect("bounded sequence rows lower");

        let exact_pair = rows
            .iter()
            .filter(|row| row.owner == "ExactPairValue")
            .map(|row| {
                (
                    row.id.as_str(),
                    row.lhs.as_str(),
                    row.state.as_str(),
                    row.rhs
                        .iter()
                        .map(super::RuleSymbolPlan::test_label)
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(
            exact_pair,
            [(
                "ExactPairValueItemsSequencePair",
                "ExactPair",
                "positional_pair",
                vec![
                    "adjacent-value:Item".to_owned(),
                    "literal:<T>".to_owned(),
                    "literal:<P>".to_owned(),
                    "adjacent-value:Item".to_owned(),
                    "literal:<T>".to_owned(),
                ],
            )],
            "exact-two positional lowering must not request unreachable last/middle helpers",
        );

        let bounded_uniform = rows
            .iter()
            .filter(|row| {
                row.owner == "BoundedUniformValue" && row.role.as_deref() == Some("items")
            })
            .map(|row| {
                (
                    row.id.as_str(),
                    row.state.as_str(),
                    row.rhs
                        .iter()
                        .map(super::RuleSymbolPlan::test_label)
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(
            bounded_uniform,
            [
                (
                    "BoundedUniformValueItemsSequenceNonEmpty",
                    "sequence_non_empty",
                    vec!["helper:BoundedUniformValueItemsSequenceCategory".to_owned()],
                ),
                (
                    "BoundedUniformValueItemsSequenceCount1Continue",
                    "sequence_count_1_continue",
                    vec![
                        "adjacent-value:Item".to_owned(),
                        "literal:<T>".to_owned(),
                        "literal:<S>".to_owned(),
                        "helper:BoundedUniformValueItemsSequenceCount2Category".to_owned(),
                    ],
                ),
                (
                    "BoundedUniformValueItemsSequenceCount2Final",
                    "sequence_count_2_final",
                    vec!["adjacent-value:Item".to_owned(), "literal:<T>".to_owned(),],
                ),
                (
                    "BoundedUniformValueItemsSequenceCount2Continue",
                    "sequence_count_2_continue",
                    vec![
                        "adjacent-value:Item".to_owned(),
                        "literal:<T>".to_owned(),
                        "literal:<S>".to_owned(),
                        "helper:BoundedUniformValueItemsSequenceCount3Category".to_owned(),
                    ],
                ),
                (
                    "BoundedUniformValueItemsSequenceCount3Final",
                    "sequence_count_3_final",
                    vec!["adjacent-value:Item".to_owned(), "literal:<T>".to_owned(),],
                ),
                (
                    "BoundedUniformValueItemsSequenceCount3Continue",
                    "sequence_count_3_continue",
                    vec![
                        "adjacent-value:Item".to_owned(),
                        "literal:<T>".to_owned(),
                        "literal:<S>".to_owned(),
                        "helper:BoundedUniformValueItemsSequenceCount4Category".to_owned(),
                    ],
                ),
                (
                    "BoundedUniformValueItemsSequenceCount4Final",
                    "sequence_count_4_final",
                    vec!["adjacent-value:Item".to_owned(), "literal:<T>".to_owned(),],
                ),
            ],
            "finite uniform bounds use constant-size counted transitions only",
        );

        let bounded_positional = rows
            .iter()
            .filter(|row| row.owner == "BoundedPositionalValue")
            .map(|row| {
                (
                    row.id.as_str(),
                    row.lhs.as_str(),
                    row.state.as_str(),
                    row.rhs
                        .iter()
                        .map(super::RuleSymbolPlan::test_label)
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(
            bounded_positional,
            [
                (
                    "BoundedPositionalValueItemsSequencePair",
                    "BoundedPositional",
                    "positional_pair",
                    vec![
                        "adjacent-value:Item".to_owned(),
                        "literal:<T>".to_owned(),
                        "literal:<P>".to_owned(),
                        "adjacent-value:Item".to_owned(),
                        "literal:<T>".to_owned(),
                    ],
                ),
                (
                    "BoundedPositionalValueItemsSequenceThreePlus",
                    "BoundedPositional",
                    "positional_three_plus",
                    vec![
                        "adjacent-value:Item".to_owned(),
                        "literal:<T>".to_owned(),
                        "literal:<F>".to_owned(),
                        "helper:BoundedPositionalValueItemsSequenceCategory".to_owned(),
                    ],
                ),
                (
                    "BoundedPositionalValueItemsSequenceCount2Last",
                    "BoundedPositionalValueItemsSequenceCategory",
                    "positional_count_2_last",
                    vec![
                        "adjacent-value:Item".to_owned(),
                        "literal:<T>".to_owned(),
                        "literal:<L>".to_owned(),
                        "adjacent-value:Item".to_owned(),
                        "literal:<T>".to_owned(),
                    ],
                ),
                (
                    "BoundedPositionalValueItemsSequenceCount2Middle",
                    "BoundedPositionalValueItemsSequenceCategory",
                    "positional_count_2_middle",
                    vec![
                        "adjacent-value:Item".to_owned(),
                        "literal:<T>".to_owned(),
                        "literal:<M>".to_owned(),
                        "helper:BoundedPositionalValueItemsSequenceCount3Category".to_owned(),
                    ],
                ),
                (
                    "BoundedPositionalValueItemsSequenceCount3Last",
                    "BoundedPositionalValueItemsSequenceCount3Category",
                    "positional_count_3_last",
                    vec![
                        "adjacent-value:Item".to_owned(),
                        "literal:<T>".to_owned(),
                        "literal:<L>".to_owned(),
                        "adjacent-value:Item".to_owned(),
                        "literal:<T>".to_owned(),
                    ],
                ),
            ],
            "finite positional derivations select pair versus first/middle/last exactly",
        );

        let zero_uniform = rows
            .iter()
            .filter(|row| row.owner == "ZeroUniformValue")
            .map(|row| {
                (
                    row.id.as_str(),
                    row.lhs.as_str(),
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
            zero_uniform,
            [
                (
                    "ZeroUniformValueItemsSequenceEmpty",
                    "ZeroUniform",
                    "sequence_empty",
                    Some("ZeroUniformZeroUniform"),
                    vec![],
                ),
                (
                    "ZeroUniformValueItemsSequenceNonEmpty",
                    "ZeroUniform",
                    "sequence_non_empty",
                    Some("ZeroUniformZeroUniform"),
                    vec!["helper:ZeroUniformValueItemsSequenceCategory".to_owned()],
                ),
                (
                    "ZeroUniformValueItemsSequenceSingleton",
                    "ZeroUniformValueItemsSequenceCategory",
                    "sequence_singleton",
                    None,
                    vec!["adjacent-value:Item".to_owned(), "literal:<T>".to_owned(),],
                ),
                (
                    "ZeroUniformValueItemsSequenceRecursive",
                    "ZeroUniformValueItemsSequenceCategory",
                    "sequence_recursive",
                    None,
                    vec![
                        "adjacent-value:Item".to_owned(),
                        "literal:<T>".to_owned(),
                        "literal:<S>".to_owned(),
                        "helper:ZeroUniformValueItemsSequenceCategory".to_owned(),
                    ],
                ),
            ],
            "the nullable owner branch must be distinct from a nonempty separated tail",
        );
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
                    "UniformValueItemsSequenceSingleton",
                    "UniformValueItemsSequenceCategory",
                    "UniformValue",
                    Some("items"),
                    "sequence_singleton",
                    None,
                    vec!["adjacent-value:Item".to_owned(), "literal:<T>".to_owned(),],
                ),
                (
                    "UniformValueItemsSequenceRecursive",
                    "UniformValueItemsSequenceCategory",
                    "UniformValue",
                    Some("items"),
                    "sequence_recursive",
                    None,
                    vec![
                        "adjacent-value:Item".to_owned(),
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
                    vec!["adjacent-value:Item".to_owned(), "literal:<T>".to_owned(),],
                ),
                (
                    "PositionalValueItemsSequencePair",
                    "Positional",
                    Some("items"),
                    "positional_pair",
                    Some("PositionalPositional"),
                    vec![
                        "adjacent-value:Item".to_owned(),
                        "literal:<T>".to_owned(),
                        "literal:<P>".to_owned(),
                        "adjacent-value:Item".to_owned(),
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
                        "adjacent-value:Item".to_owned(),
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
                        "adjacent-value:Item".to_owned(),
                        "literal:<T>".to_owned(),
                        "literal:<L>".to_owned(),
                        "adjacent-value:Item".to_owned(),
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
                        "adjacent-value:Item".to_owned(),
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
            "runtime concord_class did not lower to Any: {runtime}"
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
                "Lexical :: Verb (Verbs :: Act , FeatureConstraint :: Exact (ConcordClass :: Other))"
            ),
            "known concord_class did not lower to Exact: {exact}"
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
                            right_boundary: LexicalBoundary::Separated,
                        }),
                        L(LexicalTerminal {
                            matcher: Lexical::Noun(FeatureConstraint::Any),
                            owner: LexicalOwnerTemplate::Static {
                                kind: LexicalProvenanceKind::Codec,
                                stable_id: "codec:Resource",
                            },
                            right_boundary: LexicalBoundary::Separated,
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
                            right_boundary: LexicalBoundary::Separated,
                        }),
                        N(Category::Expr),
                        L(LexicalTerminal {
                            matcher: Lexical::Marker,
                            owner: LexicalOwnerTemplate::Static {
                                kind: LexicalProvenanceKind::Codec,
                                stable_id: "codec:Marker",
                            },
                            right_boundary: LexicalBoundary::Separated,
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
                        right_boundary: LexicalBoundary::Separated,
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
                        right_boundary: LexicalBoundary::Separated,
                    })],
                },
                Rule {
                    id: RuleId::TagSolo,
                    lhs: Category::Tag,
                    rhs: &[L(LexicalTerminal {
                        matcher: Lexical::Mode,
                        owner: LexicalOwnerTemplate::Vocab { declaration: "Mode" },
                        right_boundary: LexicalBoundary::Separated,
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
                            right_boundary: LexicalBoundary::Separated,
                        }),
                        L(LexicalTerminal {
                            matcher: Lexical::Pair,
                            owner: LexicalOwnerTemplate::Static {
                                kind: LexicalProvenanceKind::Codec,
                                stable_id: "codec:Pair",
                            },
                            right_boundary: LexicalBoundary::Separated,
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
        assert_eq!(generated.len(), 7);
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
        assert_eq!(enum_variants(&actual[4]), ids);

        let normalize = |item: syn::Item| {
            prettyplease::unparse(&syn::File {
                shebang: None,
                attrs: Vec::new(),
                items: vec![item],
            })
        };
        assert_eq!(
            normalize(actual[6].clone()),
            normalize(expected_synthetic_projection_rules())
        );

        let construction_origins = ["leaf", "nested", "action", "idle", "solo", "document"]
            .map(|name| (crate::SourceDeclarationKind::Construction, name));
        for &index in &[0, 1, 2, 4, 5] {
            assert_eq!(
                generated[index]
                    .origins
                    .iter()
                    .map(|origin| (origin.kind(), origin.name()))
                    .collect::<Vec<_>>(),
                construction_origins,
            );
        }
        assert_eq!(
            generated[3]
                .origins
                .iter()
                .map(|origin| (origin.kind(), origin.name()))
                .collect::<Vec<_>>(),
            [(crate::SourceDeclarationKind::Root, "Document")],
        );
        assert_eq!(
            generated[6]
                .origins
                .iter()
                .map(|origin| (origin.kind(), origin.name()))
                .collect::<Vec<_>>(),
            construction_origins
                .into_iter()
                .chain([(crate::SourceDeclarationKind::Root, "Document")])
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
        let adapter = items[3].tokens.to_string();
        let rules = items.last().unwrap().tokens.to_string();
        assert!(rules.contains("FeatureConstraint :: Exact (Number :: Singular)"));
        assert!(!rules.contains("Lexical :: Literal (\"!\")"));
        assert!(!rules.contains("Lexical :: EndOfInput"));
        assert!(adapter.contains("Lexical :: Literal (\"!\")"));
        assert!(adapter.contains("Lexical :: EndOfInput"));
        assert!(adapter.contains("stable_id : \"root:Phrase/punctuation\""));
        assert!(adapter.contains("LexicalOwnerTemplate :: Static"));
        assert!(adapter.contains("right_boundary : LexicalBoundary :: LeftAdjacent"));
        assert_eq!(
            adapter.matches("LexicalBoundary :: LeftAdjacent").count(),
            2,
            "root punctuation is projected once in each adapter API",
        );
        assert_eq!(
            adapter.matches("LexicalBoundary :: Separated").count(),
            2,
            "the following end-of-input terminal stays separated in each adapter API",
        );
    }
}
