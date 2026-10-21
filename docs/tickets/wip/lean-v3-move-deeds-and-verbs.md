---
needs: []
---
# Zone moves carry no "put"/"return" Deed; the printed verb lives on the Template

Standard constraints apply. This is finding 2 of the spelling-layer audit ("one phrase, two
encodings") and finding B3 of the warts audit. It takes over
[lean-v3-rules-and-card-fidelity](lean-v3-rules-and-card-fidelity.md) §3 "`Return` vs `put`".

## Governing principle (owner, 2026-10-21)

- Printed wording lives on Templates, which carry their spellings as text templates in the DSL
  this workbench prototypes. It does not live in Core AST nodes (the semantic expression types in
  `Words.lean`, `Events.lean` and `Abilities.lean`) or in Deed stamps.
- A distinction belongs in the Core AST only if the CR or the checker's anaphora resolution
  distinguishes it.
- Separate Templates keep distinct printed wordings even when they expand to the same tree.

## Ruling (owner, 2026-10-21)

- Remove `CoreDeed.put` and `CoreDeed.return_` (`Words.lean:247`) and the
  `.enact (.core .put)` / `.enact (.core .return_)` wrappers in the move Templates. "Put" and
  "return" are ordinary English for a Zone Change [CR#400.7]; no rule defines either as a Deed.
- A Participial Description over a move ("the returned card", "the creature returned this way")
  reads the `.move` itself, not a Deed.
- Keep the separate put and return Templates for their spellings.
- Align each card to the Template of the verb it prints.

## Verified state (2026-10-21)

- **Wrappers.** `put` and `putWith` (`Templates.lean:1110-1116`) wrap `.enact (.core .put)`.
  `returnTo` (`:1118-1121`), `returnToBattlefield` (`:1123-1126`) and `returnToHand`
  (`:1514-1516`) wrap `.enact (.core .return_)`. `returnToBattlefieldTransformed` (`:1128`)
  goes through `returnTo`. `putInto` (`:1043-1052`), `returnToBattlefieldWithCounters`
  (`:1131-1134`) and the four `putOntoBattlefield*` (`:1135-1146`) wrap nothing.
- **Guessing.** `Action.inferredDeed` (`Check/Phrase.lean:179-200`) stamps a bare move to a
  graveyard `.core .put` and a bare move to a hand `.core .return_` (`:187-188`). A move to the
  battlefield gets no stamp at all, because `mkStamp none` is `none` (`Check/Words.lean:836-838`).
  That is why "the creature returned this way" is refused after `putOntoBattlefield` and accepted
  after `returnToBattlefield`. `DidClause.anchorDeed?` (`Check/Phrase.lean:222-228`) reads the
  anchor through `inferredDeed`.
- **Observation Point.** `putInto` computes it from the written origin [CR#603.6c,603.10a].
  `put`, `putWith`, `returnTo`, `returnToBattlefield` and `returnToHand` hard-code `.arrival`, so
  "return this card from your graveyard to your hand" (`dredge` `Definitions/Keywords.lean:534`,
  `recover` `:1216`) is observed on arrival, while the same move through `putInto` is observed on
  departure. With the Deeds gone, this is the only remaining difference between a put tree and a
  return tree. The verb does not decide it; the written origin does.
- **Proof sites that name the Deeds:** `putHasNoStandaloneParticiple`
  (`Proofs/Anaphora.lean:599`); the stamps passed to `applyZoneMove` in
  `Proofs/ControllerSacrifice.lean:352,378,565,582,586,594,638,656,658,856`; the hand-built
  stamp in `Proofs/RefusalReachability.lean:94`.
- The Rust `CoreDeed` in `crates/xtask/src/expansions.rs:535` is the v2 mirror
  (`deckmaste_semantics_v2`), which is deletion-bound. It is unaffected.

## Change

1. Delete `CoreDeed.put` and `CoreDeed.return_`. Every move Template expands to a bare `.move`.
2. Build every move Template on one internal helper that computes the Observation Point from the
   written origin, as `putInto` does today. A put Template and a return Template with the same
   subject, origin, destination and riders then expand to the same tree.
3. Stamp a moved referent with its move even when no Deed names the move. Today `applyZoneMove`
   drops the stamp when the Deed is `none`. The stamp shape is the implementer's choice: an
   optional Deed, or a separate movement record. A Participial Description whose relative clause
   is a `.move` anchors on that movement: the destination it writes, and the origin if one is
   written. "The creature returned to the battlefield this way" and "the creature put onto the
   battlefield this way" then read the same referent. ("Card" would not do here: it reads the zone
   the object is in now, and the battlefield is no card zone; a Card Type reads the permanent the
   object is on the battlefield [CR#608.2h].) Delete `inferredDeed`'s graveyard and hand cases. Its
   exile case stays: exile is a Keyword Action [CR#701.13a].
4. Rewrite the card sites below onto the Template of their printed verb.
5. Update `lean/CONTRACTS.md` ("Movement destinations" and any statement of `put`/`return_`
   stamps). Also update the Deed and Core Deed lists in `docs/decisions/semantics-v3.md` if they
   name either Deed.

[lean-v3-checker-and-drt-fixes](lean-v3-checker-and-drt-fixes.md) §2 ("Unify deed stamping")
also rewrites `inferredDeed` and the Stamp. Neither ticket needs the other first. Whichever lands
second rebases. If §2 lands first, step 3 builds on its unified stamp.

## Sites

Line numbers verified 2026-10-21 against Scryfall Oracle text. The ruling's estimate of "about 53
`putInto … hand` sites" does not hold. Only 13 hand sites print "return"; the other 24
`putInto … hand` sites print "put".

**Printed "return … to … hand", written `putInto … hand`** (13). Rewrite with `returnToHand`:
- *Unsummon* `Cards/Anaphora.lean:610`
- *Skyblade's Boon* `Cards/Cost.lean:743`
- *Academy Journeymage* `Cards/Cost.lean:932`
- *Blood Spatter Analysis* `Cards/Counters.lean:1073`
- *Pulse of the Forge* `Cards/Damage.lean:1035`
- *The Last Ronin* `Cards/Damage.lean:1339`
- *Rogues' Gallery* `Cards/Description.lean:352`
- *Wind Zendikon* `Cards/Keyword.lean:345`
- *Repeal* `Cards/Mana.lean:559`
- *Gush* `Cards/Static.lean:569`
- *Thwart* `Cards/Static.lean:611`
- *Hammer of Bogardan* `Cards/Turn.lean:77`
- *Thawing Glaciers* `Cards/Turn.lean:491`

**Printed "return … to the battlefield", written with a put Template** (17). Rewrite with
`returnToBattlefield` or `returnTo … battlefield`, keeping the riders:
- *Cloudshift* `Cards/Anaphora.lean:23`
- *Flickering Spirit* `:37`
- *Ever After* `:175`
- *Another Round* `:191`
- *Continue?* `Cards/Choice.lean:38`
- *Synod Sanctum* `Cards/Cost.lean:76`
- *Cold Storage* `:85`
- *Missy* `Cards/Faces.lean:57`
- *Yedora, Grave Gardener* `:74`
- *Storm of Souls* `Cards/Keyword.lean:858`
- *Death or Glory* `Cards/Piles.lean:169`
- *Eerie Ultimatum* `Cards/Static.lean:870`
- *Graceful Reprieve* `Cards/Trigger.lean:19`
- *Dread Slaver* `:458`
- *Faith's Reward* `:599`
- *Turn to Mist* `Cards/Turn.lean:33`
- *Voyager Staff* `:40`

**Already on a return Template** (no rewrite; their trees lose the wrapper, so their pins are
rechecked):
- Cards: *Bond of Revival* `Cards/Anaphora.lean:43`, *Open the Vaults* `:559`, *Churning Eddy*
  `:649`, *Consuming Tide* `Cards/Choice.lean:1440`, *Obelisk of Undoing* `Cards/Cost.lean:817`,
  *Flickering Ward* `:820`, *Daydream* `Cards/Counters.lean:56`, *Infernal Vessel* `:835`,
  *Mistbreath Elder* `:1165,1167`, *Harvest Hand* `Cards/Faces.lean:350`, *Midnight Scavengers*
  `:403`, *Custodi Squire* `Cards/Piles.lean:130`, *Scythe of the Wretched*
  `Cards/Trigger.lean:967`.
- Definitions: champion `Definitions/Keywords.lean:368`, dash `:442`, dredge `:534`, ninjutsu
  `:1043`, persist `:1090`, recover `:1216`, sneak `:1292`, soulshift `:1324`, undying `:1498`,
  unearth `:1504`, web-slinging `:1557`, and earthbend `Definitions/Actions.lean:662`.

**Overlaps:**
- *Pulse of the Forge* `Cards/Damage.lean:1033` is also rewritten by
  [lean-v3-spell-damage-dealer](lean-v3-spell-damage-dealer.md).
- *Custodi Squire*'s trigger (`Cards/Piles.lean:124`) is rewritten by
  [lean-v3-trigger-words](lean-v3-trigger-words.md).

Whichever ticket lands second rebases.

## Tests

- **Retire** `putHasNoStandaloneParticiple` (`Proofs/Anaphora.lean:599`): the Deed it pins no
  longer exists.
- **Rewrite** the `ControllerSacrifice` and `RefusalReachability` stamps listed above onto the
  new move stamp. Keep each assertion's claim.
- **Add** a pin that a return Template and a put Template to the same destination expand to the
  same tree. For example, `returnToHand s` and `putInto s hand`, and `returnToBattlefield s` and
  `putOntoBattlefield s`. This is the tree identity the governing principle accepts.
- **Add** a pin that a move from a graveyard to a hand is observed on departure through both
  `returnToHand` (with a written origin) and `putInto` [CR#603.10a].
- **Add** a pin that, after *Cloudshift*'s exile and return, "the creature returned this way" and
  "the creature put onto the battlefield this way" both resolve to the returned card ("card"
  reads the zone the object is in now, which here is no card zone). Today the first
  is refused after a put Template.
- The `ok…` theorems for every rewritten card stay green under `./lean/scripts/build`
  (`--wfail`).
