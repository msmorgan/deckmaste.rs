---
needs: [lean-v3-template-helpers-into-definitions]
---
# Counters, designations, relations and subtype frames are self-contained catalog items

Standard constraints apply. Lean first (`lean/Semantics/`). Part 2 of 5 of ruling #1 of the owner's
batch 4 (2026-10-22). Supersedes step 2 of lean-v3-facts-from-declarations (WIP, owned elsewhere; that ticket's step 1 still lands first).

## Ruling (owner, 2026-10-22, batch 4): the Facts tables are deleted

This replaces step 2 of `lean-v3-facts-from-declarations`' A/B; the owner chose neither.

> Make CardType and all catalog items self-contained `semantic_definition`s whose values carry
> their own fields and whose raw constructors are marked `[semantic_expression, internal_expansion]`.
> `Authoring.inspect` already rejects raw `[internal_expansion]` constructors in cards, so every
> `known*` table check is redundant with the authoring boundary.

The whole ruling is split across five tickets that land one at a time, in this order:
`lean-v3-template-helpers-into-definitions` (B1), `lean-v3-catalog-items-self-contained` (B2),
`lean-v3-keyword-facts-deleted` (B3), `lean-v3-action-facts-deleted` (B4),
`lean-v3-card-type-composite-definition` (B5, which also deletes `Check/Facts.lean` and
`Check/FactTypes.lean`). The order differs from the first suggested split (CardType first): see
B1 "Why first".

## The clauses this ticket carries

> (2) Eliminate all six tables in `Check/Facts.lean` ... :
> - **counterFacts:** `holder : Kind := .object` and `confers` go on `CounterKind` in
>   `Definitions/Counters.lean`, with an internal_expansion constructor; `c.scope` reads `c.holder`.
> - **designationTable:** `scope : DesignationScope` goes on `DesignationLabel` in
>   `Definitions/Designations.lean` (`zone`/`type`/`half` are unread); internal_expansion
>   constructor.
> - **relationTable:** `left`/`right` are unread; marking RelationLabel's constructor
>   internal_expansion makes an undeclared relation impossible.
> - **subtypeFacts:** either `frame : Option FrameFeature := none` on Subtype (saga, adventure,
>   room), or drop the frame-vs-subtype check as an authoring mistake, as in #2. Record this as an
>   open choice for the implementer, unless the code makes one option clearly right.

## Authoring boundary (VERIFY, 2026-10-22, paths under `lean/Semantics/`)

- `Authoring.inspect` (`Authoring.lean:427-433`, `inspectCore` `:345`) returns `.raw name` for any
  constructor of a `semantic_expression`-tagged inductive that is not `semantic_literal`
  (`Authoring.lean:394-398`); `Form.rawNames` collects them (`:55-60`). The `spelled` elaborator
  (`Check/Card.lean:411-431`, structure `Spelled` `:404`) refuses a card whose form has any:
  "Card definitions must use semantic templates or definitions; raw constructors: [...]". Pinned
  for `Ability.keyword` and `Subtype.of` / `Subtype.spell` in `Proofs/Authoring.lean:~101-125`.
- The rejection is driven by `semantic_expression`, not by `internal_expansion`. `[internal_expansion]`
  only suppresses the generated primitive template for the constructor (`declare_semantic_primitives`,
  `Authoring.lean:246-253`). So a type must carry **both** tags, as the ruling says.
- **Gap 1 (part of the work, not a STOP).** `CounterKind` (`Words.lean:566`), `DesignationLabel`
  (`:228`), `RelationLabel` (`:236`), `KeywordLabel` (`:258`), `AbilityWordLabel` (`:264`), `Deed`
  (`:299`) and `CardType` (`:26`) are **not** `semantic_expression` today: `Words.lean:637` tags only
  `Delta` and `Subtype`, and `classify_semantic_syntax` (`Abilities.lean:1363`) classifies only
  types declared in `Abilities.lean`. A raw `.named "typo"` therefore passes `spelled` today, and
  that is why the `known*` checks are load-bearing today. Each ticket tags its types.
- **Ruled (owner, 2026-10-23), STOP-1 accepted.** The authoring boundary (`Authoring.inspect` /
  `spelled`) covers only `Spelled` cards. Card fragments in `Cards/` and everything in `Proofs/`
  are fixtures and keep no `known*` check; a typo in a fixture surfaces when the full card is
  spelled. Original analysis: the boundary covers
  only terms run through `spelled <| …`: 835 `: Spelled` card definitions in `Cards/`. About 750
  more `Cards/` items are fragments (`def x : Ability := …; theorem … Ability.check … by decide`,
  e.g. `Cards/Keyword.lean:947-958`) and all of `Proofs/` call `Check` directly, so a typo'd label
  there is no longer refused once the `known*` checks go. The ruling says "in cards". Fragments
  and `Proofs/` are fixtures; no `known*` check is kept for them.

## Verified (2026-10-22, paths under `lean/Semantics/`)

- **Row counts** (`Check/Facts.lean`): `counterFacts` 218 rows (`:268`); `designationTable` 21
  (`:488`); `relationTable` 3 (`:511`); `subtypeFacts` 3 (`:516`: Saga, Adventure, Room). 214 of the
  218 counters hold on `.object`; `energy`, `experience`, `poison` and `radCounter` are `.player`.
- **`counterFacts` has no `confers`.** The row type `CounterFacts` (`Check/FactTypes.lean:46`)
  is `label` and `holder` only. Conferrals live in the separate `Definition.counter kind holder
  confers` (`Rules.lean:107`), read by `Definition.check` (`Check/Rules.lean:47-52`).
- **Ruled (owner, 2026-10-23), STOP-2 accepted, extended to subtypes.** Conferrals do not go on the
  value. `CounterKind` and `CardType` carry only non-recursive fields; their `confers` stay on the
  `Definition.*` rows in `Rules.lean`, as counters already do. For consistency subtypes do the
  same: whatever a subtype confers (Saga's chapters and lore rule, Siege's protector and
  last-counter trigger, and the rest) lives on a `Definition.subtype` row, not on the `Subtype`
  value. Verified how subtypes are encoded today: `Definition.subtype (subtype : Subtype) (rules :
  List Conferral)` already exists (`Rules.lean:110`, checked at `Check/Rules.lean:51`), the
  `Subtype` value (`Words.lean:341-344`) carries no conferrals, and `Definitions/Subtypes.lean`
  holds only bare `semantic_definition x : Subtype := .of host "X"` entries with no
  `Definition.subtype` rows yet. So nothing moves; the Saga and Siege rules are written as
  `Definition.subtype` rows (by `lean-v3-card-type-composite-definition` for Siege and the
  card-type-adjacent ones). Do not add a `confers` field to `Subtype`. The owner considered moving
  `CardType` into the `Abilities.lean` mutual block and declined. Original analysis: the ruling puts
  `confers` (a `List Conferral`) on `CounterKind`. `Conferral` (`Rules.lean:51`) holds `Ability`,
  `StaticSpec`, `Condition` and `Action` (`Abilities.lean`), and those hold `CounterKind`
  (`Abilities.lean:612-620`, `CounterKindSource.printed`; `ProjAxis.counter`, `Words.lean:573`),
  `CardType` and `Subtype` from `Words.lean`, which `Abilities.lean` imports. A `CounterKind`
  declared in `Words.lean` cannot hold a `Conferral` declared after `Abilities.lean`; putting it
  in one is one giant mutual block with `Ability`. **Chosen:** the value carries only the
  non-recursive columns (`holder` here), and `confers` stays on the separate `Definition` record
  keyed by the value, as it is today (the checker never reads `confers` from the counter table, so
  nothing the checker needs is lost). The rest of this ticket proceeds with `holder`.
- **Readers of the counter table**: `counterFactsFor`/`knownCounter`/`CounterKind.scope`
  (`Check/Words.lean:1763-1769`), `CounterKind.known` (`Check/Keywords.lean:137`), `ProjAxis.scope`
  (`Check/Words.lean:1772`), `LocalShape.lean:90,93`, `Check/AbilityRules.lean:89`
  (`CounterKind.check`), `Check/Phrase.lean:1686, 2431, 2449, 2456, 2931` (`refuse kind.known
  .knownCounter`); `.scope` users `Check/Phrase.lean:2458, 2953`.
- **Designation fields `zone`, `type` and `half` are unread** (verified). `DesignationFacts.zone` is
  read only by `DesignationLabel.seedZone` (`Check/Words.lean:1702`), which has zero callers (the
  `seedZone` at `Check/Phrase.lean:150-165, 455` is `Predicate.seedZone`). `DesignationFacts.type`
  has no reader. `DesignationFacts.half` is read only by the pin `Proofs/Tables.lean:51-53`
  (`bothDoorLabelsAreDeclaredHalves`). The live readers use `scope` only: `DesignationLabel.scope`/
  `holder`/`possessorOk`/`heldByItsCard`/`gameWide`, `designationHolderOk`
  (`Check/Words.lean:1692-1727`), at `Check/Phrase.lean:2506, 2880, 3060, 3355, 3360-3361`. Note:
  `Rules.lean:121-123`'s `Definition.designation` also carries `zone`, `type` and `half`
  (and `effectful`); the same columns, same fate.
- **`DesignationLabel.known` has no checker caller** (only `Proofs/Tables.lean:56-59`); an undeclared
  designation is refused today only through `scope = none` at the readers above.
- **Relation fields `left` and `right` are unread** (verified): `RelationLabel.left`/`.right`
  (`Check/Words.lean:1737,1739`) have zero callers. `Phrase.lean:3418-3425` (`.relate`) checks both
  operands as objects and `relation.known`. `lean/CONTRACTS.md:1037` says each side is checked
  at the kind the declaration gives; that sentence is wrong today (also `Definition.relation`
  carries `left`/`right`, `Rules.lean:128`).
- **Subtype frame**: `Subtype.frame`/`framesWith` (`Check/Words.lean:1643-1646`) read `subtypeFacts`;
  users `Check/Card.lean:89` (Saga chapter lines) and `:319` (`adventureInsetOk`). `Subtype` is
  `inductive Subtype | of (host : CardType) (label : String) | spell (label : String)`
  (`Words.lean:341-344`), tagged `[semantic_expression]` and `[internal_expansion]`
  (`Words.lean:637,643`): **it is an inductive with two constructors and no extra fields, not a
  struct**. The ruling's pattern ("like Subtype") holds for the tagging and the catalog, but a
  `frame` field means changing the constructors or the 400-odd catalog entries
  (`Definitions/Subtypes.lean`), e.g. `.of .artifact "Equipment"` becomes `.of .artifact
  "Equipment" (frame := none)`.

## Open choice (implementer; the code does not make one option clearly right)

Subtype frame: (a) `frame : Option FrameFeature := none` on `Subtype.of`/`spell` (saga, adventure,
room), keeping the refusal that a chapter line needs a Saga etc. [CR#714.1,715.1,709.5j] and
deleting `subtypeFacts`; or (b) drop the frame-vs-subtype check as an authoring mistake, as in
ticket `lean-v3-card-class-gate-removal`. Record the choice and its reason in the landing. Weigh:
(a) keeps a refusal that is arguably text meaning (a chapter line names the frame it needs), at the
cost of widening a constructor used 400 times (default `none` keeps those sites unchanged if the
field has a default); (b) is smaller and matches the gate ruling, but `adventureInsetOk` and the
chapter-line check then guard nothing the author could get wrong that the card's own subtypes do
not already show.

## Change

1. `CounterKind` gets `holder : Kind := .object` (STOP-2: not `confers`; conferrals stay on
   `Definition.counter`, and likewise on `Definition.subtype` for subtypes, with no new field on
   `Subtype`). Because it needs a field
   it becomes a structure or a two-field constructor: `.named name (holder := …)`; `name` stays.
   Tag it `[semantic_expression, internal_expansion]`; declare each counter in
   `Definitions/Counters.lean` with its holder (`energy`, `experience`, `poison`, `radCounter` are
   `.player`). `CounterKind.scope` reads `c.holder`. Delete `counterFacts`, `CounterFacts`,
   `counterFactsFor`, `knownCounter`, `CounterKind.known`, `distinctCounterLabels` and every
   `refuse kind.known .knownCounter` site (keeping `counterHolderKind`, a different law).
2. `DesignationLabel` gets `scope : DesignationScope` (declare `DesignationScope` first; it is at
   `Words.lean:248`, after the label). Tag the same way; declare each of the 21 designations with
   its scope. Drop `zone`, `type`, `half` (and `DesignationLabel.seedZone`, which nothing calls).
   Delete `designationTable`, `DesignationFacts`, `findDesignation`, `DesignationLabel.facts`,
   `.known`. Keep `RoomHalf` only if `lean-core-room-declarations` still needs it (it is a
   `maybe` ticket; `lean-v3-keyword-bodies-and-catalog-sync` §2 retires `half`).
3. `RelationLabel`: tag `[semantic_expression, internal_expansion]`; delete `relationTable`,
   `RelationFacts`, `RelationLabel.facts/.known/.left/.right`, and the `relation.known` refusal at
   `Check/Phrase.lean:3425` (`.knownRelation`). Fix `lean/CONTRACTS.md:1037` to say what the
   checker does.
4. Subtype frame: per the open choice. Per STOP-2, `Subtype` gains no `confers`; only `frame` may be
   added.
5. Reconcile `Rules.lean`'s `Definition` variants for these four: `Definition.designation`
   (`zone`/`type`/`half`) and `Definition.relation` (`left`/`right`) carry the same unread columns;
   drop them together, and update `Check/Rules.lean:47-59`, `Proofs/Rules.lean` and the doc comment
   at `Rules.lean:96-100` ("`Check.Facts` is generated from these").
6. Retire every `known*` pin of these types once the matching authoring pin exists (below).

## Tests

- **Retired** (each replaced by an authoring-boundary pin; reason: the check moved to the boundary,
  so a checker-level `known*` refusal no longer exists): `Proofs/Tables.lean`
  `anUndeclaredDayNightLabelIsNotADesignation` (`:56-59`), `everyDayNightLabelIsADesignation`
  (`:44-49`; becomes a pin that `day` and `night` are declared with `heldByGame` scope),
  `bothDoorLabelsAreDeclaredHalves` (`:51`; the `half` column is gone), and the counter
  pins in `Proofs/Counters.lean` that go through `CounterKind.known`/`CounterKind.check`
  (`okBoostCounterKind` `:310`, `badSetBoostCounter` `:314`, `okPoisonCounterLabel`; read each, keep
  the ones about `scope`/holder, which `holder` still answers).
- **Replaced**, in `Proofs/Authoring.lean`: `#guard_msgs` pins that a raw `CounterKind.named
  "typo"`, `DesignationLabel.named "dusk"` and `RelationLabel.named "unrelated"` in a `spelled`
  card are refused with "raw constructors: [...]", like the `Subtype.of` pins (`:115-125`).
- **Added** (the generator's retired xtask tests, per `lean-v3-facts-from-declarations` Step 1.7):
  `decide` pins over the declared values: the holder of each of `energy`, `experience`, `poison`,
  `radCounter` is `.player` and `p1p1Counter`'s is `.object`; the scope of `monarch`, `commander`,
  `day`, `citysBlessing`.
- No fixture-level `known*` pins are kept (STOP-1): `Cards/` fragments and `Proofs/` terms are not
  checked against the deleted tables.
- Kept: the `distinct*` pins have nothing to check once each item is its own declaration (a
  duplicate declaration is a Lean error); retire `counterLabelsDistinct`, `designationLabelsDistinct`
  and `subtypeFactsDistinct` with that reason.
- `cargo test -p xtask -p deckmaste_semantics_v2` and `cargo clippy ... -D warnings` if the facts
  step-1 work has not already removed the crates that mirror these rows; otherwise none.

## Other tickets that touch the same files

- `lean-v3-facts-from-declarations` (WIP): Step 1 item 6 decides the unread columns before this
  ticket; this ticket then deletes them with the tables.
- `lean-v3-keyword-bodies-and-catalog-sync` (planned) §2: retires `half` /
  `RoomHalf.designation` / `DesignationLabel.half` and cross-pins catalogs against
  `Check/Facts.lean`; the cross-pin is moot once the tables are gone. Whichever lands second
  reconciles.
- `semantics-v3-designation-storage-columns` (planned): expects the designation columns back in
  `plugins_v3/`; its decision is whether they are Lean columns, which this ticket now answers for
  `scope`.
- `lean-core-room-declarations` (maybe): declares `leftHalfUnlocked`/`rightHalfUnlocked`
  (`Check/Facts.lean:499-500`).
