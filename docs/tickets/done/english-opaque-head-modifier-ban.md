---
needs: []
---
**Measure, then ban, the opaque-nominal-head-with-known-modifier shape.**
The degenerate reading where an `Opaque` nominal HEAD carries a non-opaque
`NominalModifier::Noun` (a known noun demoted to modifier under an unknown
head) has produced wrong-but-round-tripping trees in two unrelated rounds:
the declarestep `cleanup`-split defect (2026-07-25) and the pluralposs
`their owners' hands except` / `all nonland permanents not` defects
(2026-07-25, neutralized for `except`/`not` specifically by the
`has_known_word` literal repair). The conjecture: that shape is never a right
tree in this corpus — real English noun-noun compounds put the UNKNOWN word in
modifier position, not head position, when the head is known. Work: a
corpus-wide categorical measurement in the declarestep-C style (thread an
`opaque_head`-style flag through `Features::Noun` → `Features::Nominal` and
enumerate every face where the shape occurs today), then — if the measurement
confirms zero legitimate occurrences — a features-level gate rejecting the
shape, so it becomes unconstructible rather than out-competed. No cost
changes. Note the standing instrument caveat: the noun-opacity census counts
opaque HEADS only, so this gate may shift census bookkeeping
(head↔modifier); require per-row attribution.

## Completion

The selected-tree measurement found 153 instances on 102 supported faces.
Every instance was a false nominal: either a proper name after `named` (with
`card`, `token`, a card type, or a creature type demoted beneath the name), a
named-card cross-reference, or clause residue such as `also`, `but`, `using`,
`since`, `yet`, `simultaneously`, or a participle promoted to an opaque head.
No legitimate English compound used a known noun modifier with an unknown
head.

The 102 attributed faces were: Agent of Acquisitions; Ajani, Strength of the
Pride; Alpine Houndmaster; Angel's Herald; Archon's Glory; Armageddon Clock;
Attempted Murder; Awakening of Vitu-Ghazi; Barracks of the Thousand;
Behemoth's Herald; Bogbrew Witch; Bubbling Cauldron; Cloudseeder; Crown of
Empires; Crumb and Get It; Demon's Herald; Diligent Farmhand; Diseased Vermin;
Dragon's Herald; Efflorescence; Ekundu Cyclops; Ellie, Brick Master; Ellywick
Tumblestrum; Endbringer's Revel; Everquill Phoenix; Expressive Firedancer; Eye
of Duskmantle; Farid, Enterprising Salvager; Festering Newt; Forsworn Paladin;
Gemini Engine; Goblin Welder; Goldmeadow Lookout; Great Unclean One; Heroic
Charge; Icatian Skirmishers; Infinite Hourglass; Joint Assault; Jungle Patrol;
Karakyk Guardian; Kibo, Uktabi Prince; Koma, Cosmos Serpent; Koma, World-Eater;
Kookus; Kytheon's Tactics; Lightning Storm; Liliana's Triumph; Lita, Mechanical
Engineer; Lullmage's Domination; Mana Cache; Marauding Maulhorn; Marchesa,
Resolute Monarch; Minamo's Meddling; Mine Worker; Nahiri, the Lithomancer;
Nazahn, Revered Bladesmith; Nissa Revane; Nurgle's Rot; Okk; Palladia-Mors, the
Ruiner; Pardic Firecat; Pious Kitsune; Power Plant Worker; Princess Yue;
Ratonhnhaké꞉ton; Renowned Weaponsmith; Rite of the Raging Storm; Rootha,
Mastering the Moment; Rufus Shinra; Ruin in Their Wake; Sands of Time;
Scandalmonger; Scarred Puma; Scepter of Empires; Shell Shield; Shield of
Kaldra; Sower of Discord; Sphinx's Herald; Spikeshell Harrier; Svella, Ice
Shaper; Task Mage Assembly; Tatsumasa, the Dragon's Fang; Tatsunari, Toad
Rider; Tecutlan, the Searing Rift; Temur Battle Rage; Tezzeret the Schemer; The
Curse of Fenric; The Eleventh Hour; The Hive; The Myriad Pools; Throne of
Empires; Tombstone Stairwell; Tooth and Claw; Tower Worker; Urza's Engine;
Volrath's Dungeon; Wall of Kelp; Weapons Manufacturing; Well of Knowledge;
When We Were Young; Wirefly Hive; and Yotia Declares War.

`Features::Noun` now records whether a lexical noun is opaque, and
`Features::Nominal` preserves whether its head is opaque through every
attachment. `NominalNounModifier` categorically rejects only the measured bad
direction: a known noun before an opaque head. The grammatical inverse remains
available, including multiple opaque modifiers before a known head (`a shiny
strange card`). No costs changed.

The normalized full unresolved-dump diff accounts for all 102 measured faces:
their 158 lexical-noun rows disappear and become 100 new structural spans;
Rufus Shinra and Tatsunari, Toad Rider expand existing recoveries instead of
adding an occurrence. No unattributed face moves. Structural recovery changes
from 3,421 spans / 63,445 source tokens to 3,521 / 65,182, while noun opacity
changes from 964 to 806 and flavor-header opacity remains 622. This is the
intended bookkeeping shift from rejecting wrong-but-round-tripping trees.

All 567 library tests and 111 public-API tests pass. The supported corpus
round-trips 31,685/31,685 clean with zero mismatches or render errors.
