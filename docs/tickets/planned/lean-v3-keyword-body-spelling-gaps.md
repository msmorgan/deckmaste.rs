---
needs: [semantics-v3-peer-files-and-toposort]
---
# Daybound and reconfigure say all of what their CR text means

Standard constraints apply. Found by the 2026-10-25 triage of an external audit ("Atraxa"),
verified against `wmyykqnv`. Paths are under `lean/Semantics/`. Keyword Definitions carry their
full bodies, so a body that drops part of its "means" text is a fidelity gap. Both bodies below
were probed on 2026-10-25 (`cd lean && lake env lean` on scratch files
`/tmp/claude-1000/-home-msmorgan-Projects-deckmaste-rs/4e5532ee-6640-4aad-9769-888a72f918d1/scratchpad/probe/Probe.lean`
and `Probe2.lean`) and check `[]`.

Two parts, each separately committable.

## Part 1: reconfigure [CR#702.151b]

[CR#702.151a]: Reconfigure [cost] means "[Cost]: Attach this permanent to another target
creature you control. Activate only as a sorcery" and "[Cost]: Unattach this permanent. Activate
only if this permanent is attached to a creature and only as a sorcery." [CR#702.151b]:
"Attaching an Equipment with reconfigure to another creature causes the Equipment to stop being
a creature until it becomes unattached from that creature." The Oracle reminder (*Lizard
Blades*, *Lion Sash*) prints "While attached, this isn't a creature." The body
(`Definitions/Keywords.lean:1486-1498`) writes only [CR#702.151a].

Write:
```lean
semantic_definition reconfigure (cost : Cost) : Ability :=
  .keyword (.named "reconfigure") [.cost (.inCaller cost)]
    [ activatedOnlyDuring (.inCaller cost)
        (attachTo thisPermanent (target (and [isCreature, isOther, youControl])))
        asSorcery,
      .activated (.inCaller cost)
        (unattach thisPermanent)
        asSorcery none
        (is thisPermanent (attachedTo (a isCreature)))
        none,
      static
        (onlyWhile
          (characteristicChange thisPermanent [.typeLine .loses { types := some [.creature] }])
          (is thisPermanent (attachedTo (a isCreature)))) ]
```
- The second ability's condition is respelled through `attachedTo` (`Templates.lean:39`); it is
  `rfl`-equal to today's raw `.attachment .attachment none (some (a isCreature))`.
- The third static follows impending's type removal (`Definitions/Keywords.lean:1029-1030`).
- Add [CR#702.151b] to the docstring (`:1486-1488`).

Pins:
- Added: *Lizard Blades* (Vintage-legal) in `Cards/Cost.lean` beside the Equipment cards
  (`:805-819`):
  ```lean
  /-- Lizard Blades -/
  def lizardBlades : Spelled := spelled <| singleFacedCard
    { name := "Lizard Blades", cost := [generic 1, pip .red],
      types := [.artifact, .creature], subtypes := [equipment, lizard],
      text :=
        [ doubleStrike,
          static (abilityGrant (equipped creature) doubleStrike),
          reconfigure [generic 2] ],
      power := stat 1, toughness := stat 1 }
  ```
  Confirm its Oracle text in `data/derived/cards.jsonl` first.
- Rewritten if its pattern no longer matches: `reconfigureUnattachesWithoutTheAttachDeed`
  (`Proofs/Keyword.lean:505-508`) matches on the ability list; extend the pattern to the third
  element, keeping the claim about the second.
- Must stay green: `reconfigureAnchorsOnTheSource` (`Proofs/ContrastAnchor.lean:106-107`),
  `keywordBodiesKeepTheirActionDeeds` (`Proofs/Keyword.lean:551`).

## Part 2: daybound [CR#702.145b]

[CR#702.145b]: "Daybound" means "If it is night and this permanent is represented by a
double-faced card, it enters transformed," "As it becomes night, if this permanent is front face
up, transform it," and "This permanent can't transform except due to its daybound ability."
The first static (`Definitions/Keywords.lean:497-499`) omits "this permanent is represented by a
double-faced card". A copy of a daybound creature, or a creature granted daybound, is the case
the clause is for [CR#701.27a,701.27c].

No existing construct says it: `Predicate.currentFace` (`Abilities.lean:294`) is a permanent's
face; `isTransformed` (`Templates.lean:441`) is `.currentFace .back`; `Card.transforming` and
`Card.modalDfc` (`Card.lean:44-45`) are the printed frame, not a predicate; `TokenSpec.doubleFaced`
is token creation.

Change:
1. Add a nullary Predicate beside `isCopyOfACard` (`Abilities.lean:288`):
   ```lean
   /-- The member is represented by a double-faced card [CR#712.1,702.145b]. -/
   | isDoubleFacedCard
   ```
   and its Template beside `isCopyOfACard` (`Templates.lean:396`):
   `semantic_template isDoubleFacedCard : Predicate := .isDoubleFacedCard`.
   `.isCopyOfACard` has no checker arm (every Predicate match falls through a wildcard to kind
   `.object`); the build confirms whether the new atom needs one.
2. Glossary: add **Double-Faced Card** to `docs/contexts/game-model/CONTEXT.md` beside
   **Card Face** (`:587`) with [CR#712.1], through the `domain-modeling` skill, before naming the
   identifier; if the glossary entry settles on another name, use that name for the constructor
   and Template.
3. Daybound's first static becomes
   ```lean
   static
     (replace (would thisPermanent enter)
       (doIf (and [gameIs night, is thisPermanent isDoubleFacedCard])
         (withRiders [.entersTransformed]))),
   ```
   The other two statics are unchanged, except that `:502` writes `is thisPermanent (currentFace
   .front)` through the Template (`rfl`-equal).
4. Nightbound (`Definitions/Keywords.lean:1271-1279`, "As it becomes day, if this permanent is
   back face up, transform it" [CR#702.145e]): write `:1275` as `is thisPermanent (currentFace
   .back)` instead of `isTransformed`, which is `rfl`-equal and names the CR's "back face up".

Pins:
- Added: *Tavern Ruffian // Tavern Smasher* (Vintage-legal) in `Cards/Faces.lean` beside
  `afflictedDeserter` (`:317-339`):
  ```lean
  def tavernRuffian : Spelled := spelled <| .transforming
    { characteristics :=
      { name := "Tavern Ruffian", cost := [generic 3, pip .red], types := [.creature],
        subtypes := [human, warrior, werewolf], text := [ daybound ],
        power := stat 2, toughness := stat 5 } }
    { characteristics :=
      { name := "Tavern Smasher", types := [.creature], subtypes := [werewolf],
        text := [ nightbound ], power := stat 6, toughness := stat 5 } }
  ```
  Confirm both faces in `data/derived/cards.jsonl` first.
- Added: `dayboundEntersTransformedOnlyAsADoubleFacedCard` in `Proofs/OffStack.lean`, asserting by
  `rfl` that daybound's first replacement condition is `and [gameIs night, is thisPermanent
  isDoubleFacedCard]`, modelled on `enchantReplacesTheEntering` (`:261`).
- Must stay green: `okDaybound` (`Proofs/OffStack.lean:100`), `okNightbound` (`:103`), and the
  nightbound pin at `Proofs/Attachments.lean:95`.

## What must not change

The other statics of both keywords; the day/night game rules [CR#702.145c,702.145d,702.145f]
stay out of these bodies.

## Acceptance

`./lean/scripts/build` (`--wfail`), `cargo xtask lean check`, the CR citation gates (bless
[CR#702.151b] and [CR#712.1] if newly cited).
