---
needs: [builtin-v2-spelling-stub-design]
---
Define the neutral v2 macro-declaration schema, its source reader, and the
normalized grammar-row contract ratified by the
[builtin-v2 spelling/grammar ADR](../../decisions/builtin-v2-macro-spelling-and-grammar.md).
This ticket owns data validation and normalization, not the parser, dependency
resolution, semantic frame matching, or English-authored cards.

Put the owned serde-readable types in `macro_ron`, separate from legacy
`MacroDef`. A declaration has an explicit kind and category-scoped name,
positional parameter types, a `spelling` string containing only literal text
and `<Param(n)>` holes, optional closed `grammar` data, and optional typed
semantic body. Do not add v2 fields to the permissive v1 reader or translate
v2 records through `template:`, `frames:`, named parameters, text offsets,
or the old `kinds:` registry. A nursery record may omit its signature/body
while retaining identity and grammar.

The grammar source model admits only the reviewed recipes: verb, noun, and
fixed term/clause/keyword surfaces. Every verb stores exactly one valence:
`Intransitive`, `Transitive`, `Numerative`, or `Custom`. Serialize
`Custom` as a finite nonempty set of ordered tail shapes whose atom enum is
exactly `Literal(String) | Amount | ObjectNounPhrase`. An individual empty
shape is legal; it represents a no-tail alternative. Reject an empty shape
set, duplicate alternatives, empty literals, unknown atoms, recursion,
repetition, precedence, callbacks, semantic guards, and arbitrary category
names.

Normalize verbs and nouns through exactly two morphology recipes:

- `english_verb(bare)` derives third-person singular as `bare + "s"`;
- `english_noun(singular)` derives plural as `singular + "s"`.

An omitted derived-form field selects the default. An explicit string replaces
the derived row instead of adding an alias, and an explicit unavailable value
suppresses the row when the author has no attestation. Reject a replacement
equal to the derived surface. Do not implement `-es`, `-ies`, compound-head
discovery, title-case morphology, or any catalog-backed guess. Pin examples in
which Destroy omits its override, Scry replaces `scrys` with `scries`, a
compound verb replaces the whole form, Sheep or Merfolk replaces the derived
plural with an invariant, and `players` is rejected as a redundant override
for `player`.

Normalization produces owned, source-attributed rows without consulting the
English parser or a canonical catalog. Each row carries declaration kind,
category-scoped textual identity, closed recipe, complete realized surfaces,
features, and verb valence/tail shapes. Preserve all distinct rows with the
same surface. Enforce identity-local coherence between `grammar` and
`spelling`: a verb frame's initial literal head must be that declaration's
own bare surface, and each fixed recipe applies its corresponding local
surface check. A declaration with grammar head `mill` and spelling
`scry <Param(0)>` is an error even if another declaration owns `scry`.
Validate every `Param(n)` against the positional vector when a typed
signature exists; never create a named-parameter environment.

Provide a narrow bootstrap reader for committed records at their final
`plugins/builtin_v2/macros/stubs/...` paths. It may accept only the builtin
root and exists so Stage 5 can consume handwritten corpus definitions before
the general plugin provider lands. It must call the ordinary v2 reader and
normalizer, produce the ordinary normalized row type, and own no alternate
manifest or schema. The later provider ticket will replace this entry point,
not migrate its source content.

Acceptance uses source-span/path-authenticated failures for wrong declaration
kind, category/name mismatch, invalid holes, missing verb valence, every
invalid `Custom` case, grammar/spelling mismatch, duplicate same-file
identity, invalid morphology states, and redundant replacements. Round-trip
representative nursery and graduated declarations through serde and prove
their normalized rows are independent of filesystem iteration order. Standard
constraints apply.
