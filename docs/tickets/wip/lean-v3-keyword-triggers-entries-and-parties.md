---
needs: []
---
# Keyword Definitions: delayed-trigger timing, entry replacements, roll modifiers, holders and parties

Standard constraints apply. Provenance: `docs/evidence/mtg-rules-review-2026-10-21/departures-lean.md`
#16, #27, #29, #33, #40, #46, #57, #58, #62, #63. Verified against head 2026-10-21. All latent in
Lean; the `plugins_v2` RON mirror named in each item has the same shape and changes with it.

## 1. Blitz, dash and warp create their delayed trigger as the permanent enters

Blitz (`Definitions/Keywords.lean:316-335`, the trigger at `:319-323`), dash (`:435-445`) and warp
(`:1537-1552`) create it with `.asEvent (does you (.pay .itsManaCost … (some (.named K))))` during
casting and reach the permanent through `.becomes thisSpell`. The cited rule, [CR#603.7g], covers
only a static ability that "allows a player to take an action". These delayed triggers are created
"as that permanent is put onto the battlefield" [CR#608.3g], and their controller is "the player
who controlled that spell as it resolved" [CR#603.7d]. The models diverge when the spell is
countered (a trigger is left over), when the spell changes control before resolving (the caster
cannot sacrifice a permanent they don't control [CR#701.21a]), and when it is copied (the copy's
token permanent [CR#608.3f] gets no trigger).

Change: key the trigger to the permanent being put onto the battlefield as the spell resolves,
guarded by `keywordCostPaid`, and name the permanent directly. Cite [CR#608.3g,603.7a,603.7d].
Mirrors: `blitz.ron`, `dash.ron`, `warp.ron` (`asEvent(`). Pins: countered, stolen and copied
spells.

## 2. Off-Stack Static citations

`asEvent` and `whileHolds` (`Abilities.lean:909-917`) say they work "without using the stack
[CR#604.1]" and that a delayed trigger takes the static's object as source [CR#603.7a,603.7g]. The
same text is in `Proofs/OffStack.lean:7-11`, `docs/contexts/game-model/CONTEXT.md:336-341`
(**Off-Stack Static**), `crates/deckmaste_semantics_v2/src/abilities.rs:757-760` and
`docs/decisions/semantics-v3.md:563`. [CR#604.1] does not mention the stack; "Static abilities
don't use the stack" is [CR#113.9]. After item 1 no `asEvent` creates a delayed trigger through
[CR#603.7g].

Change: cite [CR#113.9] for the no-stack claim (add it to `whileHolds`) and [CR#608.3g,603.7d] for
the delayed-trigger source and controller. Mark any remaining use for an action-performing static
as an inference by analogy. Amend the glossary entry through the `domain-modeling` skill with the
corrected citation.

## 3. An Aura with no legal object to enchant doesn't enter

The enchant entry clause (`Definitions/Keywords.lean:582-588`) is `.asEvent (does .any (.move
thisAura (not stack) (some battlefield) .arrival none [])) (attachTo thisAura (a (.inCaller
quality)))`, which treats an empty choice as "if able": the Aura still enters, then is put into the
graveyard by [CR#704.5m], so its enters and leaves triggers fire. [CR#303.4g]: "the Aura remains in
its current zone, unless that zone is the stack … If the Aura is a token, it isn't created".
[CR#303.4f] applies only when the effect "doesn't specify the object or player"; [CR#303.4i] covers
a specified host it can't legally enchant.

Change: model [CR#303.4f,303.4g,303.4i] as a replacement on the entry (arguably on the Aura
subtype). If a legal host exists, the Aura enters attached to one its controller chooses; if none
exists, it does not move (graveyard if it was on the stack; a token is not created). Mirror:
`enchant.ron:31`. The archived ADR's "[CR#303.4f,303.4g,101.3]" for "if able"
(`docs/archive/decisions/semantics-v2.md:708-712`) is a miscitation; it gets an erratum note, not a rewrite
(the archive errata are collected in
[docs-superseded-and-archived-errata](docs-superseded-and-archived-errata.md)).

## 4. Sunburst ignores type-changing effects

Sunburst (`Definitions/Keywords.lean:1399-1410`) branches on `doIf (is thisPermanent isCreature)`.
[CR#702.44a]: "If this object is entering as a creature, ignoring any type-changing effects that
would affect it"; otherwise [CR#614.12] would count existing continuous effects, so *Engineered
Explosives* under *March of the Machines* would get +1/+1 counters. No such construct exists;
`sunburst.ron:8` quotes the clause but implements nothing.

Change: add a predicate "would enter as a creature, ignoring type-changing effects", use it in both
branches, and keep the [CR#702.44b] colored-mana gate explicit. Whether a type-setting
characteristic-defining ability [CR#613.1d,613.3] counts as ignored is underdetermined; record
the choice as a UD entry in `docs/engine-adrs.md` rather than deciding it silently.

## 5. Gift's permanent half is an intervening "if" on the gift cost

`Definitions/Keywords.lean:737-749` is `when (does thisPermanent enter) (if_ := some (is this_
isPermanentType)) (doIf (keywordCostPaid (.named "gift") this_) …)`, so it triggers and resolves
when no gift was promised. [CR#702.174b]: "When this permanent enters, if its gift cost was paid,
[effect]"; under [CR#603.4] it otherwise does not trigger, which matters once [CR#702.174c]'s
give-a-gift triggers (*Jolly Gerbils*) are modeled.

Change (no `whenIf` Template exists; use the offspring shape, `:1061-1065`): `when (does
thisPermanent enter) (if_ := some (and [is this_ isPermanentType, keywordCostPaid (.named "gift")
this_])) (instruct (the isChosenPlayer) (.inCaller promise))`. Same in `gift.ron:24-31`.

## 6. Hideaway's look right follows the permanent's controller

`Definitions/Keywords.lean:828-838` grants `visibility .lookAt you` only, fixed at resolution.
[CR#702.75a]: "The player who controls the permanent that exiled this card may look at this card";
a past viewer keeps the right [CR#406.3] (*Windbrisk Heights* ruling, 2022-04-29). Keep the grant
to `you` and add one to the permanent's current controller, re-evaluated continuously. Mirror:
`hideaway.ron:21-24`.

## 7. Hexproof, shroud and protection on players

Hexproof (`Definitions/Keywords.lean:810-812`), protection (`:1115-1135`) and shroud
(`:1279-1280`) name only `thisPermanent`/`thisCreature`, yet a player grant (*Leyline of
Sanctity*, *Ivory Mask*, *Teferi's Protection*) is accepted. [CR#702.11c] defines hexproof "on a
player"; [CR#702.18a] "This permanent or player can't be the target"; [CR#702.16b,702.16c,702.16e]
cover a "permanent or player". Write the bodies over the holder (`this_`); keep protection's
Equipment/Fortification clause on permanents [CR#702.16d] and its blocking clause on creatures
[CR#702.16f]. Mirrors: `hexproof.ron:14`, `shroud.ron:12`, `protection.ron`. Pin a player grant.

## 8. Detain and goad last until the ability controller's next turn

Detain (`Definitions/Actions.lean:641`) and goad (`:680`) end at `untilNextTurnOf (controllerOf
this_)`, the source's controller. [CR#701.35a,701.15a]: "Until the next turn of the controller of
that spell or ability"; an ability's controller is fixed by [CR#113.8], and the two differ when the
source changes control before resolution (*New Prahv Guildmage*). Use `untilYourNextTurn`
(`Templates.lean:827-828`). Check goad's "player other than the controller of the permanent, spell,
or ability that caused it to be goaded" [CR#701.15b] for the same mix-up in the goaded
Designation's semantics.

## 9. Convert matches "transforms" patterns

`convert` (`Definitions/Actions.lean:625-627`) enacts its own deed `"convert"`, and
`convert.ron:1-4` says the deed label "is what distinguishes the two events [CR#701.27e]". The rule
says the opposite: a "transforms into" ability "triggers if the object either transforms or
converts" [CR#701.27e], the transform rules apply to converting [CR#701.28a], and "can't transform"
blocks converting [CR#701.28f]. `crates/deckmaste_core/src/action.rs:142-143` and
`docs/keyword-policy.md:238-240,355` already fold convert into Transform. Follow them: one
Transform Deed with a name tag, or make every transform-keyed trigger (`Proofs/Anaphora.lean:676`)
and prohibition match convert. Correct the RON comment. *Cult of the Waxing Moon*'s missing
complement is in the fidelity ticket and lands on top of this.

## 10. "After you roll a die" is a roll modifier, not a trigger

`after` is `.triggered` (`Templates.lean:1815-1820`), used by `xenosquirrelsShift`
(`Cards/Description.lean:493-496`) and pinned by `okAfterRollShift` (`Proofs/Trigger.lean:254-259`).
That followed `docs/tickets/done/workbench-witness-sweep-residues.md:222-228` ("`Events.TriggerWord`
gains `After`"). Triggered abilities use "when", "whenever" or "at" [CR#113.3c]; the roll's
result is "the final number … After considering all applicable modifiers" [CR#706.2], and
"Modifiers may be optional and/or have associated costs" [CR#706.2a]. A trigger waits for priority, so it
resolves after the roll's instruction has already used the result and can never shift it (*Night Shift of the Living Dead* ruling,
2022-10-07: "you do get to see all the results before deciding whether to use it").

Change: remove `after → triggered`. Model these as a static roll-modifier ability, optional and
possibly costed, applied before the result is final and ordered under [CR#706.2b]; a printed
"only once each turn" is a Usage Limit. Flip `okAfterRollShift`. The done ticket
`lean-v3-trigger-words.md:70` lists Xenosquirrels as a printed-word trigger site; record here that
this supersedes it. The engine pipeline is [engine-replace-roll](engine-replace-roll.md).

## Verification

`./lean/scripts/build`; `cargo xtask lean check`; `cargo test -p xtask -p deckmaste_semantics_v2`
and clippy for both; the CR citation gates.
