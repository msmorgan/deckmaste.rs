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

- Declare *To solve* in `catalogs/keyword_abilities.ron` as a `variants:` entry
  on `"Solved"` (`phrase: "To solve"`, `form: DashCondition`, citing
  `[CR#719.3a,719.3b]`), matching how `DashCondition` works on `"Companion"`
  (`keyword_abilities.ron:501`) and how `variants:` works on other multi-form
  entries in `keyword_abilities.ron`.
- Then exclude `Inflection` `Preterite` on `Declarative` (after
  `english-v4-next-time-replacement-clause` gives Ring of Ma'rûf and Mangara's
  Tome their imperative Reading).

## Done when

- The three Cases read with *To solve* over a Condition.
- A matrix Declarative has no preterite Reading, and no face is lost.

Standard constraints apply.
