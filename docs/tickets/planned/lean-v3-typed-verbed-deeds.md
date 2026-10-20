---
needs: [lean-v3-authoring-impl-14-ast-and-authoring-cleanup]
---
# Name the deed a participle reads without a string

"The exiled card", "the creature sacrificed this way" and their kin are
`theVerbed (.action "exile") …`: the keyword action's label written as a
string at the use site. There are 57 such sites (27 in `Cards/`, 11 in
`Definitions/`, 19 in `Proofs/`), over nine actions: destroy, discard, exile,
mill, reveal, sacrifice, search, tap, untap. A misspelt label is refused only
indirectly: an enacted deed with an undeclared label is refused (`knownAct`),
but a participle whose label no deed stamps reaches nothing and fails as an
anaphor, not as a misspelling.

The open question is naming. A keyword action's Definition already takes the
bare name (`exile`, `sacrifice`), so a typed `Deed` for each action cannot
reuse it.

Proposal: generate one `Deed` value per keyword action, named in the `Deed`
namespace as the collision rule for registry declarations already does
(`Deed.exile`, `Deed.sacrifice`), from the same table that feeds
`keywordActionLabels` (`cargo xtask facts generate`). Rewrite the participle
sites to use those values, so a misspelling fails to elaborate. The
alternative is a `deedOf (action : Action) : Deed` template that reads the
label off a Definition's own scope header. It needs no new names, but it
cannot be total over every action and needs a fallback.
