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
as-Clause only, and 0 Readings have both analyses. The comparative Complement
*as [that permanent]* with the *until* Adjunct accounts for the remaining 76.
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

**STOP before dispatch (orchestrator).** The Change below retires the 30
finite as-Clause Readings that this ruling retained and called "an attested
ambiguity". The ADR's deferred-exclusion rule (re-apply once the prerequisite
lands, if zero faces are lost) supports the retirement, but the ruling's own
words keep them licensed. The two cannot both stand: confirm that the
2026-10-08 ruling covered only the same-as landing and that this ticket may
retire the Readings, or rule otherwise, before an implementer claims it.

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

- Deputy of Detention unit "… with the same name as that permanent until this
  creature leaves the battlefield": 106 → 76. The comparative Complement
  *as [that permanent]* with the outer *until* Adjunct survives; 0 Readings
  keep *as* + finite Clause.
- Banishment: the same drop from its as-Clause Readings. The same-as record
  measured it at 296 at `lqzlkrrp`, together with Deputy. The expected count
  after the change is that figure less the as-Clause-only Readings, measured
  on the claim parent.
- Negative probe: "Exile a creature with the same name as that permanent until
  this creature leaves the battlefield." Count the as-Clause Readings before
  and after; they go to 0.
- Controls: Utopia Sprawl and Kindred Discovery keep their digests. Every
  supported face whose Readings include an *until* PP keeps its Readings,
  except the as-Clause Readings named above.

## Method

Standard constraints apply. Deltas:

1. Verification is a bounded census of every face the touched lexemes can
   reach: every supported face containing *until*, every supported face
   containing *as*, and 300 random covered faces. Compare it with
   `cargo xtask english-v3 diff` against the claim parent on the same
   manifest. Expected gain is 0 and zero faces may be lost. A count may drop
   only for an as-Clause-only Reading retired by this ticket, and each such
   face is listed by name.
2. Probe first. Write each negative probe as a test, and run the tests once at
   the end.
3. Run tests and clippy on the crates touched (`deckmaste_lexical_source`,
   `deckmaste_english_v3`). No Lean check is needed.
4. No guard may name a lexeme, construction, verb, noun, preposition or card.
   Read declared features only. Re-spell tests; never delete them.
5. The landing record is the six items in CLAUDE.md.
