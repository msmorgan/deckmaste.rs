---
needs: []
---
# Share characteristic edits and declare keyword argument schemas

Replace specialized characteristic-edit and keyword-parameter combinations
with shared typed payloads. Apply the
[campaign contract](lean-core-bindings-and-composition.md#campaign-contract).
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
arguments. A declaration schema specifies the admissible combinations and
their order. Preserve the role and scope difference between quality arguments,
subject arguments, amounts, costs, and deck conditions. The checker reads
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
