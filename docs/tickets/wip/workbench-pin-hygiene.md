---
needs: [workbench-table-machinery, workbench-zone-gate-unify]
---
**Keep every pin's positive twin in-tree, make the docstrings name the refused
spelling, and change `VERIFY.md` to require both.** Cleanroom review
2026-09-03, F18. The pins themselves are sound — 577 over 183 distinct
obligations, 31 reproduced, 29 with a positive twin, none vacuous — but
nothing re-checks that after a core change.

**Twins.** A twin survives in-tree for only 43 of 577 pins (`ProofsG` 37,
`ProofsF` 4, `ProofsC` 1, `ProofsD` 1; `Proofs`, `ProofsB`, `ProofsE` none),
because `idris/VERIFY.md` tells the author to delete the scratch twin once the
pin is written. So when a gate loosens, the pin goes vacuous silently. Fix:
`ProofsG`'s convention becomes the rule — the positive twin lives beside the
pin — and `VERIFY.md` is edited to say so.

**Docstrings that name the wrong thing.** Eight pins quote a printed sentence
the workbench does spell, through a sibling constructor, so they read as
unspellability evidence when they pin one spelling: `ProofsC:417
badStaticTargets` ("Target creature can't attack" is refused only as a
`Static` — `Untargeting` — and is spellable as `Continuously (deontic (target
creature) …)`), and likewise `ProofsF:164`, `ProofsD:415`, `ProofsC:434`,
`ProofsG:291`, `ProofsG:1034`, `ProofsG:913`, `ProofsF:96`. Fix: each
docstring names the *spelling* it refuses and, where a sibling spells the
sentence, says which.

**Refusal messages.** 463 of 577 pins refuse only through a bare `So (…)`
gate, whose message is `Can't find an implementation for So False.`;
`VERIFY.md`'s "a type error names the failure" holds for the 114 that name a
`data` witness. Where the message carries the meaning, prefer the `data`
idiom: `workbench-table-machinery`'s `OptOk` and `workbench-zone-gate-unify`'s
`ZoneIs` (which keeps its argument symbolic) are the two carriers, and 43 of
the naming pins' slots are the one-inhabitant witnesses the latter folds — so
land this after those two and re-spell against what they leave.

Size: S, plus a `VERIFY.md` edit.

Also (audit N9): `docs/idris-workbench-closure-tables.md` §2 still anchors `Experimental.idr` line ranges and the deleted `CantMoreThan`; re-anchor to `Module.decl` names as §3 already is.

Done when: the build is 23/23 with 0 errors and 0 warnings; every pin has a
positive twin in-tree and `VERIFY.md` requires it instead of asking for the
scratch twin to be deleted; the eight docstrings name the refused spelling and
its spellable sibling; the pin count is unchanged or higher (no pin deleted);
each twin is shown to fail when its pin's gate is loosened, and the landing
record gives the twin count before and after. Standard constraints apply.
