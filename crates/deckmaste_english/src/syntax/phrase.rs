use std::sync::Arc;

use super::ability::Ability;
use super::ability::Cost;
use super::ability::QuotedAbility;
use super::clause::Clause;
use super::clause::IndependentClause;
use super::clause::InfinitiveClause;
use super::clause::PredicateConjunction;
use super::clause::RelativeClause;
use crate::Numeral;
use crate::catalog::CatalogAtom;
use crate::word::Adjective;
use crate::word::ColorWord;
use crate::word::NounInstance;
use crate::word::Pronoun;
use crate::word::PronounCase;
use crate::word::Vocab;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThisCardForm {
    /// The 2024 Oracle shortened self-reference. [CR#201.5c] treats a card's
    /// shortened name as though it used the card's full name.
    AbbreviatedName,
    /// The card's full name.
    FullName,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NumberLiteral {
    pub value: i32,
    pub numeral: Numeral,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScalarSign {
    None,
    Plus,
    Minus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScalarValue {
    Integer(u32),
    X,
    Star,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SignedScalar {
    pub sign: ScalarSign,
    pub value: ScalarValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PowerToughness {
    pub power: SignedScalar,
    pub toughness: SignedScalar,
}

/// The comparative word heading an `N or …` quantity floor or ceiling. Every
/// distinction the surface draws — `more`/`greater` above the bound,
/// `fewer`/`less` at or below it — is preserved here so the renderer replays
/// the exact word rather than guessing one from the bound's direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quantity {
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

impl Quantity {
    #[must_use]
    pub const fn noun_cardinality(self) -> NounCardinality {
        match self {
            Self::Exact(number) if number.value == 1 => NounCardinality::SingularOrMass,
            // `up to one creature`, `more than one creature`, `fewer than one
            // creature` — same licence, now guarded through the value sum.
            // `Variable` can never satisfy it: `X` may resolve to 0 or to any
            // number > 1, so an `X`-bounded head is never a bare singular.
            Self::UpTo(value) | Self::MoreThan(value) | Self::FewerThan(value)
                if value.is_one() =>
            {
                NounCardinality::SingularOrMass
            }
            Self::Or(first, second) if first.value == 1 && second.value == 1 => {
                NounCardinality::SingularOrMass
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
            | Self::Both => NounCardinality::PluralOrMass,
            Self::AtLeast(_) | Self::ThatMany => NounCardinality::PluralCount,
            Self::ThatMuch => NounCardinality::Mass,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Determiner {
    The,
    Each,
    Another,
    Indefinite(IndefiniteArticle),
    Demonstrative(Demonstrative),
    Target(Option<Quantity>),
    Quantity(Quantity),
    Possessive(Possessor),
    All,
    Any,
    No,
}

impl Determiner {
    #[must_use]
    pub const fn noun_cardinality(&self) -> NounCardinality {
        match self {
            Self::Each | Self::Another | Self::Indefinite(_) | Self::Target(None) => {
                NounCardinality::SingularCount
            }
            // The singular demonstratives determine a singular count noun (`that
            // creature`) or a mass one (`that damage`); only `these`/`those` are
            // barred from mass. Mirrors `DeterminerKey::cardinality` in the
            // grammar, which is what the parser enforces.
            Self::Demonstrative(Demonstrative::This | Demonstrative::That) => {
                NounCardinality::SingularOrMass
            }
            Self::Demonstrative(Demonstrative::These | Demonstrative::Those)
            | Self::Target(Some(Quantity::Or(_, _) | Quantity::X | Quantity::Both)) => {
                NounCardinality::PluralCount
            }
            Self::Target(Some(Quantity::Exact(number))) if number.value == 1 => {
                NounCardinality::SingularCount
            }
            Self::Target(Some(
                Quantity::UpTo(value) | Quantity::MoreThan(value) | Quantity::FewerThan(value),
            )) if value.is_one() => NounCardinality::SingularCount,
            Self::Target(Some(
                Quantity::Exact(_)
                | Quantity::AtLeast(_)
                | Quantity::OrComparison(_, _)
                | Quantity::UpTo(_)
                | Quantity::MoreThan(_)
                | Quantity::FewerThan(_)
                | Quantity::ThatMany,
            )) => NounCardinality::PluralCount,
            Self::Target(Some(Quantity::ThatMuch)) => NounCardinality::Mass,
            Self::Quantity(quantity) => quantity.noun_cardinality(),
            Self::All => NounCardinality::PluralOrMass,
            Self::The | Self::Possessive(_) | Self::Any | Self::No => {
                NounCardinality::Unconstrained
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndefiniteArticle {
    A,
    An,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Demonstrative {
    This,
    That,
    These,
    Those,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Possessor {
    Pronoun(Pronoun),
    NounPhrase(Box<NounPhrase>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NounCardinality {
    SingularCount,
    SingularOrMass,
    PluralCount,
    Mass,
    PluralOrMass,
    Unconstrained,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
    Coordinated(CoordinatedNounPhrase),
    /// An arithmetic value expression combining value operands into a new
    /// numeric value: `<value> minus <value>` or `half <value>`. Additive
    /// `<value> plus <value>` rides the ordinary [`Self::Coordinated`] path
    /// (conjunction [`NounPhraseConjunction::Plus`]); `twice <value>` rides the
    /// copular precomplement-adverb path — both already parse.
    Arithmetic(ArithmeticValue),
}

/// An arithmetic combination of value operands, carried structurally so the
/// renderer replays the operator rather than deriving it. Operands are
/// themselves noun-phrase values (a nominal such as `the number of lands you
/// control`, or a bare number).
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rounding {
    Up,
    Down,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartitiveNounPhrase {
    pub head: PartitiveHead,
    pub whole: Box<NounPhrase>,
}

/// The quantifier heading a partitive `<head> of <whole>`. `Quantity` covers
/// the counted partitives (`one of them`, `more than one of X`); `Each` is the
/// distributive `each of X`, whose determiner is not a count quantity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartitiveHead {
    Quantity(Quantity),
    Each,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoordinatedNounPhrase {
    pub first: Box<NounPhrase>,
    pub rest: Vec<NounPhraseCoordination>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NounPhraseCoordination {
    /// The connective introducing this member: `None` on the asyndetic
    /// comma-separated interior members of an Oxford head list (`enchantment,`
    /// in `artifact, enchantment, or land`); `Some` on a bare `and`/`or`/`plus`
    /// member and on the final Oxford member.
    pub conjunction: Option<NounPhraseConjunction>,
    pub comma: bool,
    pub phrase: NounPhrase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NounPhraseConjunction {
    And,
    Or,
    Plus,
    AndOr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NominalPhrase {
    pub determiner: Option<Determiner>,
    pub modifiers: Vec<NominalModifier>,
    pub head: NounInstance,
    pub complements: Vec<NominalComplement>,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoordinatedModifier {
    pub first: Box<NominalModifier>,
    pub rest: Vec<ModifierCoordination>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifierCoordination {
    /// The connective introducing this member: `None` on the asyndetic
    /// comma-separated members of an Oxford list (`artifact,` in `artifact,
    /// creature, and land`); `Some` on the final `and`/`or`/`and/or` member.
    pub conjunction: Option<PredicateConjunction>,
    pub comma: bool,
    pub modifier: NominalModifier,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdjectivePhrase {
    /// A numeral degree measure premodifying the head (`2 greater`). A
    /// premodifier, never a complement: `complements` renders post-head.
    /// Carries its own notation so `two greater` never renders `2 greater`.
    pub degree: Option<NumberLiteral>,
    pub head: Adjective,
    pub complements: Vec<AdjectiveComplement>,
}

/// A coordinated run of predicative adjective phrases filling one copular or
/// intransitive-`be` complement slot: `green and white` (Glistening Deluge),
/// `red or green` (Aether Gust), `legendary and snow` (Moritte of the Frost),
/// `green and/or white` (Glistening Deluge). It reuses the landed coordination
/// idiom — a first conjunct plus a list of [`AdjectivePhraseCoordination`]
/// continuations, each recording its own comma and optional connective — so the
/// renderer replays the exact surface list. Reached only from predicative
/// positions (a copular complement or an intransitive-`be` adjective
/// complement); the attributive modifier list stays a
/// [`CoordinatedModifier`](crate::syntax::CoordinatedModifier), so no
/// attributive slot competes with it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoordinatedAdjectivePhrase {
    pub first: Box<AdjectivePhrase>,
    pub rest: Vec<AdjectivePhraseCoordination>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdjectivePhraseCoordination {
    /// The connective introducing this member: `None` on the asyndetic
    /// comma-separated interior members of an Oxford list; `Some` on a bare
    /// `and`/`or`/`and/or` member and on the final Oxford member. The
    /// disjunctive-or-conjunctive `and/or` is admitted here because a supported
    /// copular witness (Glistening Deluge) attests it.
    pub conjunction: Option<PredicateConjunction>,
    pub comma: bool,
    pub phrase: AdjectivePhrase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComparisonMarker {
    Than,
    ThanOrEqualTo,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComparisonComplement {
    pub marker: ComparisonMarker,
    pub standard: Box<Phrase>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdjectiveComplement {
    Comparison(ComparisonComplement),
    PostnominalComparison(ComparisonComplement),
    Prepositional(PrepositionalPhrase),
    Infinitive(InfinitiveClause),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NominalComplement {
    Adjective(AdjectivePhrase),
    Prepositional(PrepositionalPhrase),
    Infinitive(InfinitiveClause),
    Relative(RelativeClause),
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
}

/// The concrete-color argument of a `devotion` value nominal [CR#700.5]. The
/// color identities are carried structurally so the renderer replays the exact
/// words rather than deriving them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DevotionColors {
    /// A single color: `devotion to green`.
    Color(ColorWord),
    /// A two-color pair joined by `and`: `devotion to white and black`.
    Pair(ColorWord, ColorWord),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrepositionalPhrase {
    pub preposition: Preposition,
    pub object: Box<Phrase>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
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
