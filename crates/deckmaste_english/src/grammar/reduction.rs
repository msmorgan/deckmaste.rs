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
        | RuleTag::NounPhraseAdditiveCoordination
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
                form,
                adjunct,
                ..
            } = children.get(2)?.features
            else {
                return None;
            };
            Some(Features::Nominal {
                head: identity.clone(),
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
            nominal_with_prefix(
                children.get(1)?,
                InitialSound::Consonant,
                false,
                AdjectiveComparisonState::NotComparative,
                true,
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
                coordination: NounPhraseCoordinationState::None,
                recipient_passive_theme: false,
                rules_object_followup: false,
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
                coordination: NounPhraseCoordinationState::None,
                recipient_passive_theme: false,
                rules_object_followup: false,
            })
        }
        RuleTag::NounPhraseThisCard | RuleTag::NounPhraseFullThisCard => {
            let Features::NounPhrase {
                agreement,
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
                coordination: NounPhraseCoordinationState::None,
                recipient_passive_theme: false,
                rules_object_followup: false,
            })
        }
        RuleTag::NounPhraseAdditiveCoordination => {
            reduce_noun_phrase_additive_coordination(children)
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

fn reduce_noun_phrase_additive_coordination(
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
) -> Option<Reduced> {
    let Features::NounPhrase {
        agreement: first_agreement,
        adjunct: first_adjunct,
        set_exception,
        recipient_passive_theme: first_theme,
        ..
    } = children.first()?.features
    else {
        return None;
    };
    let conjunction = Conjunction::Plus;
    let Features::NounPhrase {
        agreement: next_agreement,
        adjunct: next_adjunct,
        recipient_passive_theme: next_theme,
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
        coordination: NounPhraseCoordinationState::Binary(conjunction),
        recipient_passive_theme: *first_theme && *next_theme,
        rules_object_followup: false,
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
            if matches!(element.name, "noun_phrase_member" | "nominal_phrase_member")
                && present_fields & 0b11 == 0
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
                fields,
                present_fields,
                variant: None,
            }
        }
        R::ElementVariant { variant, .. } => Features::GeneratedElement {
            fields: vec![children.first()?.features.clone()],
            present_fields: 1,
            variant: Some(variant),
        },
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
                elements: vec![(fields.clone(), *present_fields, *variant)],
            }
        }
        R::SequenceExtend { group, element } => {
            let Features::GeneratedSequence { elements } = children.first()?.features else {
                return None;
            };
            let element = group.element_data.get(element)?;
            if matches!(element.name, "noun_phrase_member" | "nominal_phrase_member")
                && !elements
                    .last()
                    .is_some_and(|(_, present, _)| present & 0b01 != 0 && present & 0b10 == 0)
            {
                // Once another member follows, the previous one is known to
                // be nonfinal: it must carry a comma and must not already
                // carry the closing conjunction. Pruning at the extension
                // boundary prevents invalid list permutations from reaching
                // every surrounding NounPhrase prediction.
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
            let mut elements = elements.clone();
            elements.push((fields.clone(), *present_fields, *variant));
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
    if !generated_surface_sequence_scalars_match(rule, &fields) {
        return None;
    }
    let noun_phrase_features = match construction.feature_combinators {
        [] => Features::None,
        [feature] if feature.combinator == "noun_phrase_coordination" => {
            noun_phrase_coordination_features(construction.id, &fields)?
        }
        _ => return None,
    };
    let features = match (rule.context, preposition) {
        (super::rules::GeneratedRuleContext::Value, None) => noun_phrase_features,
        (super::rules::GeneratedRuleContext::SharedPreposition, Some(preposition)) => {
            generated_prepositional_coordination_features(construction.id, preposition, &fields)?
        }
        _ => return None,
    };
    let mut local_cost = super::ParseCost::default();
    if rule.context == super::rules::GeneratedRuleContext::SharedPreposition {
        local_cost.attachment_count = 1;
    }
    if construction.id == "shared_determiner_nominal" {
        local_cost.attachment_count = local_cost
            .attachment_count
            .saturating_add(generated_relative_complement_count(rule, &fields));
        if generated_complements_include_keyword_argument(rule, &fields) {
            local_cost.precedence = 1;
        }
    };
    Some(Reduction {
        features,
        local_cost,
    })
}

fn generated_relative_complement_count(
    rule: super::rules::GeneratedRuleRef,
    fields: &[Option<&Features>],
) -> u32 {
    if matches!(
        fields.get(1).copied().flatten(),
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
    let Some(Features::GeneratedSequence { elements }) = fields.get(3).copied().flatten() else {
        return 0;
    };
    let Some(element) = rule
        .group
        .element_data
        .iter()
        .find(|element| element.name == "nominal_complement")
    else {
        return 0;
    };
    u32::try_from(
        elements
            .iter()
            .filter(|(features, _, variant)| {
                variant
                    .and_then(|variant| element.variants.get(variant))
                    .is_some_and(|variant| variant.name == "Relative")
                    && matches!(features.first(), Some(Features::RelativeClause { .. }))
            })
            .count(),
    )
    .unwrap_or(u32::MAX)
}

fn generated_prepositional_coordination_features(
    construction: &str,
    preposition: Preposition,
    fields: &[Option<&Features>],
) -> Option<Features> {
    match construction {
        "noun_phrase_coordination" => {
            let Features::GeneratedSequence { elements } = fields.get(1)?.as_ref()? else {
                return None;
            };
            let (last, _) = elements.split_last()?;
            let Features::NounPhrase {
                set_exception,
                rules_object_followup,
                ..
            } = last.0.get(2)?
            else {
                return None;
            };
            let licensed_set_object = elements.len() == 1
                && matches!(preposition, Preposition::To | Preposition::From)
                && *set_exception == SetExceptionState::Host;
            let licensed_among_list = elements.len() >= 2 && preposition == Preposition::Among;
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
        "shared_determiner_nominal" => Some(Features::PrepositionalPhrase {
            preposition,
            nominal_attachment: true,
            shared_determiner_object: true,
            nearer_relative_host: false,
        }),
        _ => None,
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
            let Some(Features::GeneratedSequence { elements }) =
                fields.get(field_index).copied().flatten()
            else {
                return false;
            };
            let expected = elements.len() >= 2;
            elements
                .iter()
                .all(|(_, present, _)| (*present & (1_u64 << comma_index) != 0) == expected)
        })
}

fn generated_complements_include_keyword_argument(
    rule: super::rules::GeneratedRuleRef,
    fields: &[Option<&Features>],
) -> bool {
    let Some(Features::GeneratedSequence { elements }) = fields.get(3).copied().flatten() else {
        return false;
    };
    let Some(element) = rule
        .group
        .element_data
        .iter()
        .find(|element| element.name == "nominal_complement")
    else {
        return false;
    };
    elements.iter().any(|(_, _, variant)| {
        variant
            .and_then(|variant| element.variants.get(variant))
            .is_some_and(|variant| variant.name == "KeywordArgument")
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
                coordination: first_coordination,
                recipient_passive_theme: first_theme,
                ..
            } = fields.first()?.as_ref()?
            else {
                return None;
            };
            let Features::GeneratedSequence { elements } = fields.get(1)?.as_ref()? else {
                return None;
            };
            generated_coordination_sequence_is_valid(elements)?;
            if *set_exception == SetExceptionState::Closed
                || elements.len() >= 2 && *first_coordination != NounPhraseCoordinationState::None
            {
                return None;
            }
            let mut common_adjunct = *first_adjunct;
            let mut last_agreement = *first_agreement;
            let mut all_themes = *first_theme;
            let mut any_theme = *first_theme;
            for (element, _, _) in elements {
                let Features::NounPhrase {
                    agreement,
                    adjunct,
                    recipient_passive_theme,
                    ..
                } = element.get(2)?
                else {
                    return None;
                };
                if common_adjunct != *adjunct {
                    common_adjunct = None;
                }
                last_agreement = *agreement;
                all_themes &= *recipient_passive_theme;
                any_theme |= *recipient_passive_theme;
            }
            if any_theme && !all_themes {
                return None;
            }
            let conjunction = final_generated_conjunction(elements)?;
            Some(Features::NounPhrase {
                agreement: coordination_agreement(conjunction, *first_agreement, last_agreement)?,
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
        "shared_determiner_nominal" => {
            let Features::Determiner {
                cardinality,
                article,
                demonstrative_this,
                set_exception_host,
            } = fields.first()?.as_ref()?
            else {
                return None;
            };
            let first = nominal_coordination_member(fields.get(1)?.as_ref()?)?;
            let Features::GeneratedSequence { elements } = fields.get(2)?.as_ref()? else {
                return None;
            };
            generated_coordination_sequence_is_valid(elements)?;
            if !first.shared_determiner_open {
                return None;
            }
            let mut common_adjunct = first.adjunct;
            let mut last_form = first.form;
            let mut members = Vec::with_capacity(elements.len());
            for (element, _, _) in elements {
                let member = nominal_coordination_member(element.get(2)?)?;
                if !generated_determiner_accepts(
                    *cardinality,
                    *article,
                    member.form,
                    member.initial_sound,
                ) || *demonstrative_this && !member.demonstrative_shared_determiner
                {
                    return None;
                }
                if common_adjunct != member.adjunct {
                    common_adjunct = None;
                }
                last_form = member.form;
                members.push(member);
            }
            if members
                .iter()
                .take(members.len().saturating_sub(2))
                .any(|member| !member.shared_determiner_open)
                || !generated_determiner_accepts(
                    *cardinality,
                    *article,
                    first.form,
                    first.initial_sound,
                )
            {
                return None;
            }
            let conjunction = final_generated_conjunction(elements)?;
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
                )?,
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
        _ => None,
    }
}

fn generated_coordination_sequence_is_valid(
    elements: &[(Vec<Features>, u64, Option<usize>)],
) -> Option<()> {
    let (last, nonfinal) = elements.split_last()?;
    for (_, present, _) in nonfinal {
        if present & 1 == 0 || present & (1 << 1) != 0 {
            return None;
        }
    }
    if last.1 & (1 << 1) == 0 {
        return None;
    }
    if elements.len() == 1 && last.1 & 1 != 0 {
        return None;
    }
    Some(())
}

fn final_generated_conjunction(
    elements: &[(Vec<Features>, u64, Option<usize>)],
) -> Option<Conjunction> {
    let (fields, present, _) = elements.last()?;
    if present & (1 << 1) == 0 {
        return None;
    }
    let Features::Conjunction(conjunction) = fields.get(1)? else {
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

struct NominalCoordinationMember {
    form: NounForm,
    initial_sound: InitialSound,
    adjunct: Option<super::BareNominalAdjunct>,
    shared_determiner_open: bool,
    demonstrative_shared_determiner: bool,
}

fn nominal_coordination_member(features: &Features) -> Option<NominalCoordinationMember> {
    match features {
        Features::Noun {
            form,
            initial_sound,
            adjunct,
            ..
        } => Some(NominalCoordinationMember {
            form: *form,
            initial_sound: *initial_sound,
            adjunct: *adjunct,
            shared_determiner_open: true,
            demonstrative_shared_determiner: true,
        }),
        Features::Nominal {
            form,
            initial_sound,
            determined: false,
            adjunct,
            shared_determiner_open,
            demonstrative_shared_determiner,
            ..
        } => Some(NominalCoordinationMember {
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

    fn noun_phrase(number: Number) -> Features {
        Features::NounPhrase {
            agreement: Some(Agreement {
                person: Person::Third,
                number,
            }),
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
            elements: vec![(
                vec![
                    Features::None,
                    Features::Conjunction(conjunction),
                    noun_phrase(number),
                ],
                1 << 1 | 1 << 2,
                None,
            )],
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
