---
needs: []
---
**Give piles their own kind.** Residue of `semantic-query-domains`
(2026-09-04), whose core side split `Sort` into `ReferentSort | Amount | Pile`
and left the workbench's twin untouched by region discipline: `kindOfW PileW =
Object` and `PileP : … -> Payload Object` type a pile exactly as a singular
Object. Fix: `Pile : Kind`, retyping `PileOf`/`InPile`/`PileMention`/
`Macros.onePile`; this forces kind-indexing on `Effect.Move`/`SetStatus` and
on `instrIntro`/`doesInstrIntro`/`doesPreIntro`/`riderIntro`/`doesRiderIntro`/
`doesAnnIntro`/`costActionOk`/`heldUntilOk` (the loop intros, free now that
`workbench-loop-delta` has landed).

Size: M. Done when: no pile is typed as `Object`; the pile witnesses
(`Cards/Piles.idr`) and pins still typecheck and refute; build at its module
count. Standard constraints apply, including the RON-shaped constraint.

## As landed

- `Pile : Kind` added to `Words.Kind` (`kindIx Pile = 7`, join moved to 8), with
  the `sameKindRefl` / `kindLteInL` / `kindLteInR` / `kindLteRefl` cases; cited
  [CR#700.3b] ("The pile is not an object").
- `kindOfW PileW = Pile`, so `Pro (Word PileW) _` ("that pile" / "those piles")
  is `Noun bs Pile`.
- `PileP : … -> Payload Pile`; `sized`'s pile clause and `Phrase.setZone`'s pile
  clause retyped to `Pile`.
- `PileOf … -> Noun bs Pile`; its two `nounDelta` rows bind at kind `Pile`.
- `InPile : (pile : Noun bs Pile) -> …`; `PileMention : Noun bs Pile -> Type`.
- `Macros.onePile` / `Macros.pileOfChoice` return `Noun bs Pile`.
- `Effect.SeparateIntoPiles` introduces `MkBinding TheD Pile ManyOf (PileP …)`
  in both `instrIntro` and `instrProfile`.
- `Effect.Move` is `{k : Kind}`-indexed and gated by the new witness
  `Movable` (`ObjectMoves` | `PileMoves`); `Effect.SetStatus` is
  `{k : Kind}`-indexed and gated by `StatusHolder` (`ObjectHoldsStatus`).
  `Macros.move` and `Macros.exile` thread the same index.
- The loop intros (`instrIntro` / `doesInstrIntro` / `doesPreIntro` /
  `riderIntro` / `doesRiderIntro` / `doesAnnIntro` / `costActionOk` /
  `heldUntilOk`) needed no body changes: every helper they call over the moved
  or status-bearing noun (`nomIntro`, `moveIntro`, `stampIntro`,
  `distributedDelta`, `costNounOk`) is already `{k : Kind}`-generic, and the
  `Move`/`SetStatus` patterns bind the new index implicitly. `Enact`'s
  `{auto 0 ke : EnactKeepsOuter subj e}` is untouched.
- `TheRest` (not named by the ticket) had to follow: Fact or Fiction, Steam
  Augury, Sphinx of Uthuun, Death or Glory, Riddles in the Dark, Fortune's
  Favor, Curator of Destinies, Atris and Unesh all read "the other" as
  a *pile*. It now takes the kind positionally, `TheRest : (k : Kind) ->
  (pl : Plurality) -> … -> Noun bs k`, and the group counters carry the same
  kind (see Deviations).
- Witnesses: `Cards/Piles.idr` (Death or Glory, Steam Augury, Do or Die,
  Liliana of the Veil, Tezzeret's Gatebreaker), `Cards/Faces.idr` (Riddles in
  the Dark, Fortune's Favor, Curator of Destinies, Atris), `Cards/Anaphora.idr`
  (Fact or Fiction), `Cards/Keyword.idr` (Sphinx of Uthuun),
  `Cards/Cost.idr` (Unesh, Criosphinx Sovereign) all typecheck unchanged apart from the
  `theOther Pile` spelling.
- Pins: `badCardWordReadsPiles`, `badPileWordWithoutAPartition`,
  `badPilePartitiveWithoutAPartition`, `badMembershipWithoutAPartition`
  (re-spelled), `badPileFaceAsAStatus` (re-spelled onto the kind gate),
  `badPlayerMovedToAZone` (new). All six probed non-vacuous.
- Nothing left undone.

## Landing record

Numbers before/after:

- Idris modules built: 46 → 46 (clean `rm -rf build` rebuild).
- `Words.Kind` constructors: 8 → 9.
- `Payload Object` constructors: 3 → 2 (`PileP` moved to `Payload Pile`).
- `ProofsPiles` pins: 6 → 7; pin probes flipped: 6/6.
- Nouns/predicates typed `Noun bs Object` that denote a pile: 4 → 0
  (`PileOf`, `InPile`, `PileMention`, `Macros.onePile`/`pileOfChoice`);
  `grep -rn 'Pile.*Object\|Object.*Pile' idris/src/Experimental/` returns only
  `badPileFaceAsAStatus ObjectHoldsStatus impossible`.

Gate lines:

- `cd idris && rm -rf build && ./scripts/build` — last line
  `46/46: Building Cards (src/Cards.idr)`; 0 lines matching `warning|error`.
- `cargo xtask cite check --list-noncompliant` —
  `0 non-compliant citation-looking string(s)`.
- `cargo xtask cite check` —
  `checked 14212 citations against cr.txt (eff. 2026-08-07); 0 stale`.
- `jj --no-pager diff --git | cargo xtask cite audit --diff` —
  `audited 14 citation site(s)`; each read against its claim. No `cite bless`
  was needed: [CR#700.3], [CR#700.3b], [CR#110.5d] and [CR#400.1] are already
  registered in `cr-citations.lock`.

Assurance counts: restored 0; re-spelled 2; ignored 0; added 1; removed 0.

Deviations and additions:

- **`TheRest` kind-indexed, and the group counters with it.** `objGroup`,
  `countGroups`, `countParts`, `partsTaken`, `countedGroupSize`, `theRestOk`,
  `theOtherOk`, `theRestFits`, `groupSpent`, `restSource`, `zoneOfGroup`,
  `tyOfGroup` and `provOfGroup` all take the kind as their first parameter
  instead of hard-coding `Object`. Forced by the round: nine bench witnesses
  read "the other" as a pile, and leaving `TheRest` at `Object` would have kept
  a pile typed as an object — the exact defect the ticket closes. `Macros.theRest`
  and `Macros.theOther` take the kind positionally (`Macros.theRest Object`,
  `Macros.theOther Pile`); 34 call sites updated, no new macro added.
- **Two new witnesses, `Movable` and `StatusHolder` (`Phrase.idr`).** Indexing
  `Move`/`SetStatus` by kind without a gate would admit `Move (a player)` and
  `SetStatus … (a quality)`, which the pre-round `Noun bs Object` signature
  refused. `Movable` admits `Object` and `Pile`; `StatusHolder` admits `Object`
  only ([CR#110.5d]). Named `data` witnesses rather than `So (…)`: a bare
  `So (kindLte k …)` guard was tried first and (a) collapsed to
  `So False` messages and (b) was ambiguously solved by other erased `So True`
  arguments in scope inside `scryBody`/`fateseal`/`surveilBody`.
- **`Macros.exile` kind-indexed.** Death or Glory exiles a pile
  (`Macros.exile You (Macros.pileOfChoice Macros.anOpponent)`).
- **`churningEddy` re-spelled from `Macros.move` to `Macros.returnTo … []`**
  (`Cards/Anaphora.idr`). With `move` kind-indexed, its `Both (target creature)
  (target land)` argument leaves `Joins Object Object ?k` unsolved, and the
  bench may not pin the kind with an implicit handle. `returnTo` is Object-typed
  and is the faithful macro for the printed line, "Return target creature and
  target land to their owners' hands."
- **`ProofsAnaphora` follow-through.** `Pile` rows added to
  `countOutcomesIsFold` and `countQuantOutcomesIsFold` (coverage);
  `markTyKeepsOnes`/`markTyKeepsAt` pile clauses retyped; `groupOne`,
  `countGroupsIsFold`, `countPartsIsFold`, `theRestReadsOnlyPrefix`,
  `theRestResolvesInPrefix` and `theRestGroupResolvesInPrefix` specialized to
  `Object` (they are statements about object groups).
- **`PileMention` no longer has a refuting pin.** The old
  `badMembershipInANonPile` refused `InPile (It ManyOf)` — reading the revealed
  cards as a pile. The kind index now refuses that as a type error, which is
  strictly stronger than the `PileMention` obligation and is not expressible as
  an `Unspellable` hole. The pin was re-spelled as
  `badMembershipWithoutAPartition`, which keeps the same asserted outcome (the
  sentence is unwritable) on the surviving obligation: `InPile` may not read a
  pile that no effect grouped [CR#700.3]. `PileMention` stays positively
  exercised by `okMembershipInAPile`. No `PileMention`-refuting pin was
  invented, because every remaining non-mention `Noun bs Pile` ("each of those
  piles", "the other pile") is rules-meaningful and refusing it would be
  over-refusal.
- **`badPileFaceAsAStatus` re-spelled.** Its old context (piles of revealed
  library cards) left two open obligations once `SetStatus` gained the kind
  gate — the zone and the kind. It now partitions creatures on the battlefield,
  so the zone obligation is met and the pin refutes `StatusHolder` alone.

STOP: none taken.
