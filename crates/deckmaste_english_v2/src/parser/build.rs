use super::Leaf;
use super::rules::NounNumber;
use super::rules::RuleId;
use crate::ast::Ability;
use crate::ast::Amount;
use crate::ast::Clause;
use crate::ast::Common;
use crate::ast::Connive;
use crate::ast::CountNp;
use crate::ast::DealDamage;
use crate::ast::Declarative;
use crate::ast::Demonstrative;
use crate::ast::DemonstrativeNp;
use crate::ast::Destroy;
use crate::ast::EventClause;
use crate::ast::GainLife;
use crate::ast::Imperative;
use crate::ast::NounPhrase;
use crate::ast::NumberAmount;
use crate::ast::Pronoun;
use crate::ast::PronounNp;
use crate::ast::SelfReferenceNp;
use crate::ast::Sentence;
use crate::ast::Spell;
use crate::ast::TargetNp;
use crate::ast::Triggered;
use crate::ast::VariableAmount;
use crate::ast::VerbLexeme;
use crate::ast::VerbPhrase;
use crate::ast::WhereClause;
use crate::ast::WithWhere;
use crate::context::ParseContext;
pub(crate) use crate::features::Agreement;
use crate::features::agreement_for_pronoun;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum BuildValue {
    Ability(Ability),
    Sentence(Sentence),
    Clause(Clause),
    NounPhrase(NounPhrase, Agreement),
    VerbPhrase(VerbPhrase, Agreement),
    Amount(Amount),
    Leaf(Leaf),
}

#[expect(
    clippy::too_many_lines,
    reason = "the exhaustive generated-shape construction dispatch is intentionally flat"
)]
pub(super) fn build(
    rule: RuleId,
    children: &[BuildValue],
    context: &ParseContext<'_>,
) -> Option<BuildValue> {
    match rule {
        RuleId::AbilitySpell => match children {
            [
                BuildValue::Sentence(effect),
                BuildValue::Leaf(Leaf::Literal(".")),
                BuildValue::Leaf(Leaf::EndOfInput),
            ] => Some(BuildValue::Ability(Ability::Spell(Spell {
                effect: effect.clone(),
            }))),
            _ => None,
        },
        RuleId::AbilityTriggered => match children {
            [
                BuildValue::Leaf(Leaf::TriggerWord(trigger)),
                BuildValue::Clause(Clause::Event(event)),
                BuildValue::Leaf(Leaf::Literal(",")),
                BuildValue::Sentence(effect),
                BuildValue::Leaf(Leaf::Literal(".")),
                BuildValue::Leaf(Leaf::EndOfInput),
            ] => Triggered::new(*trigger, Clause::Event(event.clone()), vec![effect.clone()])
                .map(Ability::Triggered)
                .map(BuildValue::Ability),
            _ => None,
        },
        RuleId::SentenceImperative => match children {
            [BuildValue::VerbPhrase(predicate, Agreement::Bare)] => {
                Some(BuildValue::Sentence(Sentence::Imperative(Imperative {
                    predicate: predicate.clone(),
                })))
            }
            _ => None,
        },
        RuleId::SentenceDeclarative => match children {
            [
                BuildValue::NounPhrase(subject, subject_agreement),
                BuildValue::VerbPhrase(predicate, verb_agreement),
            ] if subject_agreement == verb_agreement => {
                Some(BuildValue::Sentence(Sentence::Declarative(Declarative {
                    subject: subject.clone(),
                    predicate: predicate.clone(),
                })))
            }
            _ => None,
        },
        RuleId::SentenceWithWhere => match children {
            [
                BuildValue::Sentence(body),
                BuildValue::Leaf(Leaf::Literal(",")),
                BuildValue::Clause(Clause::Where(clause)),
            ] => Some(BuildValue::Sentence(Sentence::WithWhere(WithWhere {
                body: Box::new(body.clone()),
                clause: Clause::Where(clause.clone()),
            }))),
            _ => None,
        },
        RuleId::ClauseEvent => match children {
            [
                BuildValue::NounPhrase(subject, subject_agreement),
                BuildValue::VerbPhrase(predicate, verb_agreement),
            ] if subject_agreement == verb_agreement => {
                Some(BuildValue::Clause(Clause::Event(EventClause {
                    subject: subject.clone(),
                    predicate: predicate.clone(),
                })))
            }
            _ => None,
        },
        RuleId::ClauseWhere => match children {
            [
                BuildValue::Leaf(Leaf::Literal("where")),
                BuildValue::Leaf(Leaf::Variable(variable)),
                BuildValue::Leaf(Leaf::Verb {
                    lexeme: VerbLexeme::Be,
                    agreement: Agreement::ThirdPersonSingular,
                }),
                BuildValue::Leaf(Leaf::Literal("the")),
                BuildValue::Leaf(Leaf::Literal("number")),
                BuildValue::Leaf(Leaf::Literal("of")),
                BuildValue::NounPhrase(value, _),
            ] => Some(BuildValue::Clause(Clause::Where(WhereClause {
                variable: *variable,
                value: value.clone(),
            }))),
            _ => None,
        },
        RuleId::NounPhrasePronoun => match children {
            [BuildValue::Leaf(Leaf::Pronoun(pronoun))] => Some(BuildValue::NounPhrase(
                NounPhrase::Pronoun(PronounNp { word: *pronoun }),
                agreement_for_pronoun(*pronoun),
            )),
            _ => None,
        },
        RuleId::NounPhraseCommon => match children {
            [
                BuildValue::Leaf(Leaf::Article(article)),
                BuildValue::Leaf(Leaf::Noun {
                    noun,
                    number: NounNumber::Singular,
                }),
            ] => Some(BuildValue::NounPhrase(
                NounPhrase::Common(Common {
                    article: *article,
                    head: noun.clone(),
                }),
                Agreement::ThirdPersonSingular,
            )),
            _ => None,
        },
        RuleId::NounPhraseDemonstrative => match children {
            [
                BuildValue::Leaf(Leaf::Demonstrative(demonstrative)),
                BuildValue::Leaf(Leaf::Noun { noun, number }),
            ] => match (demonstrative, number) {
                (Demonstrative::That, NounNumber::Singular) => Some(BuildValue::NounPhrase(
                    NounPhrase::Demonstrative(DemonstrativeNp {
                        word: Demonstrative::That,
                        head: noun.clone(),
                    }),
                    Agreement::ThirdPersonSingular,
                )),
                (Demonstrative::Those, NounNumber::Plural) => Some(BuildValue::NounPhrase(
                    NounPhrase::Demonstrative(DemonstrativeNp {
                        word: Demonstrative::Those,
                        head: noun.clone(),
                    }),
                    Agreement::Bare,
                )),
                _ => None,
            },
            _ => None,
        },
        RuleId::NounPhraseTarget => match children {
            [
                BuildValue::Leaf(Leaf::Literal("target")),
                BuildValue::Leaf(Leaf::Noun {
                    noun,
                    number: NounNumber::Singular,
                }),
            ] => Some(BuildValue::NounPhrase(
                NounPhrase::Target(TargetNp { head: noun.clone() }),
                Agreement::ThirdPersonSingular,
            )),
            _ => None,
        },
        RuleId::NounPhraseSelfReference => match children {
            [BuildValue::Leaf(Leaf::SelfReference(spelling))] => {
                SelfReferenceNp::new(*spelling, context).map(|self_reference| {
                    BuildValue::NounPhrase(
                        NounPhrase::SelfReference(self_reference),
                        Agreement::ThirdPersonSingular,
                    )
                })
            }
            _ => None,
        },
        RuleId::NounPhraseCount => match children {
            [
                BuildValue::Leaf(Leaf::Noun {
                    noun,
                    number: NounNumber::Plural,
                }),
                BuildValue::Leaf(Leaf::Pronoun(Pronoun::You)),
                BuildValue::Leaf(Leaf::Verb {
                    lexeme: VerbLexeme::Control,
                    agreement: Agreement::Bare,
                }),
                BuildValue::Leaf(Leaf::Literal("with")),
                BuildValue::Leaf(Leaf::Literal("power")),
                BuildValue::Leaf(Leaf::SignedNumber(threshold)),
                BuildValue::Leaf(Leaf::Literal("or")),
                BuildValue::Leaf(Leaf::Literal("less")),
            ] => Some(BuildValue::NounPhrase(
                NounPhrase::Count(CountNp {
                    head: noun.clone(),
                    controller: Pronoun::You,
                    threshold: threshold.clone(),
                }),
                Agreement::Bare,
            )),
            _ => None,
        },
        RuleId::VerbPhraseDestroy => match children {
            [
                BuildValue::Leaf(Leaf::Verb {
                    lexeme: VerbLexeme::Destroy,
                    agreement,
                }),
                BuildValue::NounPhrase(object, _),
            ] => Some(BuildValue::VerbPhrase(
                VerbPhrase::Destroy(Destroy {
                    object: object.clone(),
                }),
                *agreement,
            )),
            _ => None,
        },
        RuleId::VerbPhraseConnive => match children {
            [
                BuildValue::Leaf(Leaf::Verb {
                    lexeme: VerbLexeme::Connive,
                    agreement,
                }),
            ] => Some(BuildValue::VerbPhrase(
                VerbPhrase::Connive(Connive),
                *agreement,
            )),
            _ => None,
        },
        RuleId::VerbPhraseDealDamage => match children {
            [
                BuildValue::Leaf(Leaf::Verb {
                    lexeme: VerbLexeme::Deal,
                    agreement,
                }),
                BuildValue::Amount(amount),
                BuildValue::Leaf(Leaf::Literal("damage")),
                BuildValue::Leaf(Leaf::Literal("to")),
                BuildValue::NounPhrase(to, _),
            ] => Some(BuildValue::VerbPhrase(
                VerbPhrase::DealDamage(DealDamage {
                    amount: amount.clone(),
                    to: to.clone(),
                }),
                *agreement,
            )),
            _ => None,
        },
        RuleId::VerbPhraseGainLife => match children {
            [
                BuildValue::Leaf(Leaf::Verb {
                    lexeme: VerbLexeme::Gain,
                    agreement,
                }),
                BuildValue::Amount(amount),
                BuildValue::Leaf(Leaf::Literal("life")),
            ] => Some(BuildValue::VerbPhrase(
                VerbPhrase::GainLife(GainLife {
                    amount: amount.clone(),
                }),
                *agreement,
            )),
            _ => None,
        },
        RuleId::AmountNumber => match children {
            [BuildValue::Leaf(Leaf::SignedNumber(number))] => {
                Some(BuildValue::Amount(Amount::Number(NumberAmount {
                    number: number.clone(),
                })))
            }
            _ => None,
        },
        RuleId::AmountVariable => match children {
            [BuildValue::Leaf(Leaf::Variable(variable))] => {
                Some(BuildValue::Amount(Amount::Variable(VariableAmount {
                    variable: *variable,
                })))
            }
            _ => None,
        },
    }
}
