---
needs: []
---
**Six flat helpers over the one `Exchange` instruction.** Split from
`plugins-v2-keyword-body-defects` on 2026-10-05. Standard constraints apply.

## Why

The keyword action `exchange` has no body; it exists so the parser can read
the verb. Being named `exchange`, it shadows any alias for the `Exchange`
instruction, so `keyword_abilities/auraSwap.ron` still writes the
`Exchange(…)` constructor and sits on the raw allowlist (`ALLOWED_RAW` in
`crates/deckmaste_semantics_v2/tests/keyword_bodies.rs`).

## What is decided

- Keep the ONE existing instruction, `Exchange(Exchanged)`, whose arms are
  `LifeTotals`, `ControlOf`, `CardsAcross`, `Zones`, `Values`, `TextBoxes`
  [CR#701.12a..701.12h]. The all-or-nothing rule ("if the entire exchange
  can't be completed, no part of the exchange occurs" [CR#701.12a]) stays on
  that one node.
- Add six flat helpers, one per arm: `exchangeControl(a, b)`,
  `exchangeLifeTotals(players)`, `exchangeCards(a, b)`, `exchangeZones(a, b)`,
  `exchangeValues(a, b)`, `exchangeTextBoxes(a, b)`.
- The bodyless `exchange` keyword action stays, for parsing.

## The work

1. Add the six helpers under `plugins_v2/builtin/macros/instructions/`.
2. Re-spell `auraSwap` (it writes `Exchange(CardsAcross(…))`) over
   `exchangeCards` and strike `keyword_abilities/auraSwap` from
   `ALLOWED_RAW`. No other file in `plugins_v2/` writes `Exchange(` today
   (canon's Arcanum Wings calls `auraSwap`).

## Proof

`cargo xtask expansions` before/after is byte-identical (a re-spelling), and
`keyword_bodies` passes with the shorter allowlist.

## Related

`lean-core-exchange-operands` (maybe/) asks whether life totals fold into
numeric operands; `research-exchange-textbox` surveys which arms cards use.
Neither blocks this.

## Landing record

The series, oldest first, on the claim `rksypqluxwro`:

- S1 `pzvvmnzmyolu`: six exchange helpers; `auraSwap` writes
  `exchangeCards`; `keyword_abilities/auraSwap` struck from `ALLOWED_RAW`.
- S2 `mqlmsvtnwowp`: cards proving the helpers (four canon, one testing).
- S3 `mzyumzrtmpmk`: `docs/decisions/semantics-v2.md` §11 and §12.1 name
  the helpers.

Each stage was gated before its commit; no stage was folded. This record is
written on top of S3 and changes only this ticket. No file under `lean/`
changed in the series.

**The helpers.** Each is a plain `instructions/` meta with a NAMED
signature, positional at its callers, no performer parameter, and a body
that is `Exchange(<arm>)`; the parameter names are the `Exchanged` arm's
own binders:

| helper | signature | body | rule |
|---|---|---|---|
| `exchangeControl` | `left: NounPhrase, right: NounPhrase` | `Exchange(ControlOf(left, right))` | [CR#701.12b] |
| `exchangeLifeTotals` | `parties: NounPhrase` | `Exchange(LifeTotals(parties))` | [CR#701.12c] |
| `exchangeCards` | `left: NounPhrase, right: NounPhrase` | `Exchange(CardsAcross(left, right))` | [CR#701.12d,701.12e] |
| `exchangeZones` | `left: ZoneExpr, right: ZoneExpr` | `Exchange(Zones(left, right))` | [CR#701.12f] |
| `exchangeValues` | `left: Amount, right: Amount` | `Exchange(Values(left, right))` | [CR#701.12g] |
| `exchangeTextBoxes` | `left: NounPhrase, right: NounPhrase` | `Exchange(TextBoxes(left, right))` | [CR#701.12h] |

Every file comment also cites the all-or-nothing rule [CR#701.12a], which
stays on the one `Exchange` node. The bodyless `exchange` keyword action is
untouched.

**No keyword-action wrapper.** The Lean bench never wraps an exchange in
`enact (.action "Exchange") … agent`: all seven bench sites (Mirror Universe,
Soul Conduit, Harness Infinity, Vedalken Squirrel-Whacker, Avarice Totem,
Arcanum Wings, Deadpool, Trading Card) write
`Primitives.Instruction.exchange (Primitives.Exchanged.<arm> …)` directly,
`Macros.lean` has no exchange macro, and the deed table's `("Exchange", {
agentRole := playerAgent })` row is reached only by an `enact` nobody
writes. The helpers therefore produce the bare `Exchange(…)` term, which is
also what keeps `auraSwap` byte-identical.

**Proof.**

- Per-stage gate (S1, S2, S3):

  | check | S1 | S2 | S3 |
  |---|---|---|---|
  | `cargo xtask lean-check`, `plugins_v2/canon` | 122/122 | 126/126 | (no card change) |
  | `cargo xtask lean-check`, `plugins_v2/testing` | 4/4 | 5/5 | (no card change) |
  | `cargo xtask facts check` | up to date | up to date | not run (docs only) |
  | derived gate command, `--no-fail-fast` | 87 binaries, 1166 passed, 0 failed, 1 ignored | same | same |
  | `cite check --list-noncompliant` / `cite check` | 0 / 0 stale | 0 / 0 stale | 0 / 0 stale |

  The derived command (`cargo xtask gate --changed`, identical at every
  stage): `cargo test -p deckmaste_construction_core -p
  deckmaste_lexical_source -p deckmaste_semantics_v2 -p
  deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask`. The one
  ignored test was ignored before this landing.
- `cargo xtask expansions` for `plugins_v2/builtin`, baseline at the claim
  against S1 and against S3 (`diff -r`, outputs kept outside the repo):
  1501 → 1507 declarations, 1453 → 1459 printed, 48 skipped both times, 0
  failed. The only differences are the six new `instructions/exchange*.expanded`
  files and their six index lines plus the count line;
  `keyword_abilities/auraSwap.expanded` is byte-identical (`cmp`), so
  Arcanum Wings' term is unchanged. S1 → S3: identical.
- Citations: every newly written `[CR#701.12a..701.12h]` and `[CR#702.3b]`
  site was read against its rule through `cite audit --diff`; all were
  already in `cr-citations.lock`, so nothing was blessed.
- Helper → the card that proves it under `lean-check`:

  | helper | card | kind |
  |---|---|---|
  | `exchangeCards` | Arcanum Wings (through `auraSwap`) | canon, existing |
  | `exchangeControl` | Avarice Totem | canon, added |
  | `exchangeLifeTotals` | Soul Conduit | canon, added |
  | `exchangeZones` | Harness Infinity | canon, added |
  | `exchangeValues` | Tree of Redemption | canon, added |
  | `exchangeTextBoxes` | Text Box Exchange Probe | testing, added |

**Tests.** Restored: 0. Re-spelled: 1, `keyword_bodies.rs`'s
`ALLOWED_RAW` lost `keyword_abilities/auraSwap` (8 → 7 entries; the
ratchet test `keyword_bodies_are_written_in_card_vocabulary` passes on the
shorter list). Added: no Rust test; four canon cards and one testing card,
each a `lean-check` proof obligation. Ignored: 0. Removed: 0. No test
enumerates or counts the helper declarations, so no count was re-spelled.

**Deviations and additions.**
1. Parameter names are the `Exchanged` binders: `parties` for
   `exchangeLifeTotals` (the ticket wrote `players`), `left`/`right`
   elsewhere.
2. Four canon cards added rather than probes, each a short Vintage-legal
   card quoted from `data/derived/cards.jsonl`: Avarice Totem, Soul Conduit,
   Harness Infinity, Tree of Redemption. Soul Conduit writes its determiner
   as `described(determiner: Target(exactly(2)), predicate: anyPlayer)`
   (no `target` helper takes a count; Dead Revels writes the same form).
3. Exchange of Words, the one printed text-box exchange [CR#701.12h], needs
   "for as long as this enchantment remains on the battlefield" over two
   targets chosen on entering, so a probe proves `exchangeTextBoxes`:
   "{T}: Exchange the text boxes of this creature and another target
   creature."
4. ADR: §11's named-apart list "Seven …" → "Thirteen …", adding the six
   helpers as having no Lean macro; §12.1's alias paragraph gains a sentence
   saying `Instruction.Exchange` has no alias (the keyword action owns the
   name) and is written through the six helpers.

**STOPs.** None. Every faithful term proved.

**Counting method for §12.1.** `grep -c '^semantic_macro'
lean/Semantics/Macros.lean` is 424, and no `semantic_macro` name contains
"exchange"; the six helpers match no Lean macro, so 424 = 314 + 2 + 108 and
the 244 aliases are unchanged.

**Glossary.** No new term.

**Not applicable.** Coverage lock, selection census, licensing-checker
totals, homograph and form-literal inventories and the performance advisory:
no English grammar, lexicon or corpus input changed, so `coverage` was not
run. Lean pins and the Lean bench: no Lean file changed.

**Routed.** Nothing new. `lean-core-exchange-operands` and
`research-exchange-textbox` are unaffected.
