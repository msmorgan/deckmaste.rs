---
needs: []
---
Grow the noncanon matchup decks with now-live keywords (fliers/reach, menace,
defender, hexproof) so the 50-game gate regression-tests keyword interactions
systemically rather than per-test. Lives in the noncanon workspace/feature.

## Triage note (batch2 worker, 2026-06-14): mis-tiered — NOT claimed, left in planned/

Blocked on two counts; surfaced rather than guessed:

1. **Premise not yet true — combat keywords are not live.** The keywords this
   ticket would exercise via the 50-game gate (fliers/reach, menace, defender)
   have NO effect on attack/block legality in the engine: `legal.rs`
   (`legal_blockers`) and `combat.rs` contain zero keyword-keyed evasion /
   block-requirement logic — the only `Flying` mention in `combat.rs` is a test
   assertion that a creature does NOT have it. Adding creatures with these
   keywords to the matchup decks would exercise no interaction, so the
   regression goal is unreachable until combat keyword restrictions
   (flying/reach evasion, menace ≥2 blockers, defender can't-attack) are
   implemented — a substantial engine feature, not pinned by this ticket.
   (Hexproof's targeting `Cant(Target)` filtering IS live in `legal.rs`, but
   that is the minority of the ticket and not a combat-deck regression.)
2. **Wrong mechanism for a default-line batch claim.** The ticket "lives in the
   noncanon workspace/feature" — the long-lived non-mainline `noncanon` branch /
   `../noncanon` workspace — not a fresh default-line claim that
   `integrate`s into trunk. It should be picked up inside that feature once the
   combat keywords land, not via this batch.

Re-tier (e.g. depend on an engine combat-keyword-restrictions ticket) and route
to the noncanon feature before claiming.
