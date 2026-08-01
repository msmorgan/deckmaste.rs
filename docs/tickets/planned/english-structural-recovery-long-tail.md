---
needs: []
---
**The structural-recovery long tail — why family-scoped rounds have a hard
ceiling of roughly 508 faces, and what has to replace them.** Campaign-internal
residue of `english-structural-recovery-zero`, split out so the measurement
survives in the tree.

Every round of this campaign so far has worked the same way: find a family of
unresolved rows sharing a construction, design one production for it, land it.
This ticket records the measurement showing that technique is close to exhausted,
and that the residue behind it is a different kind of problem.

## The measurement

Taken from a full `cargo xtask english unknown --limit 100000` dump at
structural total **3759 spans / 3366 faces**, and independently reproduced by a
second instance before this ticket was written. Both passes agree exactly.

- The 3759 structural spans span only **3365 distinct texts** — almost no
  repetition at all.
- **3166 texts occur exactly once**: 3166 spans on **2858 faces**.
- Texts occurring twice: 110 texts / 220 spans / 217 faces.
- Texts occurring three times: 46 texts / 138 spans / 137 faces.
- Texts occurring five times or more: only **26 texts** / 167 spans / 166 faces.

**So 2858 of the 3366 remaining faces — 84.9% — carry a span text that appears
nowhere else in the corpus.** Only about **508 faces (15.1%)** sit in any
repeated text at all.

Length profile of the singleton spans:

| words | singleton spans |
|---|---|
| 0–3 | 73 |
| 4–10 | 494 |
| **11–20** | **1661** |
| **21–40** | **921** |
| 41+ | 17 |

The tail is not short and awkward; it is long, syntactically dense, one-off
sentences. The longest singletons give the flavour: Captain Rex Nebula (60
words), Magar of the Magic Strings (52), Blue Mage's Cane (51), Tek (49), Tribal
Golem (48), Xanathar Guild Kingpin (48), Kaboom! (47), Bello Bard of the Bramble
(45).

## The ceiling argument

A family-scoped round needs a family: several faces sharing a construction, so
that one production clears many rows. The measurement above says that after the
current families are worked, there is **nothing left with that shape**. At most
~508 faces sit in any repeated text, and the repeated texts are themselves
mostly pairs and triples — 26 texts account for the only genuinely high-count
group. Everything else is a singleton.

**Family rounds therefore have a hard ceiling of roughly 508 faces, and the
remaining ~2858 faces cannot be reached by more of the same technique.** Not
because those faces are hard individually, but because there is no family to
target: each is one sentence, appearing once.

The residue is not a list of missing constructions. It is long sentences that
**compose many independently-working constructions** — each already parsed
correctly somewhere else in the corpus — in a combination that appears exactly
once. Reaching zero from there is not more families; it is generalization of
composition:

- coordination generalization (n-ary, asyndetic, and across heterogeneous
  conjunct types, rather than per-construction coordination rules);
- feature gates that let existing productions combine without a bespoke rule per
  combination;
- a keyword-rider field, so riders attach compositionally rather than being
  enumerated per host;
- and generally, machinery that makes the grammar's existing coverage compose,
  instead of adding coverage.

This matches the campaign's own mid-course tractability prediction, made
after nine rounds: that roughly ten rounds later the remaining work becomes
design-heavy machinery rather than family sweeps. **This measurement is the
first hard evidence for that prediction**, and it arrived roughly on the
predicted schedule.

## Scoping guidance for whatever replaces family rounds

**Per-face span length is the natural difficulty proxy.** The singleton
distribution is concentrated in 11–40 words (2582 of 3166 singleton spans), and
length correlates with how many constructions a sentence composes. Any future
scoping should bucket by length rather than by regex family, and should expect
that a round's value is measured in how many *compositions* it unlocks, not how
many rows share a phrase.

Two cautions carried forward from the campaign's settled rulings, both of which
bite harder here than they did on family rounds:

- **Accuracy first.** "As long as it's representing the grammatical structure
  more accurately, small progress deltas are perfectly fine." Composition
  machinery will be tempting to evaluate on row counts; it should be evaluated on
  trees.
- **A byte-identical render is not evidence of a correct tree.** Long composed
  sentences round-trip just as happily when mis-bracketed, and there are far more
  ways to mis-bracket a 30-word sentence than an 8-word one.

## Format scope of the residue (measured 2026-07-31)

Joining a post-`anof` `cargo xtask english unknown --limit 100000` dump
(structural total 3477 spans / 3117 faces / 3111 distinct card names) against
the MTGJSON snapshot's `legalities` and `SetList` set types (`5.3.0+20260707`):

- **Current Modern (`legalities.modern == "Legal"`): 1860 names / 2065 spans**
  — 60% of the residue, failing at 8.5% of the 21,993 Modern-legal supported
  names. 13 further failing names are Modern-banned. 405 of the 1860 are also
  currently Standard-legal.
- **Ever-Standard, by core/expansion-printing proxy: 2117 names**, plus 89
  whose core/expansion printings all predate Ice Age — the Type 2
  inception-ambiguity window, including Camouflage, Word of Command, and
  Chains of Mephistopheles — with zero overlap with Modern. Exact
  ever-legality is not reconstructible from local data: MTGJSON legalities
  are current-only.
- **No core/expansion printing at all: 905 names** (set types, overlapping:
  commander 513, draft_innovation 364, promo 202, masters 134, funny 12, …) —
  including the two longest failures in the dump (Captain Rex Nebula and
  Magar of the Magic Strings, both Unfinity).

The scoped residue keeps the full tail's shape — 96% clause-role spans, ~85%
singleton texts, concentrated at 11–40 words — so format scoping changes
which zero is declarable, not the machinery needed to get there. The fail
rate is nearly flat across scopes (8.5–10%): parse difficulty tracks text
density, not card age. A declarable Modern-scoped gate is ticketed as
`english-recovery-modern-gate`.

## Not in scope here

This ticket records the measurement and the argument. It does not design the
composition machinery, and it does not claim the ~508 family-tractable faces are
worthless — they remain the cheapest remaining work and should be taken first.
It exists so that when family rounds stop paying, the reason is already written
down and does not have to be rediscovered.
