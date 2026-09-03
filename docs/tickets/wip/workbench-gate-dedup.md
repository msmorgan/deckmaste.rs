---
needs: [workbench-battlefield-gate-defect]
---
**Write the deed-fit check once, the head-type facts once, add a `nounDet`
projection, and stop re-flattening in `contradictionFree`.** 2026-09-02
workbench audit (F6, F13, F17, F18).

## One deed-fit predicate (F6, S)

"Does noun n's (kind, head types, zone) fit deeds ds in role r" is written six
times: `Events.DeedParticipant:694-697` (kind + head types),
`Effect.counterpartFits:539-544` (kind + head types + zone, with a `moved`
escape), `Effect.deedSubjectFits:2947-2951`, the two separate auto-implicits on
`Deontic` itself (`zn : ZoneFits (nounZone n) (deedsZone deeds role)` `:277`
and `dp : DeedParticipant …` `:278`), plus `CantBe:1121-1128` (`kk`/`sub`/`zn`)
and `CantMoreThan:295-300` (`pk`/`zn`).

Fix: `deedFits : Deeds -> Role -> Kind -> List CardType -> Maybe Zone -> Bool`
once; every site becomes one `So (deedFits …)`, or one `data DeedFits` wrapper
if call-site inference needs it, per the `so-synonym-loses-index-inference`
note.

## Head-type membership told seven ways (F17, S)

`Words.combatant:98` + `FightParticipant:116`, `Phrase.attackableTy:2861` +
`Attackable:2876`, `Words.damageableHalfTy:2448` + `DamageableTy:2442`,
`Events.deedFacts` rows + `DeedParticipant:694`, `Words.SpaceHosted:3061`,
`Words.Placeable:3769`, `Words.permanentType:3694` — with the facts restated:
`combatant Creature = True` *is* `deedFacts "Attack".deedAgent.roleTypes =
[Creature]` (`Events:513`), and `attackableTy = Planeswalker|Battle` *is*
`deedFacts "Attack".deedPatient.roleTypes` (`Events:514`).

Fix: `Fights`/`Attackable`/`AttackDefender` read the "Attack" deed row; delete
`combatant`, `attackableTy`, `FightParticipant` — the latter's `Just t` index
may stay as a `data` if inference needs it, wrapping `deedTypeOk "Attack"
Agent`.

## `nounDet` projection (F13, S)

`Phrase.nounDelta:1674-1746` already fixes a `Determiner` per constructor
(`bindFor EachD/AD/TheD/TargetD/CountD/AllD …`, `Words.Determiner:735`), yet
there is no `nounDet : Noun bs k -> Maybe Determiner`, so `groupMention`,
`choosable`, `costNounOk`, `nounIsYou`, `nounTargeted`, `anchorPhrase`,
`partitiveBase`, `countedMention`, `coordinatedPair`, `perMemberOk` and
`soleHolderOk` are each a 49-clause match over `Noun`. With `nounDet` plus
`nounPlur`/`nounZone`/`nounTys` most become one-liners (`groupMention n =
nounPlur n == ManyOf && nounDet n ∈ [TargetD, CountD, …]`), removing ~500
clauses from the `mutual` block that costs 29.6 s (`Phrase:8` opens one
`mutual` for the whole 3.6k-line module).

`nounDet` is the **interim** shape: `workbench-determiner-slot` (R1) makes the
determiner a slot on one `Described` constructor, after which this projection
is a field read and the derivation above becomes trivial. Build it anyway — it
is the enabling step for that ticket.

## `contradictionFree` re-flattens nine times (F18, S)

`Phrase.contradictionFree:1060-1069` calls `flattenPs ps` nine times (once per
conjunct check plus `negTypes`), so every `And [...]` gate — the most common
node in the bench — flattens its list nine times at elaboration;
`zonesOk`/`otherAnchorOk` repeat the pattern (proofs catalog §4). Fix: `let fs
= flattenPs ps in …`, one binding.

Done when: build is 23/23; `deedFits` has one definition and no site restates
the kind/type/zone trio; `combatant`, `attackableTy` and the duplicated
head-type predicates are gone; `nounDet` exists and the eleven listed
predicates read projections rather than matching `Noun`; `flattenPs` is called
once per gate; `Experimental.Phrase` and `Experimental.Cards` elaboration times
are recorded before and after. Standard constraints apply.

## As landed

- F6: `Events.deedFits ds r k ts z` / `DeedFits` (kind, head types, zone against `deedsZone`) replaces `DeedParticipant`; `Deontic`'s `zn`+`dp`, `CantBe`'s `kk`/`sub`/`zn` and `CantMoreThan`'s `pk`/`zn` are each one gate (`CantMoreThan` now also reads `headTys p` against the patient row); `counterpartFits` is one call with the `moved` escape as a `Nothing` zone; the 26 `Macros` deontic wrappers carry one `dp`. Written against the current `deedFacts` shape — `workbench-facts-tables` re-homes it.
- F17: `combatant`, `FightParticipant`, `attackableTy`, `damageableHalfTy` and the `DamageableTy` data are gone. `Fights`, `BecomesBlocking`, `StopsBlocking`, `BecomesAttacking` read `So (deedHeadTysOk "Attack"/"Block" role (nounHeadTys n))` beside their `OnBattlefield` gate (a `DeedFits` there restates the zone a second way and pre-empts `badFightGraveyard`'s `za` refusal); `Attackable` reads `deedHeadTysOk "Attack" Patient`; damage is one `Words.damageableType` with `DamageableTy` a `So` synonym over it. `Fights` now admits a head-type-less noun the way the Attack row's `roleBare` does, where `FightParticipant (Just t)` refused it; no pin covered that.
- F13: `Phrase.nounDet : Noun bs k -> Maybe Determiner` (Each/EachOf → EachD, Indefinite → AD, Definite/LibrarySlice/TheRest/TheOther → TheD, TargetGroup → TargetD, CountedGroup → CountD, AllOf → AllD, SomeOf/PileOf → PartD, NamesAgree → its group's, else Nothing) plus `Eq Determiner` over `sameDet`. Clauses: `anchorPhrase` 37→6, `choosable` 37→1, `groupMention` 37→3, `partitiveBase` 3→1, `countedMention` 3→2, `perMemberOk` 4→1, `costNounOk` 38→13, `nounIsYou` 37→2, `nounTargeted` 37→11; every predicate's value per constructor is unchanged. `coordinatedPair` and `soleHolderOk` were already two structural clauses and are not determiner facts — untouched.
- F18: `contradictionFree`, `zonesOk` and `otherAnchorOk` bind `flattenPs ps` once.
- Pins re-spelled, none added or removed, no witness touched: ten `Participant impossible` → `Oh impossible` (`badCantAttackLand`, `badCantDisjunctSubject`, `badCantBeAttacked`, `badMustAttackLand`, `badCoordinatedLandHostBlocks`, `badPlaneswalkerAttacks`, `badActivatedSpellClass`, `badCastAbilityClass`, `badLandBecomesBlocking`, `badBecomesBlockingPlaneswalker`); `badFightLand` `Fighter` → `Oh`; `{zn = ok}` → `{dp = ok}` on `badCantInGraveyard`, `badUntapCapGraveyardSet`, `badCounteredInGraveyard`, `badRegeneratedInGraveyard`.
- Probed non-vacuous by mis-stating once (each refused as "not a valid impossible case"): `badCantInGraveyard`, `badFightLand`, `badUntapCapGraveyardSet`, `badBecomesBlockingPlaneswalker`, `badComplementAnchorAnnounces`, `badEachOfDistributive`, `badPartitiveOfDescription`, `badCreatureAttackDefender`, `badDamageArtifact`.
- Build timings (load average 9–63 from sibling builds throughout; parity within noise). Clean build user CPU: before 1m44.6s / 88.1s / 65.2s, after 92.5s. Single-module check, alternating before/after at load 9–20: `Phrase` 19.6s→22.6s then 12.7s→12.1s; `Experimental.Cards` 18.4s→20.1s then 13.6s→12.9s. `--timing 1` in clean builds: `Phrase` before 20.6/21.6/14.8s, after 13.2/14.7s; `Experimental.Cards` before 25.2/27.7s, after 34.6/26.9s.
- Undone: nothing. `nounDet` is the interim projection `workbench-determiner-slot` turns into a field read.
- Merged onto the landed facts-tables, deontic-bounds-and-untap, macro-families, quality-op-axis, shared-subject-statics, own-read and singleton-sorts rounds: `deedFits` reads the `ActFacts` rows through `deedRoleOf`/`actFactsFor`; `Deontic` keeps the `bound` slot with `deonticBoundOk` and `deonticPatientOk` beside the one `dp`; the deleted `CantMoreThan`, untap constructors and play-permission macros stay deleted, and their successors (`cantMoreThan`, `mayPlayAdditionalLands`, `mayBlockAdditional`, `mayVoteAdditional`, `doesntUntap`, `mayDeclineUntap`, `untapsDuring`, `mayPlayDeed`) carry the one `dp`; `KnownActs` replaces `KnownDeeds` on every wrapper.
- Merge fix: `Proofs.badUntapLockGraveyard` hole `zn` → `dp` (through `doesntUntap`); `badUntapCapGraveyardSet` keeps its `pt` hole. Re-probed non-vacuous after the merge (graveyard → battlefield, land → creature; each `Oh impossible` refused): `badCantInGraveyard`, `badUntapLockGraveyard`, `badUntapCapGraveyardSet`, `badFightLand`.

## Landing record

- Construction count: `Noun` unchanged; `DeedParticipant`, `FightParticipant`, `DamageableTy` data types replaced by `So` synonyms; `Eq Determiner` added.
- Coverage and lock state: printed-card bench coverage unchanged, no witnesses removed, `cr-citations.lock` unchanged.
- Assurance: restored 0; re-spelled 15 pins; ignored 0; added 0; removed 0.
- Positive artifacts: 23/23 Idris build; `cite check --list-noncompliant` 0; `cite check` 0 stale of 17887; diff audit selected 0 sites.
- Deviations and additions: `CantMoreThan` gains the head-type conjunct; `Fights` bare-noun admission per the Attack row; the four `OnBattlefield` rows read head types only rather than `DeedFits` (the `workbench-battlefield-gate-defect` ruling keeps their zone gate).
- STOPs: none.
