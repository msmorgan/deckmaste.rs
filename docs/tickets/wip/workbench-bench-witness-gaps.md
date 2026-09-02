# workbench-bench-witness-gaps

Two corpus-real vocabulary arms with no bench or pin witness, surfaced by the
first `cargo xtask map idris-dead` run (idris-dead-constructor-audit, done
2026-08-27). Each wants a bench card, not a pin — nothing about them is
rules-meaningless.

- `ChapterNumber`'s `ChapterIV`/`ChapterV`/`ChapterVI` — chapter markers occur
  in supported Saga text (35 / 2 / 3 occurrences) with no witness.
- `ConferringWord`'s `StoriedW` and `RenownW` — renown (22 supported cards) and
  the storied designation (11). The type's other three arms are macro-mediated;
  these two are not.

Corpus claims measured via `jq 'select(.supported)'` over
`data/derived/cards.jsonl` at audit time; re-verify at claim.

## Optional tooling follow-up

`map idris-dead` could grow an allowlist. The audit's triage names the two
exemption classes it would have to encode — structurally unnameable type-index
constructors and vocabulary spelled only in the declaring module's fact
tables — build it only if repeat runs prove worth the encoding.

## As landed

Both arms witnessed. No pins: nothing here is rules-impossible, and the audit
was right that each wanted a bench.

**The chapter markers — two whole saga cards, and `map idris-dead` is clean of
`Chapter*`.** Re-measured 2026-09-02 over `data/derived/cards.jsonl` with
`jq 'select(.supported)'`, counting chapter-head lines that name each numeral:
**IV 38 lines, V 4, VI 3** (the audit carried 35/2/3 — IV was short).

- `burnBurnTreeAndFern` — whole. Three lines, the last of them the paired
  III/IV, which is the cheapest carrier of the fourth chapter the corpus
  offers.
- `theFlux` — whole. The only supported card that reaches VI in three lines:
  I alone, the II–V run, and VI alone. One card buys the fifth and sixth
  markers together, and the II–V run buys IV a second time.

`cargo xtask map idris-dead` no longer names `ChapterIV`, `ChapterV` or
`ChapterVI`.

**The conferring words — expansions built, benched, and now at parity with
their three siblings.** `Macros` gains `renown` ([CR#702.112a], "put N +1/+1
counters on it and it becomes renowned") and `getsEnduringStory`
([CR#702.195a], "you have an enduring story for the rest of the game"),
written on `monstrosity`'s and `getsCitysBlessing`'s own model — the gate
stays at the site, because [CR#702.112a] puts renown's "if it isn't renowned"
on the TRIGGER and not in the body.

Re-measured 2026-09-02: **renown 22 supported cards** (the audit's count
confirmed); **storied 9**, not 11 — the two dropped matches are card NAMES
carrying the word, Grub, Storied Matriarch chief among them, and not the
keyword.

- `akroanSergeant` — whole. First strike plus renown's expansion written out
  as the triggered ability the keyword names, with the intervening "if it
  isn't renowned". The cheapest of the 22.
- `storiedEnduringStory` — a FRAGMENT, and the reason is worth recording:
  [CR#702.195a]'s gate counts "permanents that are artifacts, Sagas, and/or
  legendary", a disjunction mixing two head-bearing arms (`HasType`,
  `HasSubtype`) with an adjectival one (`HasSupertype`), and
  `parallelDisjuncts` refuses a mixed `Or` — `hasHead (HasSupertype _)` is
  False against the other two's True. The threshold reads fine; the union it
  counts over is the blocker. **A mixed head/adjective disjunction is a
  remainder to route**, and it is not this word's.

**Note on the audit's own output.** `ConferringWord`'s five arms are all still
listed by `map idris-dead`, because its witness files are `Cards.idr` and the
`Proofs*`, and every conferring word is named inside a `Macros` expansion
rather than at a bench site — which is exactly the "macro-mediated" exemption
class the audit's triage already named. `StoriedW` and `RenownW` now sit in
that class with `MonstrosityW`, `AscendW` and `SaddleW` instead of nowhere,
which is the gap this ticket held. The optional allowlist follow-up is
unchanged and still unbuilt.
