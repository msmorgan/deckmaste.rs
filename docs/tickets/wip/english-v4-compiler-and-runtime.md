---
needs: []
---
# Build `deckmaste_syntax` and `deckmaste_english_v4` and prove the unified frame engine on a seed lexicon

First deliverable of [`docs/decisions/english-v4.md`](../../decisions/english-v4.md)
(§1, §4.1) and [`docs/decisions/english-lexical-analysis.md`](../../decisions/english-lexical-analysis.md)
(§§1, 4): create `crates/deckmaste_syntax` (the unversioned, data-driven syntax
engine crate) and `crates/deckmaste_english_v4` (which loads the English grammar
data file of **Schemas** and **Productions**, binds lexical catalogs into
`deckmaste_syntax`, and owns attachment-ranking heuristics and Oracle unit
splitting), and **prove the unified frame engine end-to-end on a ~15-word seed
lexicon** before authoring the full catalogs or strata.

## Syntax Engine & Runtime Capabilities to Prove

1. **Data-Driven Grammar Loader (`Schema` and `Production`):**
   - Replace proc-macro Rust codegen with a runtime grammar data file loaded and
     validated by `deckmaste_syntax`.
   - Reserve **Construction** for CxG form–meaning pairings at the spelling
     boundary; in `deckmaste_syntax` and the English grammar data file, use
     **Schema** (for high-level syntactic patterns/templates that expand into
     productions) and **Production** (for individual grammar rules).
2. **Unified Subcategorization Frame Engine:**
   - Support `frame_family` and `selected_frame` uniformly across `Verb`,
     `Preposition`, `Noun`, `Adjective`, and `Determinative`.
   - Preposition declarations carry their complement frames (`NP`,
     `KeywordPhrase`, `QuotedAbility`, `Clause`, etc.) with unselected syntactic
     host-category distribution declared **per `(preposition, complement_frame)`
     entry** (`Nominal` postmodifier, `Predicate`/`Clause` adjunct, both, or
     `ComplementOnly`).
   - Same-head complement priority: a PP filling a head's selected complement
     frame does not also attach as an unselected adjunct on that same head
     instance, and carries lower `Production Cost` than competing postmodifier
     attachments so `deckmaste_english_v4`'s attachment-choosing heuristics rank
     the complement reading first.
3. **Engine-Derived Valence Productions from Active Verb Frames:**
   - **Active** complementation (`Intransitive`, `Monotransitive(NP)`,
     `Ditransitive(NP, NP)`, `TransitiveWithPP(NP, prep)`, `Prepositional(prep)`,
     `Copular(Pred)`, `Catenative(ToInf | BareInf | Gerund)`).
   - **Passive** complementation: promote the object slot to subject (in finite
     `be + V-en` and reduced postnominal `V-en` clauses), retain remaining
     complement slots (`each card put into your hand`), and license optional
     `by <NP>` internalised complements (`[CR#701.1]` verbs included via frames).
   - **Slashed / Gapped Extraction Across All Verb Forms:** derive object-gap and
     stranded-preposition-gap productions uniformly across finite, plain/bare,
     progressive (`be + V-ing`), perfect (`have + V-en`), and passive
     (`be + V-en`) forms (`"the card you put / are putting / have put _ into your hand"`).
   - **Right-Node Raising (Shared Complements):** derive shared right-edge
     complement satisfaction across coordinated heads (`enters or leaves [the battlefield]`,
     `put into your hand or exile [_]`).
4. **Reading Normal Form & Packed-Forest Contract:**
   - Preserve both round-trip laws (`v is an admitted Reading of s => render(v) = s`
     and `v is independently constructed and grammatical => v is a Reading of parse(render(v))`),
     including lexical identities, correlated alternatives, and deferred
     validation on the packed forest without blindly enumerating all $C_n$
     attachment trees.
   - Enforce the zero-policing-flag rule (`...Use`, `...Licence`, `...Present`
     forbidden).

## Seed Prototype Proof

Prove active, passive, slash-gap (across finite, progressive, perfect, and
passive forms), and Right-Node Raising on a ~15-word seed lexicon (covering
intransitive `attack`, monotransitive `destroy`/`exile`/`control`, `NP + PP`
`put ... into/onto` and `search ... for`, copular `be`/`get`, and prepositions
`in`, `into`, `on`, `onto`, `for`, `with`, `from`, `during`), reporting both
round-trip verification and chart-item / forest-node counts.

Standard constraints apply.
