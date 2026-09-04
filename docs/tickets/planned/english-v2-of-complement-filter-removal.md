---
needs: [english-v2-attachment-class-declared]
---
# Delete the semantic filter on `of` complements

**R9 — Group R.**

Defect. `nominal_preposition_is_licensed`'s `qualified_complement`
(`crates/deckmaste_english_v2/src/constructions.rs:5029`) exists, by its own
comment, to distinguish `creature of their choice` from **the rejected**
`card of a Goblin`. That is a rules/semantic judgement enforced in the grammar,
against the rewrite ADR's founding constraint:

> "Parser acceptance tracks grammatical Oracle English, not Magic legality. …
> the most dangerous future 'fix' would be teaching the parser Magic."

`card of a Goblin` is well-formed English; it is meaningless in Magic, which is
a layer above. The predicate is also implemented as a list of attested licence
combinations (`OfAndOnLicensed | OfInAndOnLicensed`), so it is a corpus
transcription as well as a semantic filter.

Pinned shape. Delete `qualified_complement` and the `QualifiedRelational` arm
that consumes it, letting `of` postmodify any nominal whose head declares the
relational licence. If deleting it leaves two surviving readings for the same
bytes, that is a genuine selection tie and a STOP under "Ruling: derived
attachment (2026-09-02)" — report it, do not re-narrow. Report every changed
selected analysis; some `of` readings will widen.

Fences. Replacing the filter with a narrower one. Re-adding it as a `require` on
a licence value list. Any justification of the form "no card prints this".

Glossary: Relational, Complement, Postmodifier, Preposition. Record any gap.

Baseline, measured on change `oulzkkoqmvuv` — re-measure at claim. Standard
constraints apply.
