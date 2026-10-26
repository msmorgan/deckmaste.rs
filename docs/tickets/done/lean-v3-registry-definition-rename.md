---
needs: []
---
# Rename Lean's `inductive Definition` to `RegistryDefinition`

Standard constraints apply. Decided 2026-10-25: rules rows stay as DSL items (`Rules.lean`,
`Definitions/CardTypeRules.lean`, `SubtypeRules.lean`, `DesignationRules.lean` and the rows in
`Proofs/Rules.lean`), and keyword definitions read them (haste is `asThough` relative to the
summoning-sickness rows). So `inductive Definition` and `Check/Rules.lean` are not reshaped or
deleted, and this rename is one mechanical commit. Docs and Lean only; no Rust changes. Paths
are under `lean/Semantics/` unless they start with `lean/` or `docs/`; line numbers are as of
the tree at `pmvrnuxw` (re-find them with the greps below if the tree has moved).

## The defect

`inductive Definition` (`Rules.lean:87`) has the docstring "What one registry declaration MEANS:
the rules content the declared name stands for". That is the glossary's **Registry Definition**
(`docs/contexts/game-model/CONTEXT.md`, entry "**Registry Definition** (project term)"). Its
constructors are `counter`, `subtype`, `designation`, `cardType` and `relation`. The glossary's
**Definition** (same file, "**Definition** (project term)") is the trusted expansion
(`semantic_definition`), and its `_Avoid_` line reserves "Registry Definition" for exactly this
record. So the Lean identifier takes the name of the wrong entry. The rule is "rename
identifiers; never reword entries": rename the Lean type to `RegistryDefinition`. The glossary
does not change.

## What changes

Rename the type `Semantics.Definition` to `Semantics.RegistryDefinition`, and every qualified use
of its constructors and functions (`Definition.cardType`, `.counter`, `.subtype`, `.designation`,
`.relation`, `.check`, `.conferralContext`, `.controllingDesignation?`), in exactly the lines
listed below. Replace each whole-word token `Definition` on a listed line with
`RegistryDefinition`, except a token already preceded by `Registry ` (`Words.lean:617` has
"is its Registry Definition (`Definition.counter` …": only the second one changes). 85 tokens
change in total in Lean, on 81 lines (the docs add 13 lines, below).

Lean (`lean/Semantics/`; "T" = tokens on the line when more than one):

| File | Lines |
|---|---|
| `Rules.lean` | 87 (`inductive Definition where`), 103, 120, 142 (T=2: `def Definition.controllingDesignation? : Definition → …`), 148 (docstring: "The `Definition` above is a separate record …") |
| `Check/Rules.lean` | 34, 35 (T=2: `def Definition.conferralContext (d : Definition) : DRS`), 44 (T=2: `def Definition.check : Definition → List Refusal`) |
| `Definitions/CardTypeRules.lean` | 6, 13 (docstring: "as a `Definition`'s conferrals are": this one is the registry record, so it changes), 81, 117, 166, 174, 178, 181, 184, 187, 191, 194, 197 |
| `Definitions/CardTypes.lean` | 9 |
| `Definitions/DesignationRules.lean` | 6, 13, 37, 58, 62 |
| `Definitions/SubtypeRules.lean` | 6, 50, 74, 88 |
| `Proofs/Rules.lean` | 162, 163, 169, 171, 185, 188, 192, 195, 199, 205, 212, 216, 217, 222, 226, 231, 240, 291, 319, 350, 353, 356, 359, 362, 365, 368, 378, 390, 394, 397, 402, 405, 409, 413, 417, 466, 472, 490, 510, 524, 534, 539, 541, 546, 661 |
| `Proofs/Counters.lean` | 671 |
| `Proofs/LinkedReadback.lean` | 275 (T=2) |
| `Templates.lean` | 993 |
| `Words.lean` | 53, 617 (T=1: only the `Definition.counter` token) |

Docs:

| File | Lines and tokens |
|---|---|
| `lean/CONTRACTS.md` | 148 `Definition.cardType`, 150 `Definition.subtype`, 941 `Definition.designation`, 1148 `Definition.relation`, 1389 `Definition.counter`, 1393 `Definition.check`, 1406 `Definition.check`, 1407 `Definition.check` (the later "Registry Definitions" there is already correct), 1420 `Definition.designation` |
| `lean/README.md` | 89 `Definition.cardType` |
| `docs/decisions/semantics-v3.md` | 293: only the token in "a Registry Definition's conferrals by `Definition.check`" (that line's other `Definition`s are the trusted expansion; do not touch them). 622: "the registry holds its `Definition` record" becomes "its `RegistryDefinition` record". 645: `Definition.relation` |

Do not change any of these:

- Every other `Definition` in Lean docstrings and comments means the trusted expansion
  (`semantic_definition`): `Definitions/Actions.lean` (all the "(`Definition`, …)"
  docstrings), `Abilities.lean:1216`, `CallerScope.lean:6`, `Definitions.lean:16`,
  `Words.lean:312`, `Definitions/Keywords.lean:984`, `Authoring.lean`, `Check/Events.lean`,
  `Proofs/CheckerCore.lean`, `Proofs/DrtCards.lean`, `Proofs/Damage.lean`,
  `Proofs/AbilityQualifiers.lean`, `Proofs/Authoring.lean` and the rest of `semantics-v3.md`.
- `Rules.lean:149`'s historical "`Definition.*Term` projections" (retired; it names what was
  deleted).
- The glossary, `docs/tickets/done/*`, `docs/evidence/*` and `docs/archive/*`.
- Leading-dot constructor uses (`.counter …`, `.cardType …`, `.check`) need no change.
- The deletion-bound Rust `deckmaste_semantics_v2` (`rules::Definition`, and the retired
  `lean definitions` emitter at `crates/deckmaste_semantics_v2/src/lean_emit.rs:1087-1120`, which
  writes the old Lean spelling into strings and that nothing compiles). No Rust file changes.

How to find the lines again (the LSP is the check): `lean_references` on `Rules.lean:87` column
11 lists the 32 type references; after the rename the same query on `RegistryDefinition` lists
the same 32. The qualified uses are
`grep -rnP '(?<!Registry )\bDefinition\.(cardType|counter|subtype|designation|relation|check|conferralContext|controllingDesignation)' lean docs/decisions`.

## What must not change

- No behavior: no pin is added, rewritten or retired, and no constructor, field, row or
  refusal changes. The diff is whitespace-free token substitution only.
- No rules row changes (`builtinTables`, `Definitions/CardTypeRules.lean`, `SubtypeRules.lean`,
  `DesignationRules.lean`, the rows in `Proofs/Rules.lean`).
- No new `CR` citation is added or removed.

## Commit plan

One commit, `lean: the registry record is a RegistryDefinition`.

## Acceptance

- `./lean/scripts/build` (`--wfail`) passes with zero warnings and no `sorry`.
- `cargo xtask lean check`.
- `grep -rnP 'inductive Definition\b' lean` and the qualified-use grep above are both empty.
- `jj --no-pager diff --git --stat` names exactly the 11 Lean files and 3 doc files listed above
  and nothing else; the diff changes 81 Lean lines and 13 doc lines.
- The CR citation gates: `cargo xtask cite check --list-noncompliant` empty, `cargo xtask cite
  check` 0 stale, `jj --no-pager diff --git | cargo xtask cite audit --diff` (it reads STDIN).
  No rule is added, so `cargo xtask cite bless` is not run.
