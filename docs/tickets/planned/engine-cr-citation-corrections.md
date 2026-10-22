---
needs: []
---
**Correct rule citations and rules claims in code comments where the behaviour is
already right.** Comment, doc and RON-comment edits only; no behaviour change.
After the edits, `cargo xtask cite check` and `cite audit --diff` per CLAUDE.md.
Tickets in `docs/tickets/done/` that repeat a wrong cite are historical records and
are not edited.

Provenance: `docs/evidence/mtg-rules-review-2026-10-21/departures-engine.md` #23,
#24, #25, #27, #32, #38. Line numbers are as of 2026-10-21; locate by text.

## 1. Doubling Season has no "trigger half" (#23)

`TriggerMultiplier` is described as covering "the trigger half of Doubling
Season". Doubling Season is two replacement effects ([CR#614.16]: "if an effect
would create one or more tokens", "put one or more counters"); a trigger
multiplier is [CR#603.2d] ("An ability may state that a triggered ability
triggers additional times"). Replace the phrase with a real [CR#603.2d] example
(Elesh Norn, Mother of Machines' trigger clause) at:
`crates/deckmaste_core/src/continuous.rs:331`,
`crates/deckmaste_semantics/src/continuous.rs:351`,
`crates/deckmaste_engine/src/trigger.rs:1110` and `:1164`.

## 2. Opponents are [CR#102.2,102.3], not [CR#102.1] or [CR#102.4] (#24)

[CR#102.1] defines player and active/nonactive player; [CR#102.4] defines "your
team". Opponents: [CR#102.2] ("In a two-player game, a player's opponent is the
other player") and [CR#102.3] ("the player's opponents are all players not on
their team"). Cite [CR#102.2,102.3] for "opponents" (keep [CR#102.1] only on a "live
players" clause, and [CR#810.1] as the team-game instance); [CR#102.3,102.4] for
`same_team`; [CR#102.3] in place of the whole-section Two-Headed Giant cite for
"neither you nor an opponent". Sites:

- `crates/deckmaste_core/src/count.rs:201`, `:203`;
  `crates/deckmaste_semantics/src/count.rs:193`, `:195`
- `crates/deckmaste_engine/src/state.rs:808` (`same_team`), `:828-829`;
  `resolve/count.rs:225-226`; `layer.rs:1412`
- `crates/deckmaste_semantics/src/reference.rs:135`;
  `crates/deckmaste_legacy_render/src/render/fragment.rs:97`;
  `crates/deckmaste_plugin/src/idris_emit.rs:1123`;
  `crates/deckmaste_migrations/src/parsers/replacement.rs:198`, `:705`
- `crates/deckmaste_core/src/filter.rs:259`,
  `crates/deckmaste_semantics/src/filter.rs:163` (the section cite)
- `plugins/testing/cards/Opponent-count and life enters-tapped.ron:1`, `:12`

Leave the correct "player" cites of [CR#102.1] (`deckmaste_core/src/count.rs:146`,
`deckmaste_semantics/src/count.rs:584`).

## 3. Glaring Spotlight sees through plain hexproof (#25)

The obstacle is hexproof ([CR#702.11b]); [CR#702.11e] is the rule that extends an
"as though it didn't have hexproof" permission to "hexproof from [quality]".
Replace `[CR#702.11d]` with `[CR#702.11b,702.11e]` (keeping [CR#609.4] where the
as-though frame is cited) at `crates/deckmaste_core/src/deontic.rs:58`,
`crates/deckmaste_semantics/src/deontic.rs:61`,
`plugins/canon/cards/Glaring Spotlight.ron:13`,
`crates/deckmaste_engine/tests/stack.rs:282`. The other [CR#702.11d] cites (on
hexproof-from itself) are correct.

## 4. A madness discard is [CR#702.35a,614.6], not [CR#701.9c] (#27)

[CR#701.9c] covers only a discard redirected to a hidden zone ("characteristics
are considered to be undefined"). Madness exiles face up: [CR#702.35a] "discards
it, but exiles it instead", and [CR#614.6] "A modified event occurs instead,
which may in turn trigger abilities." Cite [CR#702.35a,614.6] for "a redirected
madness discard still fires 'whenever you discard'", adding [CR#702.35c,400.7k]
where the exiled card is read back; keep [CR#701.9c] only for hidden-zone redirects.
Sites: `crates/deckmaste_core/src/event.rs:353`,
`crates/deckmaste_semantics/src/event.rs:368`,
`crates/deckmaste_engine/src/eval.rs:898`, `replace_registry.rs:1296`,
`resolve/action.rs:643, 801, 2209, 2218, 2394, 2428, 2437, 2738, 2810, 3046,
6385, 6452`, `agenda.rs:24, 29`, `step/mod.rs:988`,
`crates/deckmaste_plugin/src/idris_emit.rs:3042`,
`plugins/builtin/macros/keyword/Madness.ron:30`.

The mill half of the `agenda.rs:24/29` and `step/mod.rs:988` cites
([CR#701.17c], "a redirected card is still … milled") is wrong too: [CR#701.17c]
is about finding a milled card in a public zone. Cite [CR#701.17a,614.6].

## 5. "Destroyed" has three state-based routes named in [CR#701.8b] (#32)

[CR#701.8b]: destruction comes only from "an effect that uses the word
'destroy'" or the SBAs "that check for lethal damage … or damage
from a source with deathtouch" ([CR#704.5g,704.5h]). Reword "the other is the
lethal-damage SBA" to "state-based destruction (lethal damage [CR#704.5g] or
deathtouch damage [CR#704.5h])", and "the lethal-damage SBA's deathtouch
clause" to "the deathtouch-damage SBA ([CR#704.5h])". Sites:
`crates/deckmaste_engine/src/event.rs:95-96`, `docs/conformance.md:67`,
`crates/deckmaste_core/src/filter.rs:228`,
`crates/deckmaste_semantics/src/reference.rs:240`,
`plugins/builtin/macros/event/Destroyed.ron:2-3`,
`plugins/builtin/macros/keyword/Indestructible.ron:3-4`,
`plugins/builtin/macros/effect/Regenerate.ron:19-20`,
`crates/deckmaste_engine/src/sba.rs:307`,
`crates/deckmaste_engine/src/trigger.rs:2492`, `:2504`. The "exactly two causes"
lines that mean effect vs SBA (`deckmaste_core/src/event.rs:271`,
`deckmaste_semantics/src/event.rs:286`, `deckmaste_engine/src/eval.rs:1318`) stay.

## 6. One Occurrence is [CR#603.2c,700.1], not [CR#603.3b] (#38)

[CR#603.3b] governs putting waiting triggers on the stack (APNAP). One Occurrence
for `OneOrMore` matching is [CR#603.2c] ("it can trigger repeatedly if one event
contains multiple occurrences") and [CR#700.1] ("One 'happening' may be treated
as a single event by one ability and as multiple events by another");
"SBAs after the whole batch" is [CR#704.3]; simultaneous mill and destroy-all
batches are [CR#608.2f] (with [CR#701.17a] for mill). Sites:
`crates/deckmaste_engine/src/step/act.rs:123`, `:236`;
`resolve/action.rs:973`; `step/mod.rs:2568`;
`plugins/builtin/macros/ability/Chapter.ron:4` (also [CR#714.2b]). The occurrence
count is settled by each trigger's wording, so it is not registered as
underdetermined. Leave the correct trigger-ordering [CR#603.3b] cites (trigger.rs
`:605, 1206, 1220, 1240, 1259, 5358`, `decide/mod.rs:274, 715`, and the rest).

## 7. A departed recipient is a new object (docs #42)

`crates/deckmaste_engine/src/resolve/player_action.rs:551-553` (the `PutCounters`
arm): "[CR#608.2b]: an illegal target or departed `This` recipient cannot be
affected … (Heroes' Bane's power)". [CR#608.2b] covers the illegal target; the
departed untargeted recipient is unaffected because it "becomes a new object with no
memory of, or relation to, its previous existence" ([CR#400.7]), and the quantity
reads the source's last known information ([CR#608.2h]). Cite
[CR#608.2b,400.7] (adding [CR#608.2h] on the quantity clause). Provenance:
`docs/evidence/mtg-rules-review-2026-10-21/departures-docs.md` #42.
