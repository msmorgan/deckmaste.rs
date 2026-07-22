//! The temporal vocabulary (mtg-rules `temporal.md`): the timing/lookback
//! SPLIT, fixed duration markers, and the lock-point axis. Two value types,
//! no shared enum: [`Timing`] is a permission window ("Activate only
//! [timing]", flash), [`Lookback`] a history window (`Condition::Happened`,
//! `Count::EventCount`, `EventFilter::Within`) — never conflating the
//! duration and history readings of "this turn".

use serde::Deserialize;
use serde::Serialize;

use crate::Expand;
use crate::PhaseStep;
use crate::WhoseTurn;

/// A timing window — a predicate over the current turn-structure position.
/// Closed rules vocabulary: new windows are CR concepts, not card macros.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum Timing {
    /// Any time the player has priority (instant timing, [CR#117.1a];
    /// flash rides this as a cast-permission window, [CR#702.8a]).
    InstantSpeed,
    /// Main phase of the player's own turn, stack empty, priority held
    /// ([CR#117.1a,307.1] — "any time you could cast a sorcery").
    SorcerySpeed,
    /// During the named player-relation's turn ([CR#500.1]).
    DuringTurn(WhoseTurn),
    /// During the named step/phase of the named player-relation's turn
    /// ("Activate only during the upkeep step of the card's owner",
    /// forecast-style, [CR#702.57b]).
    DuringStep(PhaseStep, WhoseTurn),
}

/// A history-lookback window ([CR#608.2i]) — how far back an event query
/// scans. Split from [`Timing`]: a lookback never gates a permission, a
/// timing never bounds a scan. Every history position carries one
/// explicitly (no silent default window).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum Lookback {
    /// Since the start of the current turn (morbid, raid — [CR#608.2i]).
    ThisTurn,
    /// The whole current game so far ("a creature died this game",
    /// "spells you've cast this game" [CR#608.2i]). Full history.
    ThisGame,
    /// During the previous turn ([CR#500.1] turn boundaries).
    LastTurn,
    /// Since this combat phase began ([CR#506.1]).
    ThisCombat,
    /// Since the current step began ([CR#500.1]).
    ThisStep,
    /// Since the watcher's controller's last such step — the
    /// `SinceYourLastUpkeep` family ([CR#603.2b] step anchoring).
    SinceYour(PhaseStep),
}

/// A fixed turn-structure end marker for `Duration::FixedUntil`
/// ([CR#611.2a]): "until end of turn" sweeps in cleanup ([CR#514.2]),
/// "until end of combat" at the combat phase's end ([CR#500.5a,511.2]).
/// Grows by card demand (end of your next turn, …).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum TurnMarker {
    EndOfTurn,
    EndOfCombat,
    YourNextTurn,
}

/// The lock axis (mtg-rules `temporal.md` §3): when a recorded choice or
/// value stops responding to the world. Invariant the engine grows into:
/// every stored number or set carries either a `LockPoint` (it is a
/// snapshot, with at most one sanctioned recheck — targets at resolution,
/// [CR#608.2b]; intervening-if, [CR#603.4]) or a re-evaluation rule (it is
/// a view over live state, [CR#611.3a]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum LockPoint {
    /// Targets, modes, costs-as-intended, spell/ability X
    /// ([CR#601.2b..601.2d,107.3a]).
    Announce,
    /// Triggered-ability modes ([CR#603.3c]).
    StackPlacement,
    /// The locked total cost ([CR#601.2f]).
    TotalCost,
    /// Special-action X, payment-stage substitutions
    /// ([CR#107.3d,601.2g..601.2h]).
    Payment,
    /// A resolved continuous effect's affected-object set ([CR#611.2c]).
    EffectBegin,
    /// Copiable values ([CR#707.2b]).
    CopyCreation,
    /// Combat declaration sets — attackers/blockers lock as declared
    /// ([CR#508.1a,509.1a]; the choices.md lock-stage column).
    Declaration,
    /// Pre-game choices: first turn, mulligans + bottoming, companion
    /// ([CR#103.1,103.5,103.2b]).
    PreGame,
    /// Resolution-stage choices, ward-style X, untargeted division
    /// ([CR#608.2d,702.21b]).
    Resolution,
    /// Never locked — continuously re-evaluated ([CR#611.3a] statics).
    Never,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read<T: for<'de> serde::Deserialize<'de>>(source: &str) -> T {
        crate::ron::options().from_str(source).unwrap()
    }

    /// Bare-identifier RON spellings for the closed vocabularies; the
    /// parameterized windows read flat. `Timing` and `Lookback` are two
    /// types — "this turn" only exists on the lookback side.
    #[test]
    fn timings_and_lookbacks_read() {
        assert_eq!(read::<Timing>("InstantSpeed"), Timing::InstantSpeed);
        assert_eq!(read::<Timing>("SorcerySpeed"), Timing::SorcerySpeed);
        assert_eq!(
            read::<Timing>("DuringTurn(Your)"),
            Timing::DuringTurn(WhoseTurn::Your),
        );
        assert_eq!(
            read::<Timing>("DuringStep(Beginning(Upkeep), Your)"),
            Timing::DuringStep(
                PhaseStep::Beginning(crate::BeginningStep::Upkeep),
                WhoseTurn::Your
            ),
        );
        assert_eq!(read::<Lookback>("ThisTurn"), Lookback::ThisTurn);
        assert_eq!(read::<Lookback>("ThisGame"), Lookback::ThisGame);
        assert_eq!(read::<Lookback>("LastTurn"), Lookback::LastTurn);
        assert_eq!(read::<Lookback>("ThisCombat"), Lookback::ThisCombat);
        assert_eq!(read::<Lookback>("ThisStep"), Lookback::ThisStep);
        assert_eq!(
            read::<Lookback>("SinceYour(Beginning(Upkeep))"),
            Lookback::SinceYour(PhaseStep::Beginning(crate::BeginningStep::Upkeep)),
        );
        let err = crate::ron::options()
            .from_str::<Timing>("ThisTurn")
            .unwrap_err();
        assert!(
            err.to_string().contains("ThisTurn"),
            "expected a timing mismatch for `ThisTurn`, got: {err}"
        );
    }

    #[test]
    fn markers_and_locks_round_trip() {
        for marker in [
            TurnMarker::EndOfTurn,
            TurnMarker::EndOfCombat,
            TurnMarker::YourNextTurn,
        ] {
            let written = crate::ron::options().to_string(&marker).unwrap();
            assert_eq!(read::<TurnMarker>(&written), marker);
        }
        let written = crate::ron::options()
            .to_string(&LockPoint::Announce)
            .unwrap();
        assert_eq!(written, "Announce");
        assert_eq!(read::<LockPoint>(&written), LockPoint::Announce);
    }
}
