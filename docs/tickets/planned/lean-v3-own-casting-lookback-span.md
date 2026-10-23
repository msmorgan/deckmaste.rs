---
needs: [lean-v3-cast-predicates-and-ordinals]
---
# A lookback span for an object's own casting or activation, then fold `castBy`, `castFrom` and `activatedBy` into `did`

Standard constraints apply. Provenance: split out of
[lean-v3-cast-predicates-and-ordinals](lean-v3-cast-predicates-and-ordinals.md) (row B6 of
[lean-v3-cr-structure-principle-audit](../done/lean-v3-cr-structure-principle-audit.md)) by the owner's
third-batch ruling (2026-10-22). That ticket fixes the `castBy` / `castFrom` stepping bug and lands the
ordinal unification; this one does the deletion.

## Goal

Add a lookback span for the object's own current-incarnation casting or activation [CR#400.7].
The `.unanchoredLookback`, participial (`participialDescription`) and uniquifying refusals must
exempt it. Then fold `Predicate.castBy`, `castFrom` and `activatedBy` into `Predicate.did`:
`did player (cast which)`, `did any (cast which (from_ := some zone))`, `did player (activate which)`.

## Ruling (owner, 2026-10-22, batch 4): approved

Add `LookbackSpan.thisObject` for the object's current existence [CR#400.7]. Fold `castBy`,
`castFrom` and `activatedBy` into `Predicate.did … (within := some .thisObject)`. Keep duration
spans for history-window lookbacks ("a spell you cast this turn"). The design below is approved.

## Design (approved)

Everything below was evaluated by reading code on 2026-10-22; the implementer confirms by running
pins. Paths are under `lean/Semantics/`. Line numbers drift: re-verified 2026-10-22 in this
workspace, `LookbackSpan` is `Abilities.lean:131-139`, `DidClause` `:149`, `stepLookbackSpanWith`
`Check/Phrase.lean:963-971` (add `.thisObject` beside `.thisWay | .triggering` at `:971`),
`DidClause.anchor?` `Check/Phrase.lean:239`, `Period.thisObject` `Words.lean:627`,
`Predicate.castBy`/`castFrom`/`activatedBy` `Abilities.lean:230,231,306`. The Defect-side claims
below hold; only the numbers moved.

- Add `thisObject` to `LookbackSpan` (`Abilities.lean:135-144`), reusing the meaning of
  `Period.thisObject` (`Words.lean:620-622`: the object's own existence, until it changes zones and
  becomes a new object [CR#400.7]). Its checker arm is `stepLookbackSpanWith`
  (`Check/Phrase.lean:932-940`): add `| .thisObject => (drs, [])` beside `.thisWay | .triggering`.
- "Artifact spells you cast" (*Foundry Inspector*, *Fist of Suns*), "if you cast it from your hand"
  (*Arcane Proxy*, *Mulldrifter*) and "spend this mana only to cast a spell from your graveyard" are
  `did ... (within := some .thisObject)`: the "proposed action" and "current incarnation" regimes. A
  "historical window" ("a spell you cast this turn", *Cycle of Life*) keeps its duration span.
  `within = none` stays reserved for attributive participles (`DidClause` doc,
  `Abilities.lean:145-149`).
- How each refusal passes:
  1. `participialDescription`, via `DidClause.anchor?` (`Check/Phrase.lean:239-245`): `| some _ =>
     none` returns no anchor for any written span, so only the bare form is a Participial
     Description.
  2. `unanchoredLookback`, via `stepRelativeDidWith` (`:1112`, `:2735`, `:2810`): `refuse
     clause.within.isSome`, so any written span passes; no call-site change.
  3. `uniquifying`, via `uniquifies` (`Check/PredicateShape.lean:79-86`): a `the`-headed description
     needs a uniquifying predicate (`Check/Phrase.lean:2458`). `.thisObject` forms are not
     `the`-headed. The ordinal uniquifies: `uniquifies` loses its `castBy _ rank => rank.isSome` arm
     and gains `| .did clause => clause.ordinal.isSome`, reading `DidClause.ordinal` from the
     predecessor ticket.
- The bare forms are refused today by design: `allOf (and [isArtifact, isSpell, did you (cast
  which)])` gives `participialDescription (.deed (.action "cast"))`; `target (and [isSpell, did any
  (cast which (from_ := some graveyard))])` gives `unanchoredLookback`; `the (and [isSpell, did you
  (cast which) thisTurn])` gives `uniquifying`. Only spanned lookbacks pass today
  (`Proofs/Anaphora.lean:243,250`).

## Change

1. `LookbackSpan.thisObject`, its Template (name at the implementer's discretion; check the glossary
   first), the checker arm and the `uniquifies` arm.
2. Rewrite every use to `did` (proposed-action and current-incarnation uses take `(within := some
   .thisObject)`; historical windows keep their duration span). Delete the three predicates, their
   Templates (`castBy`, `nthCastBy` `Templates.lean:602-607`, `castFrom` `:635`, `Predicate.activatedBy`
   `:626-627`), the name-table entry (`:337-338`; the Ability Template `activatedBy`, `:1898`, *Soul
   Ransom*, stays), `uniquifies`' `castBy` arm and `activatedBy` in `seedZone`/`seedsAbility`
   (`Check/Phrase.lean:158,461`; each use also has `abilityHead`, which seeds the same).
   - `castBy`: `Cards/Cost.lean:146,161,168,178,179,180,186,196,202,234,469,486,545,632,748,868,876,954`,
     `Cards/Keyword.lean:152,157,182,1085,1100`, `Cards/Static.lean:118,641,649` (`:118`, *Cycle of
     Life*, is a historical window), `Cards/Mana.lean:204`, `Cards/Deontic.lean:306`,
     `Cards/Trigger.lean:700`, `Cards/Faces.lean:616`, `Proofs/GetsBothDeltas.lean:181`; `nthCastBy`:
     `Cards/Trigger.lean:166`, `Cards/Cost.lean:154`.
   - `castFrom`: `Definitions/Keywords.lean:221,1292` (rebound; `lean-v3-keyword-class-guards-and-observation`
     also edits it), `Cards/Trigger.lean:596,722`, `Cards/Keyword.lean:1085`, `Cards/Damage.lean:812,1287`,
     `Cards/Mana.lean:299,575,582,594`, `Cards/Description.lean:250`, `Cards/Cost.lean:632,748`,
     `Proofs/TextChanges.lean:34`, `Proofs/Anaphora.lean:231`.
   - `activatedBy` (Predicate): `Cards/Cost.lean:444,474,487,523,546`.
3. Docs: `lean/CONTRACTS.md:1273,1280`; ADR `docs/decisions/semantics-v3.md:419`.

## Tests

- `okArtifactSpellsYouCastReadsItsOwnCasting`, `okCastItFromYourHandReadsItsOwnCasting`,
  `okSpendOnlyToCastFromYourGraveyard`: each `did ... (within := some .thisObject)` checks clean
  [CR#400.7].
- `badBareDidCastIsParticipial`: no span keeps refusing as a Participial Description.
- `okThisObjectPassesTheLookbackChecks`: anchor, `unanchoredLookback` and `uniquifying` all pass.
- Keep green and rewrite onto `did`: `okCastFromGraveyard` (`Proofs/Anaphora.lean:230`),
  `okBracketedDescription` (`Proofs/TextChanges.lean:32`), `cycleOfLifeKeepsItsCastCreatureTarget`
  (`Proofs/GetsBothDeltas.lean:178`), `okOnceUponATimeFirstCast` (`Cards/Trigger.lean:167`),
  `okTroubleInPairsArms` (`:673`), `okResoluteVeggiesaurThirdDie` (`Cards/Counters.lean:820`),
  `okWavebreakHippocamp` (`Cards/Trigger.lean:483`), and every card pin at the sites above.

## Other tickets on the same files

- `lean-v3-alternative-cost-unification` writes *Omniscience*'s and keeps *Fist of Suns*' subjects
  with `castBy`/`castFrom`; whichever lands second rewrites them.
- [lean-v3-this-way-lookback-grounding](../wip/lean-v3-this-way-lookback-grounding.md) edits
  `stepDidClauseWith` and `anchor?`: land after it.
