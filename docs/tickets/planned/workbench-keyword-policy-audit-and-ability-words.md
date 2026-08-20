---
needs: []
---
# Audit the keyword classification, and transcribe the ability words

Two delegable tasks, neither needing a designer: both are inventory work against
a rubric that is already written down. They are grouped because they touch the
same keyword-authoring surface and can be claimed together or apart.

## 1. Keyword classification audit

`docs/keyword-policy.md` declares **9 intrinsic** (5 implemented), **19
composite-given**, **215 composite**, **1 marker**. The user estimates ~90%
confidence in that classification and suspects stragglers that should be
composite, plus one or two missed entries. The rubric is written down, so every
entry is checkable one at a time.

Do this first: the descriptive classification is pinned to **mtg-rules skill
v1.7.0** — confirm that pin is current before auditing, since a stale pin is a
plausible source of the misses.

(Check `docs-keyword-policy-refresh` before starting; if that ticket already
covers part of this surface, fold rather than duplicate.)

## 2. Ability words

Settled by user ruling: the shape is `AbilityWord Name Ability` — the word is
retained because it prints, and the shape correlation is authored as a macro. No
design dialogue is owed. What is missing is the measurement and the
transcription: measure the attested inventory of ability words, then transcribe
them against that shape.

## Consumption boundary

`docs/keyword-policy.md` for part 1; for part 2 the ability-word macro
definitions under `plugins/builtin/macros/` and whichever grammar site names the
words. No engine change in either part.

## Acceptance

- Part 1: every entry checked against the rubric, with the mtg-rules pin
  confirmed current first; reclassifications and misses listed.
- Part 2: the attested inventory is measured and transcribed; no word is added
  that the corpus does not print.

Standard constraints apply.
