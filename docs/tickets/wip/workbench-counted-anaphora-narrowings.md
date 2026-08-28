---
needs: []
---
# Counted anaphora narrowings: admit the bare-"it" residue without preference

**Ruling (user, 2026-08-27): clause recency is permanently OFF the table.** A
corpus audit (5,153 candidate sites over supported cards) plus two independent
cleanroom consultations converged: every positional/preference rule
(nearest-wins, subject-continuity, mention-recency, clause-age) silently
misbinds real supported lines — a term that typechecks while meaning the wrong
thing, which no gate can catch. The correct program is the house doctrine
generalized: **counted uniqueness over a construction-proven smaller candidate
set; count survivors, never rank; ties refuse.** `ItAt`/`ItVerbed` were the
first two rows of this family; this ticket is the rest.

The measured residue this addresses is the ~886 same-carrier bare-"it"
refusals (docs/tickets/done/workbench-anaphora-a-bare-it.md); after these
rows, the estimated genuinely-refused remainder is double-digit occurrences
(unverified until each family is built and re-measured). A refused bare "it"
never blocks a card — the explicit spelling (`That w`/`TheVerbed`/`This`)
always remains.

## The rows

Each row is a `countBy` fold with the ADR's per-anaphor proof cost
(`countXIsFold`, prefix-resolution lemmas); nothing reorders `bs`; no new
index on `Effect`.

1. **De-pronominalization templates (not reads at all).** Where the printed
   "it" is constructor spelling, no pronoun read exists: copy exceptions
   ([CR#707.9]; `CopyExcept` already does this), "counters on it"
   (`CounterCompare`), "to its owner's [zone]" (`Bare` scope), plus the
   missing templates: a shared-subject coordination form (retiring the
   `itAsPermanent` workaround; "gets +4/+4 and gains trample" is VP
   coordination, not anaphora), a `controllerSacrifices` template ("X's
   controller sacrifices it", 12/12 corpus-consistent), and the subject-bound
   "deals damage equal to its power" (204/204 subject-bound, incl. Legolas,
   Master Archer).
2. **`ItRole` — role-indexed uniqueness.** `countRole r bs = 1` where the
   enclosing construction proves a role identity. First row: the
   damage-replacement family — "If X would deal damage…, IT deals … instead"
   is the REPLACED EVENT'S SOURCE by [CR#614.6] ("A modified event occurs
   instead"; verified verbatim). Census: 60 supported faces, 44 with an
   object-compatible nearer mention, 0 role reversals. The construction marks
   the source binding; stale marks must be cleared before typing the
   replacement (mark leakage across branches/quotes is THE implementation
   risk — refuse on multiple marks, never select).
   Covers: Blind Fury, City on Fire, Inquisitor's Flail.
3. **Co-argument exclusion (Principle B).** A bare "it" in a verb's object
   slot excludes that verb's own subject/co-argument: count uniqueness over
   the prefix minus the co-argument's delta (and minus `SelfD` when the
   co-argument is `This`). Probed: 0 supported lines where a non-reflexive
   object corefers with its co-argument. Applies PER CONSTRUCTOR, not per
   sentence (Bruna's nested description must not be excluded).
   Covers: attach-to-it (63 cards), forced-block ("target creature blocks
   it" — Fighter Class, Feral Contest, Tower Above).
4. **Previous-sibling delta.** In `Sequentially`/coordination, count
   uniqueness over the immediately preceding sibling's OWN delta first; fall
   back to the bare gate when it holds no singular object.
   Covers: "tap target creature and put a stun counter on it" (Stunning
   Shot), "create a token. Put a counter on it".
5. **Producer-label extensions.** `verbFacts` rows for Return, Untap,
   GainControl; a `TokenOrigin` narrowing (`ItToken` over the origin already
   on the payload). Corpus: "create … token. It …" = token 70/70; Return/Put/
   Untap continuations all producer-bound.
6. **Everything else stays refused** and is authored explicitly: the
   genuinely ambiguous pairs (Acolyte of the Inferno vs Basalt Golem, Gaze of
   Pain, the Tamiyo shapes) are resolved in English only by play knowledge —
   refusing the pronoun spelling is correct behavior, recorded per family.

## Pins

- NO preference or ranking anywhere; every admission is a counted gate over a
  provably smaller set; ties refuse. This is the ruling, not a default.
- Exclude a candidate only from a rules- or construction-entailed fact.
- Only the macro owning a construction may emit a role-scoped pronoun.
- ProofsAnaphora's existing lemmas stay; each new gate pays its own fold and
  prefix lemmas ([the ADR's per-anaphor cost]).
- The enters-as-copy constructor obligation (copy-family ticket) stands:
  copy sources are never announced.
- Corpus tallies above from the 2026-08-27 audit + consultations: the CR
  quotes and the damage-family scale are verified; per-family counts are to
  be RE-MEASURED at claim before sizing any row (evidence lives in
  session-local scratch, deliberately not carried — the counts in this
  ticket are the claim to re-verify, not the authority).

## Acceptance

- Each row lands with its measured family re-counted, at least one whole-card
  bench witness, and the refusing shapes still refusing (`badTwoChoosers…`
  siblings and the Principle-B zero probed).
- Blind Fury, Stunning Shot, an attach carrier and Fighter Class bench.
- The residue is re-measured after all rows and recorded with named families.
- `idris/scripts/build` PASS; ProofsAnaphora prefix lemmas intact.

Standard constraints apply.
