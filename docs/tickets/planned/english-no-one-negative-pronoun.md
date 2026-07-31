---
needs: []
---
**`no one` is a fixed negative pronoun and we parse it as `no` + the numeral
`one`.** Found while measuring `Numeral` in round `sfresidue` (2026-07-30).

Plague of Vermin, `Repeat this process until no one pays life.` — the
subordinate clause misparses, reading `no one` as determiner `No` plus a
`NumberLiteral` of value 1, rather than the single negative pronoun it is. The
face still round-trips clean (the pieces re-render to the same bytes), which is
why no gate has ever flagged it.

It surfaced as a **`Numeral` counter-example**: the stored numeral form on that
face disagreed with the otherwise clean three-tier rule (construction, then
magnitude ≥ 100, then head class) documented on `Numeral`. Every other apparent
counter-example traced to a misattributed nested `NumberLiteral`; this one is a
genuine defect, and it is the reason the `Numeral` rule could not be shown exact
in that round.

## Shape

`no one` (and check `anyone`, `everyone`, `someone`, and `no other` while you
are there) should lex or reduce as one pronoun. Compare how the grammar already
handles other fixed multi-word pronominals. The risk to watch is the reverse
error: `no` followed by a genuine numeral (`no 1/1 creature` shapes) must keep
its determiner reading.

## Verify

Plague of Vermin's `until no one pays life` parses with a negative-pronoun
subject. Round-trip stays 31685 clean and the recovery census stays
byte-identical — this face parses cleanly today, so it is a tree fix, not a
recovery fix. Then re-measure the `Numeral` derivation and record whether
removing this counter-example makes the three-tier rule exact; if it does, the
field can be deleted and the doc on `Numeral` says so. Standard constraints
apply.
