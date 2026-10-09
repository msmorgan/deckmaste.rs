---
needs: []
---
**Off-stack statics: "as [event], do" and "any time [condition] holds, do".**
Ruling 2026-10-09 ("An off-stack static is a special action's sibling",
`docs/decisions/semantics-v2.md` §7). Bodies twelve census residues: dash,
blitz, warp, rebound, ascend, storied, daybound, nightbound, devour, riot,
tribute, unleash. Standard constraints apply.

## Shape

- Two `StaticSpec` rows beside `specialAction (when) (cost) (does)`, each
  carrying an `Instruction` that never uses the stack
  [CR#604.1,603.7a,603.7g,614.1c]:
  - `asEvent (event : GameEvent) (does : Instruction)`: "as [event], do";
  - `whileHolds (condition : Condition) (does : Instruction)`: "any time
    [condition] holds, do".
- `NounPhrase.becomes (subject : NounPhrase)`: the object's successor after
  its next zone change [CR#400.7] ("the permanent this spell becomes").
- `deonticRule` gains `except : Option Cause`, `Cause = byKeyword (label :
  KeywordLabel) | byObjects (predicate : Predicate)`: "can't transform except
  due to its daybound ability" [CR#702.145b], "can't be blocked except by
  artifact creatures …" [CR#702.13b]. Fear, intimidate and the evasion
  family re-spell onto `byObjects` where they use a negated predicate today.
- Checker: kinds and references; `becomes` takes an object; `except` reads a
  keyword label or an object predicate. No zone or timing rules.
- Rust mirror, RON helpers `asEvent`, `whileHolds`, `becomes`, bench macros.

## Bodies

- dash, blitz, warp: `asEvent(paysCost(you, byKeyword(K)),
  delay(beginningOf(endStep, none, next), <return to hand | sacrifice |
  exile>(becomes(thisSpell))))` [CR#702.109a,702.152a,702.185a,603.7g], plus
  haste (dash, blitz) and blitz's draw trigger via `conditional(…,
  costWasPaid)`; warp's later-turn cast via `onALaterTurn`.
- rebound: a `replacement` of the resolve-to-graveyard zone change by "exile
  it; delay(beginningOf(upkeep, you, next), may(cast it withoutPaying))",
  under `castFrom(hand)` [CR#702.88a,603.7f]. No new node.
- ascend (permanent): `whileHolds(and([<you control ten or more
  permanents>, not(hasDesignation(citysBlessing, you))]),
  gainDesignation(you, citysBlessing, restOfGame))` [CR#702.131b]; ascend
  (spell): the same under the instant-or-sorcery class guard, as gift
  [CR#702.131a]. storied likewise over its permanents and designation
  [CR#702.195a].
- daybound: conditional `entersTransformed` if it is night;
  `asEvent(gameBecomes(night), doIf(frontFaceUp, transform(this)))`; the
  can't-transform deontic with `except byKeyword(daybound)` [CR#702.145b].
  nightbound mirrors it [CR#702.145e]. [CR#702.145c,702.145d,702.145g,502.2]
  are the engine's.
- devour: `asEvent(enters(this), sequentially([may(sacrifice(anyNumberOf(
  <quality>))), entersWithCounters(this, times(N, countOf(theVerbed(
  sacrifice, Many, …))), p1p1Counter)]))`, written as the general
  `devourQuality(creature, N)` [CR#702.82a,702.82c], as connive reads
  `conniveN(1)`.
- riot: `mayOrElse(entersWithCounters(this, 1, p1p1Counter, additional),
  gains(haste))` [CR#702.136a].
- tribute: `sequentially([choose(an(opponent)), act(that(Opponent),
  may(entersWithCounters(this, N, p1p1Counter, additional)))])`
  [CR#702.104a].
- unleash: the optional counter plus the existing conditional can't-block
  [CR#702.98a].
- Counter kinds are always written. "It devoured" [CR#702.82b] and "tribute
  wasn't paid" [CR#702.104b] read `semantics-v2-linked-readback`'s `objects`
  and `taken` facets; if that ticket has not landed, add no readback pins and
  name it in the landing record.
- amplify [CR#702.38a] is an entry clause too but needs
  `semantics-v2-sharing`'s `shares`; body it here only if sharing has landed.

## Witnesses (canon, non-keyword)

Armor of Thorns (`becomes`), a shockland ("pay 2 life; if you don't, it
enters tapped"), Game Trail (reveal land), Dracoplasm, Callous Oppressor, one
"it becomes day as this enters" card (`asEvent(enters(this),
setGameDesignation(day))`), and a canon card per keyword bodied. Pins:
`becomes` of a player is refused (a kind); each body and witness checks.

## Siblings

Cross-references `semantics-v2-linked-readback` (readbacks) and
`semantics-v2-sharing` (amplify, intimidate's `except`); neither blocks.
Refresh before integrate; `Words`/`Phrase`/`Abilities` conflicts expected.
