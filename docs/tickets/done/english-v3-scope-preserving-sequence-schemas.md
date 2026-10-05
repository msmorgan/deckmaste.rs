---
needs: []
---
# Evaluate compact sequence schemas without erasing attachment scope

Prototype flatter declaration authoring for existing modifier/complement families
while compiling to the current scope-preserving nested grammatical structures.
The agreed middle ground is compact authoring, explicit host boundaries and retained
Readings; blanket flattening of linguistic attachment is rejected. Separate this
code-economy work from english-v3-attachment-domain-audit's grammatical audit.

Pinned experiment: same-host nominal Postmodifier-chain and VP Adjunct-chain
normalization reduced none of nine probed fragments, including either 184-Reading
Avacyn ability. More aggressive nominal pre/post-chain normalization likewise
left counts unchanged, but excluded TargetedNominal. This refutes the same-host
associative-bracketing explanation for this specimen, not every possible grammar
redundancy. Flat storage is optional; an ordered application sequence can preserve
nested scope, whereas separate pre/post vectors alone may erase their relative
scope. CGEL Ch. 5 section 14.2, pp. 444–447 supports stacked nominal structure.

Preserve lexical identity, nested PP complements, pre/postmodifier relative scope,
coordination groups, targeting layers, auxiliary/clause attachment domains and
selected Complement roles. A sequence representation must state which structure
each modifier applies to. Do not assert that intersective meaning or PP Category
proves co-attachment, and do not normalize auxiliary levels as an authoring cleanup.

Acceptance for a prototype: regenerate the existing nine exact probes and prove
Reading-identity equivalence, not just unchanged recognition or counts; independently
constructed values must retain both roundtrip laws. Keep both attachments in
“sources of the color of your choice”. Report named families, schema instances,
compiled Productions and well-formatted declaration lines separately. Hiding many
specialized variants behind macros does not meet the economy requirement. Adopt a
prototype only if it actually simplifies declarations/compiler interfaces without
new complexity or altered admissions; a documented negative result is useful.

Exploration tree qymzymor (parent sqnmlrnz; grammar baseline vwtnryos, covered 13,226).
Optional reproducible diagnostics: /tmp/english-v3-attachment-probes/analyze.py and
analysis-summary.json; do not require scratch files to exist. Standard constraints apply.

## Result: negative, nothing adopted

Same-host nominal Postmodifier-chain and VP Adjunct-chain normalization reduced
none of the nine probes (2, 16, 7, 162, 110, 184, 184, 184 and 184 Readings), and
the two Of attachments stay distinct. A flat sequence representation would add a
translation layer without removing an alternative or simplifying the recursive
modifier declarations, so none is adopted. Avacyn's Reading count is not
associative bracketing; what it is, is the subject of
`english-v3-attachment-domain-audit` and its follow-ups. Declaration economy for
verb frames is a separate matter owned by `english-v3-generic-frame-consumption`.

## Landing record

Documentation only. Stamped on change `qymzymor`, lock `covered` 13,226: the
probe counts are the exploration's on that tree. The archived attempt
(`archive-english-v3-compact-grammar`) reports regenerating the same nine counts,
on a tree with unrelated grammar changes; neither was re-run for this landing.

**Prove.** No source changed; no identity lost, structural laws untouched, no
guard added.

**Disclose.** No identity newly covered; census, checker total and construction
count unchanged. Deviation: the acceptance asked for proof of Reading-identity
equivalence for a prototype; no prototype is kept, so there is nothing to prove
equivalent. No STOP. No glossary gap.

**Report.** No coverage or performance run; nothing executable changed. `cargo xtask
gate --changed` was not run to completion: this landing changes ticket files only. Tests
restored 0, re-spelled 0, ignored 0, added 0, removed 0.
