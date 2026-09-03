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
