---
needs: [english-v3-vocabulary-batch-1]
---
# Vocabulary batch 2: the next unknown words that existing routes already read

Priority: high. Same contract as `english-v3-vocabulary-batch-1`: its rulings
V1–V9 apply verbatim, with these deltas.

## Why

After batch 1 the blocker report (`cargo xtask english-v3 blockers`) still
lists several hundred unknown words; at trunk `vwlkmvns`, before batch 1
landed, 487 distinct unknown words were the sole blocker on 1,148 faces and
1,673 unread faces were vocabulary-only.

## Goal

Declare the next vocabulary-only words, most valuable first.

## Analysis

Orchestrator rulings 2026-10-10 (deltas to batch 1):

- W1. **Candidates** come from the `word:` table of
  `/home/msmorgan/Projects/deckmaste.rs/target/english-v3/blockers.json`,
  ranked by sole-blocker faces, skipping every word already known on the
  claim parent and every word on batch 1's follow-up lists ("needs
  construction" and "blocked on declaration path"). Work down until 80 words
  are declared, the next word is sole blocker on fewer than 3 faces, or the
  time box ends.
- W2. A word that fails the vocabulary-only test is listed and skipped (V2,
  V6, V7); never STOP for a single word.
- W3. Adjectives that are attested only attributively go in the
  `AttributiveAdjective` class (V9 notes that class does not yet block
  predicative use; that is a separate grammar-hygiene follow-up).
- W4. Adverbs get every attested position licence, not only the first one
  that makes a witness read (batch 1 under-declared *secretly*).
- W5. Test rows pin the new word's role (the verb's predicate span, or the NP
  headed by the noun), never the whole unit at its own start category.

## Method

As batch 1: passes of about 15 words; one bounded census at the end on every
face containing a declared word plus 300 random covered controls; zero lost;
zero increases on covered faces; full suites and clippy for every crate whose
code or consumed data changed; `cargo xtask facts check` if a keyword
definition's `grammar:` changed. Time box: three hours, then finish the pass
and verify.

## Landing record

As batch 1: per-word table, two witnesses per word, fifteen spot checks across
at least ten words, the follow-up list split by reason.
