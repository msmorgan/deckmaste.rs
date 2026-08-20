---
needs: [english-v2-open-grammar-environment]
---
**Generate the lexeme tier: `morphology` and spelled `lexeme … using`
declarations own the verb and closed-noun surfaces, the handwritten
inflection pair is deleted, and the irregular-override inventory becomes a
counted category.** Plan 03 shipped three of four terminal tiers generated;
the `lexeme` tier was deferred by a recorded mid-plan amendment and remains
handwritten, outside the single-authority audit, and uncounted. This
ticket closes it before the buildout grows more verbs. Authority:
`docs/decisions/english-v2-rewrite.md` §Terminals (morphology is a named
closed recipe over one feature axis with declared irregular overrides; a
rising irregular count is the signal to extend the recipe) and
`docs/decisions/builtin-v2-macro-spelling-and-grammar.md`. Every item is
pinned — if something cannot be completed as written, STOP and report.

1. **Declarations.** Implement the `morphology` declaration (named closed
   recipes over one feature axis; recipe names are compiler-known tokens,
   never Rust paths) and spelled `lexeme` declarations naming their
   morphology: `english_verb` (lemma for Bare; lemma + `s` for
   ThirdPersonSingular) and `english_noun` (lemma for Singular; lemma +
   `s` for Plural), with explicit whole-surface overrides
   (`Be = "be" { Bare = "are", ThirdPersonSingular = "is" }`). Variant
   identifiers are NEVER converted into lemmas — every lemma and every
   irregular surface is spelled in the declaration. Overrides replace the
   recipe-derived surface exactly; empty lemmas or surfaces, incomplete
   feature coverage, and duplicate entries are validation errors.
2. **Recipes are strictly regular (ruling, 2026-08-20).** Builtin
   morphology recipes carry ONLY exceptionless mechanical
   transformations: append-`s` today, append-`ed` when past forms arrive.
   A transformation with even one attested exception is lexical, not
   regular — the corpus itself proves `-f` does not imply `-ves`
   (Lhurgoyfs, Serfs beside Dwarves, Wolves) and name-like nouns resist
   `-ies` — so the four `-ies` members (Allies, Armies, Mercenaries,
   sorceries) and four `-ves` members (Dwarves, Elves, Werewolves,
   Wolves) REMAIN declared per-word overrides. The Plan 03 spec's
   pause-trigger ("three overrides sharing a transformation pause for a
   recipe extension") is superseded: a declared-override list growing
   with the lexicon is normal, not a recipe-extension signal. The ~300
   open-declaration nouns with no declared plural remain unavailable
   until attested — record that policy beside the recipes.
3. **Deletion.** Generated scanner and renderer surfaces replace the
   handwritten pair: `features::inflect` and the handwritten verb-scanning
   path are DELETED, and the generated renderer stops calling a
   handwritten symbol by hardcoded name. Also generate the
   `agreement_for_<vocab>` definitions the emitters currently reference
   but never define. No dual authority survives this ticket.
4. **Audit and counts.** The single-authority audit inventory gains the
   generated verb-surface items, plus a test proving every generated
   terminal item appears in the audit inventory — the target list is
   verified against the emission plan, not hand-trusted. The counted
   report adds `morphology_irregulars` as a source-ordered inventory (one
   entry per member with at least one override, its complete ordered
   override table as evidence); the counted-list schema becomes 2 and the
   xtask expectations pin the new category exactly.
5. **Provenance.** Closed-lexeme stable owner IDs gain their pinned third
   segment (`lexeme:<declaration>/<member>/<feature-value>`), matching the
   open-declaration form already shipped.

Acceptance: all suites green; the corpus gates unchanged (32,285 total /
48 covered / 0 ties / 0 internal failures — this ticket must not change
acceptance; a delta means STOP); the handwritten pair gone with the audit
proving single authority; the counted report shows `morphology irregulars`
containing `Be` and every declared noun override; both round-trip
laws still byte-exact on the covered set; `cargo clippy --workspace
--all-targets -- -D warnings` clean. Standard constraints apply.
