---
needs: [semantics-v2-definition-bodies]
---
**The +1/+1 and -1/-1 annihilation rule has no home.** STOP recorded by
`semantics-v2-definition-bodies` while converting `counter_kinds/m1M1Counter.ron`
to a `Definition` body. [CR#704.5q]: "If a permanent has both a +1/+1 counter and
a -1/-1 counter on it, N +1/+1 and N -1/-1 counters are removed from it, where N
is the smaller of the number of +1/+1 and -1/-1 counters on it."

Two things block it, and the second is the interesting one.

1. **`Amount` cannot say "the smaller of".** `ArithOp` is
   `plus | minus | times | differenceBetween`; there is no minimum over two
   amounts, and `AggregateOp` (`sum | min | max`) ranges over a group or a
   domain, not over a pair of amounts. The v1 record wrote
   `Min(CounterCount(This, p1P1Counter), CounterCount(This, m1M1Counter))`.
2. **It is not a conferral.** A `Conferral` scopes the definition itself — the
   counter's bearer — but [CR#704.5q] fires on a permanent that has BOTH kinds,
   which is a predicate over the permanent. Its home is a `SbaRule` in
   `plugins_v2/builtin/rules/sba/`, beside the three the workbench already
   writes, not the -1/-1 counter's `confers` list where v1 put it.

So: decide whether `Amount` grows a binary minimum (and what its Lean name and
CR citation are — [CR#704.5q] is the only rule that needs one today), then write
the row as an `SbaRule` with `scope := .and [+1/+1 counters, -1/-1 counters]`
and a pin in `Proofs/Rules.lean` beside `toughnessZero`. The v1 record is
preserved verbatim in `m1M1Counter.ron`'s STOP comment until then.

`Proofs/Rules.lean` already omits the two lethal-damage rows [CR#704.5g,704.5h]
for the same class of reason; a resume should consider whether one amount
addition unblocks more than one row. Standard constraints apply.
