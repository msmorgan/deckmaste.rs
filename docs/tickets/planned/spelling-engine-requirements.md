---
needs: []
---
**The spelling engine's requirement bundle from the semantics program —
the interface the english effort's rounds implement.** Design:
`docs/decisions/semantics-spelling-lowering.md`
(§8). Requirements, not implementation; the english effort owns the how
and the round decomposition.

## The list

1. **Occurrence classes**: `Argument(param)` vs `Mention(param, Full |
   Pronoun | Demonstrative)` on frame specs and compiled frames — one
   definition parameter, several licensed surface occurrences (the
   `${0:pro}` predecessor generalized); frames stay linear except via
   declared occurrence roles.
2. **Whole-scope recovery** with transient in-memory handles for binder/
   discourse linking, erased to indices — never serialized.
3. **A discourse environment** with R1-nearest / R2-uniqueness resolution
   that SURFACES ambiguity as candidates (never assembly-order guessing),
   zone-change aware (the Ephemerate "it" vs Cloudshift "that card"
   product-channel pair is the canonical fixture).
4. **Multi-sentence frames** (rooted at paragraph level) for `May`-class
   words.
5. **A role key generalizing `FramePosition`**: cost component / main
   effect / trigger consequent / condition / keyword argument.
6. **A semicolon junction** in the English AST (currently absent from
   clause coordination).
7. Eventually: **typed English-AST hole substitution** replacing textual
   splice-and-reparse (the reassembly-check hazard class disappears).

## Gates

Per-item, defined by the implementing rounds; this ticket closes when
every item is either landed or superseded by a recorded english-effort
decision.
