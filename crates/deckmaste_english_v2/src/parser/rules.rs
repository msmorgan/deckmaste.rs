use super::engine::Rule;
use super::engine::RulePosition;
use crate::ast::VerbLexeme;

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

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub(crate) enum RuleId {
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

impl RuleId {
    #[cfg(test)]
    pub(crate) const COUNT: usize = 19;

    pub(crate) const fn construction(self) -> Construction {
        match self {
            Self::AbilitySpell => Construction::AbilitySpell,
            Self::AbilityTriggered => Construction::AbilityTriggered,
            Self::SentenceImperative => Construction::SentenceImperative,
            Self::SentenceDeclarative => Construction::SentenceDeclarative,
            Self::SentenceWithWhere => Construction::SentenceWithWhere,
            Self::ClauseEvent => Construction::ClauseEvent,
            Self::ClauseWhere => Construction::ClauseWhere,
            Self::NounPhrasePronoun => Construction::NounPhrasePronoun,
            Self::NounPhraseCommon => Construction::NounPhraseCommon,
            Self::NounPhraseDemonstrative => Construction::NounPhraseDemonstrative,
            Self::NounPhraseTarget => Construction::NounPhraseTarget,
            Self::NounPhraseSelfReference => Construction::NounPhraseSelfReference,
            Self::NounPhraseCount => Construction::NounPhraseCount,
            Self::VerbPhraseDestroy => Construction::VerbPhraseDestroy,
            Self::VerbPhraseConnive => Construction::VerbPhraseConnive,
            Self::VerbPhraseDealDamage => Construction::VerbPhraseDealDamage,
            Self::VerbPhraseGainLife => Construction::VerbPhraseGainLife,
            Self::AmountNumber => Construction::AmountNumber,
            Self::AmountVariable => Construction::AmountVariable,
        }
    }

    pub(crate) const fn index(self) -> usize {
        self as usize
    }
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

pub(crate) const RULES: &[Rule<Category, Lexical, RuleId>] = &[
    Rule {
        id: RuleId::AbilitySpell,
        lhs: Category::Ability,
        rhs: &[
            N(Category::Sentence),
            L(Lexical::Literal(".")),
            L(Lexical::EndOfInput),
        ],
    },
    Rule {
        id: RuleId::AbilityTriggered,
        lhs: Category::Ability,
        rhs: &[
            L(Lexical::TriggerWord),
            N(Category::Clause),
            L(Lexical::Literal(",")),
            N(Category::Sentence),
            L(Lexical::Literal(".")),
            L(Lexical::EndOfInput),
        ],
    },
    Rule {
        id: RuleId::SentenceImperative,
        lhs: Category::Sentence,
        rhs: &[N(Category::VerbPhrase)],
    },
    Rule {
        id: RuleId::SentenceDeclarative,
        lhs: Category::Sentence,
        rhs: &[N(Category::NounPhrase), N(Category::VerbPhrase)],
    },
    Rule {
        id: RuleId::SentenceWithWhere,
        lhs: Category::Sentence,
        rhs: &[
            N(Category::Sentence),
            L(Lexical::Literal(",")),
            N(Category::Clause),
        ],
    },
    Rule {
        id: RuleId::ClauseEvent,
        lhs: Category::Clause,
        rhs: &[N(Category::NounPhrase), N(Category::VerbPhrase)],
    },
    Rule {
        id: RuleId::ClauseWhere,
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
    },
    Rule {
        id: RuleId::NounPhrasePronoun,
        lhs: Category::NounPhrase,
        rhs: &[L(Lexical::Pronoun)],
    },
    Rule {
        id: RuleId::NounPhraseCommon,
        lhs: Category::NounPhrase,
        rhs: &[L(Lexical::Article), L(Lexical::Noun(NounNumber::Singular))],
    },
    Rule {
        id: RuleId::NounPhraseDemonstrative,
        lhs: Category::NounPhrase,
        rhs: &[
            L(Lexical::Demonstrative),
            L(Lexical::Noun(NounNumber::Either)),
        ],
    },
    Rule {
        id: RuleId::NounPhraseTarget,
        lhs: Category::NounPhrase,
        rhs: &[
            L(Lexical::Literal("target")),
            L(Lexical::Noun(NounNumber::Singular)),
        ],
    },
    Rule {
        id: RuleId::NounPhraseSelfReference,
        lhs: Category::NounPhrase,
        rhs: &[L(Lexical::SelfReference)],
    },
    Rule {
        id: RuleId::NounPhraseCount,
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
    },
    Rule {
        id: RuleId::VerbPhraseDestroy,
        lhs: Category::VerbPhrase,
        rhs: &[
            L(Lexical::Verb(VerbLexeme::Destroy)),
            N(Category::NounPhrase),
        ],
    },
    Rule {
        id: RuleId::VerbPhraseConnive,
        lhs: Category::VerbPhrase,
        rhs: &[L(Lexical::Verb(VerbLexeme::Connive))],
    },
    Rule {
        id: RuleId::VerbPhraseDealDamage,
        lhs: Category::VerbPhrase,
        rhs: &[
            L(Lexical::Verb(VerbLexeme::Deal)),
            N(Category::Amount),
            L(Lexical::Literal("damage")),
            L(Lexical::Literal("to")),
            N(Category::NounPhrase),
        ],
    },
    Rule {
        id: RuleId::VerbPhraseGainLife,
        lhs: Category::VerbPhrase,
        rhs: &[
            L(Lexical::Verb(VerbLexeme::Gain)),
            N(Category::Amount),
            L(Lexical::Literal("life")),
        ],
    },
    Rule {
        id: RuleId::AmountNumber,
        lhs: Category::Amount,
        rhs: &[L(Lexical::SignedNumber)],
    },
    Rule {
        id: RuleId::AmountVariable,
        lhs: Category::Amount,
        rhs: &[L(Lexical::Variable)],
    },
];

#[cfg(test)]
mod tests {
    use super::RULES;
    use super::RuleId;

    #[test]
    fn slice_table_has_exactly_one_row_and_build_arm_per_rule_id() {
        assert_eq!(RULES.len(), RuleId::COUNT);
        assert!(
            RULES
                .iter()
                .enumerate()
                .all(|(index, rule)| rule.id.index() == index)
        );
    }
}
