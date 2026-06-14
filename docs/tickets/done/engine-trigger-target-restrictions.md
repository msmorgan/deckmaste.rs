---
needs: []
---
Trigger placement (`trigger.rs:701`, `place_one_trigger`) computes legal target sets
via `legal_targets` but skips the `Cant(Target)` / hexproof / protection filtering
(`cant_target_rows` / `target_forbidden_by`) that the announce path applies
(`cast.rs:256`), so a triggered ability can illegally target a hexproof permanent.
Extract a shared `surface_target_choice(player, specs, targeting_id)` used by both
`announce_targets` and `place_one_trigger` — this dedups the `ChooseTargets`
construction AND closes the targeting gap in one move. Correctness bug,
release-blocker (publish-prep).
