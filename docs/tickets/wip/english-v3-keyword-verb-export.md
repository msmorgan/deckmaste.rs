---
needs: []
---
# Let a fixed keyword export its verb: this creature mutates / crews a Vehicle / exploits a creature

Priority: high. One mechanism, a batch of terms.

## Why

`english-v3-vocabulary-batch-1` found keyword terms that pass the
vocabulary-only test (their witness reads once the word is known) but have no
declaration path: the keyword is declared through `grammar: FixedKeyword(…)`
in its `plugins_v2` definition, and that row has no way to export the verb
the keyword is also used as. Unread faces touching each (blocker report,
trunk `vwlkmvns`): *crew(s/ed)* 36, *mutate(s)* 33, *paired* 27, *exploit(s)*
24, *saddle(s/d)* 16, *escape(s/d)* 16, *haunt(s)* 9; about 160 in all
(surface counts, not gain forecasts). By owner ruling these terms are declared
by the macro that owns them, never as free-standing core words.

## Goal

A `FixedKeyword` grammar row can declare the verb use of its keyword (lemma,
attested frames, attested forms by the regular paradigm), and the lexical
loader exports it as a verb lexeme owned by that definition. Then declare the
verb for each term above whose witness reads.

## Analysis

Orchestrator rulings 2026-10-10:

- KV1. Extend the existing grammar row (`crates/deckmaste_construction_core/src/macro_def.rs`,
  the `FixedKeyword` fields near `participial_adjective`) with one optional
  field for the verb use, reusing the verb-declaration shape `verbs.ron`
  already has (frames by the existing frame kinds). No new frame kind, Role
  or Construction; no grammar (`declarations.rs`) edit.
- KV2. The exported verb is one lexeme per keyword, owned by the definition.
  Where the keyword already exports a participial Adjective (*crewed*,
  *paired*), the passive/perfect participle of the new verb must not give any
  face two Readings for one constituency: if both would read the same string,
  keep the verb and retire the adjective export for that keyword, re-spelling
  its tests.
- KV3. Vocabulary-only test per term (ruling V2 of batch 1): substitute a
  declared verb with the same frame; if the substituted unit reads, declare;
  if not, list the term with the missing piece and move on. Do not STOP for a
  single term.
- KV4. Attested frames only (`cargo xtask english-v3 grep`): for example
  intransitive *mutates*, transitive *crews a Vehicle* / *exploits a
  creature*, passive *becomes crewed* / *is paired with*.
- KV5. Plugin bodies are untouched; only `grammar:` fields change.
  `cargo xtask facts check` after; `cargo xtask lean check` is not needed
  unless a non-`grammar:` field changed (it must not).

## Witnesses

Two attested units per declared term, chosen under KV3 and recorded with
before 0 and after ≥ 1 with the verb analysis.

## Method

Standard constraints apply. Bounded census: every supported face matching any
declared term's surface forms plus 300 random covered controls; zero lost;
zero increases on covered faces (KV2). Full suites and clippy for
`deckmaste_construction_core`, `deckmaste_lexical_source`,
`deckmaste_english_v3` and any other crate consuming the changed data. Tests
assert the exported lexeme's frames and each witness's verb constituent.

## Out of scope

- Predicate and event helper terms (*historic*, *outlaw*, *commit a crime*):
  a different loader exclusion (`BUILTIN_FAMILIES`), its own ticket.
- Terms with no owning definition (*modified*, *party*, *multikicker*).

## Landing record

Six items as specified in `docs/tickets/README.md`, with a per-term table and
a follow-up list.
