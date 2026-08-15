# Census of CR designations

2026-08-15. Source: `data/rules/cr.txt`, the Comprehensive Rules snapshot
effective August 7, 2026.

This is a complete census of the formal *designations* the Comprehensive Rules
define — statuses that a player, an object, or the game itself **holds**, as
distinct from the rules' ordinary use of "designate" as a verb. It was built by
sweeping every occurrence of the word family (`designat*`, case-insensitive) in
the CR text — 91 matching lines, including the glossary — and classifying each
one. Every hit is accounted for: it either defines or supports one of the
entries below, or appears in the appendix of excluded verb-only uses.

Two groups. **Group A** are the designations the CR names as such ("X is a
designation …"). **Group B** are persistent statuses the CR confers with
"designated as" phrasing and, in some cases, calls a "designation" in passing,
but never introduces with a defining "is a designation" sentence; they are
included because a player or card holds them across time in the same way.

Gain/loss facts below are only what the rules state. The CR gives no
designation-specific rule for restarting the game: a restart ends the game and
all players begin a new one under the normal starting procedure
[CR#103,727.1]. Only the commander designation is stated to survive a zone
change [CR#903.3].

## Summary

| Designation | Holder | Defining rule | Exclusivity |
| --- | --- | --- | --- |
| Goaded | permanent | [CR#701.15b] | per-object binary; tracked separately per goading player |
| Monstrous | permanent | [CR#701.37b] | per-object binary, one-way while on the battlefield |
| Ring-bearer | permanent | [CR#701.54b] | one per player at a time |
| Suspected | permanent | [CR#701.60b] | per-object binary |
| Harnessed | permanent | [CR#701.64b] | per-object binary, one-way while on the battlefield |
| Renowned | permanent | [CR#702.112b] | per-object binary, one-way while on the battlefield |
| City's blessing | player | [CR#702.131c] | per-player binary; any number of players, never lost |
| Sector (alpha/beta/gamma) | permanent | [CR#702.158b] | three-valued, exactly one per permanent that has any |
| Saddled | permanent | [CR#702.171b] | per-object binary, until end of turn |
| Enduring story | player | [CR#702.195b] | per-player binary; any number of players, never lost |
| Left half unlocked / right half unlocked | permanent | [CR#709.5c] | paired independent binaries on one permanent |
| Level | permanent | [CR#716.2b] | numeric, one value per permanent |
| Solved | permanent | [CR#719.3b] | per-object binary, one-way while on the battlefield |
| Prepared | permanent | [CR#722.3a] | per-object binary; cannot be gained twice over |
| Monarch | player | [CR#725.1] | unique per game (or none) |
| Initiative | player | [CR#726.1] | unique per game (or none) |
| Day / night | the game | [CR#731.1] | game-level; neither, then exactly one forever after |
| Protector † | player, per battle | [CR#310.9] | one protector per battle |
| Planar controller † | player | [CR#311.5] | normally one; several in Grand Melee |
| Commander † | card | [CR#903.3] | one per deck, or two with a partner ability |
| Archenemy † | player | [CR#904.2a] | exactly one player in an Archenemy game |

† Group B — conferred by "designated as" phrasing; see the note above.

## Group A — designations the CR names as such

### Goaded

- **Holder:** object (permanent). **Defined:** [CR#701.15b].
- **Gained:** a spell or ability goads the creature [CR#701.15a].
- **Lost:** expires "until the next turn of the controller of that spell or
  ability" [CR#701.15a]. Not part of the permanent's copiable values, and
  neither an ability [CR#701.15b].
- **Exclusivity:** per-object binary, but goading is tracked per player: a
  creature can be goaded by multiple players, creating additional combat
  requirements, while the same player goading it again does nothing
  [CR#701.15c,701.15d].
- > "Goaded is a designation a permanent can have." [CR#701.15b]

### Monstrous

- **Holder:** object (permanent). **Defined:** [CR#701.37b].
- **Gained:** resolving a "monstrosity N" instruction, which also puts N +1/+1
  counters on the permanent if it isn't already monstrous [CR#701.37a].
- **Lost:** only by leaving the battlefield [CR#701.37b]. Not copiable, not an
  ability.
- **Exclusivity:** per-object binary and one-way for as long as the permanent
  stays on the battlefield.
- > "Once a permanent becomes monstrous, it stays monstrous until it leaves
  > the battlefield." [CR#701.37b]

### Ring-bearer

- **Holder:** object (permanent). **Defined:** [CR#701.54b].
- **Gained:** each time the Ring tempts you, you choose a creature you control
  and it becomes your Ring-bearer [CR#701.54a].
- **Lost:** when another creature becomes your Ring-bearer, or when another
  player gains control of it [CR#701.54a]. Not a copiable value [CR#701.54b].
  Abilities that ask whether a creature "is your Ring-bearer" require it to be
  on the battlefield under your control and to have the designation
  [CR#701.54e].
- **Exclusivity:** one Ring-bearer per player at a time; different players may
  each have one.
- > "Ring-bearer is a designation a permanent can have." [CR#701.54b]

### Suspected

- **Holder:** object (permanent). **Defined:** [CR#701.60b].
- **Gained:** a spell or ability suspects the creature [CR#701.60a].
- **Lost:** when it leaves the battlefield, or when a spell or ability causes
  it to no longer be suspected [CR#701.60a]. Neither an ability nor copiable
  [CR#701.60b].
- **Exclusivity:** per-object binary; a suspected permanent can't become
  suspected again [CR#701.60d]. While suspected it has menace and "This
  creature can't block" [CR#701.60c].
- > "Suspected is a designation a permanent can have. Only permanents can have
  > the suspected designation." [CR#701.60b]

### Harnessed

- **Holder:** object (permanent). **Defined:** [CR#701.64b].
- **Gained:** a "harness [this permanent]" instruction, which does nothing if
  the permanent is already harnessed [CR#701.64a].
- **Lost:** only by leaving the battlefield [CR#701.64b]. Neither an ability
  nor copiable.
- **Exclusivity:** per-object binary, one-way while on the battlefield.
- > "Harnessed is a designation that has no rules meaning other than to act as
  > a marker that other spells and abilities can identify." [CR#701.64b]

### Renowned

- **Holder:** object (permanent). **Defined:** [CR#702.112b].
- **Gained:** the renown triggered ability resolving after the creature deals
  combat damage to a player, if it isn't already renowned [CR#702.112a].
- **Lost:** only by leaving the battlefield [CR#702.112b]. Neither an ability
  nor copiable.
- **Exclusivity:** per-object binary; with multiple instances of renown, the
  first to resolve makes the creature renowned and the rest do nothing
  [CR#702.112c].
- > "Renowned is a designation that has no rules meaning other than to act as
  > a marker that the renown ability and other spells and abilities can
  > identify." [CR#702.112b]

### City's blessing

- **Holder:** player. **Defined:** [CR#702.131c].
- **Gained:** ascend, as a spell ability or a static ability, once you control
  ten or more permanents and don't already have it — "for the rest of the
  game" [CR#702.131a,702.131b].
- **Lost:** never; the rules state no loss condition.
- **Exclusivity:** per-player binary, non-exclusive — any number of players may
  have it simultaneously [CR#702.131c]. After a player gets it, continuous
  effects are reapplied before trigger conditions are checked [CR#702.131d].
- > "The city's blessing is a designation that has no rules meaning other than
  > to act as a marker that other rules and effects can identify." [CR#702.131c]

### Sector designations (alpha sector, beta sector, gamma sector)

- **Holder:** object (permanent). **Defined:** [CR#702.158b].
- **Gained:** as a state-based action, while a permanent with space sculptor
  and creatures without a sector designation are on the battlefield together:
  players who don't control a space sculptor permanent choose sectors for
  their creatures first, then the other players do
  [CR#702.158a,702.158c,704.5u].
- **Lost:** kept "until no player controls a permanent with space sculptor or
  an ability whose source has space sculptor" [CR#702.158b]. Not a copiable
  value.
- **Exclusivity:** three-valued rather than binary — a permanent that has a
  sector designation has exactly one of the three. Two permanents are in the
  same sector if each has the same sector designation [CR#702.158e]; abilities
  that "choose a sector" pick one of the three designations [CR#702.158d].
- > "A sector designation is a designation a permanent can have. The sector
  > designations are alpha sector, beta sector, and gamma sector."
  > [CR#702.158b]

### Saddled

- **Holder:** object (permanent). **Defined:** [CR#702.171b].
- **Gained:** activating a saddle ability, tapping other untapped creatures
  with total power N or greater [CR#702.171a].
- **Lost:** at end of turn, or when the permanent leaves the battlefield
  [CR#702.171b]. Not part of copiable values.
- **Exclusivity:** per-object binary, and unlike most markers it expires with
  the turn.
- > "Once a permanent has become saddled, it stays saddled until the end of the
  > turn or it leaves the battlefield." [CR#702.171b]

### Enduring story

- **Holder:** player. **Defined:** [CR#702.195b].
- **Gained:** the storied static ability, any time you control three or more
  permanents that are artifacts, Sagas, and/or legendary and you don't have one
  — "for the rest of the game" [CR#702.195a].
- **Lost:** never; the rules state no loss condition.
- **Exclusivity:** per-player binary, non-exclusive — any number of players may
  have it at the same time [CR#702.195b]. Continuous effects are reapplied
  before trigger conditions are checked afterwards [CR#702.195c].
- > "Enduring story is a designation that has no rules meaning other than to
  > act as a marker that other rules and effects can identify." [CR#702.195b]

### Left half unlocked / right half unlocked (the unlocked designations)

- **Holder:** object (permanent on the battlefield). **Defined:** [CR#709.5c].
- **Gained:** as the permanent enters, for whichever half was cast as a spell
  [CR#709.5d]; by paying a locked half's mana cost as a special action, the
  "unlock cost" [CR#116.2m,709.5e]; or by an effect instructing a player to
  "unlock" a half [CR#709.5f].
- **Lost:** by an effect instructing a player to "lock" a half, which removes
  the appropriate unlocked designation [CR#709.5g].
- **Exclusivity:** a paired binary — one permanent can hold neither, either, or
  both. The shared type line's two static abilities suppress the name, mana
  cost, and rules text of any half whose designation is absent [CR#709.5].
  Abilities keyed on unlocking trigger on the designation being given, whether
  on entry or later [CR#709.5h]; "fully unlocks" triggers cover gaining the
  second designation or both at once [CR#709.5i].
- > "'Left half unlocked' and 'right half unlocked' are designations that a
  > permanent on the battlefield can have. Together, they are called the
  > unlocked designations." [CR#709.5c]

### Level

- **Holder:** object (any permanent). **Defined:** [CR#716.2b].
- **Gained/changed:** activating a class level bar — "[Cost]: This Class's
  level becomes N", activatable only if the Class is level N-1 and only as a
  sorcery [CR#716.2a].
- **Lost:** no loss condition is stated; the rules say instead that a Class
  retains its level even if it stops being a Class [CR#716.2b]. A permanent
  with no level is treated as level 1 when a rule or effect refers to its level
  [CR#716.2d]. Levels are not a copiable characteristic.
- **Exclusivity:** numeric rather than binary — one level value per permanent.
- > "A level is a designation that any permanent can have." [CR#716.2b]

### Solved

- **Holder:** object (permanent). **Defined:** [CR#719.3b].
- **Gained:** the "to solve — [condition]" triggered ability, at the beginning
  of your end step if the condition holds and the Case isn't already solved
  [CR#719.3a].
- **Lost:** only by leaving the battlefield [CR#719.3b]. Neither an ability nor
  copiable.
- **Exclusivity:** per-object binary, one-way while on the battlefield. A
  Case's "Solved — [ability text]" functions only while the Case has the
  designation [CR#719.3c].
- > "Solved is a designation a permanent can have." [CR#719.3b]

### Prepared

- **Holder:** object (permanent). **Defined:** [CR#722.3a].
- **Gained:** effects that cause a permanent with a prepare spell to become
  prepared, or that state it enters prepared; a permanent can't gain it unless
  it has a prepare spell, nor gain it if it already has it [CR#722.3a]. On
  gaining it (or phasing in prepared), the controller creates a copy in exile
  with only the prepare spell's characteristics, castable while it remains
  [CR#722.3c].
- **Lost:** an effect making the permanent "unprepared" [CR#722.3b], or the
  moment the exiled copy becomes cast [CR#722.3c].
- **Exclusivity:** per-object binary; explicitly non-stacking.
- > "Prepared is a designation that acts as a marker which rules and effects
  > can identify." [CR#722.3a]

### Monarch

- **Holder:** player. **Defined:** [CR#725.1].
- **Gained:** an effect instructing a player to become the monarch; there is no
  monarch until one does [CR#725.1]. The inherent triggered ability "Whenever a
  creature deals combat damage to the monarch, its controller becomes the
  monarch" also transfers it [CR#725.2].
- **Lost:** when another player becomes the monarch [CR#725.3]. If the monarch
  leaves the game, the active player becomes the monarch simultaneously, else
  the next player in turn order who can; if no one can, the game continues with
  no monarch [CR#725.4].
- **Exclusivity:** unique per game — at most one monarch, possibly none. A
  continuous effect that depends on who the monarch is does nothing while there
  is none [CR#725.5].
- > "The monarch is a designation a player can have." [CR#725.1]

### The initiative

- **Holder:** player. **Defined:** [CR#726.1].
- **Gained:** an effect instructing a player to take the initiative; there is
  none until one does [CR#726.1]. An inherent triggered ability transfers it
  when creatures deal combat damage to the player who has it [CR#726.2].
- **Lost:** when another player takes it [CR#726.3]. If the player who has it
  leaves the game, the active player takes it simultaneously, else the next
  player in turn order [CR#726.4].
- **Exclusivity:** unique per game — only one player at a time, and
  re-instructing the current holder triggers the venture ability without
  creating a second initiative designation [CR#726.5].
- > "The initiative is a designation a player can have." [CR#726.1]

### Day and night

- **Holder:** **the game itself** — the one designation in the census held by
  neither a player nor an object. **Defined:** [CR#731.1].
- **Gained:** the game starts with neither designation; daybound and nightbound
  abilities, and other effects, can make it day or night [CR#731.1].
- **Lost:** only by swapping to the other value — "day becomes night" and
  "night becomes day" mean the game loses the first designation and gains the
  second [CR#731.1a]. There is no route back to holding neither.
- **Exclusivity:** game-level and three-state-then-two: neither at first, then
  exactly one of day or night from that point forward. The untap step checks
  the previous turn to see whether it should change, and skips the check
  entirely if the game has neither designation
  [CR#703.4b,703.4c,731.2,731.2a,731.2b].
- > "Day and night are designations that the game itself can have. The game
  > starts with neither designation. … Once it has become day or night, the
  > game will have exactly one of those designations from that point forward."
  > [CR#731.1]

## Group B — statuses conferred by "designated as"

### Protector (of a battle)

- **Holder:** player, but held per battle rather than globally. **Defined:**
  [CR#310.9]. The glossary entry "Protect, Protector" points here; the CR never
  writes "protector is a designation".
- **Gained:** as a battle enters the battlefield, its controller chooses a
  player permitted by the battle's battle type; with no battle type, only the
  controller may be chosen [CR#310.9a]. A Siege's protector must be an opponent
  of its controller [CR#310.12a].
- **Lost:** when another player becomes that battle's protector [CR#310.9f]. It
  does not change if the permanent stops being a battle or becomes a copy of
  another battle [CR#310.9g]. A state-based action repairs a battle with no
  protector or an ineligible one, and puts the battle into its owner's
  graveyard if no player can be chosen [CR#310.11,704.5x].
- **Exclusivity:** one protector per battle at a time; one player may protect
  several battles.
- > "Each battle has a player designated as its protector." [CR#310.9]

### Planar controller

- **Holder:** player. **Defined:** [CR#311.5], restated verbatim for phenomena
  and in the Planechase rules [CR#312.4,901.6] and referenced from
  [CR#109.4d,800.4p].
- **Gained:** normally the active player is the planar controller [CR#311.5].
  If the current planar controller would leave the game, the next player in
  turn order who wouldn't leave becomes it first [CR#311.5,800.4p]. In Grand
  Melee, each player who starts with a turn marker sets a starting plane and is
  a planar controller [CR#901.14a].
- **Lost:** "The new planar controller retains that designation until they
  leave the game or a different player becomes the active player, whichever
  comes first" [CR#311.5]. In Grand Melee, a departing player ceases to be a
  planar controller with no replacement when their leaving would reduce the
  turn-marker count [CR#901.14b].
- **Exclusivity:** normally one per game; several simultaneously in Grand Melee
  [CR#901.14a]. Under shared team turns the planar controller is the primary
  player of the active team, with the same retention clause [CR#901.12b].
- > "The controller of a face-up plane card is the player designated as the
  > planar controller." [CR#311.5]

### Commander

- **Holder:** the **card** — explicitly not the object represented by it.
  **Defined:** [CR#903.3]; introduced at [CR#903.1] and restated for the Brawl
  option at [CR#903.12c].
- **Gained:** during deck construction, one legendary card per deck, which must
  be a creature card, a Vehicle card, or a Spacecraft card with one or more
  power/toughness boxes [CR#903.3]. Partner abilities let a player designate
  two cards instead of one [CR#702.124a].
- **Lost:** never stated to be lost. This is the only designation the CR states
  survives a zone change. A melded or merged permanent that includes the
  commander is that player's commander [CR#903.3b,903.3c].
- **Exclusivity:** one per deck, or two under a partner ability; no combination
  of partner abilities can give a player more than two [CR#702.124g].
- > "This designation is not a characteristic of the object represented by the
  > card; rather, it is an attribute of the card itself. The card retains this
  > designation even when it changes zones." [CR#903.3]

### Archenemy

- **Holder:** player. **Defined:** [CR#904.2a]. The CR uses only the verb form;
  the glossary defines the archenemy as the player in an Archenemy game playing
  with a scheme deck.
- **Gained:** fixed at setup — one of the two teams consists of exactly that
  one player [CR#904.2a].
- **Lost:** never during play; the rules state no transfer or loss.
- **Exclusivity:** exactly one player per Archenemy game.
- > "One of the teams consists of exactly one player, who is designated the
  > archenemy." [CR#904.2a]

## Appendix — excluded verb-only uses

Fourteen rule texts use the word family as an ordinary verb or adjective
without conferring a status that a player, object, or the game holds. They are
listed so the sweep can be seen to be exhaustive.

- [CR#100.3] — "specially designated cards" among the game materials some
  casual variants require. Adjective for a physical card class, not a status.
- [CR#408.3] — the same phrase, for cards that start the game in the command
  zone in Planechase, Vanguard, Commander, Archenemy, and Conspiracy Draft.
- [CR#702.124a,702.124f..702.124k] and [CR#702.124m] — the partner abilities'
  "you may designate two legendary cards as your commander." This is the *act*
  of assigning the commander designation defined at [CR#903.3], not a separate
  designation; see the Commander entry.
- [CR#705.1] — coin flips: "designate one side to be 'heads,'" and designating
  that odds means heads on a substituted die roll. A one-off labelling of a
  physical object.
- [CR#807.4e..807.4g] — Grand Melee turn markers "designated for removal".
  This is the closest borderline case: the marking persists, attaches to a
  specific turn marker, and can even stack (a marker may be designated for
  removal multiple times, and removal decrements the count on the marker to its
  right). It is excluded because a turn marker is a game accessory rather than
  an object or player, and the CR never calls this a designation — it is a
  scheduled-removal flag consumed by [CR#807.4g].

The remaining matches all support entries above rather than introducing
anything new: the supporting subrules cited in each section, the restatements
of the planar controller rule [CR#109.4d,312.4,800.4p,901.6], the state-based
actions for sectors and protectors [CR#704.5u,704.5x], the day/night untap
checks [CR#703.4b,703.4c], and the twenty-five glossary entries (city's
blessing, commander, day, night, enduring story, goaded, harness, harnessed,
initiative, level, lock, locked, monarch, monstrous, prepared, protect/
protector, renowned, Ring-bearer, saddled, solved, storied, unlock, unlocked,
unprepared, ascend), each of which points at a rule already covered here.
