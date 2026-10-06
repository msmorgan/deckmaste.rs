---
needs: []
---
# Delete every lexeme with zero tokens in the supported corpus

## Why

The user ruled 2026-10-06 that the grammar should support all and only Magic
vocabulary. These lexemes "snuck in" and appear on no card.

## Goal

The lexicon (`crates/deckmaste_lexical_source/lexicon/core.ron`, `verbs.ron`
and every other lexicon file in that directory) declares only vocabulary that
occurs on at least one supported-corpus face. Known dead entries found
2026-10-06: the prepositions *against*, *because*, *within* and *through*; each
carries a `PrepositionFunctionLicence` but has zero supported tokens. Audit the
whole lexicon, every category (prepositions, verbs, nouns, adjectives,
determiners and the rest), not just those four. The governing rule is the
"Pruning rule for licensed functions and attachments" in
`docs/decisions/english-lexical-analysis.md`.

## Method

1. Measure before: full corpus `cargo xtask english-v3 --all --workers 12
   --samples-per-face 0 --output <gitignored or scratch path>`. Record the
   covered-face count and total Readings, stamped with the change id.
2. Determine supported tokens per lexeme. Use whatever the xtask already
   exposes (check `cargo xtask english-v3 --help` and
   `crates/xtask/src/english_v3/` for a token/lexeme census; the lexicon loader
   reports lexeme-to-token mappings). If no existing output gives per-lexeme
   token counts, a throwaway script run from scratch space is acceptable, but
   it must not be checked in: no new xtask subcommands, flags, fixtures or
   tooling (standing rule).
3. Delete every lexeme with zero supported tokens, including any frames,
   feature declarations and plugin references that only it used. A
   `plugins_v2` keyword-action `grammar:` frame field that references a deleted
   lexeme may be edited; plugin bodies are untouched.
4. Measure after: full corpus again. Covered faces must be unchanged (zero
   lost faces); Readings may only decrease. Any lost face is a defect: fix it
   within the ticket first; STOP and report only if the fix fails or needs a
   ruling.
5. Gate: `cargo xtask gate --changed --from <claim change id> --run`; clippy
   and fmt clean; `cargo xtask cite check --list-noncompliant` reports 0 from
   the workspace root.

## Landing record

Standard PROVE/DISCLOSE/REPORT (see `CLAUDE.md`), plus:

- every deleted lexeme, grouped by category, with its zero-token evidence;
- before/after covered-face and Readings counts, each stamped with its change
  id;
- test counts: restored, re-spelled, ignored, added, removed. A test that named
  a deleted lexeme is re-spelled against a live lexeme with the same asserted
  outcome, never deleted;
- timings as integers in nanoseconds (e.g. `30,000,000,000 ns`), never decimal
  seconds.

## Out of scope

Adding vocabulary, changing any licence on a surviving lexeme, and grammar
changes beyond removing dead references.
