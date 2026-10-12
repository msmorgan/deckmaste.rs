---
needs: [grammar-closed-class-words]
---
# Analysis key in the census

Give every reading a second identity, its analysis key, and record it in the census. The tree stays the Reading; the key says which head each dependent attaches to, and in what function. This is the instrument that decides what a later change may remove: the approved cases (order of applying modifiers to one head, grouping within a same-coordinator chain, clause versus verb-phrase coordination with nothing to tell them apart) are those where trees differ and the key does not.

A Reading value carries constructor name, form index and named ordered fields (`crates/deckmaste_construction_v3_core/src/emit.rs:692–711`), not declared head or function annotations, so the key needs a per-construction mapping. This ticket comes after the authoring-layer tickets because by then every construction already names its head, which the mapping uses. Cost: 255 construction declarations, 71 schemas and 83 instance blocks; reusable schema mappings reduce repetition, but every expanded form and frame alternative needs coverage. Planning estimate: hundreds of annotation sites, roughly 1,000–3,000 annotation lines on today's grammar, plus compiler parsing, checking and emission, occurrence tracing, canonicalization, rendering and tests (not a measured patch size). The mapping annotations count as grammar lines (see `grammar-size-report-and-check-mode`); this landing is the one approved exception to the line total not rising, and it reports by how much it rose.

## Definition

**Identity.** A Reading remains a tree; its analysis key is a second identity. Two trees share a key only through the three approved equivalences below. Equal realization, apparent semantic equivalence or common lexical heads never license equality.

**Schema.** A key holds: version and source context; lexical occurrences; structural hosts; typed dependency edges; scope expressions; coordination groups; gap, binding and sharing relations; quotation and document boundaries; retained grammatical distinctions. An occurrence is anchored by its half-open Unicode-scalar span in the analyzed source and its enclosing quotation or document boundary, and stores lexeme identity, lexical category, inflected form, variant and grammatical lexical value. Numerals, symbols and flavor labels keep their values. Two occurrences of `card` at different positions are different nodes. Chart ids, enumeration order and constructor traversal paths are never anchors.

**Heads and functions.** An `analysis` mapping is declared in the grammar per construction and form, reusable through schemas and specialized where necessary. It is never inferred from a field named `head`.

| Family | Head or host |
|---|---|
| Lexical construction | Its lexical occurrence |
| Ordinary headed phrase | Declared head child; a noun is head with its determiner as a scope operator; a determinative is head in fused uses |
| Auxiliary, copula | Local head (attachments to `can't`, `be` and `blocked` stay distinct; copular and passive structure preserved) |
| Frame-selected preposition | Its own phrase host, with a complement-slot identity; never merged with adjunct function |
| Clause | Structural clause host with subject and predicate relations |
| Coordination | Structural coordination host, not its first conjunct |
| Document, paragraph, Ability, mode, quotation | Typed structural container |
| Gapped construction | Declared head plus explicit gap node |
| Headless notation, quantity, cost, list (slash pair, quantity, cost) | Typed structural host with ordered operands; no manufactured lexical head |
| Pure grammatical wrapper | Explicit transparent mapping, keeping any distinction it contributes |

Functions are typed: `subject`, `predicate`, `object`, `indirect-object`, `predicative-complement`, `complement(slot)`, `adjunct(kind)`, `modifier(kind)`, `determiner`, `marker`, `supplement`, `conjunct(index)`, and structural roles `cost`, `body`, `header`, `mode(index)`, `parameter(slot)`, `operand(index)`. Gap binding and sharing are separate relations, not dependency labels. Frame slots get stable identities; a frame-selected PP complement cannot become an adjunct.

**Compiler obligations.** The compiler rejects: any unmapped construction, form, category specialization or selected-frame alternative; missing, multiply consumed or ill-typed fields; unhandled optional or repeated fields; invalid head, function, slot, scope or gap references; lost lexical occurrences, invalid sharing or undeclared modifier classes; an unaccounted distinction between alternative forms. A total key visitor is generated from the checked mapping. No mapping is an error and a failed enumeration or check for that face, reported by the census; there is no opaque or lossy fallback. Static checks establish completeness and consistency; the linguistic assignments still need review.

**Retained structure.** Everything not removed by the three collapses stays: lexical identity and form, attachment head, dependent function, coordination host level, voice, clause type, gap function, sharing, scope. These are stable declared tags, not constructor spelling. A transparent wrapper must not identify two analyses outside the approved equivalences.

**Collapse (a), modifier order.** Only a connected run of modifiers is canonicalized: attached to the same head occurrence, declared `OrderInsensitive` for that use, with unchanged functions and internal keys and no intervening scope-sensitive operation. The run becomes an occurrence-preserving, canonically sorted collection; surface order and attachment are unchanged. Every modifier use declares `OrderInsensitive` or `ScopeSensitive`, never by spelling. Initial order-insensitive classes: colour, type and state restrictions, and restrictive postmodifiers. Everything else is scope-sensitive: *other*, focus, quantity, degree, determiners, modals, negation, non-intersective modifiers.

**Scope.** Each scope-sensitive operation is `Apply(operator occurrence, operand expression)`, preserving nesting and the operand's constituent and coordination extent. Insensitive modifiers commute only within one scope region and never across `other`, `only`, a determiner, a modal or another sensitive operation. So `other(red(creatures) + relative)` stays distinct from `other(red(creatures)) + relative`; a flat dependency list is insufficient.

**Coordination and collapse (b), grouping.** Store host kind, ordered conjuncts, anchored coordinators, correlative markers and external or shared dependencies. A nested group is flattened only when it has the same coordinator identity and compatible host kind and flattening preserves every dependent's target and scope. Mixed `and`/`or` nesting is kept; correlative pair boundaries, attachment to a subgroup and shared complements (one occurrence with relations to each selecting head) are preserved; coordinated argument clusters are not flattened into unrelated lists. Coordination hosts stay distinct (Nominal versus NounPhrase included); the only collapse is (c).

**Collapse (c), clause versus verb-phrase coordination.** Bare imperative alternatives normalize to coordinated imperative clauses only when: every conjunct has the same imperative force and unchanged lexical forms; no overt subject or distinct subject/gap binding separates the alternatives; coordinator structure, complements, sharing and functions match; no auxiliary, modal, negation, modifier or other operator changes scope. `Draw a card and gain 1 life.` qualifies; `You draw a card and gain 1 life.` and `You may draw a card and discard a card.` do not. The rule is not extended to other subjectless coordination.

**Gaps, passives, sharing.** A gap is anchored to its governor, declared function or frame slot and construction domain, not an invented source token; it records its binder and extraction path, including stranding and pied-piping; subject, object, PP-complement and adjunct gaps stay distinct. Passive voice, the subject and internalized-argument relations and the agent-complement function are preserved, as is adjectival state versus verbal passive. Shared material is represented once with explicit sharing-group membership and all selecting heads. Ordinary pronouns are not resolved to antecedents to build a key.

**Quotation and atomic terms.** A quotation keeps its boundaries, grant or complement function and nested document structure; positions stay tied to the original source. A plugin-declared multiword term is one lexical occurrence spanning the whole term, with no invented internal heads (`venture into the dungeon`).

**Encoding and rendering.** A versioned, deterministic encoding: unordered relations sorted, conjuncts and operands in order, structural nodes canonically numbered; the hash of it goes in reports. A readable bracketed renderer is a deliverable (the judgment set needs it), for example:

```text
[destroy@0:7
  object=[creature@15:23 determiner=target@8:14]
  adjunct=[with@24:28 complement=flying@29:35]]
```

It also displays `scope=`, `gap=`, `binds=`, `shared=`, host kind and lexical-form differences, so judges never need tree dumps to tell keys apart.

**Removal gate.** For every stable enumeration unit, an approved collapse requires identical sets of keys before and after, with complete validated enumeration and compatible versions and context, surrounding-structure keys included. Changed unit boundaries need reconstruction to a common comparison unit or an explicit migration; comparing new line numbers silently is forbidden. The check mode exits non-zero on a lost face, a missing mapping, incomplete enumeration, an incompatible baseline or a prohibited key-set difference.

## Required tests

`NP` is category noun-phrase; otherwise sentence. Tree counts are measured on today's grammar; key counts are the expectations to assert. For every row with one key, assert the actual edges, scope and bindings, not only the count. Rows 39–40 record defects and are not positive correctness judgments.

| # | Text | Cat. | Trees → keys | Principal assertion |
|---:|---|---|---:|---|
| 1 | red creatures you control | NP | 2 → 1 | Modifier order |
| 2 | red creatures with flying | NP | 2 → 1 | Modifier order |
| 3 | red artifact creatures you control | NP | 3 → 1 | Several insensitive modifiers |
| 4 | red nonlegendary artifact creatures you control | NP | 4 → 1 | Same |
| 5 | tapped creatures you control | NP | 2 → 1 | Declared state restriction |
| 6 | other red creatures you control | NP | 3 → 2 | `other` scope kept |
| 7 | only red creatures you control | NP | 2 → 1 | Same outer focus; inner order only |
| 8 | another red creature you control | NP | 2 → 1 | Same outer determiner; inner order only |
| 9 | Draw a card and gain 1 life. | | 2 → 1 | Bare imperative |
| 10 | Draw a card and gain 1 life and create a token. | | 6 → 1 | Imperative plus same-coordinator grouping |
| 11 | Draw a card and gain 1 life and create a token and discard a card. | | 21 → 1 | Longer chain |
| 12 | This creature has flying and haste and vigilance. | | 2 → 1 | Same-coordinator grouping |
| 13 | Draw two cards, then discard a card. | | 2 → 1 | Bare imperative; `then` kept |
| 14 | Draw a card and draw a card. | | 2 → 1 | Four distinct repeated-word occurrences |
| 15 | Draw a card or gain 1 life and create a token. | | 6 → 2 | Mixed grouping kept |
| 16 | You may draw a card and discard a card. | | 3 → 3 | Modal scope, clause structure, forms |
| 17 | You draw a card and gain 1 life. | | 2 → 2 | Shared subject versus imperative; forms |
| 18 | You draw a card and you gain 1 life. | | 1 → 1 | Repeated overt subjects |
| 19 | Destroy target creature with flying. | | 2 → 2 | Different attachment heads |
| 20 | Destroy all creatures and planeswalkers. | | 2 → 2 | Determiner scope |
| 21 | red creatures and planeswalkers | NP | 3 → 3 | Attachment extent, coordination host |
| 22 | Target creature is tapped. | | 2 → 2 | Adjectival lexeme versus verbal passive |
| 23 | Search your library for a card. | | 3 → 3 | Selected complement, VP adjunct, noun modifier |
| 24 | This creature can't be blocked. | | 2 → 2 | State/action distinction |
| 25 | This creature can't be blocked by Walls. | | 7 → 7 | Auxiliary attachments, complement/adjunct roles |
| 26 | the card you cast | NP | 2 → 2 | Present/preterite forms |
| 27 | the card you put into your hand | NP | 2 → 2 | Forms; object-gap binding kept |
| 28 | Artifact spells you cast have convoke. | | 4 → 2 | Modifier collapse, tense preserved |
| 29 | Spells you cast cost {1} less to cast. | | 4 → 4 | Two independent tense distinctions |
| 30 | This creature gets +1/+1 for each other creature you control. | | 2 → 2 | Non-intersective scope |
| 31 | Creatures block creatures with flying. | | 2 → 2 | Occurrences and attachment kept |
| 32 | This creature has either flying or haste. | | 1 → 1 | Correlative markers |
| 33 | Put a card into your hand and a card into your graveyard. | | 1 → 1 | Paired argument clusters |
| 34 | Tap or untap target creature. | | 1 → 1 | Shared object |
| 35 | Venture into the dungeon. | | 1 → 1 | Atomic plugin term |
| 36 | the creature that attacks | NP | 1 → 1 | Subject-relative binding |
| 37 | the creature you attack | NP | 1 → 1 | Object-relative binding |
| 38 | Target creature gains "Destroy target creature with flying. Draw a card." until end of turn. | | 2 → 2 | Nested quotation; internal attachment |
| 39 | the card you have put into your hand | NP | 1 → 1 | Existing wrong structure stays identifiable |
| 40 | the card you are putting into your hand | NP | 0 → 0 | Missing analysis is a recorded obligation |

## Change

- Compute the key for each enumerated reading, as defined above.
- The census records, per unit, the number of trees and the number of distinct keys, and `diff` reports the set of keys before and after.
- Deliver the readable bracketed renderer.
- One census run then gives the answer to how much of the 724,921 readings is derivational artifact. Report it.
- Output stays under `target/english-v3/`. The landing updates the census description in `crates/deckmaste_english_v3/README.md` and the compiler README for the mapping.

## Gate

Reading digests identical on every face, enforced by the census check mode from `grammar-size-report-and-check-mode` as an identical-digest landing, which gains its analysis-key landing type here. Note: census delta (none); the size report, with the tracked grammar line total raised by the mapping annotations and the amount reported (the one approved exception); the judgment-set line does not apply yet.
