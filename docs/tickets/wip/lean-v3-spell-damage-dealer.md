---
needs: []
---
# "<this> deals N damage" is one Handoff to the source, on spells as on permanents

Standard constraints apply. This covers finding D1 of the warts audit and finding 7 of the
spelling-layer audit ("one phrase, two encodings"). It takes over the bullet "Standardize
damage-source performer handoffs across `Cards/*.lean`" from
[lean-v3-rules-and-card-fidelity](lean-v3-rules-and-card-fidelity.md) §3.

## Governing principle (owner, 2026-10-21)

- Printed wording lives on Templates, which carry their spellings as text templates in the DSL
  this workbench prototypes. It does not live in Core AST nodes (the semantic expression types in
  `Words.lean`, `Events.lean` and `Abilities.lean`) or in Deed stamps.
- A distinction belongs in the Core AST only if the CR or the checker's anaphora resolution
  distinguishes it.
- Separate Templates keep distinct printed wordings even when they expand to the same tree.

## Rulings (owner, 2026-10-21)

1. A spell that prints "<name> deals N damage" is written `instruct this_ (dealDamage …)`, as a
   permanent is. An Object that deals damage is the Source of that damage [CR#120.1]. A bare
   `dealDamage` outside a Handoff has the controller, a Player, as its Actor.
2. `dealDamageOwnPower` and the pronoun form ("<X> deals damage equal to its power to <Y>") get
   one tree. The rules meaning decides the tree, and the Template keeps the wording.
3. Align *Karplusan Yeti*, which uses both forms.
4. (2026-10-21, on this ticket's STOP) Fix `D′` here. In a bare pronoun's ranking, a referent
   re-mentioned in a direct argument slot of the current clause (in `drs.pendingRoles`) ranks at
   `currentClause`, by the best of its buffered slot roles. Before the fix, "That creature deals
   damage equal to its power to this creature" read "its" as the earlier clause's subject,
   *Karplusan Yeti*. ADR §1.2 and §1.5 are amended to match.

## Design (ruling 2)

- "Its power" is an ordinary possessive pronoun. D′ resolves it to the clause subject, because a
  possessor is not a clause-mate and the subject ranks first. A re-mentioned subject ("that
  creature", "it") ranks first too, once ruling 4 is applied.
- The tree is the pronoun form, `instruct X (dealDamage (powerOf it) Y)`. The Binding Identity
  `dealDamageOwnPower.source` cannot be reproduced from printed text, and the pronoun reading
  already checks (`possessiveItReadsCurrentSubject`, `Proofs/Anaphora.lean:706-713`).
- `dealDamageOwnPower` (`Templates.lean:1189-1195`) stays as the Template for the wording, and
  expands to that tree.
- Every card that prints "<X> deals damage equal to its power to <Y>" writes the Template.
- Drop `dealDamageOwnPower` from the multi-mention Template examples in
  `docs/decisions/semantics-v3.md` §2.2, since it no longer mentions its parameter twice.

## Verified state (2026-10-21)

- *Radiant Flames* (`Cards/Mana.lean:743`) is a `spelled` sorcery already written
  `spell (instruct this_ (dealDamage …))`. The spell form checks today, and no checker change is
  needed for ruling 1.
- Damage with no Handoff on a spell occurs only in `Cards/Damage.lean`. Other files' bare-looking
  `dealDamage` calls are event patterns (*Oona's Blackguard*, *Aya of Alexandria*, *Dread*).

## Sites

Line numbers verified 2026-10-21 against Scryfall Oracle text.

**Spells printing "<name> deals …", written without a Handoff** (35 cards, 47 calls, all in
`Cards/Damage.lean`):
- *Lightning Bolt* `:20`
- *Barrage of Boulders* `:22`
- *Arc Trail* `:35` (two calls)
- *Sudden Demise* `:63`
- *Arrows of Justice* `:72`
- *Flames of the Raze-Boar* `:76,78`
- *Arc Blade* `:88`
- *Abrade* `:91`
- *Bot Bashing Time* `:107`
- *Thunderwave* `:124,128,130`
- *Chandra's Revolution* `:168`
- *Sizzling Barrage* `:178`
- *Chain Reaction* `:200`
- *Harsh Sustenance* `:218`
- *Psychic Purge* `:277`
- *Lava Axe* `:539`
- *Searing Flesh* `:545`
- *Incinerate* `:846`
- *Galvanic Blast* `:867,869` (the `instead` replacement too)
- *Furious Reprisal* `:873`
- *Lavalanche* `:927-928`
- *Flame Wave* `:933-934`
- *Bonfire of the Damned* `:951-952`
- *Chandra's Fury* `:957-958`
- *Blightning* `:1014`
- *Rakdos's Return* `:1020`
- *Pulse of the Forge* `:1033`
- *Searing Blaze* `:1059-1060,1067-1068`
- *Burn at the Stake* `:1095`
- *Explosive Singularity* `:1106`
- *Fall of the Titans* `:1127`
- *Tribal Flames* `:1136`
- *Destructive Revelry* `:1197`
- *Crackling Doom* `:1268`
- *Fry* `:1375`

Where one sentence names the source once for several recipients ("Lavalanche deals X damage to
target player or planeswalker and each creature …", *Arc Trail*, *Searing Blaze*), wrap the whole
`simultaneously`/`sequentially` in one `instruct this_`, so the source is written once. *Angrath,
Minotaur Pirate*'s +1 (`:961-964`) already hands each call to `this_` separately; align it to the
same shape.

**Permanents printing "It deals …", written without a Handoff:**
- *Immersturm Skullcairn* `Cards/Damage.lean:47`
- *Pyrite Spellbomb* `:53`

These become `instruct this_ (dealDamage …)`. How a printed "it" for the source is spelled is
finding 8 of the spelling-layer audit, which is not ruled.

**"<X> deals damage equal to its power to <Y>"**, rewritten onto `dealDamageOwnPower`:
- *Rabid Bite* `Cards/Damage.lean:25` (the pronoun form)
- *Deadshot* `:40`
- *Karplusan Yeti* `:57-58` (both forms today; "That creature deals damage equal to its power to
  this creature" is the second)
- *Nibelheim Aflame* `:96`
- *Garruk Relentless* `Cards/Faces.lean:538-539` (written `powerOf (that creature)`)
- the emblem of *Arlinn, Embraced by the Moon* `Cards/Faces.lean:324` (written
  `powerOf thisCreature`)
- *Aggressive Instinct* `Cards/Anaphora.lean:664` already writes the Template, and its tree
  changes with the expansion.

**Overlaps.** Whichever ticket lands second rebases.
- [lean-v3-another-contrast-anchor](lean-v3-another-contrast-anchor.md) rewrites the "other"
  predicate on the same lines: *Arc Trail* `:35`, *Flames of the Raze-Boar* `:78`, *Deadshot*
  `:40`, *Nibelheim Aflame* `:96`.
- [lean-v3-rules-and-card-fidelity](lean-v3-rules-and-card-fidelity.md) §3 adds the discard
  Handoff on *Immersturm Skullcairn* `:47`.
- [lean-v3-move-deeds-and-verbs](lean-v3-move-deeds-and-verbs.md) rewrites *Pulse of the
  Forge*'s return (`:1035`).
- [lean-v3-trigger-words](lean-v3-trigger-words.md) rewrites *Arlinn Kord* (`Cards/Faces.lean`).

*Which of You Burns Brightest?* (`Cards/Damage.lean:966-969`) is a Scheme card, which is out of
scope. This ticket does not touch it.

Record the convention in `lean/CONTRACTS.md` ("The performer and the handoff"): "<this> deals"
on any card is a Handoff to the source.

## Tests

- **Rewrite** `badOwnTwoInDelta` (`Proofs/Anaphora.lean:761-765`). It pins `[.singular]`, a
  refusal produced by the Binding-Identity expansion. Assert instead what is wanted: a plural
  source ("both target creatures … its power") is refused because the singular pronoun cannot
  resolve. Assert membership of the anaphor refusal, not the exact list.
- **Keep** `okOwnPowerWithoutNewMention`, `ownSurvivesSecondSingular` and
  `ownPowerCanReadAnExistingReference` (`Proofs/ActionFamilies.lean:230`). They must stay green
  on the new expansion.
- **Add** a pin that `dealDamageOwnPower X Y` and `instruct X (dealDamage (powerOf it) Y)` expand
  to the same tree.
- **Add** a pin that, in *Karplusan Yeti*'s second sentence, "its" resolves to "that creature",
  not to this creature.
- **Add** a pin that *Lightning Bolt*'s source referent is the Actor of its `dealDamage`.
- The `ok…` theorems for every rewritten card stay green under `./lean/scripts/build`
  (`--wfail`).

## Not ruled

Whether the checker should refuse a `dealDamage` whose Actor is a Player is open. Only Objects
deal damage [CR#120.1], but such a refusal would be a new game-rule refusal, and CONTRACTS.md
freezes those without a dated ruling. This ticket does not add it.
