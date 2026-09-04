---
needs: []
---
**Split the keyword `regime` column into the two facts it carries.** Residue
of `workbench-facts-residues` (2026-09-04, STOP 5). `keywordFacts.regime`
is read two ways: `Effect.keywordBodyFits` takes `AtCasting` as "the body
keys on a spell cast", `grantSubjectFits` takes it as "functions only while
on the stack". Prowess, extort and increment are triggered abilities of
permanents [CR#702.108a,702.101a,702.191a] that function on the battlefield
[CR#113.6], so a grant of prowess to a permanent is refused today: Bria,
Riptide Rogue ("Other creatures you control have prowess", vintage-legal,
supported) is unspellable, and it is one of two such grants in the corpus.

Fix: keep `regime` for the body's cast-keyed reading and add a
`functionsOnStack` column (hand-kept in the xtask overlay like the other
gate columns) that `grantSubjectFits` reads instead; rewrite
`Effect.abRegime`/`grantSubjectFits`; bench Bria, Riptide Rogue and the
other grant; keep the pin that refuses granting a stack-only keyword to a
permanent, probed non-vacuous.

Size: S–M. Done when: both grants are benched; the pin still refutes;
`cargo xtask facts labels` exits 0; build at its module count. Standard
constraints apply, including the RON-shaped constraint.
