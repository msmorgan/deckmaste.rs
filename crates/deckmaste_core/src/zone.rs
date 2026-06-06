use serde::{Deserialize, Serialize};

/// A game zone (CR 400.1). Vintage-legal scope: no ante.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Zone {
    Battlefield,
    Command,
    Exile,
    Graveyard,
    Hand,
    Library,
    Stack,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zones_round_trip() {
        let options = crate::ron::options();
        for zone in [
            Zone::Battlefield,
            Zone::Command,
            Zone::Exile,
            Zone::Graveyard,
            Zone::Hand,
            Zone::Library,
            Zone::Stack,
        ] {
            let written = options.to_string(&zone).unwrap();
            let parsed: Zone = options.from_str(&written).unwrap();
            assert_eq!(parsed, zone);
        }
    }
}
