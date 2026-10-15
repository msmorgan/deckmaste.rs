---
needs: [lean-v3-definitions-and-templates]
---
# Re-spell Lean proofs and card bench onto Semantics v3 and prove DRT witnesses

**Update `lean/Semantics/Proofs/` and `lean/Semantics/Cards/` to the
`semantics-v3` spine and clean-sheet DRT checker, prove the `D′` and `AC>R1`
witness theorems, flip the 6 resolved ambiguity pins, and classify and pin the 11
previously measured `AC>R1` corpus sites.** Fourth implementation step of
[`docs/decisions/semantics-v3.md`](../../decisions/semantics-v3.md) (§1.5, §4).
Standard constraints apply.

## Decisions already made (`docs/decisions/semantics-v3.md` §1.5)

- **Witness theorems in `lean/Semantics/Proofs/`:**
  - Replace the positional `shiftAddress` / `countBy == 1` witness lemmas with
    DRT witness theorems for `J1` (accessibility by domain, carrier-word zone,
    and consuming-verb applicability `[CR#109.2,701.21a]`), `J2` (clause-mate
    exclusion sparing possessors and small-clause objects), `D′` (nearest-clause
    then `.subj < .obj < .other` ranking for bare pronouns, refusing ties), and
    `AC>R1` (clause-mate and pronoun-coreference exclusion followed by recency
    for demonstratives).
- **Ambiguity pins (`semantics-v2-anaphor-resolution-heuristics` answer key):**
  - Re-spell the 6 ambiguity pins resolved by `D′` and `AC>R1` as positive
    acceptances asserting the intended referent from
    `docs/evidence/anaphora-measurement-2026-10-06/key_pins.txt`.
  - Retain the true-tie / out-of-reach refusal pins (`P07`, `P08`, `P09`,
    `P15`, `P16`) as `.anaphor` refusals.
  - Replace P12’s obsolete v2 ambiguity expectation with a positive v3 witness
    selecting the second opponent by `AC>R1`, including the selected referent identity.
  - None of the 21 measured ambiguity pins is deleted without a named replacement.
- **Required pins for the 11 previously measured `AC>R1` sites:**
  - Retain a named witness for each site in `lean/Semantics/Proofs/`, classifying
    the actual v3 result. Where the raw wording selects the wrong referent,
    prove that failure and the explicit-selector correction. Where stable
    referent identities already select correctly, prove that raw acceptance
    and intended referent instead. The owner approved this correction after
    the full Eriette attachment event proved correct under v3 introduction-ID
    recency; the older measurement counted repeated mentions as newer.
    Required sites:
    1. *Tahngarth, First Mate*
    2. *Unpredictable Cyclone*
    3. *Eriette, the Beguiler*
    4. *Runesword*
    5. *Scythe of the Wretched*
    6. *Bronze Bombshell*
    7. *Gisela, Blade of Goldnight*
    8. *Solphim, Mayhem Dominus*
    9. *Ram Through*
    10. *Mangara's Equity*
    11. *Noetic Scales*
- **Required card witnesses (`by decide`):**
  - *Feral Contest* (`Cards/Counters.lean`) and *Cloud, Ex-SOLDIER*
    (`Cards/Trigger.lean`) check cleanly using the general `D′` rule without
    `requireBlockIt` / `attachToIt` window overrides.
  - New pins for *Monstrous Step* and *Kalitas, Bloodchief of Ghet* resolve to
    the intended referents by `decide` (if `J2`'s co-arguments-only boundary
    refuses or misresolves *Kalitas* or *Broken Visage*, report that against
    `J2` rather than bending the boundary).
  - *Vampiric Embrace* resolves to the enchanted creature via `J1` zone
    exclusion (`[CR#109.2]`), while *Scythe of the Wretched* is pinned in the 11
    previously measured sites above.
  - `airbend` (`[CR#701.65a]`) checks cleanly for both a single target
    (`"Airbend target creature"`, flipping `airbendByCardOneTiesUnderStrictRule`
    to `[]`) and multiple targets (`airbendItRefusesSeveral` re-spelled as an
    acceptance under the per-card loop).

## Proof

- Port `lean/Semantics/Cards/*.lean`, `lean/Semantics/Cards.lean`,
  `lean/Semantics/Proofs/*.lean`, and `lean/Semantics/Proofs.lean` once from
  `lean/archive/SemanticsV2/` onto the finished `v3` spine, checker, and macros,
  and restore `defaultTargets = ["Semantics", "SemanticsCards", "SemanticsProofs"]`
  in `lean/lakefile.toml`.
- Delete `lean/archive/SemanticsV2/` once all proofs and cards are ported.
- `lean/scripts/build` (`lake build --wfail`) succeeds across all jobs with zero
  warnings and zero `sorry`.
- Every card in `lean/Semantics/Cards/` and every theorem in
  `lean/Semantics/Proofs/` reduces by `decide`.
- Report all tests/pins added, re-spelled, and retired.
