---
needs: [workbench-bare-plural-det, workbench-binding-regressions, workbench-prevention-shield]
---
**Fold the constructor pairs that differ by one axis already present as a
value, retire the single-inhabitant sorts and dead surface, and add the
missing macros for rules-meaningful printed shapes.** Cleanroom review
2026-09-03, F10 + F13 + F17 + F20, and audit-2 N7. Each fold is a
one-line-per-site bench migration; they are independent and can land in any
order.

## F10 — one-axis pairs

- `Phrase.idr:274–277` `Monocolored | Multicolored | ExactlyColors n` (`colorCountOk n = 2..5`, `Words.idr:591`; probe `ExactlyColors 1` → `So False`) → `ColorCount (r : Comparator) (n : Nat)`.
- `Phrase.idr:263` `Unblocked` — `And [creature, Not Blocked]` and `And [creature, Unblocked]` both typecheck; `combatRoleClashOf` (`:941–957`) re-implements `noNegatedPair` for this pair. Drop both.
- `Phrase.idr:252–254` `CastBy n | NthCastBy ord n per` → `CastBy n (Maybe (Ordinal, RankPeriod))`.
- `Phrase.idr:280–286` `HasDesignation d {HeldBy k}` / `HasCardDesignation d {HeldByCard}` → one, `HeldByCard` giving kind `Object`.
- `Phrase.idr:1497–1501` `PossessorOf ax n {one}` / `PossessorsOf ax grp` (probe: `PossessorOf ControllerAx (each creature)` → `ManyOf = OneOf`) → one constructor, plurality = `nounPlur n`; `nounDelta` (`:1630–1633`) already computes both from the same shape (ADR §4).
- `Phrase.idr:1483–1484` `TheRest` / `TheOther` → `TheRest (pl : Plurality)`; `theOtherOk = theRestOk && size = parts+1` (`Words.idr:1433`).
- `Phrase.idr:1458–1459` `TheGrantor | TheEmblemGrantor` → one, marker-worded or zone-indexed.
- `Phrase.idr:1842–1885` six `Amount`s each gated `countOutcomes s bs = 1` on one `OutcomeSort` → `TheOutcome (s : OutcomeSort) {ok}`; `ThatMuch` (`:1838`) stays as the sort-blind read.
- `Phrase.idr:1835–1837` `Times (per : Nat) a` / `TimesOf (per : Amount) a` → `TimesOf (Lit per) a` (`forEachAmount`, `Effect.idr:906`, already treats them identically).
- `Phrase.idr:1829`/`:1863` `EventCount` / `EventSum` — identical rows plus one `So (eventHasMagnitude ev)` → one constructor with a `Count | Sum` axis; the `happened*`/`eventCount*` macro family (`Macros.idr:2302–2398`) halves.
- `Effect.idr:1233–1237` `Enact v e` / `Does subj v e` → `Enact (subj : Maybe (Noun bs Player)) v e` (ADR §7: the performer is an optional positional slot); removes `doesEffIntro`/`doesPreIntro`/`doesRiderIntro`/`doesAnnIntro`. Land `workbench-binding-regressions` first — its distributive fix is on those clauses.
- `Effect.idr:1127–1137` `CopyStack` / `CopyCard` → one `Copy` over a `Copiable`-or-card-zone gate (`CopyCard` drops `exc` for no evident rules reason).
- `Effect.idr:412–430` `Prevents` / `Redirects` / `Scales` share `(kind, src, scope, use)` → `DamageRule kind src scope (op : DamageOp) use`, `DamageOp = Prevent cut also | Redirect to | Scale op`; `Scales.src : Noun bs Object` looks accidental. Sequence after `workbench-prevention-shield`, which adds the `Shield` cut.
- `Effect.idr:330–339` `Gets` / `HasBasePt` → the `Adds | Sets` axis `Becomes` already takes (`CharOp`, `:185`); keep the [CR#613.4b] / [CR#613.4c] layer distinction as the op value.
- `Triggers.idr:219` `LastCounterRemoved kind n by` vs `:228` `CounterEvent CounterTaken …` — "the last" is a `CounterBatch`-like axis; `Triggers.idr:278` `UnlocksDoor who door` is already named `VerbedAct "Unlock"` by `eventName` (`:337`) yet is a separate constructor from `VerbedEvent`.

## N7 — `SharedSubject`

`Effect.StaticEffect.SharedSubject n parts` and `AndAlso parts` differ by one
slot, with pairwise identical `staticIntro`/`isCoord`/`staticKind`/
`staticChoiceDelta`/`clauseStaticOk` clauses (`Effect.idr:701, 763, 804, 811,
2806`). Fold to `AndAlso (subject : Maybe (Noun bs Object)) parts` with
`partsIntro` over `maybe bs selfSubjIntro subject` — the same criterion as
every pair above — **if it typechecks**; if the subject-introducing binding
scope genuinely refuses the fold, keep two rows and record the refusal on the
declaration.

## F13 — single inhabitants, dead axes, closed surface labels

`Words.Causer = AnEffect` (`Words.idr:616`), `LoseCause = ZeroOrLessLife`
(`:1636`; [CR#704.5a..704.5c] names three loss causes, only the first is
printed as an immunity), `TurnPoint = AttackersDeclared` (`:3825`) — retire as
sorts. (`Phrase.Ascribable`, `Phrase.idr:2846`, is folded by
`workbench-zone-gate-unify`; leave it.) `VerbedMarking` (`:1753`) is carried
by `Reach.Verbed` but no gate consults it — `verbedMarkingOk v _ = isJust
(participleOf v)` (`:1757`) and `reaches (Verbed v w _)` (`:2082`) both ignore
it: keep the surface data, drop or rename `VerbedMarkingOk` (it is
`actNamesParticiple`). `AbilityWordName` (`:2607–2619`) is a closed 60-value
enum while `FlavorWordLabel = String` (`:2623`) is open; ability words have no
rules meaning [CR#207.2c], so this is the open-label case exactly — `ItalicWord
= AnAbilityWord String | AFlavorWord String`. `spellType n = MkSubtype Instant
n` (`:2845`) hardwires one host for a class that hosts two [CR#205.3k], which
`subsFitLine` (`:3502–3508`) then special-cases.

## F17 / F20 — missing macros, dead surface

Missing macros (the compositions typecheck, so these are macros, not
constructors): `exchangeControl a b` — Avarice Totem, probed as
`Simultaneously [gainControl (controllerOf (target (And [Permanent, Not
land]))) thisArtifact Nothing, gainControl (controllerOf thisArtifact) (That
PermanentW) Nothing]`; `fateseal` — `LookReq` (`Macros.idr:1470–1527`) admits
only the agent's own library, so widen it; `amass`, hand-expanded as a
five-step `Sequentially` at `Cards.idr:308–317`; `explore`, `investigate`,
`populate` (`TokenCopyOf`, `Effect.idr:256`, covers populate).

Dead surface to delete: `Effect.GameBecomes` (`Effect.idr:1100`), written by
no card, macro or pin; macros `ifSo` (`Macros.idr:711`) and `jointCard`
(`:257`), referenced nowhere. 275 mapped constructors appear in neither
`Cards.idr` nor `Macros.idr`; 158 of them are `Words` closed-enum members that
the `AbilityWordName` opening above removes. Use `cargo xtask map idris-dead`
to re-derive the list — an unexercised constructor is a prompt to look, never
by itself a reason to delete.

Size: L.

Also (audit N8): the remaining twins `Effect.LosesCounters` / `RemoveCountersAmong` and `PossessorOf` / `PossessorsOf` fold on the same rule as the pairs above.

Also (binding-regressions residue): `Effect.doesPreIntro`/`doesAnnIntro`/`doesRiderIntro` keep their `ManyOf` fall-throughs, so an `InsteadOf`/`ThisWay`/rider read of a distributive patient is still refused; lift them the way `doesEffIntro` was (`distributedDelta`) when folding `Does`/`Enact`.

Also (keyword-ability-body STOP residue): `Effect.costPaidByYou` conflates payer with agent (`Do (ChangeLife who _)` → `nounIsYou who`), so "Cumulative upkeep — an opponent gains 1 life" (Wall of Shards) cannot take the keyword body; separate payer from agent on the cost rows so an opponent-agent cost is paid by you.

Done when: the build is 23/23 with 0 errors and 0 warnings; each folded pair
is one constructor with the axis as a value, and every former site of the
deleted member is re-spelled and typechecks; `SharedSubject` is folded or its
refusal is recorded on `AndAlso`; the three single-inhabitant sorts,
`GameBecomes`, `ifSo` and `jointCard` are gone; `ItalicWord` carries an open
ability-word label and `spellType` admits both hosts; Avarice Totem, Spin into
Myth and an amass card are typechecking bench witnesses through their new
macros; every pin that named a deleted constructor is re-spelled and still
refutes for its named reason; the landing record gives the constructor count
before and after. Standard constraints apply.

## As landed

### F10 — one-axis pairs

- `Monocolored | Multicolored | ExactlyColors n` → `Phrase.ColorCount (r : Comparator) (n : Nat)`
  gated `So (colorBoundOk r n)` (`Words.idr`, `[CR#105.1]`: an object is 0..5 colors, `Eq 0`
  is `IsColorless`). Macros `monocolored = ColorCount Eq 1` and `multicolored =
  ColorCount AtLeast 2`; 3 bench sites re-spelled through them, one `ExactlyColors 2`
  site reads `ColorCount Eq 2`. Pin `ProofsG.badExactlyOneColor` **inverted** — exactly
  one colour is now the printed `monocolored`, so it is re-spelled as the positive
  witness `monocoloredIsOneColor` and a new pin `badExactlyZeroColors` carries the
  `n >= 1` half; `badExactlySixColors` re-spelled through `ColorCount Eq 6`.
- `Unblocked` dropped with `combatRoleClashOf`/`anyCombatRoleClash`/`noCombatRoleClash`
  and their `contradictionFree` conjunct; `Macros.unblocked = Not Blocked` is the
  printed lemma. `ProofsB.badBlockedAndUnblocked` re-spelled through it (now refused by
  `noNegatedPair`, the general rule, instead of the one-pair clash table).
- `CastBy n | NthCastBy ord n per` → `CastBy n (rank : Maybe (Ordinal, RankPeriod))`;
  `uniquifies (CastBy _ rank) = isJust rank` replaces the `NthCastBy` row. Macros
  `castBy` / `nthCastBy`; 29 bench + 1 proof site re-spelled.
- `HasDesignation d {HeldBy k}` / `HasCardDesignation d {HeldByCard}` → one
  `HasDesignation d {designationHolder d = Just k}` (`Words.designationHolder` maps
  `HeldByCard` to `Object` and `HeldByGame` to `Nothing`); `hasHead` now reads
  `Words.heldByItsCard d`. `ProofsE.badObjectMonarch` keeps its spelling and its
  `Refl impossible`. The `So (designationChecked d)` gate is gone from this
  constructor — see Deviations.
- `TheRest` / `TheOther` → `TheRest (pl : Plurality)` gated `So (theRestFits pl bs)`
  (`theRestFits OneOf = theOtherOk`, `ManyOf = theRestOk`). Macros `theRest` / `theOther`; 33 sites
  across the bench and the proof modules re-spelled. The two `Words` tables are untouched, so the
  three `theOtherOk`/`theRestOk` table assertions in `Cards.idr` stand unchanged.
- `TheGrantor | TheEmblemGrantor` → `TheGrantor (m : MarkerWord)`; the zone is
  `markerZone m` and the stamp origin `Words.grantorOrigin m`. Sites read
  `TheGrantor PermanentMarker` (Leonin Bola's unattach) and `TheGrantor EmblemMarker`
  (`ProofsG.emblemGrantorRead`).
- `PreventedThisWay | RemovedThisWay | TheResult | TheTotal | ShortOfCeiling` →
  `TheOutcome (s : OutcomeSort) {countOutcomes s bs = 1}`; `readAmount` dispatches
  through `Words.outcomeComparable`. `ThatMuch` stays. Macros `preventedThisWay`,
  `removedThisWay`, `shortOfCeiling` join the existing `theResult` / `theTotal`
  (which now expand to the same term — the printed word is the only difference).
  17 bench and 9 proof sites re-spelled.
- **Undone: `TheDifference`.** The ticket groups it with the five above, but its gate is
  `countOnes Gap bs = 1` over a `Kind` binding, not `countOutcomes s bs` over an
  `OutcomeSort` — there is no `Gap` outcome sort. It is not the same axis and folding it
  would need a second index on `TheOutcome`; left as its own constructor.
- `Times (per : Nat) a` / `TimesOf (per : Amount) a` → `TimesOf` alone, which gains
  `So (amtNonZero per)` so the literal-zero multiplier `Times 0` stays refused;
  `Macros.times per a = TimesOf (Lit per) a` and `nForEach`'s gate moved from
  `IsSucc n` to the same `So`. `Proofs.badForEachZero` re-spelled (`ItIsSucc` → `Oh`).
  35 bench sites re-spelled.
- `EventCount` / `EventSum` → `EventTally (op : TallyOp) …` with
  `Events.TallyOp = TallyCount | TallySum` and the magnitude gate lifted to
  `So (tallyOk op ev)`. `Macros.eventSum` added beside `eventCount`;
  `ProofsG.badRollAsMagnitude` keeps its `{qm = ok}` spelling.
- `Enact v e` / `Does subj v e` → `Enact (subj : Maybe (Noun bs Player)) v e` with
  `e : Effect (agentCtx subj)` (`Phrase.agentCtx`, the `Nothing`/`Just` split). The four
  `does*Intro` helper tables stay (they are the `Just` branch), and `costPaidByYou`
  reads `Do (Enact (Just subj) _ _)`. 20 macro sites and 4 proof/bench sites re-spelled.
- `CopyStack` / `CopyCard` → `Copy (src : CopySort) agent what times exc` with
  `Words.CopySort = FromStack | FromCardZone`, `CopySourceOk src what` (a type-level
  function: `Copiable what` on the stack, `So (isCardZone (nounZone what))` in a card
  zone) and `Words.copyPayloadIn`/`copyLandsIn` carrying the zone. `CopyCard` gains the
  `exc` list it was missing. `ProofsE.badCopyPermanent` keeps `SpellCopied impossible`.
- `Prevents | Redirects | Scales` → `DamageRule kind src scope (op : DamageOp …) use`
  with `DamageOp = Prevent cut also | Redirect cut to | Scale sc`, `damageOpKind`
  (Prevention vs Replacement), `damageOpIntro` and `So (damageOpUseOk op use)` carrying
  the `[CR#615.7]` shield gate. `Scales.src` is now a `DamageAgent` like the other two.
  47 bench sites, 3 macros and 9 `ProofsF` pins re-spelled (all nine still refuse).
- `Gets` / `HasBasePt` → `Gets (op : CharOp) n pow tou` with
  `So (ptOpOk op pow tou)` — `Sets` takes only unsigned shifts, `Loses` is refused
  ([CR#613.4b] sets, [CR#613.4c] modifies) — and `staticKind` reading `ptOpKind op`.
  `Macros.hasBasePt` wraps the two `Amount`s in `PtUp`; the 105 raw `Gets` bench/proof
  sites read `Gets Adds`. New pins `ProofsG.badSetBasePtDownward` and `badLosePtOp`.
- `LastCounterRemoved kind n by` → `CounterEvent CounterTaken (Just kind) n LastCounter
  by False` — `Events.CounterBatch` gains `LastCounter`, `counterEventName` takes the
  batch, and `counterBatchOk` keeps the batch to a named-kind removal. Macros
  `lastCounterRemoved` / `lastCounterRemovedBy`; `ProofsD.badLastPoisonCounterRemoved`,
  `badExileCheckOnSortedSelf`, `ProofsE.badAnnouncingRemovalAgent` and Veiling Oddity
  re-spelled. New pin `ProofsG.badLastCounterOnPlacement`.
- **Undone: `UnlocksDoor` vs `VerbedEvent`.** `eventName` already names it
  `VerbedAct "Unlock"`, but its patient is a `Door bs`, not the `Maybe (Noun bs k)` that
  `VerbedEvent` carries — the two differ in the patient's *type*, not in one value, so
  the fold needs a `Door`-admitting complement on `VerbedEvent`. That is a design change,
  not a re-spelling; recorded rather than forced.

### N7 — `SharedSubject`

- Folded. `AndAlso (subject : Maybe (Noun bs Object)) parts` with
  `parts : StaticParts n (subjCtx subject)` (`Phrase.subjCtx`, the `selfSubjIntro`
  analogue of `agentCtx`). It typechecks; the five pairwise-identical clauses
  (`isCoord`, `staticKind`, `staticIntro`, `staticChoiceDelta`, `clauseStaticOk`)
  became one each. `Macros.sharedSubject` and the two `ProofsAnaphora` pins keep their
  spellings through `AndAlso (Just n)`.

### F13 — single inhabitants, dead axes, closed surface labels

- `Words.Causer` retired: `Maybe Causer` was only ever consulted as `isJust`, so the
  slot on `CounterEvent` and `TokensCreated` is now `(byEffect : Bool)`, and
  `creationVoiceOk` / `causedByOk` take the `Bool`.
- `Words.LoseCause` retired: `NoLossFrom who ZeroOrLessLife` → `NoLossFromZeroLife who`,
  the printed immunity ([CR#704.5a]). Phyrexian Unlife re-spelled.
- `Words.TurnPoint` retired: `BeforePoint AttackersDeclared w` →
  `BeforeAttackersDeclared w`, and `pointWindowOk`/`PointWindowOk` lose the dead index.
- `VerbedMarkingOk` deleted: it ignored the marking and was character-for-character
  `actNamesParticiple`, which already existed. `Words.ActNamesParticiple` is the gate;
  the six `Macros` gates, two `ProofsAnaphora` signatures and
  `ProofsG.copyParticipleUnwritten` (now `actNamesParticiple "Copy" = False`) re-spelled.
  `VerbedMarking` itself stays as surface data, as the ticket asks.
- `AbilityWordName` (60 rows) deleted: `AnAbilityWord` now carries an open
  `AbilityWordLabel = String`, the same shape as `FlavorWordLabel`, because ability
  words have no rules meaning [CR#207.2c]. The 19 bench sites carry the printed words
  ("landfall", "council's dilemma", …).
- `spellType` no longer hardwires Instant: `Subtype` gains `MkSpellSubtype label`
  ([CR#205.3k], instants and sorceries share their spell types), `subtypeType` is now
  `Maybe CardType`, and the new `subtypeFits` replaces the `spellSubtype` special case
  in `subsFitLine` and `addedFits`. `Phrase.seedTy`/`seedType`/`seedTypeAlts` and
  `ChoiceDomain.TypeOtherThan` re-spelled; a spell subtype's type alternatives are
  `[Instant, Sorcery]`.

### F17 — missing macros

- `Macros.exchangeControlOfThis t other` — the `Simultaneously` pair the cleanroom
  probed, with the two anaphora obligations forwarded. Bench witness
  `Cards.avariceTotemExchange` (Avarice Totem, Vintage-supported).
- `Macros.fateseal amt` [CR#701.29a] — `LookReq` already admitted a non-`You` agent
  (`TheirOneLookReq`/`TheirManyLookReq`), so no widening was needed; the macro is
  `scryTheirMany anOpponent`. Bench witness `Cards.spinIntoMyth` (Spin into Myth).
- `Macros.amass sub n` [CR#701.47a] — the five-step `Sequentially` lifted out of
  `Cards.amassZombiesTwo`, which now reads `Macros.amass "Zombie" 2`.
- **Undone: `explore`, `investigate`, `populate`.** Each needs surface the workbench
  does not yet spell — `explore` a reveal-and-branch chain, `investigate` a token
  carrying an activated ability (no Clue spelling exists in the bench), `populate`
  a choose-then-copy whose `TokenCopyOf` read of the chosen token does not resolve
  (`countReach Bare OneOf (effIntro (choose …)) = 1` fails). These are new spellings,
  not macros over existing ones; recorded rather than forced.

### F20 — dead surface

- **Undone, deliberately: `Effect.GameBecomes`.** It is not dead surface: 83
  Vintage-supported cards print "it becomes day"/"it becomes night" outside reminder
  text, and it is the only spelling for them. Kept, and given the bench witness it was
  missing — `Cards.tovolarNightfall` (Tovolar, Dire Overlord).
- **Undone: `Macros.ifSo` and `Macros.jointCard`.** Both are referenced —
  `ifSo` by `ProofsG.nestedStaticConditionals`, `jointCard` by
  `ProofsF.jointCrossAbilityChoice` — so the premise "referenced nowhere" is false.
  `ifSo` is also the one macro for the `IfSo` connective in a family where
  `asLongAs`/`unlessSo`/`onlyIfSo` each have one, and `jointCard` is the only spelling
  for a joint-choice card. Deleting either would cost a witness.

### Routed residues

- **Distributive intro fall-throughs (binding-regressions residue).** `doesPreIntro`,
  `doesRiderIntro` and `doesAnnIntro` each gain the `ManyOf` `Move` clause that
  `doesEffIntro` got, lifted through `distributedDelta`; the `ManyOf` fall-throughs are
  unchanged, as in that round. Witnesses
  `ProofsAnaphora.distributedDeedReadsBackPluralUnderAnnouncement`,
  `…UnderCondition` and `distributedDeedRiderReadsBackPlural`; pin
  `badDistributedAnnouncedDiscardSingular`.
- **Payer vs agent on cost rows (keyword-ability-body residue).**
  `costPaidByYou (Do (ChangeLife who _)) = nounIsYou who` splits on the direction:
  a life *payment* comes out of the payer's own total [CR#119.4] and still demands
  `nounIsYou`, while *granting* life is an action the payer takes and its recipient is
  free. Wall of Shards now takes the keyword body
  (`Macros.cumulativeUpkeep (Do (gainsLife anOpponent (Lit 1)))`), and the new pin
  `ProofsG.badOpponentPaysYourCost` keeps "an opponent loses 1 life" refused.
- **Counter twins (audit N8) — undone.** `LosesCounters` / `RemoveCountersAmong` are
  not one axis. `RemoveCountersAmong` pools one quantity across a `PartitiveBase`
  where `RemoveCounters` applies per member; `LosesCounters` is the player-possessive
  intransitive with a different announcement (`EncAgentless`, and no
  `outcomeB CountersRemoved` in `effIntro`) and a plain `Maybe Amount` instead of a
  `Quantity`. A fold needs a pooled/per-member axis plus two conditional gates —
  a design decision, not a re-spelling.
- **`PossessorOf` / `PossessorsOf` — not touched**, per the round brief: the sibling
  `workbench-possessor-noun` owns them.

## Landing record

Constructor counts (`cargo xtask map idris`), before → after:

- `Words` 103 types / 446 constructors → 100 / 385
- `Events` 19 / 105 → 20 / 108
- `Phrase` 38 / 239 → 38 / 226
- `Triggers` 19 / 80 → 19 / 79
- `Effect` 50 / 271 → 51 / 268
- `Card` 9 / 23 → 9 / 23
- total 238 types / 1164 constructors → 237 / 1089

Gates (all foreground, last line of each):

- `cd idris && rm -rf build && ./scripts/build` → `23/23: Building Cards (src/Cards.idr)`;
  23 modules, 0 `Error` and 0 `Warning` lines.
- `cargo xtask cite check --list-noncompliant` → `0 non-compliant citation-looking string(s)`.
- `cargo xtask cite check` → `checked 17749 citations against cr.txt (eff. 2026-08-07); 0 stale`.
- `cargo xtask cite bless` → `blessed 1571 rules at cr_date 2026-08-07`; `cr-citations.lock`
  is byte-identical (every cited rule was already registered), so the file is untouched.
- `jj --no-pager diff --git > /tmp/ap.diff && cargo xtask cite audit --diff < /tmp/ap.diff` →
  `audited 9 citation site(s)`. Each rule was read against its claim; two first drafts
  were wrong-number-right-topic ([CR#701.28a] is Convert, [CR#701.44a] is Explore) and
  were corrected to [CR#701.29a] (fateseal) and [CR#701.47a] (amass) before landing.

Pin probes (positive twin checked in a scratch module, then the module deleted; each pin
also mis-stated once and the message read):

- `monocoloredIsOneColor` is the in-tree positive twin for `badExactlyZeroColors` /
  `badExactlySixColors`.
- `badSetBasePtDownward` / `badLosePtOp`: twin `Gets Sets thisCreature (PtUp 1) (PtUp 1)`
  typechecks; both pins are accepted as `impossible` by the build.
- `badLastCounterOnPlacement`: twin `Macros.lastCounterRemoved Time thisEnchantment`
  typechecks.
- `badOpponentPaysYourCost`: the positive form (an opponent *gains* 1 life) is Wall of
  Shards on the bench; the refused form reports `Can't find an implementation for So False`.
- The three distributive-intro witnesses were each re-checked with the new `ManyOf`
  `Move` clause reverted: `annIntro` →
  `Can't find an implementation for countReach (Word CardW) ManyOf (annIntro (discard (each Opponent) (a (InZone handZ)))) = 1`,
  `preIntro` → the same over `preIntro`, `riderIntro` →
  `… countReach (Verbed "Sacrifice" PermanentW Attributive) ManyOf (riderIntro (sacrifice (each Opponent) (a creature))) = 1`.
  All three lifts are non-vacuous.

Assurance counts: restored 0; **re-spelled** 24 pins/witnesses in `Proofs*` and ~250 bench
sites in `Cards.idr`; ignored with blockers 0; **added** 5 pins
(`badExactlyZeroColors`, `badSetBasePtDownward`, `badLosePtOp`,
`badLastCounterOnPlacement`, `badOpponentPaysYourCost`,
`badDistributedAnnouncedDiscardSingular` — 6, against `badExactlyOneColor` becoming a
witness), 4 proof witnesses and 3 bench witnesses (Avarice Totem, Spin into Myth,
Tovolar); **removed** 0.

Deviations and additions:

- `HasDesignation` lost its `So (designationChecked d)` gate. On the `HeldBy k` branch
  that gate was vacuous (`designationChecked` is `True` for every designation whose
  scope is `HeldBy _`, and `False` only for `CommanderD`, which `designationScope`
  already separates), and after the fold the constructor covers the card-held case that
  the row was written to exclude. The table itself is untouched and its three other
  consumers are unchanged.
- `nounProv TheOther` used to fall through to `Nothing` while `nounProv TheRest` gave
  `provOfGroup bs`. The fold keeps `provOfGroup bs` for both: "the other" reads the same
  group as "the rest", so the fall-through looked accidental (the `Scales.src` case).
- `Words.copyPayload` is now `copyPayloadIn ph ty (Just Stack)`; the generalised form is
  what lets one `Copy` carry both zones.
- `TheResult` and `TheTotal` now expand to the identical term `TheOutcome RollResult`;
  the printed distinction lives only in the two macros, as the cleanroom observed.
- `GameBecomes` was kept and witnessed rather than deleted (see F20 above) — a
  deliberate departure from the ticket's "Done when".
- `Macros.ifSo` and `Macros.jointCard` were kept (see F20 above) — likewise.
- New helpers beyond the ticket's letter: `Words.colorBoundOk`, `designationHolder`,
  `heldByItsCard`, `theRestFits`, `grantorOrigin`, `outcomeComparable`, `CopySort` /
  `copyLandsIn` / `copyPayloadIn`, `subtypeFits`, `ActNamesParticiple`;
  `Events.TallyOp` / `tallyOk` / `counterBatchOk`; `Phrase.amtNonZero`, `agentCtx`,
  `subjCtx`, `CopySourceOk`; `Effect.ptOpOk`, `ptOpKind`, `DamageOp` and its three
  projections. Each is the axis's projection, not new surface.
- No construction, witness, test or lock was removed, and no scratch module remains
  (`idris/src/Experimental/Scratch.idr` was used for every probe and deleted).

STOPs taken: none. Five items are recorded above as undone with their reasons
(`TheDifference`, `UnlocksDoor`, the N8 counter twins, `explore`/`investigate`/`populate`,
and the three F20 deletions that turned out to be live surface); none of them
contradicts a recorded ruling, so none required a STOP.
