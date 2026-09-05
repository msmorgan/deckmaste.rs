use std::collections::HashMap;
use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::quote;

use crate::identifier::key as identifier_key;
use crate::identifier::local_name;
use crate::identifier::spelling_key;
use crate::semantic::FiniteDomainPlan;
use crate::semantic::FiniteValuePlan;
use crate::semantic::FormGuardPlan;
use crate::semantic::InvariantPlan;
use crate::semantic::PredicateMemberPlan;
use crate::semantic::PredicateSubjectPlan;
use crate::semantic::SemanticPlan;
use crate::semantic::StructuralFieldKindPlan;
use crate::semantic::StructuralFieldPlan;
use crate::semantic::ValueKindPlan;

pub(crate) mod ast;
pub(crate) mod build;
pub(crate) mod final_constituent;
pub(crate) mod render;
pub(crate) mod rules;
pub(crate) mod runtime;
pub(crate) mod scanner;
pub(crate) mod terminal;
pub(crate) mod visit;

pub(super) use crate::identifier::ADMISSIBLE_SITES_TYPE;
pub(super) use crate::identifier::ATTACHMENT_SITE_PATH_TYPE;
pub(super) use crate::identifier::ATTACHMENT_SITE_STEP_TYPE;

pub(super) fn onset(value: crate::macro_def::Onset) -> TokenStream {
    match value {
        crate::macro_def::Onset::Consonant => quote! { Onset::Consonant },
        crate::macro_def::Onset::Vowel => quote! { Onset::Vowel },
    }
}

pub(super) fn surface_feature(value: crate::macro_def::SurfaceFeature) -> TokenStream {
    match value {
        crate::macro_def::SurfaceFeature::PLAIN => {
            quote! { ::deckmaste_construction_core::macro_def::SurfaceFeature::PLAIN }
        }
        crate::macro_def::SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT => {
            quote! { ::deckmaste_construction_core::macro_def::SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT }
        }
        crate::macro_def::SurfaceFeature::PAST_PARTICIPLE => {
            quote! { ::deckmaste_construction_core::macro_def::SurfaceFeature::PAST_PARTICIPLE }
        }
        crate::macro_def::SurfaceFeature::Inflectional(
            crate::macro_def::InflectionalForm::Preterite,
        ) => {
            quote! { ::deckmaste_construction_core::macro_def::SurfaceFeature::Inflectional(::deckmaste_construction_core::macro_def::InflectionalForm::Preterite) }
        }
        crate::macro_def::SurfaceFeature::Inflectional(
            crate::macro_def::InflectionalForm::GerundParticiple,
        ) => {
            quote! { ::deckmaste_construction_core::macro_def::SurfaceFeature::Inflectional(::deckmaste_construction_core::macro_def::InflectionalForm::GerundParticiple) }
        }
        crate::macro_def::SurfaceFeature::Singular => {
            quote! { ::deckmaste_construction_core::macro_def::SurfaceFeature::Singular }
        }
        crate::macro_def::SurfaceFeature::Plural => {
            quote! { ::deckmaste_construction_core::macro_def::SurfaceFeature::Plural }
        }
        crate::macro_def::SurfaceFeature::Fixed => {
            quote! { ::deckmaste_construction_core::macro_def::SurfaceFeature::Fixed }
        }
        crate::macro_def::SurfaceFeature::BoundSuffix => {
            quote! { ::deckmaste_construction_core::macro_def::SurfaceFeature::BoundSuffix }
        }
        crate::macro_def::SurfaceFeature::BlockLabel => {
            quote! { ::deckmaste_construction_core::macro_def::SurfaceFeature::BlockLabel }
        }
    }
}

pub(super) fn emit_form_guard_expression<F>(
    construction: &crate::semantic::ConstructionPlan,
    form_index: usize,
    mut comparison: F,
) -> syn::Result<Option<TokenStream>>
where
    F: FnMut(&FiniteDomainPlan, &FiniteValuePlan) -> syn::Result<TokenStream>,
{
    fn predicate_expression<F>(
        predicate: &crate::semantic::FinitePredicatePlan,
        comparison: &mut F,
    ) -> syn::Result<TokenStream>
    where
        F: FnMut(&FiniteDomainPlan, &FiniteValuePlan) -> syn::Result<TokenStream>,
    {
        let alternatives = predicate
            .accepting()
            .iter()
            .map(|assignment| {
                let atoms = predicate
                    .domains()
                    .iter()
                    .zip(assignment.values())
                    .map(|(domain, value)| comparison(domain, value))
                    .collect::<syn::Result<Vec<_>>>()?;
                Ok(quote! { (#(#atoms)&&*) })
            })
            .collect::<syn::Result<Vec<_>>>()?;
        Ok(quote! { (#(#alternatives)||*) })
    }

    let form = &construction.forms()[form_index];
    match form.guard() {
        FormGuardPlan::Unguarded => Ok(None),
        FormGuardPlan::Predicate(predicate) => {
            predicate_expression(predicate, &mut comparison).map(Some)
        }
        FormGuardPlan::Otherwise { .. } => {
            let guarded = form
                .guard()
                .guarded_form_indexes()
                .expect("otherwise guard carries prior indexes")
                .iter()
                .map(|index| {
                    construction.forms()[*index]
                        .guard()
                        .predicate()
                        .ok_or_else(|| {
                            syn::Error::new(
                                proc_macro2::Span::call_site(),
                                "fallback references a non-predicate form",
                            )
                        })
                        .and_then(|predicate| predicate_expression(predicate, &mut comparison))
                })
                .collect::<syn::Result<Vec<_>>>()?;
            Ok(Some(quote! { !(#(#guarded)||*) }))
        }
    }
}

const RUST_SOURCE_MARGIN: usize = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SemanticTypeKind {
    Category,
    Product,
    Sum,
}

pub(super) struct SemanticType<'a> {
    pub(super) name: &'a str,
    pub(super) kind: SemanticTypeKind,
    source_index: usize,
}

pub(super) fn semantic_types(plan: &SemanticPlan) -> Vec<SemanticType<'_>> {
    let mut seen_categories = HashSet::new();
    let mut types = plan
        .constructions()
        .iter()
        .filter_map(|construction| {
            (!plan.explicit_sum_owns_construction_category(construction.category())
                && seen_categories.insert(construction.category()))
            .then_some(SemanticType {
                name: construction.category(),
                kind: SemanticTypeKind::Category,
                source_index: construction.source_index(),
            })
        })
        .chain(
            plan.constructions()
                .iter()
                .filter(|construction| {
                    plan.explicit_sum_owns_construction_category(construction.category())
                })
                .map(|construction| SemanticType {
                    name: construction.element_type(),
                    kind: SemanticTypeKind::Product,
                    source_index: construction.source_index(),
                }),
        )
        .chain(plan.products().iter().map(|product| SemanticType {
            name: product.name(),
            kind: SemanticTypeKind::Product,
            source_index: product.source_index(),
        }))
        .chain(plan.sums().iter().map(|sum| SemanticType {
            name: sum.name(),
            kind: SemanticTypeKind::Sum,
            source_index: sum.source_index(),
        }))
        .collect::<Vec<_>>();
    types.sort_by_key(|item| item.source_index);
    types
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum StructuralCarrierKind {
    Optional,
    Sequence,
}

pub(super) struct StructuralCarrier<'a> {
    pub(super) owner: &'a str,
    pub(super) diagnostic_owner: &'a str,
    pub(super) field: &'a StructuralFieldPlan,
    pub(super) kind: StructuralCarrierKind,
    source_index: usize,
    field_index: usize,
}

pub(super) use crate::identifier::StructuralHelperCategoryState;

pub(super) struct StructuralHelperCategory<'a> {
    pub(super) owner: &'a str,
    pub(super) diagnostic_owner: &'a str,
    pub(super) field: &'a StructuralFieldPlan,
    pub(super) state: StructuralHelperCategoryState,
    pub(super) name: String,
}

impl StructuralCarrier<'_> {
    pub(super) fn value_variant(&self) -> String {
        match self.kind {
            StructuralCarrierKind::Optional => format!(
                "{}{}Optional",
                crate::identifier::pascal_case(self.owner),
                crate::identifier::pascal_case(self.field.name()),
            ),
            StructuralCarrierKind::Sequence => self
                .field
                .helper_names()
                .expect("sealed sequence helper inventory")
                .all()[0]
                .to_owned(),
        }
    }

    pub(super) fn category_variant(&self) -> String {
        match self.kind {
            StructuralCarrierKind::Optional => format!("{}Category", self.value_variant()),
            StructuralCarrierKind::Sequence => self
                .field
                .helper_names()
                .expect("sealed sequence helper inventory")
                .all()[1]
                .to_owned(),
        }
    }
}

pub(super) fn structural_carriers(plan: &SemanticPlan) -> Vec<StructuralCarrier<'_>> {
    let mut carriers = plan
        .products()
        .iter()
        .flat_map(|product| {
            product
                .fields()
                .iter()
                .enumerate()
                .filter_map(move |(field_index, field)| {
                    structural_carrier_kind(field.kind()).map(|kind| StructuralCarrier {
                        owner: product.name(),
                        diagnostic_owner: product.name(),
                        field,
                        kind,
                        source_index: product.source_index(),
                        field_index,
                    })
                })
        })
        .chain(plan.constructions().iter().flat_map(|construction| {
            construction
                .fields()
                .iter()
                .enumerate()
                .filter_map(move |(field_index, field)| {
                    let field = field.structural_plan()?;
                    structural_carrier_kind(field.kind()).map(|kind| StructuralCarrier {
                        owner: construction.element_type(),
                        diagnostic_owner: construction.category(),
                        field,
                        kind,
                        source_index: construction.source_index(),
                        field_index,
                    })
                })
        }))
        .collect::<Vec<_>>();
    carriers.sort_by_key(|carrier| (carrier.source_index, carrier.field_index));
    carriers
}

pub(super) fn structural_helper_categories(
    plan: &SemanticPlan,
) -> Vec<StructuralHelperCategory<'_>> {
    structural_carriers(plan)
        .into_iter()
        .flat_map(|carrier| {
            let base = carrier.category_variant();
            let states = match carrier.field.kind() {
                StructuralFieldKindPlan::Zeroable(_) | StructuralFieldKindPlan::Required(_) => {
                    Vec::new()
                }
                StructuralFieldKindPlan::Optional(_) => {
                    vec![(StructuralHelperCategoryState::Optional, base)]
                }
                StructuralFieldKindPlan::Sequence {
                    bounds, surface, ..
                } => crate::identifier::structural_sequence_helper_categories(
                    carrier.owner,
                    carrier.field.name(),
                    bounds.max(),
                    if matches!(
                        surface.separator(),
                        Some(crate::semantic::SeparatorPlan::Positional(_))
                    ) {
                        crate::identifier::StructuralSequenceStyle::Positional
                    } else {
                        crate::identifier::StructuralSequenceStyle::Uniform
                    },
                ),
            };
            states
                .into_iter()
                .map(move |(state, name)| StructuralHelperCategory {
                    owner: carrier.owner,
                    diagnostic_owner: carrier.diagnostic_owner,
                    field: carrier.field,
                    state,
                    name,
                })
        })
        .collect()
}

fn structural_carrier_kind(kind: &StructuralFieldKindPlan) -> Option<StructuralCarrierKind> {
    match kind {
        StructuralFieldKindPlan::Required(_) | StructuralFieldKindPlan::Zeroable(_) => None,
        StructuralFieldKindPlan::Optional(_) => Some(StructuralCarrierKind::Optional),
        StructuralFieldKindPlan::Sequence { .. } => Some(StructuralCarrierKind::Sequence),
    }
}

pub(super) fn value_kind_type(value: &ValueKindPlan) -> syn::Ident {
    let name = match value {
        ValueKindPlan::Category(name)
        | ValueKindPlan::Lex(name)
        | ValueKindPlan::Identity(name)
        | ValueKindPlan::Product(name)
        | ValueKindPlan::Sum(name) => name,
    };
    crate::identifier::emitted_ident(name, proc_macro2::Span::call_site())
}

pub(super) fn structural_field_type(field: &StructuralFieldPlan) -> TokenStream {
    let ty = structural_carrier_type(field.kind());
    if field.is_recursive() {
        quote! { Box<#ty> }
    } else {
        ty
    }
}

pub(super) fn structural_carrier_type(kind: &StructuralFieldKindPlan) -> TokenStream {
    let value = value_kind_type(kind.value());
    match kind {
        StructuralFieldKindPlan::Required(_) => quote! { #value },
        StructuralFieldKindPlan::Zeroable(_) | StructuralFieldKindPlan::Optional(_) => {
            quote! { Option<#value> }
        }
        StructuralFieldKindPlan::Sequence { .. } => quote! { Vec<#value> },
    }
}

pub(super) fn feature_type(feature: crate::feature::Feature) -> TokenStream {
    match feature {
        crate::feature::Feature::ConcordClass => quote! { ConcordClass },
        crate::feature::Feature::BareLocativeLicense => quote! { BareLocativeLicense },
        crate::feature::Feature::BareDurationLicense => quote! { BareDurationLicense },
        crate::feature::Feature::Cardinality => quote! { Cardinality },
        crate::feature::Feature::Compoundability => quote! { Compoundability },
        crate::feature::Feature::Countability => quote! { Countability },
        crate::feature::Feature::HomographLicense => quote! { HomographLicense },
        crate::feature::Feature::InflectionalForm => quote! { InflectionalForm },
        crate::feature::Feature::MannerAnaphorClass => quote! { MannerAnaphorClass },
        crate::feature::Feature::ModifierLicense => quote! { ModifierLicense },
        crate::feature::Feature::DeterminerNumber => quote! { DeterminerNumber },
        crate::feature::Feature::Quantification => quote! { Quantification },
        crate::feature::Feature::FusedHeadLicense => quote! { FusedHeadLicense },
        crate::feature::Feature::Focus => quote! { Focus },
        crate::feature::Feature::PrepositionComplementKind => {
            quote! { PrepositionComplementKind }
        }
        crate::feature::Feature::LocativeTemporalLicense => quote! { LocativeTemporalLicense },
        crate::feature::Feature::NominalForm => quote! { NominalForm },
        crate::feature::Feature::NominalLicense => quote! { NominalLicense },
        crate::feature::Feature::Number => quote! { Number },
        crate::feature::Feature::Onset => quote! { Onset },
        crate::feature::Feature::Participle => quote! { Participle },
        crate::feature::Feature::PossessiveEnding => quote! { PossessiveEnding },
        crate::feature::Feature::Properness => quote! { Properness },
        crate::feature::Feature::Relationality => quote! { Relationality },
        crate::feature::Feature::BareLocativeComplement => quote! { BareLocativeComplement },
        crate::feature::Feature::PrepositionAttachment => quote! { PrepositionAttachment },
    }
}

pub(super) fn emit_invariant_expression(
    invariant: &InvariantPlan,
    subjects: &HashMap<String, TokenStream>,
) -> syn::Result<TokenStream> {
    if invariant.alternatives().is_empty() {
        return Err(internal_invariant_expression(
            "sealed predicate has no alternatives",
        ));
    }
    let alternatives = invariant
        .alternatives()
        .iter()
        .map(|alternative| {
            let atoms = alternative
                .atoms()
                .iter()
                .map(|atom| {
                    let subject = atom.subject();
                    let key = subject.semantic_key();
                    let expression = subjects.get(&key).ok_or_else(|| {
                        internal_invariant_expression(&format!(
                            "missing typed subject expression `{key}`"
                        ))
                    })?;
                    emit_predicate_atom(subject, atom.allowed(), expression)
                })
                .collect::<syn::Result<Vec<_>>>()?;
            Ok(if atoms.is_empty() {
                quote! { true }
            } else {
                quote! { #(#atoms)&&* }
            })
        })
        .collect::<syn::Result<Vec<_>>>()?;
    Ok(quote! { #(#alternatives)||* })
}

fn emit_predicate_atom(
    subject: &PredicateSubjectPlan,
    allowed: &[PredicateMemberPlan],
    expression: &TokenStream,
) -> syn::Result<TokenStream> {
    if allowed.is_empty() {
        return Err(internal_invariant_expression(
            "sealed predicate atom has no allowed members",
        ));
    }
    match subject {
        PredicateSubjectPlan::CategoryRole { category, .. } => {
            let category = local_ident(category);
            let variants = allowed
                .iter()
                .map(|member| match member {
                    PredicateMemberPlan::Variant(variant) => Ok(variant),
                    PredicateMemberPlan::Presence(_) | PredicateMemberPlan::Feature(_) => Err(
                        internal_invariant_expression("category subject has a feature member"),
                    ),
                })
                .collect::<syn::Result<Vec<_>>>()?;
            Ok(quote! { matches!(#expression, #(#category::#variants(_))|*) })
        }
        PredicateSubjectPlan::VocabRole {
            terminal, optional, ..
        } => {
            let terminal = local_ident(terminal);
            if *optional && let [PredicateMemberPlan::Presence(present)] = allowed {
                return Ok(quote! { #expression.is_some() == #present });
            }
            let variants = allowed
                .iter()
                .map(|member| match member {
                    PredicateMemberPlan::Variant(variant) => Ok(variant),
                    PredicateMemberPlan::Presence(_) | PredicateMemberPlan::Feature(_) => Err(
                        internal_invariant_expression("vocabulary subject has a feature member"),
                    ),
                })
                .collect::<syn::Result<Vec<_>>>()?;
            if *optional {
                Ok(quote! { matches!(#expression, Some(#(#terminal::#variants)|*)) })
            } else {
                Ok(quote! { matches!(#expression, #(#terminal::#variants)|*) })
            }
        }
        PredicateSubjectPlan::OptionalPresenceRole { .. } => {
            let [PredicateMemberPlan::Presence(present)] = allowed else {
                return Err(internal_invariant_expression(
                    "optional-presence subject has a non-presence member",
                ));
            };
            Ok(quote! { #expression == #present })
        }
        PredicateSubjectPlan::RoleFeature { feature, .. }
        | PredicateSubjectPlan::ConstructionFeature(feature) => {
            let optional = matches!(
                subject,
                PredicateSubjectPlan::RoleFeature { optional: true, .. }
            );
            let feature_type = match feature {
                crate::feature::Feature::ConcordClass => local_ident("ConcordClass"),
                crate::feature::Feature::BareLocativeLicense => local_ident("BareLocativeLicense"),
                crate::feature::Feature::BareDurationLicense => local_ident("BareDurationLicense"),
                crate::feature::Feature::Cardinality => local_ident("Cardinality"),
                crate::feature::Feature::Compoundability => local_ident("Compoundability"),
                crate::feature::Feature::Countability => local_ident("Countability"),
                crate::feature::Feature::HomographLicense => local_ident("HomographLicense"),
                crate::feature::Feature::InflectionalForm => local_ident("InflectionalForm"),
                crate::feature::Feature::MannerAnaphorClass => local_ident("MannerAnaphorClass"),
                crate::feature::Feature::ModifierLicense => local_ident("ModifierLicense"),
                crate::feature::Feature::DeterminerNumber => local_ident("DeterminerNumber"),
                crate::feature::Feature::Quantification => local_ident("Quantification"),
                crate::feature::Feature::FusedHeadLicense => local_ident("FusedHeadLicense"),
                crate::feature::Feature::Focus => local_ident("Focus"),
                crate::feature::Feature::PrepositionComplementKind => {
                    local_ident("PrepositionComplementKind")
                }
                crate::feature::Feature::LocativeTemporalLicense => {
                    local_ident("LocativeTemporalLicense")
                }
                crate::feature::Feature::NominalForm => local_ident("NominalForm"),
                crate::feature::Feature::NominalLicense => local_ident("NominalLicense"),
                crate::feature::Feature::Number => local_ident("Number"),
                crate::feature::Feature::Onset => local_ident("Onset"),
                crate::feature::Feature::Participle => local_ident("Participle"),
                crate::feature::Feature::PossessiveEnding => local_ident("PossessiveEnding"),
                crate::feature::Feature::Properness => local_ident("Properness"),
                crate::feature::Feature::Relationality => local_ident("Relationality"),
                crate::feature::Feature::BareLocativeComplement => {
                    local_ident("BareLocativeComplement")
                }
                crate::feature::Feature::PrepositionAttachment => {
                    local_ident("PrepositionAttachment")
                }
            };
            let members = allowed
                .iter()
                .map(|member| {
                    let PredicateMemberPlan::Feature(member) = member else {
                        return Err(internal_invariant_expression(
                            "feature subject has a variant member",
                        ));
                    };
                    if !feature.domain().contains(member.value()) {
                        return Err(internal_invariant_expression(
                            "feature subject member is outside its sealed domain",
                        ));
                    }
                    Ok(local_ident(member.value().key()))
                })
                .collect::<syn::Result<Vec<_>>>()?;
            if optional {
                Ok(quote! { matches!(#expression, None | Some(#(#feature_type::#members)|*)) })
            } else {
                Ok(quote! { matches!(#expression, #(#feature_type::#members)|*) })
            }
        }
    }
}

fn internal_invariant_expression(detail: &str) -> syn::Error {
    syn::Error::new(
        proc_macro2::Span::call_site(),
        format!("internal invariant expression emitter: {detail}"),
    )
}

#[derive(Clone, Default)]
pub(super) struct LocalAllocator {
    used: HashSet<String>,
}

impl LocalAllocator {
    pub(super) fn reserve(&mut self, name: impl AsRef<str>) {
        self.used.insert(spelling_key(name.as_ref()));
    }

    pub(super) fn reserve_ident(&mut self, name: &syn::Ident) {
        self.used.insert(identifier_key(name));
    }

    pub(super) fn allocate(&mut self, preferred: &str) -> syn::Ident {
        let preferred = local_name(preferred);
        if self.used.insert(preferred.clone()) {
            return local_ident(&preferred);
        }
        for suffix in 2.. {
            let candidate = format!("{preferred}_{suffix}");
            if self.used.insert(candidate.clone()) {
                return local_ident(&candidate);
            }
        }
        unreachable!("the local binder suffix space is unbounded")
    }

    pub(super) fn allocate_ident(&mut self, preferred: &syn::Ident) -> syn::Ident {
        self.allocate(&identifier_key(preferred))
    }
}

fn local_ident(name: &str) -> syn::Ident {
    crate::identifier::emitted_ident(name, proc_macro2::Span::call_site())
}

pub(super) fn call_match_arm(
    pattern: &TokenStream,
    call: &TokenStream,
    indentation: usize,
) -> TokenStream {
    let candidate = quote! { #pattern => #call, };
    if indentation + compact_rust_width(&candidate) + 1 >= RUST_SOURCE_MARGIN {
        quote! { #pattern => { #call; } }
    } else {
        quote! { #pattern => #call }
    }
}

fn compact_rust_width(tokens: &TokenStream) -> usize {
    tokens
        .to_string()
        .replace(" :: ", "::")
        .replace(" . ", ".")
        .replace(" (", "(")
        .replace("[ ", "[")
        .replace("{ ", "{")
        .replace(" )", ")")
        .replace(" ]", "]")
        .replace(" }", "}")
        .replace(" ,", ",")
        .replace(" ;", ";")
        .chars()
        .count()
}

pub(super) fn declaration_kind(kind: crate::macro_def::DeclarationKind) -> TokenStream {
    use crate::macro_def::DeclarationKind;

    match kind {
        DeclarationKind::KeywordAction => {
            quote! { ::deckmaste_construction_core::macro_def::DeclarationKind::KeywordAction }
        }
        DeclarationKind::KeywordAbility => {
            quote! { ::deckmaste_construction_core::macro_def::DeclarationKind::KeywordAbility }
        }
        DeclarationKind::AbilityWord => {
            quote! { ::deckmaste_construction_core::macro_def::DeclarationKind::AbilityWord }
        }
        DeclarationKind::FlavorWord => {
            quote! { ::deckmaste_construction_core::macro_def::DeclarationKind::FlavorWord }
        }
        DeclarationKind::Type => {
            quote! { ::deckmaste_construction_core::macro_def::DeclarationKind::Type }
        }
        DeclarationKind::TurnPart => {
            quote! { ::deckmaste_construction_core::macro_def::DeclarationKind::TurnPart }
        }
        DeclarationKind::CounterKind => {
            quote! { ::deckmaste_construction_core::macro_def::DeclarationKind::CounterKind }
        }
        DeclarationKind::Designation => {
            quote! { ::deckmaste_construction_core::macro_def::DeclarationKind::Designation }
        }
        DeclarationKind::Subtype(_) => unreachable!("open_verb validation excludes subtype kinds"),
    }
}

pub(super) fn subtype_category(category: crate::macro_def::SubtypeCategory) -> TokenStream {
    use crate::macro_def::SubtypeCategory;

    match category {
        SubtypeCategory::Artifact => {
            quote! { ::deckmaste_construction_core::macro_def::SubtypeCategory::Artifact }
        }
        SubtypeCategory::Battle => {
            quote! { ::deckmaste_construction_core::macro_def::SubtypeCategory::Battle }
        }
        SubtypeCategory::Creature => {
            quote! { ::deckmaste_construction_core::macro_def::SubtypeCategory::Creature }
        }
        SubtypeCategory::Enchantment => {
            quote! { ::deckmaste_construction_core::macro_def::SubtypeCategory::Enchantment }
        }
        SubtypeCategory::Land => {
            quote! { ::deckmaste_construction_core::macro_def::SubtypeCategory::Land }
        }
        SubtypeCategory::Planeswalker => {
            quote! { ::deckmaste_construction_core::macro_def::SubtypeCategory::Planeswalker }
        }
        SubtypeCategory::Spell => {
            quote! { ::deckmaste_construction_core::macro_def::SubtypeCategory::Spell }
        }
    }
}

pub(super) fn grammar_position(position: crate::macro_def::GrammarPosition) -> TokenStream {
    use crate::macro_def::GrammarPosition;

    match position {
        GrammarPosition::Verb => {
            quote! { ::deckmaste_construction_core::macro_def::GrammarPosition::Verb }
        }
        GrammarPosition::Noun => {
            quote! { ::deckmaste_construction_core::macro_def::GrammarPosition::Noun }
        }
        GrammarPosition::FixedTerm => {
            quote! { ::deckmaste_construction_core::macro_def::GrammarPosition::FixedTerm }
        }
        GrammarPosition::FixedClause => {
            quote! { ::deckmaste_construction_core::macro_def::GrammarPosition::FixedClause }
        }
        GrammarPosition::FixedKeyword => {
            quote! { ::deckmaste_construction_core::macro_def::GrammarPosition::FixedKeyword }
        }
    }
}

pub(super) fn closed_lexeme_owner_id(
    declaration: &str,
    member: &str,
    feature: crate::macro_def::SurfaceFeature,
) -> syn::LitStr {
    let feature = match feature {
        crate::macro_def::SurfaceFeature::PLAIN => "bare",
        crate::macro_def::SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT => "third_person_singular",
        crate::macro_def::SurfaceFeature::Singular => "singular",
        crate::macro_def::SurfaceFeature::Plural => "plural",
        crate::macro_def::SurfaceFeature::PAST_PARTICIPLE => "participle",
        crate::macro_def::SurfaceFeature::Inflectional(
            crate::macro_def::InflectionalForm::Preterite,
        ) => "preterite",
        crate::macro_def::SurfaceFeature::Inflectional(
            crate::macro_def::InflectionalForm::GerundParticiple,
        ) => "gerund_participle",
        crate::macro_def::SurfaceFeature::Fixed
        | crate::macro_def::SurfaceFeature::BoundSuffix
        | crate::macro_def::SurfaceFeature::BlockLabel => {
            unreachable!("closed lexemes use only ConcordClass or Number features")
        }
    };
    syn::LitStr::new(
        &format!("lexeme:{declaration}/{member}/{feature}"),
        proc_macro2::Span::call_site(),
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn local_allocator_keys_raw_names_semantically_and_legalizes_keywords() {
        let mut allocator = super::LocalAllocator::default();
        allocator.reserve("context");

        let cases = [
            ("r#payload", "payload"),
            ("payload", "payload_2"),
            ("r#context", "context_2"),
            ("where", "where_value"),
            ("r#where", "where_value_2"),
            ("r#self", "self_value"),
            ("Self", "self_value_2"),
            ("super", "super_value"),
            ("crate", "crate_value"),
            (
                "not an ident",
                "_generated_local_6e_6f_74_20_61_6e_20_69_64_65_6e_74",
            ),
        ];
        for (preferred, expected) in cases {
            assert_eq!(allocator.allocate(preferred), expected, "{preferred}");
        }
    }
}
