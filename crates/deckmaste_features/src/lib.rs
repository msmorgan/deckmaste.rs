//! Closed vocabulary shared by English parsing, rendering, and spelling.

use serde::Serialize;
use serde::Serializer;

/// The layer where a grammatical feature participates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum FeatureStratum {
    InherentRealization,
    Selection,
    SurfaceWitness,
    DiscourseOccurrence,
}

impl FeatureStratum {
    pub const ALL: [Self; 4] = [
        Self::InherentRealization,
        Self::Selection,
        Self::SurfaceWitness,
        Self::DiscourseOccurrence,
    ];
}

macro_rules! feature_inventory {
    ($($variant:ident => ($name:literal, $stratum:ident),)+) => {
        /// The closed set of grammatical concepts shared across English layers.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
        pub enum FeatureKind {
            $($variant,)+
        }

        impl FeatureKind {
            pub const ALL: &[Self] = &[$(Self::$variant,)+];

            #[must_use]
            pub const fn name(self) -> &'static str {
                match self {
                    $(Self::$variant => $name,)+
                }
            }

            #[must_use]
            pub const fn stratum(self) -> FeatureStratum {
                match self {
                    $(Self::$variant => FeatureStratum::$stratum,)+
                }
            }
        }
    };
}

feature_inventory! {
    Person => ("person", InherentRealization),
    Number => ("number", InherentRealization),
    VerbSlot => ("verb-slot", InherentRealization),
    NounCardinality => ("noun-cardinality", InherentRealization),
    Onset => ("onset", InherentRealization),
    Gender => ("gender", InherentRealization),
    PronounClass => ("pronoun-class", InherentRealization),
    PronounCase => ("pronoun-case", InherentRealization),
    LexicalValency => ("lexical-valency", Selection),
    GapState => ("gap-state", Selection),
    ComplementRole => ("complement-role", Selection),
    PredicateAttachmentPhase => ("predicate-attachment-phase", Selection),
    NominalAttachmentPhase => ("nominal-attachment-phase", Selection),
    Conjunction => ("conjunction", Selection),
    Contraction => ("contraction", SurfaceWitness),
    Punctuation => ("punctuation", SurfaceWitness),
    OptionalMaterial => ("optional-material", SurfaceWitness),
    OccurrenceRole => ("occurrence-role", DiscourseOccurrence),
}

/// A grammatical concept that has a fixed entry in [`FeatureKind`].
pub trait GrammaticalFeature {
    const KIND: FeatureKind;
}

/// A grammatical feature permitted to participate in chart identity.
pub trait ChartFeature: GrammaticalFeature {}

/// A grouping of chart features.
pub trait ChartFeatureBundle {}

impl ChartFeatureBundle for () {}

/// A value carried only to preserve an exact surface realization.
pub trait SurfaceWitnessPayload {}

/// A spelling-owned grammatical feature that is excluded from construction
/// matching.
pub trait DiscourseFeature: GrammaticalFeature {}

/// An exact parse preserves both its semantic tree and its surface witnesses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ExactParse<T, W> {
    ast: T,
    surface: W,
}

impl<T, W> ExactParse<T, W> {
    #[must_use]
    pub const fn new(ast: T, surface: W) -> Self {
        Self { ast, surface }
    }

    #[must_use]
    pub const fn ast(&self) -> &T {
        &self.ast
    }

    #[must_use]
    pub const fn surface(&self) -> &W {
        &self.surface
    }

    #[must_use]
    pub fn into_parts(self) -> (T, W) {
        (self.ast, self.surface)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum Person {
    Second,
    Third,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum Number {
    Singular,
    Plural,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum VerbSlot {
    Infinitive,
    Imperative,
    Present { person: Person, number: Number },
    Past { person: Person, number: Number },
    PresentParticiple,
    PastParticiple,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum NounCardinality {
    SingularCount,
    SingularOrMass,
    PluralCount,
    Mass,
    PluralOrMass,
    Unconstrained,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(rename = "InitialSound")]
pub enum Onset {
    Consonant,
    Vowel,
}

/// Derives an onset from the first character of a surface spelling.
#[must_use]
pub fn surface_initial_sound(surface: &str) -> Onset {
    surface
        .chars()
        .next()
        .map_or(Onset::Consonant, character_initial_sound)
}

fn character_initial_sound(character: char) -> Onset {
    if matches!(character.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u') {
        Onset::Vowel
    } else {
        Onset::Consonant
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum Gender {
    Masculine,
    Feminine,
    Neuter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(rename = "Pronoun")]
pub enum PronounClass {
    You,
    It(Gender),
    They,
    EachOther,
    Itself,
    Himself,
    YoursAbsolute,
}

impl PronounClass {
    pub const ALL: [Self; 9] = [
        Self::You,
        Self::It(Gender::Neuter),
        Self::They,
        Self::It(Gender::Masculine),
        Self::It(Gender::Feminine),
        Self::EachOther,
        Self::Itself,
        Self::Himself,
        Self::YoursAbsolute,
    ];

    const POSSESSIVE_FORMS: &'static [(Self, &'static str)] = &[
        (Self::You, "your"),
        (Self::It(Gender::Masculine), "his"),
        (Self::It(Gender::Feminine), "her"),
        (Self::It(Gender::Neuter), "its"),
        (Self::They, "their"),
    ];

    #[must_use]
    pub fn from_possessive_spelling(surface: &str) -> Option<Self> {
        Self::POSSESSIVE_FORMS
            .iter()
            .find_map(|(pronoun, spelling)| {
                surface.eq_ignore_ascii_case(spelling).then_some(*pronoun)
            })
    }

    #[must_use]
    pub fn possessive_spelling(self) -> Option<&'static str> {
        Self::POSSESSIVE_FORMS
            .iter()
            .find_map(|(pronoun, spelling)| (*pronoun == self).then_some(*spelling))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum PronounCase {
    Subject,
    Object,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(rename = "RelativeGap")]
pub enum GapState {
    Subject,
    Object,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum ComplementRole {
    SelectedComplement,
    Adjunct,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum Conjunction {
    And,
    Or,
    Then,
    Plus,
    AndOr,
}

impl Conjunction {
    #[must_use]
    pub const fn spelling(self) -> &'static str {
        match self {
            Self::And => "and",
            Self::Or => "or",
            Self::Then => "then",
            Self::Plus => "plus",
            Self::AndOr => "and/or",
        }
    }

    /// Returns the canonical conjunction represented by `spelling`.
    #[must_use]
    pub fn from_spelling(spelling: &str) -> Option<Self> {
        [Self::And, Self::Or, Self::Then, Self::Plus, Self::AndOr]
            .into_iter()
            .find(|conjunction| spelling.eq_ignore_ascii_case(conjunction.spelling()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Contraction {
    Full,
    Contracted,
}

impl Contraction {
    #[must_use]
    pub const fn is_contracted(self) -> bool {
        matches!(self, Self::Contracted)
    }
}

impl From<bool> for Contraction {
    fn from(contracted: bool) -> Self {
        if contracted { Self::Contracted } else { Self::Full }
    }
}

impl Serialize for Contraction {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_bool(self.is_contracted())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Comma {
    Absent,
    Present,
}

impl Comma {
    #[must_use]
    pub const fn is_present(self) -> bool {
        matches!(self, Self::Present)
    }
}

impl From<bool> for Comma {
    fn from(present: bool) -> Self {
        if present { Self::Present } else { Self::Absent }
    }
}

impl Serialize for Comma {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_bool(self.is_present())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum OptionalMaterial {
    Absent,
    Present,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum MentionKind {
    Full,
    Pronoun,
    Demonstrative,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum OccurrenceRole {
    Argument,
    Mention(MentionKind),
}

macro_rules! grammatical_feature {
    ($type:ty => $kind:ident) => {
        impl GrammaticalFeature for $type {
            const KIND: FeatureKind = FeatureKind::$kind;
        }
    };
}

macro_rules! chart_feature {
    ($type:ty => $kind:ident) => {
        grammatical_feature!($type => $kind);
        impl ChartFeature for $type {}
    };
}

chart_feature!(Person => Person);
chart_feature!(Number => Number);
chart_feature!(VerbSlot => VerbSlot);
chart_feature!(NounCardinality => NounCardinality);
chart_feature!(Onset => Onset);
chart_feature!(Gender => Gender);
chart_feature!(PronounClass => PronounClass);
chart_feature!(PronounCase => PronounCase);
chart_feature!(GapState => GapState);
chart_feature!(ComplementRole => ComplementRole);
chart_feature!(Conjunction => Conjunction);

grammatical_feature!(Contraction => Contraction);
impl SurfaceWitnessPayload for Contraction {}
grammatical_feature!(Comma => Punctuation);
impl SurfaceWitnessPayload for Comma {}
grammatical_feature!(OptionalMaterial => OptionalMaterial);
impl SurfaceWitnessPayload for OptionalMaterial {}
impl SurfaceWitnessPayload for () {}

grammatical_feature!(OccurrenceRole => OccurrenceRole);
impl DiscourseFeature for OccurrenceRole {}

/// Stratum of every public vocabulary type, keyed by type ident. This is
/// the string-keyed face of the trait stratification above, for consumers
/// (the construction declaration compiler) that meet these types as
/// identifiers before any Rust type exists. Kept beside the types so a new
/// vocabulary entry and its row land in one review.
pub const TYPE_STRATA: &[(&str, FeatureStratum)] = &[
    ("Person", FeatureStratum::InherentRealization),
    ("Number", FeatureStratum::InherentRealization),
    ("VerbSlot", FeatureStratum::InherentRealization),
    ("NounCardinality", FeatureStratum::InherentRealization),
    ("Onset", FeatureStratum::InherentRealization),
    ("Gender", FeatureStratum::InherentRealization),
    ("PronounClass", FeatureStratum::InherentRealization),
    ("PronounCase", FeatureStratum::InherentRealization),
    ("GapState", FeatureStratum::Selection),
    ("ComplementRole", FeatureStratum::Selection),
    ("Conjunction", FeatureStratum::Selection),
    ("Contraction", FeatureStratum::SurfaceWitness),
    ("Comma", FeatureStratum::SurfaceWitness),
    ("OptionalMaterial", FeatureStratum::SurfaceWitness),
    ("OccurrenceRole", FeatureStratum::DiscourseOccurrence),
    ("MentionKind", FeatureStratum::DiscourseOccurrence),
];

#[cfg(test)]
mod tests {
    use std::fmt;

    use serde::ser;

    use super::*;

    #[test]
    fn inventory_is_closed_unique_and_stratified() {
        let mut names = FeatureKind::ALL
            .iter()
            .map(|kind| kind.name())
            .collect::<Vec<_>>();
        let declared = names.len();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), declared, "feature names must be unique");
        for stratum in FeatureStratum::ALL {
            assert!(
                FeatureKind::ALL
                    .iter()
                    .any(|kind| kind.stratum() == stratum),
                "{stratum:?} has no declared feature",
            );
        }
    }

    #[test]
    fn type_strata_names_the_vocabulary() {
        let get = |ident: &str| {
            TYPE_STRATA
                .iter()
                .find(|(name, _)| *name == ident)
                .map(|(_, stratum)| *stratum)
        };
        assert_eq!(get("Comma"), Some(FeatureStratum::SurfaceWitness));
        assert_eq!(get("Contraction"), Some(FeatureStratum::SurfaceWitness));
        assert_eq!(
            get("OptionalMaterial"),
            Some(FeatureStratum::SurfaceWitness)
        );
        assert_eq!(get("Conjunction"), Some(FeatureStratum::Selection));
        assert_eq!(
            get("OccurrenceRole"),
            Some(FeatureStratum::DiscourseOccurrence)
        );
        assert_eq!(
            get("MentionKind"),
            Some(FeatureStratum::DiscourseOccurrence)
        );
        assert_eq!(get("Person"), Some(FeatureStratum::InherentRealization));
        assert_eq!(TYPE_STRATA.len(), 16);
    }

    #[test]
    fn conjunction_parses_every_canonical_spelling() {
        for conjunction in [
            Conjunction::And,
            Conjunction::Or,
            Conjunction::Then,
            Conjunction::Plus,
            Conjunction::AndOr,
        ] {
            assert_eq!(
                Conjunction::from_spelling(conjunction.spelling()),
                Some(conjunction),
            );
            assert_eq!(
                Conjunction::from_spelling(&conjunction.spelling().to_ascii_uppercase()),
                Some(conjunction),
            );
        }
        assert_eq!(Conjunction::from_spelling("and or"), None);
        assert_eq!(Conjunction::from_spelling(""), None);
    }

    #[test]
    fn contraction_serializes_as_its_boolean_witness() {
        assert!(!serialize_boolean(Contraction::Full));
        assert!(serialize_boolean(Contraction::Contracted));
    }

    #[test]
    fn comma_serializes_as_its_boolean_witness() {
        assert!(!serialize_boolean(Comma::Absent));
        assert!(serialize_boolean(Comma::Present));
    }

    fn serialize_boolean(value: impl Serialize) -> bool {
        value
            .serialize(BooleanSerializer)
            .expect("the value must serialize as a boolean")
    }

    #[derive(Debug, thiserror::Error)]
    #[error("expected a boolean serialization")]
    struct BooleanSerializationError;

    impl ser::Error for BooleanSerializationError {
        fn custom<T: fmt::Display>(_message: T) -> Self {
            Self
        }
    }

    struct BooleanSerializer;

    macro_rules! unsupported {
        ($($name:ident($($argument:ident: $type:ty),*)),+ $(,)?) => {
            $(
                fn $name(self, $($argument: $type),*) -> Result<Self::Ok, Self::Error> {
                    Err(BooleanSerializationError)
                }
            )+
        };
    }

    impl ser::Serializer for BooleanSerializer {
        type Ok = bool;
        type Error = BooleanSerializationError;
        type SerializeSeq = ser::Impossible<bool, BooleanSerializationError>;
        type SerializeTuple = ser::Impossible<bool, BooleanSerializationError>;
        type SerializeTupleStruct = ser::Impossible<bool, BooleanSerializationError>;
        type SerializeTupleVariant = ser::Impossible<bool, BooleanSerializationError>;
        type SerializeMap = ser::Impossible<bool, BooleanSerializationError>;
        type SerializeStruct = ser::Impossible<bool, BooleanSerializationError>;
        type SerializeStructVariant = ser::Impossible<bool, BooleanSerializationError>;

        fn serialize_bool(self, value: bool) -> Result<Self::Ok, Self::Error> {
            Ok(value)
        }

        #[rustfmt::skip]
        unsupported!(
            serialize_i8(_value: i8),
            serialize_i16(_value: i16),
            serialize_i32(_value: i32),
            serialize_i64(_value: i64),
            serialize_i128(_value: i128),
            serialize_u8(_value: u8),
            serialize_u16(_value: u16),
            serialize_u32(_value: u32),
            serialize_u64(_value: u64),
            serialize_u128(_value: u128),
            serialize_f32(_value: f32),
            serialize_f64(_value: f64),
            serialize_char(_value: char),
            serialize_str(_value: &str),
            serialize_bytes(_value: &[u8]),
            serialize_none(),
            serialize_unit(),
            serialize_unit_struct(_name: &'static str),
            serialize_unit_variant(_name: &'static str, _index: u32, _variant: &'static str),
        );

        fn serialize_some<T: ?Sized + Serialize>(
            self,
            _value: &T,
        ) -> Result<Self::Ok, Self::Error> {
            Err(BooleanSerializationError)
        }

        fn serialize_newtype_struct<T: ?Sized + Serialize>(
            self,
            _name: &'static str,
            _value: &T,
        ) -> Result<Self::Ok, Self::Error> {
            Err(BooleanSerializationError)
        }

        fn serialize_newtype_variant<T: ?Sized + Serialize>(
            self,
            _name: &'static str,
            _variant_index: u32,
            _variant: &'static str,
            _value: &T,
        ) -> Result<Self::Ok, Self::Error> {
            Err(BooleanSerializationError)
        }

        fn serialize_seq(self, _length: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
            Err(BooleanSerializationError)
        }

        fn serialize_tuple(self, _length: usize) -> Result<Self::SerializeTuple, Self::Error> {
            Err(BooleanSerializationError)
        }

        fn serialize_tuple_struct(
            self,
            _name: &'static str,
            _length: usize,
        ) -> Result<Self::SerializeTupleStruct, Self::Error> {
            Err(BooleanSerializationError)
        }

        fn serialize_tuple_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            _variant: &'static str,
            _length: usize,
        ) -> Result<Self::SerializeTupleVariant, Self::Error> {
            Err(BooleanSerializationError)
        }

        fn serialize_map(self, _length: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
            Err(BooleanSerializationError)
        }

        fn serialize_struct(
            self,
            _name: &'static str,
            _length: usize,
        ) -> Result<Self::SerializeStruct, Self::Error> {
            Err(BooleanSerializationError)
        }

        fn serialize_struct_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            _variant: &'static str,
            _length: usize,
        ) -> Result<Self::SerializeStructVariant, Self::Error> {
            Err(BooleanSerializationError)
        }

        fn collect_str<T: ?Sized + fmt::Display>(
            self,
            _value: &T,
        ) -> Result<Self::Ok, Self::Error> {
            Err(BooleanSerializationError)
        }
    }
}
