---
needs: [lean-v3-checker-and-drt-fixes, lean-v3-authoring-impl-14-ast-and-authoring-cleanup]
---
# Fix Rules and Oracle meaning defects across Lean v3 Definitions, Templates, and Cards

**Fix the rules-meaning and Oracle-fidelity defects in `lean/Semantics/Templates.lean`,
`lean/Semantics/Definitions/`, and `lean/Semantics/Cards/` identified in
[`docs/evidence/lean-v3-opus-findings.md`](../../evidence/lean-v3-opus-findings.md)
(§§1.1–1.17, §3.2, §3.4, §4.7) and
[`docs/evidence/lean-v3-astra-findings.md`](../../evidence/lean-v3-astra-findings.md)
(§1), plus the 2026-10-21 mtg-rules review
(`docs/evidence/mtg-rules-review-2026-10-21/departures-lean.md`; entry numbers below as "review
#N", re-verified against head 2026-10-21).** Standard constraints apply.

## Scope

### 1. Negative P/T Doubling & Lossy Counter Movement (Opus §§1.1, 1.11; Astra §1)

- **Negative power/toughness doubling/tripling (§1.1):** Under
  `[CR#107.1b,701.10c,701.11c]`, doubling or tripling a negative power or
  toughness makes it more negative rather than clamping to 0. Fix `double*` and
  `triple*` in `lean/Semantics/Definitions/Actions.lean:440-475` and the regimes in
  `lean/Semantics/Check/NumberSlots.lean` (`:20-21` reads every `.up`/`.down` as `.clamped`;
  `:119` clamps every non-`.sets` stat edit; the `:5-8` docstring already exempts doubling but
  nothing implements it) so P/T doubling/tripling is `.signed`: a dedicated delta, or the
  [CR#701.10c] `.down` expansion ("gets -X/-0, where X is the difference between 0 and its
  power") per axis (review #25).
  - Ordinary `+X/+X` stays clamped: pin a *Chameleon Colossus* case beside new negative
    double and triple pins.
  - Correct `Proofs/Numbers.lean:131-134`, which says "[CR#107.1b] exempts doubling from
    clamping" for damage: damage doubling is a replacement effect [CR#701.10g] and not on
    [CR#107.1b]'s list.
- **Counter movement fidelity (Astra §1, Opus §1.11):**
  - Fix `moveCounters` (`lean/Semantics/Templates.lean:1506-1511`, a guardless
    `.sequentially [.removeCounters …, .putCounters …]`) so moving counters
    (`[CR#122.5]`: "If either of these actions isn't possible … no counter is removed from or
    put onto anything") is all-or-nothing: atomic, or guarded on both halves being possible
    (different objects, kind present, destination can take counters, both in the right zone).
    The kind-identity half is already done at head (`kind.getD .those`); `lean/CONTRACTS.md:1068-1072`
    still says `sameAs` and "not modelled" and is corrected in the same change. Add proofs for
    the destination-gone and same-object cases (review #42).
  - Fix *Star Pupil* and *Denry Klin* (`lean/Semantics/Cards/Counters.lean:426,394`), which
    put `(.lit 1) (sameKindAs …)`, one of each kind (`CounterKindSource.sameAs`,
    `Abilities.lean:585-588`). Corrected 2026-10-21 (review #9): neither `moveCounters` nor
    `thatMuch thoseCounters` is right. *Star Pupil* puts "the same number of each kind of
    counter" and "doesn't move counters" [CR#122.8], so it is no [CR#122.5] move; and
    `thoseCounters` (`.those`) means counters already put or removed this way, of which there are
    none. Loop per kind as `doubleCounters` does (`Definitions/Actions.lean:274-279`):
    `.doForEachKind .counterKind (some src) .counterKind (putCounters (.statOf .boundCounter src)
    .bound dst)`. *Star Pupil*'s source is read by last known information [CR#608.2h];
    *Denry Klin*'s fix rests on its own Oracle text ("put the same number of each kind of
    counter"), since [CR#122.8] covers only an object that left the battlefield.

### 2. Keyword & Action Definition Fidelity Fixes (Opus §§1.2, 1.7–1.9, 1.15, 1.17, 3.2)

- **`bloodthirst` (`[CR#702.54a]`, §1.2, review #1, critical):** Fix
  `lean/Semantics/Definitions/Keywords.lean:336`, `doIf (did anOpponent (.dealDamage .anyAmount
  .which) soFarThisTurn)`. `Condition.did` puts its subject in the agent slot
  (`Templates.lean:2087-2097`), so it reads "an opponent dealt damage", which never holds: the
  object that deals damage is its source [CR#120.1]. Write the agentless `Condition.was
  (dealDamage anyAmount anOpponent) soFarThisTurn`, the shape *Furious Spinesplitter* uses
  (`Cards/Counters.lean:139-143`). The checker cannot catch an agent/patient swap, so add a pin
  on the tree. The RON mirror (`bloodthirst.ron`) is already right.
- **`reconfigure` (`[CR#702.151a,701.3d]`, §1.7):** Fix `Keywords.lean:1201-1202`
  to use `unattach` instead of stamping unattach as `enact (.action "attach")`.
- **`regenerate` (`[CR#701.19c]`, §1.8):** Fix `Definitions/Actions.lean:655-675`
  so creating a regeneration replacement shield does not carry the `"regenerate"`
  deed on creation — only the shield's application (`regeneration`) carries the
  deed — and strengthen `regenerationEventDescribesTheApplicationRatherThanTheShield`
  to pin the deed.
- **`buyback`, `rebound`, and `haunt` spell half (`[CR#702.27a,702.55a,702.88a]`, §1.9):**
  Restrict their `stack → graveyard` replacement/trigger (`Keywords.lean:347,805,1197`, each
  `.move thisSpell stack (some graveyard) .departure none []`) to the [CR#608.2n] resolution move,
  so a spell that doesn't resolve for any reason (countered [CR#701.6a], or all targets illegal
  [CR#608.2b]) does not return to hand, rebound, or haunt (review #28). Neither `Causing`
  (`Abilities.lean:479-482`) nor `ObservationPoint` (`:21-23`) can single out that move, so add a
  discriminator. Same in `buyback.ron`, `rebound.ron`, `haunt.ron`, whose comments already say "as
  it resolves". Leave flashback, aftermath and harmonize on any departure
  [CR#702.34a,702.127a,702.180a].
- **`endure` (`[CR#701.63a]`, §1.17):** Fix `endure` in `Definitions/Actions.lean`
  to reference `firstMention .actor` (like `explore` and `connive`) rather than
  `that .permanent`.
- **Smaller keyword fidelity fixes (§1.15):**
  - `timeTravel` (`[CR#701.56a,702.62b]`, `Definitions/Actions.lean:782-783`): include the
    `"has suspend"` filter on exiled cards (`.hasKeyword (.the (.named "suspend"))`; review #26).
    The mirror `timeTravel.ron:20-28` has the same gap, and its comment (`:4-5`) cites [CR#702.62a]
    for the suspended state, which is [CR#702.62b].
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

- **Celestial Convergence (`Cards/Counters.lean:203-210`, §1.3, review #7):** Nest the
  life-tie → `drawGame` branch inside the `"if there are no omen counters on ~"`
  condition. Pin both the tie-with-counters and the zero-counter-tie cases ("no default for
  ties" [CR#609.5]).
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
  Use the Definition `exchangeControl thisCreature (target isCreature)`
  (`Definitions/Actions.lean:286`; the raw form is `exchange (controlOf thisCreature (target
  isCreature))`, as *Avarice Totem*, `Cards/Keyword.lean:1607-1608`) instead of
  `simultaneously [gainControl ..., gainControl ...]` (`Cards/Anaphora.lean:66-69`). Corrected
  2026-10-21: the earlier `exchange (controlOf …) (controlOf …)` was malformed, since `controlOf`
  takes both operands (review #4).
- **One-shot resolution `"cast it"` (`[CR#608.2g]`, §1.13):** Encode *Memory
  Plunder* (`Cards/Choice.lean:1005-1015`), *Flawless Forgery*
  (`Cards/Copy.lean:56-61`), and *Arcane Proxy* (`Cards/Faces.lean:601-614`) as
  one-shot optional casting during resolution (`may (cast ...)`) rather than a
  durationless static permission `establish (can you (cast ...))`.
  - **Madness (review #34):** the may-arm of `mayOrElse` in `Definitions/Keywords.lean:930-945`
    is the same durationless `establish (can theActor (cast thisCard …))`, which lasts until the
    game ends [CR#611.2a] with normal timing. [CR#702.35a]: its owner "may cast it … If that
    player doesn't, they put this card into their graveyard". Replace the arm with a direct
    `cast thisCard (from_ := some exileZone) (payment := .payingInstead cost)`; the graveyard
    fallback already exists. Same in `madness.ron`.
- **`"Return"` vs `"put"` (§3.4):** moved to
  [lean-v3-move-deeds-and-verbs](lean-v3-move-deeds-and-verbs.md). The owner ruling of
  2026-10-21 removes `CoreDeed.return_` and `.put`, so the sites no longer "carry
  `.core .return_`"; they take the Template of their printed verb.
- **Restore dropped Oracle clauses (§1.14):** Complete the omitted clauses on
  *Qarsi Deceiver* (`Cards/Mana.lean:302-306`), *Overgrown Zealot* (`:284-287`),
  *Candles' Glow* (`Cards/Damage.lean:440`), *Giggling Skitterspike*
  (`Cards/Trigger.lean:223`), *Cyclone* (`Cards/Counters.lean:542`), *Vedalken
  Squirrel-Whacker* (`Cards/Anaphora.lean:264`), and *Purging Scythe*
  (`Cards/Damage.lean:227-234`). Directions recorded 2026-10-21:
  - *Purging Scythe* (review #7): it deals 2 damage to `the` least-toughness creature and only
    then makes a `choose` nothing reads. The tie sentence modifies the earlier text
    [CR#608.2c,609.5]: choose among the least-toughness creatures first, then deal 2 damage to
    the chosen one.
  - *Overgrown Zealot* and *Qarsi Deceiver* (review #19): the spend purpose covers only morph and
    disguise costs (`CostNamed` is `containing | ofKeyword`, `Words.lean:456-459`). Turning a
    manifested or cloaked permanent face up is the Special Action of [CR#116.2b], "pay that cost,
    then turn the permanent face up" [CR#701.40b,701.58b] (Zealot ruling, 2024-09-20; Qarsi's
    Oracle text names manifested creatures). Add a spend purpose for the turn-face-up Special
    Action (neither `ofSpecialAction` nor `faceUpManaCost` exists), use it for Zealot, and add the
    manifest arm to Qarsi.

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

### 5. Card witnesses that drop or misread clauses (mtg-rules review, 2026-10-21)

Each site below was re-verified at head on 2026-10-21. Under [CR#608.2c] the whole text governs.
Where a done ticket recorded the current shape as intended, this section supersedes that record.

- *Darksteel Garrison* (`Cards/Cost.lean:848`, review #6): "Whenever fortified land becomes
  tapped" is `does any (tapForMana (fortified land))`. A "becomes tapped" trigger fires on the
  change from untapped to tapped [CR#603.2e], not only for mana (ruling 2007-05-01). Write `does
  (fortified land) (become tapped)` (`Status.become`, `Templates.lean:1055`, as *Gideon's Avenger*,
  `Cards/Counters.lean:116`).
- *Lightmine Field* (`Cards/Damage.lean:253-254`, review #10): the amount is `numberOf (those
  creature)`, the triggering group. The Oracle text counts "attacking creatures", determined as the
  effect applies [CR#608.2h]; a creature removed from combat isn't attacking [CR#506.4] (rulings
  2010-06-15). Write `dealDamage (countOf isAttacking) (eachOf (those creature))`. This supersedes
  `docs/tickets/done/workbench-bare-plural-det.md:77-80`'s re-spelling as "the number of those
  creatures".
- *Inkshield* (review #11, §3 above): `damageRule` needs `(kind := some .combat)`
  (`Templates.lean:1896-1897`), and the token is `creatureToken 2 1 [.white, .black] [inkling]`
  with `(abilities := [flying])`.
- *Temple Altisaur* (`Cards/Damage.lean:508-510`, review #11): the recipient lacks "another".
  Write it with the bare "another" (`isOther`; ADR [semantics-v3](../../decisions/semantics-v3.md)
  §0.1 resolves the Contrast Anchor to the source).
- *Aether Gust* (`Cards/Choice.lean:983`, review #11): `and [isPermanent, hasColor .red]` drops
  "or green" and "spell or". Write `and [or [isSpell, isPermanent], or [hasColor .red, hasColor
  .green]]`. If the spell-or-permanent subject is still refused (the documented
  `badSpellOrPermanentSubject` gap), keep "or green" and leave that half to its gap.
- *Quakebringer* (`Cards/Damage.lean:903-909`, review #12): "This ability triggers only if …" is
  passed to `at_`'s `if_`, the intervening "if" [CR#603.4], which is rechecked on resolution, so the
  ability does nothing if the Giant dies in response. [CR#603.4] "only applies to an 'if' that
  immediately follows a trigger condition". Use the `qualified` guard (`Templates.lean:1839-1842`,
  doc `Abilities.lean:1231-1236`), the carrier `solved` uses for [CR#702.169c]. `qualified`'s `from_`
  holds one zone; the battlefield-or-graveyard functioning may need the guard alone. Supersedes the
  note in `docs/tickets/done/workbench-coordination-1-condition-disjunction.md:126-129`.
- *Keeper of the Flame* (`Cards/Damage.lean:1322-1324`, review #14): the life comparison sits in
  the target's description, so it is rechecked on resolution [CR#608.2b]; the ruling (2009-10-01)
  needs it true only "as you activate the ability". Either make it an activation-time condition
  checked only at [CR#601.2c,602.2b] and excluded from the [CR#608.2b] recheck, or write `target
  isOpponent` plus an activation-time legality condition. Update the `when` rider's docstring
  (`Abilities.lean:1086-1087`) and the pin `okChoiceWithActivationRider`
  (`Proofs/Choice.lean:542-548`), whose docstring says "the rider times the announcement, not the
  comparison".
- *Cult of the Waxing Moon* (`Cards/Faces.lean:372-374`, review #17): `whenever (does any (turnOver
  (a (and [isPermanent, youControl])))) …` has no deed and no "non-Human creature" complement. The
  ability triggers if the object transforms or converts and "has the specified characteristic
  immediately after" [CR#701.27e]. Write it with `.enact (.action "transform") (.turnOver …) (some
  (and [isCreature, not (hasSubtype human)]))` and match converts per
  [lean-v3-keyword-triggers-entries-and-parties](lean-v3-keyword-triggers-entries-and-parties.md)
  §9. `docs/tickets/done/workbench-transform-tail.md:56` claims the complement landed; head lacks
  it.
- *Archfiend's Vessel* (`Cards/Keyword.lean:1174-1178`, review #18): "exile it. If you do, create
  …" is `triggerReflexively`. Reflexive triggers are only "when [a player] [does or doesn't]" or
  "when [something happens] this way" [CR#603.12]; "If you do" is followed in order [CR#608.2c].
  Write `doIfDone (exile it) (some (createToken …))` (`Templates.lean:1468-1470`).
- *Memoricide* (`Cards/Choice.lean:967-971`, review #47): the count is fixed (`counted anyNumber
  …`), but "exile them" is missing before the shuffle. Add it. (The Idris witness is
  deletion-bound; leave it.)
- Ties: *Padeem* (`Cards/Mana.lean:947-950`), *Seraphic Greatsword* (`Cards/Keyword.lean:1592`),
  *Undercover Butler* (`Cards/Deontic.lean:233`) and the `dethrone` Definition
  (`Definitions/Keywords.lean:481-487`) all write "the X with the most … or tied for the most" as a
  singular `the (and [… superlative .max …])`, which drops the tie (review #48). "The Magic game
  has no default for ties" [CR#609.5]; "or tied for" is membership in the tied set
  ([CR#702.105a]: "the player with the most life or tied for most life"). Encode membership ("you
  control an artifact satisfying `superlative .max manaValue`", "attacks a player satisfying
  `superlative .max lifeTotal`"). Keep `the` for definite superlatives that print their own tie
  sentence.

Tests: a pin per site on the corrected tree; for *Lightmine Field*, *Quakebringer* and *Keeper of
the Flame*, a pin on the timing distinction itself.

## Verification

- `./lean/scripts/build` (`lake build --wfail`) passes with zero warnings and
  zero `sorry`s.
- `cargo xtask lean check` (RON mirrors change); `cargo test -p xtask -p deckmaste_semantics_v2` and
  `cargo clippy -p xtask -p deckmaste_semantics_v2 --tests -- -D warnings` pass.
- `cargo xtask cite check --list-noncompliant` is empty and
  `cargo xtask cite check` reports 0 stale citations.
