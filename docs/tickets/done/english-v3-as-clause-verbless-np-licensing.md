---
needs: []
---
# Stop the finite as-Clause route licensing a bare NP after *as*

Priority: medium.

## Why

This is a hygiene ticket and adds no Construction. It comes from the
same-as-comparative review. Deputy of Detention and Banishment read *… with the
same name as that permanent until this creature leaves the battlefield* with 30
extra Readings. In those Readings, *as* takes a finite Clause whose Subject is
the NP *[that permanent until this creature]* and whose verb is *leaves*. The
same-as landing record (`docs/tickets/done/english-v3-same-as-comparative.md`,
item 4) measured this on Deputy's unit at `lqzlkrrp`: 30 of 106 Readings are
as-Clause only, and 0 Readings have both analyses. The remaining 76 have the comparative Complement *as [that permanent]*.
Inspection at `lztruvkm` finds that 46 of these still attach *until* as a
nominal Modifier; only 30 attach it as a VP or Clause Adjunct.
The user's ruling of 2026-10-08 retained these Readings in that landing and
routed them here.

## Prior ruling and recon stub (kept verbatim from the routing commit)

> Recon residual routed from `english-v3-same-as-comparative` by the user ruling
> of 2026-10-08. Audit the existing *as*-Clause route's licensing of a bare NP
> after *as* with no verb; establish the responsible Production and declared
> licence before proposing a bounded hygiene repair. Distinguish that route from
> the intentionally preposed predicative-NP *as* use and from the selected
> *same … as* comparative Complement.
>
> Inherited finite *as*-Clause alternatives remain licensed by this ruling.
> Deputy of Detention and Banishment provide an attested ambiguity in
> *with the same name as that permanent until this creature leaves the battlefield*:
> the comparative Complement *as [that permanent]* beside a finite Adjunct
> *as [[that permanent until this creature] leaves the battlefield]*.
> These alternatives were not introduced by the same-as ticket. Reconcile the
> verbless-NP licensing separately; no string may receive the same comparative
> Complement analysis twice. The retained-Reading contract remains authoritative.
>
> Evidence: `target/english-v3/english-v3-same-as-comparative/after.json`,
> `as-clause-overlap.json`, and the Deputy-of-Detention/Banishment scope probes.

**STOP before dispatch resolved by the user rulings of 2026-10-08 and
2026-10-09.** The same-as retention was scoped to that landing. This ticket
may remove Until's entire noun-Modifier licence: all nominal *until*
attachments are wrong Oracle analyses, regardless of Complement shape.
Measured 106 → 30 units are authorised, provided the survivors retain the
comparative Complement and a VP or Clause duration Adjunct, and zero faces
are lost. The prior routing quotation above is historical, not a standing
licence.

Line numbers refer to `qsupksky`.

## Finding at minting: the Clause has an overt verb

The title follows the routing name. The probe evidence in the same-as record,
however, shows an overt finite verb (*leaves*) in the as-Clause. Requiring an
overt verb would therefore not remove these Readings. The defect is the
Subject NP. *until this creature* is admitted as a noun Modifier of
*permanent*, because Until declares a Modifier licence:

`crates/deckmaste_lexical_source/lexicon/core.ron:205`:

```ron
"vocab:Preposition/Until": {"PrepositionFunctionLicence": "Adjunct|Modifier|PreposedAdjunct", "FiniteClauseComplement": "Yes", "LocativeUse": "No", "PrepositionComplement": "NounPhrase", "NominalBareClass": "Boundary"},
```

That Modifier licence ("until-M") is one of the seven exclusions the
2026-10-06 licensing landing deferred.
`docs/tickets/planned/english-v3-systemic-residuals.md:358–365` records that
Deputy of Detention and Banishment were its only dependants. It also records
that until-M waits for a same-name comparison composition, which the same-as
landing has now provided. That file names the re-application owner of the
deferred exclusions as itself, together with
`english-v3-selected-preposition-nominal-licensing`. This ticket carries out
the until-M re-application.

The route itself is attested and stays. `ClauseComplementPreposition`
(`crates/deckmaste_english_v3/src/declarations.rs:3081–3099`) admits *as* +
finite Clause through `"FiniteClauseComplement": "Yes"` (`core.ron:203`).
Supported witnesses of bare *as* + finite Clause:

- Utopia Sprawl: "As this Aura enters, choose a color."
- Kindred Discovery: "As this enchantment enters, choose a creature type."
- Lilah, Undefeated Slickshot: "… as it resolves."

*as long as* and *as though* are separate lexemes and are not part of this
route. The as-Clause route is not a pruning candidate.

## Change

1. Re-apply the until-M exclusion. Remove `Modifier` from Until's
   `PrepositionFunctionLicence`, leaving `Adjunct|PreposedAdjunct`, as the
   ADR pruning rule requires once the prerequisite has landed. The exclusion
   holds only if the bounded census loses zero faces. If a face is lost, find
   it, record it as an attested until-M use, keep the licence, and STOP with
   the face named.
2. Update the until-M bullet in
   `docs/tickets/planned/english-v3-systemic-residuals.md:358–361` to record
   the re-application and its change id.
3. If the 30 as-Clause Readings survive step 1, probe the surviving
   alternatives and fix them through a declared feature. Do not name *as* or
   any other lexeme.

## Witnesses

- Deputy of Detention's attested Sentence unit, "Exile all other nonland
  permanents that player controls with the same name as that permanent until
  this creature leaves the battlefield.": 106 → 30; full face 296 → 96.
  All surviving Readings have comparative Complement *as [that permanent]*
  and a VP or Clause *until* Adjunct; none has a finite *as*-Clause.
- Banishment's corresponding Sentence unit: 106 → 30; full face 296 → 96.
- Negative probe: "Exile a creature with the same name as that permanent until
  this creature leaves the battlefield.": 17 → 5; finite *as*-Clause
  Readings 5 → 0.
- Controls: Utopia Sprawl and Kindred Discovery keep their digests; Lilah,
  Undefeated Slickshot's *as it resolves* PP keeps its Reading.
- User ruling (2026-10-09): *until* PPs in Oracle function as VP or Clause
  Adjuncts; every nominal Modifier attachment is a wrong Reading. The full
  until-M exclusion is authorised, with zero lost faces. This supersedes the
  original 106 → 76 expectation: removing only 30 finite *as*-Clause Readings
  overlooked another 46 comparative Readings with nominal *until* attachment.
  Sauron, the Lidless Eye 44 → 32, Trapjaw Tyrant 10 → 8 and Vault Guardsman
  11 → 8 are also authorised corrections under this ruling. Do not narrow by
  Complement shape. The landing record names the five ruling witnesses and
  links the complete named decreases list with its retired Reading shapes.

## Method

Standard constraints apply. Deltas:

1. Verification is a bounded census of every face the touched lexemes can
   reach: every supported face containing *until*, every supported face
   containing *as*, and 300 random covered faces. Compare it with
   `cargo xtask english-v3 diff` against the claim parent on the same
   manifest. Expected gain is 0 and zero faces may be lost. Every count drop
   caused by removing a nominal *until* Modifier attachment
   is a correction under the user ruling of 2026-10-09; the stock diff and
   linked decreases list name every affected face. Use six workers and `/usr/bin/time -v`, record maximum RSS,
   and kill and report any run exceeding 30 GB resident memory.
2. Probe first. Write each negative probe as a test, and run the tests once at
   the end.
3. Run tests and clippy on the crates touched (`deckmaste_lexical_source`,
   `deckmaste_english_v3`). No Lean check is needed.
4. No guard may name a lexeme, construction, verb, noun, preposition or card.
   Read declared features only. Re-spell tests; never delete them.
5. The landing record is the six items in CLAUDE.md.

## Landing record

1. **Census delta** (`lztruvkm`, claim-parent grammar before the licence edit → after): selector is all 7,195 supported analysed-text faces containing ` until ` or ` as `, union 300 covered controls selected by stock diff with seed 20261008: 7,432 distinct faces. Covered 5,760 → 5,760; Readings 2,208,493 → 2,086,355. Stock diff: lost 0, gained 0, decreases 619, increases 0, digest-only 0, unchanged 6,813; source changes/unmatched identities 0. Lost faces: none. All 619 decreases retire wrong nominal Until attachments under the user ruling of 2026-10-09; the [complete named decreases list and retired shapes](../../../target/english-v3/english-v3-as-clause-verbless-np-licensing/decreases.md) renders stock `diff.json`. Named ruling witnesses: Deputy of Detention and Banishment 296 → 96, retiring the malformed finite as-Clause Subject and nominal duration attachments beside the comparison; Sauron, the Lidless Eye 44 → 32, retiring `PostmodifiedNominal` with `BareTemporalPreposition` (*until end of turn*); Trapjaw Tyrant 10 → 8 and Vault Guardsman 11 → 8, retiring nominal `ClauseComplementPreposition` attachments (*until this creature leaves the battlefield*), including Vault's coordinated Object. All 7,432 enumerations complete and all 2,086,355 Readings pass exact realization, declaration admission and traversal; issues, limited enumerations, duplicate Readings and internal failures are 0. Against the supplied batch reference `mqrlzszkuszn`, the same selector has covered 5,601 → 5,760 and Readings 607,213 → 2,086,355: lost 0, gained 159, decreases 601, increases 0, digest-only 32. The 159 gains and prior digest changes already existed on the claim parent; this ticket gains none. After refresh (`wsltzqrp`), the same manifest has 5,799 covered faces and 2,160,167 Readings, all validated without issues. Stock `diff-refresh.json` against the verified feature reports lost 0, gained 39, decreases/increases 0, digest-only 114; these incoming changes belong to the trunk cohort `xrmmlsot`, `uzrzywom`, `rwxrkzuw`, `pplynlpn`, `vzovykxk`, and `oxtrqtkp`. Combined claim-parent → refreshed stock `diff-final.json`: lost 0, gained 39, decreases 619, increases 0, digest-only 95, unchanged 6,679. The complete 619-name decreases list remains unchanged.
2. **Witnesses** (`lztruvkm`): Deputy of Detention and Banishment's attested Sentence units each have 106 → 30; full faces each 296 → 96. Every survivor retains comparative Complement *as [that permanent]* and a VP or Clause Until Adjunct; the inspected Deputy tree puts the complete Until PP in the dependent of an outer `ClausalPreposition`. Finite as-Clause alternatives in each unit are 30 → 0. The required negative probe has 17 → 5 Readings and finite as-Clause alternatives 5 → 0. Utopia Sprawl (9), Kindred Discovery (1), and Lilah, Undefeated Slickshot's *as it resolves* PP (1) preserve their exact inspected trees. Before probes exhibited the defect before the licence edit; independent admission and roundtrip tests remain in the suite.
3. **Spot checks** (`lztruvkm`): no newly covered faces. Ten retained-face checks: Deputy of Detention (96) and Banishment (96), comparative Complement with VP/Clause duration Adjunct; Sauron, the Lidless Eye (32), temporal Adjuncts; Trapjaw Tyrant (8) and Vault Guardsman (8), finite Until Adjuncts; Utopia Sprawl (9) and Kindred Discovery (1), initial finite as-Clause Adjuncts; Static Orb (16), initial *as long as* finite-Clause PP; Magmatic Force (1), initial *at the beginning of each upkeep* Adjunct; Tidebinder Mage (24), duration Adverb Complement of *for*, with a finite as-Clause in its equative comparison. Inspected trees realize byte-exactly. The two initial as controls and Magmatic Force were checked outside the final selector by the small census/probes.
4. **Deviations and STOPs** (`lztruvkm`): the initial whole-licence exclusion gave 106 → 30, contradicting the ticket's 106 → 76 expectation. STOP resolved by the user ruling of 2026-10-09: all nominal Until attachments are wrong Oracle analyses, regardless of Complement shape. The old expectation overlooked 46 comparative Readings with nominal duration attachment beside the 30 malformed as-Clause Readings. Updated expectations and until-M's systemic-residuals bullet accordingly; the earlier same-as retention STOP was also explicitly resolved as landing-scoped. Removed only Until's declared Modifier licence; no Construction, frame, feature or grammar code added/deleted, and no word-naming guard. The shared grammar Productions remain necessary for other licensed prepositions and already enforce the declared licence. Four regression tests added; no existing tests changed. No glossary gap. The interrupted pre-ruling census produced no after result; the bounded census above is the final pre-refresh evidence. Refresh brought grammar/lexicon/core changes, so the same bounded manifest and only the affected-crate tests were re-run. Its main-module append conflict was resolved by retaining all three suite registrations, including both incoming suites.
5. **Tests** (`lztruvkm`): `cargo test -p deckmaste_english_v3` passed (622); `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` passed; `cargo test -p deckmaste_lexical_source` passed (27 unit + 35 integration); `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` passed. Each ran once, with no linker retry. Restored 0; re-spelled 0; ignored-with-blocker 0; added 4; removed 0. No xtask tests or Lean checks. Formatting was restricted to the new test file; incidental formatting from the earlier main-module pass was restored before verification. Conditional post-refresh checks (`wsltzqrp`): `cargo test -p deckmaste_english_v3` passed (653), and `cargo test -p deckmaste_lexical_source` passed (28 unit + 35 integration), without linker retry. No second full test suite or clippy run.
6. **Timing** (`lztruvkm`): final pre-refresh bounded census 534.01 seconds wall time under `/usr/bin/time -v`, 6 workers; maximum resident set size 13,463,292 KiB. Foreground memory supervision remained below 30 GB throughout. Conditional post-refresh bounded census (`wsltzqrp`): 476.31 seconds for setup/census, 1,215.13 seconds total command wall time including rebuild, 6 workers, maximum resident set size 12,770,036 KiB; also below 30 GB.
