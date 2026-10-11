---
needs: []
---
# Read a partitive headed by *any*, *all*, *either* or *another*: If any of that mana is spent on a creature spell, … / remove all of them

## Why

The Determinative-headed partitive reads for *each* only (NP "each of those
cards" 1, "one of those cards" 1 through the Cardinal route). NP "any of
those cards" 0, "any of that mana" 0, "all of them" 0, "all of those cards"
0, "either of those creatures" 0, "another of those counters" 0. Blocker
report at trunk `ymqwzour` (`target/english-v3/blockers-wave8.txt`): key
`at:any of` 15, part of failing token `of` 77.
`cargo xtask english-v3 grep -i '\b(any|either|another|all) of\b'` finds
**35** faces, all unread; **34** have that line as their only failing line:
*any of* 19 (19 sole), *all of* 8 (8), *either of* 6 (5), *another of* 2 (2).
`english-v3-systemic-residuals` lists "Explicit partitives *any of / all of /
none of* (0 covered)" as owned there; this ticket takes *any* and *all* over.

## Goal

A Determinative that declares the partitive use heads an NP with an *of*
Complement: *any of those cards*, *any of that mana*, *any of the exiled
card's colors*, *all of them*, *either of those creatures*, *another of those
counters*, with the number each head declares and byte-exact realisation.

## Analysis

CGEL Ch. 5 §9.1, p. 411: "The partitive may be explicit, with an *of* phrase
in post-head modifier position"; "Here the head is followed by a complement
consisting of *of* + a partitive oblique, an NP which can be plural,
non-count, or, under restricted conditions, count singular", with [6] "*some
of the books* … *all of them*" and "*some of the meat* … *all of it*". Those
back a Determinative fused head with a plural or non-count oblique. Which
Determinatives Oracle text licenses, and the number each exports, are project
rulings. Glossary: **Partitive Noun Phrase**, **Fused Determiner-Head**.

Existing route (`crates/deckmaste_english_v3/src/declarations.rs`):
`DeterminerHeadPartitiveNounPhrase` (`[head: lexical(Determinative), " ",
complement: PrepositionPhrase, modifiers: repeat(PartitiveModifier, "")]`,
`require head.NominalComplementMarker = Of`, `require
complement.ObliqueNumber = Plural`, `require head.DeterminerUse =
SingularCount`, `export number = Singular`). Only
`vocab:FloatedQuantifier/Each` declares `NominalComplementMarker: Of`
(`lexicon/core.ron`). `Determinative/Either` and `Determinative/Another` are
`SingularCount` and lack only the marker; `Determinative/Any` is
`Unrestricted` and `FloatedQuantifier/All` is `PluralOrMass`.

Orchestrator rulings 2026-10-10:

- PD1. Refine `DeterminerHeadPartitiveNounPhrase`; no new Construction.
  *either* and *another* are data: declare `NominalComplementMarker: Of`.
- PD2. For *any* and *all* the two `require`s become tables over declared
  features: the head's `DeterminerUse` against the oblique's `ObliqueNumber`
  (as `partitive_oblique` does for the Cardinal route), and the exported
  number. *all of* + plural is Plural; *any of* + plural admits both numbers
  (attested: *if any of those cards remain exiled* and *if any of those cards
  shares a card type*: two Readings of the NP are not expected, the verb
  selects); *any of that mana* / *any of that damage* is Singular.
- PD3. The licence is the Determinative's declared marker and
  `DeterminerUse`; no Determinative is named in the grammar.
- PD4. One analysis per string. *each of those creatures* keeps exactly its
  Readings (133 covered lines contain *each of*). The twin to avoid: *all* as
  a floated quantifier or predeterminer followed by an *of* PP attached
  elsewhere (*put all of them …* must not also read as *put all* + PP).
- PD5. **Superseded by the owner principle below.** Declare only attested heads. *none of* (3 faces; *none* is an unknown
  word), *both of* (1), *some of* / *neither of* / *most of* (0) are not
  declared here; do not add a check whose only job is to block them.
- PD6. Newly covered faces are gains. No Reading-count increase on a covered
  face is expected (0 of the 35 faces are covered; *any of* / *all of* occur
  on no covered face). Report any increase with its shape before landing.

## Witnesses

Each is the only failing line of its face in `blockers-wave8.json` (census
`ymqwzour`, 0 Readings). Fragment probes ran 2026-10-10 on the `ymqwzour`
binary (`--readings 60`, enumeration complete). Trunk then moved to
`pxmykwnl` and the prebuilt probe no longer loads plugin data, so controls
marked (census) give the covered line's `reading_count` in that file.
Re-probe every witness and control on the claim parent first.

- Chrome Mox: "{T}: Add one mana of any of the exiled card's colors."
- Arena of Glory: "{R}, {T}, Exert this land: Add {R}{R}. If any of that mana
  is spent on a creature spell, it gains haste until end of turn."
- Mindblade Render: "Whenever your opponents are dealt combat damage, if any
  of that damage was dealt by a Warrior, you draw a card and you lose 1 life."
- Ludevic's Test Subject: "{1}{U}: Put a hatchling counter on this creature.
  Then if there are five or more hatchling counters on it, remove all of them
  and transform it."
- Tamiyo, Field Researcher: "+1: Choose up to two target creatures. Until
  your next turn, whenever either of those creatures deals combat damage, you
  draw a card."

Probed 0: "If any of those cards remain exiled, draw a card."; "Remove all of
them."; "Destroy any of them that are Walls." Controls that must not change:
NP "each of those cards" 1; NP "one of those cards" 1; "Destroy one of them."
1; "If one of those cards remains exiled, draw a card." 1; Stall for Time
"Tap up to two target creatures. If this spell was kicked, put a stun counter
on each of those creatures." 1 (census); Strategic Planning "Look at the top
three cards of your library. Put one of them into your hand and the rest into
your graveyard." 3 (census).

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching

    \b(any|all|either|another|each|both|none|neither|some|many|most|much|more|fewer|less) of\b

(case-insensitive; 421 faces on the claim parent, including the full existing
class and *each of* controls) plus 300 random covered
controls, `--workers 6 --samples-per-face 0`, compared with `cargo xtask
english-v3 diff` against the claim parent. Zero lost; PD6 for increases. Full
`cargo test -p` and `cargo clippy -p … --tests -- -D warnings` for
`deckmaste_lexical_source` and `deckmaste_english_v3`, once, at the end.
PD1 is lexicon data and loads at runtime; PD2 is a grammar edit (rebuild
about 2–3 minutes, the six test binaries about 75 seconds). Tests assert the
role (the Determinative as fused head, the *of* PP as its partitive
Complement, the exported number; the PD4 negative), never an incidental count
or a Debug-printed tree. Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- Quantity-headed partitives (*one or more of*, *up to two of*, *that many
  of*, *1 of that damage*): `english-v3-partitive-quantity-phrases`.
- *all but one of them*: `english-v3-all-but-exception`.
- Unifying the two partitive Constructions' oblique checks (routed by
  `english-v3-systemic-residuals` to `english-v3-licence-plumbing-economy`).
- Faces whose line has additional blockers beyond this Construction. Three
  Wishes now gains through its existing oblique Relative Clause; no relative
  machinery is added here.

## Settled scope and residual

The owner's 2026-10-10 class ruling overrides PD5 and the vocabulary exclusions
above: all existing quantificational Determinatives with partitive use are
licensed, including hypothetical lines; the attested unknown *none* is added.
No unattested Lexeme is minted. W9-A preserves independent sentence ambiguity.

PD7 (orchestrator, 2026-10-10) excludes a relative on a partitive's fused head
from this landing unless sibling routes already support the same shape.
On `qnvlmzuu`, Document probes (enumeration complete, no issues) give:

| Shape | Determinative route | Cardinal route |
| --- | --- | --- |
| Destroy any of them that are Walls. | 0 | — |
| Destroy each of them that are Walls. | 0 | — |
| Destroy two of them that are Walls. | — | 0 |
| Destroy any of those creatures that are Walls. | 1 | — |
| Destroy each of those creatures that are Walls. | 1 | — |
| Destroy two of those creatures that are Walls. | — | 1 |

Neither route supports a relative on the fused head: this is a shared
follow-up, not an asymmetry. All noun-oblique controls select the relative
inside `SubjectRelativeNominal` under the *of* Complement, not on the fused
head. Corpus attestation via `english-v3 grep`, restricted to unread faces in
the selector, identifies **Blow Your House Down** as the sole pronoun-oblique
relative residual (*any of them that are Walls*). The broader relative search
also found noun-oblique relatives and unrelated later *that* constituents;
those do not demonstrate this blocker. Evidence is under
`target/english-v3/english-v3-partitive-determinative-heads/pd7-*.json`.

## Landing record

1. **Census delta** — Bounded claim-parent census (code at `pkllzssk`, measured
   on clean `qnvlmzuu`) versus `qnvlmzuu`: 720 faces; covered **588 → 614**;
   Readings **26,781 → 29,019**. Built-in `english-v3 diff --lines`: **26 gains,
   0 lost, 0 decreases, 0 increases, 0 digest-only**, 694 unchanged; no issues.
   No previously covered Reading was removed. Selector: case-insensitive
   `\b(any|all|either|another|each|both|none|neither|some|many|most|much|more|fewer|less) of\b`
   anywhere in supported analysed text, 421 surface faces, plus 300 covered
   controls from the built-in manifest generator (seed 20261008), one overlap.
   `manifest.json` is the 720-face union; `before.json`,
   `after.json`, and `diff.json` are the pre-refresh evidence.
   After refresh, measured on `oktskxwv`: covered **615**, Readings **29,034**;
   claim-parent delta **27 gains**, still 0 lost/decreases/increases/digest-only.
   `diff-incoming.json` isolates **Leela, Sevateem Warrior (0 → 15)** as the
   sole trunk gain, attributable to `zvoyutomxpyp`: its newly declared
   *Doctor's companion* reads through `BareKeyword` (one keyword-line Reading,
   plus 14 on its existing ability line). `after-refresh.json` and
   `diff-refresh.json` hold that census and comparison. After the second
   refresh, measured on `mxnowyml`: covered **616**, Readings **29,049**;
   claim-parent delta **28 gains**, still 0 lost/decreases/increases/digest-only.
   `diff-incoming-2.json` isolates **Love on the Battlefield (0 → 15)**,
   enabled by `oyqrxmwzntlz`'s *exactly* modifier: `AdverbCardinalModifier`
   under the Cardinal in *exactly two creatures*, alongside this feature's
   *either of those creatures* selected Complement. Final evidence:
   `after-refresh-2.json` and `diff-refresh-2.json`. Feature-only gains remain
   26; the two additional gains depend on incoming trunk changes.
   Latest quantity refresh, measured on `lnvmztyr` (code in the closed feature,
   verification folded into `mxnowyml`): covered **636**, Readings **31,052**;
   claim-parent delta **48 gains**, 0 lost/decreases/increases, **5 digest-only**.
   `pusrqwmlwqkm` brings 20 gains and seven digest-only changes relative to the
   prior recheck. Its Cardinal head is now wrapped as a quantitative
   Determiner; counts are unchanged: Accumulate Wisdom 12, Florian, Voldaren
   Scion 361, Haphazard Bombardment 8, Jeska, Thrice Reborn 40, See the Truth 6;
   Breena, the Demagogue 7 and Dimensional Breach 4 were feature gains earlier.
   The first five are the final claim-parent digest-only list. No Reading
   count decrease was imported. Final evidence: `after-refresh-3.json`,
   `diff-refresh-3.json`, and `diff-incoming-3.json`; 26 feature-only gains and
   22 incoming-dependent gains. Accepted integration refresh imported Waterbend
   plugin data and tests; `after-integrate-refresh.json` and
   `diff-integrate-refresh.json` confirm the same totals.
   `diff-incoming-integrate-refresh.json` reports all 720 faces unchanged.
   Integration was refused because default moved. The next refresh imported
   `mlnortmq` (declaration batch B): **Esix, Fractal Bloom (0 → 129)**
   gains its central *you may instead choose* analysis. Final bounded totals:
   **637 covered**, **31,181 Readings**, **49 gains**, 0 lost/decreases/
   increases, 5 digest-only; 26 feature gains and 23 incoming-dependent gains.
   Evidence: `after-integrate-refresh-2.json`, `diff-integrate-refresh-2.json`,
   and `diff-incoming-integrate-refresh-2.json`.
2. **Witnesses** — Claim-parent → `qnvlmzuu` fragment probes: Chrome Mox **0 → 1**;
   Arena of Glory **0 → 2**; Mindblade Render **0 → 9**; Ludevic's Test Subject
   **0 → 12**; Tamiyo, Field Researcher **0 → 1**. Plural *any* clause **0 → 1**;
   *Remove all of them.* **0 → 1**. *Destroy any of them that are Walls.*
   **0 → 0**, shared residual under PD7 above. NP witnesses *any of those
   cards*, *any of that mana*, *all of them*, *all of those cards*, *either of
   those creatures*, *another of those counters*, and *none of them* **0 → 1**.
   Controls unchanged: *each of those cards* 1, *one of those cards* 1,
   *Destroy one of them.* 1, singular *one* conditional 1, Stall for Time 1,
   Strategic Planning 3. W9-A independent controls replacing the partitive
   with its oblique give Arena 2, Mindblade 9, Ludevic 12; Tamiyo with the
   cardinal *one* gives 1. Chrome with *each* gives 1; the two unintended
   singular-count genitive routes were rejected by Countability selection.
3. **Spot checks** — Ten newly covered faces on `qnvlmzuu`, with selected
   `DeterminerHeadPartitiveNounPhrase` analyses: Chrome Mox (*any*, plural
   *the exiled card's colors*); Arena of Glory (*any*, mass *that mana*);
   Mindblade Render (*any*, mass *that damage*); Ludevic's Test Subject (*all*,
   plural pronoun *them*); Tamiyo, Field Researcher (*either*, plural *those
   creatures*, singular head); Clockspinning (*another*, plural *those
   counters*, singular head); Exotic Pets (*either*, plural *those tokens*,
   singular head); Call of the Death-Dweller (*either*, pronoun *them*,
   singular head); Satoru, the Infiltrator (*none*, plural pronoun *them*);
   Firemane Commando (*none*, plural *those creatures*). The *of* phrase is
   the selected Complement on each route, not a floated quantifier analysis.
4. **Deviations and STOPs** — Owner principle overrides PD5. The class is
   declared by shared `NominalComplementMarker: Of` membership in the
   existing lexical `feature_additions` rows of `lexicon/core.ron`, plus the
   native *none* row; no shared class row already spans these members.
   Members: **all, both, each, either, neither, another, any, some, most,
   many, much, more, fewer, less, none**. *Few* and *several* do not exist in
   this Lexicon and are not minted; Cardinal *one* keeps its separate route.
   No Construction or frame added/deleted. The existing partitive route is
   refined with feature tables; Countability is projected through NP and PP
   summaries to exclude singular count obliques from mass selection.
   `SingularOrPlural` Agreement admits both finite verb numbers without two
   NP Readings. Glossary explanations extend Countability and Determiner
   Agreement Number; PD2 supports project agreement, CGEL §9.1 p. 411 backs
   plural versus non-count obliques. The relative-modifier STOP is resolved
   by PD7: shared residual, no extension. A later refresh conflicted on
   `suite_3.rs`; Harmony resolved it in `qnvlmzuu`, retaining both modules
   exactly once, and the full suites passed. Refresh also restored the
   original WIP ticket beside a prematurely moved completed ticket. The
   completed record is restored to the owned WIP path: Kata integration,
   after review, owns the final move to `done/` (tickets README).
   No CR rule claims added.
5. **Tests** — All passed on `qnvlmzuu`:
   `cargo test -p deckmaste_english_v3` (985 tests);
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`;
   `cargo test -p deckmaste_lexical_source` (86 tests);
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings`.
   Citation checks have no noncompliant or stale citations; diff audit adds
   no rule claims. The same four commands passed again after refresh on
   `oktskxwv`: English 988 tests (three additional trunk tests), lexical source
   86 tests; both clippy checks clean. `cargo build -p xtask` passed before
   that recheck. After the exactly-cardinal refresh and registration repair,
   the build and all four commands passed once more on `mxnowyml`: English
   996 tests, lexical source 86 tests; both clippy checks clean. The quantity
   refresh on `lnvmztyr` also passed the build and the same full test/clippy
   commands: English **1,004 tests**, lexical source **86 tests**, both clippy
   checks clean. Kanban check reports no duplicates, cycles, or dangling
   needs. Every refresh with crate/plugin changes received a bounded census,
   full suites and clippy; no full corpus census was run.
   Accepted integration refresh also passed `cargo build -p xtask` and the
   same four full test/clippy commands: English **1,008 tests**, lexical
   source **86 tests**, both clippy checks clean. Evidence logs have the
   `integrate-refresh` suffix under the same target evidence directory.
   After that integration refusal and declaration refresh, the build and
   same four full commands passed again: English **1,015 tests**, lexical
   source **87 tests**, both clippy checks clean (`integrate-refresh-2` logs).
   Assurance: restored 0,
   re-spelled 0, ignored 0, added 9, removed 0; no existing test retired.
   The new module is registered once in `tests/suite_3.rs` and asserts
   selected roles, value equality, roundtrip laws, number selection,
   hypothetical class members, and the PD7 shared residual.
6. **Timing** — Final integration refresh bounded census: **18.92 s wall**
   (previous rechecks: 25.70 s and 41.41 s), 6 workers,
   0 samples per face (`/usr/bin/time -v`). Full corpus census omitted under
   the binding verification budget; the orchestrator owns that batch check.
