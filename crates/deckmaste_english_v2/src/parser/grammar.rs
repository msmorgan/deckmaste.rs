#![allow(
    dead_code,
    reason = "the public parser stage consumes these crate-internal grammar contracts"
)]

use deckmaste_catalogs::CatalogKind;

use super::engine::ChartFailure;
use super::engine::Child;
use super::engine::Forest;
use super::engine::LexicalMatch;
use super::engine::PackedNode;
use super::engine::Rule;
use super::engine::RulePosition;
use super::engine::SeedPolicy;
use super::engine::parse;
use crate::ast::*;
use crate::catalogs::ParserCatalogs;
use crate::context::ParseContext;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub(crate) enum Category {
    Ability,
    Sentence,
    Clause,
    NounPhrase,
    VerbPhrase,
    Amount,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub(crate) enum Construction {
    AbilitySpell,
    AbilityTriggered,
    SentenceImperative,
    SentenceDeclarative,
    SentenceWithWhere,
    ClauseEvent,
    ClauseWhere,
    NounPhrasePronoun,
    NounPhraseCommon,
    NounPhraseDemonstrative,
    NounPhraseTarget,
    NounPhraseSelfReference,
    NounPhraseCount,
    VerbPhraseDestroy,
    VerbPhraseConnive,
    VerbPhraseDealDamage,
    VerbPhraseGainLife,
    AmountNumber,
    AmountVariable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub(crate) enum NounNumber {
    Singular,
    Plural,
    Either,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub(crate) enum Lexical {
    Literal(&'static str),
    TriggerWord,
    Article,
    Demonstrative,
    Pronoun,
    Variable,
    Noun(NounNumber),
    Verb(VerbLexeme),
    SignedNumber,
    SelfReference,
}

use RulePosition::Lexical as L;
use RulePosition::Nonterminal as N;

pub(crate) const RULES: &[Rule<Category, Lexical, Construction>] = &[
    Rule {
        lhs: Category::Ability,
        rhs: &[N(Category::Sentence), L(Lexical::Literal("."))],
        construction: Construction::AbilitySpell,
    },
    Rule {
        lhs: Category::Ability,
        rhs: &[
            L(Lexical::TriggerWord),
            N(Category::Clause),
            L(Lexical::Literal(",")),
            N(Category::Sentence),
            L(Lexical::Literal(".")),
        ],
        construction: Construction::AbilityTriggered,
    },
    Rule {
        lhs: Category::Sentence,
        rhs: &[N(Category::VerbPhrase)],
        construction: Construction::SentenceImperative,
    },
    Rule {
        lhs: Category::Sentence,
        rhs: &[N(Category::NounPhrase), N(Category::VerbPhrase)],
        construction: Construction::SentenceDeclarative,
    },
    Rule {
        lhs: Category::Sentence,
        rhs: &[
            N(Category::Sentence),
            L(Lexical::Literal(",")),
            N(Category::Clause),
        ],
        construction: Construction::SentenceWithWhere,
    },
    Rule {
        lhs: Category::Clause,
        rhs: &[N(Category::NounPhrase), N(Category::VerbPhrase)],
        construction: Construction::ClauseEvent,
    },
    Rule {
        lhs: Category::Clause,
        rhs: &[
            L(Lexical::Literal("where")),
            L(Lexical::Variable),
            L(Lexical::Verb(VerbLexeme::Be)),
            L(Lexical::Literal("the")),
            L(Lexical::Literal("number")),
            L(Lexical::Literal("of")),
            N(Category::NounPhrase),
        ],
        construction: Construction::ClauseWhere,
    },
    Rule {
        lhs: Category::NounPhrase,
        rhs: &[L(Lexical::Pronoun)],
        construction: Construction::NounPhrasePronoun,
    },
    Rule {
        lhs: Category::NounPhrase,
        rhs: &[L(Lexical::Article), L(Lexical::Noun(NounNumber::Singular))],
        construction: Construction::NounPhraseCommon,
    },
    Rule {
        lhs: Category::NounPhrase,
        rhs: &[
            L(Lexical::Demonstrative),
            L(Lexical::Noun(NounNumber::Either)),
        ],
        construction: Construction::NounPhraseDemonstrative,
    },
    Rule {
        lhs: Category::NounPhrase,
        rhs: &[
            L(Lexical::Literal("target")),
            L(Lexical::Noun(NounNumber::Singular)),
        ],
        construction: Construction::NounPhraseTarget,
    },
    Rule {
        lhs: Category::NounPhrase,
        rhs: &[L(Lexical::SelfReference)],
        construction: Construction::NounPhraseSelfReference,
    },
    Rule {
        lhs: Category::NounPhrase,
        rhs: &[
            L(Lexical::Noun(NounNumber::Plural)),
            L(Lexical::Pronoun),
            L(Lexical::Verb(VerbLexeme::Control)),
            L(Lexical::Literal("with")),
            L(Lexical::Literal("power")),
            L(Lexical::SignedNumber),
            L(Lexical::Literal("or")),
            L(Lexical::Literal("less")),
        ],
        construction: Construction::NounPhraseCount,
    },
    Rule {
        lhs: Category::VerbPhrase,
        rhs: &[
            L(Lexical::Verb(VerbLexeme::Destroy)),
            N(Category::NounPhrase),
        ],
        construction: Construction::VerbPhraseDestroy,
    },
    Rule {
        lhs: Category::VerbPhrase,
        rhs: &[L(Lexical::Verb(VerbLexeme::Connive))],
        construction: Construction::VerbPhraseConnive,
    },
    Rule {
        lhs: Category::VerbPhrase,
        rhs: &[
            L(Lexical::Verb(VerbLexeme::Deal)),
            N(Category::Amount),
            L(Lexical::Literal("damage")),
            L(Lexical::Literal("to")),
            N(Category::NounPhrase),
        ],
        construction: Construction::VerbPhraseDealDamage,
    },
    Rule {
        lhs: Category::VerbPhrase,
        rhs: &[
            L(Lexical::Verb(VerbLexeme::Gain)),
            N(Category::Amount),
            L(Lexical::Literal("life")),
        ],
        construction: Construction::VerbPhraseGainLife,
    },
    Rule {
        lhs: Category::Amount,
        rhs: &[L(Lexical::SignedNumber)],
        construction: Construction::AmountNumber,
    },
    Rule {
        lhs: Category::Amount,
        rhs: &[L(Lexical::Variable)],
        construction: Construction::AmountVariable,
    },
];

pub(crate) struct SliceGrammar<'a> {
    pub(crate) catalogs: &'a ParserCatalogs,
    pub(crate) context: &'a ParseContext<'a>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Agreement {
    Bare,
    ThirdPersonSingular,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Leaf {
    Literal(&'static str),
    TriggerWord(TriggerWord),
    Article(Article),
    Demonstrative(Demonstrative),
    Pronoun(Pronoun),
    Variable(Variable),
    Noun {
        noun: Noun,
        number: NounNumber,
    },
    Verb {
        lexeme: VerbLexeme,
        agreement: Agreement,
    },
    SignedNumber(SignedNumber),
    SelfReference(SelfReferenceSpelling),
}

pub(crate) fn parse_forest(
    grammar: &SliceGrammar<'_>,
    text: &str,
) -> Result<Forest<Construction, Leaf>, ChartFailure<Category, Lexical>> {
    parse(
        RULES,
        Category::Ability,
        SeedPolicy::StartOnly,
        text.len(),
        |lexical, offset| grammar.scan(lexical, text, offset),
    )
}

impl SliceGrammar<'_> {
    fn scan(&self, lexical: Lexical, text: &str, offset: usize) -> Vec<LexicalMatch<Leaf>> {
        match lexical {
            Lexical::Literal(literal @ ("." | ",")) => text[offset..]
                .starts_with(literal)
                .then_some(LexicalMatch {
                    end: offset + literal.len(),
                    value: Leaf::Literal(literal),
                })
                .into_iter()
                .collect(),
            Lexical::Literal(literal) => self
                .word(text, offset, literal)
                .map(|end| LexicalMatch {
                    end,
                    value: Leaf::Literal(literal),
                })
                .into_iter()
                .collect(),
            Lexical::TriggerWord => self.closed_word(
                text,
                offset,
                "whenever",
                Leaf::TriggerWord(TriggerWord::Whenever),
            ),
            Lexical::Article => [
                ("a", Leaf::Article(Article::A)),
                ("an", Leaf::Article(Article::An)),
            ]
            .into_iter()
            .filter_map(|(word, value)| {
                self.word(text, offset, word)
                    .map(|end| LexicalMatch { end, value })
            })
            .collect(),
            Lexical::Demonstrative => [
                ("that", Leaf::Demonstrative(Demonstrative::That)),
                ("those", Leaf::Demonstrative(Demonstrative::Those)),
            ]
            .into_iter()
            .filter_map(|(word, value)| {
                self.word(text, offset, word)
                    .map(|end| LexicalMatch { end, value })
            })
            .collect(),
            Lexical::Pronoun => [
                ("it", Leaf::Pronoun(Pronoun::It)),
                ("you", Leaf::Pronoun(Pronoun::You)),
            ]
            .into_iter()
            .filter_map(|(word, value)| {
                self.word(text, offset, word)
                    .map(|end| LexicalMatch { end, value })
            })
            .collect(),
            Lexical::Variable => self.closed_word(text, offset, "X", Leaf::Variable(Variable::X)),
            Lexical::Noun(number) => self.scan_noun(text, offset, number),
            Lexical::Verb(lexeme) => self.scan_verb(text, offset, lexeme),
            Lexical::SignedNumber => self.scan_signed_number(text, offset),
            Lexical::SelfReference => [
                (
                    self.context.card_name(),
                    Leaf::SelfReference(SelfReferenceSpelling::Full),
                ),
                (
                    self.context.abbreviated_card_name(),
                    Leaf::SelfReference(SelfReferenceSpelling::Abbreviated),
                ),
            ]
            .into_iter()
            .filter_map(|(word, value)| {
                self.word(text, offset, word)
                    .map(|end| LexicalMatch { end, value })
            })
            .collect(),
        }
    }

    fn closed_word(
        &self,
        text: &str,
        offset: usize,
        word: &str,
        value: Leaf,
    ) -> Vec<LexicalMatch<Leaf>> {
        self.word(text, offset, word)
            .map(|end| LexicalMatch { end, value })
            .into_iter()
            .collect()
    }

    fn word(&self, text: &str, offset: usize, word: &str) -> Option<usize> {
        let prefix = usize::from(offset != 0);
        let remainder = text.get(offset..)?;
        let remainder = (prefix == 0)
            .then_some(remainder)
            .or_else(|| remainder.strip_prefix(' '))?;
        let word = (offset == 0)
            .then(|| capitalize(word))
            .unwrap_or_else(|| word.to_owned());
        remainder
            .starts_with(&word)
            .then_some(offset + prefix + word.len())
    }

    fn scan_noun(&self, text: &str, offset: usize, wanted: NounNumber) -> Vec<LexicalMatch<Leaf>> {
        let mut matches = Vec::new();
        for (noun, singular) in std::iter::once((
            Noun::Lexeme(NounLexeme::Player),
            "player".to_owned(),
        ))
        .chain(CatalogKind::ALL.into_iter().flat_map(|kind| {
            self.catalogs
                .set()
                .get(kind)
                .iter()
                .filter_map(move |spelling| {
                    CatalogIdentity::new(self.catalogs, kind, spelling.clone())
                        .map(|identity| (Noun::Catalog(identity), rendered_catalog(kind, spelling)))
                })
        })) {
            for (number, word) in noun_forms(&singular, wanted) {
                if let Some(end) = self.word(text, offset, &word) {
                    matches.push(LexicalMatch {
                        end,
                        value: Leaf::Noun {
                            noun: noun.clone(),
                            number,
                        },
                    });
                }
            }
        }
        matches
    }

    fn scan_verb(&self, text: &str, offset: usize, lexeme: VerbLexeme) -> Vec<LexicalMatch<Leaf>> {
        [Agreement::Bare, Agreement::ThirdPersonSingular]
            .into_iter()
            .filter_map(|agreement| {
                self.word(text, offset, inflect(lexeme, agreement))
                    .map(|end| LexicalMatch {
                        end,
                        value: Leaf::Verb { lexeme, agreement },
                    })
            })
            .collect()
    }

    fn scan_signed_number(&self, text: &str, offset: usize) -> Vec<LexicalMatch<Leaf>> {
        let prefix = usize::from(offset != 0);
        let Some(remainder) = text.get(offset..) else {
            return Vec::new();
        };
        let Some(number) = (prefix == 0)
            .then_some(remainder)
            .or_else(|| remainder.strip_prefix(' '))
        else {
            return Vec::new();
        };
        let (sign, digits) = number
            .strip_prefix('-')
            .map_or((Sign::Positive, number), |digits| (Sign::Negative, digits));
        let digit_length = digits.bytes().take_while(u8::is_ascii_digit).count();
        let digits = &digits[..digit_length];
        let Some(magnitude) = (!digits.is_empty())
            .then(|| digits.parse::<u32>().ok())
            .flatten()
        else {
            return Vec::new();
        };
        (magnitude.to_string() == digits)
            .then_some(LexicalMatch {
                end: offset + prefix + usize::from(sign == Sign::Negative) + digit_length,
                value: Leaf::SignedNumber(SignedNumber { sign, magnitude }),
            })
            .into_iter()
            .collect()
    }
}

fn capitalize(word: &str) -> String {
    let Some(first) = word.chars().next() else {
        return String::new();
    };
    first.to_uppercase().chain(word.chars().skip(1)).collect()
}

fn rendered_catalog(kind: CatalogKind, spelling: &str) -> String {
    match kind {
        CatalogKind::CardTypes | CatalogKind::Supertypes => spelling.to_lowercase(),
        CatalogKind::AbilityWords
        | CatalogKind::ArtifactTypes
        | CatalogKind::BattleTypes
        | CatalogKind::CardNames
        | CatalogKind::CounterKindPhrases
        | CatalogKind::CreatureTypes
        | CatalogKind::EnchantmentTypes
        | CatalogKind::KeywordAbilities
        | CatalogKind::KeywordActions
        | CatalogKind::LandTypes
        | CatalogKind::PlaneswalkerTypes
        | CatalogKind::SpellTypes => spelling.to_owned(),
    }
}

fn noun_forms(singular: &str, wanted: NounNumber) -> Vec<(NounNumber, String)> {
    match wanted {
        NounNumber::Singular => vec![(NounNumber::Singular, singular.to_owned())],
        NounNumber::Plural => vec![(NounNumber::Plural, format!("{singular}s"))],
        NounNumber::Either => vec![
            (NounNumber::Singular, singular.to_owned()),
            (NounNumber::Plural, format!("{singular}s")),
        ],
    }
}

fn inflect(lexeme: VerbLexeme, agreement: Agreement) -> &'static str {
    match (lexeme, agreement) {
        (VerbLexeme::Destroy, Agreement::Bare) => "destroy",
        (VerbLexeme::Destroy, Agreement::ThirdPersonSingular) => "destroys",
        (VerbLexeme::Connive, Agreement::Bare) => "connive",
        (VerbLexeme::Connive, Agreement::ThirdPersonSingular) => "connives",
        (VerbLexeme::Deal, Agreement::Bare) => "deal",
        (VerbLexeme::Deal, Agreement::ThirdPersonSingular) => "deals",
        (VerbLexeme::Gain, Agreement::Bare) => "gain",
        (VerbLexeme::Gain, Agreement::ThirdPersonSingular) => "gains",
        (VerbLexeme::Control, Agreement::Bare) => "control",
        (VerbLexeme::Control, Agreement::ThirdPersonSingular) => "controls",
        (VerbLexeme::Be, Agreement::Bare) => "are",
        (VerbLexeme::Be, Agreement::ThirdPersonSingular) => "is",
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum BuildValue {
    Ability(Ability),
    Sentence(Sentence),
    Clause(Clause),
    NounPhrase(NounPhrase, Agreement),
    VerbPhrase(VerbPhrase, Agreement),
    Amount(Amount),
    Leaf(Leaf),
}

pub(crate) fn materialize(forest: &Forest<Construction, Leaf>) -> Vec<Ability> {
    let mut abilities = Vec::new();
    for root in forest.accepted_roots() {
        for value in materialize_node(forest, root) {
            if let BuildValue::Ability(ability) = value {
                push_unique(&mut abilities, ability);
            }
        }
    }
    abilities
}

fn materialize_node(
    forest: &Forest<Construction, Leaf>,
    node: &PackedNode<Construction, Leaf>,
) -> Vec<BuildValue> {
    let mut values = Vec::new();
    for family in &node.families {
        let mut combinations = vec![Vec::new()];
        for child in &family.children {
            let child_values = match child {
                Child::Node(id) => materialize_node(forest, forest.node(*id)),
                Child::Lexical(leaf) => vec![BuildValue::Leaf(leaf.clone())],
            };
            let mut next = Vec::new();
            for combination in combinations {
                for child_value in &child_values {
                    let mut combination = combination.clone();
                    combination.push(child_value.clone());
                    next.push(combination);
                }
            }
            combinations = next;
        }
        for children in combinations {
            if let Some(value) = build(node.construction, &children) {
                push_unique(&mut values, value);
            }
        }
    }
    values
}

fn push_unique<T: PartialEq>(values: &mut Vec<T>, value: T) {
    if !values.contains(&value) {
        values.push(value);
    }
}

fn build(construction: Construction, children: &[BuildValue]) -> Option<BuildValue> {
    match (construction, children) {
        (
            Construction::AbilitySpell,
            [
                BuildValue::Sentence(effect),
                BuildValue::Leaf(Leaf::Literal(".")),
            ],
        ) => Some(BuildValue::Ability(Ability::Spell(Spell {
            effect: effect.clone(),
        }))),
        (
            Construction::AbilityTriggered,
            [
                BuildValue::Leaf(Leaf::TriggerWord(trigger)),
                BuildValue::Clause(Clause::Event(event)),
                BuildValue::Leaf(Leaf::Literal(",")),
                BuildValue::Sentence(effect),
                BuildValue::Leaf(Leaf::Literal(".")),
            ],
        ) => Triggered::new(*trigger, Clause::Event(event.clone()), vec![effect.clone()])
            .map(Ability::Triggered)
            .map(BuildValue::Ability),
        (
            Construction::SentenceImperative,
            [BuildValue::VerbPhrase(predicate, Agreement::Bare)],
        ) => Some(BuildValue::Sentence(Sentence::Imperative(Imperative {
            predicate: predicate.clone(),
        }))),
        (
            Construction::SentenceDeclarative,
            [
                BuildValue::NounPhrase(subject, subject_agreement),
                BuildValue::VerbPhrase(predicate, verb_agreement),
            ],
        ) if subject_agreement == verb_agreement => {
            Some(BuildValue::Sentence(Sentence::Declarative(Declarative {
                subject: subject.clone(),
                predicate: predicate.clone(),
            })))
        }
        (
            Construction::SentenceWithWhere,
            [
                BuildValue::Sentence(body),
                BuildValue::Leaf(Leaf::Literal(",")),
                BuildValue::Clause(Clause::Where(clause)),
            ],
        ) => Some(BuildValue::Sentence(Sentence::WithWhere(WithWhere {
            body: Box::new(body.clone()),
            clause: Clause::Where(clause.clone()),
        }))),
        (
            Construction::ClauseEvent,
            [
                BuildValue::NounPhrase(subject, subject_agreement),
                BuildValue::VerbPhrase(predicate, verb_agreement),
            ],
        ) if subject_agreement == verb_agreement => {
            Some(BuildValue::Clause(Clause::Event(EventClause {
                subject: subject.clone(),
                predicate: predicate.clone(),
            })))
        }
        (
            Construction::ClauseWhere,
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
            ],
        ) => Some(BuildValue::Clause(Clause::Where(WhereClause {
            variable: *variable,
            value: value.clone(),
        }))),
        (Construction::NounPhrasePronoun, [BuildValue::Leaf(Leaf::Pronoun(pronoun))]) => {
            Some(BuildValue::NounPhrase(
                NounPhrase::Pronoun(PronounNp { word: *pronoun }),
                agreement_for_pronoun(*pronoun),
            ))
        }
        (
            Construction::NounPhraseCommon,
            [
                BuildValue::Leaf(Leaf::Article(article)),
                BuildValue::Leaf(Leaf::Noun {
                    noun,
                    number: NounNumber::Singular,
                }),
            ],
        ) => Some(BuildValue::NounPhrase(
            NounPhrase::Common(Common {
                article: *article,
                head: noun.clone(),
            }),
            Agreement::ThirdPersonSingular,
        )),
        (
            Construction::NounPhraseDemonstrative,
            [
                BuildValue::Leaf(Leaf::Demonstrative(Demonstrative::That)),
                BuildValue::Leaf(Leaf::Noun {
                    noun,
                    number: NounNumber::Singular,
                }),
            ],
        ) => Some(BuildValue::NounPhrase(
            NounPhrase::Demonstrative(DemonstrativeNp {
                word: Demonstrative::That,
                head: noun.clone(),
            }),
            Agreement::ThirdPersonSingular,
        )),
        (
            Construction::NounPhraseDemonstrative,
            [
                BuildValue::Leaf(Leaf::Demonstrative(Demonstrative::Those)),
                BuildValue::Leaf(Leaf::Noun {
                    noun,
                    number: NounNumber::Plural,
                }),
            ],
        ) => Some(BuildValue::NounPhrase(
            NounPhrase::Demonstrative(DemonstrativeNp {
                word: Demonstrative::Those,
                head: noun.clone(),
            }),
            Agreement::Bare,
        )),
        (
            Construction::NounPhraseTarget,
            [
                BuildValue::Leaf(Leaf::Literal("target")),
                BuildValue::Leaf(Leaf::Noun {
                    noun,
                    number: NounNumber::Singular,
                }),
            ],
        ) => Some(BuildValue::NounPhrase(
            NounPhrase::Target(TargetNp { head: noun.clone() }),
            Agreement::ThirdPersonSingular,
        )),
        (
            Construction::NounPhraseSelfReference,
            [BuildValue::Leaf(Leaf::SelfReference(spelling))],
        ) => Some(BuildValue::NounPhrase(
            NounPhrase::SelfReference(SelfReferenceNp {
                spelling: *spelling,
            }),
            Agreement::ThirdPersonSingular,
        )),
        (
            Construction::NounPhraseCount,
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
            ],
        ) => Some(BuildValue::NounPhrase(
            NounPhrase::Count(CountNp {
                head: noun.clone(),
                controller: Pronoun::You,
                threshold: threshold.clone(),
            }),
            Agreement::ThirdPersonSingular,
        )),
        (
            Construction::VerbPhraseDestroy,
            [
                BuildValue::Leaf(Leaf::Verb {
                    lexeme: VerbLexeme::Destroy,
                    agreement,
                }),
                BuildValue::NounPhrase(object, _),
            ],
        ) => Some(BuildValue::VerbPhrase(
            VerbPhrase::Destroy(Destroy {
                object: object.clone(),
            }),
            *agreement,
        )),
        (
            Construction::VerbPhraseConnive,
            [
                BuildValue::Leaf(Leaf::Verb {
                    lexeme: VerbLexeme::Connive,
                    agreement,
                }),
            ],
        ) => Some(BuildValue::VerbPhrase(
            VerbPhrase::Connive(Connive),
            *agreement,
        )),
        (
            Construction::VerbPhraseDealDamage,
            [
                BuildValue::Leaf(Leaf::Verb {
                    lexeme: VerbLexeme::Deal,
                    agreement,
                }),
                BuildValue::Amount(amount),
                BuildValue::Leaf(Leaf::Literal("damage")),
                BuildValue::Leaf(Leaf::Literal("to")),
                BuildValue::NounPhrase(to, _),
            ],
        ) => Some(BuildValue::VerbPhrase(
            VerbPhrase::DealDamage(DealDamage {
                amount: amount.clone(),
                to: to.clone(),
            }),
            *agreement,
        )),
        (
            Construction::VerbPhraseGainLife,
            [
                BuildValue::Leaf(Leaf::Verb {
                    lexeme: VerbLexeme::Gain,
                    agreement,
                }),
                BuildValue::Amount(amount),
                BuildValue::Leaf(Leaf::Literal("life")),
            ],
        ) => Some(BuildValue::VerbPhrase(
            VerbPhrase::GainLife(GainLife {
                amount: amount.clone(),
            }),
            *agreement,
        )),
        (Construction::AmountNumber, [BuildValue::Leaf(Leaf::SignedNumber(number))]) => {
            Some(BuildValue::Amount(Amount::Number(NumberAmount {
                number: number.clone(),
            })))
        }
        (Construction::AmountVariable, [BuildValue::Leaf(Leaf::Variable(variable))]) => {
            Some(BuildValue::Amount(Amount::Variable(VariableAmount {
                variable: *variable,
            })))
        }
        _ => None,
    }
}

fn agreement_for_pronoun(pronoun: Pronoun) -> Agreement {
    match pronoun {
        Pronoun::It => Agreement::ThirdPersonSingular,
        Pronoun::You => Agreement::Bare,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::path::Path;

    use super::Category;
    use super::ChartFailure;
    use super::Construction;
    use super::Forest;
    use super::Leaf;
    use super::Lexical;
    use super::RULES;
    use super::SliceGrammar;
    use super::materialize;
    use super::parse_forest;
    use crate::catalogs::ParserCatalogs;
    use crate::context::ParseContext;
    use crate::render::Render;

    fn slice_candidates(
        text: &str,
        card_name: &str,
    ) -> Result<Forest<Construction, Leaf>, ChartFailure<Category, Lexical>> {
        let catalogs = ParserCatalogs::load(
            &Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/gen/catalogs"),
        )
        .expect("canonical generated catalogs load");
        let context = ParseContext::new(card_name);
        let grammar = SliceGrammar {
            catalogs: &catalogs,
            context: &context,
        };

        parse_forest(&grammar, text)
    }

    #[test]
    fn slice_table_has_one_rule_per_construction_form() {
        let constructions = RULES
            .iter()
            .map(|rule| rule.construction)
            .collect::<BTreeSet<_>>();
        assert_eq!(RULES.len(), 19);
        assert_eq!(constructions.len(), 19);
    }

    #[test]
    fn scanner_accepts_multi_token_context_identity_and_catalog_nouns() {
        assert!(
            slice_candidates(
                "Zacama deals 3 damage to target creature.",
                "Zacama, Primal Calamity"
            )
            .is_ok()
        );
    }

    #[test]
    fn scanner_rejects_case_and_spacing_that_render_would_not_emit() {
        for text in [
            "destroy target creature.",
            "Destroy Target creature.",
            "Destroy  target creature.",
            "Destroytarget creature.",
            "Whenever a player connives, you Gain X life.",
        ] {
            assert!(
                slice_candidates(text, "Context Card").is_err(),
                "accepted {text:?}"
            );
        }
    }

    #[test]
    fn scanner_accepts_lowercase_you_after_the_trigger_comma() {
        assert!(
            slice_candidates(
                "Whenever a player connives, you gain X life.",
                "Context Card"
            )
            .is_ok()
        );
    }

    #[test]
    fn materialize_builds_checked_slice_values() {
        for (text, card_name) in [
            ("Destroy target creature.", "Context Card"),
            (
                "Whenever a player connives, you gain X life.",
                "Context Card",
            ),
            (
                "You gain X life, where X is the number of creatures you control with power 2 or less.",
                "Context Card",
            ),
            (
                "Zacama deals 3 damage to target creature.",
                "Zacama, Primal Calamity",
            ),
        ] {
            let forest = slice_candidates(text, card_name).expect("scanner accepts rendered input");
            let candidates = materialize(&forest);
            assert_eq!(candidates.len(), 1, "unexpected candidates for {text:?}");
            assert_eq!(candidates[0].render(&ParseContext::new(card_name)), text);
        }
    }

    #[test]
    fn materialize_rejects_families_with_invalid_agreement_or_count_facts() {
        for text in [
            "You gains X life.",
            "Creatures it controls with power 2 or less gain X life.",
        ] {
            let forest = slice_candidates(text, "Context Card").expect("scanner accepts words");
            assert!(materialize(&forest).is_empty(), "materialized {text:?}");
        }
    }
}
