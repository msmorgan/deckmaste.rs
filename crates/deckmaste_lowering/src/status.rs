//! `status` — authored grammar to engine AST.
//!
//! Scaffolded once from the authored grammar's own source, then HAND-OWNED.
//! Arms here are identities while the two grammars mirror each other; any arm
//! that stops being one carries its justification in place (this crate is the
//! divergence ledger — docs/decisions/authoring-spelling-lowering.md §9).

use crate::Lower;

impl Lower for deckmaste_authoring::Status {
    type Target = deckmaste_core::Status;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Tapped => deckmaste_core::Status::Tapped,
            Self::Untapped => deckmaste_core::Status::Untapped,
            Self::Flipped => deckmaste_core::Status::Flipped,
            Self::Unflipped => deckmaste_core::Status::Unflipped,
            Self::FaceDown => deckmaste_core::Status::FaceDown,
            Self::FaceUp => deckmaste_core::Status::FaceUp,
            Self::PhasedOut => deckmaste_core::Status::PhasedOut,
            Self::PhasedIn => deckmaste_core::Status::PhasedIn,
        }
    }
}

impl Lower for deckmaste_authoring::Face {
    type Target = deckmaste_core::Face;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Up => deckmaste_core::Face::Up,
            Self::Down => deckmaste_core::Face::Down,
        }
    }
}

impl Lower for deckmaste_authoring::Phasing {
    type Target = deckmaste_core::Phasing;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::In => deckmaste_core::Phasing::In,
            Self::Out => deckmaste_core::Phasing::Out,
        }
    }
}

impl Lower for deckmaste_authoring::FaceDownSpec {
    type Target = deckmaste_core::FaceDownSpec;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Listed(f0) => deckmaste_core::FaceDownSpec::Listed(f0.lower()),
        }
    }
}

impl Lower for deckmaste_authoring::FaceDownCharacteristics {
    type Target = deckmaste_core::FaceDownCharacteristics;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::FaceDownCharacteristics {
            name: self.name.lower(),
            types: self.types.lower(),
            subtypes: self.subtypes.lower(),
            abilities: self.abilities.lower(),
            power: self.power.lower(),
            toughness: self.toughness.lower(),
        }
    }
}
