---
needs: []
---
# Retire twin Templates and align each site to the Template of its printed wording

Standard constraints apply. Found by the 2026-10-25 triage of an external audit ("Atraxa"),
verified against the tree at `wmyykqnv`. Paths are under `lean/Semantics/`; line numbers are as
of that change.

## Rulings in force

- **Owner, 2026-10-21** (`docs/tickets/done/lean-v3-move-deeds-and-verbs.md:11-25`): printed
  wording lives on Templates; separate Templates keep distinct printed wordings even when they
  expand to the same tree; keep the separate put and return Templates; align each card to the
  Template of the verb it prints. So `putOntoBattlefield` and `returnToBattlefield` are **not**
  twins and both stay. A twin here is two Templates for one printed wording, or a Template that
  adds nothing over the composition the card prints.
- A Template over a Definition lives in `Definitions/` (`lean/CONTRACTS.md:184-189`; the import
  chain `Templates ← Phrasings ← Actions` forbids moving it). The four token Templates at
  `Definitions/Actions.lean:573-584` are correctly placed; "stranded" was not a finding.
- **Keyword Definitions keep their full bodies** (owner, 2026-10-25). Their sites below are
  respelled, not removed.

## 1. `put` and `putWith` are one wording

`put s d` (`Templates.lean:1145`) is `putWith s d []` (`:1149`); both are "put X <destination>"
with the destination carrying its own preposition ("on the bottom", "third from the top").

- Give `put` a third parameter `(riders : List MoveRider := [])` and expand it to
  `zoneChange subject destination .anyLocation riders`. Delete `putWith`.
- Rewrite its two users: `Proofs/DefinitionsAndTemplates.lean:123` (the last conjunct of
  `putAndReturnExpandToOneTree` becomes `returnTo subject destination riders = put subject
  destination riders`) and `Proofs/OffStack.lean:295` (`riderNegationIsOnlyAPattern`:
  `put (target isCreature) battlefield [.not (.entersAs .tapped)]`).
- The `put` sites `Cards/Choice.lean:1121` (Lost Hours), `:1127` (Aether Gust) and
  `Cards/Anaphora.lean:274` (Deem Inferior) stay as they are.
- `Cards/Cost.lean:680` (Void Maw, "Put a card exiled with this creature into its owner's
  graveyard") prints "into", which is `putInto`'s wording. `lean-v3-card-head-noun-sweep` §1
  row 6 makes that change together with its head noun; skip this line here if that ticket has
  landed, and make only the verb change (`putInto`) if it has not.

## 2. "Return X to the battlefield" is `returnToBattlefield`

`returnTo X battlefield r` and `returnToBattlefield X r` are one wording with one tree; only
`returnToBattlefield` can write a printed origin (`from_`). Rewrite each site:

| Site | Text | Write |
|---|---|---|
| `Cards/Anaphora.lean:658-662` (`openTheVaults`) | Open the Vaults: "Return all artifact and enchantment cards from all graveyards to the battlefield under their owners' control." | `returnToBattlefield (allOf (and [isCard, or [isArtifact, isEnchantment], in_ (graveyardOf (bare isPlayer))])) [under (ownerOf them)] (from_ := some (graveyardOf (bare isPlayer)))`. The printed origin makes the move observed on departure, as for the other graveyard origins pinned by `graveyardOriginsAreDepartureObserved` (`Proofs/CardFidelity.lean:158`). |
| `Cards/Turn.lean:647` (Animate Dead) | "Return enchanted creature card to the battlefield under your control" | `returnToBattlefield (enchanted (.ofType .card .creature)) [under you]` |
| `Cards/Trigger.lean:945-946` (Scythe of the Wretched) | "return that card to the battlefield under your control" | `returnToBattlefield (that card) [(under you)]` |
| `Definitions/Actions.lean:663` (earthbend's delayed return) | "return it to the battlefield tapped under your control" [CR#701.66a] | `returnToBattlefield (that land) [.entersAs .tapped, .under .actor]` |
| `Definitions/Keywords.lean:401-402` (champion) | "return the exiled card to the battlefield under its owner's control" [CR#702.72a] | `returnToBattlefield (the (and [was (exile which), isCard])) [.under (ownerOf it)]` |
| `Definitions/Keywords.lean:1366-1367` (persist) | "return it to the battlefield under its owner's control with a -1/-1 counter on it" [CR#702.79a] | `returnToBattlefieldWithCounters thisPermanent (ownerOf thisPermanent) 1 m1m1Counter` |
| `Definitions/Keywords.lean:1838-1839` (undying) | the same with a +1/+1 counter [CR#702.93a] | `returnToBattlefieldWithCounters thisPermanent (ownerOf thisPermanent) 1 p1p1Counter` |
| `Templates.lean:1162` (body of `returnToBattlefieldTransformed`) | | `returnToBattlefield subject [.entersTransformed, (.under controller)]` |

Proof fixtures written `returnTo … battlefield`, rewritten the same way (keep each claim):
`Proofs/Attachments.lean:178`, `Proofs/LinkedReadback.lean:53`, `:153`, `:475`, `:588`,
`Proofs/Anaphora.lean:582`. `returnTo` keeps its non-battlefield uses
(`Proofs/LinkedReadback.lean:414,489`, `Proofs/Attachments.lean:188,191`,
`Proofs/CorpusRecency.lean:166`); leave them.

Before editing persist and undying, read [CR#702.79a] and [CR#702.93a] in `data/rules/cr.txt`
and confirm the wording; the tree must stay the same (`.withCounters 1 (.printed k) .fresh` is
what the coercion `CounterKind → CounterKindSource`, `Templates.lean:2240`, already produces).

## 3. "Put X onto the battlefield" with riders

No `putOntoBattlefield*` Template takes free riders, so Dance of the Dead
(`Cards/Turn.lean:662-663`, "Put enchanted creature card onto the battlefield tapped under your
control") is written `putInto … battlefield (riders := …)`, and Oracle never prints "into the
battlefield".

- Give `putOntoBattlefield` (`Templates.lean:1169`) a third parameter
  `(riders : List MoveRider := [])`, after `from_`, and pass it to `zoneChange`.
- Write Dance of the Dead as `putOntoBattlefield (enchanted (.ofType .card .creature))
  (riders := [entersAs .tapped, under you])`.
- Proof fixtures whose docstring prints "Return … to the battlefield" but which write
  `putInto … battlefield`: `Proofs/Mana.lean:60` (`okIfDoneReadsDoneBody`) and
  `Proofs/Turn.lean:118`, `:129`, `:140`, `:148`, `:157`, `:165` (`okParticipialDiscardedCard`,
  `deedsAccumulateOnAReferent`, `badParticipialWrongVerb`, `badParticipialWrongNoun`,
  `badParticipialAmbig`, `okBareCardRead`). Write `returnToBattlefield X` at each; where a
  docstring prints "Put … onto the battlefield", write `putOntoBattlefield X` instead. Each
  refusal pin keeps its exact refusal list.
- Do not touch `putInto … battlefield` in fixtures that print no verb
  (`Proofs/Composition.lean:49,57`, `Proofs/Description.lean:378`, `Proofs/Turn.lean:48,110`).

## 4. Templates with no use

Delete each; nothing calls them (`lean_references`, 2026-10-25):

- `move` (`Templates.lean:2166-2169`): no Oracle wording; it shadows the raw primitive.
- `random` (`Templates.lean:218`): literally `countedAtRandom q p`.
- `entersAttachedTo` (`Templates.lean:2254`).

## 5. Token Templates

- **`createTappedAttacking`** (`Definitions/Actions.lean:580-581`) hard-codes `entersAttacking
  none`, so Seraphic Greatsword (`Cards/Keyword.lean:1621-1627`, "create a 4/4 white Angel
  creature token with flying that's tapped and attacking that player") bypasses it. Add a third
  parameter `(defender : Option NounPhrase := none)`, as `putOntoBattlefieldTappedAttacking` has
  (`Templates.lean:1186-1188`), and pass it to `.entersAttacking`. Write Seraphic Greatsword's
  creation as `createTappedAttacking 1 { characteristics := … } (some (that player))` with the
  same bundle. Its other users, `Cards/Keyword.lean:43` (Fire Navy Trebuchet) and
  `Definitions/Keywords.lean:1219`, are unchanged.
- **`createTokenCopy`** (`Definitions/Actions.lean:583-584`) adds nothing over
  `create 1 (copyOf X) riders`, which cards already write for the same wording (Cackling
  Counterpart `Cards/Keyword.lean:436-437`, Hate Mirage `:812`). Delete it and write its three
  sites through the composition, keeping their riders:
  `Definitions/Keywords.lean:685` (encore), `:1264-1267` (myriad) and
  `Definitions/Actions.lean:705` (populate). Encore's `thatToken` readback and myriad's
  `.entersAttacking` rider must still check.
- `createToken` and `createTappedToken` stay ("a tapped Fish" prints "tapped" before the noun).
  Whether `createToken` should give way to a coercion is a separate owner question.

## 6. `exiledWithThisArtifact`

`exiledWithThisArtifact` (`Definitions/Phrasings.lean:100`, `.in_ (.exiledWith thisArtifact)`) is
an alias over "exiled with" + "this artifact"; every other source composes
`in_ (exiledWith thisCreature|thisSaga|this_)`. Delete it and rewrite its sites, restoring the
printed "card" where it is dropped:

| Site | Oracle | Write |
|---|---|---|
| `Cards/Description.lean:389` (Idol of Endurance) | "…cast a creature spell from among cards exiled with this artifact…" | `allOf (cardIn (exiledWith thisArtifact))` in place of `allOf (and [isCard, exiledWithThisArtifact])` |
| `Cards/Cost.lean:74` (Synod Sanctum) | "Return all cards exiled with this artifact to the battlefield under your control." | `allOf (cardIn (exiledWith thisArtifact))` |
| `Cards/Cost.lean:82` (Cold Storage) | "Return each creature card exiled with this artifact to the battlefield under your control." | `each (and [isCreature, cardIn (exiledWith thisArtifact)])` |
| `Proofs/Authoring.lean:347` | the written twin of Cold Storage | the same, in lockstep: `costCoercionsKeepTheCard` (`:350-356`) asserts the two are equal |
| `Cards/Choice.lean:394` (Muse Vessel) | "Choose a card exiled with this artifact." | `a (cardIn (exiledWith thisArtifact))` |
| `Cards/Mana.lean:1012` (Chrome Mox) | "{T}: Add one mana of any of the exiled card's colors." | `ofAnyColorOf (the (and [was (exile which), isCard]))`, the spelling of "the exiled card" at `Cards/Turn.lean:42` and `Cards/Description.lean:348`. The fragment has no imprint ability in scope, so write Chrome Mox as a `spelled` card with both its abilities (imprint: "When this artifact enters, you may exile a nonartifact, nonland card from your hand.") and keep `okChromeMoxMana` as the check of the mana ability inside it. If "the exiled card" does not resolve to the imprint's exiled card, STOP and report. |
| `Cards/Mana.lean:1029` (Ice Cauldron) | "…Spend this mana only to cast the last card exiled with this artifact." | `toCast (cardIn (exiledWith thisArtifact))`. Its missing "last" and "amount" are an owner question, not this ticket's. |

## 7. One-site alignments

- `Definitions/Keywords.lean:683` (encore, "For each opponent, …" [CR#702.141a]):
  `.doForEach (each isOpponent)` → `.doForEach eachOpponent` (`Templates.lean:430`), if the types
  agree; if `eachOpponent` is not a `NounPhrase` of the expected shape, leave the site.
- `Cards/Keyword.lean:359` (`enchantedBecomesVanilla`, used by Frogify `:366`, Kenrith's
  Transformation `:387`, Amphibian Downpour `:395`; "…with base power and toughness 1/1"):
  `getsPt it (setTo (.lit power)) (setTo (.lit toughness))` → `getsBase it (.lit power)
  (.lit toughness)` (`Templates.lean:1397`). Same tree.
- `Definitions/Keywords.lean:1497` (reconfigure's unattach condition, "Activate only if this
  permanent is attached to a creature" [CR#702.151a]):
  `(is thisPermanent (.attachment .attachment none (some (a isCreature))))` →
  `(is thisPermanent (attachedTo (a isCreature)))` (`Templates.lean:39`), if `attachedTo`
  expands to that term; otherwise leave it and say so.
- `theRingTemptsYou` (`Definitions/Actions.lean:727-773`):
  - `:751-752` write `and [isCreature, hasDesignation ringBearer, youControl]` twice for "your
    Ring-bearer" beside the local `ringBearers` (`:728`). "Ring-bearer" is a designation a
    permanent has [CR#701.54b], and the emblem prints "Your Ring-bearer" with no "creature"
    [CR#701.54c]. Write `ringBearers` at both places.
  - `:766` writes `instruct (controllerOf (that creature)) (sacrifice (that creature))` for "the
    blocking creature's controller sacrifices it"; write `controllerSacrifices (that creature)`
    (`Definitions/Actions.lean:542`), whose body is that instruct.

## What must not change

- `putInto`, `putOntoBattlefield`, `returnToBattlefield`, `returnToHand`, the fixed-rider
  `putOntoBattlefield*`/`returnToBattlefield*` Templates, `createToken`, `createTappedToken`,
  `create`, Core `Action.move` and every checker module.
- No card changes its printed verb except as listed. No pin is weakened.

## Pins

- Rewritten: `putAndReturnExpandToOneTree`, `riderNegationIsOnlyAPattern`,
  `costCoercionsKeepTheCard` (lockstep twin), the six `Proofs/Turn.lean` fixtures and
  `okIfDoneReadsDoneBody` (spelling only, claims kept), the six `returnTo … battlefield` fixtures
  in §2, and `okChromeMoxMana` (now over the spelled card).
- Must stay green: `okOpenTheVaults`, `okAnimateDead` (`Proofs/LinkedReadback.lean:530`),
  `okChampion` (`:338`), `okDanceOfTheDead`, `okFireNavyTrebuchet`,
  `seraphicGreatsword` (spelled) and its `Proofs/CardFidelity.lean:275` pin,
  `okEncoreDefinition` (`Proofs/KeywordAbilityCensus.lean:136`), `okHateMirageTokens`,
  `okCreatureSpellFromAmongExiled`, `okSynodSanctumReturn`, `okMuseVesselPlay`,
  `okIceCauldronNotedMana`, `enchantedVanillaChangesOnlyItsAttachmentHost`
  (`Proofs/GetsBothDeltas.lean:260-264`), `okTheRingTemptsYou` (`Proofs/Keyword.lean:398`),
  `reconfigureAnchorsOnTheSource` (`Proofs/ContrastAnchor.lean:107`), `Proofs/Keyword.lean:505`,
  `keywordBodiesKeepTheirActionDeeds` (`Proofs/Keyword.lean:551`).
- Added: `persistReturnsWithACounter` and `undyingReturnsWithACounter` in `Proofs/Keyword.lean`,
  each a `decide` that the keyword's ability checks `[]`; `putTakesRiders` in
  `Proofs/DefinitionsAndTemplates.lean`: `put s d r = putInto s d none r` by `rfl` (pins that
  the merged `put` is the "into"-free twin of `putInto`, same tree).

## Acceptance

- `./lean/scripts/build` (`--wfail`), `cargo xtask lean check`.
- `grep -rn 'putWith\|exiledWithThisArtifact\|createTokenCopy\|returnTo [^(]*battlefield'
  lean/Semantics` and `grep -rnw 'random\|entersAttachedTo' lean/Semantics/Templates.lean` are
  empty.
- CR citation gates.
