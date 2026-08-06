//! `status` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for deckmaste_semantics::Status {
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

impl Lower for deckmaste_semantics::Face {
    type Target = deckmaste_core::Face;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Up => deckmaste_core::Face::Up,
            Self::Down => deckmaste_core::Face::Down,
        }
    }
}

impl Lower for deckmaste_semantics::Phasing {
    type Target = deckmaste_core::Phasing;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::In => deckmaste_core::Phasing::In,
            Self::Out => deckmaste_core::Phasing::Out,
        }
    }
}

impl Lower for deckmaste_semantics::FaceDownSpec {
    type Target = deckmaste_core::FaceDownSpec;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Listed(f0) => deckmaste_core::FaceDownSpec::Listed(f0.lower()),
        }
    }
}

impl Lower for deckmaste_semantics::FaceDownCharacteristics {
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

    use std::assert_matches;

    use crate::Lower;
    use crate::assert_lowers;
    use crate::minimal::*;

    #[test]
    fn lowers_status_tapped() {
        assert_matches!(
            deckmaste_semantics::Status::Tapped.lower(),
            deckmaste_core::Status::Tapped
        );
    }

    #[test]
    fn lowers_status_untapped() {
        assert_matches!(
            deckmaste_semantics::Status::Untapped.lower(),
            deckmaste_core::Status::Untapped
        );
    }

    #[test]
    fn lowers_status_flipped() {
        assert_matches!(
            deckmaste_semantics::Status::Flipped.lower(),
            deckmaste_core::Status::Flipped
        );
    }

    #[test]
    fn lowers_status_unflipped() {
        assert_matches!(
            deckmaste_semantics::Status::Unflipped.lower(),
            deckmaste_core::Status::Unflipped
        );
    }

    #[test]
    fn lowers_status_face_down() {
        assert_matches!(
            deckmaste_semantics::Status::FaceDown.lower(),
            deckmaste_core::Status::FaceDown
        );
    }

    #[test]
    fn lowers_status_face_up() {
        assert_matches!(
            deckmaste_semantics::Status::FaceUp.lower(),
            deckmaste_core::Status::FaceUp
        );
    }

    #[test]
    fn lowers_status_phased_out() {
        assert_matches!(
            deckmaste_semantics::Status::PhasedOut.lower(),
            deckmaste_core::Status::PhasedOut
        );
    }

    #[test]
    fn lowers_status_phased_in() {
        assert_matches!(
            deckmaste_semantics::Status::PhasedIn.lower(),
            deckmaste_core::Status::PhasedIn
        );
    }

    #[test]
    fn lowers_face_up() {
        assert_matches!(
            deckmaste_semantics::Face::Up.lower(),
            deckmaste_core::Face::Up
        );
    }

    #[test]
    fn lowers_face_down() {
        assert_matches!(
            deckmaste_semantics::Face::Down.lower(),
            deckmaste_core::Face::Down
        );
    }

    #[test]
    fn lowers_phasing_in() {
        assert_matches!(
            deckmaste_semantics::Phasing::In.lower(),
            deckmaste_core::Phasing::In
        );
    }

    #[test]
    fn lowers_phasing_out() {
        assert_matches!(
            deckmaste_semantics::Phasing::Out.lower(),
            deckmaste_core::Phasing::Out
        );
    }

    #[test]
    fn lowers_face_down_spec_listed() {
        assert_matches!(
            deckmaste_semantics::FaceDownSpec::Listed(minimal_face_down_characteristics()).lower(),
            deckmaste_core::FaceDownSpec::Listed(deckmaste_core::FaceDownCharacteristics {
                name: None,
                types: _,
                subtypes: _,
                abilities: _,
                power: None,
                toughness: None
            })
        );
    }

    #[test]
    fn lowers_face_down_characteristics() {
        assert_matches!(
            deckmaste_semantics::FaceDownCharacteristics {
                name: None,
                types: Vec::new(),
                subtypes: Vec::new(),
                abilities: Vec::new(),
                power: None,
                toughness: None
            }
            .lower(),
            deckmaste_core::FaceDownCharacteristics {
                name: None,
                types: _,
                subtypes: _,
                abilities: _,
                power: None,
                toughness: None
            }
        );
    }
}
