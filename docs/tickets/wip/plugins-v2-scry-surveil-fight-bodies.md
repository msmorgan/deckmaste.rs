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
