---
needs: [grammar-generic-coordination]
---
# Closed-class words in the grammar crate, typed classes, checked references

Lexical distinctions reach syntax as untyped string flags today. 72 of 184 features are in no category interface, 37 of those are read at exactly one site, and 39 are written for exactly one holder in the lexicon, for example `InfinitivalMarker { Yes }` (held by *to*), `RelativeSubordinator { Yes }` (*that*), `FocusingModifierUse { Restrictive }` (*only*), `SelectedTailLicence` (*put*). `core.ron` carries about 192 per-lexeme patches of grammar-internal flag names, and an authored value outside a feature's domain silently becomes absent (`crates/deckmaste_construction_v3_core/src/emit.rs:317–331`).

## Change

- Closed-class words (determinatives, prepositions, coordinators, subordinators, auxiliaries, pronouns, negators) are declared in Rust in the grammar crate, with their class memberships, so a reference to anything else cannot compile. `crates/deckmaste_lexical_source/lexicon/vocabulary.ron` moves there and is deleted, taking its `provenance_path` with it. `verbs.ron` and the nouns in `core.ron` stay data.
- The per-lexeme feature patches in `core.ron` become class memberships.
- The grammar refers directly only to core closed-class lexemes, by lexeme id, never by string literal; a direct reference is used when exactly one lexeme can fill the slot, a typed class when two or more can. The compiler checks every reference. Everything else (plugin terms, catalog entries, card names, open-class core words) reaches the grammar only through typed classes and frames.
- The loader and compiler reject unknown ids, class names and values; anything newly rejected is fixed as its own change, not waved through. `unsupported_frames()` (`selected_frame.rs.txt:275`, called only from tests) is called by the tool once and its result reported.
- Report, as findings, every class with one member and every feature with one holder, and what was done with each.

## Gate

Reading digests identical on every face. If provenance strings change when `vocabulary.ron` moves, identity is shown by the per-face reading digests; the lexicon inventory digest may change once and the change is reported. Three-line note: census delta (none); size delta per declaration kind (ratchet lowered); judgment-set result.
