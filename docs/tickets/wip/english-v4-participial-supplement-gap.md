---
needs: []
---
# ParticipialSupplement passes a Gap through its comma

A side finding of `english-v4-child-name-control-and-category-fission-audit`.
The owner approved a ticket for it in round 9.

## Owner's ruling (round 9, side findings)

"ParticipialSupplement passes a Gap through its comma (transparent head)
against the engine's own adjunct rule (CGEL Ch. 12 §4, p. 1058): measure, then
drop the transparency or keep with a reason."

## The finding, as the audit found it

ParticipialSupplement (`english.ron:611`; *Clause , VerbPhrase*, a
past-participial passive supplement) declares `transparent: ["head"]`. So a
Gap passes through a comma-set post-dependent. The engine's adjunct schemas
close that Gap (`crates/deckmaste_syntax/src/compile.rs`, the adjunct schemas,
about lines 1482–1485; CGEL, Ch. 12 §4, p. 1058). The audit did not measure
which Readings depend on it.

## Change

1. Measure. Remove `transparent: ["head"]` from ParticipialSupplement in a
   probe, and list every unit and judgment whose Readings change.
2. If no endorsed analysis needs the Gap to pass, drop the transparency.
3. If one does, keep it, with a comment that states the construction and
   cites CGEL or `docs/oracle-style-guide.md`.

## Acceptance (round 9: `docs/decisions/english-v4.md` §4.4, item 9)

- No face is lost.
- Every judgment's positive assertions hold.
- Nothing ungrammatical is admitted.
- `cargo xtask english-v4 strata --check` passes.
- No pathological forest explosion on real cards.

The landing reports the cheapest-tier figures and the judgment-set S/A/P/F,
but they are not gates (item 7). No Attachment Class, Production Cost tweak or
residual ticket is added to win a rank (item 8).

Standard constraints apply.

Census delta (measured on `<change>` → this landing): faces read, total Readings and Surplus before and after; lost faces by name, or "none".
Judgment set: the result of running it.
