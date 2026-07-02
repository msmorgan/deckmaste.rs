---
needs: []
---
**The load-time elaborator (`deckmaste_cards::elaborate`) + the Idris table
emitter, landed together.** One walk over each card's grammar tree validates
every binding-context rule under the CURRENT grammar (zero node changes), driven
by tables generated from `idris/src/Core.idr`'s total functions. The two halves
land in ONE ticket deliberately: if the Rust rules were hand-written first and
the emitter followed, the two implementations would drift exactly the way the
old hand-mirrored Idris/Rust grammars did. No checker rule without a table row
and a fixture pair.

**Position:** this supersedes the "soundness is structural, no separate checker"
stance recorded in [[core-bindable-unification]] — the load-time elaborator is
required because serialization erases structural guarantees and the corpus is
runtime-loaded RON; a shape the Rust type system can't refuse must be refused at
load, never at resolve time.

## The walker

`deckmaste_cards::elaborate(card) -> Result<ElabCard, ElabError>`: one walk, one
context, table-driven.

```rust
struct Ctx {                       // elaborator state — never serialized
    stack: Vec<Ante>,              // most-recent-first antecedent stack
    may_target: bool,              // positional discipline for Targeted
    has_x: bool,                   // {X} in the carrying cost, or where_x
}
struct Ante { sort: Sort, card: Cardinality /*One|Many*/, site: Site,
              expected_zone: Option<Zone> }
enum Site { TargetSlot(usize), Product, EventRole(Role), Loop, Allot,
            Chosen, Label(Ident) }
```

Output is `ElabCard` (elaborated IR). In this ticket it runs behind the existing
`cargo xtask validate` (crates/xtask/src/validate.rs) over a plugin dir; wiring
into `Plugin::load` with the deny/warn rollout is [[cards-elab-load-gate]].
Existing canon must elaborate clean — explicit `Target(0)`/event-role
references elaborate trivially.

## Rule families and error codes

Every rule has a stable error code and a TWIN reject fixture under
`crates/deckmaste_cards/tests/reject/` that must fail with exactly that code.
The full code space is laid out now; families whose grammar nodes don't exist
yet activate in the ticket that mints the node.

1. `E-BIND-*` — reference resolution and unbound anaphor reads
   (`It`/`That`/`They`/`ThatMany`/`Allotment`/`ChosenNumber`/`X`/event roles);
   later also the ambiguity gate of [[core-anaphor-surface]].
2. `E-CAPS-*` — refinement atoms and body anaphora checked against the event
   caps meet; amount aggregation requires an amount-guaranteeing event.
3. `E-POS-TARGETED` — `Targeted` legal only at spell/mode/triggered/activated/
   `Reflexive`/`Delayed` roots [CR#115.1a..115.1e,601.2c]; never in
   replacement/static/loop/intervening-if positions.
4. `E-KIND-*` — filter kind lattice (object vs player vs any; `OneOf` joins,
   a filter's kind must be ≤ its slot's kind); deed patient scope per relation;
   counter/designation scopes and subtype categories vs loaded registries.
5. `E-FLOOR-*` — card/token floors (types nonempty; subtype categories ⊆ types
   [CR#205.3d]; loyalty ⇒ Planeswalker; defense ⇒ Battle; token specs
   permanent-types-only); literal range order; divide literal ≥ minimum group
   size [CR#601.2d]; `Nth ≥ 1`; modal count ≤ modes; ballot tiebreak in range;
   nonempty produced-mana `OneOf`.
6. `E-COST-*` — cost-eligibility of `Do(action)` cost components [CR#118.3],
   recursing through keyword composites (Sinister Concoction's "Mill a card"
   stays legal; `Do(WinGame)` does not); `X` without `has_x`/`where_x`.
7. `E-COPY-EXCEPT` — copy-except lists may touch only copiable characteristics
   (name, mana cost, color/indicator, card types, subtypes, supertypes, rules
   text/abilities, power, toughness, loyalty) applied at copiable-value time
   [CR#707.2,707.3,109.3]; `SetController`/status riders/counter ops rejected.
8. `E-MACRO-*` — definition-time macro body checks and binder-contract
   validation (activates with [[macro-typed-holes-contracts]]).

Context threading rules the walker enforces now: `Delayed` bodies drop all
`TargetSlot` antecedents and keep `Product` antecedents with their
`expected_zone` [CR#603.7c]; `Reflexive` sees the full outer context
[CR#603.12a]; replacement/static bodies set `may_target = false`; static
ability position stays live re-gathering [CR#604].

## The table emitter

`idris2 --exec emitTables` (in `idris/`) writes `data/grammar-tables/*.ron`
from Core.idr's total functions — in this ticket, the rows the CURRENT model
defines: event caps per event kind, cost-eligibility/cost-caps, binder gating,
counter/designation/subtype scopes, zone/floor rules. Later tickets extend the
catalog (entailments and lanes in [[core-eventfilter-master-forms]]; the full
v2 set in [[idris-tables-fixtures-v2]]). Every emitted row carries its
bracketed CR citation and flows through `cargo xtask cite check`; `cargo xtask
cite audit --diff` reads generated rows like prose. The Rust elaborator
CONSUMES the emitted tables; it never restates a row in code.

**Two known holes in the current Idris model that the emitted tables must NOT
inherit** (fix at the emitter/model boundary, with a reject fixture each):

- `Facet.Patient`'s free kind (idris/src/Core.idr:966 — `Patient : Predicate b
  k -> Facet b` leaves `k` a free implicit, unconstrained by the event's
  `patientKind`). Emitted caps rows must gate patient-side predicates by the
  kind the event actually fixes.
- `actionEventCaps`'s catch-all (Core.idr:1511 — `actionEventCaps _ = NoCaps`).
  Only Sacrifice/Discard/Move have real rows; every other cost action silently
  grants no event caps. The emitted cost-caps table must carry an explicit row
  per cost-eligible action — a missing row is an emitter error, never a silent
  no-caps default.

Port the Idris Spec failing blocks as reject fixtures where they express rules
this walker enforces.

## Done

- `deckmaste_cards::elaborate` walks every card of a loaded plugin; `cargo
  xtask validate` runs it and reports per-card `E-*` errors.
- `data/grammar-tables/*.ron` generated by `idris2 --exec emitTables`, rows
  CR-cited; the walker reads them at build/validate time.
- Every active rule has a reject fixture failing with its exact code; the two
  Idris holes above have fixtures proving the tables don't inherit them.
- `plugins/builtin`, `plugins/canon`, `plugins/testing` elaborate clean.

## Verification

- `idris2 --build mtg.ipkg` (in `idris/`) then `idris2 --exec emitTables`.
- `cargo xtask validate` (defaults to builtin) and against `plugins/canon`,
  `plugins/testing` — zero elaboration failures.
- `cargo test -p deckmaste_cards` — reject fixtures each fail with their code.
- `cargo xtask cite check` — 0 stale; `--list-noncompliant` empty; `cargo
  xtask cite audit --diff` read against every generated row; `bless` new rules.
