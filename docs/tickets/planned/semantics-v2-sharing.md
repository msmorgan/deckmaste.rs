---
needs: []
---
**Sharing a value on an axis, as a predicate and as a group agreement.**
Ruling 2026-10-09 ("Sharing is at least one value in common on an axis",
`docs/decisions/semantics-v2.md` §7). Bodies intimidate, conspire's cost,
amplify and prowl. Standard constraints apply.

## Shape

- `Predicate.shares (axis : QualitySort) (agreement : Agreement) (with :
  NounPhrase)`, `Agreement = share | differ`: the member and `with` have at
  least one value in common on the axis (`share`) or none (`differ`)
  [CR#201.2a,201.2c,105.2,205.3m]. On the name axis `share` is "at least one
  name in common", never equality: an object with two names has the same
  name as an object with either one [CR#201.2a].
- `NounPhrase.agree (axis : QualitySort) (agreement : Agreement) (group :
  NounPhrase)`: the group-mutual form, a constraint on a selection ("two
  creatures that share a creature type"), not a predicate on one member.
- Checker: kinds only (`with` an object; `agree` over a plural group). It
  does not check that the axis is defined for the member's class.
- Rust mirror, RON helpers `shares` and `agree`, bench macros.

## Retires (re-spelled, same asserted outcome)

- `named(sameAs(X))` → `shares(cardName, share, X)`: every site in
  `lean/Semantics/Cards/` (Anaphora, Counters, Trigger, Cost, Keyword),
  `lean/Semantics/Proofs/` (Counters, Keyword, Zone, Piles), the bench and
  canon; 87 sites by the recon count, a mechanical re-spell.
  `NameSource.sameAs` goes if nothing else reads it.
- `namesAgree` (Phrase, `plugins_v2/builtin/macros/nouns/namesAgree.ron`,
  `Proofs/Zone.lean`) → `agree(cardName, share, …)`.
- `cardsShare (scope) (axis)` (Abilities, `Cards/Keyword.lean`,
  `Proofs/Keyword.lean`) → `agree` over the deck-scoped group.

## Bodies

- intimidate: can't be blocked except by artifact creatures and/or
  creatures `shares(color, share, it)` [CR#702.13b]. The "except" is
  `semantics-v2-off-stack-statics`' deontic `except : Option Cause`; if that
  has not landed, write intimidate as today's negated-predicate deontic over
  `or([artifactCreature, shares(color, share, it)])` and name the sibling in
  the landing record.
- conspire's cost: tap two untapped creatures you control that each
  `shares(color, share, thisSpell)` [CR#702.78a]. The copy half is
  `semantics-v2-spell-copies`'; if that has not landed, conspire stays
  bodyless with its STOP naming it.
- amplify: reveal any number of cards from hand `shares(subtype(creature),
  share, it)` [CR#702.38a], inside `semantics-v2-off-stack-statics`' entry
  clause `asEvent(enters(this), …)`. Body it here only if that ticket has
  landed; otherwise leave the STOP naming it.
- prowl: `happened(dealtCombatDamage(a(and([shares(subtype(creature), share,
  thisSpell), youControl])), a(player)), within(thisTurn))` [CR#702.76a];
  the creature's types are last-known information, the engine's
  [CR#608.2h].

## Witnesses (canon)

A "shares a creature type" non-keyword card, a "different name" (`differ`)
card [CR#201.2c], a "two creatures that share a color" card (`agree`), and a
canon card for each keyword bodied. Pins: `shares` on the name axis is not
equality (a two-named object matches either name); `agree` over one object is
refused as a plurality error.

## Siblings

Cross-references `semantics-v2-off-stack-statics` (except, entry clause)
and `semantics-v2-spell-copies` (conspire's copy); neither blocks. Refresh
before integrate; `Words`/`Phrase`/`Abilities` conflicts are expected.
