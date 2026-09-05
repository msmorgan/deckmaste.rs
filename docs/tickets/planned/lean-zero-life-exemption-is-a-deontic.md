---
needs: []
---
**"Doesn't lose the game for having 0 or less life" is a deontic, not a
constructor.** `StaticSpec.noLossFromZeroLife (player)` in
`Semantics/Abilities.lean` hard-codes one card sentence (Lich, Phyrexian Unlife).
Platinum Angel's and Lich's Mastery's "you can't lose the game" already goes
through `StaticSpec.deontic` as `playerCant (.core .loseGame)`; the zero-life
sentence is the same deontic narrowed to one cause, the state-based action of
[CR#704.5a]. Give the deontic a cause qualifier (a slot on `DeonticRider` or a
`DeonticPatient` variant naming a state-based cause; the claimant chooses and
records why), express the sentence as `cant [.core .loseGame]` with that cause,
delete `noLossFromZeroLife`, and re-spell its pins against the deontic with
the same expected lists. The checker's laws for the deontic then cover it
(player subject, no target), and `AbilityRules`/`Abilities` lose their special
cases.

Decisions already made: core-rules deeds are the closed `CoreDeed` taxonomy
(landed 2026-09-05), so `loseGame` is matched structurally; a constructor that
exists for one card's wording is the shape the review rejected.
