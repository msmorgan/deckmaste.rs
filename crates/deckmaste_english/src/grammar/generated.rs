//! Chart assembly for generated construction groups: category allocation,
//! atom-to-production mapping, and rule registration. Production assemblies
//! activate production coordination by default; test assemblies can still
//! select isolated groups through [`GeneratedActivation::Groups`].

use std::collections::BTreeMap;

use deckmaste_construction_compiler::runtime::AtomData;
use deckmaste_construction_compiler::runtime::ConstructionData;
use deckmaste_construction_compiler::runtime::ElementData;
use deckmaste_construction_compiler::runtime::FieldKindData;
use deckmaste_construction_compiler::runtime::GroupData;

use super::EnglishLexicalSlot;
use super::Expected;
use super::Nonterminal;
use super::rules::GeneratedAuxRuleRef;
use super::rules::GeneratedRuleContext;
use super::rules::GeneratedRuleRef;
use super::rules::RuleBuilder;
use super::rules::RuleImpl;
use crate::construction::ConstructionId;
use crate::construction::ProductionId;
use crate::surface::Punctuation;

#[derive(Debug, Clone, Copy)]
pub(super) enum GeneratedActivation {
    /// The production constructicon.
    Production,
    /// Test-only handwritten control with no generated group active.
    #[cfg(test)]
    Inactive,
    /// Test assemblies only: register exactly these groups, in slice order.
    #[cfg(test)]
    Groups(&'static [&'static GroupData]),
}

impl GeneratedActivation {
    pub(super) fn groups(self) -> Option<&'static [&'static GroupData]> {
        match self {
            Self::Production => Some(crate::constructions::coordination::GROUPS),
            #[cfg(test)]
            Self::Inactive => None,
            #[cfg(test)]
            Self::Groups(groups) => Some(groups),
        }
    }

    pub(super) const fn is_production(self) -> bool {
        matches!(self, Self::Production)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum GeneratedAssemblyError {
    /// A non-internal category with no explicit English engine mapping.
    UnknownCategory {
        construction: &'static str,
        category: &'static str,
    },
    /// The generated literal table currently admits `","` only.
    UnsupportedLiteral {
        construction: &'static str,
        literal: &'static str,
    },
    /// A lexeme codec without a closed engine slot.
    UnknownLexemeCodec {
        construction: &'static str,
        codec: &'static str,
    },
    /// Multi-segment atom paths have no chart meaning yet.
    UnsupportedAtomPath {
        construction: &'static str,
        path: &'static str,
    },
    /// An atom naming a field this construction does not declare.
    UnknownAtomField {
        construction: &'static str,
        field: &'static str,
    },
    /// A `Hole` atom on a field shape other than `Subtree` or `Sequence`, OR
    /// a `Lexeme` atom on a field
    /// shape other than `Scalar` — including `Optional { Scalar }`, e.g.
    /// `lex(opt field)`, which `validate.rs`'s `resolved_is_scalar` admits
    /// as a legal, renderable EC014 declaration but which has no production
    /// meaning here. Fires for either atom kind, hence the name — the
    /// field's SHAPE, not the atom's declared legality, is what's
    /// unsupported.
    UnsupportedAtomKind {
        construction: &'static str,
        field: &'static str,
    },
    /// `bind` is legal only while the family's owner row is Handwritten;
    /// activation as a generated group is the Generated owner state.
    ///
    /// `register_generated` preflights this before registering any rule.
    /// Authoring consequence: a single `bind`
    /// construction anywhere in an active group makes the WHOLE group
    /// permanently unactivatable as generated, including that group's own
    /// own-mode (non-`bind`) siblings. There is no partial activation; the
    /// fix is to remove or relocate the `bind` construction, not to work
    /// around it per-construction.
    BindWhileGenerated {
        construction: &'static str,
    },
    UnknownElement {
        construction: &'static str,
        element: &'static str,
    },
    TooManyOptionalAtoms {
        owner: &'static str,
    },
}

/// Deterministic internal-category ids: the sorted set of `internal`
/// constructions' category names across the active groups, in order. Shared
/// categories collapse to one id; ids never depend on group or registration
/// order.
pub(super) fn internal_categories(groups: &[&'static GroupData]) -> BTreeMap<&'static str, u16> {
    let mut names = BTreeMap::new();
    for group in groups {
        for construction in group.constructions {
            if construction.internal {
                names.insert(construction.category, 0_u16);
            }
        }
    }
    for (index, (_, id)) in names.iter_mut().enumerate() {
        *id = u16::try_from(index).expect("more than u16::MAX internal categories");
    }
    names
}

/// Non-internal declaration categories map explicitly onto English chart
/// categories. Payload-specific conversion remains a lowering concern.
fn engine_category(name: &str) -> Option<Nonterminal> {
    Some(match name {
        "NounPhrase" => Nonterminal::NounPhrase,
        "NominalPhrase" => Nonterminal::Nominal,
        "Determiner" => Nonterminal::Determiner,
        "AdjectivePhrase" => Nonterminal::AdjectivePhrase,
        "CoordinatedAdjectivePhrase" => Nonterminal::CoordinatedModifier,
        "PrepositionalPhrase" => Nonterminal::PrepositionalPhrase,
        "InfinitiveClause" => Nonterminal::InfinitiveClause,
        "RelativeClause" => Nonterminal::RelativeClause,
        "TransitivePredicate" => Nonterminal::ReducedRecipientPassive,
        "Quantity" => Nonterminal::Quantity,
        "DevotionColors" => Nonterminal::DevotionColors,
        "PowerToughness" => Nonterminal::PowerToughness,
        "IndependentClause" => Nonterminal::Clause,
        "KeywordArgument" => Nonterminal::PredicatedArgumentBare,
        _ => return None,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
enum AuxCategoryKind {
    Element,
    Sequence,
}

type AuxCategories = BTreeMap<(&'static str, &'static str, AuxCategoryKind), u16>;

fn auxiliary_categories(groups: &[&'static GroupData], internal_count: usize) -> AuxCategories {
    let mut result = BTreeMap::new();
    for group in groups {
        for element in group.element_data {
            for kind in [AuxCategoryKind::Element, AuxCategoryKind::Sequence] {
                result.insert((group.name, element.name, kind), 0);
            }
        }
    }
    for (offset, id) in result.values_mut().enumerate() {
        *id = u16::try_from(internal_count + offset).expect("generated categories exceed u16::MAX");
    }
    result
}

/// Resolves a category referenced from a `Hole` atom's `Subtree` field. Such
/// a reference names a category, not a specific construction, so it may
/// legitimately land on any active group's internal category or (once the
/// table grows) an engine nonterminal.
fn category_nonterminal(
    cats: &BTreeMap<&'static str, u16>,
    construction: &'static ConstructionData,
    category: &'static str,
) -> Result<Nonterminal, GeneratedAssemblyError> {
    if let Some(&id) = cats.get(category) {
        return Ok(Nonterminal::Generated(id));
    }
    engine_category(category).ok_or(GeneratedAssemblyError::UnknownCategory {
        construction: construction.id,
        category,
    })
}

/// Resolves a construction's own left-hand-side category. Membership is
/// decided by the construction's OWN `internal` flag, never by whether some
/// other construction happens to declare the same category name as internal
/// — otherwise a non-internal construction sharing a category name with an
/// internal sibling would be silently admitted as `Nonterminal::Generated`
/// instead of surfacing `UnknownCategory`.
fn lhs_category_nonterminal(
    cats: &BTreeMap<&'static str, u16>,
    construction: &'static ConstructionData,
) -> Result<Nonterminal, GeneratedAssemblyError> {
    if construction.internal {
        let id = *cats.get(construction.category).expect(
            "internal_categories collected this construction's category from the same group set",
        );
        return Ok(Nonterminal::Generated(id));
    }
    engine_category(construction.category).ok_or(GeneratedAssemblyError::UnknownCategory {
        construction: construction.id,
        category: construction.category,
    })
}

fn codec_slot(codec: &'static str) -> Option<EnglishLexicalSlot> {
    match codec {
        "Conjunction" => Some(EnglishLexicalSlot::Conjunction),
        "Comma" => Some(EnglishLexicalSlot::Punctuation(Punctuation::Comma)),
        _ => None,
    }
}

fn atom_expected(
    cats: &BTreeMap<&'static str, u16>,
    aux: &AuxCategories,
    group: &'static GroupData,
    construction: &'static ConstructionData,
    atom: AtomData,
) -> Result<Expected<Nonterminal, EnglishLexicalSlot>, GeneratedAssemblyError> {
    match atom {
        AtomData::Literal(",") => Ok(Expected::Lexical(EnglishLexicalSlot::Punctuation(
            Punctuation::Comma,
        ))),
        AtomData::Literal(literal) => Err(GeneratedAssemblyError::UnsupportedLiteral {
            construction: construction.id,
            literal,
        }),
        AtomData::Hole(path) | AtomData::Lexeme(path) => {
            if path.contains('.') {
                return Err(GeneratedAssemblyError::UnsupportedAtomPath {
                    construction: construction.id,
                    path,
                });
            }
            let field = construction
                .fields
                .iter()
                .find(|field| field.name == path)
                .ok_or(GeneratedAssemblyError::UnknownAtomField {
                    construction: construction.id,
                    field: path,
                })?;
            match (atom, field.kind) {
                (AtomData::Hole(_), FieldKindData::Subtree { category, .. }) => Ok(
                    Expected::Nonterminal(category_nonterminal(cats, construction, category)?),
                ),
                (AtomData::Hole(_), FieldKindData::Sequence { element }) => aux
                    .get(&(group.name, element, AuxCategoryKind::Sequence))
                    .copied()
                    .map(Nonterminal::Generated)
                    .map(Expected::Nonterminal)
                    .ok_or(GeneratedAssemblyError::UnknownElement {
                        construction: construction.id,
                        element,
                    }),
                (AtomData::Lexeme(_), FieldKindData::Scalar { codec }) => codec_slot(codec)
                    .map(Expected::Lexical)
                    .ok_or(GeneratedAssemblyError::UnknownLexemeCodec {
                        construction: construction.id,
                        codec,
                    }),
                _ => Err(GeneratedAssemblyError::UnsupportedAtomKind {
                    construction: construction.id,
                    field: path,
                }),
            }
        }
    }
}

fn field_expected(
    cats: &BTreeMap<&'static str, u16>,
    construction: &'static ConstructionData,
    kind: FieldKindData,
) -> Result<Expected<Nonterminal, EnglishLexicalSlot>, GeneratedAssemblyError> {
    match kind {
        FieldKindData::Subtree { category, .. } => Ok(Expected::Nonterminal(category_nonterminal(
            cats,
            construction,
            category,
        )?)),
        FieldKindData::Scalar { codec } => codec_slot(codec).map(Expected::Lexical).ok_or(
            GeneratedAssemblyError::UnknownLexemeCodec {
                construction: construction.id,
                codec,
            },
        ),
        FieldKindData::SurfaceScalar { codec } => codec_slot(codec).map(Expected::Lexical).ok_or(
            GeneratedAssemblyError::UnknownLexemeCodec {
                construction: construction.id,
                codec,
            },
        ),
        FieldKindData::Optional { inner } => field_expected(cats, construction, *inner),
        FieldKindData::Sequence { .. } => Err(GeneratedAssemblyError::UnsupportedAtomKind {
            construction: construction.id,
            field: "nested sequence element",
        }),
    }
}

/// Registers every form of every construction of every active group, in
/// declaration order, with EXPLICIT ordinals from the declaration data.
pub(super) fn register_generated(
    builder: &mut RuleBuilder,
    groups: &[&'static GroupData],
    cats: &BTreeMap<&'static str, u16>,
) -> Result<(), GeneratedAssemblyError> {
    if let Some(construction) = groups
        .iter()
        .flat_map(|group| group.constructions)
        .find(|construction| construction.bind_path.is_some())
    {
        return Err(GeneratedAssemblyError::BindWhileGenerated {
            construction: construction.id,
        });
    }
    let aux = auxiliary_categories(groups, cats.len());
    if groups.iter().any(|group| {
        group.element_data.iter().any(|element| {
            element.variants.iter().any(|variant| {
                matches!(
                    variant.payload,
                    FieldKindData::Subtree {
                        category: "PowerToughness",
                        ..
                    }
                )
            })
        })
    }) {
        builder.add_generated(
            RuleImpl::GeneratedAux(GeneratedAuxRuleRef::Transparent),
            ProductionId {
                construction: ConstructionId::new("__generated_power_toughness"),
                ordinal: 0,
            },
            Nonterminal::PowerToughness,
            [Expected::Lexical(EnglishLexicalSlot::PowerToughness)],
        );
    }
    for group in groups {
        for (element_index, element) in group.element_data.iter().enumerate() {
            register_element(builder, group, element_index, element, cats, &aux)?;
        }
        for (construction_index, construction) in group.constructions.iter().enumerate() {
            let lhs = lhs_category_nonterminal(cats, construction)?;
            for (form_index, form) in construction.forms.iter().enumerate() {
                let sequence_atoms = form
                    .atoms
                    .iter()
                    .enumerate()
                    .filter_map(|(index, atom)| match atom {
                        AtomData::Hole(path) => construction
                            .fields
                            .iter()
                            .find(|field| field.name == *path)
                            .and_then(|field| {
                                matches!(field.kind, FieldKindData::Sequence { .. })
                                    .then_some(index)
                            }),
                        AtomData::Literal(_) | AtomData::Lexeme(_) => None,
                    })
                    .collect::<Vec<_>>();
                if sequence_atoms.len() > 15
                    || sequence_atoms
                        .iter()
                        .any(|&index| index >= u64::BITS as usize)
                {
                    return Err(GeneratedAssemblyError::TooManyOptionalAtoms {
                        owner: construction.id,
                    });
                }
                for subset in 0..(1_u64 << sequence_atoms.len()) {
                    if construction
                        .feature_combinators
                        .iter()
                        .any(|feature| feature.combinator == "noun_phrase_coordination")
                        && sequence_atoms
                            .iter()
                            .enumerate()
                            .any(|(position, &atom_index)| {
                                matches!(form.atoms.get(atom_index), Some(AtomData::Hole("rest")))
                                    && subset & (1_u64 << position) == 0
                            })
                    {
                        // The named English combinator requires a nonempty
                        // coordination tail. Do not register an impossible
                        // broad NounPhrase production and wait until reduction
                        // to discover that `rest.len() >= 1` failed.
                        continue;
                    }
                    let mut rhs = Vec::with_capacity(form.atoms.len());
                    let mut present = 0_u64;
                    for (atom_index, atom) in form.atoms.iter().enumerate() {
                        let sequence_position =
                            sequence_atoms.iter().position(|&i| i == atom_index);
                        if let Some(position) = sequence_position {
                            if subset & (1_u64 << position) == 0 {
                                continue;
                            }
                            present |= 1_u64 << atom_index;
                        }
                        rhs.push(atom_expected(cats, &aux, group, construction, *atom)?);
                    }
                    if rhs.is_empty() {
                        continue;
                    }
                    if sequence_atoms
                        .iter()
                        .any(|&index| present & (1_u64 << index) == 0)
                        && matches!(
                            rhs.as_slice(),
                            [Expected::Nonterminal(target)] if *target == lhs
                        )
                    {
                        continue;
                    }
                    builder.add_generated_with_cost(
                        RuleImpl::Generated(GeneratedRuleRef {
                            group,
                            construction: construction_index,
                            form: form_index,
                            sequence_atoms: present,
                            context: GeneratedRuleContext::Value,
                        }),
                        ProductionId {
                            construction: ConstructionId::new(construction.id),
                            ordinal: form.ordinal,
                        },
                        lhs,
                        rhs.clone(),
                        generated_cost(construction),
                    );
                    if matches!(
                        construction.id,
                        "noun_phrase_coordination" | "shared_determiner_nominal"
                    ) {
                        let mut prepositional_rhs = Vec::with_capacity(rhs.len() + 1);
                        prepositional_rhs.push(Expected::Lexical(EnglishLexicalSlot::Preposition));
                        prepositional_rhs.extend(rhs.iter().copied());
                        builder.add_generated_with_cost(
                            RuleImpl::Generated(GeneratedRuleRef {
                                group,
                                construction: construction_index,
                                form: form_index,
                                sequence_atoms: present,
                                context: GeneratedRuleContext::SharedPreposition,
                            }),
                            ProductionId {
                                construction: ConstructionId::new(construction.id),
                                ordinal: form.ordinal,
                            },
                            Nonterminal::PrepositionalPhrase,
                            prepositional_rhs,
                            generated_cost(construction),
                        );
                    }
                }
            }
        }
    }
    Ok(())
}

fn generated_cost(construction: &ConstructionData) -> super::ParseCost {
    match construction.id {
        "noun_phrase_coordination" => super::ParseCost {
            precedence: 1,
            ..super::ParseCost::default()
        },
        _ => super::ParseCost::default(),
    }
}

fn register_element(
    builder: &mut RuleBuilder,
    group: &'static GroupData,
    element_index: usize,
    element: &'static ElementData,
    cats: &BTreeMap<&'static str, u16>,
    aux: &AuxCategories,
) -> Result<(), GeneratedAssemblyError> {
    let element_nt =
        Nonterminal::Generated(aux[&(group.name, element.name, AuxCategoryKind::Element)]);
    let sequence_nt =
        Nonterminal::Generated(aux[&(group.name, element.name, AuxCategoryKind::Sequence)]);
    let representative = group
        .constructions
        .first()
        .expect("validated generated groups contain a construction");
    let mut ordinal = 0_u16;

    if element.variants.is_empty() && !element.fields.is_empty() {
        if element.fields.len() > u64::BITS as usize {
            return Err(GeneratedAssemblyError::TooManyOptionalAtoms {
                owner: element.name,
            });
        }
        let optional = element
            .fields
            .iter()
            .enumerate()
            .filter_map(|(index, field)| {
                matches!(field.kind, FieldKindData::Optional { .. })
                    .then_some(index)
                    .or_else(|| {
                        matches!(
                            field.kind,
                            FieldKindData::Scalar { codec: "Comma" }
                                | FieldKindData::SurfaceScalar { codec: "Comma" }
                        )
                        .then_some(index)
                    })
            })
            .collect::<Vec<_>>();
        if optional.len() > 15 {
            return Err(GeneratedAssemblyError::TooManyOptionalAtoms {
                owner: element.name,
            });
        }
        for subset in 0..(1_u64 << optional.len()) {
            let mut rhs = Vec::new();
            let mut present_fields = 0_u64;
            for (field_index, field) in element.fields.iter().enumerate() {
                if let Some(position) = optional.iter().position(|&i| i == field_index) {
                    if subset & (1_u64 << position) == 0 {
                        continue;
                    }
                }
                present_fields |= 1_u64 << field_index;
                rhs.push(field_expected(cats, representative, field.kind)?);
            }
            if matches!(element.name, "noun_phrase_member" | "nominal_phrase_member")
                && present_fields & 0b11 == 0
            {
                // Neither delimiter can participate in an admitted member;
                // omitting this rule avoids a delimiter-free recursive NP
                // sequence in the chart.
                continue;
            }
            let rules_object_rhs = if element.name == "noun_phrase_member"
                && matches!(
                    element.fields.last().map(|field| field.kind),
                    Some(FieldKindData::Subtree {
                        category: "NounPhrase",
                        ..
                    })
                ) {
                let mut alternate = rhs.clone();
                alternate.last_mut().map(|expected| {
                    *expected = Expected::Nonterminal(Nonterminal::RulesObjectNounPhrase);
                });
                Some(alternate)
            } else {
                None
            };
            builder.add_generated(
                RuleImpl::GeneratedAux(GeneratedAuxRuleRef::ElementStruct {
                    group,
                    element: element_index,
                    present_fields,
                }),
                ProductionId {
                    construction: ConstructionId::new(element.name),
                    ordinal,
                },
                element_nt,
                rhs,
            );
            ordinal = ordinal
                .checked_add(1)
                .expect("element rule ordinal overflow");
            if let Some(rhs) = rules_object_rhs {
                builder.add_generated(
                    RuleImpl::GeneratedAux(GeneratedAuxRuleRef::ElementStruct {
                        group,
                        element: element_index,
                        present_fields,
                    }),
                    ProductionId {
                        construction: ConstructionId::new(element.name),
                        ordinal,
                    },
                    element_nt,
                    rhs,
                );
                ordinal = ordinal
                    .checked_add(1)
                    .expect("element rule ordinal overflow");
            }
        }
    } else {
        for (variant_index, variant) in element.variants.iter().enumerate() {
            builder.add_generated(
                RuleImpl::GeneratedAux(GeneratedAuxRuleRef::ElementVariant {
                    group,
                    element: element_index,
                    variant: variant_index,
                }),
                ProductionId {
                    construction: ConstructionId::new(element.name),
                    ordinal,
                },
                element_nt,
                [field_expected(cats, representative, variant.payload)?],
            );
            ordinal = ordinal
                .checked_add(1)
                .expect("element rule ordinal overflow");
        }
    }

    if ordinal == 0 {
        return Ok(());
    }
    builder.add_generated(
        RuleImpl::GeneratedAux(GeneratedAuxRuleRef::SequenceSeed {
            group,
            element: element_index,
        }),
        ProductionId {
            construction: ConstructionId::new(element.name),
            ordinal,
        },
        sequence_nt,
        [Expected::Nonterminal(element_nt)],
    );
    builder.add_generated(
        RuleImpl::GeneratedAux(GeneratedAuxRuleRef::SequenceExtend {
            group,
            element: element_index,
        }),
        ProductionId {
            construction: ConstructionId::new(element.name),
            ordinal: ordinal
                .checked_add(1)
                .expect("element rule ordinal overflow"),
        },
        sequence_nt,
        [
            Expected::Nonterminal(sequence_nt),
            Expected::Nonterminal(element_nt),
        ],
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use deckmaste_construction_compiler::runtime::AtomData;
    use deckmaste_construction_compiler::runtime::ConstructionData;
    use deckmaste_construction_compiler::runtime::ElementData;
    use deckmaste_construction_compiler::runtime::FieldData;
    use deckmaste_construction_compiler::runtime::FieldKindData;
    use deckmaste_construction_compiler::runtime::FormData;
    use deckmaste_construction_compiler::runtime::GroupData;

    use super::super::EnglishLexicalSlot;
    use super::super::Expected;
    use super::super::rules::RegistrationOrder;
    use super::super::rules::RuleBuilder;
    use super::GeneratedAssemblyError;
    use super::internal_categories;
    use super::register_generated;
    use crate::surface::Punctuation;

    const fn construction(
        id: &'static str,
        category: &'static str,
        internal: bool,
        fields: &'static [FieldData],
        forms: &'static [FormData],
    ) -> ConstructionData {
        ConstructionData {
            id,
            category,
            internal,
            own_type: Some("Synthetic"),
            bind_path: None,
            deserialize: false,
            selection_unique: false,
            dominates: &[],
            fields,
            witnesses: &[],
            forms,
            feature_combinators: &[],
            erased_builder: None,
        }
    }

    const WORD_FIELDS: &[FieldData] = &[FieldData {
        name: "word",
        kind: FieldKindData::Scalar {
            codec: "Conjunction",
        },
    }];
    const WORD_FORM: &[FormData] = &[FormData {
        name: "only",
        ordinal: 0,
        guarded: false,
        atoms: &[AtomData::Lexeme("word")],
    }];

    #[test]
    fn unknown_category_is_refused() {
        const CONSTRUCTIONS: &[ConstructionData] = &[construction(
            "np_only",
            "DefinitelyUnknown",
            false,
            WORD_FIELDS,
            WORD_FORM,
        )];
        const GROUP: GroupData = GroupData {
            name: "g",
            elements: &[],
            element_data: &[],
            constructions: CONSTRUCTIONS,
        };
        let cats = internal_categories(&[&GROUP]);
        let mut builder = RuleBuilder::default();
        let error = register_generated(&mut builder, &[&GROUP], &cats).unwrap_err();
        assert_eq!(
            error,
            GeneratedAssemblyError::UnknownCategory {
                construction: "np_only",
                category: "DefinitelyUnknown",
            }
        );
    }

    /// Controller finding: a non-internal construction sharing a category
    /// name with an internal sibling in the same group must still surface
    /// `UnknownCategory` — membership is decided by the construction's own
    /// `internal` flag, never by name presence in the internal-category map.
    #[test]
    fn non_internal_construction_sharing_an_internal_category_name_is_refused() {
        const CONSTRUCTIONS: &[ConstructionData] = &[
            construction("internal_member", "Shared", true, WORD_FIELDS, WORD_FORM),
            construction(
                "non_internal_member",
                "Shared",
                false,
                WORD_FIELDS,
                WORD_FORM,
            ),
        ];
        const GROUP: GroupData = GroupData {
            name: "g",
            elements: &[],
            element_data: &[],
            constructions: CONSTRUCTIONS,
        };
        let cats = internal_categories(&[&GROUP]);
        assert!(
            cats.contains_key("Shared"),
            "the internal member must seed the category map"
        );
        let mut builder = RuleBuilder::default();
        let error = register_generated(&mut builder, &[&GROUP], &cats).unwrap_err();
        assert_eq!(
            error,
            GeneratedAssemblyError::UnknownCategory {
                construction: "non_internal_member",
                category: "Shared",
            }
        );
    }

    #[test]
    fn unsupported_literal_is_refused() {
        const FORM: &[FormData] = &[FormData {
            name: "only",
            ordinal: 0,
            guarded: false,
            atoms: &[AtomData::Literal("and")],
        }];
        const CONSTRUCTIONS: &[ConstructionData] =
            &[construction("lit_and", "Internal", true, &[], FORM)];
        const GROUP: GroupData = GroupData {
            name: "g",
            elements: &[],
            element_data: &[],
            constructions: CONSTRUCTIONS,
        };
        let cats = internal_categories(&[&GROUP]);
        let mut builder = RuleBuilder::default();
        let error = register_generated(&mut builder, &[&GROUP], &cats).unwrap_err();
        assert_eq!(
            error,
            GeneratedAssemblyError::UnsupportedLiteral {
                construction: "lit_and",
                literal: "and"
            }
        );
    }

    #[test]
    fn comma_literal_is_admitted() {
        const FORM: &[FormData] = &[FormData {
            name: "only",
            ordinal: 0,
            guarded: false,
            atoms: &[AtomData::Literal(",")],
        }];
        const CONSTRUCTIONS: &[ConstructionData] =
            &[construction("lit_comma", "Internal", true, &[], FORM)];
        const GROUP: GroupData = GroupData {
            name: "g",
            elements: &[],
            element_data: &[],
            constructions: CONSTRUCTIONS,
        };
        let cats = internal_categories(&[&GROUP]);
        let mut builder = RuleBuilder::default();
        register_generated(&mut builder, &[&GROUP], &cats).expect("comma literal must assemble");
        let rule_book = builder.finish(RegistrationOrder::Normal);
        assert_eq!(
            rule_book.rules[0].rhs,
            vec![Expected::Lexical(EnglishLexicalSlot::Punctuation(
                Punctuation::Comma
            ))]
        );
    }

    #[test]
    fn unknown_lexeme_codec_is_refused() {
        const FIELDS: &[FieldData] = &[FieldData {
            name: "who",
            kind: FieldKindData::Scalar { codec: "Person" },
        }];
        const FORM: &[FormData] = &[FormData {
            name: "only",
            ordinal: 0,
            guarded: false,
            atoms: &[AtomData::Lexeme("who")],
        }];
        const CONSTRUCTIONS: &[ConstructionData] =
            &[construction("codec_test", "Internal", true, FIELDS, FORM)];
        const GROUP: GroupData = GroupData {
            name: "g",
            elements: &[],
            element_data: &[],
            constructions: CONSTRUCTIONS,
        };
        let cats = internal_categories(&[&GROUP]);
        let mut builder = RuleBuilder::default();
        let error = register_generated(&mut builder, &[&GROUP], &cats).unwrap_err();
        assert_eq!(
            error,
            GeneratedAssemblyError::UnknownLexemeCodec {
                construction: "codec_test",
                codec: "Person",
            }
        );
    }

    #[test]
    fn sequence_hole_registers_element_seed_and_extension_rules() {
        const FIELDS: &[FieldData] = &[FieldData {
            name: "items",
            kind: FieldKindData::Sequence { element: "Foo" },
        }];
        const FORM: &[FormData] = &[FormData {
            name: "only",
            ordinal: 0,
            guarded: false,
            atoms: &[AtomData::Hole("items")],
        }];
        const CONSTRUCTIONS: &[ConstructionData] =
            &[construction("seq_test", "Internal", true, FIELDS, FORM)];
        const ELEMENTS: &[ElementData] = &[ElementData {
            name: "Foo",
            bind_path: None,
            fields: &[FieldData {
                name: "phrase",
                kind: FieldKindData::Subtree {
                    category: "NounPhrase",
                    boxed: false,
                },
            }],
            variants: &[],
            erased_builders: &[],
            erased_sequence_builder: None,
        }];
        const GROUP: GroupData = GroupData {
            name: "g",
            elements: &["Foo"],
            element_data: ELEMENTS,
            constructions: CONSTRUCTIONS,
        };
        let cats = internal_categories(&[&GROUP]);
        let mut builder = RuleBuilder::default();
        register_generated(&mut builder, &[&GROUP], &cats)
            .expect("declared sequence elements assemble");
        let book = builder.finish(RegistrationOrder::Normal);
        assert_eq!(book.rules.len(), 4);
        assert_eq!(book.rules[0].lhs, super::super::Nonterminal::Generated(1));
        assert_eq!(
            book.rules[1].rhs,
            vec![Expected::Nonterminal(super::super::Nonterminal::Generated(
                1
            ))]
        );
        assert_eq!(
            book.rules[2].rhs,
            vec![
                Expected::Nonterminal(super::super::Nonterminal::Generated(2)),
                Expected::Nonterminal(super::super::Nonterminal::Generated(1)),
            ]
        );
        assert_eq!(
            book.rules[3].rhs,
            vec![Expected::Nonterminal(super::super::Nonterminal::Generated(
                2
            ))]
        );
    }

    #[test]
    fn dotted_atom_path_is_refused() {
        const FORM: &[FormData] = &[FormData {
            name: "only",
            ordinal: 0,
            guarded: false,
            atoms: &[AtomData::Hole("members.last")],
        }];
        const CONSTRUCTIONS: &[ConstructionData] =
            &[construction("dotted", "Internal", true, &[], FORM)];
        const GROUP: GroupData = GroupData {
            name: "g",
            elements: &[],
            element_data: &[],
            constructions: CONSTRUCTIONS,
        };
        let cats = internal_categories(&[&GROUP]);
        let mut builder = RuleBuilder::default();
        let error = register_generated(&mut builder, &[&GROUP], &cats).unwrap_err();
        assert_eq!(
            error,
            GeneratedAssemblyError::UnsupportedAtomPath {
                construction: "dotted",
                path: "members.last",
            }
        );
    }

    #[test]
    fn bind_while_generated_is_refused() {
        const CONSTRUCTIONS: &[ConstructionData] = &[ConstructionData {
            id: "bound",
            category: "Internal",
            internal: true,
            own_type: None,
            bind_path: Some("x::Y"),
            deserialize: false,
            selection_unique: false,
            dominates: &[],
            fields: &[],
            witnesses: &[],
            forms: &[],
            feature_combinators: &[],
            erased_builder: None,
        }];
        const GROUP: GroupData = GroupData {
            name: "g",
            elements: &[],
            element_data: &[],
            constructions: CONSTRUCTIONS,
        };
        let cats = internal_categories(&[&GROUP]);
        let mut builder = RuleBuilder::default();
        let error = register_generated(&mut builder, &[&GROUP], &cats).unwrap_err();
        assert_eq!(
            error,
            GeneratedAssemblyError::BindWhileGenerated {
                construction: "bound"
            }
        );
    }

    #[test]
    fn real_coordination_group_assembles_as_generated() {
        let groups = crate::constructions::coordination::GROUPS;
        let cats = internal_categories(groups);
        let mut builder = RuleBuilder::default();
        register_generated(&mut builder, groups, &cats)
            .expect("every real coordination category and sequence must assemble");
        let book = builder.finish(RegistrationOrder::Normal);
        assert!(
            book.rules.iter().any(|rule| {
                rule.production.construction.as_str() == "noun_phrase_coordination"
            })
        );
        assert!(
            book.rules.iter().any(|rule| {
                rule.production.construction.as_str() == "shared_determiner_nominal"
            })
        );
    }
}
