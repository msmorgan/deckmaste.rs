---
needs: [lean-v3-catalog-items-self-contained]
---
# Delete `keywordFacts`: keyword abilities are typed declarations

Standard constraints apply. Lean first (`lean/Semantics/`). Part 3 of 5 of ruling #1 of the owner's
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
`Check/FactTypes.lean`).

## The clause this ticket carries

> - **keywordFacts:** mark `Ability.keyword`/`KeywordLabel` internal_expansion, so cards can only
>   invoke the typed functions in `Definitions/Keywords.lean`.
>   - The card-class columns go per #2.
>   - `argumentSchemas`/`keywordParamFits` are deleted, since Lean types enforce parameters.
>   - `wantsModes`: add `static (modeChoice ...)` to entwine and escalate (matching spree and
>     tiered), and have `keywordWantsModes` inspect the body for `.modeChoice`.
>   - `paidCost`: `LinkSource.named` inspects params/body on the `Ability.keyword` node directly.

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

- **Row counts** (`Check/Facts.lean:73-266`): `keywordFacts` has 193 rows. 67 set
  `onInstantOrSorceryCard := true`, 12 set `onPermanentCard := false` (those two columns go with
  `lean-v3-card-class-gate-removal`), 68 set `paidCost := true`, 4 set `wantsModes := true`
  (entwine, escalate, spree, tiered).
- **FALSE as stated, harmless: "entwine and escalate lack `modeChoice`".** They already have it:
  `entwine` is `[static (.modeChoice (.allInstead (some (.inCaller cost)) none))]`
  (`Definitions/Keywords.lean:624-626`) and `escalate` is `[static (.modeChoice (.extraModesCost
  (.inCaller cost)))]` (`:644-646`); `spree` (`:1496-1497`, `.setQuantity (orMore 1)`) and `tiered`
  (`:1568-1569`, `.setQuantity (oneThrough 1)`) have it as well. So the "add `modeChoice`" step is
  a no-op, and `keywordWantsModes` (`Check/Card.lean:228-237`, the `.keyword k _ _ =>
  (keywordFactsFor k).elim false (·.wantsModes)` arm at `:232`) can read the body for
  `.modeChoice` directly, through `inCaller`/`conditional` wrappers, and the `wantsModes` column
  goes. Corrected quietly.
- **`LinkSource.named` cannot read params/body today (the ruling's premise is false as stated).**
  `LinkSource.named` (`Check/Keywords.lean:115-120`) takes a `LinkSource`, and
  `LinkSource.byKeyword`/`byNthKeyword` hold only a `KeywordLabel`: there is no `Ability.keyword`
  node in scope. Its users are the templates `paidCostRead`, `timesPaid`, `costWasPaid`,
  `choiceWasTaken` (`Templates.lean:1324-1337`), `keywordCostPaid` (`:2184`) and the pins
  `Proofs/Faces.lean:795-811` (`alternativeCostReadbacks`, `modalCostReadbacks`), which check
  `paidCostRead (.byKeyword k) none .this` against `DRS.ofList []`, with no card at all.
- **`paidCost` is not derivable from a cost parameter.** 59 rows have a `.cost` parameter and all
  59 are `paidCost`; 9 more are `paidCost` with no cost parameter: `casualty` (number), `gift`
  (none), `bargain` (none), `conspire` (none), `devour` (number; quality and number),
  `jumpStart` (none), `offering` (quality), `teamwork` (number), `tribute` (number).
- **STOP-3 (narrowed, awaiting the owner): where `paidCost` comes from.** The ruling's design
  ("inspect params/body on the `Ability.keyword` node") needs a node, and `LinkSource` does not
  carry one. Two ways to give it one: (a) resolve the label against the enclosing card's abilities
  when checking (the checker already threads a header context, `Check/HeaderContext.lean`) and
  inspect that node's params and body (a cost parameter, or an `addedCost`/`altCost` in the body,
  which covers casualty, bargain, conspire, jump-start, offering...); the context-free pins at
  `Proofs/Faces.lean:795-811` then need a card; or (b) a typed marker on the keyword's definition,
  which is a one-column table again. **Proposed:** (a). Until ruled, the implementer does steps
  1-3 and 6 and leaves `paidCost` and its `LinkSource.named` readers in place.
- **STOP-4 (narrowed, awaiting the owner): the term-level readers of `argumentSchemas`.** The
  ruling deletes `argumentSchemas` "since Lean types enforce parameters". That holds for invoking
  a keyword (`keywordParamFits`, `Check/Phrase.lean:1587`, used `:4121-4123`). Four other readers
  ask about keyword *terms* and *labels*, where no typed function is called:
  `KeywordFamily.ok` (`Check/Keywords.lean:98-104`: a family sort needs a quality-parameter
  schema), `KeywordTerm.known` `.theWith` (`:109`: needs a number schema), `keywordCosts` and
  `CostNamed.nameable` (`:95-96, :129-131`: "the cost of keyword K" needs a cost schema), and
  `checkKeywordParamsWith` (`Check/Phrase.lean:1613`, which also applies each position's
  `qualityDomain`: `.object` for equip, cycling, emerge, splice). **Proposed:** derive the shape
  from the declared function's own signature (the parameter types of the `semantic_definition` in
  `Definitions/Keywords.lean`, read at elaboration time, as `spliceSchemas` does,
  `Authoring.lean:154`), or make each term form a typed template over the declaration. The
  implementer picks only after the owner confirms the direction.
- Remaining readers of `keywordFactsFor`/`knownKeyword`: `Check/Keywords.lean:20-21, 91, 99,
  107, 116-117, 122`, `Check/Card.lean:30-31` (card class, ticket A), `:232`, `Check/Phrase.lean:1644,
  4121`. `distinctKeywordWords` (`Check/Keywords.lean:16`) and `Proofs/Tables.lean`
  `keywordWordsDistinct`.
- The generic by-label templates `keyword`, `keywordCosting`, `keywordQuality`, `keywordSubject`,
  `keywordNumber`, `keywordQualityCosting`, `keywordNumberCosting` (`Templates.lean:1827-1851`),
  and `keywordGrant` (`:1979`) are an untyped invocation path: with `KeywordLabel` internal a card
  cannot pass a raw label, so they are used only by `Proofs/` and `Definitions/` today (0 uses in
  `Cards/`).

## Change

1. Tag `Ability.keyword` and `KeywordLabel` (and `AbilityWordLabel`, whose `known` table
   `abilityWordNames` is the CR's list and is not one of the six tables: leave that table and its
   `knownAbilityWord` refusal, `Check/Keywords.lean:88`, `Check/Phrase.lean:4166`, unless the owner
   widens the ruling) `[semantic_expression, internal_expansion]`. Keep `Definitions/Keywords.lean`
   spelling `.keyword (.named "…")` inside definition bodies, as `ascend` does at `:240`.
2. Delete the card-class columns (ticket A; if A has not landed, A first).
3. `wantsModes`: delete the column; `keywordWantsModes` inspects the body for `.modeChoice`.
4. `paidCost` and the term-level `argumentSchemas` readers: after STOP-3 and STOP-4.
5. Delete `keywordFacts`, `KeywordFacts`, `KeywordParamShape`, `KeywordParamSpec`,
   `KeywordSchema`, `keywordFactsFor`, `knownKeyword`, `distinctKeywordWords`, `keywordParamFits`,
   `keywordSchemaFor`, and the `knownKeywordTerm`/`keywordParamFits` refusals once nothing raises
   them (after STOP-1 and STOP-4).
6. Re-spell or delete the by-label templates above and re-spell their `Proofs/` uses with the
   typed functions.

## Tests

- **Retired**, each replaced by an authoring pin in `Proofs/Authoring.lean` (a raw
  `KeywordLabel.named "flyign"` in a `spelled` card is refused); reason: the check moved to the
  boundary: `badUnknownKeywordPredicate` (`Proofs/Faces.lean:208`), `badUnknownKeywordLabel`
  (`Proofs/Mana.lean:117`), `badUnknownKeywordClass` (`Proofs/Keyword.lean:264`),
  `badPrintedAfterUnknownKeyword` (`Proofs/AbilityQualifiers.lean:90`), `badTimesPaidUnknownKeyword`
  (`Proofs/Description.lean:353`), the unknown-keyword pins at `Proofs/Anaphora.lean:208,214`,
  `Proofs/Keyword.lean:404,411`, `Proofs/CharacteristicEdits.lean:57`, `Proofs/Tables.lean`
  `keywordWordsDistinct`.
- **Retired as unrepresentable** (a typed function cannot be applied to the wrong parameters;
  state that reason for each): the `keywordParamFits` refusals at `Proofs/Faces.lean:236, 546,
  551`, `Proofs/KeywordArguments.lean:15, 22, 27, 65`, `Proofs/Keyword.lean:236, 258`,
  `Proofs/Mana.lean:102, 107, 113, 118, 224`. Where a pin's claim is also "the arguments are
  stepped", keep the positive half (`Proofs/KeywordArguments.lean:9-33`).
- **Kept/moved**: `alternativeCostReadbacks`, `modalCostReadbacks` (`Proofs/Faces.lean:795-811`)
  move with STOP-3's resolution; `hexproof_and_cycling_admit_their_cr_variants` (the retired xtask
  test) is carried by `Proofs/Keyword.lean:352` and `KeywordArguments.lean:9-33` (confirm).
- **Added**: `okEntwineEscalateSpreeTieredWantModes` (each body contains `.modeChoice`), and a
  negative (a keyword without it does not want modes).

## Other tickets that touch the same files

- `lean-v3-card-class-gate-removal` (planned): deletes the two class columns and
  `keywordCardOk` from these rows.
- `lean-v3-facts-from-declarations` (WIP): Step 1 item 5 does the same in its workspace.
- `lean-v3-keyword-bodies-and-catalog-sync` (planned) adds rows to `keywordActionLabels` and
  cross-pins catalogs against `Check/Facts.lean`; whichever lands second reconciles.
- `lean-v3-checker-runtime-state-audit` (planned) classes the remaining class rules.
- `lean-v3-ability-cost-substitutes` (planned) writes *Thick-Skinned Goblin* over `Action.pay`
  with `keyword := some (.named "echo")`, a raw label: re-spell it with the typed echo
  declaration after this lands.
