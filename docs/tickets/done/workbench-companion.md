---
needs: []
---
**Add a small `DeckCondition` sort so Companion's deck restriction has a
spelling [CR#702.139a].** Fresh workbench review 2026-09-03, R3, resolved by
ruling.

**Ruling (settled 2026-09-03): Companion is a deck condition, a sort of its
own.** The condition is a property of the starting deck, evaluated outside the
game, so it is not a `Condition bs` over game state and not a `Predicate` over
an object in a zone. It is a small sort — quantified restrictions over the
cards in the starting deck ("every card has X", "no card has X", a bound on a
characteristic) — read by the Companion keyword row and nothing else.

10 supported cards print Companion.

The sort stays small on purpose: it admits the shapes the ten printed
conditions need and no general query language. A condition the CR does not
make meaningful for a starting deck is refused, and the pin names the reason.

The Companion `keywordFacts` row gains the condition as its parameter; the
shape list from `workbench-facts-from-ron` is the natural home for it if that
ticket has landed, and a one-off row otherwise — this ticket does not depend
on it.

Size: M.

Done when: all ten printed companions are typechecking bench witnesses, or the
landing record names each one left out and why; the `DeckCondition` sort
admits nothing beyond the shapes those cards need; a pin refuses a deck
condition over game state (a zone read, a battlefield object) as
CR-meaningless for a starting deck, probed non-vacuous; the build is 44/44
with 0 errors and 0 warnings. Standard constraints apply, plus the RON-shaped
constraint: a core constructor is admissible only if the RON re-emitter can
produce it from a RON node, and a macro only if it names a RON macro
(`docs/decisions/workbench-ron-shaped-and-label-rulings.md`).

## As landed

**The sort.** `DeckCondition` and its two helper sorts live in `Effect.idr`'s
mutual block, immediately above `KeywordParam` (placement deviation below).
The card side of every constructor is an ordinary `Predicate [] Object` gated
by `DeckReadable` — a card set aside for the starting deck is outside the game
[CR#103.2b], so only its characteristics are readable [CR#109.3].

`DeckCondition`, and the companion each constructor serves:

- `EveryCardIs (scope : Predicate [] Object) (trait : DeckTrait)` — Gyruda,
  Kaheera, Keruga, Lurrus, Obosh, Zirda.
- `NoCardIs (scope : Predicate [] Object) (trait : DeckTrait)` — Jegantha.
- `CardsDiffer (scope : Predicate [] Object) (ax : QualitySort)` — Lutri.
- `CardsShare (scope : Predicate [] Object) (ax : QualitySort)` — Umori.
- `DeckSizeOverMinimum (extra : Nat)` — Yorion.

`DeckTrait`, the one-card side:

- `ACharacteristic (p : Predicate [] Object)` (gated `DeckReadable`) — Kaheera,
  Keruga, Lurrus, Obosh.
- `ManaValueParity (par : ManaParity)` (`EvenValue`/`OddValue`) — Gyruda, Obosh.
- `RepeatedManaSymbol` — Jegantha.
- `HasAbilityOf (cls : AbilityClass)` — Zirda.
- `AnyTraitOf (ts : List DeckTrait)` — Keruga, Obosh ("… and land cards").

The two gates:

- `DeckReadable p` (`ReadsCharacteristics`, over `deckReadable p = True`)
  admits `IsCard`, `Permanent`, `HasType`, `HasSubtype`, a `Compare` on
  `[CharAxis ManaValue]` against a `Lit` bound, and `And`/`Not` over those —
  the four scope shapes and two requirement shapes the printed conditions
  need, and nothing else.
- `DeckComparable ax` (`ComparesCharacteristic`, over `deckComparable ax =
  True`) admits `CardName` and `CardTypeQ`, the two characteristics Lutri and
  Umori compare across the deck.

**Omissions: none.** All ten supported printings are typechecking bench
witnesses in `Cards/Keyword.idr` (`gyrudaCompanion`, `jeganthaCompanion`,
`kaheeraCompanion`, `kerugaCompanion`, `lurrusCompanion`, `lutriCompanion`,
`oboshCompanion`, `umoriCompanion`, `yorionCompanion`, `zirdaCompanion`).
Three further printings of Companion in the card data are unsupported and are
not bench cards: Lutri, Pauper Otter and The Companion of the Wilds (both read
an expansion symbol or set, not a characteristic [CR#109.3]) and Treizeci, Sun
of Serra ("nostalgic", a card-frame reading). The bench `lutriCompanion` is
Lutri, the Spellchaser.

**The row and the overlay.** `KeywordShapes.KeywordParamShape` gained
`DeckConditionParam` (index 7); `Effect.KeywordParam` gained
`ParamDeckCondition`, and `paramShapeOf` maps it to the new shape. In
`crates/xtask/src/facts.rs` `Shape::DeckCondition` renders as
`DeckConditionParam` and `Shape::from_params` maps the stub signature
`["Condition"]` to it — `Companion.ron` is the only `params: [Condition]` stub
in the tree. The overlay gained `Row { label: "Companion", ..D }`: every gate
column is at its default, so the row's one derived column is
`paramShapes := [DeckConditionParam]`. `Macros.companion` spells the ability;
`companion` is the stub's own `spelling`.

## Landing record

Numbers before → after: `keywordFacts` 96 → 97 rows; `KeywordParamShape` 7 → 8
shapes; Idris modules 46 → 46; bench witnesses in `Cards/Keyword.idr` +10;
`ProofsKeyword` pins +3 with +2 twins. Coverage lock untouched; no
construction added or retired.

Gates (all foreground):

- `cd idris && ./scripts/build` (clean tree, `build/` removed first) —
  `46/46: Building Cards (src/Cards.idr)`, exit 0, 0 `Error`/`Warning` lines,
  `real 0m54.196s`
- `cargo xtask facts check` — `…/idris/src/Experimental/FactsGen.idr is up to
  date`
- `cargo xtask facts labels` — `keyword abilities: 195 stubs, 97 rows, 101
  stubs without a row, 3 rows without a stub` (exit 1: the pre-existing
  worklist recorded by `workbench-facts-from-ron`, one stub closer)
- `cargo test -p xtask` — `test result: ok. 441 passed; 0 failed; 1 ignored`
- `cargo fmt --check` — clean; `cargo clippy -p xtask --all-targets` — 0
  warnings
- `cargo xtask cite check --list-noncompliant` — `0 non-compliant
  citation-looking string(s)`
- `cargo xtask cite check` — `checked 14188 citations against cr.txt
  (eff. 2026-08-07); 0 stale`
- `cargo xtask cite bless` — `blessed 1415 rules`; newly registered
  [CR#702.139b] and [CR#100.2b], each read against the claim citing it
- `cargo xtask cite audit --diff` — `audited 26 citation site(s)` over the
  whole diff (15 in the Idris sources, the rest re-cited in this record), each rule
  read against its claim ([CR#103.2b] the revealed card remains outside the
  game, [CR#109.3] the characteristics list and its tapped/controller
  exclusions, [CR#400.1] a zone is a place objects are during a game,
  [CR#202.3] mana value, [CR#702.139a,702.139b] Companion and the
  after-sideboard deck, [CR#100.2a,100.2b] the format's minimum deck size)

Pin probes (foreground, scratch module typechecked and removed):

- `EveryCardIs (InZone Macros.battlefieldZ) …` — `Can't find an implementation
  for DeckReadable (InZone (ZoneAt Battlefield Bare)).`
- `ACharacteristic (HasStatus Tapped)` — `Can't find an implementation for
  DeckReadable (HasStatus Tapped).`
- `CardsShare (And [Not Macros.land, IsCard]) Color` — `Can't find an
  implementation for DeckComparable Color.`
- Mis-stated once: the same `Unspellable` asserted against the twin's own
  characteristic read fails with `misstated ReadsCharacteristics is not a
  valid impossible case.`, so the three pins refuse something.
- The row is the gate, not the witnesses: `Macros.keyword "Companion"` and
  `KeywordAbility "Flying" (Just (ParamDeckCondition …))` are both refused by
  the pre-existing `KeywordParamFits`.

Assurance counts: restored 0; re-spelled 0; ignored with blockers 0; added 15
(10 bench witnesses, 3 pins, 2 positive twins); removed 0.

Deviations and additions:

- **Placement.** The brief put the sort in `KeywordShapes.idr`/`Words.idr`; it
  is in `Effect.idr` instead. `DeckReadable` gates a `Predicate [] Object`,
  and `Predicate` is defined in `Phrase.idr`, which follows `Words.idr` in the
  module order — `Words` cannot see it. Only the shape enum entry
  (`DeckConditionParam`) went into `KeywordShapes.idr`, which the generated
  `FactsGen` needs. No new module; the count stays 46.
- **The card side reuses `Predicate`, gated, rather than a fresh vocabulary.**
  The ticket requires a pin refusing a game-state read as CR-meaningless. A
  self-contained trait vocabulary would make that term unwritable rather than
  refused, and the pin would be vacuous. Reusing `Predicate` under
  `DeckReadable` is what makes `EveryCardIs (InZone …)` a refusal.
- **`DeckTrait` carries three readings that are not `Predicate`
  constructors** — `ManaValueParity`, `RepeatedManaSymbol`, `HasAbilityOf`.
  The predicate vocabulary carries none of them, and no non-companion printed
  card asks for them, so they are companion-local rather than three new
  `Predicate` constructors in a shared region.
- **`AnyTraitOf` is the sort's only disjunction**, added for Keruga's and
  Obosh's "… and land cards". `deckReadable` admits no `Or` at the predicate
  level: the printed scopes need only `And` ("each permanent card", "each
  creature card") and `Not` ("each nonland card").
- **A second gate, `DeckComparable`, beyond the ticket's letter.** The ticket
  asked for one pin. `CardsDiffer`/`CardsShare` would otherwise take any
  `QualitySort`, including `CounterKindQ`, which is meaningless for a card
  outside the game; the gate holds the sort to Lutri's name and Umori's card
  type. Pinned and probed above.
- **`DeckSizeOverMinimum` takes a `Nat`, not an `Amount`.** Yorion's twenty is
  a literal, no printed companion reads a game quantity, and `Amount` carries
  game-state reads that would need a gate of their own.
- **`cargo xtask cite bless` pruned three unrelated lock entries** — three
  rule-718 keys, visible in the `cr-citations.lock` diff, for rules no longer
  cited anywhere in the tree. The bless output is kept as generated rather
  than hand-restored, since the same run registered the newly cited rules.

STOP taken: none.
