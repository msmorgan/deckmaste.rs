use std::collections::BTreeMap;
use std::collections::BTreeSet;

use deckmaste_catalogs::CatalogKind;

use super::engine::ChartFailure;
use super::engine::Child;
use super::engine::Family;
use super::engine::Forest;
use super::engine::LexicalMatch;
use super::engine::NodeId;
use super::engine::Rule;
use super::engine::RulePosition;
use super::engine::SeedPolicy;
use super::engine::parse;
use crate::ast::Ability;
use crate::ast::Amount;
use crate::ast::Article;
use crate::ast::CatalogIdentity;
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
use crate::ast::Noun;
use crate::ast::NounLexeme;
use crate::ast::NounPhrase;
use crate::ast::NumberAmount;
use crate::ast::Pronoun;
use crate::ast::PronounNp;
use crate::ast::SelfReferenceNp;
use crate::ast::SelfReferenceSpelling;
use crate::ast::Sentence;
use crate::ast::Sign;
use crate::ast::SignedNumber;
use crate::ast::Spell;
use crate::ast::TargetNp;
use crate::ast::TriggerWord;
use crate::ast::Triggered;
use crate::ast::Variable;
use crate::ast::VariableAmount;
use crate::ast::VerbLexeme;
use crate::ast::VerbPhrase;
use crate::ast::WhereClause;
use crate::ast::WithWhere;
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
    EndOfInput,
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
        rhs: &[
            N(Category::Sentence),
            L(Lexical::Literal(".")),
            L(Lexical::EndOfInput),
        ],
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
            L(Lexical::EndOfInput),
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
    EndOfInput,
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
        |construction, family, forest| {
            completion_has_checked_build(*construction, family, forest, grammar.context)
        },
    )
}

impl SliceGrammar<'_> {
    fn scan(&self, lexical: Lexical, text: &str, offset: usize) -> Vec<LexicalMatch<Leaf>> {
        match lexical {
            Lexical::EndOfInput => (offset == text.len())
                .then_some(LexicalMatch {
                    end: offset,
                    value: Leaf::EndOfInput,
                })
                .into_iter()
                .collect(),
            Lexical::Literal(literal @ ("." | ",")) => text[offset..]
                .starts_with(literal)
                .then_some(LexicalMatch {
                    end: offset + literal.len(),
                    value: Leaf::Literal(literal),
                })
                .into_iter()
                .collect(),
            Lexical::Literal(literal) => Self::word(text, offset, literal)
                .map(|end| LexicalMatch {
                    end,
                    value: Leaf::Literal(literal),
                })
                .into_iter()
                .collect(),
            Lexical::TriggerWord => Self::closed_word(
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
                Self::word(text, offset, word).map(|end| LexicalMatch { end, value })
            })
            .collect(),
            Lexical::Demonstrative => [
                ("that", Leaf::Demonstrative(Demonstrative::That)),
                ("those", Leaf::Demonstrative(Demonstrative::Those)),
            ]
            .into_iter()
            .filter_map(|(word, value)| {
                Self::word(text, offset, word).map(|end| LexicalMatch { end, value })
            })
            .collect(),
            Lexical::Pronoun => [
                ("it", Leaf::Pronoun(Pronoun::It)),
                ("you", Leaf::Pronoun(Pronoun::You)),
            ]
            .into_iter()
            .filter_map(|(word, value)| {
                Self::word(text, offset, word).map(|end| LexicalMatch { end, value })
            })
            .collect(),
            Lexical::Variable => Self::closed_word(text, offset, "X", Leaf::Variable(Variable::X)),
            Lexical::Noun(number) => self.scan_noun(text, offset, number),
            Lexical::Verb(lexeme) => Self::scan_verb(text, offset, lexeme),
            Lexical::SignedNumber => Self::scan_signed_number(text, offset),
            Lexical::SelfReference => std::iter::once((
                self.context.card_name(),
                Leaf::SelfReference(SelfReferenceSpelling::Full),
            ))
            .chain(
                (self.context.abbreviated_card_name() != self.context.card_name()).then_some((
                    self.context.abbreviated_card_name(),
                    Leaf::SelfReference(SelfReferenceSpelling::Abbreviated),
                )),
            )
            .filter_map(|(word, value)| {
                Self::identity(text, offset, word).map(|end| LexicalMatch { end, value })
            })
            .collect(),
        }
    }

    fn closed_word(text: &str, offset: usize, word: &str, value: Leaf) -> Vec<LexicalMatch<Leaf>> {
        Self::word(text, offset, word)
            .map(|end| LexicalMatch { end, value })
            .into_iter()
            .collect()
    }

    fn word(text: &str, offset: usize, word: &str) -> Option<usize> {
        let prefix = usize::from(offset != 0);
        let remainder = text.get(offset..)?;
        let remainder = (prefix == 0)
            .then_some(remainder)
            .or_else(|| remainder.strip_prefix(' '))?;
        let word = if offset == 0 { capitalize(word) } else { word.to_owned() };
        let end = offset + prefix + word.len();
        (remainder.starts_with(&word) && has_lexical_boundary(text, end)).then_some(end)
    }

    fn identity(text: &str, offset: usize, identity: &str) -> Option<usize> {
        let prefix = usize::from(offset != 0);
        let remainder = text.get(offset..)?;
        let remainder = (prefix == 0)
            .then_some(remainder)
            .or_else(|| remainder.strip_prefix(' '))?;
        let end = offset + prefix + identity.len();
        (!identity.is_empty()
            && end > offset
            && remainder.starts_with(identity)
            && has_lexical_boundary(text, end))
        .then_some(end)
    }

    fn scan_noun(&self, text: &str, offset: usize, wanted: NounNumber) -> Vec<LexicalMatch<Leaf>> {
        let mut matches = Vec::new();
        for (noun, singular) in std::iter::once((
            Noun::Lexeme(NounLexeme::Player),
            "player".to_owned(),
        ))
        .chain(
            self.catalogs
                .set()
                .get(CatalogKind::CardTypes)
                .iter()
                .filter_map(|spelling| {
                    CatalogIdentity::new(self.catalogs, CatalogKind::CardTypes, spelling.clone())
                        .map(|identity| {
                            (
                                Noun::Catalog(identity),
                                rendered_catalog(CatalogKind::CardTypes, spelling),
                            )
                        })
                }),
        ) {
            for (number, word) in noun_forms(&singular, wanted) {
                if let Some(end) = Self::word(text, offset, &word) {
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

    fn scan_verb(text: &str, offset: usize, lexeme: VerbLexeme) -> Vec<LexicalMatch<Leaf>> {
        [Agreement::Bare, Agreement::ThirdPersonSingular]
            .into_iter()
            .filter_map(|agreement| {
                Self::word(text, offset, inflect(lexeme, agreement)).map(|end| LexicalMatch {
                    end,
                    value: Leaf::Verb { lexeme, agreement },
                })
            })
            .collect()
    }

    fn scan_signed_number(text: &str, offset: usize) -> Vec<LexicalMatch<Leaf>> {
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
        let end = offset + prefix + usize::from(sign == Sign::Negative) + digit_length;
        (magnitude.to_string() == digits && has_lexical_boundary(text, end))
            .then_some(LexicalMatch {
                end,
                value: Leaf::SignedNumber(SignedNumber { sign, magnitude }),
            })
            .into_iter()
            .collect()
    }
}

fn has_lexical_boundary(text: &str, end: usize) -> bool {
    matches!(text.as_bytes().get(end), None | Some(b' ' | b',' | b'.'))
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct BuiltCandidate {
    value: BuildValue,
    constructions: Vec<Construction>,
    positions: Vec<RulePosition<Category, Lexical>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Candidate {
    pub ability: Ability,
    pub constructions: Vec<Construction>,
    pub positions: Vec<RulePosition<Category, Lexical>>,
}

#[derive(Default)]
struct MaterializationState {
    memo: BTreeMap<NodeId, Vec<BuiltCandidate>>,
    in_progress: BTreeSet<NodeId>,
}

struct MaterializationOutcome {
    values: Vec<BuiltCandidate>,
    cycle_pruned: bool,
}

pub(crate) fn materialize(
    forest: &Forest<Construction, Leaf>,
    context: &ParseContext<'_>,
) -> Vec<Candidate> {
    let mut candidates = Vec::new();
    let mut state = MaterializationState::default();
    for root in forest.accepted_root_ids() {
        for built in materialize_node(forest, root, context, &mut state).values {
            if let BuildValue::Ability(ability) = built.value {
                push_unique(
                    &mut candidates,
                    Candidate {
                        ability,
                        constructions: built.constructions,
                        positions: built.positions,
                    },
                );
            }
        }
    }
    candidates
}

fn materialize_node(
    forest: &Forest<Construction, Leaf>,
    node_id: NodeId,
    context: &ParseContext<'_>,
    state: &mut MaterializationState,
) -> MaterializationOutcome {
    if let Some(values) = state.memo.get(&node_id) {
        return MaterializationOutcome {
            values: values.clone(),
            cycle_pruned: false,
        };
    }
    if !state.in_progress.insert(node_id) {
        return MaterializationOutcome {
            values: Vec::new(),
            cycle_pruned: true,
        };
    }

    let node = forest.node(node_id);
    let mut values = Vec::new();
    let mut cycle_pruned = false;
    for family in &node.families {
        let outcome = materialize_family(forest, node.construction, family, context, state);
        cycle_pruned |= outcome.cycle_pruned;
        for built in outcome.values {
            push_unique(&mut values, built);
        }
    }
    state.in_progress.remove(&node_id);
    if !cycle_pruned {
        state.memo.insert(node_id, values.clone());
    }
    MaterializationOutcome {
        values,
        cycle_pruned,
    }
}

fn completion_has_checked_build(
    construction: Construction,
    family: &Family<Leaf>,
    forest: &Forest<Construction, Leaf>,
    context: &ParseContext<'_>,
) -> bool {
    !materialize_family(
        forest,
        construction,
        family,
        context,
        &mut MaterializationState::default(),
    )
    .values
    .is_empty()
}

fn materialize_family(
    forest: &Forest<Construction, Leaf>,
    construction: Construction,
    family: &Family<Leaf>,
    context: &ParseContext<'_>,
    state: &mut MaterializationState,
) -> MaterializationOutcome {
    let rule = RULES
        .iter()
        .find(|rule| rule.construction == construction)
        .expect("every construction has exactly one declared rule");
    let mut combinations = vec![Vec::new()];
    let mut cycle_pruned = false;
    for child in &family.children {
        let child_values = match child {
            Child::Node(id) => {
                let outcome = materialize_node(forest, *id, context, state);
                cycle_pruned |= outcome.cycle_pruned;
                outcome.values
            }
            Child::Lexical(leaf) => vec![BuiltCandidate {
                value: BuildValue::Leaf(leaf.clone()),
                constructions: Vec::new(),
                positions: Vec::new(),
            }],
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
    let mut values = Vec::new();
    for children in combinations {
        let child_values = children
            .iter()
            .map(|child| child.value.clone())
            .collect::<Vec<_>>();
        if let Some(value) = build(construction, &child_values, context) {
            let mut constructions = vec![construction];
            let mut positions = rule.rhs.to_vec();
            for child in children {
                constructions.extend(child.constructions);
                positions.extend(child.positions);
            }
            push_unique(
                &mut values,
                BuiltCandidate {
                    value,
                    constructions,
                    positions,
                },
            );
        }
    }
    MaterializationOutcome {
        values,
        cycle_pruned,
    }
}

fn push_unique<T: PartialEq>(values: &mut Vec<T>, value: T) {
    if !values.contains(&value) {
        values.push(value);
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "the exhaustive generated-shape construction dispatch is intentionally flat"
)]
fn build(
    construction: Construction,
    children: &[BuildValue],
    context: &ParseContext<'_>,
) -> Option<BuildValue> {
    match construction {
        Construction::AbilitySpell => match children {
            [
                BuildValue::Sentence(effect),
                BuildValue::Leaf(Leaf::Literal(".")),
                BuildValue::Leaf(Leaf::EndOfInput),
            ] => Some(BuildValue::Ability(Ability::Spell(Spell {
                effect: effect.clone(),
            }))),
            _ => None,
        },
        Construction::AbilityTriggered => match children {
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
        Construction::SentenceImperative => match children {
            [BuildValue::VerbPhrase(predicate, Agreement::Bare)] => {
                Some(BuildValue::Sentence(Sentence::Imperative(Imperative {
                    predicate: predicate.clone(),
                })))
            }
            _ => None,
        },
        Construction::SentenceDeclarative => match children {
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
        Construction::SentenceWithWhere => match children {
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
        Construction::ClauseEvent => match children {
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
        Construction::ClauseWhere => match children {
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
        Construction::NounPhrasePronoun => match children {
            [BuildValue::Leaf(Leaf::Pronoun(pronoun))] => Some(BuildValue::NounPhrase(
                NounPhrase::Pronoun(PronounNp { word: *pronoun }),
                agreement_for_pronoun(*pronoun),
            )),
            _ => None,
        },
        Construction::NounPhraseCommon => match children {
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
        Construction::NounPhraseDemonstrative => match children {
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
        Construction::NounPhraseTarget => match children {
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
        Construction::NounPhraseSelfReference => match children {
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
        Construction::NounPhraseCount => match children {
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
        Construction::VerbPhraseDestroy => match children {
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
        Construction::VerbPhraseConnive => match children {
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
        Construction::VerbPhraseDealDamage => match children {
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
        Construction::VerbPhraseGainLife => match children {
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
        Construction::AmountNumber => match children {
            [BuildValue::Leaf(Leaf::SignedNumber(number))] => {
                Some(BuildValue::Amount(Amount::Number(NumberAmount {
                    number: number.clone(),
                })))
            }
            _ => None,
        },
        Construction::AmountVariable => match children {
            [BuildValue::Leaf(Leaf::Variable(variable))] => {
                Some(BuildValue::Amount(Amount::Variable(VariableAmount {
                    variable: *variable,
                })))
            }
            _ => None,
        },
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

    use super::BuildValue;
    use super::Category;
    use super::ChartFailure;
    use super::Construction;
    use super::Forest;
    use super::Leaf;
    use super::Lexical;
    use super::NounNumber;
    use super::RULES;
    use super::SliceGrammar;
    use super::materialize;
    use super::parse_forest;
    use crate::ast::Clause;
    use crate::ast::Connive;
    use crate::ast::Imperative;
    use crate::ast::NounPhrase;
    use crate::ast::Pronoun;
    use crate::ast::PronounNp;
    use crate::ast::Sentence;
    use crate::ast::Sign;
    use crate::ast::SignedNumber;
    use crate::ast::Variable;
    use crate::ast::VerbLexeme;
    use crate::ast::VerbPhrase;
    use crate::ast::WhereClause;
    use crate::ast::WithWhere;
    use crate::catalogs::ParserCatalogs;
    use crate::context::ParseContext;
    use crate::parser::engine::Child;
    use crate::parser::engine::Family;
    use crate::parser::engine::NodeId;
    use crate::parser::engine::PackedNode;
    use crate::parser::engine::RulePosition;
    use crate::render::Render;

    fn slice_candidates(
        text: &str,
        card_name: &str,
    ) -> Result<Forest<Construction, Leaf>, ChartFailure<Category, Lexical>> {
        let catalogs = ParserCatalogs::load(
            &Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/gen/catalogs"),
        )
        .expect("canonical generated catalogs load");
        let context = context(card_name);
        let grammar = SliceGrammar {
            catalogs: &catalogs,
            context: &context,
        };

        parse_forest(&grammar, text)
    }

    fn context(card_name: &str) -> ParseContext<'_> {
        ParseContext::new(card_name).expect("test card names are valid parse contexts")
    }

    fn assert_acyclic_number_survives(forest: &Forest<Construction, Leaf>) {
        let mut state = super::MaterializationState::default();
        let outcome =
            super::materialize_node(forest, NodeId(0), &context("Context Card"), &mut state);

        assert_eq!(outcome.values.len(), 1);
        assert!(matches!(
            outcome.values[0].value,
            BuildValue::Amount(crate::ast::Amount::Number(crate::ast::NumberAmount {
                number: SignedNumber {
                    sign: Sign::Positive,
                    magnitude: 3,
                },
            }))
        ));
    }

    fn with_where_family(body: NodeId) -> Family<Leaf> {
        Family {
            children: vec![
                Child::Node(body),
                Child::Lexical(Leaf::Literal(",")),
                Child::Node(NodeId(4)),
            ],
        }
    }

    #[test]
    fn materialization_rejects_a_direct_nullable_cycle_but_keeps_an_acyclic_family() {
        let forest = Forest::from_test_parts(
            vec![PackedNode {
                construction: Construction::AmountNumber,
                start: 0,
                end: 0,
                families: vec![
                    Family {
                        children: vec![Child::Node(NodeId(0))],
                    },
                    Family {
                        children: vec![Child::Lexical(Leaf::SignedNumber(SignedNumber {
                            sign: Sign::Positive,
                            magnitude: 3,
                        }))],
                    },
                ],
            }],
            vec![NodeId(0)],
        );

        assert_acyclic_number_survives(&forest);
    }

    #[test]
    fn materialization_rejects_an_indirect_nullable_cycle_but_keeps_an_acyclic_family() {
        let forest = Forest::from_test_parts(
            vec![
                PackedNode {
                    construction: Construction::AmountNumber,
                    start: 0,
                    end: 0,
                    families: vec![
                        Family {
                            children: vec![Child::Node(NodeId(1))],
                        },
                        Family {
                            children: vec![Child::Lexical(Leaf::SignedNumber(SignedNumber {
                                sign: Sign::Positive,
                                magnitude: 3,
                            }))],
                        },
                    ],
                },
                PackedNode {
                    construction: Construction::AmountNumber,
                    start: 0,
                    end: 0,
                    families: vec![Family {
                        children: vec![Child::Node(NodeId(0))],
                    }],
                },
            ],
            vec![NodeId(0)],
        );

        assert_acyclic_number_survives(&forest);
    }

    #[test]
    fn cycle_tainted_results_are_recomputed_for_a_later_clean_root() {
        let forest = Forest::from_test_parts(
            vec![
                PackedNode {
                    construction: Construction::SentenceWithWhere,
                    start: 0,
                    end: 0,
                    families: vec![with_where_family(NodeId(1)), with_where_family(NodeId(2))],
                },
                PackedNode {
                    construction: Construction::SentenceWithWhere,
                    start: 0,
                    end: 0,
                    families: vec![with_where_family(NodeId(0))],
                },
                PackedNode {
                    construction: Construction::SentenceImperative,
                    start: 0,
                    end: 0,
                    families: vec![Family {
                        children: vec![Child::Node(NodeId(3))],
                    }],
                },
                PackedNode {
                    construction: Construction::VerbPhraseConnive,
                    start: 0,
                    end: 0,
                    families: vec![Family {
                        children: vec![Child::Lexical(Leaf::Verb {
                            lexeme: VerbLexeme::Connive,
                            agreement: super::Agreement::Bare,
                        })],
                    }],
                },
                PackedNode {
                    construction: Construction::ClauseWhere,
                    start: 0,
                    end: 0,
                    families: vec![Family {
                        children: vec![
                            Child::Lexical(Leaf::Literal("where")),
                            Child::Lexical(Leaf::Variable(Variable::X)),
                            Child::Lexical(Leaf::Verb {
                                lexeme: VerbLexeme::Be,
                                agreement: super::Agreement::ThirdPersonSingular,
                            }),
                            Child::Lexical(Leaf::Literal("the")),
                            Child::Lexical(Leaf::Literal("number")),
                            Child::Lexical(Leaf::Literal("of")),
                            Child::Node(NodeId(5)),
                        ],
                    }],
                },
                PackedNode {
                    construction: Construction::NounPhrasePronoun,
                    start: 0,
                    end: 0,
                    families: vec![Family {
                        children: vec![Child::Lexical(Leaf::Pronoun(Pronoun::You))],
                    }],
                },
            ],
            vec![NodeId(0), NodeId(1)],
        );
        let context = context("Context Card");
        let mut state = super::MaterializationState::default();
        let clause = Clause::Where(WhereClause {
            variable: Variable::X,
            value: NounPhrase::Pronoun(PronounNp { word: Pronoun::You }),
        });
        let base = Sentence::Imperative(Imperative {
            predicate: VerbPhrase::Connive(Connive),
        });
        let once = Sentence::WithWhere(WithWhere {
            body: Box::new(base),
            clause: clause.clone(),
        });

        let first = super::materialize_node(&forest, NodeId(0), &context, &mut state);
        assert_eq!(first.values.len(), 1);
        assert_eq!(first.values[0].value, BuildValue::Sentence(once.clone()));

        let later = super::materialize_node(&forest, NodeId(1), &context, &mut state);
        assert_eq!(later.values.len(), 1);
        assert_eq!(
            later.values[0].value,
            BuildValue::Sentence(Sentence::WithWhere(WithWhere {
                body: Box::new(once),
                clause,
            }))
        );
        assert_eq!(
            later.values[0].constructions,
            vec![
                Construction::SentenceWithWhere,
                Construction::SentenceWithWhere,
                Construction::SentenceImperative,
                Construction::VerbPhraseConnive,
                Construction::ClauseWhere,
                Construction::NounPhrasePronoun,
                Construction::ClauseWhere,
                Construction::NounPhrasePronoun,
            ]
        );
        assert_eq!(
            later.values[0].positions,
            vec![
                RulePosition::Nonterminal(Category::Sentence),
                RulePosition::Lexical(Lexical::Literal(",")),
                RulePosition::Nonterminal(Category::Clause),
                RulePosition::Nonterminal(Category::Sentence),
                RulePosition::Lexical(Lexical::Literal(",")),
                RulePosition::Nonterminal(Category::Clause),
                RulePosition::Nonterminal(Category::VerbPhrase),
                RulePosition::Lexical(Lexical::Verb(VerbLexeme::Connive)),
                RulePosition::Lexical(Lexical::Literal("where")),
                RulePosition::Lexical(Lexical::Variable),
                RulePosition::Lexical(Lexical::Verb(VerbLexeme::Be)),
                RulePosition::Lexical(Lexical::Literal("the")),
                RulePosition::Lexical(Lexical::Literal("number")),
                RulePosition::Lexical(Lexical::Literal("of")),
                RulePosition::Nonterminal(Category::NounPhrase),
                RulePosition::Lexical(Lexical::Pronoun),
                RulePosition::Lexical(Lexical::Literal("where")),
                RulePosition::Lexical(Lexical::Variable),
                RulePosition::Lexical(Lexical::Verb(VerbLexeme::Be)),
                RulePosition::Lexical(Lexical::Literal("the")),
                RulePosition::Lexical(Lexical::Literal("number")),
                RulePosition::Lexical(Lexical::Literal("of")),
                RulePosition::Nonterminal(Category::NounPhrase),
                RulePosition::Lexical(Lexical::Pronoun),
            ]
        );
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
            let context = context(card_name);
            let candidates = materialize(&forest, &context);
            assert_eq!(candidates.len(), 1, "unexpected candidates for {text:?}");
            assert_eq!(candidates[0].ability.render(&context), text);
        }
    }

    #[test]
    fn chart_completion_rejects_invalid_agreement_and_count_facts() {
        for text in [
            "You gains X life.",
            "Creatures you control with power 2 or less gains X life.",
        ] {
            assert!(
                slice_candidates(text, "Context Card").is_err(),
                "completed invalid chart family for {text:?}"
            );
        }
    }

    #[test]
    fn count_np_requires_you_and_bare_control() {
        for text in [
            "You gain X life, where X is the number of creatures it control with power 2 or less.",
            "You gain X life, where X is the number of creatures you controls with power 2 or less.",
        ] {
            assert!(
                slice_candidates(text, "Context Card").is_err(),
                "completed invalid count noun phrase for {text:?}"
            );
        }

        assert!(
            slice_candidates(
                "You gain X life, where X is the number of creatures you control with power 2 or less.",
                "Context Card"
            )
            .is_ok()
        );
    }

    #[test]
    fn chart_completion_rejects_a_construction_category_mismatch() {
        let text = "Whenever where X is the number of creatures you control with power 2 or less, you gain X life.";
        let Err(failure) = slice_candidates(text, "Context Card") else {
            panic!("a where clause cannot satisfy an event-clause construction role");
        };

        assert!(!failure.live.is_empty());
    }

    #[test]
    fn materialize_accepts_a_plural_count_subject_with_a_bare_verb() {
        let text = "Creatures you control with power 2 or less gain X life.";
        let forest = slice_candidates(text, "Context Card").expect("scanner accepts words");
        let context = context("Context Card");
        let candidates = materialize(&forest, &context);
        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].ability.render(&context), text);
    }

    #[test]
    fn materialize_preserves_preorder_constructions_and_declared_positions() {
        let forest = slice_candidates("Destroy target creature.", "Context Card").unwrap();
        let candidates = materialize(&forest, &context("Context Card"));

        assert_eq!(candidates.len(), 1);
        assert_eq!(
            candidates[0].constructions,
            vec![
                Construction::AbilitySpell,
                Construction::SentenceImperative,
                Construction::VerbPhraseDestroy,
                Construction::NounPhraseTarget,
            ]
        );
        assert_eq!(
            candidates[0].positions,
            vec![
                RulePosition::Nonterminal(Category::Sentence),
                RulePosition::Lexical(Lexical::Literal(".")),
                RulePosition::Lexical(Lexical::EndOfInput),
                RulePosition::Nonterminal(Category::VerbPhrase),
                RulePosition::Lexical(Lexical::Verb(VerbLexeme::Destroy)),
                RulePosition::Nonterminal(Category::NounPhrase),
                RulePosition::Lexical(Lexical::Literal("target")),
                RulePosition::Lexical(Lexical::Noun(NounNumber::Singular)),
            ]
        );
    }
}
