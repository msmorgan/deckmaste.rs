---
needs: []
---
# Declare the phrasal partner-variant keywords: Choose a Background

## Why

Three families of parameterless keyword lines are spelled as phrases and have
no declaration, so their lines read nowhere. Blocker report at trunk
`ymqwzour` (`target/english-v3/blockers-wave8.txt`): keys `at:background
<end>` 26, `at:<start> doctor's` 21, `word:forever` 5 (plus `word:character`,
`word:select`, `word:father`, `word:son`, `at:survivors <end>`).
`cargo xtask english-v3 grep` counts, all unread:

| Surface | Pattern | Faces | Sole failing line |
|---|---|---|---|
| *Choose a Background* | `Choose a Background` | 32 | 26 |
| *Doctor's companion* | `(?i)doctor's companion` | 28 | 22 |
| *Partner—Friends forever* | `Partner—Friends forever` | 7 | 5 |
| *Partner—Character select* | `Partner—Character select` | 6 | 5 |
| *Partner—Survivors* | `Partner—Survivors` | 4 | 4 |
| *Partner—Father & son* | `Partner—Father & son` | 2 | 2 |

**79** faces, **64** with the line as their only failing line. Two of the
*doctor's companion* faces also use it as a mention (*a card with doctor's
companion*, An Unearthly Child; *creature spell with doctor's companion*,
Rose Noble).

This is a data ticket: a declaration the existing `BareKeyword` route reads.

## Goal

Each surface above reads as a Keyword Line through its own keyword
declaration, and *doctor's companion* also as a keyword mention, with
byte-exact realisation.

## Analysis

No CGEL citation applies (Oracle keyword names).

Existing route: `BareKeyword` (`crates/deckmaste_english_v3/src/
declarations.rs`, `[head: lexical(Keyword)]`, `require
bare_keyword_use(head.KeywordParameterClass) = Yes`) under `KeywordLine`.
Multiword keyword surfaces are already data: `firstStrike.ron`
(`FixedKeyword(surface: "first strike")`), `splitSecond.ron`,
`cumulativeUpkeep.ron`. `partner.ron` declares only `surface: "partner"`
("Partner" reads 1). Today the lexer sees *Doctor* + *'s* + keyword
*companion*, *Choose* + *a* + *Background*, and keyword *partner* + an
unknown word.

Orchestrator rulings 2026-10-10:

- PK1. One keyword declaration per surface under
  `plugins_v2/builtin/macros/keyword_abilities/`, `params: []`, `grammar:
  FixedKeyword(surface: …)` only, no engine body (the bodyless precedent is
  typecycling's R8a; bodies belong to `semantics-v2-keyword-ability-census`).
  Six declarations. No Construction is added or changed.
- PK2. Each partner variant is its own keyword whose surface is the whole
  printed phrase including the dash (*partner—friends forever*). It is not
  the keyword *partner* plus a dashed parameter: the variants share no
  parameter class and name different abilities. *Partner with <name>* is a
  different thing (Out of scope).
- PK3. Surfaces are declared as printed mid-line, with line-initial capital
  applied positionally: *choose a Background* keeps its capital subtype
  word; *doctor's companion* is lowercase (attested in both mentions);
  *partner—friends forever* is declared so that the printed *Partner—Friends
  forever* realises byte-exactly. If the surface machinery cannot carry an
  internal capital, an apostrophe clitic, an em dash or *&* inside a declared
  multiword surface, fix that in the surface compiler
  (`deckmaste_lexical_source` / `deckmaste_construction_core`) for declared
  surfaces generally. If it would need a grammar Construction instead, STOP.
- PK4. One analysis per string. The twins to avoid: (a) *Choose a
  Background* as an imperative Sentence without its period (today "Choose a
  Background." with a period reads 1 as a Sentence; that string is not on
  any card and its count must not change); (b) *Doctor's companion* as a
  genitive NP or as the *companion* keyword with a stray possessor;
  (c) *Partner—Friends forever* as `BareKeyword` *partner* followed by a
  dashed payload.
- PK5. Newly covered faces are gains. No Reading-count increase on a covered
  face is expected: the 51 covered faces matching `\bPartner\b` and the
  covered *companion* and *Background* faces must keep their digests. Any
  increase is reported with its shape before landing.

## Witnesses

Each is the face's only failing line. Probed with the `ymqwzour` binary
(`--category document --readings 3`): witness 0. Re-probe on the claim
parent first.

- Abdel Adrian, Gorion's Ward: "Choose a Background"
- Clara Oswald: "Doctor's companion"
- Bjorna, Nightfall Alchemist: "Partner—Friends forever"
- Leonardo, the Balance: "Partner—Character select"
- Kratos, Stoic Father: "Partner—Father & son"

Also 0 today: "Partner—Survivors" (Joel, Resolute Survivor). Controls that
must not change: "Partner" 1; "First strike; swampwalk" 1; "Choose a
Background." (sentence, with period) 1; a *Companion —* line (Lurrus of the
Dream-Den) and a Background Type Line keep their digests in the census.
The mention control "… creature spell with flying, draw a card." was not
probed: the trunk moved under the prebuilt binary; probe it on the claim
parent.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `(?i)\b(partner|background|companion)\b` (198 faces) plus 300
random covered controls (`--random 300 --seed 20261010`), `--workers 6
--samples-per-face 0`, compared with `cargo xtask english-v3 diff` against
the claim parent. Zero lost; PK5 for increases. `cargo test -p` and `cargo
clippy -p … --tests -- -D warnings` for `deckmaste_english_v3`,
`deckmaste_lexical_source`, `deckmaste_construction_core` and
`deckmaste_semantics_v2` (keyword inventories count declarations; re-spell
those tests, never delete them). Plugin `grammar:` data loads at runtime, so
no grammar rebuild is needed unless PK3's fallback is hit. Tests assert that
each line is one `BareKeyword` whose head is the new Lexeme, and the PK4
negatives. Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- *Partner with <name>* (52 faces, 43 sole): the keyword *partner* with a
  *with* + Name parameter; needs its own ticket.
- Engine bodies and deck-construction semantics of every partner variant.
- *Legendary Background* static text beyond the keyword line.

## Landing record

To be written by the claimant: the six items of `docs/tickets/README.md`.
