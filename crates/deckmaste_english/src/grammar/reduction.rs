use super::AdjectiveComparisonClass;
use super::AdjectiveComparisonState;
use super::Agreement;
use super::Child;
use super::Conjunction;
use super::EnglishGrammar;
use super::Features;
use super::GapState;
use super::IndefiniteArticle;
use super::InitialSound;
use super::NominalAttachmentPhase;
use super::NounCardinality;
use super::NounForm;
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
use super::noun_phrase_accepts_set_exception;
use super::opacity;

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
        | RuleTag::Noun
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
        | RuleTag::NounPhraseDamageCoordination
        | RuleTag::SharedDeterminerNominal
        | RuleTag::NounPhraseSharedDeterminer
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
        | RuleTag::NounPhraseCoordination
        | RuleTag::NounPhraseAdditiveCoordination
        | RuleTag::NounPhraseListSingle
        | RuleTag::NounPhraseListComma
        | RuleTag::NounPhraseCoordinationOxford
        | RuleTag::NounPhraseMinus
        | RuleTag::NounPhraseHalf
        | RuleTag::NounPhraseHalfRoundedUp
        | RuleTag::NounPhraseHalfRoundedDown
        | RuleTag::PrepositionalPhraseCoordinated
        | RuleTag::PrepositionalPhraseRulesObjectCoordinated
        | RuleTag::PrepositionalPhraseSharedDeterminer
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
        | RuleTag::RelativeContractedCopularCoordinatedAdjective
        | RuleTag::Sentence => clause::reduce_clause(tag, children)?,
        RuleTag::NounOpaque => opacity::reduce_opacity(tag, children)?,
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
                set_exception_host,
            } = children.first()?.features
            else {
                return None;
            };
            Some(Features::Determiner {
                cardinality: *cardinality,
                article: *article,
                set_exception_host: *set_exception_host,
            })
        }
        RuleTag::DeterminerTarget => Some(Features::Determiner {
            cardinality: NounCardinality::SingularCount,
            article: None,
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
        pronoun_case: None,
        adjunct: None,
        set_exception: SetExceptionState::Ineligible,
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
        RuleTag::Adjective | RuleTag::Noun => Some(propagate(children.first()?)),
        RuleTag::AdjectivePhraseFaceUp | RuleTag::AdjectivePhraseFaceDown => {
            Some(Features::Adjective {
                initial_sound: InitialSound::Consonant,
                comparison: AdjectiveComparisonState::NotComparative,
                card_orientation: true,
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
            } = children.first()?.features
            else {
                return None;
            };
            Some(Features::Adjective {
                initial_sound: *initial_sound,
                comparison: AdjectiveComparisonState::Complete,
                card_orientation: *card_orientation,
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
            })
        }
        RuleTag::ComparisonStandard
        | RuleTag::ComparisonThan
        | RuleTag::ComparisonThanOrEqualTo => Some(Features::None),
        RuleTag::NominalNoun => {
            let child = children.first()?;
            let Features::Noun {
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
                recipient_passive_theme: *recipient_passive_theme,
            })
        }
        RuleTag::NominalAdjective => {
            let Features::Adjective {
                initial_sound,
                comparison,
                card_orientation: false,
            } = children.first()?.features
            else {
                return None;
            };
            nominal_with_prefix(children.get(1)?, *initial_sound, false, *comparison)
        }
        RuleTag::NominalCombatStepName => {
            // Mirrors `NominalNoun`'s Noun→Nominal base case (this rule
            // *creates* a nominal from terminals, not `NominalNounModifier`,
            // which prepends onto an existing one). `initial_sound` is
            // overridden to `Consonant`: the phrase's true leftmost token is
            // `declare`, not the vowel-initial `attackers`.
            let Features::Noun { form, adjunct, .. } = children.get(2)?.features else {
                return None;
            };
            Some(Features::Nominal {
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
                recipient_passive_theme: false,
            })
        }
        RuleTag::NominalNounModifier => {
            let Features::Noun {
                initial_sound,
                opaque: modifier_opaque,
                ..
            } = children.first()?.features
            else {
                return None;
            };
            let Features::Nominal { opaque_head, .. } = children.get(1)?.features else {
                return None;
            };
            if *opaque_head && !*modifier_opaque {
                return None;
            }
            nominal_with_prefix(
                children.get(1)?,
                *initial_sound,
                false,
                AdjectiveComparisonState::NotComparative,
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
            nominal_with_prefix(
                children.get(1)?,
                InitialSound::Consonant,
                false,
                AdjectiveComparisonState::NotComparative,
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
            )
        }
        RuleTag::NominalDeterminer => {
            let Features::Determiner {
                cardinality,
                article,
                set_exception_host,
            } = children.first()?.features
            else {
                return None;
            };
            let Features::Nominal {
                form,
                initial_sound,
                determined,
                modified,
                attachment,
                comparison,
                adjunct,
                opaque_head,
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
                recipient_passive_theme: *recipient_passive_theme,
            })
        }
        RuleTag::NominalPrepositional | RuleTag::RulesObjectFollowupNominalPrepositional => {
            let Features::Nominal {
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
                recipient_passive_theme: *recipient_passive_theme,
            })
        }
        RuleTag::NominalInfinitive => {
            let Features::Nominal {
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
                recipient_passive_theme: *recipient_passive_theme,
            })
        }
        RuleTag::NominalKeywordSymbolArgument => {
            let Features::Noun {
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
                recipient_passive_theme: *recipient_passive_theme,
            })
        }
        RuleTag::NominalPowerToughnessComplement => {
            let Features::Nominal {
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
                recipient_passive_theme,
            } = children.first()?.features
            else {
                return None;
            };
            Some(Features::Nominal {
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
                recipient_passive_theme,
            } = children.first()?.features
            else {
                return None;
            };
            Some(Features::Nominal {
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
                recipient_passive_theme: *recipient_passive_theme,
            })
        }
        RuleTag::NominalComparison => {
            let Features::Nominal {
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
                recipient_passive_theme,
            } = children.first()?.features
            else {
                return None;
            };
            Some(Features::Nominal {
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
            let Features::Noun { .. } = children.first()?.features else {
                return None;
            };
            Some(Features::Nominal {
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
                recipient_passive_theme: false,
            })
        }
        RuleTag::NominalTimesClause => {
            // `times <clause>`: a plural `times` head with a finite clause as a
            // reduced adjunct-relative complement.
            let Features::Noun { .. } = children.first()?.features else {
                return None;
            };
            let Features::Clause { finite: true, .. } = children.get(1)?.features else {
                return None;
            };
            Some(Features::Nominal {
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
            } = children.first()?.features
            else {
                return None;
            };
            Some(Features::CoordinatedModifier {
                initial_sound: *initial_sound,
                all_adjectives: true,
            })
        }
        RuleTag::ModifierConjunctNoun => {
            let Features::Noun { initial_sound, .. } = children.first()?.features else {
                return None;
            };
            Some(Features::CoordinatedModifier {
                initial_sound: *initial_sound,
                all_adjectives: false,
            })
        }
        RuleTag::ModifierConjunctNegated => {
            // A `non-` conjunct always renders `non…`, a consonant onset.
            Some(Features::CoordinatedModifier {
                initial_sound: InitialSound::Consonant,
                all_adjectives: false,
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
            } = children.first()?.features
            else {
                return None;
            };
            let Features::CoordinatedModifier {
                all_adjectives: appended_adjectives,
                ..
            } = children.get(2)?.features
            else {
                return None;
            };
            Some(Features::CoordinatedModifier {
                initial_sound: *initial_sound,
                all_adjectives: *all_adjectives && *appended_adjectives,
            })
        }
        RuleTag::CoordinatedModifierConjoined | RuleTag::CoordinatedModifierOxford => {
            let Features::CoordinatedModifier {
                initial_sound,
                all_adjectives,
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
                ..
            } = children.last()?.features
            else {
                return None;
            };
            Some(Features::CoordinatedModifier {
                initial_sound: *initial_sound,
                all_adjectives: *all_adjectives && *closing_adjectives,
            })
        }
        RuleTag::NominalCoordinatedModifier => {
            let Features::CoordinatedModifier { initial_sound, .. } = children.first()?.features
            else {
                return None;
            };
            nominal_with_prefix(
                children.get(1)?,
                *initial_sound,
                false,
                AdjectiveComparisonState::NotComparative,
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
        RuleTag::NounPhraseDamageCoordination => {
            let Features::Nominal {
                adjunct: first_adjunct,
                recipient_passive_theme: true,
                ..
            } = children.first()?.features
            else {
                return None;
            };
            let Features::Conjunction(conjunction) = children.get(1)?.features else {
                return None;
            };
            let Features::Nominal {
                form: next_form,
                adjunct: next_adjunct,
                recipient_passive_theme: true,
                ..
            } = children.get(2)?.features
            else {
                return None;
            };
            let agreement = match conjunction {
                Conjunction::And => Some(Agreement {
                    person: Person::Third,
                    number: Number::Plural,
                }),
                Conjunction::Or | Conjunction::AndOr => Some(Agreement {
                    person: Person::Third,
                    number: match next_form {
                        NounForm::Plural => Number::Plural,
                        NounForm::Singular | NounForm::Mass => Number::Singular,
                    },
                }),
                Conjunction::Then | Conjunction::Plus => return None,
            };
            Some(Features::NounPhrase {
                agreement,
                pronoun_case: None,
                adjunct: (*first_adjunct == *next_adjunct)
                    .then_some(*first_adjunct)
                    .flatten(),
                set_exception: SetExceptionState::Ineligible,
            })
        }
        RuleTag::SharedDeterminerNominal => {
            let Features::Determiner {
                cardinality,
                article,
                set_exception_host,
            } = children.first()?.features
            else {
                return None;
            };
            let (first_form, first_initial_sound, first_adjunct, first_modified) =
                match children.get(1)?.features {
                    Features::Noun {
                        form,
                        initial_sound,
                        adjunct,
                        ..
                    } => (*form, *initial_sound, *adjunct, false),
                    Features::Nominal {
                        form,
                        initial_sound,
                        determined: false,
                        modified: true,
                        adjunct,
                        ..
                    } => (*form, *initial_sound, *adjunct, true),
                    _ => return None,
                };
            let Features::Conjunction(conjunction) = children.get(2)?.features else {
                return None;
            };
            let (next_form, next_initial_sound, next_adjunct, next_modified) =
                match children.get(3)?.features {
                    Features::Noun {
                        form,
                        initial_sound,
                        adjunct,
                        ..
                    } => (*form, *initial_sound, *adjunct, false),
                    Features::Nominal {
                        form,
                        initial_sound,
                        determined: false,
                        modified: true,
                        adjunct,
                        ..
                    } => (*form, *initial_sound, *adjunct, true),
                    _ => return None,
                };
            if first_modified != next_modified {
                return None;
            }
            if !cardinality_accepts(*cardinality, first_form)
                || !cardinality_accepts(*cardinality, next_form)
                || !article_accepts(*article, first_initial_sound)
                || !article_accepts(*article, next_initial_sound)
            {
                return None;
            }
            let agreement = match conjunction {
                Conjunction::And => Some(Agreement {
                    person: Person::Third,
                    number: Number::Plural,
                }),
                Conjunction::Or | Conjunction::AndOr => Some(Agreement {
                    person: Person::Third,
                    number: match next_form {
                        NounForm::Plural => Number::Plural,
                        NounForm::Singular | NounForm::Mass => Number::Singular,
                    },
                }),
                Conjunction::Then | Conjunction::Plus => return None,
            };
            Some(Features::NounPhrase {
                agreement,
                pronoun_case: None,
                adjunct: (first_adjunct == next_adjunct)
                    .then_some(first_adjunct)
                    .flatten(),
                set_exception: if *set_exception_host {
                    SetExceptionState::Host
                } else {
                    SetExceptionState::Ineligible
                },
            })
        }
        RuleTag::NounPhraseSharedDeterminer => {
            let Features::NounPhrase { .. } = children.first()?.features else {
                return None;
            };
            Some(propagate(children.first()?))
        }
        RuleTag::NounPhraseSetExceptionBare | RuleTag::NounPhraseSetExceptionFor => {
            let host = children.first()?;
            if !noun_phrase_accepts_set_exception(host.features)
                || !matches!(children.last()?.features, Features::NounPhrase { .. })
            {
                return None;
            }
            let Features::NounPhrase {
                agreement,
                pronoun_case,
                adjunct,
                ..
            } = host.features
            else {
                return None;
            };
            Some(Features::NounPhrase {
                agreement: *agreement,
                pronoun_case: *pronoun_case,
                adjunct: *adjunct,
                set_exception: SetExceptionState::Closed,
            })
        }
        RuleTag::NounPhraseNominal
        | RuleTag::RulesObjectNounPhrase
        | RuleTag::ReducedRecipientPassiveTheme => {
            let Features::Nominal {
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
                pronoun_case: None,
                adjunct,
                set_exception: if *set_exception_host {
                    SetExceptionState::Host
                } else {
                    SetExceptionState::Ineligible
                },
            })
        }
        RuleTag::NounPhraseSubjectPronoun | RuleTag::NounPhraseObjectPronoun => {
            noun_phrase_from_pronoun(children.first()?)
        }
        RuleTag::NounPhraseReciprocal => noun_phrase_from_pronoun(children.first()?),
        RuleTag::NounPhraseQuantity => {
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
                pronoun_case: None,
                adjunct: None,
                set_exception: SetExceptionState::Ineligible,
            })
        }
        RuleTag::NounPhraseThisCard | RuleTag::NounPhraseFullThisCard => {
            let Features::NounPhrase {
                agreement,
                pronoun_case,
                adjunct,
                set_exception,
            } = children.first()?.features
            else {
                return None;
            };
            Some(Features::NounPhrase {
                agreement: *agreement,
                pronoun_case: *pronoun_case,
                adjunct: *adjunct,
                set_exception: *set_exception,
            })
        }
        RuleTag::NounPhrasePossessiveThisCard => {
            let Features::PossessiveThisCard { agreement } = children.first()?.features else {
                return None;
            };
            Some(Features::NounPhrase {
                agreement: Some(*agreement),
                pronoun_case: None,
                adjunct: None,
                set_exception: SetExceptionState::Ineligible,
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
                pronoun_case: None,
                adjunct: None,
                set_exception: SetExceptionState::Ineligible,
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
                pronoun_case: None,
                adjunct: None,
                set_exception: SetExceptionState::Ineligible,
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
                pronoun_case: None,
                adjunct: None,
                set_exception: SetExceptionState::Ineligible,
            })
        }
        RuleTag::NounPhraseCoordination
        | RuleTag::NounPhraseAdditiveCoordination
        | RuleTag::NounPhraseCoordinationOxford => reduce_noun_phrase_coordination(tag, children),
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
        RuleTag::NounPhraseListSingle => {
            let Features::NounPhrase { .. } = children.first()?.features else {
                return None;
            };
            Some(children.first()?.features.clone())
        }
        RuleTag::NounPhraseListComma => {
            // The run keeps its first member's agreement; the closing rule
            // recomputes the coordinated agreement from the final conjunction.
            let Features::NounPhrase { .. } = children.first()?.features else {
                return None;
            };
            let Features::NounPhrase { .. } = children.get(2)?.features else {
                return None;
            };
            Some(children.first()?.features.clone())
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
        RuleTag::PrepositionalPhraseCoordinated
        | RuleTag::PrepositionalPhraseRulesObjectCoordinated => {
            reduce_coordinated_prepositional_phrase(tag, children)
        }
        RuleTag::PrepositionalPhraseSharedDeterminer => {
            let Features::Preposition(preposition) = children.first()?.features else {
                return None;
            };
            let Features::NounPhrase { .. } = children.get(1)?.features else {
                return None;
            };
            Some(Features::PrepositionalPhrase {
                preposition: *preposition,
                nominal_attachment: true,
                shared_determiner_object: true,
                nearer_relative_host: false,
            })
        }
        RuleTag::PrepositionalPhrase => {
            let Features::Preposition(preposition) = children.first()?.features else {
                return None;
            };
            let Features::PrepositionalObject { gerund } = children.get(1)?.features else {
                return None;
            };
            Some(Features::PrepositionalPhrase {
                preposition: *preposition,
                nominal_attachment: !(*preposition == Preposition::By && *gerund),
                shared_determiner_object: false,
                nearer_relative_host: false,
            })
        }
        RuleTag::PrepositionalObject => Some(Features::PrepositionalObject {
            gerund: matches!(children.first()?.features, Features::GerundClause),
        }),
        _ => None,
    }
}

fn reduce_coordinated_prepositional_phrase(
    tag: RuleTag,
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
) -> Option<Reduced> {
    let (conjunction_index, next_index) = if children.len() == 5 { (3, 4) } else { (2, 3) };
    let Features::Preposition(preposition) = children.first()?.features else {
        return None;
    };
    let Features::NounPhrase { .. } = children.get(1)?.features else {
        return None;
    };
    let Features::Conjunction(Conjunction::And | Conjunction::Or | Conjunction::AndOr) =
        children.get(conjunction_index)?.features
    else {
        return None;
    };
    let Features::NounPhrase { set_exception, .. } = children.get(next_index)?.features else {
        return None;
    };
    if tag == RuleTag::PrepositionalPhraseCoordinated {
        let binary_set_object = children.len() == 4
            && matches!(preposition, Preposition::To | Preposition::From)
            && *set_exception == SetExceptionState::Host;
        let among_option_list = children.len() == 5 && *preposition == Preposition::Among;
        if !binary_set_object && !among_option_list {
            return None;
        }
    }
    Some(Features::PrepositionalPhrase {
        preposition: *preposition,
        nominal_attachment: true,
        shared_determiner_object: false,
        nearer_relative_host: *preposition == Preposition::To,
    })
}

pub(super) fn reduce_noun_phrase_coordination(
    tag: RuleTag,
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
) -> Option<Reduced> {
    let Features::NounPhrase {
        agreement: first_agreement,
        adjunct: first_adjunct,
        set_exception,
        ..
    } = children.first()?.features
    else {
        return None;
    };
    // The Oxford close reads its conjunction after the comma (`, or`), so its
    // conjunction and next member sit one slot later than the binary rule.
    let (conjunction_index, next_index) =
        if tag == RuleTag::NounPhraseCoordinationOxford { (2, 3) } else { (1, 2) };
    let conjunction = if tag == RuleTag::NounPhraseAdditiveCoordination {
        Conjunction::Plus
    } else {
        let Features::Conjunction(conjunction) = children.get(conjunction_index)?.features else {
            return None;
        };
        match conjunction {
            Conjunction::And | Conjunction::Or | Conjunction::AndOr => *conjunction,
            // `then` and lexical `plus` never enter ordinary noun-phrase
            // coordination; additive productions introduce `Plus` directly.
            Conjunction::Then | Conjunction::Plus => return None,
        }
    };
    let Features::NounPhrase {
        agreement: next_agreement,
        adjunct: next_adjunct,
        ..
    } = children.get(next_index)?.features
    else {
        return None;
    };
    let agreement = match conjunction {
        Conjunction::And => Some(Agreement {
            person: Person::Third,
            number: Number::Plural,
        }),
        Conjunction::Or | Conjunction::AndOr => *next_agreement,
        Conjunction::Plus => *first_agreement,
        Conjunction::Then => return None,
    };
    Some(Features::NounPhrase {
        agreement,
        pronoun_case: None,
        adjunct: (*first_adjunct == *next_adjunct)
            .then_some(*first_adjunct)
            .flatten(),
        set_exception: *set_exception,
    })
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
            Features::GeneratedElement { fields }
        }
        R::ElementVariant { .. } => Features::GeneratedElement {
            fields: vec![children.first()?.features.clone()],
        },
        R::SequenceSeed { .. } => {
            let Features::GeneratedElement { fields } = children.first()?.features else {
                return None;
            };
            Features::GeneratedSequence {
                elements: vec![fields.clone()],
            }
        }
        R::SequenceExtend { .. } => {
            let Features::GeneratedSequence { elements } = children.first()?.features else {
                return None;
            };
            let Features::GeneratedElement { fields } = children.get(1)?.features else {
                return None;
            };
            let mut elements = elements.clone();
            elements.push(fields.clone());
            Features::GeneratedSequence { elements }
        }
    };
    Some(Reduction {
        features,
        local_cost: super::ParseCost::default(),
    })
}

pub(super) fn reduce_generated(
    rule: super::rules::GeneratedRuleRef,
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
) -> Option<Reduction<Features>> {
    use deckmaste_construction_compiler::runtime::AtomData;

    let construction = rule.group.constructions.get(rule.construction)?;
    let form = construction.forms.get(rule.form)?;
    let mut child_index = 0_usize;
    let mut fields = vec![None; construction.fields.len()];
    for (atom_index, atom) in form.atoms.iter().enumerate() {
        if matches!(atom, AtomData::Literal(_)) {
            child_index += 1;
            continue;
        }
        let path = match atom {
            AtomData::Hole(path) | AtomData::Lexeme(path) => *path,
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
    let features = match construction.feature_combinators {
        [] => Features::None,
        [feature] if feature.combinator == "noun_phrase_coordination" => {
            noun_phrase_coordination_features(construction.id, &fields)?
        }
        _ => return None,
    };
    Some(Reduction {
        features,
        local_cost: super::ParseCost::default(),
    })
}

fn noun_phrase_coordination_features(
    construction: &str,
    fields: &[Option<&Features>],
) -> Option<Features> {
    match construction {
        "noun_phrase_coordination" => {
            let Features::NounPhrase {
                agreement: first_agreement,
                adjunct: first_adjunct,
                set_exception,
                ..
            } = fields.first()?.as_ref()?
            else {
                return None;
            };
            let Features::GeneratedSequence { elements } = fields.get(1)?.as_ref()? else {
                return None;
            };
            let mut common_adjunct = *first_adjunct;
            let mut last_agreement = *first_agreement;
            for element in elements {
                let Features::NounPhrase {
                    agreement, adjunct, ..
                } = element.get(2)?
                else {
                    return None;
                };
                if common_adjunct != *adjunct {
                    common_adjunct = None;
                }
                last_agreement = *agreement;
            }
            let conjunction = final_generated_conjunction(elements)?;
            Some(Features::NounPhrase {
                agreement: coordination_agreement(conjunction, *first_agreement, last_agreement)?,
                pronoun_case: None,
                adjunct: common_adjunct,
                set_exception: *set_exception,
            })
        }
        "shared_determiner_nominal" => {
            let Features::Determiner {
                cardinality,
                article,
                set_exception_host,
            } = fields.first()?.as_ref()?
            else {
                return None;
            };
            let first = nominal_coordination_member(fields.get(1)?.as_ref()?)?;
            let Features::GeneratedSequence { elements } = fields.get(2)?.as_ref()? else {
                return None;
            };
            let mut common_adjunct = first.2;
            let mut last_form = first.0;
            for element in elements {
                let member = nominal_coordination_member(element.get(2)?)?;
                if member.3 != first.3
                    || !cardinality_accepts(*cardinality, member.0)
                    || !article_accepts(*article, member.1)
                {
                    return None;
                }
                if common_adjunct != member.2 {
                    common_adjunct = None;
                }
                last_form = member.0;
            }
            if !cardinality_accepts(*cardinality, first.0) || !article_accepts(*article, first.1) {
                return None;
            }
            let conjunction = final_generated_conjunction(elements)?;
            Some(Features::NounPhrase {
                agreement: coordination_agreement(
                    conjunction,
                    Some(Agreement {
                        person: Person::Third,
                        number: match first.0 {
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
                )?,
                pronoun_case: None,
                adjunct: common_adjunct,
                set_exception: if *set_exception_host {
                    SetExceptionState::Host
                } else {
                    SetExceptionState::Ineligible
                },
            })
        }
        _ => None,
    }
}

fn final_generated_conjunction(elements: &[Vec<Features>]) -> Option<Conjunction> {
    let Features::Conjunction(conjunction) = elements.last()?.get(1)? else {
        return None;
    };
    matches!(
        conjunction,
        Conjunction::And | Conjunction::Or | Conjunction::AndOr
    )
    .then_some(*conjunction)
}

fn coordination_agreement(
    conjunction: Conjunction,
    first: Option<Agreement>,
    last: Option<Agreement>,
) -> Option<Option<Agreement>> {
    Some(match conjunction {
        Conjunction::And => Some(Agreement {
            person: Person::Third,
            number: Number::Plural,
        }),
        Conjunction::Or | Conjunction::AndOr => last,
        Conjunction::Plus => first,
        Conjunction::Then => return None,
    })
}

fn nominal_coordination_member(
    features: &Features,
) -> Option<(
    NounForm,
    InitialSound,
    Option<super::BareNominalAdjunct>,
    bool,
)> {
    match features {
        Features::Noun {
            form,
            initial_sound,
            adjunct,
            ..
        } => Some((*form, *initial_sound, *adjunct, false)),
        Features::Nominal {
            form,
            initial_sound,
            determined: false,
            modified: true,
            adjunct,
            ..
        } => Some((*form, *initial_sound, *adjunct, true)),
        _ => None,
    }
}

pub(super) fn propagate(child: &Child<'_, EnglishGrammar<'_, '_>>) -> Reduced {
    child.features.clone()
}

pub(super) fn nominal_with_prefix(
    nominal: &Child<'_, EnglishGrammar<'_, '_>>,
    initial_sound: InitialSound,
    leading_opacity: bool,
    prefix_comparison: AdjectiveComparisonState,
) -> Option<Reduced> {
    let Features::Nominal {
        form,
        determined,
        attachment,
        comparison,
        adjunct,
        opaque_head,
        set_exception_host,
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
        recipient_passive_theme: *recipient_passive_theme,
    })
}

pub(super) fn noun_phrase_from_pronoun(
    child: &Child<'_, EnglishGrammar<'_, '_>>,
) -> Option<Reduced> {
    let Features::NounPhrase {
        agreement,
        pronoun_case,
        adjunct,
        set_exception,
    } = child.features
    else {
        return None;
    };
    Some(Features::NounPhrase {
        agreement: *agreement,
        pronoun_case: *pronoun_case,
        adjunct: *adjunct,
        set_exception: *set_exception,
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

    fn noun_phrase(number: Number) -> Features {
        Features::NounPhrase {
            agreement: Some(Agreement {
                person: Person::Third,
                number,
            }),
            pronoun_case: None,
            adjunct: None,
            set_exception: SetExceptionState::Ineligible,
        }
    }

    fn sequence(conjunction: Conjunction, number: Number) -> Features {
        Features::GeneratedSequence {
            elements: vec![vec![
                Features::None,
                Features::Conjunction(conjunction),
                noun_phrase(number),
            ]],
        }
    }

    #[test]
    fn generated_noun_coordination_combines_agreement_by_conjunction() {
        let first = noun_phrase(Number::Singular);
        let and = sequence(Conjunction::And, Number::Singular);
        let fields = [Some(&first), Some(&and)];
        assert!(matches!(
            noun_phrase_coordination_features("noun_phrase_coordination", &fields),
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
            noun_phrase_coordination_features("noun_phrase_coordination", &fields),
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
            noun_phrase_coordination_features(
                "noun_phrase_coordination",
                &[Some(&first), Some(&then)]
            )
            .is_none()
        );
    }
}
