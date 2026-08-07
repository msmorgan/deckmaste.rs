pub(crate) mod ability;
mod clause;
pub(crate) mod construction;
mod opacity;

#[cfg(test)]
#[path = "tests/nominal.rs"]
mod nominal;

mod generated;

#[cfg(test)]
mod exact;

mod lowering;
#[path = "parse_nonterminal.rs"]
mod parse_support;
mod reduction;
mod rules;
mod scan;

#[cfg(test)]
#[path = "tests/litaudit.rs"]
mod litaudit_tests;

use std::collections::HashMap;

use lowering::Lowered;
use lowering::take;
pub(crate) use parse_support::ParsedNonterminal;
#[cfg(test)]
use parse_support::parse_nonterminal;
#[cfg(test)]
use parse_support::parse_nonterminal_with_mode;
pub(crate) use parse_support::parse_nonterminal_with_self_reference;
use reduction::Reduced;
use reduction::propagate;
use reduction::reduce;
use rules::RegistrationOrder;
use rules::RuleBuilder;
use rules::RuleImpl;
use scan::accepts_possessive_modifier_prefix;
use scan::accepts_set_exception_prefix;
pub(crate) use scan::adjective_comparison_state;
use scan::copula_agreement;
use scan::lexical_word_matches;
use scan::literal_match;
use scan::noun_phrase_accepts_set_exception;
use scan::parse_power_toughness;
use scan::parse_symbol_sequence;
use scan::possessive_this_card_match;
use scan::pronoun_match;
use scan::this_card_matches;

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
use crate::features::ChartFeature;
use crate::features::ChartFeatureBundle;
use crate::features::Conjunction;
use crate::features::FeatureKind;
use crate::features::GapState;
#[cfg(test)]
use crate::features::GapState as RelativeGap;
use crate::features::GrammaticalFeature;
use crate::features::NounCardinality;
use crate::features::Number;
use crate::features::Onset as InitialSound;
use crate::features::Person;
use crate::features::PronounCase;
use crate::features::PronounClass as Pronoun;
use crate::features::SurfaceWitnessPayload;
use crate::features::VerbSlot;
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
use crate::syntax::NominalComplement;
use crate::syntax::NominalModifier;
use crate::syntax::NominalPhrase;
use crate::syntax::NounPhrase;
use crate::syntax::NumberLiteral;
use crate::syntax::OpaqueLexeme;
use crate::syntax::OracleSymbol;
use crate::syntax::Phrase;
use crate::syntax::Polarity;
use crate::syntax::Possessor;
use crate::syntax::PowerToughness;
use crate::syntax::Preposition;
use crate::syntax::PrepositionalPhrase;
use crate::syntax::PreverbModifier;
use crate::syntax::Quantity;
use crate::syntax::QuantityValue;
use crate::syntax::RelativeClause;
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
use crate::word::LexicalSlot;
use crate::word::Noun;
use crate::word::NounInstance;
use crate::word::NounUsage;
use crate::word::PredicateComplementKind;
use crate::word::PredicateFrame;
use crate::word::PronounInstance;
use crate::word::Verb;
use crate::word::VerbInstance;
use crate::word::Vocab;
use crate::word::Vocabulary;
use crate::word::WordMatch;
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

#[allow(
    dead_code,
    reason = "subordinate verb dependents are staged for later grammar milestones"
)]
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

/// The grammatical contribution of an auxiliary to chart identity.
///
/// Exact contraction is retained by [`EnglishSurfaceWitness`]; it does not
/// change which constructions the auxiliary can enter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct AuxiliaryFeatures {
    auxiliary: Auxiliary,
    inflection: AuxiliaryInflection,
}

impl From<AuxiliaryInstance> for AuxiliaryFeatures {
    fn from(instance: AuxiliaryInstance) -> Self {
        Self {
            auxiliary: instance.auxiliary,
            inflection: instance.inflection,
        }
    }
}

impl AuxiliaryFeatures {
    const fn with_contraction(
        self,
        contraction: crate::features::Contraction,
    ) -> AuxiliaryInstance {
        AuxiliaryInstance {
            auxiliary: self.auxiliary,
            inflection: self.inflection,
            contracted_negation: contraction,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ExistentialKey {
    verb_slot: VerbSlot,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum EnglishSurfaceWitness {
    #[default]
    None,
    Contraction(crate::features::Contraction),
}

impl EnglishSurfaceWitness {
    const fn contraction(self) -> Option<crate::features::Contraction> {
        match self {
            Self::None => None,
            Self::Contraction(contraction) => Some(contraction),
        }
    }
}

impl SurfaceWitnessPayload for EnglishSurfaceWitness {}

fn matching_contraction(
    matches: impl Fn(crate::features::Contraction) -> bool,
) -> Option<crate::features::Contraction> {
    let mut matches = [
        crate::features::Contraction::Full,
        crate::features::Contraction::Contracted,
    ]
    .into_iter()
    .filter(|&contraction| matches(contraction));
    let contraction = matches.next()?;
    matches.next().is_none().then_some(contraction)
}

fn subject_auxiliary_spelling_matches(
    key: SubjectAuxiliaryKey,
    matches: impl Fn(&str) -> bool,
) -> bool {
    SUBJECT_AUXILIARY_FORMS
        .iter()
        .filter(|(_, subject, auxiliaries)| {
            if *subject != key.subject {
                return false;
            }
            let Agreement { person, number } = subject.agreement();
            auxiliaries.iter().any(|&auxiliary| {
                AuxiliaryFeatures {
                    auxiliary,
                    inflection: AuxiliaryInflection::Present { person, number },
                } == key.auxiliary
            })
        })
        .filter(|(surface_index, _, _)| matches(SUBJECT_AUXILIARY_SURFACES[*surface_index]))
        .count()
        == 1
}

impl ContractedSubjectKey {
    fn agreement(self) -> Agreement {
        match self {
            Self::Pronoun(Pronoun::You) => Agreement {
                person: Person::Second,
                number: Number::Singular,
            },
            Self::Pronoun(Pronoun::They) => Agreement {
                person: Person::Third,
                number: Number::Plural,
            },
            Self::Pronoun(Pronoun::It(_)) | Self::Demonstrative(Demonstrative::That) => Agreement {
                person: Person::Third,
                number: Number::Singular,
            },
            Self::Pronoun(
                Pronoun::EachOther | Pronoun::Itself | Pronoun::Himself | Pronoun::YoursAbsolute,
            )
            | Self::Demonstrative(
                Demonstrative::This | Demonstrative::These | Demonstrative::Those,
            ) => unreachable!("these subjects have no contracted-auxiliary form"),
        }
    }
}

const SUBJECT_AUXILIARY_SURFACES: [&str; 8] = [
    "you're", "you've", "he's", "she's", "it's", "that's", "they're", "they've",
];

const SUBJECT_AUXILIARY_FORMS: &[(usize, ContractedSubjectKey, &[Auxiliary])] = &[
    (
        0,
        ContractedSubjectKey::Pronoun(Pronoun::You),
        &[Auxiliary::Be],
    ),
    (
        1,
        ContractedSubjectKey::Pronoun(Pronoun::You),
        &[Auxiliary::Have],
    ),
    (
        2,
        ContractedSubjectKey::Pronoun(Pronoun::It(crate::word::Gender::Masculine)),
        &[Auxiliary::Be, Auxiliary::Have],
    ),
    (
        3,
        ContractedSubjectKey::Pronoun(Pronoun::It(crate::word::Gender::Feminine)),
        &[Auxiliary::Be, Auxiliary::Have],
    ),
    (
        4,
        ContractedSubjectKey::Pronoun(Pronoun::It(crate::word::Gender::Neuter)),
        &[Auxiliary::Be, Auxiliary::Have],
    ),
    (
        5,
        ContractedSubjectKey::Demonstrative(Demonstrative::That),
        &[Auxiliary::Be, Auxiliary::Have],
    ),
    (
        6,
        ContractedSubjectKey::Pronoun(Pronoun::They),
        &[Auxiliary::Be],
    ),
    (
        7,
        ContractedSubjectKey::Pronoun(Pronoun::They),
        &[Auxiliary::Have],
    ),
];

#[allow(
    dead_code,
    reason = "ability-root nonterminals are staged for later grammar milestones"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Nonterminal {
    Quantity,
    /// Generated complement payload category backed by the closed P/T lexer.
    PowerToughness,
    Determiner,
    Adjective,
    AdjectivePhrase,
    ComparisonStandard,
    ComparisonComplement,
    Noun,
    Nominal,
    NounPhrase,
    /// A nominal whose rightmost complement is an object-gap relative with a
    /// predicate that requires a rules object. This category is predicted
    /// only while building the final member of a coordinated PP object, so
    /// its attachment state does not leak into ordinary noun-phrase keys.
    RulesObjectNominal,
    /// A [`Self::RulesObjectNominal`] extended by at least one later relative;
    /// subsequent PP complements stay in this category. The two-stage shape
    /// lets the parser consume `permanent ... controls that shares ... with
    /// it` without treating every PP after `... you control` as nominal.
    RulesObjectFollowupNominal,
    /// Noun-phrase promotion for either rules-object nominal category.
    RulesObjectNounPhrase,
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
    /// The comma run of a sibling prepositional coordination (`from Vampires,
    /// from Werewolves`). Reached only from the Oxford-close rule, so a bare
    /// comma run of prepositional phrases never coordinates on its own —
    /// the same restriction [`Self::NounPhraseList`] carries.
    PrepositionalPhraseList,
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
    /// An exception rider whose latest member was added by an asyndetic comma
    /// (`except A, B`). It may grow or close with a conjunction; attaching it
    /// directly to the host remains a dispreferred fallback.
    ExceptionRiderList,
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
    /// An internal helper category from a generated construction group. The
    /// id is allocated deterministically from the sorted internal-category
    /// names of the active groups — never from registration order.
    Generated(u16),
}

#[allow(
    dead_code,
    reason = "ability-word scanning is staged for later grammar milestones"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum EnglishLexicalSlot {
    Number(Numeral),
    /// All supported numeral notations behind one generated scalar codec.
    QuantityNumber,
    /// The closed four-word comparative vocabulary used by quantity bounds.
    ComparativeWord,
    /// A declaration-owned fixed word or word sequence.
    GeneratedLiteral(&'static str),
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
    /// A noun-phrase conjunction, including the additive `plus` form.
    NounPhraseConjunction,
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
    /// The generated opaque-noun identity provider. One prediction yields
    /// singular, plural, and mass candidates with their form retained in the
    /// lexical feature/meaning pair.
    OpaqueNoun,
}

impl EnglishLexicalSlot {
    /// Fixed word surfaces owned by this lexical slot. Sequence-valued slots
    /// return their words in order; alternative-valued slots return every
    /// accepted one-token surface. The exhaustive match is intentional: a new
    /// lexical slot must state whether it owns fixed literal words.
    const fn literal_surfaces(self) -> &'static [&'static str] {
        match self {
            Self::TimesNoun => &["times"],
            Self::NumberNoun => &["number"],
            Self::PreverbAdverb => &["next"],
            Self::CombatStepDeclare => &["declare"],
            Self::CombatStepParticipants => &["attackers", "blockers"],
            Self::CombatStepHead => &["step"],
            Self::VerbParticle(VerbParticle::In) => &["in"],
            Self::VerbParticle(VerbParticle::Out) => &["out"],
            Self::CoinResult(crate::syntax::CoinSide::Heads) => &["up", "heads"],
            Self::CoinResult(crate::syntax::CoinSide::Tails) => &["up", "tails"],
            Self::FromWord => &["from"],
            Self::DeterminerTarget => &["target"],
            Self::Than => &["than"],
            Self::OrEqualTo => &["or", "equal", "to"],
            Self::RelativeMarker => &["who", "that"],
            Self::Face => &["face"],
            Self::Up => &["up"],
            Self::Down => &["down"],
            Self::Not => &["not"],
            Self::To => &["to"],
            Self::Of => &["of"],
            Self::ForWord => &["for"],
            Self::EachDeterminer => &["each"],
            Self::AnyDeterminer => &["any"],
            Self::Reciprocal => &["each", "other"],
            Self::RatherThan => &["rather", "than"],
            Self::Plus => &["plus"],
            Self::Minus => &["minus"],
            Self::Half => &["half"],
            Self::Rounded => &["rounded"],
            Self::Existential => &["there", "there's"],
            Self::SubjectAuxiliary => &SUBJECT_AUXILIARY_SURFACES,
            Self::Except => &["except"],

            Self::Number(_)
            | Self::QuantityNumber
            | Self::ComparativeWord
            | Self::GeneratedLiteral(_)
            | Self::Noun(_)
            | Self::PossessiveNoun
            | Self::Verb(_)
            | Self::ReducedRecipientPassiveParticiple
            | Self::Adjective
            | Self::ColorWord
            | Self::DevotionValue
            | Self::NegatedModifier
            | Self::Adverb
            | Self::SentenceAdverbial
            | Self::Frequency
            | Self::Pronoun(_)
            | Self::Auxiliary
            | Self::AbilityItem
            | Self::AbilityWord
            | Self::SymbolArgumentKeywordNoun
            | Self::ExplicitPredicatedKeywordNoun
            | Self::AtomCarriedPredicatedKeywordNoun
            | Self::Determiner
            | Self::Demonstrative
            | Self::ThisCard
            | Self::FullThisCard
            | Self::PossessiveThisCard
            | Self::Preposition
            | Self::OracleSymbol
            | Self::SymbolSequence
            | Self::QuotedAbility
            | Self::PowerToughness
            | Self::Punctuation(_)
            | Self::Subordinator
            | Self::Conjunction
            | Self::NounPhraseConjunction
            | Self::Copula
            | Self::OpaqueNoun => &[],
        }
    }

    /// Every fixed-literal slot, including deliberately opacity-visible slots.
    /// Scanner arms and the opacity predicate both consume this table.
    const LITERAL_SLOTS: &'static [Self] = &[
        Self::TimesNoun,
        Self::NumberNoun,
        Self::PreverbAdverb,
        Self::CombatStepDeclare,
        Self::CombatStepParticipants,
        Self::CombatStepHead,
        Self::VerbParticle(VerbParticle::In),
        Self::VerbParticle(VerbParticle::Out),
        Self::CoinResult(crate::syntax::CoinSide::Heads),
        Self::CoinResult(crate::syntax::CoinSide::Tails),
        Self::FromWord,
        Self::DeterminerTarget,
        Self::Than,
        Self::OrEqualTo,
        Self::RelativeMarker,
        Self::Face,
        Self::Up,
        Self::Down,
        Self::Not,
        Self::To,
        Self::Of,
        Self::ForWord,
        Self::EachDeterminer,
        Self::AnyDeterminer,
        Self::Reciprocal,
        Self::RatherThan,
        Self::Plus,
        Self::Minus,
        Self::Half,
        Self::Rounded,
        Self::Existential,
        Self::SubjectAuxiliary,
        Self::Except,
    ];

    /// Whether one surface from [`Self::literal_surfaces`] participates in the
    /// opacity known-word invariant. Indexing avoids a second spelling table.
    const fn reserves_literal_for_opacity(self, surface_index: usize) -> bool {
        match self {
            Self::Plus
            | Self::Except
            | Self::Not
            | Self::Up
            | Self::Than
            | Self::Down
            | Self::Minus => true,
            // Reserve `who` but not determiner-known `that`, and measured
            // `there` but not the still-unaudited contraction `there's`.
            Self::RelativeMarker | Self::Existential => surface_index == 0,
            // `it's` and `that's` are measured. The other contractions remain
            // opacity-visible until their second-order retain effects are audited.
            Self::SubjectAuxiliary => matches!(surface_index, 4 | 5),
            // In particular, `out` must remain opacifiable for an unlicensed
            // verb-particle pair; `both`, `half`, `rounded`, `rather`, and the
            // remaining contracted forms are likewise explicit measured-work
            // opt-outs, not accidental omissions.
            Self::Number(_)
            | Self::QuantityNumber
            | Self::ComparativeWord
            | Self::GeneratedLiteral(_)
            | Self::Noun(_)
            | Self::PossessiveNoun
            | Self::Verb(_)
            | Self::ReducedRecipientPassiveParticiple
            | Self::Adjective
            | Self::ColorWord
            | Self::DevotionValue
            | Self::TimesNoun
            | Self::NumberNoun
            | Self::PreverbAdverb
            | Self::CombatStepDeclare
            | Self::CombatStepParticipants
            | Self::CombatStepHead
            | Self::NegatedModifier
            | Self::Adverb
            | Self::SentenceAdverbial
            | Self::VerbParticle(_)
            | Self::CoinResult(_)
            | Self::Frequency
            | Self::Pronoun(_)
            | Self::Auxiliary
            | Self::AbilityItem
            | Self::AbilityWord
            | Self::SymbolArgumentKeywordNoun
            | Self::FromWord
            | Self::ExplicitPredicatedKeywordNoun
            | Self::AtomCarriedPredicatedKeywordNoun
            | Self::Determiner
            | Self::Demonstrative
            | Self::DeterminerTarget
            | Self::OrEqualTo
            | Self::Face
            | Self::To
            | Self::Of
            | Self::ForWord
            | Self::EachDeterminer
            | Self::AnyDeterminer
            | Self::Reciprocal
            | Self::ThisCard
            | Self::FullThisCard
            | Self::PossessiveThisCard
            | Self::Preposition
            | Self::OracleSymbol
            | Self::SymbolSequence
            | Self::QuotedAbility
            | Self::PowerToughness
            | Self::Punctuation(_)
            | Self::Subordinator
            | Self::RatherThan
            | Self::Conjunction
            | Self::NounPhraseConjunction
            | Self::Half
            | Self::Rounded
            | Self::Copula
            | Self::OpaqueNoun => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum OpacityMode {
    Exact,
    OpaqueNouns,
}

const OPACITY_STATE_LIMIT: usize = 50_000;

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
pub(crate) struct QuantityFeatures {
    cardinality: NounCardinality,
    standalone_number: Number,
    is_one: bool,
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

impl GrammaticalFeature for PredicateAttachmentPhase {
    const KIND: FeatureKind = FeatureKind::PredicateAttachmentPhase;
}

impl ChartFeature for PredicateAttachmentPhase {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum NominalAttachmentPhase {
    Open,
    Prepositional {
        /// The attached PP or infinitive contains a strictly nearer host for a
        /// following relative clause. The farther nominal remains a fallback
        /// when no internal attachment completes.
        nearer_relative_host: bool,
    },
    Relative,
    /// The current right edge is an object-gap relative whose predicate
    /// requires a rules object. The next complement replaces this phase; it
    /// deliberately records no historical `has_*` state.
    RulesObjectRelative,
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

impl GrammaticalFeature for NominalAttachmentPhase {
    const KIND: FeatureKind = FeatureKind::NominalAttachmentPhase;
}

impl ChartFeature for NominalAttachmentPhase {}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum NounPhraseCoordinationState {
    None,
    Binary(Conjunction),
    Oxford(Conjunction),
    Shared,
}

/// Coarse selectional class retained only while deciding whether complete
/// noun phrases can coordinate. Unknown nouns remain unconstrained; known
/// game entities do not coordinate with known qualities or measurements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum CoordinationDomain {
    Entity,
    NonEntity,
    Damage,
    Power,
    Toughness,
    PowerToughness,
    /// A partitive selection (`one of those cards`) that can host following
    /// elliptical `one` members in a flat coordination.
    SelectionHost,
    /// A bare elliptical `one` whose coordination needs a preceding semantic
    /// host rather than an unrelated destination noun.
    SelectionContinuation,
}

/// Chart identity facts restricted to inherent realization and grammatical
/// selection; exact surface witnesses live outside this bundle.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct GeneratedElementFeatures {
    fields: std::sync::Arc<[Features]>,
    present_fields: u64,
    variant: Option<usize>,
}

/// A persistent generated sequence. Every extension shares its complete
/// prefix instead of copying an ever-growing vector into another chart key.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct GeneratedSequenceFeatures {
    previous: Option<std::sync::Arc<Self>>,
    element: GeneratedElementFeatures,
    len: usize,
}

impl GeneratedSequenceFeatures {
    fn seed(element: GeneratedElementFeatures) -> Self {
        Self {
            previous: None,
            element,
            len: 1,
        }
    }

    fn extend(previous: &std::sync::Arc<Self>, element: GeneratedElementFeatures) -> Self {
        Self {
            previous: Some(std::sync::Arc::clone(previous)),
            element,
            len: previous.len + 1,
        }
    }

    fn elements(&self) -> Vec<&GeneratedElementFeatures> {
        let mut elements = Vec::with_capacity(self.len);
        let mut next = Some(self);
        while let Some(sequence) = next {
            elements.push(&sequence.element);
            next = sequence.previous.as_deref();
        }
        elements.reverse();
        elements
    }
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
        cardinality: NounCardinality,
        article: Option<IndefiniteArticle>,
        /// `this` alone rejects the attached-role participles `equipped` and
        /// `enchanted` as shared-scope nominal continuations.
        demonstrative_this: bool,
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
        /// True only for a lexically scanned past participle. The generated
        /// postpositive adjective-plus-PP construction uses this semantic bit
        /// to admit only the `blocked by ...` family.
        past_participle: bool,
        /// Whether this modifier can remain inside a nominal coordinated under
        /// the demonstrative determiner `this`.
        demonstrative_shared_determiner: bool,
    },
    Noun {
        /// Catalog identity used only to reject a coordinated type modifier
        /// that repeats its following semantic head. Ordinary lexical nouns
        /// stay packed by grammatical features.
        identity: Option<CatalogAtom>,
        coordination_domain: Option<CoordinationDomain>,
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
        head: Option<CatalogAtom>,
        coordination_domain: Option<CoordinationDomain>,
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
        /// Whether this member can be followed by another nominal under one
        /// determiner. PPs, infinitives, and complete plural relatives close
        /// that scope before a following conjunction.
        shared_determiner_open: bool,
        /// Preserved from attributive modifiers so `this equipped creature or
        /// artifact` cannot acquire a shared-determiner analysis.
        demonstrative_shared_determiner: bool,
        /// Preserves the lexical `damage` theme flag through ordinary nominal
        /// modifiers while rejecting opaque or complemented lookalikes.
        recipient_passive_theme: bool,
    },
    NounPhrase {
        agreement: Option<Agreement>,
        coordination_domain: Option<CoordinationDomain>,
        pronoun_case: Option<PronounCase>,
        adjunct: Option<BareNominalAdjunct>,
        set_exception: SetExceptionState,
        coordination: NounPhraseCoordinationState,
        /// Preserves the lexical damage-theme gate after a nominal becomes a
        /// complete noun phrase, so generated coordination cannot mix a
        /// recipient with a later damage theme inside the recipient's PP.
        recipient_passive_theme: bool,
        /// The noun phrase was completed through the selectionally constrained
        /// rules-object followup category. Generated PP projections use this
        /// to retain final-member relative attachment.
        rules_object_followup: bool,
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
        /// The missing direct object must be an object in the rules sense and
        /// therefore cannot be supplied by a mass-noun antecedent.
        object_gap_requires_rules_object: bool,
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
        /// Preserves the lexical head's object-gap host constraint through
        /// auxiliaries and predicate attachments.
        object_gap_requires_rules_object: bool,
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
        /// The noun-phrase object is coordinated under one shared determiner.
        shared_determiner: bool,
    },
    PrepositionalPhrase {
        preposition: Preposition,
        nominal_attachment: bool,
        /// The object was parsed as nominal coordination under one shared
        /// determiner, rather than as coordination of complete noun phrases.
        shared_determiner_object: bool,
        /// The object exposes a nearer nominal attachment site.
        nearer_relative_host: bool,
    },
    VerbParticle(VerbParticle),
    CoinResult(crate::syntax::CoinSide),
    RelativeClause {
        gap: GapState,
        antecedent_agreement: Option<Agreement>,
        /// For an object gap, whether the matrix verb requires a rules object
        /// and thus refuses a mass-noun antecedent.
        object_gap_requires_rules_object: bool,
        /// True only when an explicit relative currently ends at a bare
        /// lexical `be` (`that was`), where a following participle must extend
        /// the same relative rather than attach as a reduced sibling.
        bare_copular_tail: bool,
    },
    Auxiliary(AuxiliaryFeatures),
    Conjunction(Conjunction),
    Existential {
        number: Number,
    },
    Copula(CopulaAgreement),
    SubjectAuxiliary {
        subject: ContractedSubjectKey,
        agreement: Agreement,
        auxiliary: AuxiliaryFeatures,
    },
    /// A coordinated list of exception clauses gathered under a leading
    /// `except` marker. The rider carries no agreement of its own — each
    /// conjunct is an independently agreeing finite clause.
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
        noun_heads: Vec<CatalogAtom>,
    },
    GeneratedElement {
        fields: std::sync::Arc<[Features]>,
        present_fields: u64,
        variant: Option<usize>,
    },
    GeneratedSequence {
        /// Share immutable element fields and complete sequence prefixes.
        /// Struct fields retain the grammatical features consumed by the
        /// outer construction. A variant's identity plus its checked payload
        /// category is already its complete outer feature; retaining the
        /// payload's recursive feature tree would only duplicate an inner
        /// constituent that the construction never inspects.
        tail: std::sync::Arc<GeneratedSequenceFeatures>,
    },
}

impl Features {
    fn auxiliary(instance: AuxiliaryInstance) -> Self {
        Self::Auxiliary(instance.into())
    }
}

impl ChartFeatureBundle for Features {}

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

/// Reinterprets a number that filled a compound-quantity slot.
///
/// The surface `X` is the game's variable ([CR#107.3]), never the Roman
/// numeral ten. `Numeral::parse` is canonicalising (`canonical`,
/// numeral.rs:103-111: a parse succeeds only when `format` reproduces the
/// input), and `Numeral::Roman` is the only notation whose canonical
/// spelling of any value is `X`, so `(Numeral::Roman, 10)` holds **exactly
/// when** the scanned surface was `X`.
const fn quantity_value(number: NumberLiteral) -> QuantityValue {
    if matches!(number.numeral, Numeral::Roman) && number.value == 10 {
        QuantityValue::Variable
    } else {
        QuantityValue::Literal(number)
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
            quantity_value(NumberLiteral {
                value: 10,
                numeral: Numeral::Roman,
            }),
            crate::syntax::QuantityValue::Variable,
        );
        assert_eq!(
            quantity_value(NumberLiteral {
                value: 10,
                numeral: Numeral::Cardinal,
            }),
            crate::syntax::QuantityValue::Literal(crate::syntax::NumberLiteral {
                value: 10,
                numeral: Numeral::Cardinal,
            }),
        );
        assert_eq!(
            quantity_value(NumberLiteral {
                value: 3,
                numeral: Numeral::Roman,
            }),
            crate::syntax::QuantityValue::Literal(crate::syntax::NumberLiteral {
                value: 3,
                numeral: Numeral::Roman,
            }),
        );
    }
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
    GeneratedLiteral(&'static str),
    Number(NumberLiteral),
    ComparativeWord(crate::syntax::ComparativeWord),
    Determiner(Determiner),
    Noun(NounInstance),
    Adjective(Adjective),
    Adverb(Vocab),
    VerbParticle(VerbParticle),
    CoinResult(crate::syntax::CoinSide),
    Frequency(FrequencyPhrase),
    Pronoun(PronounInstance),
    Auxiliary(AuxiliaryFeatures),
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
    Conjunction(Conjunction),
    Subordinator(crate::syntax::Subordinator),
    RelativeMarker(RelativeMarker),
    Existential(ExistentialKey),
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
    auxiliary: AuxiliaryFeatures,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::EnumIter, strum::IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
enum RuleTag {
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
    NounPhraseSetExceptionBare,
    NounPhraseSetExceptionFor,
    NounPhraseNominal,
    RulesObjectNounPhrase,
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
    /// A zero-cost passive-predicate attachment for a PP whose object has one
    /// shared determiner. The ordinary PP rule retains its historical cost.
    VerbPhrasePassiveSharedDeterminerPrepositional,
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
    /// A finite clause followed by a subject-shared copula, nominal
    /// complement, and additive `in` prepositional adjunct.
    ClauseCoordinationCopularNounPrepositional,
    ClauseCoordinationCopularNounPrepositionalComma,
    ClauseCoordinationCopularNounPrepositionalAsyndetic,
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
    /// The two-member comma base of a sibling prepositional run. Requiring a
    /// pair keeps a comma out of two-member coordinations (see the rule).
    PrepositionalPhraseListPair,
    /// An asyndetic comma continuation of a sibling prepositional run (`from
    /// Vampires, from Werewolves`).
    PrepositionalPhraseListComma,
    /// Prepositional phrases coordinated as siblings, each repeating its own
    /// preposition (`from blue and from black`). Distinct from
    /// generated noun coordination inside one prepositional object, which
    /// shares one preposition across coordinated objects (`from artifacts,
    /// creatures, and enchantments`).
    PrepositionalPhraseSiblingCoordinated,
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
    /// `nominal_quantity_complement` for the P/T token.
    NominalPowerToughnessComplement,
}

pub(crate) struct EnglishGrammar<'source, 'catalogs> {
    source: &'source str,
    catalogs: &'catalogs Catalogs,
    start: Nonterminal,
    rules: Vec<Rule<Nonterminal, EnglishLexicalSlot>>,
    impls: Vec<RuleImpl>,
    rules_by_lhs: HashMap<Nonterminal, Vec<RuleId>>,
    opacity_mode: OpacityMode,
    self_reference: SelfReference,
}

impl<'source, 'catalogs> EnglishGrammar<'source, 'catalogs> {
    #[allow(
        dead_code,
        reason = "the exact-mode constructor is exercised by grammar invariant tests"
    )]
    pub(crate) fn new(
        source: &'source str,
        catalogs: &'catalogs Catalogs,
        start: Nonterminal,
    ) -> Self {
        Self::with_opacity_mode(
            source,
            catalogs,
            start,
            OpacityMode::Exact,
            SelfReference::default(),
        )
    }

    fn with_opacity_mode(
        source: &'source str,
        catalogs: &'catalogs Catalogs,
        start: Nonterminal,
        opacity_mode: OpacityMode,
        self_reference: SelfReference,
    ) -> Self {
        Self::with_opacity_mode_and_registration_order(
            source,
            catalogs,
            start,
            opacity_mode,
            self_reference,
            RegistrationOrder::Normal,
            generated::GeneratedActivation::Production,
        )
    }

    fn with_opacity_mode_and_registration_order(
        source: &'source str,
        catalogs: &'catalogs Catalogs,
        start: Nonterminal,
        opacity_mode: OpacityMode,
        self_reference: SelfReference,
        registration_order: RegistrationOrder,
        activation: generated::GeneratedActivation,
    ) -> Self {
        let mut builder = RuleBuilder::default();
        builder.add_nominal_rules();
        builder.add_clause_rules();
        // The calls below retain their feature-round grouping for authoring
        // locality. `finish` may reorder whole construction families; stable
        // production identity and declared dominance own selection semantics.
        builder.add_coordination_rules();
        // Coordination *consumers* are kept beside the list constructions they
        // read, independent of their eventual numeric lookup IDs.
        builder.add_coordination_consumer_rules();
        builder.add_possessive_modifier_rules();
        // The coin-result predicate's dot-1 gate is categorical (the pending
        // `Come` frame), rather than an ordering preference.
        builder.add_coin_result_rules();
        // This rule's dot-1 gate (`Features::Subordinator(While)`) is likewise
        // categorical.
        builder.add_while_gerund_rules();
        builder.add_reduced_recipient_passive_rules();
        // This widens `NounPhrase`, but its dot-1 host gate is categorical.
        builder.add_set_exception_rules();
        // The subject-shared copular continuation also has a categorical dot-1
        // host gate.
        builder.add_shared_copular_coordination_rules();
        // These scoped categories retain the final member of coordinated PP
        // objects; family order does not decide their selection.
        builder.add_rules_object_attachment_rules();
        if let Some(groups) = activation.groups() {
            let cats = generated::internal_categories(groups);
            generated::register_generated(&mut builder, groups, &cats)
                .expect("active generated groups must assemble");
        }
        let rule_book = builder.finish(registration_order);
        Self {
            source,
            catalogs,
            start,
            rules: rule_book.rules,
            impls: rule_book.impls,
            rules_by_lhs: rule_book.rules_by_lhs,
            opacity_mode,
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

    fn literal_token_match(
        &self,
        tokens: &[Token],
        start: usize,
        slot: EnglishLexicalSlot,
    ) -> Option<usize> {
        let [expected] = slot.literal_surfaces() else {
            return None;
        };
        self.one_token_match(tokens, start, expected)
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

    fn spelling_match(&self, tokens: &[Token], start: usize, spelling: &str) -> Option<usize> {
        let words = spelling.split_whitespace().collect::<Vec<_>>();
        self.words_match(tokens, start, &words)
    }

    fn token_range_matches_spelling(
        &self,
        tokens: &[Token],
        start: usize,
        end: usize,
        spelling: &str,
    ) -> bool {
        end > start && self.spelling_match(tokens, start, spelling) == Some(end)
    }

    fn literal_words_match(
        &self,
        tokens: &[Token],
        start: usize,
        slot: EnglishLexicalSlot,
    ) -> Option<usize> {
        self.words_match(tokens, start, slot.literal_surfaces())
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
    type SurfaceWitness = EnglishSurfaceWitness;

    fn start(&self) -> Self::Nonterminal {
        self.start
    }

    fn rules(&self) -> &[Rule<Self::Nonterminal, Self::LexicalSlot>] {
        &self.rules
    }

    fn rules_for(&self, lhs: Self::Nonterminal) -> &[RuleId] {
        self.rules_by_lhs.get(&lhs).map_or(&[], Vec::as_slice)
    }

    fn lexical_surface_witness(
        &self,
        slot: Self::LexicalSlot,
        tokens: &[Self::Token],
        start: usize,
        lexical_match: &LexicalMatch<Self::Features, Self::Meaning>,
    ) -> Self::SurfaceWitness {
        let contraction = match (&lexical_match.meaning, slot) {
            (
                MeaningKey::Auxiliary(key),
                EnglishLexicalSlot::Auxiliary | EnglishLexicalSlot::Copula,
            ) => matching_contraction(|contraction| {
                Vocabulary::new()
                    .render_auxiliary(key.with_contraction(contraction))
                    .is_some_and(|spelling| {
                        self.token_range_matches_spelling(
                            tokens,
                            start,
                            lexical_match.end,
                            spelling,
                        )
                    })
            }),
            (MeaningKey::Existential(key), EnglishLexicalSlot::Existential) => {
                matching_contraction(|contraction| {
                    ExistentialForm::new(key.verb_slot, contraction)
                        .map(ExistentialForm::spelling)
                        .is_some_and(|spelling| {
                            self.token_range_matches_spelling(
                                tokens,
                                start,
                                lexical_match.end,
                                spelling,
                            )
                        })
                })
            }
            (MeaningKey::SubjectAuxiliary(key), EnglishLexicalSlot::SubjectAuxiliary)
                if subject_auxiliary_spelling_matches(*key, |spelling| {
                    self.token_range_matches_spelling(tokens, start, lexical_match.end, spelling)
                }) =>
            {
                Some(crate::features::Contraction::Contracted)
            }
            _ => return EnglishSurfaceWitness::None,
        };
        contraction.map_or(
            EnglishSurfaceWitness::None,
            EnglishSurfaceWitness::Contraction,
        )
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
                        meaning: MeaningKey::Number(NumberLiteral {
                            value,
                            numeral: notation,
                        }),
                        local_cost: ParseCost {
                            reading_dispreference: u32::from(notation == Numeral::Ordinal),
                            precedence: u32::from(notation == Numeral::Roman && surface == "X"),
                            ..ParseCost::default()
                        },
                    })
                    .into_iter()
                    .collect()
            }
            EnglishLexicalSlot::QuantityNumber => {
                let mut matches = Vec::new();
                for (end, number) in self.recognized_numerals(tokens, start) {
                    matches.push(LexicalMatch {
                        end,
                        features: Features::Number {
                            is_one: number.value == 1,
                        },
                        meaning: MeaningKey::Number(number),
                        local_cost: ParseCost {
                            reading_dispreference: u32::from(number.numeral == Numeral::Ordinal),
                            precedence: u32::from(
                                number.numeral == Numeral::Roman && number.value == 10,
                            ),
                            ..ParseCost::default()
                        },
                    });
                }
                matches
            }
            EnglishLexicalSlot::ComparativeWord => self
                .token_text(tokens, start)
                .and_then(crate::word::comparative_word)
                .map(|word| LexicalMatch {
                    end: start + 1,
                    features: Features::None,
                    meaning: MeaningKey::ComparativeWord(word),
                    local_cost: ParseCost::default(),
                })
                .into_iter()
                .collect(),
            EnglishLexicalSlot::GeneratedLiteral(literal) => self
                .spelling_match(tokens, start, literal)
                .map(|end| LexicalMatch {
                    end,
                    features: Features::None,
                    meaning: MeaningKey::GeneratedLiteral(literal),
                    local_cost: ParseCost::default(),
                })
                .into_iter()
                .collect(),
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
            slot @ EnglishLexicalSlot::FromWord => self
                .literal_token_match(tokens, start, slot)
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
                            MeaningKey::Noun(noun)
                                if matches!(
                                    noun.kind(),
                                    crate::word::NounInstanceKind::Mass(Noun::Catalog(atom))
                                        if keyword_atom_carries_from(atom)
                                )
                        )
                    })
                    .map(|candidate| candidate.end)
                    .max();
                matches
                    .into_iter()
                    .filter(|candidate| {
                        carries_from_max_end.is_none_or(|max_end| candidate.end >= max_end)
                    })
                    .collect()
            }
            EnglishLexicalSlot::AtomCarriedPredicatedKeywordNoun => self
                .catalog_matches(tokens, start, CatalogSlot::KeywordAbilityNoun)
                .into_iter()
                .filter(|candidate| {
                    matches!(
                        &candidate.meaning,
                        MeaningKey::Noun(noun)
                            if matches!(
                                noun.kind(),
                                crate::word::NounInstanceKind::Mass(Noun::Catalog(atom))
                                    if keyword_atom_carries_from(atom)
                            )
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
                        identity: Some(atom.clone()),
                        coordination_domain: Some(CoordinationDomain::NonEntity),
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
            slot @ EnglishLexicalSlot::TimesNoun => self
                .literal_token_match(tokens, start, slot)
                .map(|end| LexicalMatch {
                    end,
                    features: Features::Noun {
                        identity: None,
                        coordination_domain: None,
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
            slot @ EnglishLexicalSlot::NumberNoun => self
                .literal_token_match(tokens, start, slot)
                .map(|end| LexicalMatch {
                    end,
                    features: Features::Noun {
                        identity: None,
                        coordination_domain: Some(CoordinationDomain::NonEntity),
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
            slot @ EnglishLexicalSlot::CombatStepDeclare => self
                .literal_token_match(tokens, start, slot)
                .map(|end| literal_match(end, LiteralKey::CombatStepDeclare))
                .into_iter()
                .collect(),
            slot @ EnglishLexicalSlot::CombatStepParticipants => {
                let matches_surface = self.token_text(tokens, start).is_some_and(|surface| {
                    slot.literal_surfaces()
                        .iter()
                        .any(|expected| surface.eq_ignore_ascii_case(expected))
                });
                if !matches_surface {
                    return Vec::new();
                }
                self.word_matches(tokens, start, LexicalSlot::Noun(NounUsage::Count))
                    .into_iter()
                    .filter(|lexical_match| {
                        matches!(
                            &lexical_match.meaning,
                            MeaningKey::Noun(noun)
                                if matches!(
                                    noun.kind(),
                                    crate::word::NounInstanceKind::Plural(_)
                                )
                        )
                    })
                    .collect()
            }
            slot @ EnglishLexicalSlot::CombatStepHead => {
                if self.literal_token_match(tokens, start, slot).is_none() {
                    return Vec::new();
                }
                self.word_matches(tokens, start, LexicalSlot::Noun(NounUsage::Count))
                    .into_iter()
                    .filter(|lexical_match| {
                        matches!(
                            &lexical_match.meaning,
                            MeaningKey::Noun(noun)
                                if matches!(
                                    noun.kind(),
                                    crate::word::NounInstanceKind::Singular(_)
                                )
                        )
                    })
                    .collect()
            }
            slot @ EnglishLexicalSlot::PreverbAdverb => {
                if self.literal_token_match(tokens, start, slot).is_none() {
                    return Vec::new();
                }
                self.word_matches(tokens, start, LexicalSlot::Adverb)
            }
            EnglishLexicalSlot::NegatedModifier => self.scan_negated_modifier(tokens, start),
            EnglishLexicalSlot::Adverb => self.word_matches(tokens, start, LexicalSlot::Adverb),
            EnglishLexicalSlot::SentenceAdverbial => {
                self.word_matches(tokens, start, LexicalSlot::SentenceAdverbial)
            }
            slot @ EnglishLexicalSlot::VerbParticle(particle) => self
                .literal_token_match(tokens, start, slot)
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
            slot @ EnglishLexicalSlot::CoinResult(side) => self
                .literal_words_match(tokens, start, slot)
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
            slot @ EnglishLexicalSlot::DeterminerTarget => self
                .literal_token_match(tokens, start, slot)
                .map(|end| LexicalMatch {
                    end,
                    features: Features::Determiner {
                        cardinality: NounCardinality::SingularCount,
                        article: None,
                        demonstrative_this: false,
                        set_exception_host: false,
                    },
                    meaning: MeaningKey::Literal(LiteralKey::Target),
                    local_cost: ParseCost::default(),
                })
                .into_iter()
                .collect(),
            slot @ EnglishLexicalSlot::Than => self
                .literal_token_match(tokens, start, slot)
                .map(|end| literal_match(end, LiteralKey::Than))
                .into_iter()
                .collect(),
            slot @ EnglishLexicalSlot::OrEqualTo => self
                .literal_words_match(tokens, start, slot)
                .map(|end| literal_match(end, LiteralKey::OrEqualTo))
                .into_iter()
                .collect(),
            slot @ EnglishLexicalSlot::RelativeMarker => slot
                .literal_surfaces()
                .iter()
                .copied()
                .zip([RelativeMarker::Who, RelativeMarker::That])
                .filter_map(|(surface, marker)| {
                    self.one_token_match(tokens, start, surface)
                        .map(|end| LexicalMatch {
                            end,
                            features: Features::None,
                            meaning: MeaningKey::RelativeMarker(marker),
                            local_cost: ParseCost::default(),
                        })
                })
                .collect(),
            slot @ EnglishLexicalSlot::Face => self
                .literal_token_match(tokens, start, slot)
                .map(|end| literal_match(end, LiteralKey::Face))
                .into_iter()
                .collect(),
            slot @ EnglishLexicalSlot::Up => self
                .literal_token_match(tokens, start, slot)
                .map(|end| literal_match(end, LiteralKey::Up))
                .into_iter()
                .collect(),
            slot @ EnglishLexicalSlot::Down => self
                .literal_token_match(tokens, start, slot)
                .map(|end| literal_match(end, LiteralKey::Down))
                .into_iter()
                .collect(),
            slot @ EnglishLexicalSlot::Not => self
                .literal_token_match(tokens, start, slot)
                .map(|end| literal_match(end, LiteralKey::Not))
                .into_iter()
                .collect(),
            slot @ EnglishLexicalSlot::Minus => self
                .literal_token_match(tokens, start, slot)
                .map(|end| literal_match(end, LiteralKey::Minus))
                .into_iter()
                .collect(),
            slot @ EnglishLexicalSlot::Half => self
                .literal_token_match(tokens, start, slot)
                .map(|end| literal_match(end, LiteralKey::Half))
                .into_iter()
                .collect(),
            slot @ EnglishLexicalSlot::Rounded => self
                .literal_token_match(tokens, start, slot)
                .map(|end| literal_match(end, LiteralKey::Rounded))
                .into_iter()
                .collect(),
            slot @ EnglishLexicalSlot::To => self
                .literal_token_match(tokens, start, slot)
                .map(|end| literal_match(end, LiteralKey::To))
                .into_iter()
                .collect(),
            slot @ EnglishLexicalSlot::Of => self
                .literal_token_match(tokens, start, slot)
                .map(|end| LexicalMatch {
                    end,
                    features: Features::Preposition(Preposition::Of),
                    meaning: MeaningKey::Preposition(Preposition::Of),
                    local_cost: ParseCost::default(),
                })
                .into_iter()
                .collect(),
            slot @ EnglishLexicalSlot::ForWord => self
                .literal_token_match(tokens, start, slot)
                .map(|end| LexicalMatch {
                    end,
                    features: Features::Preposition(Preposition::For),
                    meaning: MeaningKey::Preposition(Preposition::For),
                    local_cost: ParseCost::default(),
                })
                .into_iter()
                .collect(),
            slot @ EnglishLexicalSlot::EachDeterminer => self
                .literal_token_match(tokens, start, slot)
                .map(|end| LexicalMatch {
                    end,
                    features: Features::Determiner {
                        cardinality: NounCardinality::SingularCount,
                        article: None,
                        demonstrative_this: false,
                        set_exception_host: true,
                    },
                    meaning: MeaningKey::Determiner(Determiner::Each),
                    local_cost: ParseCost::default(),
                })
                .into_iter()
                .collect(),
            slot @ EnglishLexicalSlot::AnyDeterminer => self
                .literal_token_match(tokens, start, slot)
                .map(|end| LexicalMatch {
                    end,
                    features: Features::Determiner {
                        cardinality: NounCardinality::Unconstrained,
                        article: None,
                        demonstrative_this: false,
                        set_exception_host: false,
                    },
                    meaning: MeaningKey::Determiner(Determiner::Any),
                    local_cost: ParseCost::default(),
                })
                .into_iter()
                .collect(),
            slot @ EnglishLexicalSlot::Reciprocal => self
                .literal_words_match(tokens, start, slot)
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
            | EnglishLexicalSlot::NounPhraseConjunction
            | EnglishLexicalSlot::Plus
            | EnglishLexicalSlot::Except) => self.scan_clause_lexical(slot, tokens, start),
            EnglishLexicalSlot::OpaqueNoun if self.opacity_mode != OpacityMode::Exact => {
                let already_known = self.has_known_word(tokens, start);
                if already_known {
                    Vec::new()
                } else {
                    let mut matches = opacity::scan_opaque(self.source, tokens, start);
                    matches.retain(|candidate| {
                        !((start + 1)..candidate.end)
                            .any(|index| self.has_known_word(tokens, index))
                    });
                    matches
                }
            }
            EnglishLexicalSlot::OpaqueNoun => Vec::new(),
        }
    }

    fn reduce(
        &self,
        rule: RuleId,
        children: &[Child<'_, Self>],
    ) -> Option<Reduction<Self::Features>> {
        match self.impls.get(rule.index()).copied()? {
            RuleImpl::Handwritten(tag) => reduce(tag, children),
            RuleImpl::Generated(generated) => reduction::reduce_generated(generated, children),
            RuleImpl::GeneratedAux(generated) => {
                reduction::reduce_generated_aux(generated, children)
            }
        }
    }

    fn intermediate_cost(
        &self,
        rule: RuleId,
        completed_children: &[Self::Features],
        rule_start: usize,
        latest_child_start: usize,
        end: usize,
    ) -> ParseCost {
        match self.impls.get(rule.index()).copied() {
            Some(RuleImpl::Generated(generated)) => {
                let Some(construction) = generated.group.constructions.get(generated.construction)
                else {
                    return ParseCost::default();
                };
                let Some(construction) = generated::NominalConstruction::from_id(construction.id)
                else {
                    return ParseCost::default();
                };
                let attachment_distance =
                    u32::try_from(latest_child_start.saturating_sub(rule_start))
                        .unwrap_or(u32::MAX)
                        .max(1);
                let attachment_extent = u32::try_from(end.saturating_sub(rule_start))
                    .unwrap_or(u32::MAX)
                    .max(1);
                match construction {
                    generated::NominalConstruction::NominalRelative
                        if completed_children.len() == 2
                            && matches!(
                                completed_children.get(1),
                                Some(Features::RelativeClause {
                                    gap: GapState::Object,
                                    object_gap_requires_rules_object: true,
                                    ..
                                })
                            ) =>
                    {
                        ParseCost {
                            attachment_count: 1,
                            attachment_distance,
                            ..ParseCost::default()
                        }
                    }
                    generated::NominalConstruction::RulesObjectFollowupNominalRelative
                    | generated::NominalConstruction::RulesObjectFollowupNominalPrepositional
                        if completed_children.len() == 2 =>
                    {
                        ParseCost {
                            attachment_count: 1,
                            attachment_distance,
                            attachment_extent,
                            ..ParseCost::default()
                        }
                    }
                    _ => ParseCost::default(),
                }
            }
            Some(RuleImpl::Handwritten(_) | RuleImpl::GeneratedAux(_)) | None => {
                ParseCost::default()
            }
        }
    }

    fn accepts_prefix(
        &self,
        rule: RuleId,
        completed_children: usize,
        latest_child: &Self::Features,
    ) -> bool {
        match self.impls.get(rule.index()).copied() {
            Some(RuleImpl::Handwritten(tag)) => {
                accepts_possessive_modifier_prefix(tag, completed_children, latest_child)
                    && accepts_set_exception_prefix(tag, completed_children, latest_child)
                    && clause::accepts_predicate_prefix(tag, completed_children, latest_child)
            }
            Some(RuleImpl::Generated(generated)) => {
                let Some(construction) = generated.group.constructions.get(generated.construction)
                else {
                    return false;
                };
                generated::NominalConstruction::from_id(construction.id).is_none_or(
                    |construction| match construction {
                        generated::NominalConstruction::PredicatedArgumentFromExtend
                        | generated::NominalConstruction::PredicatedArgumentBareExtend
                            if completed_children == 2 =>
                        {
                            matches!(latest_child, Features::Conjunction(Conjunction::And))
                        }
                        generated::NominalConstruction::ReducedRecipientPassiveNominalAdjunct
                            if completed_children == 1 =>
                        {
                            matches!(
                                latest_child,
                                Features::VerbPhrase {
                                    object,
                                    frame,
                                    ..
                                } if frame.is_recipient_passive() && object.has_direct_object()
                            )
                        }
                        _ => true,
                    },
                )
            }
            Some(RuleImpl::GeneratedAux(_)) => true,
            None => false,
        }
    }

    fn state_limit(&self) -> Option<usize> {
        (self.opacity_mode == OpacityMode::OpaqueNouns).then_some(OPACITY_STATE_LIMIT)
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

/// Whether a keyword atom's own catalog surface ends in a preposition —
/// `Partner with`, `Hexproof from`, `Splice onto`. The generalization of
/// [`keyword_atom_carries_from`], and the same kind of property: read off the
/// closed catalog surface, never a keyword-name list.
///
/// Such an atom has already consumed the preposition that introduces its
/// argument, so whatever follows is that preposition's complement — a card
/// name for `Partner with`, a quality for `Hexproof from`. It is therefore
/// never a bare noun-phrase argument in the [CR#702.5a] enchant sense, even
/// when the tokens happen to parse as one.
pub(crate) fn keyword_atom_carries_preposition(atom: &CatalogAtom) -> bool {
    atom.canonical()
        .rsplit(' ')
        .next()
        .is_some_and(|last| crate::syntax::Preposition::from_spelling(last).is_some())
}

#[cfg(test)]
mod surface_witness_tests {
    use super::*;

    fn witness_for(
        source: &str,
        slot: EnglishLexicalSlot,
        meaning: MeaningKey,
        start: usize,
        end: usize,
    ) -> EnglishSurfaceWitness {
        let catalogs = Catalogs::default();
        let grammar = EnglishGrammar::new(source, &catalogs, Nonterminal::Clause);
        let tokens = lex(source).tokens;
        grammar.lexical_surface_witness(
            slot,
            &tokens,
            start,
            &LexicalMatch {
                end,
                features: Features::None,
                meaning,
                local_cost: ParseCost::default(),
            },
        )
    }

    fn can_key() -> MeaningKey {
        MeaningKey::Auxiliary(AuxiliaryFeatures {
            auxiliary: Auxiliary::Can,
            inflection: AuxiliaryInflection::Base,
        })
    }

    fn singular_existential_key() -> MeaningKey {
        MeaningKey::Existential(ExistentialKey {
            verb_slot: VerbSlot::Present {
                person: Person::Third,
                number: Number::Singular,
            },
        })
    }

    fn you_have_key() -> MeaningKey {
        MeaningKey::SubjectAuxiliary(SubjectAuxiliaryKey {
            subject: ContractedSubjectKey::Pronoun(Pronoun::You),
            auxiliary: AuxiliaryFeatures {
                auxiliary: Auxiliary::Have,
                inflection: AuxiliaryInflection::Present {
                    person: Person::Second,
                    number: Number::Singular,
                },
            },
        })
    }

    #[test]
    fn lexical_witness_validates_full_and_contracted_auxiliary_spellings() {
        assert_eq!(
            witness_for("CAN", EnglishLexicalSlot::Auxiliary, can_key(), 0, 1),
            EnglishSurfaceWitness::Contraction(crate::features::Contraction::Full),
        );
        assert_eq!(
            witness_for("CAN'T", EnglishLexicalSlot::Auxiliary, can_key(), 0, 1),
            EnglishSurfaceWitness::Contraction(crate::features::Contraction::Contracted),
        );
        assert_eq!(
            witness_for("ban", EnglishLexicalSlot::Auxiliary, can_key(), 0, 1),
            EnglishSurfaceWitness::None,
        );
    }

    #[test]
    fn lexical_witness_validates_full_and_contracted_existential_spellings() {
        assert_eq!(
            witness_for(
                "THERE IS",
                EnglishLexicalSlot::Existential,
                singular_existential_key(),
                0,
                2,
            ),
            EnglishSurfaceWitness::Contraction(crate::features::Contraction::Full),
        );
        assert_eq!(
            witness_for(
                "there  is",
                EnglishLexicalSlot::Existential,
                singular_existential_key(),
                0,
                2,
            ),
            EnglishSurfaceWitness::Contraction(crate::features::Contraction::Full),
        );
        assert_eq!(
            witness_for(
                "there\tis",
                EnglishLexicalSlot::Existential,
                singular_existential_key(),
                0,
                2,
            ),
            EnglishSurfaceWitness::Contraction(crate::features::Contraction::Full),
        );
        assert_eq!(
            witness_for(
                "THERE'S",
                EnglishLexicalSlot::Existential,
                singular_existential_key(),
                0,
                1,
            ),
            EnglishSurfaceWitness::Contraction(crate::features::Contraction::Contracted),
        );
        assert_eq!(
            witness_for(
                "there as",
                EnglishLexicalSlot::Existential,
                singular_existential_key(),
                0,
                2,
            ),
            EnglishSurfaceWitness::None,
        );
    }

    #[test]
    fn lexical_witness_validates_contracted_subject_spelling_and_key() {
        assert_eq!(
            witness_for(
                "YOU'VE",
                EnglishLexicalSlot::SubjectAuxiliary,
                you_have_key(),
                0,
                1,
            ),
            EnglishSurfaceWitness::Contraction(crate::features::Contraction::Contracted),
        );
        assert_eq!(
            witness_for(
                "you'd",
                EnglishLexicalSlot::SubjectAuxiliary,
                you_have_key(),
                0,
                1,
            ),
            EnglishSurfaceWitness::None,
        );
    }

    #[test]
    fn lexical_witness_rejects_out_of_range_and_overlong_spans() {
        assert_eq!(
            witness_for("can", EnglishLexicalSlot::Auxiliary, can_key(), 1, 2),
            EnglishSurfaceWitness::None,
        );
        assert_eq!(
            witness_for("can fly", EnglishLexicalSlot::Auxiliary, can_key(), 0, 2,),
            EnglishSurfaceWitness::None,
        );
    }
}
