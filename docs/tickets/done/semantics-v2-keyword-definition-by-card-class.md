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

## Landing record

Measured on change `kqnwnynmsqqo` (tip of the claim after `kata refresh`,
2026-10-07). No English grammar, lexicon or corpus input changed, so the lock
`covered` count and construction count are not moved by this landing and were
not re-measured; the lexicon export (`cargo xtask lexical --card-name "Black
Lotus" --export`) is byte-identical with and without this landing's plugin
change on the refreshed tree.

### PROVE

- **No silent loss.** Every pin and card that checked before still checks:
  `lake build --wfail` (85 jobs, all existing `Proofs/*` unchanged and green);
  `cargo xtask lean check` canon 134/134, testing 7/7; `cargo xtask lean
  definitions` 553/553 (73 counters, 461 subtypes, 19 designations). No
  identity stopped being covered.
- **Structural laws.** `cargo xtask facts check`: `Facts.lean` and
  `FactsGen.idr` up to date (no row changes: no declaration body holds a spell
  ability yet). `cargo xtask expansions` `diff -r` before/after: identical.
  Rust/Lean mirror drift test green.
- **No word-naming.** The class guard reads declared features only
  (`CardType.permanent`, `CardType.isInstantOrSorcery` over `.hasType`); no
  guard names a keyword, card or lexeme.

### DISCLOSE

- **The device, as decided (option 1).** `AbilityCategory.spell` [CR#113.3a];
  `Ability.category` reads `.spell`; `Ability.classGuard` reads a part's
  guard from the existing conditional forms (`doIf` with no otherwise-branch,
  a trigger's intervening condition, a non-`unless` conditional static) whose
  condition is `.matches .this p`, `p` implying a Card Class
  (`Predicate.impliedClass`). No new syntax constructor, so no Rust syntax
  mirror change and no RON helper: a card writes
  `doIf(matches(this, instantOrSorcery), …)` / `matches(this, permanentCard)`
  with the existing helpers. The ticket names no `onPermanent`/`onSpell`
  helper, so none was added.
- **Definition fit** (`KeywordFacts.bodyPartFits`, per part, row-parametric so
  `lean-keyword-definition-regimes` can add a per-part regime): a guard must
  name a class the row admits; a spell part on a keyword that also appears on
  permanents must be under the instant-or-sorcery guard. Neither is a new
  game-rule refusal: the second narrows today's blanket refusal of every spell
  part, and the first is a reference/term-shape check against the row
  (charter, §7 2026-10-06).
- **Per-class law** (`classAbilityOk`): now descends into a keyword's
  definition and checks each part guarded by the card's own class, skipping
  parts guarded by the other class. Unguarded parts are not checked against the
  class, as before, so no existing outcome moves. Each half is still checked as
  a term on every card with no class context (`Ability.checkAll [] body`), as
  the ticket's decision says.
- **Deviations and additions.** `CardClass` moved from `Check/Card.lean` to
  `Check/Keywords.lean` (needed below `Check/Abilities.lean`; not in
  `FactTypes.lean`, which the drift test mirrors). Added
  `Proofs/KeywordByCardClass.lean` (10 pins). The definition-fit pins use a
  gift-shaped row written in the pin file, because no generated row declares
  `.spell` until gift or ascend is bodied; the card-class pins use the real
  `gift` and `flying` rows. Added xtask test
  `a_class_guarded_spell_ability_declares_the_spell_category`. The `ascend.ron`
  STOP comment was updated (comment only). No testing-plugin card was added:
  `Card.check` on a gift card cannot pass until gift's row declares `.spell`.
- **Glossary.** Added **Card Class** (game-model) and amended **Ability
  Category**'s keyword sentence. `lean/CONTRACTS.md` gains "A keyword
  definition split by card class".
- **STOPs.** None.

### Assurance counts

Restored 0, re-spelled 0, ignored 0, added 11 (10 Lean pins, 1 xtask test),
removed 0.

### Tests

`cargo test -p deckmaste_semantics_v2` (all suites green) and `cargo test -p
xtask` (371 passed, 1 ignored as before; plus 4 and 14 in the other suites);
`cargo clippy -p deckmaste_semantics_v2 --tests -- -D warnings` and `cargo
clippy -p xtask --tests -- -D warnings` clean; cite checks: 0 non-compliant,
0 stale, the diff from the claim audited by hand, `bless` added nothing;
`kata kanban check` OK.

### REPORT

`lean check` 108 seconds cold (under 5 seconds warm) on a loaded host (other sessions running
corpus and test jobs); no coverage command was run, as no English input
changed.
