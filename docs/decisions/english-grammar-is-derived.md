# English grammar is derived

## Decision

One Rust-native construction declaration is the defining authority for each
English construction family. Three projections are generated from it:

1. **Parse** — recognition, feature checking, reduction, and lowering.
2. **Render** — total linearization from the same AST shape to English bytes.
3. **Build** — public smart constructors and validation adapters that compute
   local agreement and reject impossible combinations.

Neither direction may keep a separately maintained copy of a construction's
linguistic content. Derivation removes the drift class that discipline alone
managed; it does not by itself prove inversion — the construction laws,
ambiguity checks, and the declaration compiler's own suite remain
load-bearing.

The stack stays layered:

| layer | owner | responsibility |
|---|---|---|
| constructicon | construction declarations | form--meaning families, holes, inheritance, specificity |
| constituency service | chart and packed forest | constituents, feature flow, gaps, ambiguity |
| realization | linearization records | morphology, surface parameters, byte production |

Migration is a ratchet: a family is either derived or explicitly handwritten
in a registry, and each family deletes its handwritten parse, reduction,
lowering, and renderer paths in the change that derives it. The registry is a
ratchet state, never an end state; difficulty in a milestone identifies a
compiler or backend requirement and never narrows the target back to a
permanent partial grammar. If a pilot family falsifies a construction law,
the failed law or backend limitation is recorded, the prototype is removed,
and this decision is re-reviewed — a second partial grammar is not preserved
to justify the direction.

This contract was selected from three competing formalization candidates
(constructicon derivation, formalism layering, GF-first linearization
discipline); the comparative record survives in repository history.

## Shared feature vocabulary

One canonical Rust type per grammatical concept; parser and renderer never
mint parallel versions. The vocabulary is stratified so facts enter chart
identity only when parsing needs them:

- **inherent realization features** — person, number, noun cardinality, verb
  slot, onset, case, and pronoun class;
- **selection features** — lexical valency, gap state, complement role, and
  attachment phase;
- **surface witnesses** — contraction, punctuation, optional material, and
  other distinctions retained solely for exact replay, never in chart
  identity; and
- **discourse occurrence roles** — `Argument` versus
  `Mention(Full | Pronoun | Demonstrative)`, declared on spelling-frame
  occurrences and resolved by the spelling side's whole-scope discourse
  environment, never inferred from local subtree features.

Surface witnesses travel on packed derivation alternatives and exact parse
results. Packing may share an otherwise identical chart node, but it may not
discard or coalesce alternatives that carry distinct witnesses.

## Construction declarations

A declaration states: a stable `ConstructionId` independent of registry and
rule order; the AST shape it constructs and destructures; a typed
linearization expression and its accepted surface domain; typed holes with
category, feature, valency, and role constraints; the surface witnesses
required for exact replay; explicit dominance relations for overlapping
constructions; and any reusable monotonic packaging traits.

Holes are not assumed to be whole constituents: the declaration language
covers category-typed subtree holes, field-slice/lens holes for flattened
records, scalar holes, and identity holes. Each class lands when a migrating
family first needs it.

The compiler emits parser productions with their reductions and lowering; a
total destructurer and linearizer; public smart constructors and validation
adapters; construction metadata for `inspect`; and structural registration
data used to prove that no handwritten mirror remains.

For every exact construction and admitted `(ast, surface)` combination:

~~~text
(ast, surface) in parse_as(
    construction,
    linearize(construction, ast, surface),
)

for every (ast, surface) in parse_as(construction, bytes):
    linearize(construction, ast, surface) == bytes
~~~

Impossible combinations fail declaration validation or exhaustive matching;
there is no string fallback, default match arm, or renderer guess based on
one lexeme.

For an invariant-bearing migrated AST family, generated validation is the
only admitted ingress, not merely the preferred Rust API. Direct field
construction is sealed outside generated code, and deserialization passes
through the same validator. A family whose public or serialized
representation still bypasses validation is explicitly unmigrated rather
than a weakening of the build guarantee.

## Exactness domain

The guarantee is byte-exact replay of normalized, name-bearing Oracle text
and of fragment entry points explicitly declared exact. Byte-exactness
belongs to this layer; canonical wording belongs to the spelling relation
([Authoring, spelling, lowering](authoring-spelling-lowering.md) §3).
Canonicalizing helpers are a separate API and may not masquerade as exact
parsers.

If one exact entry point accepts two spellings that lower to the same AST,
the distinction is retained as a typed AST or surface-witness value. Free
surface witnesses are first-class, not an escape hatch: `ClauseCoordination`'s
comma is documented house-style variation with no derivable rule and stays
stored. Derivability of a stored surface fact is a per-construction
measurement, not a law: swap the derivation into the renderer with the field
still stored and measure with `cargo xtask english roundtrip --list`; a
refuted parameter stays stored with the refutation documented on the field.

## Ambiguity, specificity, and attachment

Specificity is declared and construction-local: the compiler validates an
acyclic dominance relation and rejects candidate sets that promise one
answer but have incomparable maxima. Legitimate ambiguity remains packed and
visible; assembly order never silently decides linguistic meaning, and
generated families must not depend on rule identity or insertion order.
Existing parse costs may remain as explicit, inspectable tie-breakers
between structurally equivalent derivations; statistical or learned runtime
ranking is excluded.

Lexical valency and hole roles distinguish selected complements, noun
postmodifiers, predicate adjuncts, and other attachment owners; global
attachment costs do not substitute for those local constraints. `inspect`
reports the selected `ConstructionId`, the decisive guard, feature, or role,
the cost, and any equally viable alternatives.

## Parser backends

The construction language is backend-independent. Fan-out-one productions
compile to the existing CFG chart, which remains the constituency service
where its representation is sufficient. A construction requiring
tuple-valued or discontinuous yields requires an explicit PMCFG extension
(tuple spans, specialized categories) or an explicitly measured CFG
approximation plus filtering — built when a family first meets the
criterion. The families owned by the handwritten ability layer must gain a
generated backend before the program is complete. "PMCFG-style" names the
logical linearization formalism, not a claim that the single-span chart
already implements PMCFG.

## Hierarchy, spelling seam, and corpus

Shared packaging uses a small monotonic, SBCG-style type lattice;
default-overriding inheritance is admitted only after a documented case
where monotonic sharing fails, and no general reentrant HPSG/DAG engine is
introduced.

The constructicon describes English syntax; authored meanings and `frames:`
on defs remain spelling-owned. Typed AST substitution replaces render-time
textual splice-and-reparse for each migrated hole class; a substituted
subtree carries its inherent features so agreement computes compositionally.
Occurrence selection, discourse linking, role keys, and multi-sentence frame
scope remain spelling-engine concerns; no binder or discourse resolution
grows into the construction matcher.

Corpus attestation lints the authored guard and role inventory and may
annotate omissions by null-instantiation class; the corpus is evidence for a
declared rule, never a probabilistic authority. Direct AST assertions and
`inspect` are the semantic authorities; bracket output is diagnostic.

The [Oracle text style guide](../oracle-style-guide.md) is the CR- and
corpus-derived catalog of Oracle's surface conventions; consult it when
deciding whether a surface distinction is semantics-bearing or house style.
Like the corpus it is evidence, never an authority over a declared rule; it
cites no implementation and is amended only with corpus evidence, never to
match this grammar.

## Non-goals

- A GF toolchain or runtime dependency — a second defining authority whose
  value decays ([Authoring, spelling, lowering](authoring-spelling-lowering.md)
  §9).
- A second grammar authority or third semantic IR (§14).
- Probabilistic or learned parse-time ranking.
- A general HPSG/DAG feature-structure engine.
- Per-verb construction entries for open-class composition (§14).
- Binder or discourse resolution inside the construction matcher (§14).
- Preserving accidental rule order as semantics.

## Consequences

- A migrated family's declaration is its only linguistic authority; a
  structural audit proves no handwritten parser, reduction, lowering,
  renderer, or duplicate registration remains for it.
- Unmigrated families remain governed by
  [English productions ship their inverse](english-productions-ship-their-inverse.md)
  until derived.
- `cargo xtask english roundtrip --require-clean` stays green on every
  migration; permuting generated family registration leaves semantic
  selections unchanged unless declared precedence says otherwise; and
  compile-fail and negative deserialization fixtures prove sealed families
  cannot be bypassed.
- Active consumers (`deckmaste_spelling`, `xtask`) migrate in lockstep with
  affected families: AST visibility and constructor changes are migration
  work, not free cleanup.
- The program is complete when every supported English family is derived,
  raw construction of migrated invariant-bearing families is sealed, and no
  paired handwritten grammar/renderer authority remains.

## Reference literature

- Ranta, *Grammatical Framework* (2011); Angelov 2009 and *The Mechanics of
  the Grammatical Framework* (2011) — linearization types; incremental PMCFG
  parsing, the recorded escape hatch.
- Scott 2008; Aycock & Horspool 2002; Leo 1991 — packed forests, nullables,
  right recursion.
- Boas & Sag 2012 (SBCG); Lascarides & Copestake 1999 (YADU) — monotonic
  hierarchy and the deferred default machinery.
- Steels 2011 (FCG) — bidirectional construction application and reassembly
  failure modes.
- Ruppenhofer et al., *FrameNet II*; Stefanowitsch & Gries 2003 — valence,
  null instantiation, collostructional analysis.
- GF Resource Grammar Library — reference analyses, never code.

## Tracked references

- [English clauses are structural](english-clauses-are-structural.md)
- [English productions ship their inverse](english-productions-ship-their-inverse.md)
- [Macro templates are bidirectional](macro-templates-are-bidirectional.md)
- [Authoring, spelling, lowering](authoring-spelling-lowering.md)
- [Oracle text style guide](../oracle-style-guide.md)
