---
needs: []
---
**Helpers that still declare a required parameter after a defaulted one.**
Residue of `plugins-v2-keyword-helper-additions` (2026-10-05), which
reordered only helpers whose builtin caller was forced into named form
(`activated`, `damage`, `deonticRule`, `verbedEvent`, predicate
`attachment`). Standard constraints apply.

A positional call can never omit a defaulted binder that precedes a
required one, so these defaults only serve named calls: `abilityGrantFrom`,
`attacksWith`, `choice`, `compare`, `doForEachKind`, `insertPart`,
`moveCounters`, `nthOccurrence`, `oneEachOf`, `paysCost`, `replacement`,
`rollsDice`, `spell` (every card writes `spell(timing: None, instruction:
…)`), `tappedForMana`, `zoneChange` (Eumidian Terrabotanist and Storm Fleet
Spy omit `from` by name). Also: `returnToBattlefield(subject, riders, agent,
from)` is already required-first, but unearth names `agent` and `from` to
skip `riders`; no order serves unearth and persist/undying/earthbend both
positionally. `doIfDone(body, if_did, if_not)`: fading names `if_not`; it is
"if you can't", not the `may` shape `mayOrElse` covers.

Each reorder must leave `cargo xtask expansions` and every card term
byte-identical, with every positional caller rewritten.

## Landing record

The series, oldest first, on the claim `plugins-v2-helper-parameter-order-residue`:

- S1 `wrtqnlumysmp`: fifteen helper reorders and every caller re-spelled.

Every expansion (index included) and every per-card dump (Debug and emitted
Lean, 269 files over canon 127 and testing 6) is byte-identical to the claim
(`diff -r` empty, before and after), so nothing semantic changed. The
finder script (a required binder after a `Default` one, over every
`plugins_v2/*/macros/` declaration) found exactly the ticket's list plus
`meta/CounterKind` (see Deviations).

**Reorders** (required binders first in their old order, defaulted ones after
in theirs; `?` is defaulted):

- `abilityGrantFrom(subject, classes?, source, except?)` → `(subject, source, classes?, except?)`
- `choice(occasion, subject, sort, domain?, disclosure)` → `(occasion, subject, sort, disclosure, domain?)`
- `replacement(event, alternatives?, timing?, replacement, use, limit?)` → `(event, replacement, use, alternatives?, timing?, limit?)`
- `spell(timing?, instruction)` → `(instruction, timing?)`
- `attacksWith(player, defender?, attackers)` → `(player, attackers, defender?)`
- `nthOccurrence(ordinal, per?, event)` → `(ordinal, event, per?)`
- `paysCost(player?, outcome, whose, keyword)` → `(outcome, whose, keyword, player?)`
- `rollsDice(player, batch, sides?, watch)` → `(player, batch, watch, sides?)`
- `tappedForMana(player?, source, type?)` → `(source, player?, type?)`
- `zoneChange(subject, from?, to?, observation)` → `(subject, observation, from?, to?)`
- `doForEachKind(axis, domain?, sort, body)` → `(axis, sort, body, domain?)`
- `insertPart(part, anchor?, count, followed_by?, agent?)` → `(part, count, anchor?, followed_by?, agent?)`
- `moveCounters(amount, kind?, source, destination)` → `(amount, source, destination, kind?)`
- `oneEachOf(roles?, pool)` → `(pool, roles?)`
- `compare(axes?, comparator, bound)` → `(comparator, bound, axes?)`

**Callers changed.**

- `spell`: 37 cards wrote `spell(timing: None, instruction: X)`; each is now
  `spell(X)` (the explicit `timing: None` is the default, dropped).
- Positional callers reordered into the new order, 13 sites: `zoneChange` in
  buyback, madness, recover, earthbend; `compare` in mentor, skulk, soulshift,
  training, transfigure, transmute, proliferate; `attacksWith` in melee;
  `moveCounters` in graft.
- `zoneChange` named to positional, 5 cards: Cirdan the Shipwright, Ingot
  Chewer, Ravener, Thraben Inspector, Trove Tracker.
- Left named, because positional would skip the middle default `from` to give
  `to`: Eumidian Terrabotanist and Storm Fleet Spy.
- No caller of `abilityGrantFrom`, `choice`, `replacement`, `nthOccurrence`,
  `paysCost`, `rollsDice`, `tappedForMana`, `doForEachKind`, `insertPart` or
  `oneEachOf` exists in `plugins_v2/` outside their own declarations, so
  those reorders touch only the declaration.

**Proof.** Standard constraints apply, plus deltas.

- `cargo xtask lean-check`: canon 127/127, testing 6/6.
- `cargo xtask facts check`: up to date. Cite checks: 0 non-compliant, 0 stale.
- `cargo xtask gate --changed`: `cargo test -p deckmaste_construction_core -p
  deckmaste_lexical_source -p deckmaste_semantics_v2 -p
  deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask`, run with
  `--no-fail-fast`: 96 binaries, 1220 passed, 0 failed, 1 ignored (ignored
  before this landing).

**Tests.** Restored 0, re-spelled 0, ignored 0, added 0, removed 0. The
`spell(timing: None, instruction: …)` RON in
`crates/xtask/tests/plugins_v2_declarations.rs` is a named call, still valid
under the new order, and passes unchanged; it was left as written.

**Deviations and additions.**
1. `meta/CounterKind` (`name, params, spelling, grammar, compound_stem?,
   compound_onset, holder?, confers?`) also matches the pattern but is a meta
   declaration written as a record of named fields by every counter, not a
   helper called positionally; left alone.
2. `returnToBattlefield`, `unearth` and `doIfDone`: no change, as the ticket
   says. `doIfDone` is already required-first; no order serves unearth and
   persist/undying/earthbend positionally at once.
3. Reorders are by the owner's standing decision (2026-10-05).

**Not applicable.** English grammar corpus gates, coverage lock and
word-naming checkers (no English crate changed in code or data), CR
citations added (none), glossary gaps (none), performance advisory (no
compiler change).

**Post-refresh.** `kata refresh` was a no-op: the claim already sat on the
current default line, so the tree is the one measured above (gate 96
binaries, 1220 passed, 0 failed, 1 ignored; byte-identity, lean-check 127/127
and 6/6, facts and cite checks as stated). No conflicts on `trunk()..@`.
