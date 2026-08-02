---
needs: [english-lint-selectional]
---
**Widen the selectional table beyond `Control`/`Own`.** The shipped table has
two verbs, so its 431 findings are a floor on the misattachment family, not a
measure of it.

The constraint that makes widening real work: **each entry needs its own CR
basis.** [CR#109.1] licenses "the object of `control` must be an object"; it
licenses nothing about `sacrifice`, `exile`, `tap`, or `cast`, each of which
needs its own rule read against its own claim. Rule numbers come from the CR,
never from memory, and `jj diff --git | cargo xtask cite audit --diff` is the
check that a right-number-wrong-topic cite has not crept in — the hash checker
cannot catch that. (Run bare with no piped diff, it silently audits 0 sites
and still exits 0.)

Candidate verbs, each pending its own citation: `sacrifice`, `exile`,
`destroy`, `tap`, `untap`, `cast`. Build from `cargo xtask map enums` for the
verb inventory rather than from prose.

Soundness stays paramount: where a verb genuinely admits an open-ended host
set, leave it out rather than guessing. A small table that never lies beats a
broad one that needs triage — recall is explicitly subordinate here. Standard
constraints apply.
