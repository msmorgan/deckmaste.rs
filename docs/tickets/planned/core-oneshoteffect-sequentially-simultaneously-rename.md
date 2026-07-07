---
needs: []
---
**Rename the one-shot-effect vocabulary to match the Idris north-star's intent:
`Effect` → `OneShotEffect`, `Sequence` → `Sequentially`, `Simultaneous` →
`Simultaneously`.** A pure, semantics-preserving rename across Rust + Idris +
emitter + every card. No behavior change — the two variants already exist and
`resolve.rs` already treats them exactly as the new names describe.

## Rename

- Rust `enum Effect` → `OneShotEffect` (`crates/deckmaste_core/src/effect.rs`),
  matching Idris's `OneShotEffect` type name.
- `Effect::Sequence(Vec<_>)` → `Sequentially(Vec<_>)` — explicit "then", ordered
  sub-effects; each member reads state updated by its predecessors ([CR#608.2c]).
  In the engine this is: schedule each member as its own `WorkItem::RunEffect`,
  so its events flow through the machinery before the next member runs.
- `Effect::Simultaneous(Vec<_>)` → `Simultaneously(Vec<_>)` — one pre-application
  snapshot, one batch / one timestamp, SBAs after the whole batch, all-or-nothing
  ([CR#608.2f], [CR#701.12a]). In the engine this is: collect every member's
  events into one `Occurrence::Batch` emitted together.
- Rename the Idris constructors to match, and update the RON macro/card spellings
  (`Sequence([...])`, the exchange-family bodies) and the emitter in lock-step.

## NOT in scope (explicit decision)

- **No reclassification of existing bodies.** `Sequentially` stays the DEFAULT.
  The "no 'then' → `Simultaneously`" heuristic is REJECTED: it would silently
  break every card whose later clause reads an earlier clause's result — "draw
  two cards. Discard a card." (discard must see the drawn cards); "create a
  token, sacrifice it"; "exile the top card, you may play it". Absence of "then"
  does NOT imply simultaneity — most multi-instruction bodies are sequential.
- `Simultaneously` is the *marked* form, correct only for the genuinely
  simultaneous shapes (one action distributed over many objects/players,
  [CR#608.2f]; exchanges, [CR#701.12a]; fight, [CR#701.14a]; text that says
  "simultaneously"). Flipping specific existing bodies to it is a **separate
  per-card audit ticket** if ever wanted — not this one.

## Also un-gate `Simultaneously` for verb bodies (small, rides along)

The resolve arm currently `todo!()`s on non-verb / choice-bearing `Simultaneous`
members ("restricted to the exchange-family macros' bodies"). Verb, choice-free
members already work; generalizing beyond the exchange allow-list is what the
fight decomposition (`core-fight-primitive-to-macro`) needs. Either land the
un-gate here or leave it to that ticket — but note the dependency so they don't
collide.

## Done

- No `Effect::Sequence`, `Effect::Simultaneous`, or `enum Effect` remain
  (`grep` clean); Idris + emitter + all cards re-spelled.
- parse⇄render round-trip holds; idris-check count unchanged; render fidelity
  holds; `cargo test --workspace` green.
