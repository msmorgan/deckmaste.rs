---
needs: []
---
# `deckmaste_syntax` names no grammar feature outside the lexicon mapping table

## Owner's ruling (round 7, item 2)

"Line (a): sanction the lexicon feature-bundle mapping table as the single
declared lexicon-to-syntax interface; ticket removing the remaining hardcoded
grammar feature names (AdjectivalPassive, DeterminativeDistribution, etc.) from
deckmaste_syntax so they are driven from english.ron declarations instead of
hardcoded strings."

This builds on round 6, ruling 1 (line (a)). `deckmaste_syntax` never hardcodes a grammar
feature or value name. Purely structural `Summary` bits (`fronted`,
`post_adjunct`, …) are allowed.

## Goal

Remove hardcoded grammar names and engine flags from `deckmaste_syntax`. The
fix adds no new `Summary`, `State` or `ProductionDecl` field and no new
`closure.rs` pass: each site is driven by what `english.ron` already
declares, or by a declaration naming existing features and values.

## The sanctioned interface

The lexicon feature-bundle mapping table is the one place where the engine
projects lexical facts onto grammar features by name. That covers:

- `PROJECTED` in `crates/deckmaste_syntax/src/compile.rs` (about line 40);
- `leaf_facts` and `projected_values` in `crates/deckmaste_syntax/src/step.rs`
  (about lines 1365–1490), including the `WordForm` → `VerbForm` and numeral
  `Number`/`Sign` projections;
- the lexeme `Countability` lookup in `compile.rs` (about line 413).

Document it as the sanctioned interface in `crates/deckmaste_syntax/README.md`.
It stays.

## Sites to remove

Re-grepped on the round-7 base. Line numbers drift.

| File:line | Names | Use |
|---|---|---|
| `compile.rs:1555–1557` | `AdjectivalPassive` / `Lexical` | a participial premodifier excludes a lexical Adjectival Passive |
| `compile.rs:1559` | `Inflection` (value from `WordForm`) | the participle premodifier's form. It may belong in the mapping table; decide |
| `compile.rs:1811, 1825` | `ComplementKind` / `Intransitive` | a bare non-verbal Valence phrase |
| `compile.rs:1830, 1872, 1925` | `Voice` / `Passive` | passive Valence derivations |
| `compile.rs:1877–1878, 1949–1950` | `VerbForm` / `PastParticiple` | the passive participle |
| `compile.rs:1952–1953` | `VerbKind` / `Auxiliary` | the passive's auxiliary |
| `compile.rs:2000, 2063` | `ComplementKind` / `NP`, `Amount` | a stranded preposition's phrase |
| `compile.rs:2118` | `AdjectivalPassive` / `Lexical`, `VerbKind` / `Auxiliary` | exclusions on the adjectival passive |
| `compile.rs:2127, 2151` | `AdjectivePosition` / `Predicative` | the adjectival passive |
| `compile.rs:2133, 2157` | `Voice` / `Passive` | the adjectival passive |
| `compile.rs:2412–2417` | `Case` / `Nominative` | an oblique's Case |
| `reading.rs:638–641` | Category `PrepositionPhrase`, `DeterminativeDistribution` / `Distributive` | the Ranking Key's distributive-adjunct class |

Out of scope: `gap_named` (the engine's own Gap kinds), `FORBIDDEN_SUFFIXES`
(a policing check on feature names), and Production display names.

## Change

- Drive each site from a declaration in `english.ron` instead of a string
  literal. That can be a parameter on the schema that uses it (as `Valence`
  already declares `passive: Some("By")`), or one declared role naming an
  existing feature and value.
- Add no new feature or value. A declaration names features and values that
  already exist.
- A grammar without the declaration gets no such derivation, rather than a
  silent default.

## Done when

- Outside the mapping table and its tests, `deckmaste_syntax/src` has no
  string literal naming a grammar feature, value or Category.
- The census, Reading Normal Forms and judgment set are identical: this is a
  refactor.
- The README records the mapping table as the declared interface.

Standard constraints apply.

Census delta (measured on `zqoxrvwo` → this landing, `qxxtsxny`): faces read 28,800 → 28,800, total Readings 237,320 → 237,320, Surplus 101,963 → 101,963; lost faces: none. The census report is identical unit for unit (all 92,934 units: verdict, derivations, segments, Surplus, cheapest tier).
Judgment set: holds (every enforced judgment passes); `english-v4 parse --limit 8` over all 3,268 judgment entries prints byte-identical best Readings, trees and Ranking Keys before and after.
