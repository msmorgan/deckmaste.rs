---
needs: []
---
**`ACTION_OVERLAY` is hand-written Lean living in Rust.** Finding recorded by
`semantics-v2-definition-bodies`, whose per-column pass found no derivation and
no declaration home for twelve of `ActFacts`' thirteen columns.

`lean/Semantics/Check/Words.lean` already hand-writes two of the checker's three
deed-fact sources in Lean itself: `coreDeedFacts : CoreDeed → ActFacts` (15
rows) and `abilityDeedFacts : List (KeywordLabel × ActFacts)` (3 rows). The
third, `actFacts : List (KeywordActionLabel × ActFacts)`, is generated into
`Check/Facts.lean` from `crates/xtask/src/facts/action_overlay.rs` — 65 rows of
Lean record syntax written inside Rust string literals, where no Lean tooling
reads them and `decide` never sees them until the generator has run.

The columns are the checker's own data, not the registry's: `agentRole` (45
rows) and `patientRole` (15) are the gate a deontic clause reads, authored as a
bench sentence spells its deed; `stepwise`, `intransitive`, `feature`,
`counterfactual`, `rides`, `plays`, `bounded` and `opponentsLibrary` are the
same. `participle` (8 rows) is NOT the declaration's English participle: five
keyword actions declare `grammar: Verb(participle: …)`, and the two sets
disagree in both directions (`cast` and `activate` declare one and carry no
`ActFacts` participle; `destroy`, `discard`, `mill`, `tap` and `untap` carry one
and declare none). `dest` (13 rows) is the one column the `facts labels` report
already calls a CR fact authored from the [CR#701] entry.

So move the 65 rows into `Check/Words.lean` beside `abilityDeedFacts`, as
`actFacts` written in Lean; drop `actFacts` from the generated module and from
`facts.rs`; keep the generator's two-way check that every keyword-action
declaration has a row and every row a declaration. `Facts.lean` shrinks by 65
rows and the checker gains a hand-written table it can be read and proved
against. `distinctActLabels actFacts` in `Proofs/Tables.lean` keeps working.

The same question stands for the `KeywordFacts` gate columns (`regime` 53 rows,
`functionsOnStack` 47, `onInstantOrSorceryCard` 66, `paidCost` 64,
`onPermanentCard` 13, `wantsModes` 4, and the six `extra` argument-schema
variants), but those are ALSO read by the frozen Idris reference generator, so
they cannot move until it retires (`semantics-v1-cutover`). Decide the two
separately. Standard constraints apply.
