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

#[cfg(test)]
mod tests {
    #![allow(
        unused_imports,
        reason = "a module may need only one assertion, or no helper"
    )]

    use crate::assert_lowers;
    use crate::assert_lowers_debug;
    use crate::minimal::*;

    #[test]
    fn lowers_status_tapped() {
        assert_lowers(deckmaste_authoring::Status::Tapped);
    }

    #[test]
    fn lowers_status_untapped() {
        assert_lowers(deckmaste_authoring::Status::Untapped);
    }

    #[test]
    fn lowers_status_flipped() {
        assert_lowers(deckmaste_authoring::Status::Flipped);
    }

    #[test]
    fn lowers_status_unflipped() {
        assert_lowers(deckmaste_authoring::Status::Unflipped);
    }

    #[test]
    fn lowers_status_face_down() {
        assert_lowers(deckmaste_authoring::Status::FaceDown);
    }

    #[test]
    fn lowers_status_face_up() {
        assert_lowers(deckmaste_authoring::Status::FaceUp);
    }

    #[test]
    fn lowers_status_phased_out() {
        assert_lowers(deckmaste_authoring::Status::PhasedOut);
    }

    #[test]
    fn lowers_status_phased_in() {
        assert_lowers(deckmaste_authoring::Status::PhasedIn);
    }

    #[test]
    fn lowers_face_up() {
        assert_lowers(deckmaste_authoring::Face::Up);
    }

    #[test]
    fn lowers_face_down() {
        assert_lowers(deckmaste_authoring::Face::Down);
    }

    #[test]
    fn lowers_phasing_in() {
        assert_lowers(deckmaste_authoring::Phasing::In);
    }

    #[test]
    fn lowers_phasing_out() {
        assert_lowers(deckmaste_authoring::Phasing::Out);
    }

    #[test]
    fn lowers_face_down_spec_listed() {
        assert_lowers(deckmaste_authoring::FaceDownSpec::Listed(
            minimal_face_down_characteristics(),
        ));
    }

    #[test]
    fn lowers_face_down_characteristics() {
        assert_lowers(deckmaste_authoring::FaceDownCharacteristics {
            name: None,
            types: Vec::new(),
            subtypes: Vec::new(),
            abilities: Vec::new(),
            power: None,
            toughness: None,
        });
    }
}
