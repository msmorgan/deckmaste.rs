use std::ops::Deref;
use std::ops::DerefMut;

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
    /// A finite clause whose subject scopes over one predicate expression.
    /// Predicate coordination lives inside that expression, so the subject is
    /// a sibling of the whole coordinated phrase rather than being buried in
    /// its first conjunct.
    Predicated(Option<Subject>, PredicateExpression),
    Imperative(Predicate),
    /// A modal clause. This leaf representation is retained for an
    /// uncoordinated clause; when it participates in predicate coordination,
    /// the modal is promoted to [`Predicate::Deontic`] like every other
    /// conjunct.
    Deontic(Subject, Modal, Option<Predicate>),
    Existential(ExistentialClause),
    Proform(Subject, ProPredicate),
    Complex(ComplexClause),
    /// Coordination of complete clauses, each with its own subject. This is
    /// distinct from [`PredicateExpression::Coordinated`], where one subject
    /// scopes over every predicate conjunct.
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
    Deontic(DeonticPredicate),
    /// A predicate plus dependents whose scope ends before the next coordinated
    /// predicate (`P1 unless C or P2`, `P1, where C, then P2`).
    Attached(AttachedPredicate),
}

/// The predicate constituent of a finite clause.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PredicateExpression {
    Simple(Predicate),
    Coordinated(Coordination<Predicate>),
}

/// A modal predicate. The inner predicate is absent under VP-ellipsis (`If
/// you can't, …`); modality remains inside the predicate layer in either
/// case.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeonticPredicate {
    pub modal: Modal,
    pub inner: Option<Box<Predicate>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttachedPredicate {
    pub predicate: Box<Predicate>,
    pub attachments: Vec<ClauseAttachment>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PredicateHead {
    pub auxiliaries: Vec<AuxiliaryInstance>,
    pub first_auxiliary_contracted_with_subject: bool,
    pub preverb_modifiers: Vec<PreverbModifier>,
    pub verb: VerbInstance,
    /// The finite verbal quantifier float (`Two target creatures each get
    /// ...`): renders as literal `each` prepended before every auxiliary,
    /// preverb modifier, and the lexical verb. Never true together with
    /// `first_auxiliary_contracted_with_subject`: the grammar that sets this
    /// flag never also contracts a subject auxiliary, since `each`
    /// intervenes between the subject and the verb phrase.
    pub distributive_each: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreverbModifier {
    Not,
    Also,
    /// The literal word `next` in its preverbal-adverb reading (`when you
    /// *next* cast an instant or sorcery spell this turn`).
    Next,
}

/// Shared shell for predicates with a lexical [`PredicateHead`]. `K` carries
/// only the complement structure that distinguishes predicate kinds; the head
/// and trailing elements have one representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeadedPredicate<K> {
    pub head: PredicateHead,
    pub kind: K,
    pub elements: Vec<PredicateElement>,
}

impl<K> Deref for HeadedPredicate<K> {
    type Target = K;

    fn deref(&self) -> &Self::Target {
        &self.kind
    }
}

impl<K> DerefMut for HeadedPredicate<K> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.kind
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transitive {
    pub pre_object_elements: Vec<PredicateElement>,
    pub object: PredicateObject,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Intransitive;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Passive {
    /// The theme retained post-verbally under recipient passivization: the
    /// `damage` of `an opponent was dealt damage this turn`. `None` for every
    /// ordinary passive, where the promoted subject IS the theme. Licensed
    /// only by a frame whose `is_recipient_passive()` holds, so the field can
    /// never be populated by a verb that does not lexically take a recipient.
    pub retained_object: Option<PredicateObject>,
}

pub type TransitivePredicate = HeadedPredicate<Transitive>;
pub type IntransitivePredicate = HeadedPredicate<Intransitive>;
pub type PassivePredicate = HeadedPredicate<Passive>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CopularPredicate {
    pub copula: Copula,
    /// Whether the predication is negated by a free-standing `not`
    /// (`it's not your turn`). Negation is normally spelled on the copula
    /// itself (`isn't` — `AuxiliaryInstance::contracted_negation`), but when
    /// the subject and auxiliary contract there is no auxiliary token left
    /// to carry it, so English spells it separately. It is one fact about
    /// the predication, not an adverb: `precomplement_adverbs` holds `Vocab`
    /// adverbs (`still`), and `not` has its own structural home elsewhere
    /// (`PreverbModifier::Not`).
    pub negated: bool,
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
pub struct ObjectGap;

pub type ObjectGapPredicate = HeadedPredicate<ObjectGap>;

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
    /// `None` on the asyndetic comma-separated interior members of an Oxford
    /// list; `Some` on a bare `and`/`or` member and on the final Oxford
    /// member. Mirrors
    /// [`NounPhraseCoordination`](super::phrase::NounPhraseCoordination).
    pub conjunction: Option<PredicateConjunction>,
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
    /// The result tail of the closed `come up heads`/`come up tails`
    /// coin-result predicate [CR#705.1,705.2]. Only the narrow `Come` frame
    /// (see `PredicateFrame::requires_coin_result`) ever attaches this
    /// element; the renderer maps each typed value back to its exact two-word
    /// surface without inspecting any stored string.
    CoinResult(CoinSide),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VerbParticle {
    In,
    Out,
}

/// The designated side of a coin, carried only inside the closed
/// `come up heads`/`come up tails` result predicate [CR#705.1,705.2]. Not a
/// general noun or adjective reading of `heads`/`tails` — see
/// `PredicateElement::CoinResult`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CoinSide {
    Heads,
    Tails,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PredicateAdjunct {
    Adverb(Vocab),
    Frequency(FrequencyPhrase),
    Temporal(NounPhrase),
    Manner(NounPhrase),
    Prepositional(PrepositionalPhrase),
    /// A closed exception tail on a passive restriction (`can't be blocked
    /// except by creatures with flying`): distinct from an ordinary
    /// prepositional adjunct (the exception carveout changes the reading,
    /// not merely the agent) and from the clausal `ExceptionRider`, which
    /// requires a comma and a full finite clause complement. The parser
    /// constructs this variant only for a `By` preposition; the renderer
    /// does not inspect the stored preposition, it renders literal
    /// `except ` followed by the ordinary PP rendering, so the admitted
    /// word class stays render/parse inverse.
    /// [CR#508.1c,509.1b,702.9b,702.13b,702.36b,702.111b]
    Exception(PrepositionalPhrase),
    Dependent(Box<DependentClause>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FrequencyPhrase {
    pub bound: FrequencyBound,
    pub count: FrequencyCount,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FrequencyBound {
    MoreThan,
    NoMoreThan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
pub struct Attachment<T> {
    pub position: AttachmentPosition,
    pub comma: bool,
    pub payload: T,
}

pub type ClauseAttachment = Attachment<ClauseAttachmentKind>;

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
    /// A trailing run of two or more coordinated `only …` timing restrictions
    /// (`only as a sorcery and only once each turn`). Each member repeats
    /// `only`, which the shape carries rather than storing an adverb per
    /// member; the members are independent gates on the action
    /// [CR#601.3,602.5]. A one-member run never derives — a lone `only X`
    /// keeps its existing split residence (the `only` adverb in the matrix
    /// predicate's `elements`, an `if`-clause as its own `Dependent`
    /// attachment). The attachment's own `comma` flag is always `false`; every
    /// separator inside the run is carried by [`RestrictionCoordination`].
    Restriction(RestrictionRun),
}

/// A coordinated run of `only …` restriction members trailing a host clause.
/// Mirrors [`ExceptionRider`]'s topology.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestrictionRun {
    /// A member's payload: one adjunct for a `Prepositional`/`Dependent`
    /// member (`only as a sorcery`, `only if …`), **two** for the flat
    /// `Adverb + Temporal` pair the existing grammar already uses for `once
    /// each turn` (`Activate only once each turn.` lowers to two sibling
    /// elements today — verified by probe, not assumed — so a coordinated
    /// `only once each turn` member must carry both to round-trip). Never
    /// empty.
    pub first: Vec<PredicateAdjunct>,
    /// Never empty: the run exists only when a coordinator joins two or more
    /// members. Mirrors [`ExceptionRider::rest`].
    pub rest: Vec<RestrictionCoordination>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestrictionCoordination {
    /// `None` on the asyndetic comma-separated interior members of an Oxford
    /// list; `Some(PredicateConjunction::And)` on a bare `and` member and on
    /// the final Oxford member. Mirrors [`ExceptionConjunct`].
    pub conjunction: Option<PredicateConjunction>,
    pub comma: bool,
    /// See [`RestrictionRun::first`] for why this is a (non-empty) list.
    pub adjuncts: Vec<PredicateAdjunct>,
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

pub type DependentAttachment = Attachment<DependentClause>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttachmentPosition {
    BeforeMatrix,
    AfterMatrix,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoordinationJunction {
    /// `None` records an asyndetic comma junction; coordinated junctions carry
    /// their overt connective.
    pub conjunction: Option<PredicateConjunction>,
    pub comma: bool,
}

/// A validated coordination of two or more uniform conjuncts.
///
/// Conjuncts and the junctions between them are stored separately so no
/// conjunct is structurally privileged as `first`. The private fields keep
/// the invariant `junctions.len() + 1 == conjuncts.len()` intact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Coordination<T> {
    conjuncts: Vec<T>,
    junctions: Vec<CoordinationJunction>,
}

impl<T> Coordination<T> {
    pub fn new(first: T, junction: CoordinationJunction, second: T) -> Self {
        Self {
            conjuncts: vec![first, second],
            junctions: vec![junction],
        }
    }

    #[must_use]
    pub fn conjuncts(&self) -> &[T] {
        &self.conjuncts
    }

    #[must_use]
    pub fn junctions(&self) -> &[CoordinationJunction] {
        &self.junctions
    }

    pub(crate) fn push(&mut self, junction: CoordinationJunction, conjunct: T) {
        self.junctions.push(junction);
        self.conjuncts.push(conjunct);
    }
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

/// A complete-clause coordination continuation. Subjectless continuations are
/// folded into the predicate expression of the preceding clause before this
/// layer is built, so every member here is a complete independent clause.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoordinatedClauseMember {
    Independent(Box<IndependentClause>),
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
    /// The counterfactual connective `as though`, which trails a finite
    /// clause stating the respect in which the matrix event is to be treated
    /// differently (`you may cast this spell as though it had flash`). Like
    /// `as long as` it is carried as a single two-word subordinator lexeme
    /// and replayed verbatim by the renderer.
    AsThough,
}

impl Subordinator {
    // Longest shared prefixes come first so scanning chooses the whole closed
    // lexeme before its one-word prefix.
    pub(crate) const FORMS: &'static [(Self, &'static str)] = &[
        (Self::ForAsLongAs, "for as long as"),
        (Self::TheNextTime, "the next time"),
        (Self::AsLongAs, "as long as"),
        (Self::AsThough, "as though"),
        (Self::RatherThan, "rather than"),
        (Self::When, "when"),
        (Self::If, "if"),
        (Self::As, "as"),
        (Self::While, "while"),
        (Self::Unless, "unless"),
        (Self::Until, "until"),
        (Self::Because, "because"),
        (Self::Before, "before"),
        (Self::After, "after"),
        (Self::Where, "where"),
    ];

    pub(crate) fn spelling(self) -> &'static str {
        Self::FORMS
            .iter()
            .find_map(|(subordinator, spelling)| (*subordinator == self).then_some(*spelling))
            .expect("every subordinator has one spelling")
    }
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

impl PredicateConjunction {
    const FORMS: &'static [(Self, &'static str)] = &[
        (Self::And, "and"),
        (Self::Or, "or"),
        (Self::Then, "then"),
        (Self::AndOr, "and/or"),
    ];

    pub(crate) fn from_spelling(surface: &str) -> Option<Self> {
        Self::FORMS.iter().find_map(|(conjunction, spelling)| {
            surface
                .eq_ignore_ascii_case(spelling)
                .then_some(*conjunction)
        })
    }

    pub(crate) fn spelling(self) -> &'static str {
        Self::FORMS
            .iter()
            .find_map(|(conjunction, spelling)| (*conjunction == self).then_some(*spelling))
            .expect("every predicate conjunction has one spelling")
    }
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

impl ExistentialForm {
    pub(crate) const FORMS: &'static [(Self, &'static str)] = &[
        (Self::Is, "there is"),
        (Self::ContractedIs, "there's"),
        (Self::Are, "there are"),
        (Self::Was, "there was"),
        (Self::Were, "there were"),
    ];

    pub(crate) fn spelling(self) -> &'static str {
        Self::FORMS
            .iter()
            .find_map(|(form, spelling)| (*form == self).then_some(*spelling))
            .expect("every existential form has one spelling")
    }

    pub(crate) const fn number(self) -> crate::word::Number {
        match self {
            Self::Is | Self::ContractedIs | Self::Was => crate::word::Number::Singular,
            Self::Are | Self::Were => crate::word::Number::Plural,
        }
    }
}
