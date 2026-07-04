//! The emitted cause-verb ENTAILMENT table ([CR#701] keyword actions),
//! shared with the load-time elaborator: each closed
//! [`deckmaste_core::CauseVerb`]'s fact form — its master-form kind and the
//! zone coordinates it fixes. `Sacrifice` entails a Battlefield→Graveyard
//! `ZoneChange` [CR#701.21a], so "dies" matches a sacrifice STRUCTURALLY
//! and the engine carries no per-verb matching arm; the verb-view forms
//! that remain (`Played`, the `WillDestroy` intent's zone coordinates) read
//! their fact shape from a row here instead of hardcoding it.
//!
//! One source of truth: `crates/deckmaste_cards/tables/entailments.ron` is
//! emitted by `idris/src/EmitTables.idr` and `include_str!`-shared — the
//! elaborator's caps/contradiction checks and the engine's matching can
//! never disagree on a verb's fact form.

use std::collections::HashMap;
use std::sync::LazyLock;

use deckmaste_core::Zone;
use serde::Deserialize;

/// One entailment row: the verb's canonical spelling, its entailed
/// master-form KIND, the zone coordinates it fixes (`None` = the verb
/// leaves the coordinate open), and the `amount` caps column — whether the
/// verb's occurrences fix an amount antecedent (a discard clause's card
/// count feeds "that many", [CR#107.3]); the apply funnel honors exactly
/// the rows the elaborator admits. The remaining caps columns
/// (object/actor) stay elaborator-only (serde skips unknown fields).
#[derive(Debug, Deserialize)]
pub(crate) struct EntailmentRow {
    pub verb: String,
    /// The entailed master-form kind (`"ZoneChange"`, `"Damage"`, …).
    pub kind: String,
    pub from: Option<Zone>,
    pub to: Option<Zone>,
    /// Whether the verb's occurrences carry an amount ([CR#107.3]).
    pub amount: bool,
}

#[derive(Debug, Deserialize)]
struct EntailmentFile {
    rows: Vec<EntailmentRow>,
}

/// The verb's entailment row, if the closed vocabulary carries one — the
/// emitter covers every [`deckmaste_core::CauseVerb`], so a miss means the
/// verb is outside the closed set.
pub(crate) fn entailment(verb: &str) -> Option<&'static EntailmentRow> {
    static TABLE: LazyLock<HashMap<String, EntailmentRow>> = LazyLock::new(|| {
        let file: EntailmentFile =
            ron::from_str(include_str!("../../deckmaste_cards/tables/entailments.ron"))
                .expect("generated entailments table parses");
        file.rows.into_iter().map(|r| (r.verb.clone(), r)).collect()
    });
    TABLE.get(verb)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The engine reads the same emitted rows the elaborator does — the
    /// Play row (the land drop, [CR#701.18a]) fixes `to: Battlefield`, the
    /// Destroy row ([CR#701.8a]) the Battlefield→Graveyard move.
    #[test]
    fn table_parses_and_carries_the_verb_fact_forms() {
        let play = entailment("Play").expect("Play row");
        assert_eq!(play.kind, "ZoneChange");
        assert_eq!(play.from, None);
        assert_eq!(play.to, Some(Zone::Battlefield));

        let destroy = entailment("Destroy").expect("Destroy row");
        assert_eq!(destroy.kind, "ZoneChange");
        assert_eq!(destroy.from, Some(Zone::Battlefield));
        assert_eq!(destroy.to, Some(Zone::Graveyard));

        assert!(entailment("Tap").is_none(), "open fact verbs have no row");
    }
}
