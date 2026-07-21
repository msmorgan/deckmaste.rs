use super::phrase::NounPhrase;
use super::phrase::Phrase;
use super::phrase::PrepositionalPhrase;
use super::phrase::UnknownPhrase;
use crate::word::AuxiliaryInstance;
use crate::word::VerbInstance;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Clause {
    Simple(SimpleClause),
    Elliptical(Phrase),
    Conditional(ConditionalClause),
    Coordinated(CoordinatedClause),
    Unknown(UnknownPhrase),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimpleClause {
    pub subject: Option<Subject>,
    pub predicate: VerbPhrase,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Subject {
    NounPhrase(NounPhrase),
    Unknown(UnknownPhrase),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerbPhrase {
    pub auxiliaries: Vec<AuxiliaryInstance>,
    pub preverb_modifiers: Vec<PreverbModifier>,
    pub verb: VerbInstance,
    pub dependents: Vec<VerbDependent>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreverbModifier {
    Not,
    Also,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerbDependent {
    DirectObject(NounPhrase),
    IndirectObject(NounPhrase),
    PredicateComplement(Phrase),
    Scalar(Phrase),
    Statistic(Phrase),
    Prepositional(PrepositionalPhrase),
    Infinitive(InfinitiveClause),
    Subordinate(Box<Clause>),
    Adverbial(Phrase),
    Unknown(UnknownPhrase),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InfinitiveClause {
    pub marker: InfinitiveMarker,
    pub predicate: Box<VerbPhrase>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InfinitiveMarker {
    Bare,
    To,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelativeClause {
    pub gap: RelativeGap,
    pub clause: Box<Clause>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelativeGap {
    Subject,
    Object,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionalClause {
    pub subordinator: Subordinator,
    pub position: ConditionalPosition,
    pub condition: Box<Clause>,
    pub consequence: Box<Clause>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConditionalPosition {
    BeforeConsequence,
    AfterConsequence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Subordinator {
    When,
    If,
    Unless,
    AsLongAs,
    Until,
    Because,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoordinatedClause {
    pub first: Box<Clause>,
    pub rest: Vec<ClauseCoordination>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClauseCoordination {
    pub conjunction: PredicateConjunction,
    pub comma: bool,
    pub clause: Clause,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PredicateConjunction {
    And,
    Or,
    Then,
}
