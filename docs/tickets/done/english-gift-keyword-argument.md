---
needs: []
design: true
---
**[design] Shape the `Gift a [something]` keyword argument.** Diagnosed
2026-07-26 (queued by explicit user disposition during the english-structural-
recovery-zero campaign; diagnosis is complete — do NOT re-derive it).

`Gift` is already recognized as a keyword ability
(`data/gen/catalogs/keyword-abilities.txt`). The block is **argument shaping**,
at two layers:

- `KeywordArgument` (`crates/deckmaste_english/src/syntax/ability.rs`) has no
  nominal variant. Its arms are `Absent`, `Counted(Quantity)`,
  `Costed(KeywordCost)`, `CountedCost`, `Predicated(PredicatedArgument)`
  (preposition-introduced), `Statted`, `Named { separator, label }`,
  `Recovered`.
- **The decisive one:** `opens_like_keyword_argument`
  (`crates/deckmaste_english/src/grammar/ability.rs`) returns true only for a
  first token of `TokenKind::Integer | OracleSymbol | SymbolSequence |
  PowerToughness`, or the words `from`/`for` via `predicated_preposition`.
  `Gift a card` opens with the determiner `a`, so the gate returns false and
  the tokens are never shaped as a keyword argument **at all** — not even as
  `Recovered`, the designed graceful degradation. That is why these faces land
  in the clause-recovery cell rather than the keyword-argument cell.

## Corpus (supported faces; verified 2026-07-26)

24 occurrences, all currently whole-line CLAUSE recoveries:
`Gift a card` ×12, `Gift a tapped Fish` ×6, `Gift a Food` ×3,
`Gift a Treasure` ×1, `Gift an Octopus` ×1, `Gift an extra turn` ×1.

These are **exactly** the six forms the CR enumerates, and the enumeration is
closed: [CR#702.174a] writes the keyword as "Gift a [something]", and
[CR#702.174d..702.174i] define the complete legal parameter set — Food, card,
tapped Fish, extra turn, Treasure, Octopus. Every one of the six is attested
in the corpus and every one is unparsed.

## Modeling direction

Each subrule reads "'Gift a X' means the effect is …", i.e. the `[something]`
is a **selector label choosing a defined effect**, not a compositional noun
phrase whose parts carry independent meaning. `KeywordArgument::Named` may
therefore already be the right shape: `KeywordArgumentSeparator::Space` exists
and `keyword_label_is_bare` already passes for these labels (no `.!?:`).

## The design tension the round must resolve (stated so it is not rediscovered)

Widening the opener gate to admit determiner-initial tokens widens the argument
slot for **every** keyword ability, not just Gift — a licensing question with
real over-fire risk. A `Gift`-specific special case is forbidden by the
campaign's standing shapes-not-identities rule (no keyword→shape mapping;
catalog-backed matching is the sanctioned licensing).

**Stage 0 must therefore measure, before any code is written,** whether a
determiner-initial label opener over-fires anywhere else in the supported
corpus. Only then choose between widening the gate, licensing the label form
from catalog metadata, or another mechanism.

## Constraints

Standard constraints apply. Cite [CR#702.174a] and the relevant
[CR#702.174d..702.174i] subrules; rule numbers come from the CR text, never
memory. Round-trip must stay clean and the round must not add opaque leaves,
split recovery spans, or weaken lexical-slot constraints.
