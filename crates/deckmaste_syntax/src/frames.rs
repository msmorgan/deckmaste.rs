//! Subcategorization Frames: the Complements each head licenses.

use std::collections::BTreeMap;

use deckmaste_lexical::LexemeId;
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
    /// A Nominal followed by an Amount, as in *amass Zombies 2*.
    NominalAmount,
    /// A Cost, as in *waterbend {3}*.
    Cost,
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
}

/// What fills a Frame Slot.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Ord, PartialOrd, Serialize)]
pub enum SlotFill {
    NP,
    /// A Preposition Phrase headed by one of the listed prepositions.
    PP(Vec<LexemeId>),
    Pred,
    Clause(ClauseKind),
    ToInf,
    BareInf,
    Gerund,
    KeywordPhrase,
    QuotedAbility,
    Nominal,
    /// A numeral or variable naming how many, as in *scry 2* or *scry X*.
    Amount,
    /// A cost to pay, as in *waterbend {3}*.
    Cost,
    /// No Complement: an intransitive preposition, as *out* in *phase out*.
    Intransitive,
}

/// A [`SlotFill`] without its markers: the key a grammar maps to a Category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Ord, PartialOrd, Serialize)]
pub enum SlotKind {
    Quality,
    Ability,
    Condition,
    Measure,
    Name,
    NP,
    PP,
    Pred,
    FiniteClause,
    VerblessClause,
    ToInf,
    BareInf,
    Gerund,
    KeywordPhrase,
    QuotedAbility,
    Nominal,
    Amount,
    Cost,
    Intransitive,
}

impl SlotFill {
    #[must_use]
    pub fn kind(&self) -> SlotKind {
        match self {
            Self::NP => SlotKind::NP,
            Self::PP(_) => SlotKind::PP,
            Self::Pred => SlotKind::Pred,
            Self::Clause(ClauseKind::Finite) => SlotKind::FiniteClause,
            Self::Clause(ClauseKind::Verbless) => SlotKind::VerblessClause,
            Self::ToInf => SlotKind::ToInf,
            Self::BareInf => SlotKind::BareInf,
            Self::Gerund => SlotKind::Gerund,
            Self::KeywordPhrase => SlotKind::KeywordPhrase,
            Self::QuotedAbility => SlotKind::QuotedAbility,
            Self::Nominal => SlotKind::Nominal,
            Self::Amount => SlotKind::Amount,
            Self::Cost => SlotKind::Cost,
            Self::Intransitive => SlotKind::Intransitive,
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
            FrameFamily::Monotransitive => matches!(shape[..], [(Object, SlotFill::NP)]),
            FrameFamily::Ditransitive => matches!(
                shape[..],
                [(IndirectObject, SlotFill::NP), (Object, SlotFill::NP)]
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
                    [(Complement, SlotFill::Clause(ClauseKind::Finite))]
                )
            }
            FrameFamily::Amount => matches!(shape[..], [(Complement, SlotFill::Amount)]),
            FrameFamily::NominalAmount => matches!(
                shape[..],
                [
                    (Complement, SlotFill::Nominal),
                    (Complement, SlotFill::Amount)
                ]
            ),
            FrameFamily::Cost => matches!(shape[..], [(Complement, SlotFill::Cost)]),
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
}

impl Hosts {
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
        )
    }
}

/// One complement frame of a preposition with its Host Category.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Ord, PartialOrd, Serialize)]
pub struct PrepositionFrame {
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
    Mark(char, deckmaste_lexical::Binding),
}
