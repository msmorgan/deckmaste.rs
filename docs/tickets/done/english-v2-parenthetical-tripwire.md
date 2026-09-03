Turn the rules-bearing parenthetical allowlist into a tripwire
(parenthetical-guard review M2). The guard preserves five whole-string
matches; an unknown mid-line parenthetical is still stripped silently, so
a new spelling (or a variant of a guarded one) silently deletes rules
text. Inverse the guard: normalization classifies every mid-line
parenthetical group against a counted, review-surfaced list — the 5
rules-bearing (preserve) and the 9 reminder-shaped variants (strip; 135
uses today) — and a group matching neither is a normalization ERROR that
names the card, never a silent strip. Identity-stability probe: no
currently-selected unit's normalized text changes (reviewer-verified
baseline: exactly 17 units differ from the raw strip). Record the two
classification lists in the landing record. Standard constraints apply.

## Landing record

Normalization now returns a card-naming error for every true mid-line
parenthetical not present in one of the following exhaustive lists. The
production-data inventory test runs the classifier over every Vintage-playable
unit, pins each spelling's use count, and compares every normalized identity
with the previous guarded normalizer.

Rules-bearing spellings preserved byte-exactly:

| spelling | uses |
| --- | ---: |
| `(as long as this creature is on the battlefield)` | 2 |
| `(even if this card isn't on the battlefield)` | 1 |
| `(front face up)` | 11 |
| `(if it's still on the battlefield)` | 2 |
| `(or {1})` | 1 |
| **total** | **17** |

Reminder-shaped spellings stripped:

| spelling | uses |
| --- | ---: |
| `(For example, you may change "black creatures can't attack" to "blue creatures can't attack.")` | 1 |
| `(a ticket counter)` | 2 |
| `(an energy counter)` | 23 |
| `(energy counter)` | 1 |
| `(energy counters)` | 16 |
| `(four energy counters)` | 7 |
| `(the Fridge)` | 1 |
| `(three energy counters)` | 21 |
| `(two energy counters)` | 64 |
| **total** | **136** |

| gate | before | after |
| --- | ---: | ---: |
| corpus units | 32,641 | 32,641 |
| selected and covered identities | 15,932 | 15,932 |
| construction declarations | 398 | 398 |
| coverage lock | current, 15,932 identities | current, unchanged |

The coverage check reports zero selected-uncovered units, ties, internal
failures, exceptions, round-trip mismatches, ownership failures, lexicon
collisions, gaps, overlaps, synthetic claims, or provenance-plan mismatches.
The identity-stability probe finds zero changes from the previous guarded
normalizer and exactly 17 units differing from an unguarded raw strip.

Positive gates: `cargo fmt --all -- --check`; all 412 xtask tests plus the
determinism integration test; strict all-target xtask Clippy; and
`cargo xtask english_v2 coverage --check`.

### Deviations and additions

- The ticket's 135-use reminder total is stale by one. The checked-in snapshot
  has 136 uses across the stated nine spellings; per-spelling counts above
  account for every use.
- Six synthetic inspect fixtures used the deliberately unclassified spelling
  `(reminder text)`. They now use the classified `(the Fridge)` spelling while
  retaining the same normalized fixture text and test purpose.
- No constructions or tests beyond the ticket's requested tripwire,
  inventory, error, and identity-stability coverage were added or deleted.
- No STOP was taken.
