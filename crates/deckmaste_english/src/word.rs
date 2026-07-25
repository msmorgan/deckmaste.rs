use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::sync::OnceLock;

use crate::catalog::CatalogAtom;
use crate::catalog::KeywordAction;
use crate::syntax::ComparativeWord;
use crate::syntax::NumberLiteral;
use crate::syntax::OpaqueLexeme;
use crate::syntax::Preposition;
use crate::syntax::VerbParticle;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Person {
    Second,
    Third,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Number {
    Singular,
    Plural,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tense {
    Present,
    Past,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Pronoun {
    You,
    It(Gender),
    They,
    EachOther,
    /// Reflexive `itself`. Object case only, like [`Self::EachOther`].
    Itself,
    /// Reflexive `himself`. Object case only, like [`Self::EachOther`].
    Himself,
    /// Absolute possessive `yours` (`turns other than yours`). Object case
    /// only, like [`Self::EachOther`].
    YoursAbsolute,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Gender {
    Masculine,
    Feminine,
    Neuter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PronounCase {
    Subject,
    Object,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PronounInstance {
    pub pronoun: Pronoun,
    pub case: PronounCase,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Noun {
    Word(Vocab),
    Catalog(CatalogAtom),
    Die(NumberLiteral),
    Gerund(Verb),
    Opaque(OpaqueLexeme),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NounInstance {
    Singular(Noun),
    Plural(Noun),
    Mass(Noun),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NounDeclension {
    Regular,
    Irregular {
        singular: &'static str,
        plural: &'static str,
    },
    Invariant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Countability {
    Count,
    Mass,
    CountOrMass,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NounDefinition {
    pub noun: Noun,
    pub declension: NounDeclension,
    pub countability: Countability,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Verb {
    Word(Vocab),
    KeywordAction(KeywordAction),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VerbSlot {
    Infinitive,
    Imperative,
    Present { person: Person, number: Number },
    Past { person: Person, number: Number },
    PresentParticiple,
    PastParticiple,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VerbInstance {
    pub verb: Verb,
    pub slot: VerbSlot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum ArgumentRequirement {
    Forbidden,
    Optional,
    Required,
}

impl ArgumentRequirement {
    pub(crate) const fn accepts(self) -> bool {
        !matches!(self, Self::Forbidden)
    }

    pub(crate) const fn is_satisfied_by(self, present: bool) -> bool {
        match self {
            Self::Forbidden => !present,
            Self::Optional => true,
            Self::Required => present,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum PredicateComplementKind {
    Adjective,
    Infinitive,
    Ability,
    Scalar,
    Statistic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum BareNominalAdjunct {
    Temporal,
    Manner,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum PrepositionalRole {
    SelectedComplement,
    Adjunct,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(
    clippy::struct_excessive_bools,
    reason = "frame model is stable and shared across many call sites"
)]
pub(crate) struct PredicateFrame {
    direct_object: ArgumentRequirement,
    indirect_object: ArgumentRequirement,
    adjective_complement: bool,
    infinitive_complement: bool,
    ability_complement: bool,
    scalar_complement: bool,
    statistic_complement: bool,
    selected_preposition: ArgumentRequirement,
    selected_prepositions: &'static [Preposition],
    prepositional_adjuncts: bool,
    bare_nominal_adjuncts: &'static [BareNominalAdjunct],
    pub(crate) particles: &'static [VerbParticle],
    proform: bool,
    /// Whether the verb takes a causative bare-infinitive complement after its
    /// object (`have <object> <bare VP>`). Set only for the causative `have`;
    /// every other frame leaves it off, so the `VerbPhrase = VerbPhrase
    /// VerbPhrase` production reduces to nothing outside this construction.
    causative_complement: bool,
    /// Whether the passive of this frame promotes the RECIPIENT (indirect
    /// object) rather than the theme, RETAINING the direct object
    /// post-verbally: `X was dealt damage`. Set only on a passive-only frame;
    /// `predicate_arguments_complete` rejects it outright in the active, so
    /// no double-object active reading is ever licensed.
    recipient_passive: bool,
}

impl PredicateFrame {
    pub(crate) const OPEN: Self = Self {
        direct_object: ArgumentRequirement::Optional,
        indirect_object: ArgumentRequirement::Forbidden,
        adjective_complement: true,
        infinitive_complement: true,
        ability_complement: true,
        scalar_complement: true,
        statistic_complement: true,
        selected_preposition: ArgumentRequirement::Forbidden,
        selected_prepositions: &[],
        prepositional_adjuncts: true,
        bare_nominal_adjuncts: &[BareNominalAdjunct::Temporal, BareNominalAdjunct::Manner],
        particles: &[],
        proform: false,
        causative_complement: false,
        recipient_passive: false,
    };

    const fn with_direct_object(mut self, requirement: ArgumentRequirement) -> Self {
        self.direct_object = requirement;
        self
    }

    const fn with_indirect_object(mut self, requirement: ArgumentRequirement) -> Self {
        self.indirect_object = requirement;
        self
    }

    const fn with_selected_prepositions(
        mut self,
        requirement: ArgumentRequirement,
        prepositions: &'static [Preposition],
    ) -> Self {
        self.selected_preposition = requirement;
        self.selected_prepositions = prepositions;
        self
    }

    const fn with_prepositional_adjuncts(mut self, allowed: bool) -> Self {
        self.prepositional_adjuncts = allowed;
        self
    }

    const fn with_particles(mut self, particles: &'static [VerbParticle]) -> Self {
        self.particles = particles;
        self
    }

    const fn with_proform(mut self) -> Self {
        self.proform = true;
        self
    }

    const fn with_causative_complement(mut self) -> Self {
        self.causative_complement = true;
        self
    }

    const fn with_recipient_passive(mut self) -> Self {
        self.recipient_passive = true;
        self
    }

    pub(crate) const fn direct_object(self) -> ArgumentRequirement {
        self.direct_object
    }

    pub(crate) const fn indirect_object(self) -> ArgumentRequirement {
        self.indirect_object
    }

    pub(crate) const fn selected_preposition(self) -> ArgumentRequirement {
        self.selected_preposition
    }

    pub(crate) const fn is_proform(self) -> bool {
        self.proform
    }

    pub(crate) const fn causative_complement(self) -> bool {
        self.causative_complement
    }

    pub(crate) const fn is_recipient_passive(self) -> bool {
        self.recipient_passive
    }

    pub(crate) const fn licenses_complement(self, kind: PredicateComplementKind) -> bool {
        match kind {
            PredicateComplementKind::Adjective => self.adjective_complement,
            PredicateComplementKind::Infinitive => self.infinitive_complement,
            PredicateComplementKind::Ability => {
                self.direct_object.accepts() && self.ability_complement
            }
            PredicateComplementKind::Scalar => {
                self.direct_object.accepts() && self.scalar_complement
            }
            PredicateComplementKind::Statistic => {
                self.direct_object.accepts() && self.statistic_complement
            }
        }
    }

    pub(crate) fn prepositional_role(self, preposition: Preposition) -> Option<PrepositionalRole> {
        if self.selected_prepositions.contains(&preposition) {
            Some(PrepositionalRole::SelectedComplement)
        } else if self.prepositional_adjuncts {
            Some(PrepositionalRole::Adjunct)
        } else {
            None
        }
    }

    pub(crate) fn licenses_bare_nominal_adjunct(self, adjunct: BareNominalAdjunct) -> bool {
        self.bare_nominal_adjuncts.contains(&adjunct)
    }

    pub(crate) fn licenses_particle(self, particle: VerbParticle) -> bool {
        self.particles.contains(&particle)
    }
}

const OPEN_PREDICATE_FRAMES: &[PredicateFrame] = &[PredicateFrame::OPEN];
const INTRANSITIVE_PREDICATE_FRAME: PredicateFrame =
    PredicateFrame::OPEN.with_direct_object(ArgumentRequirement::Forbidden);
const INTRANSITIVE_PREDICATE_FRAMES: &[PredicateFrame] = &[INTRANSITIVE_PREDICATE_FRAME];
const REQUIRED_OBJECT_PREDICATE_FRAMES: &[PredicateFrame] =
    &[PredicateFrame::OPEN.with_direct_object(ArgumentRequirement::Required)];
/// `have` as a grant/possession verb (required object) plus its causative
/// reading, which additionally takes a bare-infinitive complement after the
/// object (`have this creature enter as a copy of …`).
const HAVE_PREDICATE_FRAMES: &[PredicateFrame] = &[
    PredicateFrame::OPEN.with_direct_object(ArgumentRequirement::Required),
    PredicateFrame::OPEN
        .with_direct_object(ArgumentRequirement::Required)
        .with_causative_complement(),
];
pub(crate) const PROFORM_PREDICATE_FRAMES: &[PredicateFrame] = &[
    INTRANSITIVE_PREDICATE_FRAME.with_proform(),
    PredicateFrame::OPEN.with_proform(),
];
const ASK_PREDICATE_FRAMES: &[PredicateFrame] = &[
    PredicateFrame::OPEN.with_direct_object(ArgumentRequirement::Required),
    PredicateFrame::OPEN
        .with_direct_object(ArgumentRequirement::Required)
        .with_indirect_object(ArgumentRequirement::Required),
];
/// `deal` is ditransitive in the rules idiom: the theme is the direct object
/// and the recipient surfaces in a `to` phrase in the active voice. Its
/// passive promotes the RECIPIENT and retains the theme (`an opponent was
/// dealt damage this turn`), which no other frame in this grammar does. The
/// retained-object frame is passive-only — the canonical-template domain has
/// no double-object active (`deals target player 2 damage`) — so the active
/// `deals N damage to X` reading remains exactly `PredicateFrame::OPEN` and
/// its trees are unchanged by construction, not by cost.
const RECIPIENT_PASSIVE_PREDICATE_FRAMES: &[PredicateFrame] = &[
    PredicateFrame::OPEN,
    PredicateFrame::OPEN
        .with_direct_object(ArgumentRequirement::Required)
        .with_indirect_object(ArgumentRequirement::Required)
        .with_recipient_passive(),
];
const ATTACK_PREDICATE_FRAMES: &[PredicateFrame] =
    &[INTRANSITIVE_PREDICATE_FRAME, PredicateFrame::OPEN];
const LOOK_PREDICATE_FRAMES: &[PredicateFrame] = &[
    INTRANSITIVE_PREDICATE_FRAME.with_prepositional_adjuncts(false),
    INTRANSITIVE_PREDICATE_FRAME
        .with_selected_prepositions(ArgumentRequirement::Optional, &[Preposition::At]),
];
const PHASE_PREDICATE_FRAMES: &[PredicateFrame] =
    &[PredicateFrame::OPEN.with_particles(&[VerbParticle::In, VerbParticle::Out])];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VerbForm {
    Regular,
    Irregular(IrregularVerbDef),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IrregularVerbDef {
    pub present_second: Option<&'static str>,
    pub present_third_singular: Option<&'static str>,
    pub present_third_plural: Option<&'static str>,
    pub past_second: Option<&'static str>,
    pub past_third_singular: Option<&'static str>,
    pub past_third_plural: Option<&'static str>,
    pub present_participle: Option<&'static str>,
    pub past_participle: Option<&'static str>,
}

impl IrregularVerbDef {
    pub const EMPTY: Self = Self {
        present_second: None,
        present_third_singular: None,
        present_third_plural: None,
        past_second: None,
        past_third_singular: None,
        past_third_plural: None,
        present_participle: None,
        past_participle: None,
    };

    #[must_use]
    pub const fn with_present(
        mut self,
        second: &'static str,
        third_singular: &'static str,
        third_plural: &'static str,
    ) -> Self {
        self.present_second = Some(second);
        self.present_third_singular = Some(third_singular);
        self.present_third_plural = Some(third_plural);
        self
    }

    #[must_use]
    pub const fn with_present_third_singular(mut self, form: &'static str) -> Self {
        self.present_third_singular = Some(form);
        self
    }

    #[must_use]
    pub const fn with_past(mut self, form: &'static str) -> Self {
        self.past_second = Some(form);
        self.past_third_singular = Some(form);
        self.past_third_plural = Some(form);
        self
    }

    #[must_use]
    pub const fn with_past_agreement(
        mut self,
        second: &'static str,
        third_singular: &'static str,
        third_plural: &'static str,
    ) -> Self {
        self.past_second = Some(second);
        self.past_third_singular = Some(third_singular);
        self.past_third_plural = Some(third_plural);
        self
    }

    #[must_use]
    pub const fn with_present_participle(mut self, form: &'static str) -> Self {
        self.present_participle = Some(form);
        self
    }

    #[must_use]
    pub const fn with_past_participle(mut self, form: &'static str) -> Self {
        self.past_participle = Some(form);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VerbDefinition {
    pub verb: Verb,
    pub form: VerbForm,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorWord {
    White,
    Blue,
    Black,
    Red,
    Green,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardOrientation {
    FaceUp,
    FaceDown,
}

impl CardOrientation {
    #[must_use]
    pub const fn spelling(self) -> &'static str {
        match self {
            Self::FaceUp => "face up",
            Self::FaceDown => "face down",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Adjective {
    Word(Vocab),
    Color(ColorWord),
    CardOrientation(CardOrientation),
    Participle(Tense, Verb),
    Catalog(CatalogAtom),
    Ordinal(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InitialSound {
    Consonant,
    Vowel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Auxiliary {
    Can,
    Could,
    Do,
    May,
    Might,
    Must,
    Shall,
    Should,
    Will,
    Would,
    Be,
    Have,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AuxiliaryInflection {
    Base,
    Present {
        person: Person,
        number: Number,
    },
    Past {
        person: Person,
        number: Number,
    },
    /// The past-subjunctive `were`, carried with no person/number: licensed
    /// only under `as though` (see the `Features::Subordinator`/`subjunctive`
    /// licensing gate in `grammar`). Never agrees like `Past`.
    PastSubjunctive,
    PresentParticiple,
    PastParticiple,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AuxiliaryInstance {
    pub auxiliary: Auxiliary,
    pub inflection: AuxiliaryInflection,
    pub contracted_negation: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NounUsage {
    Count,
    Mass,
    Either,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LexicalSlot {
    Noun(NounUsage),
    Verb(VerbSlot),
    Adjective,
    Adverb,
    SentenceAdverbial,
    Pronoun(PronounCase),
    Auxiliary,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WordMatch {
    Noun(NounInstance),
    Verb(VerbInstance),
    Adjective(Adjective),
    Adverb(Vocab),
    SentenceAdverbial(Vocab),
    Pronoun(PronounInstance),
    Auxiliary(AuxiliaryInstance),
}

/// The comparison capability of an adjective, recorded as vocabulary metadata
/// rather than matched by spelling in grammar control flow. Every member takes
/// a `… than X` complement (the parser marks it comparison-pending);
/// `OrComparative` members additionally head an `N or <word>` quantity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum AdjectiveComparison {
    /// `less`/`fewer`/`greater`/`more` — heads `N or <word>` and `<word> than
    /// X`.
    OrComparative(ComparativeWord),
    /// `other` — heads `other than X` only, never a quantity bound.
    ThanOnly,
}

/// Which comparison class produced a comparison-pending adjective. Derived
/// from the per-`Vocab` comparison metadata, never from a spelling match.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum AdjectiveComparisonClass {
    OrComparative,
    ThanOnly,
}

impl AdjectiveComparison {
    pub(crate) const fn class(self) -> AdjectiveComparisonClass {
        match self {
            Self::OrComparative(_) => AdjectiveComparisonClass::OrComparative,
            Self::ThanOnly => AdjectiveComparisonClass::ThanOnly,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[allow(
    clippy::struct_excessive_bools,
    reason = "each bool is an independent, orthogonal part-of-speech/capability flag, not encodable state"
)]
struct VocabDefinition {
    spelling: &'static str,
    noun: Option<(NounDeclension, Countability)>,
    catalog_noun: bool,
    verb: Option<VerbForm>,
    predicate_frames: &'static [PredicateFrame],
    bare_nominal_adjunct: Option<BareNominalAdjunct>,
    adjective: bool,
    adverb: bool,
    sentence_adverbial: bool,
    comparison: Option<AdjectiveComparison>,
    initial_sound: Option<InitialSound>,
}

impl VocabDefinition {
    const fn new(spelling: &'static str) -> Self {
        Self {
            spelling,
            noun: None,
            catalog_noun: false,
            verb: None,
            predicate_frames: &[],
            bare_nominal_adjunct: None,
            adjective: false,
            adverb: false,
            sentence_adverbial: false,
            comparison: None,
            initial_sound: None,
        }
    }

    const fn noun(mut self, declension: NounDeclension, countability: Countability) -> Self {
        self.noun = Some((declension, countability));
        self
    }

    const fn irregular_catalog_noun(mut self, plural: &'static str) -> Self {
        self.noun = Some((
            NounDeclension::Irregular {
                singular: self.spelling,
                plural,
            },
            Countability::Count,
        ));
        self.catalog_noun = true;
        self
    }

    const fn invariant_catalog_noun(mut self) -> Self {
        self.noun = Some((NounDeclension::Invariant, Countability::Count));
        self.catalog_noun = true;
        self
    }

    const fn verb(mut self, form: VerbForm) -> Self {
        self.verb = Some(form);
        self.predicate_frames = OPEN_PREDICATE_FRAMES;
        self
    }

    const fn predicate_frames(mut self, frames: &'static [PredicateFrame]) -> Self {
        self.predicate_frames = frames;
        self
    }

    const fn bare_nominal_adjunct(mut self, adjunct: BareNominalAdjunct) -> Self {
        self.bare_nominal_adjunct = Some(adjunct);
        self
    }

    const fn adjective(mut self) -> Self {
        self.adjective = true;
        self
    }

    const fn comparison(mut self, comparison: AdjectiveComparison) -> Self {
        self.adjective = true;
        self.comparison = Some(comparison);
        self
    }

    const fn adverb(mut self) -> Self {
        self.adverb = true;
        self
    }

    /// Closed-class discourse adverbial that may front a clause before a
    /// comma; deliberately distinct from `adverb()` so it cannot fill
    /// verb-phrase or copular adverb slots.
    const fn sentence_adverbial(mut self) -> Self {
        self.sentence_adverbial = true;
        self
    }

    const fn initial_sound(mut self, sound: InitialSound) -> Self {
        self.initial_sound = Some(sound);
        self
    }

    fn merge_regular(mut self, regular: Self) -> Self {
        if self.noun.is_none() && !self.catalog_noun {
            self.noun = regular.noun;
        }
        if self.verb.is_none() {
            self.verb = regular.verb;
            self.predicate_frames = regular.predicate_frames;
        }
        self.bare_nominal_adjunct = self.bare_nominal_adjunct.or(regular.bare_nominal_adjunct);
        self.adjective |= regular.adjective;
        self.adverb |= regular.adverb;
        self
    }

    fn add_regular_part_of_speech(&mut self, part_of_speech: &str) {
        match part_of_speech {
            "noun_count" => self.add_regular_noun(Countability::Count),
            "noun_mass" => self.add_regular_noun(Countability::Mass),
            "noun_count_or_mass" => self.add_regular_noun(Countability::CountOrMass),
            "verb" => {
                self.verb = Some(VerbForm::Regular);
                self.predicate_frames = OPEN_PREDICATE_FRAMES;
            }
            "adjective" => self.adjective = true,
            "adverb" => self.adverb = true,
            _ => panic!("unknown regular-vocabulary part of speech: {part_of_speech:?}"),
        }
    }

    fn add_regular_noun(&mut self, countability: Countability) {
        let countability = match self.noun {
            None => countability,
            Some((NounDeclension::Regular, existing)) => match (existing, countability) {
                (Countability::Count, Countability::Mass)
                | (Countability::Mass, Countability::Count) => Countability::CountOrMass,
                (Countability::CountOrMass, _) | (_, Countability::CountOrMass) => {
                    Countability::CountOrMass
                }
                (Countability::Count, Countability::Count)
                | (Countability::Mass, Countability::Mass) => existing,
            },
            Some((declension, _)) => {
                panic!("regular vocabulary unexpectedly used {declension:?}")
            }
        };
        self.noun = Some((NounDeclension::Regular, countability));
    }
}

const REGULAR_VOCABULARY_TSV: &str = include_str!("regular-vocabulary.tsv");

fn regular_vocabulary() -> &'static [VocabDefinition] {
    static VOCABULARY: OnceLock<Vec<VocabDefinition>> = OnceLock::new();
    VOCABULARY.get_or_init(|| {
        let mut definitions: Vec<VocabDefinition> = Vec::new();
        let mut previous = None;

        for line in REGULAR_VOCABULARY_TSV.lines() {
            if let Some(previous) = previous {
                assert!(
                    previous < line,
                    "regular-vocabulary table must be sorted and deduplicated"
                );
            }
            previous = Some(line);

            let (lemma, part_of_speech) = line
                .split_once('\t')
                .unwrap_or_else(|| panic!("malformed regular-vocabulary row: {line:?}"));
            assert!(
                !lemma.is_empty()
                    && lemma
                        .bytes()
                        .all(|byte| { byte.is_ascii_lowercase() || matches!(byte, b'-' | b'\'') }),
                "invalid regular-vocabulary lemma: {lemma:?}"
            );

            if definitions
                .last()
                .is_none_or(|entry| entry.spelling != lemma)
            {
                definitions.push(VocabDefinition::new(lemma));
            }
            definitions
                .last_mut()
                .expect("a regular-vocabulary definition was just inserted")
                .add_regular_part_of_speech(part_of_speech);
        }

        definitions
    })
}

fn regular_definition(spelling: &str) -> Option<VocabDefinition> {
    regular_vocabulary()
        .binary_search_by_key(&spelling, |definition| definition.spelling)
        .ok()
        .map(|index| regular_vocabulary()[index])
}

/// Surface → comparative-word index, derived from the per-[`Vocab`] comparison
/// metadata so the fact lives in one place. Only `OrComparative` adjectives
/// (`less`/`fewer`/`greater`/`more`) can head an `N or <word>` quantity;
/// `other` (`ThanOnly`) is deliberately absent.
fn comparison_lexicon() -> &'static HashMap<&'static str, ComparativeWord> {
    static LEXICON: OnceLock<HashMap<&'static str, ComparativeWord>> = OnceLock::new();
    LEXICON.get_or_init(|| {
        Vocab::ALL
            .iter()
            .filter_map(|&vocab| match vocab.comparison() {
                Some(AdjectiveComparison::OrComparative(word)) => Some((vocab.spelling(), word)),
                Some(AdjectiveComparison::ThanOnly) | None => None,
            })
            .collect()
    })
}

/// The comparative word a surface form names when it heads an `N or <word>`
/// quantity, or `None` if the form is not a quantity-heading comparative.
pub(crate) fn comparative_word(surface: &str) -> Option<ComparativeWord> {
    comparison_lexicon().get(surface).copied()
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct RegularVocab(&'static str);

impl fmt::Debug for RegularVocab {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

macro_rules! vocabulary {
    ($(
        $variant:ident($spelling:literal)
        $(.$method:ident($($argument:expr),* $(,)?))*;
    )+) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum Vocab {
            $($variant,)+
            Regular(RegularVocab),
        }

        impl Vocab {
            pub const ALL: &'static [Self] = &[$(Self::$variant,)+];

            #[must_use]
            pub const fn spelling(self) -> &'static str {
                match self {
                    $(Self::$variant => $spelling,)+
                    Self::Regular(regular) => regular.0,
                }
            }

            pub(crate) fn predicate_frames(self) -> &'static [PredicateFrame] {
                self.definition().predicate_frames
            }

            pub(crate) fn bare_nominal_adjunct(self) -> Option<BareNominalAdjunct> {
                self.definition().bare_nominal_adjunct
            }

            pub(crate) fn comparison(self) -> Option<AdjectiveComparison> {
                self.definition().comparison
            }

            #[allow(
                dead_code,
                reason = "mirrors comparison()/adverb() accessor precedent; unused until a semantic-IR consumer (ticket english-semantic-ir) reads it"
            )]
            pub(crate) fn sentence_adverbial(self) -> bool {
                self.definition().sentence_adverbial
            }

            fn definition(self) -> VocabDefinition {
                let definition = match self {
                    $(Self::$variant => VocabDefinition::new($spelling)
                        $(.$method($($argument),*))*,)+
                    Self::Regular(regular) => return regular_definition(regular.0)
                        .expect("regular Vocab identity must come from the checked-in table"),
                };
                regular_definition(definition.spelling)
                    .map_or(definition, |regular| definition.merge_regular(regular))
            }
        }
    };
}

vocabulary! {
    Abandon("abandon").verb(VerbForm::Regular);
    Ability("ability").noun(NounDeclension::Regular, Countability::Count);
    Able("able").adjective();
    Activate("activate").verb(VerbForm::Regular);
    Adapt("adapt").verb(VerbForm::Regular);
    Add("add").verb(VerbForm::Regular);
    Additional("additional").adjective();
    Aetherborn("Aetherborn").invariant_catalog_noun();
    Affect("affect")
        .verb(VerbForm::Regular)
        .predicate_frames(REQUIRED_OBJECT_PREDICATE_FRAMES);
    Again("again").adverb();
    Airbend("airbend").verb(VerbForm::Regular);
    Alone("alone").adverb();
    Amass("amass").verb(VerbForm::Regular);
    Apply("apply").verb(VerbForm::Regular);
    Ask("ask")
        .verb(VerbForm::Regular)
        .predicate_frames(ASK_PREDICATE_FRAMES);
    Assemble("assemble").verb(VerbForm::Regular);
    Astartes("Astartes").invariant_catalog_noun();
    Attach("attach").verb(VerbForm::Regular);
    Attack("attack")
        .verb(VerbForm::Regular)
        .predicate_frames(ATTACK_PREDICATE_FRAMES);
    Aurochs("Aurochs").invariant_catalog_noun();
    Bargain("bargain").verb(VerbForm::Regular);
    Battlefield("battlefield").noun(NounDeclension::Regular, Countability::Count);
    Be("be").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_present("are", "is", "are")
            .with_past_agreement("were", "was", "were")
            .with_present_participle("being")
            .with_past_participle("been")
    )).predicate_frames(INTRANSITIVE_PREDICATE_FRAMES);
    Become("become").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("became")
            .with_past_participle("become")
    ));
    Begin("begin").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("began")
            .with_present_participle("beginning")
            .with_past_participle("begun")
    ));
    Behold("behold").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("beheld")
            .with_past_participle("beheld")
    ));
    Bid("bid")
        .verb(VerbForm::Irregular(
            IrregularVerbDef::EMPTY
                .with_past("bid")
                .with_present_participle("bidding")
                .with_past_participle("bid")
        ))
        .noun(NounDeclension::Regular, Countability::Count);
    Bison("Bison").invariant_catalog_noun();
    Blight("blight").verb(VerbForm::Regular);
    Block("block")
        .verb(VerbForm::Regular)
        .predicate_frames(ATTACK_PREDICATE_FRAMES);
    Bolster("bolster").verb(VerbForm::Regular);
    Break("break").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("broke")
            .with_past_participle("broken")
    ));
    Card("card").noun(NounDeclension::Regular, Countability::Count);
    Cast("cast").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("cast")
            .with_past_participle("cast")
    ));
    Cause("cause").verb(VerbForm::Regular);
    Change("change").verb(VerbForm::Regular);
    Chaos("chaos").noun(NounDeclension::Regular, Countability::Mass);
    Child("Child").irregular_catalog_noun("Children");
    Choose("choose").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("chose")
            .with_past_participle("chosen")
    ));
    Clash("clash").verb(VerbForm::Regular);
    Cloak("cloak").verb(VerbForm::Regular);
    Coin("coin").noun(NounDeclension::Regular, Countability::Count);
    Collect("collect").verb(VerbForm::Regular);
    Color("color").noun(NounDeclension::Regular, Countability::Count);
    Colorless("colorless").adjective();
    Commander("commander").noun(NounDeclension::Regular, Countability::Count);
    Combat("combat")
        .noun(NounDeclension::Regular, Countability::CountOrMass)
        .bare_nominal_adjunct(BareNominalAdjunct::Temporal);
    Conjure("conjure").verb(VerbForm::Regular);
    Connive("connive").verb(VerbForm::Regular);
    Control("control").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("controlled")
            .with_present_participle("controlling")
            .with_past_participle("controlled")
    )).noun(NounDeclension::Regular, Countability::Mass);
    Controller("controller").noun(NounDeclension::Regular, Countability::Count);
    Convert("convert").verb(VerbForm::Regular);
    Copy("copy")
        .noun(NounDeclension::Regular, Countability::Count)
        .verb(VerbForm::Regular);
    Cost("cost")
        .noun(NounDeclension::Regular, Countability::Count)
        .verb(VerbForm::Irregular(
            IrregularVerbDef::EMPTY
                .with_past("cost")
                .with_past_participle("cost")
        ));
    Count("count").verb(VerbForm::Regular);
    Counter("counter")
        .noun(NounDeclension::Regular, Countability::Count)
        .verb(VerbForm::Regular);
    Create("create").verb(VerbForm::Regular);
    Custodes("Custodes").invariant_catalog_noun();
    Cyberman("Cyberman").irregular_catalog_noun("Cybermen");
    Damage("damage").noun(NounDeclension::Regular, Countability::Mass);
    Day("day").noun(NounDeclension::Regular, Countability::Count);
    Deal("deal").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("dealt")
            .with_past_participle("dealt")
    )).predicate_frames(RECIPIENT_PASSIVE_PREDICATE_FRAMES);
    Deck("deck").noun(NounDeclension::Regular, Countability::Count);
    Defend("defend").verb(VerbForm::Regular);
    Destroy("destroy").verb(VerbForm::Regular);
    Detain("detain").verb(VerbForm::Regular);
    Die("die")
        .noun(
            NounDeclension::Irregular {
                singular: "die",
                plural: "dice",
            },
            Countability::Count,
        )
        .verb(VerbForm::Irregular(
            IrregularVerbDef::EMPTY.with_present_participle("dying")
        ));
    Discard("discard").verb(VerbForm::Regular);
    Discover("discover").verb(VerbForm::Regular);
    Double("double").verb(VerbForm::Regular);
    Do("do").verb(VerbForm::Irregular(
            IrregularVerbDef::EMPTY
                .with_present_third_singular("does")
                .with_past("did")
                .with_present_participle("doing")
                .with_past_participle("done")
        )).predicate_frames(PROFORM_PREDICATE_FRAMES);
    Draft("draft").verb(VerbForm::Regular);
    Draw("draw")
        .verb(VerbForm::Irregular(
            IrregularVerbDef::EMPTY
                .with_past("drew")
                .with_past_participle("drawn")
        ))
        .noun(NounDeclension::Regular, Countability::Count);
    Drix("Drix").invariant_catalog_noun();
    Dwarf("Dwarf").irregular_catalog_noun("Dwarves");
    Earthbend("earthbend").verb(VerbForm::Regular);
    Effect("effect").noun(NounDeclension::Regular, Countability::Count);
    Elf("Elf").irregular_catalog_noun("Elves");
    Elk("Elk").invariant_catalog_noun();
    End("end").noun(NounDeclension::Regular, Countability::Count);
    Equal("equal").verb(VerbForm::Regular).adjective();
    Enchant("enchant").verb(VerbForm::Regular);
    Endure("endure").verb(VerbForm::Regular);
    Enter("enter").verb(VerbForm::Regular);
    Equip("equip").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("equipped")
            .with_present_participle("equipping")
            .with_past_participle("equipped")
    ));
    Even("even").adjective();
    Evidence("evidence").noun(NounDeclension::Regular, Countability::Mass);
    Exchange("exchange").verb(VerbForm::Regular);
    Exert("exert").verb(VerbForm::Regular);
    Exile("exile")
        .noun(NounDeclension::Regular, Countability::Mass)
        .verb(VerbForm::Regular);
    Explore("explore").verb(VerbForm::Regular);
    Fateseal("fateseal").verb(VerbForm::Regular);
    Fewer("fewer").comparison(AdjectiveComparison::OrComparative(ComparativeWord::Fewer));
    Fewest("fewest").adjective().noun(NounDeclension::Regular, Countability::CountOrMass);
    Fight("fight").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("fought")
            .with_past_participle("fought")
    ));
    Fish("Fish").invariant_catalog_noun();
    Flip("flip").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("flipped")
            .with_present_participle("flipping")
            .with_past_participle("flipped")
    ));
    Foe("foe").noun(NounDeclension::Regular, Countability::Count);
    Forage("forage").verb(VerbForm::Regular);
    Foretell("foretell").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("foretold")
            .with_past_participle("foretold")
    ));
    Friend("friend").noun(NounDeclension::Regular, Countability::Count);
    Fungus("Fungus").irregular_catalog_noun("Fungi");
    Gain("gain").verb(VerbForm::Regular);
    Game("game").noun(NounDeclension::Regular, Countability::Count);
    Get("get").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("got")
            .with_present_participle("getting")
            .with_past_participle("gotten")
    ));
    Go("go").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("went")
            .with_past_participle("gone")
    ));
    Goad("goad").verb(VerbForm::Regular);
    Graveborn("Graveborn").invariant_catalog_noun();
    Graveyard("graveyard").noun(NounDeclension::Regular, Countability::Count);
    Greater("greater").comparison(AdjectiveComparison::OrComparative(ComparativeWord::Greater));
    Greatest("greatest").adjective().noun(NounDeclension::Regular, Countability::CountOrMass);
    Hand("hand").noun(NounDeclension::Regular, Countability::Count);
    Harness("harness").verb(VerbForm::Regular);
    Have("have").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_present_third_singular("has")
            .with_past("had")
            .with_present_participle("having")
            .with_past_participle("had")
    )).predicate_frames(HAVE_PREDICATE_FRAMES);
    Heal("heal").verb(VerbForm::Regular);
    Heist("heist").verb(VerbForm::Regular);
    Hero("Hero").irregular_catalog_noun("Heroes");
    Highest("highest").adjective().noun(NounDeclension::Regular, Countability::CountOrMass);
    Hour("hour")
        .noun(NounDeclension::Regular, Countability::Count)
        .initial_sound(InitialSound::Vowel);
    Incorporate("incorporate").verb(VerbForm::Regular);
    Incubate("incubate").verb(VerbForm::Regular);
    Initiative("initiative").noun(NounDeclension::Regular, Countability::Count);
    Instead("instead").adverb();
    Investigate("investigate").verb(VerbForm::Regular);
    Jellyfish("Jellyfish").invariant_catalog_noun();
    Kick("kick").verb(VerbForm::Regular);
    Kithkin("Kithkin").invariant_catalog_noun();
    Kor("Kor").invariant_catalog_noun();
    Learn("learn").verb(VerbForm::Regular);
    Least("least").adjective().noun(NounDeclension::Regular, Countability::CountOrMass);
    Leave("leave").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("left")
            .with_past_participle("left")
    ));
    Less("less").adverb().comparison(AdjectiveComparison::OrComparative(ComparativeWord::Less));
    Library("library").noun(NounDeclension::Regular, Countability::Count);
    Life("life").noun(NounDeclension::Regular, Countability::Mass);
    Look("look")
        .verb(VerbForm::Regular)
        .predicate_frames(LOOK_PREDICATE_FRAMES);
    Lose("lose").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("lost")
            .with_past_participle("lost")
    ));
    Lowest("lowest").adjective().noun(NounDeclension::Regular, Countability::CountOrMass);
    Mana("mana").noun(NounDeclension::Regular, Countability::Mass);
    Meld("meld").verb(VerbForm::Regular);
    Merfolk("Merfolk").invariant_catalog_noun();
    Mill("mill").verb(VerbForm::Regular);
    Mode("mode").noun(NounDeclension::Regular, Countability::Count);
    Monarch("monarch").noun(NounDeclension::Regular, Countability::Count);
    Monocolored("monocolored").adjective();
    More("more").comparison(AdjectiveComparison::OrComparative(ComparativeWord::More));
    Monstrosity("monstrosity").noun(NounDeclension::Regular, Countability::Count);
    Monstrous("monstrous").adjective();
    Moonfolk("Moonfolk").invariant_catalog_noun();
    Most("most").adjective().noun(NounDeclension::Regular, Countability::CountOrMass);
    Mouse("Mouse").irregular_catalog_noun("Mice");
    Move("move").verb(VerbForm::Regular);
    Name("name").verb(VerbForm::Regular);
    Myr("Myr").invariant_catalog_noun();
    Nearest("nearest").adjective().noun(NounDeclension::Regular, Countability::CountOrMass);
    Night("night").noun(NounDeclension::Regular, Countability::Count);
    Number("number").noun(NounDeclension::Regular, Countability::Count);
    Odd("odd").adjective();
    One("one").noun(NounDeclension::Regular, Countability::Count);
    Open("open").verb(VerbForm::Regular);
    Opponent("opponent").noun(NounDeclension::Regular, Countability::Count);
    Only("only").adverb();
    Other("other")
        .comparison(AdjectiveComparison::ThanOnly)
        .noun(NounDeclension::Regular, Countability::Count);
    Otherwise("otherwise").sentence_adverbial();
    Own("own").verb(VerbForm::Regular);
    Owner("owner").noun(NounDeclension::Regular, Countability::Count);
    Ox("Ox").irregular_catalog_noun("Oxen");
    Pass("pass").verb(VerbForm::Regular);
    Pay("pay").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("paid")
            .with_past_participle("paid")
    ));
    Pegasus("Pegasus").irregular_catalog_noun("Pegasi");
    Permanent("permanent").noun(NounDeclension::Regular, Countability::Count);
    Phase("phase")
        .verb(VerbForm::Regular)
        .predicate_frames(PHASE_PREDICATE_FRAMES);
    Pile("pile").noun(NounDeclension::Regular, Countability::Count);
    Planeswalk("planeswalk").verb(VerbForm::Regular);
    Play("play").verb(VerbForm::Regular);
    Player("player").noun(NounDeclension::Regular, Countability::Count);
    Plot("plot").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("plotted")
            .with_present_participle("plotting")
            .with_past_participle("plotted")
    ));
    Poison("poison")
        .noun(NounDeclension::Regular, Countability::Count)
        .verb(VerbForm::Regular);
    Populate("populate").verb(VerbForm::Regular);
    Power("power").noun(NounDeclension::Regular, Countability::CountOrMass);
    Prevent("prevent").verb(VerbForm::Regular);
    Process("process").noun(NounDeclension::Regular, Countability::Count);
    Produce("produce").verb(VerbForm::Regular);
    Proliferate("proliferate").verb(VerbForm::Regular);
    Promise("promise").verb(VerbForm::Regular);
    Put("put").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("put")
            .with_present_participle("putting")
            .with_past_participle("put")
    ));
    Reduce("reduce").verb(VerbForm::Regular);
    Regenerate("regenerate").verb(VerbForm::Regular);
    Remove("remove").verb(VerbForm::Regular);
    Renowned("renowned").adjective();
    Repeat("repeat").verb(VerbForm::Regular);
    Reselect("reselect").verb(VerbForm::Regular);
    Rest("rest").noun(NounDeclension::Regular, Countability::Mass);
    Result("result").noun(NounDeclension::Regular, Countability::Count);
    Return("return").verb(VerbForm::Regular);
    Reveal("reveal").verb(VerbForm::Regular);
    RingBearer("Ring-bearer").noun(NounDeclension::Regular, Countability::Count);
    Roll("roll")
        .noun(NounDeclension::Regular, Countability::Count)
        .verb(VerbForm::Regular);
    Sacrifice("sacrifice").verb(VerbForm::Regular);
    Saddle("saddle").verb(VerbForm::Regular);
    Same("same").adjective();
    Samurai("Samurai").invariant_catalog_noun();
    Scry("scry").verb(VerbForm::Regular);
    Search("search").verb(VerbForm::Regular);
    Seek("seek").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("sought")
            .with_past_participle("sought")
    ));
    Set("set").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("set")
            .with_present_participle("setting")
            .with_past_participle("set")
    ));
    Shuffle("shuffle").verb(VerbForm::Regular);
    Skip("skip").verb(VerbForm::Regular);
    So("so").adverb();
    Source("source").noun(NounDeclension::Regular, Countability::Count);
    Spend("spend").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("spent")
            .with_past_participle("spent")
    ));
    Spell("spell").noun(NounDeclension::Regular, Countability::Count);
    Squid("Squid").invariant_catalog_noun();
    Starfish("Starfish").invariant_catalog_noun();
    Stand("stand").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("stood")
            .with_past_participle("stood")
    ));
    Support("support")
        .noun(NounDeclension::Regular, Countability::Mass)
        .verb(VerbForm::Regular);
    Surveil("surveil").verb(VerbForm::Regular);
    Suspend("suspend").verb(VerbForm::Regular);
    Suspect("suspect").verb(VerbForm::Regular);
    Tap("tap").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("tapped")
            .with_present_participle("tapping")
            .with_past_participle("tapped")
    ));
    Take("take").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("took")
            .with_past_participle("taken")
    ));
    Target("target")
        .noun(NounDeclension::Regular, Countability::Count)
        .verb(VerbForm::Regular)
        .adjective();
    Team("team").noun(NounDeclension::Regular, Countability::Count);
    Thalakos("Thalakos").invariant_catalog_noun();
    Then("then").adverb();
    Step("step")
        .noun(NounDeclension::Regular, Countability::Count)
        .verb(VerbForm::Regular)
        .bare_nominal_adjunct(BareNominalAdjunct::Temporal);
    Time("time")
        .noun(NounDeclension::Regular, Countability::CountOrMass)
        .verb(VerbForm::Regular)
        .bare_nominal_adjunct(BareNominalAdjunct::Temporal);
    Token("token").noun(NounDeclension::Regular, Countability::Count);
    Top("top").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("topped")
            .with_present_participle("topping")
            .with_past_participle("topped")
    ));
    Toughness("toughness").noun(NounDeclension::Regular, Countability::Mass);
    Transform("transform").verb(VerbForm::Regular);
    Treefolk("Treefolk").invariant_catalog_noun();
    Triple("triple").verb(VerbForm::Regular);
    Turn("turn")
        .noun(NounDeclension::Regular, Countability::Count)
        .verb(VerbForm::Regular)
        .bare_nominal_adjunct(BareNominalAdjunct::Temporal);
    Twice("twice").adverb();
    Type("type").noun(NounDeclension::Regular, Countability::Count);
    Value("value").noun(NounDeclension::Regular, Countability::Count);
    Untap("untap").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("untapped")
            .with_present_participle("untapping")
            .with_past_participle("untapped")
    ));
    Vedalken("Vedalken").invariant_catalog_noun();
    Venture("venture").verb(VerbForm::Regular);
    Vote("vote").verb(VerbForm::Regular);
    Waterbend("waterbend").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("waterbent")
            .with_past_participle("waterbent")
    ));
    Way("way")
        .noun(NounDeclension::Regular, Countability::Count)
        .bare_nominal_adjunct(BareNominalAdjunct::Manner);
    Werewolf("Werewolf").irregular_catalog_noun("Werewolves");
    Win("win").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("won")
            .with_present_participle("winning")
            .with_past_participle("won")
    ));
    Wolf("Wolf").irregular_catalog_noun("Wolves");
    Yell("yell").verb(VerbForm::Regular);
    Zubera("Zubera").invariant_catalog_noun();
}

impl Noun {
    pub(crate) fn bare_nominal_adjunct(&self) -> Option<BareNominalAdjunct> {
        match self {
            Self::Word(vocab) => vocab.bare_nominal_adjunct(),
            Self::Catalog(_) | Self::Die(_) | Self::Gerund(_) | Self::Opaque(_) => None,
        }
    }
}

impl Verb {
    pub(crate) fn predicate_frames(&self) -> &'static [PredicateFrame] {
        match self {
            Self::Word(vocab) => vocab.predicate_frames(),
            Self::KeywordAction(_) => OPEN_PREDICATE_FRAMES,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Vocabulary;

impl Vocabulary {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    #[must_use]
    pub fn matches(self, surface: &str, slot: LexicalSlot) -> Vec<WordMatch> {
        let key = if surface.bytes().any(|byte| byte.is_ascii_uppercase()) {
            Cow::Owned(surface.to_ascii_lowercase())
        } else {
            Cow::Borrowed(surface)
        };
        reverse_index()
            .get(key.as_ref())
            .into_iter()
            .flatten()
            .filter_map(|candidate| candidate.for_slot(slot))
            .collect()
    }

    #[must_use]
    pub fn noun_definition(self, vocab: Vocab) -> Option<NounDefinition> {
        let definition = vocab.definition();
        definition
            .noun
            .map(|(declension, countability)| NounDefinition {
                noun: Noun::Word(vocab),
                declension,
                countability,
            })
    }

    #[must_use]
    pub fn verb_definition(self, vocab: Vocab) -> Option<VerbDefinition> {
        vocab.definition().verb.map(|form| VerbDefinition {
            verb: Verb::Word(vocab),
            form,
        })
    }

    #[must_use]
    pub fn initial_sound(self, vocab: Vocab) -> InitialSound {
        let definition = vocab.definition();
        definition.initial_sound.unwrap_or_else(|| {
            if definition.spelling.starts_with(['a', 'e', 'i', 'o', 'u']) {
                InitialSound::Vowel
            } else {
                InitialSound::Consonant
            }
        })
    }

    pub(crate) fn exceptional_catalog_noun(canonical: &str) -> Option<Vocab> {
        Vocab::ALL.iter().copied().find(|vocab| {
            let definition = vocab.definition();
            definition.catalog_noun && definition.spelling.eq_ignore_ascii_case(canonical)
        })
    }

    pub(crate) fn irregular_keyword_action_head(surface: &str) -> Option<Vocab> {
        Vocab::ALL.iter().copied().find(|vocab| {
            matches!(vocab.definition().verb, Some(VerbForm::Irregular(_)))
                && vocab.spelling().eq_ignore_ascii_case(surface)
        })
    }

    pub(crate) fn render_regular_verb(lemma: &str, slot: VerbSlot) -> String {
        render_verb_form(lemma, VerbForm::Regular, slot)
    }

    #[must_use]
    pub fn render_noun(self, noun: &NounInstance) -> Option<String> {
        let (noun, form) = match noun {
            NounInstance::Singular(noun) => (noun, NounSurface::Singular),
            NounInstance::Plural(noun) => (noun, NounSurface::Plural),
            NounInstance::Mass(noun) => (noun, NounSurface::Mass),
        };

        match noun {
            Noun::Word(vocab) => Self::render_vocab_noun(*vocab, form),
            Noun::Catalog(atom) => match form {
                NounSurface::Singular | NounSurface::Mass => Some(atom.render_noun(false)),
                NounSurface::Plural => Some(atom.render_noun(true)),
            },
            Noun::Die(number) => match form {
                NounSurface::Singular => Some(format!("d{}", number.numeral.format(number.value))),
                NounSurface::Plural | NounSurface::Mass => None,
            },
            Noun::Gerund(verb) => {
                let present_participle =
                    self.render_verb_identity(verb, VerbSlot::PresentParticiple)?;
                Some(match form {
                    NounSurface::Plural => regular_plural(&present_participle),
                    NounSurface::Singular | NounSurface::Mass => present_participle,
                })
            }
            Noun::Opaque(opaque) => Some(opaque.spelling().to_owned()),
        }
    }

    #[must_use]
    pub fn render_verb(self, vocab: Vocab, slot: VerbSlot) -> Option<String> {
        let definition = vocab.definition();
        let form = definition.verb?;
        Some(render_verb_form(definition.spelling, form, slot))
    }

    #[must_use]
    pub fn render_verb_instance(self, verb: &VerbInstance) -> Option<String> {
        self.render_verb_identity(&verb.verb, verb.slot)
    }

    #[must_use]
    pub fn render_adjective(self, adjective: &Adjective) -> Option<String> {
        match adjective {
            Adjective::Word(vocab) => vocab
                .definition()
                .adjective
                .then(|| vocab.spelling().to_owned()),
            Adjective::Color(color) => Some(color.spelling().to_owned()),
            Adjective::CardOrientation(orientation) => Some(orientation.spelling().to_owned()),
            Adjective::Participle(Tense::Present, verb) => {
                self.render_verb_identity(verb, VerbSlot::PresentParticiple)
            }
            Adjective::Participle(Tense::Past, verb) => {
                self.render_verb_identity(verb, VerbSlot::PastParticiple)
            }
            Adjective::Catalog(atom) => Some(atom.render_adjective()),
            Adjective::Ordinal(value) => Some(crate::numeral::Numeral::Ordinal.format(*value)),
        }
    }

    #[must_use]
    pub const fn render_auxiliary(self, auxiliary: AuxiliaryInstance) -> Option<&'static str> {
        render_auxiliary(auxiliary)
    }

    #[must_use]
    pub const fn render_pronoun(self, pronoun: PronounInstance) -> Option<&'static str> {
        match (pronoun.pronoun, pronoun.case) {
            (Pronoun::You, PronounCase::Subject | PronounCase::Object) => Some("you"),
            (Pronoun::It(Gender::Masculine), PronounCase::Subject) => Some("he"),
            (Pronoun::It(Gender::Masculine), PronounCase::Object) => Some("him"),
            (Pronoun::It(Gender::Feminine), PronounCase::Subject) => Some("she"),
            (Pronoun::It(Gender::Feminine), PronounCase::Object) => Some("her"),
            (Pronoun::It(Gender::Neuter), PronounCase::Subject | PronounCase::Object) => Some("it"),
            (Pronoun::They, PronounCase::Subject) => Some("they"),
            (Pronoun::They, PronounCase::Object) => Some("them"),
            (Pronoun::EachOther, PronounCase::Object) => Some("each other"),
            (Pronoun::Itself, PronounCase::Object) => Some("itself"),
            (Pronoun::Himself, PronounCase::Object) => Some("himself"),
            (Pronoun::YoursAbsolute, PronounCase::Object) => Some("yours"),
            (
                Pronoun::EachOther | Pronoun::Itself | Pronoun::Himself | Pronoun::YoursAbsolute,
                PronounCase::Subject,
            ) => None,
        }
    }

    #[must_use]
    pub const fn render_possessive_pronoun(self, pronoun: Pronoun) -> Option<&'static str> {
        match pronoun {
            Pronoun::You => Some("your"),
            Pronoun::It(Gender::Masculine) => Some("his"),
            Pronoun::It(Gender::Feminine) => Some("her"),
            Pronoun::It(Gender::Neuter) => Some("its"),
            Pronoun::They => Some("their"),
            Pronoun::EachOther | Pronoun::Itself | Pronoun::Himself | Pronoun::YoursAbsolute => {
                None
            }
        }
    }

    fn render_vocab_noun(vocab: Vocab, form: NounSurface) -> Option<String> {
        let definition = vocab.definition();
        let (declension, countability) = definition.noun?;
        if !countability.accepts(form) {
            return None;
        }
        Some(render_noun_form(definition.spelling, declension, form))
    }

    fn render_verb_identity(self, verb: &Verb, slot: VerbSlot) -> Option<String> {
        match verb {
            Verb::Word(vocab) => self.render_verb(*vocab, slot),
            Verb::KeywordAction(action) => action.render(slot),
        }
    }
}

impl ColorWord {
    /// The five colors, in the fixed order rules text lists them.
    pub const ALL: [Self; 5] = [Self::White, Self::Blue, Self::Black, Self::Red, Self::Green];

    #[must_use]
    pub const fn spelling(self) -> &'static str {
        match self {
            Self::White => "white",
            Self::Blue => "blue",
            Self::Black => "black",
            Self::Red => "red",
            Self::Green => "green",
        }
    }

    /// The color a lowercase surface names, or `None`. Case-sensitive: color
    /// words are always lowercase in the value-nominal argument position.
    #[must_use]
    pub fn from_surface(surface: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|color| color.spelling() == surface)
    }
}

impl Countability {
    const fn accepts(self, form: NounSurface) -> bool {
        match self {
            Self::Count => matches!(form, NounSurface::Singular | NounSurface::Plural),
            Self::Mass => matches!(form, NounSurface::Mass),
            Self::CountOrMass => true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum NounSurface {
    Singular,
    Plural,
    Mass,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum IndexedWord {
    Noun { vocab: Vocab, form: NounSurface },
    Verb { vocab: Vocab, slot: VerbSlot },
    Adjective(Vocab),
    Participle { vocab: Vocab, tense: Tense },
    Gerund(Vocab),
    Adverb(Vocab),
    SentenceAdverbial(Vocab),
    Color(ColorWord),
    Pronoun(PronounInstance),
    Auxiliary(AuxiliaryInstance),
}

impl IndexedWord {
    fn for_slot(self, slot: LexicalSlot) -> Option<WordMatch> {
        match (self, slot) {
            (Self::Noun { vocab, form }, LexicalSlot::Noun(usage)) if usage.accepts(form) => {
                let noun = Noun::Word(vocab);
                Some(WordMatch::Noun(match form {
                    NounSurface::Singular => NounInstance::Singular(noun),
                    NounSurface::Plural => NounInstance::Plural(noun),
                    NounSurface::Mass => NounInstance::Mass(noun),
                }))
            }
            (Self::Gerund(vocab), LexicalSlot::Noun(NounUsage::Mass | NounUsage::Either)) => Some(
                WordMatch::Noun(NounInstance::Mass(Noun::Gerund(Verb::Word(vocab)))),
            ),
            (
                Self::Verb {
                    vocab,
                    slot: candidate_slot,
                },
                LexicalSlot::Verb(requested_slot),
            ) if candidate_slot == requested_slot => Some(WordMatch::Verb(VerbInstance {
                verb: Verb::Word(vocab),
                slot: candidate_slot,
            })),
            (Self::Adjective(vocab), LexicalSlot::Adjective) => {
                Some(WordMatch::Adjective(Adjective::Word(vocab)))
            }
            (Self::Participle { vocab, tense }, LexicalSlot::Adjective) => Some(
                WordMatch::Adjective(Adjective::Participle(tense, Verb::Word(vocab))),
            ),
            (Self::Adverb(vocab), LexicalSlot::Adverb) => Some(WordMatch::Adverb(vocab)),
            (Self::SentenceAdverbial(vocab), LexicalSlot::SentenceAdverbial) => {
                Some(WordMatch::SentenceAdverbial(vocab))
            }
            (Self::Color(color), LexicalSlot::Adjective) => {
                Some(WordMatch::Adjective(Adjective::Color(color)))
            }
            (Self::Pronoun(pronoun), LexicalSlot::Pronoun(case)) if pronoun.case == case => {
                Some(WordMatch::Pronoun(pronoun))
            }
            (Self::Auxiliary(auxiliary), LexicalSlot::Auxiliary) => {
                Some(WordMatch::Auxiliary(auxiliary))
            }
            _ => None,
        }
    }
}

impl NounUsage {
    const fn accepts(self, form: NounSurface) -> bool {
        match self {
            Self::Count => matches!(form, NounSurface::Singular | NounSurface::Plural),
            Self::Mass => matches!(form, NounSurface::Mass),
            Self::Either => true,
        }
    }
}

fn reverse_index() -> &'static HashMap<String, Vec<IndexedWord>> {
    static INDEX: OnceLock<HashMap<String, Vec<IndexedWord>>> = OnceLock::new();
    INDEX.get_or_init(build_reverse_index)
}

fn build_reverse_index() -> HashMap<String, Vec<IndexedWord>> {
    let vocabulary = Vocabulary::new();
    let mut index = HashMap::new();

    for &vocab in Vocab::ALL {
        let definition = vocab.definition();
        index_vocab(&mut index, vocabulary, vocab, definition);
    }

    for definition in regular_vocabulary() {
        if Vocab::ALL
            .iter()
            .copied()
            .any(|vocab| vocab.spelling() == definition.spelling)
        {
            continue;
        }
        let vocab = Vocab::Regular(RegularVocab(definition.spelling));
        index_vocab(&mut index, vocabulary, vocab, *definition);
    }

    for color in [
        ColorWord::White,
        ColorWord::Blue,
        ColorWord::Black,
        ColorWord::Red,
        ColorWord::Green,
    ] {
        insert_index(&mut index, color.spelling(), IndexedWord::Color(color));
    }

    for auxiliary in auxiliary_instances() {
        if let Some(surface) = vocabulary.render_auxiliary(auxiliary) {
            insert_index(&mut index, surface, IndexedWord::Auxiliary(auxiliary));
        }
    }

    for (surface, pronoun) in PRONOUN_FORMS {
        insert_index(&mut index, surface, IndexedWord::Pronoun(pronoun));
    }

    index
}

fn index_vocab(
    index: &mut HashMap<String, Vec<IndexedWord>>,
    vocabulary: Vocabulary,
    vocab: Vocab,
    definition: VocabDefinition,
) {
    if let Some((_, countability)) = definition.noun
        && !definition.catalog_noun
    {
        for form in [
            NounSurface::Singular,
            NounSurface::Plural,
            NounSurface::Mass,
        ] {
            if countability.accepts(form) {
                let surface = Vocabulary::render_vocab_noun(vocab, form)
                    .expect("declared noun form must render");
                insert_index(index, &surface, IndexedWord::Noun { vocab, form });
            }
        }
    }

    if definition.verb.is_some() {
        for slot in VERB_SLOTS {
            let surface = vocabulary
                .render_verb(vocab, slot)
                .expect("declared verb form must render");
            insert_index(index, &surface, IndexedWord::Verb { vocab, slot });
        }

        let present_participle = vocabulary
            .render_verb(vocab, VerbSlot::PresentParticiple)
            .expect("declared verb present participle must render");
        insert_index(
            index,
            &present_participle,
            IndexedWord::Participle {
                vocab,
                tense: Tense::Present,
            },
        );
        insert_index(index, &present_participle, IndexedWord::Gerund(vocab));

        let past_participle = vocabulary
            .render_verb(vocab, VerbSlot::PastParticiple)
            .expect("declared verb past participle must render");
        insert_index(
            index,
            &past_participle,
            IndexedWord::Participle {
                vocab,
                tense: Tense::Past,
            },
        );
    }

    if definition.adjective
        && !index.get(definition.spelling).is_some_and(|candidates| {
            candidates
                .iter()
                .any(|candidate| matches!(candidate, IndexedWord::Participle { .. }))
        })
    {
        insert_index(index, definition.spelling, IndexedWord::Adjective(vocab));
    }
    if definition.adverb {
        insert_index(index, definition.spelling, IndexedWord::Adverb(vocab));
    }
    if definition.sentence_adverbial {
        insert_index(
            index,
            definition.spelling,
            IndexedWord::SentenceAdverbial(vocab),
        );
    }
}

fn insert_index(
    index: &mut HashMap<String, Vec<IndexedWord>>,
    surface: &str,
    candidate: IndexedWord,
) {
    let candidates = index.entry(surface.to_ascii_lowercase()).or_default();
    if !candidates.contains(&candidate) {
        candidates.push(candidate);
    }
}

pub(crate) const VERB_SLOTS: [VerbSlot; 12] = [
    VerbSlot::Infinitive,
    VerbSlot::Imperative,
    VerbSlot::Present {
        person: Person::Second,
        number: Number::Singular,
    },
    VerbSlot::Present {
        person: Person::Second,
        number: Number::Plural,
    },
    VerbSlot::Present {
        person: Person::Third,
        number: Number::Singular,
    },
    VerbSlot::Present {
        person: Person::Third,
        number: Number::Plural,
    },
    VerbSlot::Past {
        person: Person::Second,
        number: Number::Singular,
    },
    VerbSlot::Past {
        person: Person::Second,
        number: Number::Plural,
    },
    VerbSlot::Past {
        person: Person::Third,
        number: Number::Singular,
    },
    VerbSlot::Past {
        person: Person::Third,
        number: Number::Plural,
    },
    VerbSlot::PresentParticiple,
    VerbSlot::PastParticiple,
];

const PRONOUN_FORMS: [(&str, PronounInstance); 14] = [
    (
        "you",
        PronounInstance {
            pronoun: Pronoun::You,
            case: PronounCase::Subject,
        },
    ),
    (
        "you",
        PronounInstance {
            pronoun: Pronoun::You,
            case: PronounCase::Object,
        },
    ),
    (
        "it",
        PronounInstance {
            pronoun: Pronoun::It(Gender::Neuter),
            case: PronounCase::Subject,
        },
    ),
    (
        "it",
        PronounInstance {
            pronoun: Pronoun::It(Gender::Neuter),
            case: PronounCase::Object,
        },
    ),
    (
        "they",
        PronounInstance {
            pronoun: Pronoun::They,
            case: PronounCase::Subject,
        },
    ),
    (
        "them",
        PronounInstance {
            pronoun: Pronoun::They,
            case: PronounCase::Object,
        },
    ),
    (
        "he",
        PronounInstance {
            pronoun: Pronoun::It(Gender::Masculine),
            case: PronounCase::Subject,
        },
    ),
    (
        "him",
        PronounInstance {
            pronoun: Pronoun::It(Gender::Masculine),
            case: PronounCase::Object,
        },
    ),
    (
        "she",
        PronounInstance {
            pronoun: Pronoun::It(Gender::Feminine),
            case: PronounCase::Subject,
        },
    ),
    (
        "her",
        PronounInstance {
            pronoun: Pronoun::It(Gender::Feminine),
            case: PronounCase::Object,
        },
    ),
    (
        "each other",
        PronounInstance {
            pronoun: Pronoun::EachOther,
            case: PronounCase::Object,
        },
    ),
    (
        "itself",
        PronounInstance {
            pronoun: Pronoun::Itself,
            case: PronounCase::Object,
        },
    ),
    (
        "himself",
        PronounInstance {
            pronoun: Pronoun::Himself,
            case: PronounCase::Object,
        },
    ),
    (
        "yours",
        PronounInstance {
            pronoun: Pronoun::YoursAbsolute,
            case: PronounCase::Object,
        },
    ),
];

fn auxiliary_instances() -> impl Iterator<Item = AuxiliaryInstance> {
    const AUXILIARIES: [Auxiliary; 12] = [
        Auxiliary::Can,
        Auxiliary::Could,
        Auxiliary::Do,
        Auxiliary::May,
        Auxiliary::Might,
        Auxiliary::Must,
        Auxiliary::Shall,
        Auxiliary::Should,
        Auxiliary::Will,
        Auxiliary::Would,
        Auxiliary::Be,
        Auxiliary::Have,
    ];
    const INFLECTIONS: [AuxiliaryInflection; 12] = [
        AuxiliaryInflection::Base,
        AuxiliaryInflection::Present {
            person: Person::Second,
            number: Number::Singular,
        },
        AuxiliaryInflection::Present {
            person: Person::Second,
            number: Number::Plural,
        },
        AuxiliaryInflection::Present {
            person: Person::Third,
            number: Number::Singular,
        },
        AuxiliaryInflection::Present {
            person: Person::Third,
            number: Number::Plural,
        },
        AuxiliaryInflection::Past {
            person: Person::Second,
            number: Number::Singular,
        },
        AuxiliaryInflection::Past {
            person: Person::Second,
            number: Number::Plural,
        },
        AuxiliaryInflection::Past {
            person: Person::Third,
            number: Number::Singular,
        },
        AuxiliaryInflection::Past {
            person: Person::Third,
            number: Number::Plural,
        },
        AuxiliaryInflection::PastSubjunctive,
        AuxiliaryInflection::PresentParticiple,
        AuxiliaryInflection::PastParticiple,
    ];

    AUXILIARIES.into_iter().flat_map(|auxiliary| {
        INFLECTIONS.into_iter().flat_map(move |inflection| {
            [false, true].map(move |contracted_negation| AuxiliaryInstance {
                auxiliary,
                inflection,
                contracted_negation,
            })
        })
    })
}

const fn render_auxiliary(instance: AuxiliaryInstance) -> Option<&'static str> {
    use Auxiliary as A;
    use AuxiliaryInflection as I;
    use Number as N;
    use Person as P;

    if instance.contracted_negation {
        return match (instance.auxiliary, instance.inflection) {
            (A::Can, I::Base) => Some("can't"),
            (A::Could, I::Base) => Some("couldn't"),
            (A::Will, I::Base) => Some("won't"),
            (
                A::Do,
                I::Base
                | I::Present {
                    person: P::Second, ..
                }
                | I::Present {
                    person: P::Third,
                    number: N::Plural,
                },
            ) => Some("don't"),
            (
                A::Do,
                I::Present {
                    person: P::Third,
                    number: N::Singular,
                },
            ) => Some("doesn't"),
            (A::Do, I::Past { .. }) => Some("didn't"),
            (
                A::Be,
                I::Present {
                    person: P::Third,
                    number: N::Singular,
                },
            ) => Some("isn't"),
            (A::Be, I::Present { .. }) => Some("aren't"),
            (
                A::Be,
                I::Past {
                    person: P::Third,
                    number: N::Singular,
                },
            ) => Some("wasn't"),
            (A::Be, I::Past { .. } | I::PastSubjunctive) => Some("weren't"),
            (
                A::Have,
                I::Present {
                    person: P::Third,
                    number: N::Singular,
                },
            ) => Some("hasn't"),
            (A::Have, I::Base | I::Present { .. }) => Some("haven't"),
            (A::Have, I::Past { .. }) => Some("hadn't"),
            _ => None,
        };
    }

    match (instance.auxiliary, instance.inflection) {
        (A::Can, I::Base) => Some("can"),
        (A::Could, I::Base) => Some("could"),
        (A::May, I::Base) => Some("may"),
        (A::Might, I::Base) => Some("might"),
        (A::Must, I::Base) => Some("must"),
        (A::Shall, I::Base) => Some("shall"),
        (A::Should, I::Base) => Some("should"),
        (A::Will, I::Base) => Some("will"),
        (A::Would, I::Base) => Some("would"),
        (
            A::Do,
            I::Base
            | I::Present {
                person: P::Second, ..
            }
            | I::Present {
                person: P::Third,
                number: N::Plural,
            },
        ) => Some("do"),
        (
            A::Do,
            I::Present {
                person: P::Third,
                number: N::Singular,
            },
        ) => Some("does"),
        (A::Do, I::Past { .. }) => Some("did"),
        (A::Do, I::PresentParticiple) => Some("doing"),
        (A::Do, I::PastParticiple) => Some("done"),
        (A::Be, I::Base) => Some("be"),
        (
            A::Be,
            I::Present {
                person: P::Third,
                number: N::Singular,
            },
        ) => Some("is"),
        (A::Be, I::Present { .. }) => Some("are"),
        (
            A::Be,
            I::Past {
                person: P::Third,
                number: N::Singular,
            },
        ) => Some("was"),
        (A::Be, I::Past { .. } | I::PastSubjunctive) => Some("were"),
        (A::Be, I::PresentParticiple) => Some("being"),
        (A::Be, I::PastParticiple) => Some("been"),
        (
            A::Have,
            I::Base
            | I::Present {
                person: P::Second, ..
            }
            | I::Present {
                person: P::Third,
                number: N::Plural,
            },
        ) => Some("have"),
        (
            A::Have,
            I::Present {
                person: P::Third,
                number: N::Singular,
            },
        ) => Some("has"),
        (A::Have, I::Past { .. } | I::PastParticiple) => Some("had"),
        (A::Have, I::PresentParticiple) => Some("having"),
        _ => None,
    }
}

fn render_verb_form(lemma: &str, form: VerbForm, slot: VerbSlot) -> String {
    if let VerbForm::Irregular(irregular) = form {
        let override_form = match slot {
            VerbSlot::Infinitive | VerbSlot::Imperative => None,
            VerbSlot::Present {
                person: Person::Second,
                ..
            } => irregular.present_second,
            VerbSlot::Present {
                person: Person::Third,
                number: Number::Singular,
            } => irregular.present_third_singular,
            VerbSlot::Present {
                person: Person::Third,
                number: Number::Plural,
            } => irregular.present_third_plural,
            VerbSlot::Past {
                person: Person::Second,
                ..
            } => irregular.past_second,
            VerbSlot::Past {
                person: Person::Third,
                number: Number::Singular,
            } => irregular.past_third_singular,
            VerbSlot::Past {
                person: Person::Third,
                number: Number::Plural,
            } => irregular.past_third_plural,
            VerbSlot::PresentParticiple => irregular.present_participle,
            VerbSlot::PastParticiple => irregular.past_participle,
        };
        if let Some(surface) = override_form {
            return surface.to_owned();
        }
    }

    match slot {
        VerbSlot::Present {
            person: Person::Third,
            number: Number::Singular,
        } => regular_third_person_singular(lemma),
        VerbSlot::Infinitive | VerbSlot::Imperative | VerbSlot::Present { .. } => lemma.to_owned(),
        VerbSlot::Past { .. } | VerbSlot::PastParticiple => regular_past(lemma),
        VerbSlot::PresentParticiple => regular_present_participle(lemma),
    }
}

fn render_noun_form(lemma: &str, declension: NounDeclension, form: NounSurface) -> String {
    match (declension, form) {
        (NounDeclension::Irregular { singular, .. }, NounSurface::Singular)
        | (NounDeclension::Irregular { singular, .. }, NounSurface::Mass) => singular.to_owned(),
        (NounDeclension::Irregular { plural, .. }, NounSurface::Plural) => plural.to_owned(),
        (NounDeclension::Invariant, _) | (NounDeclension::Regular, NounSurface::Mass) => {
            lemma.to_owned()
        }
        (NounDeclension::Regular, NounSurface::Singular) => lemma.to_owned(),
        (NounDeclension::Regular, NounSurface::Plural) => regular_plural(lemma),
    }
}

pub(crate) fn regular_plural(word: &str) -> String {
    if let Some(stem) = consonant_y_stem(word) {
        format!("{stem}ies")
    } else if has_sibilant_ending(word) {
        format!("{word}es")
    } else {
        format!("{word}s")
    }
}

fn regular_third_person_singular(verb: &str) -> String {
    if let Some(stem) = consonant_y_stem(verb) {
        format!("{stem}ies")
    } else if has_sibilant_ending(verb) || verb.ends_with('o') {
        format!("{verb}es")
    } else {
        format!("{verb}s")
    }
}

fn regular_past(verb: &str) -> String {
    if let Some(stem) = consonant_y_stem(verb) {
        format!("{stem}ied")
    } else if verb.ends_with('e') {
        format!("{verb}d")
    } else {
        format!("{verb}ed")
    }
}

fn regular_present_participle(verb: &str) -> String {
    if let Some(stem) = verb.strip_suffix("ie") {
        format!("{stem}ying")
    } else if let Some(stem) = verb.strip_suffix('e')
        && !verb.ends_with("ee")
        && !verb.ends_with("ye")
        && !verb.ends_with("oe")
    {
        format!("{stem}ing")
    } else {
        format!("{verb}ing")
    }
}

fn consonant_y_stem(word: &str) -> Option<&str> {
    let stem = word.strip_suffix('y')?;
    stem.chars()
        .next_back()
        .is_some_and(|character| !is_vowel(character))
        .then_some(stem)
}

fn has_sibilant_ending(word: &str) -> bool {
    ["s", "x", "z", "ch", "sh"]
        .iter()
        .any(|ending| word.ends_with(ending))
}

const fn is_vowel(character: char) -> bool {
    matches!(character, 'a' | 'e' | 'i' | 'o' | 'u')
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    const THIRD_PLURAL_PRESENT: VerbSlot = VerbSlot::Present {
        person: Person::Third,
        number: Number::Plural,
    };
    const THIRD_SINGULAR_PRESENT: VerbSlot = VerbSlot::Present {
        person: Person::Third,
        number: Number::Singular,
    };

    #[test]
    fn one_surface_can_fill_only_the_requested_lexical_slot() {
        let vocabulary = Vocabulary::new();

        assert_eq!(
            vocabulary.matches("cost", LexicalSlot::Noun(NounUsage::Count)),
            vec![WordMatch::Noun(NounInstance::Singular(Noun::Word(
                Vocab::Cost,
            )))]
        );
        assert_eq!(
            vocabulary.matches("cost", LexicalSlot::Verb(THIRD_PLURAL_PRESENT)),
            vec![WordMatch::Verb(VerbInstance {
                verb: Verb::Word(Vocab::Cost),
                slot: THIRD_PLURAL_PRESENT,
            })]
        );
        assert_eq!(
            vocabulary.matches("target", LexicalSlot::Adjective),
            vec![WordMatch::Adjective(Adjective::Word(Vocab::Target))]
        );
        assert_eq!(
            vocabulary.matches("target", LexicalSlot::Noun(NounUsage::Count)),
            vec![WordMatch::Noun(NounInstance::Singular(Noun::Word(
                Vocab::Target,
            )))]
        );
        assert_eq!(
            vocabulary.matches("targets", LexicalSlot::Verb(THIRD_SINGULAR_PRESENT)),
            vec![WordMatch::Verb(VerbInstance {
                verb: Verb::Word(Vocab::Target),
                slot: THIRD_SINGULAR_PRESENT,
            })]
        );
    }

    #[test]
    fn regular_count_nouns_lookup_and_render_without_source_text() {
        let vocabulary = Vocabulary::new();
        let matches = vocabulary.matches("upkeep", LexicalSlot::Noun(NounUsage::Count));
        let [WordMatch::Noun(NounInstance::Singular(Noun::Word(upkeep)))] = matches.as_slice()
        else {
            panic!("upkeep must have one singular count-noun analysis");
        };

        assert_eq!(upkeep.spelling(), "upkeep");
        assert_eq!(
            vocabulary.render_noun(&NounInstance::Singular(Noun::Word(*upkeep))),
            Some("upkeep".to_owned())
        );
        assert_eq!(
            vocabulary.render_noun(&NounInstance::Plural(Noun::Word(*upkeep))),
            Some("upkeeps".to_owned())
        );
        assert_eq!(
            vocabulary.matches("upkeeps", LexicalSlot::Noun(NounUsage::Count)),
            vec![WordMatch::Noun(NounInstance::Plural(Noun::Word(*upkeep)))]
        );
    }

    #[test]
    fn regular_mass_nouns_stay_out_of_count_noun_slots() {
        let vocabulary = Vocabulary::new();
        let matches = vocabulary.matches("knowledge", LexicalSlot::Noun(NounUsage::Mass));
        let [WordMatch::Noun(NounInstance::Mass(Noun::Word(knowledge)))] = matches.as_slice()
        else {
            panic!("knowledge must have one mass-noun analysis");
        };

        assert_eq!(knowledge.spelling(), "knowledge");
        assert_eq!(
            vocabulary.render_noun(&NounInstance::Mass(Noun::Word(*knowledge))),
            Some("knowledge".to_owned())
        );
        assert!(
            vocabulary
                .matches("knowledge", LexicalSlot::Noun(NounUsage::Count))
                .is_empty()
        );
    }

    #[test]
    fn regular_verbs_generate_and_lookup_every_inflection() {
        let vocabulary = Vocabulary::new();
        let matches = vocabulary.matches("assign", LexicalSlot::Verb(VerbSlot::Infinitive));
        let [
            WordMatch::Verb(VerbInstance {
                verb: Verb::Word(assign),
                slot: VerbSlot::Infinitive,
            }),
        ] = matches.as_slice()
        else {
            panic!("assign must have one infinitive analysis");
        };

        for (slot, surface) in [
            (THIRD_SINGULAR_PRESENT, "assigns"),
            (
                VerbSlot::Past {
                    person: Person::Third,
                    number: Number::Singular,
                },
                "assigned",
            ),
            (VerbSlot::PresentParticiple, "assigning"),
            (VerbSlot::PastParticiple, "assigned"),
        ] {
            assert_eq!(
                vocabulary.render_verb(*assign, slot).as_deref(),
                Some(surface)
            );
            assert_eq!(
                vocabulary.matches(surface, LexicalSlot::Verb(slot)),
                vec![WordMatch::Verb(VerbInstance {
                    verb: Verb::Word(*assign),
                    slot,
                })]
            );
        }
    }

    #[test]
    fn regular_adjectives_and_adverbs_fill_only_their_declared_slots() {
        let vocabulary = Vocabulary::new();
        let adjective_matches = vocabulary.matches("lethal", LexicalSlot::Adjective);
        let [WordMatch::Adjective(Adjective::Word(lethal))] = adjective_matches.as_slice() else {
            panic!("lethal must have one adjective analysis");
        };
        let adverb_matches = vocabulary.matches("already", LexicalSlot::Adverb);
        let [WordMatch::Adverb(already)] = adverb_matches.as_slice() else {
            panic!("already must have one adverb analysis");
        };

        assert_eq!(lethal.spelling(), "lethal");
        assert_eq!(already.spelling(), "already");
        assert!(vocabulary.matches("lethal", LexicalSlot::Adverb).is_empty());
        assert!(
            vocabulary
                .matches("already", LexicalSlot::Adjective)
                .is_empty()
        );
    }

    #[test]
    fn named_vocab_identity_wins_when_the_regular_table_adds_a_part_of_speech() {
        let vocabulary = Vocabulary::new();

        assert_eq!(
            vocabulary.matches("attack", LexicalSlot::Noun(NounUsage::Count)),
            vec![WordMatch::Noun(NounInstance::Singular(Noun::Word(
                Vocab::Attack,
            )))]
        );
        assert_eq!(
            vocabulary.matches("attack", LexicalSlot::Verb(VerbSlot::Infinitive)),
            vec![WordMatch::Verb(VerbInstance {
                verb: Verb::Word(Vocab::Attack),
                slot: VerbSlot::Infinitive,
            })]
        );
    }

    #[test]
    fn regular_vocabulary_table_is_sorted_deduplicated_and_well_formed() {
        const TABLE: &str = include_str!("regular-vocabulary.tsv");
        let mut previous = None;
        let mut rows = 0;

        for line in TABLE.lines() {
            let (lemma, part_of_speech) = line
                .split_once('\t')
                .unwrap_or_else(|| panic!("malformed regular-vocabulary row: {line:?}"));
            assert!(
                !lemma.is_empty()
                    && lemma
                        .bytes()
                        .all(|byte| { byte.is_ascii_lowercase() || matches!(byte, b'-' | b'\'') }),
                "invalid regular-vocabulary lemma: {lemma:?}"
            );
            assert!(
                matches!(
                    part_of_speech,
                    "adjective"
                        | "adverb"
                        | "noun_count"
                        | "noun_count_or_mass"
                        | "noun_mass"
                        | "verb"
                ),
                "unknown part of speech in regular-vocabulary row: {line:?}"
            );
            if let Some(previous) = previous {
                assert!(previous < line, "table is not sorted and deduplicated");
            }
            previous = Some(line);
            rows += 1;
        }

        assert!(rows > 900, "regular-vocabulary table is unexpectedly small");
    }

    #[test]
    fn explicit_count_or_mass_row_dominates_a_narrower_noun_row() {
        let mut definition = VocabDefinition::new("example");
        definition.add_regular_noun(Countability::Count);
        definition.add_regular_noun(Countability::CountOrMass);

        assert_eq!(
            definition.noun,
            Some((NounDeclension::Regular, Countability::CountOrMass))
        );
    }

    #[test]
    fn irregular_verbs_are_selected_by_fixed_slots() {
        let vocabulary = Vocabulary::new();

        assert_eq!(
            vocabulary.render_verb(
                Vocab::Draw,
                VerbSlot::Past {
                    person: Person::Third,
                    number: Number::Singular,
                },
            ),
            Some("drew".to_owned())
        );
        assert_eq!(
            vocabulary.render_verb(Vocab::Draw, VerbSlot::PastParticiple),
            Some("drawn".to_owned())
        );
        assert_eq!(
            vocabulary.matches("do", LexicalSlot::Verb(THIRD_PLURAL_PRESENT)),
            vec![WordMatch::Verb(VerbInstance {
                verb: Verb::Word(Vocab::Do),
                slot: THIRD_PLURAL_PRESENT,
            })]
        );
        assert_eq!(
            vocabulary.matches("does", LexicalSlot::Verb(THIRD_SINGULAR_PRESENT)),
            vec![WordMatch::Verb(VerbInstance {
                verb: Verb::Word(Vocab::Do),
                slot: THIRD_SINGULAR_PRESENT,
            })]
        );
    }

    #[test]
    fn derived_participles_and_gerunds_exist_only_in_the_requested_slot() {
        let vocabulary = Vocabulary::new();
        let target = Verb::Word(Vocab::Target);
        let draw = Verb::Word(Vocab::Draw);

        assert_eq!(
            vocabulary.matches("targeted", LexicalSlot::Adjective),
            vec![WordMatch::Adjective(Adjective::Participle(
                Tense::Past,
                target.clone(),
            ))]
        );
        assert_eq!(
            vocabulary.render_adjective(&Adjective::Participle(Tense::Past, target)),
            Some("targeted".to_owned())
        );
        assert_eq!(
            vocabulary.matches("drawing", LexicalSlot::Noun(NounUsage::Mass)),
            vec![WordMatch::Noun(NounInstance::Mass(Noun::Gerund(
                draw.clone(),
            )))]
        );
        assert_eq!(
            vocabulary.render_noun(&NounInstance::Mass(Noun::Gerund(draw))),
            Some("drawing".to_owned())
        );
        assert!(
            vocabulary
                .matches("targeted", LexicalSlot::Noun(NounUsage::Count))
                .is_empty()
        );
    }

    #[test]
    fn noun_declension_drives_both_lookup_and_rendering() {
        let vocabulary = Vocabulary::new();

        for (surface, vocab) in [("libraries", Vocab::Library), ("copies", Vocab::Copy)] {
            let noun = NounInstance::Plural(Noun::Word(vocab));
            assert_eq!(
                vocabulary.matches(surface, LexicalSlot::Noun(NounUsage::Count)),
                vec![WordMatch::Noun(noun.clone())]
            );
            assert_eq!(vocabulary.render_noun(&noun).as_deref(), Some(surface));
        }

        for noun in [
            NounInstance::Singular(Noun::Word(Vocab::Merfolk)),
            NounInstance::Plural(Noun::Word(Vocab::Merfolk)),
        ] {
            assert_eq!(vocabulary.render_noun(&noun), Some("Merfolk".to_owned()));
        }
    }

    #[test]
    fn speculative_waterbend_past_forms_are_irregular() {
        let vocabulary = Vocabulary::new();

        for slot in [
            VerbSlot::Past {
                person: Person::Third,
                number: Number::Singular,
            },
            VerbSlot::PastParticiple,
        ] {
            assert_eq!(
                vocabulary.render_verb(Vocab::Waterbend, slot),
                Some("waterbent".to_owned())
            );
            assert_eq!(
                vocabulary.matches("waterbent", LexicalSlot::Verb(slot)),
                vec![WordMatch::Verb(VerbInstance {
                    verb: Verb::Word(Vocab::Waterbend),
                    slot,
                })]
            );
        }
    }

    #[test]
    fn colors_and_pronunciation_are_semantic() {
        let vocabulary = Vocabulary::new();

        assert_eq!(
            vocabulary.matches("black", LexicalSlot::Adjective),
            vec![WordMatch::Adjective(Adjective::Color(ColorWord::Black))]
        );
        assert_eq!(vocabulary.initial_sound(Vocab::Hour), InitialSound::Vowel);
    }

    #[test]
    fn supported_auxiliary_contractions_are_structured() {
        let vocabulary = Vocabulary::new();
        let contracted_can = AuxiliaryInstance {
            auxiliary: Auxiliary::Can,
            inflection: AuxiliaryInflection::Base,
            contracted_negation: true,
        };

        assert_eq!(
            vocabulary.matches("can't", LexicalSlot::Auxiliary),
            vec![WordMatch::Auxiliary(contracted_can)]
        );
        assert_eq!(vocabulary.render_auxiliary(contracted_can), Some("can't"));
        assert!(
            vocabulary
                .matches("cannot", LexicalSlot::Auxiliary)
                .is_empty()
        );
        assert!(
            vocabulary
                .matches("mightn't've", LexicalSlot::Auxiliary)
                .is_empty()
        );
        assert_eq!(
            vocabulary.render_auxiliary(AuxiliaryInstance {
                auxiliary: Auxiliary::May,
                inflection: AuxiliaryInflection::Base,
                contracted_negation: true,
            }),
            None
        );
    }

    #[test]
    fn unsupported_first_person_pronouns_have_no_candidate() {
        let vocabulary = Vocabulary::new();

        assert!(
            vocabulary
                .matches("I", LexicalSlot::Pronoun(PronounCase::Subject))
                .is_empty()
        );
    }

    #[test]
    fn third_person_singular_pronouns_preserve_gender() {
        let vocabulary = Vocabulary::new();

        for (gender, subject, object, possessive) in [
            (Gender::Masculine, "he", "him", "his"),
            (Gender::Feminine, "she", "her", "her"),
            (Gender::Neuter, "it", "it", "its"),
        ] {
            let pronoun = Pronoun::It(gender);
            let subject_instance = PronounInstance {
                pronoun,
                case: PronounCase::Subject,
            };
            let object_instance = PronounInstance {
                pronoun,
                case: PronounCase::Object,
            };

            assert_eq!(
                vocabulary.matches(subject, LexicalSlot::Pronoun(PronounCase::Subject)),
                vec![WordMatch::Pronoun(subject_instance)]
            );
            assert_eq!(
                vocabulary.matches(object, LexicalSlot::Pronoun(PronounCase::Object)),
                vec![WordMatch::Pronoun(object_instance)]
            );
            assert_eq!(vocabulary.render_pronoun(subject_instance), Some(subject));
            assert_eq!(vocabulary.render_pronoun(object_instance), Some(object));
            assert_eq!(
                vocabulary.render_possessive_pronoun(pronoun),
                Some(possessive)
            );
        }
    }

    #[test]
    fn every_declared_form_round_trips_through_its_slot() {
        let vocabulary = Vocabulary::new();
        let mut spellings = HashSet::new();

        for &vocab in Vocab::ALL {
            let definition = vocab.definition();
            assert!(spellings.insert(definition.spelling), "duplicate {vocab:?}");
            assert!(
                definition.noun.is_some()
                    || definition.verb.is_some()
                    || definition.adjective
                    || definition.adverb
                    || definition.sentence_adverbial,
                "{vocab:?} has no lexical role"
            );

            if let Some((_, countability)) = definition.noun
                && !definition.catalog_noun
            {
                for form in [
                    NounSurface::Singular,
                    NounSurface::Plural,
                    NounSurface::Mass,
                ] {
                    if !countability.accepts(form) {
                        continue;
                    }
                    let noun = match form {
                        NounSurface::Singular => NounInstance::Singular(Noun::Word(vocab)),
                        NounSurface::Plural => NounInstance::Plural(Noun::Word(vocab)),
                        NounSurface::Mass => NounInstance::Mass(Noun::Word(vocab)),
                    };
                    let surface = vocabulary.render_noun(&noun).unwrap();
                    let usage = match form {
                        NounSurface::Singular | NounSurface::Plural => NounUsage::Count,
                        NounSurface::Mass => NounUsage::Mass,
                    };
                    assert!(
                        vocabulary
                            .matches(&surface, LexicalSlot::Noun(usage))
                            .contains(&WordMatch::Noun(noun)),
                        "noun {vocab:?} {form:?} rendered as {surface:?} but did not parse"
                    );
                }
            }

            if definition.verb.is_some() {
                for slot in VERB_SLOTS {
                    let verb = VerbInstance {
                        verb: Verb::Word(vocab),
                        slot,
                    };
                    let surface = vocabulary.render_verb(vocab, slot).unwrap();
                    assert!(
                        vocabulary
                            .matches(&surface, LexicalSlot::Verb(slot))
                            .contains(&WordMatch::Verb(verb)),
                        "verb {vocab:?} {slot:?} rendered as {surface:?} but did not parse"
                    );
                }
            }

            if definition.adjective {
                let adjective = Adjective::Word(vocab);
                assert!(
                    vocabulary
                        .matches(definition.spelling, LexicalSlot::Adjective)
                        .contains(&WordMatch::Adjective(adjective)),
                    "adjective {vocab:?} did not parse"
                );
            }

            if definition.adverb {
                assert!(
                    vocabulary
                        .matches(definition.spelling, LexicalSlot::Adverb)
                        .contains(&WordMatch::Adverb(vocab)),
                    "adverb {vocab:?} did not parse"
                );
            }

            if definition.sentence_adverbial {
                assert!(
                    vocabulary
                        .matches(definition.spelling, LexicalSlot::SentenceAdverbial)
                        .contains(&WordMatch::SentenceAdverbial(vocab)),
                    "sentence adverbial {vocab:?} did not parse"
                );
            }
        }

        for auxiliary in auxiliary_instances() {
            if let Some(surface) = vocabulary.render_auxiliary(auxiliary) {
                assert!(
                    vocabulary
                        .matches(surface, LexicalSlot::Auxiliary)
                        .contains(&WordMatch::Auxiliary(auxiliary)),
                    "auxiliary {auxiliary:?} rendered as {surface:?} but did not parse"
                );
            }
        }
    }
}
