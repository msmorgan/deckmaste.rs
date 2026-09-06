---
needs: []
---
# Share characteristic edits and declare keyword argument schemas

Replace specialized characteristic-edit and keyword-parameter combinations
with shared typed payloads. Apply the
[campaign contract](../done/lean-core-bindings-and-composition.md#campaign-contract).
These decisions were agreed on 2026-09-06; this ticket can begin independently
of the binding work.

## Characteristic edits

Use an explicit list of typed edits with set/add/remove operations. Absence of
an edit means unchanged; setting an empty value is a different operation.
Share these edits between ordinary quality changes and copy exceptions.

Retire the special `CopyExcept.types`, `name`, `pt`, `color`, and
`nonlegendary` cases through that vocabulary, and replace the overloaded
`chars`/`typesAdded` combination with explicit edits. Preserve compound type
and subtype writes, removals such as legendary, and the difference between
replacing and adding a value. Keep copy-specific consequences in the enclosing
copy context: sharing an edit payload does not make a copy exception an
unrelated later continuous effect. Ability and arrival additions retain their
own meaning rather than being forced into a characteristic slot.

Absorb `StaticSpec.abilityLoss` and `allAbilityLoss` into the ability-removal
edit. Its typed selector distinguishes specified abilities, an ability family,
and all abilities except a selection. Those selectors preserve the difference
between losing flying, losing activated abilities, and losing all abilities
except a specified class. Reuse the shared edits wherever ordinary quality
changes state the same operation; do not keep a second parallel edit family.

## Keyword arguments

Replace `KeywordParam.qualityCost` and `numberCost` with ordered typed
arguments. Registry columns specify the admissible combinations and their
order, and generate the checker's schema data. Preserve the role and scope
difference between quality arguments, subject arguments, amounts, costs, and
deck conditions. The checker reads
declared schema properties; it must not branch on keyword names.

Matching field shapes do not make `Predicate.qualityNoun` and `ofYourChoice`
aliases. The former selects quality values; the latter selects objects with a
chosen quality. Retain that semantic distinction and their domain checks.

## Blast radius

The edit list rewrites the copy bench and every `qualityChange` site, and the
argument schema re-spells the keyword bench, the largest family. Iterate on a
card subset and run the full build once at the end. The landing record's
restored/re-spelled/removed counts are where a silently weakened pin would
show; a negative twin that loses its exact refusal list is a regression, not
a re-spelling.

## Completion evidence and boundaries

Re-spell the copy and keyword bench through the shared edit and argument
forms. Exercise unchanged versus set-empty, set versus add, removal of a
supertype, copy-context consequences, each ability-selector form, and rejected
argument combinations with correct scopes. Run `lean/scripts/build` and the
facts-generation consistency check when declaration schemas change.

The older [copy grammar tail](idris-copy-grammar-tail.md) concerns frozen Idris
and Rust emission work; it is not this ticket's implementation target or a
dependency. Any registry changes here serve the Lean checker and receive the
standard checks for their affected consumers. Standard constraints apply.


## Landing record

### PROVE

Measured on change `ptrmvywl`, with the unchanged English coverage lock at
20,254 covered identities. The final `lean/scripts/build` passes the complete
syntax, checker, card bench, and proof gate with warnings fatal. Source audit
retains all 2,377 existing card-module declarations and 1,612 existing
proof-module declarations. Every existing theorem's asserted result is
unchanged; no card identity or proof is removed.

Assurance: restored 0, re-spelled 36 (33 Lean theorem statements, two authoring
rejection examples, and one Rust declaration-generation test), newly ignored
0, added 33 (21 characteristic-edit theorems, 11 keyword-argument theorems,
and one Rust registry-mutation test), removed 0. Separately, two card
definitions and one raw authoring helper change spelling. Existing authoring
rejections retain their exact messages. The Rust suite retains its one
pre-existing ignored live-corpus census test, whose stated blocker is that
it runs on demand.

`cargo xtask gate --changed` derives `cargo test -p xtask`. That closure passes:
478 unit tests, 13 command tests, and two other integration tests; zero
failures, one pre-existing ignore. `cargo xtask facts check` confirms both
Lean and frozen Idris generated facts are current. The checker reads generated
schema properties rather than keyword names. No parser source, declaration,
coverage lock, or word-naming licensing guard changes. Parser roundtrip,
lexical ownership, traversal identity, tie/internal-failure counts and
licensing inventories were not remeasured by this Lean/facts-generator gate.

### DISCLOSE

Core constructor counts: `StaticSpec` 30 to 28, `CopyExcept` 9 to 4,
`KeywordParam` 7 to 5. Separately, the four-constructor core `QualityPayload`
is retired; shared helpers add nine `CharacteristicEdit` alternatives,
three `AbilitySelection` alternatives, four `CharacteristicStat` alternatives,
and the `TypeLineChanges` record. A four-alternative `QualityPayload` remains
only as registered authoring input; no checked core field stores it. These
are family folds, not a claim that the total count across all helper enums
fell. Keyword facts now carry ordered schema lists with quality-domain data.

The new proofs exercise set-empty versus absence, adding versus setting,
compound type/subtype writes, supertype removal, all three removal selectors,
copy-specific additions, block-local numeric scope, invalid edit operations,
invalid mana symbols, argument order, and argument scope boundaries.

Deviations and additions:

- The old copy bundle's minimum-two-fields rule and blanket prohibition on
  a name inside that bundle are packing restrictions, retired with `chars`.
  A single edit and an ability-only copy input are now valid. There were no
  negative pins for those packing restrictions; all existing copy pins remain.
- A bundled chosen quality now uses the same host evidence and refusal as a
  standalone chosen-quality edit. Existing subject types can host it alongside
  types written in its edit block. A malformed bundled quality formerly added
  `qualityRead` before `becomesOk`; the shared form reports `becomesOk` with
  its normal predicate errors. No named pin asserts the former extra error.
- Written abilities in a loss input expand to specified ability removal,
  rather than being silently added. Mana-cost edits validate their symbols;
  copy edits reject invalid operations and empty add/remove values.
- Registry codecs still restrict the supported declaration combinations.
  This change makes the Lean representation ordered without expanding those
  codecs or modifying frozen Idris output.
- The first full gate exposed a misplaced proof-suite import, corrected without
  changing an assertion. A final copy-helper check also removed an empty edit
  block from ability-only inputs; the full gate was rerun after that change.

The game-model glossary now defines Characteristic Edit, Copy Exception, and
Ability Selection; `lean/CONTRACTS.md` records scope and schema contracts.
Runtime copy consequences, including characteristic-defining abilities, remain
consumer semantics: the Lean checker retains the enclosing copy context but
is not an evaluator. No unresolved STOP or glossary gap remains in this ticket.

### REPORT

English coverage remains 20,254 on `ptrmvywl`. The selection census,
construction count, licensing total, homograph/overlap inventories, and parser
performance were not remeasured because their sources did not change. No
coverage gain or performance claim is made. Citation validation reports zero
noncompliant strings and zero stale citations; both new citation sites were
read against their rule texts. A Kata refresh before final review was a no-op.
