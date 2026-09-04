use super::AdjectiveComparisonState;
use super::Child;
use super::Conjunction;
use super::CoordinationDomain;
use super::EnglishGrammar;
use super::Features;
use super::GapState;
use super::GeneratedElementFeatures;
use super::GeneratedSequenceFeatures;
use super::IndefiniteArticle;
use super::InitialSound;
use super::NominalAttachmentPhase;
use super::NounCardinality;
use super::NounForm;
use super::NounPhraseCoordinationState;
use super::Number;
use super::Person;
use super::PersonNumber;
use super::PredicateForm;
use super::Preposition;
use super::QuantityFeatures;
use super::Reduction;
use super::SetExceptionState;
use super::VerbSlot;
use super::clause;
use super::generated::GeneratedFeatureCombinator;

pub(super) type Reduced = Features;

/// Stable, declaration-oriented reasons why a generated chart edge declined.
/// These deliberately name the failed grammar seam rather than the Rust
/// helper that happened to implement it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub(crate) enum GeneratedRejection {
    MissingDeclaration,
    ChildAssembly,
    Requirement { index: usize },
    SurfaceSequence,
    FeatureProjection,
    FeatureCombination,
    ContextProjection,
    PrefixRequirement { index: usize },
    PrefixAdmission,
    AuxiliaryReduction,
}

pub(crate) fn nominal_with_prefix_features(
    nominal: &Features,
    initial_sound: InitialSound,
    leading_opacity: bool,
    prefix_comparison: AdjectiveComparisonState,
    prefix_demonstrative_shared_determiner: bool,
) -> Option<Features> {
    let Features::Nominal {
        head,
        coordination_domain,
        form,
        determined,
        attachment,
        comparison,
        adjunct,
        opaque_head,
        set_exception_host,
        shared_determiner_open,
        demonstrative_shared_determiner,
        recipient_passive_theme,
        ..
    } = nominal
    else {
        return None;
    };
    if *determined {
        return None;
    }
    let comparison = match (prefix_comparison, *comparison) {
        (AdjectiveComparisonState::Measured, _) | (_, AdjectiveComparisonState::Measured) => {
            return None;
        }
        (AdjectiveComparisonState::Pending(class), AdjectiveComparisonState::NotComparative) => {
            AdjectiveComparisonState::Pending(class)
        }
        (AdjectiveComparisonState::Pending(_), _) => return None,
        (AdjectiveComparisonState::NotComparative | AdjectiveComparisonState::Complete, state) => {
            state
        }
    };
    Some(Features::Nominal {
        head: head.clone(),
        coordination_domain: *coordination_domain,
        form: *form,
        initial_sound,
        determined: *determined,
        modified: true,
        leading_opacity,
        attachment: *attachment,
        comparison,
        adjunct: *adjunct,
        opaque_head: *opaque_head,
        set_exception_host: *set_exception_host,
        shared_determiner_open: *shared_determiner_open,
        demonstrative_shared_determiner: *demonstrative_shared_determiner
            && prefix_demonstrative_shared_determiner,
        recipient_passive_theme: *recipient_passive_theme,
    })
}

/// One admission law shared by feature reduction and completed-value
/// construction. Card-orientation adjectives are predicative/postpositive,
/// while numeral-measured comparatives are predicative-only; neither can be
/// an attributive nominal prefix.
pub(crate) const fn nominal_attributive_adjective_is_admitted(
    card_orientation: bool,
    measured: bool,
) -> bool {
    !card_orientation && !measured
}

/// One admission law shared by PP feature reduction and completed-value
/// construction. A `by` phrase headed by a gerund belongs to the predicate
/// that licensed it, not to a nominal attachment site.
pub(crate) const fn nominal_prepositional_attachment_is_admitted(nominal_attachment: bool) -> bool {
    nominal_attachment
}

pub(crate) fn reduce_nominal_noun(head: &Features) -> Option<Features> {
    let Features::Noun {
        identity,
        coordination_domain,
        form,
        initial_sound,
        adjunct,
        opaque,
        recipient_passive_theme,
    } = head
    else {
        return None;
    };
    Some(Features::Nominal {
        head: identity.clone(),
        coordination_domain: *coordination_domain,
        form: *form,
        initial_sound: *initial_sound,
        determined: false,
        modified: false,
        leading_opacity: false,
        attachment: NominalAttachmentPhase::Open,
        comparison: AdjectiveComparisonState::NotComparative,
        adjunct: *adjunct,
        opaque_head: *opaque,
        set_exception_host: false,
        shared_determiner_open: true,
        demonstrative_shared_determiner: true,
        recipient_passive_theme: *recipient_passive_theme,
    })
}

pub(crate) fn reduce_nominal_adjective(
    adjective: &Features,
    nominal: &Features,
) -> Option<Features> {
    let Features::Adjective {
        initial_sound,
        comparison,
        card_orientation,
        demonstrative_shared_determiner,
        ..
    } = adjective
    else {
        return None;
    };
    if !nominal_attributive_adjective_is_admitted(
        *card_orientation,
        matches!(comparison, AdjectiveComparisonState::Measured),
    ) {
        return None;
    }
    nominal_with_prefix_features(
        nominal,
        *initial_sound,
        false,
        *comparison,
        *demonstrative_shared_determiner,
    )
}

pub(crate) fn reduce_nominal_noun_modifier(
    noun: &Features,
    nominal: &Features,
) -> Option<Features> {
    let Features::Noun {
        coordination_domain: modifier_domain,
        initial_sound,
        opaque: modifier_opaque,
        ..
    } = noun
    else {
        return None;
    };
    let Features::Nominal {
        coordination_domain: head_domain,
        opaque_head,
        ..
    } = nominal
    else {
        return None;
    };
    if (*opaque_head && !*modifier_opaque)
        || matches!(
            (modifier_domain, head_domain),
            (
                Some(CoordinationDomain::Damage),
                Some(CoordinationDomain::Entity)
            )
        )
    {
        return None;
    }
    nominal_with_prefix_features(
        nominal,
        *initial_sound,
        false,
        AdjectiveComparisonState::NotComparative,
        true,
    )
}

pub(crate) fn reduce_nominal_combat_step_name(
    _participants: &Features,
    head: &Features,
) -> Option<Features> {
    let Features::Noun {
        identity,
        coordination_domain,
        form,
        adjunct,
        ..
    } = head
    else {
        return None;
    };
    Some(Features::Nominal {
        head: identity.clone(),
        coordination_domain: *coordination_domain,
        form: *form,
        initial_sound: InitialSound::Consonant,
        determined: false,
        modified: false,
        leading_opacity: false,
        attachment: NominalAttachmentPhase::Open,
        comparison: AdjectiveComparisonState::NotComparative,
        adjunct: *adjunct,
        opaque_head: false,
        set_exception_host: false,
        shared_determiner_open: true,
        demonstrative_shared_determiner: true,
        recipient_passive_theme: false,
    })
}

pub(crate) fn reduce_nominal_negated_modifier(nominal: &Features) -> Option<Features> {
    nominal_with_prefix_features(
        nominal,
        InitialSound::Consonant,
        false,
        AdjectiveComparisonState::NotComparative,
        true,
    )
}

pub(crate) fn reduce_nominal_quantity_modifier(
    quantity: &Features,
    nominal: &Features,
) -> Option<Features> {
    let Features::Quantity(QuantityFeatures { cardinality, .. }) = quantity else {
        return None;
    };
    let Features::Nominal { form, .. } = nominal else {
        return None;
    };
    if !cardinality_accepts(*cardinality, *form) {
        return None;
    }
    nominal_with_prefix_features(
        nominal,
        InitialSound::Consonant,
        false,
        AdjectiveComparisonState::NotComparative,
        false,
    )
}

pub(crate) fn reduce_nominal_power_toughness_modifier(
    stats: &Features,
    nominal: &Features,
) -> Option<Features> {
    let Features::PowerToughness { initial_sound } = stats else {
        return None;
    };
    nominal_with_prefix_features(
        nominal,
        *initial_sound,
        false,
        AdjectiveComparisonState::NotComparative,
        true,
    )
}

pub(crate) fn reduce_nominal_determiner(
    determiner: &Features,
    nominal: &Features,
) -> Option<Features> {
    let Features::Determiner {
        cardinality,
        article,
        set_exception_host,
        ..
    } = determiner
    else {
        return None;
    };
    let Features::Nominal {
        head,
        coordination_domain,
        form,
        initial_sound,
        determined,
        modified,
        attachment,
        comparison,
        adjunct,
        opaque_head,
        shared_determiner_open,
        demonstrative_shared_determiner,
        recipient_passive_theme,
        ..
    } = nominal
    else {
        return None;
    };
    if *determined
        || !cardinality_accepts(*cardinality, *form)
        || !article_accepts(*article, *initial_sound)
    {
        return None;
    }
    Some(Features::Nominal {
        head: head.clone(),
        coordination_domain: *coordination_domain,
        form: *form,
        initial_sound: *initial_sound,
        determined: true,
        modified: *modified,
        leading_opacity: false,
        attachment: *attachment,
        comparison: *comparison,
        adjunct: *adjunct,
        opaque_head: *opaque_head,
        set_exception_host: *set_exception_host,
        shared_determiner_open: *shared_determiner_open,
        demonstrative_shared_determiner: *demonstrative_shared_determiner,
        recipient_passive_theme: *recipient_passive_theme,
    })
}

pub(crate) fn reduce_nominal_prepositional(
    nominal: &Features,
    preposition: &Features,
) -> Option<Features> {
    let Features::Nominal {
        head,
        coordination_domain,
        form,
        initial_sound,
        determined,
        modified,
        leading_opacity,
        attachment,
        comparison,
        adjunct,
        opaque_head,
        set_exception_host,
        demonstrative_shared_determiner,
        recipient_passive_theme,
        ..
    } = nominal
    else {
        return None;
    };
    let Features::PrepositionalPhrase {
        nominal_attachment,
        nearer_relative_host,
        ..
    } = preposition
    else {
        return None;
    };
    if !nominal_prepositional_attachment_is_admitted(*nominal_attachment) {
        return None;
    }
    if matches!(
        attachment,
        NominalAttachmentPhase::ReducedRecipientPassive
            | NominalAttachmentPhase::PostpositiveAdjective
            | NominalAttachmentPhase::Comparison
    ) {
        return None;
    }
    Some(Features::Nominal {
        head: head.clone(),
        coordination_domain: *coordination_domain,
        form: *form,
        initial_sound: *initial_sound,
        determined: *determined,
        modified: *modified,
        leading_opacity: *leading_opacity,
        attachment: NominalAttachmentPhase::Prepositional {
            nearer_relative_host: *nearer_relative_host,
        },
        comparison: *comparison,
        adjunct: *adjunct,
        opaque_head: *opaque_head,
        set_exception_host: *set_exception_host,
        shared_determiner_open: false,
        demonstrative_shared_determiner: *demonstrative_shared_determiner,
        recipient_passive_theme: *recipient_passive_theme,
    })
}

pub(crate) fn reduce_nominal_infinitive(
    nominal: &Features,
    infinitive: &Features,
) -> Option<Features> {
    if !matches!(infinitive, Features::InfinitiveClause) {
        return None;
    }
    let Features::Nominal {
        head,
        coordination_domain,
        form,
        initial_sound,
        determined,
        modified,
        leading_opacity,
        attachment,
        comparison,
        adjunct,
        opaque_head,
        set_exception_host,
        demonstrative_shared_determiner,
        recipient_passive_theme,
        ..
    } = nominal
    else {
        return None;
    };
    if matches!(
        attachment,
        NominalAttachmentPhase::ReducedRecipientPassive
            | NominalAttachmentPhase::PostpositiveAdjective
            | NominalAttachmentPhase::Comparison
    ) {
        return None;
    }
    Some(Features::Nominal {
        head: head.clone(),
        coordination_domain: *coordination_domain,
        form: *form,
        initial_sound: *initial_sound,
        determined: *determined,
        modified: *modified,
        leading_opacity: *leading_opacity,
        attachment: NominalAttachmentPhase::Prepositional {
            nearer_relative_host: false,
        },
        comparison: *comparison,
        adjunct: *adjunct,
        opaque_head: *opaque_head,
        set_exception_host: *set_exception_host,
        shared_determiner_open: false,
        demonstrative_shared_determiner: *demonstrative_shared_determiner,
        recipient_passive_theme: *recipient_passive_theme,
    })
}

fn reduce_nominal_general_complement(nominal: &Features) -> Option<Features> {
    let Features::Nominal {
        head,
        coordination_domain,
        form,
        initial_sound,
        determined,
        modified,
        leading_opacity,
        attachment,
        comparison,
        adjunct,
        opaque_head,
        set_exception_host,
        shared_determiner_open,
        demonstrative_shared_determiner,
        recipient_passive_theme,
    } = nominal
    else {
        return None;
    };
    if matches!(
        attachment,
        NominalAttachmentPhase::ReducedRecipientPassive
            | NominalAttachmentPhase::PostpositiveAdjective
            | NominalAttachmentPhase::Comparison
    ) {
        return None;
    }
    Some(Features::Nominal {
        head: head.clone(),
        coordination_domain: *coordination_domain,
        form: *form,
        initial_sound: *initial_sound,
        determined: *determined,
        modified: *modified,
        leading_opacity: *leading_opacity,
        attachment: *attachment,
        comparison: *comparison,
        adjunct: *adjunct,
        opaque_head: *opaque_head,
        set_exception_host: *set_exception_host,
        shared_determiner_open: *shared_determiner_open,
        demonstrative_shared_determiner: *demonstrative_shared_determiner,
        recipient_passive_theme: *recipient_passive_theme,
    })
}

pub(crate) fn reduce_nominal_quantity_complement(
    nominal: &Features,
    _quantity: &Features,
) -> Option<Features> {
    reduce_nominal_general_complement(nominal)
}

pub(crate) fn reduce_nominal_keyword_symbol_argument(head: &Features) -> Option<Features> {
    let Features::Noun {
        identity,
        coordination_domain,
        form: NounForm::Mass,
        initial_sound,
        adjunct,
        ..
    } = head
    else {
        return None;
    };
    Some(Features::Nominal {
        head: identity.clone(),
        coordination_domain: *coordination_domain,
        form: NounForm::Mass,
        initial_sound: *initial_sound,
        determined: false,
        modified: false,
        leading_opacity: false,
        attachment: NominalAttachmentPhase::Open,
        comparison: AdjectiveComparisonState::NotComparative,
        adjunct: *adjunct,
        opaque_head: false,
        set_exception_host: false,
        shared_determiner_open: true,
        demonstrative_shared_determiner: true,
        recipient_passive_theme: false,
    })
}

#[allow(
    clippy::unnecessary_wraps,
    reason = "generated typed feature callbacks share a fallible Option interface"
)]
pub(crate) const fn reduce_predicated_quality() -> Option<Features> {
    Some(Features::PredicatedQuality)
}

pub(crate) fn reduce_predicated_argument_single(quality: &Features) -> Option<Features> {
    matches!(quality, Features::PredicatedQuality).then_some(Features::PredicatedArgument)
}

pub(crate) fn reduce_predicated_argument_extend(
    argument: &Features,
    conjunction: &Features,
    quality: &Features,
) -> Option<Features> {
    (matches!(argument, Features::PredicatedArgument)
        && matches!(conjunction, Features::Conjunction(Conjunction::And))
        && matches!(quality, Features::PredicatedQuality))
    .then_some(Features::PredicatedArgument)
}

pub(crate) fn reduce_nominal_keyword_predicated_argument(
    head: &Features,
    argument: &Features,
) -> Option<Features> {
    if !matches!(argument, Features::PredicatedArgument) {
        return None;
    }
    reduce_nominal_keyword_symbol_argument(head)
}

fn reduce_nominal_relative_common(nominal: &Features, relative: &Features) -> Option<Features> {
    let mut output = reduce_nominal_general_complement(nominal)?;
    let Features::Nominal {
        form,
        modified,
        attachment,
        shared_determiner_open,
        ..
    } = &mut output
    else {
        return None;
    };
    let rules_object_relative = matches!(
        relative,
        Features::RelativeClause {
            gap: GapState::Object,
            object_gap_requires_rules_object: true,
            ..
        }
    );
    if *form == NounForm::Mass && rules_object_relative {
        return None;
    }
    if let Features::RelativeClause {
        gap: GapState::Subject,
        antecedent_agreement: Some(agreement),
        ..
    } = relative
        && agreement.number
            != match form {
                NounForm::Plural => Number::Plural,
                NounForm::Singular | NounForm::Mass => Number::Singular,
            }
    {
        return None;
    }
    *attachment = if matches!(
        relative,
        Features::RelativeClause {
            bare_copular_tail: true,
            ..
        }
    ) {
        NominalAttachmentPhase::RelativeBareCopula
    } else if rules_object_relative {
        NominalAttachmentPhase::RulesObjectRelative
    } else {
        NominalAttachmentPhase::Relative
    };
    *modified = true;
    *shared_determiner_open = *shared_determiner_open && *form == NounForm::Singular;
    Some(output)
}

pub(crate) fn reduce_nominal_relative(nominal: &Features, relative: &Features) -> Option<Features> {
    reduce_nominal_relative_common(nominal, relative)
}

pub(crate) fn reduce_rules_object_nominal_base(nominal: &Features) -> Option<Features> {
    matches!(
        nominal,
        Features::Nominal {
            attachment: NominalAttachmentPhase::RulesObjectRelative,
            ..
        }
    )
    .then(|| nominal.clone())
}

pub(crate) fn reduce_rules_object_followup_relative(
    base: Option<&Features>,
    followup: Option<&Features>,
    relative: &Features,
) -> Option<Features> {
    reduce_nominal_relative_common(base.or(followup)?, relative)
}

pub(crate) fn reduce_rules_object_followup_prepositional(
    nominal: &Features,
    preposition: &Features,
) -> Option<Features> {
    reduce_nominal_prepositional(nominal, preposition)
}

pub(crate) fn reduce_nominal_reduced_recipient_passive(
    nominal: &Features,
    predicate: &Features,
) -> Option<Features> {
    let mut output = reduce_nominal_general_complement(nominal)?;
    if matches!(
        nominal,
        Features::Nominal {
            attachment: NominalAttachmentPhase::RelativeBareCopula,
            ..
        }
    ) {
        return None;
    }
    let Features::VerbPhrase {
        form: PredicateForm::PastParticiple,
        passive: false,
        object,
        indirect_object: false,
        frame,
        ..
    } = predicate
    else {
        return None;
    };
    if !frame.is_recipient_passive() || !object.has_direct_object() {
        return None;
    }
    let Features::Nominal { attachment, .. } = &mut output else {
        return None;
    };
    *attachment = NominalAttachmentPhase::ReducedRecipientPassive;
    Some(output)
}

pub(crate) fn reduce_recipient_passive_theme(nominal: &Features) -> Option<Features> {
    let Features::Nominal {
        coordination_domain,
        form,
        determined,
        modified,
        attachment: NominalAttachmentPhase::Open,
        adjunct,
        recipient_passive_theme: true,
        ..
    } = nominal
    else {
        return None;
    };
    Some(Features::NounPhrase {
        agreement: Some(PersonNumber {
            person: Person::Third,
            number: match form {
                NounForm::Plural => Number::Plural,
                NounForm::Singular | NounForm::Mass => Number::Singular,
            },
        }),
        coordination_domain: *coordination_domain,
        pronoun_case: None,
        adjunct: if *determined || *modified { *adjunct } else { None },
        set_exception: SetExceptionState::Ineligible,
        coordination: NounPhraseCoordinationState::None,
        recipient_passive_theme: true,
        rules_object_followup: false,
    })
}

pub(crate) fn reduce_recipient_passive_nominal_adjunct(
    predicate: &Features,
    noun_phrase: &Features,
) -> Option<Features> {
    clause::reduce_generated_recipient_passive_nominal_adjunct_features(predicate, noun_phrase)
}

pub(crate) fn reduce_nominal_postpositive_adjective(
    nominal: &Features,
    adjective: &Features,
) -> Option<Features> {
    if !matches!(
        adjective,
        Features::Adjective {
            card_orientation: false,
            comparison: AdjectiveComparisonState::NotComparative
                | AdjectiveComparisonState::Complete
                | AdjectiveComparisonState::Pending(_),
            ..
        }
    ) {
        return None;
    }
    let Features::Nominal {
        head,
        coordination_domain,
        form,
        initial_sound,
        determined,
        modified,
        leading_opacity,
        attachment: NominalAttachmentPhase::Open,
        comparison: AdjectiveComparisonState::NotComparative,
        adjunct,
        opaque_head,
        set_exception_host,
        shared_determiner_open,
        demonstrative_shared_determiner,
        recipient_passive_theme,
    } = nominal
    else {
        return None;
    };
    Some(Features::Nominal {
        head: head.clone(),
        coordination_domain: *coordination_domain,
        form: *form,
        initial_sound: *initial_sound,
        determined: *determined,
        modified: *modified,
        leading_opacity: *leading_opacity,
        attachment: NominalAttachmentPhase::PostpositiveAdjective,
        comparison: AdjectiveComparisonState::NotComparative,
        adjunct: *adjunct,
        opaque_head: *opaque_head,
        set_exception_host: *set_exception_host,
        shared_determiner_open: *shared_determiner_open,
        demonstrative_shared_determiner: *demonstrative_shared_determiner,
        recipient_passive_theme: *recipient_passive_theme,
    })
}

fn reduce_nominal_postpositive_adjective_tail(
    nominal: &Features,
    conjunction: Option<&Features>,
    adjective: &Features,
    preposition: Option<&Features>,
) -> Option<Features> {
    if conjunction.is_some_and(|conjunction| {
        !matches!(
            conjunction,
            Features::Conjunction(Conjunction::And | Conjunction::Or | Conjunction::AndOr)
        )
    }) {
        return None;
    }
    if preposition.is_some()
        && (!matches!(
            adjective,
            Features::Adjective {
                past_participle: true,
                ..
            }
        ) || !matches!(
            preposition,
            Some(Features::PrepositionalPhrase {
                preposition: Preposition::By,
                ..
            })
        ))
    {
        return None;
    }
    if !matches!(
        adjective,
        Features::Adjective {
            card_orientation: false,
            comparison: AdjectiveComparisonState::NotComparative
                | AdjectiveComparisonState::Complete
                | AdjectiveComparisonState::Pending(_),
            ..
        }
    ) {
        return None;
    }
    let Features::Nominal {
        head,
        coordination_domain,
        form,
        initial_sound,
        determined,
        modified,
        leading_opacity,
        attachment: NominalAttachmentPhase::PostpositiveAdjective,
        comparison,
        adjunct,
        opaque_head,
        set_exception_host,
        shared_determiner_open,
        demonstrative_shared_determiner,
        recipient_passive_theme,
    } = nominal
    else {
        return None;
    };
    Some(Features::Nominal {
        head: head.clone(),
        coordination_domain: *coordination_domain,
        form: *form,
        initial_sound: *initial_sound,
        determined: *determined,
        modified: *modified,
        leading_opacity: *leading_opacity,
        attachment: NominalAttachmentPhase::PostpositiveAdjective,
        comparison: *comparison,
        adjunct: *adjunct,
        opaque_head: *opaque_head,
        set_exception_host: *set_exception_host,
        shared_determiner_open: *shared_determiner_open,
        demonstrative_shared_determiner: *demonstrative_shared_determiner,
        recipient_passive_theme: *recipient_passive_theme,
    })
}

pub(crate) fn reduce_nominal_postpositive_adjective_conjoined_prepositional(
    nominal: &Features,
    conjunction: &Features,
    adjective: &Features,
    preposition: &Features,
) -> Option<Features> {
    reduce_nominal_postpositive_adjective_tail(
        nominal,
        Some(conjunction),
        adjective,
        Some(preposition),
    )
}

pub(crate) fn reduce_nominal_postpositive_adjective_continuation(
    nominal: &Features,
    conjunction: &Features,
    adjective: &Features,
) -> Option<Features> {
    reduce_nominal_postpositive_adjective_tail(nominal, Some(conjunction), adjective, None)
}

pub(crate) fn reduce_nominal_postpositive_adjective_asyndetic(
    nominal: &Features,
    adjective: &Features,
) -> Option<Features> {
    reduce_nominal_postpositive_adjective_tail(nominal, None, adjective, None)
}

pub(crate) fn reduce_nominal_comparison(
    nominal: &Features,
    _comparison: &Features,
) -> Option<Features> {
    let Features::Nominal {
        head,
        coordination_domain,
        form,
        initial_sound,
        determined,
        modified,
        leading_opacity,
        attachment: NominalAttachmentPhase::Open | NominalAttachmentPhase::Prepositional { .. },
        comparison: AdjectiveComparisonState::Pending(_),
        adjunct,
        opaque_head,
        set_exception_host,
        shared_determiner_open,
        demonstrative_shared_determiner,
        recipient_passive_theme,
    } = nominal
    else {
        return None;
    };
    Some(Features::Nominal {
        head: head.clone(),
        coordination_domain: *coordination_domain,
        form: *form,
        initial_sound: *initial_sound,
        determined: *determined,
        modified: *modified,
        leading_opacity: *leading_opacity,
        attachment: NominalAttachmentPhase::Comparison,
        comparison: AdjectiveComparisonState::Complete,
        adjunct: *adjunct,
        opaque_head: *opaque_head,
        set_exception_host: *set_exception_host,
        shared_determiner_open: *shared_determiner_open,
        demonstrative_shared_determiner: *demonstrative_shared_determiner,
        recipient_passive_theme: *recipient_passive_theme,
    })
}

pub(crate) fn reduce_devotion_color_single(color: &Features) -> Option<Features> {
    matches!(color, Features::None).then_some(Features::None)
}

pub(crate) fn reduce_devotion_color_pair(
    _first: &Features,
    conjunction: &Features,
    _second: &Features,
) -> Option<Features> {
    matches!(conjunction, Features::Conjunction(Conjunction::And)).then_some(Features::None)
}

pub(crate) fn reduce_nominal_devotion(head: &Features, _colors: &Features) -> Option<Features> {
    let Features::Noun { identity, .. } = head else {
        return None;
    };
    Some(Features::Nominal {
        head: identity.clone(),
        coordination_domain: Some(CoordinationDomain::NonEntity),
        form: NounForm::Singular,
        initial_sound: InitialSound::Consonant,
        determined: false,
        modified: false,
        leading_opacity: false,
        attachment: NominalAttachmentPhase::Prepositional {
            nearer_relative_host: false,
        },
        comparison: AdjectiveComparisonState::NotComparative,
        adjunct: None,
        opaque_head: false,
        set_exception_host: false,
        shared_determiner_open: false,
        demonstrative_shared_determiner: true,
        recipient_passive_theme: false,
    })
}

pub(crate) fn reduce_nominal_times_clause(head: &Features, clause: &Features) -> Option<Features> {
    let Features::Noun { identity, .. } = head else {
        return None;
    };
    if !matches!(clause, Features::Clause { finite: true, .. }) {
        return None;
    }
    Some(Features::Nominal {
        head: identity.clone(),
        coordination_domain: Some(CoordinationDomain::NonEntity),
        form: NounForm::Plural,
        initial_sound: InitialSound::Consonant,
        determined: false,
        modified: false,
        leading_opacity: false,
        attachment: NominalAttachmentPhase::Relative,
        comparison: AdjectiveComparisonState::NotComparative,
        adjunct: None,
        opaque_head: false,
        set_exception_host: false,
        shared_determiner_open: false,
        demonstrative_shared_determiner: true,
        recipient_passive_theme: false,
    })
}

enum CoordinationDomainCombination {
    Compatible(Option<CoordinationDomain>),
    Incompatible,
}

fn combine_coordination_domains(
    first: Option<CoordinationDomain>,
    next: Option<CoordinationDomain>,
) -> CoordinationDomainCombination {
    match (first, next) {
        (Some(CoordinationDomain::Power), Some(CoordinationDomain::Toughness)) => {
            CoordinationDomainCombination::Compatible(Some(CoordinationDomain::PowerToughness))
        }
        (
            Some(CoordinationDomain::SelectionHost),
            Some(CoordinationDomain::SelectionContinuation),
        ) => CoordinationDomainCombination::Compatible(Some(CoordinationDomain::SelectionHost)),
        (Some(CoordinationDomain::Entity), Some(CoordinationDomain::Entity)) => {
            CoordinationDomainCombination::Compatible(Some(CoordinationDomain::Entity))
        }
        (Some(CoordinationDomain::Damage), Some(CoordinationDomain::Damage)) => {
            CoordinationDomainCombination::Compatible(Some(CoordinationDomain::Damage))
        }
        (Some(CoordinationDomain::Entity), Some(CoordinationDomain::Damage))
        | (Some(CoordinationDomain::Damage), Some(CoordinationDomain::Entity)) => {
            CoordinationDomainCombination::Incompatible
        }
        (Some(_), Some(_)) => {
            CoordinationDomainCombination::Compatible(Some(CoordinationDomain::NonEntity))
        }
        (Some(domain), None) | (None, Some(domain)) => {
            CoordinationDomainCombination::Compatible(Some(domain))
        }
        (None, None) => CoordinationDomainCombination::Compatible(None),
    }
}

pub(super) fn reduce_generated_aux(
    rule: super::rules::GeneratedAuxRuleRef,
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
) -> Option<Reduction<Features>> {
    use super::rules::GeneratedAuxRuleRef as R;
    let features = match rule {
        R::Transparent => children.first()?.features.clone(),
        R::ElementStruct {
            group,
            element,
            present_fields,
        } => {
            let element = group.element_data.get(element)?;
            if let Some((comma, conjunction)) =
                super::generated::coordination_delimiter_fields(group, element)
                && present_fields & (1_u64 << comma | 1_u64 << conjunction) == 0
            {
                // Every admitted coordination member contributes either its
                // comma or its conjunction. Reject the delimiter-free shape
                // before it can seed an arbitrarily ambiguous NP sequence in
                // opacity mode; the construction-level quantified checks
                // still decide which delimited shape is final/nonfinal.
                return None;
            }
            let mut children = children.iter();
            let mut fields = Vec::with_capacity(element.fields.len());
            for index in 0..element.fields.len() {
                fields.push(if present_fields & (1_u64 << index) == 0 {
                    Features::None
                } else {
                    children.next()?.features.clone()
                });
            }
            if children.next().is_some() {
                return None;
            }
            Features::GeneratedElement {
                fields: fields.into(),
                present_fields,
                variant: None,
            }
        }
        R::ElementVariant {
            group,
            element,
            variant,
        } => {
            let declaration = group.element_data.get(element)?.variants.get(variant)?;
            let payload = children.first()?.features;
            if !generated_payload_matches(declaration.payload, payload) {
                return None;
            }
            Features::GeneratedElement {
                fields: std::sync::Arc::default(),
                present_fields: 1,
                variant: Some(variant),
            }
        }
        R::SequenceSeed { .. } => {
            let Features::GeneratedElement {
                fields,
                present_fields,
                variant,
            } = children.first()?.features
            else {
                return None;
            };
            Features::GeneratedSequence {
                tail: std::sync::Arc::new(GeneratedSequenceFeatures::seed(
                    GeneratedElementFeatures {
                        fields: fields.clone(),
                        present_fields: *present_fields,
                        variant: *variant,
                    },
                )),
            }
        }
        R::SequenceExtend { group, element } => {
            let Features::GeneratedSequence { tail } = children.first()?.features else {
                return None;
            };
            let element = group.element_data.get(element)?;
            if let Some((comma, conjunction)) =
                super::generated::coordination_delimiter_fields(group, element)
                && !(tail.element.present_fields & (1_u64 << comma) != 0
                    && tail.element.present_fields & (1_u64 << conjunction) == 0)
            {
                // Once another member follows, the previous one is known
                // to be nonfinal: it must carry a comma and must not
                // already carry the closing conjunction.
                return None;
            }
            let Features::GeneratedElement {
                fields,
                present_fields,
                variant,
            } = children.get(1)?.features
            else {
                return None;
            };
            Features::GeneratedSequence {
                tail: std::sync::Arc::new(GeneratedSequenceFeatures::extend(
                    tail,
                    GeneratedElementFeatures {
                        fields: fields.clone(),
                        present_fields: *present_fields,
                        variant: *variant,
                    },
                )),
            }
        }
    };
    Some(Reduction {
        features,
        local_cost: super::ParseCost::default(),
    })
}

pub(super) fn reduce_generated_aux_traced(
    rule: super::rules::GeneratedAuxRuleRef,
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
) -> Result<Reduction<Features>, GeneratedRejection> {
    reduce_generated_aux(rule, children).ok_or(GeneratedRejection::AuxiliaryReduction)
}

fn generated_payload_matches(
    kind: deckmaste_construction_compiler::runtime::FieldKindData,
    payload: &Features,
) -> bool {
    use deckmaste_construction_compiler::runtime::FieldKindData;

    match kind {
        // These declaration categories project through broader chart
        // nonterminals. Preserve the declared Rust subtype before packing so
        // selection cannot choose a branch that only fails during lowering.
        FieldKindData::Subtree {
            category: "IndependentClause",
            ..
        } => matches!(
            payload,
            Features::Clause {
                standalone: true,
                ..
            }
        ),
        FieldKindData::Subtree {
            category: "CoordinatedAdjectivePhrase",
            ..
        } => matches!(
            payload,
            Features::CoordinatedModifier {
                all_adjectives: true,
                ..
            }
        ),
        FieldKindData::Subtree {
            category: "TransitivePredicate",
            ..
        } => matches!(
            payload,
            Features::VerbPhrase {
                form: PredicateForm::PastParticiple,
                passive: false,
                object,
                indirect_object: false,
                frame,
                ..
            } if frame.is_recipient_passive() && object.has_direct_object()
        ),
        _ => true,
    }
}

pub(super) fn reduce_generated(
    rule: super::rules::GeneratedRuleRef,
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
) -> Option<Reduction<Features>> {
    use deckmaste_construction_compiler::runtime::AtomData;

    let construction = rule.group.constructions.get(rule.construction)?;
    let form = construction.forms.get(rule.form)?;
    let (preposition, mut child_index) = match rule.context {
        super::rules::GeneratedRuleContext::Value
        | super::rules::GeneratedRuleContext::ObjectGap
        | super::rules::GeneratedRuleContext::ReducedRecipientPassive => (None, 0_usize),
        super::rules::GeneratedRuleContext::SharedPreposition => {
            let Features::Preposition(preposition) = children.first()?.features else {
                return None;
            };
            (Some(*preposition), 1_usize)
        }
    };
    let mut fields = vec![None; construction.fields.len()];
    for (atom_index, atom) in form.atoms.iter().enumerate() {
        if matches!(atom, AtomData::Literal(_)) {
            child_index += 1;
            continue;
        }
        let path = match atom {
            AtomData::Hole(path) | AtomData::Lexeme(path) | AtomData::Identity(path) => *path,
            AtomData::Literal(_) => unreachable!(),
        };
        let field_index = construction
            .fields
            .iter()
            .position(|field| field.name == path)?;
        if matches!(
            construction.fields[field_index].kind,
            deckmaste_construction_compiler::runtime::FieldKindData::Sequence { .. }
        ) && rule.sequence_atoms & (1_u64 << atom_index) == 0
        {
            continue;
        }
        fields[field_index] = Some(children.get(child_index)?.features);
        child_index += 1;
    }
    if child_index != children.len() {
        return None;
    }
    generated_requirements_match(rule.group, construction, &fields)?;
    if !generated_surface_sequence_scalars_match(rule, &fields) {
        return None;
    }
    let feature_target = match rule.context {
        super::rules::GeneratedRuleContext::Value
        | super::rules::GeneratedRuleContext::SharedPreposition => "features",
        super::rules::GeneratedRuleContext::ObjectGap => "object_gap",
        super::rules::GeneratedRuleContext::ReducedRecipientPassive => "reduced_passive",
    };
    let typed_features = super::generated::typed_feature_projection(
        rule.group,
        rule.construction,
        feature_target,
        &fields,
    );
    let noun_phrase_features = match typed_features {
        Some(features) => features?,
        None => generated_construction_features(rule.group, construction, &fields)?,
    };
    let features = match (rule.context, preposition) {
        (
            super::rules::GeneratedRuleContext::Value
            | super::rules::GeneratedRuleContext::ObjectGap
            | super::rules::GeneratedRuleContext::ReducedRecipientPassive,
            None,
        ) => noun_phrase_features,
        (super::rules::GeneratedRuleContext::SharedPreposition, Some(preposition)) => {
            generated_prepositional_coordination_features(
                rule,
                construction,
                GeneratedFeatureCombinator::from_construction(construction)?,
                preposition,
                &fields,
            )?
        }
        _ => return None,
    };
    let mut local_cost = super::ParseCost::default();
    if matches!(
        super::generated::typed_feature_projection(
            rule.group,
            rule.construction,
            "precedence",
            &fields,
        ),
        Some(Some(_))
    ) {
        local_cost.precedence = local_cost.precedence.saturating_add(1);
    }
    if matches!(
        super::generated::typed_feature_projection(
            rule.group,
            rule.construction,
            "reading_dispreference",
            &fields,
        ),
        Some(Some(_))
    ) {
        local_cost.reading_dispreference = local_cost.reading_dispreference.saturating_add(1);
    }
    if rule.context == super::rules::GeneratedRuleContext::SharedPreposition {
        local_cost.attachment_count = 1;
    }
    if GeneratedFeatureCombinator::from_construction(construction)
        == Some(GeneratedFeatureCombinator::SharedDeterminerCoordination)
    {
        local_cost.attachment_count = local_cost
            .attachment_count
            .saturating_add(generated_group_complement_count(construction, &fields));
    }
    Some(Reduction {
        features,
        local_cost,
    })
}

pub(super) fn reduce_generated_traced(
    rule: super::rules::GeneratedRuleRef,
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
) -> Result<Reduction<Features>, GeneratedRejection> {
    if let Some(reduction) = reduce_generated(rule, children) {
        return Ok(reduction);
    }

    let Some(construction) = rule.group.constructions.get(rule.construction) else {
        return Err(GeneratedRejection::MissingDeclaration);
    };
    if construction.forms.get(rule.form).is_none() {
        return Err(GeneratedRejection::MissingDeclaration);
    }
    let child_features = children
        .iter()
        .map(|child| child.features.clone())
        .collect::<Vec<_>>();
    let Some(fields) = generated_completed_field_features(rule, &child_features) else {
        return Err(GeneratedRejection::ChildAssembly);
    };
    if let Some(index) = construction
        .requirements
        .iter()
        .chain(construction.recognition_requirements)
        .position(|requirement| {
            generated_predicate_matches(rule.group, construction, &fields, requirement.predicate)
                != Some(true)
        })
    {
        return Err(GeneratedRejection::Requirement { index });
    }
    if !generated_surface_sequence_scalars_match(rule, &fields) {
        return Err(GeneratedRejection::SurfaceSequence);
    }
    let feature_target = match rule.context {
        super::rules::GeneratedRuleContext::Value
        | super::rules::GeneratedRuleContext::SharedPreposition => "features",
        super::rules::GeneratedRuleContext::ObjectGap => "object_gap",
        super::rules::GeneratedRuleContext::ReducedRecipientPassive => "reduced_passive",
    };
    match super::generated::typed_feature_projection(
        rule.group,
        rule.construction,
        feature_target,
        &fields,
    ) {
        Some(None) => return Err(GeneratedRejection::FeatureProjection),
        None if generated_construction_features(rule.group, construction, &fields).is_none() => {
            return Err(GeneratedRejection::FeatureCombination);
        }
        Some(Some(_)) | None => {}
    }
    Err(GeneratedRejection::ContextProjection)
}

pub(crate) fn reduce_verb_phrase_base(head: &Features) -> Option<Features> {
    let Features::Verb {
        slot,
        frame,
        head_is_copular,
        object_gap_requires_rules_object,
    } = head
    else {
        return None;
    };
    let form = predicate_form(*slot);
    Some(Features::VerbPhrase {
        form,
        passive: false,
        dependent_count: 0,
        object: super::PredicateObjectState::None,
        indirect_object: false,
        selected_preposition: false,
        phase: super::PredicateAttachmentPhase::Object,
        frame: *frame,
        bare: true,
        head_is_copular: *head_is_copular,
        object_gap_requires_rules_object: *object_gap_requires_rules_object,
        subjunctive: false,
    })
}

pub(in crate::grammar) const fn predicate_form(
    slot: crate::features::VerbSlot,
) -> super::PredicateForm {
    match slot {
        crate::features::VerbSlot::Imperative => super::PredicateForm::Imperative,
        crate::features::VerbSlot::Infinitive => super::PredicateForm::Infinitive,
        crate::features::VerbSlot::Present { person, number }
        | crate::features::VerbSlot::Past { person, number } => {
            super::PredicateForm::Finite(Some(super::PersonNumber { person, number }))
        }
        crate::features::VerbSlot::PresentParticiple => super::PredicateForm::PresentParticiple,
        crate::features::VerbSlot::PastParticiple => super::PredicateForm::PastParticiple,
    }
}

pub(super) fn generated_completed_field_features(
    rule: super::rules::GeneratedRuleRef,
    children: &[Features],
) -> Option<Vec<Option<&Features>>> {
    use deckmaste_construction_compiler::runtime::AtomData;
    use deckmaste_construction_compiler::runtime::FieldKindData;

    let construction = rule.group.constructions.get(rule.construction)?;
    let form = construction.forms.get(rule.form)?;
    let mut fields = vec![None; construction.fields.len()];
    let mut child_index = usize::from(matches!(
        rule.context,
        super::rules::GeneratedRuleContext::SharedPreposition
    ));
    for (atom_index, atom) in form.atoms.iter().enumerate() {
        if matches!(atom, AtomData::Literal(_)) {
            child_index += 1;
            continue;
        }
        let path = match atom {
            AtomData::Hole(path) | AtomData::Lexeme(path) | AtomData::Identity(path) => *path,
            AtomData::Literal(_) => unreachable!(),
        };
        let field_index = construction
            .fields
            .iter()
            .position(|field| field.name == path)?;
        if matches!(
            construction.fields[field_index].kind,
            FieldKindData::Sequence { .. }
        ) && rule.sequence_atoms & (1_u64 << atom_index) == 0
        {
            continue;
        }
        fields[field_index] = Some(children.get(child_index)?);
        child_index += 1;
    }
    (child_index == children.len()).then_some(fields)
}

fn prefix_requirement_rejects(
    predicate: deckmaste_construction_compiler::runtime::PredicateData,
    path: &str,
    feature: &Features,
) -> bool {
    use deckmaste_construction_compiler::runtime::PredicateData;

    match predicate {
        PredicateData::In {
            path: candidate,
            allowed,
        } if candidate == path => generated_scalar_in(feature, allowed) == Some(false),
        PredicateData::All(predicates) => predicates
            .iter()
            .any(|predicate| prefix_requirement_rejects(*predicate, path, feature)),
        PredicateData::LenAtLeast { .. }
        | PredicateData::LenIs { .. }
        | PredicateData::In { .. }
        | PredicateData::IsSome { .. }
        | PredicateData::IsNone { .. }
        | PredicateData::Any(_) => false,
    }
}

pub(super) fn generated_accepts_prefix(
    rule: super::rules::GeneratedRuleRef,
    completed_children: usize,
    latest_child: &Features,
) -> bool {
    use deckmaste_construction_compiler::runtime::AtomData;

    let Some(construction) = rule.group.constructions.get(rule.construction) else {
        return false;
    };
    let Some(form) = construction.forms.get(rule.form) else {
        return false;
    };
    let context_offset = usize::from(matches!(
        rule.context,
        super::rules::GeneratedRuleContext::SharedPreposition
    ));
    let Some(atom_index) = completed_children
        .checked_sub(1)
        .and_then(|index| index.checked_sub(context_offset))
    else {
        return true;
    };
    let Some(atom) = form.atoms.get(atom_index) else {
        return true;
    };
    let path = match atom {
        AtomData::Hole(path) | AtomData::Lexeme(path) | AtomData::Identity(path) => *path,
        AtomData::Literal(_) => return true,
    };
    if matches!(
        construction.evidence.map(|evidence| evidence.source),
        Some(deckmaste_construction_compiler::runtime::EvidenceSourceData::Requirement(
            guarded_path
        )) if guarded_path == path
    ) && construction
        .requirements
        .iter()
        .chain(construction.recognition_requirements)
        .any(|requirement| prefix_requirement_rejects(requirement.predicate, path, latest_child))
    {
        return false;
    }
    let Some(prefix_output) = construction
        .feature_combinators
        .iter()
        .find(|output| output.target == "prefix_admission")
    else {
        return true;
    };
    if prefix_output.args != [path] {
        return true;
    }
    let Some(field_index) = construction
        .fields
        .iter()
        .position(|field| field.name == path)
    else {
        return false;
    };
    let mut fields = vec![None; construction.fields.len()];
    fields[field_index] = Some(latest_child);
    matches!(
        super::generated::typed_feature_projection(
            rule.group,
            rule.construction,
            "prefix_admission",
            &fields,
        ),
        Some(Some(_))
    )
}

pub(super) fn generated_accepts_prefix_traced(
    rule: super::rules::GeneratedRuleRef,
    completed_children: usize,
    latest_child: &Features,
) -> Result<(), GeneratedRejection> {
    if generated_accepts_prefix(rule, completed_children, latest_child) {
        return Ok(());
    }
    let Some(construction) = rule.group.constructions.get(rule.construction) else {
        return Err(GeneratedRejection::MissingDeclaration);
    };
    let Some(form) = construction.forms.get(rule.form) else {
        return Err(GeneratedRejection::MissingDeclaration);
    };
    let context_offset = usize::from(matches!(
        rule.context,
        super::rules::GeneratedRuleContext::SharedPreposition
    ));
    let Some(atom_index) = completed_children
        .checked_sub(1)
        .and_then(|index| index.checked_sub(context_offset))
    else {
        return Err(GeneratedRejection::PrefixAdmission);
    };
    let Some(atom) = form.atoms.get(atom_index) else {
        return Err(GeneratedRejection::PrefixAdmission);
    };
    let path = match atom {
        deckmaste_construction_compiler::runtime::AtomData::Hole(path)
        | deckmaste_construction_compiler::runtime::AtomData::Lexeme(path)
        | deckmaste_construction_compiler::runtime::AtomData::Identity(path) => *path,
        deckmaste_construction_compiler::runtime::AtomData::Literal(_) => {
            return Err(GeneratedRejection::PrefixAdmission);
        }
    };
    if let Some(index) = construction
        .requirements
        .iter()
        .chain(construction.recognition_requirements)
        .position(|requirement| {
            prefix_requirement_rejects(requirement.predicate, path, latest_child)
        })
    {
        return Err(GeneratedRejection::PrefixRequirement { index });
    }
    Err(GeneratedRejection::PrefixAdmission)
}

fn generated_requirements_match(
    group: &deckmaste_construction_compiler::runtime::GroupData,
    construction: &deckmaste_construction_compiler::runtime::ConstructionData,
    fields: &[Option<&Features>],
) -> Option<()> {
    construction
        .requirements
        .iter()
        .chain(construction.recognition_requirements)
        .all(|requirement| {
            generated_predicate_matches(group, construction, fields, requirement.predicate)
                == Some(true)
        })
        .then_some(())
}

fn generated_predicate_matches(
    group: &deckmaste_construction_compiler::runtime::GroupData,
    construction: &deckmaste_construction_compiler::runtime::ConstructionData,
    fields: &[Option<&Features>],
    predicate: deckmaste_construction_compiler::runtime::PredicateData,
) -> Option<bool> {
    use deckmaste_construction_compiler::runtime::PredicateData;

    match predicate {
        PredicateData::LenAtLeast { path, min } => {
            Some(generated_sequence_len(construction, fields, path)? >= usize::try_from(min).ok()?)
        }
        PredicateData::LenIs { path, len } => {
            Some(generated_sequence_len(construction, fields, path)? == usize::try_from(len).ok()?)
        }
        PredicateData::In { path, allowed } => {
            if let Some(index) = generated_top_level_field_index(construction, path) {
                return fields
                    .get(index)?
                    .map_or(Some(false), |feature| generated_scalar_in(feature, allowed));
            }
            generated_selected_elements(
                group,
                construction,
                fields,
                path,
                |element, declaration, member_field| {
                    if member_field == "variant" {
                        let variant = element.variant?;
                        let name = declaration.variants.get(variant)?.name;
                        return Some(allowed.contains(&name));
                    }
                    let (field_index, field) = declaration
                        .fields
                        .iter()
                        .enumerate()
                        .find(|(_, field)| field.name == member_field)?;
                    if element.present_fields & (1_u64 << field_index) == 0 {
                        return Some(matches!(
                            field.kind,
                            deckmaste_construction_compiler::runtime::FieldKindData::Optional { .. }
                        ));
                    }
                    generated_scalar_in(element.fields.get(field_index)?, allowed)
                },
            )
        }
        PredicateData::IsSome { path } => {
            if let Some(index) = generated_top_level_field_index(construction, path) {
                return Some(fields.get(index)?.is_some());
            }
            generated_selected_elements(
                group,
                construction,
                fields,
                path,
                |element, declaration, member_field| {
                    let field_index = declaration
                        .fields
                        .iter()
                        .position(|field| field.name == member_field)?;
                    Some(element.present_fields & (1_u64 << field_index) != 0)
                },
            )
        }
        PredicateData::IsNone { path } => {
            if let Some(index) = generated_top_level_field_index(construction, path) {
                return Some(fields.get(index)?.is_none());
            }
            generated_selected_elements(
                group,
                construction,
                fields,
                path,
                |element, declaration, member_field| {
                    let field_index = declaration
                        .fields
                        .iter()
                        .position(|field| field.name == member_field)?;
                    Some(element.present_fields & (1_u64 << field_index) == 0)
                },
            )
        }
        PredicateData::All(children) => {
            for child in children {
                if !generated_predicate_matches(group, construction, fields, *child)? {
                    return Some(false);
                }
            }
            Some(true)
        }
        PredicateData::Any(children) => {
            for child in children {
                if generated_predicate_matches(group, construction, fields, *child)? {
                    return Some(true);
                }
            }
            Some(false)
        }
    }
}

fn generated_top_level_field_index(
    construction: &deckmaste_construction_compiler::runtime::ConstructionData,
    path: &str,
) -> Option<usize> {
    (!path.contains('.')).then(|| {
        construction
            .fields
            .iter()
            .position(|field| field.name == path)
    })?
}

fn generated_sequence_len(
    construction: &deckmaste_construction_compiler::runtime::ConstructionData,
    fields: &[Option<&Features>],
    path: &str,
) -> Option<usize> {
    let field_index = construction
        .fields
        .iter()
        .position(|field| field.name == path)?;
    match fields.get(field_index)?.as_ref() {
        Some(Features::GeneratedSequence { tail }) => Some(tail.len),
        None => Some(0),
        Some(_) => None,
    }
}

fn generated_selected_elements(
    group: &deckmaste_construction_compiler::runtime::GroupData,
    construction: &deckmaste_construction_compiler::runtime::ConstructionData,
    fields: &[Option<&Features>],
    path: &str,
    mut predicate: impl FnMut(
        &GeneratedElementFeatures,
        &deckmaste_construction_compiler::runtime::ElementData,
        &str,
    ) -> Option<bool>,
) -> Option<bool> {
    let mut segments = path.split('.');
    let sequence_name = segments.next()?;
    let selector = segments.next()?;
    let member_field = segments.next()?;
    if segments.next().is_some() {
        return None;
    }
    let field_index = construction
        .fields
        .iter()
        .position(|field| field.name == sequence_name)?;
    let deckmaste_construction_compiler::runtime::FieldKindData::Sequence { element } =
        construction.fields.get(field_index)?.kind
    else {
        return None;
    };
    let declaration = group
        .element_data
        .iter()
        .find(|declaration| declaration.name == element)?;
    let elements = match fields.get(field_index)?.as_ref() {
        Some(Features::GeneratedSequence { tail }) => tail.elements(),
        None => Vec::new(),
        Some(_) => return None,
    };
    match selector {
        "first" => elements.first().map_or(Some(true), |element| {
            predicate(element, declaration, member_field)
        }),
        "last" => elements.last().map_or(Some(true), |element| {
            predicate(element, declaration, member_field)
        }),
        "nonfinal" => {
            for element in elements.iter().take(elements.len().saturating_sub(1)) {
                if !predicate(element, declaration, member_field)? {
                    return Some(false);
                }
            }
            Some(true)
        }
        _ => None,
    }
}

fn generated_scalar_in(feature: &Features, allowed: &[&str]) -> Option<bool> {
    let name = match feature {
        Features::Conjunction(Conjunction::And) => "And",
        Features::Conjunction(Conjunction::Or) => "Or",
        Features::Conjunction(Conjunction::Then) => "Then",
        Features::Conjunction(Conjunction::Plus) => "Plus",
        Features::Conjunction(Conjunction::AndOr) => "AndOr",
        Features::Preposition(Preposition::After) => "After",
        Features::Preposition(Preposition::Among) => "Among",
        Features::Preposition(Preposition::As) => "As",
        Features::Preposition(Preposition::At) => "At",
        Features::Preposition(Preposition::Before) => "Before",
        Features::Preposition(Preposition::Between) => "Between",
        Features::Preposition(Preposition::By) => "By",
        Features::Preposition(Preposition::During) => "During",
        Features::Preposition(Preposition::For) => "For",
        Features::Preposition(Preposition::From) => "From",
        Features::Preposition(Preposition::In) => "In",
        Features::Preposition(Preposition::Into) => "Into",
        Features::Preposition(Preposition::Of) => "Of",
        Features::Preposition(Preposition::On) => "On",
        Features::Preposition(Preposition::Onto) => "Onto",
        Features::Preposition(Preposition::To) => "To",
        Features::Preposition(Preposition::Until) => "Until",
        Features::Preposition(Preposition::Under) => "Under",
        Features::Preposition(Preposition::With) => "With",
        Features::Preposition(Preposition::Without) => "Without",
        _ => return None,
    };
    Some(allowed.contains(&name))
}

fn generated_group_complement_count(
    construction: &deckmaste_construction_compiler::runtime::ConstructionData,
    fields: &[Option<&Features>],
) -> u32 {
    let Some(combinator) = GeneratedFeatureCombinator::from_construction(construction) else {
        return 0;
    };
    let Some(first_field) = combinator.first_member_field_index(construction) else {
        return 0;
    };
    if matches!(
        fields.get(first_field).copied().flatten(),
        Some(Features::Nominal {
            attachment: NominalAttachmentPhase::Relative
                | NominalAttachmentPhase::RulesObjectRelative
                | NominalAttachmentPhase::RelativeBareCopula,
            ..
        })
    ) {
        // A relative before the conjunction establishes parallel member
        // scope. Do not reward moving the closing member's relative onto the
        // completed group in that shape.
        return 0;
    }
    let Some(complements_field) = combinator.complements_field_index(construction) else {
        return 0;
    };
    let Some(Features::GeneratedSequence { tail }) =
        fields.get(complements_field).copied().flatten()
    else {
        return 0;
    };
    u32::try_from(tail.len).unwrap_or(u32::MAX)
}

fn generated_prepositional_coordination_features(
    rule: super::rules::GeneratedRuleRef,
    construction: &deckmaste_construction_compiler::runtime::ConstructionData,
    combinator: GeneratedFeatureCombinator,
    preposition: Preposition,
    fields: &[Option<&Features>],
) -> Option<Features> {
    match combinator {
        GeneratedFeatureCombinator::CompleteSentence => None,
        GeneratedFeatureCombinator::CompleteNounPhraseCoordination => {
            let rest_field = combinator.rest_field_index(construction)?;
            let Features::GeneratedSequence { tail } = fields.get(rest_field)?.as_ref()? else {
                return None;
            };
            let last = &tail.element;
            let value_field = combinator.member_value_field_index(rule.group, construction)?;
            let Features::NounPhrase {
                set_exception,
                rules_object_followup,
                ..
            } = last.fields.get(value_field)?
            else {
                return None;
            };
            let licensed_set_object = tail.len == 1
                && matches!(preposition, Preposition::To | Preposition::From)
                && *set_exception == SetExceptionState::Host;
            let licensed_among_list = tail.len >= 2 && preposition == Preposition::Among;
            if !*rules_object_followup && !licensed_set_object && !licensed_among_list {
                return None;
            }
            Some(Features::PrepositionalPhrase {
                preposition,
                nominal_attachment: true,
                role_members: vec![crate::grammar::PrepositionalRoleMember {
                    preposition,
                    nominal_attachment: true,
                }],
                shared_determiner_object: false,
                nearer_relative_host: preposition == Preposition::To,
            })
        }
        GeneratedFeatureCombinator::SharedDeterminerCoordination => {
            Some(Features::PrepositionalPhrase {
                preposition,
                nominal_attachment: true,
                role_members: vec![crate::grammar::PrepositionalRoleMember {
                    preposition,
                    nominal_attachment: true,
                }],
                shared_determiner_object: true,
                nearer_relative_host: false,
            })
        }
    }
}

fn generated_surface_sequence_scalars_match(
    rule: super::rules::GeneratedRuleRef,
    fields: &[Option<&Features>],
) -> bool {
    use deckmaste_construction_compiler::runtime::FieldKindData;

    let Some(construction) = rule.group.constructions.get(rule.construction) else {
        return false;
    };
    construction
        .fields
        .iter()
        .enumerate()
        .filter_map(|(field_index, field)| {
            let FieldKindData::Sequence { element } = field.kind else {
                return None;
            };
            let element = rule
                .group
                .element_data
                .iter()
                .find(|candidate| candidate.name == element)?;
            element
                .fields
                .iter()
                .enumerate()
                .find_map(|(surface_index, field)| {
                    matches!(field.kind, FieldKindData::SurfaceScalar { codec: "Comma" })
                        .then_some((field_index, surface_index, element))
                })
        })
        .all(|(field_index, comma_index, element)| {
            let Some(Features::GeneratedSequence { tail }) =
                fields.get(field_index).copied().flatten()
            else {
                return false;
            };
            let Some(sequence) = construction.fields.get(field_index) else {
                return false;
            };
            let Some((_, conjunction_index)) =
                super::generated::coordination_delimiter_fields(rule.group, element)
            else {
                return false;
            };
            let Some(conjunction) = element.fields.get(conjunction_index) else {
                return false;
            };
            let open_comma_prefix = construction.requirements.iter().any(|requirement| {
                matches!(
                    requirement.predicate,
                    deckmaste_construction_compiler::runtime::PredicateData::IsNone { path }
                        if path
                            == format!("{}.last.{}", sequence.name, conjunction.name)
                )
            });
            let expected = open_comma_prefix || tail.len >= 2;
            tail.elements()
                .into_iter()
                .all(|element| (element.present_fields & (1_u64 << comma_index) != 0) == expected)
        })
}

fn generated_construction_features(
    group: &deckmaste_construction_compiler::runtime::GroupData,
    construction: &deckmaste_construction_compiler::runtime::ConstructionData,
    fields: &[Option<&Features>],
) -> Option<Features> {
    let feature = match construction.feature_combinators {
        [] => {
            let mut identities =
                construction
                    .fields
                    .iter()
                    .enumerate()
                    .filter_map(|(index, field)| {
                        matches!(
                            field.kind,
                            deckmaste_construction_compiler::runtime::FieldKindData::Identity { .. }
                        )
                        .then_some(index)
                    });
            let identity = identities.next();
            if identities.next().is_some() {
                return None;
            }
            return identity.map_or(Some(Features::None), |index| {
                fields.get(index).copied().flatten().cloned()
            });
        }
        [feature] => feature,
        _ => return None,
    };
    let quantity_args = feature
        .args
        .iter()
        .map(|arg| {
            use deckmaste_construction_compiler::runtime::FieldKindData;

            let index = construction
                .fields
                .iter()
                .position(|field| field.name == *arg)?;
            match construction.fields.get(index)?.kind {
                FieldKindData::Scalar { codec: "Numeral" }
                | FieldKindData::TypedScalar {
                    codec: "Numeral", ..
                } => match fields.get(index).copied().flatten()? {
                    Features::Number { is_one } => Some(
                        crate::constructions::quantity::QuantityFeatureArgument::NumberIsOne(
                            *is_one,
                        ),
                    ),
                    _ => None,
                },
                FieldKindData::Optional { .. } => Some(
                    crate::constructions::quantity::QuantityFeatureArgument::Present(
                        fields.get(index).copied().flatten().is_some(),
                    ),
                ),
                _ => None,
            }
        })
        .collect::<Option<Vec<_>>>();
    if let Some(projection) = quantity_args
        .as_deref()
        .and_then(|args| crate::constructions::quantity::project_features(feature.combinator, args))
    {
        return Some(Features::Quantity(QuantityFeatures {
            cardinality: projection.cardinality,
            standalone_number: projection.standalone_number,
            is_one: projection.is_one,
        }));
    }
    let combinator = GeneratedFeatureCombinator::from_name(feature.combinator)?;
    let args = feature
        .args
        .iter()
        .map(|arg| {
            let index = construction
                .fields
                .iter()
                .position(|field| field.name == *arg)?;
            fields.get(index).copied()
        })
        .collect::<Option<Vec<_>>>()?;
    if combinator == GeneratedFeatureCombinator::CompleteSentence {
        let [
            Some(Features::Clause {
                standalone: true,
                subjunctive: false,
                ..
            }),
        ] = args.as_slice()
        else {
            return None;
        };
        return Some(Features::Sentence);
    }
    let member_element = combinator.member_element(construction)?;
    let element = group
        .element_data
        .iter()
        .find(|element| element.name == member_element)?;
    let member_value_field = combinator.member_value_field_index(group, construction)?;
    let (_, conjunction_field) = super::generated::coordination_delimiter_fields(group, element)?;
    match (combinator, args.as_slice()) {
        (GeneratedFeatureCombinator::CompleteNounPhraseCoordination, [Some(first), Some(rest)]) => {
            complete_noun_phrase_coordination_features(
                first,
                rest,
                member_value_field,
                conjunction_field,
            )
        }
        (
            GeneratedFeatureCombinator::SharedDeterminerCoordination,
            [Some(determiner), Some(first), Some(rest), complements],
        ) => shared_determiner_coordination_features(
            determiner,
            first,
            rest,
            *complements,
            member_value_field,
            conjunction_field,
        ),
        _ => None,
    }
}

fn complete_noun_phrase_coordination_features(
    first: &Features,
    rest: &Features,
    member_value_field: usize,
    conjunction_field: usize,
) -> Option<Features> {
    let Features::NounPhrase {
        agreement: first_agreement,
        coordination_domain: first_domain,
        adjunct: first_adjunct,
        set_exception,
        coordination: first_coordination,
        recipient_passive_theme: first_theme,
        ..
    } = first
    else {
        return None;
    };
    let Features::GeneratedSequence { tail } = rest else {
        return None;
    };
    let elements = tail.elements();
    if *set_exception == SetExceptionState::Closed
        || elements.len() >= 2 && *first_coordination != NounPhraseCoordinationState::None
    {
        return None;
    }
    let mut common_adjunct = *first_adjunct;
    let mut coordination_domain = *first_domain;
    let mut last_agreement = *first_agreement;
    let mut last_coordination = NounPhraseCoordinationState::None;
    let mut all_themes = *first_theme;
    for (index, element) in elements.iter().enumerate() {
        let Features::NounPhrase {
            agreement,
            coordination_domain: member_domain,
            adjunct,
            coordination: member_coordination,
            recipient_passive_theme,
            ..
        } = element.fields.get(member_value_field)?
        else {
            return None;
        };
        if elements.len() >= 2
            && index == 0
            && first_domain.is_none()
            && *member_domain == Some(CoordinationDomain::SelectionContinuation)
        {
            return None;
        }
        if common_adjunct != *adjunct {
            common_adjunct = None;
        }
        coordination_domain =
            match combine_coordination_domains(coordination_domain, *member_domain) {
                CoordinationDomainCombination::Compatible(domain) => domain,
                CoordinationDomainCombination::Incompatible => return None,
            };
        last_agreement = *agreement;
        last_coordination = *member_coordination;
        all_themes &= *recipient_passive_theme;
    }
    let conjunction = final_generated_conjunction(tail, conjunction_field)?;
    if elements.len() == 1 {
        let nested_matches = |state| matches!(state, NounPhraseCoordinationState::Binary(inner) if inner == conjunction);
        match (
            nested_matches(*first_coordination),
            nested_matches(last_coordination),
        ) {
            // An unpunctuated same-conjunction chain is not a flat
            // three-member coordination. The one licensed one-sided
            // grouping is the local `power and toughness` idiom before
            // a following quality.
            (false, true) => return None,
            (true, false) if *first_domain != Some(CoordinationDomain::PowerToughness) => {
                return None;
            }
            _ => {}
        }
    }
    Some(Features::NounPhrase {
        agreement: coordination_agreement(conjunction, *first_agreement, last_agreement),
        coordination_domain,
        pronoun_case: None,
        adjunct: common_adjunct,
        set_exception: *set_exception,
        coordination: if elements.len() >= 2 {
            NounPhraseCoordinationState::Oxford(conjunction)
        } else {
            NounPhraseCoordinationState::Binary(conjunction)
        },
        recipient_passive_theme: all_themes,
        rules_object_followup: false,
    })
}

fn shared_determiner_coordination_features(
    determiner: &Features,
    first: &Features,
    rest: &Features,
    complements: Option<&Features>,
    member_value_field: usize,
    conjunction_field: usize,
) -> Option<Features> {
    let Features::Determiner {
        cardinality,
        article,
        demonstrative_this,
        set_exception_host,
    } = determiner
    else {
        return None;
    };
    let first = nominal_coordination_member(first)?;
    let Features::GeneratedSequence { tail } = rest else {
        return None;
    };
    let elements = tail.elements();
    if !first.shared_determiner_open {
        return None;
    }
    let mut common_adjunct = first.adjunct;
    let mut coordination_domain = first.coordination_domain;
    let mut last_form = first.form;
    let mut members = Vec::with_capacity(elements.len());
    for (index, element) in elements.iter().enumerate() {
        let member = nominal_coordination_member(element.fields.get(member_value_field)?)?;
        if elements.len() >= 2
            && index == 0
            && first.coordination_domain.is_none()
            && member.coordination_domain == Some(CoordinationDomain::SelectionContinuation)
        {
            return None;
        }
        if !generated_determiner_accepts(*cardinality, *article, member.form, member.initial_sound)
            || *demonstrative_this && !member.demonstrative_shared_determiner
        {
            return None;
        }
        if common_adjunct != member.adjunct {
            common_adjunct = None;
        }
        coordination_domain =
            match combine_coordination_domains(coordination_domain, member.coordination_domain) {
                CoordinationDomainCombination::Compatible(domain) => domain,
                CoordinationDomainCombination::Incompatible => return None,
            };
        last_form = member.form;
        members.push(member);
    }
    // A bare-first / modified-final sequence whose relative follows the whole
    // surface group has the common-head attachment as its constructionally
    // available reading. The relative may reach this reducer either as the
    // group's complement or already attached to the final nominal member.
    // Reject only those two relative-bearing shapes: complete-head ambiguity
    // remains available for an uncomplemented list such as
    // `a creature or land card`.
    let common_head_relative_candidate = bare_prefix_modified_final(&first, &members);
    if common_head_relative_candidate
        && members.last().is_some_and(|member| {
            member.modified && (complements.is_some() || member.has_relative_postmodifier())
        })
    {
        return None;
    }
    if members
        .iter()
        .take(members.len().saturating_sub(2))
        .any(|member| !member.shared_determiner_open)
        || !generated_determiner_accepts(*cardinality, *article, first.form, first.initial_sound)
    {
        return None;
    }
    let conjunction = final_generated_conjunction(tail, conjunction_field)?;
    Some(Features::NounPhrase {
        agreement: coordination_agreement(
            conjunction,
            Some(PersonNumber {
                person: Person::Third,
                number: match first.form {
                    NounForm::Plural => Number::Plural,
                    NounForm::Singular | NounForm::Mass => Number::Singular,
                },
            }),
            Some(PersonNumber {
                person: Person::Third,
                number: match last_form {
                    NounForm::Plural => Number::Plural,
                    NounForm::Singular | NounForm::Mass => Number::Singular,
                },
            }),
        ),
        coordination_domain,
        pronoun_case: None,
        adjunct: common_adjunct,
        set_exception: if *set_exception_host {
            SetExceptionState::Host
        } else {
            SetExceptionState::Ineligible
        },
        coordination: NounPhraseCoordinationState::Shared,
        recipient_passive_theme: false,
        rules_object_followup: false,
    })
}

fn final_generated_conjunction(
    sequence: &GeneratedSequenceFeatures,
    conjunction_field: usize,
) -> Option<Conjunction> {
    if sequence.element.present_fields & (1 << conjunction_field) == 0 {
        return None;
    }
    let Features::Conjunction(conjunction) = sequence.element.fields.get(conjunction_field)? else {
        return None;
    };
    matches!(
        conjunction,
        Conjunction::And | Conjunction::Or | Conjunction::Plus | Conjunction::AndOr
    )
    .then_some(*conjunction)
}

fn coordination_agreement(
    conjunction: Conjunction,
    first: Option<PersonNumber>,
    last: Option<PersonNumber>,
) -> Option<PersonNumber> {
    match conjunction {
        Conjunction::And => Some(PersonNumber {
            person: Person::Third,
            number: Number::Plural,
        }),
        Conjunction::Or | Conjunction::AndOr => last,
        Conjunction::Plus => first,
        Conjunction::Then => None,
    }
}

struct NominalCoordinationMember {
    coordination_domain: Option<CoordinationDomain>,
    form: NounForm,
    initial_sound: InitialSound,
    adjunct: Option<super::BareNominalAdjunct>,
    modified: bool,
    attachment: Option<NominalAttachmentPhase>,
    shared_determiner_open: bool,
    demonstrative_shared_determiner: bool,
}

impl NominalCoordinationMember {
    fn has_relative_postmodifier(&self) -> bool {
        matches!(
            self.attachment,
            Some(
                NominalAttachmentPhase::Relative
                    | NominalAttachmentPhase::RulesObjectRelative
                    | NominalAttachmentPhase::RelativeBareCopula
            )
        )
    }
}

fn bare_prefix_modified_final(
    first: &NominalCoordinationMember,
    rest: &[NominalCoordinationMember],
) -> bool {
    !first.modified
        && rest.last().is_some_and(|member| member.modified)
        && rest[..rest.len().saturating_sub(1)]
            .iter()
            .all(|member| !member.modified)
}

fn nominal_coordination_member(features: &Features) -> Option<NominalCoordinationMember> {
    match features {
        Features::Noun {
            coordination_domain,
            form,
            initial_sound,
            adjunct,
            ..
        } => Some(NominalCoordinationMember {
            coordination_domain: *coordination_domain,
            form: *form,
            initial_sound: *initial_sound,
            adjunct: *adjunct,
            modified: false,
            attachment: None,
            shared_determiner_open: true,
            demonstrative_shared_determiner: true,
        }),
        Features::Nominal {
            coordination_domain,
            form,
            initial_sound,
            determined: false,
            modified,
            attachment,
            adjunct,
            shared_determiner_open,
            demonstrative_shared_determiner,
            ..
        } => Some(NominalCoordinationMember {
            coordination_domain: *coordination_domain,
            form: *form,
            initial_sound: *initial_sound,
            adjunct: *adjunct,
            modified: *modified,
            attachment: Some(*attachment),
            shared_determiner_open: *shared_determiner_open,
            demonstrative_shared_determiner: *demonstrative_shared_determiner,
        }),
        _ => None,
    }
}

fn generated_determiner_accepts(
    cardinality: NounCardinality,
    article: Option<IndefiniteArticle>,
    form: NounForm,
    initial_sound: InitialSound,
) -> bool {
    if cardinality == NounCardinality::Unconstrained && form != NounForm::Singular {
        return false;
    }
    cardinality_accepts(cardinality, form) && article_accepts(article, initial_sound)
}

pub(super) fn cardinality_accepts(cardinality: NounCardinality, form: NounForm) -> bool {
    match cardinality {
        NounCardinality::SingularCount => form == NounForm::Singular,
        NounCardinality::SingularOrMass => matches!(form, NounForm::Singular | NounForm::Mass),
        NounCardinality::PluralCount => form == NounForm::Plural,
        NounCardinality::Mass => form == NounForm::Mass,
        NounCardinality::PluralOrMass => matches!(form, NounForm::Plural | NounForm::Mass),
        NounCardinality::Unconstrained => true,
    }
}

pub(super) fn article_accepts(article: Option<IndefiniteArticle>, sound: InitialSound) -> bool {
    match article {
        Some(IndefiniteArticle::A) => sound == InitialSound::Consonant,
        Some(IndefiniteArticle::An) => sound == InitialSound::Vowel,
        None => true,
    }
}

#[allow(
    dead_code,
    reason = "declaration and diagnostic probes retain the explicit agreement predicate"
)]
pub(super) fn slot_agrees(slot: VerbSlot, agreement: PersonNumber) -> bool {
    matches!(
        slot,
        VerbSlot::Present { person, number } | VerbSlot::Past { person, number }
            if person == agreement.person && number == agreement.number
    )
}

#[cfg(test)]
mod generated_tests {
    use super::*;
    use crate::constructions::coordination;
    use crate::constructions::nominal;

    fn noun_phrase(number: Number) -> Features {
        Features::NounPhrase {
            agreement: Some(PersonNumber {
                person: Person::Third,
                number,
            }),
            coordination_domain: None,
            pronoun_case: None,
            adjunct: None,
            set_exception: SetExceptionState::Ineligible,
            coordination: NounPhraseCoordinationState::None,
            recipient_passive_theme: false,
            rules_object_followup: false,
        }
    }

    fn sequence(conjunction: Conjunction, number: Number) -> Features {
        Features::GeneratedSequence {
            tail: std::sync::Arc::new(GeneratedSequenceFeatures::seed(GeneratedElementFeatures {
                fields: std::sync::Arc::from([
                    Features::None,
                    Features::Conjunction(conjunction),
                    noun_phrase(number),
                ]),
                present_fields: 1 << 1 | 1 << 2,
                variant: None,
            })),
        }
    }

    fn construction(
        id: &str,
    ) -> &'static deckmaste_construction_compiler::runtime::ConstructionData {
        coordination::GROUPS[0]
            .constructions
            .iter()
            .find(|construction| construction.id == id)
            .expect("the coordination construction is declared")
    }

    fn nominal_rule(id: &str) -> super::super::rules::GeneratedRuleRef {
        let construction = nominal::GROUPS[0]
            .constructions
            .iter()
            .position(|construction| construction.id == id)
            .expect("the nominal construction is declared");
        super::super::rules::GeneratedRuleRef {
            group: nominal::GROUPS[0],
            construction,
            form: 0,
            sequence_atoms: 0,
            context: super::super::rules::GeneratedRuleContext::Value,
        }
    }

    fn open_nominal() -> Features {
        Features::Nominal {
            head: None,
            coordination_domain: Some(CoordinationDomain::Entity),
            form: NounForm::Singular,
            initial_sound: InitialSound::Consonant,
            determined: false,
            modified: false,
            leading_opacity: false,
            attachment: NominalAttachmentPhase::PostpositiveAdjective,
            comparison: AdjectiveComparisonState::NotComparative,
            adjunct: None,
            opaque_head: false,
            set_exception_host: false,
            shared_determiner_open: true,
            demonstrative_shared_determiner: true,
            recipient_passive_theme: false,
        }
    }

    fn prepositional_phrase(preposition: Preposition, nominal_attachment: bool) -> Features {
        Features::PrepositionalPhrase {
            preposition,
            nominal_attachment,
            role_members: vec![crate::grammar::PrepositionalRoleMember {
                preposition,
                nominal_attachment,
            }],
            shared_determiner_object: false,
            nearer_relative_host: false,
        }
    }

    #[test]
    fn prepositional_list_preserves_every_member_role_fact() {
        let pair = crate::constructions::coordination::reduce_prepositional_phrase_list_features(
            &prepositional_phrase(Preposition::During, true),
            &prepositional_phrase(Preposition::At, false),
        )
        .expect("the Oxford prefix contains two PPs");
        let Features::PrepositionalPhrase {
            nominal_attachment,
            role_members,
            ..
        } = crate::constructions::coordination::reduce_prepositional_phrase_sibling_features(
            None,
            Some(&pair),
            Some(&Features::None),
            &Features::Conjunction(Conjunction::And),
            &prepositional_phrase(Preposition::During, true),
        )
        .expect("the Oxford close produces one coordinated PP feature")
        else {
            panic!("the PP feature category must be retained")
        };
        assert_eq!(
            role_members
                .iter()
                .map(|member| member.preposition)
                .collect::<Vec<_>>(),
            vec![Preposition::During, Preposition::At, Preposition::During],
        );
        assert!(!nominal_attachment);
    }

    #[test]
    fn conjoined_postpositive_pp_reduction_rejects_a_non_by_preposition() {
        // Mutation caught: remove the construction-specific semantic gate
        // from generated nominal feature reduction while leaving builder and
        // inverse validation intact.
        let features = [
            open_nominal(),
            Features::Conjunction(Conjunction::Or),
            Features::Adjective {
                initial_sound: InitialSound::Consonant,
                comparison: AdjectiveComparisonState::NotComparative,
                card_orientation: false,
                infinitive_complement: false,
                past_participle: true,
                demonstrative_shared_determiner: true,
            },
            Features::PrepositionalPhrase {
                preposition: Preposition::In,
                nominal_attachment: true,
                role_members: vec![crate::grammar::PrepositionalRoleMember {
                    preposition: Preposition::In,
                    nominal_attachment: true,
                }],
                shared_determiner_object: false,
                nearer_relative_host: false,
            },
        ];
        let children = features
            .iter()
            .map(|features| Child { features })
            .collect::<Vec<_>>();
        assert!(
            reduce_generated(
                nominal_rule("nominal_postpositive_adjective_conjoined_prepositional"),
                &children,
            )
            .is_none()
        );
    }

    #[test]
    fn generated_requirements_enforce_coordination_sequence_shape() {
        let group = coordination::GROUPS[0];
        let construction = construction("noun_phrase_coordination");
        let first = noun_phrase(Number::Singular);
        let valid = sequence(Conjunction::And, Number::Singular);
        assert!(
            generated_requirements_match(group, construction, &[Some(&first), Some(&valid)],)
                .is_some()
        );

        let missing_final_conjunction = Features::GeneratedSequence {
            tail: std::sync::Arc::new(GeneratedSequenceFeatures::seed(GeneratedElementFeatures {
                fields: std::sync::Arc::from([
                    Features::None,
                    Features::None,
                    noun_phrase(Number::Singular),
                ]),
                present_fields: 1 << 2,
                variant: None,
            })),
        };
        assert!(
            generated_requirements_match(
                group,
                construction,
                &[Some(&first), Some(&missing_final_conjunction)],
            )
            .is_none()
        );

        let nonfinal =
            std::sync::Arc::new(GeneratedSequenceFeatures::seed(GeneratedElementFeatures {
                fields: std::sync::Arc::from([
                    Features::None,
                    Features::Conjunction(Conjunction::And),
                    noun_phrase(Number::Singular),
                ]),
                present_fields: 1 << 1 | 1 << 2,
                variant: None,
            }));
        let invalid_interior_conjunction = Features::GeneratedSequence {
            tail: std::sync::Arc::new(GeneratedSequenceFeatures::extend(
                &nonfinal,
                GeneratedElementFeatures {
                    fields: std::sync::Arc::from([
                        Features::None,
                        Features::Conjunction(Conjunction::Or),
                        noun_phrase(Number::Singular),
                    ]),
                    present_fields: 1 << 1 | 1 << 2,
                    variant: None,
                },
            )),
        };
        assert!(
            generated_requirements_match(
                group,
                construction,
                &[Some(&first), Some(&invalid_interior_conjunction)],
            )
            .is_none()
        );
    }

    #[test]
    fn generated_requirements_enforce_group_complement_role() {
        let group = coordination::GROUPS[0];
        let construction = construction("shared_determiner_nominal");
        let rest = sequence(Conjunction::Or, Number::Singular);
        let complement_element = group
            .element_data
            .iter()
            .find(|element| element.name == "nominal_complement")
            .expect("the nominal complement element is declared");
        let complement = |variant_name| {
            let variant = complement_element
                .variants
                .iter()
                .position(|variant| variant.name == variant_name)
                .expect("the complement variant is declared");
            Features::GeneratedSequence {
                tail: std::sync::Arc::new(GeneratedSequenceFeatures::seed(
                    GeneratedElementFeatures {
                        fields: std::sync::Arc::default(),
                        present_fields: 1,
                        variant: Some(variant),
                    },
                )),
            }
        };
        let relative = complement("Relative");
        let fields = [None, None, Some(&rest), Some(&relative)];
        assert!(generated_requirements_match(group, construction, &fields).is_some());

        let prepositional = complement("Prepositional");
        let fields = [None, None, Some(&rest), Some(&prepositional)];
        assert!(generated_requirements_match(group, construction, &fields).is_none());

        let quantity = complement("Quantity");
        let fields = [None, None, Some(&rest), Some(&quantity)];
        assert!(generated_requirements_match(group, construction, &fields).is_none());

        let fields = [None, None, Some(&rest), None];
        assert!(generated_requirements_match(group, construction, &fields).is_some());
    }

    #[test]
    fn generated_noun_coordination_combines_agreement_by_conjunction() {
        let first = noun_phrase(Number::Singular);
        let and = sequence(Conjunction::And, Number::Singular);
        let fields = [Some(&first), Some(&and)];
        assert!(matches!(
            generated_construction_features(
                coordination::GROUPS[0],
                construction("noun_phrase_coordination"),
                &fields,
            ),
            Some(Features::NounPhrase {
                agreement: Some(PersonNumber {
                    number: Number::Plural,
                    ..
                }),
                ..
            })
        ));

        let or = sequence(Conjunction::Or, Number::Plural);
        let fields = [Some(&first), Some(&or)];
        assert!(matches!(
            generated_construction_features(
                coordination::GROUPS[0],
                construction("noun_phrase_coordination"),
                &fields,
            ),
            Some(Features::NounPhrase {
                agreement: Some(PersonNumber {
                    number: Number::Plural,
                    ..
                }),
                ..
            })
        ));
    }

    #[test]
    fn generated_noun_coordination_rejects_noncoordinating_then() {
        let first = noun_phrase(Number::Singular);
        let then = sequence(Conjunction::Then, Number::Singular);
        assert!(
            generated_construction_features(
                coordination::GROUPS[0],
                construction("noun_phrase_coordination"),
                &[Some(&first), Some(&then)]
            )
            .is_none()
        );
    }

    #[test]
    fn generated_feature_roles_follow_reordered_member_metadata() {
        use deckmaste_construction_compiler::runtime::ConstructionData;
        use deckmaste_construction_compiler::runtime::ElementData;
        use deckmaste_construction_compiler::runtime::FeatureCombinatorData;
        use deckmaste_construction_compiler::runtime::FieldData;
        use deckmaste_construction_compiler::runtime::FieldKindData;
        use deckmaste_construction_compiler::runtime::GroupData;

        const CONJUNCTION: FieldKindData = FieldKindData::Scalar {
            codec: "NounPhraseConjunction",
        };
        const ELEMENT_FIELDS: &[FieldData] = &[
            FieldData {
                name: "phrase",
                kind: FieldKindData::Subtree {
                    category: "NounPhrase",
                    boxed: false,
                },
            },
            FieldData {
                name: "comma",
                kind: FieldKindData::SurfaceScalar { codec: "Comma" },
            },
            FieldData {
                name: "conjunction",
                kind: FieldKindData::Optional {
                    inner: &CONJUNCTION,
                },
            },
        ];
        const ELEMENTS: &[ElementData] = &[ElementData {
            name: "member",
            bind_path: None,
            fields: ELEMENT_FIELDS,
            variants: &[],
            erased_builders: &[],
            erased_sequence_builder: None,
        }];
        const CONSTRUCTION_FIELDS: &[FieldData] = &[
            FieldData {
                name: "first",
                kind: FieldKindData::Subtree {
                    category: "NounPhrase",
                    boxed: true,
                },
            },
            FieldData {
                name: "rest",
                kind: FieldKindData::Sequence { element: "member" },
            },
        ];
        const COMBINATORS: &[FeatureCombinatorData] = &[FeatureCombinatorData {
            target: "first",
            combinator: "complete_noun_phrase_coordination",
            args: &["first", "rest"],
        }];
        const CONSTRUCTIONS: &[ConstructionData] = &[ConstructionData {
            id: "reordered",
            category: "NounPhrase",
            internal: false,
            own_type: Some("Reordered"),
            bind_path: None,
            lens: None,
            projection_variant: None,
            fields: CONSTRUCTION_FIELDS,
            witnesses: &[],
            deserialize: false,
            selection_unique: false,
            dominates: &[],
            dominated_by: &[],
            forms: &[],
            requirements: &[],
            recognition_requirements: &[],
            feature_combinators: COMBINATORS,
            evidence: None,
            erased_partial_builder: None,
            erased_builder: None,
            erased_projector: None,
            erased_linearizer:
                deckmaste_construction_compiler::runtime::unavailable_erased_linearizer,
        }];
        const GROUP: GroupData = GroupData {
            name: "reordered",
            backend: deckmaste_construction_compiler::runtime::ConstructionBackendData::Chart,
            elements: &["member"],
            element_data: ELEMENTS,
            lenses: &[],
            constructions: CONSTRUCTIONS,
        };

        let first = noun_phrase(Number::Singular);
        let rest = Features::GeneratedSequence {
            tail: std::sync::Arc::new(GeneratedSequenceFeatures::seed(GeneratedElementFeatures {
                fields: std::sync::Arc::from([
                    noun_phrase(Number::Singular),
                    Features::None,
                    Features::Conjunction(Conjunction::And),
                ]),
                present_fields: 1 << 0 | 1 << 2,
                variant: None,
            })),
        };
        assert!(matches!(
            generated_construction_features(
                &GROUP,
                &CONSTRUCTIONS[0],
                &[Some(&first), Some(&rest)],
            ),
            Some(Features::NounPhrase {
                agreement: Some(PersonNumber {
                    number: Number::Plural,
                    ..
                }),
                ..
            })
        ));
    }

    #[test]
    fn quantity_features_follow_declared_combinator_not_construction_id() {
        use deckmaste_construction_compiler::runtime::ConstructionData;
        use deckmaste_construction_compiler::runtime::FeatureCombinatorData;
        use deckmaste_construction_compiler::runtime::FieldData;
        use deckmaste_construction_compiler::runtime::FieldKindData;
        use deckmaste_construction_compiler::runtime::GroupData;

        const FIELDS: &[FieldData] = &[FieldData {
            name: "number",
            kind: FieldKindData::TypedScalar {
                value_type: "NumberLiteral",
                codec: "Numeral",
            },
        }];
        const COMBINATORS: &[FeatureCombinatorData] = &[FeatureCombinatorData {
            target: "features",
            combinator: "quantity_exact",
            args: &["number"],
        }];
        const CONSTRUCTIONS: &[ConstructionData] = &[ConstructionData {
            id: "registered_after_the_old_quantity_table",
            category: "Quantity",
            internal: false,
            own_type: None,
            bind_path: Some("Quantity"),
            lens: None,
            projection_variant: None,
            fields: FIELDS,
            witnesses: &[],
            deserialize: false,
            selection_unique: true,
            dominates: &[],
            dominated_by: &[],
            forms: &[],
            requirements: &[],
            recognition_requirements: &[],
            feature_combinators: COMBINATORS,
            evidence: None,
            erased_partial_builder: None,
            erased_builder: None,
            erased_projector: None,
            erased_linearizer:
                deckmaste_construction_compiler::runtime::unavailable_erased_linearizer,
        }];
        const GROUP: GroupData = GroupData {
            name: "quantity_probe",
            backend: deckmaste_construction_compiler::runtime::ConstructionBackendData::Chart,
            elements: &[],
            element_data: &[],
            lenses: &[],
            constructions: CONSTRUCTIONS,
        };
        let one = Features::Number { is_one: true };

        assert_eq!(
            generated_construction_features(&GROUP, &CONSTRUCTIONS[0], &[Some(&one)]),
            Some(Features::Quantity(QuantityFeatures {
                cardinality: NounCardinality::SingularOrMass,
                standalone_number: Number::Singular,
                is_one: true,
            })),
            "a registered declaration must not need a second construction-ID feature table",
        );
    }
}
