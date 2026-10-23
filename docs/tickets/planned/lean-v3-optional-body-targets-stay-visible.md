---
needs: [lean-v3-historical-read-from-recorded-zone-change]
---
# A target written in an optional body is invisible after it (Tuktuk Scrapper)

Standard constraints apply. Found by
[lean-v3-historical-read-from-recorded-zone-change](../wip/lean-v3-historical-read-from-recorded-zone-change.md).

## Problem

`may` is `Action.withContinuation .optional`. Under the owner ruling of 2026-10-22
(`docs/decisions/semantics-v3.md` §1.5, the Participial Descriptions paragraph: "this way" ranges
over the events that happened), whatever an optional body or arm stamps escapes it to Participial
Descriptions only (`continuationStamped`, `DRS.participialOnly`, the `.withContinuation` arm in
`lean/Semantics/Check/Phrase.lean`). A target written in the body is no exception, so after
`may (destroy (target isArtifact))` nothing names the target: not "it", not "that artifact", in a
lookback or out of one.

Probe evidence (scratch `lake env lean` runs from `DRS.ofList []`, after
lean-v3-historical-read-from-recorded-zone-change):

- `[may (destroy (target isArtifact)), doIf (did it (putInto .actor graveyard) thisWay) (draw 1)]`
  gives `[anaphor bare one 0]`.
- `[may (destroy (target isCreature)), doIf (did (that creature) die thisWay) (draw 1)]` gives
  `[anaphor (word creature) one 0]`: the written-origin read fails the same way, so the cause is
  not the lookback's origin.
- `[may (tap (target isArtifact)), instruct this_ (dealDamage 3 (controllerOf (that artifact)))]`
  gives `[anaphor (word artifact) one 0]`: no lookback is involved.

The same bodies without `may` check clean.

## Proposal (approved 2026-10-23, see Ruling)

A target is chosen as the spell or ability is put on the stack [CR#601.2c,603.3d,115.1], before
it resolves and independently of whether the optional action is later taken. So a target written
in an optional body is introduced as an ordinary referent outside it, and stays visible to later
text whether or not the player acts. Anything else the body touches (an object it chose, created,
or moved, or a target's post-body successor) stays participial-only. This mirrors §1.3's measured
domain, where a target announced inside the closed domain survives the closure
[CR#601.2c,115.1] (*Tibalt, the Fiend-Blooded*).

How it narrows the 2026-10-22 ruling: the ruling's "what the body acted on escapes only to
Participial Descriptions" becomes "what the body acted on, other than the targets it announced".
A target's announcement is no event the optional body causes; the destroy, tap or exile done to
it stays an event that happened only if the player acts, and its stamp stays participial-only.
What the pronoun reads is the target as announced, with no stamp from the body.

Pins of that ruling and its neighbours, by what they become:

- Kept: `Lookback.optionalBodyObjectEscapesOnlyToParticipialReads` ("you may exile a creature you
  control" names no target, so "it" still has no antecedent), `Lookback.optionalBodyEventGroundsThisWay`,
  `ReferenceScopes.optionalBranchesDoNotExportCreatedObjects`,
  `ReferenceScopes.optionalBodyReadsItsEnclosingPlayer`, `Lookback.objectStampedOnTwoPathsIsReadOnce`.
- To reconcile: `SelectionEvidence.ordinaryLocalsAndOptionalTargetsRemainClosed`, third conjunct
  (an optional target inside a player loop exports no plural group). Whether a target announced
  in an optional body inside `doForEach` is a loop-local the loop abstracts is a decision this
  ticket must take with the owner.

## Ruling (owner, 2026-10-23)

Approved. A target written inside an optional body becomes an ordinary referent after it: it is
chosen as the spell or ability goes on the stack, independent of whether the optional action is
taken [CR#601.2c,603.3d,115.1]. Non-target referents the body touches stay participial-only.

Inside a per-player loop (`doForEach`), the target is an ordinary loop-local, so the pin
`SelectionEvidence.ordinaryLocalsAndOptionalTargetsRemainClosed` keeps its third conjunct (an
optional target inside a player loop exports no plural group). This closes the open decision under
"To reconcile" above.

## Change

- In the `.withContinuation .optional` arm of `lean/Semantics/Check/Phrase.lean`
  (`continuationStamped`, `DRS.participialOnly`), a target announced in the body is introduced as
  an ordinary referent after the arm, not stamped participial-only; every other stamp stays
  participial-only.
- Inside `doForEach`, the target is a loop-local the loop abstracts; no plural group is exported.
- Update `docs/decisions/semantics-v3.md` §1.5 (Participial Descriptions paragraph): "what the body
  acted on, other than the targets it announced".

## Witnesses (data/derived/cards.jsonl)

- *Tuktuk Scrapper*: "Whenever this creature or another Ally you control enters, you may destroy
  target artifact. If that artifact is put into a graveyard this way, this creature deals damage
  to that artifact's controller equal to the number of Allies you control." Spelled in full, it
  refuses twice with `anaphor (word artifact) one 0` (the lookback and "that artifact's
  controller"):

  ```lean
  def tuktukScrapper : Spelled := spelled <| singleFacedCard
    { name := "Tuktuk Scrapper", cost := [generic 3, pip .red], types := [.creature],
      subtypes := [goblin, artificer, ally],
      text :=
        [ whenever
            (does (eitherOf thisCreature (a (and [ally, youControl, isOther]))) enter)
            (sequentially
              [ may (destroy (target isArtifact)),
                doIf (did (that artifact) (putInto .actor graveyard) thisWay)
                  (instruct thisCreature
                    (dealDamage (countOf (and [ally, youControl]))
                      (controllerOf (that artifact)))) ]) ],
      power := stat 2, toughness := stat 2 }
  ```

- *Werewolf Ransacker* (*Afflicted Deserter // Werewolf Ransacker*): "Whenever this creature
  transforms into Werewolf Ransacker, you may destroy target artifact. If that artifact is put into
  a graveyard this way, this creature deals 3 damage to that artifact's controller." The same two
  refusals:

  ```lean
  whenever
    (.does .any (.enact (.action "transform") (.turnOver thisCreature)
      (some (named "Werewolf Ransacker")) none))
    (sequentially
      [ may (destroy (target isArtifact)),
        doIf (did (that artifact) (putInto .actor graveyard) thisWay)
          (instruct thisCreature (dealDamage 3 (controllerOf (that artifact)))) ])
  ```

## Tests

- `okTuktukScrapper`: *Tuktuk Scrapper*, as above, in `lean/Semantics/Cards/Anaphora.lean`.
- The Werewolf Ransacker trigger above, as an `Ability.check … = []` pin. The whole card
  (*Afflicted Deserter // Werewolf Ransacker*) lands with
  [lean-v3-counted-cast-lookback-singular](lean-v3-counted-cast-lookback-singular.md).
- The three probes above become pins that check clean.
- A negative: "you may exile a creature you control" (no target) keeps
  `optionalBodyObjectEscapesOnlyToParticipialReads`.
- `SelectionEvidence.ordinaryLocalsAndOptionalTargetsRemainClosed` keeps all three conjuncts
  unchanged (ruling 2026-10-23).
- The three probes are checked on both sides of `may`: the target is read when the player acts and
  when the player does not.
