---
needs: []
---
**"Doesn't lose the game for having 0 or less life" is a deontic, not a
constructor.** `StaticSpec.noLossFromZeroLife (player)` in
`lean/Semantics/Abilities.lean` hard-codes one card sentence (Lich, Phyrexian Unlife).
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

## Landing record

Change `nzvxnvzqultsxlzsvprunszkmyumnuvy`; English lock `covered`: 20,254.

**PROVE:** The dedicated static constructor and its checking, introduction,
and numerical-slot cases are removed. Phyrexian Unlife now spells the same
accepted assertion as a loss prohibition with a state-based cause. The shared
deontic laws enforce the player/agent relationship and untargeted static
abilities; the rider requires every deed to match the cause's closed
`CoreDeed`. Nine new exact-result pins cover players, player groups, creatures,
targeted static and continuous uses, an unrelated deed, mixed deeds, an empty
deed list, and the patient role. Assurance: 1 existing card pin re-spelled;
9 added; 0 restored, ignored, or removed. All existing assertions retain
their expected results. The refreshed `lean/scripts/build` passed all 61 jobs
without warnings. LSP diagnostics are empty for the proof and card modules;
the audited mixed-deed refusal proof uses only `propext`.

**DISCLOSE:** `DeonticRider.stateBased` was chosen because a cause qualifies
why the deed happens, whereas `DeonticPatient` identifies another participant.
`StateBasedCause.nonpositiveLife` names the condition, independently of the
prohibition and its subject. The Game Model glossary now defines State-Based
Cause. There is no card-name or vocabulary-string guard. No accepted card is
lost and no new card acceptance is claimed. Deviations and additions: the
nine boundary pins and glossary entry support the replacement; no unresolved
STOPs or glossary gaps.

**REPORT:** English coverage and declarations are untouched; English selection,
structural inventories, licensing counts and performance were not remeasured.
Citation checks report 0 noncompliant and 0 stale sites; all 5 changed sites
were read against their rule text.
