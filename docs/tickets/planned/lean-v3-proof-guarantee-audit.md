---
needs: [lean-v3-proofs-and-cards]
---
# Audit grouped v3 proof replacements and restore still-valid guarantees

**Go through every grouped, indirect and representation-retired entry in the
`lean-v3-proofs-and-cards` accounting ledger claim by claim. Restore each v2
guarantee that still holds under v3 at its original strength, and clean up the
stale names and pointers that port left behind.** Standard constraints apply.

The ledger is
`docs/evidence/lean-v3-proofs-and-cards-2026-10-15/proof-accounting.md`
(untracked; copied from the implementer's `/tmp` report). It accounts for all
201 absent v2 theorem names, but its own summary admits that an accounting
entry does not prove the original guarantee survived.

## Decisions already made

- **Real Oracle anaphora may resolve correctly.** The owner's standing answer
  is that a real card's anaphora resolving to its intended referent needs no
  STOP. A v2 refusal of real wording that now reads correctly stays an
  acceptance plus an identity witness. This applies only to real Oracle
  wording, not to synthetic refusal pins.
- **The approved changes stay.** The P12 and P13 re-spellings, the LKI reads
  (`[CR#608.2h]`) and the Vampiric Embrace attachment-host rule were raised
  with the owner. The 11 AC>R1 site classifications follow from the standing
  answer above. Do not reopen any of them.
- **A guarantee the type system enforces counts as kept.** Where v3 makes the
  old bad input unwritable (for example, `badScryPatient`: v3 scry takes an
  amount), name that structural reason in the ledger instead of adding a
  witness.

## Work

1. **Grouped and indirect replacements.** Check each such row in the ledger
   against what the old theorem actually guaranteed. Examples are the
   `ReferenceScopes.*WithOuter`/`*WithoutOuter`/`*KeepsCreature` families,
   `chooserScopeIgnoresOuter`, `comparisonScopeIgnoresOuter`, the library-slice
   rows and `ControllerSacrifice.exactProfileForEveryOuterContext`. For each
   row, do one of the following:
   - Name the current theorem that proves the same claim.
   - Restore the claim as a v3 theorem.
   - Record why the claim no longer holds or cannot be written down in v3.
2. **Universally quantified proofs.** `ownStatIdentityStableForEveryOuterContext`
   and `ownStatStableForEveryOuterContext` were replaced by particular
   contexts. Where the property still holds under v3 (a D′ subject read is the
   same whatever outer context surrounds it), prove it universally again, in
   terms of v3 names instead of v2's window arithmetic. Apply the same test to
   any other `ForEvery…`/`∀` theorem the ledger retired.
3. **Card-specific assertions.** Where the ledger maps a card's own assertion
   to a general rule witness plus the card's acceptance proof, restore the
   card-level identity assertion if it still holds.
4. **Refusal constructors.** For every `Check/Refusal.lean` constructor the
   port left unreachable (for example `verbPatientOk`, `keepsOuter`,
   `enactKeepsOuter`, `complementWritten`, `amtNonZero`), either restore the
   check, with its pin, or delete the constructor because v3 enforces the
   guarantee structurally.
5. **Cleanup:**
   - `Proofs/Anaphora.lean:765,776` point at the deleted
     `GetsBothDeltas.*TwoHalves` theorems.
   - `Actor.badLiteralAmassAfterDestroy` asserts `= []` but is still named and
     documented as a refusal.
   - `ReferenceScopes.formerDepthReachesOuterArtifact` now selects the
     creature, and its statement duplicates `ownStatWithOuter`.
   - These pairs are exact duplicates: `Anaphora.okCondSubjectRead` /
     `conditionItReadsCurrentSource` and
     `BindingIdentity.airbendSeveralTargetsCheck` / `okAirbendByCardUpToTwo`.
     Keep one theorem of each pair under the name its pin needs.
   - `Proofs/BindingIdentity.lean:9,285` cite a `semantics-v3.md` §7 that does
     not exist.
   - `docs/decisions/semantics-v3.md` §1.5 (J2) names `badItAcrossOwnSlot`,
     which no longer exists. Point it at the current witness, and add a J2
     small-clause witness ("counters on it", *Weeping Angel*) if none exists.
   - The *Kalitas, Bloodchief of Ghet* fragment in `Proofs/DrtWitnesses.lean`
     leaves out "If that creature dies this way". Check `DrtCards.monstrousStep`'s
     `.other` against the Oracle text.

## Proof

- Every grouped, indirect or representation-retired ledger row records one of
  these: an exact current theorem, a restored theorem, or a stated reason the
  claim no longer holds.
- Update the ledger in place, in `docs/evidence/`, not in `/tmp`.
- `lean/scripts/build` succeeds with zero warnings and no `sorry`.
- Report the theorems added, restored and retired, and the constructors
  deleted.
