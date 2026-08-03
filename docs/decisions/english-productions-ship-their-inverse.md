# English productions ship their inverse

## Decision

Every `deckmaste_english` production that admits a phrase ships, in the same
change, a renderer inverse that regenerates the source bytes. Recovery
preserves its span verbatim, so an unparsed phrase always round-trips by
construction; from the moment a phrase parses, fidelity depends on the
renderer being an exact inverse. Lowering the recovery census is therefore
half of any grammar slice — the other half is keeping the corpus-wide
round-trip gate green.

Nothing links the two directions at compile time: the renderer matches on the
lowered AST, not on `RuleTag`, so a rule that reuses existing AST shapes
compiles and ships with no renderer change at all. The requirements below are
the discipline that replaces the missing compiler check.

[English grammar is derived](english-grammar-is-derived.md) supplies that
compiler check for migrated construction families: both directions are
generated from one declaration. The discipline below continues to govern the
handwritten families while the migration ratchet runs.

## Requirements

- **Build the render inverse in the same slice.** A production that lands
  without its inverse converts a verbatim-recovery guarantee into a silent
  corruption.
- **Carry the distinction in the AST, not in the renderer's guesswork.** If a
  production makes a position meaningful, the lowered type must record it.
  (Canonical example, since fixed: `TransitivePredicate` once held `object`
  beside a flat `elements` list with no pre-object marker, so pre-object
  adverbs were re-derived from spelling; `pre_object_elements` now records
  the position and the renderer replays it.)
- **Never branch rendering on a surface spelling.** A production admitted for
  a word class must render for the whole word class; matching one lexeme
  silently corrupts every other member.
- **Test the inverse direction too.** A round-trip test over the motivating
  card only proves the case that prompted the rule. Add the mirror case the
  new rule now also admits — post-object where the motivator was pre-object,
  and vice versa.

## Consequences

A slice that lowers recovery while breaking round-trip is a net loss and does
not land. Corpus-wide fidelity is enforced by `cargo xtask english roundtrip
--require-clean` and the trunk inspect round-trip gate, so a slice that
trades fidelity for recovery surfaces as named mismatches instead of silent
corruption.

## Tracked references

- [English clauses are structural](english-clauses-are-structural.md)
- [English grammar is derived](english-grammar-is-derived.md)
- [Macro templates are bidirectional](macro-templates-are-bidirectional.md)
