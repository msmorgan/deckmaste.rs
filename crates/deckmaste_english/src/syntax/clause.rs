use super::ability::Ability;
use super::ability::QuotedAbility;
use super::phrase::AdjectivePhrase;
use super::phrase::NounPhrase;
use super::phrase::NumberLiteral;
use super::phrase::OracleSymbol;
use super::phrase::PowerToughness;
use super::phrase::PrepositionalPhrase;
use super::phrase::Quantity;
use crate::catalog::CatalogAtom;
use crate::word::AuxiliaryInstance;
use crate::word::VerbInstance;
use crate::word::Vocab;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Clause {
    Independent(IndependentClause),
    Dependent(DependentClause),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IndependentClause {
    Transitive(Subject, TransitivePredicate),
    Intransitive(Subject, IntransitivePredicate),
    Copular(Subject, CopularPredicate),
    Passive(Subject, PassivePredicate),
    Imperative(Predicate),
    Deontic(Subject, Modal, Predicate),
    Existential(ExistentialClause),
    Proform(Subject, ProPredicate),
    Complex(ComplexClause),
    Coordinated(CoordinatedIndependentClause),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DependentClause {
    Subordinate(Subordinator, SubordinateBody),
    Relative(RelativeClause),
    Infinitive(InfinitiveClause),
    Gerund(GerundClause),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubordinateBody {
    Finite(Box<IndependentClause>),
    Infinitive(InfinitiveClause),
    Gerund(GerundClause),
    Elliptical(EllipticalClause),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EllipticalClause {
    Adjective(AdjectivePhrase),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subject(pub NounPhrase);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Predicate {
    Transitive(TransitivePredicate),
    Intransitive(IntransitivePredicate),
    Copular(CopularPredicate),
    Passive(PassivePredicate),
    Proform(ProPredicate),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PredicateHead {
    pub auxiliaries: Vec<AuxiliaryInstance>,
    pub first_auxiliary_contracted_with_subject: bool,
    pub preverb_modifiers: Vec<PreverbModifier>,
    pub verb: VerbInstance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreverbModifier {
    Not,
    Also,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransitivePredicate {
    pub head: PredicateHead,
    pub object: PredicateObject,
    pub elements: Vec<PredicateElement>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntransitivePredicate {
    pub head: PredicateHead,
    pub elements: Vec<PredicateElement>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PassivePredicate {
    pub head: PredicateHead,
    pub elements: Vec<PredicateElement>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CopularPredicate {
    pub copula: Copula,
    pub precomplement_adverbs: Vec<Vocab>,
    pub complement: CopularComplement,
    pub adjuncts: Vec<PredicateAdjunct>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Copula {
    pub auxiliary: AuxiliaryInstance,
    pub contracted_with_subject: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CopularComplement {
    NounPhrase(NounPhrase),
    Adjective(AdjectivePhrase),
    Prepositional(PrepositionalPhrase),
    CatalogAtom(CatalogAtom),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Modal {
    pub auxiliary: AuxiliaryInstance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProPredicate {
    pub auxiliary: AuxiliaryInstance,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectGapPredicate {
    pub head: PredicateHead,
    pub elements: Vec<PredicateElement>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PredicateObject {
    NounPhrase(NounPhrase),
    Ability(AbilityObject),
    Quantity(Quantity),
    OracleSymbol(OracleSymbol),
    SymbolSequence(Vec<OracleSymbol>),
    PowerToughness(PowerToughness),
    EmbeddedAbility(Box<Ability>),
    QuotedAbility(Box<QuotedAbility>),
    Coordinated(CoordinatedPredicateObject),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoordinatedPredicateObject {
    pub first: Box<PredicateObject>,
    pub rest: Vec<PredicateObjectCoordination>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PredicateObjectCoordination {
    pub conjunction: PredicateConjunction,
    pub comma: bool,
    pub object: PredicateObject,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbilityObject {
    pub ability: CatalogAtom,
    pub argument: Option<Box<PredicateObject>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PredicateComplement {
    IndirectObject(NounPhrase),
    Adjective(AdjectivePhrase),
    Prepositional(PrepositionalPhrase),
    Infinitive(InfinitiveClause),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PredicateElement {
    Complement(PredicateComplement),
    Adjunct(PredicateAdjunct),
    Particle(VerbParticle),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VerbParticle {
    In,
    Out,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PredicateAdjunct {
    Adverb(Vocab),
    Frequency(FrequencyPhrase),
    Temporal(NounPhrase),
    Manner(NounPhrase),
    Prepositional(PrepositionalPhrase),
    Dependent(Box<DependentClause>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrequencyPhrase {
    pub bound: FrequencyBound,
    pub count: FrequencyCount,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FrequencyBound {
    MoreThan,
    NoMoreThan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrequencyCount {
    Once,
    Twice,
    Times(NumberLiteral),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InfinitiveClause {
    pub negated: bool,
    pub marker: InfinitiveMarker,
    pub predicate: Box<Predicate>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GerundClause {
    pub predicate: Box<Predicate>,
    pub attachments: Vec<DependentAttachment>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InfinitiveMarker {
    Bare,
    To,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelativeClause {
    pub marker: RelativeMarker,
    pub gap: RelativeGap,
    pub body: RelativeBody,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelativeMarker {
    That,
    Which,
    Who,
    Zero,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelativeGap {
    Subject,
    Object,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelativeBody {
    SubjectGap(Predicate),
    ObjectGap {
        subject: Subject,
        predicate: ObjectGapPredicate,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComplexClause {
    pub matrix: Box<IndependentClause>,
    pub attachments: Vec<ClauseAttachment>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClauseAttachment {
    pub position: AttachmentPosition,
    pub comma: bool,
    pub kind: ClauseAttachmentKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClauseAttachmentKind {
    Dependent(DependentClause),
    Adjunct(PredicateAdjunct),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependentAttachment {
    pub position: AttachmentPosition,
    pub comma: bool,
    pub clause: DependentClause,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttachmentPosition {
    BeforeMatrix,
    AfterMatrix,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoordinatedIndependentClause {
    pub first: Box<IndependentClause>,
    pub rest: Vec<ClauseCoordination>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClauseCoordination {
    pub conjunction: Option<PredicateConjunction>,
    pub comma: bool,
    pub member: CoordinatedClauseMember,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoordinatedClauseMember {
    Independent(Box<IndependentClause>),
    SharedPredicate(Predicate),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Subordinator {
    When,
    If,
    As,
    While,
    Unless,
    AsLongAs,
    ForAsLongAs,
    Until,
    Because,
    RatherThan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PredicateConjunction {
    And,
    Or,
    Then,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExistentialClause {
    pub form: ExistentialForm,
    pub pivot: NounPhrase,
    pub adjuncts: Vec<PredicateAdjunct>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExistentialForm {
    Is,
    ContractedIs,
    Are,
    Was,
    Were,
}
