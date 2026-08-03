---
needs: []
---
**Two structural defects that the stored serial comma was hiding.** Split out of
`english-derived-serial-comma` (round `sfdiet`, 2026-07-30): deriving
`comma` from member count is exact for five coordination structs, but
`NounPhraseCoordination` (39 faces) and `NominalPhraseCoordination` (1 face)
disagree, and the disagreements are **not** derivation failures — they are wrong
trees. Both fields stay stored until this is fixed; deleting them first would
turn a silent misparse into a render regression.

Why it was invisible: the renderer replays whatever lowering recorded, so a
wrong comma bit round-trips clean. The corpus gate cannot see a bit it is being
handed. Deriving the bit is what exposes the tree — the same mechanism that
caught Giant Oyster in round `ppcoord`.

## Defect 1 — a clause-boundary comma folded into a phrase coordination

The aura/pacifism template: `Enchanted creature can't attack or block, and its
activated abilities can't be activated.` The stored tree is

~~~
CoordinatedNounPhrase {
  first: "block",
  rest: [ { conjunction: Some(And), comma: true, phrase: "its activated abilities" } ],
}
~~~

i.e. it reads `can't attack or [block, and its activated abilities]`, coordinating
a verb with a noun phrase. The comma marks a **clause** boundary — two
independent clauses with different subjects (`Enchanted creature` / `its
activated abilities`) — and belongs at clause level, not inside the object.

Confirmed witnesses on this template: Arrest, Petrify, Faith's Fetters, Ice
Cage, Prison Term, Prison Sentence, Suppression Bonds, Stasis Cocoon, Bound in
Gold, Desert's Hold, Detention Vortex, Krasis Incubation, Lawmage's Binding,
Nahiri's Binding, One Thousand Lashes, Planar Disruption, Realmbreaker's Grasp,
Trapped in the Tower, Volrath's Curse, Arachnus Web, Academic Probation,
Alpine Moon, Lost in Thought, Figure of Fable.

`NominalPhraseCoordination`'s single witness is the same shape: Summon: Esper
Valigarmanda, `an instant or sorcery card exiled with this Saga, and mana of any
type can be spent to cast that spell`.

## Defect 2 — a flat Oxford list built as nested binary coordinations

Telling Time: `Put one of those cards into your hand, one on top of your
library, and one on the bottom of your library.` This is a genuine three-member
Oxford list, so `rest.len() >= 2` should hold and the derivation should agree —
it does not, which means the list is not one three-member coordination at any
single level. Diagnose whether it nests as binary coordinations (each level then
seeing `rest.len() == 1`), and if so whether the flat list is recoverable.

**The 39 have not been fully partitioned between these two defects.** The
measuring round reported all 39 as the aura template; a spot check refuted that
for Telling Time. Enumerate the split before fixing: candidates for defect 2 are
the non-aura names — Telling Time, Moment of Truth, Sway of the Stars, Shadow
the Hedgehog, Eldrazi Mimic, Shape Stealer, Halfdane, All Will Be One,
Exuberant Wolfbear, Nicol Bolas God-Pharaoh, Arwen Mortal Queen, Frodo Sauron's
Bane, Linvala Shield of Sea Gate, Shiko and Narset Unified, The Tale of Tamiyo.

## Severity

Defect 1 mis-brackets very common cards (the whole Pacifism/Arrest aura family).
The rendered English is unaffected, so no fidelity gate fails, but every
consumer of the tree — the engine, the Idris soundness gate, the semantic IR —
sees a verb coordinated with a noun phrase. Worth ranking above ordinary
recovery work despite the clean round trip.

## Verify

Delete `NounPhraseCoordination.comma` and `NominalPhraseCoordination.comma`,
derive as `conjunction.is_none() || rest.len() >= 2` (exact for the other five
structs), and require `cargo xtask english roundtrip --require-clean`. The
recovery census must stay byte-identical: fixing a bracketing is not licensed to
change what parses. Standard constraints apply.

---

## Progress, round `sfdiet2` (2026-07-30) — 22 of 40 fixed, NOT done

Both fields stay stored. 18 faces still disagree with the derived comma, so
deleting either would break the round trip.

### Root cause of defect 1, and the fix

`Vocab::Block` is hand-curated as a verb with no noun reading, but
`VocabDefinition::merge_regular` silently merges it with a generic-English
fallback row in `regular-vocabulary.tsv` (`block  noun_count`). That fallback
exists for ordinary dictionary words; it gave `block` a count-noun reading that
made a second complete derivation reachable at the top-level `Clause` node:

- **wrong** — split at `or`, so clause 2 is `block, and its activated abilities
  can't be activated` with `block` heading a bogus noun-phrase subject;
- **right** — split at `, and`, so clause 1 is `Enchanted creature can't [attack
  or block]` (predicate coordination, exactly how Pacifism alone parses) and
  clause 2 is `its activated abilities can't be activated`.

Deleting the `block  noun_count` row is the whole fix — one line. The evidence
that this is safe is the corpus, not the CR: **zero** determiner-plus-`block`
occurrences across the 31,685 supported faces. (The rules text around the
declare-blockers step uses `blocker`, `blocking` and `blocked`, but never bare
`block` as a count noun — worth noting that this is a lexical observation, not
something [CR#509.1] states, so the corpus count is what carries the claim.)
Substituting a verb with no noun reading at all makes the parser choose
correctly every time, which is how the diagnosis was confirmed.

**Fixed (22):** Arrest, Petrify, Faith's Fetters, Ice Cage, Prison Term, Prison
Sentence, Suppression Bonds, Stasis Cocoon, Desert's Hold, Detention Vortex,
Krasis Incubation, Lawmage's Binding, Nahiri's Binding, One Thousand Lashes,
Planar Disruption, Realmbreaker's Grasp, Trapped in the Tower, Volrath's Curse,
Arachnus Web, Academic Probation, Lost in Thought, Linvala Shield of Sea Gate.

### What remains — 18 faces, and the partition is now verified

**Defect 1 residue (4).** Same wrong-outer-split, but no lexical fix available:
Bound in Gold (pivots on `crew`, a genuinely needed noun), Alpine Moon (pivots
on `abilities`, an ordinary noun — the wrong split wins even with no lexical
contamination at all), Shiko and Narset Unified, and Summon: Esper
Valigarmanda (the sole `NominalPhraseCoordination` witness, a shared-determiner
variant). The correct reading is structurally reachable — verified in isolation
for Bound in Gold — so this is a parse-selection cost problem. Raising
`NounPhraseCoordinationOxford`'s precedence cost from 1 to 20 had **zero**
effect on the chosen parse, so the cost gap is not where it looks; expect real
Earley-cost work.

**Defect 2 is not one bug but three (14).** The earlier guess that these were
all a nesting-depth issue is wrong:
- *PP misattachment across conjuncts* — Telling Time, Moment of Truth: object
  lists like `Put X into your hand, Y on top, and Z on the bottom` attach
  prepositional phrases to the wrong host.
- *Shared-determiner collapse* — Sway of the Stars: a three-member Oxford list
  whose first two members collapse into a `CoordinatedNominalPhrase`, leaving
  the outer `rest.len() == 1` so the count formula undercounts. All Will Be One,
  Nicol Bolas God-Pharaoh and The Tale of Tamiyo need the same scrutiny.
- *The `power and toughness … power and toughness` idiom* — Eldrazi Mimic, Shape
  Stealer, Halfdane, Exuberant Wolfbear, Figure of Fable, Frodo Sauron's Bane:
  flattened into one three-member bare-`and` chain with the trailing
  prepositional phrase misattached to the second `toughness` instead of scoping
  over the whole predicate. **Figure of Fable was previously mis-filed as
  defect 1** — it has nothing to do with a clause-boundary comma.
- *Not fully triaged* — Arwen Mortal Queen, Shadow the Hedgehog; confirmed only
  that no `ClauseCoordination` is involved, so not defect 1.

### Method note

Measure by swapping the derivation into the renderer with the field still
stored, then `roundtrip --list`; a stored bit round-trips clean whatever it
holds. The current count is **18** (was 40). Recovery census must stay
byte-identical throughout: these faces parse cleanly today, just wrongly.
