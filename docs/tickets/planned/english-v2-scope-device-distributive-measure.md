---
needs: [english-v2-scope-device-collapse]
---
# The scope device, phase 4: a distributive measure attaches to the Predicate

**B7 phase 4.** Principle (iv) of the ordered principle set, the last
elimination the device needs. A Prepositional Phrase whose Complement's
Determiner declares distributive quantification is a measure over the Predicate
— a multiplier — and has no Nominal-Postmodifier derivation.

Design: `docs/memory/scratch/plan09-postmortem/b7-scope-device-design.md`
(gitignored), §B principle (iv), OPEN-2 and §G.2 phase 4. The Q1-Q6 rulings, the
OPEN rulings and the routed residues live on
`english-v2-underspecified-adjunct-attachment` (phase 1) as inherited context.

Runs after the collapse so the corpus movement it causes is measured against a
tree that already packs: an elimination and a collapse landing together cannot
be told apart in the census.

## Letter

- A new sealed feature domain — a **quantification axis** — on the Determiner
  vocabulary, distinguishing distributive quantification from everything else.
  OPEN-2 is ruled (2026-09-04): the Determiner vocabulary carries the axis. The
  Prepositional-Complement side is **refused** — it re-creates the
  per-preposition switch the fallout audit recorded as F3.
- One requirement at the Nominal-Postmodifier site, reading that declared axis
  through the generated accessor. One general rule over declared features: never
  a per-preposition switch, never a list of Complement kinds, never a named
  Determiner.

≈80 lines plus witnesses (§G.2).

## Acceptance witnesses (§G.5)

Decided, not packed:

- `Draw a card for each Island you control.` — the measure attaches to the
  Predicate; the Nominal-Postmodifier derivation does not exist.
- `Draw a card for each creature you control.` — same.
- the two `put … on … for each …` units (Animal Friend, General Leo Cristophe),
  one of which today applies a restrictive Postmodifier to a rigid self-name
  with no head noun to restrict.

Expected disposition from §F, provenance and not a target: 153 R1
distributive-measure misselections (classes A, D, E) return to the Predicate
multiplier, plus the 2 degradations above.

Negative:

- the negative probes restored by R1's review still reject;
- `unresolved_ties == 0`;
- byte-exact roundtrip.

## Fences (§G.4)

- No `require`, `checked by`, comment or literal naming a lexeme, Construction,
  verb, noun, preposition or card. A permitted guard reads a declared feature or
  a declared role property.
- No `SELECTION_EXCEPTIONS` entry, no dominance edge between named
  Constructions, no new specificity weight or tier. This is an elimination, not
  an ordering: expressing it as a preference weight is the F12 defect.
- No new AST Category, and no field holding an alternative subtree.
- Attestation is provenance: the axis's values answer "which English fact
  excludes the missing values?", never a witness count.
- No `run_in_background` on a gate; report positive artifacts.

Standard constraints apply. Gate scope: `cargo test --workspace`.

## Baseline

Measured on `lpvvplmyynul`, phase 1's base — re-measure at claim (phase 3 will
have moved every corpus figure). Lock `covered` 17,601; constructions 387;
32,641 units; census 13,759 unique / 3,842 specificity-resolved / 0
exception-resolved / 15,040 parse failures / 0 unresolved ties.

## Glossary gaps

Terms the design needs that `docs/contexts/oracle-english/CONTEXT.md` does not
define: Scope, Attachment, Head, Premodifier, Peripheral, Bracketing, Mobility.
Listed as gaps; none is coined into the tracked glossary by this ticket.
