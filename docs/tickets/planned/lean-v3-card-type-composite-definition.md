---
needs: [lean-v3-action-facts-deleted]
---
# CardType is a composite `semantic_definition`; delete `Check/Facts.lean` and `Check/FactTypes.lean`

Standard constraints apply. Lean first (`lean/Semantics/`). Part 5 of 5 of ruling #1 of the owner's
batch 4 (2026-10-22); the last, so it also deletes the two files. Supersedes step 2 of lean-v3-facts-from-declarations (WIP, owned elsewhere; that ticket's step 1 still lands first).

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
`Check/FactTypes.lean`).

## The clauses this ticket carries

> (1) **CardType** becomes a composite `semantic_definition` struct (like Subtype) carrying:
> - its name;
> - `permanent : Bool`;
> - its activated characteristic stats (`[.power, .toughness]`, `[.loyalty]`, `[.defense]`);
> - `confers : List Conferral := []`.
>
> Its raw constructor is internal_expansion. Each card type is declared in `Definitions/` over
> shared primitives and templates:
> - **artifact, enchantment:** normal permanents castable `asSorcery` [CR#301.1,303.1].
> - **creature:** power/toughness [CR#302.4]; attack/block [CR#302.5]; summoning sickness
>   [CR#302.6]; takes marked damage [CR#120.3e]; SBAs [CR#704.5f,704.5g].
> - **planeswalker:** loyalty [CR#306.5]; enters with loyalty [CR#306.5b]; can be attacked
>   [CR#306.6]; damage removes loyalty [CR#120.3c]; SBA [CR#704.5i].
> - **battle:** defense [CR#310.4]; enters with defense [CR#310.4b]; can be attacked [CR#310.5];
>   damage removes defense [CR#120.3h]; SBA [CR#704.5v]; chosen protector [CR#310.9a]. Siege, in
>   `Definitions/Subtypes.lean`, adds the opponent protector [CR#310.12a] and the last-counter
>   trigger [CR#310.12b]. (The owner's text cited three battle rules by numbers the CR snapshot does not carry for those claims; corrected under Verified.)
> - **instant/sorcery:** `permanent := false`, `asInstant` [CR#304.1] vs `asSorcery` [CR#307.1].
> - **land:** play permission [CR#305.1].
> - **kindred** [CR#308.1].
>
> (2) ... Delete `lean/Semantics/Check/Facts.lean` and `lean/Semantics/Check/FactTypes.lean`.

## Authoring boundary (VERIFY; full account in `lean-v3-catalog-items-self-contained`)

`Authoring.inspect` (`Authoring.lean:345,427`) returns `.raw` for a constructor of any
`semantic_expression` type that is not `semantic_literal`, and `spelled` (`Check/Card.lean:411-431`)
refuses a card holding one; `[internal_expansion]` only suppresses the generated primitive template
(`Authoring.lean:246-253`). `KeywordLabel`, `AbilityWordLabel` and `Deed` are not
`semantic_expression` today (`Words.lean:637` tags only `Delta` and `Subtype`), so this ticket tags
them. **STOP-1 (narrowed, awaiting the owner)**: terms outside `spelled` (about 750 fragment
definitions in `Cards/` and all of `Proofs/`) are not covered by the boundary; proposed: they are
fixtures and keep no `known*` check. Every step except the final deletion of the `known*` checks
proceeds either way.

## Verified (2026-10-22, paths under `lean/Semantics/`)

- **CR citations corrected (quietly).** In the CR snapshot (`data/rules/cr.txt`): the chosen protector
  is [CR#310.9a] (`:1826`); [CR#310.10] is "a battle can't be attached" (`:1840`); Siege's
  opponent-only protector is [CR#310.12a] (`:1846`) and its last-counter trigger [CR#310.12b]
  (`:1848`); [CR#310.11] is the protector reassignment rule. The Siege state-based action is
  [CR#704.5v] (`:5551`), the non-Siege one [CR#704.5w] (`:5553`, also [CR#310.8]). The rest of the
  ruling's citations were read against the snapshot and say what is claimed
  [CR#301.1,303.1,302.4,302.5,302.6,120.3c,120.3e,120.3h,704.5f,704.5g,704.5i,306.5,306.5b,306.6,310.4,310.4b,310.5,304.1,307.1,305.1,308.1].
  Use the corrected ones.
- **`CardType` today**: `inductive CardType` with ten constructors (creature, artifact, land,
  enchantment, instant, sorcery, planeswalker, battle, kindred, dungeon), `deriving DecidableEq,
  Repr` (`Words.lean:26-29`); not `semantic_expression`. `dungeon` is a card type the ruling's
  list omits; keep it declared (non-permanent, not castable [CR#309.2,309.2c]).
- **`Subtype` does not have the shape "composite struct (like Subtype)" suggests.** It is an
  inductive with two constructors `of (host : CardType) (label : String)` and `spell (label : String)`
  (`Words.lean:341-344`), tagged `[semantic_expression]` and `[internal_expansion]`
  (`Words.lean:637,643`) with a catalog of `semantic_definition`s in `Definitions/Subtypes.lean`.
  "Like Subtype" therefore means the tagging and catalog pattern, not a struct with extra fields.
  CardType is genuinely a new shape: a structure whose catalog entries are
  `semantic_definition CardType.creature : CardType := …` (the precedent for a namespaced entry is
  `Subtype.army`, `Definitions/Subtypes.lean`), so `types := [.creature]` keeps resolving.
- **STOP-2 applies (narrowed, awaiting the owner): `confers : List Conferral` on `CardType` is a
  type cycle** (full account in `lean-v3-catalog-items-self-contained`): `Conferral`
  (`Rules.lean:51`) holds `Ability`/`StaticSpec`, which hold `CardType` (`Predicate.hasType`,
  `Subtype.of`, `QualitySort.subtype`, `KindAxis.subtype`). **Proposed:** the value carries name,
  `permanent` and `stats`; the conferrals stay on the separate record `Definition.cardType`
  (`Rules.lean:124`) already keyed by the value, declared in the same `Definitions/` file. The
  card-type declarations below are written either way; only where `confers` lives waits.
- **The existing record already holds three of the four fields**: `Definition.cardType
  (cardType : CardType) (permanentType : Bool) (confers : List Conferral)` (`Rules.lean:124`), checked
  at `Check/Rules.lean:59`, with pins `artifactTypeDefinition`, `creatureTypeDefinition`
  (`[combatant]`), `landTypeDefinition` (`[playableFrom yourHand asSorcery]`),
  `instantTypeDefinition` (`[castableFrom yourHand asInstant]`) and `okDungeonType`
  (`Proofs/Rules.lean:281-300`). The composite subsumes it.
- **Hardcoded per-type knowledge to replace by fields**: `CardType.permanent`
  (`Check/Words.lean:1820-1822`: false for kindred, instant, sorcery, dungeon) and
  `CardType.isInstantOrSorcery` (`:1824`); `normalizeTypes` (`:25-27`, the canonical order);
  `Stat.comparedType` (`:38-42`: power/toughness need creature, loyalty planeswalker, defense
  battle); `boxSuitsTypes` (`Check/Card.lean:198-201`) and `cardBoxOk` (`:210-214`);
  `anyPermanentType`/`anyInstantOrSorceryType`/`typesCombinable` (`:22-27`), `cardClassOf`
  (`:17-19`), `flipHalfOk` (`:320`), `:384`.
- **Layering**: Siege's rules need templates but `Definitions/Subtypes.lean` imports only
  `Semantics.Primitives` (`:1`) and `Check.lean:13` imports it. After
  `lean-v3-template-helpers-into-definitions` the Definitions layer sits above Templates, so
  `Subtypes.lean` may import `Templates`; check that `Check.lean:13` still builds, or drop that
  import (nothing in `Check/` should read a catalog item; the checker reads the fields).
- **Equality cost**: card types are compared with `decide` throughout the checker
  (`tys.elem .creature`, `typesDistinct`). A structure with `confers : List Conferral` would need
  `DecidableEq` over `Ability` for each comparison. Under the STOP-2 proposal the value is
  `name`/`permanent`/`stats` and equality is cheap; if `confers` ends on the value, define `BEq`/
  `DecidableEq` by `name` and prove it lawful.

## Change

1. `CardType` becomes a structure `{ name : String, permanent : Bool, stats : List Stat }`
   (STOP-2: plus `confers`), tagged `[semantic_expression, internal_expansion]`, declared in
   `Words.lean`; each of the ten types is a `semantic_definition CardType.x` in a new
   `Definitions/CardTypes.lean`, over shared primitives and templates, with the CR rules in the
   clauses above (corrected citations). Siege's protector and last-counter rules go in
   `Definitions/Subtypes.lean`.
2. Replace the hardcoded knowledge above by field reads: `CardType.permanent` becomes the field;
   `Stat.comparedType` and `boxSuitsTypes` read `stats`; `normalizeTypes` takes its order from the
   declared list; `isInstantOrSorcery` is derived (a non-permanent type with an `asInstant` or
   `asSorcery` cast permission) or kept as a field if deriving it is unclear (say which).
3. Every `match` on a card-type constructor (`| .kindred | .instant …`, `Check/Words.lean:1820-1826`)
   and every pattern use becomes a field read or an equality test; grep `\.(creature|artifact|land|
   enchantment|instant|sorcery|planeswalker|battle|kindred|dungeon)` in `Check/` and fix each.
4. Retire `Definition.cardType` (`Rules.lean:124`) and its `Check/Rules.lean:59` arm if `confers`
   moves to the value; otherwise keep it as the conferral carrier (STOP-2).
5. **Delete `Check/Facts.lean` and `Check/FactTypes.lean`.** By now they should hold only the
   `FrameFeature` type (if B2 chose the subtype-frame option) and empty tables; move
   `FrameFeature` next to `Subtype` (`Words.lean`). Remove their imports (`Check.lean:11-12`,
   `Check/Words.lean:2`, `Check/Keywords.lean`), delete `Proofs/Tables.lean` if it holds nothing
   (and its import in `Proofs.lean`), and fix the docs: `lean/CONTRACTS.md`, `lean/README.md`,
   ADR `semantics-v3.md` where they name the tables, and the `Rules.lean:96-100` doc comment.
   Account for each hit of `rg 'Check/Facts|FactTypes|keywordFacts|counterFacts|designationTable|
   relationTable|subtypeFacts|actFacts|keywordActionLabels'`.

## Tests

- **Rewritten**: `Proofs/Rules.lean:281-300` (`okArtifactType`, `okCreatureType`, `okLandType`,
  `okInstantType`, `okDungeonType`) now check the declared values, not hand-built `Definition`s;
  the claim (what each type confers, CR cited) is unchanged.
- **Added**: `permanent` per type (`instant`, `sorcery`, `kindred`, `dungeon` are not permanents
  [CR#110.4,304.1,307.1,308.1,309.2]; the other six are), `stats` per type (creature `[power,
  toughness]` [CR#302.4], planeswalker `[loyalty]` [CR#306.5], battle `[defense]` [CR#310.4]), a
  raw `CardType` constructor in a `spelled` card is refused (`Proofs/Authoring.lean`).
- **Kept, must pass unchanged**: the `cardLineOk` / `typesCombinable` / `boxSuitsTypes` pins in
  `Proofs/Faces.lean` (`badCreatureCardNoPt`, `badSpellAbilityOnPermanent`, …) and every card bench.
- **Retired**: whatever remains of `Proofs/Tables.lean`, with the reason "its tables are gone".
- `lean/scripts/build` (`lake build --wfail`), `cargo xtask lean check` if plugin definitions
  change, the CR citation gates; `cargo test -p xtask -p deckmaste_semantics_v2` only if
  `lean-v3-facts-from-declarations` Step 1 left them standing.

## Other tickets that touch the same files

- `lean-v3-facts-from-declarations` (WIP): Step 1 edits the headers of `Check/Facts.lean` and
  `FactTypes.lean` that this ticket deletes; step 1 lands first.
- `lean-v3-authoring-surface` (planned umbrella; `hasType (t : CardType)` and the type words,
  `:520-529`), `parse-subject-filter-stringly-channels` and `engine-static-ability-zone-gating`
  (Rust-side card-type mentions): read, no edit.
- `lean-core-room-declarations` (maybe): cites `Check/Facts.lean:499-500`.
- `lean-v3-card-class-gate-removal`, `lean-v3-keyword-facts-deleted`, `lean-v3-action-facts-deleted`
  (planned): earlier tickets of this set; they must have landed.
