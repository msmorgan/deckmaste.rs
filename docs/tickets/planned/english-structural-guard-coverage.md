---
needs: []
design: true
---
**[design] There is no coverage meter for tree correctness — the analogue of
`cr-coverage.lock` for parse structure.** The instruments exist and work; what
is missing is a statement of *how much of the structural space they cover*.
`shapes` emits a worklist ("output is a worklist, never a finding"); `lint`
emits findings but subordinates recall to soundness by design, so its four
checks bound what can be *reported*, not what is *true*. Between them, the
quantity "what fraction of structural distinctions are under a sound guard, and
which are only round-tripping" is unmeasured — and round-trip cannot see
wrong-host attachment, which is the defect class that matters.

## Why this is not the instruments already built

Recorded so the overlap question does not have to be re-litigated:

- `recovery` measures whether a tree was *produced*. Silent on correctness.
- `roundtrip` measures losslessness. A misparse round-trips byte-identically —
  see the `shapes` module comment.
- `shapes` is rarity-based *discovery*. Rarity is not proof; graduating a
  family into a sound check costs adjudication, which is the real bottleneck
  this meter would quantify.
- `lint` is sound *adjudication*, deliberately low-recall.
- `probe` / [[english-adversarial-instrument-landing]] is the *active* branch:
  synthesized text checked against structural laws. Different corpus (unprinted
  compositions), different failure mode (composition admission unsoundness),
  and its `no-tie` law was measured into the ground. This ticket is passive and
  printed-only, so that failure mode cannot arise here — printed oracle text is
  grammatical by construction, so there is no admission rule to get wrong.

## The design hazard, named first

**The likely failure is a number that does not discriminate.** `no_tie` is the
worked example: a law firing on ~93% of ordinary printed faces taught nothing
and, per its own doc comment, "trains reviewers to ignore it." A guard-coverage
figure can fail the same way in either direction — trivially near-100% if the
denominator is coarse enough, or permanently near-0% if every unguarded
distinction counts against it. If a defensible denominator cannot be
constructed, **the correct outcome is to record why and close this**, in the
manner of [[english-structural-recovery-long-tail]]. Do not ship a meter that
cannot move.

Sketch of a denominator worth testing, not a decision: the `Shape` node
vocabulary is already the corpus-wide structural alphabet, and `shapes` already
frequency-ranks it. Coverage could be defined over *attachment sites* — host/
modifier pairs realized in the corpus — with each site classified guarded (a
`lint` check would fire on the wrong host), unguarded, or
unrepresentable-by-construction. That reuses two existing instruments and
makes the third-category count itself informative.

## Why now rather than later

Sequencing against [[english-semantic-ir]]: the IR is the first consumer that
will *act* on tree shape, and misparses surfacing through it appear as card
defects at maximum distance from their cause. The meter is worth more before
that consumption is corpus-wide than after. It does not block the IR and the IR
does not block it; only the value ordering couples them.

Standard constraints apply.
