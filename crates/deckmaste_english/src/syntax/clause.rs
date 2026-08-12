use std::ops::Deref;

use super::ability::Ability;
use super::ability::QuotedAbility;
use super::phrase::AdjectivePhrase;
use super::phrase::CoordinatedAdjectivePhrase;
use super::phrase::CountedEnergy;
use super::phrase::NounPhrase;
use super::phrase::NumberLiteral;
use super::phrase::OracleSymbol;
use super::phrase::PowerToughness;
use super::phrase::PrepositionalPhrase;
use super::phrase::Quantity;
use crate::catalog::CatalogAtom;
pub use crate::constructions::relative::RelativeClause;
use crate::features::Comma;
use crate::features::Conjunction;
use crate::features::Contraction;
use crate::word::AuxiliaryInstance;
use crate::word::PredicateFrame;
use crate::word::VerbInstance;
use crate::word::Vocab;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub enum Clause {
    Independent(IndependentClause),
    Dependent(DependentClause),
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub enum IndependentClause {
    /// A subject (explicit for ordinary finite clauses, implicit for
    /// imperatives) governing one recursively compositional predicate
    /// expression.
    Finite(FiniteClause),
    Existential(ExistentialClause),
    Complex(ComplexClause),
    /// Coordination of complete clauses, each with its own subject. This is
    /// distinct from [`PredicateExpression::Coordinated`], where one subject
    /// scopes over every predicate conjunct.
    Coordinated(CoordinatedIndependentClause),
}

impl IndependentClause {
    /// Returns the explicit subject of an ordinary finite clause.
    #[must_use]
    pub fn subject(&self) -> Option<&Subject> {
        match self {
            Self::Finite(finite) => finite.subject(),
            _ => None,
        }
    }

    /// Returns the predicate expression owned by an ordinary finite clause.
    #[must_use]
    pub fn predicate_expression(&self) -> Option<&PredicateExpression> {
        match self {
            Self::Finite(finite) => Some(finite.predicate()),
            _ => None,
        }
    }

    /// Returns an uncoordinated predicate when this is an ordinary finite
    /// clause with a single predicate expression.
    #[must_use]
    pub fn simple_predicate(&self) -> Option<&Predicate> {
        let PredicateExpression::Simple(predicate) = self.predicate_expression()? else {
            return None;
        };
        Some(predicate)
    }
}

/// The canonical semantic owner for an ordinary independent clause.
///
/// A missing subject represents the implicit addressee of an imperative. The
/// predicate expression owns every shared-subject continuation, including
/// modal scope, so leaf predicate classes never duplicate subject ownership.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct FiniteClause {
    pub(crate) subject: Option<Subject>,
    pub(crate) predicate: PredicateExpression,
}

impl FiniteClause {
    pub(crate) const fn from_declaration_parts(
        subject: Option<Subject>,
        predicate: PredicateExpression,
    ) -> Self {
        Self { subject, predicate }
    }

    #[must_use]
    pub const fn subject(&self) -> Option<&Subject> {
        self.subject.as_ref()
    }

    #[must_use]
    pub const fn predicate(&self) -> &PredicateExpression {
        &self.predicate
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub enum DependentClause {
    Subordinate(Subordinator, SubordinateBody),
    Relative(RelativeClause),
    Infinitive(InfinitiveClause),
    Gerund(GerundClause),
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub enum SubordinateBody {
    Finite(Box<IndependentClause>),
    CoordinatedFinite(CoordinatedSubordinateBody),
    Infinitive(InfinitiveClause),
    Gerund(GerundClause),
    Elliptical(EllipticalClause),
}

/// Two finite conditions coordinated inside one subordinate attachment while
/// repeating the subordinator before the second condition (`if A or if B`).
/// The outer [`DependentClause::Subordinate`] owns the first marker; this node
/// retains only the repeated marker whose spelling would otherwise be lost.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct CoordinatedSubordinateBody {
    pub(crate) first: Box<IndependentClause>,
    pub(crate) conjunction: Conjunction,
    pub(crate) repeated_subordinator: Subordinator,
    pub(crate) next: Box<IndependentClause>,
}

impl CoordinatedSubordinateBody {
    pub(crate) const fn from_declaration_parts(
        first: Box<IndependentClause>,
        conjunction: Conjunction,
        repeated_subordinator: Subordinator,
        next: Box<IndependentClause>,
    ) -> Self {
        Self {
            first,
            conjunction,
            repeated_subordinator,
            next,
        }
    }

    #[must_use]
    pub const fn first(&self) -> &IndependentClause {
        &self.first
    }

    #[must_use]
    pub const fn conjunction(&self) -> Conjunction {
        self.conjunction
    }

    #[must_use]
    pub const fn repeated_subordinator(&self) -> Subordinator {
        self.repeated_subordinator
    }

    #[must_use]
    pub const fn next(&self) -> &IndependentClause {
        &self.next
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub enum EllipticalClause {
    Adjective(AdjectivePhrase),
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct Subject(pub NounPhrase);

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub enum PredicateExpression {
    Simple(Predicate),
    /// A recursively grouped predicate coordination. Recursive conjuncts keep
    /// mixed connectives explicit: `(attack or block) and has ...` is one
    /// `and` coordination whose first conjunct is the nested `or` group.
    Coordinated(Coordination<PredicateExpression>),
}

/// A modal predicate. The inner predicate is absent under VP-ellipsis (`If
/// you can't, …`); modality remains inside the predicate layer in either
/// case.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct DeonticPredicate {
    pub(crate) modal: Modal,
    pub(crate) inner: Option<Box<PredicateExpression>>,
}

impl DeonticPredicate {
    /// Returns the modal auxiliary governing this predicate.
    #[must_use]
    pub const fn modal(&self) -> &Modal {
        &self.modal
    }

    /// Returns the governed predicate, or `None` for verbal ellipsis.
    #[must_use]
    pub fn inner(&self) -> Option<&PredicateExpression> {
        self.inner.as_deref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct AttachedPredicate {
    pub(crate) scope: AttachmentScope<Predicate>,
}

impl AttachedPredicate {
    /// Returns the predicate whose local scope owns the trailing attachments.
    #[must_use]
    pub fn predicate(&self) -> &Predicate {
        self.scope.host()
    }

    /// Returns the single outer attachment owned by this predicate member.
    #[must_use]
    pub fn attachment(&self) -> &ClauseAttachment {
        self.scope.attachment()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct PredicateHead {
    pub(crate) auxiliaries: Vec<AuxiliaryInstance>,
    /// **Measured, field KEPT** (surface-fact diet, 2026-07-30 measurement
    /// round): the ticket's construction-kind hypothesis — non-passive
    /// positions contract, passive `it is` does not — is FALSIFIED. Coding
    /// `!is_passive && !auxiliaries.is_empty()` as the derivation produced 161
    /// mismatches and 731 render errors. Witness: Adanto Vanguard, "As long
    /// as this creature **is** attacking, it gets +2/+0." is a non-passive
    /// predicate with an auxiliary present, so the rule forces a contraction
    /// attempt, but the corpus never contracts a full noun-phrase subject
    /// ("this creature's attacking" does not occur). The render errors are
    /// a second, independent symptom: `contraction_suffix` only covers
    /// `Auxiliary::Be`/`Have` (subject contraction), not `Auxiliary::Do`
    /// (negation, "doesn't"/"didn't"), so every negated do-support predicate
    /// that the rule now tries to contract blows up too. The data instead
    /// points at **subject pronominality** — only pronoun subjects contract,
    /// full noun phrases never do — as the better hypothesis, though that
    /// alone cannot be exact either: the corpus has both `it's` (1132
    /// occurrences) and `it is` (73). Field stays stored.
    ///
    /// Separately, negation contraction needs no field and none was added:
    /// across all 31685 supported faces there are zero occurrences of
    /// `cannot`/`does not`/`do not`/`is not`/`did not`/`are not`, against
    /// `can't` 3131 / `don't` 728 / `doesn't` 450 / `isn't` 246 / `didn't`
    /// 128 / `aren't` 54 — the corpus has no variation to store.
    pub(crate) first_auxiliary_contracted_with_subject: Contraction,
    pub(crate) preverb_modifiers: Vec<PreverbModifier>,
    pub(crate) verb: VerbInstance,
    /// Lexical selection belongs to the sealed generated predicate owner. It
    /// is retained for checked inverse rendering but is not a surface fact.
    #[serde(skip)]
    pub(crate) frame: PredicateFrame,
    /// The finite verbal quantifier float (`Two target creatures each get
    /// ...`): renders as literal `each` prepended before every auxiliary,
    /// preverb modifier, and the lexical verb. Never true together with
    /// `first_auxiliary_contracted_with_subject`: the grammar that sets this
    /// flag never also contracts a subject auxiliary, since `each`
    /// intervenes between the subject and the verb phrase.
    pub(crate) distributive_each: bool,
}

impl PredicateHead {
    #[must_use]
    pub fn auxiliaries(&self) -> &[AuxiliaryInstance] {
        &self.auxiliaries
    }

    #[must_use]
    pub const fn first_auxiliary_contracted_with_subject(&self) -> Contraction {
        self.first_auxiliary_contracted_with_subject
    }

    #[must_use]
    pub fn preverb_modifiers(&self) -> &[PreverbModifier] {
        &self.preverb_modifiers
    }

    #[must_use]
    pub const fn verb(&self) -> &VerbInstance {
        &self.verb
    }

    #[must_use]
    pub const fn distributive_each(&self) -> bool {
        self.distributive_each
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct HeadedPredicate<K> {
    pub(crate) head: PredicateHead,
    pub(crate) kind: K,
    pub(crate) elements: Vec<PredicateElement>,
}

impl<K> HeadedPredicate<K> {
    #[must_use]
    pub const fn head(&self) -> &PredicateHead {
        &self.head
    }

    #[must_use]
    pub const fn kind(&self) -> &K {
        &self.kind
    }

    #[must_use]
    pub fn elements(&self) -> &[PredicateElement] {
        &self.elements
    }
}

impl<K> Deref for HeadedPredicate<K> {
    type Target = K;

    fn deref(&self) -> &Self::Target {
        &self.kind
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct Transitive {
    pub(crate) pre_object_elements: Vec<PredicateElement>,
    pub(crate) object: PredicateObject,
}

impl Transitive {
    /// Returns dependents that precede the direct object in surface order.
    #[must_use]
    pub fn pre_object_elements(&self) -> &[PredicateElement] {
        &self.pre_object_elements
    }

    /// Returns the checked direct object.
    #[must_use]
    pub const fn object(&self) -> &PredicateObject {
        &self.object
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct Intransitive;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct Passive {
    /// The theme retained post-verbally under recipient passivization: the
    /// `damage` of `an opponent was dealt damage this turn`. `None` for every
    /// ordinary passive, where the promoted subject IS the theme. Licensed
    /// only by a frame whose `is_recipient_passive()` holds, so the field can
    /// never be populated by a verb that does not lexically take a recipient.
    pub(crate) retained_object: Option<PredicateObject>,
}

impl Passive {
    /// Returns the postverbal theme retained by a recipient passive.
    #[must_use]
    pub const fn retained_object(&self) -> Option<&PredicateObject> {
        self.retained_object.as_ref()
    }
}

pub type TransitivePredicate = HeadedPredicate<Transitive>;
pub type IntransitivePredicate = HeadedPredicate<Intransitive>;
pub type PassivePredicate = HeadedPredicate<Passive>;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct CopularPredicate {
    pub(crate) copula: Copula,
    /// Whether the predication is negated by a free-standing `not`
    /// (`it's not your turn`). Negation is normally spelled on the copula
    /// itself (`isn't` — `AuxiliaryInstance::contracted_negation`), but when
    /// the subject and auxiliary contract there is no auxiliary token left
    /// to carry it, so English spells it separately. It is one fact about
    /// the predication, not an adverb: `precomplement_adverbs` holds `Vocab`
    /// adverbs (`still`), and `not` has its own structural home elsewhere
    /// (`PreverbModifier::Not`).
    pub(crate) negated: bool,
    /// The distributive floating quantifier `each` sitting between the copula
    /// and the complement (`Rosie's power and toughness are *each* equal to
    /// …`). It quantifies the coordinated subject but surfaces
    /// post-copularly, so it is a different position from the
    /// [`PartitiveHead::Each`](crate::syntax::PartitiveHead) of `each of
    /// X`; carried here as a flag and replayed by the renderer in its fixed
    /// slot rather than synthesized from the subject's shape.
    pub(crate) distributive_each: bool,
    pub(crate) precomplement_adverbs: Vec<Vocab>,
    pub(crate) complement: CopularComplement,
    pub(crate) adjuncts: Vec<PredicateAdjunct>,
}

impl CopularPredicate {
    /// Returns the checked copular head.
    #[must_use]
    pub const fn copula(&self) -> &Copula {
        &self.copula
    }

    /// Whether negation is realized as a free-standing `not`.
    #[must_use]
    pub const fn negated(&self) -> bool {
        self.negated
    }

    /// Whether the post-copular distributive `each` is present.
    #[must_use]
    pub const fn distributive_each(&self) -> bool {
        self.distributive_each
    }

    /// Returns adverbs between the copula and complement.
    #[must_use]
    pub fn precomplement_adverbs(&self) -> &[Vocab] {
        &self.precomplement_adverbs
    }

    /// Returns the selected copular complement.
    #[must_use]
    pub const fn complement(&self) -> &CopularComplement {
        &self.complement
    }

    /// Returns trailing copular adjuncts in surface order.
    #[must_use]
    pub fn adjuncts(&self) -> &[PredicateAdjunct] {
        &self.adjuncts
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct Copula {
    pub(crate) auxiliary: AuxiliaryInstance,
    /// **Measured, field KEPT** (surface-fact diet, 2026-07-30 measurement
    /// round): the ticket's construction-kind hypothesis — copular positions
    /// always contract — is FALSIFIED, more heavily than the sibling
    /// `first_auxiliary_contracted_with_subject` field. Coding unconditional
    /// contraction produced 1617 mismatches and 162 render errors. Witnesses:
    /// Abominable Treefolk, "Abominable Treefolk's power and toughness **are**
    /// each equal to the number of snow permanents you control." forces
    /// "...**'re** each equal to..." for a full noun-phrase subject; Abzan
    /// Monument, "**X is** the greatest toughness among creatures you
    /// control." forces "X's the greatest..." even for a single-letter
    /// variable subject. Render errors come from past/subjunctive copulas
    /// ("as though it **were** mana of any color" — Abstruse Appropriation):
    /// `contraction_suffix` has no arm for `Past`, so forcing contraction
    /// there hits its `_ => Err(...)` case. Same read as the sibling field:
    /// **subject pronominality**, not copular position, predicts contraction
    /// — and even that is not exact (`it's` 1132 vs `it is` 73 on the
    /// supported corpus). Field stays stored.
    pub(crate) contracted_with_subject: Contraction,
}

impl Copula {
    /// Returns the inflected copular auxiliary.
    #[must_use]
    pub const fn auxiliary(&self) -> AuxiliaryInstance {
        self.auxiliary
    }

    /// Whether the copula contracts with its subject.
    #[must_use]
    pub const fn contracted_with_subject(&self) -> Contraction {
        self.contracted_with_subject
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct Modal {
    pub(crate) auxiliary: AuxiliaryInstance,
}

impl Modal {
    /// Returns the checked modal auxiliary.
    #[must_use]
    pub const fn auxiliary(&self) -> AuxiliaryInstance {
        self.auxiliary
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct ProPredicate {
    pub(crate) auxiliary: AuxiliaryInstance,
}

impl ProPredicate {
    /// Returns the checked proform auxiliary.
    #[must_use]
    pub const fn auxiliary(&self) -> AuxiliaryInstance {
        self.auxiliary
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ObjectGap;

pub type ObjectGapPredicate = HeadedPredicate<ObjectGap>;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub enum PredicateObject {
    NounPhrase(NounPhrase),
    Ability(AbilityObject),
    Quantity(Quantity),
    CountedEnergy(CountedEnergy),
    OracleSymbol(OracleSymbol),
    SymbolSequence(Vec<OracleSymbol>),
    PowerToughness(PowerToughness),
    EmbeddedAbility(Box<Ability>),
    QuotedAbility(Box<QuotedAbility>),
    Coordinated(CoordinatedPredicateObject),
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct CoordinatedPredicateObject {
    pub(crate) first: Box<PredicateObject>,
    pub(crate) rest: Vec<PredicateObjectCoordination>,
}

impl CoordinatedPredicateObject {
    /// Returns the first coordinated object.
    #[must_use]
    pub fn first(&self) -> &PredicateObject {
        self.first.as_ref()
    }

    /// Returns the remaining coordinated objects in surface order.
    #[must_use]
    pub fn rest(&self) -> &[PredicateObjectCoordination] {
        &self.rest
    }
}

/// One non-first member of a predicate-object coordination.
///
/// No `comma` flag: on the supported corpus the serial comma is exactly
/// determined by member count and connective — absent only on an asyndetic
/// interior member (which always takes one) or on a connective member of a
/// two-member list (which never does), present on every connective member of
/// a three-or-more-member (Oxford) list — measured with 0 exceptions across
/// all 31685 supported faces. The renderer derives it from
/// [`CoordinatedPredicateObject::rest`] rather than the AST carrying a field
/// that could contradict it, following the same idiom as
/// [`PrepositionalPhraseCoordination`](super::phrase::PrepositionalPhraseCoordination)
/// and [`TriggerConditionCoordination`](crate::syntax::TriggerConditionCoordination).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct PredicateObjectCoordination {
    /// `None` on the asyndetic comma-separated interior members of an Oxford
    /// list; `Some` on a bare `and`/`or` member and on the final Oxford
    /// member. Mirrors
    /// [`NounPhraseCoordination`](super::phrase::NounPhraseCoordination).
    pub(crate) conjunction: Option<Conjunction>,
    pub(crate) object: PredicateObject,
}

impl PredicateObjectCoordination {
    /// Returns the connective introducing this member, when present.
    #[must_use]
    pub const fn conjunction(&self) -> Option<Conjunction> {
        self.conjunction
    }

    /// Returns the coordinated object.
    #[must_use]
    pub const fn object(&self) -> &PredicateObject {
        &self.object
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct AbilityObject {
    pub(crate) ability: CatalogAtom,
    pub(crate) argument: Option<Box<PredicateObject>>,
}

impl AbilityObject {
    /// Returns the keyword-ability identity.
    #[must_use]
    pub const fn ability(&self) -> &CatalogAtom {
        &self.ability
    }

    /// Returns the checked ability argument, when present.
    #[must_use]
    pub fn argument(&self) -> Option<&PredicateObject> {
        self.argument.as_deref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum VerbParticle {
    Down,
    In,
    Out,
    Up,
}

/// The designated side of a coin, carried only inside the closed
/// `come up heads`/`come up tails` result predicate [CR#705.1,705.2]. Not a
/// general noun or adjective reading of `heads`/`tails` — see
/// `PredicateElement::CoinResult`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum CoinSide {
    Heads,
    Tails,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
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
    /// The ability-owner postmodifier `with "<ability>"`.
    ///
    /// This is not an ordinary prepositional phrase: the quoted ability is a
    /// heterogeneous phrase-boundary payload, not a prepositional object.
    AbilityPostmodifier(AbilityPostmodifier),
    Dependent(Box<DependentClause>),
}

/// A quoted ability attached after a complete predicate by literal `with`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct AbilityPostmodifier {
    ability: Box<QuotedAbility>,
}

impl AbilityPostmodifier {
    pub(crate) fn from_quoted_ability(ability: QuotedAbility) -> Self {
        Self {
            ability: Box::new(ability),
        }
    }

    /// The quoted ability carried by this postmodifier.
    #[must_use]
    pub fn ability(&self) -> &QuotedAbility {
        self.ability.as_ref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub struct FrequencyPhrase {
    pub(crate) bound: FrequencyBound,
    pub(crate) count: FrequencyCount,
}

impl FrequencyPhrase {
    /// Returns the frequency bound selected by the construction.
    #[must_use]
    pub const fn bound(&self) -> FrequencyBound {
        self.bound
    }

    /// Returns the number of admitted occurrences.
    #[must_use]
    pub const fn count(&self) -> FrequencyCount {
        self.count
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum FrequencyBound {
    MoreThan,
    NoMoreThan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum FrequencyCount {
    Once,
    Twice,
    Times(NumberLiteral),
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct InfinitiveClause {
    negated: bool,
    marker: InfinitiveMarker,
    predicate: Box<Predicate>,
}

/// A checked gerund clause with no generic attachment escape hatch.
///
/// A gerund clause admits either one complete present-participle predicate or
/// the recursive `matrix rather than alternative` relation. Position,
/// punctuation, and subordinator are fixed by that relation and therefore are
/// not stored.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct GerundClause(GerundClauseKind);

/// The direct semantic alternatives of a checked [`GerundClause`].
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub enum GerundClauseKind {
    Base {
        predicate: Box<Predicate>,
    },
    RatherThan {
        matrix: Box<GerundClause>,
        alternative: Box<GerundClause>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum InfinitiveMarker {
    Bare,
    To,
}

impl InfinitiveClause {
    pub(crate) fn from_declaration_parts(
        negated: bool,
        marker: InfinitiveMarker,
        predicate: Predicate,
    ) -> Self {
        Self {
            negated,
            marker,
            predicate: Box::new(predicate),
        }
    }

    pub(crate) fn declaration_bare(predicate: Predicate) -> Self {
        Self::from_declaration_parts(false, InfinitiveMarker::Bare, predicate)
    }

    #[must_use]
    pub const fn negated(&self) -> bool {
        self.negated
    }

    #[must_use]
    pub const fn marker(&self) -> InfinitiveMarker {
        self.marker
    }

    #[must_use]
    pub fn predicate(&self) -> &Predicate {
        &self.predicate
    }
}

impl GerundClause {
    pub(crate) fn from_base_declaration(predicate: Predicate) -> Self {
        Self(GerundClauseKind::Base {
            predicate: Box::new(predicate),
        })
    }

    pub(crate) fn from_rather_than_declaration(
        matrix: GerundClause,
        alternative: GerundClause,
    ) -> Self {
        Self(GerundClauseKind::RatherThan {
            matrix: Box::new(matrix),
            alternative: Box::new(alternative),
        })
    }

    /// Returns the validated base or recursive `rather than` relation.
    #[must_use]
    pub const fn kind(&self) -> &GerundClauseKind {
        &self.0
    }
}

/// The `that`/`who`/zero choice heading a subject-gap relative clause
/// (object-gap relatives are always [`Self::Zero`] — see the
/// `RelativeObject`/`RelativeObjectContractedSubject` lowering arms — and a
/// contracted-subject-auxiliary relative is hardcoded to [`Self::That`]).
///
/// **Measured, field KEPT** (surface-fact sweep-residue, 2026-07-30
/// measurement round). Counted on the supported corpus: 10,352 `Zero`, 1,209
/// `That`, 143 `Who`. The raised animacy hypothesis holds almost exactly:
/// every `Who` witness but one has a `player`/`opponent`/`players`/
/// `opponents` antecedent (90 + 67 + 3 + 11), and all but one `player`-class
/// antecedent among the `That` witnesses instead takes `who` (e.g. Braids,
/// Arisen Nightmare's "each opponent **who** doesn't", Admiral Beckett
/// Brass's "a player **who** was dealt combat damage"). The sole exception:
/// Tymna the Weaver's "the number of opponents **that** were dealt combat
/// damage this turn" — a near-word-for-word match to Admiral Beckett Brass's
/// `who` phrasing, differing only in the marker. Close but not exact; field
/// stays stored.
///
/// `Which` was a fourth variant with **zero construction sites** — the
/// lexicon only ever scans `who`/`that` into this slot (see
/// `EnglishLexicalSlot::RelativeMarker`'s surface list and its
/// `zip([RelativeMarker::Who, RelativeMarker::That])` in `grammar/mod.rs`) —
/// so it has been removed; the renderer's match no longer needs an arm for
/// it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum RelativeMarker {
    That,
    Who,
    Zero,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub enum RelativeBody {
    SubjectGap(Predicate),
    ObjectGap {
        subject: Subject,
        predicate: ObjectGapPredicate,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ComplexClause {
    pub(crate) scope: AttachmentScope<IndependentClause>,
}

impl ComplexClause {
    pub(crate) fn from_declaration_parts(
        host: IndependentClause,
        attachment: ClauseAttachment,
    ) -> Self {
        Self {
            scope: AttachmentScope::from_declaration_parts(host, attachment),
        }
    }

    #[must_use]
    pub fn host(&self) -> &IndependentClause {
        self.scope.host()
    }

    #[must_use]
    pub fn attachment(&self) -> &ClauseAttachment {
        self.scope.attachment()
    }
}

/// One explicitly scoped attachment edge around a semantic host.
///
/// Nesting these edges records attachment association directly. No flat list
/// or construction-history rule is needed to recover which attachment was
/// applied outermost.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct AttachmentScope<H> {
    pub(crate) host: Box<H>,
    pub(crate) attachment: Box<ClauseAttachment>,
}

impl<H> AttachmentScope<H> {
    pub(crate) fn from_declaration_parts(host: H, attachment: ClauseAttachment) -> Self {
        Self {
            host: Box::new(host),
            attachment: Box::new(attachment),
        }
    }

    #[must_use]
    pub fn host(&self) -> &H {
        self.host.as_ref()
    }

    #[must_use]
    pub fn attachment(&self) -> &ClauseAttachment {
        self.attachment.as_ref()
    }
}

/// **Measured, `comma` field KEPT** (surface-fact diet, 2026-07-30
/// measurement round): attachments are independent pre-/post-matrix riders on
/// a host, not members of a coordination — there is no `conjunction` field,
/// and recursive [`AttachmentScope`] edges preserve association directly
/// rather than deriving punctuation from a `first`/`rest` member count. Across
/// its construction sites (generated attachment declarations and
/// `grammar/ability.rs`) `comma` takes at least 3 distinct forms:
/// fronted/trailing comma witnesses, no-comma witnesses, and ability-layer
/// attachments. There is no coordination count from which to derive it.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct Attachment<T> {
    pub(crate) position: AttachmentPosition,
    pub(crate) comma: Comma,
    pub(crate) payload: T,
}

impl<T> Attachment<T> {
    pub(crate) const fn from_declaration_parts(
        position: AttachmentPosition,
        comma: Comma,
        payload: T,
    ) -> Self {
        Self {
            position,
            comma,
            payload,
        }
    }

    #[must_use]
    pub const fn position(&self) -> AttachmentPosition {
        self.position
    }

    #[must_use]
    pub const fn comma(&self) -> Comma {
        self.comma
    }

    #[must_use]
    pub const fn payload(&self) -> &T {
        &self.payload
    }
}

pub type ClauseAttachment = Attachment<ClauseAttachmentKind>;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
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
    /// A trailing `only …` action restriction. A run with coordinated members
    /// spells `only X and only Y`; a singleton is licensed only by the
    /// contrastive surface `but only X`. A lone unintroduced `only X` keeps its
    /// existing split residence (the `only` adverb in the matrix predicate's
    /// `elements`, an `if`-clause as its own `Dependent` attachment). Members
    /// are independent gates on the action [CR#601.3,602.5]. The attachment's
    /// own `comma` flag is always `false`; every separator inside a coordinated
    /// run is carried by [`RestrictionCoordination`].
    Restriction(RestrictionRun),
}

/// A coordinated run of `only …` restriction members trailing a host clause.
/// Mirrors [`ExceptionRider`]'s topology.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct RestrictionRun {
    /// A member's payload: one adjunct for a `Prepositional`/`Dependent`
    /// member (`only as a sorcery`, `only if …`), **two** for the flat
    /// `Adverb + Temporal` pair the existing grammar already uses for `once
    /// each turn` (`Activate only once each turn.` lowers to two sibling
    /// elements today — verified by probe, not assumed — so a coordinated
    /// `only once each turn` member must carry both to round-trip). Never
    /// empty.
    pub(crate) first: RestrictionMember,
    /// Empty only for the contrastive singleton surface `but only X`; otherwise
    /// the final member carries `and`. Mirrors [`ExceptionRider::rest`].
    pub(crate) rest: Vec<RestrictionCoordination>,
}

impl RestrictionRun {
    pub(crate) fn from_declaration_parts(
        first: RestrictionMember,
        rest: Vec<RestrictionCoordination>,
    ) -> Self {
        Self { first, rest }
    }

    #[must_use]
    pub const fn first(&self) -> &RestrictionMember {
        &self.first
    }

    #[must_use]
    pub fn rest(&self) -> &[RestrictionCoordination] {
        &self.rest
    }
}

/// One non-first member of an `only …` restriction run.
///
/// No `comma` flag: on the supported corpus the serial comma is exactly
/// determined by member count and connective — absent only on an asyndetic
/// interior member (which always takes one) or on a connective member of a
/// two-member list (which never does), present on every connective member of
/// a three-or-more-member (Oxford) list — measured with 0 exceptions across
/// all 31685 supported faces. The renderer derives it from
/// [`RestrictionRun::rest`] rather than the AST carrying a field that could
/// contradict it, following the same idiom as
/// [`PrepositionalPhraseCoordination`](super::phrase::PrepositionalPhraseCoordination)
/// and [`TriggerConditionCoordination`](crate::syntax::TriggerConditionCoordination).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct RestrictionCoordination {
    /// `None` on the asyndetic comma-separated interior members of an Oxford
    /// list; `Some(PredicateConjunction::And)` on a bare `and` member and on
    /// the final Oxford member. Mirrors [`ExceptionConjunct`].
    pub(crate) conjunction: Option<Conjunction>,
    /// See [`RestrictionRun::first`] for why this is a (non-empty) list.
    pub(crate) member: RestrictionMember,
}

impl RestrictionCoordination {
    pub(crate) const fn from_declaration_parts(
        conjunction: Option<Conjunction>,
        member: RestrictionMember,
    ) -> Self {
        Self {
            conjunction,
            member,
        }
    }

    #[must_use]
    pub const fn conjunction(&self) -> Option<Conjunction> {
        self.conjunction
    }

    #[must_use]
    pub const fn member(&self) -> &RestrictionMember {
        &self.member
    }
}

/// One non-empty `only …` member inside a coordinated restriction run.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct RestrictionMember {
    pub(crate) adjuncts: Vec<PredicateAdjunct>,
}

impl RestrictionMember {
    pub(crate) fn from_declaration_parts(adjuncts: Vec<PredicateAdjunct>) -> Option<Self> {
        (!adjuncts.is_empty()).then_some(Self { adjuncts })
    }

    #[must_use]
    pub fn adjuncts(&self) -> &[PredicateAdjunct] {
        &self.adjuncts
    }
}

impl std::ops::Deref for RestrictionMember {
    type Target = Vec<PredicateAdjunct>;

    fn deref(&self) -> &Self::Target {
        &self.adjuncts
    }
}

/// A coordinated list of exception clauses trailing a host clause under a
/// leading `except`. The first conjunct and each continuation is an
/// independent finite clause; the coordination is recorded (conjunction per
/// member; the comma is derived) so the renderer replays the exact surface
/// list.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ExceptionRider {
    pub(crate) first: Box<IndependentClause>,
    pub(crate) rest: Vec<ExceptionConjunct>,
}

impl ExceptionRider {
    pub(crate) fn from_declaration_parts(
        first: IndependentClause,
        rest: Vec<ExceptionConjunct>,
    ) -> Self {
        Self {
            first: Box::new(first),
            rest,
        }
    }

    #[must_use]
    pub fn first(&self) -> &IndependentClause {
        &self.first
    }

    #[must_use]
    pub fn rest(&self) -> &[ExceptionConjunct] {
        &self.rest
    }
}

/// A comma-open exception rider awaiting a final conjunct.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ExceptionRiderList(pub(crate) ExceptionRider);

impl ExceptionRiderList {
    pub(crate) fn into_rider(self) -> ExceptionRider {
        self.0
    }

    pub(crate) const fn rider(&self) -> &ExceptionRider {
        &self.0
    }
}

/// One non-first member of an exception-rider coordination.
///
/// No `comma` flag: on the supported corpus the serial comma is exactly
/// determined by member count and connective — absent only on an asyndetic
/// interior member (which always takes one) or on a connective member of a
/// two-member list (which never does), present on every connective member of
/// a three-or-more-member (Oxford) list — measured with 0 exceptions across
/// all 31685 supported faces. The renderer derives it from
/// [`ExceptionRider::rest`] rather than the AST carrying a field that could
/// contradict it, following the same idiom as
/// [`PrepositionalPhraseCoordination`](super::phrase::PrepositionalPhraseCoordination)
/// and [`TriggerConditionCoordination`](crate::syntax::TriggerConditionCoordination).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ExceptionConjunct {
    pub(crate) conjunction: Option<Conjunction>,
    pub(crate) clause: IndependentClause,
}

impl ExceptionConjunct {
    pub(crate) const fn from_declaration_parts(
        conjunction: Option<Conjunction>,
        clause: IndependentClause,
    ) -> Self {
        Self {
            conjunction,
            clause,
        }
    }

    #[must_use]
    pub const fn conjunction(&self) -> Option<Conjunction> {
        self.conjunction
    }

    #[must_use]
    pub const fn clause(&self) -> &IndependentClause {
        &self.clause
    }
}

pub type DependentAttachment = Attachment<DependentClause>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum AttachmentPosition {
    BeforeMatrix,
    AfterMatrix,
}

/// **Measured, `comma` field KEPT** (surface-fact diet, 2026-07-30
/// measurement round): the count-based derivation that deleted `comma` from
/// [`NounPhraseCoordination`](super::phrase::NounPhraseCoordination)-shaped
/// siblings (`comma == conjunction.is_none() || rest.len() >= 2`) was tried
/// here too and misses by 1011 of 31685 supported faces. The dominant miss is
/// the sequencing connective `Then`: it takes a comma even in a bare
/// two-member shared-subject coordination ("Draw a card, **then** discard a
/// card." — Academy Elite; "choose target instant or sorcery card in your
/// graveyard, **then** roll a d20." — Aberrant Mind Sorcerer), which the
/// count rule never predicts since it only forces a comma at 3+ members. A
/// `Then`-aware refinement (also force a comma whenever `conjunction ==
/// Some(PredicateConjunction::Then)`) cuts the miss to 8 residual faces —
/// ordinary 2-member `and`/`or` coordinations that still take a comma,
/// apparently because each conjunct is long/clausal rather than a short
/// phrase: Angel of Jubilation, Armed with Proof, Arterial Alchemy, Gogo
/// Mysterious Mime, Neverending Torment, Nightmare Incursion, Turnabout,
/// Yasharn Implacable Earth. Neither rule is exact, so `comma` stays stored.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct CoordinationJunction {
    /// `None` records an asyndetic comma junction; coordinated junctions carry
    /// their overt connective.
    pub(crate) conjunction: Option<Conjunction>,
    pub(crate) comma: Comma,
    pub(crate) head_realization: CoordinationHeadRealization,
}

/// Whether the following conjunct spells its own lexical predicate head.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum CoordinationHeadRealization {
    Overt,
    SharedGrantElided,
}

impl CoordinationJunction {
    pub(crate) const fn from_declaration_parts(
        conjunction: Option<Conjunction>,
        comma: Comma,
    ) -> Self {
        Self {
            conjunction,
            comma,
            head_realization: CoordinationHeadRealization::Overt,
        }
    }

    pub(crate) const fn from_shared_grant_parts(
        conjunction: Option<Conjunction>,
        comma: Comma,
    ) -> Self {
        Self {
            conjunction,
            comma,
            head_realization: CoordinationHeadRealization::SharedGrantElided,
        }
    }

    #[must_use]
    pub const fn conjunction(&self) -> Option<Conjunction> {
        self.conjunction
    }

    #[must_use]
    pub const fn comma(&self) -> Comma {
        self.comma
    }

    #[must_use]
    pub const fn head_realization(&self) -> CoordinationHeadRealization {
        self.head_realization
    }
}

/// A validated coordination of two or more uniform conjuncts.
///
/// Conjuncts and the junctions between them are stored separately so no
/// conjunct is structurally privileged as `first`. The private fields keep
/// the invariant `junctions.len() + 1 == conjuncts.len()` intact.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct Coordination<T> {
    conjuncts: Vec<T>,
    junctions: Vec<CoordinationJunction>,
}

impl<T> Coordination<T> {
    pub(crate) fn new(first: T, junction: CoordinationJunction, second: T) -> Self {
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

    pub(crate) fn from_declaration_parts(
        conjuncts: Vec<T>,
        junctions: Vec<CoordinationJunction>,
    ) -> Option<Self> {
        (conjuncts.len() >= 2 && junctions.len() + 1 == conjuncts.len()).then_some(Self {
            conjuncts,
            junctions,
        })
    }

    pub(crate) fn into_declaration_parts(self) -> (Vec<T>, Vec<CoordinationJunction>) {
        (self.conjuncts, self.junctions)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct CoordinatedIndependentClause {
    pub(crate) first: Box<IndependentClause>,
    pub(crate) rest: Vec<ClauseCoordination>,
}

impl CoordinatedIndependentClause {
    pub(crate) fn from_declaration_parts(
        first: Box<IndependentClause>,
        rest: Vec<ClauseCoordination>,
    ) -> Self {
        Self { first, rest }
    }

    #[must_use]
    pub const fn first(&self) -> &IndependentClause {
        &self.first
    }

    #[must_use]
    pub fn rest(&self) -> &[ClauseCoordination] {
        &self.rest
    }
}

/// **Measured, `comma` field KEPT** (surface-fact diet, 2026-07-30
/// measurement round, re-run after the `block`-vocabulary tree fix): unlike
/// every sibling coordination in this module, full-clause coordination
/// *inverts* the usual pattern instead of merely under-predicting it. The
/// count-based rule that is exact for [`PredicateObjectCoordination`],
/// [`RestrictionCoordination`], [`ExceptionConjunct`],
/// [`PrepositionalPhraseCoordination`](super::phrase::PrepositionalPhraseCoordination)
/// and [`TriggerConditionCoordination`](crate::syntax::TriggerConditionCoordination)
/// — `comma == conjunction.is_none() || rest.len() >= 2` — mismatches 303 of
/// 31685 faces here (a plain `comma == true` constant is worse still, missing
/// 1016: most two-member clause coordinations do *not* take a comma). Adding
/// the `Then`-aware refinement that fixed
/// [`CoordinationJunction`] (also force a comma whenever `conjunction ==
/// Some(PredicateConjunction::Then)`, which is 180/180 exact here too) cuts
/// the miss to 127 residual faces, all two-member `and`/`or` coordinations
/// that keep the comma despite the count rule predicting none.
///
/// The residue is not a further structural pattern: the *same* continuation
/// text, following a first clause of the *same* [`IndependentClause`]
/// variant, takes the comma on one card and not another. Gelid Shackles
/// ("Enchanted creature can't block, **and** its activated abilities can't be
/// activated.", first clause 4 words) takes the comma; Dovin Baan ("target
/// creature gets -3/-0 **and** its activated abilities can't be activated.",
/// 11 words) and Edifice of Authority (10 words) do not — the opposite of
/// what a first-conjunct length/complexity rule would predict, on the
/// identical second conjunct. Likewise "you/they may spend mana as though it
/// were mana of any color…" takes a comma on Share the Spoils, Gale's
/// Redirection, Covetous Urge, Cunning Rhetoric and Mezzio Mugger but not on
/// Daxos of Meletis, Grenzo Havoc Raiser, Hurl Through Hell, Robber of the
/// Rich or The Ruinous Powers, again with no AST-visible distinguishing
/// feature. This is Oracle text's own serial-comma-before-a-coordinating-
/// conjunction styling drifting across printings/authors, not a grammar rule
/// the parser can recover, so the bit stays stored.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ClauseCoordination {
    pub(crate) conjunction: Option<Conjunction>,
    pub(crate) comma: Comma,
    pub(crate) member: CoordinatedClauseMember,
}

impl ClauseCoordination {
    pub(crate) const fn from_declaration_parts(
        conjunction: Option<Conjunction>,
        comma: Comma,
        member: CoordinatedClauseMember,
    ) -> Self {
        Self {
            conjunction,
            comma,
            member,
        }
    }

    #[must_use]
    pub const fn conjunction(&self) -> Option<Conjunction> {
        self.conjunction
    }

    #[must_use]
    pub const fn comma(&self) -> Comma {
        self.comma
    }

    #[must_use]
    pub const fn member(&self) -> &CoordinatedClauseMember {
        &self.member
    }
}

/// A complete-clause coordination continuation. Subjectless continuations are
/// folded into the predicate expression of the preceding clause before this
/// layer is built, so every member here is a complete independent clause.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub enum CoordinatedClauseMember {
    Independent(Box<IndependentClause>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
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

/// Compatibility name for the selection-stratum conjunction feature.
pub use crate::features::Conjunction as PredicateConjunction;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ExistentialClause {
    pub(crate) form: ExistentialForm,
    pub(crate) pivot: NounPhrase,
}

impl ExistentialClause {
    pub(crate) const fn from_declaration_parts(form: ExistentialForm, pivot: NounPhrase) -> Self {
        Self { form, pivot }
    }

    /// Returns the validated existential verb form.
    #[must_use]
    pub const fn form(&self) -> ExistentialForm {
        self.form
    }

    /// Returns the noun phrase whose number agrees with the existential form.
    #[must_use]
    pub const fn pivot(&self) -> &NounPhrase {
        &self.pivot
    }
}

/// **Measured, contracted `is` KEPT** (surface-fact diet, 2026-07-30
/// measurement round): unlike its `PredicateHead`/`Copula` siblings, this one
/// cleanly confirms half of the ticket's hypothesis. Deriving "sentence-
/// initial existential never contracts" (`there's → there is` unconditionally)
/// leaves only 9 mismatches out of 31685 supported faces, and all 9 are
/// **subordinate-position** `there's`, never sentence-initial: "Aang has
/// vigilance **as long as there's** a Lesson card in your graveyard." (Aang,
/// A Lot to Learn), "**if there's** a Lesson card in your graveyard, draw a
/// card." (Leaves from the Vine, Dragonfly Swarm) — plus Fire Nation Cadets,
/// First-Time Flyer, Moraug Fury of Akoum, Shauku Endbringer, Walltop
/// Sentries, World at War. So the "subordinate positions contract" half of
/// the hypothesis holds too, precisely on the residual; deriving would need
/// clause-position context this node doesn't carry, so the field stays
/// stored rather than half-deriving it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub struct ExistentialForm {
    verb_slot: crate::features::VerbSlot,
    contraction: Contraction,
}

impl ExistentialForm {
    const IS: Self = Self {
        verb_slot: crate::features::VerbSlot::Present {
            person: crate::features::Person::Third,
            number: crate::features::Number::Singular,
        },
        contraction: Contraction::Full,
    };
    const CONTRACTED_IS: Self = Self {
        verb_slot: crate::features::VerbSlot::Present {
            person: crate::features::Person::Third,
            number: crate::features::Number::Singular,
        },
        contraction: Contraction::Contracted,
    };
    const ARE: Self = Self {
        verb_slot: crate::features::VerbSlot::Present {
            person: crate::features::Person::Third,
            number: crate::features::Number::Plural,
        },
        contraction: Contraction::Full,
    };
    const WAS: Self = Self {
        verb_slot: crate::features::VerbSlot::Past {
            person: crate::features::Person::Third,
            number: crate::features::Number::Singular,
        },
        contraction: Contraction::Full,
    };
    const WERE: Self = Self {
        verb_slot: crate::features::VerbSlot::Past {
            person: crate::features::Person::Third,
            number: crate::features::Number::Plural,
        },
        contraction: Contraction::Full,
    };

    pub(crate) const FORMS: &'static [(Self, &'static str)] = &[
        (Self::IS, "there is"),
        (Self::CONTRACTED_IS, "there's"),
        (Self::ARE, "there are"),
        (Self::WAS, "there was"),
        (Self::WERE, "there were"),
    ];

    #[must_use]
    pub const fn new(
        verb_slot: crate::features::VerbSlot,
        contraction: Contraction,
    ) -> Option<Self> {
        match (verb_slot, contraction) {
            (
                crate::features::VerbSlot::Present {
                    person: crate::features::Person::Third,
                    number: crate::features::Number::Singular,
                },
                Contraction::Full | Contraction::Contracted,
            )
            | (
                crate::features::VerbSlot::Present {
                    person: crate::features::Person::Third,
                    number: crate::features::Number::Plural,
                }
                | crate::features::VerbSlot::Past {
                    person: crate::features::Person::Third,
                    number: crate::features::Number::Singular | crate::features::Number::Plural,
                },
                Contraction::Full,
            ) => Some(Self {
                verb_slot,
                contraction,
            }),
            _ => None,
        }
    }

    #[must_use]
    pub const fn verb_slot(self) -> crate::features::VerbSlot {
        self.verb_slot
    }

    #[must_use]
    pub const fn contraction(self) -> Contraction {
        self.contraction
    }

    pub(crate) fn spelling(self) -> &'static str {
        Self::FORMS
            .iter()
            .find_map(|(form, spelling)| (*form == self).then_some(*spelling))
            .expect("every existential form has one spelling")
    }

    pub(crate) const fn number(self) -> crate::word::Number {
        match self.verb_slot {
            crate::features::VerbSlot::Present { number, .. }
            | crate::features::VerbSlot::Past { number, .. } => number,
            _ => unreachable!(),
        }
    }
}
