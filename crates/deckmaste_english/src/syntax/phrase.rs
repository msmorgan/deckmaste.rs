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
use crate::constructions::coordination::CoordinatedNominalPhrase;
use crate::constructions::coordination::CoordinatedNounPhrase;
use crate::features::Comma;
use crate::features::Conjunction;
use crate::word::Adjective;
use crate::word::ColorWord;
use crate::word::InitialSound;
use crate::word::NounInstance;
use crate::word::Pronoun;
use crate::word::PronounCase;
use crate::word::Vocab;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Quantity(QuantityRepr);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum QuantityRepr {
    Exact(NumberLiteral),
    AtLeast(QuantityValue),
    /// `N or <word>` — a comparative floor (`N or more/greater`) or ceiling
    /// (`N or fewer/less`); the word is carried structurally.
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

/// A read-only view of a validated [`Quantity`]. Constructing a view does not
/// construct a quantity; use the checked `Quantity::try_*` functions for that.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

impl serde::Serialize for Quantity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStructVariant;

        match self.0 {
            QuantityRepr::Exact(number) => {
                serializer.serialize_newtype_variant("Quantity", 0, "Exact", &number)
            }
            QuantityRepr::AtLeast(value) => {
                serializer.serialize_newtype_variant("Quantity", 1, "AtLeast", &value)
            }
            QuantityRepr::OrComparison(value, comparative) => {
                let mut variant =
                    serializer.serialize_struct_variant("Quantity", 2, "OrComparison", 2)?;
                variant.serialize_field("value", &value)?;
                variant.serialize_field("comparative", &comparative)?;
                variant.end()
            }
            QuantityRepr::Or(first, second) => {
                let mut variant = serializer.serialize_struct_variant("Quantity", 3, "Or", 2)?;
                variant.serialize_field("first", &first)?;
                variant.serialize_field("second", &second)?;
                variant.end()
            }
            QuantityRepr::UpTo(value) => {
                serializer.serialize_newtype_variant("Quantity", 4, "UpTo", &value)
            }
            QuantityRepr::MoreThan(value) => {
                serializer.serialize_newtype_variant("Quantity", 5, "MoreThan", &value)
            }
            QuantityRepr::FewerThan(value) => {
                serializer.serialize_newtype_variant("Quantity", 6, "FewerThan", &value)
            }
            QuantityRepr::X => serializer.serialize_unit_variant("Quantity", 7, "X"),
            QuantityRepr::Both => serializer.serialize_unit_variant("Quantity", 8, "Both"),
            QuantityRepr::ThatMany => serializer.serialize_unit_variant("Quantity", 9, "ThatMany"),
            QuantityRepr::ThatMuch => serializer.serialize_unit_variant("Quantity", 10, "ThatMuch"),
        }
    }
}

#[allow(
    non_snake_case,
    non_upper_case_globals,
    reason = "private compatibility shims preserve enum-like internal construction sites"
)]
impl Quantity {
    pub(crate) const fn Exact(number: NumberLiteral) -> Self {
        Self(QuantityRepr::Exact(number))
    }

    pub(crate) const fn AtLeast(value: QuantityValue) -> Self {
        Self(QuantityRepr::AtLeast(value))
    }

    pub(crate) const fn OrComparison(value: QuantityValue, word: ComparativeWord) -> Self {
        Self(QuantityRepr::OrComparison(value, word))
    }

    pub(crate) const fn Or(first: NumberLiteral, second: NumberLiteral) -> Self {
        Self(QuantityRepr::Or(first, second))
    }

    pub(crate) const fn UpTo(value: QuantityValue) -> Self {
        Self(QuantityRepr::UpTo(value))
    }

    pub(crate) const fn MoreThan(value: QuantityValue) -> Self {
        Self(QuantityRepr::MoreThan(value))
    }

    pub(crate) const fn FewerThan(value: QuantityValue) -> Self {
        Self(QuantityRepr::FewerThan(value))
    }

    pub(crate) const X: Self = Self(QuantityRepr::X);
    pub(crate) const Both: Self = Self(QuantityRepr::Both);
    pub(crate) const ThatMany: Self = Self(QuantityRepr::ThatMany);
    pub(crate) const ThatMuch: Self = Self(QuantityRepr::ThatMuch);

    pub(crate) const fn repr(&self) -> &QuantityRepr {
        &self.0
    }

    /// Returns the validated semantic shape without exposing a construction
    /// path around the generated builders.
    #[must_use]
    pub const fn kind(self) -> QuantityKind {
        match self.0 {
            QuantityRepr::Exact(number) => QuantityKind::Exact(number),
            QuantityRepr::AtLeast(value) => QuantityKind::AtLeast(value),
            QuantityRepr::OrComparison(value, word) => QuantityKind::OrComparison(value, word),
            QuantityRepr::Or(first, second) => QuantityKind::Or(first, second),
            QuantityRepr::UpTo(value) => QuantityKind::UpTo(value),
            QuantityRepr::MoreThan(value) => QuantityKind::MoreThan(value),
            QuantityRepr::FewerThan(value) => QuantityKind::FewerThan(value),
            QuantityRepr::X => QuantityKind::X,
            QuantityRepr::Both => QuantityKind::Both,
            QuantityRepr::ThatMany => QuantityKind::ThatMany,
            QuantityRepr::ThatMuch => QuantityKind::ThatMuch,
        }
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

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub enum Determiner {
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

// Grammar lexical meanings are hash-consed. Most determiners are closed,
// nonrecursive syntax values and hash structurally; noun-phrase possessors are
// built during lowering rather than scanned, so a shared tag is sufficient for
// that recursive branch. Hash collisions are permitted, while equal values
// still necessarily produce equal hashes.
impl Hash for Determiner {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
            Self::Demonstrative(demonstrative) => demonstrative.hash(state),
            Self::Target(quantity) => quantity.hash(state),
            Self::Quantity(quantity) => quantity.hash(state),
            Self::Possessive(Possessor::Pronoun(pronoun)) => {
                0_u8.hash(state);
                pronoun.hash(state);
            }
            Self::Possessive(Possessor::NounPhrase(_)) => 1_u8.hash(state),
            Self::Indefinite
            | Self::The
            | Self::Each
            | Self::Another
            | Self::All
            | Self::Any
            | Self::No => {}
        }
    }
}

impl Determiner {
    const SIMPLE_FORMS: &'static [(Self, &'static str)] = &[
        (Self::The, "the"),
        (Self::Each, "each"),
        (Self::Another, "another"),
        (Self::All, "all"),
        (Self::Any, "any"),
        (Self::No, "no"),
    ];

    pub(crate) fn from_spelling(surface: &str) -> Option<Self> {
        Self::SIMPLE_FORMS
            .iter()
            .find_map(|(determiner, spelling)| {
                surface
                    .eq_ignore_ascii_case(spelling)
                    .then(|| determiner.clone())
            })
            .or_else(|| IndefiniteArticle::from_spelling(surface).map(|_| Self::Indefinite))
            .or_else(|| Demonstrative::from_spelling(surface).map(Self::Demonstrative))
            .or_else(|| {
                Pronoun::from_possessive_spelling(surface)
                    .map(|pronoun| Self::Possessive(Possessor::Pronoun(pronoun)))
            })
    }

    pub(crate) fn closed_spelling(&self) -> Option<&'static str> {
        Self::SIMPLE_FORMS
            .iter()
            .find_map(|(determiner, spelling)| (determiner == self).then_some(*spelling))
            .or_else(|| match self {
                Self::Demonstrative(demonstrative) => Some(demonstrative.spelling()),
                Self::Possessive(Possessor::Pronoun(pronoun)) => pronoun.possessive_spelling(),
                // Indefinite has no fixed spelling: which word it renders as
                // depends on the initial sound of the material that follows
                // it, which this determiner-only method has no access to. The
                // two callers that can hold one (`NominalPhrase`,
                // `CoordinatedNominalPhrase`) derive it themselves before
                // ever reaching the generic determiner renderer.
                Self::Indefinite
                | Self::Possessive(Possessor::NounPhrase(_))
                | Self::Target(_)
                | Self::Quantity(_) => None,
                Self::The | Self::Each | Self::Another | Self::All | Self::Any | Self::No => {
                    unreachable!("simple forms returned above")
                }
            })
    }

    #[must_use]
    pub fn noun_cardinality(&self) -> NounCardinality {
        match self {
            Self::Each | Self::Another | Self::Indefinite | Self::Target(None) => {
                NounCardinality::SingularCount
            }
            // The singular demonstratives determine a singular count noun (`that
            // creature`) or a mass one (`that damage`); only `these`/`those` are
            // barred from mass. This is also the parser's cardinality table;
            // grammar features are keyed directly on this syntax value.
            Self::Demonstrative(Demonstrative::This | Demonstrative::That) => {
                NounCardinality::SingularOrMass
            }
            Self::Demonstrative(Demonstrative::These | Demonstrative::Those) => {
                NounCardinality::PluralCount
            }
            Self::Target(Some(quantity)) => match quantity.noun_cardinality() {
                NounCardinality::SingularOrMass => NounCardinality::SingularCount,
                NounCardinality::PluralOrMass => NounCardinality::PluralCount,
                cardinality => cardinality,
            },
            Self::Quantity(quantity) => quantity.noun_cardinality(),
            Self::All => NounCardinality::PluralOrMass,
            Self::The | Self::Possessive(_) | Self::Any | Self::No => {
                NounCardinality::Unconstrained
            }
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

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub enum Possessor {
    Pronoun(Pronoun),
    NounPhrase(Box<NounPhrase>),
}

/// Compatibility name for the inherent-realization noun-cardinality feature.
pub use crate::features::NounCardinality;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub enum NounPhrase {
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
    /// enchantment`). This is one determined selection whose nominal material
    /// is coordinated, distinct from [`Self::Coordinated`] complete noun
    /// phrases (`target artifact and target enchantment`).
    CoordinatedNominal(CoordinatedNominalPhrase),
    Coordinated(CoordinatedNounPhrase),
    /// A trailing set exclusion over a complete noun phrase. The wrapper can
    /// scope over a coordinated included set while the excluded set remains
    /// any ordinary noun phrase.
    SetException(SetExceptionNounPhrase),
    /// An arithmetic value expression combining value operands into a new
    /// numeric value: `<value> minus <value>` or `half <value>`. Additive
    /// `<value> plus <value>` rides the ordinary [`Self::Coordinated`] path
    /// (conjunction [`NounPhraseConjunction::Plus`]); `twice <value>` rides the
    /// copular precomplement-adverb path — both already parse.
    Arithmetic(ArithmeticValue),
}

/// **Measured, `comma` field KEPT** (surface-fact diet, 2026-07-30
/// measurement round): this is a binary wrapper (one included side, one
/// excluded side), not a coordination list — there is no `conjunction` field
/// and no `first`/`rest` parent to count members over, so the serial-comma
/// derivation used elsewhere in this family does not apply. `comma` varies
/// independently of `marker`: the single lowering site
/// (`grammar/lowering.rs`, `RuleTag::NounPhraseSetExceptionBare` /
/// `NounPhraseSetExceptionFor`) assigns both `true` and `false` for *both*
/// markers depending only on which of 4 `(tag, children.len())` grammar
/// productions matched — i.e. `comma` records only whether the surface had a
/// comma token before `except`/`except for`, a fact with no other structural
/// witness on this node.
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum PartitiveHead {
    Quantity(Quantity),
    Each,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct NominalPhraseCoordination {
    #[serde(serialize_with = "super::legacy_serde::serialize_optional_noun_phrase_conjunction")]
    pub conjunction: Option<Conjunction>,
    pub phrase: NominalPhrase,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct NounPhraseCoordination {
    /// The connective introducing this member: `None` on the asyndetic
    /// comma-separated interior members of an Oxford head list (`enchantment,`
    /// in `artifact, enchantment, or land`); `Some` on a bare `and`/`or`/`plus`
    /// member and on the final Oxford member.
    #[serde(serialize_with = "super::legacy_serde::serialize_optional_noun_phrase_conjunction")]
    pub conjunction: Option<Conjunction>,
    pub phrase: NounPhrase,
}

/// Compatibility name for the selection-stratum conjunction feature.
pub use crate::features::Conjunction as NounPhraseConjunction;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct NominalPhrase {
    pub(crate) determiner: Option<Determiner>,
    pub(crate) modifiers: Vec<NominalModifier>,
    pub(crate) head: NounInstance,
    pub(crate) complements: Vec<NominalComplement>,
}

impl NominalPhrase {
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
    #[serde(serialize_with = "super::legacy_serde::serialize_optional_predicate_conjunction")]
    pub conjunction: Option<Conjunction>,
    pub modifier: NominalModifier,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct AdjectivePhrase {
    /// A numeral degree measure premodifying the head (`2 greater`). A
    /// premodifier, never a complement: `complements` renders post-head.
    /// Carries its own notation so `two greater` never renders `2 greater`.
    pub degree: Option<NumberLiteral>,
    pub head: Adjective,
    pub complements: Vec<AdjectiveComplement>,
}

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
    #[serde(serialize_with = "super::legacy_serde::serialize_optional_predicate_conjunction")]
    pub conjunction: Option<Conjunction>,
    pub phrase: AdjectivePhrase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum ComparisonMarker {
    Than,
    ThanOrEqualTo,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ComparisonComplement {
    pub marker: ComparisonMarker,
    pub standard: Box<Phrase>,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
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

/// A prepositional phrase, simple or coordinated. Coordination is a variant of
/// the phrase type itself — exactly as [`NounPhrase::Coordinated`] is — so
/// every slot that already accepts a prepositional phrase (nominal complements,
/// clause adjuncts, exception riders, keyword arguments) admits the coordinated
/// form without opting in. A sibling `Phrase` variant would instead require
/// each of those slots to widen separately, which is the reachability gap that
/// left `Protection from blue, from black, and from red` unparsed.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub enum PrepositionalPhrase {
    Simple(SimplePrepositionalPhrase),
    Coordinated(CoordinatedPrepositionalPhrase),
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct SimplePrepositionalPhrase {
    pub preposition: Preposition,
    pub object: Box<Phrase>,
}

impl PrepositionalPhrase {
    /// Builds the uncoordinated form.
    #[must_use]
    pub fn simple(preposition: Preposition, object: Phrase) -> Self {
        Self::Simple(SimplePrepositionalPhrase {
            preposition,
            object: Box::new(object),
        })
    }

    /// The first member. Call this only where the head member genuinely answers
    /// the question (which preposition introduces the phrase); a site that must
    /// see every conjunct wants [`Self::members`] instead, since treating the
    /// head as the whole phrase is the misattachment this type exists to
    /// prevent.
    #[must_use]
    pub const fn head(&self) -> &SimplePrepositionalPhrase {
        match self {
            Self::Simple(simple) => simple,
            Self::Coordinated(coordinated) => &coordinated.first,
        }
    }

    /// Mutable [`Self::head`], carrying the same caveat.
    pub const fn head_mut(&mut self) -> &mut SimplePrepositionalPhrase {
        match self {
            Self::Simple(simple) => simple,
            Self::Coordinated(coordinated) => &mut coordinated.first,
        }
    }

    /// The last member — the one whose surface ends the phrase. Sites asking
    /// what the phrase *ends* with (trailing punctuation, a closing quote) want
    /// this, not [`Self::head`].
    #[must_use]
    pub fn tail(&self) -> &SimplePrepositionalPhrase {
        match self {
            Self::Simple(simple) => simple,
            Self::Coordinated(coordinated) => coordinated
                .rest
                .last()
                .map_or(&*coordinated.first, |coordination| &coordination.phrase),
        }
    }

    /// The sole member, or `None` when this phrase is coordinated.
    #[must_use]
    pub const fn as_simple(&self) -> Option<&SimplePrepositionalPhrase> {
        match self {
            Self::Simple(simple) => Some(simple),
            Self::Coordinated(_) => None,
        }
    }

    /// Every member in surface order.
    pub fn members(&self) -> impl Iterator<Item = &SimplePrepositionalPhrase> {
        let rest = match self {
            Self::Simple(_) => [].iter(),
            Self::Coordinated(coordinated) => coordinated.rest.iter(),
        };
        std::iter::once(self.head()).chain(rest.map(|coordination| &coordination.phrase))
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
    pub first: Box<SimplePrepositionalPhrase>,
    pub rest: Vec<PrepositionalPhraseCoordination>,
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
    #[serde(serialize_with = "super::legacy_serde::serialize_optional_noun_phrase_conjunction")]
    pub conjunction: Option<Conjunction>,
    pub phrase: SimplePrepositionalPhrase,
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
