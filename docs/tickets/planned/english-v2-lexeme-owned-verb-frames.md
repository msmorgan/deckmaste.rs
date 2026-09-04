---
needs: [english-v2-attachment-class-declared, xtask-legacy-pins-to-provenance, english-v2-clause-level-duration, english-v2-copular-complement-sum, english-v2-form-template-defects, english-v2-granted-ability-coordination, english-v2-locative-coordination-arms, english-v2-coordination-member-merge, english-v2-remaining-prepositions, english-v2-of-complement-filter-removal, english-v2-possessive-nominal-form-collapse, english-v2-locative-licence-set]
---
# Verb frames become lexeme-owned data

**R12 — Group R, capstone. DESIGN brief first: this ticket is not a direct
claim.** It needs a design brief before implementation, like
`english-v2-underspecified-adjunct-attachment`. It sits last because every
earlier Group R landing removes tails from the inventory it generalizes, so the
design should be written against the smaller set.

Authority: `docs/contexts/oracle-english/CONTEXT.md` — "**Verb Frame**: An
ordered lexical schema describing the complements and fixed markers a verb
licenses" — and the rewrite ADR's "Amendment: Verb Frame and execution-context
vocabulary (2026-09-04)": "A lexeme owns a `VerbFrameSet` containing `VerbFrame`
schemas."

Defect. The grammar owns a closed, hand-maintained list of complement sequences
and a lexeme picks one from it. `crates/deckmaste_english_v2/src/constructions.rs`
declares roughly thirty-three `codec …Verb { generate declaration_verb { tail =
[...] } }` entries whose names record the corpus rather than the grammar —
`ObjectEqualityToVerb` and `ObjectToEqualityVerb` are two codecs for two
orderings of one frame; `EnterWithCountersVerb` is a mechanic promoted to a
grammar primitive; `OrderedVerb` bakes `"in" ObjectOrder "order"` into a tail;
`LookAtVerb`, `ProVerbHead`, `HaveKeywordAbilityVerb` and `GetPowerToughnessVerb`
are named for single verbs. `core_verbs.ron` shows the same shape from the other
side: `Deal` carries six hand-written `Predicate([...])` tails, and `Turn`'s only
frame is `Predicate([Literal("face"), Literal("up")])` — no object role, no
`face down`, beside a one-member `vocab FaceOrientation { FaceUp = "face up" }`.

The declaration side is already open — a stub may write
`frame_set: Custom(frames: [[Literal("with"), Lex("Preposition","For"), Amount,
ObjectNounPhrase, PredicativeComplement]])` — but a declared frame only realizes
if a codec's planned tail matches it atom for atom
(`VerbFrameKey::matches_frame_set`,
`crates/deckmaste_construction_core/src/emit/runtime.rs` and `semantic.rs`). So
those ~33 tails **are** the realizable frame space: a declaration whose
complement sequence no codec spells cannot parse, however it is written. That is
`docs/memory/scratch/plan09-postmortem/overfit.md` F1 one layer up — F1's fix
(core verbs moved out of `lexeme VerbLexeme` into `core_verbs.ron` data) landed;
the tails stayed a grammar-owned list of what the corpus has printed.

Shape to design (open dimensions the brief must pin). One `declaration_verb`
codec whose tail comes from the `VerbFrameSet` row, so a frame is data and the
named codecs collapse to rows. Open: how the generated AST and rule family are
named when the tail is not statically known; whether the frame atom vocabulary
stays sealed (it should — the atoms are categories, not words) and what it
contains; how `VerbFrameKey` degrades when there is one codec; whether
`core_verbs.ron`'s `Predicate([...])` and the stub `Custom(frames: [...])`
surface unify. Touches `deckmaste_construction_core`.

Fences. A per-mechanic or per-verb codec added on the way. A `Literal` in a tail
that spells a word a vocabulary member owns — the closed-class single-owner
amendment applies to tails. Any census used to decide which tails exist. A
`checked by` naming a verb.

Glossary: Verb Frame, Verb Frame Set, Lexical Verb Phrase, Complement, Measure
Complement, Verb Frame Key. Record any gap.

Baseline, measured on change `oulzkkoqmvuv` (388 constructions, 17,052 / 32,641
covered) — re-measure at claim. Standard constraints apply; `cargo test
--workspace` (touches `deckmaste_construction_core/src/emit/`).
