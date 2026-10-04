---
needs: []
---
# License frequency and prepositional adjuncts before landing lexical expansion

Repair the grammatical admission exposed by the eight-entry lexical draft in
workspace `english-v3-triage`. The draft is deliberately unintegrated. Standard
constraints and the retained-Reading contract apply; the English Lean project
is retired.

Use declared grammatical distribution and selected frames to distinguish
frequency NPs from Objects and to license PPs at nominal, predicate and clause
hosts. Preserve all independently grammatical alternatives. Do not use named
words, verbs or cards in admission guards, semantic target filtering, or
destructive selection. Adding a missing vocabulary entry must not turn an
invalid analysis into a claimed coverage gain.

Authentic acceptance witnesses:

- Jadelight Spelunker: `When this creature enters, it explores X times.`
  Require the frequency-adjunct structure for `X times`; the draft's sole
  Reading incorrectly makes it the Object of `explores`.
- Blessings of Nature: `Distribute four +1/+1 counters among any number of
  target creatures.` followed by `Miracle {G}`.
- Stolen Goodies: `Distribute three +1/+1 counters among any number of target
  creatures you control.`
- Verdurous Gearhulk: `Trample` followed by `When this creature enters,
  distribute four +1/+1 counters among any number of target creatures you
  control.`
- Grove's Bounty: `Distribute X +1/+1 counters among any number of target
  creatures you control.`

For the four distribution faces, retain the complete nominal Complement of
`among`, including the nominal attachment of `of target creatures ...`.
Reject the independently attached `of` PP at the distribute VP or enclosing
Clause. Audit the remaining nominal attachment alternatives independently;
different game meanings alone are not evidence of invalid English.

Assert complete allowed structural Reading sets with authentic positive and
negative witnesses and independently constructed values. Keep both roundtrip
laws, lexical identity and traversal. Extend existing tests rather than
discarding them. Reconcile the draft's eight lexical owners before landing it.
Elven Rite and Throw a Line still fail at `among one or two target creatures`;
classify that residual separately rather than assuming this repair fixes it.

## Audit evidence

Measured working change `vqksnuwx` on the local supported snapshot, with eight
workers: baseline 29,063 No / 2,629 One / 1,136 Multiple; draft 29,058 No /
2,630 One / 1,140 Multiple. No previously covered identity was lost, and no
previously covered face changed its Reading count. Mechanical issues, internal
failures, cycles and incomplete enumerations were zero. These mechanical
successes do not establish linguistic correctness.

All 453 Readings of the five newly admitted faces were retained for the
structural audit. Besides Jadelight Spelunker's wrong Object, 170 distribution
Readings detach the `of` PP: Blessings of Nature 8 Clause / 6 VP; Stolen
Goodies 24 / 18; Verdurous Gearhulk 54 / 18; Grove's Bounty 24 / 18. None of
these five faces is credited as a valid coverage gain pending repair.

The vocabulary audit found no demonstrated missing inflection of an existing
verb in the inspected gaps. Absent lemmas include trigger, remain, resolve,
distribute, effect, emblem, devotion and the ordinary noun time. All four new
verbs use regular default morphology; `triggered` and `triggering` need no
doubling override. Devotion is mass-only with no plural slot in this sense.

The 92 source diagnostics separate into 42 counter-kind declarations lacking
lexical grammar, 34 superseded/compositionally replaced vocabulary declarations
and 16 retired nonattestation restrictions. The counter-kind work remains a
source/category and nominal-modifier obligation for `english-v3-systemic-residuals`;
do not label every counter word an adjective to fit current premodification.
Past-participial premodification in `triggered ability` remains a separate
grammar obligation for that same audit owner.

Reports are scratch evidence, not gate inputs. Reproduce with
`cargo xtask lexical --all --workers 8 --output /tmp/lexical-audit.json --export
/tmp/lexical-audit.ron` and `cargo xtask english-v3 --all --workers 8
--samples-per-face 1 --output /tmp/english-v3-audit.json`. Enumerate and store
every tree for the five named gains using the same command with repeated
`--card-name` selectors and `--samples-per-face 1000000`.
