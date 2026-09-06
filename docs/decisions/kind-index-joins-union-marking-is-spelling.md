# The kind index joins; union marking is spelling

> Workbench succession (2026-09-05): current workbench evidence lives in
> `lean/`; Idris paths below are historical. See
> [Lean is the workbench](lean-is-the-workbench.md).

## Decision

The semantics layer takes the join-lattice orientation: a **small general
core** — lattice-kinded references, generic coordination — in which a
cross-kind mention is a join (`AnObject \/ APlayer`) rather than a marked
constructor. This is old semantics' design, assessed sound (its failure was
migration difficulty, not shape), and the workbench prototypes the semantics
layer cards will be written in, so the workbench takes that orientation too.

The layer pursues two goals at once, under different design pressures. **The
core is small and easy to lower** into engine-runnable code, and that — not
surface affinity — is what a core shape is judged by; meta constructs, which
never appear on a card outside an expansion, are ordinary and acceptable there.
**The card language is an expressive vocabulary of functions over that core**,
neatly mimicking oracle English, and easy mapping between parsed English and
semantics, in both directions, is a stated goal of that vocabulary rather than
of the core. Today those functions are plain Idris functions, sometimes
dependently typed; they are called macros because the macro doctrine is where
they are headed as this effort completes.

Common phrasings — the union family included — belong to that vocabulary and
not to the core's constructors: "any target" [CR#115.4] is a function expanding
to a joined-kind term. The workbench's error was rank: it promoted phrasing
facts to core constructors when they belonged in the card language and the
spelling declarations, freezing phrasings into type-level law. A function keeps
the authoring ergonomics and the surface affinity while staying cheap to add,
delete, or reshape.

The marked union constructions — `AnyTarget`, the cross-kind union head
`KindJoin`, the mixed group `YouAnd`, and the `That` / `UnionP` anaphor that
reads one back — are reclassified accordingly. Their measured tables are
**spelling-boundary knowledge**, facts about what English writes, feeding
english_v2's construction declarations; their authoring role passes to the card
language's functions over the joined-kind core. Neither role is a core
constructor.

This settles the direction only. Execution shapes are decided per round with
the campaign's measured inventory — the workbench closure tables and the
workbench union-family tickets — as specification input, never as
re-litigations of the direction: whether the join is flat or pair-carrying (the demonstrative
echo copies its antecedent's kind pair 33 of 33 times), how a kind-indexed
`Binding` is re-derived under a joined kind, and every per-site migration.

## Rationale

The 2026-08-15 layering evidence stands, re-scoped. "The surface always marks
union sites" — a rules-minted class word, an explicit disjunction, explicit
coordination — is a fact about English. It specifies the spelling layer rather
than constraining the semantics kind index; reading it as a constraint on the
representation put English's marking inside the meaning.

The census measured the two designs instead of arguing them. The join wins the
projection layer, where placelessness, type absence, and the union rows'
damage-recipient admission follow from the joined kind's empty zone and type
instead of being written down; it costs the discourse layer, where the echoed
kind pair and the binding-index argument become measured silences. Those are
discourse and surface obligations, so they move to the spelling boundary —
where the collapse rule the census could not derive from any lattice already
belongs, since which words a demonstrative spells is a spelling decision.

Overgeneration in the semantics is therefore tolerated by design: a semantics
value with no production has no English and is refused at the boundary, an
authoring-time error rather than model unsoundness.

## Consequences

- In the core, union sites are ordinary coordination at a joined kind, not
  named rows; the card language may still name the familiar phrasings, as
  functions expanding to such terms.
- A core shape is judged by lowerability and by its algebra, never by surface
  affinity, and a meta construct is legitimate there. Phrasing knowledge that
  would otherwise become a core constructor goes into the card language or the
  spelling declarations instead.
- Every measurement the census tagged as a gate that must move in re-homes at
  the spelling boundary as construction-declaration content, preserved
  verbatim: a measured zero dropped in the move becomes a silent yes.
- At lowering, grammar Player and every union may map onto core's single object
  sort with an is-player predicate; the refusal list (no zone, no card
  characteristics, no destroy/exile/tap) lives in core. Grammar gates using
  zone presence as a proxy cannot key on zone for a zoneless player-object.
- A conjoined recipient is a noun-coordination question, not a kind-unification
  question.

## Tracked references

- [Workbench closure tables](../idris-workbench-closure-tables.md) — the
  campaign's measured closure and slot inventory; the per-site union
  translation is carried by the workbench union-family tickets.
- [Semantics v2](semantics-v2.md) — the workbench contract this shape lands in.
- [Macros are declarative](macros-are-declarative.md) — the typed-template
  doctrine the card language's functions are headed for.
- [Builtin-v2 macro spelling and grammar](builtin-v2-macro-spelling-and-grammar.md)
  — macro `spelling` as the typed phrase shape such phrasings are declared as.
- [Semantics, spelling, lowering](semantics-spelling-lowering.md) — the layer
  split the union facts are sorted against.
- [English grammar is derived](english-grammar-is-derived.md) — where the
  marked constructions land, as declarations.
- [English productions ship their inverse](english-productions-ship-their-inverse.md)
  — the boundary refusing a value with no production.
