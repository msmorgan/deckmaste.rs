---
needs: []
---
# Vocabulary batch 1: declare the commonest unknown words that existing routes already read

Priority: high. Data only. This ticket is deliberately a batch: the
one-construction-per-ticket rule is about grammar changes, and a declaration
an existing Construction already reads is not one. Owner decision 2026-10-10.

## Why

At the per-line baseline (`sskyxnyvmruk`; file
`/home/msmorgan/Projects/deckmaste.rs/target/english-v3/baseline-per-line.json`)
2,027 of 6,292 unread faces contain at least one word with no lexical
reading. The commonest, by unread faces (surface counts, not gain
forecasts; *divided* and *produce(d)* have since landed):

*starting* 84, *half* 83, *rounded* 77, *historic* 56, *tied* 51, *declared*
48, *modified* 47, *exactly* 45, *party* 40, *take* 40, *kind* 35, *pile(s)*
34 + 27, *door* 33, *difference* 32, *move* 32, *mutates* 32, *using* 32,
*excess* 30, *deck* 28, *crime* 27, *opening* 27, *paired* 27, *result* 26,
*begin* 25, *crews* 25, *switch* 25, *true* 25, *draft* 24, *promised* 24,
*exploits* 23, *bargained* 20, *unspent* 20, *commit* 19, *empower* 19,
*multikicker* 19, *completed* 17.

Each has so far cost a whole ticket (*produce*: 1 h 43 min for 43 faces).

## Goal

Declare every candidate word that is **vocabulary-only**: a word whose
witness reads once the word is known, through Constructions and frames that
exist today. Words that need a new Construction or frame kind are not
implemented; they are listed for follow-up tickets.

## Analysis

Orchestrator rulings 2026-10-10:

- V1. **Candidates.** Take the unknown words of the unread faces in the
  baseline file above (`.faces[] | select(.census == "no") | .unknown_words`),
  lower-cased and grouped by lemma, and work down from the most frequent
  until 60 words have been declared or the next word touches fewer than 8
  unread faces. Skip any word already known on the claim parent.
- V2. **The vocabulary-only test.** For each candidate pick two attested
  units (`cargo xtask english-v3 grep`, `--as-card` probes). Substitute a
  declared word of the category and frame you intend to give it (the
  *produce* ticket's control was *add*) and probe. If the substituted unit
  reads at least once with the intended analysis, the candidate is
  vocabulary-only: declare it. If no substitution reads, it is not: record
  the word, its face count and the missing piece in the follow-up list and
  move on. Do not build a Construction, a frame kind, a Role or a feature in
  this ticket.
- V3. **Where the declaration goes.** A keyword action, keyword ability or
  other game term owned by a `plugins_v2` definition (*mutate*, *crew*,
  *exploit*, *bargain*, *multikicker*, *historic*, *party*, *modified*, …)
  is declared through that definition's `grammar:` field or the macro that
  declares the term, never as a free-standing core word. Ordinary English
  words go in `lexicon/verbs.ron` or `lexicon/core.ron`, in the shortest
  form the loader accepts; forms the loader derives are exempt from the
  pruning rule (ruling R16e, `english-v3-produce-verb`).
- V4. **Attested frames and categories only.** A verb gets the frames its
  corpus uses need, copied from an existing verb with the same
  complementation; a noun its attested countability; an adjective its
  attested positions. Categories follow CGEL, cited only where the passage
  says it (for example *exactly* as a focusing modifier), otherwise cited as
  this ruling.
- V5. **One lexical reading per use.** A new declaration must not give an
  already-covered face a second Reading by homography with an existing
  lexeme. Any Reading-count increase on a previously covered face is a
  defect: narrow the declaration or drop the word to the follow-up list.
- V6. *half* / *rounded up/down*, *exactly N*, *tied for*, *the difference
  between* and similar are expected to fail V2 because they need a
  Construction. That is the intended outcome for them: list, do not build.

## Witnesses

Chosen during the work under V2 and recorded per word: two units per declared
word, each 0 before and at least 1 after with the intended analysis.

## Method

Standard constraints apply. Deltas:

1. Verification budget: one bounded census after all declarations, on every
   supported face containing any declared word (`cargo xtask english-v3 grep
   -i --names`, one pattern per word, unioned) plus 300 random covered
   controls, `--workers 6 --samples-per-face 0` under `/usr/bin/time -v`,
   compared with `cargo xtask english-v3 diff` against the claim parent on
   the same manifest. Zero lost faces; zero increases on covered faces (V5).
2. Tests: one table-driven test over the declared words (each row: card,
   unit, category, expected Reading count, one asserted constituent), plus
   the lexical-source test for each new declaration class. No test pins a
   sample cost or scans a Debug-printed tree.
3. Full `cargo test -p` and clippy for `deckmaste_lexical_source`,
   `deckmaste_english_v3` and any plugin-consuming crate whose data changed;
   `cargo xtask lean check` only if a plugin definition's non-`grammar:`
   content changed (it must not). `cargo xtask facts check` if a keyword
   definition gained a `grammar:` field.
4. No guard may name a lexeme, construction, verb, noun, preposition or
   card. No grammar edits at all; a needed one sends the word to the
   follow-up list.

## Out of scope

- Any Construction, frame kind, Role or feature.
- Words below the V1 cut-off; they are batch 2, chosen from
  `english-v3-blocker-report` output once it exists.

## Landing record

Six items as specified in `docs/tickets/README.md`, with these shapes:

1. **Census delta** on the bounded manifest, plus a per-word table: word,
   category/frame, where declared, unread faces touching it before, faces
   gained that contain it.
2. **Witnesses**: the two units per declared word with counts after.
3. **Spot checks**: fifteen gained faces across at least ten different
   words, with the selected analysis.
4. **Deviations and STOPs**, and the **follow-up list**: every candidate
   that failed V2, with face count and the missing piece in one line.
5. **Tests**: commands, outcomes and counts.
6. **Timing**: bounded census wall time and peak RSS.
