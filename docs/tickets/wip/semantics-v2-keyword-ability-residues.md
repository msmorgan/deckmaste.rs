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

Scope: the routed section (offspring bodied, squad bodied, an impending
canon card). The 87-declaration census in the opening paragraph moves to
the planned ticket `semantics-v2-keyword-ability-census` (see item 4).

1. **Census delta** (measured on `pztunmpn`). No English grammar, lexicon
   or corpus input changed. `cargo xtask lexical --card-name "Black Lotus"
   --export` gives byte-identical output on `solupslo` and `vwtunvlp`, and
   the later `pztunmpn` touches no lexical input. So covered faces and
   Readings did not move and no census was run. Lost faces: none. The Lean
   canon went from 138 to 140 cards proved (`lean check` 140/140, testing
   8/8); definitions 553/553, unchanged.
2. **Witnesses** (`pztunmpn`). Steelburr Champion (offspring) and Overlord
   of the Boilerbilges (impending) each prove `Card.check = []`. Squad is
   witnessed by `okSquadDefinition`. No English Readings are involved.
3. **Spot checks.** There are two new canon faces, not ten, because the
   ticket adds two cards. Steelburr Champion expands offspring to the
   additional cost plus "When this permanent enters, if its offspring cost
   was paid, create a token that's a copy of it, except it's 1/1"
   (`copyOf(thisPermanent, [Edits([Stat(Sets, Power, 1), Stat(Sets,
   Toughness, 1)])])`) [CR#702.175a]. Overlord of the Boilerbilges reads
   impending's four abilities, with its end-step trigger off the stack
   [CR#702.176a].
4. **Deviations and STOPs.**
   - STOP, resolved. Squad's first ability, "you may pay [cost] any number
     of times" [CR#702.157a], had no spelling. Two shapes were put to the
     owner: (a) a `PayTimes` field on `addedCost`; (b)
     `addedCost(perform(pay(cost, AnyNumberOfTimes)), true)`. Option (b)
     would have made `timesPaid` count one payment of a compound cost.
     **Owner ruling 2026-10-08: option (a).** `addedCost` now takes `times :
     PayTimes` in Lean (`StaticSpec.addedCost`) and in Rust
     (`StaticSpec::AddedCost { times }`). The RON macro defaults it to
     `Once`, so every existing body keeps its meaning. Squad is bodied as
     `addedCost(Param(0), true, AnyNumberOfTimes)` plus its enters trigger
     creating `timesPaid(ByKeyword(squad), thisCreature)` copies. Replicate
     and multikicker can now take the same shape. Replicate is still
     blocked by its "any targets" clause, and multikicker is not declared.
   - The census and the Lean-first route for the unowned remainder are not
     done here. They move to the new planned ticket.
   - Added beyond the letter of the ticket: Steelburr Champion, an
     offspring canon card.
   - `Facts.lean` regenerated: the offspring and squad rows gain their two
     components each. `facts check` is up to date and the Idris table is
     unchanged.
   - No glossary gap.
5. **Tests** (`pztunmpn`, after `kata refresh`).
   - Lean: `lake build --wfail` green (88 jobs). `cargo xtask lean check`
     140/140 canon, 8/8 testing. `cargo xtask lean definitions` 553/553.
     `cargo xtask facts check` up to date.
   - `cargo test -p deckmaste_semantics_v2` and `cargo test -p
     deckmaste_construction_core` green.
   - `cargo test -p xtask`: 374 + 4 green (1 ignored, as before). In
     `tests/main.rs`, 13 passed and 1 failed:
     `a_definition_that_breaks_a_definition_law_is_singled_out_and_the_gate_fails`.
     That failure is pre-existing on trunk. It relies on `grantable`, which
     `lean-checker-sweep-game-rules` (`mzlrtupu`) retired. The sibling
     `semantics-v2-keyword-action-residues` re-spells it (`qntmrymk`), and
     that re-spell is not on trunk yet.
   - `cargo clippy -p {deckmaste_semantics_v2,deckmaste_construction_core,xtask}
     --tests -- -D warnings` clean.
   - Cite checks: 0 non-compliant, 0 stale. The audit read 16 sites
     against their rule text. Nothing new to bless.
   - Counts: restored 0. Re-spelled 7: the Lean `addedCost` terms gain
     `.once`, in `Cards/Choice`, `Cards/Static`, `Cards/Damage` (×2),
     `Proofs/Mana`, `Proofs/GiftVariants` and
     `Proofs/KeywordDefinitionRegimes`. Ignored 0. Added 7: the Lean pins
     `okOffspringRegistryRow`, `okOffspringDefinition`,
     `okSquadSecondAbility`, `okSquadRegistryRow` and `okSquadDefinition`,
     plus two canon card proofs. Removed 0.
6. **Timing.** No census was run. One `cargo xtask lean check` took 123 seconds
   (lake default jobs).
