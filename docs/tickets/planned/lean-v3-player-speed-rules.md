---
needs: [plugins-v3-builtin-primitives-and-templates]
---
# Speed: the start your engines! state-based action and the speed trigger

Standard constraints apply.

Start your engines! is bodiless: its rule is a state-based action, "If a player controls a
permanent with start your engines! and that player has no speed, their speed becomes 1"
[CR#702.179a,704.5aa], a rules-table row (`SbaRule`) beside deathtouch's
(`Proofs/Rules.lean`, `deathtouchDestruction`). The row cannot be written yet:

- No `Action` sets or raises a player's speed. `speedOf` (`.playerStat .speed`) reads it.
- "Has no speed" is not "speed 0": players have no speed until a rule or effect sets it
  [CR#702.179b]. 0 is only what an effect reading speed sees [CR#702.179f]. The row needs a
  condition that a player has a speed.

Add both: an action that sets a player's speed and raises it by an amount (raising from no speed
sets it [CR#702.179c]), and the has-speed condition. Then write the row [CR#704.5aa] and the inherent
triggered ability every player with speed 1 or more has [CR#702.179d], a sourceless ability
controlled by that player, written as the designation rows write theirs
(`Definitions/DesignationRules.lean`).
