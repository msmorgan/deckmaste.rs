---
needs: []
---
**Thin the macro layer to one macro per verb lemma, collapse the `mayPlayDeed`
family, and delete the 26 dead macros.** 2026-09-02 workbench audit (F2, F3).
Authority: `docs/decisions/semantics-v2.md §7` — "One macro per verb lemma …
the old macro layer's agreement pairs (draw/draws) do not carry into v2" —
which `Macros.idr` as it stands contradicts.

## Agreement and arity families (F2, M)

`drawACard/drawCards/drawsACard:968-977`; `scry/surveil/scryOne/surveilOne/
playerScries/playerSurveils/playerScriesOne/playerSurveilsOne:1393-1501` (the
two `playerSurveils*` are dead); `exile/exiles`; `transform/transforms/
transformsInto`; `gainControl/gainsControl`; `shuffleInto/shufflesInto`;
`discard/discards/discardsACard/discardsACardAtRandom/discardN`;
`youWinTheFlip/youWinACoinFlip`; `youLoseTheFlip/youLoseACoinFlip` (dead);
`rollADie/rollDice/youRollADie/youRollDice`; `flipACoin/youFlipACoin/flipCoins/
flipACoinFor`; `may/mayThen/mayElse/mayThenElse`; `doThen/doElse/doThenElse`.

Fix: one macro per lemma with explicit agent and amount (`draw who n`, `scry
who n`, `surveil who n`, `flipCoins who n`, …); sed the bench to the surviving
spelling.

## The `mayPlayDeed` family (F3, S)

`Macros:1788-2441` (~650 lines): `mayPlayDeed, mayPlay, mayCastFrom,
mayCastFromPaying, mayPlayFrom, mayCastFromLimited, mayCastFromOnly,
mayPlayFromEachYourTurn, mayCastFromFree, mayCastFromEachYourTurn,
mayCastAsThough, mayCastFromWhileSearching` enumerate combinations of
`Effect.DeonticRider.PlayRider:522`'s five slots (`from, limit, window,
exclusive, payment`), and twelve of them re-state the same six-line
`deonticPatientOk`/`deonticRiderOk` preamble (`grep -c deonticRiderOk
Macros.idr` = 12; `mayCastFrom` and `mayCastFromPaying` differ in one
`PlayPayment` cell). Fix: keep `mayPlayDeed deed who what rider` and name the
rider values (`fromZ z`, `free`, `paying c`, `onceEachYourTurn`); the proof
preamble lives once.

## Dead macros

Delete: `aCardInHand, canDo, cantAttackOrBlock, comesUp, convert, handPick,
happenedToAt, itAsCard, itAsPermanent, itsAnAbility, lookedRest, losesCounters,
mayCastFromEachYourTurn, monoHybridPip, noRiders, oneOrBoth, playerSurveils,
playerSurveilsOne, preventNextBy, proliferable, proliferated, secretlyChoose,
shuffledIntoZ, theirTopCard, theyLookAtTop, youLoseACoinFlip`.

Also: `Macros.itPrior:2160` demands `{auto 0 sp : effIntro prev = effDelta
prev ++ bs}` from its caller; the macro computes that proof itself (2 bench
uses re-spelled).

Done when: build is 23/23; every named macro above is gone or is the single
surviving lemma spelling; `itPrior` takes no caller-supplied proof; no macro re-states the `deonticRiderOk` preamble more
than once (`grep -c deonticRiderOk Macros.idr` = 1); every bench witness that
used a deleted macro is re-spelled against the survivor and still typechecks.
Standard constraints apply.

## As landed

- Agreement and arity families collapsed to the explicit-agent `draw`, `scry`, `surveil`, `exile`, `discard`, `gainControl`, `shuffleInto`, `flipCoins`, and `rollDice` spellings; `may` and `IfDone` branches use their core constructors directly.
- The play-permission family collapsed to `mayPlayDeed deed who what rider`, with `fromZ`, `free`, `paying`, `onceEachYourTurn`, `duringEachYourTurn`, and `whileSearching` rider values and one `deonticRiderOk` preamble.
- All 26 dead macros were deleted and every use under `idris/src/Experimental/` was re-spelled through a surviving macro or core constructor; no part was left undone.
- `itPrior` now splits `effIntro prev` with its internal `takeDropAppend` proof; the re-spelled printed witnesses are `thranduilsCompany` and `stunningShot`.
- Other re-spelled printed witnesses include `opt`, `serumVisions`, `consider`, `bumiScryMode`, `carefulStudy`, `chanceEncounter`, `panglacialWurmCast`, and `castSmallCreatureFromTopOnceEachTurn`; representative re-spelled `Unspellable` pins are `badCastsPluralComplement`, `badStaleCarrier`, `badDiscardIt`, `badDrawnCardRemention`, `badGainControlGraveyard`, `badNestedInstead`, `badIfNotReadsMandatoryBody`, `badThatMuchAfterFlip`, `badAmountRollRow`, and `badIfDoneOverScheduledBody`, with no new or re-spelled `failing` block.
