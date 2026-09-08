//! Declared lexical analysis and realization, independent of construction
//! parsing.

mod analysis;
mod model;
mod morphology;
pub mod numeral;
mod pronunciation;

pub use deckmaste_lexical_model::Case;
pub use deckmaste_lexical_model::Category;
pub use deckmaste_lexical_model::Countability;
pub use deckmaste_lexical_model::FeatureBundle;
pub use deckmaste_lexical_model::Finiteness;
pub use deckmaste_lexical_model::Frame;
pub use deckmaste_lexical_model::FrameItem;
pub use deckmaste_lexical_model::FrameSlot;
pub use deckmaste_lexical_model::LexemeId;
pub use deckmaste_lexical_model::LexicalReading;
pub use deckmaste_lexical_model::LexicalValue;
pub use deckmaste_lexical_model::Number;
pub use deckmaste_lexical_model::Numeral;
pub use deckmaste_lexical_model::Onset;
pub use deckmaste_lexical_model::Person;
pub use deckmaste_lexical_model::Relation;
pub use deckmaste_lexical_model::SurfaceCase;
pub use deckmaste_lexical_model::SurfaceFeatures;
pub use deckmaste_lexical_model::Tense;
pub use deckmaste_lexical_model::WordForm;

pub use crate::analysis::*;
pub use crate::model::*;
pub use crate::morphology::default_participle;
pub use crate::numeral::NumeralCodec;
pub use crate::numeral::ParseNumeralError;
