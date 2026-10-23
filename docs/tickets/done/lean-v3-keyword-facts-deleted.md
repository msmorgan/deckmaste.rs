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
them. **Ruled (owner, 2026-10-23), STOP-1 accepted**: the authoring boundary
(`Authoring.inspect` / `spelled`) covers only `Spelled` cards. Card fragments in `Cards/` and
everything in `Proofs/` are fixtures and keep no `known*` check; a typo in a fixture surfaces when
the full card is spelled. No `known*` check is kept for non-`spelled` input, so the deletion steps
are not gated.

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
- **Ruled (owner, 2026-10-23), STOP-3: proposal (a) accepted.** `paidCost` resolves the label
  against the enclosing card's abilities (via the header context, `Check/HeaderContext.lean`) and
  inspects that node's params and body: a cost parameter, or an `addedCost`/`altCost` in the body.
  The context-free pins at `Proofs/Faces.lean:795-811` get a card. The original analysis follows.
  Where `paidCost` comes from: the ruling's design
  ("inspect params/body on the `Ability.keyword` node") needs a node, and `LinkSource` does not
  carry one. Two ways to give it one: (a) (chosen) resolve the label against the enclosing card's abilities
  when checking (the checker already threads a header context, `Check/HeaderContext.lean`) and
  inspect that node's params and body (a cost parameter, or an `addedCost`/`altCost` in the body,
  which covers casualty, bargain, conspire, jump-start, offering...); the context-free pins at
  `Proofs/Faces.lean:795-811` then need a card; or (b) a typed marker on the keyword's definition,
  which is a one-column table again (rejected).
- **Ruled (owner, 2026-10-23), STOP-4: proposal accepted.** The term-level readers derive a
  keyword's parameter shape from its `semantic_definition`'s own signature at elaboration time, as
  `spliceSchemas` does (`Authoring.lean:154`). The readers are `KeywordFamily.ok`,
  `KeywordTerm.known` `.theWith`, `keywordCosts`/`CostNamed.nameable` and `checkKeywordParamsWith`,
  the last including `qualityDomain`. The original analysis follows. The
  ruling deletes `argumentSchemas` "since Lean types enforce parameters". That holds for invoking
  a keyword (`keywordParamFits`, `Check/Phrase.lean:1587`, used `:4121-4123`). Four other readers
  ask about keyword *terms* and *labels*, where no typed function is called:
  `KeywordFamily.ok` (`Check/Keywords.lean:98-104`: a family sort needs a quality-parameter
  schema), `KeywordTerm.known` `.theWith` (`:109`: needs a number schema), `keywordCosts` and
  `CostNamed.nameable` (`:95-96, :129-131`: "the cost of keyword K" needs a cost schema), and
  `checkKeywordParamsWith` (`Check/Phrase.lean:1613`, which also applies each position's
  `qualityDomain`: `.object` for equip, cycling, emerge, splice). **Chosen:** derive the shape
  from the declared function's own signature (the parameter types of the `semantic_definition` in
  `Definitions/Keywords.lean`, read at elaboration time, as `spliceSchemas` does).
- Remaining readers of `keywordFactsFor`/`knownKeyword`: `Check/Keywords.lean:20-21, 91, 99,
  107, 116-117, 122`, `Check/Card.lean:30-31` (card class, ticket A), `:232`, `Check/Phrase.lean:1644,
  4121`. `distinctKeywordWords` (`Check/Keywords.lean:16`) and `Proofs/Tables.lean`
  `keywordWordsDistinct`.
- The generic by-label templates `keyword`, `keywordCosting`, `keywordQuality`, `keywordSubject`,
  `keywordNumber`, `keywordQualityCosting`, `keywordNumberCosting` (`Templates.lean:1827-1851`),
  and `keywordGrant` (`:1979`) are an untyped invocation path: with `KeywordLabel` internal a card
  cannot pass a raw label, so they are used only by `Proofs/` and `Definitions/` today (0 uses in
  `Cards/`).

## Ruling (owner, 2026-10-23): after the STOP on STOP-3

The STOP-3 premise was false: *Ertai's Trickery* ("Counter target spell if it was kicked.") and
*Merfolk Falconer* ("Whenever you cast a kicked spell, scry 2.") read the kicker cost of a spell
they do not print, so no lookup against the enclosing card's abilities can decide `paidCost`.
STOP-3 (a) and STOP-4 are superseded by this ruling, which governs where the sections above and
below disagree.

1. **Authored labels.** Each keyword `semantic_definition` generates an authored `KeywordLabel`
   constant, and `elabKeywordName` emits that constant. `KeywordLabel.named` is
   `[internal_expansion]`, with `[semantic_expression]`. A typo ("kickre") fails at elaboration.
2. **Delete the shape checks; no schema collector.** With `Ability.keyword` and the stub helpers
   (`keywordCosting`, `keywordQualityCosting`, `keywordNumber`, `keywordQuality`,
   `keywordSubject`) restricted to internal or definition use and `KeywordLabel` authoring-checked,
   the remaining checks only police authoring mistakes ("if it was flying", "flying costs",
   "flying abilities"), the principle of ruling #2. Delete `paidCost`/`paysCost`, `KeywordSchema`,
   `KeywordParamShape`, `keywordParamFits`, the `CostNamed.nameable` keyword-cost check (with the
   `.pay` keyword check the ability-cost landing added), the `KeywordFamily.ok` schema check and
   `theWith`'s schema check. Their negative pins are retired (deleted, not flipped), each named
   with this reason; `badPaymentOfCostlessKeyword` goes too. The linked-cost readback by label
   stays, so *Ertai's Trickery* and *Merfolk Falconer* still read "kicked"; pin both.
3. **Object-only qualities come from real bodies.** `emergeFrom` and `Ability.splice` already force
   `.object` through their bodies. `equipQuality` [CR#702.6c] (the ruling cited the rule after it,
   on multiple equip abilities; [CR#702.6c] is the quality restriction) and
   `typecycling` [CR#702.29e] get their CR bodies in place of the `keywordQualityCosting` stubs,
   and `qualifiedCostRequiresObjectQuality` is rewritten over `typecycling`. No `qualityDomain`
   field.
   - *Found while landing:* only `emergeFrom` forced `.object` (through `sacrifice`'s patient
     kind). The checker did not refuse a conjunction whose conjuncts name kinds that do not
     overlap, so `Ability.splice .anyPlayer` and the new bodies (`and [isCard, quality]`,
     `and [quality, isCreature, youControl]`) checked clean. Fixed at the defect: a conjunct
     naming a kind the head's does not overlap is refused `kindMismatch` (`Check/Phrase.lean`).
4. **`wantsModes`.** `keywordWantsModes` inspects the body.

## Change

1. Tag `Ability.keyword` and `KeywordLabel` (and `AbilityWordLabel`, whose `known` table
   `abilityWordNames` is the CR's list and is not one of the six tables: leave that table and its
   `knownAbilityWord` refusal, `Check/Keywords.lean:88`, `Check/Phrase.lean:4166`, unless the owner
   widens the ruling) `[semantic_expression, internal_expansion]`. Keep `Definitions/Keywords.lean`
   spelling `.keyword (.named "…")` inside definition bodies, as `ascend` does at `:240`.
2. Delete the card-class columns (ticket A; if A has not landed, A first).
3. `wantsModes`: delete the column; `keywordWantsModes` inspects the body for `.modeChoice`.
4. `paidCost`: make `LinkSource.named` (and `paidCostRead`, `timesPaid`, `costWasPaid`,
   `choiceWasTaken`, `keywordCostPaid`) resolve the label against the enclosing card's abilities
   through the header context (`Check/HeaderContext.lean`), and read that node's params and body
   (a cost parameter, or an `addedCost`/`altCost`): this covers the 59 cost-parameter rows and
   casualty, bargain, conspire, jump-start, offering and the rest. Then re-spell
   `Proofs/Faces.lean:795-811` over a card. Term-level `argumentSchemas` readers
   (`KeywordFamily.ok`, `KeywordTerm.known` `.theWith`, `keywordCosts`/`CostNamed.nameable`,
   `checkKeywordParamsWith` including `qualityDomain`): derive the parameter shape from the
   keyword's `semantic_definition` signature at elaboration time, as `spliceSchemas` does; no
   hand table remains.
5. Delete `keywordFacts`, `KeywordFacts`, `KeywordParamShape`, `KeywordParamSpec`,
   `KeywordSchema`, `keywordFactsFor`, `knownKeyword`, `distinctKeywordWords`, `keywordParamFits`,
   `keywordSchemaFor`, and the `knownKeywordTerm`/`keywordParamFits` refusals once nothing raises
   them (after step 4).
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
  are re-spelled over a card (STOP-3 ruling: `paidCost` reads the enclosing card's keyword node); `hexproof_and_cycling_admit_their_cr_variants` (the retired xtask
  test) is carried by `Proofs/Keyword.lean:352` and `KeywordArguments.lean:9-33` (confirm).
- **Added**: `paidCost` pins on a card: a cost-parameter keyword, and one paid through an
  `addedCost`/`altCost` body (casualty or bargain), read `true`; a keyword with neither reads
  `false`. Term-level reader pins (family sort needs a quality parameter, `.theWith` a number,
  keyword cost a cost) now derive from the declaration signature.
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
