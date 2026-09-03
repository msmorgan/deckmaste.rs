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
