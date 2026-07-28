#![allow(
    dead_code,
    reason = "later grammar milestones consume the staged clause and ability categories"
)]

pub(crate) mod ability;
mod clause;
mod nominal;
mod opacity;

use std::collections::HashMap;

use crate::Numeral;
use crate::Span;
use crate::catalog::CatalogAtom;
use crate::catalog::CatalogKind;
use crate::catalog::CatalogSlot;
use crate::catalog::CatalogValue;
use crate::catalog::Catalogs;
use crate::chart::ChartResult;
use crate::chart::ChartStats;
use crate::chart::Child;
use crate::chart::Expected;
use crate::chart::Grammar;
use crate::chart::GrammarError;
use crate::chart::LexicalMatch;
use crate::chart::Reduction;
use crate::chart::Rule;
use crate::chart::RuleId;
use crate::chart::parse_chart;
use crate::forest::BestParse;
use crate::forest::ForestError;
use crate::forest::ForestStats;
use crate::forest::ForestSymbol;
use crate::forest::NodeId;
use crate::forest::ParseCost;
use crate::forest::ParseForest;
use crate::identity::SelfReference;
use crate::surface::Punctuation;
use crate::surface::Token;
use crate::surface::TokenKind;
use crate::surface::collapse_full_names;
use crate::surface::lex;
use crate::syntax::AdjectivePhrase;
use crate::syntax::Clause;
use crate::syntax::ComparativeWord;
use crate::syntax::ComparisonComplement;
use crate::syntax::ComparisonMarker;
use crate::syntax::CopularComplement;
use crate::syntax::Demonstrative;
use crate::syntax::Determiner;
use crate::syntax::ExistentialForm;
use crate::syntax::FrequencyBound;
use crate::syntax::FrequencyCount;
use crate::syntax::FrequencyPhrase;
use crate::syntax::GerundClause;
use crate::syntax::IndefiniteArticle;
use crate::syntax::InfinitiveMarker;
use crate::syntax::KeywordArgument;
use crate::syntax::KeywordCost;
use crate::syntax::NominalComplement;
use crate::syntax::NominalModifier;
use crate::syntax::NominalPhrase;
use crate::syntax::NounPhrase;
use crate::syntax::OpaqueLexeme;
use crate::syntax::OracleSymbol;
use crate::syntax::Phrase;
use crate::syntax::Polarity;
use crate::syntax::Possessor;
use crate::syntax::PowerToughness;
use crate::syntax::PredicatedArgument;
use crate::syntax::PredicatedQuality;
use crate::syntax::Preposition;
use crate::syntax::PrepositionalPhrase;
use crate::syntax::PreverbModifier;
use crate::syntax::Quantity;
use crate::syntax::RelativeClause;
use crate::syntax::RelativeGap;
use crate::syntax::RelativeMarker;
use crate::syntax::Sentence;
use crate::syntax::SetExceptionMarker;
use crate::syntax::SetExceptionNounPhrase;
use crate::syntax::Subject;
use crate::syntax::ThisCardForm;
use crate::syntax::VerbParticle;
use crate::word::Adjective;
use crate::word::AdjectiveComparisonClass;
use crate::word::Auxiliary;
use crate::word::AuxiliaryInflection;
use crate::word::AuxiliaryInstance;
use crate::word::BareNominalAdjunct;
use crate::word::CardOrientation;
use crate::word::InitialSound;
use crate::word::LexicalSlot;
use crate::word::Noun;
use crate::word::NounInstance;
use crate::word::NounUsage;
use crate::word::Number;
use crate::word::Person;
use crate::word::PredicateComplementKind;
use crate::word::PredicateFrame;
use crate::word::PrepositionalRole;
use crate::word::Pronoun;
use crate::word::PronounCase;
use crate::word::PronounInstance;
use crate::word::Verb;
use crate::word::VerbInstance;
use crate::word::VerbSlot;
use crate::word::Vocab;
use crate::word::Vocabulary;
use crate::word::WordMatch;
use crate::word::comparative_word;
use crate::word::surface_initial_sound;

#[derive(Debug, Clone, PartialEq, Eq)]
struct VerbPhrase {
    auxiliaries: Vec<AuxiliaryInstance>,
    first_auxiliary_contracted_with_subject: bool,
    preverb_modifiers: Vec<PreverbModifier>,
    verb: VerbInstance,
    frame: PredicateFrame,
    dependents: Vec<VerbDependent>,
    /// Set only by `lower_simple_clause`'s two distributive-`each` float
    /// arms, after the completed predicate is taken — never inferred from
    /// subject shape. Copied into `PredicateHead::distributive_each` by
    /// `finish_predicate`.
    distributive_each: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct VerbAnalysis {
    instance: VerbInstance,
    frame: PredicateFrame,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum VerbDependent {
    DirectObject(NounPhrase),
    IndirectObject(NounPhrase),
    PredicateComplement(Phrase),
    Scalar(Phrase),
    Statistic(Phrase),
    Prepositional(PrepositionalPhrase),
    Temporal(NounPhrase),
    Manner(NounPhrase),
    Infinitive(InfinitiveClause),
    Subordinate(Box<Clause>),
    Adverbial(Phrase),
    Frequency(FrequencyPhrase),
    Particle(VerbParticle),
    CoinResult(crate::syntax::CoinSide),
    CoordinatedObject(crate::syntax::CoordinatedPredicateObject),
    /// A coordinated predicative-adjective complement (`are green and white`),
    /// finished into a [`PredicateComplement::CoordinatedAdjective`].
    CoordinatedAdjective(crate::syntax::CoordinatedAdjectivePhrase),
    /// A closed `except by <PP>` exception tail on a passive predicate,
    /// finished into [`PredicateAdjunct::Exception`]. Distinct from an
    /// ordinary `Prepositional` dependent so it cannot be mistaken for the
    /// verb frame's own selected-preposition complement.
    Exception(PrepositionalPhrase),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct InfinitiveClause {
    negated: bool,
    marker: InfinitiveMarker,
    predicate: Box<VerbPhrase>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SimpleClause {
    subject: Option<Subject>,
    predicate: VerbPhrase,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ContractedSubjectAuxiliary {
    subject: Subject,
    auxiliary: AuxiliaryInstance,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CopularRemainder {
    /// See [`crate::syntax::CopularPredicate::negated`].
    negated: bool,
    distributive_each: bool,
    precomplement_adverbs: Vec<Vocab>,
    complement: CopularComplement,
    /// Trailing prepositional adjuncts of the copular predication (`it's
    /// legendary *in addition to its other types*`). The `become`/`is`
    /// intransitive path already carries these as verb-phrase adjuncts; a
    /// copular clause records them here and the renderer replays them after the
    /// complement.
    adjuncts: Vec<crate::syntax::PredicateAdjunct>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum ContractedSubjectKey {
    Pronoun(Pronoun),
    Demonstrative(Demonstrative),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Nonterminal {
    Quantity,
    Determiner,
    Adjective,
    AdjectivePhrase,
    ComparisonStandard,
    ComparisonComplement,
    Noun,
    Nominal,
    NounPhrase,
    /// A single coordinable attributive modifier — an adjective phrase, a noun,
    /// or a `non-` negated modifier — the atom of a coordinated modifier list.
    ModifierConjunct,
    /// An open, comma-separated run of [`Self::ModifierConjunct`] atoms with no
    /// closing conjunction yet (`artifact, creature`). Reached only by the list
    /// and coordination-closing rules, never by the nominal prepend, so a bare
    /// comma run never becomes a modifier on its own.
    ModifierList,
    /// A closed coordinated modifier list (`white and blue`, `artifact,
    /// creature, and land`), consumed by the nominal prepend rule as one
    /// modifier slot.
    CoordinatedModifier,
    /// An open, comma-separated run of noun phrases with no closing conjunction
    /// yet (`artifact, enchantment`). Reached only by the list-extension and
    /// Oxford-close rules, never as a standalone noun phrase, so a bare comma
    /// run of noun phrases never coordinates on its own.
    NounPhraseList,
    /// One member of a mana-amount list: a lone oracle symbol (`{G}`) or a
    /// contiguous symbol group (`{C}{U}`). Reached only through the mana-list
    /// productions.
    ManaAmount,
    /// An open, comma-separated run of [`Self::ManaAmount`] atoms with no
    /// closing conjunction yet (`{W}, {B}`). Reached only by the
    /// list-extension and Oxford-close rules, so a bare comma run never
    /// becomes a mana amount on its own.
    ManaAmountList,
    /// A closed coordinated mana-amount run (`{R} or {G}`, `{W}, {B}, or
    /// {G}`, `{W}{W}, …, and {G}{G}`), consumed only by the verb-phrase
    /// attachment, so a bare single amount can never reach that attachment.
    CoordinatedManaAmount,
    /// The concrete-color argument of the `devotion` value nominal: a single
    /// color word or a two-color `and` pair. Reached only through the devotion
    /// production, so its bare-color rules never leak into ordinary phrases.
    DevotionColors,
    FrequencyPhrase,
    PossessiveNounPhrase,
    PrepositionalPhrase,
    PrepositionalObject,
    RelativeClause,
    Verb,
    VerbPhrase,
    /// A bare past-participial predicate whose lexical head carries the
    /// recipient-passive frame. Kept separate from `VerbPhrase` so a nominal
    /// attachment predicts only the one licensed participle, never the whole
    /// clause-level verb-phrase grammar.
    ReducedRecipientPassive,
    /// The retained `damage` theme of a reduced recipient passive. It admits
    /// no postnominal complement, so `damage by …` cannot swallow the
    /// participle's agent PP inside its object, and opacity can never invent a
    /// theme from an unrelated post-participial word.
    ReducedRecipientPassiveTheme,
    ObjectGapVerbPhrase,
    InfinitiveClause,
    GerundClause,
    CopularRemainder,
    /// A coordinated list of exception clauses under a leading `except` marker,
    /// trailing a copy (or other) host clause. Reached only through the
    /// [`ClauseExcepted`](RuleTag::ClauseExcepted) attachment, so its
    /// finite-clause coordination never competes with the general clause
    /// coordination.
    ExceptionRider,
    /// One `only <adjunct>` restriction-run member. Internal to the run; never
    /// attaches to a clause on its own.
    RestrictionMember,
    /// A coordinated run of two or more [`Self::RestrictionMember`]s, reached
    /// only through the [`ClauseRestrictionRun`](RuleTag::ClauseRestrictionRun)
    /// attachment.
    RestrictionRun,
    SimpleClause,
    Clause,
    Sentence,
    Paragraph,
    Cost,
    /// One explicit-`from` quality of a `kwgrant`-round predicated keyword
    /// argument (`from black`, `from artifacts`) — Stage B.
    PredicatedQualityFrom,
    /// A `from`-quality list, one or more [`Self::PredicatedQualityFrom`]
    /// members joined by `and` (never `or`, the confirmed deferral) — Stage
    /// B. Reached only through the list-extension/nominal-attachment rules.
    PredicatedArgumentFrom,
    /// The single bare (no `from`) first quality of an atom-carried
    /// predicated keyword line/grant (`monocolored`, `each color`,
    /// `artifacts, creatures, and enchantments`) — Stage C.
    PredicatedQualityBare,
    /// A bare-headed quality list: one [`Self::PredicatedQualityBare`]
    /// optionally followed by `and`-joined [`Self::PredicatedQualityFrom`]
    /// repeats — Stage C.
    PredicatedArgumentBare,
    KeywordAbility,
    KeywordAbilityList,
    ActivatedAbility,
    TriggeredAbility,
    LoyaltyAbility,
    ModalAbility,
    Ability,
    OracleText,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum EnglishLexicalSlot {
    Number(Numeral),
    Noun(NounUsage),
    PossessiveNoun,
    Verb(VerbSlot),
    /// A past participle whose lexical frame licenses recipient passivization.
    /// The scan-time frame filter is the production's first categorical gate;
    /// generic verb phrases are never predicted from the nominal attachment.
    ReducedRecipientPassiveParticiple,
    Adjective,
    /// A single color word (`white`/`blue`/`black`/`red`/`green`) in the
    /// devotion value nominal's argument. Distinct from [`Self::Adjective`] so
    /// the bare-color devotion production sees only colors.
    ColorWord,
    /// The `devotion` value nominal head [CR#700.5], matched as a lowercase
    /// whole word from the hand-curated rules-nominal table. Gates the
    /// bare-color argument production to exactly this word.
    DevotionValue,
    /// The plural noun `times` heading the `number of times <clause>` measured
    /// value. Distinct from the ordinary noun slot so the finite-clause
    /// complement attaches to exactly this word.
    TimesNoun,
    /// The singular vocabulary noun `Vocab::Number` heading the `any number
    /// of <plural NounPhrase>` notional-plural production. Distinct from the
    /// ordinary noun slot so that production's categorical gate lives at scan
    /// (dot 1), not at reduce [`anof` round].
    NumberNoun,
    /// The literal word `next` in its preverbal-adverb reading (`when you
    /// *next* cast an instant or sorcery spell this turn`). Recognized only as
    /// this exact literal token — never the ordinary [`Self::Adverb`] slot — so
    /// the preverbal production sees only this word and no other adverb becomes
    /// placeable between a subject and its finite verb.
    PreverbAdverb,
    /// The literal word `declare` heading the `declare attackers`/`declare
    /// blockers` combat-step formative [CR#508.1,509.1]. Recognized only as
    /// this exact literal token — never the ordinary `Verb` slot — so the
    /// production built from it is structurally incapable of matching any
    /// other span (the `restrict`-round `Verb(Imperative)` regression this
    /// slot replaces; see `declarestep-forest-dump.txt`).
    CombatStepDeclare,
    /// The literal word `attackers`/`blockers` in the combat-step formative.
    /// Recognized only as one of those two exact surfaces — never the
    /// ordinary `Noun` slot.
    CombatStepParticipants,
    /// The literal word `step` heading the combat-step nominal. Recognized
    /// only as this exact literal token.
    CombatStepHead,
    /// A single-token `non-` negation whose residue resolves as a modifier base
    /// (`nonland`, `nonblack`, `non-Human`, `nonattacking`). Scanned as
    /// sub-word morphology, not a chart production, because the prefix is
    /// not a token.
    NegatedModifier,
    Adverb,
    /// An adverb licensed by vocabulary metadata to front a clause before a
    /// comma; distinct from [`Self::Adverb`] so ordinary adverbs never reach
    /// the fronting production.
    SentenceAdverbial,
    VerbParticle(VerbParticle),
    CoinResult(crate::syntax::CoinSide),
    Frequency,
    Pronoun(PronounCase),
    Auxiliary,
    AbilityItem,
    AbilityWord,
    /// A keyword-ability catalog atom in symbol-argument grant position
    /// (`ward {2}`, `equip {1}`) — every catalog keyword atom scanned as the
    /// same mass `Noun::Catalog` edge [`CatalogSlot::KeywordAbilityNoun`]
    /// already uses. An ordinary noun never enters this slot; only a
    /// catalog-surface property (keyword-atom membership) gates it, never a
    /// keyword-name list [`kwgrant` round].
    SymbolArgumentKeywordNoun,
    /// The exact literal word `from` introducing a Stage B predicated
    /// keyword quality (`ward` is Stage A; `from black` is Stage B). Never
    /// the generic [`Self::Preposition`] scan: Stage B admits only `from`,
    /// not every nominal preposition [`kwgrant` round].
    FromWord,
    /// A keyword-ability catalog atom in explicit-`from`-quality grant
    /// position (`protection from black`) — Stage B. Suppresses a shorter
    /// atom at this position when a longer catalog atom that itself carries
    /// the final word `from` (`Hexproof from`) also matches here, so
    /// `Hexproof` can never win over `Hexproof from` at the same start.
    ExplicitPredicatedKeywordNoun,
    /// A keyword-ability catalog atom whose canonical spelling itself ends
    /// in the standalone word `from` (`Hexproof from`) — Stage C. Only such
    /// an atom licenses a bare (no explicit `from`) first quality.
    AtomCarriedPredicatedKeywordNoun,
    Determiner,
    Demonstrative,
    DeterminerTarget,
    QuantityAtLeast,
    QuantityOr,
    QuantityX,
    QuantityBoth,
    QuantityThatMany,
    QuantityThatMuch,
    QuantityBound(BoundedQuantityKind),
    Than,
    OrEqualTo,
    RelativeMarker,
    Face,
    Up,
    Down,
    Not,
    To,
    Of,
    /// The exact preposition `for` inside `except for <NounPhrase>`.
    ForWord,
    EachDeterminer,
    /// The closed-class `any` determiner heading the `any number of <plural
    /// NounPhrase>` notional-plural production. Distinct from the ordinary
    /// `Determiner` slot so the production's dot-0 item scans nothing but
    /// `any` [`anof` round].
    AnyDeterminer,
    Reciprocal,
    ThisCard,
    FullThisCard,
    PossessiveThisCard,
    Preposition,
    OracleSymbol,
    SymbolSequence,
    /// A quoted ability (`"…"`) filling an object or coordinated-conjunct slot.
    /// Scanned as one lexical unit spanning both delimiters; the interior
    /// parses recursively as an [`Ability`](crate::syntax::Ability) at
    /// lowering.
    QuotedAbility,
    PowerToughness,
    Punctuation(Punctuation),
    Subordinator,
    RatherThan,
    Conjunction,
    Plus,
    /// The word `minus` heading a subtraction value expression.
    Minus,
    /// The word `half` heading a halving value expression.
    Half,
    /// The word `rounded` heading a `rounded up`/`rounded down` rider.
    Rounded,
    Existential,
    Copula,
    SubjectAuxiliary,
    /// The word `except` heading an exception rider. A dedicated slot (not a
    /// generic subordinator) so the rider's finite-clause coordination is
    /// reached only through the exception productions.
    Except,
    Opaque(OpacitySlot),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum OpacityMode {
    Exact,
    OpaqueNouns,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum BoundedQuantityKind {
    MoreThan,
    FewerThan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum OpacityProfile {
    Exact,
    Nouns,
}

const OPACITY_STATE_LIMIT: usize = 50_000;

impl OpacityProfile {
    const fn mode(self) -> OpacityMode {
        match self {
            Self::Exact => OpacityMode::Exact,
            Self::Nouns => OpacityMode::OpaqueNouns,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum OpacitySlot {
    Noun(NounForm),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum NounForm {
    Singular,
    Plural,
    Mass,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Cardinality {
    SingularCount,
    SingularOrMass,
    PluralCount,
    Mass,
    PluralOrMass,
    Unconstrained,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct QuantityFeatures {
    cardinality: Cardinality,
    standalone_number: Number,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Agreement {
    person: Person,
    number: Number,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum PredicateForm {
    Imperative,
    Infinitive,
    Finite(Option<Agreement>),
    PresentParticiple,
    PastParticiple,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum PredicateObjectState {
    None,
    Direct,
    Ability,
    AbilityWithArgument,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum PredicateAttachmentPhase {
    Object,
    Tail,
    PrepositionalTail,
    /// Terminal: an `except by <PP>` exception tail has closed the predicate.
    /// No further attachment of any kind is licensed once this phase is
    /// reached.
    ExceptionTail,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum NominalAttachmentPhase {
    Open,
    Prepositional,
    Relative,
    /// An otherwise-complete explicit relative ending in a bare copular verb
    /// (`creature that was`). A following participle belongs to that copula as
    /// its auxiliary complement, not to a second markerless relative.
    RelativeBareCopula,
    /// A closed markerless recipient-passive relative. Any following PP or
    /// bare temporal/manner nominal belongs to the participial predicate and
    /// must be consumed before it attaches to the nominal; closing here keeps
    /// `by …` from being mis-bracketed as a complement of the antecedent noun.
    ReducedRecipientPassive,
    PostpositiveAdjective,
    Comparison,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum AdjectiveComparisonState {
    NotComparative,
    /// Awaiting a `… than X` complement. The class records which vocabulary
    /// comparison produced it: only `OrComparative` (`less`/`fewer`/`greater`/
    /// `more`) admits a numeral degree measure; `ThanOnly` (`other`) must not
    /// — round `copsubj` Stage B fired on `three other creatures` because this
    /// distinction was erased here.
    Pending(AdjectiveComparisonClass),
    Complete,
    /// Completed by a numeral degree premodifier (`2 greater`). A distinct
    /// completion class, not `Complete`: it is **predicative only**, and every
    /// attributive/coordinating consumer below rejects it. Keeping it a
    /// separate variant makes the compiler force that audit at each match.
    Measured,
}

/// What a scanned copula constrains. `Indicative` carries the exact
/// person/number the subject must match — the strictness this pathway has
/// always had. `PastSubjunctive` carries no agreement at all and is licensed
/// only where `conditional_reduction` consumes the `subjunctive` flag under
/// `Subordinator::AsThough`; it is not, and must never become, an agreement
/// bypass for indicative readings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum CopulaAgreement {
    Indicative(Agreement),
    PastSubjunctive,
}

impl PredicateObjectState {
    const fn has_direct_object(self) -> bool {
        !matches!(self, Self::None)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum SetExceptionState {
    Ineligible,
    Host,
    Closed,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum Features {
    None,
    Number {
        is_one: bool,
    },
    Quantity(QuantityFeatures),
    PowerToughness {
        initial_sound: InitialSound,
    },
    Determiner {
        cardinality: Cardinality,
        article: Option<IndefiniteArticleKey>,
        /// True only for the set-denoting `all`/`each` determiner class used
        /// by nominal set exceptions. In particular, an indefinite `a card`
        /// must not acquire the deferred draw-event `except the first one`
        /// reading.
        set_exception_host: bool,
    },
    Adjective {
        initial_sound: InitialSound,
        comparison: AdjectiveComparisonState,
        card_orientation: bool,
    },
    Noun {
        form: NounForm,
        initial_sound: InitialSound,
        adjunct: Option<BareNominalAdjunct>,
        /// Whether this lexical noun is licensed opacity rather than known
        /// vocabulary or a catalog atom.
        opaque: bool,
        /// True only for the lexical `damage` theme licensed by the dedicated
        /// recipient-passive reduced-relative construction.
        recipient_passive_theme: bool,
    },
    Nominal {
        form: NounForm,
        initial_sound: InitialSound,
        determined: bool,
        /// Whether this nominal carries any modifier at all (adjective, noun
        /// modifier, quantity, power/toughness, or negated modifier) — set
        /// `true` only by `nominal_with_prefix`, the shared helper behind
        /// every modifier-adding rule. A nominal that is both `!determined`
        /// and `!modified` is completely bare, and per the `NounPhraseNominal`
        /// gate below must not surface its `adjunct` licensing: bare residue
        /// of a split compound (e.g. the bare `step` left over when `draw
        /// step` mis-brackets) must never itself qualify as a bare temporal
        /// adjunct head [declarestep-plan-C.md §2].
        modified: bool,
        leading_opacity: bool,
        attachment: NominalAttachmentPhase,
        comparison: AdjectiveComparisonState,
        adjunct: Option<BareNominalAdjunct>,
        /// Whether the nominal's semantic head is an opaque noun. This remains
        /// stable as modifiers and complements attach.
        opaque_head: bool,
        /// Preserves whether the nominal is headed by a set-denoting `all` or
        /// `each` determiner through modifiers and ordinary complements.
        set_exception_host: bool,
        /// Preserves the lexical `damage` theme flag through ordinary nominal
        /// modifiers while rejecting opaque or complemented lookalikes.
        recipient_passive_theme: bool,
    },
    NounPhrase {
        agreement: Option<Agreement>,
        pronoun_case: Option<PronounCase>,
        adjunct: Option<BareNominalAdjunct>,
        set_exception: SetExceptionState,
    },
    PossessiveThisCard {
        agreement: Agreement,
    },
    PossessiveNounPhrase {
        form: NounForm,
        initial_sound: InitialSound,
        determined: bool,
    },
    Verb {
        slot: VerbSlot,
        frame: PredicateFrame,
        head_is_copular: bool,
    },
    VerbPhrase {
        form: PredicateForm,
        passive: bool,
        object: PredicateObjectState,
        indirect_object: bool,
        selected_preposition: bool,
        phase: PredicateAttachmentPhase,
        frame: PredicateFrame,
        bare: bool,
        /// Preserves whether the lexical predicate head is `be`; auxiliary
        /// folding never changes it.
        head_is_copular: bool,
        /// Set only by a `PastSubjunctive` `Be` auxiliary heading this verb
        /// phrase; threaded unchanged by reduces. Licensing gate: only
        /// `RuleTag::ClauseSubordinateAfter`/`…Comma` may accept a
        /// subjunctive body, and only under `Subordinator::AsThough` — see
        /// `Features::Subordinator` and the reduce arm in `clause.rs`.
        subjunctive: bool,
    },
    InfinitiveClause,
    GerundClause,
    SimpleClause {
        agreement: Option<Agreement>,
        has_subject: bool,
        standalone: bool,
        has_direct_object: bool,
        host_addressee_subject: bool,
        host_modal: bool,
        subjunctive: bool,
    },
    Clause {
        agreement: Option<Agreement>,
        standalone: bool,
        finite: bool,
        host_addressee_subject: bool,
        host_modal: bool,
        subjunctive: bool,
    },
    Sentence,
    Preposition(Preposition),
    /// A subordinating conjunction lexeme, carried so `RuleTag::
    /// ClauseSubordinateAfter`/`…Comma` can identify `Subordinator::AsThough`
    /// specifically — the only subordinator permitted to host a
    /// subjunctive body.
    Subordinator(crate::syntax::Subordinator),
    PrepositionalObject {
        gerund: bool,
    },
    PrepositionalPhrase {
        preposition: Preposition,
        nominal_attachment: bool,
    },
    VerbParticle(VerbParticle),
    CoinResult(crate::syntax::CoinSide),
    RelativeClause {
        gap: RelativeGap,
        antecedent_agreement: Option<Agreement>,
        /// True only when an explicit relative currently ends at a bare
        /// lexical `be` (`that was`), where a following participle must extend
        /// the same relative rather than attach as a reduced sibling.
        bare_copular_tail: bool,
    },
    Auxiliary(AuxiliaryInstance),
    Conjunction(crate::syntax::PredicateConjunction),
    Existential {
        number: Number,
    },
    Copula(CopulaAgreement),
    SubjectAuxiliary {
        subject: ContractedSubjectKey,
        agreement: Agreement,
        auxiliary: AuxiliaryInstance,
    },
    /// A coordinated list of exception clauses gathered under a leading
    /// `except` marker. Fieldless: the rider carries no agreement of its own —
    /// each conjunct is an independently agreeing finite clause.
    ExceptionRider,
    /// One `only …` restriction-run member. Fieldless, mirroring
    /// `ExceptionRider`.
    RestrictionMember,
    /// A coordinated run of two or more restriction-run members. Fieldless.
    RestrictionRun,
    /// One quality of a `kwgrant`-round predicated keyword argument (`from
    /// black`, `monocolored`). Fieldless: the reduce/lower distinguish shape
    /// from the child productions, not from this marker, which exists only
    /// so the rule does not reduce indistinguishably as `Features::None`.
    PredicatedQuality,
    /// A coordinated list of one or more [`Self::PredicatedQuality`]
    /// members. Fieldless, mirroring `PredicatedQuality`.
    PredicatedArgument,
    /// A modifier conjunct, list, or closed coordinated modifier. Carries the
    /// first conjunct's initial sound so the nominal prepend can set the a/an
    /// of the whole phrase (`an artifact, creature, and land card`, `a
    /// white and blue creature`); a leading `non-` conjunct fixes it to a
    /// consonant.
    ///
    /// `all_adjectives` is `true` only when every conjunct is a plain positive
    /// attributive adjective (a color, a supertype, a vocabulary adjective) —
    /// never a noun or a `non-` negation. The predicative-adjective consumer
    /// (Family C's `VerbPhraseCoordinatedAdjective`) gates on it so a bare
    /// coordinated *noun* pair after a verb (`Enchant creature or Vehicle`)
    /// never reduces as a predicative adjective complement, keeping the
    /// ordinary coordinated-noun-object parse.
    CoordinatedModifier {
        initial_sound: InitialSound,
        all_adjectives: bool,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct NumberKey {
    value: i32,
    notation: Numeral,
}

impl NumberKey {
    const fn literal(self) -> crate::syntax::NumberLiteral {
        crate::syntax::NumberLiteral {
            value: self.value,
            numeral: self.notation,
        }
    }
}

/// Reinterprets a number that filled a compound-quantity slot.
///
/// The surface `X` is the game's variable ([CR#107.3]), never the Roman
/// numeral ten. `Numeral::parse` is canonicalising (`canonical`,
/// numeral.rs:103-111: a parse succeeds only when `format` reproduces the
/// input), and `Numeral::Roman` is the only notation whose canonical
/// spelling of any value is `X`, so `(Numeral::Roman, 10)` holds **exactly
/// when** the scanned surface was `X`.
/// Parses `surface` as `notation`, exactly as every closed-class lexeme
/// matches through [`Parser::one_token_match`] with `eq_ignore_ascii_case`
/// (`grammar/mod.rs:1538-1542`) — except the numeral scanners historically
/// fed raw surface text straight into `Numeral`'s canonical codec, so
/// sentence-initial capitalized cardinals (`Two target creatures`) never
/// matched [`anof` round]. Case-folds only `Numeral::Cardinal`, and only as a
/// retry after an as-written parse fails, so every other notation and every
/// already-lowercase cardinal keep today's exact behavior.
///
/// `sentence_initial` gates the retry to the one position this stage's own
/// blast-radius measurement covers. Every other lexical/catalog scan in this
/// grammar already refuses a capitalized non-sentence-initial token as a
/// common-word reading for exactly this reason (`word_matches`,
/// `grammar/mod.rs:1648-1659`; `catalog_matches`, `grammar/mod.rs:1699-1704`):
/// reading a capitalized token as a lowercase notation corrupts a proper
/// name carried mid-sentence (`named Prisoner Zero`, `attached to Three
/// Dog`). Without this gate the round-trip regressed on exactly those two
/// rows; this restores the existing idiom rather than inventing a new one.
///
/// `one` is excluded from the retry because it is also a fused-head count
/// noun; sentence-initial `One` is deliberately left to that reading — see
/// `one_is_a_dispreferenced_fused_head_noun`
/// (`tests/public_api.rs`). Letting `One` compete inside the quantity
/// scanners resolves the resulting ambiguity to a materially wrong tree
/// (`english-quantifier-float-residue.md` §1); this exclusion is the fix.
///
/// `numeral.rs` is untouched: `Numeral::Cardinal.parse` and `canonical()`
/// remain strict, single-case codecs. Only the grammar folds case, and only
/// here.
fn parse_notation(notation: Numeral, surface: &str, sentence_initial: bool) -> Option<i32> {
    if let Ok(value) = notation.parse(surface) {
        return Some(value);
    }
    if notation == Numeral::Cardinal && sentence_initial && !surface.eq_ignore_ascii_case("one") {
        let lowered = surface.to_ascii_lowercase();
        if lowered != surface {
            return notation.parse(&lowered).ok();
        }
    }
    None
}

const fn quantity_value(number: NumberKey) -> crate::syntax::QuantityValue {
    if matches!(number.notation, Numeral::Roman) && number.value == 10 {
        crate::syntax::QuantityValue::Variable
    } else {
        crate::syntax::QuantityValue::Literal(number.literal())
    }
}

#[cfg(test)]
mod quantity_value_tests {
    use super::*;

    /// `X` is the game's variable, and the predicate `(Roman, 10)` identifies
    /// it exactly: `Numeral::parse` is canonicalising, so a successful parse
    /// implies `format` reproduces the surface, and Roman is the only
    /// notation that spells any value `X`.
    #[test]
    fn roman_ten_is_the_only_notation_parsing_x() {
        for notation in [
            Numeral::Cardinal,
            Numeral::Ordinal,
            Numeral::Arabic(false),
            Numeral::Arabic(true),
            Numeral::Roman,
        ] {
            assert_eq!(
                notation.parse("X").ok(),
                if notation == Numeral::Roman { Some(10) } else { None },
                "{notation:?}",
            );
        }
        assert_eq!(Numeral::Roman.format(10), "X");
    }

    #[test]
    fn quantity_value_maps_roman_ten_to_variable_and_everything_else_to_literal() {
        assert_eq!(
            quantity_value(NumberKey {
                value: 10,
                notation: Numeral::Roman,
            }),
            crate::syntax::QuantityValue::Variable,
        );
        assert_eq!(
            quantity_value(NumberKey {
                value: 10,
                notation: Numeral::Cardinal,
            }),
            crate::syntax::QuantityValue::Literal(crate::syntax::NumberLiteral {
                value: 10,
                numeral: Numeral::Cardinal,
            }),
        );
        assert_eq!(
            quantity_value(NumberKey {
                value: 3,
                notation: Numeral::Roman,
            }),
            crate::syntax::QuantityValue::Literal(crate::syntax::NumberLiteral {
                value: 3,
                numeral: Numeral::Roman,
            }),
        );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum QuantityKey {
    Exact(NumberKey),
    AtLeast(NumberKey),
    OrComparison(NumberKey, ComparativeWord),
    Or(NumberKey, NumberKey),
    UpTo(NumberKey),
    MoreThan(NumberKey),
    FewerThan(NumberKey),
    X,
    Both,
    ThatMany,
    ThatMuch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct FrequencyKey {
    bound: FrequencyBound,
    count: FrequencyCountKey,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum FrequencyCountKey {
    Once,
    Twice,
    Times(NumberKey),
}

impl FrequencyKey {
    const fn syntax(self) -> FrequencyPhrase {
        FrequencyPhrase {
            bound: self.bound,
            count: match self.count {
                FrequencyCountKey::Once => FrequencyCount::Once,
                FrequencyCountKey::Twice => FrequencyCount::Twice,
                FrequencyCountKey::Times(number) => FrequencyCount::Times(number.literal()),
            },
        }
    }
}

impl QuantityKey {
    const fn cardinality(self) -> Cardinality {
        match self {
            Self::Exact(number)
            | Self::UpTo(number)
            | Self::MoreThan(number)
            | Self::FewerThan(number)
                if number.value == 1 =>
            {
                Cardinality::SingularOrMass
            }
            Self::Or(first, second) if first.value == 1 && second.value == 1 => {
                Cardinality::SingularOrMass
            }
            // Every `N or <word>` bound heads a plural count (`two or more
            // creatures`) or a mass characteristic (`30 or more life`), never a
            // bare singular.
            Self::OrComparison(_, _)
            | Self::Exact(_)
            | Self::Or(_, _)
            | Self::UpTo(_)
            | Self::MoreThan(_)
            | Self::FewerThan(_)
            | Self::X
            | Self::Both => Cardinality::PluralOrMass,
            Self::AtLeast(_) | Self::ThatMany => Cardinality::PluralCount,
            Self::ThatMuch => Cardinality::Mass,
        }
    }

    const fn syntax(self) -> Quantity {
        match self {
            Self::Exact(number) => Quantity::Exact(number.literal()),
            Self::AtLeast(number) => Quantity::AtLeast(quantity_value(number)),
            Self::OrComparison(number, word) => {
                Quantity::OrComparison(quantity_value(number), word)
            }
            Self::Or(first, second) => Quantity::Or(first.literal(), second.literal()),
            Self::UpTo(number) => Quantity::UpTo(quantity_value(number)),
            Self::MoreThan(number) => Quantity::MoreThan(quantity_value(number)),
            Self::FewerThan(number) => Quantity::FewerThan(quantity_value(number)),
            Self::X => Quantity::X,
            Self::Both => Quantity::Both,
            Self::ThatMany => Quantity::ThatMany,
            Self::ThatMuch => Quantity::ThatMuch,
        }
    }

    const fn features(self) -> QuantityFeatures {
        QuantityFeatures {
            cardinality: self.cardinality(),
            standalone_number: match self {
                Self::Exact(number)
                | Self::UpTo(number)
                | Self::MoreThan(number)
                | Self::FewerThan(number)
                    if number.value == 1 =>
                {
                    Number::Singular
                }
                Self::Or(first, second) if first.value == 1 && second.value == 1 => {
                    Number::Singular
                }
                Self::X | Self::ThatMuch => Number::Singular,
                _ => Number::Plural,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum DeterminerKey {
    The,
    Each,
    Another,
    Indefinite(IndefiniteArticleKey),
    Demonstrative(DemonstrativeKey),
    Target(Option<QuantityKey>),
    Quantity(QuantityKey),
    Possessive(Pronoun),
    All,
    Any,
    No,
}

impl DeterminerKey {
    const fn cardinality(&self) -> Cardinality {
        match self {
            Self::Each | Self::Another | Self::Indefinite(_) | Self::Target(None) => {
                Cardinality::SingularCount
            }
            // The singular demonstratives determine a singular count noun (`that
            // creature`) or a mass one (`that damage`, `that mana`); only the
            // plural `these`/`those` are barred from mass.
            Self::Demonstrative(DemonstrativeKey::This | DemonstrativeKey::That) => {
                Cardinality::SingularOrMass
            }
            Self::Demonstrative(DemonstrativeKey::These | DemonstrativeKey::Those) => {
                Cardinality::PluralCount
            }
            Self::Target(Some(
                QuantityKey::Exact(number)
                | QuantityKey::UpTo(number)
                | QuantityKey::MoreThan(number)
                | QuantityKey::FewerThan(number),
            )) if number.value == 1 => Cardinality::SingularCount,
            Self::Target(Some(
                QuantityKey::Exact(_)
                | QuantityKey::AtLeast(_)
                | QuantityKey::OrComparison(_, _)
                | QuantityKey::Or(_, _)
                | QuantityKey::UpTo(_)
                | QuantityKey::MoreThan(_)
                | QuantityKey::FewerThan(_)
                | QuantityKey::X
                | QuantityKey::Both
                | QuantityKey::ThatMany,
            )) => Cardinality::PluralCount,
            Self::Target(Some(QuantityKey::ThatMuch)) => Cardinality::Mass,
            Self::Quantity(quantity) => quantity.cardinality(),
            Self::All => Cardinality::PluralOrMass,
            Self::The | Self::Possessive(_) | Self::Any | Self::No => Cardinality::Unconstrained,
        }
    }

    fn syntax(&self) -> Determiner {
        match self {
            Self::The => Determiner::The,
            Self::Each => Determiner::Each,
            Self::Another => Determiner::Another,
            Self::Indefinite(IndefiniteArticleKey::A) => {
                Determiner::Indefinite(IndefiniteArticle::A)
            }
            Self::Indefinite(IndefiniteArticleKey::An) => {
                Determiner::Indefinite(IndefiniteArticle::An)
            }
            Self::Demonstrative(DemonstrativeKey::This) => {
                Determiner::Demonstrative(Demonstrative::This)
            }
            Self::Demonstrative(DemonstrativeKey::That) => {
                Determiner::Demonstrative(Demonstrative::That)
            }
            Self::Demonstrative(DemonstrativeKey::These) => {
                Determiner::Demonstrative(Demonstrative::These)
            }
            Self::Demonstrative(DemonstrativeKey::Those) => {
                Determiner::Demonstrative(Demonstrative::Those)
            }
            Self::Target(quantity) => Determiner::Target(quantity.map(QuantityKey::syntax)),
            Self::Quantity(quantity) => Determiner::Quantity(quantity.syntax()),
            Self::Possessive(pronoun) => Determiner::Possessive(Possessor::Pronoun(*pronoun)),
            Self::All => Determiner::All,
            Self::Any => Determiner::Any,
            Self::No => Determiner::No,
        }
    }

    const fn article(&self) -> Option<IndefiniteArticleKey> {
        match self {
            Self::Indefinite(article) => Some(*article),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum IndefiniteArticleKey {
    A,
    An,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum DemonstrativeKey {
    This,
    That,
    These,
    Those,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum LiteralKey {
    Face,
    Up,
    Down,
    Not,
    To,
    Target,
    Than,
    OrEqualTo,
    Plus,
    Minus,
    Half,
    Rounded,
    Except,
    CombatStepDeclare,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum MeaningKey {
    Literal(LiteralKey),
    Number(NumberKey),
    Quantity(QuantityKey),
    Determiner(DeterminerKey),
    Noun(NounInstance),
    Adjective(Adjective),
    Adverb(Vocab),
    VerbParticle(VerbParticle),
    CoinResult(crate::syntax::CoinSide),
    Frequency(FrequencyKey),
    Pronoun(PronounInstance),
    Auxiliary(AuxiliaryInstance),
    Verb(VerbAnalysis),
    Catalog(crate::catalog::CatalogAtom),
    OracleSymbol(OracleSymbol),
    SymbolSequence(Vec<OracleSymbol>),
    /// The source span of a quoted ability's interior (between the delimiters).
    /// Carried as a `Span` — `Hash`, unlike the parsed `Ability` — and reparsed
    /// at lowering by
    /// [`parse_quoted_ability_fragment`](ability::parse_quoted_ability_fragment).
    QuotedAbility(Span),
    PowerToughness(PowerToughness),
    Punctuation(Punctuation),
    Conjunction(crate::syntax::PredicateConjunction),
    Subordinator(crate::syntax::Subordinator),
    RelativeMarker(RelativeMarker),
    Existential(ExistentialForm),
    SubjectAuxiliary(SubjectAuxiliaryKey),
    ThisCard(ThisCardForm),
    Preposition(Preposition),
    NegatedModifier(NegatedModifierKey),
    Opaque(OpaqueKey),
}

/// A `non-` negation resolved to its base modifier at scan time. Held as a
/// bare [`Adjective`] or [`NounInstance`] (both `Hash`, unlike
/// `AdjectivePhrase`); lowered into a negated [`NominalModifier`]. Both
/// surface spellings (`non` and `non-`) resolve to the same key — the render
/// side derives the glyph from the base's capitalization.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct NegatedModifierKey {
    base: NegatedBase,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum NegatedBase {
    Adjective(Adjective),
    Noun(NounInstance),
}

impl NegatedModifierKey {
    fn build(&self) -> NominalModifier {
        let polarity = Polarity::Negative;
        match &self.base {
            NegatedBase::Adjective(adjective) => NominalModifier::Adjective {
                polarity,
                phrase: AdjectivePhrase {
                    degree: None,
                    head: adjective.clone(),
                    complements: Vec::new(),
                },
            },
            NegatedBase::Noun(noun) => NominalModifier::Noun {
                polarity,
                noun: noun.clone(),
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct OpaqueKey {
    slot: OpacitySlot,
    span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct SubjectAuxiliaryKey {
    subject: ContractedSubjectKey,
    auxiliary: AuxiliaryInstance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum RuleTag {
    QuantityExact,
    QuantityAtLeast,
    QuantityOr,
    QuantityX,
    QuantityBoth,
    QuantityUpTo,
    QuantityThatMany,
    QuantityThatMuch,
    QuantityMoreThan,
    QuantityFewerThan,
    DeterminerClosed,
    DeterminerTarget,
    DeterminerQuantifiedTarget,
    DeterminerQuantity,
    DeterminerPossessiveThisCard,
    PossessiveNounBase,
    PossessiveNounDetermined,
    DeterminerPossessiveNoun,
    PossessiveNounAdjective,
    Adjective,
    AdjectivePhrase,
    AdjectivePhraseFaceUp,
    AdjectivePhraseFaceDown,
    AdjectivePhraseComparison,
    AdjectivePhraseDegreeMeasure,
    ComparisonStandard,
    ComparisonThan,
    ComparisonThanOrEqualTo,
    Noun,
    NominalNoun,
    NominalAdjective,
    NominalNounModifier,
    /// The fixed `declare attackers`/`declare blockers` combat-step name
    /// [CR#508.1,509.1], built from three literal-token slots so the
    /// production is structurally incapable of matching any other span.
    NominalCombatStepName,
    NominalNegatedModifier,
    NominalQuantityModifier,
    NominalPowerToughnessModifier,
    NominalDeterminer,
    NominalPrepositional,
    NominalInfinitive,
    NominalQuantityComplement,
    /// A parameterized keyword ability's symbol-cost argument attached to
    /// its keyword-noun head in grant position (`ward {2}`) — Stage A of
    /// the `kwgrant` round.
    NominalKeywordSymbolArgument,
    /// `PredicatedQualityFrom -> From ColorWord` / `From NounPhrase` — Stage B.
    PredicatedQualityFrom,
    /// `PredicatedArgumentFrom -> PredicatedQualityFrom` — Stage B list base.
    PredicatedArgumentFromSingle,
    /// `PredicatedArgumentFrom -> PredicatedArgumentFrom Conjunction
    /// PredicatedQualityFrom` — Stage B list extension (`and` only, gated by
    /// `accepts_keyword_grant_prefix`).
    PredicatedArgumentFromExtend,
    /// `Nominal -> ExplicitPredicatedKeywordNoun PredicatedArgumentFrom` —
    /// Stage B grant nominal.
    NominalKeywordPredicatedArgument,
    /// `PredicatedQualityBare -> ColorWord | AdjectivePhrase | NounPhrase` —
    /// Stage C.
    PredicatedQualityBare,
    /// `PredicatedArgumentBare -> PredicatedQualityBare` — Stage C list base.
    PredicatedArgumentBareSingle,
    /// `PredicatedArgumentBare -> PredicatedArgumentBare Conjunction
    /// PredicatedQualityFrom` — Stage C list extension (repeated qualities
    /// keep an explicit `from`; only the first quality is bare).
    PredicatedArgumentBareExtend,
    /// `Nominal -> AtomCarriedPredicatedKeywordNoun PredicatedArgumentBare` —
    /// Stage C grant nominal.
    NominalKeywordAtomCarriedPredicatedArgument,
    NominalRelative,
    NominalReducedRecipientPassive,
    NounPhraseSetExceptionBare,
    NounPhraseSetExceptionFor,
    ReducedRecipientPassiveTheme,
    ReducedRecipientPassiveNominalAdjunct,
    NominalPostpositiveAdjective,
    NominalComparison,
    NominalDevotion,
    DevotionColorSingle,
    DevotionColorPair,
    NominalTimesClause,
    NounPhraseNominal,
    NounPhraseSubjectPronoun,
    NounPhraseObjectPronoun,
    NounPhraseReciprocal,
    NounPhraseQuantity,
    NounPhraseThisCard,
    NounPhraseFullThisCard,
    NounPhrasePossessiveThisCard,
    NounPhraseDemonstrative,
    NounPhrasePartitive,
    NounPhraseEachPartitive,
    /// `NounPhrase -> AnyDeterminer NumberNoun Of NounPhrase` — notional
    /// plural concord for `any number of <plural NounPhrase>` [`anof` round].
    /// Registered as a fallback (`add_with_cost`, `precedence: 1`) alongside
    /// the ordinary formal-singular nominal path; see the registration site
    /// for the full rationale.
    NounPhraseAnyNumberOf,
    NounPhraseCoordination,
    NounPhraseAdditiveCoordination,
    NounPhraseMinus,
    NounPhraseHalf,
    NounPhraseHalfRoundedUp,
    NounPhraseHalfRoundedDown,
    PrepositionalPhrase,
    PrepositionalObject,
    Verb,
    VerbPhraseBase,
    VerbPhraseAuxiliary,
    VerbPhraseAuxiliaryProform,
    VerbPhraseDirectObject,
    VerbPhraseIndirectObject,
    VerbPhraseAdjective,
    VerbPhrasePrepositional,
    /// The append-last `except by <PP>` exception tail on a passive
    /// predicate: `VerbPhrase -> VerbPhrase Except PrepositionalPhrase`.
    VerbPhraseExceptBy,
    VerbPhraseInfinitive,
    VerbPhraseAdverb,
    VerbPhrasePreverbAdverb,
    VerbPhraseParticle,
    /// The closed `come up heads`/`come up tails` coin-result predicate tail
    /// [CR#705.1,705.2]. Registered late (after every other rule) alongside
    /// [`VerbPhraseParticle`], its structural analogue; the pending `Come`
    /// frame (`PredicateFrame::requires_coin_result`) is what gates
    /// prediction, not a generic particle license.
    VerbPhraseCoinResult,
    VerbPhraseFrequency,
    FrequencyPhraseAdverb,
    VerbPhraseAbility,
    VerbPhraseQuotedAbility,
    VerbPhraseQuotedAbilityCoordination,
    VerbPhraseAbilityQuotedCoordination,
    VerbPhraseOracleSymbol,
    VerbPhraseSymbolSequence,
    ManaAmountSymbol,
    ManaAmountSequence,
    ManaAmountListSingle,
    ManaAmountListComma,
    ManaAmountCoordination,
    ManaAmountCoordinationOxford,
    VerbPhraseManaAmountCoordination,
    VerbPhrasePowerToughness,
    VerbPhraseQuantity,
    InfinitiveTo,
    InfinitiveNotTo,
    GerundClauseBase,
    GerundClauseSubordinateAfter,
    SimpleClauseSubject,
    /// The finite verbal quantifier float (`Two target creatures each get
    /// +2/+2 until end of turn.`): a plural/second-person subject, the
    /// dedicated `each` lexeme, and a completed finite verb phrase. The
    /// `each` lexical child is discarded and carried as
    /// `PredicateHead::distributive_each` — see `finish_predicate`.
    SimpleClauseSubjectDistributiveEach,
    SimpleClauseContractedSubject,
    SimpleClauseSubjectless,
    ClauseSimple,
    ClauseElliptical,
    ClauseCoordination,
    ClauseCoordinationComma,
    ClauseCoordinationAsyndetic,
    ClauseAdverbBefore,
    ClauseSentenceAdverbialBefore,
    ClausePrepositionalBefore,
    ClauseSubordinateBefore,
    /// `While <gerund clause>, <clause>.` — the fronted-gerund vote/action
    /// simultaneity frame [CR#701.38d]. Registered late (after every other
    /// rule, including the coin-result predicate), gated at dot 1 on exactly
    /// `Features::Subordinator(While)` so no other subordinator gains this
    /// shape and no generic fronted-gerund production is introduced.
    ClauseSubordinateGerundBefore,
    ClauseSubordinateAfterElliptical,
    ClauseSubordinateAfter,
    ClauseSubordinateAfterComma,
    ClauseSubordinateAfterInfinitive,
    ClauseExistential,
    CopularRemainderNoun,
    CopularRemainderAdjective,
    CopularRemainderPrepositional,
    CopularRemainderPowerToughness,
    CopularRemainderPrepositionalAdjunct,
    CopularRemainderAdverb,
    CopularRemainderNegated,
    CopularRemainderDistributiveEach,
    ClauseCopular,
    ClauseContractedCopular,
    /// A variable's value constraint — a modal clause whose predicate is a
    /// bare-infinitive copula over a bare numeral (`X can't be 0`). `X` is a
    /// placeholder whose value its controller chooses [CR#107.3,107.3a]; this
    /// clause restricts that chosen value. Built from a lexeme-pinned `X`
    /// subject and a lexeme-pinned numeral complement — four literal slots —
    /// so the production is structurally incapable of matching a
    /// `can't be <participle>` passive (a numeral is never a participle),
    /// which is the categorical discrimination this round is gated on.
    ClauseVariableValueConstraint,
    RelativeObject,
    RelativeObjectContractedSubject,
    RelativeSubjectContractedAuxiliary,
    RelativeSubject,
    /// The relative-clause counterpart of
    /// [`RuleTag::SimpleClauseSubjectDistributiveEach`]
    /// (`target creature cards that each have a different mana value`): a
    /// relative marker, the dedicated `each` lexeme, and a completed finite
    /// plural verb phrase.
    RelativeSubjectDistributiveEach,
    RelativeContractedCopularNoun,
    RelativeContractedCopularAdjective,
    RelativeContractedCopularPrepositional,
    /// The causative `have <object> <bare-infinitive VP>` construction
    /// (`have this creature enter as a copy of …`). The head verb's object is
    /// the causee; the bare verb phrase is its infinitival complement.
    VerbPhraseCausative,
    /// A single-conjunct exception rider (`except it isn't legendary`): the
    /// `except` marker plus one finite clause.
    ExceptionRiderSingle,
    /// A two-conjunct exception rider joined by a bare conjunction
    /// (`except A and B`).
    ExceptionRiderConjoined,
    /// An asyndetic exception-rider continuation (`…, A, B`) — a comma-joined
    /// clause with no conjunction, used for the interior members of an Oxford
    /// list.
    ExceptionRiderComma,
    /// The final Oxford member of an exception rider (`…, and C`).
    ExceptionRiderOxford,
    /// The trailing `, except <rider>` attachment on a host clause.
    ClauseExcepted,
    /// One `only <adjunct>` restriction-run member, or a `only <adjunct> and
    /// only <adjunct>` two-member run, or a comma/Oxford growth of an
    /// existing run, or the trailing attachment of a complete run onto a host
    /// clause. All these `RestrictionRun`-building and -attaching shapes share
    /// this tag; see `grammar/clause.rs` for the disambiguating arities.
    ClauseRestrictionRun,
    /// The `["only", <adjunct-or-if-clause>]` production building one
    /// restriction-run member.
    ClauseRestrictionMember,
    Sentence,
    NounOpaque,
    FrequencyPhrase,
    /// A coordinable modifier atom built from an adjective phrase.
    ModifierConjunctAdjective,
    /// A coordinable modifier atom built from a bare noun.
    ModifierConjunctNoun,
    /// A coordinable modifier atom built from a `non-` negated modifier.
    ModifierConjunctNegated,
    /// The single-atom base of an open comma-separated modifier run.
    ModifierListSingle,
    /// An asyndetic comma continuation of a modifier run (`artifact,
    /// creature`).
    ModifierListComma,
    /// A coordinated modifier closed by a bare conjunction (`white and blue`,
    /// `artifact, creature and land`) — covers the simple two-way pair and the
    /// non-Oxford list.
    CoordinatedModifierConjoined,
    /// A coordinated modifier closed by an Oxford `, and`/`, or`/`, and/or`
    /// member (`artifact, creature, and land`).
    CoordinatedModifierOxford,
    /// The nominal prepend of a coordinated modifier as one modifier slot.
    NominalCoordinatedModifier,
    /// The single-phrase base of an open noun-phrase run.
    NounPhraseListSingle,
    /// An asyndetic comma continuation of a noun-phrase run (`artifact,
    /// enchantment`) — the interior members of an Oxford head list.
    NounPhraseListComma,
    /// The final Oxford member closing a noun-phrase head list (`…, or land`).
    NounPhraseCoordinationOxford,
    // --- Coordination-consumer rules (appended after `add_coordination_rules`)
    // ---
    /// A coordinated predicative-adjective complement on an intransitive-`be`
    /// verb phrase (`are green and white`, `are green and/or white`). Consumes
    /// the closed [`Nonterminal::CoordinatedModifier`] in the
    /// adjective-complement slot; covers the non-contracted relative (`that
    /// are …`) and matrix copulars.
    VerbPhraseCoordinatedAdjective,
    /// A coordinated predicative-adjective copular complement (`it's legendary
    /// and snow`). Consumes the closed coordinated modifier as a copular
    /// remainder.
    CopularRemainderCoordinatedAdjective,
    /// A contracted relative copular with a coordinated adjective complement
    /// (`that's red or green`, `that's white or blue`).
    RelativeContractedCopularCoordinatedAdjective,
    /// A power/toughness value complement on a characteristic nominal (`base
    /// power and toughness *X/X*`). The `N/N` token sets the base
    /// characteristic; it rides the final coordinated characteristic of a
    /// `power and toughness` pair. Mirrors
    /// [`Self::NominalQuantityComplement`] for the P/T token.
    NominalPowerToughnessComplement,
}

pub(crate) struct EnglishGrammar<'source, 'catalogs> {
    source: &'source str,
    catalogs: &'catalogs Catalogs,
    start: Nonterminal,
    rules: Vec<Rule<Nonterminal, EnglishLexicalSlot>>,
    tags: Vec<RuleTag>,
    rules_by_lhs: HashMap<Nonterminal, Vec<RuleId>>,
    opacity_profile: OpacityProfile,
    self_reference: SelfReference,
}

impl<'source, 'catalogs> EnglishGrammar<'source, 'catalogs> {
    pub(crate) fn new(
        source: &'source str,
        catalogs: &'catalogs Catalogs,
        start: Nonterminal,
    ) -> Self {
        Self::with_opacity_profile(
            source,
            catalogs,
            start,
            OpacityProfile::Exact,
            SelfReference::default(),
        )
    }

    fn with_opacity_profile(
        source: &'source str,
        catalogs: &'catalogs Catalogs,
        start: Nonterminal,
        opacity_profile: OpacityProfile,
        self_reference: SelfReference,
    ) -> Self {
        let mut builder = RuleBuilder::default();
        builder.add_nominal_rules();
        builder.add_clause_rules();
        if opacity_profile != OpacityProfile::Exact {
            opacity::add_rules(&mut builder);
        }
        // Coordination inside the nominal is appended last of all so existing
        // rules keep their `RuleId`s and existing forests keep their alternative
        // indices, even under the opacity profiles.
        builder.add_coordination_rules();
        // Coordination *consumers* — new attachment points that read the landed
        // coordination nonterminals — are appended after every coordination rule
        // so they take the highest `RuleId`s in the grammar; the equal-cost
        // tiebreak then leaves every already-clean parse untouched.
        builder.add_coordination_consumer_rules();
        // Registered last of the entire grammar so every existing `RuleId`
        // keeps its numbering; append-last does not stabilize root `NodeId`
        // or same-rule alternative discovery order, so the full negative
        // gates in the `opqposs` round remain the check for that.
        builder.add_possessive_modifier_rules();
        // Registered after every other rule, including the possessive
        // modifier: the coin-result predicate's dot-1 gate is categorical
        // (the pending `Come` frame), so append order only affects tie
        // stability, not correctness.
        builder.add_coin_result_rules();
        // Registered after the coin-result predicate so every prior `RuleId`
        // stays unchanged; its own dot-1 gate (`Features::Subordinator(While)`)
        // is categorical, so append order affects only tie stability.
        builder.add_while_gerund_rules();
        builder.add_keyword_grant_rules();
        builder.add_reduced_recipient_passive_rules();
        // Append-last because this widens `NounPhrase`; its dot-1 host gate is
        // categorical, while retaining every earlier rule's stable identity.
        builder.add_set_exception_rules();
        Self {
            source,
            catalogs,
            start,
            rules: builder.rules,
            tags: builder.tags,
            rules_by_lhs: builder.rules_by_lhs,
            opacity_profile,
            self_reference,
        }
    }

    /// Matches a self-reference name spelled by `spellings` against the source
    /// tokens at `start`. Comparison is case-sensitive and by whole-token
    /// spelling. When `possessive`, the final source token must be the last
    /// name token with a trailing `'s`.
    fn match_self_reference(
        &self,
        tokens: &[Token],
        start: usize,
        spellings: &[String],
        possessive: bool,
    ) -> Option<usize> {
        if spellings.is_empty() {
            return None;
        }
        let last = spellings.len() - 1;
        for (offset, spelling) in spellings.iter().enumerate() {
            let actual = self.token_text(tokens, start + offset)?;
            let matches = if possessive && offset == last {
                actual.strip_suffix("'s") == Some(spelling.as_str())
            } else {
                actual == spelling
            };
            if !matches {
                return None;
            }
        }
        Some(start + spellings.len())
    }

    /// Whether the face's derived nickname matches the tokens at `start` and
    /// the source token there is capitalized. The full name is
    /// pre-collapsed to a single [`TokenKind::FullSelfReference`] token, so
    /// only the still-spelled nickname can collide with an ordinary lexical
    /// reading of a `Word` token. A non-self-reference reading of such a
    /// token renders it lowercase — a vocabulary lemma, the `the`
    /// determiner, a lowercase keyword-ability spelling — which corrupts
    /// the printed name, so callers dispreference those readings to let the
    /// case-preserving self-reference win. When no nickname competes (the
    /// common case, and every anonymous parse) this is `false` and ordinary
    /// readings pay nothing.
    fn nickname_lowercasing_collision(&self, tokens: &[Token], start: usize) -> bool {
        let capitalized = self
            .token_text(tokens, start)
            .and_then(|surface| surface.as_bytes().first().copied())
            .is_some_and(|byte| byte.is_ascii_uppercase());
        capitalized
            && self.self_reference.nickname().is_some_and(|spellings| {
                self.match_self_reference(tokens, start, spellings, false)
                    .is_some()
                    || self
                        .match_self_reference(tokens, start, spellings, true)
                        .is_some()
            })
    }

    fn token_text<'tokens>(&self, tokens: &'tokens [Token], start: usize) -> Option<&'source str> {
        tokens.get(start)?.span.text(self.source)
    }

    fn one_token_match(&self, tokens: &[Token], start: usize, expected: &str) -> Option<usize> {
        self.token_text(tokens, start)?
            .eq_ignore_ascii_case(expected)
            .then_some(start + 1)
    }

    fn words_match(&self, tokens: &[Token], start: usize, expected: &[&str]) -> Option<usize> {
        let end = start.checked_add(expected.len())?;
        let actual = tokens.get(start..end)?;
        actual
            .iter()
            .zip(expected)
            .all(|(token, expected)| {
                token
                    .span
                    .text(self.source)
                    .is_some_and(|surface| surface.eq_ignore_ascii_case(expected))
            })
            .then_some(end)
    }

    fn catalog_end(tokens: &[Token], start: usize, byte_length: usize) -> Option<usize> {
        let byte_start = tokens.get(start)?.span.start;
        let byte_end = byte_start.checked_add(byte_length)?;
        tokens[start..]
            .iter()
            .position(|token| token.span.end == byte_end)
            .map(|offset| start + offset + 1)
    }

    fn word_matches(
        &self,
        tokens: &[Token],
        start: usize,
        slot: LexicalSlot,
    ) -> Vec<LexicalMatch<Features, MeaningKey>> {
        let Some(token) = tokens.get(start) else {
            return Vec::new();
        };
        if token.kind != TokenKind::Word {
            return Vec::new();
        }
        let Some(surface) = token.span.text(self.source) else {
            return Vec::new();
        };
        let gated = matches!(
            slot,
            LexicalSlot::Noun(_) | LexicalSlot::Adjective | LexicalSlot::Verb(_)
        );
        let capitalized = surface
            .as_bytes()
            .first()
            .is_some_and(u8::is_ascii_uppercase);
        let sentence_initial = Self::is_sentence_initial(tokens, start);
        if gated && !sentence_initial && capitalized {
            return Vec::new();
        }
        // A vocabulary lemma is lowercase, so reading a capitalized token as one
        // lowercases it. When that token is also the face's nickname (e.g. the
        // legend `Carnage`, whose name coincides with a common noun), that
        // lowercasing reading corrupts the printed name; dispreference it so the
        // case-preserving self-reference wins. Gated on an actual nickname
        // collision, so ordinary sentence-initial words (which have no competing
        // self-reference) keep their normal cost.
        let dispreference = if self.nickname_lowercasing_collision(tokens, start) {
            ParseCost {
                reading_dispreference: 3,
                ..ParseCost::default()
            }
        } else {
            ParseCost::default()
        };
        Vocabulary::new()
            .matches(surface, slot)
            .into_iter()
            .flat_map(|word| lexical_word_matches(word, start + 1))
            .map(|mut lexical_match| {
                lexical_match.local_cost += dispreference;
                lexical_match
            })
            .collect()
    }

    fn catalog_matches(
        &self,
        tokens: &[Token],
        start: usize,
        slot: CatalogSlot,
    ) -> Vec<LexicalMatch<Features, MeaningKey>> {
        let Some(token) = tokens.get(start) else {
            return Vec::new();
        };
        let Some(suffix) = self.source.get(token.span.start..) else {
            return Vec::new();
        };
        let mut catalog_matches = self.catalogs.matches(suffix, slot);
        if Self::is_sentence_initial(tokens, start)
            && suffix
                .as_bytes()
                .first()
                .is_some_and(u8::is_ascii_uppercase)
        {
            let mut lowercased = suffix.to_owned();
            let token_len = token.span.end.saturating_sub(token.span.start);
            if let Some(initial) = lowercased.get_mut(..token_len) {
                initial.make_ascii_lowercase();
            }
            catalog_matches.extend(self.catalogs.matches(&lowercased, slot));
        }
        let collision = self.nickname_lowercasing_collision(tokens, start);
        catalog_matches
            .into_iter()
            .flat_map(|catalog_match| {
                let Some(end) = Self::catalog_end(tokens, start, catalog_match.length) else {
                    return Vec::new();
                };
                // A keyword-ability (or supertype/card-type) atom renders as a
                // lowercase noun (e.g. `prowl`), so reading a capitalized nickname
                // (`Prowl`) as one lowercases the name. Dispreference such readings
                // when the nickname collides here so the case-preserving
                // self-reference wins; type/subtype words render with their
                // canonical case and are untouched.
                let lowercasing = matches!(
                    &catalog_match.value,
                    CatalogValue::Atom(atom) if atom.renders_lowercase_noun()
                );
                let penalty = if collision && lowercasing {
                    ParseCost {
                        reading_dispreference: 3,
                        ..ParseCost::default()
                    }
                } else {
                    ParseCost::default()
                };
                let mut produced = match catalog_match.value {
                    CatalogValue::Word(word) => lexical_word_matches(word, end),
                    CatalogValue::Atom(atom) => {
                        if slot == CatalogSlot::KeywordAbilityNoun {
                            lexical_word_matches(
                                WordMatch::Noun(NounInstance::Mass(Noun::Catalog(atom))),
                                end,
                            )
                        } else {
                            vec![LexicalMatch {
                                end,
                                features: Features::None,
                                meaning: MeaningKey::Catalog(atom),
                                local_cost: ParseCost::default(),
                            }]
                        }
                    }
                };
                for candidate in &mut produced {
                    candidate.local_cost += penalty;
                }
                produced
            })
            .fold(Vec::new(), |mut matches, candidate| {
                if !matches.contains(&candidate) {
                    matches.push(candidate);
                }
                matches
            })
    }

    fn is_sentence_initial(tokens: &[Token], start: usize) -> bool {
        start == 0
            || tokens.get(start.wrapping_sub(1)).is_some_and(|token| {
                matches!(
                    token.kind,
                    TokenKind::Newline
                        | TokenKind::Bullet
                        | TokenKind::Punctuation(
                            Punctuation::Period | Punctuation::Exclamation | Punctuation::Question
                        )
                )
            })
    }

    /// Scans a single-token `non-` negation as sub-word morphology. Strips a
    /// leading `non` or `non-` and, only when the residue resolves as a known
    /// modifier base, emits one negated-modifier match per base reading. When
    /// the residue does not resolve — `none`, `nonetheless`, any narrative word
    /// — nothing fires and the token falls through to the existing paths, so
    /// those spellings stay intact. Both spellings resolve to the same key;
    /// the render side re-derives the glyph from the base's capitalization.
    fn scan_negated_modifier(
        &self,
        tokens: &[Token],
        start: usize,
    ) -> Vec<LexicalMatch<Features, MeaningKey>> {
        let Some(token) = tokens.get(start) else {
            return Vec::new();
        };
        if token.kind != TokenKind::Word {
            return Vec::new();
        }
        let Some(surface) = token.span.text(self.source) else {
            return Vec::new();
        };
        // Case-insensitive `non` prefix so sentence-initial `Noncreature` and
        // mid-sentence `nonland` both strip; the residue keeps its own case.
        if !surface
            .get(..3)
            .is_some_and(|prefix| prefix.eq_ignore_ascii_case("non"))
        {
            return Vec::new();
        }
        let rest = &surface[3..];
        let residue = rest.strip_prefix('-').unwrap_or(rest);
        if residue.is_empty() {
            return Vec::new();
        }
        self.resolve_negation_bases(residue)
            .into_iter()
            .map(|base| LexicalMatch {
                end: start + 1,
                features: Features::None,
                meaning: MeaningKey::NegatedModifier(NegatedModifierKey { base }),
                local_cost: ParseCost::default(),
            })
            .fold(Vec::new(), |mut matches, candidate| {
                if !matches.contains(&candidate) {
                    matches.push(candidate);
                }
                matches
            })
    }

    /// Resolves a stripped negation residue to the modifier bases it names,
    /// mirroring the adjective and noun scan slots but over the whole residue
    /// string. A catalog match must consume the entire residue (its length must
    /// equal the residue's) so `nonlander` — residue `lander`, a partial `land`
    /// prefix — does not spuriously fire.
    fn resolve_negation_bases(&self, residue: &str) -> Vec<NegatedBase> {
        let mut bases = Vec::new();
        let mut push = |base: NegatedBase| {
            if !bases.contains(&base) {
                bases.push(base);
            }
        };
        // Vocabulary lemmas are lowercase; a capitalized residue (`non-Phyrexian`,
        // `non-Human`) is the case-preserving catalog subtype reading, never the
        // common-word one — the same guard `word_matches` applies to a
        // capitalized surface. The catalog lookups below self-guard by case
        // policy, so they stay unconditional.
        let residue_is_capitalized = residue
            .as_bytes()
            .first()
            .is_some_and(u8::is_ascii_uppercase);
        if !residue_is_capitalized {
            let vocabulary = Vocabulary::new();
            for word in vocabulary.matches(residue, LexicalSlot::Adjective) {
                if let WordMatch::Adjective(adjective) = word {
                    push(NegatedBase::Adjective(adjective));
                }
            }
            for word in vocabulary.matches(residue, LexicalSlot::Noun(NounUsage::Either)) {
                if let WordMatch::Noun(noun) = word {
                    push(NegatedBase::Noun(noun));
                }
            }
        }
        for catalog_match in self.catalogs.matches(residue, CatalogSlot::Adjective) {
            if catalog_match.length != residue.len() {
                continue;
            }
            if let CatalogValue::Word(WordMatch::Adjective(adjective)) = catalog_match.value {
                push(NegatedBase::Adjective(adjective));
            }
        }
        for catalog_match in self
            .catalogs
            .matches(residue, CatalogSlot::Noun(NounUsage::Either))
        {
            if catalog_match.length != residue.len() {
                continue;
            }
            if let CatalogValue::Word(WordMatch::Noun(noun)) = catalog_match.value {
                push(NegatedBase::Noun(noun));
            }
        }
        bases
    }
}

impl Grammar for EnglishGrammar<'_, '_> {
    type Nonterminal = Nonterminal;
    type LexicalSlot = EnglishLexicalSlot;
    type Token = Token;
    type Features = Features;
    type Meaning = MeaningKey;

    fn start(&self) -> Self::Nonterminal {
        self.start
    }

    fn rules(&self) -> &[Rule<Self::Nonterminal, Self::LexicalSlot>] {
        &self.rules
    }

    fn rules_for(&self, lhs: Self::Nonterminal) -> &[RuleId] {
        self.rules_by_lhs.get(&lhs).map_or(&[], Vec::as_slice)
    }

    #[allow(
        clippy::too_many_lines,
        reason = "scan logic is intentionally exhaustive"
    )]
    fn scan(
        &self,
        slot: Self::LexicalSlot,
        tokens: &[Self::Token],
        start: usize,
    ) -> Vec<LexicalMatch<Self::Features, Self::Meaning>> {
        match slot {
            EnglishLexicalSlot::Number(notation) => {
                let Some(surface) = self.token_text(tokens, start) else {
                    return Vec::new();
                };
                parse_notation(notation, surface, Self::is_sentence_initial(tokens, start))
                    .map(|value| LexicalMatch {
                        end: start + 1,
                        features: Features::Number { is_one: value == 1 },
                        meaning: MeaningKey::Number(NumberKey { value, notation }),
                        local_cost: ParseCost {
                            reading_dispreference: u32::from(notation == Numeral::Ordinal),
                            precedence: u32::from(notation == Numeral::Roman && surface == "X"),
                            ..ParseCost::default()
                        },
                    })
                    .into_iter()
                    .collect()
            }
            EnglishLexicalSlot::Noun(usage) => {
                let mut matches = self.word_matches(tokens, start, LexicalSlot::Noun(usage));
                matches.extend(self.catalog_matches(tokens, start, CatalogSlot::Noun(usage)));
                if usage != NounUsage::Count {
                    matches.extend(self.catalog_matches(
                        tokens,
                        start,
                        CatalogSlot::KeywordAbilityNoun,
                    ));
                }
                if usage != NounUsage::Mass
                    && let Some(surface) = self.token_text(tokens, start)
                    && let Some(sides) = surface.strip_prefix('d')
                    && let Ok(value) = Numeral::Arabic(false).parse(sides)
                    && value > 0
                {
                    matches.extend(lexical_word_matches(
                        WordMatch::Noun(NounInstance::Singular(Noun::Die(
                            crate::syntax::NumberLiteral {
                                value,
                                numeral: Numeral::Arabic(false),
                            },
                        ))),
                        start + 1,
                    ));
                }
                matches
            }
            EnglishLexicalSlot::SymbolArgumentKeywordNoun => {
                self.catalog_matches(tokens, start, CatalogSlot::KeywordAbilityNoun)
            }
            EnglishLexicalSlot::FromWord => self
                .one_token_match(tokens, start, "from")
                .map(|end| LexicalMatch {
                    end,
                    features: Features::None,
                    meaning: MeaningKey::Preposition(Preposition::From),
                    local_cost: ParseCost::default(),
                })
                .into_iter()
                .collect(),
            EnglishLexicalSlot::ExplicitPredicatedKeywordNoun => {
                let matches = self.catalog_matches(tokens, start, CatalogSlot::KeywordAbilityNoun);
                let carries_from_max_end = matches
                    .iter()
                    .filter(|candidate| {
                        matches!(
                            &candidate.meaning,
                            MeaningKey::Noun(NounInstance::Mass(Noun::Catalog(atom)))
                                if keyword_atom_carries_from(atom)
                        )
                    })
                    .map(|candidate| candidate.end)
                    .max();
                matches
                    .into_iter()
                    .filter(|candidate| {
                        carries_from_max_end.map_or(true, |max_end| candidate.end >= max_end)
                    })
                    .collect()
            }
            EnglishLexicalSlot::AtomCarriedPredicatedKeywordNoun => self
                .catalog_matches(tokens, start, CatalogSlot::KeywordAbilityNoun)
                .into_iter()
                .filter(|candidate| {
                    matches!(
                        &candidate.meaning,
                        MeaningKey::Noun(NounInstance::Mass(Noun::Catalog(atom)))
                            if keyword_atom_carries_from(atom)
                    )
                })
                .collect(),
            EnglishLexicalSlot::Verb(verb_slot) => self.scan_verb(tokens, start, verb_slot),
            EnglishLexicalSlot::ReducedRecipientPassiveParticiple => self
                .scan_verb(tokens, start, VerbSlot::PastParticiple)
                .into_iter()
                .filter(|candidate| {
                    matches!(
                        candidate.features,
                        Features::Verb { frame, .. } if frame.is_recipient_passive()
                    )
                })
                .collect(),
            EnglishLexicalSlot::Adjective => {
                let mut matches = self.word_matches(tokens, start, LexicalSlot::Adjective);
                matches.extend(self.catalog_matches(tokens, start, CatalogSlot::Adjective));
                let gated = matches!(
                    tokens.get(start),
                    Some(token) if token.kind == TokenKind::Word
                );
                if gated && let Some(surface) = self.token_text(tokens, start) {
                    let capitalized = surface
                        .as_bytes()
                        .first()
                        .is_some_and(u8::is_ascii_uppercase);
                    let sentence_initial = Self::is_sentence_initial(tokens, start);
                    if (!capitalized || sentence_initial)
                        && let Ok(value) = Numeral::Ordinal.parse(surface)
                        && value > 0
                    {
                        matches.extend(lexical_word_matches(
                            crate::word::WordMatch::Adjective(Adjective::Ordinal(value)),
                            start + 1,
                        ));
                    }
                }
                matches
            }
            EnglishLexicalSlot::ColorWord => self
                .token_text(tokens, start)
                .and_then(crate::word::ColorWord::from_surface)
                .map(|color| LexicalMatch {
                    end: start + 1,
                    features: Features::None,
                    meaning: MeaningKey::Adjective(Adjective::Color(color)),
                    local_cost: ParseCost::default(),
                })
                .into_iter()
                .collect(),
            EnglishLexicalSlot::DevotionValue => self
                .token_text(tokens, start)
                .and_then(|surface| {
                    crate::catalog::rules_value_noun_prefix(surface)
                        .filter(|&(length, _)| length == surface.len())
                        .map(|(_, atom)| atom)
                })
                .map(|atom| LexicalMatch {
                    end: start + 1,
                    features: Features::Noun {
                        form: NounForm::Singular,
                        initial_sound: InitialSound::Consonant,
                        adjunct: None,
                        opaque: false,
                        recipient_passive_theme: false,
                    },
                    meaning: MeaningKey::Catalog(atom),
                    local_cost: ParseCost::default(),
                })
                .into_iter()
                .collect(),
            EnglishLexicalSlot::TimesNoun => self
                .one_token_match(tokens, start, "times")
                .map(|end| LexicalMatch {
                    end,
                    features: Features::Noun {
                        form: NounForm::Plural,
                        initial_sound: InitialSound::Consonant,
                        adjunct: None,
                        opaque: false,
                        recipient_passive_theme: false,
                    },
                    meaning: MeaningKey::Noun(NounInstance::Plural(Noun::Word(Vocab::Time))),
                    local_cost: ParseCost::default(),
                })
                .into_iter()
                .collect(),
            EnglishLexicalSlot::NumberNoun => self
                .one_token_match(tokens, start, "number")
                .map(|end| LexicalMatch {
                    end,
                    features: Features::Noun {
                        form: NounForm::Singular,
                        initial_sound: InitialSound::Consonant,
                        adjunct: None,
                        opaque: false,
                        recipient_passive_theme: false,
                    },
                    meaning: MeaningKey::Noun(NounInstance::Singular(Noun::Word(Vocab::Number))),
                    local_cost: ParseCost::default(),
                })
                .into_iter()
                .collect(),
            EnglishLexicalSlot::CombatStepDeclare => self
                .one_token_match(tokens, start, "declare")
                .map(|end| literal_match(end, LiteralKey::CombatStepDeclare))
                .into_iter()
                .collect(),
            EnglishLexicalSlot::CombatStepParticipants => {
                let matches_surface = self.token_text(tokens, start).is_some_and(|surface| {
                    surface.eq_ignore_ascii_case("attackers")
                        || surface.eq_ignore_ascii_case("blockers")
                });
                if !matches_surface {
                    return Vec::new();
                }
                self.word_matches(tokens, start, LexicalSlot::Noun(NounUsage::Count))
                    .into_iter()
                    .filter(|lexical_match| {
                        matches!(
                            lexical_match.meaning,
                            MeaningKey::Noun(NounInstance::Plural(_))
                        )
                    })
                    .collect()
            }
            EnglishLexicalSlot::CombatStepHead => {
                if self.one_token_match(tokens, start, "step").is_none() {
                    return Vec::new();
                }
                self.word_matches(tokens, start, LexicalSlot::Noun(NounUsage::Count))
                    .into_iter()
                    .filter(|lexical_match| {
                        matches!(
                            lexical_match.meaning,
                            MeaningKey::Noun(NounInstance::Singular(_))
                        )
                    })
                    .collect()
            }
            EnglishLexicalSlot::PreverbAdverb => {
                if self.one_token_match(tokens, start, "next").is_none() {
                    return Vec::new();
                }
                self.word_matches(tokens, start, LexicalSlot::Adverb)
            }
            EnglishLexicalSlot::NegatedModifier => self.scan_negated_modifier(tokens, start),
            EnglishLexicalSlot::Adverb => self.word_matches(tokens, start, LexicalSlot::Adverb),
            EnglishLexicalSlot::SentenceAdverbial => {
                self.word_matches(tokens, start, LexicalSlot::SentenceAdverbial)
            }
            EnglishLexicalSlot::VerbParticle(particle) => self
                .one_token_match(
                    tokens,
                    start,
                    match particle {
                        VerbParticle::In => "in",
                        VerbParticle::Out => "out",
                    },
                )
                .map(|end| LexicalMatch {
                    end,
                    features: Features::VerbParticle(particle),
                    meaning: MeaningKey::VerbParticle(particle),
                    local_cost: ParseCost::default(),
                })
                .into_iter()
                .collect(),
            // The closed two-word coin-result surface [CR#705.1,705.2]:
            // scanned as an exact literal `up heads`/`up tails`, never as a
            // general noun/adjective lookup for `heads`/`tails`.
            EnglishLexicalSlot::CoinResult(side) => self
                .words_match(
                    tokens,
                    start,
                    match side {
                        crate::syntax::CoinSide::Heads => &["up", "heads"],
                        crate::syntax::CoinSide::Tails => &["up", "tails"],
                    },
                )
                .map(|end| LexicalMatch {
                    end,
                    features: Features::CoinResult(side),
                    meaning: MeaningKey::CoinResult(side),
                    local_cost: ParseCost::default(),
                })
                .into_iter()
                .collect(),
            EnglishLexicalSlot::Frequency => self.scan_frequency(tokens, start),
            EnglishLexicalSlot::Pronoun(case) => {
                self.word_matches(tokens, start, LexicalSlot::Pronoun(case))
            }
            EnglishLexicalSlot::Auxiliary => {
                self.word_matches(tokens, start, LexicalSlot::Auxiliary)
            }
            EnglishLexicalSlot::Copula => self
                .word_matches(tokens, start, LexicalSlot::Auxiliary)
                .into_iter()
                .filter_map(|mut candidate| {
                    let MeaningKey::Auxiliary(auxiliary) = candidate.meaning else {
                        return None;
                    };
                    let agreement = copula_agreement(auxiliary)?;
                    candidate.meaning = MeaningKey::Auxiliary(auxiliary);
                    candidate.features = Features::Copula(agreement);
                    Some(candidate)
                })
                .collect(),
            EnglishLexicalSlot::SubjectAuxiliary => self.scan_subject_auxiliary(tokens, start),
            EnglishLexicalSlot::AbilityItem => {
                self.catalog_matches(tokens, start, CatalogSlot::AbilityItem)
            }
            EnglishLexicalSlot::AbilityWord => {
                self.catalog_matches(tokens, start, CatalogSlot::AbilityWord)
            }
            EnglishLexicalSlot::Determiner => self.scan_determiner(tokens, start),
            EnglishLexicalSlot::Demonstrative => self.scan_demonstrative(tokens, start),
            EnglishLexicalSlot::DeterminerTarget => self
                .one_token_match(tokens, start, "target")
                .map(|end| LexicalMatch {
                    end,
                    features: Features::Determiner {
                        cardinality: Cardinality::SingularCount,
                        article: None,
                        set_exception_host: false,
                    },
                    meaning: MeaningKey::Literal(LiteralKey::Target),
                    local_cost: ParseCost::default(),
                })
                .into_iter()
                .collect(),
            EnglishLexicalSlot::QuantityAtLeast => self.scan_at_least_quantity(tokens, start),
            EnglishLexicalSlot::QuantityOr => self.scan_or_quantity(tokens, start),
            EnglishLexicalSlot::QuantityX => self
                .one_token_match(tokens, start, "X")
                .map(|end| quantity_match(end, QuantityKey::X))
                .into_iter()
                .collect(),
            EnglishLexicalSlot::QuantityBoth => self
                .one_token_match(tokens, start, "both")
                .map(|end| quantity_match(end, QuantityKey::Both))
                .into_iter()
                .collect(),
            EnglishLexicalSlot::QuantityThatMany => self
                .words_match(tokens, start, &["that", "many"])
                .map(|end| quantity_match(end, QuantityKey::ThatMany))
                .into_iter()
                .collect(),
            EnglishLexicalSlot::QuantityThatMuch => self
                .words_match(tokens, start, &["that", "much"])
                .map(|end| quantity_match(end, QuantityKey::ThatMuch))
                .into_iter()
                .collect(),
            EnglishLexicalSlot::QuantityBound(kind) => {
                self.scan_bounded_quantity(tokens, start, kind)
            }
            EnglishLexicalSlot::Than => self
                .one_token_match(tokens, start, "than")
                .map(|end| literal_match(end, LiteralKey::Than))
                .into_iter()
                .collect(),
            EnglishLexicalSlot::OrEqualTo => self
                .words_match(tokens, start, &["or", "equal", "to"])
                .map(|end| literal_match(end, LiteralKey::OrEqualTo))
                .into_iter()
                .collect(),
            EnglishLexicalSlot::RelativeMarker => {
                [("who", RelativeMarker::Who), ("that", RelativeMarker::That)]
                    .into_iter()
                    .filter_map(|(surface, marker)| {
                        self.one_token_match(tokens, start, surface)
                            .map(|end| LexicalMatch {
                                end,
                                features: Features::None,
                                meaning: MeaningKey::RelativeMarker(marker),
                                local_cost: ParseCost::default(),
                            })
                    })
                    .collect()
            }
            EnglishLexicalSlot::Face => self
                .one_token_match(tokens, start, "face")
                .map(|end| literal_match(end, LiteralKey::Face))
                .into_iter()
                .collect(),
            EnglishLexicalSlot::Up => self
                .one_token_match(tokens, start, "up")
                .map(|end| literal_match(end, LiteralKey::Up))
                .into_iter()
                .collect(),
            EnglishLexicalSlot::Down => self
                .one_token_match(tokens, start, "down")
                .map(|end| literal_match(end, LiteralKey::Down))
                .into_iter()
                .collect(),
            EnglishLexicalSlot::Not => self
                .one_token_match(tokens, start, "not")
                .map(|end| literal_match(end, LiteralKey::Not))
                .into_iter()
                .collect(),
            EnglishLexicalSlot::Minus => self
                .one_token_match(tokens, start, "minus")
                .map(|end| literal_match(end, LiteralKey::Minus))
                .into_iter()
                .collect(),
            EnglishLexicalSlot::Half => self
                .one_token_match(tokens, start, "half")
                .map(|end| literal_match(end, LiteralKey::Half))
                .into_iter()
                .collect(),
            EnglishLexicalSlot::Rounded => self
                .one_token_match(tokens, start, "rounded")
                .map(|end| literal_match(end, LiteralKey::Rounded))
                .into_iter()
                .collect(),
            EnglishLexicalSlot::To => self
                .one_token_match(tokens, start, "to")
                .map(|end| literal_match(end, LiteralKey::To))
                .into_iter()
                .collect(),
            EnglishLexicalSlot::Of => self
                .one_token_match(tokens, start, "of")
                .map(|end| LexicalMatch {
                    end,
                    features: Features::Preposition(Preposition::Of),
                    meaning: MeaningKey::Preposition(Preposition::Of),
                    local_cost: ParseCost::default(),
                })
                .into_iter()
                .collect(),
            EnglishLexicalSlot::ForWord => self
                .one_token_match(tokens, start, "for")
                .map(|end| LexicalMatch {
                    end,
                    features: Features::Preposition(Preposition::For),
                    meaning: MeaningKey::Preposition(Preposition::For),
                    local_cost: ParseCost::default(),
                })
                .into_iter()
                .collect(),
            EnglishLexicalSlot::EachDeterminer => self
                .one_token_match(tokens, start, "each")
                .map(|end| LexicalMatch {
                    end,
                    features: Features::Determiner {
                        cardinality: Cardinality::SingularCount,
                        article: None,
                        set_exception_host: true,
                    },
                    meaning: MeaningKey::Determiner(DeterminerKey::Each),
                    local_cost: ParseCost::default(),
                })
                .into_iter()
                .collect(),
            EnglishLexicalSlot::AnyDeterminer => self
                .one_token_match(tokens, start, "any")
                .map(|end| LexicalMatch {
                    end,
                    features: Features::Determiner {
                        cardinality: Cardinality::Unconstrained,
                        article: None,
                        set_exception_host: false,
                    },
                    meaning: MeaningKey::Determiner(DeterminerKey::Any),
                    local_cost: ParseCost::default(),
                })
                .into_iter()
                .collect(),
            EnglishLexicalSlot::Reciprocal => self
                .words_match(tokens, start, &["each", "other"])
                .map(|end| pronoun_match(end, Pronoun::EachOther, PronounCase::Object))
                .into_iter()
                .collect(),
            EnglishLexicalSlot::ThisCard => self
                .self_reference
                .nickname()
                .and_then(|spellings| self.match_self_reference(tokens, start, spellings, false))
                .map(|end| this_card_matches(end, ThisCardForm::AbbreviatedName))
                .unwrap_or_default(),
            EnglishLexicalSlot::FullThisCard => tokens
                .get(start)
                .filter(|token| token.kind == TokenKind::FullSelfReference)
                .map(|_| this_card_matches(start + 1, ThisCardForm::FullName))
                .unwrap_or_default(),
            EnglishLexicalSlot::PossessiveThisCard => {
                // The full name is pre-collapsed to one token, so its possessive
                // is that token followed by a separate `'s`; the nickname is
                // still spelled out, so its possessive is matched as a sequence.
                let mut matches = Vec::new();
                if tokens
                    .get(start)
                    .is_some_and(|token| token.kind == TokenKind::FullSelfReference)
                    && let Some(end) = self.one_token_match(tokens, start + 1, "'s")
                {
                    matches.push(possessive_this_card_match(end, ThisCardForm::FullName));
                }
                if let Some(spellings) = self.self_reference.nickname()
                    && let Some(end) = self.match_self_reference(tokens, start, spellings, true)
                {
                    matches.push(possessive_this_card_match(
                        end,
                        ThisCardForm::AbbreviatedName,
                    ));
                }
                matches
            }
            EnglishLexicalSlot::Preposition => self.scan_preposition(tokens, start),
            EnglishLexicalSlot::Existential => self.scan_existential(tokens, start),
            EnglishLexicalSlot::QuotedAbility => Self::scan_quoted_ability(tokens, start),
            slot @ (EnglishLexicalSlot::PossessiveNoun
            | EnglishLexicalSlot::OracleSymbol
            | EnglishLexicalSlot::SymbolSequence
            | EnglishLexicalSlot::PowerToughness
            | EnglishLexicalSlot::Punctuation(_)
            | EnglishLexicalSlot::Subordinator
            | EnglishLexicalSlot::RatherThan
            | EnglishLexicalSlot::Conjunction
            | EnglishLexicalSlot::Plus
            | EnglishLexicalSlot::Except) => self.scan_clause_lexical(slot, tokens, start),
            EnglishLexicalSlot::Opaque(slot) if self.opacity_profile != OpacityProfile::Exact => {
                let already_known = self.has_known_word(tokens, start);
                if already_known {
                    Vec::new()
                } else {
                    let mut matches = opacity::scan_opaque(self.source, tokens, start, slot);
                    matches.retain(|candidate| {
                        !((start + 1)..candidate.end)
                            .any(|index| self.has_known_word(tokens, index))
                    });
                    matches
                }
            }
            EnglishLexicalSlot::Opaque(_) => Vec::new(),
        }
    }

    fn reduce(
        &self,
        rule: RuleId,
        children: &[Child<'_, Self>],
    ) -> Option<Reduction<Self::Features>> {
        reduce(self.tags.get(rule.index()).copied()?, children)
    }

    fn accepts_prefix(
        &self,
        rule: RuleId,
        completed_children: usize,
        latest_child: &Self::Features,
    ) -> bool {
        self.tags.get(rule.index()).copied().is_some_and(|tag| {
            accepts_possessive_modifier_prefix(tag, completed_children, latest_child)
                && accepts_keyword_grant_prefix(tag, completed_children, latest_child)
                && accepts_set_exception_prefix(tag, completed_children, latest_child)
                && clause::accepts_predicate_prefix(tag, completed_children, latest_child)
        })
    }

    fn state_limit(&self) -> Option<usize> {
        (self.opacity_profile == OpacityProfile::Nouns).then_some(OPACITY_STATE_LIMIT)
    }
}

impl EnglishGrammar<'_, '_> {
    fn scan_verb(
        &self,
        tokens: &[Token],
        start: usize,
        verb_slot: VerbSlot,
    ) -> Vec<LexicalMatch<Features, MeaningKey>> {
        let mut matches = self.word_matches(tokens, start, LexicalSlot::Verb(verb_slot));
        matches.extend(self.catalog_matches(tokens, start, CatalogSlot::Verb(verb_slot)));
        // A multi-word keyword action spells one rules-defined action; the shorter
        // verb readings that start at the same token — the vocabulary verb, or the
        // single-word keyword action whose spelling the phrase's head coincides with
        // — are decompositions of it and must lose **by cost**, not by scan order.
        // Vocabulary and catalog matches are concatenated with no cross-source dedup
        // or cost, so an equal-cost tie here would fall to interning order
        // (`alternative_index`/`NodeId`, invariant-audit §A.1).
        //
        // The test is span length, not identity: a verb match covering more than one
        // token can only be a multi-word catalog action — `word_matches` is
        // single-token by construction and `CatalogSlot::Verb` reaches only
        // `action_matches`.
        let longest = matches.iter().map(|item| item.end).max().unwrap_or(start);
        if longest > start + 1 {
            for item in &mut matches {
                if item.end < longest {
                    item.local_cost.precedence += 1;
                }
            }
        }
        matches
    }

    fn scan_clause_lexical(
        &self,
        slot: EnglishLexicalSlot,
        tokens: &[Token],
        start: usize,
    ) -> Vec<LexicalMatch<Features, MeaningKey>> {
        match slot {
            EnglishLexicalSlot::PossessiveNoun => self.scan_possessive_noun(tokens, start),
            EnglishLexicalSlot::OracleSymbol => self
                .magic_match(tokens, start, TokenKind::OracleSymbol)
                .and_then(|(surface, end)| {
                    OracleSymbol::new(surface).map(|symbol| LexicalMatch {
                        end,
                        features: Features::None,
                        meaning: MeaningKey::OracleSymbol(symbol),
                        local_cost: ParseCost::default(),
                    })
                })
                .into_iter()
                .collect(),
            EnglishLexicalSlot::SymbolSequence => self
                .magic_match(tokens, start, TokenKind::SymbolSequence)
                .and_then(|(surface, end)| {
                    parse_symbol_sequence(surface).map(|symbols| LexicalMatch {
                        end,
                        features: Features::None,
                        meaning: MeaningKey::SymbolSequence(symbols),
                        local_cost: ParseCost::default(),
                    })
                })
                .into_iter()
                .collect(),
            EnglishLexicalSlot::PowerToughness => self
                .magic_match(tokens, start, TokenKind::PowerToughness)
                .and_then(|(surface, end)| {
                    parse_power_toughness(surface).map(|power_toughness| LexicalMatch {
                        end,
                        features: Features::PowerToughness {
                            initial_sound: power_toughness.initial_sound(),
                        },
                        meaning: MeaningKey::PowerToughness(power_toughness),
                        local_cost: ParseCost::default(),
                    })
                })
                .into_iter()
                .collect(),
            EnglishLexicalSlot::Punctuation(expected) => tokens
                .get(start)
                .filter(|token| token.kind == TokenKind::Punctuation(expected))
                .map(|_| LexicalMatch {
                    end: start + 1,
                    features: Features::None,
                    meaning: MeaningKey::Punctuation(expected),
                    local_cost: ParseCost::default(),
                })
                .into_iter()
                .collect(),
            EnglishLexicalSlot::Subordinator => self.scan_subordinators(tokens, start),
            EnglishLexicalSlot::RatherThan => self
                .words_match(tokens, start, &["rather", "than"])
                .map(|end| LexicalMatch {
                    end,
                    features: Features::None,
                    meaning: MeaningKey::Subordinator(crate::syntax::Subordinator::RatherThan),
                    local_cost: ParseCost::default(),
                })
                .into_iter()
                .collect(),
            EnglishLexicalSlot::Conjunction => self.scan_conjunction(tokens, start),
            EnglishLexicalSlot::Plus => self
                .one_token_match(tokens, start, "plus")
                .map(|end| literal_match(end, LiteralKey::Plus))
                .into_iter()
                .collect(),
            EnglishLexicalSlot::Except => self
                .one_token_match(tokens, start, "except")
                .map(|end| literal_match(end, LiteralKey::Except))
                .into_iter()
                .collect(),
            _ => Vec::new(),
        }
    }

    /// Literal lexemes the grammar scans as whole `Word` tokens but that no
    /// vocabulary, catalog, determiner, preposition, conjunction, or
    /// subordinator scanner would report. `has_known_word` must know every
    /// one of them: the opacity gate's invariant is "never opacify a word
    /// the lexicon already knows", and a literal swallowed as an opaque
    /// noun can head a nominal that captures real nouns as its modifiers
    /// and completes a sentence that should stay recovered (round
    /// `pluralposs`, `except`/`not`).
    ///
    /// Adding a new literal-scanned slot means adding its surface here. The
    /// compiler cannot yet force that — see the derivation residue in
    /// `litaudit-plan.md` §2.
    // Deviation from plan §1.2 (round litaudit, recorded in
    // litaudit-mechanic-report.md): the plan's "same class, no witness
    // today" defensive additions (`both`, `half`, `rounded`, `out`,
    // `rather`, `there's`, `he's`/`she's`/`they're`/`they've`/`you've`,
    // `'s`) are NOT measured zero-cost. `out` breaks
    // `directional_particle_requires_a_licensed_verb_pair`
    // (grammar/clause.rs), which depends on `out` staying opacifiable as a
    // noun fallback when paired with an unlicensed verb. `you've` has a
    // second-order retain effect (litaudit-plan.md §4) that removes two
    // `lexical noun` dump rows outside the measured 47 (`surveilled`,
    // `completed`), tripping the round's own hard stop condition ("any
    // removed row not among the 47"). Since the plan explicitly did not
    // measure this class before declaring it zero-cost, the mechanic holds
    // the const to only the measured-exposure literals plus the
    // already-present hand arms, and tickets the zero-exposure class for a
    // follow-up round that measures each one individually.
    const OPACITY_RESERVED_LITERALS: &'static [&'static str] = &[
        // already present as hand arms before this round
        "plus", "who", "except", "not",
        // measured exposure (litaudit-plan.md §1.2) — the hard floor
        "there", "up", "than", "it's", "that's", "down", "minus", "X",
    ];

    fn has_known_word(&self, tokens: &[Token], start: usize) -> bool {
        if tokens
            .get(start)
            .is_none_or(|token| token.kind != TokenKind::Word)
        {
            return true;
        }
        let vocabulary_slots = [
            LexicalSlot::Noun(NounUsage::Either),
            LexicalSlot::Adjective,
            LexicalSlot::Adverb,
            LexicalSlot::Pronoun(PronounCase::Subject),
            LexicalSlot::Pronoun(PronounCase::Object),
            LexicalSlot::Auxiliary,
        ];
        vocabulary_slots
            .into_iter()
            .any(|slot| !self.word_matches(tokens, start, slot).is_empty())
            || crate::word::VERB_SLOTS.into_iter().any(|slot| {
                !self
                    .word_matches(tokens, start, LexicalSlot::Verb(slot))
                    .is_empty()
            })
            || !self.scan_determiner(tokens, start).is_empty()
            || !self.scan_preposition(tokens, start).is_empty()
            || !self.scan_conjunction(tokens, start).is_empty()
            || self.subordinator_at(tokens, start).is_some()
            || Self::OPACITY_RESERVED_LITERALS
                .iter()
                .any(|literal| self.one_token_match(tokens, start, literal).is_some())
            || !self
                .catalog_matches(tokens, start, CatalogSlot::Noun(NounUsage::Either))
                .is_empty()
            || !self
                .catalog_matches(tokens, start, CatalogSlot::Adjective)
                .is_empty()
            || !self
                .catalog_matches(tokens, start, CatalogSlot::AbilityItem)
                .is_empty()
            || crate::word::VERB_SLOTS.into_iter().any(|slot| {
                !self
                    .catalog_matches(tokens, start, CatalogSlot::Verb(slot))
                    .is_empty()
            })
    }

    fn scan_subordinators(
        &self,
        tokens: &[Token],
        start: usize,
    ) -> Vec<LexicalMatch<Features, MeaningKey>> {
        self.subordinator_at(tokens, start)
            .map(|(end, subordinator)| LexicalMatch {
                end,
                features: Features::Subordinator(subordinator),
                meaning: MeaningKey::Subordinator(subordinator),
                local_cost: ParseCost::default(),
            })
            .into_iter()
            .collect()
    }

    fn subordinator_at(
        &self,
        tokens: &[Token],
        start: usize,
    ) -> Option<(usize, crate::syntax::Subordinator)> {
        let surface = self.token_text(tokens, start)?;
        if surface.eq_ignore_ascii_case("for")
            && let Some(end) = self.words_match(tokens, start, &["for", "as", "long", "as"])
        {
            return Some((end, crate::syntax::Subordinator::ForAsLongAs));
        }
        if surface.eq_ignore_ascii_case("the")
            && let Some(end) = self.words_match(tokens, start, &["the", "next", "time"])
        {
            return Some((end, crate::syntax::Subordinator::TheNextTime));
        }
        if surface.eq_ignore_ascii_case("as") {
            if let Some(end) = self.words_match(tokens, start, &["as", "long", "as"]) {
                return Some((end, crate::syntax::Subordinator::AsLongAs));
            }
            if let Some(end) = self.words_match(tokens, start, &["as", "though"]) {
                return Some((end, crate::syntax::Subordinator::AsThough));
            }
            return Some((start + 1, crate::syntax::Subordinator::As));
        }
        let subordinator = if surface.eq_ignore_ascii_case("if") {
            crate::syntax::Subordinator::If
        } else if surface.eq_ignore_ascii_case("while") {
            crate::syntax::Subordinator::While
        } else if surface.eq_ignore_ascii_case("unless") {
            crate::syntax::Subordinator::Unless
        } else if surface.eq_ignore_ascii_case("before") {
            crate::syntax::Subordinator::Before
        } else if surface.eq_ignore_ascii_case("after") {
            crate::syntax::Subordinator::After
        } else if surface.eq_ignore_ascii_case("until") {
            // The clausal `until <event clause>` complement that bounds an
            // iterated action (`reveal cards … until you reveal a creature
            // card`). Also lexed as `Preposition::Until` for the durational
            // adjunct `until end of turn`; the two readings are disjoint by
            // what follows — only a finite `Clause` completes the subordinate
            // rule, only a noun phrase completes the prepositional one — so no
            // clean parse gains a competing alternative.
            crate::syntax::Subordinator::Until
        } else if surface.eq_ignore_ascii_case("where") {
            crate::syntax::Subordinator::Where
        } else {
            return None;
        };
        Some((start + 1, subordinator))
    }

    fn scan_existential(
        &self,
        tokens: &[Token],
        start: usize,
    ) -> Vec<LexicalMatch<Features, MeaningKey>> {
        [
            ((&["there", "is"] as &[&str]), ExistentialForm::Is),
            ((&["there's"] as &[&str]), ExistentialForm::ContractedIs),
            ((&["there", "are"] as &[&str]), ExistentialForm::Are),
            ((&["there", "was"] as &[&str]), ExistentialForm::Was),
            ((&["there", "were"] as &[&str]), ExistentialForm::Were),
        ]
        .into_iter()
        .filter_map(|(words, form)| {
            self.words_match(tokens, start, words)
                .map(|end| LexicalMatch {
                    end,
                    features: Features::Existential {
                        number: match form {
                            ExistentialForm::Is
                            | ExistentialForm::ContractedIs
                            | ExistentialForm::Was => Number::Singular,
                            ExistentialForm::Are | ExistentialForm::Were => Number::Plural,
                        },
                    },
                    meaning: MeaningKey::Existential(form),
                    local_cost: ParseCost::default(),
                })
        })
        .collect()
    }

    fn scan_subject_auxiliary(
        &self,
        tokens: &[Token],
        start: usize,
    ) -> Vec<LexicalMatch<Features, MeaningKey>> {
        [
            (
                "you're",
                ContractedSubjectKey::Pronoun(Pronoun::You),
                Auxiliary::Be,
                Person::Second,
                Number::Singular,
            ),
            (
                "you've",
                ContractedSubjectKey::Pronoun(Pronoun::You),
                Auxiliary::Have,
                Person::Second,
                Number::Singular,
            ),
            (
                "he's",
                ContractedSubjectKey::Pronoun(Pronoun::It(crate::word::Gender::Masculine)),
                Auxiliary::Be,
                Person::Third,
                Number::Singular,
            ),
            (
                "he's",
                ContractedSubjectKey::Pronoun(Pronoun::It(crate::word::Gender::Masculine)),
                Auxiliary::Have,
                Person::Third,
                Number::Singular,
            ),
            (
                "she's",
                ContractedSubjectKey::Pronoun(Pronoun::It(crate::word::Gender::Feminine)),
                Auxiliary::Be,
                Person::Third,
                Number::Singular,
            ),
            (
                "she's",
                ContractedSubjectKey::Pronoun(Pronoun::It(crate::word::Gender::Feminine)),
                Auxiliary::Have,
                Person::Third,
                Number::Singular,
            ),
            (
                "it's",
                ContractedSubjectKey::Pronoun(Pronoun::It(crate::word::Gender::Neuter)),
                Auxiliary::Be,
                Person::Third,
                Number::Singular,
            ),
            (
                "it's",
                ContractedSubjectKey::Pronoun(Pronoun::It(crate::word::Gender::Neuter)),
                Auxiliary::Have,
                Person::Third,
                Number::Singular,
            ),
            (
                "that's",
                ContractedSubjectKey::Demonstrative(Demonstrative::That),
                Auxiliary::Be,
                Person::Third,
                Number::Singular,
            ),
            (
                "that's",
                ContractedSubjectKey::Demonstrative(Demonstrative::That),
                Auxiliary::Have,
                Person::Third,
                Number::Singular,
            ),
            (
                "they're",
                ContractedSubjectKey::Pronoun(Pronoun::They),
                Auxiliary::Be,
                Person::Third,
                Number::Plural,
            ),
            (
                "they've",
                ContractedSubjectKey::Pronoun(Pronoun::They),
                Auxiliary::Have,
                Person::Third,
                Number::Plural,
            ),
        ]
        .into_iter()
        .filter_map(|(surface, subject, auxiliary, person, number)| {
            self.one_token_match(tokens, start, surface).map(|end| {
                let auxiliary = AuxiliaryInstance {
                    auxiliary,
                    inflection: AuxiliaryInflection::Present { person, number },
                    contracted_negation: false,
                };
                LexicalMatch {
                    end,
                    features: Features::SubjectAuxiliary {
                        subject,
                        agreement: Agreement { person, number },
                        auxiliary,
                    },
                    meaning: MeaningKey::SubjectAuxiliary(SubjectAuxiliaryKey {
                        subject,
                        auxiliary,
                    }),
                    local_cost: ParseCost {
                        precedence: u32::from(
                            auxiliary.auxiliary == Auxiliary::Have && surface.ends_with("'s"),
                        ),
                        ..ParseCost::default()
                    },
                }
            })
        })
        .collect()
    }

    fn scan_possessive_noun(
        &self,
        tokens: &[Token],
        start: usize,
    ) -> Vec<LexicalMatch<Features, MeaningKey>> {
        let Some(token) = tokens
            .get(start)
            .filter(|token| token.kind == TokenKind::Word)
        else {
            return Vec::new();
        };
        let Some(surface) = token.span.text(self.source) else {
            return Vec::new();
        };
        // One possessive, two spellings. `owner's` is a single word token because
        // the tokenizer glues an apostrophe into a word when a letter follows it;
        // the plural `owners'` ends the word there and leaves the genitive marker
        // as its own bare apostrophe token (`word_end` in `surface.rs`). The
        // s-apostrophe arm therefore consumes two tokens and takes the whole
        // surface as the stem.
        let (stem, end, plural_only) = match surface.strip_suffix("'s") {
            Some(stem) => (stem, start + 1, false),
            None if self.bare_genitive_apostrophe(tokens, start) => (surface, start + 2, true),
            None => return Vec::new(),
        };
        // Without this filter a singular noun whose lemma ends in `s` would match
        // the bare-apostrophe arm and then render back as `…'s`, breaking
        // round-trip. The bare marker is licensed only by a plural `NounInstance`.
        let word_filter = |word: &WordMatch| {
            !plural_only || matches!(word, WordMatch::Noun(NounInstance::Plural(_)))
        };
        let mut matches = Vocabulary::new()
            .matches(stem, LexicalSlot::Noun(NounUsage::Either))
            .into_iter()
            .filter(word_filter)
            .flat_map(|word| lexical_word_matches(word, end))
            .collect::<Vec<_>>();
        matches.extend(
            self.catalogs
                .matches(stem, CatalogSlot::Noun(NounUsage::Either))
                .into_iter()
                .filter(|catalog_match| catalog_match.length == stem.len())
                .flat_map(|catalog_match| match catalog_match.value {
                    CatalogValue::Word(word) if word_filter(&word) => {
                        lexical_word_matches(word, end)
                    }
                    CatalogValue::Word(_) | CatalogValue::Atom(_) => Vec::new(),
                }),
        );
        // A possessive vocabulary noun renders its stem lowercase (`fang's`), so
        // reading a capitalized nickname's possessive (`Fang's`) as one corrupts
        // the printed name. Dispreference such readings when the nickname collides
        // here so the case-preserving
        // [`PossessiveThisCard`](EnglishLexicalSlot::PossessiveThisCard)
        // self-reference wins; this is the possessive counterpart of the same
        // penalty in `word_matches` and `catalog_matches`.
        if self.nickname_lowercasing_collision(tokens, start) {
            let dispreference = ParseCost {
                reading_dispreference: 3,
                ..ParseCost::default()
            };
            for candidate in &mut matches {
                candidate.local_cost += dispreference;
            }
        }
        matches
    }

    /// Whether the token after the word at `start` is the bare genitive
    /// apostrophe of an s-plural possessive (`owners'`). The apostrophe must be
    /// a punctuation token butted directly against the word — the tokenizer
    /// never emits it as part of the word, and no space intervenes in the
    /// canonical templates — and the word must end in `s`, which is the only
    /// spelling the bare marker attaches to.
    fn bare_genitive_apostrophe(&self, tokens: &[Token], start: usize) -> bool {
        let Some(word) = tokens.get(start) else {
            return false;
        };
        let Some(surface) = word.span.text(self.source) else {
            return false;
        };
        if !surface.ends_with('s') {
            return false;
        }
        tokens.get(start + 1).is_some_and(|marker| {
            marker.kind == TokenKind::Punctuation(Punctuation::Apostrophe)
                && marker.span.start == word.span.end
        })
    }

    fn magic_match<'grammar>(
        &'grammar self,
        tokens: &[Token],
        start: usize,
        kind: TokenKind,
    ) -> Option<(&'grammar str, usize)> {
        let token = tokens.get(start).filter(|token| token.kind == kind)?;
        Some((token.span.text(self.source)?, start + 1))
    }

    fn scan_conjunction(
        &self,
        tokens: &[Token],
        start: usize,
    ) -> Vec<LexicalMatch<Features, MeaningKey>> {
        use crate::syntax::PredicateConjunction;

        let Some(surface) = self.token_text(tokens, start) else {
            return Vec::new();
        };
        let conjunction = if surface.eq_ignore_ascii_case("and") {
            PredicateConjunction::And
        } else if surface.eq_ignore_ascii_case("or") {
            PredicateConjunction::Or
        } else if surface.eq_ignore_ascii_case("then") {
            PredicateConjunction::Then
        } else if surface.eq_ignore_ascii_case("and/or") {
            PredicateConjunction::AndOr
        } else {
            return Vec::new();
        };
        vec![LexicalMatch {
            end: start + 1,
            features: Features::Conjunction(conjunction),
            meaning: MeaningKey::Conjunction(conjunction),
            local_cost: ParseCost::default(),
        }]
    }

    fn scan_determiner(
        &self,
        tokens: &[Token],
        start: usize,
    ) -> Vec<LexicalMatch<Features, MeaningKey>> {
        let Some(surface) = self.token_text(tokens, start) else {
            return Vec::new();
        };
        let key = if surface.eq_ignore_ascii_case("the") {
            DeterminerKey::The
        } else if surface.eq_ignore_ascii_case("each") {
            DeterminerKey::Each
        } else if surface.eq_ignore_ascii_case("another") {
            DeterminerKey::Another
        } else if surface.eq_ignore_ascii_case("a") {
            DeterminerKey::Indefinite(IndefiniteArticleKey::A)
        } else if surface.eq_ignore_ascii_case("an") {
            DeterminerKey::Indefinite(IndefiniteArticleKey::An)
        } else if surface.eq_ignore_ascii_case("this") {
            DeterminerKey::Demonstrative(DemonstrativeKey::This)
        } else if surface.eq_ignore_ascii_case("that") {
            DeterminerKey::Demonstrative(DemonstrativeKey::That)
        } else if surface.eq_ignore_ascii_case("these") {
            DeterminerKey::Demonstrative(DemonstrativeKey::These)
        } else if surface.eq_ignore_ascii_case("those") {
            DeterminerKey::Demonstrative(DemonstrativeKey::Those)
        } else if surface.eq_ignore_ascii_case("all") {
            DeterminerKey::All
        } else if surface.eq_ignore_ascii_case("any") {
            DeterminerKey::Any
        } else if surface.eq_ignore_ascii_case("no") {
            DeterminerKey::No
        } else if surface.eq_ignore_ascii_case("your") {
            DeterminerKey::Possessive(Pronoun::You)
        } else if surface.eq_ignore_ascii_case("his") {
            DeterminerKey::Possessive(Pronoun::It(crate::word::Gender::Masculine))
        } else if surface.eq_ignore_ascii_case("her") {
            DeterminerKey::Possessive(Pronoun::It(crate::word::Gender::Feminine))
        } else if surface.eq_ignore_ascii_case("its") {
            DeterminerKey::Possessive(Pronoun::It(crate::word::Gender::Neuter))
        } else if surface.eq_ignore_ascii_case("their") {
            DeterminerKey::Possessive(Pronoun::They)
        } else {
            return Vec::new();
        };
        // `The` heading a multi-word nickname (e.g. `The Beast`) also parses as
        // the lowercase `the` determiner leading an opaque noun; that reading
        // lowercases the name's first word. Dispreference it so the nickname,
        // which reproduces the capitalized `The`, wins.
        let local_cost =
            if key == DeterminerKey::The && self.nickname_lowercasing_collision(tokens, start) {
                ParseCost {
                    reading_dispreference: 3,
                    ..ParseCost::default()
                }
            } else {
                ParseCost::default()
            };
        vec![LexicalMatch {
            end: start + 1,
            features: Features::Determiner {
                cardinality: key.cardinality(),
                article: key.article(),
                set_exception_host: matches!(key, DeterminerKey::All | DeterminerKey::Each),
            },
            meaning: MeaningKey::Determiner(key),
            local_cost,
        }]
    }

    fn scan_demonstrative(
        &self,
        tokens: &[Token],
        start: usize,
    ) -> Vec<LexicalMatch<Features, MeaningKey>> {
        let Some(surface) = self.token_text(tokens, start) else {
            return Vec::new();
        };
        let (demonstrative, number) = if surface.eq_ignore_ascii_case("this") {
            (crate::syntax::Demonstrative::This, Number::Singular)
        } else if surface.eq_ignore_ascii_case("that") {
            (crate::syntax::Demonstrative::That, Number::Singular)
        } else if surface.eq_ignore_ascii_case("these") {
            (crate::syntax::Demonstrative::These, Number::Plural)
        } else if surface.eq_ignore_ascii_case("those") {
            (crate::syntax::Demonstrative::Those, Number::Plural)
        } else {
            return Vec::new();
        };
        vec![LexicalMatch {
            end: start + 1,
            features: Features::NounPhrase {
                agreement: Some(Agreement {
                    person: Person::Third,
                    number,
                }),
                pronoun_case: None,
                adjunct: None,
                set_exception: SetExceptionState::Ineligible,
            },
            meaning: MeaningKey::Determiner(DeterminerKey::Demonstrative(match demonstrative {
                crate::syntax::Demonstrative::This => DemonstrativeKey::This,
                crate::syntax::Demonstrative::That => DemonstrativeKey::That,
                crate::syntax::Demonstrative::These => DemonstrativeKey::These,
                crate::syntax::Demonstrative::Those => DemonstrativeKey::Those,
            })),
            local_cost: ParseCost::default(),
        }]
    }

    fn scan_at_least_quantity(
        &self,
        tokens: &[Token],
        start: usize,
    ) -> Vec<LexicalMatch<Features, MeaningKey>> {
        // `at least N` floors the count; `N or <word>` bounds it with a
        // comparative word (`more`/`greater` above, `fewer`/`less` at or
        // below). The word is resolved from vocabulary comparison metadata —
        // no comparative spelling is matched here.
        let (surface, number_position, end, comparative) =
            if let Some(number_start) = self.words_match(tokens, start, &["at", "least"]) {
                let Some(surface) = self.token_text(tokens, number_start) else {
                    return Vec::new();
                };
                (surface, number_start, number_start + 1, None)
            } else {
                let Some(surface) = self.token_text(tokens, start) else {
                    return Vec::new();
                };
                let Some(after_or) = self.one_token_match(tokens, start + 1, "or") else {
                    return Vec::new();
                };
                let Some(word_surface) = self.token_text(tokens, after_or) else {
                    return Vec::new();
                };
                let Some(word) = comparative_word(word_surface) else {
                    return Vec::new();
                };
                (surface, start, after_or + 1, Some(word))
            };
        let sentence_initial = Self::is_sentence_initial(tokens, number_position);
        [
            Numeral::Cardinal,
            Numeral::Ordinal,
            Numeral::Arabic(false),
            Numeral::Arabic(true),
            Numeral::Roman,
        ]
        .into_iter()
        .filter_map(|notation| {
            parse_notation(notation, surface, sentence_initial).map(|value| {
                let number = NumberKey { value, notation };
                quantity_match(
                    end,
                    match comparative {
                        Some(word) => QuantityKey::OrComparison(number, word),
                        None => QuantityKey::AtLeast(number),
                    },
                )
            })
        })
        .collect()
    }

    fn scan_bounded_quantity(
        &self,
        tokens: &[Token],
        start: usize,
        kind: BoundedQuantityKind,
    ) -> Vec<LexicalMatch<Features, MeaningKey>> {
        let opening = match kind {
            BoundedQuantityKind::MoreThan => "more",
            BoundedQuantityKind::FewerThan => "fewer",
        };
        let Some(number_start) = self.words_match(tokens, start, &[opening, "than"]) else {
            return Vec::new();
        };
        let Some(surface) = self.token_text(tokens, number_start) else {
            return Vec::new();
        };
        let end = number_start + 1;
        let sentence_initial = Self::is_sentence_initial(tokens, number_start);
        [
            Numeral::Cardinal,
            Numeral::Ordinal,
            Numeral::Arabic(false),
            Numeral::Arabic(true),
            Numeral::Roman,
        ]
        .into_iter()
        .filter_map(|notation| {
            parse_notation(notation, surface, sentence_initial).map(|value| {
                let number = NumberKey { value, notation };
                let quantity = match kind {
                    BoundedQuantityKind::MoreThan => QuantityKey::MoreThan(number),
                    BoundedQuantityKind::FewerThan => QuantityKey::FewerThan(number),
                };
                quantity_match(end, quantity)
            })
        })
        .collect()
    }

    /// Scans a quoted ability (`"…"`) as one lexical unit. Matches only when a
    /// double quote opens at `start` and a later double quote closes it with a
    /// non-empty interior; the match spans both delimiters and carries the
    /// interior's source span, reparsed to an
    /// [`Ability`](crate::syntax::Ability) at lowering. Earley prediction
    /// restricts the scan to positions where a grant/coordination rule
    /// expects an object, so the interior tokens never invite a scan of
    /// their own.
    fn scan_quoted_ability(
        tokens: &[Token],
        start: usize,
    ) -> Vec<LexicalMatch<Features, MeaningKey>> {
        if tokens.get(start).map(|token| token.kind)
            != Some(TokenKind::Punctuation(Punctuation::DoubleQuote))
        {
            return Vec::new();
        }
        let Some(close) = tokens[start + 1..]
            .iter()
            .position(|token| token.kind == TokenKind::Punctuation(Punctuation::DoubleQuote))
            .map(|offset| start + 1 + offset)
        else {
            return Vec::new();
        };
        // The interior must hold at least one token — an empty `""` is never a
        // quoted ability.
        if close == start + 1 {
            return Vec::new();
        }
        let interior_start = tokens[start + 1].span.start;
        let interior_end = tokens[close - 1].span.end;
        vec![LexicalMatch {
            end: close + 1,
            features: Features::None,
            meaning: MeaningKey::QuotedAbility(Span::new(interior_start, interior_end)),
            local_cost: ParseCost::default(),
        }]
    }

    fn scan_frequency(
        &self,
        tokens: &[Token],
        start: usize,
    ) -> Vec<LexicalMatch<Features, MeaningKey>> {
        let (bound, count_start) =
            if let Some(end) = self.words_match(tokens, start, &["no", "more", "than"]) {
                (FrequencyBound::NoMoreThan, end)
            } else if let Some(end) = self.words_match(tokens, start, &["more", "than"]) {
                (FrequencyBound::MoreThan, end)
            } else {
                return Vec::new();
            };
        if let Some(end) = self.one_token_match(tokens, count_start, "once") {
            return vec![frequency_match(
                end,
                FrequencyKey {
                    bound,
                    count: FrequencyCountKey::Once,
                },
            )];
        }
        if let Some(end) = self.one_token_match(tokens, count_start, "twice") {
            return vec![frequency_match(
                end,
                FrequencyKey {
                    bound,
                    count: FrequencyCountKey::Twice,
                },
            )];
        }
        let Some(surface) = self.token_text(tokens, count_start) else {
            return Vec::new();
        };
        let Some(end) = self.one_token_match(tokens, count_start + 1, "times") else {
            return Vec::new();
        };
        let sentence_initial = Self::is_sentence_initial(tokens, count_start);
        [
            Numeral::Cardinal,
            Numeral::Ordinal,
            Numeral::Arabic(false),
            Numeral::Arabic(true),
            Numeral::Roman,
        ]
        .into_iter()
        .filter_map(|notation| {
            parse_notation(notation, surface, sentence_initial).map(|value| {
                frequency_match(
                    end,
                    FrequencyKey {
                        bound,
                        count: FrequencyCountKey::Times(NumberKey { value, notation }),
                    },
                )
            })
        })
        .collect()
    }

    fn scan_or_quantity(
        &self,
        tokens: &[Token],
        start: usize,
    ) -> Vec<LexicalMatch<Features, MeaningKey>> {
        let Some(first_surface) = self.token_text(tokens, start) else {
            return Vec::new();
        };
        let Some(second_start) = self.one_token_match(tokens, start + 1, "or") else {
            return Vec::new();
        };
        let Some(second_surface) = self.token_text(tokens, second_start) else {
            return Vec::new();
        };
        let end = second_start + 1;
        let notations = [
            Numeral::Cardinal,
            Numeral::Ordinal,
            Numeral::Arabic(false),
            Numeral::Arabic(true),
            Numeral::Roman,
        ];
        let first_sentence_initial = Self::is_sentence_initial(tokens, start);
        let second_sentence_initial = Self::is_sentence_initial(tokens, second_start);
        let mut matches = Vec::new();
        for first_notation in notations {
            let Some(first_value) =
                parse_notation(first_notation, first_surface, first_sentence_initial)
            else {
                continue;
            };
            for second_notation in notations {
                let Some(second_value) =
                    parse_notation(second_notation, second_surface, second_sentence_initial)
                else {
                    continue;
                };
                let mut candidate = quantity_match(
                    end,
                    QuantityKey::Or(
                        NumberKey {
                            value: first_value,
                            notation: first_notation,
                        },
                        NumberKey {
                            value: second_value,
                            notation: second_notation,
                        },
                    ),
                );
                if first_notation == Numeral::Ordinal || second_notation == Numeral::Ordinal {
                    candidate.local_cost.reading_dispreference += 1;
                }
                matches.push(candidate);
            }
        }
        matches
    }

    fn scan_preposition(
        &self,
        tokens: &[Token],
        start: usize,
    ) -> Vec<LexicalMatch<Features, MeaningKey>> {
        let Some(surface) = self.token_text(tokens, start) else {
            return Vec::new();
        };
        let preposition = if surface.eq_ignore_ascii_case("after") {
            Preposition::After
        } else if surface.eq_ignore_ascii_case("among") {
            Preposition::Among
        } else if surface.eq_ignore_ascii_case("as") {
            Preposition::As
        } else if surface.eq_ignore_ascii_case("at") {
            Preposition::At
        } else if surface.eq_ignore_ascii_case("before") {
            Preposition::Before
        } else if surface.eq_ignore_ascii_case("between") {
            Preposition::Between
        } else if surface.eq_ignore_ascii_case("by") {
            Preposition::By
        } else if surface.eq_ignore_ascii_case("during") {
            Preposition::During
        } else if surface.eq_ignore_ascii_case("for") {
            Preposition::For
        } else if surface.eq_ignore_ascii_case("from") {
            Preposition::From
        } else if surface.eq_ignore_ascii_case("in") {
            Preposition::In
        } else if surface.eq_ignore_ascii_case("into") {
            Preposition::Into
        } else if surface.eq_ignore_ascii_case("of") {
            Preposition::Of
        } else if surface.eq_ignore_ascii_case("on") {
            Preposition::On
        } else if surface.eq_ignore_ascii_case("onto") {
            Preposition::Onto
        } else if surface.eq_ignore_ascii_case("to") {
            Preposition::To
        } else if surface.eq_ignore_ascii_case("until") {
            Preposition::Until
        } else if surface.eq_ignore_ascii_case("under") {
            Preposition::Under
        } else if surface.eq_ignore_ascii_case("with") {
            Preposition::With
        } else if surface.eq_ignore_ascii_case("without") {
            Preposition::Without
        } else {
            return Vec::new();
        };
        vec![LexicalMatch {
            end: start + 1,
            features: Features::Preposition(preposition),
            meaning: MeaningKey::Preposition(preposition),
            local_cost: ParseCost::default(),
        }]
    }
}

#[derive(Default)]
struct RuleBuilder {
    rules: Vec<Rule<Nonterminal, EnglishLexicalSlot>>,
    tags: Vec<RuleTag>,
    rules_by_lhs: HashMap<Nonterminal, Vec<RuleId>>,
}

impl RuleBuilder {
    fn add(
        &mut self,
        tag: RuleTag,
        lhs: Nonterminal,
        rhs: impl IntoIterator<Item = Expected<Nonterminal, EnglishLexicalSlot>>,
    ) {
        self.add_with_cost(tag, lhs, rhs, ParseCost::default());
    }

    fn add_with_cost(
        &mut self,
        tag: RuleTag,
        lhs: Nonterminal,
        rhs: impl IntoIterator<Item = Expected<Nonterminal, EnglishLexicalSlot>>,
        local_cost: ParseCost,
    ) {
        let id = RuleId::new(self.rules.len());
        self.rules.push(Rule {
            lhs,
            rhs: rhs.into_iter().collect(),
            local_cost,
        });
        self.tags.push(tag);
        self.rules_by_lhs.entry(lhs).or_default().push(id);
    }

    #[allow(
        clippy::too_many_lines,
        reason = "nominal rule construction is intentionally broad"
    )]
    fn add_nominal_rules(&mut self) {
        use EnglishLexicalSlot as L;
        use Expected::Lexical as l;
        use Expected::Nonterminal as n;
        use Nonterminal as N;

        for notation in [
            Numeral::Cardinal,
            Numeral::Ordinal,
            Numeral::Arabic(false),
            Numeral::Arabic(true),
            Numeral::Roman,
        ] {
            self.add(
                RuleTag::QuantityExact,
                N::Quantity,
                [l(L::Number(notation))],
            );
            self.add(
                RuleTag::QuantityUpTo,
                N::Quantity,
                [l(L::Up), l(L::To), l(L::Number(notation))],
            );
        }
        self.add(
            RuleTag::QuantityAtLeast,
            N::Quantity,
            [l(L::QuantityAtLeast)],
        );
        self.add(RuleTag::QuantityOr, N::Quantity, [l(L::QuantityOr)]);
        self.add(RuleTag::QuantityX, N::Quantity, [l(L::QuantityX)]);
        self.add(RuleTag::QuantityBoth, N::Quantity, [l(L::QuantityBoth)]);
        self.add(
            RuleTag::QuantityThatMany,
            N::Quantity,
            [l(L::QuantityThatMany)],
        );
        self.add(
            RuleTag::QuantityThatMuch,
            N::Quantity,
            [l(L::QuantityThatMuch)],
        );
        self.add(
            RuleTag::QuantityMoreThan,
            N::Quantity,
            [l(L::QuantityBound(BoundedQuantityKind::MoreThan))],
        );
        self.add(
            RuleTag::QuantityFewerThan,
            N::Quantity,
            [l(L::QuantityBound(BoundedQuantityKind::FewerThan))],
        );

        self.add(RuleTag::DeterminerClosed, N::Determiner, [l(L::Determiner)]);
        self.add(
            RuleTag::DeterminerTarget,
            N::Determiner,
            [l(L::DeterminerTarget)],
        );
        self.add(
            RuleTag::DeterminerQuantifiedTarget,
            N::Determiner,
            [n(N::Quantity), l(L::DeterminerTarget)],
        );
        self.add(RuleTag::DeterminerQuantity, N::Determiner, [n(N::Quantity)]);
        self.add(
            RuleTag::DeterminerPossessiveThisCard,
            N::Determiner,
            [l(L::PossessiveThisCard)],
        );
        self.add(
            RuleTag::DeterminerPossessiveNoun,
            N::Determiner,
            [n(N::PossessiveNounPhrase)],
        );

        self.add(RuleTag::Adjective, N::Adjective, [l(L::Adjective)]);
        self.add(
            RuleTag::AdjectivePhrase,
            N::AdjectivePhrase,
            [n(N::Adjective)],
        );
        self.add(
            RuleTag::AdjectivePhraseFaceUp,
            N::AdjectivePhrase,
            [l(L::Face), l(L::Up)],
        );
        self.add(
            RuleTag::AdjectivePhraseFaceDown,
            N::AdjectivePhrase,
            [l(L::Face), l(L::Down)],
        );
        self.add(
            RuleTag::ComparisonStandard,
            N::ComparisonStandard,
            [n(N::NounPhrase)],
        );
        self.add(
            RuleTag::ComparisonStandard,
            N::ComparisonStandard,
            [n(N::AdjectivePhrase)],
        );
        self.add(
            RuleTag::ComparisonStandard,
            N::ComparisonStandard,
            [n(N::Clause)],
        );
        self.add(
            RuleTag::ComparisonThan,
            N::ComparisonComplement,
            [l(L::Than), n(N::ComparisonStandard)],
        );
        self.add(
            RuleTag::ComparisonThanOrEqualTo,
            N::ComparisonComplement,
            [l(L::Than), l(L::OrEqualTo), n(N::ComparisonStandard)],
        );
        self.add(
            RuleTag::AdjectivePhraseComparison,
            N::AdjectivePhrase,
            [n(N::Adjective), n(N::ComparisonComplement)],
        );
        // `2 greater` / `two greater`: a numeral degree premodifier on an
        // `OrComparative` adjective. Predicative only — see
        // `AdjectiveComparisonState::Measured`.
        for notation in [Numeral::Cardinal, Numeral::Arabic(false)] {
            self.add(
                RuleTag::AdjectivePhraseDegreeMeasure,
                N::AdjectivePhrase,
                [l(L::Number(notation)), n(N::Adjective)],
            );
        }
        self.add(RuleTag::Noun, N::Noun, [l(L::Noun(NounUsage::Either))]);
        self.add(
            RuleTag::PossessiveNounBase,
            N::PossessiveNounPhrase,
            [l(L::PossessiveNoun)],
        );
        self.add(
            RuleTag::PossessiveNounDetermined,
            N::PossessiveNounPhrase,
            [n(N::Determiner), n(N::PossessiveNounPhrase)],
        );

        self.add(RuleTag::NominalNoun, N::Nominal, [n(N::Noun)]);
        self.add_with_cost(
            RuleTag::NominalAdjective,
            N::Nominal,
            [n(N::AdjectivePhrase), n(N::Nominal)],
            ParseCost {
                precedence: 1,
                ..ParseCost::default()
            },
        );
        self.add_with_cost(
            RuleTag::NominalNounModifier,
            N::Nominal,
            [n(N::Noun), n(N::Nominal)],
            ParseCost {
                precedence: 1,
                ..ParseCost::default()
            },
        );
        // `declare attackers`/`declare blockers` — every child is a
        // literal-token slot, so this production can only ever match those
        // two exact three-word sequences. Plain `add`, no cost: cost cannot
        // fix a recognition-breadth problem (the `restrict`-round
        // regression), so none is used here on purpose.
        self.add(
            RuleTag::NominalCombatStepName,
            N::Nominal,
            [
                l(L::CombatStepDeclare),
                l(L::CombatStepParticipants),
                l(L::CombatStepHead),
            ],
        );
        self.add_with_cost(
            RuleTag::NominalNegatedModifier,
            N::Nominal,
            [l(L::NegatedModifier), n(N::Nominal)],
            ParseCost {
                precedence: 1,
                ..ParseCost::default()
            },
        );
        self.add_with_cost(
            RuleTag::NominalQuantityModifier,
            N::Nominal,
            [n(N::Quantity), n(N::Nominal)],
            ParseCost {
                precedence: 1,
                ..ParseCost::default()
            },
        );
        self.add_with_cost(
            RuleTag::NominalPowerToughnessModifier,
            N::Nominal,
            [l(L::PowerToughness), n(N::Nominal)],
            ParseCost {
                precedence: 1,
                ..ParseCost::default()
            },
        );
        self.add(
            RuleTag::NominalDeterminer,
            N::Nominal,
            [n(N::Determiner), n(N::Nominal)],
        );
        self.add(
            RuleTag::NominalPrepositional,
            N::Nominal,
            [n(N::Nominal), n(N::PrepositionalPhrase)],
        );
        self.add(
            RuleTag::NominalInfinitive,
            N::Nominal,
            [n(N::Nominal), n(N::InfinitiveClause)],
        );
        self.add_with_cost(
            RuleTag::NominalQuantityComplement,
            N::Nominal,
            [n(N::Nominal), n(N::Quantity)],
            ParseCost {
                precedence: 1,
                ..ParseCost::default()
            },
        );
        self.add(
            RuleTag::NominalRelative,
            N::Nominal,
            [n(N::Nominal), n(N::RelativeClause)],
        );
        self.add(
            RuleTag::NominalPostpositiveAdjective,
            N::Nominal,
            [n(N::Nominal), n(N::AdjectivePhrase)],
        );
        self.add(
            RuleTag::NominalComparison,
            N::Nominal,
            [n(N::Nominal), n(N::ComparisonComplement)],
        );

        // The `devotion` value nominal with its mandatory concrete-color
        // argument: `devotion to <color>` or `devotion to <color> and <color>`.
        // Gated by the dedicated `DevotionValue` head so the bare-color
        // `DevotionColors` rules stay out of ordinary phrases; the generic
        // `devotion to a/each/that color` shapes take the count-noun +
        // prepositional path instead.
        self.add(
            RuleTag::NominalDevotion,
            N::Nominal,
            [l(L::DevotionValue), l(L::To), n(N::DevotionColors)],
        );
        self.add(
            RuleTag::DevotionColorSingle,
            N::DevotionColors,
            [l(L::ColorWord)],
        );
        self.add(
            RuleTag::DevotionColorPair,
            N::DevotionColors,
            [l(L::ColorWord), l(L::Conjunction), l(L::ColorWord)],
        );

        // `the number of times <clause>`: the plural `times` head takes a bare
        // finite clause as a reduced adjunct-relative complement. Gated by the
        // dedicated `TimesNoun` head so no other noun admits a bare clause.
        self.add(
            RuleTag::NominalTimesClause,
            N::Nominal,
            [l(L::TimesNoun), n(N::Clause)],
        );

        self.add(RuleTag::NounPhraseNominal, N::NounPhrase, [n(N::Nominal)]);
        self.add(
            RuleTag::NounPhraseSubjectPronoun,
            N::NounPhrase,
            [l(L::Pronoun(PronounCase::Subject))],
        );
        self.add(
            RuleTag::NounPhraseObjectPronoun,
            N::NounPhrase,
            [l(L::Pronoun(PronounCase::Object))],
        );
        self.add(
            RuleTag::NounPhraseReciprocal,
            N::NounPhrase,
            [l(L::Reciprocal)],
        );
        self.add(RuleTag::NounPhraseQuantity, N::NounPhrase, [n(N::Quantity)]);
        self.add(RuleTag::NounPhraseThisCard, N::NounPhrase, [l(L::ThisCard)]);
        self.add(
            RuleTag::NounPhraseFullThisCard,
            N::NounPhrase,
            [l(L::FullThisCard)],
        );
        self.add(
            RuleTag::NounPhrasePossessiveThisCard,
            N::NounPhrase,
            [l(L::PossessiveThisCard)],
        );
        self.add(
            RuleTag::NounPhraseDemonstrative,
            N::NounPhrase,
            [l(L::Demonstrative)],
        );
        self.add(
            RuleTag::NounPhrasePartitive,
            N::NounPhrase,
            [n(N::Quantity), l(L::Of), n(N::NounPhrase)],
        );
        self.add(
            RuleTag::NounPhraseEachPartitive,
            N::NounPhrase,
            [l(L::EachDeterminer), l(L::Of), n(N::NounPhrase)],
        );
        // `any number of <plural NounPhrase>` — notional plural concord
        // [`anof` round]. The categorical gate is the dedicated `any` and
        // `number` lexical slots, which scan nothing but those two literal
        // words; that gate is already enforced at dot 0/dot 1, before the
        // recursive `NounPhrase` is even predicted, so no `accepts_prefix`
        // change is needed. The one remaining condition — the final noun
        // phrase must be plural — depends on the fourth child and cannot be
        // hoisted before reduce. Registered with a precedence dispreference:
        // this is a fallback behind the ordinary formal-singular nominal
        // analysis of the same surface, not a competing analysis of a
        // different surface. `precedence` only breaks ties among derivations
        // that both complete, so the formal-singular reading wins whenever it
        // completes, and this production is the only parse whenever a plural
        // predicate makes the singular reading fail to complete. Same idiom
        // as `NounPhraseCoordination` below.
        self.add_with_cost(
            RuleTag::NounPhraseAnyNumberOf,
            N::NounPhrase,
            [
                l(L::AnyDeterminer),
                l(L::NumberNoun),
                l(L::Of),
                n(N::NounPhrase),
            ],
            ParseCost {
                precedence: 1,
                ..ParseCost::default()
            },
        );
        self.add_with_cost(
            RuleTag::NounPhraseCoordination,
            N::NounPhrase,
            [n(N::NounPhrase), l(L::Conjunction), n(N::NounPhrase)],
            ParseCost {
                precedence: 1,
                ..ParseCost::default()
            },
        );
        self.add_with_cost(
            RuleTag::NounPhraseAdditiveCoordination,
            N::NounPhrase,
            [n(N::NounPhrase), l(L::Plus), n(N::NounPhrase)],
            ParseCost {
                precedence: 1,
                ..ParseCost::default()
            },
        );
        // Arithmetic value expressions. `plus` rides the additive coordination
        // above and `twice` the copular precomplement adverb, so only the
        // subtraction and halving operators are added here as structured value
        // nodes.
        self.add_with_cost(
            RuleTag::NounPhraseMinus,
            N::NounPhrase,
            [n(N::NounPhrase), l(L::Minus), n(N::NounPhrase)],
            ParseCost {
                precedence: 1,
                ..ParseCost::default()
            },
        );
        self.add(
            RuleTag::NounPhraseHalf,
            N::NounPhrase,
            [l(L::Half), n(N::NounPhrase)],
        );
        self.add(
            RuleTag::NounPhraseHalfRoundedUp,
            N::NounPhrase,
            [
                l(L::Half),
                n(N::NounPhrase),
                l(L::Punctuation(Punctuation::Comma)),
                l(L::Rounded),
                l(L::Up),
            ],
        );
        self.add(
            RuleTag::NounPhraseHalfRoundedDown,
            N::NounPhrase,
            [
                l(L::Half),
                n(N::NounPhrase),
                l(L::Punctuation(Punctuation::Comma)),
                l(L::Rounded),
                l(L::Down),
            ],
        );

        self.add(
            RuleTag::PrepositionalPhrase,
            N::PrepositionalPhrase,
            [l(L::Preposition), n(N::PrepositionalObject)],
        );
        self.add(
            RuleTag::PrepositionalObject,
            N::PrepositionalObject,
            [n(N::NounPhrase)],
        );
        self.add(
            RuleTag::PrepositionalObject,
            N::PrepositionalObject,
            [n(N::PrepositionalPhrase)],
        );
        self.add(
            RuleTag::PrepositionalObject,
            N::PrepositionalObject,
            [n(N::GerundClause)],
        );
        self.add(
            RuleTag::PrepositionalObject,
            N::PrepositionalObject,
            [l(L::Adverb)],
        );
    }

    fn add_clause_rules(&mut self) {
        clause::add_rules(self);
    }

    /// The markerless recipient-passive relative (`a creature dealt damage
    /// this way`). Its dedicated nonterminal starts with a scan-time-gated
    /// participle and reuses only the predicate extensions the construction
    /// needs. It never predicts generic `VerbPhrase`, whose registration here
    /// previously perturbed unrelated `do so` derivations.
    fn add_reduced_recipient_passive_rules(&mut self) {
        use EnglishLexicalSlot as L;
        use Expected::Lexical as l;
        use Expected::Nonterminal as n;
        use Nonterminal as N;

        self.add(
            RuleTag::VerbPhraseBase,
            N::ReducedRecipientPassive,
            [l(L::ReducedRecipientPassiveParticiple)],
        );
        self.add(
            RuleTag::ReducedRecipientPassiveTheme,
            N::ReducedRecipientPassiveTheme,
            [n(N::Nominal)],
        );
        self.add(
            RuleTag::VerbPhraseDirectObject,
            N::ReducedRecipientPassive,
            [
                n(N::ReducedRecipientPassive),
                n(N::ReducedRecipientPassiveTheme),
            ],
        );
        self.add(
            RuleTag::ReducedRecipientPassiveNominalAdjunct,
            N::ReducedRecipientPassive,
            [n(N::ReducedRecipientPassive), n(N::NounPhrase)],
        );
        self.add(
            RuleTag::VerbPhrasePrepositional,
            N::ReducedRecipientPassive,
            [n(N::ReducedRecipientPassive), n(N::PrepositionalPhrase)],
        );
        self.add(
            RuleTag::VerbPhraseAdverb,
            N::ReducedRecipientPassive,
            [n(N::ReducedRecipientPassive), l(L::Adverb)],
        );
        self.add(
            RuleTag::VerbPhraseFrequency,
            N::ReducedRecipientPassive,
            [n(N::ReducedRecipientPassive), n(N::FrequencyPhrase)],
        );
        self.add(
            RuleTag::NominalReducedRecipientPassive,
            N::Nominal,
            [n(N::Nominal), n(N::ReducedRecipientPassive)],
        );
    }

    /// A noun-phrase-denotation exception (`all creatures except (for)
    /// Dragons`).
    /// The completed host is checked at dot 1 before either `except` surface is
    /// predicted, preventing open noun fragments from launching this recursive
    /// noun-phrase attachment.
    fn add_set_exception_rules(&mut self) {
        use EnglishLexicalSlot as L;
        use Expected::Lexical as l;
        use Expected::Nonterminal as n;
        use Nonterminal as N;

        self.add(
            RuleTag::NounPhraseSetExceptionBare,
            N::NounPhrase,
            [n(N::NounPhrase), l(L::Except), n(N::NounPhrase)],
        );
        self.add(
            RuleTag::NounPhraseSetExceptionFor,
            N::NounPhrase,
            [
                n(N::NounPhrase),
                l(L::Except),
                l(L::ForWord),
                n(N::NounPhrase),
            ],
        );
        self.add(
            RuleTag::NounPhraseSetExceptionBare,
            N::NounPhrase,
            [
                n(N::NounPhrase),
                l(L::Punctuation(Punctuation::Comma)),
                l(L::Except),
                n(N::NounPhrase),
            ],
        );
        self.add(
            RuleTag::NounPhraseSetExceptionFor,
            N::NounPhrase,
            [
                n(N::NounPhrase),
                l(L::Punctuation(Punctuation::Comma)),
                l(L::Except),
                l(L::ForWord),
                n(N::NounPhrase),
            ],
        );
    }

    /// General coordination inside the nominal, appended last so every existing
    /// rule keeps its `RuleId` and every existing parse forest keeps its
    /// alternative indices. Two independent shapes:
    ///
    /// * **Modifier coordination** — a coordinated run of attributive modifiers
    ///   filling one modifier slot (`white and blue`, `artifact, creature, and
    ///   land`). The list is gathered on the dedicated `ModifierList`/
    ///   `CoordinatedModifier` nonterminals so a bare comma run never becomes a
    ///   standalone modifier, and only the closed form prepends to a nominal.
    /// * **Head-list coordination** — comma/Oxford extensions of the existing
    ///   binary noun-phrase coordination (`target artifact, enchantment, or
    ///   land`), so a shared-determiner list of heads joins one construction.
    fn add_coordination_rules(&mut self) {
        use EnglishLexicalSlot as L;
        use Expected::Lexical as l;
        use Expected::Nonterminal as n;
        use Nonterminal as N;

        // A coordinable modifier atom: an adjective phrase, a bare noun, or a
        // `non-` negated modifier (polarity composes per conjunct).
        self.add(
            RuleTag::ModifierConjunctAdjective,
            N::ModifierConjunct,
            [n(N::AdjectivePhrase)],
        );
        self.add(
            RuleTag::ModifierConjunctNoun,
            N::ModifierConjunct,
            [n(N::Noun)],
        );
        self.add(
            RuleTag::ModifierConjunctNegated,
            N::ModifierConjunct,
            [l(L::NegatedModifier)],
        );

        // The open comma-separated run, gathered left to right.
        self.add(
            RuleTag::ModifierListSingle,
            N::ModifierList,
            [n(N::ModifierConjunct)],
        );
        self.add(
            RuleTag::ModifierListComma,
            N::ModifierList,
            [
                n(N::ModifierList),
                l(L::Punctuation(Punctuation::Comma)),
                n(N::ModifierConjunct),
            ],
        );

        // Closing the run with a conjunction. The bare-conjunction close covers
        // both the simple two-way pair (`white and blue`) and the non-Oxford
        // list (`artifact, creature and land`); the Oxford close adds the comma
        // before the final conjunction (`artifact, creature, and land`).
        self.add_with_cost(
            RuleTag::CoordinatedModifierConjoined,
            N::CoordinatedModifier,
            [
                n(N::ModifierList),
                l(L::Conjunction),
                n(N::ModifierConjunct),
            ],
            ParseCost {
                precedence: 1,
                ..ParseCost::default()
            },
        );
        self.add_with_cost(
            RuleTag::CoordinatedModifierOxford,
            N::CoordinatedModifier,
            [
                n(N::ModifierList),
                l(L::Punctuation(Punctuation::Comma)),
                l(L::Conjunction),
                n(N::ModifierConjunct),
            ],
            ParseCost {
                precedence: 1,
                ..ParseCost::default()
            },
        );

        // The coordinated run fills one modifier slot on the nominal, binding
        // tighter than the rest of the modifier stack.
        self.add(
            RuleTag::NominalCoordinatedModifier,
            N::Nominal,
            [n(N::CoordinatedModifier), n(N::Nominal)],
        );

        // Head-list coordination: comma/Oxford extension of the existing binary
        // noun-phrase coordination. The open run is gathered on the dedicated
        // `NounPhraseList` nonterminal so a bare comma run never coordinates on
        // its own; only the Oxford close (`, and`/`, or` + a final member)
        // produces a coordinated noun phrase. Two-way `A and B` and un-comma'd
        // `A and B or C` chains already ride the existing binary rule.
        self.add(
            RuleTag::NounPhraseListSingle,
            N::NounPhraseList,
            [n(N::NounPhrase)],
        );
        self.add(
            RuleTag::NounPhraseListComma,
            N::NounPhraseList,
            [
                n(N::NounPhraseList),
                l(L::Punctuation(Punctuation::Comma)),
                n(N::NounPhrase),
            ],
        );
        self.add_with_cost(
            RuleTag::NounPhraseCoordinationOxford,
            N::NounPhrase,
            [
                n(N::NounPhraseList),
                l(L::Punctuation(Punctuation::Comma)),
                l(L::Conjunction),
                n(N::NounPhrase),
            ],
            ParseCost {
                precedence: 1,
                ..ParseCost::default()
            },
        );
    }

    /// Attachment points that *consume* the landed coordination nonterminals at
    /// positions other than the nominal head. Appended after
    /// [`Self::add_coordination_rules`] so every rule here takes a higher
    /// `RuleId` than the coordination machinery it reads.
    ///
    /// * **Predicative-adjective coordination** (Family C) — a closed
    ///   [`Nonterminal::CoordinatedModifier`] filling a copular or
    ///   intransitive-`be` adjective complement (`it's legendary and snow`,
    ///   `that's red or green`, `that are green and/or white`). The three rules
    ///   below cover the matrix copular remainder, the contracted relative
    ///   copular, and the non-contracted intransitive-`be` verb phrase;
    ///   lowering converts the modifier list into a
    ///   [`CoordinatedAdjectivePhrase`](crate::syntax::CoordinatedAdjectivePhrase),
    ///   rejecting any non-adjective conjunct so the attributive-only shapes
    ///   stay out of predicative position.
    fn add_coordination_consumer_rules(&mut self) {
        use EnglishLexicalSlot as L;
        use Expected::Lexical as l;
        use Expected::Nonterminal as n;
        use Nonterminal as N;

        // Family C: predicative-adjective coordination.
        self.add(
            RuleTag::VerbPhraseCoordinatedAdjective,
            N::VerbPhrase,
            [n(N::VerbPhrase), n(N::CoordinatedModifier)],
        );
        self.add(
            RuleTag::CopularRemainderCoordinatedAdjective,
            N::CopularRemainder,
            [n(N::CoordinatedModifier)],
        );
        self.add(
            RuleTag::RelativeContractedCopularCoordinatedAdjective,
            N::RelativeClause,
            [l(L::SubjectAuxiliary), n(N::CoordinatedModifier)],
        );

        // Family A: a power/toughness value complement on a characteristic
        // nominal (`base power and toughness X/X`). Mirrors the quantity
        // complement (`base power 2`) for the `N/N` token; the shared `base`
        // modifier and coordinated `power and toughness` heads ride the existing
        // nominal-modifier and noun-phrase coordination, with the value recorded
        // on the final characteristic.
        self.add_with_cost(
            RuleTag::NominalPowerToughnessComplement,
            N::Nominal,
            [n(N::Nominal), l(L::PowerToughness)],
            ParseCost {
                precedence: 1,
                ..ParseCost::default()
            },
        );
    }

    /// A premodified possessor: `[AdjP] [PossessiveNounPhrase]`, e.g. `the
    /// sacrificed creature's`. Registered append-last (see call site) so
    /// existing `RuleId`s are untouched; same cost as `NominalAdjective`.
    fn add_possessive_modifier_rules(&mut self) {
        use Expected::Nonterminal as n;
        use Nonterminal as N;

        self.add_with_cost(
            RuleTag::PossessiveNounAdjective,
            N::PossessiveNounPhrase,
            [n(N::AdjectivePhrase), n(N::PossessiveNounPhrase)],
            ParseCost {
                precedence: 1,
                ..ParseCost::default()
            },
        );
    }

    /// Registers the closed `come up heads`/`come up tails` coin-result
    /// predicate tail [CR#705.1,705.2] after every other rule in the
    /// grammar, mirroring `VerbPhraseParticle`'s structural placement. Its
    /// dot-1 gate (`accepts_predicate_prefix`) requires the narrow pending
    /// `Come` frame before either alternative is even predicted, so no other
    /// verb phrase can reach this production.
    fn add_coin_result_rules(&mut self) {
        use Expected::Lexical as l;
        use Expected::Nonterminal as n;
        use Nonterminal as N;

        for side in [
            crate::syntax::CoinSide::Heads,
            crate::syntax::CoinSide::Tails,
        ] {
            self.add(
                RuleTag::VerbPhraseCoinResult,
                N::VerbPhrase,
                [n(N::VerbPhrase), l(EnglishLexicalSlot::CoinResult(side))],
            );
        }
    }

    /// Registers the fronted `While <gerund clause>, <clause>.` production
    /// [CR#701.38d] after every other rule, including the coin-result
    /// predicate. The dot-1 gate in `accepts_predicate_prefix` requires
    /// `Features::Subordinator(While)` before `GerundClause` is even
    /// predicted, so this never cascades into a general fronted-gerund
    /// shape for other subordinators.
    fn add_while_gerund_rules(&mut self) {
        use Expected::Lexical as l;
        use Expected::Nonterminal as n;
        use Nonterminal as N;

        self.add(
            RuleTag::ClauseSubordinateGerundBefore,
            N::Clause,
            [
                l(EnglishLexicalSlot::Subordinator),
                n(N::GerundClause),
                l(EnglishLexicalSlot::Punctuation(Punctuation::Comma)),
                n(N::Clause),
            ],
        );
    }

    /// `kwgrant` round, Stage A: a parameterized keyword ability's symbol-cost
    /// argument fused onto its keyword-noun head in one production —
    /// `ward {2}`, `equip {1}`. The lexical head is one of the dedicated
    /// keyword-noun slots (never the ordinary noun slot), so an ordinary noun
    /// can never enter this rule; only a catalog-surface property (keyword
    /// atom membership) gates it.
    fn add_keyword_grant_rules(&mut self) {
        use Expected::Lexical as l;
        use Expected::Nonterminal as n;
        use Nonterminal as N;

        self.add(
            RuleTag::NominalKeywordSymbolArgument,
            N::Nominal,
            [
                l(EnglishLexicalSlot::SymbolArgumentKeywordNoun),
                l(EnglishLexicalSlot::OracleSymbol),
            ],
        );
        self.add(
            RuleTag::NominalKeywordSymbolArgument,
            N::Nominal,
            [
                l(EnglishLexicalSlot::SymbolArgumentKeywordNoun),
                l(EnglishLexicalSlot::SymbolSequence),
            ],
        );

        // Stage B: explicit `from` qualities (`protection from black`).
        self.add(
            RuleTag::PredicatedQualityFrom,
            N::PredicatedQualityFrom,
            [
                l(EnglishLexicalSlot::FromWord),
                l(EnglishLexicalSlot::ColorWord),
            ],
        );
        self.add(
            RuleTag::PredicatedQualityFrom,
            N::PredicatedQualityFrom,
            [l(EnglishLexicalSlot::FromWord), n(N::NounPhrase)],
        );
        self.add(
            RuleTag::PredicatedArgumentFromSingle,
            N::PredicatedArgumentFrom,
            [n(N::PredicatedQualityFrom)],
        );
        self.add(
            RuleTag::PredicatedArgumentFromExtend,
            N::PredicatedArgumentFrom,
            [
                n(N::PredicatedArgumentFrom),
                l(EnglishLexicalSlot::Conjunction),
                n(N::PredicatedQualityFrom),
            ],
        );
        self.add(
            RuleTag::NominalKeywordPredicatedArgument,
            N::Nominal,
            [
                l(EnglishLexicalSlot::ExplicitPredicatedKeywordNoun),
                n(N::PredicatedArgumentFrom),
            ],
        );

        // Stage C: the atom itself carries `from` (`Hexproof from black`).
        self.add(
            RuleTag::PredicatedQualityBare,
            N::PredicatedQualityBare,
            [l(EnglishLexicalSlot::ColorWord)],
        );
        self.add(
            RuleTag::PredicatedQualityBare,
            N::PredicatedQualityBare,
            [n(N::AdjectivePhrase)],
        );
        self.add(
            RuleTag::PredicatedQualityBare,
            N::PredicatedQualityBare,
            [n(N::NounPhrase)],
        );
        self.add(
            RuleTag::PredicatedArgumentBareSingle,
            N::PredicatedArgumentBare,
            [n(N::PredicatedQualityBare)],
        );
        self.add(
            RuleTag::PredicatedArgumentBareExtend,
            N::PredicatedArgumentBare,
            [
                n(N::PredicatedArgumentBare),
                l(EnglishLexicalSlot::Conjunction),
                n(N::PredicatedQualityFrom),
            ],
        );
        self.add(
            RuleTag::NominalKeywordAtomCarriedPredicatedArgument,
            N::Nominal,
            [
                l(EnglishLexicalSlot::AtomCarriedPredicatedKeywordNoun),
                n(N::PredicatedArgumentBare),
            ],
        );
    }
}

/// Whether `atom`'s canonical spelling ends in the standalone word `from`
/// (`Hexproof from`, not merely a spelling that happens to contain the
/// substring `from`). A catalog-surface property, checked once here and
/// reused by `grammar/ability.rs`'s keyword-line parser so the chart scanner
/// and the keyword-line parser never grow two subtly different definitions
/// of the atom-carried preposition — `kwgrant` round Stage C.
pub(crate) fn keyword_atom_carries_from(atom: &CatalogAtom) -> bool {
    atom.canonical().rsplit(' ').next() == Some("from")
}

fn lexical_word_matches(word: WordMatch, end: usize) -> Vec<LexicalMatch<Features, MeaningKey>> {
    let single = |features, meaning| {
        vec![LexicalMatch {
            end,
            features,
            meaning,
            local_cost: ParseCost::default(),
        }]
    };
    match word {
        WordMatch::Noun(noun) => {
            let form = noun_form(&noun);
            let Some(initial_sound) = noun_initial_sound(&noun) else {
                return Vec::new();
            };
            let adjunct = noun_adjunct_kind(&noun);
            vec![LexicalMatch {
                end,
                features: Features::Noun {
                    form,
                    initial_sound,
                    adjunct,
                    opaque: false,
                    recipient_passive_theme: matches!(
                        noun,
                        NounInstance::Singular(Noun::Word(Vocab::Damage))
                            | NounInstance::Plural(Noun::Word(Vocab::Damage))
                            | NounInstance::Mass(Noun::Word(Vocab::Damage))
                    ),
                },
                // `other` is scanned as a count noun only so the anaphoric
                // fused head `the other`/`the others` (the sibling of `the
                // rest`) can head a nominal. Everywhere else `other` is an
                // attributive adjective (`each other creature`, `all other
                // permanents`); dispreference the noun reading so the adjective
                // reading — equal in every structural cost — always wins in
                // modifier position, and the noun reading surfaces only as the
                // fused head, where no adjective reading completes.
                //
                // `one` is the same shape against the number-literal reading:
                // it is scanned as a count noun so the anaphoric fused head
                // (`a different one of those creatures`, `each one`) can head a
                // nominal, but dispreferenced so a competing `NumberLiteral`
                // quantity reading (`one card`, `one or more counters`) always
                // wins wherever both complete.
                // Productive agent nouns are dispreferred against explicit
                // lexical nouns. Thus technical words such as `player`,
                // `controller`, and `owner` keep their lexical identity, while
                // an unclaimed surface such as `voter` is verb-backed.
                local_cost: ParseCost {
                    reading_dispreference: u32::from(is_fused_head_noun(&noun))
                        + u32::from(is_derived_agent_noun(&noun)),
                    ..ParseCost::default()
                },
                meaning: MeaningKey::Noun(noun),
            }]
        }
        WordMatch::Verb(verb) => verb
            .verb
            .predicate_frames()
            .iter()
            .copied()
            .map(|frame| LexicalMatch {
                end,
                features: Features::Verb {
                    slot: verb.slot,
                    frame,
                    head_is_copular: verb.verb == Verb::Word(Vocab::Be),
                },
                meaning: MeaningKey::Verb(VerbAnalysis {
                    instance: verb.clone(),
                    frame,
                }),
                local_cost: ParseCost::default(),
            })
            .collect(),
        WordMatch::Adjective(adjective) => {
            let Some(initial_sound) = adjective_initial_sound(&adjective) else {
                return Vec::new();
            };
            single(
                Features::Adjective {
                    initial_sound,
                    comparison: adjective_comparison_state(&adjective),
                    card_orientation: false,
                },
                MeaningKey::Adjective(adjective),
            )
        }
        WordMatch::Adverb(adverb) | WordMatch::SentenceAdverbial(adverb) => {
            single(Features::None, MeaningKey::Adverb(adverb))
        }
        WordMatch::Pronoun(pronoun) => single(
            noun_phrase_features(pronoun.pronoun, Some(pronoun.case)),
            MeaningKey::Pronoun(pronoun),
        ),
        WordMatch::Auxiliary(auxiliary) => vec![LexicalMatch {
            end,
            features: Features::Auxiliary(auxiliary),
            meaning: MeaningKey::Auxiliary(auxiliary),
            local_cost: ParseCost {
                // `were`/`weren't` are ambiguous between indicative
                // Past{Third,Plural} and the subjunctive: prefer the
                // indicative reading whenever both are available (e.g. `they
                // were untapped`), leaving subjunctive as the only surviving
                // reading where no indicative subject agrees (`it were`).
                reading_dispreference: u32::from(matches!(
                    auxiliary.inflection,
                    crate::word::AuxiliaryInflection::PastSubjunctive
                )),
                ..ParseCost::default()
            },
        }],
    }
}

fn quantity_match(end: usize, quantity: QuantityKey) -> LexicalMatch<Features, MeaningKey> {
    LexicalMatch {
        end,
        features: Features::Quantity(quantity.features()),
        meaning: MeaningKey::Quantity(quantity),
        local_cost: ParseCost::default(),
    }
}

fn frequency_match(end: usize, frequency: FrequencyKey) -> LexicalMatch<Features, MeaningKey> {
    LexicalMatch {
        end,
        features: Features::None,
        meaning: MeaningKey::Frequency(frequency),
        local_cost: ParseCost::default(),
    }
}

fn literal_match(end: usize, literal: LiteralKey) -> LexicalMatch<Features, MeaningKey> {
    LexicalMatch {
        end,
        features: Features::None,
        meaning: MeaningKey::Literal(literal),
        local_cost: ParseCost::default(),
    }
}

fn pronoun_match(
    end: usize,
    pronoun: Pronoun,
    case: PronounCase,
) -> LexicalMatch<Features, MeaningKey> {
    let instance = PronounInstance { pronoun, case };
    LexicalMatch {
        end,
        features: noun_phrase_features(pronoun, Some(case)),
        meaning: MeaningKey::Pronoun(instance),
        local_cost: ParseCost::default(),
    }
}

/// A self-reference resolves to one card, but a joint `and` face (`Aang and
/// Katara`) names two creatures, and its text agrees plurally (`When Aang and
/// Katara enter, …`). The recognized name carries no number, so both
/// third-person agreements are offered; the verb's own inflection selects one
/// (`… enters` singular, `… enter` plural). The lowered [`ThisCardForm`] is
/// identical either way — the number lives on the verb — so an
/// agreement-neutral verb packs to one AST.
fn this_card_matches(end: usize, form: ThisCardForm) -> Vec<LexicalMatch<Features, MeaningKey>> {
    [Number::Singular, Number::Plural]
        .into_iter()
        .map(|number| LexicalMatch {
            end,
            features: Features::NounPhrase {
                agreement: Some(Agreement {
                    person: Person::Third,
                    number,
                }),
                pronoun_case: None,
                adjunct: None,
                set_exception: SetExceptionState::Ineligible,
            },
            meaning: MeaningKey::ThisCard(form),
            local_cost: this_card_cost(form),
        })
        .collect()
}

fn possessive_this_card_match(
    end: usize,
    form: ThisCardForm,
) -> LexicalMatch<Features, MeaningKey> {
    LexicalMatch {
        end,
        features: Features::PossessiveThisCard {
            agreement: Agreement {
                person: Person::Third,
                number: Number::Singular,
            },
        },
        meaning: MeaningKey::ThisCard(form),
        local_cost: this_card_cost(form),
    }
}

/// The tiebreak cost of reading tokens as a self-reference. A
/// non-self-reference reading pays nothing here and so wins any tie on the
/// structural fields; among self-references the fuller
/// [`ThisCardForm::FullName`] outranks a derived nickname.
fn this_card_cost(form: ThisCardForm) -> ParseCost {
    ParseCost {
        reading_dispreference: match form {
            ThisCardForm::FullName => 1,
            ThisCardForm::AbbreviatedName => 2,
        },
        ..ParseCost::default()
    }
}

fn noun_phrase_features(pronoun: Pronoun, case: Option<PronounCase>) -> Features {
    let agreement = match pronoun {
        Pronoun::You => Some(Agreement {
            person: Person::Second,
            number: Number::Singular,
        }),
        Pronoun::It(_) => Some(Agreement {
            person: Person::Third,
            number: Number::Singular,
        }),
        Pronoun::They => Some(Agreement {
            person: Person::Third,
            number: Number::Plural,
        }),
        Pronoun::EachOther | Pronoun::Itself | Pronoun::Himself | Pronoun::YoursAbsolute => None,
    };
    Features::NounPhrase {
        agreement,
        pronoun_case: case,
        adjunct: None,
        set_exception: SetExceptionState::Ineligible,
    }
}

const fn copula_agreement(auxiliary: AuxiliaryInstance) -> Option<CopulaAgreement> {
    if !matches!(auxiliary.auxiliary, Auxiliary::Be) {
        return None;
    }
    match auxiliary.inflection {
        AuxiliaryInflection::Present { person, number }
        | AuxiliaryInflection::Past { person, number } => {
            Some(CopulaAgreement::Indicative(Agreement { person, number }))
        }
        AuxiliaryInflection::PastSubjunctive => Some(CopulaAgreement::PastSubjunctive),
        AuxiliaryInflection::Base
        | AuxiliaryInflection::PresentParticiple
        | AuxiliaryInflection::PastParticiple => None,
    }
}

fn noun_form(noun: &NounInstance) -> NounForm {
    match noun {
        NounInstance::Singular(_) => NounForm::Singular,
        NounInstance::Plural(_) => NounForm::Plural,
        NounInstance::Mass(_) => NounForm::Mass,
    }
}

/// Whether this noun is a dual-reading lexeme scanned as a noun purely to
/// license an anaphoric fused head (`the other`/`the others`, `a different one
/// of those creatures`, `each one`). Its noun reading is dispreferenced so the
/// competing reading — the attributive adjective for `other`, the
/// `NumberLiteral` quantity for `one` — wins wherever both complete; see
/// [`lexical_word_matches`].
fn is_fused_head_noun(noun: &NounInstance) -> bool {
    let inner = match noun {
        NounInstance::Singular(noun) | NounInstance::Plural(noun) | NounInstance::Mass(noun) => {
            noun
        }
    };
    matches!(inner, Noun::Word(Vocab::Other | Vocab::One))
}

fn is_derived_agent_noun(noun: &NounInstance) -> bool {
    let inner = match noun {
        NounInstance::Singular(noun) | NounInstance::Plural(noun) | NounInstance::Mass(noun) => {
            noun
        }
    };
    matches!(inner, Noun::Agentive(_))
}

fn noun_adjunct_kind(noun: &NounInstance) -> Option<BareNominalAdjunct> {
    let noun = match noun {
        NounInstance::Singular(noun) | NounInstance::Plural(noun) | NounInstance::Mass(noun) => {
            noun
        }
    };
    noun.bare_nominal_adjunct()
}

fn parse_symbol_sequence(source: &str) -> Option<Vec<OracleSymbol>> {
    let mut rest = source;
    let mut symbols = Vec::new();
    while let Some(close) = rest.find('}') {
        let end = close + 1;
        symbols.push(OracleSymbol::new(rest.get(..end)?)?);
        rest = rest.get(end..)?;
    }
    (rest.is_empty() && !symbols.is_empty()).then_some(symbols)
}

fn noun_initial_sound(noun: &NounInstance) -> Option<InitialSound> {
    let noun_identity = match noun {
        NounInstance::Singular(noun) | NounInstance::Plural(noun) | NounInstance::Mass(noun) => {
            noun
        }
    };
    match noun_identity {
        Noun::Word(vocab) => Some(Vocabulary::new().initial_sound(*vocab)),
        Noun::Catalog(atom) => Some(surface_initial_sound(atom.canonical())),
        Noun::Die(_) => Some(InitialSound::Consonant),
        Noun::Gerund(_) | Noun::Agentive(_) => Vocabulary::new()
            .render_noun(noun)
            .map(|surface| surface_initial_sound(&surface)),
        Noun::Opaque(opaque) => Some(surface_initial_sound(opaque.spelling())),
    }
}

fn adjective_initial_sound(adjective: &Adjective) -> Option<InitialSound> {
    match adjective {
        Adjective::Word(vocab) => Some(Vocabulary::new().initial_sound(*vocab)),
        _ => Vocabulary::new()
            .render_adjective(adjective)
            .map(|surface| surface_initial_sound(&surface)),
    }
}

fn adjective_comparison_state(adjective: &Adjective) -> AdjectiveComparisonState {
    match adjective {
        // Comparison capability is vocabulary metadata (`Vocab::comparison`),
        // not a spelling match; the class it carries decides which completions
        // are legal (see `AdjectiveComparisonState`).
        Adjective::Word(word) => word
            .comparison()
            .map_or(AdjectiveComparisonState::NotComparative, |comparison| {
                AdjectiveComparisonState::Pending(comparison.class())
            }),
        _ => AdjectiveComparisonState::NotComparative,
    }
}

fn parse_power_toughness(surface: &str) -> Option<PowerToughness> {
    let (power, toughness) = surface.split_once('/')?;
    Some(PowerToughness {
        power: parse_signed_scalar(power)?,
        toughness: parse_signed_scalar(toughness)?,
    })
}

fn parse_signed_scalar(surface: &str) -> Option<crate::syntax::SignedScalar> {
    use crate::syntax::ScalarSign;
    use crate::syntax::ScalarValue;

    let (sign, body) = if let Some(body) = surface.strip_prefix('+') {
        (ScalarSign::Plus, body)
    } else if let Some(body) = surface
        .strip_prefix('-')
        .or_else(|| surface.strip_prefix('−'))
    {
        (ScalarSign::Minus, body)
    } else {
        (ScalarSign::None, surface)
    };
    let value = match body {
        "X" => ScalarValue::X,
        "*" => ScalarValue::Star,
        _ => ScalarValue::Integer(body.parse().ok()?),
    };
    Some(crate::syntax::SignedScalar { sign, value })
}

/// Dot-1 gate for `PossessiveNounAdjective`: the completed child-0 adjective
/// phrase must be a plain, comparative-pending, or complete adjective, never
/// a measured or card-orientation one. These are categorical facts available
/// from child 0, so they are checked here rather than only in `reduce`,
/// which would otherwise launch avoidable predictions after an inadmissible
/// prefix. The `determined` gate needs child 1 and stays in `reduce`. Every
/// other tag/dot is unconstrained here and remains governed by
/// `clause::accepts_predicate_prefix`.
/// Dot-2 gate for the Stage B/C predicated-argument list extensions
/// (`PredicatedArgumentFromExtend`/`PredicatedArgumentBareExtend`): after the
/// list and the conjunction complete, only `and` may extend the list before
/// the next quality is predicted. `or` is the confirmed deferral (five Stage
/// B rows; `renderer.rs`'s `KeywordArgument::Predicated` hardcodes `" and "`
/// between qualities, so an `or` cannot round-trip and must never be
/// normalized to `and`). A categorical fact available from the just-completed
/// conjunction child, so it is checked here (before the next quality's
/// prediction) rather than only in `reduce` — the reduce arm duplicates this
/// as a defensive invariant, per the plan's prefix-gate discipline
/// [`kwgrant` round].
fn accepts_keyword_grant_prefix(
    tag: RuleTag,
    completed_children: usize,
    latest_child: &Features,
) -> bool {
    if !matches!(
        tag,
        RuleTag::PredicatedArgumentFromExtend | RuleTag::PredicatedArgumentBareExtend
    ) || completed_children != 2
    {
        return true;
    }
    matches!(
        latest_child,
        Features::Conjunction(crate::syntax::PredicateConjunction::And)
    )
}

fn accepts_possessive_modifier_prefix(
    tag: RuleTag,
    completed_children: usize,
    latest_child: &Features,
) -> bool {
    if tag != RuleTag::PossessiveNounAdjective || completed_children != 1 {
        return true;
    }
    matches!(
        latest_child,
        Features::Adjective {
            comparison: AdjectiveComparisonState::NotComparative
                | AdjectiveComparisonState::Pending(_)
                | AdjectiveComparisonState::Complete,
            card_orientation: false,
            ..
        }
    )
}

fn noun_phrase_accepts_set_exception(features: &Features) -> bool {
    matches!(
        features,
        Features::NounPhrase {
            set_exception: SetExceptionState::Host,
            ..
        }
    )
}

fn noun_phrase_is_closed_set_exception(features: &Features) -> bool {
    matches!(
        features,
        Features::NounPhrase {
            set_exception: SetExceptionState::Closed,
            ..
        }
    )
}

/// Dot-1 gate for recursive noun-phrase set exceptions and the neighbouring
/// coordination rules. Only an `all`/`each` set may predict `except`; a
/// completed exception cannot grow another exception or become the first
/// member of an outer coordination.
fn accepts_set_exception_prefix(
    tag: RuleTag,
    completed_children: usize,
    latest_child: &Features,
) -> bool {
    if matches!(
        tag,
        RuleTag::NounPhraseSetExceptionBare | RuleTag::NounPhraseSetExceptionFor
    ) && completed_children == 1
    {
        return noun_phrase_accepts_set_exception(latest_child);
    }

    if matches!(
        tag,
        RuleTag::NounPhraseCoordination
            | RuleTag::NounPhraseAdditiveCoordination
            | RuleTag::NounPhraseListSingle
    ) && completed_children == 1
    {
        return !noun_phrase_is_closed_set_exception(latest_child);
    }

    true
}

#[allow(clippy::too_many_lines, reason = "reduce matches on all rule tags")]
fn reduce(
    tag: RuleTag,
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
) -> Option<Reduction<Features>> {
    let features = match tag {
        RuleTag::QuantityExact
        | RuleTag::QuantityAtLeast
        | RuleTag::QuantityOr
        | RuleTag::QuantityX
        | RuleTag::QuantityBoth
        | RuleTag::QuantityUpTo
        | RuleTag::QuantityThatMany
        | RuleTag::QuantityThatMuch
        | RuleTag::QuantityMoreThan
        | RuleTag::QuantityFewerThan
        | RuleTag::DeterminerClosed
        | RuleTag::DeterminerTarget
        | RuleTag::DeterminerQuantifiedTarget
        | RuleTag::DeterminerQuantity
        | RuleTag::DeterminerPossessiveThisCard => reduce_quantity_or_determiner(tag, children)?,
        RuleTag::FrequencyPhrase | RuleTag::FrequencyPhraseAdverb => Features::None,
        RuleTag::PossessiveNounBase
        | RuleTag::PossessiveNounDetermined
        | RuleTag::DeterminerPossessiveNoun
        | RuleTag::PossessiveNounAdjective => reduce_possessive_noun_phrase(tag, children)?,
        RuleTag::Adjective
        | RuleTag::AdjectivePhrase
        | RuleTag::AdjectivePhraseFaceUp
        | RuleTag::AdjectivePhraseFaceDown
        | RuleTag::AdjectivePhraseComparison
        | RuleTag::AdjectivePhraseDegreeMeasure
        | RuleTag::ComparisonStandard
        | RuleTag::ComparisonThan
        | RuleTag::ComparisonThanOrEqualTo
        | RuleTag::Noun
        | RuleTag::NominalNoun
        | RuleTag::NominalAdjective
        | RuleTag::NominalNounModifier
        | RuleTag::NominalCombatStepName
        | RuleTag::NominalNegatedModifier
        | RuleTag::NominalQuantityModifier
        | RuleTag::NominalPowerToughnessModifier
        | RuleTag::NominalDeterminer
        | RuleTag::NominalPrepositional
        | RuleTag::NominalInfinitive
        | RuleTag::NominalQuantityComplement
        | RuleTag::NominalKeywordSymbolArgument
        | RuleTag::PredicatedQualityFrom
        | RuleTag::PredicatedArgumentFromSingle
        | RuleTag::PredicatedArgumentFromExtend
        | RuleTag::NominalKeywordPredicatedArgument
        | RuleTag::PredicatedQualityBare
        | RuleTag::PredicatedArgumentBareSingle
        | RuleTag::PredicatedArgumentBareExtend
        | RuleTag::NominalKeywordAtomCarriedPredicatedArgument
        | RuleTag::NominalPowerToughnessComplement
        | RuleTag::NominalRelative
        | RuleTag::NominalReducedRecipientPassive
        | RuleTag::NominalPostpositiveAdjective
        | RuleTag::NominalComparison
        | RuleTag::NominalDevotion
        | RuleTag::DevotionColorSingle
        | RuleTag::DevotionColorPair
        | RuleTag::NominalTimesClause
        | RuleTag::ModifierConjunctAdjective
        | RuleTag::ModifierConjunctNoun
        | RuleTag::ModifierConjunctNegated
        | RuleTag::ModifierListSingle
        | RuleTag::ModifierListComma
        | RuleTag::CoordinatedModifierConjoined
        | RuleTag::CoordinatedModifierOxford
        | RuleTag::NominalCoordinatedModifier => reduce_nominal(tag, children)?,
        RuleTag::NounPhraseNominal
        | RuleTag::NounPhraseSetExceptionBare
        | RuleTag::NounPhraseSetExceptionFor
        | RuleTag::ReducedRecipientPassiveTheme
        | RuleTag::NounPhraseSubjectPronoun
        | RuleTag::NounPhraseObjectPronoun
        | RuleTag::NounPhraseReciprocal
        | RuleTag::NounPhraseQuantity
        | RuleTag::NounPhraseThisCard
        | RuleTag::NounPhraseFullThisCard
        | RuleTag::NounPhrasePossessiveThisCard
        | RuleTag::NounPhraseDemonstrative
        | RuleTag::NounPhrasePartitive
        | RuleTag::NounPhraseEachPartitive
        | RuleTag::NounPhraseAnyNumberOf
        | RuleTag::NounPhraseCoordination
        | RuleTag::NounPhraseAdditiveCoordination
        | RuleTag::NounPhraseListSingle
        | RuleTag::NounPhraseListComma
        | RuleTag::NounPhraseCoordinationOxford
        | RuleTag::NounPhraseMinus
        | RuleTag::NounPhraseHalf
        | RuleTag::NounPhraseHalfRoundedUp
        | RuleTag::NounPhraseHalfRoundedDown
        | RuleTag::PrepositionalPhrase
        | RuleTag::PrepositionalObject => reduce_phrase(tag, children)?,
        RuleTag::Verb
        | RuleTag::VerbPhraseBase
        | RuleTag::VerbPhraseAuxiliary
        | RuleTag::VerbPhraseAuxiliaryProform
        | RuleTag::VerbPhraseDirectObject
        | RuleTag::VerbPhraseIndirectObject
        | RuleTag::VerbPhraseAdjective
        | RuleTag::VerbPhrasePrepositional
        | RuleTag::VerbPhraseExceptBy
        | RuleTag::VerbPhraseInfinitive
        | RuleTag::VerbPhraseAdverb
        | RuleTag::VerbPhrasePreverbAdverb
        | RuleTag::VerbPhraseParticle
        | RuleTag::VerbPhraseCoinResult
        | RuleTag::VerbPhraseFrequency
        | RuleTag::VerbPhraseAbility
        | RuleTag::VerbPhraseQuotedAbility
        | RuleTag::VerbPhraseQuotedAbilityCoordination
        | RuleTag::VerbPhraseAbilityQuotedCoordination
        | RuleTag::VerbPhraseOracleSymbol
        | RuleTag::VerbPhraseSymbolSequence
        | RuleTag::VerbPhraseManaAmountCoordination
        | RuleTag::ManaAmountSymbol
        | RuleTag::ManaAmountSequence
        | RuleTag::ManaAmountListSingle
        | RuleTag::ManaAmountListComma
        | RuleTag::ManaAmountCoordination
        | RuleTag::ManaAmountCoordinationOxford
        | RuleTag::VerbPhrasePowerToughness
        | RuleTag::VerbPhraseQuantity
        | RuleTag::ReducedRecipientPassiveNominalAdjunct
        | RuleTag::InfinitiveTo
        | RuleTag::InfinitiveNotTo
        | RuleTag::GerundClauseBase
        | RuleTag::GerundClauseSubordinateAfter
        | RuleTag::SimpleClauseSubject
        | RuleTag::SimpleClauseSubjectDistributiveEach
        | RuleTag::SimpleClauseContractedSubject
        | RuleTag::SimpleClauseSubjectless
        | RuleTag::ClauseSimple
        | RuleTag::ClauseElliptical
        | RuleTag::ClauseCoordination
        | RuleTag::ClauseCoordinationComma
        | RuleTag::ClauseCoordinationAsyndetic
        | RuleTag::ClauseAdverbBefore
        | RuleTag::ClauseSentenceAdverbialBefore
        | RuleTag::ClausePrepositionalBefore
        | RuleTag::ClauseSubordinateBefore
        | RuleTag::ClauseSubordinateGerundBefore
        | RuleTag::ClauseSubordinateAfterElliptical
        | RuleTag::ClauseSubordinateAfter
        | RuleTag::ClauseSubordinateAfterComma
        | RuleTag::ClauseSubordinateAfterInfinitive
        | RuleTag::ClauseExistential
        | RuleTag::CopularRemainderNoun
        | RuleTag::CopularRemainderAdjective
        | RuleTag::CopularRemainderPrepositional
        | RuleTag::CopularRemainderPowerToughness
        | RuleTag::CopularRemainderPrepositionalAdjunct
        | RuleTag::CopularRemainderAdverb
        | RuleTag::CopularRemainderNegated
        | RuleTag::CopularRemainderDistributiveEach
        | RuleTag::ClauseCopular
        | RuleTag::ClauseContractedCopular
        | RuleTag::ClauseVariableValueConstraint
        | RuleTag::RelativeObject
        | RuleTag::RelativeObjectContractedSubject
        | RuleTag::RelativeSubjectContractedAuxiliary
        | RuleTag::RelativeSubject
        | RuleTag::RelativeSubjectDistributiveEach
        | RuleTag::RelativeContractedCopularNoun
        | RuleTag::RelativeContractedCopularAdjective
        | RuleTag::RelativeContractedCopularPrepositional
        | RuleTag::ClauseExcepted
        | RuleTag::ClauseRestrictionRun
        | RuleTag::ClauseRestrictionMember
        | RuleTag::ExceptionRiderSingle
        | RuleTag::ExceptionRiderConjoined
        | RuleTag::ExceptionRiderComma
        | RuleTag::ExceptionRiderOxford
        | RuleTag::VerbPhraseCausative
        | RuleTag::VerbPhraseCoordinatedAdjective
        | RuleTag::CopularRemainderCoordinatedAdjective
        | RuleTag::RelativeContractedCopularCoordinatedAdjective
        | RuleTag::Sentence => clause::reduce_clause(tag, children)?,
        RuleTag::NounOpaque => opacity::reduce_opacity(tag, children)?,
    };
    Some(Reduction {
        features,
        local_cost: clause::reduction_cost(tag, children),
    })
}

fn reduce_possessive_noun_phrase(
    tag: RuleTag,
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
) -> Option<Reduced> {
    match tag {
        RuleTag::PossessiveNounBase => {
            let Features::Noun {
                form,
                initial_sound,
                ..
            } = children.first()?.features
            else {
                return None;
            };
            Some(Features::PossessiveNounPhrase {
                form: *form,
                initial_sound: *initial_sound,
                determined: false,
            })
        }
        RuleTag::PossessiveNounDetermined => {
            let Features::Determiner {
                cardinality,
                article,
                ..
            } = children.first()?.features
            else {
                return None;
            };
            let Features::PossessiveNounPhrase {
                form,
                initial_sound,
                determined,
            } = children.get(1)?.features
            else {
                return None;
            };
            if *determined
                || !cardinality_accepts(*cardinality, *form)
                || !article_accepts(*article, *initial_sound)
            {
                return None;
            }
            Some(Features::PossessiveNounPhrase {
                form: *form,
                initial_sound: *initial_sound,
                determined: true,
            })
        }
        RuleTag::DeterminerPossessiveNoun => Some(Features::Determiner {
            cardinality: Cardinality::Unconstrained,
            article: None,
            set_exception_host: false,
        }),
        RuleTag::PossessiveNounAdjective => {
            let Features::Adjective {
                initial_sound,
                comparison:
                    AdjectiveComparisonState::NotComparative
                    | AdjectiveComparisonState::Pending(_)
                    | AdjectiveComparisonState::Complete,
                card_orientation: false,
            } = children.first()?.features
            else {
                return None;
            };
            let Features::PossessiveNounPhrase {
                form,
                determined: false,
                ..
            } = children.get(1)?.features
            else {
                return None;
            };
            Some(Features::PossessiveNounPhrase {
                form: *form,
                initial_sound: *initial_sound,
                determined: false,
            })
        }
        _ => None,
    }
}

type Reduced = Features;

fn reduce_quantity_or_determiner(
    tag: RuleTag,
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
) -> Option<Reduced> {
    match tag {
        RuleTag::QuantityExact => {
            let Features::Number { is_one } = children.first()?.features else {
                return None;
            };
            Some(Features::Quantity(number_quantity_features(*is_one)))
        }
        RuleTag::QuantityAtLeast
        | RuleTag::QuantityOr
        | RuleTag::QuantityX
        | RuleTag::QuantityBoth
        | RuleTag::QuantityMoreThan
        | RuleTag::QuantityFewerThan
        | RuleTag::QuantityThatMany
        | RuleTag::QuantityThatMuch => {
            let Features::Quantity(features) = children.first()?.features else {
                return None;
            };
            Some(Features::Quantity(*features))
        }
        RuleTag::QuantityUpTo => {
            let Features::Number { is_one } = children.get(2)?.features else {
                return None;
            };
            Some(Features::Quantity(number_quantity_features(*is_one)))
        }
        RuleTag::DeterminerClosed => {
            let Features::Determiner {
                cardinality,
                article,
                set_exception_host,
            } = children.first()?.features
            else {
                return None;
            };
            Some(Features::Determiner {
                cardinality: *cardinality,
                article: *article,
                set_exception_host: *set_exception_host,
            })
        }
        RuleTag::DeterminerTarget => Some(Features::Determiner {
            cardinality: Cardinality::SingularCount,
            article: None,
            set_exception_host: false,
        }),
        RuleTag::DeterminerQuantifiedTarget => {
            let Features::Quantity(QuantityFeatures { cardinality, .. }) =
                children.first()?.features
            else {
                return None;
            };
            Some(Features::Determiner {
                cardinality: target_cardinality(*cardinality),
                article: None,
                set_exception_host: false,
            })
        }
        RuleTag::DeterminerQuantity => {
            let Features::Quantity(QuantityFeatures { cardinality, .. }) =
                children.first()?.features
            else {
                return None;
            };
            Some(Features::Determiner {
                cardinality: *cardinality,
                article: None,
                set_exception_host: false,
            })
        }
        RuleTag::DeterminerPossessiveThisCard => {
            let Features::PossessiveThisCard { .. } = children.first()?.features else {
                return None;
            };
            Some(Features::Determiner {
                cardinality: Cardinality::Unconstrained,
                article: None,
                set_exception_host: false,
            })
        }
        _ => None,
    }
}

/// Features for an arithmetic value expression: a third-person singular numeric
/// value with no pronoun case or bare-nominal adjunct.
const fn arithmetic_value_features() -> Features {
    Features::NounPhrase {
        agreement: Some(Agreement {
            person: Person::Third,
            number: Number::Singular,
        }),
        pronoun_case: None,
        adjunct: None,
        set_exception: SetExceptionState::Ineligible,
    }
}

const fn number_cardinality(is_one: bool) -> Cardinality {
    if is_one { Cardinality::SingularOrMass } else { Cardinality::PluralOrMass }
}

const fn number_quantity_features(is_one: bool) -> QuantityFeatures {
    QuantityFeatures {
        cardinality: number_cardinality(is_one),
        standalone_number: if is_one { Number::Singular } else { Number::Plural },
    }
}

const fn target_cardinality(cardinality: Cardinality) -> Cardinality {
    match cardinality {
        Cardinality::SingularOrMass => Cardinality::SingularCount,
        Cardinality::PluralOrMass => Cardinality::PluralCount,
        other => other,
    }
}

#[allow(
    clippy::too_many_lines,
    reason = "quantity/reduction mapping is intentionally long"
)]
fn reduce_nominal(tag: RuleTag, children: &[Child<'_, EnglishGrammar<'_, '_>>]) -> Option<Reduced> {
    match tag {
        RuleTag::Adjective | RuleTag::Noun => Some(propagate(children.first()?)),
        RuleTag::AdjectivePhraseFaceUp | RuleTag::AdjectivePhraseFaceDown => {
            Some(Features::Adjective {
                initial_sound: InitialSound::Consonant,
                comparison: AdjectiveComparisonState::NotComparative,
                card_orientation: true,
            })
        }
        RuleTag::AdjectivePhrase => {
            let Features::Adjective { .. } = children.first()?.features else {
                return None;
            };
            Some(children.first()?.features.clone())
        }
        RuleTag::AdjectivePhraseComparison => {
            let Features::Adjective {
                initial_sound,
                comparison: AdjectiveComparisonState::Pending(_),
                card_orientation,
            } = children.first()?.features
            else {
                return None;
            };
            Some(Features::Adjective {
                initial_sound: *initial_sound,
                comparison: AdjectiveComparisonState::Complete,
                card_orientation: *card_orientation,
            })
        }
        RuleTag::AdjectivePhraseDegreeMeasure => {
            let Features::Number { .. } = children.first()?.features else {
                return None;
            };
            let Features::Adjective {
                initial_sound,
                // Only the `N or <word>` comparison class takes a degree
                // measure; `other` (`ThanOnly`) must not — see
                // `AdjectiveComparisonState`.
                comparison:
                    AdjectiveComparisonState::Pending(AdjectiveComparisonClass::OrComparative),
                card_orientation: false,
            } = children.get(1)?.features
            else {
                return None;
            };
            Some(Features::Adjective {
                // Inert: `Measured` never reaches an article-bearing position
                // (`nominal_with_prefix` rejects it), so this is carried, not
                // used.
                initial_sound: *initial_sound,
                comparison: AdjectiveComparisonState::Measured,
                card_orientation: false,
            })
        }
        RuleTag::ComparisonStandard
        | RuleTag::ComparisonThan
        | RuleTag::ComparisonThanOrEqualTo => Some(Features::None),
        RuleTag::NominalNoun => {
            let child = children.first()?;
            let Features::Noun {
                form,
                initial_sound,
                adjunct,
                opaque,
                recipient_passive_theme,
            } = child.features
            else {
                return None;
            };
            Some(Features::Nominal {
                form: *form,
                initial_sound: *initial_sound,
                determined: false,
                modified: false,
                leading_opacity: false,
                attachment: NominalAttachmentPhase::Open,
                comparison: AdjectiveComparisonState::NotComparative,
                adjunct: *adjunct,
                opaque_head: *opaque,
                set_exception_host: false,
                recipient_passive_theme: *recipient_passive_theme,
            })
        }
        RuleTag::NominalAdjective => {
            let Features::Adjective {
                initial_sound,
                comparison,
                card_orientation: false,
            } = children.first()?.features
            else {
                return None;
            };
            nominal_with_prefix(children.get(1)?, *initial_sound, false, *comparison)
        }
        RuleTag::NominalCombatStepName => {
            // Mirrors `NominalNoun`'s Noun→Nominal base case (this rule
            // *creates* a nominal from terminals, not `NominalNounModifier`,
            // which prepends onto an existing one). `initial_sound` is
            // overridden to `Consonant`: the phrase's true leftmost token is
            // `declare`, not the vowel-initial `attackers`.
            let Features::Noun { form, adjunct, .. } = children.get(2)?.features else {
                return None;
            };
            Some(Features::Nominal {
                form: *form,
                initial_sound: InitialSound::Consonant,
                determined: false,
                modified: false,
                leading_opacity: false,
                attachment: NominalAttachmentPhase::Open,
                comparison: AdjectiveComparisonState::NotComparative,
                adjunct: *adjunct,
                opaque_head: false,
                set_exception_host: false,
                recipient_passive_theme: false,
            })
        }
        RuleTag::NominalNounModifier => {
            let Features::Noun {
                initial_sound,
                opaque: modifier_opaque,
                ..
            } = children.first()?.features
            else {
                return None;
            };
            let Features::Nominal { opaque_head, .. } = children.get(1)?.features else {
                return None;
            };
            if *opaque_head && !*modifier_opaque {
                return None;
            }
            nominal_with_prefix(
                children.get(1)?,
                *initial_sound,
                false,
                AdjectiveComparisonState::NotComparative,
            )
        }
        RuleTag::NominalQuantityModifier => {
            // The quantity class keeps a fixed consonant onset. The corpus prints no
            // vowel-onset quantity modifier under an indefinite article in either
            // direction (`an eight …`, `a eight …`, `a one …`, `an one …`: zero
            // supported witnesses each), and deriving the sound here would mean
            // widening `QuantityFeatures`, which sits inside the Earley item key
            // (`ItemKey::prefix_features`) and is consumed at every quantity
            // position in the grammar. The divergence from
            // `Renderer::modifier_initial_sound`'s spelling-derived quantity arm
            // is known, unreachable on the supported corpus, and left as ticketed
            // residue.
            nominal_with_prefix(
                children.get(1)?,
                InitialSound::Consonant,
                false,
                AdjectiveComparisonState::NotComparative,
            )
        }
        RuleTag::NominalPowerToughnessModifier => {
            let Features::PowerToughness { initial_sound } = children.first()?.features else {
                return None;
            };
            nominal_with_prefix(
                children.get(1)?,
                *initial_sound,
                false,
                AdjectiveComparisonState::NotComparative,
            )
        }
        RuleTag::NominalNegatedModifier => {
            // The `non-` prefix fixes the phrase's initial sound to a consonant
            // (`a nonland permanent`, never `an`), regardless of the base.
            nominal_with_prefix(
                children.get(1)?,
                InitialSound::Consonant,
                false,
                AdjectiveComparisonState::NotComparative,
            )
        }
        RuleTag::NominalDeterminer => {
            let Features::Determiner {
                cardinality,
                article,
                set_exception_host,
            } = children.first()?.features
            else {
                return None;
            };
            let Features::Nominal {
                form,
                initial_sound,
                determined,
                modified,
                attachment,
                comparison,
                adjunct,
                opaque_head,
                recipient_passive_theme,
                ..
            } = children.get(1)?.features
            else {
                return None;
            };
            if *determined
                || !cardinality_accepts(*cardinality, *form)
                || !article_accepts(*article, *initial_sound)
            {
                return None;
            }
            Some(Features::Nominal {
                form: *form,
                initial_sound: *initial_sound,
                determined: true,
                modified: *modified,
                leading_opacity: false,
                attachment: *attachment,
                comparison: *comparison,
                adjunct: *adjunct,
                opaque_head: *opaque_head,
                set_exception_host: *set_exception_host,
                recipient_passive_theme: *recipient_passive_theme,
            })
        }
        RuleTag::NominalPrepositional => {
            let Features::Nominal {
                form,
                initial_sound,
                determined,
                modified,
                leading_opacity,
                attachment,
                comparison,
                adjunct,
                opaque_head,
                set_exception_host,
                recipient_passive_theme,
            } = children.first()?.features
            else {
                return None;
            };
            let Features::PrepositionalPhrase {
                nominal_attachment: true,
                ..
            } = children.get(1)?.features
            else {
                return None;
            };
            if matches!(
                attachment,
                NominalAttachmentPhase::ReducedRecipientPassive
                    | NominalAttachmentPhase::PostpositiveAdjective
                    | NominalAttachmentPhase::Comparison
            ) {
                return None;
            }
            Some(Features::Nominal {
                form: *form,
                initial_sound: *initial_sound,
                determined: *determined,
                modified: *modified,
                leading_opacity: *leading_opacity,
                attachment: NominalAttachmentPhase::Prepositional,
                comparison: *comparison,
                adjunct: *adjunct,
                opaque_head: *opaque_head,
                set_exception_host: *set_exception_host,
                recipient_passive_theme: *recipient_passive_theme,
            })
        }
        RuleTag::NominalInfinitive => {
            let Features::Nominal {
                form,
                initial_sound,
                determined,
                modified,
                leading_opacity,
                attachment,
                comparison,
                adjunct,
                opaque_head,
                set_exception_host,
                recipient_passive_theme,
            } = children.first()?.features
            else {
                return None;
            };
            if !matches!(children.get(1)?.features, Features::InfinitiveClause)
                || matches!(
                    attachment,
                    NominalAttachmentPhase::ReducedRecipientPassive
                        | NominalAttachmentPhase::PostpositiveAdjective
                        | NominalAttachmentPhase::Comparison
                )
            {
                return None;
            }
            Some(Features::Nominal {
                form: *form,
                initial_sound: *initial_sound,
                determined: *determined,
                modified: *modified,
                leading_opacity: *leading_opacity,
                attachment: NominalAttachmentPhase::Prepositional,
                comparison: *comparison,
                adjunct: *adjunct,
                opaque_head: *opaque_head,
                set_exception_host: *set_exception_host,
                recipient_passive_theme: *recipient_passive_theme,
            })
        }
        RuleTag::NominalKeywordSymbolArgument => {
            let Features::Noun {
                form: NounForm::Mass,
                initial_sound,
                adjunct,
                ..
            } = children.first()?.features
            else {
                return None;
            };
            // The second child is the terminal symbol/symbol-sequence match;
            // it carries `Features::None` and no reduce-time validation
            // beyond that shape, per the grammar mechanism section.
            if !matches!(children.get(1)?.features, Features::None) {
                return None;
            }
            Some(Features::Nominal {
                form: NounForm::Mass,
                initial_sound: *initial_sound,
                determined: false,
                modified: false,
                leading_opacity: false,
                attachment: NominalAttachmentPhase::Open,
                comparison: AdjectiveComparisonState::NotComparative,
                adjunct: *adjunct,
                opaque_head: false,
                set_exception_host: false,
                recipient_passive_theme: false,
            })
        }
        RuleTag::PredicatedQualityFrom | RuleTag::PredicatedQualityBare => {
            Some(Features::PredicatedQuality)
        }
        RuleTag::PredicatedArgumentFromSingle | RuleTag::PredicatedArgumentBareSingle => {
            if !matches!(children.first()?.features, Features::PredicatedQuality) {
                return None;
            }
            Some(Features::PredicatedArgument)
        }
        RuleTag::PredicatedArgumentFromExtend | RuleTag::PredicatedArgumentBareExtend => {
            if !matches!(children.first()?.features, Features::PredicatedArgument) {
                return None;
            }
            // Defensive invariant, mirroring the `accepts_prefix` gate: only
            // `and` may extend the list; an `or` must never be normalized
            // away (the confirmed Stage B deferral).
            if !matches!(
                children.get(1)?.features,
                Features::Conjunction(crate::syntax::PredicateConjunction::And)
            ) {
                return None;
            }
            if !matches!(children.get(2)?.features, Features::PredicatedQuality) {
                return None;
            }
            Some(Features::PredicatedArgument)
        }
        RuleTag::NominalKeywordPredicatedArgument
        | RuleTag::NominalKeywordAtomCarriedPredicatedArgument => {
            let Features::Noun {
                form: NounForm::Mass,
                initial_sound,
                adjunct,
                ..
            } = children.first()?.features
            else {
                return None;
            };
            if !matches!(children.get(1)?.features, Features::PredicatedArgument) {
                return None;
            }
            Some(Features::Nominal {
                form: NounForm::Mass,
                initial_sound: *initial_sound,
                determined: false,
                modified: false,
                leading_opacity: false,
                attachment: NominalAttachmentPhase::Open,
                comparison: AdjectiveComparisonState::NotComparative,
                adjunct: *adjunct,
                opaque_head: false,
                set_exception_host: false,
                recipient_passive_theme: false,
            })
        }
        RuleTag::NominalQuantityComplement
        | RuleTag::NominalRelative
        | RuleTag::NominalReducedRecipientPassive => {
            let Features::Nominal {
                form,
                initial_sound,
                determined,
                modified,
                leading_opacity,
                attachment,
                comparison,
                adjunct,
                opaque_head,
                set_exception_host,
                recipient_passive_theme,
            } = children.first()?.features
            else {
                return None;
            };
            if matches!(
                attachment,
                NominalAttachmentPhase::ReducedRecipientPassive
                    | NominalAttachmentPhase::PostpositiveAdjective
                    | NominalAttachmentPhase::Comparison
            ) {
                return None;
            }
            if tag == RuleTag::NominalRelative
                && let Features::RelativeClause {
                    gap: RelativeGap::Subject,
                    antecedent_agreement: Some(agreement),
                    ..
                } = children.get(1)?.features
                && agreement.number
                    != match form {
                        NounForm::Plural => Number::Plural,
                        NounForm::Singular | NounForm::Mass => Number::Singular,
                    }
            {
                return None;
            }
            if tag == RuleTag::NominalReducedRecipientPassive {
                if *attachment == NominalAttachmentPhase::RelativeBareCopula {
                    return None;
                }
                let Features::VerbPhrase {
                    form: PredicateForm::PastParticiple,
                    passive: false,
                    object,
                    indirect_object: false,
                    frame,
                    ..
                } = children.get(1)?.features
                else {
                    return None;
                };
                if !frame.is_recipient_passive() || !object.has_direct_object() {
                    return None;
                }
            }
            Some(Features::Nominal {
                form: *form,
                initial_sound: *initial_sound,
                determined: *determined,
                modified: *modified,
                leading_opacity: *leading_opacity,
                attachment: match tag {
                    RuleTag::NominalRelative => {
                        if matches!(
                            children.get(1)?.features,
                            Features::RelativeClause {
                                bare_copular_tail: true,
                                ..
                            }
                        ) {
                            NominalAttachmentPhase::RelativeBareCopula
                        } else {
                            NominalAttachmentPhase::Relative
                        }
                    }
                    RuleTag::NominalReducedRecipientPassive => {
                        NominalAttachmentPhase::ReducedRecipientPassive
                    }
                    _ => *attachment,
                },
                comparison: *comparison,
                adjunct: *adjunct,
                opaque_head: *opaque_head,
                set_exception_host: *set_exception_host,
                recipient_passive_theme: *recipient_passive_theme,
            })
        }
        RuleTag::NominalPowerToughnessComplement => {
            let Features::Nominal {
                form,
                initial_sound,
                determined,
                modified,
                leading_opacity,
                attachment,
                comparison,
                adjunct,
                opaque_head,
                set_exception_host,
                recipient_passive_theme,
            } = children.first()?.features
            else {
                return None;
            };
            if matches!(
                attachment,
                NominalAttachmentPhase::ReducedRecipientPassive
                    | NominalAttachmentPhase::PostpositiveAdjective
                    | NominalAttachmentPhase::Comparison
            ) {
                return None;
            }
            Some(Features::Nominal {
                form: *form,
                initial_sound: *initial_sound,
                determined: *determined,
                modified: *modified,
                leading_opacity: *leading_opacity,
                attachment: *attachment,
                comparison: *comparison,
                adjunct: *adjunct,
                opaque_head: *opaque_head,
                set_exception_host: *set_exception_host,
                recipient_passive_theme: *recipient_passive_theme,
            })
        }
        RuleTag::NominalPostpositiveAdjective => {
            let Features::Adjective {
                card_orientation: false,
                // Everything but `Measured`: a degree phrase must not land
                // postnominally (`creature 2 greater`).
                comparison:
                    AdjectiveComparisonState::NotComparative
                    | AdjectiveComparisonState::Complete
                    | AdjectiveComparisonState::Pending(_),
                ..
            } = children.get(1)?.features
            else {
                return None;
            };
            let Features::Nominal {
                form,
                initial_sound,
                determined,
                modified,
                leading_opacity,
                attachment: NominalAttachmentPhase::Open,
                comparison: AdjectiveComparisonState::NotComparative,
                adjunct,
                opaque_head,
                set_exception_host,
                recipient_passive_theme,
            } = children.first()?.features
            else {
                return None;
            };
            Some(Features::Nominal {
                form: *form,
                initial_sound: *initial_sound,
                determined: *determined,
                modified: *modified,
                leading_opacity: *leading_opacity,
                attachment: NominalAttachmentPhase::PostpositiveAdjective,
                comparison: AdjectiveComparisonState::NotComparative,
                adjunct: *adjunct,
                opaque_head: *opaque_head,
                set_exception_host: *set_exception_host,
                recipient_passive_theme: *recipient_passive_theme,
            })
        }
        RuleTag::NominalComparison => {
            let Features::Nominal {
                form,
                initial_sound,
                determined,
                modified,
                leading_opacity,
                attachment: NominalAttachmentPhase::Open | NominalAttachmentPhase::Prepositional,
                comparison: AdjectiveComparisonState::Pending(_),
                adjunct,
                opaque_head,
                set_exception_host,
                recipient_passive_theme,
            } = children.first()?.features
            else {
                return None;
            };
            Some(Features::Nominal {
                form: *form,
                initial_sound: *initial_sound,
                determined: *determined,
                modified: *modified,
                leading_opacity: *leading_opacity,
                attachment: NominalAttachmentPhase::Comparison,
                comparison: AdjectiveComparisonState::Complete,
                adjunct: *adjunct,
                opaque_head: *opaque_head,
                set_exception_host: *set_exception_host,
                recipient_passive_theme: *recipient_passive_theme,
            })
        }
        RuleTag::DevotionColorSingle => {
            // The single-color argument: the child is a bare color word.
            matches!(children.first()?.features, Features::None).then_some(Features::None)
        }
        RuleTag::DevotionColorPair => {
            // The two-color argument is joined by `and`, never `or`.
            matches!(
                children.get(1)?.features,
                Features::Conjunction(crate::syntax::PredicateConjunction::And)
            )
            .then_some(Features::None)
        }
        RuleTag::NominalDevotion => {
            // `devotion to <color>` is a singular measured value that a
            // possessive determiner (`your`) then wraps.
            let Features::Noun { .. } = children.first()?.features else {
                return None;
            };
            Some(Features::Nominal {
                form: NounForm::Singular,
                initial_sound: InitialSound::Consonant,
                determined: false,
                modified: false,
                leading_opacity: false,
                attachment: NominalAttachmentPhase::Prepositional,
                comparison: AdjectiveComparisonState::NotComparative,
                adjunct: None,
                opaque_head: false,
                set_exception_host: false,
                recipient_passive_theme: false,
            })
        }
        RuleTag::NominalTimesClause => {
            // `times <clause>`: a plural `times` head with a finite clause as a
            // reduced adjunct-relative complement.
            let Features::Noun { .. } = children.first()?.features else {
                return None;
            };
            let Features::Clause { finite: true, .. } = children.get(1)?.features else {
                return None;
            };
            Some(Features::Nominal {
                form: NounForm::Plural,
                initial_sound: InitialSound::Consonant,
                determined: false,
                modified: false,
                leading_opacity: false,
                attachment: NominalAttachmentPhase::Relative,
                comparison: AdjectiveComparisonState::NotComparative,
                adjunct: None,
                opaque_head: false,
                set_exception_host: false,
                recipient_passive_theme: false,
            })
        }
        RuleTag::ModifierConjunctAdjective => {
            // Only plain attributive adjectives coordinate as modifiers: a
            // comparative or a card-orientation adjective is not an atom of a
            // color/type/supertype list.
            let Features::Adjective {
                initial_sound,
                comparison: AdjectiveComparisonState::NotComparative,
                card_orientation: false,
            } = children.first()?.features
            else {
                return None;
            };
            Some(Features::CoordinatedModifier {
                initial_sound: *initial_sound,
                all_adjectives: true,
            })
        }
        RuleTag::ModifierConjunctNoun => {
            let Features::Noun { initial_sound, .. } = children.first()?.features else {
                return None;
            };
            Some(Features::CoordinatedModifier {
                initial_sound: *initial_sound,
                all_adjectives: false,
            })
        }
        RuleTag::ModifierConjunctNegated => {
            // A `non-` conjunct always renders `non…`, a consonant onset.
            Some(Features::CoordinatedModifier {
                initial_sound: InitialSound::Consonant,
                all_adjectives: false,
            })
        }
        RuleTag::ModifierListSingle => {
            let Features::CoordinatedModifier { .. } = children.first()?.features else {
                return None;
            };
            Some(children.first()?.features.clone())
        }
        RuleTag::ModifierListComma => {
            // The list keeps its first conjunct's onset regardless of what a
            // comma continuation appends; `all_adjectives` holds only while every
            // appended conjunct is itself an adjective.
            let Features::CoordinatedModifier {
                initial_sound,
                all_adjectives,
            } = children.first()?.features
            else {
                return None;
            };
            let Features::CoordinatedModifier {
                all_adjectives: appended_adjectives,
                ..
            } = children.get(2)?.features
            else {
                return None;
            };
            Some(Features::CoordinatedModifier {
                initial_sound: *initial_sound,
                all_adjectives: *all_adjectives && *appended_adjectives,
            })
        }
        RuleTag::CoordinatedModifierConjoined | RuleTag::CoordinatedModifierOxford => {
            let Features::CoordinatedModifier {
                initial_sound,
                all_adjectives,
            } = children.first()?.features
            else {
                return None;
            };
            let conjunction_index =
                if tag == RuleTag::CoordinatedModifierConjoined { 1 } else { 2 };
            // `and`/`or`/`and/or` close a modifier list; `then` never does.
            let Features::Conjunction(conjunction) = children.get(conjunction_index)?.features
            else {
                return None;
            };
            if matches!(conjunction, crate::syntax::PredicateConjunction::Then) {
                return None;
            }
            let Features::CoordinatedModifier {
                all_adjectives: closing_adjectives,
                ..
            } = children.last()?.features
            else {
                return None;
            };
            Some(Features::CoordinatedModifier {
                initial_sound: *initial_sound,
                all_adjectives: *all_adjectives && *closing_adjectives,
            })
        }
        RuleTag::NominalCoordinatedModifier => {
            let Features::CoordinatedModifier { initial_sound, .. } = children.first()?.features
            else {
                return None;
            };
            nominal_with_prefix(
                children.get(1)?,
                *initial_sound,
                false,
                AdjectiveComparisonState::NotComparative,
            )
        }
        _ => None,
    }
}

fn reduce_phrase(tag: RuleTag, children: &[Child<'_, EnglishGrammar<'_, '_>>]) -> Option<Reduced> {
    match tag {
        RuleTag::NounPhraseSetExceptionBare | RuleTag::NounPhraseSetExceptionFor => {
            let host = children.first()?;
            if !noun_phrase_accepts_set_exception(host.features)
                || !matches!(children.last()?.features, Features::NounPhrase { .. })
            {
                return None;
            }
            let Features::NounPhrase {
                agreement,
                pronoun_case,
                adjunct,
                ..
            } = host.features
            else {
                return None;
            };
            Some(Features::NounPhrase {
                agreement: *agreement,
                pronoun_case: *pronoun_case,
                adjunct: *adjunct,
                set_exception: SetExceptionState::Closed,
            })
        }
        RuleTag::NounPhraseNominal | RuleTag::ReducedRecipientPassiveTheme => {
            let Features::Nominal {
                form,
                determined,
                modified,
                attachment,
                adjunct,
                set_exception_host,
                recipient_passive_theme,
                ..
            } = children.first()?.features
            else {
                return None;
            };
            if tag == RuleTag::ReducedRecipientPassiveTheme
                && (*attachment != NominalAttachmentPhase::Open || !*recipient_passive_theme)
            {
                return None;
            }
            let agreement = Some(Agreement {
                person: Person::Third,
                number: match form {
                    NounForm::Plural => Number::Plural,
                    NounForm::Singular | NounForm::Mass => Number::Singular,
                },
            });
            // A completely bare nominal (no determiner, no modifier) must not
            // surface its bare-temporal/manner-adjunct licensing: it is
            // always the bare residue of a compound head (`step` left over
            // from `draw step`), never a legitimate standalone adjunct
            // [declarestep-plan-C.md §2].
            let adjunct = if *determined || *modified { *adjunct } else { None };
            Some(Features::NounPhrase {
                agreement,
                pronoun_case: None,
                adjunct,
                set_exception: if *set_exception_host {
                    SetExceptionState::Host
                } else {
                    SetExceptionState::Ineligible
                },
            })
        }
        RuleTag::NounPhraseSubjectPronoun | RuleTag::NounPhraseObjectPronoun => {
            noun_phrase_from_pronoun(children.first()?)
        }
        RuleTag::NounPhraseReciprocal => noun_phrase_from_pronoun(children.first()?),
        RuleTag::NounPhraseQuantity => {
            let Features::Quantity(QuantityFeatures {
                standalone_number, ..
            }) = children.first()?.features
            else {
                return None;
            };
            Some(Features::NounPhrase {
                agreement: Some(Agreement {
                    person: Person::Third,
                    number: *standalone_number,
                }),
                pronoun_case: None,
                adjunct: None,
                set_exception: SetExceptionState::Ineligible,
            })
        }
        RuleTag::NounPhraseThisCard | RuleTag::NounPhraseFullThisCard => {
            let Features::NounPhrase {
                agreement,
                pronoun_case,
                adjunct,
                set_exception,
            } = children.first()?.features
            else {
                return None;
            };
            Some(Features::NounPhrase {
                agreement: *agreement,
                pronoun_case: *pronoun_case,
                adjunct: *adjunct,
                set_exception: *set_exception,
            })
        }
        RuleTag::NounPhrasePossessiveThisCard => {
            let Features::PossessiveThisCard { agreement } = children.first()?.features else {
                return None;
            };
            Some(Features::NounPhrase {
                agreement: Some(*agreement),
                pronoun_case: None,
                adjunct: None,
                set_exception: SetExceptionState::Ineligible,
            })
        }
        RuleTag::NounPhraseDemonstrative => {
            let Features::NounPhrase { .. } = children.first()?.features else {
                return None;
            };
            Some(children.first()?.features.clone())
        }
        RuleTag::NounPhrasePartitive => {
            let Features::Quantity(QuantityFeatures {
                standalone_number, ..
            }) = children.first()?.features
            else {
                return None;
            };
            Some(Features::NounPhrase {
                agreement: Some(Agreement {
                    person: Person::Third,
                    number: *standalone_number,
                }),
                pronoun_case: None,
                adjunct: None,
                set_exception: SetExceptionState::Ineligible,
            })
        }
        RuleTag::NounPhraseEachPartitive => {
            // The `each` slot only ever scans "each", so the distributive
            // partitive is always grammatically third-person singular.
            let Features::Determiner { .. } = children.first()?.features else {
                return None;
            };
            Some(Features::NounPhrase {
                agreement: Some(Agreement {
                    person: Person::Third,
                    number: Number::Singular,
                }),
                pronoun_case: None,
                adjunct: None,
                set_exception: SetExceptionState::Ineligible,
            })
        }
        RuleTag::NounPhraseAnyNumberOf => {
            // Defensively recheck the specialized lowered children as the
            // neighbouring arms do. The `any`/`number` slots only ever scan
            // those two literal words, so this arm's sole remaining
            // condition is the fourth child's agreement: the notional
            // plural reading is licensed only when the final noun phrase
            // is third-person plural [`anof` round].
            let Features::Determiner { .. } = children.first()?.features else {
                return None;
            };
            let Features::Noun {
                form: NounForm::Singular,
                ..
            } = children.get(1)?.features
            else {
                return None;
            };
            let Features::Preposition(Preposition::Of) = children.get(2)?.features else {
                return None;
            };
            let Features::NounPhrase {
                agreement:
                    Some(Agreement {
                        number: Number::Plural,
                        ..
                    }),
                ..
            } = children.get(3)?.features
            else {
                return None;
            };
            Some(Features::NounPhrase {
                agreement: Some(Agreement {
                    person: Person::Third,
                    number: Number::Plural,
                }),
                pronoun_case: None,
                adjunct: None,
                set_exception: SetExceptionState::Ineligible,
            })
        }
        RuleTag::NounPhraseCoordination
        | RuleTag::NounPhraseAdditiveCoordination
        | RuleTag::NounPhraseCoordinationOxford => reduce_noun_phrase_coordination(tag, children),
        RuleTag::NounPhraseListSingle => {
            let Features::NounPhrase { .. } = children.first()?.features else {
                return None;
            };
            Some(children.first()?.features.clone())
        }
        RuleTag::NounPhraseListComma => {
            // The run keeps its first member's agreement; the closing rule
            // recomputes the coordinated agreement from the final conjunction.
            let Features::NounPhrase { .. } = children.first()?.features else {
                return None;
            };
            let Features::NounPhrase { .. } = children.get(2)?.features else {
                return None;
            };
            Some(children.first()?.features.clone())
        }
        RuleTag::NounPhraseMinus => {
            // Both operands must be noun-phrase values; the result is a
            // singular numeric value.
            let Features::NounPhrase { .. } = children.first()?.features else {
                return None;
            };
            let Features::NounPhrase { .. } = children.get(2)?.features else {
                return None;
            };
            Some(arithmetic_value_features())
        }
        RuleTag::NounPhraseHalf
        | RuleTag::NounPhraseHalfRoundedUp
        | RuleTag::NounPhraseHalfRoundedDown => {
            let Features::NounPhrase { .. } = children.get(1)?.features else {
                return None;
            };
            Some(arithmetic_value_features())
        }
        RuleTag::PrepositionalPhrase => {
            let Features::Preposition(preposition) = children.first()?.features else {
                return None;
            };
            let Features::PrepositionalObject { gerund } = children.get(1)?.features else {
                return None;
            };
            Some(Features::PrepositionalPhrase {
                preposition: *preposition,
                nominal_attachment: !(*preposition == Preposition::By && *gerund),
            })
        }
        RuleTag::PrepositionalObject => Some(Features::PrepositionalObject {
            gerund: matches!(children.first()?.features, Features::GerundClause),
        }),
        _ => None,
    }
}

fn reduce_noun_phrase_coordination(
    tag: RuleTag,
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
) -> Option<Reduced> {
    let Features::NounPhrase {
        agreement: first_agreement,
        adjunct: first_adjunct,
        set_exception,
        ..
    } = children.first()?.features
    else {
        return None;
    };
    // The Oxford close reads its conjunction after the comma (`, or`), so its
    // conjunction and next member sit one slot later than the binary rule.
    let (conjunction_index, next_index) =
        if tag == RuleTag::NounPhraseCoordinationOxford { (2, 3) } else { (1, 2) };
    let conjunction = if tag == RuleTag::NounPhraseAdditiveCoordination {
        crate::syntax::NounPhraseConjunction::Plus
    } else {
        let Features::Conjunction(conjunction) = children.get(conjunction_index)?.features else {
            return None;
        };
        match conjunction {
            crate::syntax::PredicateConjunction::And => crate::syntax::NounPhraseConjunction::And,
            crate::syntax::PredicateConjunction::Or => crate::syntax::NounPhraseConjunction::Or,
            crate::syntax::PredicateConjunction::AndOr => {
                crate::syntax::NounPhraseConjunction::AndOr
            }
            // `then` never joins noun phrases.
            crate::syntax::PredicateConjunction::Then => return None,
        }
    };
    let Features::NounPhrase {
        agreement: next_agreement,
        adjunct: next_adjunct,
        ..
    } = children.get(next_index)?.features
    else {
        return None;
    };
    let agreement = match conjunction {
        crate::syntax::NounPhraseConjunction::And => Some(Agreement {
            person: Person::Third,
            number: Number::Plural,
        }),
        crate::syntax::NounPhraseConjunction::Or | crate::syntax::NounPhraseConjunction::AndOr => {
            *next_agreement
        }
        crate::syntax::NounPhraseConjunction::Plus => *first_agreement,
    };
    Some(Features::NounPhrase {
        agreement,
        pronoun_case: None,
        adjunct: (*first_adjunct == *next_adjunct)
            .then_some(*first_adjunct)
            .flatten(),
        set_exception: *set_exception,
    })
}

fn propagate(child: &Child<'_, EnglishGrammar<'_, '_>>) -> Reduced {
    child.features.clone()
}

fn nominal_with_prefix(
    nominal: &Child<'_, EnglishGrammar<'_, '_>>,
    initial_sound: InitialSound,
    leading_opacity: bool,
    prefix_comparison: AdjectiveComparisonState,
) -> Option<Reduced> {
    let Features::Nominal {
        form,
        determined,
        attachment,
        comparison,
        adjunct,
        opaque_head,
        set_exception_host,
        recipient_passive_theme,
        ..
    } = nominal.features
    else {
        return None;
    };
    if *determined {
        return None;
    }
    let comparison = match (prefix_comparison, *comparison) {
        // `Measured` is predicative-only: reject it attributively in either
        // position.
        (AdjectiveComparisonState::Measured, _) | (_, AdjectiveComparisonState::Measured) => {
            return None;
        }
        (AdjectiveComparisonState::Pending(class), AdjectiveComparisonState::NotComparative) => {
            AdjectiveComparisonState::Pending(class)
        }
        (AdjectiveComparisonState::Pending(_), _) => return None,
        (AdjectiveComparisonState::NotComparative | AdjectiveComparisonState::Complete, state) => {
            state
        }
    };
    Some(Features::Nominal {
        form: *form,
        initial_sound,
        determined: *determined,
        // The one site that sets this: adding any modifier (adjective, noun
        // modifier, quantity, power/toughness, negated modifier) through
        // this shared helper makes the nominal non-bare.
        modified: true,
        leading_opacity,
        attachment: *attachment,
        comparison,
        adjunct: *adjunct,
        opaque_head: *opaque_head,
        set_exception_host: *set_exception_host,
        recipient_passive_theme: *recipient_passive_theme,
    })
}

fn noun_phrase_from_pronoun(child: &Child<'_, EnglishGrammar<'_, '_>>) -> Option<Reduced> {
    let Features::NounPhrase {
        agreement,
        pronoun_case,
        adjunct,
        set_exception,
    } = child.features
    else {
        return None;
    };
    Some(Features::NounPhrase {
        agreement: *agreement,
        pronoun_case: *pronoun_case,
        adjunct: *adjunct,
        set_exception: *set_exception,
    })
}

fn cardinality_accepts(cardinality: Cardinality, form: NounForm) -> bool {
    match cardinality {
        Cardinality::SingularCount => form == NounForm::Singular,
        Cardinality::SingularOrMass => matches!(form, NounForm::Singular | NounForm::Mass),
        Cardinality::PluralCount => form == NounForm::Plural,
        Cardinality::Mass => form == NounForm::Mass,
        Cardinality::PluralOrMass => matches!(form, NounForm::Plural | NounForm::Mass),
        Cardinality::Unconstrained => true,
    }
}

fn article_accepts(article: Option<IndefiniteArticleKey>, sound: InitialSound) -> bool {
    match article {
        Some(IndefiniteArticleKey::A) => sound == InitialSound::Consonant,
        Some(IndefiniteArticleKey::An) => sound == InitialSound::Vowel,
        None => true,
    }
}

fn slot_agrees(slot: VerbSlot, agreement: Agreement) -> bool {
    matches!(
        slot,
        VerbSlot::Present { person, number } | VerbSlot::Past { person, number }
            if person == agreement.person && number == agreement.number
    )
}

type EnglishChart = ChartResult<Nonterminal, EnglishLexicalSlot, Features, MeaningKey>;
type EnglishForest = ParseForest<Nonterminal, EnglishLexicalSlot, Features, MeaningKey>;

#[derive(Debug)]
pub(crate) struct ParsedNonterminal {
    pub(crate) chart: EnglishChart,
    root: NodeId,
    best: BestParse,
    syntax: Lowered,
    opacity_mode: OpacityMode,
}

impl ParsedNonterminal {
    pub(crate) fn noun_phrase(&self) -> Option<&NounPhrase> {
        match &self.syntax {
            Lowered::NounPhrase(noun_phrase) => Some(noun_phrase),
            _ => None,
        }
    }

    pub(crate) fn sentence(&self) -> Option<&crate::syntax::Sentence> {
        match &self.syntax {
            Lowered::Sentence(sentence) => Some(sentence),
            _ => None,
        }
    }

    fn simple_clause(&self) -> Option<&SimpleClause> {
        match &self.syntax {
            Lowered::SimpleClause(clause) => Some(clause),
            _ => None,
        }
    }

    pub(crate) fn clause(&self) -> Option<&Clause> {
        match &self.syntax {
            Lowered::Clause(clause) => Some(clause),
            _ => None,
        }
    }

    pub(crate) fn quantity(&self) -> Option<&Quantity> {
        match &self.syntax {
            Lowered::Quantity(quantity) => Some(quantity),
            _ => None,
        }
    }

    pub(crate) fn prepositional_phrase(&self) -> Option<&PrepositionalPhrase> {
        match &self.syntax {
            Lowered::PrepositionalPhrase(preposition) => Some(preposition),
            _ => None,
        }
    }

    pub(crate) fn adjective_phrase(&self) -> Option<&AdjectivePhrase> {
        match &self.syntax {
            Lowered::AdjectivePhrase(adjective) => Some(adjective),
            _ => None,
        }
    }

    pub(crate) fn root_rule(&self) -> Option<usize> {
        let node = self.chart.forest.node(self.root);
        let alternative = self.best.alternative(self.root)?;
        node.alternatives
            .get(alternative)?
            .rule
            .map(crate::chart::RuleId::index)
    }

    pub(crate) fn root_tied_alternatives(&self) -> &[usize] {
        self.best.tied_alternatives(self.root)
    }

    pub(crate) const fn cost(&self) -> ParseCost {
        self.best.cost
    }

    pub(crate) const fn chart_stats(&self) -> ChartStats {
        self.chart.stats
    }

    pub(crate) fn forest_stats(&self) -> ForestStats {
        self.chart.forest.stats()
    }

    pub(crate) const fn opacity_mode(&self) -> OpacityMode {
        self.opacity_mode
    }
}

#[derive(Debug)]
pub(crate) enum ParseNonterminalError {
    Grammar(GrammarError),
    NoCompleteParse(Nonterminal),
    Forest(ForestError),
    Lowering,
}

pub(crate) fn parse_nonterminal(
    source: &str,
    catalogs: &Catalogs,
    nonterminal: Nonterminal,
) -> Result<ParsedNonterminal, ParseNonterminalError> {
    parse_nonterminal_with_self_reference(source, catalogs, nonterminal, &SelfReference::default())
}

pub(crate) fn parse_nonterminal_with_self_reference(
    source: &str,
    catalogs: &Catalogs,
    nonterminal: Nonterminal,
    self_reference: &SelfReference,
) -> Result<ParsedNonterminal, ParseNonterminalError> {
    let surface = lex(source);
    let tokens = collapse_full_names(source, surface.tokens, self_reference.full_name());
    match parse_nonterminal_with_profile(
        source,
        catalogs,
        nonterminal,
        &tokens,
        OpacityProfile::Exact,
        self_reference,
    ) {
        Ok(parsed) => Ok(parsed),
        Err(ParseNonterminalError::NoCompleteParse(_)) => parse_nonterminal_with_profile(
            source,
            catalogs,
            nonterminal,
            &tokens,
            OpacityProfile::Nouns,
            self_reference,
        ),
        Err(error) => Err(error),
    }
}

fn parse_nonterminal_with_profile(
    source: &str,
    catalogs: &Catalogs,
    nonterminal: Nonterminal,
    tokens: &[Token],
    opacity_profile: OpacityProfile,
    self_reference: &SelfReference,
) -> Result<ParsedNonterminal, ParseNonterminalError> {
    let grammar = EnglishGrammar::with_opacity_profile(
        source,
        catalogs,
        nonterminal,
        opacity_profile,
        self_reference.clone(),
    );
    let chart = parse_chart(&grammar, tokens).map_err(ParseNonterminalError::Grammar)?;
    if chart.roots.is_empty() {
        return Err(ParseNonterminalError::NoCompleteParse(nonterminal));
    }
    let forest = &chart.forest;
    let mut syntax = None;
    let (root, best) = chart
        .forest
        .best_root_matching(chart.roots.iter().copied(), |root, best| {
            syntax = lower(&grammar, forest, root, best);
            syntax.is_some()
        })
        .map_err(ParseNonterminalError::Forest)?
        .ok_or(ParseNonterminalError::Lowering)?;
    let syntax = syntax.ok_or(ParseNonterminalError::Lowering)?;
    Ok(ParsedNonterminal {
        chart,
        root,
        best,
        syntax,
        opacity_mode: opacity_profile.mode(),
    })
}

#[cfg(test)]
mod root_lowering_tests {
    use super::*;
    use crate::syntax::IndependentClause;

    /// The pre-ticket algorithm: choose exactly one root by cost and stable
    /// node order, then let a lowering decline fail the whole parse.
    fn parse_single_best_root(
        source: &str,
        catalogs: &Catalogs,
        nonterminal: Nonterminal,
    ) -> Result<ParsedNonterminal, ParseNonterminalError> {
        let surface = lex(source);
        let grammar = EnglishGrammar::with_opacity_profile(
            source,
            catalogs,
            nonterminal,
            OpacityProfile::Exact,
            SelfReference::default(),
        );
        let chart =
            parse_chart(&grammar, &surface.tokens).map_err(ParseNonterminalError::Grammar)?;
        let (root, best) = chart
            .forest
            .best_root(chart.roots.iter().copied())
            .map_err(ParseNonterminalError::Forest)?
            .ok_or(ParseNonterminalError::NoCompleteParse(nonterminal))?;
        let syntax =
            lower(&grammar, &chart.forest, root, &best).ok_or(ParseNonterminalError::Lowering)?;
        Ok(ParsedNonterminal {
            chart,
            root,
            best,
            syntax,
            opacity_mode: OpacityMode::Exact,
        })
    }

    fn fixture_catalogs() -> Catalogs {
        Catalogs::default()
            .with_catalog(CatalogKind::CardType, ["Artifact", "Creature", "Land"])
            .with_catalog(CatalogKind::CreatureType, ["Goblin", "Human"])
    }

    #[test]
    fn lowering_decline_falls_through_to_the_next_ranked_root() {
        let catalogs = fixture_catalogs();
        assert!(matches!(
            parse_single_best_root("copy that spell", &catalogs, Nonterminal::Clause),
            Err(ParseNonterminalError::Lowering)
        ));

        let parsed = parse_nonterminal("copy that spell", &catalogs, Nonterminal::Clause)
            .expect("a lowerable imperative root follows the rejected nominal reading");
        assert!(matches!(
            parsed.clause(),
            Some(Clause::Independent(IndependentClause::Imperative(_)))
        ));
    }

    #[test]
    fn successful_single_root_selections_are_unchanged() {
        let catalogs = fixture_catalogs();
        for source in [
            "you draw a card",
            "target creature can't block this turn",
            "it is your turn",
            "there are no creatures on the battlefield",
            "damage can't be prevented",
        ] {
            let before = parse_single_best_root(source, &catalogs, Nonterminal::Clause)
                .unwrap_or_else(|error| {
                    panic!("control failed before retry for {source:?}: {error:?}")
                });
            let after =
                parse_nonterminal(source, &catalogs, Nonterminal::Clause).unwrap_or_else(|error| {
                    panic!("control failed after retry for {source:?}: {error:?}")
                });
            assert_eq!(after.root, before.root, "root changed for {source:?}");
            assert_eq!(
                after.root_rule(),
                before.root_rule(),
                "rule changed for {source:?}"
            );
            assert_eq!(after.cost(), before.cost(), "cost changed for {source:?}");
            assert_eq!(
                after.root_tied_alternatives(),
                before.root_tied_alternatives(),
                "root tie changed for {source:?}",
            );
            assert_eq!(
                after.chart_stats(),
                before.chart_stats(),
                "chart changed for {source:?}"
            );
            assert_eq!(
                after.forest_stats(),
                before.forest_stats(),
                "forest changed for {source:?}"
            );
            assert_eq!(
                after.clause(),
                before.clause(),
                "syntax changed for {source:?}"
            );
        }
    }
}

#[derive(Debug)]
enum Lowered {
    Number(NumberKey),
    Quantity(Quantity),
    Determiner(Determiner),
    Adjective(Adjective),
    AdjectivePhrase(AdjectivePhrase),
    ComparisonComplement(ComparisonComplement),
    Noun(NounInstance),
    NominalModifier(NominalModifier),
    CoordinatedModifier(crate::syntax::CoordinatedModifier),
    Nominal(NominalPhrase),
    DevotionColors(crate::syntax::DevotionColors),
    PossessiveNominal(NominalPhrase),
    NounPhrase(NounPhrase),
    Catalog(crate::catalog::CatalogAtom),
    Adverb(Vocab),
    VerbParticle(VerbParticle),
    CoinResult(crate::syntax::CoinSide),
    Frequency(FrequencyPhrase),
    Auxiliary(AuxiliaryInstance),
    SubjectAuxiliary(ContractedSubjectAuxiliary),
    OracleSymbol(OracleSymbol),
    SymbolSequence(Vec<OracleSymbol>),
    /// One quality of a `kwgrant`-round predicated keyword argument.
    PredicatedQuality(crate::syntax::PredicatedQuality),
    /// A coordinated list of [`Self::PredicatedQuality`] members, in surface
    /// order.
    PredicatedArgument(crate::syntax::PredicatedArgument),
    /// Uniform payload for all six mana-list rules.
    ManaAmount(crate::syntax::PredicateObject),
    PowerToughness(PowerToughness),
    Pronoun(PronounInstance),
    ThisCard(ThisCardForm),
    Preposition(Preposition),
    Phrase(Phrase),
    PrepositionalPhrase(PrepositionalPhrase),
    Verb(VerbAnalysis),
    VerbPhrase(VerbPhrase),
    InfinitiveClause(InfinitiveClause),
    GerundClause(GerundClause),
    CopularRemainder(CopularRemainder),
    SimpleClause(SimpleClause),
    EllipticalClause(crate::syntax::EllipticalClause),
    Clause(Clause),
    RelativeClause(RelativeClause),
    Sentence(Sentence),
    Conjunction(crate::syntax::PredicateConjunction),
    Subordinator(crate::syntax::Subordinator),
    RelativeMarker(RelativeMarker),
    Existential(ExistentialForm),
    ExceptionRider(crate::syntax::ExceptionRider),
    /// One restriction-run member's adjunct sequence (one adjunct for most
    /// members, two for the flat `once each turn` adverb+temporal pair).
    RestrictionMember(Vec<crate::syntax::PredicateAdjunct>),
    RestrictionRun(crate::syntax::RestrictionRun),
    Ignored,
}

fn lower(
    grammar: &EnglishGrammar<'_, '_>,
    forest: &EnglishForest,
    node: NodeId,
    best: &BestParse,
) -> Option<Lowered> {
    let forest_node = forest.node(node);
    match forest_node.key.symbol {
        ForestSymbol::Lexical(_) => {
            return lower_lexical(grammar, forest_node.key.lexical_value()?);
        }
        ForestSymbol::Intermediate { .. } => return None,
        ForestSymbol::Nonterminal(_) => {}
    }
    let alternative = forest_node.alternatives.get(best.alternative(node)?)?;
    let rule = alternative.rule?;
    let tag = *grammar.tags.get(rule.index())?;
    let [intermediate] = alternative.children.as_slice() else {
        return None;
    };
    let mut child_nodes = Vec::new();
    selected_rule_children(forest, *intermediate, best, &mut child_nodes)?;
    let mut children = child_nodes
        .iter()
        .map(|&child| lower(grammar, forest, child, best))
        .collect::<Option<Vec<_>>>()?;
    lower_rule(tag, &mut children)
}

fn selected_rule_children(
    forest: &EnglishForest,
    intermediate: NodeId,
    best: &BestParse,
    children: &mut Vec<NodeId>,
) -> Option<()> {
    let node = forest.node(intermediate);
    if !matches!(node.key.symbol, ForestSymbol::Intermediate { .. }) {
        return None;
    }
    let alternative = node.alternatives.get(best.alternative(intermediate)?)?;
    match alternative.children.as_slice() {
        [child] => children.push(*child),
        [previous, child] => {
            selected_rule_children(forest, *previous, best, children)?;
            children.push(*child);
        }
        _ => return None,
    }
    Some(())
}

fn lower_lexical(grammar: &EnglishGrammar<'_, '_>, meaning: &MeaningKey) -> Option<Lowered> {
    Some(match meaning {
        MeaningKey::Literal(_) | MeaningKey::Punctuation(_) => Lowered::Ignored,
        MeaningKey::Number(number) => Lowered::Number(*number),
        MeaningKey::Quantity(quantity) => Lowered::Quantity(quantity.syntax()),
        MeaningKey::Determiner(determiner) => Lowered::Determiner(determiner.syntax()),
        MeaningKey::Noun(noun) => Lowered::Noun(noun.clone()),
        MeaningKey::Adjective(adjective) => Lowered::Adjective(adjective.clone()),
        MeaningKey::Adverb(adverb) => Lowered::Adverb(*adverb),
        MeaningKey::VerbParticle(particle) => Lowered::VerbParticle(*particle),
        MeaningKey::CoinResult(side) => Lowered::CoinResult(*side),
        MeaningKey::Frequency(frequency) => Lowered::Frequency(frequency.syntax()),
        MeaningKey::Pronoun(pronoun) => Lowered::Pronoun(*pronoun),
        MeaningKey::Auxiliary(auxiliary) => Lowered::Auxiliary(*auxiliary),
        MeaningKey::SubjectAuxiliary(subject_auxiliary) => {
            let subject = match subject_auxiliary.subject {
                ContractedSubjectKey::Pronoun(pronoun) => NounPhrase::Pronoun {
                    pronoun,
                    case: PronounCase::Subject,
                },
                ContractedSubjectKey::Demonstrative(demonstrative) => {
                    NounPhrase::Demonstrative(demonstrative)
                }
            };
            Lowered::SubjectAuxiliary(ContractedSubjectAuxiliary {
                subject: Subject(subject),
                auxiliary: subject_auxiliary.auxiliary,
            })
        }
        MeaningKey::Verb(verb) => Lowered::Verb(verb.clone()),
        MeaningKey::Catalog(atom) => Lowered::Catalog(atom.clone()),
        MeaningKey::OracleSymbol(symbol) => Lowered::OracleSymbol(symbol.clone()),
        MeaningKey::SymbolSequence(symbols) => Lowered::SymbolSequence(symbols.clone()),
        MeaningKey::QuotedAbility(span) => {
            let interior = span.text(grammar.source)?;
            Lowered::Phrase(Phrase::QuotedAbility(Box::new(
                ability::parse_quoted_ability_fragment(
                    interior,
                    grammar.catalogs,
                    &grammar.self_reference,
                ),
            )))
        }
        MeaningKey::PowerToughness(power_toughness) => Lowered::PowerToughness(*power_toughness),
        MeaningKey::Conjunction(conjunction) => Lowered::Conjunction(*conjunction),
        MeaningKey::Subordinator(subordinator) => Lowered::Subordinator(*subordinator),
        MeaningKey::RelativeMarker(marker) => Lowered::RelativeMarker(*marker),
        MeaningKey::Existential(form) => Lowered::Existential(*form),
        MeaningKey::ThisCard(form) => Lowered::ThisCard(*form),
        MeaningKey::Preposition(preposition) => Lowered::Preposition(*preposition),
        MeaningKey::NegatedModifier(key) => Lowered::NominalModifier(key.build()),
        MeaningKey::Opaque(key) => {
            let opaque = OpaqueLexeme::new(key.span.text(grammar.source)?);
            match key.slot {
                OpacitySlot::Noun(form) => {
                    let noun = Noun::Opaque(opaque);
                    Lowered::Noun(match form {
                        NounForm::Singular => NounInstance::Singular(noun),
                        NounForm::Plural => NounInstance::Plural(noun),
                        NounForm::Mass => NounInstance::Mass(noun),
                    })
                }
            }
        }
    })
}

#[allow(
    clippy::too_many_lines,
    reason = "the rule-tag dispatch is intentionally one flat match over every tag"
)]
fn lower_rule(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    match tag {
        RuleTag::QuantityExact
        | RuleTag::QuantityAtLeast
        | RuleTag::QuantityOr
        | RuleTag::QuantityX
        | RuleTag::QuantityBoth
        | RuleTag::QuantityUpTo
        | RuleTag::QuantityThatMany
        | RuleTag::QuantityThatMuch
        | RuleTag::QuantityMoreThan
        | RuleTag::QuantityFewerThan
        | RuleTag::DeterminerClosed
        | RuleTag::DeterminerTarget
        | RuleTag::DeterminerQuantifiedTarget
        | RuleTag::DeterminerQuantity
        | RuleTag::DeterminerPossessiveThisCard => lower_quantity_or_determiner(tag, children),
        RuleTag::FrequencyPhrase => take(children, 0),
        RuleTag::FrequencyPhraseAdverb => {
            let Lowered::Adverb(adverb) = take(children, 0)? else {
                return None;
            };
            if adverb != Vocab::Only {
                return None;
            }
            let Lowered::Frequency(freq) = take(children, 1)? else {
                return None;
            };
            Some(Lowered::Frequency(crate::syntax::FrequencyPhrase {
                bound: crate::syntax::FrequencyBound::NoMoreThan,
                count: freq.count,
            }))
        }
        RuleTag::PossessiveNounBase
        | RuleTag::PossessiveNounDetermined
        | RuleTag::DeterminerPossessiveNoun
        | RuleTag::PossessiveNounAdjective => lower_possessive_noun_phrase(tag, children),
        RuleTag::Adjective
        | RuleTag::AdjectivePhrase
        | RuleTag::AdjectivePhraseFaceUp
        | RuleTag::AdjectivePhraseFaceDown
        | RuleTag::AdjectivePhraseComparison
        | RuleTag::AdjectivePhraseDegreeMeasure
        | RuleTag::ComparisonStandard
        | RuleTag::ComparisonThan
        | RuleTag::ComparisonThanOrEqualTo
        | RuleTag::Noun
        | RuleTag::NominalNoun
        | RuleTag::NominalAdjective
        | RuleTag::NominalNounModifier
        | RuleTag::NominalCombatStepName
        | RuleTag::NominalNegatedModifier
        | RuleTag::NominalQuantityModifier
        | RuleTag::NominalPowerToughnessModifier
        | RuleTag::NominalDeterminer
        | RuleTag::NominalPrepositional
        | RuleTag::NominalInfinitive
        | RuleTag::NominalQuantityComplement
        | RuleTag::NominalKeywordSymbolArgument
        | RuleTag::PredicatedQualityFrom
        | RuleTag::PredicatedArgumentFromSingle
        | RuleTag::PredicatedArgumentFromExtend
        | RuleTag::NominalKeywordPredicatedArgument
        | RuleTag::PredicatedQualityBare
        | RuleTag::PredicatedArgumentBareSingle
        | RuleTag::PredicatedArgumentBareExtend
        | RuleTag::NominalKeywordAtomCarriedPredicatedArgument
        | RuleTag::NominalPowerToughnessComplement
        | RuleTag::NominalRelative
        | RuleTag::NominalReducedRecipientPassive
        | RuleTag::NominalPostpositiveAdjective
        | RuleTag::NominalComparison
        | RuleTag::NominalDevotion
        | RuleTag::DevotionColorSingle
        | RuleTag::DevotionColorPair
        | RuleTag::NominalTimesClause
        | RuleTag::ModifierConjunctAdjective
        | RuleTag::ModifierConjunctNoun
        | RuleTag::ModifierConjunctNegated
        | RuleTag::ModifierListSingle
        | RuleTag::ModifierListComma
        | RuleTag::CoordinatedModifierConjoined
        | RuleTag::CoordinatedModifierOxford
        | RuleTag::NominalCoordinatedModifier => lower_nominal(tag, children),
        RuleTag::NounPhraseNominal
        | RuleTag::NounPhraseSetExceptionBare
        | RuleTag::NounPhraseSetExceptionFor
        | RuleTag::ReducedRecipientPassiveTheme
        | RuleTag::NounPhraseSubjectPronoun
        | RuleTag::NounPhraseObjectPronoun
        | RuleTag::NounPhraseReciprocal
        | RuleTag::NounPhraseQuantity
        | RuleTag::NounPhraseThisCard
        | RuleTag::NounPhraseFullThisCard
        | RuleTag::NounPhrasePossessiveThisCard
        | RuleTag::NounPhraseDemonstrative
        | RuleTag::NounPhrasePartitive
        | RuleTag::NounPhraseEachPartitive
        | RuleTag::NounPhraseAnyNumberOf
        | RuleTag::NounPhraseCoordination
        | RuleTag::NounPhraseAdditiveCoordination
        | RuleTag::NounPhraseListSingle
        | RuleTag::NounPhraseListComma
        | RuleTag::NounPhraseCoordinationOxford
        | RuleTag::NounPhraseMinus
        | RuleTag::NounPhraseHalf
        | RuleTag::NounPhraseHalfRoundedUp
        | RuleTag::NounPhraseHalfRoundedDown
        | RuleTag::PrepositionalPhrase
        | RuleTag::PrepositionalObject => lower_phrase(tag, children),
        RuleTag::Verb
        | RuleTag::VerbPhraseBase
        | RuleTag::VerbPhraseAuxiliary
        | RuleTag::VerbPhraseAuxiliaryProform
        | RuleTag::VerbPhraseDirectObject
        | RuleTag::VerbPhraseIndirectObject
        | RuleTag::VerbPhraseAdjective
        | RuleTag::VerbPhrasePrepositional
        | RuleTag::VerbPhraseExceptBy
        | RuleTag::VerbPhraseInfinitive
        | RuleTag::VerbPhraseAdverb
        | RuleTag::VerbPhrasePreverbAdverb
        | RuleTag::VerbPhraseParticle
        | RuleTag::VerbPhraseCoinResult
        | RuleTag::VerbPhraseFrequency
        | RuleTag::VerbPhraseAbility
        | RuleTag::VerbPhraseQuotedAbility
        | RuleTag::VerbPhraseQuotedAbilityCoordination
        | RuleTag::VerbPhraseAbilityQuotedCoordination
        | RuleTag::VerbPhraseOracleSymbol
        | RuleTag::VerbPhraseSymbolSequence
        | RuleTag::VerbPhraseManaAmountCoordination
        | RuleTag::ManaAmountSymbol
        | RuleTag::ManaAmountSequence
        | RuleTag::ManaAmountListSingle
        | RuleTag::ManaAmountListComma
        | RuleTag::ManaAmountCoordination
        | RuleTag::ManaAmountCoordinationOxford
        | RuleTag::VerbPhrasePowerToughness
        | RuleTag::VerbPhraseQuantity
        | RuleTag::ReducedRecipientPassiveNominalAdjunct
        | RuleTag::InfinitiveTo
        | RuleTag::InfinitiveNotTo
        | RuleTag::GerundClauseBase
        | RuleTag::GerundClauseSubordinateAfter
        | RuleTag::SimpleClauseSubject
        | RuleTag::SimpleClauseSubjectDistributiveEach
        | RuleTag::SimpleClauseContractedSubject
        | RuleTag::SimpleClauseSubjectless
        | RuleTag::ClauseSimple
        | RuleTag::ClauseElliptical
        | RuleTag::ClauseCoordination
        | RuleTag::ClauseCoordinationComma
        | RuleTag::ClauseCoordinationAsyndetic
        | RuleTag::ClauseAdverbBefore
        | RuleTag::ClauseSentenceAdverbialBefore
        | RuleTag::ClausePrepositionalBefore
        | RuleTag::ClauseSubordinateBefore
        | RuleTag::ClauseSubordinateGerundBefore
        | RuleTag::ClauseSubordinateAfterElliptical
        | RuleTag::ClauseSubordinateAfter
        | RuleTag::ClauseSubordinateAfterComma
        | RuleTag::ClauseSubordinateAfterInfinitive
        | RuleTag::ClauseExistential
        | RuleTag::CopularRemainderNoun
        | RuleTag::CopularRemainderAdjective
        | RuleTag::CopularRemainderPrepositional
        | RuleTag::CopularRemainderPowerToughness
        | RuleTag::CopularRemainderPrepositionalAdjunct
        | RuleTag::CopularRemainderAdverb
        | RuleTag::CopularRemainderDistributiveEach
        | RuleTag::CopularRemainderNegated
        | RuleTag::ClauseCopular
        | RuleTag::ClauseContractedCopular
        | RuleTag::ClauseVariableValueConstraint
        | RuleTag::RelativeObject
        | RuleTag::RelativeObjectContractedSubject
        | RuleTag::RelativeSubjectContractedAuxiliary
        | RuleTag::RelativeSubject
        | RuleTag::RelativeSubjectDistributiveEach
        | RuleTag::RelativeContractedCopularNoun
        | RuleTag::RelativeContractedCopularAdjective
        | RuleTag::RelativeContractedCopularPrepositional
        | RuleTag::ClauseExcepted
        | RuleTag::ClauseRestrictionRun
        | RuleTag::ClauseRestrictionMember
        | RuleTag::ExceptionRiderSingle
        | RuleTag::ExceptionRiderConjoined
        | RuleTag::ExceptionRiderComma
        | RuleTag::ExceptionRiderOxford
        | RuleTag::VerbPhraseCausative
        | RuleTag::VerbPhraseCoordinatedAdjective
        | RuleTag::CopularRemainderCoordinatedAdjective
        | RuleTag::RelativeContractedCopularCoordinatedAdjective
        | RuleTag::Sentence => clause::lower_clause(tag, children),
        RuleTag::NounOpaque => opacity::lower_opacity(tag, children),
    }
}

fn lower_possessive_noun_phrase(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    match tag {
        RuleTag::PossessiveNounBase => {
            let Lowered::Noun(head) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::PossessiveNominal(NominalPhrase {
                determiner: None,
                modifiers: Vec::new(),
                head,
                complements: Vec::new(),
            }))
        }
        RuleTag::PossessiveNounDetermined => {
            let Lowered::Determiner(determiner) = take(children, 0)? else {
                return None;
            };
            let Lowered::PossessiveNominal(mut nominal) = take(children, 1)? else {
                return None;
            };
            nominal.determiner = Some(determiner);
            Some(Lowered::PossessiveNominal(nominal))
        }
        RuleTag::DeterminerPossessiveNoun => {
            let Lowered::PossessiveNominal(possessor) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::Determiner(Determiner::Possessive(
                Possessor::NounPhrase(Box::new(NounPhrase::Nominal(possessor))),
            )))
        }
        RuleTag::PossessiveNounAdjective => {
            let Lowered::AdjectivePhrase(adjective) = take(children, 0)? else {
                return None;
            };
            let Lowered::PossessiveNominal(mut nominal) = take(children, 1)? else {
                return None;
            };
            if introduces_proper_name(&adjective) {
                open_name_interior(&mut nominal);
            }
            nominal.modifiers.insert(
                0,
                NominalModifier::Adjective {
                    polarity: Polarity::Positive,
                    phrase: adjective,
                },
            );
            Some(Lowered::PossessiveNominal(nominal))
        }
        _ => None,
    }
}

fn lower_quantity_or_determiner(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    match tag {
        RuleTag::QuantityExact => {
            let Lowered::Number(number) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::Quantity(Quantity::Exact(number.literal())))
        }
        RuleTag::QuantityAtLeast
        | RuleTag::QuantityOr
        | RuleTag::QuantityX
        | RuleTag::QuantityBoth
        | RuleTag::QuantityMoreThan
        | RuleTag::QuantityFewerThan => {
            let Lowered::Quantity(quantity) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::Quantity(quantity))
        }
        RuleTag::QuantityUpTo => {
            let Lowered::Number(number) = take(children, 2)? else {
                return None;
            };
            Some(Lowered::Quantity(Quantity::UpTo(quantity_value(number))))
        }
        RuleTag::QuantityThatMany => Some(Lowered::Quantity(Quantity::ThatMany)),
        RuleTag::QuantityThatMuch => Some(Lowered::Quantity(Quantity::ThatMuch)),
        RuleTag::DeterminerClosed => take(children, 0),
        RuleTag::DeterminerTarget => Some(Lowered::Determiner(Determiner::Target(None))),
        RuleTag::DeterminerQuantifiedTarget => {
            let Lowered::Quantity(quantity) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::Determiner(Determiner::Target(Some(quantity))))
        }
        RuleTag::DeterminerQuantity => {
            let Lowered::Quantity(quantity) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::Determiner(Determiner::Quantity(quantity)))
        }
        RuleTag::DeterminerPossessiveThisCard => {
            let Lowered::ThisCard(form) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::Determiner(Determiner::Possessive(
                Possessor::NounPhrase(Box::new(NounPhrase::ThisCard(form))),
            )))
        }
        _ => None,
    }
}

/// Whether an adjectival modifier is the `named` participle that introduces a
/// proper name (e.g. `creature named Storm Crow`).
fn introduces_proper_name(adjective: &AdjectivePhrase) -> bool {
    matches!(
        adjective.head,
        Adjective::Participle(_, Verb::Word(Vocab::Name))
    )
}

/// Re-labels keyword-ability catalog atoms inside a proper name so they are
/// carried as case-preserved opaque tokens. Inside `named Storm Crow`, `Storm`
/// is the first word of the name, not the keyword ability, so it must keep its
/// matched source spelling instead of being lowercased by the keyword-atom noun
/// case policy. Everything to the right of the `named` participle — the
/// accumulated modifiers and the head — is name interior at the point this
/// runs.
///
/// Only keyword-ability atoms are re-labelled: their spelling is the matched
/// surface, so opacifying them is casing-faithful, whereas subtype/type atoms
/// already render case- and inflection-faithfully (their spelling is a
/// canonical singular). This also leaves the adjectival `differently named
/// <type>` reading — which shares this flat shape but carries no keyword atom —
/// untouched.
fn open_name_interior(nominal: &mut NominalPhrase) {
    detach_keyword_noun(&mut nominal.head);
    for modifier in &mut nominal.modifiers {
        if let NominalModifier::Noun { noun, .. } = modifier {
            detach_keyword_noun(noun);
        }
    }
}

fn detach_keyword_noun(noun: &mut NounInstance) {
    let inner = match noun {
        NounInstance::Singular(inner) | NounInstance::Plural(inner) | NounInstance::Mass(inner) => {
            inner
        }
    };
    if let Noun::Catalog(atom) = inner
        && atom.kind == CatalogKind::KeywordAbility
    {
        *inner = Noun::Opaque(OpaqueLexeme::new(atom.spelling()));
    }
}

#[allow(
    clippy::too_many_lines,
    reason = "lowering nominals is intentionally long"
)]
fn lower_nominal(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    match tag {
        RuleTag::Adjective | RuleTag::Noun => take(children, 0),
        RuleTag::AdjectivePhraseFaceUp | RuleTag::AdjectivePhraseFaceDown => {
            let orientation = match tag {
                RuleTag::AdjectivePhraseFaceUp => CardOrientation::FaceUp,
                RuleTag::AdjectivePhraseFaceDown => CardOrientation::FaceDown,
                _ => return None,
            };
            Some(Lowered::AdjectivePhrase(AdjectivePhrase {
                degree: None,
                head: Adjective::CardOrientation(orientation),
                complements: Vec::new(),
            }))
        }
        RuleTag::AdjectivePhrase => {
            let Lowered::Adjective(head) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::AdjectivePhrase(AdjectivePhrase {
                degree: None,
                head,
                complements: Vec::new(),
            }))
        }
        RuleTag::AdjectivePhraseComparison => {
            let Lowered::Adjective(head) = take(children, 0)? else {
                return None;
            };
            let Lowered::ComparisonComplement(comparison) = take(children, 1)? else {
                return None;
            };
            Some(Lowered::AdjectivePhrase(AdjectivePhrase {
                degree: None,
                head,
                complements: vec![crate::syntax::AdjectiveComplement::Comparison(comparison)],
            }))
        }
        RuleTag::AdjectivePhraseDegreeMeasure => {
            let Lowered::Number(number) = take(children, 0)? else {
                return None;
            };
            let Lowered::Adjective(head) = take(children, 1)? else {
                return None;
            };
            Some(Lowered::AdjectivePhrase(AdjectivePhrase {
                degree: Some(number.literal()),
                head,
                complements: Vec::new(),
            }))
        }
        RuleTag::ComparisonStandard => {
            let standard = match take(children, 0)? {
                Lowered::Clause(clause) => Phrase::Clause(Box::new(clause)),
                Lowered::NounPhrase(noun_phrase) => Phrase::NounPhrase(Box::new(noun_phrase)),
                Lowered::AdjectivePhrase(adjective) => Phrase::AdjectivePhrase(Box::new(adjective)),
                _ => return None,
            };
            Some(Lowered::Phrase(standard))
        }
        RuleTag::ComparisonThan | RuleTag::ComparisonThanOrEqualTo => {
            let standard_index = if tag == RuleTag::ComparisonThan { 1 } else { 2 };
            let Lowered::Phrase(standard) = take(children, standard_index)? else {
                return None;
            };
            let marker = if tag == RuleTag::ComparisonThan {
                ComparisonMarker::Than
            } else {
                ComparisonMarker::ThanOrEqualTo
            };
            Some(Lowered::ComparisonComplement(ComparisonComplement {
                marker,
                standard: Box::new(standard),
            }))
        }
        RuleTag::NominalNoun => {
            let Lowered::Noun(head) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::Nominal(NominalPhrase {
                determiner: None,
                modifiers: Vec::new(),
                head,
                complements: Vec::new(),
            }))
        }
        RuleTag::NominalCombatStepName => {
            let Lowered::Noun(participants) = take(children, 1)? else {
                return None;
            };
            let Lowered::Noun(head) = take(children, 2)? else {
                return None;
            };
            // Redundant by design (§2.4): the literal-token slots are the
            // defense. If this guard ever fires, the slots have a bug — it
            // is not the safety mechanism.
            if !matches!(
                participants,
                NounInstance::Plural(
                    Noun::Word(_) | Noun::Agentive(Verb::Word(Vocab::Attack | Vocab::Block))
                )
            ) {
                return None;
            }
            if !matches!(head, NounInstance::Singular(Noun::Word(_))) {
                return None;
            }
            Some(Lowered::Nominal(NominalPhrase {
                determiner: None,
                modifiers: vec![NominalModifier::CombatStepName { participants }],
                head,
                complements: Vec::new(),
            }))
        }
        RuleTag::NominalAdjective => {
            let Lowered::AdjectivePhrase(adjective) = take(children, 0)? else {
                return None;
            };
            let Lowered::Nominal(mut nominal) = take(children, 1)? else {
                return None;
            };
            if introduces_proper_name(&adjective) {
                open_name_interior(&mut nominal);
            }
            nominal.modifiers.insert(
                0,
                NominalModifier::Adjective {
                    polarity: Polarity::Positive,
                    phrase: adjective,
                },
            );
            Some(Lowered::Nominal(nominal))
        }
        RuleTag::NominalNounModifier => {
            let Lowered::Noun(noun) = take(children, 0)? else {
                return None;
            };
            let Lowered::Nominal(mut nominal) = take(children, 1)? else {
                return None;
            };
            nominal.modifiers.insert(
                0,
                NominalModifier::Noun {
                    polarity: Polarity::Positive,
                    noun,
                },
            );
            Some(Lowered::Nominal(nominal))
        }
        RuleTag::NominalNegatedModifier => {
            let Lowered::NominalModifier(modifier) = take(children, 0)? else {
                return None;
            };
            let Lowered::Nominal(mut nominal) = take(children, 1)? else {
                return None;
            };
            nominal.modifiers.insert(0, modifier);
            Some(Lowered::Nominal(nominal))
        }
        RuleTag::NominalQuantityModifier => {
            let Lowered::Quantity(quantity) = take(children, 0)? else {
                return None;
            };
            let Lowered::Nominal(mut nominal) = take(children, 1)? else {
                return None;
            };
            nominal
                .modifiers
                .insert(0, NominalModifier::Quantity(quantity));
            Some(Lowered::Nominal(nominal))
        }
        RuleTag::NominalPowerToughnessModifier => {
            let Lowered::PowerToughness(modifier) = take(children, 0)? else {
                return None;
            };
            let Lowered::Nominal(mut nominal) = take(children, 1)? else {
                return None;
            };
            nominal
                .modifiers
                .insert(0, NominalModifier::PowerToughness(modifier));
            Some(Lowered::Nominal(nominal))
        }
        RuleTag::NominalDeterminer => {
            let Lowered::Determiner(determiner) = take(children, 0)? else {
                return None;
            };
            let Lowered::Nominal(mut nominal) = take(children, 1)? else {
                return None;
            };
            nominal.determiner = Some(determiner);
            Some(Lowered::Nominal(nominal))
        }
        RuleTag::NominalPrepositional => {
            let Lowered::Nominal(mut nominal) = take(children, 0)? else {
                return None;
            };
            let Lowered::PrepositionalPhrase(preposition) = take(children, 1)? else {
                return None;
            };
            nominal
                .complements
                .push(NominalComplement::Prepositional(preposition));
            Some(Lowered::Nominal(nominal))
        }
        RuleTag::NominalInfinitive => {
            let Lowered::Nominal(mut nominal) = take(children, 0)? else {
                return None;
            };
            let Lowered::InfinitiveClause(infinitive) = take(children, 1)? else {
                return None;
            };
            nominal
                .complements
                .push(NominalComplement::Infinitive(clause::finish_infinitive(
                    infinitive,
                )?));
            Some(Lowered::Nominal(nominal))
        }
        RuleTag::NominalQuantityComplement => {
            let Lowered::Nominal(mut nominal) = take(children, 0)? else {
                return None;
            };
            let Lowered::Quantity(quantity) = take(children, 1)? else {
                return None;
            };
            nominal
                .complements
                .push(NominalComplement::Quantity(quantity));
            Some(Lowered::Nominal(nominal))
        }
        RuleTag::PredicatedQualityFrom => {
            let quality = match take(children, 1)? {
                Lowered::Adjective(Adjective::Color(color)) => Phrase::ColorWord(color),
                Lowered::NounPhrase(noun_phrase) => Phrase::NounPhrase(Box::new(noun_phrase)),
                _ => return None,
            };
            Some(Lowered::PredicatedQuality(PredicatedQuality {
                preposition: Some(Preposition::From),
                quality,
            }))
        }
        RuleTag::PredicatedQualityBare => {
            let quality = match take(children, 0)? {
                Lowered::Adjective(Adjective::Color(color)) => Phrase::ColorWord(color),
                Lowered::AdjectivePhrase(adjective) => Phrase::AdjectivePhrase(Box::new(adjective)),
                Lowered::NounPhrase(noun_phrase) => Phrase::NounPhrase(Box::new(noun_phrase)),
                _ => return None,
            };
            Some(Lowered::PredicatedQuality(PredicatedQuality {
                preposition: None,
                quality,
            }))
        }
        RuleTag::PredicatedArgumentFromSingle | RuleTag::PredicatedArgumentBareSingle => {
            let Lowered::PredicatedQuality(quality) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::PredicatedArgument(PredicatedArgument {
                qualities: vec![quality],
            }))
        }
        RuleTag::PredicatedArgumentFromExtend | RuleTag::PredicatedArgumentBareExtend => {
            let Lowered::PredicatedArgument(mut argument) = take(children, 0)? else {
                return None;
            };
            let Lowered::PredicatedQuality(quality) = take(children, 2)? else {
                return None;
            };
            argument.qualities.push(quality);
            Some(Lowered::PredicatedArgument(argument))
        }
        RuleTag::NominalKeywordPredicatedArgument
        | RuleTag::NominalKeywordAtomCarriedPredicatedArgument => {
            let Lowered::Noun(head) = take(children, 0)? else {
                return None;
            };
            let Lowered::PredicatedArgument(argument) = take(children, 1)? else {
                return None;
            };
            Some(Lowered::Nominal(NominalPhrase {
                determiner: None,
                modifiers: Vec::new(),
                head,
                complements: vec![NominalComplement::KeywordArgument(
                    KeywordArgument::Predicated(argument),
                )],
            }))
        }
        RuleTag::NominalKeywordSymbolArgument => {
            let Lowered::Noun(head) = take(children, 0)? else {
                return None;
            };
            let symbols = match take(children, 1)? {
                Lowered::OracleSymbol(symbol) => vec![symbol],
                Lowered::SymbolSequence(symbols) => symbols,
                _ => return None,
            };
            Some(Lowered::Nominal(NominalPhrase {
                determiner: None,
                modifiers: Vec::new(),
                head,
                complements: vec![NominalComplement::KeywordArgument(KeywordArgument::Costed(
                    KeywordCost::Symbols(symbols),
                ))],
            }))
        }
        RuleTag::NominalPowerToughnessComplement => {
            let Lowered::Nominal(mut nominal) = take(children, 0)? else {
                return None;
            };
            let Lowered::PowerToughness(power_toughness) = take(children, 1)? else {
                return None;
            };
            nominal
                .complements
                .push(NominalComplement::PowerToughness(power_toughness));
            Some(Lowered::Nominal(nominal))
        }
        RuleTag::NominalRelative => {
            let Lowered::Nominal(mut nominal) = take(children, 0)? else {
                return None;
            };
            let Lowered::RelativeClause(relative) = take(children, 1)? else {
                return None;
            };
            nominal
                .complements
                .push(NominalComplement::Relative(relative));
            Some(Lowered::Nominal(nominal))
        }
        RuleTag::NominalReducedRecipientPassive => {
            let Lowered::Nominal(mut nominal) = take(children, 0)? else {
                return None;
            };
            let Lowered::VerbPhrase(predicate) = take(children, 1)? else {
                return None;
            };
            nominal
                .complements
                .push(NominalComplement::ReducedRecipientPassive(
                    clause::finish_reduced_recipient_passive(predicate)?,
                ));
            Some(Lowered::Nominal(nominal))
        }
        RuleTag::NominalPostpositiveAdjective => {
            let Lowered::Nominal(mut nominal) = take(children, 0)? else {
                return None;
            };
            let Lowered::AdjectivePhrase(adjective) = take(children, 1)? else {
                return None;
            };
            nominal
                .complements
                .push(NominalComplement::Adjective(adjective));
            Some(Lowered::Nominal(nominal))
        }
        RuleTag::NominalComparison => {
            let Lowered::Nominal(mut nominal) = take(children, 0)? else {
                return None;
            };
            let Lowered::ComparisonComplement(comparison) = take(children, 1)? else {
                return None;
            };
            let adjective = nominal.modifiers.iter_mut().rev().find_map(|modifier| {
                let NominalModifier::Adjective {
                    phrase: adjective, ..
                } = modifier
                else {
                    return None;
                };
                (matches!(
                    adjective_comparison_state(&adjective.head),
                    AdjectiveComparisonState::Pending(_)
                ) && !adjective.complements.iter().any(|complement| {
                    matches!(
                        complement,
                        crate::syntax::AdjectiveComplement::Comparison(_)
                            | crate::syntax::AdjectiveComplement::PostnominalComparison(_)
                    )
                }))
                .then_some(adjective)
            })?;
            adjective
                .complements
                .push(crate::syntax::AdjectiveComplement::PostnominalComparison(
                    comparison,
                ));
            Some(Lowered::Nominal(nominal))
        }
        RuleTag::DevotionColorSingle => {
            let Lowered::Adjective(Adjective::Color(color)) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::DevotionColors(
                crate::syntax::DevotionColors::Color(color),
            ))
        }
        RuleTag::DevotionColorPair => {
            let Lowered::Adjective(Adjective::Color(first)) = take(children, 0)? else {
                return None;
            };
            let Lowered::Adjective(Adjective::Color(second)) = take(children, 2)? else {
                return None;
            };
            Some(Lowered::DevotionColors(
                crate::syntax::DevotionColors::Pair(first, second),
            ))
        }
        RuleTag::NominalDevotion => {
            let Lowered::Catalog(atom) = take(children, 0)? else {
                return None;
            };
            let Lowered::DevotionColors(colors) = take(children, 2)? else {
                return None;
            };
            Some(Lowered::Nominal(NominalPhrase {
                determiner: None,
                modifiers: Vec::new(),
                head: NounInstance::Singular(Noun::Catalog(atom)),
                complements: vec![NominalComplement::Devotion(colors)],
            }))
        }
        RuleTag::NominalTimesClause => {
            let Lowered::Noun(head) = take(children, 0)? else {
                return None;
            };
            let Lowered::Clause(Clause::Independent(clause)) = take(children, 1)? else {
                return None;
            };
            Some(Lowered::Nominal(NominalPhrase {
                determiner: None,
                modifiers: Vec::new(),
                head,
                complements: vec![NominalComplement::EventClause(Box::new(clause))],
            }))
        }
        RuleTag::ModifierConjunctAdjective => {
            let Lowered::AdjectivePhrase(phrase) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::NominalModifier(NominalModifier::Adjective {
                polarity: Polarity::Positive,
                phrase,
            }))
        }
        RuleTag::ModifierConjunctNoun => {
            let Lowered::Noun(noun) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::NominalModifier(NominalModifier::Noun {
                polarity: Polarity::Positive,
                noun,
            }))
        }
        RuleTag::ModifierConjunctNegated => {
            // The `non-` lexeme already lowers to a negated `NominalModifier`.
            let Lowered::NominalModifier(modifier) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::NominalModifier(modifier))
        }
        RuleTag::ModifierListSingle => {
            let Lowered::NominalModifier(first) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::CoordinatedModifier(
                crate::syntax::CoordinatedModifier {
                    first: Box::new(first),
                    rest: Vec::new(),
                },
            ))
        }
        RuleTag::ModifierListComma
        | RuleTag::CoordinatedModifierConjoined
        | RuleTag::CoordinatedModifierOxford => {
            let Lowered::CoordinatedModifier(mut coordinated) = take(children, 0)? else {
                return None;
            };
            let (conjunction, comma, modifier_index) = match tag {
                RuleTag::ModifierListComma => (None, true, 2),
                RuleTag::CoordinatedModifierConjoined => (Some(1), false, 2),
                RuleTag::CoordinatedModifierOxford => (Some(2), true, 3),
                _ => return None,
            };
            let conjunction = match conjunction {
                Some(index) => {
                    let Lowered::Conjunction(conjunction) = take(children, index)? else {
                        return None;
                    };
                    Some(conjunction)
                }
                None => None,
            };
            let Lowered::NominalModifier(modifier) = take(children, modifier_index)? else {
                return None;
            };
            coordinated.rest.push(crate::syntax::ModifierCoordination {
                conjunction,
                comma,
                modifier,
            });
            Some(Lowered::CoordinatedModifier(coordinated))
        }
        RuleTag::NominalCoordinatedModifier => {
            let Lowered::CoordinatedModifier(coordinated) = take(children, 0)? else {
                return None;
            };
            let Lowered::Nominal(mut nominal) = take(children, 1)? else {
                return None;
            };
            nominal
                .modifiers
                .insert(0, NominalModifier::Coordinated(coordinated));
            Some(Lowered::Nominal(nominal))
        }
        _ => None,
    }
}

#[allow(
    clippy::too_many_lines,
    reason = "lowering noun phrases is intentionally long"
)]
fn lower_phrase(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    match tag {
        RuleTag::NounPhraseSetExceptionBare | RuleTag::NounPhraseSetExceptionFor => {
            let (marker, comma, excluded_index) = match (tag, children.len()) {
                (RuleTag::NounPhraseSetExceptionBare, 3) => (SetExceptionMarker::Bare, false, 2),
                (RuleTag::NounPhraseSetExceptionBare, 4) => (SetExceptionMarker::Bare, true, 3),
                (RuleTag::NounPhraseSetExceptionFor, 4) => (SetExceptionMarker::For, false, 3),
                (RuleTag::NounPhraseSetExceptionFor, 5) => (SetExceptionMarker::For, true, 4),
                _ => return None,
            };
            let Lowered::NounPhrase(included) = take(children, 0)? else {
                return None;
            };
            let Lowered::NounPhrase(excluded) = take(children, excluded_index)? else {
                return None;
            };
            Some(Lowered::NounPhrase(NounPhrase::SetException(
                SetExceptionNounPhrase {
                    included: Box::new(included),
                    marker,
                    comma,
                    excluded: Box::new(excluded),
                },
            )))
        }
        RuleTag::NounPhraseNominal | RuleTag::ReducedRecipientPassiveTheme => {
            let Lowered::Nominal(nominal) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::NounPhrase(NounPhrase::Nominal(nominal)))
        }
        RuleTag::NounPhraseSubjectPronoun
        | RuleTag::NounPhraseObjectPronoun
        | RuleTag::NounPhraseReciprocal => {
            let Lowered::Pronoun(pronoun) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::NounPhrase(NounPhrase::Pronoun {
                pronoun: pronoun.pronoun,
                case: pronoun.case,
            }))
        }
        RuleTag::NounPhraseQuantity => {
            let Lowered::Quantity(quantity) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::NounPhrase(NounPhrase::Quantity(quantity)))
        }
        RuleTag::NounPhraseThisCard | RuleTag::NounPhraseFullThisCard => {
            let Lowered::ThisCard(form) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::NounPhrase(NounPhrase::ThisCard(form)))
        }
        RuleTag::NounPhrasePossessiveThisCard => {
            let Lowered::ThisCard(form) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::NounPhrase(NounPhrase::Possessive(
                Possessor::NounPhrase(Box::new(NounPhrase::ThisCard(form))),
            )))
        }
        RuleTag::NounPhraseDemonstrative => {
            let Lowered::Determiner(crate::syntax::Determiner::Demonstrative(demonstrative)) =
                take(children, 0)?
            else {
                return None;
            };
            Some(Lowered::NounPhrase(NounPhrase::Demonstrative(
                demonstrative,
            )))
        }
        RuleTag::NounPhrasePartitive => {
            let Lowered::Quantity(quantity) = take(children, 0)? else {
                return None;
            };
            let Lowered::Preposition(Preposition::Of) = take(children, 1)? else {
                return None;
            };
            let Lowered::NounPhrase(whole) = take(children, 2)? else {
                return None;
            };
            Some(Lowered::NounPhrase(NounPhrase::Partitive(
                crate::syntax::PartitiveNounPhrase {
                    head: crate::syntax::PartitiveHead::Quantity(quantity),
                    whole: Box::new(whole),
                },
            )))
        }
        RuleTag::NounPhraseEachPartitive => {
            let Lowered::Determiner(crate::syntax::Determiner::Each) = take(children, 0)? else {
                return None;
            };
            let Lowered::Preposition(Preposition::Of) = take(children, 1)? else {
                return None;
            };
            let Lowered::NounPhrase(whole) = take(children, 2)? else {
                return None;
            };
            Some(Lowered::NounPhrase(NounPhrase::Partitive(
                crate::syntax::PartitiveNounPhrase {
                    head: crate::syntax::PartitiveHead::Each,
                    whole: Box::new(whole),
                },
            )))
        }
        RuleTag::NounPhraseAnyNumberOf => {
            // Lower to the byte-identical ordinary nominal shape: the two
            // analyses (this notional-plural reduce vs. the formal-singular
            // `NounPhraseNominal` path) must differ only in parse features,
            // never in stored or rendered structure [`anof` round].
            let Lowered::Determiner(determiner @ crate::syntax::Determiner::Any) =
                take(children, 0)?
            else {
                return None;
            };
            let Lowered::Noun(head @ NounInstance::Singular(Noun::Word(Vocab::Number))) =
                take(children, 1)?
            else {
                return None;
            };
            let Lowered::Preposition(preposition @ Preposition::Of) = take(children, 2)? else {
                return None;
            };
            let Lowered::NounPhrase(whole) = take(children, 3)? else {
                return None;
            };
            Some(Lowered::NounPhrase(NounPhrase::Nominal(NominalPhrase {
                determiner: Some(determiner),
                modifiers: Vec::new(),
                head,
                complements: vec![NominalComplement::Prepositional(PrepositionalPhrase {
                    preposition,
                    object: Box::new(Phrase::NounPhrase(Box::new(whole))),
                })],
            })))
        }
        RuleTag::NounPhraseCoordination | RuleTag::NounPhraseAdditiveCoordination => {
            let Lowered::NounPhrase(first) = take(children, 0)? else {
                return None;
            };
            let conjunction = if tag == RuleTag::NounPhraseAdditiveCoordination {
                crate::syntax::NounPhraseConjunction::Plus
            } else {
                let Lowered::Conjunction(conjunction) = take(children, 1)? else {
                    return None;
                };
                match conjunction {
                    crate::syntax::PredicateConjunction::And => {
                        crate::syntax::NounPhraseConjunction::And
                    }
                    crate::syntax::PredicateConjunction::Or => {
                        crate::syntax::NounPhraseConjunction::Or
                    }
                    crate::syntax::PredicateConjunction::AndOr => {
                        crate::syntax::NounPhraseConjunction::AndOr
                    }
                    // `then` never joins noun phrases.
                    crate::syntax::PredicateConjunction::Then => return None,
                }
            };
            let Lowered::NounPhrase(next) = take(children, 2)? else {
                return None;
            };
            let coordination = crate::syntax::NounPhraseCoordination {
                conjunction: Some(conjunction),
                comma: false,
                phrase: next,
            };
            Some(Lowered::NounPhrase(NounPhrase::Coordinated(
                push_noun_phrase_coordination(first, coordination),
            )))
        }
        RuleTag::NounPhraseListSingle => take(children, 0),
        RuleTag::NounPhraseListComma | RuleTag::NounPhraseCoordinationOxford => {
            let Lowered::NounPhrase(first) = take(children, 0)? else {
                return None;
            };
            let (conjunction, next_index) = if tag == RuleTag::NounPhraseCoordinationOxford {
                let Lowered::Conjunction(conjunction) = take(children, 2)? else {
                    return None;
                };
                let conjunction = match conjunction {
                    crate::syntax::PredicateConjunction::And => {
                        crate::syntax::NounPhraseConjunction::And
                    }
                    crate::syntax::PredicateConjunction::Or => {
                        crate::syntax::NounPhraseConjunction::Or
                    }
                    crate::syntax::PredicateConjunction::AndOr => {
                        crate::syntax::NounPhraseConjunction::AndOr
                    }
                    // `then` never joins noun phrases.
                    crate::syntax::PredicateConjunction::Then => return None,
                };
                (Some(conjunction), 3)
            } else {
                (None, 2)
            };
            let Lowered::NounPhrase(next) = take(children, next_index)? else {
                return None;
            };
            let coordination = crate::syntax::NounPhraseCoordination {
                conjunction,
                comma: true,
                phrase: next,
            };
            Some(Lowered::NounPhrase(NounPhrase::Coordinated(
                push_noun_phrase_coordination(first, coordination),
            )))
        }
        RuleTag::NounPhraseMinus
        | RuleTag::NounPhraseHalf
        | RuleTag::NounPhraseHalfRoundedUp
        | RuleTag::NounPhraseHalfRoundedDown => lower_arithmetic_phrase(tag, children),
        RuleTag::PrepositionalObject => {
            let phrase = match take(children, 0)? {
                Lowered::NounPhrase(object) => Phrase::NounPhrase(Box::new(object)),
                Lowered::PrepositionalPhrase(object) => {
                    Phrase::PrepositionalPhrase(Box::new(object))
                }
                Lowered::GerundClause(object) => Phrase::Clause(Box::new(Clause::Dependent(
                    crate::syntax::DependentClause::Gerund(object),
                ))),
                Lowered::Adverb(object) => Phrase::Adverb(object),
                _ => return None,
            };
            Some(Lowered::Phrase(phrase))
        }
        RuleTag::PrepositionalPhrase => {
            let Lowered::Preposition(preposition) = take(children, 0)? else {
                return None;
            };
            let Lowered::Phrase(object) = take(children, 1)? else {
                return None;
            };
            Some(Lowered::PrepositionalPhrase(PrepositionalPhrase {
                preposition,
                object: Box::new(object),
            }))
        }
        _ => None,
    }
}

fn lower_arithmetic_phrase(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    let value = match tag {
        RuleTag::NounPhraseMinus => {
            let Lowered::NounPhrase(left) = take(children, 0)? else {
                return None;
            };
            let Lowered::NounPhrase(right) = take(children, 2)? else {
                return None;
            };
            crate::syntax::ArithmeticValue::Minus {
                left: Box::new(left),
                right: Box::new(right),
            }
        }
        RuleTag::NounPhraseHalf
        | RuleTag::NounPhraseHalfRoundedUp
        | RuleTag::NounPhraseHalfRoundedDown => {
            let Lowered::NounPhrase(value) = take(children, 1)? else {
                return None;
            };
            let rounding = match tag {
                RuleTag::NounPhraseHalfRoundedUp => Some(crate::syntax::Rounding::Up),
                RuleTag::NounPhraseHalfRoundedDown => Some(crate::syntax::Rounding::Down),
                _ => None,
            };
            crate::syntax::ArithmeticValue::Half {
                value: Box::new(value),
                rounding,
            }
        }
        _ => return None,
    };
    Some(Lowered::NounPhrase(NounPhrase::Arithmetic(value)))
}

fn take(children: &mut [Lowered], index: usize) -> Option<Lowered> {
    let child = children.get_mut(index)?;
    Some(std::mem::replace(child, Lowered::Ignored))
}

/// Appends a coordination member to a noun-phrase coordination, extending an
/// existing flat list in place or opening a fresh one — the shared tail of the
/// binary, comma, and Oxford coordination rules.
fn push_noun_phrase_coordination(
    first: NounPhrase,
    coordination: crate::syntax::NounPhraseCoordination,
) -> crate::syntax::CoordinatedNounPhrase {
    match first {
        NounPhrase::Coordinated(mut coordinated) => {
            coordinated.rest.push(coordination);
            coordinated
        }
        first => crate::syntax::CoordinatedNounPhrase {
            first: Box::new(first),
            rest: vec![coordination],
        },
    }
}

#[cfg(test)]
mod litaudit_tests {
    use super::*;

    fn fixture_catalogs() -> Catalogs {
        Catalogs::default()
            .with_catalog(CatalogKind::CreatureType, ["Goblin", "Human"])
            .with_catalog(CatalogKind::CardType, ["Artifact", "Creature", "Land"])
            .with_catalog(CatalogKind::Supertype, ["Basic", "Legendary"])
    }

    #[test]
    fn every_reserved_literal_is_a_known_word() {
        // has_known_word's own invariant: a word the lexicon already knows —
        // including hand-written literal lexemes, not just vocabulary/catalog
        // slots — must never be reported as unknown (and so must never be
        // opacified). Extends pluralposs's
        // `a_known_literal_lexeme_is_never_opacifiable` (which folds into this
        // test) to the full `OPACITY_RESERVED_LITERALS` table (round litaudit).
        let catalogs = fixture_catalogs();
        for literal in EnglishGrammar::OPACITY_RESERVED_LITERALS {
            let surface = crate::surface::lex(literal);
            let grammar = EnglishGrammar::new(literal, &catalogs, Nonterminal::NounPhrase);
            assert!(
                grammar.has_known_word(&surface.tokens, 0),
                "{literal} must be a known word"
            );
        }
    }

    #[test]
    fn reserved_literals_are_never_opaque_nouns() {
        // Per-literal parse assertions for the seven reserved literals with
        // corpus witnesses (litaudit-plan.md §1.2). Whichever of the two
        // outcomes ((i) re-parse via the literal's real production, or (ii)
        // the sentence honestly loses its only complete parse) the face lands
        // in, no `Opaque(OpaqueLexeme(<literal>))` node may survive in the
        // tree: an `Err` result (outcome (ii): no complete parse) or an `Ok`
        // result whose debug dump contains no reserved-literal `OpaqueLexeme`
        // both satisfy the invariant.
        let catalogs = fixture_catalogs();
        let witnesses: &[(&str, &str)] = &[
            (
                "there",
                "Exile target creature card from a graveyard that was put there this turn.",
            ),
            (
                "up",
                "Whenever this creature deals combat damage to a player, return up to that many target permanents that player controls to their owner's hand.",
            ),
            (
                "than",
                "Damage that would reduce your life total to less than 1 reduces it to 1 instead.",
            ),
            (
                "it's",
                "Whenever one or more +1/+1 counters are put on another permanent you control, if it's the first time +1/+1 counters have been put on that permanent this turn, put a +1/+1 counter on this creature.",
            ),
            (
                "that's",
                "Target creature card in your graveyard that's an artifact or that has mana value 3 or less gains escape until end of turn.",
            ),
            (
                "down",
                "You may return this card from your graveyard to the battlefield face up or face down.",
            ),
            (
                "minus",
                "If one or more -1/-1 counters would be put on a creature you control, that many -1/-1 counters minus one are put on it instead.",
            ),
        ];
        for (literal, source) in witnesses {
            match parse_nonterminal(source, &catalogs, Nonterminal::Sentence) {
                Err(_) => {
                    // Outcome (ii): the sentence honestly loses its only
                    // complete parse once the literal can no longer be
                    // swallowed as an opaque noun. No tree, so no opaque node
                    // of any kind can survive.
                }
                Ok(parsed) => {
                    let dump = format!("{parsed:#?}");
                    let needle = format!("OpaqueLexeme(\n        \"{literal}\"");
                    let needle_inline = format!("OpaqueLexeme(\"{literal}\")");
                    assert!(
                        !dump.contains(&needle) && !dump.contains(&needle_inline),
                        "{literal}: reserved literal survived as an opaque noun: {dump}"
                    );
                }
            }
        }
    }

    #[test]
    fn the_opacity_gate_is_the_predicates_only_caller() {
        // Pins litaudit-plan.md §4 structurally: has_known_word gates
        // opacification only, at its two call sites in `scan`'s
        // `EnglishLexicalSlot::Opaque` arm. This is the executable half; the
        // source-grep gate (`rg -n 'has_known_word'`) is run by the mechanic
        // per round and is not itself a test.
        let catalogs = fixture_catalogs();

        // Call site 1 (`already_known`, :2027): a known-word start yields no
        // opaque candidates at all.
        let source = "who";
        let surface = crate::surface::lex(source);
        let grammar = EnglishGrammar::with_opacity_profile(
            source,
            &catalogs,
            Nonterminal::NounPhrase,
            OpacityProfile::Nouns,
            SelfReference::default(),
        );
        let matches = grammar.scan(
            EnglishLexicalSlot::Opaque(OpacitySlot::Noun(NounForm::Singular)),
            &surface.tokens,
            0,
        );
        assert!(
            matches.is_empty(),
            "a known-word start must never yield an opaque candidate: {matches:?}"
        );

        // Call site 2 (the `retain`, :2032-2035): an opaque candidate that
        // would span a known reserved literal is dropped even when the head
        // token itself is unknown.
        let source = "gloopmonster minus one";
        let surface = crate::surface::lex(source);
        let grammar = EnglishGrammar::with_opacity_profile(
            source,
            &catalogs,
            Nonterminal::NounPhrase,
            OpacityProfile::Nouns,
            SelfReference::default(),
        );
        let matches = grammar.scan(
            EnglishLexicalSlot::Opaque(OpacitySlot::Noun(NounForm::Singular)),
            &surface.tokens,
            0,
        );
        assert!(
            matches.iter().all(|candidate| !(1..candidate.end)
                .any(|index| grammar.has_known_word(&surface.tokens, index))),
            "no surviving opaque candidate may span a known word: {matches:?}"
        );
    }
}
