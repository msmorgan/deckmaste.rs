use crate::forest::NodeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum AdjectivePhraseMeaning {
    Head { adjective: NodeId },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum NominalMeaning {
    Head { noun: NodeId },
    Adjective { adjective: NodeId, nominal: NodeId },
    NounModifier { noun: NodeId, nominal: NodeId },
    PowerToughnessModifier { modifier: NodeId, nominal: NodeId },
    Determined { determiner: NodeId, nominal: NodeId },
    Prepositional { nominal: NodeId, phrase: NodeId },
    Relative { nominal: NodeId, clause: NodeId },
    UnknownModifier { modifier: NodeId, nominal: NodeId },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum NounPhraseMeaning {
    Nominal {
        nominal: NodeId,
    },
    Pronoun {
        pronoun: NodeId,
    },
    Reciprocal {
        pronoun: NodeId,
    },
    ThisCard {
        reference: NodeId,
    },
    Coordinated {
        first: NodeId,
        conjunction: NodeId,
        next: NodeId,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum PossessiveNounPhraseMeaning {
    Head {
        noun: NodeId,
    },
    Determined {
        determiner: NodeId,
        possessor: NodeId,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum PrepositionalPhraseMeaning {
    NounObject { preposition: NodeId, object: NodeId },
    UnknownObject { preposition: NodeId, object: NodeId },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum VerbPhraseMeaning {
    Head {
        verb: NodeId,
    },
    Auxiliary {
        auxiliary: NodeId,
        predicate: NodeId,
    },
    DirectObject {
        predicate: NodeId,
        object: NodeId,
    },
    Adjective {
        predicate: NodeId,
        adjective: NodeId,
    },
    Prepositional {
        predicate: NodeId,
        phrase: NodeId,
    },
    Infinitive {
        predicate: NodeId,
        clause: NodeId,
    },
    Adverb {
        predicate: NodeId,
        adverb: NodeId,
    },
    Ability {
        predicate: NodeId,
        ability: NodeId,
    },
    OracleSymbol {
        predicate: NodeId,
        symbol: NodeId,
    },
    PowerToughness {
        predicate: NodeId,
        value: NodeId,
    },
    Quantity {
        predicate: NodeId,
        quantity: NodeId,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum InfinitiveClauseMeaning {
    To { marker: NodeId, predicate: NodeId },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum SimpleClauseMeaning {
    Subject { subject: NodeId, predicate: NodeId },
    Subjectless { predicate: NodeId },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum RelativeClauseMeaning {
    ObjectGap { clause: NodeId },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum ClauseMeaning {
    Simple {
        clause: NodeId,
    },
    Elliptical {
        phrase: NodeId,
    },
    Coordinated {
        first: NodeId,
        comma: Option<NodeId>,
        conjunction: NodeId,
        next: NodeId,
    },
    ConditionalBefore {
        subordinator: NodeId,
        condition: NodeId,
        comma: NodeId,
        consequence: NodeId,
    },
    ConditionalAfterElliptical {
        consequence: NodeId,
        subordinator: NodeId,
        condition: NodeId,
    },
    ConditionalAfter {
        consequence: NodeId,
        subordinator: NodeId,
        condition: NodeId,
    },
    Existential {
        form: NodeId,
        pivot: NodeId,
    },
    CopularNoun {
        subject: NodeId,
        copula: Option<NodeId>,
        complement: NodeId,
    },
    CopularAdjective {
        subject: NodeId,
        copula: Option<NodeId>,
        complement: NodeId,
    },
    CopularPrepositional {
        subject: NodeId,
        copula: Option<NodeId>,
        complement: NodeId,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum SentenceMeaning {
    Period { clause: NodeId, punctuation: NodeId },
    Exclamation { clause: NodeId, punctuation: NodeId },
    Question { clause: NodeId, punctuation: NodeId },
    Unpunctuated { clause: NodeId },
}
