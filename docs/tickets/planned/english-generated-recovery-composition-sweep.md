---
needs: [english-derived-grammar-completion]
---
**Clear the first repeated structural-recovery families after grammar
derivation.** The current supported snapshot exposes four high-frequency
composition gaps whose simpler parts already parse independently:

- the Saga instruction “Exile this Saga, then return it to the battlefield
  transformed under your control” (Azusa's Many Journeys and 28 peers);
- “Look at the top N cards …, then put them back in any order” (Sage Owl and
  28 peers);
- the Strive rider “This spell costs … more to cast for each target beyond the
  first” (Launch the Fleet and 21 peers); and
- `Pay <number> {E}` activation costs (Aetherworks Marvel and 13 peers).

Implement these as generated constructions or principled extensions of their
existing typed roles. Do not add card identities, verbatim recovery leaves,
new lexical opacity, surface-spelling branches, parse costs, or dominance edges
to force the witnesses. Preserve every tied holistic parse, and test the
intended trees directly under normal, reversed, and fixed-shuffle registration.

Extend `cargo xtask english recovery` with an optional grouped listing so the
role, recovered text, occurrence count, token count, and a bounded set of face
names are inspectable without an ad hoc corpus script. No corpus rows or
aggregate baseline are committed.

Completion requires all four named families to disappear from structural
recovery, total recovered source tokens to decrease, no other recovery role to
increase, source-independent exact round-trip to remain clean, and direct AST,
`inspect`, spelling-frame, Clippy, formatting, citation, and parent-relative
performance gates to pass. Standard constraints apply.
