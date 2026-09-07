---
needs: []
---
# Analyze declared vocabulary independently of grammar

Implement the lexical interface in
[the accepted decision](../../decisions/english-lexical-analysis.md#lexical-analysis).
Reuse catalog identities, provider provenance, known forms/irregular data and
immutable indexes. Core and plugin vocabulary feed the same analysis interface;
their source formats need not be unified. Keep morphological generation shared
between analysis and declaration-driven rendering. No grammar/compiler dependency
may be required merely to inspect a word's analyses.

Pin token boundaries, separators, capitalization, bound/overlapping multiword
analyses, token occurrences and derived positions through supported corpus
examples. Preserve raw versus normalized input provenance. Analyses retain
lexeme/category/form identity and correlated feature bundles, with explicit
feature applicability. Lexemes supply grammatical properties and frames;
the tagger does not select a contextual reading.

Implement one default per applicable inflection and explicit replacing irregular
overrides. State whether each default is suffix append or a deterministic
orthographic algorithm, test its actual cases, and list valid explicit
alternatives. No unknown-word/POS/derivation speculation or inferred paradigm
classes. Preserve noun/verb homographs and distinct forms with the same spelling.

Run lexical analysis over the entire supported corpus independently of parsing.
Produce the unique-word/remainder inventory requested by the user, accounting
separately for symbols, keywords and open catalogs. Unknown forms are explicit
gaps with source occurrences; this ticket establishes accounting and the shared
interface, not exhaustive vocabulary completion. The lexical-source ticket
owns the remaining inventory.

Acceptance: direct analysis and generation fixtures cover homographs, default
and irregular inflections, replacement rather than additive override behavior,
explicit variants, multiword overlap, sentence-initial forms and unknown-word
exclusions. Preserve exact surfaces through the declared lexical realization
interface; report the corpus remainder and runtime. No full-card coverage gain
is claimed before chart integration. Standard constraints apply.
