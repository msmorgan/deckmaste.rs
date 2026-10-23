---
needs: []
---
# "The same is true for …" is a Template producing one Ability

Standard constraints apply. Provenance: row B7 of
[lean-v3-cr-structure-principle-audit](../done/lean-v3-cr-structure-principle-audit.md).

## Ruling (owner, 2026-10-22)

- Retire the `Ability.alsoForKeywords` constructor. `alsoForKeywords` becomes a
  `semantic_template` producing ONE Ability [CR#113.2c]: a single `.triggered` whose action runs
  `sequentially` over the keywords, or a single `.static (conjunction …)`. *Odric, Lunarch
  Marshal*'s rulings of 2016-04-08 speak of "Odric's ability": one ability.
- ~~Rename `AbilityLost` to `SpecifiedAbility` and retype `addedAbilities`.~~ Dropped by the
  second-batch ruling below (option B).
- Delete the wrapper arms and the keyword-extension checker functions and refusals.

"The same is true for" a list of keywords grants each variant of a keyword that has variants
[CR#702.1c]; each paragraph is one ability [CR#113.2c].

## Verified (2026-10-22)

Paths are under `lean/Semantics/`.

- `| alsoForKeywords (ability : Ability) (keywords : List KeywordTerm)` (`Abilities.lean:1321-1323`).
- **`alsoForKeywords` is already a `semantic_template`** (`Templates.lean:1940-1942`) that expands
  to the constructor; the command is `Authoring.lean:214-237`. The change is a new expansion plus
  deleting the constructor. No Template so far takes a function parameter (here `KeywordTerm →
  Action` or `→ StaticSpec`); `Authoring.inspect` admits lambdas (`Authoring.lean:360`), and cards
  already pass them.
- `Action.sequentially` (`Abilities.lean:1246`, Template `Templates.lean:1606`; refuses an empty
  list, steps each member as a new clause, `Check/Phrase.lean:3536-3543`).
  `StaticSpec.conjunction (subject : Option NounPhrase) (parts : List StaticSpec)`
  (`Abilities.lean:946`; Template `conjunction parts (subject := none)`,
  `Templates.lean:1954-1956`; refuses an empty list and a nested conjunction,
  `Check/Phrase.lean:3769-3779`).
- **Seven cards, not four.**

  | Card | Site | Body |
  |---|---|---|
  | *Odric, Lunarch Marshal* | `Cards/Turn.lean:274` | triggered, per-keyword `doOnlyIf (establishThroughout (abilityGrant …) untilEndOfTurn) (exists_ … hasKeyword k)` |
  | *Bleeding Effect* | `Cards/Turn.lean:288` | triggered, same shape |
  | *Concerted Effort* | `Cards/Turn.lean:376` | triggered; list includes the landwalk and protection families |
  | *Urborg Scavengers* | `Cards/Counters.lean:742` | static, `onlyWhile (abilityGrant thisCreature k) (exists_ … exiledWith … hasKeyword k)` |
  | *Death-Mask Duplicant* | `Cards/Cost.lean:772` | static; includes landwalk and protection |
  | *Cairn Wanderer* | `Cards/Keyword.lean:1176` | static; includes landwalk and protection |
  | *Escaped Shapeshifter* | `Cards/Keyword.lean:1227` | static; `[firstStrike, trample, protectionFromAnyColor]` |

  The three triggered cards fit one `.triggered` over `sequentially [doIf … per keyword]`; the four
  static cards fit one `.static (conjunction [onlyWhile … per keyword])`.
- `AbilityLost = written (ability : Ability) | term (keyword : KeywordTerm)`
  (`Abilities.lean:1301-1303`). **Correction:** family terms are not `AbilityLost` constructors;
  they are `KeywordTerm.anyIn (family : KeywordFamily)` (`Words.lean:414-424`), which
  `AbilityLost.term` carries. `AbilityLost` sits in `AbilitySelection.specified (abilities : List
  AbilityLost)` (`Abilities.lean:805`), used by `removedAbilities` (`:802`) and
  `addedSelectionOf` (`:801`). `CharacteristicEdit.addedAbilities (abilities : List Ability)`
  (`:794`) takes plain abilities. So "has all landwalk abilities of <source>" is already
  `addedSelectionOf src (.specified [.term landwalkAbilities])`. Card-side family aliases:
  `Cards/Keyword.lean:1162-1168`.
- Glossary: neither **Specified Ability** nor **Ability Spec** exists. **Ability Selection**
  (`docs/contexts/game-model/CONTEXT.md`) already names "the abilities an effect identifies
  individually, by an ability family, …"; **Static Spec** makes "Spec" the house word for
  declarative syntax; **Ability Category** avoids "Ability Class" for "the families a grant or a
  removal selects" (`AbilityClass.keyword k`, `Words.lean:436-443`).
- The Odric rulings are in `data/mtgjson` (`cardRulings`), dated 2016-04-08 ("Odric's ability
  triggers at the beginning of each combat …"; "The set of creatures affected by Odric's ability …
  is determined as the ability resolves") and 2020-01-24 ("If one of those creatures has one or
  more variants of the listed keywords (for example, hexproof from white), creatures you control
  gain those specific variants"). *Death-Mask Duplicant* (2004-12-01) and *Cairn Wanderer*
  (2007-10-01) gain "all landwalk and protection abilities" of the cards.

**The checker code to delete** (the ruling's list was not attached; this is the verified
inventory, all used only by `alsoForKeywords`):

- Pass-through arms: `Check/Phrase.lean:1320` (`writesDefining`), `:2138` (`textContextWith`),
  `:3880-3882` (`Ability.check`, the only place the two refusals fire);
  `Check/CardFacts.lean:173` (`namesThisDoor`); `Check/Card.lean:71,93,144,159,236,250,262,279`.
- The keyword-extension cluster: `Ability.grantedKeyword`, `StaticSpec.keyword`, `Action.keyword`,
  `Ability.lineKeyword`, `Ability.keywordExtendable`, `keywordListOk`
  (`Check/CardFacts.lean:198-229`); `KeywordTerm.bare` (`Check/Keywords.lean:113-116`); and
  `keywordParamless` (`:97-98`), dead once the cluster goes.
- Refusals `keywordExtendable` and `keywordListOk` (`Check/Refusal.lean:150-151`).

## Ruling (owner, 2026-10-22, second batch): option B

`alsoForKeywords` grants family terms (landwalk, protection, hexproof) through the existing
`CharacteristicEdit.addedSelectionOf` (VERIFY: true, `Abilities.lean:804`, not `~615`) over the
objects the condition found: "the abilities of X", as the 2020 ruling and *Death-Mask Duplicant*'s
and *Cairn Wanderer*'s rulings read. `addedAbilities` does not carry unresolved family terms, so a
family term never goes inside it.

- Per keyword, the Template's body function maps a bare keyword to a plain grant, and a family term
  to `addedSelectionOf <the objects the condition found> (.specified [.term <family>])`
  (`AbilitySelection.specified (abilities : List AbilityLost)`, `Abilities.lean:805`, where
  `AbilityLost.term` carries the `KeywordTerm.anyIn` family). *Concerted Effort*, *Death-Mask
  Duplicant* and *Cairn Wanderer* (landwalk, protection) use it.
- In the same landing, *Odric, Lunarch Marshal*, *Bleeding Effect* and *Urborg Scavengers* use the
  hexproof family term [CR#702.11d] (they write `.the (.named "hexproof")` today): add
  `hexproofAbilities : KeywordTerm := .anyIn ⟨hexproof, none⟩` beside `protectionAbilities` and
  `landwalkAbilities` (`Cards/Keyword.lean:1162-1163`), and use it in their lists
  (`Cards/Turn.lean:280,294`, `Cards/Counters.lean:745`).
- **Is the `AbilityLost` → `SpecifiedAbility` rename needed? No.** Family terms already reach a
  grant through `addedSelectionOf` (a `specified` selection holding `AbilityLost.term (anyIn …)`),
  and nothing in the Template, the cards or the checker needs a new name or a new type for that.
- **Is the `addedAbilities` retype needed? No.** Option B leaves `addedAbilities (abilities : List
  Ability)` (`Abilities.lean:794`) as it is. The rename and the retype are dropped from this ticket.
  (`AbilityLost` is also used for additions by `addedSelectionOf`, so its name is a pre-existing wart;
  that is a naming question for a future glossary pass, not a prerequisite here, and not ticketed
  by this ruling.)

## Change

1. Re-body `alsoForKeywords` (`Templates.lean:1940`) to take the per-keyword body as a function of
   the keyword and produce one Ability: triggered → one `.triggered` with `sequentially` of the
   per-keyword steps; static → one `.static (conjunction …)`. The base keyword stays first, in
   Oracle order. Delete the constructor and every arm and function listed above.
2. Rewrite the seven cards over the Template (their source may not change if the Template keeps
   its signature).
3. Family terms and the hexproof cards, as ruled above: the Template's family-term arm goes
   through `addedSelectionOf`; no rename, no retype, no change to `addedAbilities`.
4. `lean/CONTRACTS.md:724` (the `AbilityLost` row; it names `stepCharacteristicEditWith`, but the
   stepping is `stepAbilitySelectionWith`).

## Tests

- Retire `badKeywordListOnPlainLine`, `badEmptyKeywordList`, `badKeywordListRepeatingBase` and
  `badParameterisedKeywordInList` (`Proofs/Keyword.lean:205,211,214,220`): they pin
  `keywordExtendable` and `keywordListOk`, which the ruling deletes with the wrapper. Where a law
  survives in the Template's expansion (an empty list → `nonEmpty` from `sequentially` or
  `conjunction`), rewrite the pin onto it instead, and say which.
- Rewrite `okKeywordListOverGrant` (`Proofs/Keyword.lean:198`; keep `flyingWhileExiledFlying`,
  `:192`, as the base body) against the expansion.
- Rewrite `conditionIsInTheEffect` and `trailingIfIsInTheEffect` (`Proofs/Trigger.lean:328,340`):
  the claim that the trailing "if" is no intervening "if" [CR#603.4] becomes "each `sequentially`
  step is a `doIf`".
- Add `odricIsOneAbility`: the expansion for *Odric* is a single `.triggered` (the card has one
  ability line) whose action is `sequentially` over 12 steps.
- Add `urborgScavengersIsOneStatic`: one `.static (conjunction …)`.
- Add `familyTermsGrantThroughAddedSelectionOf`: for *Death-Mask Duplicant* and *Cairn Wanderer*
  the landwalk and protection entries are `addedSelectionOf` over the cards' objects with a `.term
  (anyIn …)` selection, and no `addedAbilities` holds a family term. Add `odricGrantsHexproofVariants`
  (hexproof is the family term in *Odric*, *Bleeding Effect* and *Urborg Scavengers*, [CR#702.11d]).
- The retype's regression list (`specifiedAbilityRemoval` and `:180`
  `Proofs/CharacteristicEdits.lean:43`, `Proofs/SelectionEvidence.lean:229,236`,
  `Proofs/Rules.lean:158`, `Proofs/KeywordAbilityCensus.lean:73`,
  `grantedAbilityReadsChoicesButNotOrdinaryOuterObjects`) is not touched: nothing it pins changes.

## Other tickets on the same files

- `lean-v3-cast-predicates-and-ordinals`, `lean-v3-cause-flag-fold` and
  `lean-v3-alternative-cost-unification` edit `Cards/Cost.lean`, `Cards/Keyword.lean`,
  `Cards/Counters.lean` and `Proofs/Trigger.lean` (other cards and pins).
- `docs/tickets/planned/english-v4-stratum-7-discourse-and-long-tail.md` profiles the "same is
  true" units on the English side; `docs-glossary-cr-corrections` touches Keyword Variant
  [CR#702.1c].
