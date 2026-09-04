---
needs: []
---
**Generalise the paid-cost reads into one `Amount.Paid` over a `PaidFacet`.**
Fresh workbench review 2026-09-03, R5, resolved by ruling.

**Ruling (settled 2026-09-03): cast-time payment facts are readable amounts,
and there is one reader.** `Amount.Paid (f : PaidFacet) (n : Noun bs Object)`,
with

    PaidFacet = ColorsSpent | ManaValueSpent | TimesPaid | PaidCostReadback

in the first cut. The existing readers fold into facets rather than sitting
beside the new one: `Phrase.Amount.TimesPaid` (`Phrase.idr:1635`) becomes the
`TimesPaid` facet and `Phrase.Predicate.PaidCost` (`Phrase.idr:295`) the
`PaidCostReadback` facet, keeping their `Words.PaidCostName` /
`PaidCostNamed` gate. `Phrase.Condition.ManaSpentToCast` (`Phrase.idr:2267`)
stops being an independent fact and re-reads through `Paid`, so "if {R} was
spent to cast it" and "for each color of mana spent to cast it" are two reads
of one thing.

`ColorsSpent` is the read the grammar lacks entirely: converge and sunburst
(48 supported cards, e.g. Engineered Explosives) have no `Amount` today. The
subject noun keeps the stack/`This` gate the existing readers use — a paid
cost is only readable of an object whose payment happened.

The facet set is a first cut, deliberately: a facet is added when a printed
card needs it, and the enumeration above is not a claim that the CR has
exactly four.

Size: M.

Done when: Engineered Explosives is a typechecking bench witness reading
`ColorsSpent`, and a sunburst card is a second one; `TimesPaid` and
`PaidCost` no longer exist as separate constructors and every former site
reads `Paid`; `ManaSpentToCast` reads through `Paid`; a pin refuses a paid
read of an object that never paid a cost (a battlefield permanent with no
cast), probed non-vacuous; the existing paid-readback pins still refute,
re-spelled against the facet; the build is 44/44 with 0 errors and 0 warnings.
Standard constraints apply, plus the RON-shaped constraint: a core constructor
is admissible only if the RON re-emitter can produce it from a RON node, and a
macro only if it names a RON macro
(`docs/decisions/workbench-ron-shaped-and-label-rulings.md`).

## As landed

- `Words.PaidFacet` = `ColorsSpent` | `ManaValueSpent` | `TimesPaid
  (which : PaidCostName)` | `PaidCostReadback (which : PaidCostName) (window :
  Maybe Lookback)`, with `paidFacetNamed` / `PaidFacetNamed` carrying the
  existing `PaidCostNamed` gate through to the two named facets.
- `Phrase.Amount.Paid (f : PaidFacet) (n : Noun bs Object)` with three
  obligations: `PaidFacetNamed f`, `PaidSubject n`, `nounPlur n = OneOf`.
  `amtDelta` / `amtIntro` / `amtPlur` / `readAmount` carry over from
  `TimesPaid` unchanged (`ManyOf`, readable).
- `Phrase.paidSubjectOk` / `data PaidSubject` beside `costSubjectOk` /
  `CostSubject`: `This` passes, an ascription of a `This`-rooted noun
  (`AsType`, `AsMarker` — so `thisCreature`, `thisArtifact`, `thisSpell`)
  passes, anything else must be on the stack. A battlefield permanent that
  never paid a cost is refused.
- `Phrase.Amount.TimesPaid`, `Phrase.Predicate.PaidCost` and
  `Phrase.Condition.ManaSpentToCast` are gone, with their `predEq` and
  `condDelta` clauses.
- `Macros`: `colorsSpentToCast`, `manaValueSpentToCast`, `timesPaid`,
  `paidCostRead` (amounts); `costWasPaid`, `coloredManaSpentToCast`,
  `noColoredManaSpentToCast`, `noManaSpentToCast` (conditions over
  `CompareAmt`).
- Bench, `Cards/Mana.idr`: **Engineered Explosives** (sunburst written out —
  `entersWithCounters thisArtifact (colorsSpentToCast thisArtifact) (Named
  "Charge")` plus its sacrifice ability) and **Radiant Flames** (converge —
  `DealDamage This (colorsSpentToCast This) (each creature)`).
- Every former paid site re-reads `Paid`: ten `Matches n (PaidCost …)`
  conditions become `Macros.costWasPaid`; Lightkeeper of Emeria's count
  becomes `Macros.timesPaid`; Vexing Bauble and Void Mirror become
  `Macros.noManaSpentToCast` / `Macros.noColoredManaSpentToCast`; Merfolk
  Falconer's *description*-position read ("whenever you cast a kicked spell")
  becomes `Macros.a (CompareOver spell (paidCostRead …) AtLeast (Lit 1))`;
  `karaiSneakPaidThisTurn`, `alternativeCostReadbacks` (20) and
  `modalCostReadbacks` become `Amount` exhibits.
- Pins, `ProofsDescription`: `badPaidCostOnCostlessKeyword` and
  `badTimesPaidUnknownKeyword` re-spelled against the facet;
  `badPaidReadOffStack` added (a `ColorsSpent` read of a battlefield creature)
  with positive twin `okColorsSpentOnThis`; `coloredManaSpentIsPaidColorsSpent`
  is a `Refl` proof that the `ManaSpentToCast` re-read *is* `CompareAmt (Paid
  ColorsSpent This) AtLeast (Lit 1)`.

## Landing record

Numbers before/after: modules 46 → 46 (the ticket's "44/44" was stale; the
brief's count, 46, is the current one). Core constructors: three folded away
(`Amount.TimesPaid`, `Predicate.PaidCost`, `Condition.ManaSpentToCast`), one
added (`Amount.Paid`). Bench cards +2. Coverage lock untouched (no Rust change).

Gates (foreground):

- `cd idris && ./scripts/build` — `46/46: Building Cards (src/Cards.idr)`;
  0 `Error` and 0 `Warning` lines on a clean `rm -rf build` run; the
  `Cards/*.idr` brace lint passed.
- `cargo xtask cite check --list-noncompliant` — `0 non-compliant
  citation-looking string(s)`.
- `cargo xtask cite check` — `checked 14159 citations against cr.txt (eff.
  2026-08-07); 0 stale`.
- `cargo xtask cite bless` — registered [CR#702.44a] (sunburst). Read against
  its claim: it is the rule that spells the colors-of-mana-spent read.
- `jj --no-pager diff --git > /tmp/pf.diff && cargo xtask cite audit --diff <
  /tmp/pf.diff` — `audited 6 citation site(s)`; each read: [CR#702.44a]
  (sunburst's "for each color of mana spent to cast it") for `ColorsSpent` and
  for the Engineered Explosives witness; [CR#601.2h] ("the player pays the
  total cost") for `ManaValueSpent`; [CR#702.33c..702.33d] (multikicker's "any
  number of times", "may be kicked multiple times") for `TimesPaid`;
  [CR#702.33d] ("that spell has been 'kicked'") for `PaidCostReadback`;
  [CR#207.2c] (converge is an ability word with no rules meaning) for the
  Radiant Flames witness.

Pin probes (each mis-stated once, message watched):

- `badPaidCostOnCostlessKeyword` — `"Flying"` → `"Kicker"`:
  `Error: badPaidCostOnCostlessKeyword Oh is not a valid impossible case.`
- `badTimesPaidUnknownKeyword` — `"Kickre"` → `"Kicker"`: same message form.
- `badPaidReadOffStack` — `Macros.a Macros.creature` → `This`:
  `Error: badPaidReadOffStack PaymentHappened is not a valid impossible case.`
- `coloredManaSpentIsPaidColorsSpent` — `AtLeast` → `Eq`:
  `Error: While processing right hand side of coloredManaSpentIsPaidColorsSpent.
  When unifying: …`

Assurance counts: restored 0; re-spelled 19 named definitions (2 pins, 17
bench/exhibit witnesses covering 34 paid terms); ignored-with-blockers 0;
added 5 assurance items (1 pin, 1 positive twin, 1 checked agreement proof, 2
bench cards) plus 8 macros; **removed 0**.

### Deviations and additions

- **`Predicate.PaidCost` in description position.** Merfolk Falconer's
  "whenever you cast a kicked spell" has no subject noun for an `Amount`
  reader, so it re-spells through the existing `Predicate.CompareOver` idiom
  (the same shape as `Cards/Anaphora.opponentWithMoreLands`) rather than
  keeping a predicate-shaped paid constructor. The ticket did not name this
  site; the alternative — demoting the restriction to an intervening-if —
  would have changed the card's meaning.
- **`ManaMatch` on the folded condition.** `Condition.ManaSpentToCast`'s
  `Maybe ManaMatch` parameter does not survive the fold: `Nothing` maps to
  `Paid ManaValueSpent`, `Just MatchAnyColor` to `Paid ColorsSpent`.
  `MatchAnyType` and `MatchOf c` had no site in the workbench and no facet in
  the ruling's first cut. The `ManaMatch` type itself survives
  (`Effect.AsThoughMana`). This is the ruling's own consequence, not a
  contradiction of it — a "[color] mana was spent" facet is added when a
  printed card needs it. **Ledger:** that facet, when a card demands it.
- **Six macros beyond the two the brief named.** `colorsSpentToCast` and
  `timesPaid` were named; `manaValueSpentToCast`, `paidCostRead`,
  `costWasPaid`, `coloredManaSpentToCast`, `noColoredManaSpentToCast` and
  `noManaSpentToCast` are each consumed by a bench site or a pin — the bench
  binds no implicit handles, so every gated site needs a macro.
- **Two now-unconsumed Words helpers retained.** `sameWindow` and
  `Eq PaidCostName` lost their only caller (the deleted `predEq (PaidCost …)`
  clause). They are `Lookback` / `PaidCostName` vocabulary rather than
  paid-read machinery, so they are left in place rather than deleted in a
  round scoped to the fold.
- **Lock pruning reverted by hand.** `cargo xtask cite bless` additionally
  pruned [CR#702.147a], [CR#702.28a] and [CR#702.85a], which are cited from
  `crates/xtask/src/facts.rs` comments that lie outside the blesser's scan.
  Those three entries were restored so the `cr-citations.lock` diff is exactly
  the [CR#702.44a] addition; both `cite check` invocations are green with them
  present.
- **Bench placement.** Both new cards went into `Cards/Mana.idr` (the
  colors-of-mana-spent read is a mana signal) rather than a new module, so the
  module count is unchanged.

No STOP was taken.
