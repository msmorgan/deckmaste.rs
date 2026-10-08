---
needs: []
---
**Model gaps behind the 87 keyword-ability declarations still bodyless
after `semantics-v2-macro-bodies-keyword-abilities`.** That ticket's
second landing record (`docs/tickets/done/`) holds the per-declaration
STOP text: 95 defined, 13 settled empty as rules machinery, 87 STOPs.
Pieces of the 87 are already owned elsewhere —
`semantics-v2-subject-param-kind`, `semantics-v2-macro-param-kinds`,
`semantics-v2-macro-capture-and-plurality` — but no ticket owns the list.
This one does: first landing is the census, every one of the 87 named
with its STOP cause and the ticket that owns it (or "this ticket"), then
the Lean-first route for the unowned remainder, each shape added to
`lean/Semantics` with its pins, mirrored, then the declaration bodied.
Standard constraints apply.

## Routed from `lean-keyword-definition-regimes` (2026-10-08)

That ticket landed the law (each component of a keyword definition is checked
against its own regime) and no bodies. Owed here: Offspring's body (now
unblocked: an additional cost and an enters trigger [CR#702.175a]); Squad's
second ability (its first still lacks a repetition count [CR#702.157a]); and a
canon card that reads Impending, whose landed body now fits its row
[CR#702.176a].

## Landing record

Scope of this landing: the routed section only (offspring, squad's second
ability, an impending canon card). The 87-declaration census the opening
paragraph asks for is not attempted here; it moves to the planned ticket
`semantics-v2-keyword-ability-census` with the squad STOP (see item 4).

1. **Census delta** (measured on `vwtunvlp`). No English grammar, lexicon
   or corpus input changed: `cargo xtask lexical --card-name "Black Lotus"
   --export` is byte-identical on `solupslo` (the claim) and `vwtunvlp`, so
   covered faces and Readings are not moved and no census was run. Lost
   faces: none. Lean canon 138 to 140 cards proved (`lean check`, 140/140,
   testing 8/8); definitions 553/553 unchanged.
2. **Witnesses** (`vwtunvlp`). Steelburr Champion (offspring, now bodied)
   and Overlord of the Boilerbilges (impending, body unchanged since
   `lean-keyword-definition-regimes`) each prove `Card.check = []`. No
   English Readings are involved.
3. **Spot checks.** Two newly covered canon faces, not ten (the ticket adds
   two cards): Steelburr Champion expands offspring to the additional cost
   plus "When this permanent enters, if its offspring cost was paid, create
   a token that's a copy of it, except it's 1/1" (`copyOf(thisPermanent,
   [Edits([Stat(Sets, Power, 1), Stat(Sets, Toughness, 1)])])`)
   [CR#702.175a]; Overlord of the Boilerbilges reads impending's four
   abilities with its end-step trigger off the stack [CR#702.176a].
4. **Deviations and STOPs.**
   - STOP (left open): squad's first ability, "you may pay [cost] any number
     of times" [CR#702.157a], has no spelling: `addedCost` carries no
     repetition. Two shapes, neither settled by a ruling: (a) a `PayTimes`
     field on `addedCost` (the existing `once`/`anyNumberOfTimes`/`upTo`
     type), shared by squad, replicate and multikicker; (b)
     `addedCost(perform(pay(cost, AnyNumberOfTimes)), true)` with no model
     change, rejected here because "for each time its squad cost was paid"
     (`timesPaid`) would then count one payment of a compound cost. Squad's
     declaration stays bodyless (a partial definition would misstate the
     keyword and disagree with its row's regime); its second ability is
     written and pinned (`okSquadSecondAbility`) and its intended spelling is
     recorded in `squad.ron`.
   - The 87-declaration census and the Lean-first route for the unowned
     remainder are not done in this landing; both, and the squad STOP, move
     to the new planned ticket `semantics-v2-keyword-ability-census`.
   - Added beyond the letter: Steelburr Champion as an offspring canon card,
     so the new body is exercised by `lean check`.
   - `Facts.lean` regenerated (`cargo xtask facts generate`): offspring's row
     gains its two components; `facts check` up to date; Idris table
     unchanged.
   - No glossary gap.
5. **Tests** (`vwtunvlp`). `lake build --wfail` green (88 jobs); `cargo
   xtask lean check` 140/140 canon, 8/8 testing; `cargo xtask lean
   definitions` 553/553; `cargo test -p deckmaste_semantics_v2` green;
   `cargo test -p deckmaste_construction_core` green; `cargo test -p xtask`:
   374 + 4 green (1 ignored as before), and in `tests/main.rs` 13 passed and
   1 failed, `a_definition_that_breaks_a_definition_law_is_singled_out_and_the_gate_fails`.
   That failure is already on trunk: it relies on `grantable`, which
   `lean-checker-sweep-game-rules` (`mzlrtupu`) retired, and the sibling
   `semantics-v2-keyword-action-residues` re-spells it (`qntmrymk`). `cargo
   clippy -p {deckmaste_semantics_v2,deckmaste_construction_core,xtask}
   --tests -- -D warnings` clean. Cite checks: 0 non-compliant, 0 stale, 8
   sites audited against their rule text, nothing new to bless. Counts:
   restored 0, re-spelled 0, ignored 0, added 5 (Lean pins
   `okOffspringRegistryRow`, `okOffspringDefinition`, `okSquadSecondAbility`;
   two canon card proofs), removed 0.
6. **Timing.** No census run. `cargo xtask lean check` 149.5 s, one run (lake
   default jobs).
