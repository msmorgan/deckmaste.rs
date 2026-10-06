---
needs: []
---
**The Lean bench writes "a card in <zone>" without `isCard`.** Residue of
`plugins-v2-keyword-helper-additions` (2026-10-05), which added the conjunct
only where a canon RON card writes `cardIn` and a Lean term mirrors it
(`okThoughtseize`). Standard constraints apply.

A token can sit in a hand, graveyard or exile until state-based actions are
checked [CR#111.7,704.5d], so `.inZone hand` alone also admits tokens. At the
landing, 139 bench lines under `lean/Semantics/Cards/` match `inZone` of a
hand, graveyard or exile (Choice 32, Damage 15, Keyword 13, Static 13,
Counters 11, Mana 9, Trigger 9, Anaphora 8, Cost 6, Description 6, Turn 6,
Piles 5, Faces 4, Copy 1, Deontic 1). Each needs reading against its printed
text ("a card", "a creature card", a token-admitting phrase), and pins whose
expected value changes are STOPs. Counted with `grep -rcE 'inZone
(hand|graveyard|exileZone|\(handOf|\(graveyardOf|yourHand|yourGraveyard)'
lean/Semantics/Cards/*.lean`.

## Landing record

The series, oldest first, on the claim `pyqvkwuyuylk`:

- S1 `zoyqlylmoppo`: "lean bench: a card in a zone is a card, as cardIn writes it"
  [CR#111.7,704.5d]. Bare `inZone <zone>` becomes `and [isCard, inZone <zone>]`,
  the conjunction order `cardIn` expands to, in qualification as the file already
  spells (`Primitives.Predicate.` in Cards, `.` in Proofs).

**What counted as a site.** A predicate that is exactly `inZone Z` with Z a hand,
graveyard, exile, library or possessed form (`handOf`, `graveyardOf`, `yourHand`,
`yourLibrary`), as the patient of discard, choose, exile, count, shuffle, reveal,
scaledMana, `cardsAcross`, `captureOperands` and the like. Edited by one script
over `lean/Semantics/Cards/*.lean` and `lean/Semantics/Proofs/*.lean`.

**Changed: 128 sites in 26 files.** No macro centralises the phrase
(`Macros.lean` has `inZone` only for `spell`, `permanent` and a battlefield
match), so there is no macro-level change; every site was edited in place.

- Cards, 81: Choice 22, Damage 11, Trigger 7, Description 6, Static 6, Anaphora 5,
  Counters 5, Keyword 5, Turn 4, Cost 3, Piles 3, Mana 2, Deontic 1, Faces 1.
- Proofs, 47: Zone 11, Anaphora 7, GetsBothDeltas 6, Deontic 5, ActionFamilies 4,
  Choice 3, Turn 3, Lookback 2, MacroParameters 2, Mana 2, Damage 1, Rules 1.

**Left alone, with reasons (about 120 sites).**

- `and [<type>, inZone <zone>]` where the type is a card type or subtype
  (`creature`, `land`, `instant`, `permanentCard`, `not land`, `room`, a subtype):
  "a creature card in your graveyard" already says card in the owner's reading.
  The RON canon spells none of these with `cardIn`, so nothing contradicts.
- `and [colorIs ..., inZone ...]`, `and [other, hasPossessor .., inZone exileZone]`,
  `and [named ..., inZone ...]`, `and [.., otherThan this, inZone yourHand]`
  (Cost 638, Static 623, Counters 47, Choice 275, Keyword 1735 and Proofs
  twins, Trigger 412, Anaphora 548 already `isCard` elsewhere): the predicate
  is not "a card" by type; not clear the printed text says "card"; listed, not
  changed.
- `matches X (inZone zone)` conditions about `this`, `it`, `thisCreature`
  (Keyword 1296, Damage 922, Proofs/Zone 379, 656): a given object's location,
  not a card-in-zone phrase.
- `.not (.inZone ..)`, `.or [.inZone hand, .inZone graveyard]` (Proofs/Zone 363),
  `the (.inZone exileZone)` (Zone 948), `tappedForMana ... (a (.inZone graveyard))`
  (Proofs/Mana 401): predicate-shape pins, not patients of a card move.
- `inZone battlefield`, `inZone stack`: permanents and spells.
- Proofs/Anaphora 700-739 (`verbedEvent ... (a (.inZone library))` etc.) were
  changed; Keyword 1720 and Zone 913 (`hasSubtype Aura, inZone`) left as typed.

**Pins stopped on: 0.** No pin's expected value changed; all builds and `decide`
pins held at their old outcome (`isCard` adds no check finding).

**Proof.**

- `cd lean && ./scripts/build`: 82 jobs, build completed successfully, every
  pin at its old outcome.
- `cargo xtask lean-check`: canon 127/127, testing 5/5.
- `cargo xtask facts check`: up to date. `cargo xtask cite check
  --list-noncompliant`: 0; `cite check`: 15997 citations, 0 stale. Cites read
  against `data/rules/cr.txt`: [CR#111.7] (token in a non-battlefield zone ceases to
  exist, a state-based action) and [CR#704.5d] (the SBA). Nothing blessed.
- Canon cards writing `cardIn` (Thoughtseize, Deadeye Tracker, Dangerous Wager,
  Stormbind, Farm-Market): only Thoughtseize has a Lean counterpart
  (`okThoughtseize`, `and [not land, and [isCard, inZone (handOf they)]]`,
  spelling the same as `and([not(land), cardIn(handOf(they))])`).
  The other four have none under `lean/Semantics`.

**Assurance counts.** Restored 0, re-spelled 0, ignored 0, added 0, removed 0
tests; 128 terms gained a conjunct.

**Ticket corrections.** The ticket's count of 139 was by a grep of
`inZone` of a zone name including typed conjunctions; the bare-patient count
is 128 (Cards 81, Proofs 47). It named only Cards; Proofs held 47 sites.

Not applicable: Rust gate (`cargo xtask gate --changed` has no changed Rust
crate; only `lean/` changed), coverage lock and corpus figures, performance
advisory, glossary gap (none).
