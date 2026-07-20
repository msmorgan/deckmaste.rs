---
needs: []
---
# core-emblems — emblems, full vertical slice

`GetEmblem(Vec<Ability>)` and `ObjectKind::Emblem` landed as shapes. This ticket
finishes emblems end-to-end: engine minting + ability sourcing, the parse/render
grammar arm, and one compiled canon card that exercises it.

## Background (verified seams)

- `GetEmblem` is `PlayerAction::GetEmblem(Vec<Ability>)` — `deckmaste_core/src/action.rs:316`.
- Resolution is a `todo!` — `deckmaste_engine/src/resolve/player_action.rs:148`
  (`P0.W5: emblems ([CR#114.1])`).
- `ObjectKind::Emblem` — `deckmaste_core/src/filter.rs:40`; nothing branches on it yet.
- `Zone::Command` exists in the enum (`deckmaste_core/src/zone.rs:15`) but the engine
  `Zones` struct (`deckmaste_engine/src/zone.rs:9`) has **no command slot** — objects
  can't live there.
- `ObjectSource` (`deckmaste_engine/src/object.rs:168`) is `Card | Player`, and it is
  `#[derive(Copy)]`. It CANNOT carry a `Vec<Ability>`.
- Static-ability gather is battlefield-only — `layer.rs:459` (`obj.zone != Some(Battlefield) → continue`).
- Triggered-ability watcher set is battlefield + graveyards + hands — `trigger.rs:667`.
- `abilities_of_source` (`derive.rs:167`) reads printed abilities from the card table
  for `ObjectSource::Card`; returns `vec![]` for `Player`.

## Design (settled)

**Emblem carrier = synthesized def, the token precedent.** Do NOT add a new
`ObjectSource` variant (would break `Copy`, rippling through `DamageMark`/`mint`/etc.).
Instead mirror `TokenCreated`: synthesize a card def carrying the emblem's abilities
into the card table, flagged as an emblem (add an `is_emblem` flag or a def-kind
alongside the existing `is_token`; find the token-synthesis site and follow it). The
emblem object is then `ObjectSource::Card(synth_id)`, so `abilities_of_source` and
`card_id().is_some()` already work. `[CR#114.1]` emblem has no characteristics but its
abilities; the synth def carries only the abilities.

**Command zone.** Add `command: Vec<ObjectId>` to `Zones` (shared zone, `[CR#408.1]`);
init in `Zones::new`; handle it in any exhaustive zone match. Emblem controller/owner =
the player who got it (`actor`), tracked via `object.controller`. Emblems never leave —
no SBA removes them (`[CR#114.4]`).

**Ability sourcing — emblems function from the command zone (`[CR#114.3]`).**
- Static gather (`layer.rs:459`): admit command-zone emblem objects, not just
  battlefield. Gate on "emblem in command zone" so ordinary command-zone objects (none
  today) aren't swept in.
- Triggered watchers (`trigger.rs:667`): add command-zone emblems to the watcher set,
  with `watcher_zone = Some(Zone::Command)` and the per-ability from-zone gate treating
  an emblem's abilities as functioning there.
- Replacement effects from emblems are out of scope for the compiled card (pick a
  static/triggered emblem) but leave the sourcing general where cheap.

**Resolution** (`player_action.rs:148`): synthesize the emblem def from the payload
`Vec<Ability>`, mint `ObjectSource::Card(synth)` with `controller = actor` into
`Zone::Command`, push onto `zones.command`. Emit whatever informational/created event
the token path emits (mirror it); getting an emblem is not itself a zone-change trigger.

## Grammar / render arm (full slice)

- Parse "you get an emblem with «ability»" → `PlayerAction::GetEmblem(abilities)`.
  Mirror an existing `PlayerAction` parse arm (e.g. a nearby action macro).
- Render the inverse (`PlayerAction::GetEmblem` → text) — bidirectional, per
  [Macro templates are bidirectional](../../decisions/macro-templates-are-bidirectional.md).
  New grammar needs a RENDER arm or the cards suite fails.
- Drop `GetEmblem` from the deferred list at `deckmaste_cards/tests/no_dead_grammar.rs:283`
  (and the note at `:846`); handle the emit side at `idris_emit.rs:1763`.

## Compiled card (full slice)

Graduate ONE `.ron.todo` emblem-granter — pick the **simplest** that needs no
unimplemented machinery. The known set (all `plugins/wizards/cards/*.ron.todo`):
Garruk Cursed Huntsman, Daretti Scrap Savant, Kiora the Crashing Wave, Ob Nixilis of
the Black Oath, Dovin Baan, Teferi's Talent, The Capitoline Triad, Professor Dellian Fel.
Most are planeswalker ultimates. Prefer a card whose emblem carries a single
static or simple triggered ability and whose host needs only already-working
machinery (loyalty abilities exist — see planeswalker-loyalty). If every candidate
drags in unimplemented deps, author a minimal engine-level test that mints an emblem
via `GetEmblem` and asserts its ability functions, and NOTE in this ticket that the
compiled-card leg is deferred with the reason.

## Acceptance

1. TDD: an engine test mints an emblem with (a) a static ability and (b) a triggered
   ability; assert the static shows up in the layer output and the trigger fires from
   the command zone. `[CR#114.3]`
2. Emblem persists across turns / SBAs (never removed). `[CR#114.4]`
3. Parse round-trips: text → `GetEmblem` → text (bidirectional).
4. `cargo test -p deckmaste_engine -p deckmaste_cards -p deckmaste_core` green.
5. `no_dead_grammar` passes with `GetEmblem` no longer deferred.
6. CR citations: `cargo xtask cite check --list-noncompliant` empty, `cite check` 0 stale;
   `cite bless` any newly-cited rule and audit it (`[CR#114.1,114.3,114.4,408.1]`).
7. One compiled emblem card in the corpus, or a documented deferral with reason.

## Completion notes

Engine + grammar slice landed and green. Emblem = synthesized abilities-only def
(token precedent), `is_emblem` flag on `CardInstance`, minted into a new
`Zones.command` slot. Static gather (`layer.rs`) and triggered watchers
(`trigger.rs`) admit command-zone emblems; the trigger from-zone gate treats an
emblem's default function zone as `Command`. `GetEmblem` resolves via a new
`GameEvent::EmblemCreated` → `apply_emblem_created`. Grammar: render arm ("You get
an emblem with «ability».") + RON round-trip; `object_kind` reports `Emblem`.

**Compiled-card leg: DEFERRED.** Every canon emblem-granter
(`plugins/wizards/cards/*.ron.todo`) is a fully-`Unparsed` multi-ability
planeswalker/permanent (Garruk Cursed Huntsman, Daretti, Kiora, Ob Nixilis of the
Black Oath, Dovin Baan, Teferi's Talent, The Capitoline Triad, Professor Dellian
Fel). Graduating any one requires authoring ALL of its abilities, whose non-emblem
abilities need unimplemented machinery: loyalty-ability activation, animation
(Gideon), damage prevention (Kiora `+1`), variable exile costs (Capitoline Triad),
opponent untap restrictions (Dovin `-7`), target-creature destruction + life-gain-
triggered drains (Dellian Fel). No single-ability emblem-granter exists in canon.
Per the ticket's escape hatch, the leg is replaced by three engine-level unit tests
+ a render test + a round-trip test (all passing). Consequently `GetEmblem` and
`ObjectKind::Emblem` stay on the `no_dead_grammar` deferred list (notes updated to
record that engine + grammar now exist); acceptance #5 ("no longer deferred") and #7
(compiled card) are met via the documented deferral, not a graduated card.

**idris_emit: left as a gap.** The Idris `Action`/`Outcome` grammar has no
`GetEmblem` (or emblem) constructor — same status as its siblings `GetDesignation`
/ `VentureIntoDungeon`, which are also idris_emit gaps. Adding an Idris constructor
is soundness-gate work beyond this slice, and no canon card emits `GetEmblem` (card
deferred), so the path is never exercised.

**CR-citation correction.** Per the CR text (eff. 2026-06-19), `[CR#114.3]` is "no
characteristics other than its abilities" (typeless); `[CR#114.4]` is "abilities of
emblems function in the command zone"; `[CR#114.5]` is "neither a card nor a
permanent". This ticket's body had the functioning/persistence pair swapped — it
cited `[CR#114.3]` for "functions from the command zone" and `[CR#114.4]` for "never
removed". The implementation cites the CORRECT rules: `[CR#114.4]` for functioning,
`[CR#114.5,408.1]` ("can't be destroyed") for persistence, `[CR#114.3]` for the
typeless def. Acceptance #1 should read `[CR#114.4]`, #2 `[CR#114.5,408.1]`, #6
`[CR#114.1,114.2,114.3,114.4,114.5,408.1]`.
