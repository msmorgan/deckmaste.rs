---
needs: []
---
# Stratum 7: long-tail quotes and mentions

Part of `english-v4-stratum-7-discourse-and-long-tail`. Implement in `crates/deckmaste_english_v4` per [`docs/decisions/english-v4.md`](../../decisions/english-v4.md) (§6, Stratum 7).

## Scope

Quoted text used as a constituent, and mentioned words and phrases.

Quotes:
- **Brutal Suppression, Drought**: *cost an additional "Sacrifice a land" to activate*. Hypothesis: a quoted imperative acts as a count nominal after *an additional*.
- **Volrath's Shapeshifter**: *has the text "{2}: Discard a card."*. Hypothesis: an appositive quoted ability after the noun *text*.
- **Urza's Saga**: a single-quoted ability nested inside a double-quoted one. Hypothesis: nested quotation depth is unsupported.

Mentions:
- **Teferi's Realm**: *chooses [artifact, creature, land, or non-Aura enchantment]*. Hypothesis: a multiword mention in a coordination.
- **Sarpadian Empires, Vol. VII**: *choose [white Citizen, blue Camarid, …, or green Saproling]*. Hypothesis: each mention is a color adjective plus a subtype noun.
- **Artificial Evolution**: *The new creature type can't be [Wall]*. Hypothesis: a general predicative mention multiplied forests, so this needs a narrow, head-selected route. Measure.
  Artificial Evolution must parse. Route: a copular *be* whose subject's head is a type noun (*creature type*, *card type*, ...) takes a subtype name as its predicative complement. The general version (any mention after *be* or *become*) quadrupled forests, for example Gavony Dawnguard 115 -> 460 on *becomes night* / *is red*. Acceptance: zero growth on those units.
  Design note: the clause naming the creature type is a specifying copular clause (CGEL, Ch. 4 §5.5.2, pp. 266–268), with the type name as its specified element. License a bare `Class("Subtype")` predicative only when the subject's head is `noun:type`, never for any mention after *be*. Expected forest growth is zero.
- **Grim Captain's Call**: *then do the same for [Vampire, Dinosaur, and Merfolk]*. Hypothesis: a mention coordination as the complement of *for*.

## Witness cards

- Brutal Suppression
- Drought
- Volrath's Shapeshifter
- Urza's Saga
- Teferi's Realm
- Sarpadian Empires, Vol. VII
- Artificial Evolution
- Grim Captain's Call

## Estimated faces

8 faces.

Raging River moved to `crates/deckmaste_english_v4/declined.txt` (pre-8th Edition, unique construction); it is no longer a witness here.

## Hypotheses, not findings

Every diagnosis above comes from a triage of the unread faces and is a hypothesis. Before changing anything, confirm it against the code and a parse of the card, and drop or replace any diagnosis that does not hold. Measure each construction before and after (§4.4, principle 5), and add a positive judgment per witness asserting its intended analysis (`crates/deckmaste_english_v4/judgments/`).

## Constraints

Governance is `docs/decisions/english-v4.md` §4.4, Principles: admit English, suppress nothing grammatical, add the general construction the text needs (any features, values, Categories, Productions or engine capabilities, each justified by a CGEL, OSG or CR citation), and keep the engine free of grammar names. Standard constraints apply.

## Landing note

Per `docs/tickets/README.md`, end the ticket with these two lines, stamped with the change id they were measured on:

1. **Census delta**: faces read and total Readings before and after; lost faces by name, or "none".
2. **Judgment set**: the result of running it.

## Review notes (hypotheses, verify first)

- The Artificial Evolution route is right.
- *Grim Captain's Call*, *do the same for Vampire, Dinosaur, and Merfolk*, interacts with the `for` + AdjP fix (rescan-lexical-leaks).

## Findings

- **Urza's Saga**: the hypothesis was wrong. Nested single quotes in a quoted ability already read (chapter II, one Reading). The unread unit was chapter III, *an artifact card with mana cost {0} or {1}*: *mana cost* took no cost complement. It now has `Complement(Cost)` and is count and mass (bare after *with*, as *mana value* is).
- **Brutal Suppression, Drought**: a quoted imperative is a singular count noun that a modifier premodifies (`QuotedCostPremodifier`). Unrestricted, it duplicated the quoted-ability Reading in every *X and "…"* coordination, so it is premodifier-only.
- **Volrath's Shapeshifter**: *text* takes a quoted ability as a frame Complement (CGEL, Ch. 5 §14.3, p. 447).
- **Teferi's Realm, Sarpadian Empires**: `CompoundMention`, with a modifier that is one attributive adjective (*white*) or *non-* plus one cited word. A cited noun before a cited noun would give *choose target creature* a second Reading (Sewers of Estark 36 -> 72), so it is not a modifier.
- **Artificial Evolution**: `SpecifyingClause` and `SpecifyingModalClause`. The noun *type* carries `Variable: Cited` (lexical field `variable`), and only a Subject headed by it takes a subtype name as the value of *be*. A Predicative `Class("Subtype")` after any *be* grew 63 units (*are Plains*, *are Bears*, *are Equipment*) as duplicates of the bare-NP predicative, and the Subject cannot be seen from inside a VP, so the clause is flat: Subject, optional modal, *be*, value.
- **Grim Captain's Call stays deferred** (follow-up `english-v4-s7-cited-for-complement`).

## Landing

1. **Census delta** (measured on rmtxvlmm): faces read 32782 -> 32789 of 32828, total Readings 337038 -> 337083; lost faces: none. Gained: Artificial Evolution, Brutal Suppression, Drought, Sarpadian Empires Vol. VII, Teferi's Realm, Urza's Saga, Volrath's Shapeshifter. Every changed unit is one of those faces' own previously unread units; no other unit changed.
2. **Judgment set**: `cargo test -p deckmaste_english_v4` and `-p deckmaste_lexical_source_v4` pass, including the new `judgments/quotes_and_compound_mentions.txt`; `strata --check` exits 0.
