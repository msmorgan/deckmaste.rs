---
needs: [plugins-v2-implicit-actor-spelling]
---
**Give `scry`, `surveil` and `fight` their bodies, ported from Lean.** Split
from `plugins-v2-keyword-body-defects` on 2026-10-05. Written in the
implicit-actor spelling, so it follows `plugins-v2-implicit-actor-spelling`.
Standard constraints apply.

## The change

The three keyword action declarations under
`plugins_v2/builtin/macros/keyword_actions/` have no body. Lean has one for
each in `lean/Semantics/Macros.lean`:

- `scry` [CR#701.22a] is `lookAndSort` over the performer's library;
  `surveil` [CR#701.25a] is `lookAndSortInto … graveyard`. In Lean both re-read
  their agent with `agentRef`; under the handoff the performer is `actor`, a
  closed noun phrase like `you`, so the RON body writes `actor` and needs no
  re-read. Port `lookAndSort` / `lookAndSortInto` as RON helpers if they do
  not exist.
- `fight` [CR#701.14a..701.14c] guards that both are still creatures on the
  battlefield and then has each deal damage equal to its power to the other,
  simultaneously. Lean writes its two parameters as `capture` parameters.

Calls read `scry(1)`, `surveil(1)`, `fight(a, b)`. Declare the parameters the
calls need (`scry` declares none today though its frames take an amount).

If `fight` cannot be written without capture parameters, STOP and route it to
`semantics-v2-macro-capture-and-plurality` (which owns what RON can say about
capture); land `scry` and `surveil` without it.

## Proof

`cargo xtask lean-check` passes; a canon card or pin per action shows the
body checks (an existing canon scry or fight card, if there is one, re-spelled
over the new call). `cargo xtask expansions` shows the three new bodies and
the callers that change.

## Out of scope

The other 21 bodyless keyword actions wait for a card that needs them
(listed in `plugins-v2-keyword-body-defects`).

## Landing record

The series, on the claim `vknpstpykkts`:

- S1 `zmvpwxrtvrxx`: `semantics-v2-macro-capture-and-plurality` gains the
  evidence below (the three terms, why none is a template, the smallest
  change for each).
- S2: this record.

**Outcome: none of the three landed.** `scry`, `surveil` and `fight` stay
bodyless; no file under `plugins_v2/`, `crates/` or `lean/` changed. Each is
a STOP below, routed to `semantics-v2-macro-capture-and-plurality`, which
already named all three and both devices before this claim ("plurality
computed from an `Amount` argument … Scry, Surveil, Fateseal, Connive";
"`capture` parameters … Fight, Regenerate"). No new ticket was needed.

**The terms.** Evaluated with `#eval` against the Lean macros on this tree
(a scratch file outside the repo, built with `lake build Semantics.Macros`):

- `scry n (agent := .actor)` [CR#701.22a]: `enact (action "Scry")
  (sequentially [expose lookAt (cards (librarySlice top n actor)) actor,
  move (someOf (counted (range none none)) none (pro bare PL (introduced
  [object]))) wherever (library (oneEnd bottom) (some anyOrder) none bare)
  [], move (theRest object many) wherever (library (oneEnd top) (some
  anyOrder) none bare) []]) (some actor)`. `agentRef .actor` is `.actor`, as
  the ticket said. `PL` is `one` for `n = 1` and `many` for `n = 2`.
- `surveil n (agent := .actor)` [CR#701.25a]: the same with
  `zone graveyard bare` as the first move's destination; `PL` likewise
  `one` / `many`.
- `fight l r` [CR#701.14a..701.14c]: `withBindings D [subject l, subject r]
  (doIf (and [matches L …, matches R …]) (enact (action "Fight")
  (simultaneously [dealDamage L (statOf power L) R, dealDamage R (statOf
  power R) L])) none)`, each read `L`/`R` being `pro (parameter SHAPE)
  x.plur (parameter D i)`, or `x` itself when `x` is `actor` or already a
  parameter read. Evaluated: `D` is 0 at the root and 3 under a context of
  depth 3; `SHAPE.selfDefined` is `false` for `target creatureYouControl`
  and `true` for `thisCreature`; `fight actor it` reads `actor` as itself.

**Proof.** Gate on the S1 tree (`zmvpwxrtvrxx`):

| check | result |
|---|---|
| `cargo xtask lean-check`, `plugins_v2/canon` | 126/126 cards prove `Card.check = []` |
| `cargo xtask lean-check`, `plugins_v2/testing` | 5/5 |
| `cargo xtask facts check` | up to date |
| `cargo xtask gate --changed` | "No workspace crates are affected by the changed paths."; no test command derived, none run |
| `cargo xtask cite check` | 0 stale |
| `cargo xtask cite check --list-noncompliant` | 4, all in `AGENTS.md` (the CLAUDE.md symlink added on trunk by "chore: add AGENTS.md symlink"), none in this series |
| `cargo xtask cite audit --diff` (S1 diff piped in) | 0 citation sites |

`cargo xtask expansions` was not run: no declaration or card changed.
What is proven: the Lean terms above, by evaluation. What is argued, not
proven: that no template over the current macro layer can produce them (the
STOPs give the reason for each).

**Tests.** Restored 0, re-spelled 0, ignored 0, added 0, removed 0.
`ALLOWED_RAW` in `keyword_bodies.rs` is unchanged.

**Deviations and additions.** None to code. S1 also records, as a related
finding, that `instructions/regenerationApplication.ron` writes its subject
six times where Lean's `regenerationApplication` captures it; it has no
callers.

**STOPs.** Three, all unresolved here and routed to
`semantics-v2-macro-capture-and-plurality`.

1. *Scry: the looked-cards pronoun takes the amount's plurality.*
   `lookedCards` reads `slice.plur`, which is `Amount.plur n`. The
   `introduced [object]` window is fixed, but a template writes one
   plurality, so `scry(1)` and `scry(2)` cannot both expand to the Lean
   term. No macro spells a `Pro` over a given window either (every
   `pronouns/` macro reads `Whole`; `NounPhrase.pro` is
   `internal_expansion`), and the macros-only restriction plus the
   shrink-only `ALLOWED_RAW` forbid writing it raw. Smallest change: a
   plurality read off an `Amount` argument in the RON macro language, or a
   Lean re-spelling of `lookAndSort` whose pronoun does not carry it; then
   `lookedCards` (or its replacement) becomes a helper declaration and
   `scry` calls it.
2. *Surveil:* the same, through `lookAndSortInto`.
3. *Fight: the captures are computed.* The scope number is the expansion's
   nesting depth, and each read's `NounShape` and plurality are functions of
   the argument; `withBindings` and `Reach.parameter` have no alias.
   Repeating the parameter instead would select each target four times.
   Smallest change: a typed capture parameter that the semantics_v2 loader
   turns into `WithBindings` with a loader-allocated scope and the read
   `MacroCapture.read` computes. When it lands, the declaration takes
   `agent: None`: the deed row `("Fight", {})` gives no player performer,
   while `Scry` and `Surveil` are `playerAgent` and take the ordinary
   wrapper.

**What the ticket got wrong.** It treated `scry` and `surveil` as a port
("Port `lookAndSort` / `lookAndSortInto` as RON helpers"); both helpers are in
the "It computes" bucket of `docs/decisions/semantics-v2.md` §12.1, and the
amount-derived plurality was already this ticket's own STOP class in
`semantics-v2-macro-capture-and-plurality`. Its fight instruction (STOP and
route if capture is needed) was right.

**Not applicable.** ADR §12.1 counts and the residue count in
`plugins-v2-keyword-body-defects` (21 bodyless, which does not count these
three) are unchanged, since nothing moved out of a bucket. Coverage lock,
selection census, licensing-checker totals, overlap inventories and the
performance advisory: no English grammar, lexicon or corpus input changed.
Glossary: no new term.
