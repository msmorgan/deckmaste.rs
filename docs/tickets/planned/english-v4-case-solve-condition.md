---
needs: [english-v4-next-time-replacement-clause]
---
# *To solve —* takes a Condition, and a main clause is never preterite

A Case's *To solve — [Condition]* [CR#719.3a] reads today as a Flavor Word
label over an ordinary Sentence. Three of them state a past-event condition
in the preterite:

- *To solve — Three or more creatures attacked this turn.*
- *To solve — Three or more sources you controlled dealt damage this turn.*
- *To solve — Three or more creature cards were put into graveyards from
  anywhere this turn.*

`english-v4-clause-tense-selection` wanted the matrix `Declarative` to
exclude the preterite (Oracle rules text is in the simple present:
oracle-style-guide §1, "Voice and tense"); that would remove the preterite
Readings of *Creature spells you cast cost {1} less …* and the like, but it
loses these three faces. They are conditions, not declarations.

## Change

- Decide where *To solve* lives (it is a CR-defined label, not a keyword
  ability [CR#702.1] or an ability word [CR#207.2c]) and give it a
  `Condition` Complement, as *Companion*'s `DashCondition` has.
- Then exclude `Inflection` `Preterite` on `Declarative`, and check
  `english-v4-next-time-replacement-clause`, whose two faces read only by a
  preterite main clause.

## Done when

- The three Cases read with *To solve* over a Condition.
- A matrix Declarative has no preterite Reading, and no face is lost.

Standard constraints apply.
