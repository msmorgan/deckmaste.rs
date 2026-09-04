---
needs: [english-v2-require-through-optional-role]
---
**Give lexical schemas, instantiated phrases, and evaluation contexts distinct
names.** Use Oracle English [`Verb Frame`, `Lexical Verb Phrase`, `Verb Phrase`,
and `Complement`](../../contexts/oracle-english/CONTEXT.md) alongside Game Model
[`Execution Frame`](../../contexts/game-model/CONTEXT.md). A Verb Frame is the lexical schema
that licenses ordered complements and fixed markers; a Lexical Verb Phrase is
an instantiation of that schema; an Execution Frame is the engine evaluation
context. Do not use `valency` as a catch-all for all three.

Apply this mapping after the optional-role WIP lands:

- rename the lexeme-owned `VerbValence`, whose custom case contains a set of
  ordered shapes, to `VerbFrameSet`; each member shape is one `VerbFrame`;
- keep or rename normalized `VerbFrameKey` explicitly as a compiler
  compatibility key, not as the lexical schema itself;
- rename realized AST `BaseVerbFrame` values to `LexicalVerbPhrase`;
- qualify engine evaluation-context `Frame` values as `ExecutionFrame`; and
- rename the current `Numerative` role to `MeasureComplement`.

Apply the mapping through English-v2/core/data types, constructors, providers,
diagnostics, and tests. Append dated superseding amendments to the affected
verb-valence sections of the builtin-v2 grammar and English-v2 rewrite
decisions; retain their historical text. Prefer the short `VerbFrame` family
over `VerbSubcategorization` or `ComplementationPattern`.

Acceptance makes schema-versus-instance types unambiguous at their API seams
and leaves generic `Frame` only where a qualification adds no information.
