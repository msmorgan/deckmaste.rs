---
needs: [english-v2-adjunct-licence-removal]
---
# One scope device for right-peripheral and shared constituents

**B7 — the scope device.** DESIGN TICKET; the design is written and reviewed
before an implementer starts. Authority: rewrite ADR "Ruling: adjunct licences
removed; attachment misselection is a recorded class (2026-09-03)" and
"Amendment: one scope device, principles before packing (2026-09-04)" (Q1-Q4).
Design: `docs/memory/scratch/plan09-postmortem/b7-scope-device-design.md`
(gitignored).

## Scope (Q1)

One device covers right-peripheral Adjunct attachment **and** Coordination
bracketing. Two candidates that differ only in where a right-peripheral
Constituent attaches, or in how far a Constituent realized once beside a
Coordination scopes over it, are one ambiguity class: *scope of a
right-peripheral or shared Constituent*. Adjunct height, Postmodifier scope over
a Coordination, a shared Determiner, a shared Modifier, a shared head and a
shared preposition are that one class. Was: right-peripheral Adjunct attachment
only (2026-09-03 letter, superseded 2026-09-04 by coordinator ruling Q1).

## Ordered principles, then packing (Q2)

Declared English principles decide first; whatever survives is packed. Each is
one general rule over derivation shapes and declared features, stated once —
never a corpus count, never a word, never a dominance edge between named
constructions. In order:

1. A qualification Complement (scalar comparison, degree measure, power and
   toughness value, granted keyword line, quoted ability) predicates a property
   of a Nominal and has no verb-Adjunct site. Landed: the complement-kind site
   capability.
2. A frame-selected role preempts the Postmodifier derivation of the same
   preposition, at every preposition the Verb Frame declares and at every depth
   of the complement it governs. `english-v2-role-preemption-depth` (B6).
3. An identity claim outranks a lexeme claim over the same bytes. Landed: the
   Identity specificity tier.
4. A distributive measure attaches to the Predicate as its multiplier and has no
   Nominal-Postmodifier derivation. **Lands inside this ticket.**

Not admitted, therefore packed: the scope of a shared Determiner over a
Coordination. English is ambiguous there and Semantics chooses.

## Representation (Q3)

The ambiguous Constituent is hoisted to its highest admissible host and carries
the set of admissible lower hosts as a derived value on that host, like the zero
Determiner: a declared field kind, no new AST category, no bytes in any
Realization. Sites are named by role path, never by candidate ordinal or rule
index. Render is unaffected — every host renders the same bytes — leaf traversal
is unchanged, and the canonical construction path is the hoisted one. Consumers
(semantics workbench, xtask diagnostic display) read the slot. A first-class
packed AST node holding alternative subtrees is deferred to the re-layering
wayfinder.

## Census (Q4)

A packed candidate is one candidate: not a tie, never a STOP. `packed_units` is
a REPORT figure emitted beside the resolution counts, with its identities
listed, and it does not enter the `selected == unique + specificity_resolved +
exception_resolved` partition identity. A tie that is not a scope tie remains a
STOP.

## Sequencing (Q6)

B7 lands before `english-v2-attachment-class-declared` (R1) and
`english-v2-locative-coordination-arms` (R7) re-measure and land; both are
parked pending it. `english-v2-role-preemption-depth` (B6) lands as part of this
ticket or immediately after it. `english-v2-frame-complement-coordination` mints
the missing coherent Coordination for a Verb Frame's complement cluster, which
principle 2 makes a hard prerequisite of R1's re-measure. Nothing partial.

## Routed residues this ticket absorbs

2026-09-04, from the `english-v2-with-preposition` landing review: fourteen
identities where a free comitative Adjunct has no Adjunct site and attaches to
the object Nominal (suspend template, attack instrument), plus Vicious Rivalry
and seventeen siblings where Postmodifier scope over a Coordination is the same
choice one level down. Admissibility is no longer the defect once R1's declared
classes land; the survivors are this class.

2026-09-04, from the `english-v2-possessive-nominal-form-collapse` (R10) landing
STOP: the possessive Coordination arm. `possessed_reference` has no Coordination
arm, so `your creature and artifact` has no derivation; its sibling
`genitive_determiner_coordination_reference` is the shape the arm copies. The
arm admits `Destroy your creature and artifact.` and breaks Aquatic Alchemist //
Bubble Up (`eabbc2128de743c6555fa404e3102827062681275d373db6f3adbed32980abbb`),
whose two bracketings differ only in how far a shared Modifier and a shared head
scope over the Coordination. Both are English; the device packs them.

2026-09-04, from the `english-v2-attachment-class-declared` (R1) landing review:
153 distributive-measure misselections (principle 4 decides them), ~79 object
characteristics pulled into relative Predicates (packed), 2 prohibition-scope
inversions (packed), and 62 gains bought by an incoherent bracketing (principle
2 eliminates it; the coherent reading needs
`english-v2-frame-complement-coordination`).

2026-09-04: `english-v2-granted-ability-coordination` takes over the granted-line
residue for this item only.

Standard constraints apply. Gate scope: `cargo test --workspace` (the device is
a declaration-compiler field kind).
