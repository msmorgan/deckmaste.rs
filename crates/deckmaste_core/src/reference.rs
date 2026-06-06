use serde::{Deserialize, Serialize};

/// A bound variable: a value fixed earlier (at announce, by the rules of
/// the position, or by a binder) and referenced later. References name
/// *objects*; amounts live in Quantity (future module).
///
/// Players are objects — `You`, `ControllerOf`, `OwnerOf` resolve to
/// player objects.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Reference {
    /// The object this ability is printed on / the resolving spell.
    This,
    /// The controller of this ability (CR 109.5).
    You,
    /// The nth target this ability announced (CR 115.3, 601.2c).
    Target(usize),
    ControllerOf(Box<Reference>),
    OwnerOf(Box<Reference>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn references_round_trip() {
        let options = crate::ron::options();
        for reference in [
            Reference::This,
            Reference::You,
            Reference::Target(1),
            Reference::ControllerOf(Box::new(Reference::Target(0))),
            Reference::OwnerOf(Box::new(Reference::Target(0))),
        ] {
            let written = options.to_string(&reference).unwrap();
            let parsed: Reference = options.from_str(&written).unwrap();
            assert_eq!(parsed, reference);
        }
    }

    #[test]
    fn target_index_reads_bare() {
        let parsed: Reference = crate::ron::options().from_str("Target(0)").unwrap();
        assert_eq!(parsed, Reference::Target(0));
    }
}
