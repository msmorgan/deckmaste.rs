use super::ability::Ability;
use super::ability::QuotedAbility;
use super::phrase::AdjectivePhrase;
use super::phrase::CoordinatedAdjectivePhrase;
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
    /// A modal clause. The predicate is `None` when the verb phrase is elided
    /// under the modal (VP-ellipsis, e.g. "If you can't, …"); the modal then
    /// renders alone with no synthesized pro-verb.
    Deontic(Subject, Modal, Option<Predicate>),
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
    pub pre_object_elements: Vec<PredicateElement>,
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
    /// The distributive floating quantifier `each` sitting between the copula
    /// and the complement (`Rosie's power and toughness are *each* equal to
    /// …`). It quantifies the coordinated subject but surfaces
    /// post-copularly, so it is a different position from the
    /// [`PartitiveHead::Each`](crate::syntax::PartitiveHead) of `each of
    /// X`; carried here as a flag and replayed by the renderer in its fixed
    /// slot rather than synthesized from the subject's shape.
    pub distributive_each: bool,
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
    /// A coordinated run of predicative adjective phrases (`it's legendary and
    /// snow`, `that's red or green`, `that are green and/or white`). It reuses
    /// the landed coordination idiom over adjective phrases; the copula and
    /// relative-copular positions both consume it through the same
    /// [`CoordinatedAdjectivePhrase`] shape.
    CoordinatedAdjective(CoordinatedAdjectivePhrase),
    Prepositional(PrepositionalPhrase),
    /// A power/toughness statistic predicated of the subject (`it's 7/7`). The
    /// value is the same `N/N` token that heads a stat-setting object, carried
    /// here so a copular clause can assert a permanent's power and toughness.
    PowerToughness(PowerToughness),
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
    /// A coordinated run of predicative adjective phrases in an intransitive
    /// `be` complement (`that are green and white`, `that are green and/or
    /// white`). Shares the [`CoordinatedAdjectivePhrase`] shape with the
    /// copular complement so relative and matrix predications coordinate
    /// identically.
    CoordinatedAdjective(CoordinatedAdjectivePhrase),
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
    ModalSubjectGap {
        modal: Modal,
        predicate: Option<Predicate>,
    },
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
    /// A trailing `except <clause>[, <clause>]…` rider. The conjuncts are the
    /// modifications or exceptions a copy effect applies to the copying process
    /// [CR#707.9]; each is an ordinary finite independent clause parsed by the
    /// existing clause productions.
    Exception(ExceptionRider),
    /// A trailing appositive elaboration introduced by a spaced em dash: the
    /// dash body of a clause whose tail names a choice and then spells its
    /// coordinated options (`… faces a villainous choice — <clause>, or
    /// <clause>`). Licensed purely on shape — a complete clause matrix, a
    /// spaced ` — `, then a top-level `or`-coordinated run of independent
    /// clauses — never on any word in the matrix. The body is always an
    /// [`IndependentClause::Coordinated`] whose members are the options; the
    /// renderer reproduces the ` — ` separator, so the attachment's own `comma`
    /// flag is unused.
    Appositive(Box<IndependentClause>),
}

/// A coordinated list of exception clauses trailing a host clause under a
/// leading `except`. The first conjunct and each continuation is an
/// independent finite clause; the coordination is recorded (comma and optional
/// conjunction per member) so the renderer replays the exact surface list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExceptionRider {
    pub first: Box<IndependentClause>,
    pub rest: Vec<ExceptionConjunct>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExceptionConjunct {
    pub conjunction: Option<PredicateConjunction>,
    pub comma: bool,
    pub clause: IndependentClause,
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
    Before,
    After,
    /// The grammaticalized temporal-frequency connective `the next time`, which
    /// fronts a one-shot replacement/prevention window (`The next time a source
    /// … would deal damage to you this turn, prevent that damage`). Like the
    /// other multi-word connectives (`as long as`, `rather than`) it is carried
    /// as a single subordinator lexeme and replayed verbatim by the renderer.
    TheNextTime,
    /// The variable-definition connective `where`, which trails a clause with a
    /// finite copular body binding a variable to a value (`…, where X is the
    /// number of creatures you control`). Unlike the adverbial subordinators it
    /// does not gate its matrix; it defines the value the matrix's `X` denotes.
    /// It is carried as a distinct subordinator so the binding is recorded in
    /// the AST rather than inferred from the surface word.
    Where,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PredicateConjunction {
    And,
    Or,
    Then,
    /// The disjunctive-or-conjunctive connective spelled `and/or`, lexed as a
    /// single word token. It joins coordinated nominal modifiers (`white and/or
    /// blue`, Amphibious Kavu); it is never a valid clause, predicate-object,
    /// or noun-phrase connective, so every coordination outside the
    /// modifier list rejects it.
    AndOr,
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
