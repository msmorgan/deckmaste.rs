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
ticket is that ticket. **The design below is proposed, not ruled.**

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

## Design (proposed, not ruled)

- **Decide first whether `retention` is the construct.** If it is, give it a doc comment naming
  [CR#702.16n,702.16p,303.4c,704.5m], a checker law, and the template its role. If not, replace it
  and re-spell the six cards on the replacement in the same landing.
- **The carve-out is an exemption limited to one State-Based Cause.** The project already has this
  shape: *Lich*'s "you don't lose the game for having 0 or less life" is a `loseGame` prohibition
  with a state-based cause (`lean-zero-life-exemption-is-a-deontic`,
  [CR#704.5a]; glossary: **State-Based Cause**). [CR#702.16n] says the same of Auras: the exemption
  covers this effect's protection and leaves any other instance of protection to act as normal.
  The proposal is to model the carve-out as an exemption keyed to the effect that would remove the
  Attachment, not to the Attachment alone.
- **What "this effect" ranges over is a question to settle, not assume.** In *Guardian Beast* the
  exemption concerns only the "can't be enchanted" conjunct, not "indestructible" or "can't gain
  control". Decide whether the construct wraps the whole conjunction (as `retention` does today),
  or attaches to the conjunct it limits.
- **"Already attached" is a point in time, not a filter.** [CR#702.16p] reads the Attachments when
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

1. Settle the design above, with the owner where a ruling is needed (the STOP is on "what the
   exemption ranges over" and "retire or keep `retention`").
2. Implement the construct in `Abilities.lean`, `Templates.lean`, `Check/` and any plugin
   definition it touches (`cargo xtask lean check` if plugin definitions change).
3. Spell *Guardian Beast* in full and replace `guardianBeastProtection` with the card.
4. Spell the nine unspelled cycle cards, and re-spell the six existing ones if `retention` is
   replaced.
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
  indestructible" or "can't gain control of" (rewritten if the design wraps the whole
  conjunction, with the reason for the change named).
- A reading pin that the exemption's referent ("those artifacts", "this Aura", "it") resolves to
  the effect's own subject and host, not to the nearest unrelated mention.
