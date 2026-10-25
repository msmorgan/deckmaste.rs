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
#N", re-verified against head 2026-10-21).** Standard constraints apply. File:line sites below
were re-located against the Lean source on 2026-10-24.

## Scope

### 1. Negative P/T Doubling & Lossy Counter Movement (Opus §§1.1, 1.11; Astra §1)

- **Negative power/toughness doubling/tripling (§1.1):** Under
  `[CR#107.1b,701.10c,701.11c]`, doubling or tripling a negative power or
  toughness makes it more negative rather than clamping to 0. Fix `double*` and
  `triple*` in `lean/Semantics/Definitions/Actions.lean:467-519` (each still `get … (.up (powerOf
  …))` / `(.up (times 2 …))`) and the regimes in `lean/Semantics/Check/NumberSlots.lean`
  (`Delta.numberSlots`, `:20-22`, reads every `.up`/`.down` as `.clamped`;
  `CharacteristicEdit.numberSlots`, `:104-107`, clamps every non-`.sets` stat edit; the `:5-6`
  `NumberRegime` docstring already exempts doubling but nothing implements it) so P/T
  doubling/tripling is `.signed`: a dedicated delta, or the
  [CR#701.10c] `.down` expansion ("gets -X/-0, where X is the difference between 0 and its
  power") per axis (review #25).
  - Ordinary `+X/+X` stays clamped: pin a *Chameleon Colossus* case beside new negative
    double and triple pins.
  - Correct `Proofs/Numbers.lean:131-134`, which says "[CR#107.1b] exempts doubling from
    clamping" for damage: damage doubling is a replacement effect [CR#701.10g] and not on
    [CR#107.1b]'s list.
    **Reconciled 2026-10-24: done** (no single landing ticket): `okDoubledDamageClamped`'s
    docstring (`Proofs/Numbers.lean:142-146`) now says [CR#107.1b] exempts doubling only of a life
    total or a power or toughness, damage being doubled by replacement [CR#701.10g].
  **Done** (`vrlrtwru`): the [CR#701.10c] `.down` expansion per axis. The six `double*`/`triple*`
  Definitions are `getsScaledStats` (`Definitions/Actions.lean`): per axis, `doIf (… .less 0)`
  gets `.down (per × differenceBetween 0 value)`, then `doIf (… .atLeast 0)` gets `.up (per ×
  value)`; scaling keeps the sign, so exactly one fires, and every clamped amount is a magnitude.
  The `NumberRegime` docstring says how doubling is written. Pins (`Proofs/Numbers.lean`):
  `negativePowerDoublesBelowZero`, `negativeAxisTriplesBelowZero`,
  `doublingChangesAreClampedMagnitudes`, `chameleonColossusBoostStaysClamped`. The two guards per
  axis are a workaround: the intended spelling is one `doIf (… .less 0) … (some …)` per axis, which
  the checker refuses because an otherwise-branch cannot read a subject first mentioned in the
  condition; [lean-v3-doif-otherwise-reads-condition-mentions](../planned/lean-v3-doif-otherwise-reads-condition-mentions.md)
  fixes that and rewrites `getsScaledStats`. v2 has no number regime; its RON helpers
  (`instructions/doublePower.ron` and kin) were left alone because v2 is deletion-bound.
- **Counter movement fidelity (Astra §1, Opus §1.11):**
  - Fix `moveCounters` (`lean/Semantics/Templates.lean:1539-1546`, a guardless
    `.sequentially [.removeCounters …, .putCounters …]`) so moving counters
    (`[CR#122.5]`: "If either of these actions isn't possible … no counter is removed from or
    put onto anything") is all-or-nothing: atomic, or guarded on both halves being possible
    (different objects, kind present, destination can take counters, both in the right zone).
    The kind-identity half is already done at head (`kind.getD .those`); `lean/CONTRACTS.md:1296-1300`
    still says `sameAs` and "not modelled" and is corrected in the same change. Add proofs for
    the destination-gone and same-object cases (review #42) beside the existing `moveCounters`
    pins (`Proofs/Counters.lean:155-163`).
    **Done** (`xwvlrsst`; owner ruling 2026-10-25, option (a)): the atomic `Action.moveCounters
    amount kind source destination` (`Abilities.lean`), modelled on `.exchange` [CR#701.12a], which
    the engine runs all or nothing. The `moveCounters` Template builds it, so the card call sites
    are unchanged; the checker steps it as its two halves, so it still publishes both outcomes
    and keeps the `kind.getD .those` read-back (`moveCounterPublishesRemovalAndPlacement` passes
    unchanged). `lean/CONTRACTS.md` drops the "not modelled" note. Pins
    `moveCountersToAGoneDestinationIsOneAction`, `moveCountersOntoTheSameObjectIsOneAction`
    (`Proofs/Counters.lean`). No v2 mirror: v2 is deletion-bound.
  - Fix *Star Pupil* and *Denry Klin* (`lean/Semantics/Cards/Counters.lean:421,391`), which
    put `(.lit 1) (sameKindAs …)`, one of each kind (`CounterKindSource.sameAs`,
    `Abilities.lean:648`; `sameKindAs`, `Templates.lean:2230`). Corrected 2026-10-21 (review #9):
    neither `moveCounters` nor
    `thatMuch thoseCounters` is right. *Star Pupil* puts "the same number of each kind of
    counter" and "doesn't move counters" [CR#122.8], so it is no [CR#122.5] move; and
    `thoseCounters` (`.those`) means counters already put or removed this way, of which there are
    none. Loop per kind as `doubleCounters` does (`Definitions/Actions.lean:292-297`):
    `.doForEachKind .counterKind (some src) .counterKind (putCounters (.statOf .boundCounter src)
    .bound dst)`. *Star Pupil*'s source is read by last known information [CR#608.2h];
    *Denry Klin*'s fix rests on its own Oracle text ("put the same number of each kind of
    counter"), since [CR#122.8] covers only an object that left the battlefield.
    **Done** (`uwkwtzpz`): both write `doForEachKind .counterKind .counterKind (putCounters (statOf
    .boundCounter src) thatKind dst) (domain := some src)` (the Templates of the ticket's raw form;
    cards may not write `.doForEachKind`). Pins `starPupilPutsTheSameNumberOfEachKind` and
    `denryKlinPutsTheSameNumberOfEachKind` (`Proofs/Counters.lean`).

### 2. Keyword & Action Definition Fidelity Fixes (Opus §§1.2, 1.7–1.9, 1.15, 1.17, 3.2)

- **`bloodthirst` (`[CR#702.54a]`, §1.2, review #1, critical):** Fix
  `lean/Semantics/Definitions/Keywords.lean:346`, `doIf (did anOpponent (.dealDamage .anyAmount
  .which) soFarThisTurn)`. `Condition.did` puts its subject in the agent slot
  (`Templates.lean:2193-2195`), so it reads "an opponent dealt damage", which never holds: the
  object that deals damage is its source [CR#120.1]. Write the agentless `Condition.was
  (dealDamage anyAmount anOpponent) soFarThisTurn`, the shape *Furious Spinesplitter* uses
  (`Cards/Counters.lean:136-140`). The checker cannot catch an agent/patient swap, so add a pin
  on the tree. The RON mirror (`bloodthirst.ron`) is already right.
  **Done** (`nrkylvmw`): `Condition.was (dealDamage anyAmount anOpponent) soFarThisTurn`; pin
  `bloodthirstReadsTheOpponentAsTheDamagedPlayer` (`Proofs/Keyword.lean`).
- **`reconfigure` (`[CR#702.151a,701.3d]`, §1.7):** Fix `Keywords.lean:1364-1365`
  to use `unattach` (`Templates.lean:76`) instead of stamping unattach as `enact (.action
  "attach")`.
  **Done** (`nnpxuxqr`): the Lean body already wrote the raw `.attachment .unattached` with no
  deed and now writes `unattach thisPermanent`; the stamp survived in the RON mirror
  (`reconfigure.ron`, `enact(attach, …)`), now dropped. Pin
  `reconfigureUnattachesWithoutTheAttachDeed` (`Proofs/Keyword.lean`).
- **`regenerate` (`[CR#701.19c]`, §1.8):** Fix `Definitions/Actions.lean:612-617`
  so creating a regeneration replacement shield does not carry the `"regenerate"`
  deed on creation (it is still `.ownScope (some (.action "regenerate"))` around the shield's
  `.establish`) — only the shield's application (`regeneration`, `:137-152`) carries the
  deed — and strengthen `regenerationEventDescribesTheApplicationRatherThanTheShield`
  (`Proofs/ActionFamilies.lean:295-306`) to pin the deed.
  **Done** (`nnpxuxqr`): `regenerate` is `.ownScope none`; the strengthened pin asserts
  `regenerate` has no deed, `regeneration` has `regeneration.deed`, and stepping the shield causes
  no event of that deed. `regenerate.ron` already named no deed.
- **`buyback`, `rebound`, and `haunt` spell half (`[CR#702.27a,702.55a,702.88a]`, §1.9):**
  Restrict their `stack → graveyard` replacement/trigger (`Keywords.lean:347,805,1197`, each
  `.move thisSpell stack (some graveyard) .departure none []`) to the [CR#608.2n] resolution move,
  so a spell that doesn't resolve for any reason (countered [CR#701.6a], or all targets illegal
  [CR#608.2b]) does not return to hand, rebound, or haunt (review #28). Neither `Causing`
  (`Abilities.lean:479-482`) nor `ObservationPoint` (`:21-23`) can single out that move, so add a
  discriminator. Same in `buyback.ron`, `rebound.ron`, `haunt.ron`, whose comments already say "as
  it resolves". Leave flashback, aftermath and harmonize on any departure
  [CR#702.34a,702.127a,702.180a].
  **Reconciled 2026-10-24: live** (partly: the Lean half is done by
  [lean-v3-keyword-class-guards-and-observation](../done/lean-v3-keyword-class-guards-and-observation.md):
  buyback `Definitions/Keywords.lean:358`, haunt `:885` and rebound `:1351` scope the move with
  `.causedBy`/`.causes (.event (does .any (resolve thisSpell)))`. Live: the RON mirrors still
  match any stack-to-graveyard move, `plugins_v2/builtin/macros/keyword_abilities/buyback.ron:16`,
  `rebound.ron:19`, `haunt.ron:24`.)
  **Done** (`npzmoknl`): the three mirrors write the move as `causes(Event(verbedEvent(resolve,
  thisSpell)), zoneChange(…))`, mirroring Lean's `resolve` Definition into v2 as
  `keyword_actions/resolve.ron` over a new `Procedure::Resolve` (Lean `Procedure.resolve`). Pin
  `resolution_move_keywords_read_the_resolution_as_the_cause`
  (`crates/deckmaste_semantics_v2/tests/keyword_bodies.rs`); `resolve` added to the macro_def
  keyword-action name and template lists.
- **`endure` (`[CR#701.63a]`, §1.17):** Fix `endure` in `Definitions/Actions.lean:691-696`
  to reference `firstMention .actor` (like `explore` and `connive`, `:427`, `:442`) rather than
  `that .permanent` (`:695`).
  **Done** (`nnpxuxqr`): `controllerOf (.firstMention "endure.permanent" .actor)` and the counters
  on its `.laterMention`. Rewrote `badEndureHandedToOpponent` (`Proofs/Actor.lean`): its stray
  `.anaphor (.word .permanent)` refusal came from the `that .permanent` read, so it now expects only
  the `.possessorKind` refusal, as `badAuthoredExploreHandedToOpponent` does, and adds the
  handed-a-creature case.
- **Smaller keyword fidelity fixes (§1.15):**
  - `timeTravel` (`[CR#701.56a,702.62b]`, `Definitions/Actions.lean:817-819`): include the
    `"has suspend"` filter on exiled cards (`.hasKeyword (.the (.named "suspend"))`; review #26).
    The mirror `timeTravel.ron:23-28` has the same gap, and its comment (`:4-5`) cites [CR#702.62a]
    for the suspended state, which is [CR#702.62b].
    **Done** (`zkzrxmwr`): the exiled-card arm has `.hasKeyword (.the (.named "suspend"))`, in Lean
    and in `timeTravel.ron` (comment now cites [CR#702.62b]). Pin `timeTravelReadsSuspendedCards`
    (`Proofs/Keyword.lean`).
  - `clash` (`[CR#701.30b..701.30d]`, `Definitions/Actions.lean:649-654`): include the opponent
    and winner condition.
    **Superseded** by [lean-v3-clash](../planned/lean-v3-clash.md) (owner ruling 2026-10-25,
    option (b): a win/lose outcome on the clash action, read back as coin flips are).
  - `melee` (`[CR#702.121a]`, `Definitions/Keywords.lean:1098-1108`): count attacked opponents
    rather than attack events.
    **Done** (`zkzrxmwr`): `forEach 1 (and [isOpponent, did you (.attackWith (a isCreature) (some
    .which)) thisCombat])` on each axis; `melee.ron` mirrors it with `forEach(1, and([opponent,
    happenedTo(attacksWith(…, relative(Player)), thisCombat)]))`. Pin `meleeCountsAttackedOpponents`.
  - `shadow` (`[CR#702.28b]`, `Definitions/Keywords.lean:1432-1434`): bind `thisCreature` rather
    than quantifying over every shadow creature in the second static ability.
    **Done** (`zkzrxmwr`): `cant (.by thisCreature) (.block (allOf (and [isCreature, not shadow])))`;
    Pin `shadowSecondHalfIsAboutThisCreature`. `shadow.ron` is unchanged: the agent-role
    `deonticRule` mirror `zkzrxmwr` wrote broke the v2 corpus read of *Spirit en-Dal* (`shadow`
    inside its own `hasKeyword(The(shadow))` in an `Any` slot reads as a recursive macro call), so
    it was reverted; v2 is deletion-bound and this item names only the Lean body.
- **Copy keywords drop "If the spell has any targets" (added 2026-10-24; storm/replicate moved
  here from `lean-v3-proofs-and-docs-alignment` §2, its only home now):** each of these rules says
  "If the spell has any targets, you may choose new targets for" the copy or copies, but each body
  ends in an unguarded `may (.chooseNewTargets …)`. Guard it with the spell's targeting (the
  `targeting … .someTarget` predicate, as at `Cards/Choice.lean:1306`):
  - `storm` [CR#702.40a] `Definitions/Keywords.lean:27-36`; its docstring (`:25-26`) also quotes
    "You may choose new targets for the copies" without the condition;
  - `replicate` [CR#702.56a] `Keywords.lean:1382-1389`;
  - `gravestorm` [CR#702.69a] `Keywords.lean:835-841`;
  - `casualty` [CR#702.153a] `Keywords.lean:362-370`;
  - `conspire` [CR#702.78a] `Keywords.lean:413-424`;
  - `epic` [CR#702.50a] `Keywords.lean:627-636`.
  `demonstrate` (`Keywords.lean:485-493`) is correctly unguarded: [CR#702.144a] grants "you may
  choose new targets for the copy" with no condition.
  **Superseded** by the owner ruling of 2026-10-25: the item is dropped and the bodies stay
  unguarded. `may (chooseNewTargets …)` already says it, and whether the spell has targets is the
  stack object's runtime state. History: `zkzrxmwr` guarded each body with `doIf (is thisSpell (targeting
  .any .someTarget))`, which is wrong and was reverted by `rrrusrnr`: "that targets" reads the
  targets' current state [CR#115.9b], so a target that left its zone drops out, while "if the spell
  has any targets" asks whether targets were chosen for it, and a vanished or illegal target is
  still one the copy may keep [CR#707.10,707.10c]. Epic's guard was always false (the spell has
  left the stack by each upkeep), and storm's turned false when *Grapeshot*'s target was
  sacrificed. The pin
  `copyKeywordsGuardNewTargetsOnTheSpellsTargets` was retired with the guard and
  `stormCountsEarlierThisTurn` restored to the unguarded tail; the v2 RON mirrors are unguarded
  too, as wanted.
- **Route keyword bodies through keyword-action `Definitions` (§3.2):**
  - Replace raw `.search` / `.shuffle` in `transmute` and `transfigure`
    (`Keywords.lean:1632-1646`, `:1619-1630`; Definitions `search` and `shuffle` at
    `Definitions/Actions.lean:747,167`), raw `.exchange` in `auraSwap` (`Keywords.lean:263`), raw
    `.expose .reveal` in `ninjutsu` (`:1193`), `transmute` (`:1644`) and `amplify` (`:229`) with the
    corresponding `Definitions` (`reveal`, `Definitions/Actions.lean:215`) so their deeds are
    preserved. Splice no longer has a raw reveal (see below). The raw `setStatus .untapped` in
    `doesntUntap` moved to [lean-v3-untap-statics](../done/lean-v3-untap-statics.md), now done.
    **Reconciled 2026-10-24: live** (partly: splice is superseded by
    [lean-v3-keyword-cast-permissions-and-costs](../done/lean-v3-keyword-cast-permissions-and-costs.md)
    §6, which rewrote `Ability.splice` (`Definitions/Keywords.lean:1512-1518`) as `.revealedAsCast`
    with no `.expose .reveal`; transmute, transfigure, auraSwap, ninjutsu and amplify are live. No
    card-exchange Definition exists for auraSwap's `.exchange (.cardsAcross …)`; only
    `exchangeControl`, `Definitions/Actions.lean:304`.)
    **Done** (`srkrnpok`): transmute and transfigure write `search yourLibrary …` and `shuffle`;
    aura swap writes `exchangeCards` (`Definitions/Actions.lean:293`, which does exist, over
    `.cardsAcross`). Ninjutsu, transmute and amplify already revealed through `reveal`. Pin
    `keywordBodiesKeepTheirActionDeeds` (`Proofs/Keyword.lean`).

### 3. Card Encoding Fidelity Fixes (Opus §§1.3–1.6, 1.10, 1.12–1.14, 3.4, 4.7)

- **Celestial Convergence (`Cards/Counters.lean:195-202`, §1.3, review #7):** Nest the
  life-tie → `drawGame` branch inside the `"if there are no omen counters on ~"`
  condition. Pin both the tie-with-counters and the zero-counter-tie cases ("no default for
  ties" [CR#609.5]).
  **Done** (`kskwzvou`): under the no-counters condition, `doIf tie drawGame (some <the highest wins>)`,
  the tie sentence modifying the win [CR#608.2c]. Pins `celestialConvergenceTieWithCountersDoesNothing`
  and `celestialConvergenceZeroCounterTieDraws` (`Proofs/Counters.lean`).
- **Berserk & Ghor-Clan Rampager (§1.4):** moved to
  [lean-v3-trigger-words](../done/lean-v3-trigger-words.md) §3, which writes both as one shared-subject
  `establishFor` with the target written once, after
  [lean-v3-gets-printed-zero](../done/lean-v3-gets-printed-zero.md) turns *Berserk*'s `modification`
  pair into one `getsPt` (owner ruling 2026-10-21: the printed "+0" stays).
  **Reconciled 2026-10-24: done** by [lean-v3-trigger-words](../done/lean-v3-trigger-words.md):
  *Berserk* is `establishFor (target isCreature) [… getsPt it (up (letter .x)) (up 0) …]`
  (`Cards/Turn.lean:483-485`), *Ghor-Clan Rampager* `establishFor (target …) [getsPt it (up 4)
  (up 4), abilityGrant it trample]` (`Cards/Cost.lean:687-688`).
- **Inkshield (`Cards/Damage.lean:462-469`, §1.5):** Prevent `some .combat` damage
  (not `none`), and give the created 2/1 white and black token the `inkling`
  subtype (`Definitions/Subtypes.lean:180`) and `flying`.
  **Done** (`kskwzvou`): `would any (dealDamage anyAmount you (some .combat))`, token
  `creatureToken 2 1 [.white, .black] [inkling] [flying]`. Pin
  `inkshieldPreventsCombatDamageForFlyingInklings` (`Proofs/Damage.lean`).
- **Mishra's Factory (`Cards/Mana.lean:141,145`, `[CR#205.3m]`, §1.6):** Use the
  catalog `assemblyWorker` (`"Assembly-Worker"`) subtype instead of the
  misspelled `"AssemblyWorker"`.
  **Reconciled 2026-10-24: done** by [lean-v3-known-subtype-check](../done/lean-v3-known-subtype-check.md):
  `Cards/Mana.lean:132,136` write `assemblyWorker` (`Definitions/Subtypes.lean:56`, "Assembly-Worker").
- **Performer handoffs on discard, unless-pays, and damage (§§1.10, 4.7):**
  - Wrap `"that player discards"` and `"each player/opponent discards"` in
    `instruct <player> (discard ...)` (`[CR#701.9a]`; `discard` reads the performer's own hand,
    `Definitions/Actions.lean:159-165`) on *Hypnotic Specter*
    (`hypnoticSpecterDiscard`, `Cards/Anaphora.lean:318`), *Immersturm Skullcairn*
    (`Cards/Damage.lean:45-46`), *Oona's Blackguard* (`Cards/Counters.lean:709`), *Sonic Shrieker*
    (`Cards/Damage.lean:892`), *Rankle* and *Braids's Frightful Return*
    (`Cards/Choice.lean:159,96-97`), *Liliana of the Veil* (`Cards/Piles.lean:190`),
    *Syphon Mind* (`Cards/Counters.lean:1167`), and *Rafter Demon* (`Cards/Keyword.lean:1009`).
    **Done** (`moukpvun`): each writes `instruct <player> (discard (… (cardIn hand)))`; the "each
    player/opponent" sites drop their `choose … , discard them` pair for `instruct each… (discard
    (a (cardIn hand)))`. Pin `discardIsHandedToTheDiscardingPlayer` (`Proofs/CardFidelity.lean`).
  - Fix `"unless its controller pays"` on *Mana Leak* (`Cards/Anaphora.lean:220-222`),
    *Override*, *Rakshasa's Disdain*, *Rune Snag* (`Cards/Choice.lean:886,892,322`),
    and *Tidal Flats* (`:917-922`) so the spell/ability's controller — not the
    invited payer — performs `counter it`, and generalize `payLife` to `Amount`
    so *Killing Wave*, *Heat Wave*, and *Font of Agonies* use `payLife` instead of
    `perform (loseLife ...)`.
    **Reconciled 2026-10-24: live** (partly: `payLife (amount : Amount := anyAmount)`
    (`Templates.lean:2047`) is done by [lean-v3-pay-life-action](../done/lean-v3-pay-life-action.md),
    and *Font of Agonies* uses it (`Cards/Counters.lean:855`). Live: the five counterspell sites
    still `instruct (controllerOf …) (doUnless (counter it) …)`; *Killing Wave*
    (`Cards/Choice.lean:904-906`) and *Heat Wave* (`Cards/Deontic.lean:637-638`) still write
    `loseLife`.)
    **Done** (`moukpvun`): the five sites hand the counter (Tidal Flats: the first-strike grant) back
    with `doUnless (instruct you …) cost`; *Killing Wave* and *Heat Wave* pay `payLife`. Pins
    `unlessPaysCountersByTheCounterspellsController`, `lifePaymentsArePayLife`.
  - Standardizing damage-source performer handoffs moved to
    [lean-v3-spell-damage-dealer](../done/lean-v3-spell-damage-dealer.md) (owner ruling 2026-10-21:
    "<this> deals" is a Handoff to the source on spells as on permanents).
    **Reconciled 2026-10-24: done** by [lean-v3-spell-damage-dealer](../done/lean-v3-spell-damage-dealer.md):
    spell damage is written `instruct this_ (dealDamage …)` (e.g. `Cards/Damage.lean:45,216`).
- **Phyrexian Infiltrator (`Cards/Anaphora.lean:74-77`, `[CR#701.12a,701.12b]`, §1.12):**
  Use the Definition `exchangeControl thisCreature (target isCreature)`
  (`Definitions/Actions.lean:304`; the raw form is `exchange (controlOf thisCreature (target
  isCreature))`, as *Avarice Totem*, `Cards/Keyword.lean:1596`) instead of
  `simultaneously [gainControl ..., gainControl ...]` (`Cards/Anaphora.lean:75-77`). Corrected
  2026-10-21: the earlier `exchange (controlOf …) (controlOf …)` was malformed, since `controlOf`
  takes both operands (review #4).
  **Done** (`kskwzvou`): `exchangeControl thisCreature (target isCreature)`. Pin
  `phyrexianInfiltratorExchangesControl` (`Proofs/Damage.lean`).
- **One-shot resolution `"cast it"` (`[CR#608.2g]`, §1.13):** Encode *Memory
  Plunder* (`Cards/Choice.lean:1011-1014`), *Flawless Forgery*
  (`Cards/Copy.lean:55-60`), and *Arcane Proxy* (`Cards/Faces.lean:667-675`) as
  one-shot optional casting during resolution (`may (cast ...)`) rather than a
  durationless static permission `establish (can you (cast ...))`.
  **Done** (`kskwzvou`): all three write `may (cast …)`. Pins `memoryPlunderCastsDuringResolution`,
  `copiedCardIsCastDuringResolution` (new `Proofs/CardFidelity.lean`; Arcane Proxy's line is the
  same tree as Flawless Forgery's).
  - **Madness (review #34):** the may-arm of `mayOrElse` in `Definitions/Keywords.lean:1068-1074`
    is the same durationless `establish (can theActor (cast thisCard …))`, which lasts until the
    game ends [CR#611.2a] with normal timing. [CR#702.35a]: its owner "may cast it … If that
    player doesn't, they put this card into their graveyard". Replace the arm with a direct
    `cast thisCard (from_ := some exileZone) (payment := .payingInstead cost)`; the graveyard
    fallback already exists. Same in `madness.ron` (`:28-29`).
    **Done** (`kskwzvou`): the may-arm is `cast thisCard (from_ := some exileZone) (payment :=
    .payingInstead cost)`; `madness.ron` writes `enact(cast, procedure(Cast, this,
    PayingInstead(Param(0))))`. Pin `madnessCastsDuringTheTriggersResolution`; rewrote
    `castPermissionsKeepPayingInstead` (`Proofs/KeywordCastPermissions.lean`), which pinned the
    old durationless permission, to the one-shot cast.
- **`"Return"` vs `"put"` (§3.4):** moved to
  [lean-v3-move-deeds-and-verbs](../done/lean-v3-move-deeds-and-verbs.md). The owner ruling of
  2026-10-21 removes `CoreDeed.return_` and `.put`, so the sites no longer "carry
  `.core .return_`"; they take the Template of their printed verb.
  **Reconciled 2026-10-24: done** by [lean-v3-move-deeds-and-verbs](../done/lean-v3-move-deeds-and-verbs.md):
  `CoreDeed` (`Words.lean:318-321`) has no `return_` or `put`; moves use `returnTo` /
  `returnToBattlefield` / `putOntoBattlefield*` (`Templates.lean:1132-1164`).
- **Restore dropped Oracle clauses (§1.14):** Complete the omitted clauses on
  *Qarsi Deceiver* (`Cards/Mana.lean:279-282`, manifest face-up cost), *Overgrown Zealot*
  (`:258-266`, manifest/cloak), *Candles' Glow* (`Cards/Damage.lean:451-459`, no Splice),
  *Giggling Skitterspike* (`Cards/Trigger.lean:219-220`, no "attacks"), *Cyclone*
  (`Cards/Counters.lean:526-532`, no "If you pay, … deals damage"), *Vedalken
  Squirrel-Whacker* (`Cards/Anaphora.lean:332-338`, `powerOf` not base P/T), and *Purging Scythe*
  (`Cards/Damage.lean:222-228`). Directions recorded 2026-10-21:
  - *Purging Scythe* (review #7): it deals 2 damage to `the` least-toughness creature and only
    then makes a `choose` nothing reads. The tie sentence modifies the earlier text
    [CR#608.2c,609.5]: choose among the least-toughness creatures first, then deal 2 damage to
    the chosen one.
  - *Overgrown Zealot* and *Qarsi Deceiver* (review #19): the spend purpose covers only morph and
    disguise costs (`CostNamed` is `containing | ofKeyword`, `Words.lean:533-535`). Turning a
    manifested or cloaked permanent face up is the Special Action of [CR#116.2b], "pay that cost,
    then turn the permanent face up" [CR#701.40b,701.58b] (Zealot ruling, 2024-09-20; Qarsi's
    Oracle text names manifested creatures). Add a spend purpose for the turn-face-up Special
    Action (neither `ofSpecialAction` nor `faceUpManaCost` exists), use it for Zealot, and add the
    manifest arm to Qarsi.
  **Done, partly** (`swyklnyl`): *Candles' Glow* adds `Ability.splice (hasSubtype arcane) [generic
  1, pip .white]`; *Giggling Skitterspike* adds `does thisCreature attack`; *Cyclone* is `may (pay
  …) (some <damage to each creature and player>) (some (sacrifice …))`; *Purging Scythe* chooses
  among the least-toughness creatures, then deals 2 damage to the chosen one. Pins (`Proofs/CardFidelity.lean`):
  `candlesGlowSplicesOntoArcane`, `gigglingSkitterspikeTriggersOnAttack`,
  `cycloneDealsDamageIfYouPay`, `purgingScytheChoosesAmongTheLeastThenDeals`. Retired
  `purgingScytheThemReadsTheTiedCreatures` (`Cards/Damage.lean`): it pinned "them" reading the
  tied creatures of the old encoding, which chose after dealing the damage.
  **Live** (STOPs, 2026-10-25 report): *Vedalken Squirrel-Whacker* needs a base power/toughness
  value to exchange with (no `Stat` or `ProjAxis` reads base P/T); *Overgrown Zealot* and *Qarsi
  Deceiver* need the turn-face-up spend purpose, a new `SpendPurpose`/`CostNamed` constructor whose
  shape the ticket leaves open.

### 4. Printed move text dropped by the move-deeds landing (wave review, 2026-10-21)

Reviews of [lean-v3-move-deeds-and-verbs](../done/lean-v3-move-deeds-and-verbs.md) found two kinds of
printed text that the rewritten move sites do not carry. Line numbers verified 2026-10-21
against the Lean source and the Oracle text in `data/derived/cards.jsonl`; re-located 2026-10-24.

- **"under its owner's control" is missing (`[CR#110.2a]`).** An effect that puts an object onto
  the battlefield puts it under the instructing player's control unless the effect states
  otherwise, so these three read as the caster (or ability controller) taking control. Write the
  rider as *Flickering Spirit* (`Cards/Anaphora.lean:38`) and *Another Round* (`:257`) do,
  `[(under (ownerOf …))]`, with the referent each card names:
  - *Turn to Mist* `Cards/Turn.lean:34`: `returnToBattlefield (that card)`. Oracle: "Return that
    card to the battlefield under its owner's control at the beginning of the next end step."
  - *Voyager Staff* `Cards/Turn.lean:41`: "Return the exiled card to the battlefield under its
    owner's control at the beginning of the next end step."
  - *Graceful Reprieve* `Cards/Trigger.lean:19`: "return that card to the battlefield under its
    owner's control."
  A sweep of the authored cards whose Oracle text prints "under its/their owner's control" found
  no others (*Daydream*, *Infernal Vessel* and *Yedora, Grave Gardener* already write it).
  **Done** (`rytuvxmt`): all three write `[(under (ownerOf it))]`. Pin `returnsUnderTheOwnersControl`
  (`Proofs/CardFidelity.lean`).
- **"from your graveyard" (or "from a graveyard") is not the move's origin.** The sites below
  print the origin in the move clause and write only a target restriction (`in_ yourGraveyard`)
  or nothing at all. A move written without an origin is observed on arrival, not on departure,
  so a "leaves a graveyard" trigger would not see it (`zoneChange` in `Templates.lean:1063` decides
  the Observation Point from the written origin, `[CR#603.6c,603.10a]`). Writing the origin makes
  these moves departure-observed, which is harmless for the effects themselves. Write the origin
  with the Template's own parameter (`returnToHand subject yourGraveyard`,
  `returnToBattlefield subject riders (from_ := some yourGraveyard)`; `Templates.lean:1572,1136`),
  as `dredge` and `recover` already do (`Definitions/Keywords.lean:547,1373`):
  - Origin missing on a "this card" or all-cards subject: *Hammer of Bogardan* `Cards/Turn.lean:78`
    (`returnToHand this_`), *Storm of Souls* `Cards/Keyword.lean:785`, *Eerie Ultimatum*
    `Cards/Static.lean:834-835`.
  - Origin written only as the target's `in_ yourGraveyard`: *Ever After* `Cards/Anaphora.lean:242`,
    *Rogues' Gallery* `Cards/Description.lean:337-338`, *Blood Spatter Analysis*
    `Cards/Counters.lean:1034`, *The Last Ronin* `Cards/Damage.lean:1456`.
  - Same pattern, found while verifying: *Bond of Revival* `Cards/Anaphora.lean:52`, *Midnight
    Scavengers* `Cards/Faces.lean:450-451`, and "from a graveyard" on *Rise from the Grave*
    `Cards/Anaphora.lean:235` and *Hymn of Rebirth* `Cards/Static.lean:38`. The last two use
    `putOntoBattlefieldUnderYourControl` (`Templates.lean:1158-1159`), which has no origin
    parameter; give the `putOntoBattlefield*` Templates (`Templates.lean:1148-1159`) the optional
    `from_` that `putOntoBattlefieldTappedAttacking` (`:1162-1164`) and
    `putOntoBattlefieldAttachedTo` (`:1151-1153`) already have.
  - Not in scope: restrictions that are the noun phrase's own modifier ("creature cards in your
    graveyard", as *Continue?*, *Faith's Reward*, *Death or Glory*) are not move origins.
  - **Ruling (owner, 2026-10-24): approved.** On "Return/put target [card] from your/a graveyard
    to …", keep `in_ <graveyard>` in the target's predicate, where it governs target legality
    [CR#115.1], and pass `from_ := some <graveyard>` on the move, where it governs the move's
    origin and departure observation point [CR#603.6c,603.10a]. The checker must accept both
    together; if it refuses the pair as redundant, that is a checker defect to fix, with a pin
    that the pair is accepted.
  **Done** (`rytuvxmt`): every listed site writes its origin (`returnToHand … yourGraveyard`,
  `returnToBattlefield … (from_ := some …)`, `putOntoBattlefieldUnderYourControl … (from_ := some
  graveyard)`), keeping the targets' `in_`; `putOntoBattlefield`, `putOntoBattlefieldTapped` and
  `putOntoBattlefieldUnderYourControl` gained the optional `from_`. The checker accepts the pair
  as is. Pins `graveyardOriginsAreDepartureObserved` (with `Action.check … = []` for the pair),
  `spelledGraveyardOriginsAreDepartureObserved`, and the arrival pin
  `bareReturnToBattlefieldIsObservedOnArrival`.

Tests to add (in `Proofs/` beside the existing move pins): for each rider site, a pin that the
authored action's move carries `.under` the owner; for each origin site, a pin that the move is
departure-observed from the graveyard; and a refusal pin that a bare `returnToBattlefield` with
no rider and no origin stays observed on arrival. Update any pin that names the old tree.

### 5. Card witnesses that drop or misread clauses (mtg-rules review, 2026-10-21)

Each site below was re-verified at head on 2026-10-21 and re-located 2026-10-24. Under
[CR#608.2c] the whole text governs.
Where a done ticket recorded the current shape as intended, this section supersedes that record.

- *Darksteel Garrison* (`Cards/Cost.lean:799`, review #6): "Whenever fortified land becomes
  tapped" is `does any (tapForMana (fortified land))`. A "becomes tapped" trigger fires on the
  change from untapped to tapped [CR#603.2e], not only for mana (ruling 2007-05-01). Write `does
  (fortified land) (become tapped)` (`Status.become`, `Templates.lean:1054`, as *Gideon's Avenger*,
  `Cards/Counters.lean:113`).
  **Done** (`ztopqskq`): `whenever (does (fortified land) (become tapped))`. Pin
  `darksteelGarrisonWatchesBecomingTapped`.
- *Lightmine Field* (`Cards/Damage.lean:257`, review #10): the amount is `numberOf (those
  creature)`, the triggering group. The Oracle text counts "attacking creatures", determined as the
  effect applies [CR#608.2h]; a creature removed from combat isn't attacking [CR#506.4] (rulings
  2010-06-15). Write `dealDamage (countOf isAttacking) (eachOf (those creature))`. This supersedes
  `docs/tickets/done/workbench-bare-plural-det.md:77-80`'s re-spelling as "the number of those
  creatures".
  **Done** (`ztopqskq`): `dealDamage (countOf isAttacking) (eachOf (those creature))`. Pin
  `lightmineFieldCountsAttackingCreaturesOnResolution`, on the resolution-time count itself.
- *Inkshield* (review #11, §3 above; `Cards/Damage.lean:464,468`): the replaced event `would any
  (dealDamage anyAmount you)` needs `(some .combat)` (`lean-v3-damage-composition` retired
  `damageRule`), and the token is
  `creatureToken 2 1 [.white, .black] [inkling]` with `(abilities := [flying])`.
  **Done** (`kskwzvou`), with §3's Inkshield entry.
- *Temple Altisaur* (`Cards/Damage.lean:605`, review #11): the recipient lacks "another".
  Write it with the bare "another" (`isOther`; ADR [semantics-v3](../../decisions/semantics-v3.md)
  §0.1 resolves the Contrast Anchor to the source).
  **Done** (`ztopqskq`): the recipient is `a (and [dinosaur, youControl, isOther])`. Pin
  `templeAltisaurProtectsAnotherDinosaur`.
- *Aether Gust* (`Cards/Choice.lean:980`, review #11): `and [isPermanent, hasColor .red]` drops
  "or green" and "spell or". Write `and [or [isSpell, isPermanent], or [hasColor .red, hasColor
  .green]]`. If the spell-or-permanent subject is still refused (the documented
  `badSpellOrPermanentSubject` gap), keep "or green" and leave that half to its gap.
  **Done** (`ztopqskq`): the full predicate checks; no gap left. Pin
  `aetherGustTargetsRedOrGreenSpellOrPermanent`.
- *Quakebringer* (`Cards/Damage.lean:1025-1031`, review #12): "This ability triggers only if …" is
  passed to `at_`'s `if_`, the intervening "if" [CR#603.4], which is rechecked on resolution, so the
  ability does nothing if the Giant dies in response. [CR#603.4] "only applies to an 'if' that
  immediately follows a trigger condition". Use the `qualified` guard (`Templates.lean:1887-1890`,
  doc `Abilities.lean:1399-1405`), the carrier `solved` uses for [CR#702.169c]. `qualified`'s `from_`
  holds one zone; the battlefield-or-graveyard functioning may need the guard alone. Supersedes the
  note in `docs/tickets/done/workbench-coordination-1-condition-disjunction.md:126-129`.
  **Done** (`olrrzkqs`): `qualified (at_ …) (guard := some <battlefield, or graveyard with a
  Giant>)`, the guard alone covering the two functioning zones. Pin
  `quakebringerRestrictsTriggeringOnly`, on the timing distinction (no intervening "if").
- *Keeper of the Flame* (`Cards/Damage.lean:1431-1433`, review #14): the life comparison sits in
  the target's description, so it is rechecked on resolution [CR#608.2b]; the ruling (2009-10-01)
  needs it true only "as you activate the ability". Either make it an activation-time condition
  checked only at [CR#601.2c,602.2b] and excluded from the [CR#608.2b] recheck, or write `target
  isOpponent` plus an activation-time legality condition. Update the `when` rider's docstring
  (now the `Action.choose` docstring, `Abilities.lean:1238-1239`; the rider is the `Timing`
  slot, `chooseWhile`, `Templates.lean:1191-1192`) and the pin `okChoiceWithActivationRider`
  (`Proofs/Choice.lean:544-549`), whose docstring says "the rider times the announcement, not the
  comparison".
  **Live** (STOP, 2026-10-25 report): either option needs a construct the AST lacks, a comparison
  read only at the announcement and excluded from the [CR#608.2b] recheck.
- *Cult of the Waxing Moon* (`Cards/Faces.lean:417-419`, review #17): `whenever (does any (turnOver
  (a (and [isPermanent, youControl])))) …` has no deed and no "non-Human creature" complement. The
  ability triggers if the object transforms or converts and "has the specified characteristic
  immediately after" [CR#701.27e]. Write it with `.enact (.action "transform") (.turnOver …) (some
  (and [isCreature, not (hasSubtype human)]))` and match converts per
  [lean-v3-keyword-triggers-entries-and-parties](../done/lean-v3-keyword-triggers-entries-and-parties.md)
  §9. `docs/tickets/done/workbench-transform-tail.md:56` claims the complement landed; head lacks
  it.
  **Done** (`olrrzkqs`): `does any (transform (a (and [isPermanent, youControl])) (into := some (and
  [isCreature, not (hasSubtype human)])))`. Pin
  `cultOfTheWaxingMoonWatchesTransformingIntoANonHumanCreature`.
- *Archfiend's Vessel* (`Cards/Keyword.lean:1122-1123`, review #18): "exile it. If you do, create
  …" is `triggerReflexively`. Reflexive triggers are only "when [a player] [does or doesn't]" or
  "when [something happens] this way" [CR#603.12]; "If you do" is followed in order [CR#608.2c].
  Write `doIfDone (exile it) (some (createToken …))` (`Templates.lean:1504-1506`).
  **Done** (`ztopqskq`): `doIfDone (exile it) (some <| createToken …)`. Pin
  `archfiendsVesselIfYouDoIsNoReflexiveTrigger`.
- *Memoricide* (`Cards/Choice.lean:964-968`, review #47): the count is fixed (`counted anyNumber
  …`), but "exile them" is missing before the shuffle. Add it. (The Idris witness is
  deletion-bound; leave it.)
  **Done** (`ztopqskq`): `exile them` before the shuffle. Pin `memoricideExilesWhatItFinds`.
- Ties: *Padeem* (`Cards/Mana.lean:880-886`), *Seraphic Greatsword* (`Cards/Keyword.lean:1580`),
  *Undercover Butler* (`Cards/Deontic.lean:225`) and the `dethrone` Definition
  (`Definitions/Keywords.lean:494-501`) all write "the X with the most … or tied for the most" as a
  singular `the (and [… superlative .max …])`, which drops the tie (review #48). "The Magic game
  has no default for ties" [CR#609.5]; "or tied for" is membership in the tied set
  ([CR#702.105a]: "the player with the most life or tied for most life"). Encode membership ("you
  control an artifact satisfying `superlative .max manaValue`", "attacks a player satisfying
  `superlative .max lifeTotal`"). Keep `the` for definite superlatives that print their own tie
  sentence.
  **Done** (`olrrzkqs`): Padeem is `exists_ (and [isArtifact, youControl, superlative .max …])`;
  dethrone, Seraphic Greatsword and Undercover Butler attack `a (and [… superlative .max lifeTotal
  …])`. Pin `orTiedForTheMostIsMembership`. Not done: `dethrone.ron` (v2, deletion-bound, not
  named by the item).

Tests: a pin per site on the corrected tree; for *Lightmine Field*, *Quakebringer* and *Keeper of
the Flame*, a pin on the timing distinction itself.

## Verification

- `./lean/scripts/build` (`lake build --wfail`) passes with zero warnings and
  zero `sorry`s.
- `cargo xtask lean check` (RON mirrors change); `cargo test -p xtask -p deckmaste_semantics_v2` and
  `cargo clippy -p xtask -p deckmaste_semantics_v2 --tests -- -D warnings` pass.
- `cargo xtask cite check --list-noncompliant` is empty and
  `cargo xtask cite check` reports 0 stale citations.
