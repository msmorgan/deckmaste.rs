---
needs: [lean-v3-checker-and-drt-fixes, lean-v3-authoring-impl-14-ast-and-authoring-cleanup]
---
# Fix Rules and Oracle meaning defects across Lean v3 Definitions, Templates, and Cards

**Fix the rules-meaning and Oracle-fidelity defects in `lean/Semantics/Templates.lean`,
`lean/Semantics/Definitions/`, and `lean/Semantics/Cards/` identified in
[`docs/evidence/lean-v3-opus-findings.md`](../../evidence/lean-v3-opus-findings.md)
(§§1.1–1.17, §3.2, §3.4, §4.7) and
[`docs/evidence/lean-v3-astra-findings.md`](../../evidence/lean-v3-astra-findings.md)
(§1).** Standard constraints apply.

## Scope

### 1. Negative P/T Doubling & Lossy Counter Movement (Opus §§1.1, 1.11; Astra §1)

- **Negative power/toughness doubling/tripling (§1.1):** Under
  `[CR#107.1b,701.10c,701.11c]`, doubling or tripling a negative power or
  toughness makes it more negative rather than clamping to 0. Fix `double*` and
  `triple*` in `lean/Semantics/Definitions/Actions.lean:500-552` and
  `CharacteristicEdit.numberSlots` in `lean/Semantics/Check/NumberSlots.lean:101-103`
  so P/T doubling/tripling is not marked `.clamped`.
- **Counter movement fidelity (Astra §1, Opus §1.11):**
  - Fix `moveCounters` (`lean/Semantics/Templates.lean:1533`) so moving counters
    (`[CR#122.5]`) preserves the all-or-nothing relationship between removing and
    placing counters and identifies the counters actually removed when no counter
    kind is specified (updating `lean/CONTRACTS.md` "Movement destinations").
  - Fix *Star Pupil* and *Denry Klin* (`lean/Semantics/Cards/Counters.lean:429,397`)
    so they move all counters of each kind (`thatMuch thoseCounters` / `moveCounters`)
    rather than `putCounters (.lit 1) (sameKindAs it)`.

### 2. Keyword & Action Definition Fidelity Fixes (Opus §§1.2, 1.7–1.9, 1.15, 1.17, 3.2)

- **`bloodthirst` (`[CR#702.54a]`, §1.2):** Fix `lean/Semantics/Definitions/Keywords.lean:334-338`
  so `anOpponent` is the damage recipient (`dealDamage (to := a isOpponent)`), not
  the `.actor` dealer.
- **`reconfigure` (`[CR#702.151a,701.3d]`, §1.7):** Fix `Keywords.lean:1201-1202`
  to use `unattach` instead of stamping unattach as `enact (.action "attach")`.
- **`regenerate` (`[CR#701.19c]`, §1.8):** Fix `Definitions/Actions.lean:655-675`
  so creating a regeneration replacement shield does not carry the `"regenerate"`
  deed on creation — only the shield's application (`regeneration`) carries the
  deed — and strengthen `regenerationEventDescribesTheApplicationRatherThanTheShield`
  to pin the deed.
- **`buyback`, `rebound`, and `haunt` spell half (`[CR#702.27a,702.55a,702.88a]`, §1.9):**
  Restrict their `stack → graveyard` replacement/trigger in `Keywords.lean:344-352,796,1185-1195`
  to resolution so a countered spell does not return to hand, rebound, or haunt.
- **`endure` (`[CR#701.63a]`, §1.17):** Fix `endure` in `Definitions/Actions.lean`
  to reference `firstMention .actor` (like `explore` and `connive`) rather than
  `that .permanent`.
- **Smaller keyword fidelity fixes (§1.15):**
  - `timeTravel` (`[CR#702.62b]`): include the `"has suspend"` filter on exiled
    cards.
  - `clash` (`[CR#701.30b..701.30d]`): include the opponent and winner condition.
  - `melee` (`[CR#702.121a]`): count attacked opponents rather than attack events.
  - `shadow` (`[CR#702.28b]`): bind `thisCreature` rather than quantifying over
    every shadow creature in the second static ability.
- **Route keyword bodies through keyword-action `Definitions` (§3.2):**
  - Replace raw `.search` / `.shuffle` in `transmute` and `transfigure`
    (`Keywords.lean:1439-1460`), raw `.exchange` in `auraSwap`, raw
    `.expose .reveal` in `ninjutsu`/`splice`/`amplify` with the corresponding
    `Definitions` so their deeds are preserved. The raw `setStatus .untapped` in
    `doesntUntap` moved to [lean-v3-untap-statics](lean-v3-untap-statics.md).

### 3. Card Encoding Fidelity Fixes (Opus §§1.3–1.6, 1.10, 1.12–1.14, 3.4, 4.7)

- **Celestial Convergence (`Cards/Counters.lean:205-213`, §1.3):** Nest the
  life-tie → `drawGame` branch inside the `"if there are no omen counters on ~"`
  condition.
- **Berserk & Ghor-Clan Rampager (§1.4):** moved to
  [lean-v3-trigger-words](lean-v3-trigger-words.md) §3, which writes both as one shared-subject
  `establishFor` with the target written once, after
  [lean-v3-gets-printed-zero](lean-v3-gets-printed-zero.md) turns *Berserk*'s `modification`
  pair into one `getsPt` (owner ruling 2026-10-21: the printed "+0" stays).
- **Inkshield (`Cards/Damage.lean:451-454`, §1.5):** Prevent `some .combat` damage
  (not `none`), and give the created 2/1 white and black token the `inkling`
  subtype and `flying`.
- **Mishra's Factory (`Cards/Mana.lean:141,145`, `[CR#205.3m]`, §1.6):** Use the
  catalog `assemblyWorker` (`"Assembly-Worker"`) subtype instead of the
  misspelled `"AssemblyWorker"`.
- **Performer handoffs on discard, unless-pays, and damage (§§1.10, 4.7):**
  - Wrap `"that player discards"` and `"each player/opponent discards"` in
    `instruct <player> (discard ...)` (`[CR#701.9a]`) on *Hypnotic Specter*
    (`Cards/Anaphora.lean:250`), *Immersturm Skullcairn* (`Cards/Damage.lean:47`),
    *Oona's Blackguard* (`Cards/Counters.lean:728`), *Sonic Shrieker*
    (`Cards/Damage.lean:764`), *Rankle* and *Braids's Frightful Return*
    (`Cards/Choice.lean:59,87`), `Cards/Piles.lean:202`,
    `Cards/Counters.lean:1218`, and `Cards/Keyword.lean:1055`.
  - Fix `"unless its controller pays"` on *Mana Leak* (`Cards/Anaphora.lean:151`),
    *Override*, *Rakshasa's Disdain*, *Rune Snag* (`Cards/Choice.lean:258,877,884`),
    and *Tidal Flats* (`:907-916`) so the spell/ability's controller — not the
    invited payer — performs `counter it`, and generalize `payLife` to `Amount`
    so *Killing Wave*, *Heat Wave*, and *Font of Agonies* use `payLife` instead of
    `perform (loseLife ...)`.
  - Standardizing damage-source performer handoffs moved to
    [lean-v3-spell-damage-dealer](lean-v3-spell-damage-dealer.md) (owner ruling 2026-10-21:
    "<this> deals" is a Handoff to the source on spells as on permanents).
- **Phyrexian Infiltrator (`Cards/Anaphora.lean:64-67`, `[CR#701.12a,701.12b]`, §1.12):**
  Use `exchange (controlOf thisCreature) (controlOf (target isCreature))` instead
  of `simultaneously [gainControl ..., gainControl ...]`.
- **One-shot resolution `"cast it"` (`[CR#608.2g]`, §1.13):** Encode *Memory
  Plunder* (`Cards/Choice.lean:1005-1015`), *Flawless Forgery*
  (`Cards/Copy.lean:56-61`), and *Arcane Proxy* (`Cards/Faces.lean:601-614`) as
  one-shot optional casting during resolution (`may (cast ...)`) rather than a
  durationless static permission `establish (can you (cast ...))`.
- **`"Return"` vs `"put"` (§3.4):** moved to
  [lean-v3-move-deeds-and-verbs](lean-v3-move-deeds-and-verbs.md). The owner ruling of
  2026-10-21 removes `CoreDeed.return_` and `.put`, so the sites no longer "carry
  `.core .return_`"; they take the Template of their printed verb.
- **Restore dropped Oracle clauses (§1.14):** Complete the omitted clauses on
  *Qarsi Deceiver* (`Cards/Mana.lean:300`), *Overgrown Zealot* (`:277`),
  *Candles' Glow* (`Cards/Damage.lean:440`), *Giggling Skitterspike*
  (`Cards/Trigger.lean:223`), *Cyclone* (`Cards/Counters.lean:542`), *Vedalken
  Squirrel-Whacker* (`Cards/Anaphora.lean:264`), and *Purging Scythe*
  (`Cards/Damage.lean:227-234`).

### 4. Printed move text dropped by the move-deeds landing (wave review, 2026-10-21)

Reviews of [lean-v3-move-deeds-and-verbs](lean-v3-move-deeds-and-verbs.md) found two kinds of
printed text that the rewritten move sites do not carry. Line numbers verified 2026-10-21
against the Lean source and the Oracle text in `data/derived/cards.jsonl`.

- **"under its owner's control" is missing (`[CR#110.2a]`).** An effect that puts an object onto
  the battlefield puts it under the instructing player's control unless the effect states
  otherwise, so these three read as the caster (or ability controller) taking control. Write the
  rider as *Flickering Spirit* (`Cards/Anaphora.lean:38`) and *Another Round* (`:192`) do,
  `[(under (ownerOf …))]`, with the referent each card names:
  - *Turn to Mist* `Cards/Turn.lean:33`: `returnToBattlefield (that card)`. Oracle: "Return that
    card to the battlefield under its owner's control at the beginning of the next end step."
  - *Voyager Staff* `Cards/Turn.lean:40`: "Return the exiled card to the battlefield under its
    owner's control at the beginning of the next end step."
  - *Graceful Reprieve* `Cards/Trigger.lean:19`: "return that card to the battlefield under its
    owner's control."
  A sweep of the authored cards whose Oracle text prints "under its/their owner's control" found
  no others (*Daydream*, *Infernal Vessel* and *Yedora, Grave Gardener* already write it).
- **"from your graveyard" (or "from a graveyard") is not the move's origin.** The sites below
  print the origin in the move clause and write only a target restriction (`in_ yourGraveyard`)
  or nothing at all. A move written without an origin is observed on arrival, not on departure,
  so a "leaves a graveyard" trigger would not see it (`zoneChange` in `Templates.lean` decides
  the Observation Point from the written origin, `[CR#603.6c,603.10a]`). Writing the origin makes
  these moves departure-observed, which is harmless for the effects themselves. Write the origin
  with the Template's own parameter (`returnToHand subject yourGraveyard`,
  `returnToBattlefield subject riders (from_ := some yourGraveyard)`), as `dredge` and `recover`
  already do (`Definitions/Keywords.lean:534,1216`):
  - Origin missing on a "this card" or all-cards subject: *Hammer of Bogardan* `Cards/Turn.lean:77`
    (`returnToHand this_`), *Storm of Souls* `Cards/Keyword.lean:857`, *Eerie Ultimatum*
    `Cards/Static.lean:868`.
  - Origin written only as the target's `in_ yourGraveyard`: *Ever After* `Cards/Anaphora.lean:176`,
    *Rogues' Gallery* `Cards/Description.lean:351`, *Blood Spatter Analysis*
    `Cards/Counters.lean:1074`, *The Last Ronin* `Cards/Damage.lean:1347`.
  - Same pattern, found while verifying: *Bond of Revival* `Cards/Anaphora.lean:44`, *Midnight
    Scavengers* `Cards/Faces.lean:405`, and "from a graveyard" on *Rise from the Grave*
    `Cards/Anaphora.lean:169` and *Hymn of Rebirth* `Cards/Static.lean:38`. The last two use
    `putOntoBattlefieldUnderYourControl`, which has no origin parameter; give the
    `putOntoBattlefield*` Templates the optional `from_` that `putOntoBattlefieldTappedAttacking`
    already has.
  - Not in scope: restrictions that are the noun phrase's own modifier ("creature cards in your
    graveyard", as *Continue?*, *Faith's Reward*, *Death or Glory*) are not move origins.
  - Proposed, not ruled: keep the `in_` restriction on a target where it is what makes the
    target legal, and add the origin beside it. If the checker refuses the pair as redundant,
    that is a checker question for an owner ruling, not a reason to drop either.

Tests to add (in `Proofs/` beside the existing move pins): for each rider site, a pin that the
authored action's move carries `.under` the owner; for each origin site, a pin that the move is
departure-observed from the graveyard; and a refusal pin that a bare `returnToBattlefield` with
no rider and no origin stays observed on arrival. Update any pin that names the old tree.

## Verification

- `./lean/scripts/build` (`lake build --wfail`) passes with zero warnings and
  zero `sorry`s.
- `cargo test -p xtask -p deckmaste_semantics_v2` and
  `cargo clippy -p xtask -p deckmaste_semantics_v2 --tests -- -D warnings` pass.
- `cargo xtask cite check --list-noncompliant` is empty and
  `cargo xtask cite check` reports 0 stale citations.
