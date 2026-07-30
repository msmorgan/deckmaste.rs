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
