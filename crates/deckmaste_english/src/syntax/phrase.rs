use std::hash::Hash;
use std::hash::Hasher;
use std::sync::Arc;

use super::ability::Ability;
use super::ability::Cost;
use super::ability::QuotedAbility;
use super::clause::Clause;
use super::clause::IndependentClause;
use super::clause::InfinitiveClause;
use super::clause::RelativeClause;
use super::clause::TransitivePredicate;
use crate::Numeral;
use crate::catalog::CatalogAtom;
use crate::catalog::CatalogKind;
use crate::constructions::coordination::CoordinatedNominalPhrase;
use crate::constructions::coordination::CoordinatedNounPhrase;
use crate::features::Comma;
use crate::features::Conjunction;
use crate::word::Adjective;
use crate::word::AdjectiveComparisonClass;
use crate::word::CardOrientation;
use crate::word::ColorWord;
use crate::word::InitialSound;
use crate::word::Noun;
use crate::word::NounInstance;
use crate::word::Pronoun;
use crate::word::PronounCase;
use crate::word::Vocab;
use crate::word::Vocabulary;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
pub struct RecoveredText {
    spelling: String,
    source_tokens: usize,
}

impl RecoveredText {
    #[must_use]
    pub fn new(spelling: impl Into<String>, source_tokens: usize) -> Self {
        Self {
            spelling: spelling.into(),
            source_tokens,
        }
    }

    #[must_use]
    pub fn spelling(&self) -> &str {
        &self.spelling
    }

    #[must_use]
    pub const fn source_tokens(&self) -> usize {
        self.source_tokens
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
pub struct OpaqueLexeme(String);

impl OpaqueLexeme {
    #[must_use]
    pub fn new(spelling: impl Into<String>) -> Self {
        Self(spelling.into())
    }

    #[must_use]
    pub fn spelling(&self) -> &str {
        &self.0
    }
}

/// The surface form of a self-reference to the card being parsed.
///
/// Both forms denote the same object: [CR#201.5] holds that text referring to
/// the object it is on by name means just that object. Neither variant stores a
/// spelling — the renderer re-derives it from the face name — so the two forms
/// are interchangeable referents distinguished only by how the card wrote them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum ThisCardForm {
    /// The 2024 Oracle shortened self-reference. [CR#201.5c] treats a card's
    /// shortened name as though it used the card's full name.
    AbbreviatedName,
    /// The card's full name.
    FullName,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
pub struct OracleSymbol(Arc<str>);

impl OracleSymbol {
    #[must_use]
    pub fn new(symbol: impl Into<Arc<str>>) -> Option<Self> {
        let symbol = symbol.into();
        let body = symbol.strip_prefix('{')?.strip_suffix('}')?;
        (!body.is_empty()
            && body
                .chars()
                .all(|character| !character.is_whitespace() && !matches!(character, '{' | '}')))
        .then_some(Self(symbol))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub struct NumberLiteral {
    pub value: i32,
    pub numeral: Numeral,
}

pub(crate) const fn is_valid_degree_measure_number(number: NumberLiteral) -> bool {
    matches!(number.numeral, Numeral::Cardinal | Numeral::Arabic(false))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum ScalarSign {
    None,
    Plus,
    Minus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum ScalarValue {
    Integer(u32),
    X,
    Star,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub struct SignedScalar {
    pub sign: ScalarSign,
    pub value: ScalarValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub struct PowerToughness {
    pub power: SignedScalar,
    pub toughness: SignedScalar,
}

/// The sound a number is *read* with, independent of the notation it is
/// written in: `8` and `eight` are both read "eight" (vowel onset), `0` and
/// `zero` both "zero" (consonant onset).
///
/// Derived from the cardinal spelling — the pronunciation — rather than from a
/// digit table, so it is total over every value and needs no witness-by-witness
/// extension. One exception is required, mirroring the vocabulary's
/// spelling-plus-override idiom (`Vocabulary::initial_sound`,
/// word.rs:1430-1437): `one` is spelled with a vowel but read /w/, so `1`,
/// `100` ("one hundred") and `1000` ("one thousand") are consonant onsets.
fn number_initial_sound(value: u32) -> InitialSound {
    let Ok(value) = i32::try_from(value) else {
        return InitialSound::Consonant;
    };
    let spelling = crate::numeral::Numeral::Cardinal.format(value);
    let head = spelling
        .split([' ', '-', ','])
        .next()
        .unwrap_or(spelling.as_str());
    if head == "one" {
        return InitialSound::Consonant;
    }
    crate::word::surface_initial_sound(head)
}

impl SignedScalar {
    /// The sound this scalar is read with. A sign is read `plus`/`minus`, so a
    /// signed scalar has a consonant onset whatever its value — `a +1/+1
    /// counter`, never `an`.
    #[must_use]
    pub fn initial_sound(self) -> InitialSound {
        if !matches!(self.sign, ScalarSign::None) {
            return InitialSound::Consonant;
        }
        match self.value {
            // `X` is read "ex".
            ScalarValue::X => InitialSound::Vowel,
            // `*` is read "star". No supported face prints an article before a
            // `*` power (`a */*` and `an */*` are both 0 occurrences), so this
            // is a defaulted, unwitnessed choice.
            ScalarValue::Star => InitialSound::Consonant,
            ScalarValue::Integer(value) => number_initial_sound(value),
        }
    }
}

impl PowerToughness {
    /// The sound the pair is read with — its power, the pair's leftmost
    /// surface.
    #[must_use]
    pub fn initial_sound(self) -> InitialSound {
        self.power.initial_sound()
    }
}

/// The comparative word heading an `N or …` quantity floor or ceiling. Every
/// distinction the surface draws — `more`/`greater` above the bound,
/// `fewer`/`less` at or below it — is preserved here so the renderer replays
/// the exact word rather than guessing one from the bound's direction.
///
/// **Re-measured, field KEPT** (surface-fact sweep-residue, 2026-07-30
/// measurement round). Direction (`more`/`greater` vs `fewer`/`less`) is
/// semantic and stays out of scope. The earlier refutation's cited minimal
/// pair — `total power 2 or more` (Adventurer's Airship) vs `total power 8
/// or greater` (Atarka Beastbreaker) — does **not** hold up: Adventurer's
/// Airship's "2 or more" is inside Crew's reminder text, which is stripped
/// before parsing and never reaches this field at all, so the two witnesses
/// were never a same-frame pair to begin with.
///
/// The *frame* hypothesis this round was asked to test (qualifier `with
/// power N or …` vs predicate `have total power N or …`) is also wrong, but
/// informatively: both frames behave **identically**. On the supported
/// corpus, every postnominal quantity complement attached to a scalar
/// characteristic head — `power` (395 witnesses), `toughness` (46), mana
/// `value` (443) — takes `greater`/`less` with **zero** exceptions,
/// regardless of whether the phrase is a qualifier or a predicate
/// complement. Corroborating the zero-occurrence fact this round confirmed:
/// `with power N or more` never occurs on the corpus; `with power N or
/// greater` is well attested. So frame is not the discriminator — **head
/// class** is, and three classes emerge:
/// - scalar characteristics (`power`, `toughness`, mana `value`): always
///   `greater`/`less`, no exceptions found;
/// - count nouns (`creatures`, `cards`, `lands`, counters, tokens, …): always
///   `more`/`fewer`;
/// - mass magnitudes (`life`, `damage`, `mana`): a third, hybrid pair — `more`
///   for the floor, `less` for the ceiling — never `greater`, essentially never
///   `fewer`.
///
/// This is close but not exact: Lesser Werewolf prints "this creature's
/// power is 1 or **more**" where every other `power`-headed copular
/// predicate in the corpus (e.g. Bloodshot Trainee's "this creature's power
/// is 4 or **greater**") uses `greater`. Gore Vassal's "toughness is 1 or
/// greater" rules out a floor-value-driven exception, so this one witness
/// looks like a genuine, isolated inconsistency in printed Oracle text
/// rather than a rule. Headless copular predicates (`X is N or greater`,
/// `the number of Y is N or greater`) also pattern with the scalar class
/// even with no noun in the complement itself, which is exactly why this
/// round could not turn the head-class rule into a safe derivation: it
/// requires tracing the antecedent across a clause boundary the renderer
/// does not currently carry context for. Field stays stored.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum ComparativeWord {
    Fewer,
    Greater,
    Less,
    More,
}

impl ComparativeWord {
    #[must_use]
    pub const fn spelling(self) -> &'static str {
        match self {
            Self::Fewer => "fewer",
            Self::Greater => "greater",
            Self::Less => "less",
            Self::More => "more",
        }
    }
}

/// A number position in a quantity that may hold either a literal or the
/// game's variable `X`. `X` on a card is a placeholder for a number to be
/// determined ([CR#107.3]) — never the Roman numeral ten, even though
/// `Numeral::Roman` is the only notation whose canonical spelling of a value
/// is `X`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum QuantityValue {
    Literal(NumberLiteral),
    Variable,
}

impl QuantityValue {
    /// Whether this position holds the literal one. `Variable` never does:
    /// `X` may resolve to any number, so it can never license a bare
    /// singular head.
    #[must_use]
    pub const fn is_one(self) -> bool {
        matches!(self, Self::Literal(number) if number.value == 1)
    }

    /// The literal this position holds, if any.
    #[must_use]
    pub const fn literal(self) -> Option<NumberLiteral> {
        match self {
            Self::Literal(number) => Some(number),
            Self::Variable => None,
        }
    }
}

/// A quantity admitted by Q01's generated declarations.
///
/// The inner semantic kind is public for inspection, while [`Quantity`]'s
/// private field keeps construction behind the checked `try_*` functions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub struct Quantity(QuantityKind);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum QuantityKind {
    Exact(NumberLiteral),
    AtLeast(QuantityValue),
    OrComparison(QuantityValue, ComparativeWord),
    Or(NumberLiteral, NumberLiteral),
    UpTo(QuantityValue),
    MoreThan(QuantityValue),
    FewerThan(QuantityValue),
    X,
    Both,
    ThatMany,
    ThatMuch,
}

impl Quantity {
    pub(crate) const fn from_kind(kind: QuantityKind) -> Self {
        Self(kind)
    }

    pub(crate) const fn unchecked_exact(number: NumberLiteral) -> Self {
        Self::from_kind(QuantityKind::Exact(number))
    }

    pub(crate) const fn unchecked_at_least(value: QuantityValue) -> Self {
        Self::from_kind(QuantityKind::AtLeast(value))
    }

    pub(crate) const fn unchecked_or_comparison(
        value: QuantityValue,
        word: ComparativeWord,
    ) -> Self {
        Self::from_kind(QuantityKind::OrComparison(value, word))
    }

    pub(crate) const fn unchecked_or(first: NumberLiteral, second: NumberLiteral) -> Self {
        Self::from_kind(QuantityKind::Or(first, second))
    }

    pub(crate) const fn unchecked_up_to(value: QuantityValue) -> Self {
        Self::from_kind(QuantityKind::UpTo(value))
    }

    pub(crate) const fn unchecked_more_than(value: QuantityValue) -> Self {
        Self::from_kind(QuantityKind::MoreThan(value))
    }

    pub(crate) const fn unchecked_fewer_than(value: QuantityValue) -> Self {
        Self::from_kind(QuantityKind::FewerThan(value))
    }

    pub(crate) const fn unchecked_x() -> Self {
        Self::from_kind(QuantityKind::X)
    }

    pub(crate) const fn unchecked_both() -> Self {
        Self::from_kind(QuantityKind::Both)
    }

    pub(crate) const fn unchecked_that_many() -> Self {
        Self::from_kind(QuantityKind::ThatMany)
    }

    pub(crate) const fn unchecked_that_much() -> Self {
        Self::from_kind(QuantityKind::ThatMuch)
    }

    /// Returns the validated semantic kind.
    #[must_use]
    pub const fn kind(self) -> QuantityKind {
        self.0
    }

    /// Builds an exact quantity through its generated invariant checks.
    ///
    /// # Errors
    ///
    /// Returns a declaration violation when `number` is not admitted by the
    /// generated exact-quantity declaration.
    pub fn try_exact(
        number: NumberLiteral,
    ) -> Result<Self, deckmaste_construction_compiler::runtime::DeclarationViolation> {
        crate::constructions::quantity::build_quantity_exact(number)
    }

    /// Builds an `at least` quantity through its generated invariant checks.
    ///
    /// # Errors
    ///
    /// Returns a declaration violation when `value` is not admitted by the
    /// generated lower-bound declaration.
    pub fn try_at_least(
        value: QuantityValue,
    ) -> Result<Self, deckmaste_construction_compiler::runtime::DeclarationViolation> {
        crate::constructions::quantity::build_quantity_at_least(value, None)
    }

    /// Builds an `N or comparative` quantity through its generated checks.
    ///
    /// # Errors
    ///
    /// Returns a declaration violation when the value/comparative pair is not
    /// admitted by the generated comparative-bound declaration.
    pub fn try_or_comparison(
        value: QuantityValue,
        word: ComparativeWord,
    ) -> Result<Self, deckmaste_construction_compiler::runtime::DeclarationViolation> {
        crate::constructions::quantity::build_quantity_at_least(value, Some(word))
    }

    /// Builds an `N or M` quantity through its generated invariant checks.
    ///
    /// # Errors
    ///
    /// Returns a declaration violation when either literal is not admitted by
    /// the generated disjunctive-quantity declaration.
    pub fn try_or(
        first: NumberLiteral,
        second: NumberLiteral,
    ) -> Result<Self, deckmaste_construction_compiler::runtime::DeclarationViolation> {
        crate::constructions::quantity::build_quantity_or(first, second)
    }

    /// Builds an `up to` quantity through its generated invariant checks.
    ///
    /// # Errors
    ///
    /// Returns a declaration violation when `value` is not admitted by the
    /// generated upper-bound declaration.
    pub fn try_up_to(
        value: QuantityValue,
    ) -> Result<Self, deckmaste_construction_compiler::runtime::DeclarationViolation> {
        crate::constructions::quantity::build_quantity_up_to(value)
    }

    /// Builds a `more than` quantity through its generated invariant checks.
    ///
    /// # Errors
    ///
    /// Returns a declaration violation when `value` is not admitted by the
    /// generated strict-lower-bound declaration.
    pub fn try_more_than(
        value: QuantityValue,
    ) -> Result<Self, deckmaste_construction_compiler::runtime::DeclarationViolation> {
        crate::constructions::quantity::build_quantity_more_than(value)
    }

    /// Builds a `fewer than` quantity through its generated invariant checks.
    ///
    /// # Errors
    ///
    /// Returns a declaration violation when `value` is not admitted by the
    /// generated strict-upper-bound declaration.
    pub fn try_fewer_than(
        value: QuantityValue,
    ) -> Result<Self, deckmaste_construction_compiler::runtime::DeclarationViolation> {
        crate::constructions::quantity::build_quantity_fewer_than(value)
    }

    /// Builds the variable `X` quantity through its generated constructor.
    ///
    /// # Errors
    ///
    /// Returns a declaration violation if the generated `X` declaration
    /// rejects construction.
    pub fn try_x() -> Result<Self, deckmaste_construction_compiler::runtime::DeclarationViolation> {
        crate::constructions::quantity::build_quantity_x()
    }

    /// Builds `both` through its generated constructor.
    ///
    /// # Errors
    ///
    /// Returns a declaration violation if the generated `both` declaration
    /// rejects construction.
    pub fn try_both() -> Result<Self, deckmaste_construction_compiler::runtime::DeclarationViolation>
    {
        crate::constructions::quantity::build_quantity_both()
    }

    /// Builds `that many` through its generated constructor.
    ///
    /// # Errors
    ///
    /// Returns a declaration violation if the generated `that many`
    /// declaration rejects construction.
    pub fn try_that_many()
    -> Result<Self, deckmaste_construction_compiler::runtime::DeclarationViolation> {
        crate::constructions::quantity::build_quantity_that_many()
    }

    /// Builds `that much` through its generated constructor.
    ///
    /// # Errors
    ///
    /// Returns a declaration violation if the generated `that much`
    /// declaration rejects construction.
    pub fn try_that_much()
    -> Result<Self, deckmaste_construction_compiler::runtime::DeclarationViolation> {
        crate::constructions::quantity::build_quantity_that_much()
    }

    #[must_use]
    pub fn noun_cardinality(self) -> NounCardinality {
        crate::constructions::quantity::noun_cardinality(self)
    }
}

/// A determiner admitted by D01's generated declarations.
///
/// The semantic kind is inspectable, while the private wrapper keeps invalid
/// pronoun and possessive-nominal states behind the checked builders.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct Determiner(DeterminerKind);

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub enum DeterminerKind {
    The,
    Each,
    Another,
    /// The indefinite article slot (`a`/`an`). Carries no word: on the
    /// supported corpus the choice between them is exactly the initial sound
    /// of the material that follows — a consonant sound takes `a`, a vowel
    /// sound takes `an` — measured with 0 exceptions across all 31685
    /// supported faces (the renderer's existing `nominal_initial_sound`
    /// validation already proved `expected == actual` for every supported
    /// face before this field was removed). The renderer derives the word
    /// from [`NominalPhrase`]'s or [`CoordinatedNominalPhrase`]'s own initial
    /// sound rather than the AST carrying a field that could contradict it.
    Indefinite,
    Demonstrative(Demonstrative),
    Target(Option<Quantity>),
    Quantity(Quantity),
    Possessive(Possessor),
    All,
    Any,
    No,
}

/// A closed lexical determiner identity accepted by D01's generated builder.
///
/// These values contain no independently writable construction state. Target,
/// quantity, and noun-phrase possessive shapes use their own checked builders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClosedDeterminer {
    The,
    Each,
    Another,
    Indefinite,
    Demonstrative(Demonstrative),
    PossessivePronoun(Pronoun),
    All,
    Any,
    No,
}

// Grammar lexical meanings are hash-consed. Most determiners are closed,
// nonrecursive syntax values and hash structurally; noun-phrase possessors are
// built during lowering rather than scanned, so a shared tag is sufficient for
// that recursive branch. Hash collisions are permitted, while equal values
// still necessarily produce equal hashes.
impl Hash for Determiner {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(&self.0).hash(state);
        match &self.0 {
            DeterminerKind::Demonstrative(demonstrative) => demonstrative.hash(state),
            DeterminerKind::Target(quantity) => quantity.hash(state),
            DeterminerKind::Quantity(quantity) => quantity.hash(state),
            DeterminerKind::Possessive(Possessor::Pronoun(pronoun)) => {
                0_u8.hash(state);
                pronoun.hash(state);
            }
            DeterminerKind::Possessive(Possessor::NounPhrase(_)) => 1_u8.hash(state),
            DeterminerKind::Indefinite
            | DeterminerKind::The
            | DeterminerKind::Each
            | DeterminerKind::Another
            | DeterminerKind::All
            | DeterminerKind::Any
            | DeterminerKind::No => {}
        }
    }
}

impl Determiner {
    pub(crate) const fn from_kind(kind: DeterminerKind) -> Self {
        Self(kind)
    }

    const SIMPLE_FORMS: &'static [(ClosedDeterminer, &'static str)] = &[
        (ClosedDeterminer::The, "the"),
        (ClosedDeterminer::Each, "each"),
        (ClosedDeterminer::Another, "another"),
        (ClosedDeterminer::All, "all"),
        (ClosedDeterminer::Any, "any"),
        (ClosedDeterminer::No, "no"),
    ];

    pub(crate) fn from_spelling(surface: &str) -> Option<Self> {
        Self::SIMPLE_FORMS
            .iter()
            .find_map(|(identity, spelling)| {
                surface.eq_ignore_ascii_case(spelling).then_some(*identity)
            })
            .or_else(|| {
                IndefiniteArticle::from_spelling(surface).map(|_| ClosedDeterminer::Indefinite)
            })
            .or_else(|| Demonstrative::from_spelling(surface).map(ClosedDeterminer::Demonstrative))
            .or_else(|| {
                Pronoun::from_possessive_spelling(surface).map(ClosedDeterminer::PossessivePronoun)
            })
            .and_then(|identity| {
                crate::constructions::determiner::build_determiner_closed(identity).ok()
            })
    }

    /// Returns the validated semantic kind.
    #[must_use]
    pub const fn kind(&self) -> &DeterminerKind {
        &self.0
    }

    #[must_use]
    pub fn noun_cardinality(&self) -> NounCardinality {
        match self.kind() {
            DeterminerKind::Each
            | DeterminerKind::Another
            | DeterminerKind::Indefinite
            | DeterminerKind::Target(None) => NounCardinality::SingularCount,
            // The singular demonstratives determine a singular count noun (`that
            // creature`) or a mass one (`that damage`); only `these`/`those` are
            // barred from mass. This is also the parser's cardinality table;
            // grammar features are keyed directly on this syntax value.
            DeterminerKind::Demonstrative(Demonstrative::This | Demonstrative::That) => {
                NounCardinality::SingularOrMass
            }
            DeterminerKind::Demonstrative(Demonstrative::These | Demonstrative::Those) => {
                NounCardinality::PluralCount
            }
            DeterminerKind::Target(Some(quantity)) => match quantity.noun_cardinality() {
                NounCardinality::SingularOrMass => NounCardinality::SingularCount,
                NounCardinality::PluralOrMass => NounCardinality::PluralCount,
                cardinality => cardinality,
            },
            DeterminerKind::Quantity(quantity) => quantity.noun_cardinality(),
            DeterminerKind::All => NounCardinality::PluralOrMass,
            DeterminerKind::The
            | DeterminerKind::Possessive(_)
            | DeterminerKind::Any
            | DeterminerKind::No => NounCardinality::Unconstrained,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum IndefiniteArticle {
    A,
    An,
}

impl IndefiniteArticle {
    const FORMS: &'static [(Self, &'static str)] = &[(Self::A, "a"), (Self::An, "an")];

    pub(crate) fn from_spelling(surface: &str) -> Option<Self> {
        Self::FORMS.iter().find_map(|(article, spelling)| {
            surface.eq_ignore_ascii_case(spelling).then_some(*article)
        })
    }

    pub(crate) fn spelling(self) -> &'static str {
        Self::FORMS
            .iter()
            .find_map(|(article, spelling)| (*article == self).then_some(*spelling))
            .expect("every indefinite article has one spelling")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum Demonstrative {
    This,
    That,
    These,
    Those,
}

impl Demonstrative {
    const FORMS: &'static [(Self, &'static str)] = &[
        (Self::This, "this"),
        (Self::That, "that"),
        (Self::These, "these"),
        (Self::Those, "those"),
    ];

    pub(crate) fn from_spelling(surface: &str) -> Option<Self> {
        Self::FORMS.iter().find_map(|(demonstrative, spelling)| {
            surface
                .eq_ignore_ascii_case(spelling)
                .then_some(*demonstrative)
        })
    }

    pub(crate) fn spelling(self) -> &'static str {
        Self::FORMS
            .iter()
            .find_map(|(demonstrative, spelling)| (*demonstrative == self).then_some(*spelling))
            .expect("every demonstrative has one spelling")
    }

    pub(crate) const fn number(self) -> crate::word::Number {
        match self {
            Self::This | Self::That => crate::word::Number::Singular,
            Self::These | Self::Those => crate::word::Number::Plural,
        }
    }
}

/// A pronominal or noun-phrase possessor.
///
/// Both alternatives are semantically valid on their own. The checked D01
/// boundary remains responsible for deciding where a possessor may enter a
/// determiner or noun phrase.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub enum Possessor {
    Pronoun(Pronoun),
    NounPhrase(Box<NounPhrase>),
}

/// Compatibility name for the inherent-realization noun-cardinality feature.
pub use crate::features::NounCardinality;

/// A validated noun phrase with a sealed representation.
///
/// Use the checked builders in [`crate::noun_phrase`] to construct values and
/// [`Self::kind`] to inspect them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NounPhrase {
    repr: NounPhraseRepr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum NounPhraseRepr {
    Nominal(NominalPhrase),
    Pronoun {
        pronoun: Pronoun,
        case: PronounCase,
    },
    Possessive(Possessor),
    Demonstrative(Demonstrative),
    Quantity(Quantity),
    ThisCard(ThisCardForm),
    Partitive(PartitiveNounPhrase),
    /// Coordination inside one determiner's scope (`target artifact or
    /// enchantment`).
    CoordinatedNominal(CoordinatedNominalPhrase),
    Coordinated(CoordinatedNounPhrase),
    /// A trailing set exclusion over a complete noun phrase. The wrapper can
    /// scope over a coordinated included set while the excluded set remains
    /// any ordinary noun phrase.
    SetException(SetExceptionNounPhrase),
    /// An arithmetic value expression combining value operands into a new
    /// numeric value.
    Arithmetic(ArithmeticValue),
}

/// A read-only projection of a validated [`NounPhrase`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NounPhraseKind<'a> {
    Nominal(&'a NominalPhrase),
    Pronoun { pronoun: Pronoun, case: PronounCase },
    Possessive(&'a Possessor),
    Demonstrative(Demonstrative),
    Quantity(Quantity),
    ThisCard(ThisCardForm),
    Partitive(&'a PartitiveNounPhrase),
    CoordinatedNominal(&'a CoordinatedNominalPhrase),
    Coordinated(&'a CoordinatedNounPhrase),
    SetException(&'a SetExceptionNounPhrase),
    Arithmetic(&'a ArithmeticValue),
}

impl NounPhrase {
    /// Returns a read-only projection of this noun phrase.
    #[must_use]
    pub const fn kind(&self) -> NounPhraseKind<'_> {
        match &self.repr {
            NounPhraseRepr::Nominal(value) => NounPhraseKind::Nominal(value),
            NounPhraseRepr::Pronoun { pronoun, case } => NounPhraseKind::Pronoun {
                pronoun: *pronoun,
                case: *case,
            },
            NounPhraseRepr::Possessive(value) => NounPhraseKind::Possessive(value),
            NounPhraseRepr::Demonstrative(value) => NounPhraseKind::Demonstrative(*value),
            NounPhraseRepr::Quantity(value) => NounPhraseKind::Quantity(*value),
            NounPhraseRepr::ThisCard(value) => NounPhraseKind::ThisCard(*value),
            NounPhraseRepr::Partitive(value) => NounPhraseKind::Partitive(value),
            NounPhraseRepr::CoordinatedNominal(value) => NounPhraseKind::CoordinatedNominal(value),
            NounPhraseRepr::Coordinated(value) => NounPhraseKind::Coordinated(value),
            NounPhraseRepr::SetException(value) => NounPhraseKind::SetException(value),
            NounPhraseRepr::Arithmetic(value) => NounPhraseKind::Arithmetic(value),
        }
    }

    pub(crate) const fn from_pronoun_declaration(pronoun: Pronoun, case: PronounCase) -> Self {
        Self {
            repr: NounPhraseRepr::Pronoun { pronoun, case },
        }
    }
}

impl NounPhrase {
    pub(crate) const fn from_nominal_declaration(value: NominalPhrase) -> Self {
        Self {
            repr: NounPhraseRepr::Nominal(value),
        }
    }

    pub(crate) const fn from_possessive_declaration(value: Possessor) -> Self {
        Self {
            repr: NounPhraseRepr::Possessive(value),
        }
    }

    pub(crate) const fn from_demonstrative_declaration(value: Demonstrative) -> Self {
        Self {
            repr: NounPhraseRepr::Demonstrative(value),
        }
    }

    pub(crate) const fn from_quantity_declaration(value: Quantity) -> Self {
        Self {
            repr: NounPhraseRepr::Quantity(value),
        }
    }

    pub(crate) const fn from_this_card_declaration(value: ThisCardForm) -> Self {
        Self {
            repr: NounPhraseRepr::ThisCard(value),
        }
    }

    pub(crate) const fn from_partitive_declaration(value: PartitiveNounPhrase) -> Self {
        Self {
            repr: NounPhraseRepr::Partitive(value),
        }
    }

    pub(crate) const fn from_coordinated_nominal_declaration(
        value: CoordinatedNominalPhrase,
    ) -> Self {
        Self {
            repr: NounPhraseRepr::CoordinatedNominal(value),
        }
    }

    pub(crate) const fn from_coordination_declaration(value: CoordinatedNounPhrase) -> Self {
        Self {
            repr: NounPhraseRepr::Coordinated(value),
        }
    }

    pub(crate) const fn from_set_exception_declaration(value: SetExceptionNounPhrase) -> Self {
        Self {
            repr: NounPhraseRepr::SetException(value),
        }
    }

    pub(crate) const fn from_arithmetic_declaration(value: ArithmeticValue) -> Self {
        Self {
            repr: NounPhraseRepr::Arithmetic(value),
        }
    }
}

#[allow(
    dead_code,
    non_snake_case,
    reason = "generated projection adapters select these stable declaration variant names"
)]
impl NounPhrase {
    pub(crate) const fn Nominal(value: NominalPhrase) -> Self {
        Self::from_nominal_declaration(value)
    }

    pub(crate) const fn Possessive(value: Possessor) -> Self {
        Self::from_possessive_declaration(value)
    }

    pub(crate) const fn Demonstrative(value: Demonstrative) -> Self {
        Self::from_demonstrative_declaration(value)
    }

    pub(crate) const fn Quantity(value: Quantity) -> Self {
        Self::from_quantity_declaration(value)
    }

    pub(crate) const fn ThisCard(value: ThisCardForm) -> Self {
        Self::from_this_card_declaration(value)
    }

    pub(crate) const fn Partitive(value: PartitiveNounPhrase) -> Self {
        Self::from_partitive_declaration(value)
    }

    pub(crate) const fn CoordinatedNominal(value: CoordinatedNominalPhrase) -> Self {
        Self::from_coordinated_nominal_declaration(value)
    }

    pub(crate) const fn Coordinated(value: CoordinatedNounPhrase) -> Self {
        Self::from_coordination_declaration(value)
    }

    pub(crate) const fn SetException(value: SetExceptionNounPhrase) -> Self {
        Self::from_set_exception_declaration(value)
    }

    pub(crate) const fn Arithmetic(value: ArithmeticValue) -> Self {
        Self::from_arithmetic_declaration(value)
    }
}

impl serde::Serialize for NounPhrase {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStructVariant;

        match self.kind() {
            NounPhraseKind::Nominal(value) => {
                serializer.serialize_newtype_variant("NounPhrase", 0, "Nominal", value)
            }
            NounPhraseKind::Pronoun { pronoun, case } => {
                let mut variant =
                    serializer.serialize_struct_variant("NounPhrase", 1, "Pronoun", 2)?;
                variant.serialize_field("pronoun", &pronoun)?;
                variant.serialize_field("case", &case)?;
                variant.end()
            }
            NounPhraseKind::Possessive(value) => {
                serializer.serialize_newtype_variant("NounPhrase", 2, "Possessive", value)
            }
            NounPhraseKind::Demonstrative(value) => {
                serializer.serialize_newtype_variant("NounPhrase", 3, "Demonstrative", &value)
            }
            NounPhraseKind::Quantity(value) => {
                serializer.serialize_newtype_variant("NounPhrase", 4, "Quantity", &value)
            }
            NounPhraseKind::ThisCard(value) => {
                serializer.serialize_newtype_variant("NounPhrase", 5, "ThisCard", &value)
            }
            NounPhraseKind::Partitive(value) => {
                serializer.serialize_newtype_variant("NounPhrase", 6, "Partitive", value)
            }
            NounPhraseKind::CoordinatedNominal(value) => {
                serializer.serialize_newtype_variant("NounPhrase", 7, "CoordinatedNominal", value)
            }
            NounPhraseKind::Coordinated(value) => {
                serializer.serialize_newtype_variant("NounPhrase", 8, "Coordinated", value)
            }
            NounPhraseKind::SetException(value) => {
                serializer.serialize_newtype_variant("NounPhrase", 9, "SetException", value)
            }
            NounPhraseKind::Arithmetic(value) => {
                serializer.serialize_newtype_variant("NounPhrase", 10, "Arithmetic", value)
            }
        }
    }
}

/// A binary set exclusion, not a coordination list. The generated bare and
/// `except for` declarations each provide plain and comma forms, so `comma`
/// stores the independent [`Comma`] surface witness that precedes the marker.
/// No other field on this node can recover that punctuation choice.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct SetExceptionNounPhrase {
    pub included: Box<NounPhrase>,
    pub marker: SetExceptionMarker,
    pub comma: Comma,
    pub excluded: Box<NounPhrase>,
}

/// An arithmetic combination of value operands, carried structurally so the
/// renderer replays the operator rather than deriving it. Operands are
/// themselves noun-phrase values (a nominal such as `the number of lands you
/// control`, or a bare number).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub enum ArithmeticValue {
    /// `<value> minus <value>` — subtraction, in either operand order
    /// (`3 minus the number of cards in their hand`, `its toughness minus 1`).
    Minus {
        left: Box<NounPhrase>,
        right: Box<NounPhrase>,
    },
    /// `half <value>` with an optional `, rounded up` / `, rounded down` rider
    /// (`half your life total`, `half the number of Forests you control,
    /// rounded down`).
    Half {
        value: Box<NounPhrase>,
        rounding: Option<Rounding>,
    },
}

/// The rounding rider on a `half` value expression. Magic rounds a fractional
/// result up or down as the surface directs; the direction is carried so the
/// renderer replays it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum Rounding {
    Up,
    Down,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct PartitiveNounPhrase {
    pub head: PartitiveHead,
    pub whole: Box<NounPhrase>,
}

/// The quantifier heading a partitive `<head> of <whole>`. `Quantity` covers
/// the counted partitives (`one of them`, `more than one of X`); `Each` is the
/// distributive `each of X`, whose determiner is not a count quantity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum PartitiveHead {
    Quantity(Quantity),
    Each,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct NominalPhraseCoordination {
    pub conjunction: Option<Conjunction>,
    pub phrase: NominalPhrase,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct NounPhraseCoordination {
    /// The connective introducing this member: `None` on the asyndetic
    /// comma-separated interior members of an Oxford head list (`enchantment,`
    /// in `artifact, enchantment, or land`); `Some` on a bare `and`/`or`/`plus`
    /// member and on the final Oxford member.
    pub conjunction: Option<Conjunction>,
    pub phrase: NounPhrase,
}

/// Compatibility name for the selection-stratum conjunction feature.
pub use crate::features::Conjunction as NounPhraseConjunction;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct NominalPhrase {
    determiner: Option<Determiner>,
    modifiers: Vec<NominalModifier>,
    head: NounInstance,
    complements: Vec<NominalComplement>,
}

impl NominalPhrase {
    /// Complete owner-local projection used by declaration-generated lens
    /// code. Keeping this capability beside the private fields lets generated
    /// builders rebuild validated values without widening field visibility.
    const fn from_projection_parts(
        determiner: Option<Determiner>,
        modifiers: Vec<NominalModifier>,
        head: NounInstance,
        complements: Vec<NominalComplement>,
    ) -> Self {
        Self {
            determiner,
            modifiers,
            head,
            complements,
        }
    }

    fn into_projection_parts(
        self,
    ) -> (
        Option<Determiner>,
        Vec<NominalModifier>,
        NounInstance,
        Vec<NominalComplement>,
    ) {
        (self.determiner, self.modifiers, self.head, self.complements)
    }

    // Sealed declaration capability: these mutable views are private to the
    // owner and its nominal-declaration child. Generated public builders
    // perform the final inverse validation.
    fn declaration_modifiers_mut(&mut self) -> &mut Vec<NominalModifier> {
        &mut self.modifiers
    }

    fn declaration_complements_mut(&mut self) -> &mut Vec<NominalComplement> {
        &mut self.complements
    }

    /// Deliberate test-only escape hatch for constructing malformed or
    /// independent expected values without widening the production boundary.
    #[cfg(test)]
    pub(crate) const fn test_from_projection_parts(
        determiner: Option<Determiner>,
        modifiers: Vec<NominalModifier>,
        head: NounInstance,
        complements: Vec<NominalComplement>,
    ) -> Self {
        Self::from_projection_parts(determiner, modifiers, head, complements)
    }

    /// Re-label keyword-ability catalog atoms after a `named` modifier opens
    /// proper-name interior. This mutation belongs beside the private owner;
    /// callers cannot obtain mutable field views.
    pub(crate) fn open_name_interior(&mut self) {
        fn detach(noun: &mut NounInstance) {
            let inner = noun.noun_mut();
            if let Noun::Catalog(atom) = inner
                && atom.kind == CatalogKind::KeywordAbility
            {
                *inner = Noun::Opaque(OpaqueLexeme::new(atom.spelling()));
            }
        }

        detach(&mut self.head);
        for modifier in &mut self.modifiers {
            if let NominalModifier::Noun { noun, .. } = modifier {
                detach(noun);
            }
        }
    }

    /// Builds a bare nominal through the generated noun-head declaration.
    ///
    /// # Errors
    ///
    /// Returns a declaration violation when the generated nominal base
    /// rejects `head`.
    pub fn try_from_noun(
        head: NounInstance,
    ) -> Result<Self, deckmaste_construction_compiler::runtime::DeclarationViolation> {
        crate::constructions::nominal::build_nominal_noun(head)
    }

    /// The determiner selected for this nominal, if any.
    #[must_use]
    pub const fn determiner(&self) -> Option<&Determiner> {
        self.determiner.as_ref()
    }

    /// The ordered attributive modifiers preceding the nominal head.
    #[must_use]
    pub fn modifiers(&self) -> &[NominalModifier] {
        &self.modifiers
    }

    /// The noun that heads this nominal.
    #[must_use]
    pub const fn head(&self) -> &NounInstance {
        &self.head
    }

    /// The ordered complements following the nominal head.
    #[must_use]
    pub fn complements(&self) -> &[NominalComplement] {
        &self.complements
    }
}

// The declaration module is an owner child so its generated lens and adapter
// code can use the private projection capability without exposing it to crate
// siblings.
#[path = "../constructions/nominal.rs"]
pub(crate) mod nominal_constructions;

// The determiner declaration owns checked projections over the same private
// nominal storage as the nominal declaration.
#[path = "../constructions/determiner.rs"]
pub(crate) mod determiner_constructions;

/// The productive `non-` polarity of a nominal modifier. `land card` and
/// `nonland card` are the *same* outer modifier node — an attributive
/// [`NominalModifier::Noun`] over the `land` catalog atom — differing only in
/// this flag; negation is never a mechanism or category change. Only the
/// adjective and noun modifier kinds can carry it, because those are the only
/// bases the surface negates (card/subtypes, supertypes, colors, participles,
/// and vocabulary adjectives/nouns); quantities and power/toughness never do.
///
/// On the supported corpus the hyphenation rule is exact and derivable from
/// the base's capitalization alone: solid `nonland`/`nonblack` bases are
/// always lowercase, hyphenated `non-Human` bases are always capitalized, so
/// the glyph is never recorded structurally — the renderer derives it at
/// render time instead.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum Polarity {
    Positive,
    Negative,
}

impl Polarity {
    #[must_use]
    pub const fn is_negative(self) -> bool {
        matches!(self, Self::Negative)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub enum NominalModifier {
    Adjective {
        polarity: Polarity,
        phrase: AdjectivePhrase,
    },
    Noun {
        polarity: Polarity,
        noun: NounInstance,
    },
    Quantity(Quantity),
    PowerToughness(PowerToughness),
    /// The `declare attackers` / `declare blockers` formative of a combat step
    /// name [CR#508.1,509.1]. The `declare` formative is invariant and belongs
    /// to the shape itself, exactly as `non-` belongs to the negated-modifier
    /// shape; `participants` carries the varying lexeme so the renderer
    /// reproduces the name from structure and the vocabulary, never from a
    /// string table.
    CombatStepName {
        participants: NounInstance,
    },
    /// A coordinated group of attributive modifiers filling a single modifier
    /// slot: `white and blue` (Ashiok, Nightmare Muse), `artifact, creature,
    /// and land` (Warp World), `white and/or blue` (Amphibious Kavu). It is
    /// one outer [`NominalModifier`] node — coordination is a structural
    /// fact about modifiers generally, not a per-mechanism list — so its
    /// conjuncts are ordinary [`Self::Adjective`]/[`Self::Noun`] modifiers,
    /// each keeping its own [`Polarity`]. The group binds tighter than the
    /// rest of the modifier stack: in `white and blue Bird creature token`
    /// only `white and blue` is coordinated, with `Bird` and `creature`
    /// following as separate modifiers.
    Coordinated(CoordinatedModifier),
}

/// A coordinated list of attributive modifiers. The first conjunct and each
/// continuation is an [`Self::Adjective`]/[`Self::Noun`] modifier; the
/// coordination is recorded (comma and optional conjunction per member) so the
/// renderer replays the exact surface list. Mirrors the exception-rider list
/// idiom.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct CoordinatedModifier {
    pub first: Box<NominalModifier>,
    pub rest: Vec<ModifierCoordination>,
}

/// One non-first member of an attributive modifier coordination.
///
/// No `comma` flag: on the supported corpus the serial comma is exactly
/// determined by member count and connective — absent only on an asyndetic
/// interior member (which always takes one) or on a connective member of a
/// two-member list (which never does), present on every connective member of
/// a three-or-more-member (Oxford) list — measured with 0 exceptions across
/// all 31685 supported faces. The renderer derives it from
/// [`CoordinatedModifier::rest`] rather than the AST carrying a field that
/// could contradict it, following the same idiom as
/// [`PrepositionalPhraseCoordination`] and
/// [`TriggerConditionCoordination`](crate::syntax::TriggerConditionCoordination).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ModifierCoordination {
    /// The connective introducing this member: `None` on the asyndetic
    /// comma-separated members of an Oxford list (`artifact,` in `artifact,
    /// creature, and land`); `Some` on the final `and`/`or`/`and/or` member.
    pub conjunction: Option<Conjunction>,
    pub modifier: NominalModifier,
}

mod adjective_storage {
    use super::Adjective;
    use super::AdjectiveComparisonClass;
    use super::AdjectiveComplement;
    use super::CardOrientation;
    use super::InfinitiveClause;
    use super::NumberLiteral;
    use super::Phrase;
    #[cfg(test)]
    use super::Preposition;
    use super::PrepositionalPhrase;
    #[cfg(test)]
    use super::RecoveredText;
    use super::Vocabulary;
    use super::is_valid_degree_measure_number;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
    pub enum ComparisonMarker {
        Than,
        ThanOrEqualTo,
    }

    #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
    pub struct ComparisonComplement {
        marker: ComparisonMarker,
        standard: Box<Phrase>,
    }

    impl ComparisonComplement {
        #[must_use]
        pub(crate) fn try_new(marker: ComparisonMarker, standard: Phrase) -> Option<Self> {
            matches!(
                standard,
                Phrase::NounPhrase(_) | Phrase::AdjectivePhrase(_) | Phrase::Clause(_)
            )
            .then_some(Self {
                marker,
                standard: Box::new(standard),
            })
        }

        fn has_typed_standard(&self) -> bool {
            matches!(
                self.standard.as_ref(),
                Phrase::NounPhrase(_) | Phrase::AdjectivePhrase(_) | Phrase::Clause(_)
            )
        }

        #[must_use]
        pub const fn marker(&self) -> ComparisonMarker {
            self.marker
        }

        #[must_use]
        pub fn standard(&self) -> &Phrase {
            &self.standard
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
    pub struct AdjectivePhrase {
        /// A numeral degree measure premodifying the head (`2 greater`). A
        /// premodifier, never a complement: `complements` renders post-head.
        /// Carries its own notation so `two greater` never renders `2 greater`.
        degree: Option<NumberLiteral>,
        head: Adjective,
        complements: Vec<AdjectiveComplement>,
    }

    impl AdjectivePhrase {
        /// Checked base projection used by declaration-owned lowering paths.
        #[must_use]
        pub(crate) fn try_from_lexical_head(head: Adjective) -> Option<Self> {
            (!matches!(head, Adjective::CardOrientation(_))
                && Vocabulary::new().render_adjective(&head).is_some())
            .then_some(Self {
                degree: None,
                head,
                complements: Vec::new(),
            })
        }

        #[must_use]
        pub(crate) const fn from_orientation(orientation: CardOrientation) -> Self {
            Self {
                degree: None,
                head: Adjective::CardOrientation(orientation),
                complements: Vec::new(),
            }
        }

        #[must_use]
        pub(crate) fn try_from_degree_measure(
            measure: NumberLiteral,
            head: Adjective,
        ) -> Option<Self> {
            if !is_valid_degree_measure_number(measure) {
                return None;
            }
            let comparison = match &head {
                Adjective::Word(word) => word.comparison(),
                _ => None,
            }?;
            (comparison.class() == AdjectiveComparisonClass::OrComparative
                && Vocabulary::new().render_adjective(&head).is_some())
            .then_some(Self {
                degree: Some(measure),
                head,
                complements: Vec::new(),
            })
        }

        #[must_use]
        pub(crate) fn try_attach_declared_comparison(
            self,
            comparison: ComparisonComplement,
        ) -> Option<Self> {
            self.try_attach_comparison(comparison, false)
        }

        #[must_use]
        pub(crate) fn try_attach_postnominal_comparison(
            self,
            comparison: ComparisonComplement,
        ) -> Option<Self> {
            self.try_attach_comparison(comparison, true)
        }

        fn try_attach_comparison(
            mut self,
            comparison: ComparisonComplement,
            postnominal: bool,
        ) -> Option<Self> {
            if self.degree.is_some() || !self.complements.is_empty() {
                return None;
            }
            let class = match &self.head {
                Adjective::Word(word) => word.comparison()?.class(),
                _ => return None,
            };
            if !comparison.has_typed_standard()
                || !matches!(
                    (class, comparison.marker),
                    (AdjectiveComparisonClass::OrComparative, _)
                        | (AdjectiveComparisonClass::ThanOnly, ComparisonMarker::Than)
                )
            {
                return None;
            }
            self.complements.push(if postnominal {
                AdjectiveComplement::PostnominalComparison(comparison)
            } else {
                AdjectiveComplement::Comparison(comparison)
            });
            Some(self)
        }

        #[must_use]
        pub(crate) fn try_split_declared_comparison(
            mut self,
        ) -> Option<(Self, ComparisonComplement)> {
            let [AdjectiveComplement::Comparison(comparison)] = self.complements.as_slice() else {
                return None;
            };
            let comparison = comparison.clone();
            self.complements.clear();
            self.clone()
                .try_attach_declared_comparison(comparison.clone())?;
            Some((self, comparison))
        }

        #[must_use]
        pub(crate) fn try_split_postnominal_comparison(
            mut self,
        ) -> Option<(Self, ComparisonComplement)> {
            let [AdjectiveComplement::PostnominalComparison(comparison)] =
                self.complements.as_slice()
            else {
                return None;
            };
            let comparison = comparison.clone();
            self.complements.clear();
            self.clone()
                .try_attach_postnominal_comparison(comparison.clone())?;
            Some((self, comparison))
        }

        /// Extends the J01 post-head complement family with a prepositional
        /// complement. Degree, orientation, and comparison shapes cannot
        /// cross this construction boundary.
        #[must_use]
        pub(crate) fn try_attach_declared_prepositional(
            mut self,
            preposition: PrepositionalPhrase,
        ) -> Option<Self> {
            if !self.accepts_declared_posthead_complement() {
                return None;
            }
            self.complements
                .push(AdjectiveComplement::Prepositional(preposition));
            Some(self)
        }

        /// Extends the J01 post-head complement family with an infinitive.
        #[must_use]
        pub(crate) fn try_attach_declared_infinitive(
            mut self,
            infinitive: InfinitiveClause,
        ) -> Option<Self> {
            if !self.accepts_declared_posthead_complement() {
                return None;
            }
            self.complements
                .push(AdjectiveComplement::Infinitive(infinitive));
            Some(self)
        }

        #[cfg(test)]
        #[must_use]
        pub(crate) fn try_attach_recovered_comparison_standard(
            self,
            recovery: RecoveredText,
        ) -> Option<Self> {
            self.try_attach_declared_prepositional(
                PrepositionalPhrase::from_prepositional_declaration(
                    Preposition::With,
                    Phrase::Recovered(recovery),
                ),
            )
        }

        #[must_use]
        pub(crate) fn try_split_declared_prepositional(
            mut self,
        ) -> Option<(Self, PrepositionalPhrase)> {
            let Some(AdjectiveComplement::Prepositional(preposition)) = self.complements.pop()
            else {
                return None;
            };
            self.clone()
                .try_attach_declared_prepositional(preposition.clone())?;
            Some((self, preposition))
        }

        #[must_use]
        pub(crate) fn try_split_declared_infinitive(mut self) -> Option<(Self, InfinitiveClause)> {
            let Some(AdjectiveComplement::Infinitive(infinitive)) = self.complements.pop() else {
                return None;
            };
            self.clone()
                .try_attach_declared_infinitive(infinitive.clone())?;
            Some((self, infinitive))
        }

        fn accepts_declared_posthead_complement(&self) -> bool {
            self.degree.is_none()
                && !matches!(self.head, Adjective::CardOrientation(_))
                && Vocabulary::new().render_adjective(&self.head).is_some()
                && self
                    .complements
                    .iter()
                    .all(Self::is_declared_posthead_complement)
        }

        const fn is_declared_posthead_complement(complement: &AdjectiveComplement) -> bool {
            matches!(
                complement,
                AdjectiveComplement::Prepositional(_) | AdjectiveComplement::Infinitive(_)
            )
        }

        #[must_use]
        pub const fn degree(&self) -> Option<&NumberLiteral> {
            self.degree.as_ref()
        }

        #[must_use]
        pub const fn head(&self) -> &Adjective {
            &self.head
        }

        #[must_use]
        pub fn complements(&self) -> &[AdjectiveComplement] {
            &self.complements
        }
    }

    #[cfg(test)]
    mod tests {
        use super::Adjective;
        use super::AdjectiveComplement;
        use super::AdjectivePhrase;
        use super::ComparisonComplement;
        use super::ComparisonMarker;
        use super::NumberLiteral;
        use super::Phrase;
        use crate::Numeral;
        use crate::word::Vocab;

        fn two() -> NumberLiteral {
            NumberLiteral {
                value: 2,
                numeral: Numeral::Arabic(false),
            }
        }

        #[test]
        fn generated_renderer_rejects_non_degree_notations() {
            for measure in [
                NumberLiteral {
                    value: 2,
                    numeral: Numeral::Ordinal,
                },
                NumberLiteral {
                    value: 10,
                    numeral: Numeral::Roman,
                },
                NumberLiteral {
                    value: 2_000,
                    numeral: Numeral::Arabic(true),
                },
            ] {
                let malformed = AdjectivePhrase {
                    degree: Some(measure),
                    head: Adjective::Word(Vocab::Greater),
                    complements: Vec::new(),
                };
                assert_eq!(
                    crate::adjective::render(&malformed, "Test Card", false),
                    Err(crate::RenderError::InvalidAdjectiveConstruction),
                    "non-degree notation reached generated rendering: {measure:?}",
                );
            }
        }

        #[test]
        fn generated_renderer_rejects_owner_private_malformed_shapes() {
            let target =
                AdjectivePhrase::try_from_lexical_head(Adjective::Word(Vocab::Target)).unwrap();
            let comparison = ComparisonComplement::try_new(
                ComparisonMarker::Than,
                Phrase::AdjectivePhrase(Box::new(target)),
            )
            .unwrap();
            let malformed = [
                AdjectivePhrase {
                    degree: Some(two()),
                    head: Adjective::Word(Vocab::Greater),
                    complements: vec![AdjectiveComplement::Comparison(comparison.clone())],
                },
                AdjectivePhrase {
                    degree: Some(two()),
                    head: Adjective::CardOrientation(crate::word::CardOrientation::FaceUp),
                    complements: Vec::new(),
                },
                AdjectivePhrase {
                    degree: None,
                    head: Adjective::Word(Vocab::Greater),
                    complements: vec![
                        AdjectiveComplement::Comparison(comparison.clone()),
                        AdjectiveComplement::Comparison(comparison),
                    ],
                },
                AdjectivePhrase {
                    degree: None,
                    head: Adjective::Word(Vocab::Greater),
                    complements: vec![AdjectiveComplement::Comparison(ComparisonComplement {
                        marker: ComparisonMarker::Than,
                        standard: Box::new(Phrase::NumberLiteral(two())),
                    })],
                },
            ];

            for phrase in malformed {
                assert_eq!(
                    crate::adjective::render(&phrase, "Test Card", false),
                    Err(crate::RenderError::InvalidAdjectiveConstruction),
                    "{phrase:#?}",
                );
            }
        }
    }
}

pub use adjective_storage::AdjectivePhrase;
pub use adjective_storage::ComparisonComplement;
pub use adjective_storage::ComparisonMarker;

/// A coordinated run of adjective phrases filling one predicative or
/// postnominal complement slot: `green and white` (Glistening Deluge),
/// `red or green` (Aether Gust), `legendary and snow` (Moritte of the Frost),
/// `green and/or white` (Glistening Deluge), or `creature blocking or blocked
/// by this creature` (Lesser Werewolf). It reuses the landed coordination
/// idiom — a first conjunct plus a list of [`AdjectivePhraseCoordination`]
/// continuations, each recording its own comma and optional connective — so the
/// renderer replays the exact surface list. Reached only from predicative
/// positions (a copular complement or an intransitive-`be` adjective
/// complement); the attributive modifier list stays a
/// [`CoordinatedModifier`](crate::syntax::CoordinatedModifier), so no
/// attributive slot competes with it.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct CoordinatedAdjectivePhrase {
    pub first: Box<AdjectivePhrase>,
    pub rest: Vec<AdjectivePhraseCoordination>,
}

/// One non-first member of a predicative adjective-phrase coordination.
///
/// No `comma` flag: on the supported corpus the serial comma is exactly
/// determined by member count and connective — absent only on an asyndetic
/// interior member (which always takes one) or on a connective member of a
/// two-member list (which never does), present on every connective member of
/// a three-or-more-member (Oxford) list — measured with 0 exceptions across
/// all 31685 supported faces. The renderer derives it from
/// [`CoordinatedAdjectivePhrase::rest`] rather than the AST carrying a field
/// that could contradict it, following the same idiom as
/// [`PrepositionalPhraseCoordination`] and
/// [`TriggerConditionCoordination`](crate::syntax::TriggerConditionCoordination).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct AdjectivePhraseCoordination {
    /// The connective introducing this member: `None` on the asyndetic
    /// comma-separated interior members of an Oxford list; `Some` on a bare
    /// `and`/`or`/`and/or` member and on the final Oxford member. The
    /// disjunctive-or-conjunctive `and/or` is admitted here because a supported
    /// copular witness (Glistening Deluge) attests it.
    pub conjunction: Option<Conjunction>,
    pub phrase: AdjectivePhrase,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub enum AdjectiveComplement {
    Comparison(ComparisonComplement),
    PostnominalComparison(ComparisonComplement),
    Prepositional(PrepositionalPhrase),
    Infinitive(InfinitiveClause),
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub enum NominalComplement {
    Adjective(AdjectivePhrase),
    /// Coordinated postnominal adjectives/reduced participles sharing the
    /// nominal host (`creature blocking or blocked by this creature`).
    CoordinatedAdjective(CoordinatedAdjectivePhrase),
    Prepositional(PrepositionalPhrase),
    Infinitive(InfinitiveClause),
    Relative(RelativeClause),
    /// A markerless postnominal recipient-passive relative (`a creature dealt
    /// damage this way`). The surface has no passive auxiliary, so its
    /// participial predicate is structurally transitive: the antecedent fills
    /// the promoted recipient role while `object` retains the dealt theme.
    ReducedRecipientPassive(TransitivePredicate),
    Quantity(Quantity),
    /// The mandatory `to <color>` argument of the rules-defined `devotion`
    /// value nominal [CR#700.5]. Only the concrete-color shapes ride this
    /// complement — a single color word (`devotion to green`) or a two-color
    /// pair (`devotion to white and black`). The generic `devotion to a/each/
    /// that color` shapes are ordinary `color`-headed nominals and ride the
    /// [`NominalComplement::Prepositional`] path instead.
    Devotion(DevotionColors),
    /// A power/toughness value setting a base-characteristic nominal (`base
    /// power and toughness *X/X*`, `base power and toughness *2/2*`). It is the
    /// same `N/N` token that heads a stat-setting object, carried here so the
    /// characteristic nominal records the value it is set to. Rides the final
    /// coordinated characteristic when the head is a `power and toughness`
    /// pair.
    PowerToughness(PowerToughness),
    /// A bare finite clause counting occurrences of an event — the complement
    /// of `times` in the measured value `the number of times <clause>`
    /// (Riku of Many Paths, Temporal Firestorm). It is a reduced adjunct
    /// relative — no marker, no gap — so only the clause is carried; the
    /// renderer replays it directly after `times`.
    EventClause(Box<IndependentClause>),
    /// A parameterized keyword ability's structured argument attached to its
    /// keyword-noun head in grant position (`ward {2}`, `protection from
    /// black`) — `kwgrant` round. Only the dedicated keyword-headed nominal
    /// reductions construct this; an ordinary noun never acquires it.
    KeywordArgument(super::KeywordArgument),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum SetExceptionMarker {
    Bare,
    For,
}

/// The concrete-color argument of a `devotion` value nominal [CR#700.5]. The
/// color identities are carried structurally so the renderer replays the exact
/// words rather than deriving them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum DevotionColors {
    /// A single color: `devotion to green`.
    Color(ColorWord),
    /// A two-color pair joined by `and`: `devotion to white and black`.
    Pair(ColorWord, ColorWord),
}

/// A sealed prepositional phrase, either simple or sibling-coordinated.
///
/// Every slot accepting a prepositional phrase accepts both forms. Use the
/// checked builders in [`crate::prepositional_phrase`] to construct values and
/// [`Self::kind`] to inspect them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrepositionalPhrase {
    representation: PrepositionalPhraseRepresentation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PrepositionalPhraseRepresentation {
    Simple(SimplePrepositionalPhrase),
    Coordinated(CoordinatedPrepositionalPhrase),
}

/// A borrowed view of a sealed [`PrepositionalPhrase`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrepositionalPhraseKind<'a> {
    /// One preposition and its whole object.
    Simple(&'a SimplePrepositionalPhrase),
    /// Sibling phrases that each repeat their preposition.
    Coordinated(&'a CoordinatedPrepositionalPhrase),
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct SimplePrepositionalPhrase {
    pub(crate) preposition: Preposition,
    pub(crate) object: Box<Phrase>,
}

impl PrepositionalPhrase {
    pub(crate) fn from_prepositional_declaration(preposition: Preposition, object: Phrase) -> Self {
        Self {
            representation: PrepositionalPhraseRepresentation::Simple(SimplePrepositionalPhrase {
                preposition,
                object: Box::new(object),
            }),
        }
    }

    /// Preserves the ability owner's legacy `with "..."` postmodifier without
    /// widening the four public P02 object alternatives.
    pub(crate) fn from_quoted_ability_postmodifier(quoted: QuotedAbility) -> Self {
        Self::from_prepositional_declaration(
            Preposition::With,
            Phrase::QuotedAbility(Box::new(quoted)),
        )
    }

    pub(crate) fn coordinated(
        first: SimplePrepositionalPhrase,
        rest: Vec<PrepositionalPhraseCoordination>,
    ) -> Self {
        Self {
            representation: PrepositionalPhraseRepresentation::Coordinated(
                CoordinatedPrepositionalPhrase {
                    first: Box::new(first),
                    rest,
                },
            ),
        }
    }

    pub(crate) fn into_simple(self) -> Option<SimplePrepositionalPhrase> {
        match self.representation {
            PrepositionalPhraseRepresentation::Simple(value) => Some(value),
            PrepositionalPhraseRepresentation::Coordinated(_) => None,
        }
    }

    pub(crate) fn push_coordination(
        &mut self,
        coordination: PrepositionalPhraseCoordination,
    ) -> bool {
        let PrepositionalPhraseRepresentation::Coordinated(value) = &mut self.representation else {
            return false;
        };
        value.rest.push(coordination);
        true
    }

    /// Returns the borrowed simple or coordinated representation.
    #[must_use]
    pub const fn kind(&self) -> PrepositionalPhraseKind<'_> {
        match &self.representation {
            PrepositionalPhraseRepresentation::Simple(value) => {
                PrepositionalPhraseKind::Simple(value)
            }
            PrepositionalPhraseRepresentation::Coordinated(value) => {
                PrepositionalPhraseKind::Coordinated(value)
            }
        }
    }

    /// Returns the first member.
    ///
    /// Use [`Self::members`] when every coordinated member is relevant.
    #[must_use]
    pub const fn head(&self) -> &SimplePrepositionalPhrase {
        match &self.representation {
            PrepositionalPhraseRepresentation::Simple(simple) => simple,
            PrepositionalPhraseRepresentation::Coordinated(coordinated) => &coordinated.first,
        }
    }

    /// Returns the first member mutably.
    ///
    /// Use [`Self::members`] when every coordinated member is relevant.
    pub const fn head_mut(&mut self) -> &mut SimplePrepositionalPhrase {
        match &mut self.representation {
            PrepositionalPhraseRepresentation::Simple(simple) => simple,
            PrepositionalPhraseRepresentation::Coordinated(coordinated) => &mut coordinated.first,
        }
    }

    /// Returns the last member, whose surface ends the phrase.
    #[must_use]
    pub fn tail(&self) -> &SimplePrepositionalPhrase {
        match &self.representation {
            PrepositionalPhraseRepresentation::Simple(simple) => simple,
            PrepositionalPhraseRepresentation::Coordinated(coordinated) => coordinated
                .rest
                .last()
                .map_or(&*coordinated.first, |coordination| &coordination.phrase),
        }
    }

    /// The sole member, or `None` when this phrase is coordinated.
    #[must_use]
    pub const fn as_simple(&self) -> Option<&SimplePrepositionalPhrase> {
        match &self.representation {
            PrepositionalPhraseRepresentation::Simple(simple) => Some(simple),
            PrepositionalPhraseRepresentation::Coordinated(_) => None,
        }
    }

    /// Every member in surface order.
    pub fn members(&self) -> impl Iterator<Item = &SimplePrepositionalPhrase> {
        let rest = match &self.representation {
            PrepositionalPhraseRepresentation::Simple(_) => [].iter(),
            PrepositionalPhraseRepresentation::Coordinated(coordinated) => coordinated.rest.iter(),
        };
        std::iter::once(self.head()).chain(rest.map(|coordination| &coordination.phrase))
    }
}

impl SimplePrepositionalPhrase {
    /// Returns the preposition introducing this member.
    #[must_use]
    pub const fn preposition(&self) -> Preposition {
        self.preposition
    }

    /// Returns the whole object of this member.
    #[must_use]
    pub fn object(&self) -> &Phrase {
        self.object.as_ref()
    }
}

impl serde::Serialize for PrepositionalPhrase {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match &self.representation {
            PrepositionalPhraseRepresentation::Simple(value) => {
                serializer.serialize_newtype_variant("PrepositionalPhrase", 0, "Simple", value)
            }
            PrepositionalPhraseRepresentation::Coordinated(value) => {
                serializer.serialize_newtype_variant("PrepositionalPhrase", 1, "Coordinated", value)
            }
        }
    }
}

/// Prepositional phrases coordinated as siblings, each repeating its own
/// preposition: `from blue and from black`, `from Vampires, from Werewolves,
/// and from Zombies`. The repeated preposition is what distinguishes this from
/// a single simple phrase whose *object* is coordinated (`from artifacts,
/// creatures, and enchantments`) — one shared preposition over a coordinated
/// noun phrase. Collapsing the two loses the surface distinction and
/// misattaches the second preposition as a conjunct of the first object.
///
/// Mirrors [`CoordinatedNounPhrase`]: interior asyndetic Oxford members carry
/// `conjunction: None`, and the final member carries `Some`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct CoordinatedPrepositionalPhrase {
    pub(crate) first: Box<SimplePrepositionalPhrase>,
    pub(crate) rest: Vec<PrepositionalPhraseCoordination>,
}

impl CoordinatedPrepositionalPhrase {
    /// Returns the first coordinated member.
    #[must_use]
    pub fn first(&self) -> &SimplePrepositionalPhrase {
        self.first.as_ref()
    }

    /// Returns the remaining coordinated members in surface order.
    #[must_use]
    pub fn rest(&self) -> &[PrepositionalPhraseCoordination] {
        &self.rest
    }
}

/// One non-first member of a sibling prepositional coordination.
///
/// No `comma` flag: on the supported corpus the serial comma is exactly
/// determined by member count — absent from a two-member coordination (1364
/// occurrences, 0 exceptions) and present on every member of a three-or-more
/// list (139 occurrences, 0 exceptions) — so the renderer derives it from
/// [`CoordinatedPrepositionalPhrase::rest`] rather than the AST carrying a
/// field that could contradict it. Recording it would let a lowering bug emit
/// `A, and B` and still round-trip clean, since the renderer would faithfully
/// print the stored mistake. This follows the same rule as [`Polarity`], whose
/// `non-` hyphen is derived rather than stored.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct PrepositionalPhraseCoordination {
    pub(crate) conjunction: Option<Conjunction>,
    pub(crate) phrase: SimplePrepositionalPhrase,
}

impl PrepositionalPhraseCoordination {
    /// Returns the conjunction on this member, if it closes the run.
    #[must_use]
    pub const fn conjunction(&self) -> Option<Conjunction> {
        self.conjunction
    }

    /// Returns this member's simple phrase.
    #[must_use]
    pub const fn phrase(&self) -> &SimplePrepositionalPhrase {
        &self.phrase
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum Preposition {
    After,
    Among,
    As,
    At,
    Before,
    Between,
    By,
    During,
    For,
    From,
    In,
    Into,
    Of,
    On,
    Onto,
    To,
    Until,
    Under,
    With,
    Without,
}

impl Preposition {
    const FORMS: &'static [(Self, &'static str)] = &[
        (Self::After, "after"),
        (Self::Among, "among"),
        (Self::As, "as"),
        (Self::At, "at"),
        (Self::Before, "before"),
        (Self::Between, "between"),
        (Self::By, "by"),
        (Self::During, "during"),
        (Self::For, "for"),
        (Self::From, "from"),
        (Self::In, "in"),
        (Self::Into, "into"),
        (Self::Of, "of"),
        (Self::On, "on"),
        (Self::Onto, "onto"),
        (Self::To, "to"),
        (Self::Until, "until"),
        (Self::Under, "under"),
        (Self::With, "with"),
        (Self::Without, "without"),
    ];

    pub(crate) fn from_spelling(surface: &str) -> Option<Self> {
        Self::FORMS.iter().find_map(|(preposition, spelling)| {
            surface
                .eq_ignore_ascii_case(spelling)
                .then_some(*preposition)
        })
    }

    pub(crate) fn spelling(self) -> &'static str {
        Self::FORMS
            .iter()
            .find_map(|(preposition, spelling)| (*preposition == self).then_some(*spelling))
            .expect("every preposition has one spelling")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub enum Phrase {
    Clause(Box<Clause>),
    NounPhrase(Box<NounPhrase>),
    AdjectivePhrase(Box<AdjectivePhrase>),
    PrepositionalPhrase(Box<PrepositionalPhrase>),
    Quantity(Quantity),
    Adverb(Vocab),
    CatalogAtom(CatalogAtom),
    ColorWord(ColorWord),
    Cost(Cost),
    ThisCard(ThisCardForm),
    OracleSymbol(OracleSymbol),
    SymbolSequence(Vec<OracleSymbol>),
    NumberLiteral(NumberLiteral),
    SignedScalar(SignedScalar),
    PowerToughness(PowerToughness),
    EmbeddedAbility(Box<Ability>),
    QuotedAbility(Box<QuotedAbility>),
    Recovered(RecoveredText),
}

#[cfg(test)]
mod adjective_invariant_tests {
    use super::*;

    fn two() -> NumberLiteral {
        NumberLiteral {
            value: 2,
            numeral: Numeral::Arabic(false),
        }
    }

    #[test]
    fn owner_operations_refuse_internal_invariant_bypasses() {
        assert!(
            AdjectivePhrase::try_from_lexical_head(Adjective::CardOrientation(
                crate::word::CardOrientation::FaceUp,
            ))
            .is_none()
        );
        assert!(
            AdjectivePhrase::try_from_degree_measure(
                two(),
                Adjective::CardOrientation(crate::word::CardOrientation::FaceUp),
            )
            .is_none()
        );
        assert!(
            ComparisonComplement::try_new(ComparisonMarker::Than, Phrase::NumberLiteral(two()),)
                .is_none()
        );

        let pending =
            AdjectivePhrase::try_from_lexical_head(Adjective::Word(Vocab::Greater)).unwrap();
        let target =
            AdjectivePhrase::try_from_lexical_head(Adjective::Word(Vocab::Target)).unwrap();
        let comparison = ComparisonComplement::try_new(
            ComparisonMarker::Than,
            Phrase::AdjectivePhrase(Box::new(target)),
        )
        .unwrap();
        let completed = pending
            .try_attach_declared_comparison(comparison.clone())
            .unwrap();
        assert!(
            completed
                .try_attach_declared_comparison(comparison)
                .is_none()
        );
    }

    #[test]
    fn j01_storage_is_private_to_a_sibling_of_the_nominal_owner() {
        fn struct_body<'source>(source: &'source str, definition: &str) -> &'source str {
            let body = source
                .split_once(definition)
                .unwrap_or_else(|| panic!("missing J01 definition {definition:?}"))
                .1;
            body.split_once("\n    }")
                .unwrap_or_else(|| panic!("unterminated J01 definition {definition:?}"))
                .0
        }

        fn contains_struct_literal(source: &str, type_name: &str) -> bool {
            let needle = format!("{type_name} {{");
            source.match_indices(&needle).any(|(start, _)| {
                let prefix = &source[..start];
                !prefix.ends_with("-> ")
                    && !prefix
                        .chars()
                        .next_back()
                        .is_some_and(|character| character.is_alphanumeric() || character == '_')
            })
        }

        let phrase_source = include_str!("phrase.rs");
        let storage_start = phrase_source
            .find(concat!("mod adjective_", "storage {"))
            .expect("J01 storage lives in its own private module");
        let storage_end = phrase_source
            .find(concat!("pub use adjective_", "storage::AdjectivePhrase;"))
            .expect("the public J01 types are re-exported from private storage");
        let nominal_owner = phrase_source
            .find("pub(crate) mod nominal_constructions;")
            .expect("the M01 declaration remains an owner child");
        for definition in [
            "pub struct AdjectivePhrase {",
            "pub struct ComparisonComplement {",
        ] {
            let definition = phrase_source
                .find(definition)
                .unwrap_or_else(|| panic!("missing J01 definition {definition:?}"));
            assert!(
                definition > storage_start && definition < storage_end,
                "{definition:?} is outside private J01 storage",
            );
        }
        for (definition, fields) in [
            (
                "pub struct AdjectivePhrase {",
                [
                    "degree: Option<NumberLiteral>,",
                    "head: Adjective,",
                    "complements: Vec<AdjectiveComplement>,",
                ]
                .as_slice(),
            ),
            (
                "pub struct ComparisonComplement {",
                ["marker: ComparisonMarker,", "standard: Box<Phrase>,"].as_slice(),
            ),
        ] {
            let body = struct_body(phrase_source, definition);
            assert!(
                !body
                    .lines()
                    .any(|line| line.trim_start().starts_with("pub")),
                "{definition:?} exposed raw fields outside the J01 owner",
            );
            for field in fields {
                assert!(
                    body.lines().any(|line| line.trim() == *field),
                    "{definition:?} no longer has the sealed field {field:?}",
                );
            }
        }
        assert!(
            nominal_owner < storage_start || nominal_owner > storage_end,
            "the M01 declaration must be a sibling, not a J01 storage descendant",
        );

        let nominal_source = include_str!("../constructions/nominal.rs");
        for type_name in ["AdjectivePhrase", "ComparisonComplement"] {
            assert!(
                !contains_struct_literal(nominal_source, type_name),
                "M01 regained raw J01 storage access through {type_name}",
            );
        }
    }
}

#[cfg(test)]
mod initial_sound_tests {
    use super::InitialSound;
    use super::PowerToughness;
    use super::ScalarSign;
    use super::ScalarValue;
    use super::SignedScalar;
    use super::number_initial_sound;

    fn signed(sign: ScalarSign, value: ScalarValue) -> SignedScalar {
        SignedScalar { sign, value }
    }

    #[test]
    fn number_initial_sound_reads_the_cardinal_spelling() {
        // The §2.2 verification, table-driven against the §1.1 enumeration:
        // every power the corpus prints on the `a` side must read Consonant;
        // every power it prints on the `an` side must read Vowel. This is the
        // no-over-fire pin.
        for (value, expected) in [
            (0, InitialSound::Consonant),   // "zero"
            (1, InitialSound::Consonant),   // "one" — the exception
            (2, InitialSound::Consonant),   // "two"
            (3, InitialSound::Consonant),   // "three"
            (4, InitialSound::Consonant),   // "four"
            (5, InitialSound::Consonant),   // "five"
            (6, InitialSound::Consonant),   // "six"
            (7, InitialSound::Consonant),   // "seven"
            (8, InitialSound::Vowel),       // "eight"
            (9, InitialSound::Consonant),   // "nine"
            (11, InitialSound::Vowel),      // "eleven"
            (18, InitialSound::Vowel),      // "eighteen"
            (21, InitialSound::Consonant),  // "twenty-one"
            (100, InitialSound::Consonant), // "one hundred"
            (80, InitialSound::Vowel),      // "eighty"
        ] {
            assert_eq!(number_initial_sound(value), expected, "value {value}");
        }
    }

    #[test]
    fn signed_scalar_initial_sound_sign_wins_over_value() {
        assert_eq!(
            signed(ScalarSign::Plus, ScalarValue::Integer(8)).initial_sound(),
            InitialSound::Consonant,
            "+8 is read \"plus eight\""
        );
        assert_eq!(
            signed(ScalarSign::Minus, ScalarValue::Integer(8)).initial_sound(),
            InitialSound::Consonant,
            "-8 is read \"minus eight\""
        );
        assert_eq!(
            signed(ScalarSign::None, ScalarValue::X).initial_sound(),
            InitialSound::Vowel,
            "X is read \"ex\""
        );
        assert_eq!(
            signed(ScalarSign::None, ScalarValue::Star).initial_sound(),
            InitialSound::Consonant,
            "* is read \"star\" (defaulted, unwitnessed)"
        );
    }

    #[test]
    fn power_toughness_initial_sound_reads_the_power_not_the_toughness() {
        let x_over_one = PowerToughness {
            power: signed(ScalarSign::None, ScalarValue::X),
            toughness: signed(ScalarSign::None, ScalarValue::Integer(1)),
        };
        assert_eq!(x_over_one.initial_sound(), InitialSound::Vowel);

        let one_over_x = PowerToughness {
            power: signed(ScalarSign::None, ScalarValue::Integer(1)),
            toughness: signed(ScalarSign::None, ScalarValue::X),
        };
        assert_eq!(one_over_x.initial_sound(), InitialSound::Consonant);
    }
}
