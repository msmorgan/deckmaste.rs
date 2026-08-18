---
needs: [builtin-v2-declaration-schema]
---
Port the builtin card-type registry rows to committed `builtin_v2`
declarations and give each declaration its semantic `spelling` plus any
English `grammar` facts required by the ratified
[builtin-v2 spelling/grammar ADR](../../decisions/builtin-v2-macro-spelling-and-grammar.md).
The declaration remains the single source for type identity and conferrals.

Re-author the current builtin type rows in the v2 schema rather than adapting
or loading the v1 files. Each row carries its open `TypeDef` identity,
permanent/nonpermanent datum, existing conferrals, lowercase running-text
spelling, and attested count-noun morphology. Omission selects the schema's
dumb `singular + "s"` plural; an attested exception replaces that whole
surface, and an unattested plural uses the explicit unavailable state. Grammar
owns noun realization; the type registry owns semantics. Follow style guide
§7, "Types, subtypes, and supertypes as nouns and modifiers."

Do not claim a `card-types.txt` bijection. That CR catalog includes types the
current semantic type-line axis does not represent, while open `TypeDef`
references and the closed `Type` type-line field are not yet the same domain.
The ticket must enumerate that modeled boundary in its tests and must not make
an unsupported type appear usable on a card's type line merely because its
English noun parses. Conversely, a plugin-declared type reference must not
require a hardcoded parser entry.

These are committed source records, never generated stubs. Acceptance proves
the permanent flag and conferrals survive the migration for representative
permanent and nonpermanent types, covers singular/plural and attributive uses,
loads every record through the v2 declaration reader, and leaves the closed
type-line limitation explicit. Standard constraints apply.
