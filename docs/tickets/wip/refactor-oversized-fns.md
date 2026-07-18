---
needs: []
design: true
---
Three functions blow well past the `too_many_lines` threshold (150, set in
`clippy.toml`) not because they are irreducible per-variant dispatch — the
engine's genuine flat dispatchers (`resolve_object`, `run_effect`,
`action_items`, `player_action_items`, `event_matches`) keep a permanent
`#[expect(.., reason = "one arm per ...; splitting would scatter the dispatch")]`
— but because they have grown real internal structure that wants decomposing.
(The genuine flat dispatchers that are still over the threshold — `run_effect`,
`player_action_items` — keep a permanent `#[expect(.., reason = "one arm per
...; splitting would scatter the dispatch")]`; the rest now fit under the
raised threshold and carry no annotation.)

Each of the three below currently carries a placeholder
`#[expect(clippy::too_many_lines, reason = "... tracked in
refactor-oversized-fns")]`. `#[expect]` is self-clearing: once a
function drops under the threshold its expectation goes unfulfilled and clippy
forces the attribute's removal, so these placeholders cannot silently outlive
the refactor.

- `decide::submit_decision` (953 lines as of 2026-07-16; ~635 when this ticket
  was written) — split by decision kind: priority / cast-procedure / combat.
  The outer `match (pending, decision)` is the dispatch; each arm's body is the
  candidate to extract into a per-kind handler method. Three arms are full
  procedures now (ChooseTargets ~134 lines, DeclareBlockers ~128, YesNo ~106).
- `step::apply` (713 lines as of 2026-07-16; ~400 at writing) — split by
  subsystem: stack / zone-change / player. Fat arms: DamageDealt ~98,
  TriggerFired ~94, Copied ~59. (The action-driven zone-change collapse —
  draw / land / discard → `ZoneWillChange` — is already done; that part need
  not move.)
- `tui::interactive_loop` (270 lines as of 2026-07-16; ~243 at writing) —
  extract the key-dispatch `match` (and the popup/navigation handling) out of
  the event loop into a handler that maps a key event to an action, leaving the
  loop itself short.

Re-triaged `maybe/` → `planned/` 2026-07-16: a suppression audit found the two
engine functions had grown 50–78% past the sizes recorded above while parked —
the placeholder expects were working as anesthesia, not tracking.

`[design]` (SETTLED 2026-07-17): decompose via the **command pattern** — each
dispatched enum variant gets a payload struct + handler, so the outer match
dissolves instead of scattering into grouped helper methods. Full design in
`docs/superpowers/specs/2026-07-17-refactor-oversized-fns-command-pattern-design.md`.

- `submit_decision`: `PendingDecision` → enum of payload structs implementing
  `DecisionHandler::resolve(self, &mut GameState, Decision) -> Result<(),
  DecisionError>`. Handlers live in a new `decide/pending/` dir-module grouped by
  subsystem (priority/cast/combat/mana/choice). Contract preserved: an error
  leaves `self.pending` open (clone → clear-on-success).
- `interactive_loop`: move the key dispatch onto `Interaction::on_key(key,
  &mut KeyCtx) -> KeyOutcome`, and the chrome onto `handle_global_key`; the loop
  becomes render→read→global-key→interaction-key→apply.
- `apply`: `GameEvent` → enum of payload structs implementing `EventApply`;
  handlers in a new `step/` dir-module; the 270-line `Act{committed:false}` arm
  splits per-verb. Done LAST (646 `GameEvent::` sites rewrite mechanically).

Serde-safe (none of the three enums derive Serialize); no behavioral change;
pinned by the existing suite + `cargo xtask cite check` (rule-citation comments
ride the moved arms verbatim).
