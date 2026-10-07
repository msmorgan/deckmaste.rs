---
needs: []
---
**A keyword definition cannot say "this on a permanent, that on an instant or
sorcery".** Found by the stopped `plugins-v2-gift-variants` attempt
(2026-10-05), which wrote gift's faithful body and ran the checker on probe
cards. Standard constraints apply. The model device was decided 2026-10-06:
option 1 (below).

## Evidence

A keyword definition is one fixed list of abilities (`Ability.keyword`'s
`body`, `lean/Semantics/Abilities.lean`). Gift's second ability depends on the
card's class: "On a permanent, the second ability represented by gift is 'When
this permanent enters, if its gift cost was paid, [effect].' On an instant or
sorcery spell, the second ability represented by gift is 'If this spell's gift
cost was paid, [effect].'" [CR#702.174b]. The spell form is a spell ability
[CR#113.3a], and no keyword definition may hold one:

- `Ability.category` (`lean/Semantics/Check/Abilities.lean` ~L690) returns
  `none` for `.spell`; `AbilityCategory` (`lean/Semantics/Check/FactTypes.lean`
  ~L67) has only `static | triggered | activated`; so `keywordBodyPartFits`
  refuses the part and `keywordBodyFits "Gift"` fails.
- The facts generator refuses it too: `category` in
  `crates/xtask/src/facts/lean.rs` (~L208) bails "a keyword definition is
  static, triggered or activated" on an `Ability::Spell`.
- Even with a fourth category, one list holds both forms, so a permanent
  carries the spell ability and an instant the enters trigger unless each half
  is guarded (see the recommendation below).

## How definition parts are checked today

A keyword definition's parts are checked **per card occurrence, blind to the
card's class**. `Ability.check` on `.keyword k param body`
(`lean/Semantics/Check/AbilityRules.lean` ~L755) runs `Ability.checkAll []
body` and `keywordBodyFits k body` each time a card writes the keyword, with
empty bindings and no class context. The per-class law `classAbilityOk`
(`lean/Semantics/Check/Card.lean` ~L48) stops at the keyword: on a keyword it
checks only `keywordCardOk` (the row's `onPermanentCard` /
`onInstantOrSorceryCard`), never the body's parts. So a guarded half that can
never apply on one card class is still checked on every card, and must pass
on its own — the guard does not exempt it.

## Other keywords defined by card class

Search of CR section 702 for "on a permanent" / "on an instant or sorcery"
wording, with Vintage-legal card counts from `data/scryfall/oracle-cards.jsonl`
(permanent / instant-or-sorcery):

| Keyword | Rule | Permanent form | Spell form | Cards |
|---|---|---|---|---|
| Gift | [CR#702.174b] | enters trigger | spell ability | 4 / 21 |
| Ascend | [CR#702.131a,702.131b] | static ability | spell ability | 22 / 5 |
| Haunt | [CR#702.55a] | trigger "put into a graveyard from the battlefield" | trigger "put into a graveyard during its resolution" | 7 / 3 |

Ascend has the same category split as gift (its declaration already records
the spell form as refused by `Ability.category`). Haunt is triggered on both,
so it needs the selector but not a fourth category; it is separately blocked
on the haunting link (its declaration's STOP). Gift variants on both classes:
"Gift a card" (2 / 10) and "Gift a tapped Fish" (1 / 5); "Gift an Octopus" is
permanent-only (1), Food (3), Treasure (2) and extra turn (1) spell-only.

Checked and not class-dependent: Hexproof's "on a permanent" [CR#702.11b]
contrasts a permanent with a player, not card classes; Offspring and Squad are
on permanents only; Casualty, Replicate and Kicker appear on both but define
one text for both.

## The options put to the owner

How gift's two forms are declared. Options:

1. **One gift definition holding both halves, each guarded by the card's
   class with the existing conditional forms — orchestrator's
   recommendation.** The owner asked: "what about doIf or whatever, wouldn't
   that work?" It does handle the branching. The permanent half's effect sits
   under `doIf` (`lean/Semantics/Abilities.lean` ~L438), or the trigger's
   intervening condition, on a card-class condition such as `.matches`
   (`lean/Semantics/Phrase.lean` `Condition`) of this object against a
   permanent predicate, and the spell half under the instant-or-sorcery one.
   No selector device is needed. The card class is known only at the card,
   and a variant such as "Gift a card" appears on both permanents and
   instants. With the guards inside one definition, `gift(draw(1))` serves
   both without choosing. Branching is not what blocks gift. Two checker laws
   remain the real blockers:
   - a spell ability [CR#113.3a] admitted in a keyword definition
     (`Ability.category` / `AbilityCategory` / the facts generator, above);
   - the regime law for the enters trigger (`lean-keyword-definition-regimes`).

   Because parts are checked per card occurrence and class-blind (above),
   both guarded halves are checked on every card and each must pass on its
   own.
2. **A new selector device**: the definition carries per-class parts and the
   checker picks by the card it sits on. It is more machinery than option 1
   for the same branching, and it would also let the checker skip the half
   that cannot apply.
3. **Two internal keywords** (a permanent gift and a spell gift), each a fixed
   list. Each variant would need two declarations, or the card would have to
   name which one.
4. Something else the owner names.

## Decided 2026-10-06

**Option 1: one definition, each half guarded by the card's class with the
existing conditional forms** (`docs/decisions/semantics-v2.md` §7, ruling
2026-10-06 on gift and the additional-cost keywords). No selector device.
Both guarded halves are still checked on every card, class-blind, and each
must pass on its own. Vintage-legal cards that need it (permanent /
instant-or-sorcery): gift 4 / 21, ascend 22 / 5, haunt 7 / 3. Gift's other two
model gaps are `semantics-v2-linked-choice-readback` (the chosen player) and
`lean-keyword-definition-regimes` (each part's regime).

## The work

- Admit a spell ability [CR#113.3a] in a keyword definition: a fourth category
  (`spell`) in `AbilityCategory` and `Ability.category`, with the registry
  row's `definition` declaring it.
- Confirm a card-class condition can be written in the
  definition (a `.matches` of this object against a permanent /
  instant-or-sorcery predicate) and that each guarded half checks with no
  class context.
- The facts generator (`crates/xtask/src/facts/lean.rs` `category`) accepts
  the new category.
- Pins: a gift permanent and a gift instant each accepted. Each guarded half
  is checked on both card classes. Ascend is re-checked once it is bodied.
