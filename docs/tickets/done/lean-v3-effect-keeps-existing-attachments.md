---
needs: []
---
# "This effect doesn't remove …": an effect that keeps the Attachments already on its host

Standard constraints apply. Provenance: the STOP on *Guardian Beast* in
[lean-v3-ast-merges-by-rules-meaning](../done/lean-v3-ast-merges-by-rules-meaning.md) §5 ("Sites").
Lean first (`lean/Semantics/`); the Rust mirror follows the usual cutover and is not this ticket.

## Ruling (owner, 2026-10-22)

A dedicated ticket for a construct that limits what a protection or enchantment-prohibition effect
does to Attachments already attached, then *Guardian Beast* and the cycle written in full. This
ticket is that ticket. Questions 3a and 3b are ruled (batch 4, below); the remaining design bullets
are proposals.

## Ruling on "already attached" (owner, 2026-10-22; the audit-ruling batch)

Question 3c, "already attached", is ruled **no change**. [CR#702.16n] ("this effect doesn't
remove" this Aura or all Auras) and [CR#702.16p] (*Benevolent Blessing*'s "already attached to")
are already told apart by the absence or presence of `attachedTo` on `retention`'s subject. The
ruling named *Spectra Ward* against *Benevolent Blessing* and *Guardian Beast*. Questions 3a
(keep `retention` or replace it) and 3b (scope the exemption to the conjunct it limits) were open then and are ruled in batch 4 below:
the other design bullets below remain proposals.

Verified against the bench (2026-10-22), with corrections:

- `StaticSpec.retention (spec) (subject)` is `lean/Semantics/Abilities.lean:921` (Template
  `Templates.lean:2056-2057`, checker `Check/Phrase.lean:3669-3672`).
- *Benevolent Blessing* (`Cards/Keyword.lean:1331-1343`) has subject `allOf (and [or [aura,
  equipment], youControl, attachedTo it])`: `attachedTo` present. The spelled [CR#702.16n] cards
  (*Black Ward*, *Cho-Manno's Blessing*, *Pentarch Ward*, `Cards/Keyword.lean:1300,1313,1326`;
  *Tattoo Ward*, *Floating Shield*, `Cards/Cost.lean:813,833`) all have subject `thisAura`, no
  `attachedTo`. So the distinction holds on the bench.
- *Spectra Ward* and *Guardian Beast*, the ruling's witnesses, are not spelled (see the table
  above), so neither side of the contrast is pinned through them yet.
- *Guardian Beast*'s carve-out is not [CR#702.16p], which is about protection only. Its Auras are
  removed because "can't be enchanted" makes their host illegal [CR#303.4c,704.5m]; its "already
  attached" reads like *Benevolent Blessing*'s but rests on those rules.

## Ruling (owner, 2026-10-22, second batch): 3c acknowledged

The point-in-time note is **owner-acknowledged**, for the open 3a/3b design. [CR#702.16p]
snapshots the Attachments when the protection effect starts to apply, while `attachedTo` reads
current state; the two differ when an Equipment attached without the quality later gains it (by
[CR#702.16p] it falls off, under the filter it stays). "No change" settles the encoding of the
term; the onset-time reading is an engine note, not a term change, and the bullet "'Already
attached' is a point in time" below is read that way. The test
`alreadyAttachedIsReadWhenTheEffectStarts` below is therefore no longer waiting on the owner; it
is written with the rest of the 3a/3b design. *Guardian Beast* rests on [CR#303.4c,704.5m] (an
Aura on an artifact that can't be enchanted is illegal), not on [CR#702.16p], which is about
protection only.

## Ruling (owner, 2026-10-22, batch 4): 3a yes, 3b "the part it limits"

- **3a: yes, keep `StaticSpec.retention`.** Document it as an exemption from one state-based
  consequence of the effect [CR#702.16n,702.16p,303.4c,704.5m], with a checker law and a template
  that says so.
- **3b: "the part it limits".** Apply `retention` to the specific clause it limits (*Guardian
  Beast*'s "can't be enchanted"), not to the whole conjunction. So the exemption does not reach
  "have indestructible" or "can't gain control of".
- 3c was already ruled (no change; see the two sections above).

Re-verified code locations (2026-10-22, this workspace; the earlier citations in this ticket have
drifted):

- `StaticSpec.retention (spec) (subject)`: `lean/Semantics/Abilities.lean:903`, no doc comment
  (the comment above it belongs to `characteristicDefining`); Template `Templates.lean:2136-2137`;
  checker arm `Check/Phrase.lean:3948-3951` (steps the spec, then the subject, enforces no law);
  other readers `Check/Phrase.lean:1546` (`writesDefining`), `Check/CardFacts.lean:101`
  (`namesThisDoor`), `Check/NumberSlots.lean:126`.
- `guardianBeastProtection` / `okGuardianBeastProtection`: `Cards/Deontic.lean:768,776`. Its
  conjunction is `conjunction [cant any (attachTo (allOf aura) them), abilityGrant them
  indestructible, cant … (gainControlOf them)] (bare (and [non isCreature, isArtifact,
  youControl]))`, under `onlyWhile (is thisCreature isUntapped)`.

What 3b means in the AST: the conjunction keeps its three parts; `retention` wraps only the
`cant any (attachTo (allOf aura) them)` part, with the subject "Auras already attached to those
artifacts" (an `attachedTo` filter on the host, as *Benevolent Blessing*). Where `retention`'s
spec is a conjunction member, the checker law below applies to that member.

## Problem

Some effects would, by their own terms, make an Attachment that is already attached illegal, and
the rules then remove it with a state-based action. The printed card says that this effect does not.

- A protection effect makes Auras with the quality, and Equipment and Fortifications with it,
  illegal on their host. Each is put into its owner's graveyard (Aura) or becomes unattached
  (Equipment, Fortification) as a state-based action [CR#702.16c,702.16d,704.5m,704.5n].
- An effect that says a permanent "can't be enchanted" makes an Aura already on it enchant an
  illegal object, and the same state-based action applies, because an Aura's legal hosts are fixed
  by its enchant ability "and other applicable effects" [CR#303.4c,704.5m].
- The cards add a carve-out, which the Comprehensive Rules recognise for two shapes:
  - "This effect doesn't remove this Aura" or "… Auras": the named Auras are not put into the
    graveyard as a state-based action. Other instances of protection from the same quality still
    affect them as normal [CR#702.16n].
  - "… already attached to it" (*Benevolent Blessing*): when the protection effect starts to apply,
    the permanents with the quality that are then attached, the protecting Aura included, are not
    put into graveyards. Others cannot become attached afterwards [CR#702.16p].

Today:

- *Guardian Beast* ("As long as this creature is untapped, noncreature artifacts you control can't
  be enchanted, they have indestructible, and other players can't gain control of them. This effect
  doesn't remove Auras already attached to those artifacts.") is pinned for its first sentence only,
  as `okGuardianBeastProtection` in `lean/Semantics/Cards/Deontic.lean`. The card is not spelled.
- The cycle has a form, which the earlier ticket's note ("nor does the Ward cycle's …") missed:
  `StaticSpec.retention (spec) (subject)` (`Abilities.lean`, template `retention`). Six cards use
  it: *Black Ward* and *Cho-Manno's Blessing*, *Pentarch Ward*, *Benevolent Blessing*
  (`Cards/Keyword.lean`), *Floating Shield* and *Tattoo Ward* (`Cards/Cost.lean`). But the
  constructor has no doc comment (the comment above it belongs to `characteristicDefining`), the
  checker arm (`Check/Phrase.lean`, `| .retention spec subj`) only steps the spec and then the
  subject and enforces no law, and nothing says what rule it models. Whether it is the right
  construct is part of this ticket.

The Vintage-legal printings carrying the sentence (from `data/derived/cards.jsonl`, legality
checked against the MTGJSON snapshot), 15 names:

| Spelling | Cards |
|---|---|
| "This effect doesn't remove this Aura." | White Ward, Blue Ward, Black Ward, Red Ward, Green Ward, Cho-Manno's Blessing, Flickering Ward, Floating Shield, Pentarch Ward, Pledge of Loyalty, Tattoo Ward, Ward of Lights |
| "This effect doesn't remove Auras." | Spectra Ward |
| "This effect doesn't remove Auras and Equipment you control that are already attached to it." | Benevolent Blessing |
| "This effect doesn't remove Auras already attached to those artifacts." (an enchant prohibition, not protection) | Guardian Beast |

Already spelled: Black Ward, Cho-Manno's Blessing, Pentarch Ward, Benevolent Blessing, Floating
Shield, Tattoo Ward. To spell: White Ward, Blue Ward, Green Ward, Red Ward, Spectra Ward, Pledge of
Loyalty, Ward of Lights, Flickering Ward, Guardian Beast. Each of the last four may need more than
this construct ("each color", "each color among permanents you control", a flash-conditional
sacrifice, a return-to-hand activation); a card that is blocked by something else is ticketed for
that, not spelled around.

## Design (3a and 3b ruled in batch 4; the rest proposed)

- **`retention` is the construct (3a, ruled).** Give it a doc comment naming
  [CR#702.16n,702.16p,303.4c,704.5m] that says it is an exemption from one state-based consequence
  of the effect, a checker law (below), and the template its role. Nothing is replaced, so the
  six existing cards are not re-spelled.
- **The carve-out is an exemption limited to one State-Based Cause.** The project already has this
  shape: *Lich*'s "you don't lose the game for having 0 or less life" is a `loseGame` prohibition
  with a state-based cause (`lean-zero-life-exemption-is-a-deontic`,
  [CR#704.5a]; glossary: **State-Based Cause**). [CR#702.16n] says the same of Auras: the exemption
  covers this effect's protection and leaves any other instance of protection to act as normal.
  The proposal is to model the carve-out as an exemption keyed to the effect that would remove the
  Attachment, not to the Attachment alone.
- **What "this effect" ranges over (3b, ruled): the clause it limits.** In *Guardian Beast* the
  exemption concerns only the "can't be enchanted" conjunct, not "indestructible" or "can't gain
  control". `retention` attaches to that conjunct, not to the whole conjunction.
- **(Ruled no change for the term, 2026-10-22; see the second-batch ruling above.)** **"Already attached" is a point in time, not a filter** (owner-acknowledged, second batch). [CR#702.16p] reads the Attachments when
  the effect starts to apply. `attachedTo it` as written in *Benevolent Blessing* reads the current
  state. The design must say which it models and pin the difference with an Aura attached after the
  effect began.
- **Reference scope.** The exempted Attachments are the host's own (the "enchanted" and the
  "those artifacts" subjects of the effect). Vocabulary is **Attachment** and **Attachment Host**
  (`docs/contexts/game-model/CONTEXT.md`); do not introduce a second term. Check
  `docs/decisions/semantics-v3.md` §1.3 box 3 (a prohibition box closes what it introduces, but
  definite attachment-host references project to the enclosing ability) before adding any DRS
  rule.

## Actionable change

1. Nothing is left for the owner: 3a, 3b and 3c are ruled. (The remaining design bullets are the
   implementer's to settle; STOP only if one contradicts a ruling.)
2. Keep `StaticSpec.retention`. Document it (doc comment on the constructor, Template doc comment)
   as an exemption from one state-based consequence of the effect
   [CR#702.16n,702.16p,303.4c,704.5m]. Add the checker law in the `Check/Phrase.lean:3948` arm:
   the spec must be a clause that would remove an Attachment (a protection grant, or a
   "can't be enchanted" / "can't be equipped" prohibition), and the subject must be an Attachment
   of the effect's host. Name the reason codes.
3. Spell *Guardian Beast* in full with `retention` on the "can't be enchanted" clause only, and
   replace `guardianBeastProtection` with the card.
4. Spell the nine unspelled cycle cards. The six existing ones keep their spelling.
5. `docs/decisions/semantics-v3.md`: record the construct where static specs are listed, with the
   rules above.

## Tests to add

Each asserts the intended analysis, not a count (CLAUDE.md "Assurance").

- `okGuardianBeast`: the whole card, all three conjuncts and the exemption, checks clean. Retires
  `okGuardianBeastProtection`, which pinned a fragment; say so in the landing report.
- `okWhiteWard`, `okBlueWard`, `okGreenWard`, `okRedWard`, `okSpectraWard`, `okPledgeOfLoyalty`,
  `okWardOfLights`, `okFlickeringWard`: each card checks clean (subject to what else each needs).
- The six cards already spelled keep their bench checks, unchanged in claim.
- A refusal pin for each law the construct adds: an exemption on a spec that removes no
  Attachment is refused; an exemption naming a subject that is not an Attachment of the effect's
  host is refused. Name the reason code in each pin.
- `exemptionDoesNotCoverOtherProtection` ([CR#702.16n]): an Aura that is exempt from this effect
  is still removed by a second protection effect of the same quality. If the checker cannot see
  this (it is an engine fact), the pin is on the construct's shape, and the engine consequence is
  noted in the Rust mirror ticket, not deferred silently.
- `alreadyAttachedIsReadWhenTheEffectStarts` ([CR#702.16p]): the *Benevolent Blessing* shape
  distinguishes an Attachment present at the start from one attached later.
- `guardianBeastExemptsOnlyTheEnchantProhibition`: the exemption does not reach "have
  indestructible" or "can't gain control of" (3b, ruled): `retention` is on the
  `cant any (attachTo …)` member, and a variant with `retention` wrapping the whole conjunction
  is refused by the law.
- `retentionLawRefusals`: a `retention` whose spec removes no Attachment is refused, and one whose
  subject is no Attachment of the effect's host is refused (the two refusal pins above, named
  here for the 3a law).
- A reading pin that the exemption's referent ("those artifacts", "this Aura", "it") resolves to
  the effect's own subject and host, not to the nearest unrelated mention.
