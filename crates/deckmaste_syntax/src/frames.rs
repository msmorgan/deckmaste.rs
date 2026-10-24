//! Subcategorization Frames: the Complements each head licenses.

use std::collections::BTreeMap;

use deckmaste_lexical::LexemeId;
use dpsi::Ident;
use serde::Deserialize;
use serde::Serialize;

use crate::LoadError;

/// The kind of a Subcategorization Frame, named by the shape of its slots.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Ord, PartialOrd, Serialize)]
pub enum FrameFamily {
    Intransitive,
    Monotransitive,
    Ditransitive,
    TransitiveWithPP,
    TransitiveWithComplement,
    Prepositional,
    Copular,
    Catenative,
    /// An object and a predicative complement, as in *turn it face up*.
    ComplexTransitive,
    /// An object and a non-finite complement, as in *cause you to sacrifice*.
    ComplexCatenative,
    /// A content clause, as in *decide whether to flip again*.
    ContentClause,
    /// A numeric Amount, as in *scry 2*.
    Amount,
    /// An Object followed by an Amount, as in *amass Zombies 2*.
    TransitiveAmount,
    /// A Cost, as in *waterbend {3}*.
    Cost,
    /// A fixed preposition with the verb's other Complements, as in the
    /// phrasal-copular *come up heads* (CGEL, Ch. 4 §6.3, p. 288).
    Idiom,
    NounComplement,
    AdjectiveComplement,
    DeterminativeComplement,
}

/// The grammatical relation a Frame Slot bears to its head.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Ord, PartialOrd, Serialize)]
pub enum SlotRelation {
    Object,
    IndirectObject,
    Complement,
    PredicativeComplement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Ord, PartialOrd, Serialize)]
pub enum ClauseKind {
    Finite,
    Verbless,
    /// A non-finite or verbless clause with its Subject, the Complement of
    /// *with* and *without* (*with your hand revealed*; CGEL, Ch. 14 §8.3,
    /// p. 1262, and §10(a), p. 1267).
    Absolute,
    /// An interrogative content clause (*whether ...*, *which creatures
    /// attack*, *how each player votes*).
    Interrogative,
}

/// What fills a Frame Slot.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Ord, PartialOrd, Serialize)]
pub enum SlotFill {
    NP,
    /// A Preposition Phrase headed by one of the listed prepositions.
    PP(Vec<LexemeId>),
    Pred,
    AdjP,
    /// An adverbial: an Adverb Phrase or a Noun Phrase of frequency, as
    /// *twice* or *three times* in *more than twice*.
    AdvP,
    LocativeNP,
    /// A slash measure as a Noun Phrase, taken only where a head selects
    /// one (*gets +2/+2*).
    MeasureNP,
    Clause(ClauseKind),
    /// A declarative content clause permitting the subordinator *that*:
    /// whether a content clause may be marked is a property of the head
    /// that selects it, not of the clause.
    ExpandableClause,
    /// A Clause set off by a dash after its head's Object, giving that
    /// Object's content (*faces a villainous choice — They discard their
    /// hand, or you draw three cards*): the dash is the slot's, not the
    /// clause's.
    SetOffClause,
    ToInf,
    /// A *to*-infinitival with an object Gap its verb's Subject fills
    /// (*costs {1} to cast*).
    HollowToInf,
    BareInf,
    Gerund,
    KeywordPhrase,
    QuotedAbility,
    /// A word cited rather than used, as *left or right* in *choose left or
    /// right*: a metalinguistic Noun Phrase (CGEL, Ch. 5 §8.2, p. 401),
    /// taken only where a head selects one.
    Mention,
    Nominal,
    /// A numeral or variable naming how many, as in *scry 2* or *scry X*.
    Amount,
    /// A cost to pay, as in *waterbend {3}*.
    Cost,
    /// No Complement: an intransitive preposition, as *out* in *phase out*.
    Intransitive,
    /// An unlike coordination of an Amount and a Noun Phrase (*equal to [2]
    /// or [the greatest power among ...]*). It is never declared: a
    /// preposition that selects both an Amount and a Noun Phrase selects it,
    /// since each conjunct fills the position alone (CGEL, Ch. 15 §3.1,
    /// p. 1323, rule [1]).
    AmountOrNP,
}

/// A [`SlotFill`] without its markers: the key a grammar maps to a Category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Ord, PartialOrd, Serialize)]
pub enum SlotKind {
    TypeModifier,
    NonManaCost,
    Quality,
    Ability,
    Condition,
    Measure,
    Name,
    NP,
    PP,
    Pred,
    AdjP,
    AdvP,
    LocativeNP,
    MeasureNP,
    FiniteClause,
    ExpandableClause,
    SetOffClause,
    VerblessClause,
    AbsoluteClause,
    InterrogativeClause,
    ToInf,
    HollowToInf,
    BareInf,
    Gerund,
    KeywordPhrase,
    QuotedAbility,
    Mention,
    Nominal,
    Amount,
    Cost,
    Intransitive,
    AmountOrNP,
}

impl SlotFill {
    #[must_use]
    pub fn kind(&self) -> SlotKind {
        match self {
            Self::NP => SlotKind::NP,
            Self::PP(_) => SlotKind::PP,
            Self::Pred => SlotKind::Pred,
            Self::AdjP => SlotKind::AdjP,
            Self::AdvP => SlotKind::AdvP,
            Self::LocativeNP => SlotKind::LocativeNP,
            Self::MeasureNP => SlotKind::MeasureNP,
            Self::Clause(ClauseKind::Finite) => SlotKind::FiniteClause,
            Self::ExpandableClause => SlotKind::ExpandableClause,
            Self::SetOffClause => SlotKind::SetOffClause,
            Self::Clause(ClauseKind::Verbless) => SlotKind::VerblessClause,
            Self::Clause(ClauseKind::Absolute) => SlotKind::AbsoluteClause,
            Self::Clause(ClauseKind::Interrogative) => SlotKind::InterrogativeClause,
            Self::ToInf => SlotKind::ToInf,
            Self::HollowToInf => SlotKind::HollowToInf,
            Self::BareInf => SlotKind::BareInf,
            Self::Gerund => SlotKind::Gerund,
            Self::KeywordPhrase => SlotKind::KeywordPhrase,
            Self::QuotedAbility => SlotKind::QuotedAbility,
            Self::Mention => SlotKind::Mention,
            Self::Nominal => SlotKind::Nominal,
            Self::Amount => SlotKind::Amount,
            Self::Cost => SlotKind::Cost,
            Self::Intransitive => SlotKind::Intransitive,
            Self::AmountOrNP => SlotKind::AmountOrNP,
        }
    }
}

/// One position in a Subcategorization Frame.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Ord, PartialOrd, Serialize)]
pub struct FrameSlot {
    pub relation: SlotRelation,
    pub fill: SlotFill,
}

/// An ordered list of the Complements a head licenses.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Ord, PartialOrd, Serialize)]
pub struct Frame {
    pub family: FrameFamily,
    pub slots: Vec<FrameSlot>,
    /// The frame is restricted to active voice, so it derives no passive
    /// Valence Alternation: *A packet of cigarettes costs around seven
    /// dollars*, *Jill has the best qualifications* (CGEL, Ch. 16 §10.1.2,
    /// p. 1432, [17]).
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub active_only: bool,
}

impl Frame {
    /// Whether the slot list has the shape its family names.
    #[must_use]
    pub fn is_well_formed(&self) -> bool {
        use SlotRelation::Complement;
        use SlotRelation::IndirectObject;
        use SlotRelation::Object;
        use SlotRelation::PredicativeComplement;
        let shape: Vec<_> = self.slots.iter().map(|s| (s.relation, &s.fill)).collect();
        match self.family {
            FrameFamily::Intransitive => shape.is_empty(),
            FrameFamily::Monotransitive => matches!(
                shape[..],
                [(
                    Object,
                    SlotFill::NP
                        | SlotFill::KeywordPhrase
                        | SlotFill::QuotedAbility
                        | SlotFill::Mention
                        | SlotFill::MeasureNP
                )]
            ),
            FrameFamily::Ditransitive => matches!(
                shape[..],
                [(IndirectObject, SlotFill::NP), (Object, SlotFill::NP)]
            ),
            FrameFamily::TransitiveWithComplement => matches!(
                shape[..],
                [
                    (Object, SlotFill::NP),
                    (
                        Complement,
                        SlotFill::LocativeNP | SlotFill::HollowToInf | SlotFill::SetOffClause
                    )
                ]
            ),
            FrameFamily::TransitiveWithPP => {
                matches!(
                    shape[..],
                    [(Object, SlotFill::NP), (Complement, SlotFill::PP(_))]
                )
            }
            FrameFamily::Prepositional => matches!(shape[..], [(Complement, SlotFill::PP(_))]),
            FrameFamily::Copular => matches!(shape[..], [(PredicativeComplement, SlotFill::Pred)]),
            FrameFamily::Catenative => matches!(
                shape[..],
                [(
                    Complement,
                    SlotFill::ToInf | SlotFill::BareInf | SlotFill::Gerund
                )]
            ),
            FrameFamily::ComplexTransitive => matches!(
                shape[..],
                [
                    (Object, SlotFill::NP),
                    (PredicativeComplement, SlotFill::Pred)
                ]
            ),
            FrameFamily::ComplexCatenative => matches!(
                shape[..],
                [
                    (Object, SlotFill::NP),
                    (Complement, SlotFill::ToInf | SlotFill::BareInf)
                ]
            ),
            FrameFamily::ContentClause => {
                matches!(
                    shape[..],
                    [(
                        Complement,
                        SlotFill::Clause(ClauseKind::Finite | ClauseKind::Interrogative)
                            | SlotFill::ExpandableClause
                    )]
                )
            }
            FrameFamily::Amount => matches!(shape[..], [(Complement, SlotFill::Amount)]),
            FrameFamily::TransitiveAmount => matches!(
                shape[..],
                [(Object, SlotFill::NP), (Complement, SlotFill::Amount)]
            ),
            FrameFamily::Cost => matches!(shape[..], [(Complement, SlotFill::Cost)]),
            FrameFamily::Idiom => {
                shape
                    .iter()
                    .any(|slot| matches!(slot, (Complement, SlotFill::PP(_))))
                    && shape.iter().all(|slot| {
                        matches!(
                            slot,
                            (Complement, SlotFill::PP(_)) | (PredicativeComplement, SlotFill::Pred)
                        )
                    })
            }
            FrameFamily::NounComplement
            | FrameFamily::AdjectiveComplement
            | FrameFamily::DeterminativeComplement => {
                matches!(shape[..], [(Complement, fill)] if *fill != SlotFill::Intransitive)
            }
        }
    }
}

/// Which hosts an unselected Preposition Phrase may modify (its Host Category).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Ord, PartialOrd, Serialize)]
pub enum Hosts {
    ComplementOnly,
    Nominal,
    Predicate,
    NominalAndPredicate,
    /// Only a Supplement to a Clause or line (*, where X is its power*).
    Supplement,
}

impl Hosts {
    /// The Host Categories admitted, as a set of [`HostKind`] bits.
    #[must_use]
    pub(crate) fn mask(self) -> u8 {
        use crate::grammar::HostKind;
        [HostKind::Nominal, HostKind::Predicate, HostKind::Supplement]
            .into_iter()
            .filter(|kind| self.admits(*kind))
            .fold(0, |mask, kind| mask | host_bit(kind))
    }

    #[must_use]
    pub fn admits(self, kind: crate::grammar::HostKind) -> bool {
        use crate::grammar::HostKind;
        matches!(
            (self, kind),
            (Self::Nominal | Self::NominalAndPredicate, HostKind::Nominal)
                | (
                    Self::Predicate | Self::NominalAndPredicate,
                    HostKind::Predicate
                )
                | (Self::Supplement, HostKind::Supplement)
        )
    }
}

impl Hosts {
    /// The Host Categories both admit.
    #[must_use]
    pub(crate) fn meet(self, other: Self) -> Self {
        match self.mask() & other.mask() {
            0 => Self::ComplementOnly,
            1 => Self::Nominal,
            2 => Self::Predicate,
            3 => Self::NominalAndPredicate,
            _ => Self::Supplement,
        }
    }
}

/// A [`HostKind`]'s bit in a set of admitted hosts.
pub(crate) fn host_bit(kind: crate::grammar::HostKind) -> u8 {
    use crate::grammar::HostKind;
    match kind {
        HostKind::Nominal => 1,
        HostKind::Predicate => 2,
        HostKind::Supplement => 4,
    }
}

/// One complement frame of a preposition with its Host Category.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Ord, PartialOrd, Serialize)]
pub struct PrepositionFrame {
    #[serde(default = "complement_relation")]
    pub relation: SlotRelation,
    pub complement: SlotFill,
    pub hosts: Hosts,
}

/// Subcategorization Frames by lexeme. Prepositions declare theirs in
/// `prepositions`, every other head in `frames`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FrameCatalog {
    /// Ordered parameter patterns, including the position of the head.
    #[serde(default)]
    pub parameters: BTreeMap<LexemeId, Vec<Vec<ParameterPart>>>,
    #[serde(default)]
    pub frames: BTreeMap<LexemeId, Vec<Frame>>,
    #[serde(default)]
    pub prepositions: BTreeMap<LexemeId, Vec<PrepositionFrame>>,
}

impl FrameCatalog {
    /// # Errors
    /// Malformed RON or a frame whose slots do not have its family's shape.
    pub fn from_ron_str(text: &str) -> Result<Self, LoadError> {
        let catalog: Self =
            ron::from_str(text).map_err(|error| LoadError::Format(error.to_string()))?;
        for (lexeme, frames) in &catalog.frames {
            if let Some(frame) = frames.iter().find(|frame| !frame.is_well_formed()) {
                return Err(LoadError::Invalid(format!(
                    "frame {frame:?} of {lexeme} does not have the shape of its family"
                )));
            }
        }
        Ok(catalog)
    }
}

/// One component of a catalog-declared parameter pattern.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum ParameterPart {
    Head,
    Slot(SlotKind),
    Word(LexemeId),
    Words(Vec<LexemeId>),
    /// A word set with recursive premodification, projected as a constituent.
    ModifiedWords(Vec<LexemeId>, SlotKind, SlotKind),
    Mark(char, deckmaste_lexical::Binding),
    /// A value the pattern gives a feature of its phrase, as no word: the
    /// kind of a keyword form's final Keyword Parameter.
    Feature(Ident, Ident),
}

fn complement_relation() -> SlotRelation {
    SlotRelation::Complement
}
