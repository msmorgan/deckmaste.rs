---
needs: [english-v2-lexical-analysis]
---
# Rebuild the English workbench around the v3 grammar relation

Rewrite the independent `english/` Lake project to describe the grammar that
English v3 will implement. English remains an NLP model with no import from or
interaction with Semantics. Preserve useful definitions and witnesses from the
current workbench only when they express the v3 design directly; compatibility
with its old `Analysis`, selection or packaging layers is not a goal.

Model declared Lexemes, licensed Word Forms and correlated Feature Bundles as a
non-contextual Lexical Analysis relation. Model grammatical admission separately:
Categories and Productions constrain agreement, lexical frames, countability,
voice, tense, extraction, coordination, recoverability and document boundaries.
Every admitted Reading remains available; optional preference is a separate
relation and cannot remove a Reading. Equal text does not collapse distinct
lexical identities or constituent structures, while duplicate derivations of
one Reading do not manufacture linguistic ambiguity.

Give the whole intended grammar one connected top-down shape. It must cover the
families in the reviewed source map: documents, sentences and clauses; lexical
predicates and frames; nominals, determiners and adjectives; subordination;
prepositions; relatives and extraction; coordination and sharing; measures;
contextual recoverability and ellipsis; Type Lines; and the Target Verb/Targeting
Marker rivalry. Each family needs at least one inhabitant, one exclusion where
the family has a grammatical constraint, and one interaction with another
family. This is grammar design, not a census claim or a license for permissive
catch-all productions.

State analysis-to-realization and independently-constructed-value roundtrip
relations without making either true by definition. Preserve declared spelling
variants and capitalization. Model the grammatical information that a parent
must inspect and the correlations that cannot be projected independently, but
leave Earley scheduling, chart indexes and packed-node layout to Rust.

Acceptance names every old workbench declaration kept, replaced or retired;
demonstrates noun/verb and noun/determinative homographs, agreement-sensitive
forms, a jointly invalid combination of individually valid alternatives, two
unrelated Readings of one surface, and an ambiguity-free control; and leaves no
v3 rule resting on the superseded uniqueness or destructive-selection policy.
Use Lean LSP MCP, `english/scripts/build`, and the standard axiom audit. Standard
constraints apply.
