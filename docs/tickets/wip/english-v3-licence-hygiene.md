---
needs: []
---
# Prune unattested licences and harden two licence bypasses and one separator rule

Priority: medium.

## Why

Hygiene ticket. It adds no construction. It collects four code findings
from the reviews of the except-matrix-complements landing (change `rysrlqry`,
Opus review 2026-10-08) and the gerund-preposition-complements landing. Each
item is an overgeneration: a Reading that parses on a string Oracle text does
not attest. Expected coverage gain: about 8 faces from item 4; 0 from the
others. Zero lost faces required. The deliverable is Readings that disappear on the negative probes below, with
every attested Reading kept.

Line numbers are those of the claim parent this ticket was minted on.

## Items

### 1. Remove the `core-verb:Have` Nominal `MatrixComplementLicence`

`crates/deckmaste_lexical_source/lexicon/core.ron:62`:

```ron
"core-verb:Have": {"MatrixComplementLicence": "Nominal", "PurposeAdjunctLicence": "No"},
```

The licence's only corpus witness is Scheming Fence ("This creature has all
activated abilities of the chosen permanent except for loyalty abilities.").
That face is unread for unrelated reasons (*loyalty abilities*), so the
licence covers zero faces. It also overgenerates: "This creature has flying
except for three." reads 1.

Under the ADR pruning rule (`docs/decisions/english-lexical-analysis.md`,
"Pruning rule for licensed functions and attachments"), a licence with zero
attested uses and zero lost faces is removed. Drop the
`"MatrixComplementLicence": "Nominal"` key only. Keep
`"PurposeAdjunctLicence": "No"`. `MatrixComplementLicence` defaults to `None`
(`declarations.rs:63`).

- Negative witness: "This creature has flying except for three." 1 → 0.
- Scheming Fence stays at 0 either way. Record it as a residual for the
  loyalty-ability work. Whoever makes *loyalty abilities* read re-adds the
  licence then, with Scheming Fence as its witness.

### 2. `MatrixComplementPredicate` must reject a passive host that already has a *by* adjunct

"Slivers can't be blocked by Walls except by Slivers." reads 1 today. The
Reading is `MatrixComplementPredicate` over `PrepositionPredicate(blocked,
by Walls)`. The construction (`declarations.rs:1314–1324`) requires
`head.InternalisedComplementPresent = No`. Only `MatrixComplementPredicate`
and `InternalisedComplementPredicate` (`:1326–1336`) set that feature to
`Yes`. `PrepositionPredicate`'s `SecondaryVerbPhrase` instance (`:3216–3218`)
uses `SecondaryProjection` (`:815–821`), which copies the head's `No` through
even when the attached PP is *by*-marked. So a *by* PP attached as a plain
adjunct is invisible to the check. The doubled agent string is not attested
in the corpus.

Fix through declared features, never a word guard. Proposed fix:

- *by* declares `InternalisedComplementMarker = Yes` (`core.ron:195`), and the
  PP exports it through `PrepositionHeadPermissions` (`declarations.rs:888`).
- Give the `SecondaryVerbPhrase` row of the `PrepositionPredicate` instance
  its own projection policy. It is identical to `SecondaryProjection` except
  that it exports `InternalisedComplementPresent` from a new table over
  `(head.InternalisedComplementPresent, modifier.InternalisedComplementMarker)`.
  That table yields `Yes` when either argument is `Yes`.
- Do not change `SecondaryProjection` itself: `FrequencyPredicate` and the
  matrix schema share it.

After this change, a host that has an internalised-marker PP, whether
attached as a complement or as an adjunct, counts as filled.
`MatrixComplementPredicate` and `InternalisedComplementPredicate` then
reject it through their existing `require`s. If the census shows a lost face,
look for an alternative that reads the same declared features before you
STOP. One candidate is a direct
`require` in a dedicated host check.

- Negative witness: "Slivers can't be blocked by Walls except by Slivers."
  1 → 0.
- Positive control: "Slivers can't be blocked except by Slivers." stays 1.

### 3. `PrepositionComplementPreposition` launders a complement-function restriction

The gerund landing added `ComplementFunctionRestriction`. Without declares
`Adjunct` and Of declares `Compound` (`core.ron:202,210`). A gerund PP
exports its head's restriction (`GerundComplementPreposition`,
`declarations.rs:2813–2828`, export at `:2821`), and so does a Without NP
PP (`:2672`). Every consumer of a PP checks that restriction:
`PrepositionPredicate` (`:1288`), `CompoundPrepositionPhrase` (`:2850`),
`PredicativePreposition` (`:2834`), `SourceComplement`/`LocativeComplement`
(`:2920,2928`), and the nominal sites (`:2415–2457`).

`PrepositionComplementPreposition` (`declarations.rs:2900–2914`) is the gap.
It takes a `PrepositionPhrase` Complement and checks only
`complement.MatrixComplementRequirement = None` and the source licence. It
never reads `complement.ComplementFunctionRestriction`, and it exports
`ComplementFunctionRestriction = None` (`:2903`). Of declares
`PrepositionComplement` `NounPhrase|PrepositionPhrase` and
`PrepositionPhraseComplementLicence = Unrestricted` (`core.ron:202`), so an
Of head can take any restricted PP as its Complement. Examples are *of
without paying its mana cost* (Adjunct-restricted) and *of of paying …*
(Compound-restricted, the shape the gerund landing fixed on Jazal Goldmane).
The result is an unrestricted Of PP that is then licensed as a noun Modifier
or Predicative Complement. That is the bypass: a gerund that its own
preposition admits only as an Adjunct or inside a compound PP can reach
nominal and predicative functions one level up. From is not affected, because
its `Source` licence requires `SourceComplementUse = Yes`, and Without and Of
PPs export `No`.

The fix is one `require` on a declared feature in
`PrepositionComplementPreposition`, matching the other Complement-function
consumers:

```
require complement.ComplementFunctionRestriction = None;
```

An embedded PP Complement fills a Complement function. It is neither an
Adjunct nor a compound-PP internal, so a restricted PP cannot fill it.

- Negative witness (write it as a test first, and confirm it reads ≥ 1 before
  the fix): "Exile a card of without paying its mana cost." → 0. If that
  string reads 0 before the fix for an unrelated reason, find a probe that
  reaches `PrepositionComplementPreposition` over a restricted Complement,
  such as a predicative *is of without …* or an *of of paying …* string.
  Record which probe was used. If no probe reaches the path, record the
  `require` as closing an unreachable path and keep it.
- Positive control: an attested Of/From PP-Complement Reading that already
  reads stays unchanged. *from among* or *from outside* faces are examples.
  The census per-face diff covers the rest.

`MatrixComplementPreposition` (`:2876–2895`) also reads no
`complement.ComplementFunctionRestriction`. Its Complement is constrained by
`matrix_pp_requirement`, and By's restriction is the default `None`, so no
bypass is known there. Do not change it in this ticket.

### 4. `LicensedMatrixComplementPredicate` must not key the host class on the separator

`declarations.rs:1301–1308` has two forms: a nominal exception (*except for
…*) admitted only after `" "`, and a content clause (*except it …*) admitted
only after `", "`. Oracle attests both separators for both classes (audit
2026-10-08, counts are matching cards in `data/scryfall/oracle-cards.jsonl`):
nominal hosts with ", except for" occur 3 times (Season of the Witch, Total
War, Eye of Singularity), and content hosts with a bare " except it/its" occur
5 times (Mirrored Lotus, Hosting Season, Faerie Artisans among them) against
197 with the comma. All eight faces read 0 today. Owner ruling 2026-10-08: a
construction may use a comma to decide between analyses, but it must not be
keyed on a comma where Oracle attests both spellings with the same structure.

Fix: let both separators admit both host classes. The host class is already
fixed by `licensed_matrix_complement(head.MatrixComplementLicence,
modifier.MatrixComplementRequirement)`, so the separator carries no
information the licence does not. Do not add a stored punctuation bit beyond
the form index that already exists; the two forms may stay as `" "` and `", "`
variants each admitting either requirement, or collapse to one form if the
construction language allows a separator alternative. No new construction.

- Positive witnesses (0 → ≥1): "Destroy all creatures, except for Walls."
  (Season of the Witch shape); "Create a token that's a copy of target
  creature except it isn't legendary." (bare content clause).
- Controls unchanged: every face the except landing gained (112, listed in
  `docs/tickets/done/english-v3-except-matrix-complements.md`) keeps its
  digest; "Draw a card except for three." stays 0 (licence, not separator,
  rejects it).
- A new spurious Reading on an already-covered face from the widened
  separator is a defect; the census per-face diff finds it.

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, scope).
Deltas:

1. Verification budget (binding): One full census (`--all --workers 12
   --samples-per-face 0`) on the finished feature, compared face by face
   (per-face `readings_sha256` and counts) against the claim parent's census.
   Iterate on selector subsets, never on `--all`. Tests:
   `cargo test -p deckmaste_english_v3` and `cargo test -p
   deckmaste_lexical_source`, with clippy on both (core.ron changes). No
   xtask tests, no Lean checks. No post-refresh re-census unless refresh
   touched grammar or lexicon. New tests go under `tests/suite/` and are
   registered in `tests/main.rs`. Evidence lives under the workspace's
   ignored `target/english-v3/`, never `/tmp`.
2. Write each negative probe and each positive control as a test first.
3. Zero lost faces. A lost face is a defect: fix it within the ticket first,
   and STOP only if the fix fails or needs a ruling. A covered face whose
   Reading count drops is expected only where the dropped Reading is one of
   the overgenerations named above. Name each such face and the Reading it
   lost.
4. No guard may name a lexeme, construction, verb, noun, preposition or card.
   Read declared features only. Tests are re-spelled, never deleted. No new
   xtask subcommands, flags, fixtures or tooling.
5. No CR citations. A CGEL citation may back only what the cited passage
   itself says. A project ruling is cited as a ruling.

## Landing record

1. **Census delta.** Original claim-parent sources (`rzkrrnzp`) match full census `qxqootwz`: **23,352 covered / 1,199,416 Readings**. The one feature full run (`xyoyxkkn`) yields **23,353 / 1,199,440**. After signed-cost grammar `vvxtllnq`, trunk census `xnkuznpo` has **23,459 / 1,605,535**; reconciled with the refreshed subset `tvsnunmn`, the final tally is **23,460 / 1,605,559**. Feature gain: Season of the Witch, **24 Readings** (`xyoyxkkn`, `tvsnunmn`); inherited gains: **107 faces / 406,119 Readings** (`xnkuznpo`), with **0** inherited decreases. Lost faces and removed Readings: none. All previously covered counts and digests are retained. Admission, exact realization and node/word traversal pass without issues or internal failures. The record's census is the full run `xyoyxkkn` plus the refreshed **853-face** subset `tvsnunmn`, chosen as every face the touched constructions can reach in the affected licence/separator configurations, including every incoming trunk delta face. No further full run was performed, per the owner's ruling.

2. **Witnesses.** Supported full-face counts below are from `tvsnunmn`; failing-unit probes and scope classification are at `pstyvpqn`.

   | Face | After | Failing unit or selected analysis |
   | --- | ---: | --- |
   | Season of the Witch | 24 | Nominal exception on *destroy*, with the comma form retained. |
   | Total War | 0 | NP *creatures the player hasn't controlled continuously since the beginning of the turn*: undeclared *hasn't* and *continuously*. |
   | Eye of Singularity | 0 | NP *the same name as another permanent*; the host fails even without its exception, while *destroy each permanent, except for basic lands* reads. |
   | Faerie Artisans | 0 | NP *a nontoken creature* fails; its full *create … except it's an artifact in addition to its other types* predicate has **8** Readings with either separator (`pstyvpqn`). |
   | Mirrored Lotus | Not selected | Vintage `not_legal`, `set_type: funny`; excluded from the supported census. Its cited exception is also inside reminder text. |
   | Hosting Season | Not selected | Vintage `not_legal`, `set_type: funny`; excluded from the supported census. |
   | Awoken Nephilim | Not selected | Other bare-*except it* match: Vintage `not_legal`, `set_type: funny`. |
   | Enolc, Perfect Clone | Not selected | Other bare-*except it* match: Vintage `not_legal`, `set_type: funny`. |

   The supported separator witnesses explain the estimate: one gain and three unrelated failures; the other four matches are outside the supported corpus. At `tsquzqnr`, the ticket's comma nominal probe has **1** Reading and bare content-clause probe **2**. Scheming Fence remains **0** (`tvsnunmn`), routed to loyalty-ability work; its stated loyalty-ability probe remains **0** (`tsquzqnr`).

3. **Spot checks.** Season of the Witch (**24**, `tvsnunmn`) is the one gained face: *except for creatures that couldn't attack* is a nominal matrix exception on *destroy*, preserving the comma form. Ticket negatives (`tsquzqnr`, before/after): “This creature has flying except for three.” **1 → 0**; “Slivers can't be blocked by Walls except by Slivers.” **1 → 0**; “Exile a card of without paying its mana cost.” **1 → 0**; “Draw a card except for three.” **0 → 0**. Positive controls (`xnkuznpo` → `tvsnunmn`): Shifting Sliver **1 → 1**, with its exception on an unfilled Bare Passive; Satyr Wayfinder **6 → 6**, retaining nested *from among them* as a PP Complement. This follows the owner's hygiene ruling: ten is a cap, not a quota.

4. **Deviations and STOPs.** Own changes (`tsquzqnr`): **1** feature table, **1** dedicated Secondary Verb Phrase PP projection policy and **9** tests; no constructions, categories or frames added or deleted. Removed Have's nominal matrix licence, retaining its purpose exclusion; the shared nominal route remains live for other licensed hosts. Existing separator form indices are preserved. Guards read declared features; the separator comment attributes the owner's ruling, with no new CGEL/CR claims. Glossary gaps: none. Refresh inherited `vvxtllnq`; after a safe concurrent-edit refusal, Harmony repair at `tsquzqnr` retained both test registrations and cleared the append conflict. Both record STOPs are resolved by the owner's rulings: use the gain, negative probes and positive controls for hygiene spot checks, and retain the one full run plus the **853-face** refreshed subset (`tvsnunmn`). The named-face diagnosis (`pstyvpqn`) confirms no residual separator defect; unrelated failures and scope exclusions explain the smaller gain.

5. **Tests.** `cargo test -p deckmaste_english_v3`: **487 passed** (`tsquzqnr`), **495 passed** after refresh (`tvsnunmn`). `cargo test -p deckmaste_lexical_source`: **57 passed** (`tsquzqnr`). `cargo clippy -p deckmaste_english_v3 -p deckmaste_lexical_source --tests -- -D warnings` passes (`xyoyxkkn`); refreshed `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` passes (`tvsnunmn`). No lexicon change arrived in that refresh, so lexical-source results remain applicable. Assurance (`tsquzqnr`): restored **0**, re-spelled **0**, ignored **0**, added **9**, removed **0**. Tests use `tests/main.rs` → `tests/suite/`; nightly formatting passes. No xtask tests or Lean checks ran. Full census (`xyoyxkkn`), refreshed subset (`tvsnunmn`) and residual probes (`pstyvpqn`) pass their declaration/roundtrip/traversal checks; evidence stays under ignored `target/english-v3/english-v3-licence-hygiene/`.

6. **Timing.** Full census (`xyoyxkkn`): **124.168 seconds**, **12 workers**.

## Residuals routed to wave-4 recon

Out of scope here. They are recorded so the next recon pass sees them. Counts
marked "rg" were checked on the local MTGJSON snapshot (Vintage-legal names,
surface match, not unread counts). All other counts are as reported by the
reviews.

- (a) **"except it …"**: 115 faces unread (reported by review), mostly on
  hosts other than Create/Copy/Become, likely *enter as a copy of*.
- (b) **Matrix *except by*** on hosts outside the passive-blocked route:
  Island Sanctuary ("can't be attacked except by"), Raging River, Agility
  Bobblehead, Tatsunari, Toad Rider.
- (c) From the gerund landing: 15 *anywhere else* faces unrouted (listed in
  `english-v3-gerund-preposition-complements`). Unlicensed gerund
  prepositions: *for having* (8 cards, rg), *after being* (1 card, rg),
  *while -ing* (6 faces, reported).
- (d) The *instead of* gerund residual: the compound-gerund hosts that stay
  unread after the gerund landing (its `compound-residuals.json`).
- Scheming Fence (item 1): blocked on *loyalty abilities*. When that reads,
  re-add the Have Nominal matrix licence with it as witness.
