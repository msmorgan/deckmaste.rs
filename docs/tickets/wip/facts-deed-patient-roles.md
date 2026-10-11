---
needs: []
---
**Three deeds have no patient role in the facts overlay, so no deontic can
name them.** Found across the keyword-ability rounds (2026-09-07):
`Destroy` (blocks Indestructible, "can't be destroyed" [CR#702.12b]),
`Attach` (blocks Protection's "can't be enchanted, equipped, or fortified"
rows [CR#702.16c,702.16d]), and `Reveal` (blocks Ripple) carry a
`patientRole` of `fieldObject` with `bare := false` and an empty type list
in `crates/xtask/src/facts/action_overlay.rs`, so `deedFits` refuses every
noun phrase in the patient position. Declare each deed's patient role from
the CR, regenerate, `facts check` clean, then give Indestructible,
Protection's remaining rows, and Ripple their bodies with a canon card each
proving through `lean check`. Standard constraints apply.

## Landing record

Reconciled 2026-10-10 against the tree: the facts overlay `crates/xtask/src/facts/action_overlay.rs`
is gone; deed roles are the hand-kept `actFacts` table in `lean/Semantics/Check/Words.lean`.
Already done elsewhere: Destroy's patient role and Indestructible's body
(`objectCant(destroy, thisPermanent)`, witness Gaea's Revenge's line and the keyword census);
Attach's patient and host roles and `DeonticPatient.host` (`semantics-v2-subtype-rules`).

1. **Census delta** (`mwtywluv`): `lean check` canon 235 → 236 cards, 236/236 and testing 8/8 prove
   `Card.check = []`; `lean definitions` 711 → 711. Bodied keyword abilities +1 (`protection`,
   five static rows); Facts.lean's protection row gains its five-row definition. Lost: none.
2. **Witnesses** (`mwtywluv`, each `Card.check = []`): Paladin en-Vec (new; two protection
   instances [CR#702.16g]); Abbey Gargoyles (existing, now exercising the body); Lean pins
   `okProtectionFromBlack`, `okCantBeAttachedTo` (`Proofs/Attachments.lean`), `okCantBeRevealed`
   (`Proofs/Deontic.lean`).
3. **Spot checks** (`mwtywluv`): protection's `cantBeTargetedBy(thisPermanent, allOf(or([and([spell,
   Q]), abilityOf(allOf(and([source, Q])))])))` [CR#702.16b]; `cantBeAttachedTo(allOf(and([
   hasSubtype(aura), Q])), thisPermanent)` [CR#702.16c]; the Equipment-or-Fortification row
   [CR#702.16d]; `damageRule(Any, DealtBy(allOf(and([source, Q]))), ToRecipient(thisPermanent),
   Prevent(All, None), Repeatedly)` [CR#702.16e]; `cantBeBlockedBy(thisCreature, and([creature,
   Q]))` [CR#702.16f]; reveal's patient `⟨object, [card], allTypes⟩`, bare, any zone
   [CR#701.20a,701.20b]; `objectCant(reveal, this)` checks.
4. **Deviations and STOPs.** New helper `cantBeAttachedTo` (mirror of `mayBeAttachedTo`, Forbid).
   Protection's falling-off half of [CR#702.16c,702.16d] is left to the subtypes' state-based rows
   over the attach-permission model [CR#704.5m,704.5n], per the 2026-10-10 ruling. The rows name
   `thisPermanent`; the player form is what a grant to a player reads, as hexproof's. **STOP kept:
   Ripple stays bodyless.** Measured with the pile vocabulary: the reveal of the slice and the cast
   (`them`) check; "all revealed cards not cast this way" does not. `theVerbed(reveal, Many)` is
   still refused `[anaphor (verbed (action "reveal") none attributive) many 0]`: the patient role
   does not make `revealCards` (`Instruction.expose`) stamp what it reveals, and a reveal stamp
   would displace a searched card's stamp (transmute reads `theVerbed(search, One)` after
   revealing it), so it needs a stamp design, not a row. `them`/`those cards` are refused
   `[anaphor … many 2]`; `theRest` checks but drops a chosen, uncastable card, so is not the CR's
   set. Pin `badRippleRevealedCards` unchanged and still holds. No glossary gap found.
5. **Tests** (`mwtywluv`): `lake build --wfail` pass; `cargo test -p deckmaste_semantics_v2` pass;
   `cargo test -p xtask` pass (407 + 4 + 14, 1 ignored pre-existing); `cargo test -p
   deckmaste_construction_core` pass; `cargo clippy -p <each> --tests -- -D warnings` clean for all
   three; `lean macros check`, `facts check` up to date; `cite check` 0 stale, 0 non-compliant;
   `cite audit --diff` read. Restored 0; re-spelled 0; ignored 0; added 3 Lean theorems; removed 0.
6. **Timing**: `cargo xtask lean check` 508 seconds, one run, lake default workers on 24 cores
   under shared load.
