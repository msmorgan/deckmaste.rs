---
needs: []
---
# Give predicate and event helper terms a declaration path: a historic spell / other outlaws you control / commits a crime

Priority: high. A loader and schema ticket, then a small batch of terms. No
`declarations.rs` edit.

## Why

`english-v3-vocabulary-batch-1` (rulings V3, V7, V8) found game terms that
pass the vocabulary-only test but cannot be declared: by V3 a term owned by a
`plugins_v2` definition is declared through that definition's `grammar:`
field, and the lexical loader reads only the eight `BUILTIN_FAMILIES`
(`crates/deckmaste_construction_core/src/macro_def.rs`, `read_builtin_v2`).
`plugins_v2/builtin/macros/predicates/` (115 files) and `events/` (53) are
not read, and their definitions are plain helper macros
(`(name:, kinds: [Predicate], params:, body:)`) with no `grammar:` field.
`english-v3-keyword-verb-export` (landed, `nvotwtqp`) solved the sibling case
for keyword families with a `verb:` field on `FixedKeyword`, and left these
out of scope.

Attestation (`cargo xtask english-v3 grep -i`): `\bhistoric\b` **56** faces
(owner `predicates/historic.ron`), `\boutlaws?\b` **20**
(`predicates/outlaw.ron`), `\bcommit(s|ted)? a crime\b` **27**
(`events/commitsCrime.ron`); 103 faces together. Blocker report at trunk
`vwlkmvns`: *historic* is the sole blocker on 40 faces, *outlaw(s)* on 16, and
24 faces have a single failing line whose only unknown words are *commit* /
*crime*.

Precedent to regularise (V8): *unblocked* is declared in core
(`vocab:Adjective/Unblocked`, `lexicon/core.ron`) and *colorless*,
*monocolored*, *multicolored* in the `ColorWord` class
(`lexicon/vocabulary.rs`), although `predicates/unblocked.ron`,
`colorless.ron`, `monocolored.ron` and `multicolored.ron` own them.

## Goal

1. A predicate or event helper definition can carry a `grammar:` declaration,
   and the lexical loader exports it as a lexeme owned by that definition,
   exactly as `FixedKeyword` / `FixedTerm` / `Adjective` / `Noun` rows of the
   eight families are today.
2. Declare *historic*, *outlaw* and *commit a crime* through their owning
   definitions, each term only if it passes the vocabulary-only test.

## Analysis

No CGEL citation applies: this is a declaration-path question. Everything
below is a project ruling.

Existing pieces: the `Grammar` enum (`macro_def.rs`; rows `Noun`, `Adjective`,
`FixedTerm`, `FixedClause`, `FixedKeyword` with the new `verb:` export),
`DeclarationKind`, `validate_metadata`, `read_builtin_v2`, and its consumers
(lexical `legacy/plugins.rs::load`; `crates/xtask/src/facts.rs` and
`facts/lean.rs`; the `builtin_v2_*` tests of `deckmaste_construction_core`).

Orchestrator rulings 2026-10-10:

- PD1. The path is a `grammar:` field on the owning definition, read by the
  same normalised reader. A predicate or event file without `grammar:` yields
  no lexical declaration and is not an error: the 160-odd undeclared helpers
  stay untouched.
- PD2. Reuse existing `Grammar` rows: `Adjective` for *historic*, `Noun` for
  *outlaw*, the verb-declaration shape `FixedKeyword.verb` already uses for
  *commit*, and `Noun` for *crime*. No new row unless a term cannot be
  expressed; no new frame kind, Role, feature or Construction.
- PD3. One definition may export more than one lexeme (*commit* and *crime*
  from `commitsCrime`). They are ordinary lexemes: no selectional tie between
  them, and no check whose only job is to block *commit* + another Object
  (pruning rule). Attested frames and forms only (`cargo xtask english-v3
  grep`).
- PD4. Other consumers of `read_builtin_v2` must not change behaviour: either
  the lexical loader reads the two extra families through its own entry
  point, or the declarations carry a kind those consumers ignore.
  `cargo xtask facts check` stays green with no fact added or removed.
- PD5. Plugin bodies are untouched; only `grammar:` fields are added. As in
  KV5 of `english-v3-keyword-verb-export`, `cargo xtask lean check` is not
  needed unless a non-`grammar:` field changes (it must not).
- PD6. Vocabulary-only test per term (V2 of batch 1): substitute a declared
  word of the same class; if the unit reads, declare; if not, list the term
  with the missing piece and move on. *outlaw* is used as a head noun
  (*outlaws you control*), as a premodifier (*an outlaw spell*) and negated
  (*non-outlaw creature*): each use is licensed by the declared noun-class
  properties the ADR already provides (Premodifier eligibility, negative
  formation when attested), never by a word test. A use those properties
  cannot express is listed, not built.
- PD7. Migrating *unblocked*, *colorless*, *monocolored*, *multicolored* to
  their predicate definitions is optional, and only as a pure move: identical
  lexeme properties, every face's Readings and digest unchanged apart from
  the lexeme owner. `ColorWord` membership is a declared class the grammar
  reads; if the move cannot keep it, leave all four and say so. Retire the
  core declaration of whatever moves; no double ownership.
- PD8. Newly covered faces are gains. Zero Reading-count increases on
  previously covered faces; any is reported with its shape before landing.

## Witnesses

Each is the face's only failing line at `vwlkmvns`. Probed 2026-10-10 with the
`zrrnvqto` binary and data (`--category sentence --readings 3 --as-card`):
witness 0 with a vocabulary gap; substitution control as shown. Re-probe on
the claim parent first.

- Artificer's Assistant: "Whenever you cast a historic spell, scry 1."
  (control *legendary* 2)
- Jhoira's Familiar: "Historic spells you cast cost {1} less to cast."
  (control *Colorless* 3)
- Hellspur Posse Boss: "Other outlaws you control have haste." (control
  *Pirates* 2)
- Double Down: "Whenever you cast an outlaw spell, copy that spell." (control
  *an Aura spell* 2)
- Patrolling Peacemaker: "Whenever an opponent commits a crime, proliferate."
  (control *casts a spell* 1)
- Intimidation Campaign: "Whenever you commit a crime, you may return this
  enchantment to its owner's hand." (control *cast a spell* 2)

Also 0 today: "Destroy target non-outlaw creature." (Shoot the Sheriff;
control *non-Pirate* 1); "This creature can't be blocked if you've committed
a crime this turn." (Nimble Brigand; control *cast a spell* 3). Controls that
must not change: "Whenever you cast a multicolored spell, scry 1." 2; NP "an
unblocked attacking creature" 1; NP "a monocolored creature" 1.

## Method

Standard constraints apply. Deltas:

1. Bounded census: every supported face matching (case-insensitive)
   `\b(historic|outlaws?|crimes?|unblocked|monocolored|multicolored)\b` plus
   300 random covered controls (`cargo xtask english-v3 diff --manifest …
   --random 300 --seed 20261008`; the random set carries *colorless*, 522
   faces being too many to select), `--workers 6 --samples-per-face 0`;
   compare with `cargo xtask english-v3 diff` against the claim parent. Never
   `--all`.
2. Zero lost faces; PD8 for increases; PD7 moves show as digest-only changes
   and are listed as such.
3. Full `cargo test -p` and `cargo clippy -p … --tests -- -D warnings` for
   `deckmaste_construction_core`, `deckmaste_lexical_source`,
   `deckmaste_english_v3` and `xtask` (it consumes `read_builtin_v2`), once,
   at the end; `cargo xtask facts check`. No grammar edit, so only the test
   binary rebuilds (about 7 minutes).
4. Tests assert the mechanism and the roles: a predicate / event definition
   with `grammar:` exports the lexeme with that owner; one without exports
   nothing; each witness's term has the expected Category and function.
   Never an incidental count or a Debug-printed tree.

## Out of scope

- Terms with no owning definition (*modified* 47 faces, *party*,
  *multikicker*): they need a definition first.
- "that's not historic" (Desynchronization): its *colorless* control reads 0;
  a grammar gap, not vocabulary.
- Any other helper family (`conditions/`, `relations/`, …) until a term
  there passes PD6.

## Landing record
