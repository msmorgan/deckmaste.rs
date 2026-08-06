use super::AdjectiveComparisonClass;
use super::AdjectiveComparisonState;
use super::Agreement;
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
use super::PredicateForm;
use super::Preposition;
use super::QuantityFeatures;
use super::Reduction;
use super::RuleTag;
use super::SetExceptionState;
use super::VerbSlot;
use super::clause;
use super::generated::GeneratedFeatureCombinator;
use super::noun_phrase_accepts_set_exception;

#[allow(clippy::too_many_lines, reason = "reduce matches on all rule tags")]
pub(super) fn reduce(
    tag: RuleTag,
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
) -> Option<Reduction<Features>> {
    let features = match tag {
        RuleTag::QuantityExact
        | RuleTag::QuantityAtLeast
        | RuleTag::QuantityOr
        | RuleTag::QuantityX
        | RuleTag::QuantityBoth
        | RuleTag::QuantityUpTo
        | RuleTag::QuantityThatMany
        | RuleTag::QuantityThatMuch
        | RuleTag::QuantityMoreThan
        | RuleTag::QuantityFewerThan
        | RuleTag::DeterminerClosed
        | RuleTag::DeterminerTarget
        | RuleTag::DeterminerQuantifiedTarget
        | RuleTag::DeterminerQuantity
        | RuleTag::DeterminerPossessiveThisCard => reduce_quantity_or_determiner(tag, children)?,
        RuleTag::FrequencyPhrase | RuleTag::FrequencyPhraseAdverb => Features::None,
        RuleTag::PossessiveNounBase
        | RuleTag::PossessiveNounDetermined
        | RuleTag::DeterminerPossessiveNoun
        | RuleTag::PossessiveNounAdjective => reduce_possessive_noun_phrase(tag, children)?,
        RuleTag::Adjective
        | RuleTag::AdjectivePhrase
        | RuleTag::AdjectivePhraseFaceUp
        | RuleTag::AdjectivePhraseFaceDown
        | RuleTag::AdjectivePhraseComparison
        | RuleTag::AdjectivePhraseDegreeMeasure
        | RuleTag::ComparisonStandard
        | RuleTag::ComparisonThan
        | RuleTag::ComparisonThanOrEqualTo
        | RuleTag::NominalNoun
        | RuleTag::NominalAdjective
        | RuleTag::NominalNounModifier
        | RuleTag::NominalCombatStepName
        | RuleTag::NominalNegatedModifier
        | RuleTag::NominalQuantityModifier
        | RuleTag::NominalPowerToughnessModifier
        | RuleTag::NominalDeterminer
        | RuleTag::NominalPrepositional
        | RuleTag::NominalInfinitive
        | RuleTag::NominalQuantityComplement
        | RuleTag::NominalKeywordSymbolArgument
        | RuleTag::PredicatedQualityFrom
        | RuleTag::PredicatedArgumentFromSingle
        | RuleTag::PredicatedArgumentFromExtend
        | RuleTag::NominalKeywordPredicatedArgument
        | RuleTag::PredicatedQualityBare
        | RuleTag::PredicatedArgumentBareSingle
        | RuleTag::PredicatedArgumentBareExtend
        | RuleTag::NominalKeywordAtomCarriedPredicatedArgument
        | RuleTag::NominalPowerToughnessComplement
        | RuleTag::NominalRelative
        | RuleTag::RulesObjectNominalBase
        | RuleTag::RulesObjectFollowupNominalRelative
        | RuleTag::RulesObjectFollowupNominalPrepositional
        | RuleTag::NominalReducedRecipientPassive
        | RuleTag::NominalPostpositiveAdjective
        | RuleTag::NominalPostpositiveAdjectiveConjoinedPrepositional
        | RuleTag::NominalPostpositiveAdjectiveConjoined
        | RuleTag::NominalPostpositiveAdjectiveAsyndetic
        | RuleTag::NominalPostpositiveAdjectiveOxford
        | RuleTag::NominalComparison
        | RuleTag::NominalDevotion
        | RuleTag::DevotionColorSingle
        | RuleTag::DevotionColorPair
        | RuleTag::NominalTimesClause
        | RuleTag::ModifierConjunctAdjective
        | RuleTag::ModifierConjunctNoun
        | RuleTag::ModifierConjunctNegated
        | RuleTag::ModifierListSingle
        | RuleTag::ModifierListComma
        | RuleTag::CoordinatedModifierConjoined
        | RuleTag::CoordinatedModifierOxford
        | RuleTag::NominalCoordinatedModifier => reduce_nominal(tag, children)?,
        RuleTag::NounPhraseNominal
        | RuleTag::RulesObjectNounPhrase
        | RuleTag::NounPhraseSetExceptionBare
        | RuleTag::NounPhraseSetExceptionFor
        | RuleTag::ReducedRecipientPassiveTheme
        | RuleTag::NounPhraseSubjectPronoun
        | RuleTag::NounPhraseObjectPronoun
        | RuleTag::NounPhraseReciprocal
        | RuleTag::NounPhraseQuantity
        | RuleTag::NounPhraseThisCard
        | RuleTag::NounPhraseFullThisCard
        | RuleTag::NounPhrasePossessiveThisCard
        | RuleTag::NounPhraseDemonstrative
        | RuleTag::NounPhrasePartitive
        | RuleTag::NounPhraseEachPartitive
        | RuleTag::NounPhraseAnyNumberOf
        | RuleTag::NounPhraseMinus
        | RuleTag::NounPhraseHalf
        | RuleTag::NounPhraseHalfRoundedUp
        | RuleTag::NounPhraseHalfRoundedDown
        | RuleTag::PrepositionalPhrase
        | RuleTag::PrepositionalPhraseListPair
        | RuleTag::PrepositionalPhraseListComma
        | RuleTag::PrepositionalPhraseSiblingCoordinated
        | RuleTag::PrepositionalObject => reduce_phrase(tag, children)?,
        RuleTag::Verb
        | RuleTag::VerbPhraseBase
        | RuleTag::VerbPhraseAuxiliary
        | RuleTag::VerbPhraseAuxiliaryProform
        | RuleTag::VerbPhraseDirectObject
        | RuleTag::VerbPhraseIndirectObject
        | RuleTag::VerbPhraseAdjective
        | RuleTag::VerbPhrasePrepositional
        | RuleTag::VerbPhrasePassiveSharedDeterminerPrepositional
        | RuleTag::VerbPhraseExceptBy
        | RuleTag::VerbPhraseInfinitive
        | RuleTag::VerbPhraseAdverb
        | RuleTag::VerbPhrasePreverbAdverb
        | RuleTag::VerbPhraseParticle
        | RuleTag::VerbPhraseCoinResult
        | RuleTag::VerbPhraseFrequency
        | RuleTag::VerbPhraseAbility
        | RuleTag::VerbPhraseQuotedAbility
        | RuleTag::VerbPhraseQuotedAbilityCoordination
        | RuleTag::VerbPhraseAbilityQuotedCoordination
        | RuleTag::VerbPhraseOracleSymbol
        | RuleTag::VerbPhraseSymbolSequence
        | RuleTag::VerbPhraseManaAmountCoordination
        | RuleTag::ManaAmountSymbol
        | RuleTag::ManaAmountSequence
        | RuleTag::ManaAmountListSingle
        | RuleTag::ManaAmountListComma
        | RuleTag::ManaAmountCoordination
        | RuleTag::ManaAmountCoordinationOxford
        | RuleTag::VerbPhrasePowerToughness
        | RuleTag::VerbPhraseQuantity
        | RuleTag::ReducedRecipientPassiveNominalAdjunct
        | RuleTag::InfinitiveTo
        | RuleTag::InfinitiveNotTo
        | RuleTag::GerundClauseBase
        | RuleTag::GerundClauseSubordinateAfter
        | RuleTag::SimpleClauseSubject
        | RuleTag::SimpleClauseSubjectDistributiveEach
        | RuleTag::SimpleClauseContractedSubject
        | RuleTag::SimpleClauseSubjectless
        | RuleTag::ClauseSimple
        | RuleTag::ClauseElliptical
        | RuleTag::ClauseCoordination
        | RuleTag::ClauseCoordinationComma
        | RuleTag::ClauseCoordinationAsyndetic
        | RuleTag::ClauseCoordinationCopularNounPrepositional
        | RuleTag::ClauseCoordinationCopularNounPrepositionalComma
        | RuleTag::ClauseCoordinationCopularNounPrepositionalAsyndetic
        | RuleTag::ClauseAdverbBefore
        | RuleTag::ClauseSentenceAdverbialBefore
        | RuleTag::ClausePrepositionalBefore
        | RuleTag::ClauseSubordinateBefore
        | RuleTag::ClauseSubordinateGerundBefore
        | RuleTag::ClauseSubordinateAfterElliptical
        | RuleTag::ClauseSubordinateAfter
        | RuleTag::ClauseSubordinateAfterComma
        | RuleTag::ClauseSubordinateAfterInfinitive
        | RuleTag::ClauseExistential
        | RuleTag::CopularRemainderNoun
        | RuleTag::CopularRemainderAdjective
        | RuleTag::CopularRemainderPrepositional
        | RuleTag::CopularRemainderPowerToughness
        | RuleTag::CopularRemainderPrepositionalAdjunct
        | RuleTag::CopularRemainderAdverb
        | RuleTag::CopularRemainderNegated
        | RuleTag::CopularRemainderDistributiveEach
        | RuleTag::ClauseCopular
        | RuleTag::ClauseContractedCopular
        | RuleTag::ClauseVariableValueConstraint
        | RuleTag::RelativeObject
        | RuleTag::RelativeObjectContractedSubject
        | RuleTag::RelativeSubjectContractedAuxiliary
        | RuleTag::RelativeSubject
        | RuleTag::RelativeSubjectDistributiveEach
        | RuleTag::RelativeContractedCopularNoun
        | RuleTag::RelativeContractedCopularAdjective
        | RuleTag::RelativeContractedCopularPrepositional
        | RuleTag::ClauseExcepted
        | RuleTag::ClauseRestrictionRun
        | RuleTag::ClauseRestrictionMember
        | RuleTag::ExceptionRiderSingle
        | RuleTag::ExceptionRiderConjoined
        | RuleTag::ExceptionRiderComma
        | RuleTag::ExceptionRiderOxford
        | RuleTag::VerbPhraseCausative
        | RuleTag::VerbPhraseCoordinatedAdjective
        | RuleTag::CopularRemainderCoordinatedAdjective
        | RuleTag::RelativeContractedCopularCoordinatedAdjective => {
            clause::reduce_clause(tag, children)?
        }
    };
    let mut local_cost = clause::reduction_cost(tag, children);
    if tag == RuleTag::NominalRelative
        && matches!(
            children.first().map(|child| child.features),
            Some(Features::Nominal {
                attachment: NominalAttachmentPhase::Prepositional {
                    nearer_relative_host: true,
                },
                ..
            })
        )
    {
        local_cost.precedence = local_cost.precedence.saturating_add(1);
    }
    Some(Reduction {
        features,
        local_cost,
    })
}

pub(super) fn reduce_possessive_noun_phrase(
    tag: RuleTag,
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
) -> Option<Reduced> {
    match tag {
        RuleTag::PossessiveNounBase => {
            let Features::Noun {
                form,
                initial_sound,
                ..
            } = children.first()?.features
            else {
                return None;
            };
            Some(Features::PossessiveNounPhrase {
                form: *form,
                initial_sound: *initial_sound,
                determined: false,
            })
        }
        RuleTag::PossessiveNounDetermined => {
            let Features::Determiner {
                cardinality,
                article,
                ..
            } = children.first()?.features
            else {
                return None;
            };
            let Features::PossessiveNounPhrase {
                form,
                initial_sound,
                determined,
            } = children.get(1)?.features
            else {
                return None;
            };
            if *determined
                || !cardinality_accepts(*cardinality, *form)
                || !article_accepts(*article, *initial_sound)
            {
                return None;
            }
            Some(Features::PossessiveNounPhrase {
                form: *form,
                initial_sound: *initial_sound,
                determined: true,
            })
        }
        RuleTag::DeterminerPossessiveNoun => Some(Features::Determiner {
            cardinality: NounCardinality::Unconstrained,
            article: None,
            demonstrative_this: false,
            set_exception_host: false,
        }),
        RuleTag::PossessiveNounAdjective => {
            let Features::Adjective {
                initial_sound,
                comparison:
                    AdjectiveComparisonState::NotComparative
                    | AdjectiveComparisonState::Pending(_)
                    | AdjectiveComparisonState::Complete,
                card_orientation: false,
                ..
            } = children.first()?.features
            else {
                return None;
            };
            let Features::PossessiveNounPhrase {
                form,
                determined: false,
                ..
            } = children.get(1)?.features
            else {
                return None;
            };
            Some(Features::PossessiveNounPhrase {
                form: *form,
                initial_sound: *initial_sound,
                determined: false,
            })
        }
        _ => None,
    }
}

pub(super) type Reduced = Features;

pub(super) fn reduce_quantity_or_determiner(
    tag: RuleTag,
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
) -> Option<Reduced> {
    match tag {
        RuleTag::QuantityExact => {
            let Features::Number { is_one } = children.first()?.features else {
                return None;
            };
            Some(Features::Quantity(number_quantity_features(*is_one)))
        }
        RuleTag::QuantityAtLeast
        | RuleTag::QuantityOr
        | RuleTag::QuantityX
        | RuleTag::QuantityBoth
        | RuleTag::QuantityMoreThan
        | RuleTag::QuantityFewerThan
        | RuleTag::QuantityThatMany
        | RuleTag::QuantityThatMuch => {
            let Features::Quantity(features) = children.first()?.features else {
                return None;
            };
            Some(Features::Quantity(*features))
        }
        RuleTag::QuantityUpTo => {
            let Features::Number { is_one } = children.get(2)?.features else {
                return None;
            };
            Some(Features::Quantity(number_quantity_features(*is_one)))
        }
        RuleTag::DeterminerClosed => {
            let Features::Determiner {
                cardinality,
                article,
                demonstrative_this,
                set_exception_host,
            } = children.first()?.features
            else {
                return None;
            };
            Some(Features::Determiner {
                cardinality: *cardinality,
                article: *article,
                demonstrative_this: *demonstrative_this,
                set_exception_host: *set_exception_host,
            })
        }
        RuleTag::DeterminerTarget => Some(Features::Determiner {
            cardinality: NounCardinality::SingularCount,
            article: None,
            demonstrative_this: false,
            set_exception_host: false,
        }),
        RuleTag::DeterminerQuantifiedTarget => {
            let Features::Quantity(QuantityFeatures { cardinality, .. }) =
                children.first()?.features
            else {
                return None;
            };
            Some(Features::Determiner {
                cardinality: target_cardinality(*cardinality),
                article: None,
                demonstrative_this: false,
                set_exception_host: false,
            })
        }
        RuleTag::DeterminerQuantity => {
            let Features::Quantity(QuantityFeatures { cardinality, .. }) =
                children.first()?.features
            else {
                return None;
            };
            Some(Features::Determiner {
                cardinality: *cardinality,
                article: None,
                demonstrative_this: false,
                set_exception_host: false,
            })
        }
        RuleTag::DeterminerPossessiveThisCard => {
            let Features::PossessiveThisCard { .. } = children.first()?.features else {
                return None;
            };
            Some(Features::Determiner {
                cardinality: NounCardinality::Unconstrained,
                article: None,
                demonstrative_this: false,
                set_exception_host: false,
            })
        }
        _ => None,
    }
}

/// Features for an arithmetic value expression: a third-person singular numeric
/// value with no pronoun case or bare-nominal adjunct.
pub(super) const fn arithmetic_value_features() -> Features {
    Features::NounPhrase {
        agreement: Some(Agreement {
            person: Person::Third,
            number: Number::Singular,
        }),
        coordination_domain: Some(CoordinationDomain::NonEntity),
        pronoun_case: None,
        adjunct: None,
        set_exception: SetExceptionState::Ineligible,
        coordination: NounPhraseCoordinationState::None,
        recipient_passive_theme: false,
        rules_object_followup: false,
    }
}

pub(super) const fn number_cardinality(is_one: bool) -> NounCardinality {
    if is_one {
        NounCardinality::SingularOrMass
    } else {
        NounCardinality::PluralOrMass
    }
}

pub(super) const fn number_quantity_features(is_one: bool) -> QuantityFeatures {
    QuantityFeatures {
        cardinality: number_cardinality(is_one),
        standalone_number: if is_one { Number::Singular } else { Number::Plural },
        is_one,
    }
}

pub(super) const fn target_cardinality(cardinality: NounCardinality) -> NounCardinality {
    match cardinality {
        NounCardinality::SingularOrMass => NounCardinality::SingularCount,
        NounCardinality::PluralOrMass => NounCardinality::PluralCount,
        other => other,
    }
}

#[allow(
    clippy::too_many_lines,
    reason = "quantity/reduction mapping is intentionally long"
)]
pub(super) fn reduce_nominal(
    tag: RuleTag,
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
) -> Option<Reduced> {
    match tag {
        RuleTag::Adjective => Some(propagate(children.first()?)),
        RuleTag::AdjectivePhraseFaceUp | RuleTag::AdjectivePhraseFaceDown => {
            Some(Features::Adjective {
                initial_sound: InitialSound::Consonant,
                comparison: AdjectiveComparisonState::NotComparative,
                card_orientation: true,
                demonstrative_shared_determiner: true,
            })
        }
        RuleTag::AdjectivePhrase => {
            let Features::Adjective { .. } = children.first()?.features else {
                return None;
            };
            Some(children.first()?.features.clone())
        }
        RuleTag::AdjectivePhraseComparison => {
            let Features::Adjective {
                initial_sound,
                comparison: AdjectiveComparisonState::Pending(_),
                card_orientation,
                demonstrative_shared_determiner,
            } = children.first()?.features
            else {
                return None;
            };
            Some(Features::Adjective {
                initial_sound: *initial_sound,
                comparison: AdjectiveComparisonState::Complete,
                card_orientation: *card_orientation,
                demonstrative_shared_determiner: *demonstrative_shared_determiner,
            })
        }
        RuleTag::AdjectivePhraseDegreeMeasure => {
            let Features::Number { .. } = children.first()?.features else {
                return None;
            };
            let Features::Adjective {
                initial_sound,
                // Only the `N or <word>` comparison class takes a degree
                // measure; `other` (`ThanOnly`) must not — see
                // `AdjectiveComparisonState`.
                comparison:
                    AdjectiveComparisonState::Pending(AdjectiveComparisonClass::OrComparative),
                card_orientation: false,
                demonstrative_shared_determiner,
            } = children.get(1)?.features
            else {
                return None;
            };
            Some(Features::Adjective {
                // Inert: `Measured` never reaches an article-bearing position
                // (`nominal_with_prefix` rejects it), so this is carried, not
                // used.
                initial_sound: *initial_sound,
                comparison: AdjectiveComparisonState::Measured,
                card_orientation: false,
                demonstrative_shared_determiner: *demonstrative_shared_determiner,
            })
        }
        RuleTag::ComparisonStandard
        | RuleTag::ComparisonThan
        | RuleTag::ComparisonThanOrEqualTo => Some(Features::None),
        RuleTag::NominalNoun => {
            let child = children.first()?;
            let Features::Noun {
                identity,
                coordination_domain,
                form,
                initial_sound,
                adjunct,
                opaque,
                recipient_passive_theme,
            } = child.features
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
        RuleTag::NominalAdjective => {
            let Features::Adjective {
                initial_sound,
                comparison,
                card_orientation: false,
                demonstrative_shared_determiner,
            } = children.first()?.features
            else {
                return None;
            };
            nominal_with_prefix(
                children.get(1)?,
                *initial_sound,
                false,
                *comparison,
                *demonstrative_shared_determiner,
            )
        }
        RuleTag::NominalCombatStepName => {
            // Mirrors `NominalNoun`'s Noun→Nominal base case (this rule
            // *creates* a nominal from terminals, not `NominalNounModifier`,
            // which prepends onto an existing one). `initial_sound` is
            // overridden to `Consonant`: the phrase's true leftmost token is
            // `declare`, not the vowel-initial `attackers`.
            let Features::Noun {
                identity,
                coordination_domain,
                form,
                adjunct,
                ..
            } = children.get(2)?.features
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
        RuleTag::NominalNounModifier => {
            let Features::Noun {
                coordination_domain: modifier_domain,
                initial_sound,
                opaque: modifier_opaque,
                ..
            } = children.first()?.features
            else {
                return None;
            };
            let Features::Nominal {
                coordination_domain: head_domain,
                opaque_head,
                ..
            } = children.get(1)?.features
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
            nominal_with_prefix(
                children.get(1)?,
                *initial_sound,
                false,
                AdjectiveComparisonState::NotComparative,
                true,
            )
        }
        RuleTag::NominalQuantityModifier => {
            // The quantity class keeps a fixed consonant onset. The corpus prints no
            // vowel-onset quantity modifier under an indefinite article in either
            // direction (`an eight …`, `a eight …`, `a one …`, `an one …`: zero
            // supported witnesses each), and deriving the sound here would mean
            // widening `QuantityFeatures`, which sits inside the Earley item key
            // (`ItemKey::prefix_features`) and is consumed at every quantity
            // position in the grammar. The divergence from
            // `Renderer::modifier_initial_sound`'s spelling-derived quantity arm
            // is known, unreachable on the supported corpus, and left as ticketed
            // residue.
            // A quantity-modified nominal cannot be the continuation of a
            // singular demonstrative shared across coordination: `this
            // creature and up to one other ...` starts a second, independently
            // determined noun phrase. Other determiner classes are checked by
            // their ordinary cardinality gate.
            nominal_with_prefix(
                children.get(1)?,
                InitialSound::Consonant,
                false,
                AdjectiveComparisonState::NotComparative,
                false,
            )
        }
        RuleTag::NominalPowerToughnessModifier => {
            let Features::PowerToughness { initial_sound } = children.first()?.features else {
                return None;
            };
            nominal_with_prefix(
                children.get(1)?,
                *initial_sound,
                false,
                AdjectiveComparisonState::NotComparative,
                true,
            )
        }
        RuleTag::NominalNegatedModifier => {
            // The `non-` prefix fixes the phrase's initial sound to a consonant
            // (`a nonland permanent`, never `an`), regardless of the base.
            nominal_with_prefix(
                children.get(1)?,
                InitialSound::Consonant,
                false,
                AdjectiveComparisonState::NotComparative,
                true,
            )
        }
        RuleTag::NominalDeterminer => {
            let Features::Determiner {
                cardinality,
                article,
                set_exception_host,
                ..
            } = children.first()?.features
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
            } = children.get(1)?.features
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
        RuleTag::NominalPrepositional | RuleTag::RulesObjectFollowupNominalPrepositional => {
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
                shared_determiner_open: _,
                demonstrative_shared_determiner,
                recipient_passive_theme,
            } = children.first()?.features
            else {
                return None;
            };
            let Features::PrepositionalPhrase {
                nominal_attachment: true,
                nearer_relative_host,
                ..
            } = children.get(1)?.features
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
        RuleTag::NominalInfinitive => {
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
                shared_determiner_open: _,
                demonstrative_shared_determiner,
                recipient_passive_theme,
            } = children.first()?.features
            else {
                return None;
            };
            let Features::InfinitiveClause = children.get(1)?.features else {
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
        RuleTag::NominalKeywordSymbolArgument => {
            let Features::Noun {
                identity,
                coordination_domain,
                form: NounForm::Mass,
                initial_sound,
                adjunct,
                ..
            } = children.first()?.features
            else {
                return None;
            };
            // The second child is the terminal symbol/symbol-sequence match;
            // it carries `Features::None` and no reduce-time validation
            // beyond that shape, per the grammar mechanism section.
            if !matches!(children.get(1)?.features, Features::None) {
                return None;
            }
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
        RuleTag::PredicatedQualityFrom | RuleTag::PredicatedQualityBare => {
            Some(Features::PredicatedQuality)
        }
        RuleTag::PredicatedArgumentFromSingle | RuleTag::PredicatedArgumentBareSingle => {
            if !matches!(children.first()?.features, Features::PredicatedQuality) {
                return None;
            }
            Some(Features::PredicatedArgument)
        }
        RuleTag::PredicatedArgumentFromExtend | RuleTag::PredicatedArgumentBareExtend => {
            if !matches!(children.first()?.features, Features::PredicatedArgument) {
                return None;
            }
            // Defensive invariant, mirroring the `accepts_prefix` gate: only
            // `and` may extend the list; an `or` must never be normalized
            // away (the confirmed Stage B deferral).
            if !matches!(
                children.get(1)?.features,
                Features::Conjunction(Conjunction::And)
            ) {
                return None;
            }
            if !matches!(children.get(2)?.features, Features::PredicatedQuality) {
                return None;
            }
            Some(Features::PredicatedArgument)
        }
        RuleTag::NominalKeywordPredicatedArgument
        | RuleTag::NominalKeywordAtomCarriedPredicatedArgument => {
            let Features::Noun {
                identity,
                coordination_domain,
                form: NounForm::Mass,
                initial_sound,
                adjunct,
                ..
            } = children.first()?.features
            else {
                return None;
            };
            if !matches!(children.get(1)?.features, Features::PredicatedArgument) {
                return None;
            }
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
        RuleTag::RulesObjectNominalBase => {
            let nominal = children.first()?;
            matches!(
                nominal.features,
                Features::Nominal {
                    attachment: NominalAttachmentPhase::RulesObjectRelative,
                    ..
                }
            )
            .then(|| propagate(nominal))
        }
        RuleTag::NominalQuantityComplement
        | RuleTag::NominalRelative
        | RuleTag::RulesObjectFollowupNominalRelative
        | RuleTag::NominalReducedRecipientPassive => {
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
            } = children.first()?.features
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
            let is_relative = matches!(
                tag,
                RuleTag::NominalRelative | RuleTag::RulesObjectFollowupNominalRelative
            );
            let rules_object_relative = matches!(
                children.get(1)?.features,
                Features::RelativeClause {
                    gap: GapState::Object,
                    object_gap_requires_rules_object: true,
                    ..
                }
            );
            if is_relative && *form == NounForm::Mass && rules_object_relative {
                return None;
            }
            if is_relative
                && let Features::RelativeClause {
                    gap: GapState::Subject,
                    antecedent_agreement: Some(agreement),
                    ..
                } = children.get(1)?.features
                && agreement.number
                    != match form {
                        NounForm::Plural => Number::Plural,
                        NounForm::Singular | NounForm::Mass => Number::Singular,
                    }
            {
                return None;
            }
            if tag == RuleTag::NominalReducedRecipientPassive {
                if *attachment == NominalAttachmentPhase::RelativeBareCopula {
                    return None;
                }
                let Features::VerbPhrase {
                    form: PredicateForm::PastParticiple,
                    passive: false,
                    object,
                    indirect_object: false,
                    frame,
                    ..
                } = children.get(1)?.features
                else {
                    return None;
                };
                if !frame.is_recipient_passive() || !object.has_direct_object() {
                    return None;
                }
            }
            Some(Features::Nominal {
                head: head.clone(),
                coordination_domain: *coordination_domain,
                form: *form,
                initial_sound: *initial_sound,
                determined: *determined,
                modified: *modified,
                leading_opacity: *leading_opacity,
                attachment: match tag {
                    RuleTag::NominalRelative | RuleTag::RulesObjectFollowupNominalRelative => {
                        if matches!(
                            children.get(1)?.features,
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
                        }
                    }
                    RuleTag::NominalReducedRecipientPassive => {
                        NominalAttachmentPhase::ReducedRecipientPassive
                    }
                    _ => *attachment,
                },
                comparison: *comparison,
                adjunct: *adjunct,
                opaque_head: *opaque_head,
                set_exception_host: *set_exception_host,
                shared_determiner_open: *shared_determiner_open
                    && !(is_relative && *form != NounForm::Singular),
                demonstrative_shared_determiner: *demonstrative_shared_determiner,
                recipient_passive_theme: *recipient_passive_theme,
            })
        }
        RuleTag::NominalPowerToughnessComplement => {
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
            } = children.first()?.features
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
        RuleTag::NominalPostpositiveAdjective => {
            let Features::Adjective {
                card_orientation: false,
                // Everything but `Measured`: a degree phrase must not land
                // postnominally (`creature 2 greater`).
                comparison:
                    AdjectiveComparisonState::NotComparative
                    | AdjectiveComparisonState::Complete
                    | AdjectiveComparisonState::Pending(_),
                ..
            } = children.get(1)?.features
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
                leading_opacity,
                attachment: NominalAttachmentPhase::Open,
                comparison: AdjectiveComparisonState::NotComparative,
                adjunct,
                opaque_head,
                set_exception_host,
                shared_determiner_open,
                demonstrative_shared_determiner,
                recipient_passive_theme,
            } = children.first()?.features
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
        RuleTag::NominalPostpositiveAdjectiveConjoinedPrepositional
        | RuleTag::NominalPostpositiveAdjectiveConjoined
        | RuleTag::NominalPostpositiveAdjectiveAsyndetic
        | RuleTag::NominalPostpositiveAdjectiveOxford => {
            let (conjunction_index, adjective_index) = match tag {
                RuleTag::NominalPostpositiveAdjectiveConjoinedPrepositional
                | RuleTag::NominalPostpositiveAdjectiveConjoined => (Some(1), 2),
                RuleTag::NominalPostpositiveAdjectiveAsyndetic => (None, 2),
                RuleTag::NominalPostpositiveAdjectiveOxford => (Some(2), 3),
                _ => unreachable!("matched postpositive coordination tag"),
            };
            if let Some(index) = conjunction_index
                && !matches!(
                    children.get(index)?.features,
                    Features::Conjunction(Conjunction::And | Conjunction::Or | Conjunction::AndOr)
                )
            {
                return None;
            }
            let Features::Adjective {
                card_orientation: false,
                comparison:
                    AdjectiveComparisonState::NotComparative
                    | AdjectiveComparisonState::Complete
                    | AdjectiveComparisonState::Pending(_),
                ..
            } = children.get(adjective_index)?.features
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
                leading_opacity,
                attachment: NominalAttachmentPhase::PostpositiveAdjective,
                comparison,
                adjunct,
                opaque_head,
                set_exception_host,
                shared_determiner_open,
                demonstrative_shared_determiner,
                recipient_passive_theme,
            } = children.first()?.features
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
        RuleTag::NominalComparison => {
            let Features::Nominal {
                head,
                coordination_domain,
                form,
                initial_sound,
                determined,
                modified,
                leading_opacity,
                attachment:
                    NominalAttachmentPhase::Open | NominalAttachmentPhase::Prepositional { .. },
                comparison: AdjectiveComparisonState::Pending(_),
                adjunct,
                opaque_head,
                set_exception_host,
                shared_determiner_open,
                demonstrative_shared_determiner,
                recipient_passive_theme,
            } = children.first()?.features
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
        RuleTag::DevotionColorSingle => {
            // The single-color argument: the child is a bare color word.
            matches!(children.first()?.features, Features::None).then_some(Features::None)
        }
        RuleTag::DevotionColorPair => {
            // The two-color argument is joined by `and`, never `or`.
            matches!(
                children.get(1)?.features,
                Features::Conjunction(Conjunction::And)
            )
            .then_some(Features::None)
        }
        RuleTag::NominalDevotion => {
            // `devotion to <color>` is a singular measured value that a
            // possessive determiner (`your`) then wraps.
            let Features::Noun { identity, .. } = children.first()?.features else {
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
        RuleTag::NominalTimesClause => {
            // `times <clause>`: a plural `times` head with a finite clause as a
            // reduced adjunct-relative complement.
            let Features::Noun { identity, .. } = children.first()?.features else {
                return None;
            };
            let Features::Clause { finite: true, .. } = children.get(1)?.features else {
                return None;
            };
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
        RuleTag::ModifierConjunctAdjective => {
            // Only plain attributive adjectives coordinate as modifiers: a
            // comparative or a card-orientation adjective is not an atom of a
            // color/type/supertype list.
            let Features::Adjective {
                initial_sound,
                comparison: AdjectiveComparisonState::NotComparative,
                card_orientation: false,
                ..
            } = children.first()?.features
            else {
                return None;
            };
            Some(Features::CoordinatedModifier {
                initial_sound: *initial_sound,
                all_adjectives: true,
                noun_heads: Vec::new(),
            })
        }
        RuleTag::ModifierConjunctNoun => {
            let Features::Noun {
                identity,
                initial_sound,
                ..
            } = children.first()?.features
            else {
                return None;
            };
            Some(Features::CoordinatedModifier {
                initial_sound: *initial_sound,
                all_adjectives: false,
                noun_heads: identity.iter().cloned().collect(),
            })
        }
        RuleTag::ModifierConjunctNegated => {
            // A `non-` conjunct always renders `non…`, a consonant onset.
            Some(Features::CoordinatedModifier {
                initial_sound: InitialSound::Consonant,
                all_adjectives: false,
                noun_heads: Vec::new(),
            })
        }
        RuleTag::ModifierListSingle => {
            let Features::CoordinatedModifier { .. } = children.first()?.features else {
                return None;
            };
            Some(children.first()?.features.clone())
        }
        RuleTag::ModifierListComma => {
            // The list keeps its first conjunct's onset regardless of what a
            // comma continuation appends; `all_adjectives` holds only while every
            // appended conjunct is itself an adjective.
            let Features::CoordinatedModifier {
                initial_sound,
                all_adjectives,
                noun_heads,
            } = children.first()?.features
            else {
                return None;
            };
            let Features::CoordinatedModifier {
                all_adjectives: appended_adjectives,
                noun_heads: appended_noun_heads,
                ..
            } = children.get(2)?.features
            else {
                return None;
            };
            Some(Features::CoordinatedModifier {
                initial_sound: *initial_sound,
                all_adjectives: *all_adjectives && *appended_adjectives,
                noun_heads: noun_heads
                    .iter()
                    .chain(appended_noun_heads)
                    .cloned()
                    .collect(),
            })
        }
        RuleTag::CoordinatedModifierConjoined | RuleTag::CoordinatedModifierOxford => {
            let Features::CoordinatedModifier {
                initial_sound,
                all_adjectives,
                noun_heads,
            } = children.first()?.features
            else {
                return None;
            };
            let conjunction_index =
                if tag == RuleTag::CoordinatedModifierConjoined { 1 } else { 2 };
            // `and`/`or`/`and/or` close a modifier list; sequencing and
            // additive conjunctions never do.
            let Features::Conjunction(conjunction) = children.get(conjunction_index)?.features
            else {
                return None;
            };
            match conjunction {
                Conjunction::And | Conjunction::Or | Conjunction::AndOr => {}
                Conjunction::Then | Conjunction::Plus => return None,
            }
            let Features::CoordinatedModifier {
                all_adjectives: closing_adjectives,
                noun_heads: closing_noun_heads,
                ..
            } = children.last()?.features
            else {
                return None;
            };
            Some(Features::CoordinatedModifier {
                initial_sound: *initial_sound,
                all_adjectives: *all_adjectives && *closing_adjectives,
                noun_heads: noun_heads
                    .iter()
                    .chain(closing_noun_heads)
                    .cloned()
                    .collect(),
            })
        }
        RuleTag::NominalCoordinatedModifier => {
            let Features::CoordinatedModifier {
                initial_sound,
                noun_heads,
                ..
            } = children.first()?.features
            else {
                return None;
            };
            let Features::Nominal { head, .. } = children.get(1)?.features else {
                return None;
            };
            if head.as_ref().is_some_and(|head| noun_heads.contains(head)) {
                return None;
            }
            nominal_with_prefix(
                children.get(1)?,
                *initial_sound,
                false,
                AdjectiveComparisonState::NotComparative,
                true,
            )
        }
        _ => None,
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "phrase feature reduction is an irreducible flat dispatch over grammar rule tags"
)]
pub(super) fn reduce_phrase(
    tag: RuleTag,
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
) -> Option<Reduced> {
    match tag {
        RuleTag::NounPhraseSetExceptionBare | RuleTag::NounPhraseSetExceptionFor => {
            let host = children.first()?;
            if !noun_phrase_accepts_set_exception(host.features)
                || !matches!(children.last()?.features, Features::NounPhrase { .. })
            {
                return None;
            }
            let Features::NounPhrase {
                agreement,
                coordination_domain,
                pronoun_case,
                adjunct,
                ..
            } = host.features
            else {
                return None;
            };
            Some(Features::NounPhrase {
                agreement: *agreement,
                coordination_domain: *coordination_domain,
                pronoun_case: *pronoun_case,
                adjunct: *adjunct,
                set_exception: SetExceptionState::Closed,
                coordination: NounPhraseCoordinationState::None,
                recipient_passive_theme: false,
                rules_object_followup: false,
            })
        }
        RuleTag::NounPhraseNominal
        | RuleTag::RulesObjectNounPhrase
        | RuleTag::ReducedRecipientPassiveTheme => {
            let Features::Nominal {
                coordination_domain,
                form,
                determined,
                modified,
                attachment,
                adjunct,
                set_exception_host,
                recipient_passive_theme,
                ..
            } = children.first()?.features
            else {
                return None;
            };
            if tag == RuleTag::ReducedRecipientPassiveTheme
                && (*attachment != NominalAttachmentPhase::Open || !*recipient_passive_theme)
            {
                return None;
            }
            let agreement = Some(Agreement {
                person: Person::Third,
                number: match form {
                    NounForm::Plural => Number::Plural,
                    NounForm::Singular | NounForm::Mass => Number::Singular,
                },
            });
            // A completely bare nominal (no determiner, no modifier) must not
            // surface its bare-temporal/manner-adjunct licensing: it is
            // always the bare residue of a compound head (`step` left over
            // from `draw step`), never a legitimate standalone adjunct
            // [declarestep-plan-C.md §2].
            let adjunct = if *determined || *modified { *adjunct } else { None };
            Some(Features::NounPhrase {
                agreement,
                coordination_domain: *coordination_domain,
                pronoun_case: None,
                adjunct,
                set_exception: if *set_exception_host {
                    SetExceptionState::Host
                } else {
                    SetExceptionState::Ineligible
                },
                coordination: NounPhraseCoordinationState::None,
                recipient_passive_theme: *recipient_passive_theme,
                rules_object_followup: tag == RuleTag::RulesObjectNounPhrase,
            })
        }
        RuleTag::NounPhraseSubjectPronoun | RuleTag::NounPhraseObjectPronoun => {
            noun_phrase_from_pronoun(children.first()?)
        }
        RuleTag::NounPhraseReciprocal => noun_phrase_from_pronoun(children.first()?),
        RuleTag::NounPhraseQuantity => {
            let Features::Quantity(QuantityFeatures {
                standalone_number,
                is_one,
                ..
            }) = children.first()?.features
            else {
                return None;
            };
            Some(Features::NounPhrase {
                agreement: Some(Agreement {
                    person: Person::Third,
                    number: *standalone_number,
                }),
                coordination_domain: Some(if *is_one {
                    CoordinationDomain::SelectionContinuation
                } else {
                    CoordinationDomain::NonEntity
                }),
                pronoun_case: None,
                adjunct: None,
                set_exception: SetExceptionState::Ineligible,
                coordination: NounPhraseCoordinationState::None,
                recipient_passive_theme: false,
                rules_object_followup: false,
            })
        }
        RuleTag::NounPhraseThisCard | RuleTag::NounPhraseFullThisCard => {
            let Features::NounPhrase {
                agreement,
                coordination_domain,
                pronoun_case,
                adjunct,
                set_exception,
                coordination,
                recipient_passive_theme,
                rules_object_followup,
            } = children.first()?.features
            else {
                return None;
            };
            Some(Features::NounPhrase {
                agreement: *agreement,
                coordination_domain: *coordination_domain,
                pronoun_case: *pronoun_case,
                adjunct: *adjunct,
                set_exception: *set_exception,
                coordination: *coordination,
                recipient_passive_theme: *recipient_passive_theme,
                rules_object_followup: *rules_object_followup,
            })
        }
        RuleTag::NounPhrasePossessiveThisCard => {
            let Features::PossessiveThisCard { agreement } = children.first()?.features else {
                return None;
            };
            Some(Features::NounPhrase {
                agreement: Some(*agreement),
                coordination_domain: Some(CoordinationDomain::Entity),
                pronoun_case: None,
                adjunct: None,
                set_exception: SetExceptionState::Ineligible,
                coordination: NounPhraseCoordinationState::None,
                recipient_passive_theme: false,
                rules_object_followup: false,
            })
        }
        RuleTag::NounPhraseDemonstrative => {
            let Features::NounPhrase { .. } = children.first()?.features else {
                return None;
            };
            Some(children.first()?.features.clone())
        }
        RuleTag::NounPhrasePartitive => {
            let Features::Quantity(QuantityFeatures {
                standalone_number, ..
            }) = children.first()?.features
            else {
                return None;
            };
            Some(Features::NounPhrase {
                agreement: Some(Agreement {
                    person: Person::Third,
                    number: *standalone_number,
                }),
                coordination_domain: Some(CoordinationDomain::SelectionHost),
                pronoun_case: None,
                adjunct: None,
                set_exception: SetExceptionState::Ineligible,
                coordination: NounPhraseCoordinationState::None,
                recipient_passive_theme: false,
                rules_object_followup: false,
            })
        }
        RuleTag::NounPhraseEachPartitive => {
            // The `each` slot only ever scans "each", so the distributive
            // partitive is always grammatically third-person singular.
            let Features::Determiner { .. } = children.first()?.features else {
                return None;
            };
            Some(Features::NounPhrase {
                agreement: Some(Agreement {
                    person: Person::Third,
                    number: Number::Singular,
                }),
                coordination_domain: Some(CoordinationDomain::SelectionHost),
                pronoun_case: None,
                adjunct: None,
                set_exception: SetExceptionState::Ineligible,
                coordination: NounPhraseCoordinationState::None,
                recipient_passive_theme: false,
                rules_object_followup: false,
            })
        }
        RuleTag::NounPhraseAnyNumberOf => {
            // Defensively recheck the specialized lowered children as the
            // neighbouring arms do. The `any`/`number` slots only ever scan
            // those two literal words, so this arm's sole remaining
            // condition is the fourth child's agreement: the notional
            // plural reading is licensed only when the final noun phrase
            // is third-person plural [`anof` round].
            let Features::Determiner { .. } = children.first()?.features else {
                return None;
            };
            let Features::Noun {
                form: NounForm::Singular,
                ..
            } = children.get(1)?.features
            else {
                return None;
            };
            let Features::Preposition(Preposition::Of) = children.get(2)?.features else {
                return None;
            };
            let Features::NounPhrase {
                agreement:
                    Some(Agreement {
                        number: Number::Plural,
                        ..
                    }),
                coordination_domain,
                ..
            } = children.get(3)?.features
            else {
                return None;
            };
            Some(Features::NounPhrase {
                agreement: Some(Agreement {
                    person: Person::Third,
                    number: Number::Plural,
                }),
                coordination_domain: *coordination_domain,
                pronoun_case: None,
                adjunct: None,
                set_exception: SetExceptionState::Ineligible,
                coordination: NounPhraseCoordinationState::None,
                recipient_passive_theme: false,
                rules_object_followup: false,
            })
        }
        RuleTag::PrepositionalPhraseListPair | RuleTag::PrepositionalPhraseListComma => {
            let Features::PrepositionalPhrase { .. } = children.first()?.features else {
                return None;
            };
            let Features::PrepositionalPhrase { .. } = children.get(2)?.features else {
                return None;
            };
            Some(children.first()?.features.clone())
        }
        RuleTag::PrepositionalPhraseSiblingCoordinated => {
            // Every member repeats its own preposition, so the coordination
            // keeps the first member's: that is what introduces the phrase and
            // what an enclosing slot selects on. The object is never a shared
            // determiner group — each member brought its own.
            let Features::PrepositionalPhrase {
                preposition,
                nominal_attachment,
                ..
            } = children.first()?.features
            else {
                return None;
            };
            let conjunction_index = if children.len() == 4 { 2 } else { 1 };
            let Features::Conjunction(Conjunction::And | Conjunction::Or | Conjunction::AndOr) =
                children.get(conjunction_index)?.features
            else {
                return None;
            };
            // The right edge is the last member, so a following relative
            // attaches against that member's object, not the first's.
            let Features::PrepositionalPhrase {
                nearer_relative_host,
                ..
            } = children.last()?.features
            else {
                return None;
            };
            Some(Features::PrepositionalPhrase {
                preposition: *preposition,
                nominal_attachment: *nominal_attachment,
                shared_determiner_object: false,
                nearer_relative_host: *nearer_relative_host,
            })
        }
        RuleTag::NounPhraseMinus => {
            // Both operands must be noun-phrase values; the result is a
            // singular numeric value.
            let Features::NounPhrase { .. } = children.first()?.features else {
                return None;
            };
            let Features::NounPhrase { .. } = children.get(2)?.features else {
                return None;
            };
            Some(arithmetic_value_features())
        }
        RuleTag::NounPhraseHalf
        | RuleTag::NounPhraseHalfRoundedUp
        | RuleTag::NounPhraseHalfRoundedDown => {
            let Features::NounPhrase { .. } = children.get(1)?.features else {
                return None;
            };
            Some(arithmetic_value_features())
        }
        RuleTag::PrepositionalPhrase => {
            let Features::Preposition(preposition) = children.first()?.features else {
                return None;
            };
            let Features::PrepositionalObject {
                gerund,
                shared_determiner,
            } = children.get(1)?.features
            else {
                return None;
            };
            Some(Features::PrepositionalPhrase {
                preposition: *preposition,
                nominal_attachment: !(*preposition == Preposition::By && *gerund),
                shared_determiner_object: *shared_determiner,
                nearer_relative_host: false,
            })
        }
        RuleTag::PrepositionalObject => Some(Features::PrepositionalObject {
            gerund: matches!(children.first()?.features, Features::GerundClause),
            shared_determiner: matches!(
                children.first()?.features,
                Features::NounPhrase {
                    coordination: NounPhraseCoordinationState::Shared,
                    ..
                }
            ),
        }),
        _ => None,
    }
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
        super::rules::GeneratedRuleContext::Value => (None, 0_usize),
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
    let noun_phrase_features = generated_construction_features(rule.group, construction, &fields)?;
    let features = match (rule.context, preposition) {
        (super::rules::GeneratedRuleContext::Value, None) => noun_phrase_features,
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
        PredicateData::In { path, allowed } => generated_selected_elements(
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
        ),
        PredicateData::IsSome { path } => generated_selected_elements(
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
        ),
        PredicateData::IsNone { path } => generated_selected_elements(
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
        ),
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
                shared_determiner_object: false,
                nearer_relative_host: preposition == Preposition::To,
            })
        }
        GeneratedFeatureCombinator::SharedDeterminerCoordination => {
            Some(Features::PrepositionalPhrase {
                preposition,
                nominal_attachment: true,
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
                        .then_some((field_index, surface_index))
                })
        })
        .all(|(field_index, comma_index)| {
            let Some(Features::GeneratedSequence { tail }) =
                fields.get(field_index).copied().flatten()
            else {
                return false;
            };
            let expected = tail.len >= 2;
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
            [Some(determiner), Some(first), Some(rest), _complements],
        ) => shared_determiner_coordination_features(
            determiner,
            first,
            rest,
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
            Some(Agreement {
                person: Person::Third,
                number: match first.form {
                    NounForm::Plural => Number::Plural,
                    NounForm::Singular | NounForm::Mass => Number::Singular,
                },
            }),
            Some(Agreement {
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
    first: Option<Agreement>,
    last: Option<Agreement>,
) -> Option<Agreement> {
    match conjunction {
        Conjunction::And => Some(Agreement {
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
    shared_determiner_open: bool,
    demonstrative_shared_determiner: bool,
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
            shared_determiner_open: true,
            demonstrative_shared_determiner: true,
        }),
        Features::Nominal {
            coordination_domain,
            form,
            initial_sound,
            determined: false,
            adjunct,
            shared_determiner_open,
            demonstrative_shared_determiner,
            ..
        } => Some(NominalCoordinationMember {
            coordination_domain: *coordination_domain,
            form: *form,
            initial_sound: *initial_sound,
            adjunct: *adjunct,
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

pub(super) fn propagate(child: &Child<'_, EnglishGrammar<'_, '_>>) -> Reduced {
    child.features.clone()
}

pub(super) fn nominal_with_prefix(
    nominal: &Child<'_, EnglishGrammar<'_, '_>>,
    initial_sound: InitialSound,
    leading_opacity: bool,
    prefix_comparison: AdjectiveComparisonState,
    prefix_demonstrative_shared_determiner: bool,
) -> Option<Reduced> {
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
    } = nominal.features
    else {
        return None;
    };
    if *determined {
        return None;
    }
    let comparison = match (prefix_comparison, *comparison) {
        // `Measured` is predicative-only: reject it attributively in either
        // position.
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
        // The one site that sets this: adding any modifier (adjective, noun
        // modifier, quantity, power/toughness, negated modifier) through
        // this shared helper makes the nominal non-bare.
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

pub(super) fn noun_phrase_from_pronoun(
    child: &Child<'_, EnglishGrammar<'_, '_>>,
) -> Option<Reduced> {
    let Features::NounPhrase {
        agreement,
        coordination_domain,
        pronoun_case,
        adjunct,
        set_exception,
        coordination,
        recipient_passive_theme,
        rules_object_followup,
    } = child.features
    else {
        return None;
    };
    Some(Features::NounPhrase {
        agreement: *agreement,
        coordination_domain: *coordination_domain,
        pronoun_case: *pronoun_case,
        adjunct: *adjunct,
        set_exception: *set_exception,
        coordination: *coordination,
        recipient_passive_theme: *recipient_passive_theme,
        rules_object_followup: *rules_object_followup,
    })
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
    reason = "explicit agreement probing is staged for later grammar milestones"
)]
pub(super) fn slot_agrees(slot: VerbSlot, agreement: Agreement) -> bool {
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

    fn noun_phrase(number: Number) -> Features {
        Features::NounPhrase {
            agreement: Some(Agreement {
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
                agreement: Some(Agreement {
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
                agreement: Some(Agreement {
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
            erased_builder: None,
            erased_projector: None,
        }];
        const GROUP: GroupData = GroupData {
            name: "reordered",
            elements: &["member"],
            element_data: ELEMENTS,
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
                agreement: Some(Agreement {
                    number: Number::Plural,
                    ..
                }),
                ..
            })
        ));
    }
}
