---
needs: []
---
**A player chosen while paying a cost cannot be read back by the ability
linked to it.** Found by the stopped `plugins-v2-gift-variants` attempt
(2026-10-05). Standard constraints apply. The device was decided 2026-10-06:
option 1, the paid-cost facet (below).

## Evidence

Gift's first ability is "As an additional cost to cast this spell, you may
choose an opponent" [CR#702.174a]; its second hands the effect to "the chosen
player" [CR#702.174d..702.174i]. The rules link the two: "If an object has an
ability printed on it that causes a player to 'choose a [value]' and an
ability printed on it that refers to 'the chosen [value],' 'the last chosen
[value],' or similar, those abilities are linked. The second ability refers
only to a choice made as a result of the first ability." [CR#607.2d]

In the model the second ability writes `chosenPlayer`
(`lean/Semantics/Macros.lean` ~L218, `.chosenPlayer .theChoice`), which
reads a choice made in the same ability. The opponent chosen during casting
is not in that scope, so the checker refuses it as `choiceRef theChoice
player 0` (`lean/Semantics/Check/Refusal.lean`: no choice found). Nothing
reads a player back from a paid cost: `PaidFacet`
(`lean/Semantics/Words.lean` ~L361) has only `colorsSpent`,
`manaValueSpent`, `timesPaid` and `readback` (whether a named cost was paid).
So gift's second ability, on a permanent or a spell, has no faithful way to
name its performer.

## The options put to the owner

1. **A paid-cost facet**: a `PaidFacet` (or `PaidCostName`-keyed reference)
   for "the player chosen as [named cost] was paid", read by the second ability
   through the same `byKeyword "Gift"` name `readback` already uses for "its
   gift cost was paid". Narrow; fits gift exactly.
2. **A linked-ability binding** [CR#607.1,607.2d]: the first ability publishes
   its choice under a link the keyword definition declares, and the second
   ability's `chosenPlayer` resolves through that link rather than its own
   scope. General; also serves other choose/"the chosen" pairs printed as two
   abilities.

Either way: a pin where gift's second ability reads the chosen opponent, and
a refusal where an unlinked ability reads `chosenPlayer` with no choice in
scope. Related engine-side store: `engine-linked-abilities` (v1 engine
`Reference::Linked` resolution), not a prerequisite here.

## Decided 2026-10-06

**Option 1, the paid-cost facet**, because that is how the rules link the two
abilities (`docs/decisions/semantics-v2.md` §7, ruling 2026-10-06 on gift and
the additional-cost keywords). Owner: "idk do what the CR says."

- Gift's first ability is a cost: "As an additional cost to cast this spell,
  you may choose an opponent." [CR#702.174a]. A printed ability that allows
  an additional cost and one that refers to whether it was paid are linked
  [CR#607.2i], as kicker's are [CR#702.33e]; a "choose" and "the chosen" pair
  is linked too [CR#607.2d].
- `PaidFacet` (`lean/Semantics/Words.lean`, `inductive PaidFacet`) gains a
  facet "the player chosen as [named cost] was paid", keyed by `PaidCostName`
  (same file) and read through `byKeyword` as `readback` already is for "its
  gift cost was paid". Mirror it in Rust and RON in the same landing.
- Option 2, a general linked-ability binding, is not needed for gift. The
  enters-time "choose X / the chosen X" pairs are a different family and stay
  with `engine-linked-abilities`.
- Pins as above: gift's second ability reads the chosen opponent through the
  facet, and an unlinked `chosenPlayer` with no choice in scope is still
  refused.

## Landing record

Measured on change `uxtzyuul` (tip of the claim after `kata refresh`,
2026-10-07; the refresh brought ticket files only). No English grammar,
lexicon or corpus input changed, so the lock `covered` count and construction
count are not moved and were not re-measured; the lexicon export (`cargo
xtask lexical --output … --card-name "Black Lotus" --export …`) is
byte-identical with and without this landing's plugin change.

### PROVE

- **No silent loss.** `lake build --wfail` (86 jobs; every existing
  `Proofs/*` pin unchanged and green); `cargo xtask lean check` canon
  134/134, testing 7/7; `cargo xtask lean definitions` 553/553 (73 counters,
  461 subtypes, 19 designations). No identity stopped being covered; no pin
  outside the new file changed outcome.
- **Structural laws.** `cargo xtask facts check`: `Facts.lean` and
  `FactsGen.idr` up to date (one row changed: gift gains
  `costChoosesPlayer := true`; the Idris table has no such column).
  `cargo xtask expansions` `diff -r` before/after for builtin, canon and
  testing: identical except the new `nouns/chosenAsPaid.expanded` and its
  `_index.txt` line (1508 → 1509 declarations). Rust/Lean mirror drift test
  green.
- **No word-naming.** The new check reads a declared registry column
  (`costChoosesPlayer`), never a keyword, card or lexeme.

### DISCLOSE

- **The device, as decided (option 1).** `PaidFacet.chosenPlayer which`, "the
  player chosen as [which] was paid" [CR#702.174a,607.2d,607.2i], keyed by
  `PaidCostName` and written `.byKeyword (.named "gift")`. Helper
  `chosenAsPaid(which, subject)` (Lean `semantic_macro` and
  `plugins_v2/builtin/macros/nouns/chosenAsPaid.ron`, same name; it reads as
  the CR's "chosen as … was paid"). Gift's second ability will write
  `act(chosenAsPaid(gift, this), effect)`.
- **Scope rule.** As kicker's readback: the cost is registry-named
  (`paidFacetNamed`) and the subject is the source or a spell on the stack
  (`paidSubject`). Kicker's readback has no per-card "is this keyword printed
  here" check, so neither does this one. Added: the named cost must choose a
  player when paid (`costChoosesPlayer`, gift's row only), else
  `paidChoiceInScope which` — a reference refusal (§7, "The checker's
  charter"); `.theAdditional` / `.theAlternative` declare no choice and are
  refused likewise. No game-rule refusal added.
- **Deviations and additions.**
  - `NounPhrase.paid facet subject` (Lean, Rust `NounPhrase::Paid`): a
    `PaidFacet` was read only by `Amount.paid`, and the chosen player is a
    noun phrase. Each reader refuses the other's facets (`paidFacetKind`, term
    shape). Its kind is player, singular, no zone, and it introduces a player
    binding, as `possessorOf` does.
  - New `KeywordFacts` column `costChoosesPlayer` (xtask overlay `Row`, Lean
    `FactTypes.lean`, Rust `facts.rs` mirror, `GATE_COLUMNS` text). Lean only.
  - Refusals `paidChoiceInScope` (anaphora and scope) and `paidFacetKind`
    (tables), not mirrored in Rust (`Refusal` has no mirror).
  - `Proofs/LinkedChoiceReadback.lean`, 11 pins. The two required: gift reads
    the chosen opponent (`okGiftReadsChosenPlayer`); a bare `chosenPlayer` with
    no choice in scope is refused `choiceRef theChoice player 0`
    (`badUnlinkedChosenPlayer`). Twins: kicker (no choice), flying (no cost),
    the additional cost, kind cross-reads, object context; both class-guarded
    halves check class-blind, fit a gift-shaped row, and pass `cardTextOk` on a
    creature and an instant.
  - ADR §12.1 recount: 413 = 313 declarations + 2 + 98.
- **Glossary.** Added **Linked Abilities** and **Paid-Cost Facet**
  (game-model). `lean/CONTRACTS.md` gains "The player chosen as a cost was
  paid".
- **STOPs.** None.

### Assurance counts

Restored 0, re-spelled 0, ignored 0, added 11 (Lean pins), removed 0.

### Tests

`cargo test -p deckmaste_semantics_v2` (43 + 7 + 1 + 4 + 41 passed);
`cargo test -p xtask` (371 passed, 1 ignored as before; 4 and 14 in the other
suites); `cargo clippy -p deckmaste_semantics_v2 --tests -- -D warnings` and
`cargo clippy -p xtask --tests -- -D warnings` clean; cite checks: 0
non-compliant, 0 stale, `bless` changed nothing, audit of the diff from the
claim commit read (29 sites); `kata kanban check` OK.

### REPORT

Performance advisory: `cargo xtask lean check` 1m47s wall on a host
running other sessions' workspaces. No coverage command was run (no English
input changed).
