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
- Rename `AbilityLost` to `SpecifiedAbility` (or `AbilitySpec`), and make
  `CharacteristicEdit.addedAbilities` take it, so family terms (landwalk, protection) work.
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

## STOP item (a ruling premise needs the owner)

**A family term in `addedAbilities` names no variants.** The rulings above say the grant gives
the variants that the condition's objects have ("those specific variants", "the landwalk and
protection abilities of the cards"). `addedAbilities [.term (anyIn landwalk)]` says "landwalk"
with no possessor; `addedSelectionOf source (.specified …)` already says "the abilities of X".
Before retyping `addedAbilities`, the owner confirms which: the family term inside
`addedAbilities` denotes the variants the triggering condition's objects have (and the engine
reads them there), or the per-keyword body grants through `addedSelectionOf` over the objects
the condition found. The Template and the bare-keyword cards do not wait on this.

Related, and to fix in the same landing: *Odric*, *Bleeding Effect* and *Urborg Scavengers* write
hexproof as `.the (.named "hexproof")`, but hexproof has variants
(`Definitions/Keywords.lean:857-873`; [CR#702.11d]), so by [CR#702.1c] and the 2020 ruling their
grant is a family term too.

## Change

1. Re-body `alsoForKeywords` (`Templates.lean:1940`) to take the per-keyword body as a function of
   the keyword and produce one Ability: triggered → one `.triggered` with `sequentially` of the
   per-keyword steps; static → one `.static (conjunction …)`. The base keyword stays first, in
   Oracle order. Delete the constructor and every arm and function listed above.
2. Rewrite the seven cards over the Template (their source may not change if the Template keeps
   its signature).
3. Rename `AbilityLost` and retype `addedAbilities` as ruled (after the STOP for family terms).
   Name: add the term to the glossary through the `domain-modeling` skill first; check it against
   **Ability Selection** (one Game Model term per concept: if **Ability Selection** already covers
   it, the identifier follows that entry instead), and honour **Ability Category**'s `_Avoid_`.
   Consumers: `Check/Phrase.lean:1397-1404` (`stepAbilitySelectionWith`), `:1415-1422`, `:3643`;
   `Primitives.lean:48-49` (`loses`, `adds`); `Definitions/Keywords.lean:280,629`;
   `Definitions/Actions.lean:665`; `Templates.lean:51-52,351,1034,1413-1414,1922-1923,2273-2276`;
   `Check/CardFacts.lean:102,205`; `Check/AbilityRules.lean:155,159`; cards
   `Cards/Damage.lean:1231`, `Cards/Cost.lean:889,896-898,908-909`, `Cards/Mana.lean:1040-1041`.
   If `addedAbilities` takes the new type, `.term (.the k)` beside `.written (.keyword k …)` is two
   encodings of "has flying": give them a law (or restrict `.term` there to families).
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
- If `addedAbilities` is retyped: `specifiedAbilityRemoval` and `:180`
  (`Proofs/CharacteristicEdits.lean:43`), `Proofs/SelectionEvidence.lean:229,236`,
  `Proofs/Rules.lean:158`, `Proofs/KeywordAbilityCensus.lean:73`,
  `grantedAbilityReadsChoicesButNotOrdinaryOuterObjects` (`Check/AbilityRules.lean:153`), claims
  unchanged.

## Other tickets on the same files

- `lean-v3-cast-predicates-and-ordinals`, `lean-v3-cause-flag-fold` and
  `lean-v3-alternative-cost-unification` edit `Cards/Cost.lean`, `Cards/Keyword.lean`,
  `Cards/Counters.lean` and `Proofs/Trigger.lean` (other cards and pins).
- `docs/tickets/planned/english-v4-stratum-7-discourse-and-long-tail.md` profiles the "same is
  true" units on the English side; `docs-glossary-cr-corrections` touches Keyword Variant
  [CR#702.1c].
