---
needs: [semantics-v3-peer-files-and-toposort]
---
# Decide how often a caller argument's refusal is reported

A Definition that reads a caller argument in several places reports the argument's refusal once
per read: `saddle (powerOf (that (.type .creature)))` reports its anaphor twice (once from the
keyword's parameter list, once from the body), and `protection (and [eachKind .color, isCreature])`
reports `eachKindPlacement` seven times. Decide whether a caller argument is checked once, and
pin the decision. Two related observations from the same audit:

- An as-though amount refuses `bindingless` depending on context: stepped directly, `countOf`
  introduces a referent and `.thatMuch` checks clean; inside a `StaticSpec` the results swap.
- `Refusal.phrasal` is reachable only through `Predicate.checkAt` at a pile or outcome head, and
  no card text reaches it. Decide whether it stays.

Standard constraints apply.
