---
needs: []
---
# Give `CardClass` its command-zone arm, and settle the type line's leftovers

Three card-frame residues left by
`docs/tickets/done/workbench-closure-flip-risks.md`, which widened the type
catalog with six command-zone types and then explicitly deferred the frame work
they imply. `workbench-multiface-cards` was the card-frame round that could have
taken them and did not; nothing owns them now.

## 1. `CardClass` has no command-zone arm

Quoted from the closure round's ledger:

> `cardClassOf` is binary, so a conspiracy, dungeon, phenomenon, plane, scheme
> or vanguard card classifies as `PermanentCard` and is read by
> `cardAbilityOk PermanentCard`. That is wrong for a card [CR#110.4] keeps off
> the battlefield, and it is why `selenia` typechecks. A third class belongs to
> a card-frame round, not to a catalog widening.

The ask is the third class and the `cardAbilityOk` rows that follow from it —
what abilities a card that never becomes a permanent may carry.

## 2. `typesCombinable` does not refuse a command-zone type beside another

`[Conspiracy, Creature]` passes, because neither `permanentType` nor `spellType`
is True for it. The closure round recorded this as tolerated overgeneration
rather than a defect, on the doctrine that a refusal needs a rule: "No rule
refuses the combination outright … a card-frame round can tighten it if a rule
turns up." So the deliverable is the rule search, and a pin only if it lands
one.

## 3. `typePrintOrder`'s six provisional values

The closure round gave the six new rows positions 9–14 and said so:

> It is spelling-only and no gate consumes it, so the values are provisional and
> belong with `workbench-type-line-order-is-spelling`.

That ticket is closed. Either confirm the six positions against printed cards
and drop the "provisional" marking, or move the table where the spelling
boundary keeps it — the type line's order is spelling content, per that ticket's
own verdict.

## Consumption boundary

`idris/src/Experimental/Words.idr` (`CardClass`, `cardClassOf`, `typesCombinable`,
`typePrintOrder`), `idris/src/Experimental.idr` (`cardAbilityOk` and the card
frame's readers), the pin modules `idris/src/Experimental/Proofs*.idr`, and the
evidence bench `idris/src/Experimental/Cards.idr`. If §3 moves the print order
out of the workbench it lands beside the declarations in
`crates/deckmaste_english_v2`, as `union-spellings.md` did.

## Acceptance

- A command-zone card no longer reads through `cardAbilityOk PermanentCard`, and
  the term that motivated the finding (`selenia`) is re-checked against the new
  class.
- §2 ends with either a cited rule and a pin, or a written statement that no
  rule refuses the combination.
- `typePrintOrder`'s six values are no longer marked provisional.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

## As landed (2026-08-26)

**§1 — `CommandZoneCard`, the third class.** `CardClass` gains a third arm
(`idris/src/Experimental/Card.idr`) for a card whose own rule keeps it in the
command zone. The type predicate is `commandZoneType` in `Words.idr`, beside
`permanentType` and `spellCardType` and in the same total-table idiom, one rule
per row: [CR#315.3] conspiracy, [CR#309.2c] dungeon, [CR#312.2] phenomenon,
[CR#311.2] plane, [CR#314.2] scheme, [CR#313.2] vanguard. `anyCommandZoneType`
joins its two siblings. `cardClassOf` tries the command-zone type first, since
`typesCombinable` still admits a mixed line (see §2) and that type's rule is the
one that says where the card stays.

The `cardAbilityOk CommandZoneCard` rows and their grounds:

| ability | row | rule |
|---|---|---|
| `Spell _` | `False` | the card is never cast, so it is never a resolving instant or sorcery spell [CR#113.3a] |
| `Static _` | `True` | [CR#311.4,313.4,314.4] and [CR#315.5] — statics affect the game from the command zone |
| `Triggered …` | `True` | same rules, plus [CR#309.4c] room abilities and [CR#312.5] the encounter trigger |
| `Activated c …` | `costOffBattlefield c` | [CR#113.6j]; "{T}" taps a permanent [CR#107.5] and this card never is one |
| `KeywordAbility k _` | `keywordCardOk CommandZoneCard k` | the facts table, below |
| `AlsoForKeywords`, `AbilityWord` | recurse | as the other two classes |

**How the facts table answers it: a third field, not a hardcoded read.**
`KeywordFacts` gains `onCommandZoneCard` beside `onPermanentCard` and
`onSpellCard`, and `keywordCardOk` reads it for the new class. A hardcoded
`False` was rejected: it would be a silent no for every word added later, the
mirror of the "a dropped row is a silent yes" hazard. The column is not
vacuous. **Storied is `True`** — [CR#702.195a] states the ability and its
meaning without naming a bearer at all, and [CR#313.4,314.4,315.5] give the
card a static ability that functions from the command zone. Every other row is
`False`, each for its own rule's reason, in two families: words whose rule
speaks of a permanent or a creature (the card never is one), and words that
require casting or a zone the card cannot reach. Ascend is the instructive near
miss — [CR#702.131a,702.131b] give the word a reading on a spell and on a
permanent and nowhere else, so a command-zone card has no reading to print.

**`selenia` re-checked.** Still elaborates, now through `CommandZoneCard`: its
one static is licensed by [CR#313.4], not by the permanent frame it was
silently borrowing. Its docstring says so.

**Pins** (`ProofsD.idr`, beside `badTapSorcery` and `badKeywordOnInstant`,
whose shape they copy). Both are new refusals the third class creates — under
`PermanentCard` both terms elaborated:

- `badTapVanguard` — "{T}: Draw a card." on a vanguard card. [CR#107.5,313.2].
- `badKeywordOnVanguard` — "Flying" on a vanguard card. [CR#702.9b,313.2].

Both bite: retyped to `[Artifact]` each fails with `is not a valid impossible
case`. The positives the class leaves standing were probed and elaborate: a
static on a vanguard card (`selenia`), `KeywordAbility "Storied"` on one, and a
mana-cost activated ability on a scheme card.

**Overgeneration, named.** The class is the six types together, so it answers
with the widest of their ability grants — [CR#311.4,313.4,314.4]'s three kinds.
The narrower rules are real: [CR#315.5] closes a conspiracy card's list by
enumeration at static and triggered, [CR#309.4c] gives a dungeon card's
abilities as its rooms' triggers, and [CR#312.5] names the one triggered
ability a phenomenon card has. So the workbench admits an activated ability on
any of the six. Refusing that needs the card type, not the class, and no rule
reads a narrower list off a shared frame. See the ledger.

**§2 — no rule refuses a command-zone type beside another.** The search ended
without one, and the finding is recorded at `typesCombinable`. Read: [CR#300.2]
and [CR#205.2b] admit more than one card type and exclude no combination;
[CR#300.2a] and [CR#300.2b] accommodate the two combinations they name (land,
kindred) rather than refuse them; [CR#300.1] and [CR#110.4] were read for a
single-type statement about the six and state none; [CR#108.2a], [CR#108.5] and
[CR#408.3] restrict where nontraditional cards may be, not what a type line may
say, and the conspiracy rules never call the card nontraditional, drawing it
from a player's sideboard instead [CR#315.2]. The
one argument that looked like a refusal fails: an "Artifact Plane" card would
be a permanent card by [CR#110.4a] — "a card that could be put onto the
battlefield" — while [CR#311.2] keeps it in the command zone, but that is a
permission meeting a prohibition, which [CR#101.2] already resolves in the
prohibition's favour. It is an unplayable card, not a malformed one. That is
weaker than what grounds the existing permanent/spell refusal, where [CR#110.4]
states outright that instant and sorcery cards *can't* be permanents.

So no pin, and the combination stays admitted. **Its zero is real**: over
`data/mtgjson/AllPrintings.sqlite`, no printed card or token carries any of the
six beside another card type — every printing is the type alone.

**§3 — the print order left the workbench.** The six positions cannot be
confirmed, because nothing attests them: the same measurement shows no printed
multi-type line contains a command-zone type, so no printing decides where they
sit. Per the closed ticket's own verdict the table went to the spelling
boundary instead, as `union-spellings.md` did — a measured document beside the
declarations, at
`crates/deckmaste_english_v2/docs/type-line-order.md`. `typePrintOrder` is
deleted from `Words.idr`; no order table over `CardType` remains in
`idris/src/Experimental*`, and the "provisional" marking dies with it.

The document is a real measurement, which the Idris table never was. Scope
`isFunny = 0`, 1,667 cards writing twelve distinct multi-type sequences, twelve
ordered pairs with no contradictions among them. Ranks 0–5 (Kindred,
Enchantment, Artifact, Land, Creature, Planeswalker) are attested. Ranks 6–8
are not: Battle never shares a line, and Instant and Sorcery appear only after
Kindred. Ranks 9–14 are attested at zero and marked a stated convention —
[CR#300.1]'s own alphabetical listing order, which is what the closure round's
9–14 already were. Two printings excluded by the scope contradict ranks the
table keeps (`Instant Creature` on two Mystery Booster playtest cards,
`Artifact Enchantment` on Greatest Show in the Multiverse); both are named in
the document, neither is vintage-legal.

**Gates.** `idris/scripts/build` 23/23 from a cleaned `build/`, 0 errors, 0
warnings, exit 0. `cargo check -p deckmaste_english_v2` clean. `cargo xtask cite
check --list-noncompliant` 0; `cargo xtask cite check` 0 stale after blessing
[CR#311.4], [CR#313.4], [CR#314.4] and [CR#315.5] (the only four rules new to
the lock); `jj diff --git | cargo xtask cite audit --diff` audited 29 sites and
each rule was read against its claim.

## Ledger

- **Per-type ability grants are overgenerated by the single class.** Refusing an
  activated ability on a conspiracy card [CR#315.5], or anything beyond the room
  triggers of [CR#309.4c] on a dungeon card, needs `cardAbilityOk` to see the card
  type rather than the class. Deliberately not taken here: the ticket asked for
  a class, and a per-type frame is a different shape.
- **`crates/deckmaste_english_v2/docs/type-line-order.md` still awaits a
  declaration.** This supersedes the follow-up left by
  `workbench-type-line-order-is-spelling` — the table is no longer homeless,
  but the crate still builds no type line, so when it gains one the document is
  the input the declaration is written from.
- **[CR#202.1b] names nontraditional cards among the objects with no mana
  cost.** `cardCostOk` refuses a printed mana cost only on a land card. Not
  taken: the rule says "normally includes", which is not a refusal, and
  conspiracy cards are not nontraditional. Worth a look if a rule tightens.
