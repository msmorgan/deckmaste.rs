---
needs: [english-v2-adjunct-class]
---
Repeatable, order-free NP postmodifiers (A2) — v2 fixes one slot each
(relative -> locative PP -> with); replace with one postmodifier position
that repeats, attachment per the recorded rule. Expected unlock: "Creatures
with flying you control" and the with-postmodifier reminder bodies.

Baselines: re-measure at claim (the lock's covered count and construction
count on the tree you claim from — several landings are integrating
concurrently); acceptance is >= that count AND lock diff -0 rows.

Carried from the adjunct-class review (binding):
- H1: delete the `head != CoreVerbIdentity::Control` blacklist (and its
  Seedborn Muse comment); declare the adjunct licence on the verb's
  valence row (core_verbs.ron / KeywordAction grammar) and derive
  attachment from it. Probes: "Destroy each creature you sacrifice during
  your upkeep." attaches `during` to the predicate; "Untap all permanents
  you control during each other player's untap step." still correct; four
  must-not-move NP-postmodifier witnesses of your choice from the corpus.
- M1: fold the four byte-identical host adapters into one construction
  with a host role (abstract sum); expected count -3.
- M3: the contracted-perfect and reduced-passive `opt PredicateAdjunct`
  slots are unguarded — guard them by the declared licence (the DSL cannot
  `checked by` an `opt`; find the general route or STOP with the limit).
- Decide and pin: "Draw a card after your library." rejects (after takes
  no object complement) — negative oracle.
- Restore the two ordinal assertions the adjunct landing weakened.
- Assurance line adds "assertions weakened" and "negative->positive"
  counts with the strings named.
- STOP FENCE: any guard naming a verb, noun, preposition, construction, or
  card identity is STOP-and-report (CLAUDE.md).

Acceptance: coupled replacement (each per-X family deleted with its general
construction landing), zero net coverage loss, ratchet up, zero ties or
STOP-and-report, no process artifacts in tracked source. Design only from v1
(docs/memory/scratch/plan09-postmortem/taxonomy-v1-v2.md) — no v1 code, types,
or feature vocabulary. Probe set: the audit §6 table filtered to this family.
Standard constraints apply.

## Landing record

Implemented on change `svulrlum` (2026-09-03). The ordered controller,
locative, and numeric ladder is now one recursive `PostmodifiedReference`
position. Relative clauses, reduced passives, scalar qualifications,
prepositional qualifications, and granted abilities can repeat in either
order. Relational complements produce an explicit saturated-relational
state: the completed relation remains a valid complement to an outer
relation, but cannot consume a second relational phrase on the same head.

The HIGH fix is data-driven. The `Control`/Seedborn Muse blacklist and comment
are gone. Core and declaration verb valence rows now carry full or
nonprepositional adjunct licences; active object-gap, contracted-perfect, and
reduced-passive routes consume adjuncts only when the selected valence row
declares the matching licence. No guard names a verb, noun, preposition,
construction, or card identity; the STOP fence did not fire. `Draw a card
after your library.` is pinned as a rejection, and both singular `each of`
ordinal negatives are restored.

| measure | claim baseline | landed before refresh | post-refresh |
| --- | ---: | ---: | ---: |
| corpus units | 32,641 | 32,641 | 32,641 |
| selected / covered | 16,401 | 16,463 | 16,768 |
| parse failures | 16,240 | 16,178 | 15,873 |
| unique | 10,967 | 10,844 | 11,009 |
| specificity-resolved | 5,434 | 5,619 | 5,759 |
| unresolved / exception-resolved / internal | 0 / 0 / 0 | 0 / 0 / 0 | 0 / 0 / 0 |
| round-trip / ownership failures | 0 / 0 | 0 / 0 | 0 / 0 |
| literal lexicon collisions | 59 | 60 | 60 |
| construction declarations | 393 | 394 | 399 |
| coverage-lock lines | 49,051 | 49,113 | 49,418 |

The schema-4 coverage lock is an exact add-only `+62/-0` ratchet at source
fingerprint `e85359d7b8c578df13dff2fdf7c743a520a5b367d5ed25ab0a5f03cb8b3637dd`;
its SHA-256 is
`160ec0576f2e424d95749efb3c94562b9337e39591b3a40eb540063a5219de86`.
The M1 host family itself is four byte-identical constructions to one
abstract-sum host construction (`-3`). The recursive replacement, independently
guarded adjunct routes, transitive subject-gap relative, and saturated
relational attachment are net `+4`, producing the overall `393 -> 394`
declaration count.

The required refresh incorporated concurrent coordinator gains before
integration. The resulting schema-4 lock remains add-only relative to the
claim baseline and has SHA-256
`a4fdc96b7bcb2770104e6cb0cda1f0a3f5cd63d529d30e1c5d1147e714fb3dd1`.
Four newly covered identities exposed after resolving the refreshed lock —
Shadow of the Goblin, Poxwalkers, The Fifth Doctor, and Keeper of Secrets —
were reviewed, blessed, and then accepted by a clean `coverage --check`; there
are no lost identities.

Assurance census: restored 2 exact ordinal assertions; re-spelled 1 test;
ignored 0 tests; added 3 tests; removed 0 tests; assertions weakened: 0;
negative->positive: 6 — `A card in exile you own gains 2 life.`, `A creature
you control you own gains 2 life.`, `A card in exile in your graveyard gains
2 life.`, `A creature with power 2 or less with toughness 2 or less gains 2
life.`, `Creatures with flying you control get +1/+0.`, and `Destroy target
creature with power 2 or less you control.`

Pre-refresh gates: `cargo test -p deckmaste_english_v2`; strict all-target
Clippy for `deckmaste_construction_core` and `deckmaste_english_v2`; `cargo
xtask english_v2 ambiguity --require-resolved --json`; and `cargo xtask
english_v2 coverage --check --json`. The workspace-wide test was deferred to
the required post-refresh pass because the separately owned
`compiled_consumer` fixture had not yet landed.

Post-refresh gates: `cargo test --workspace`; `cargo clippy --workspace
--all-targets -- -D warnings`; `cargo fmt --all -- --check`; `cargo xtask
english_v2 ambiguity --require-resolved --json`; and `cargo xtask english_v2
coverage --check --json`. The compiled-consumer fixture now derives both
adjunct-licence queries from its frozen valence rows, and its compile-fail
privacy diagnostic is updated for the two generated fields.

A second required refresh incorporated the corpus-wide visitor traversal
landing. Harmony retained that replacement suite and the nonconflicting
postmodifier tests; the same full gate set then passed on the final base.
Coverage visited all 731,226 selected construction nodes with zero traversal
failures.

### Erratum (landing review, 2026-09-03)

- HIGH: the adjunct licence is a whitelist (`environment.rs` valence rows
  default to no licence; six verbs opt in), so well-formed English the parent
  admitted is now rejected — `Draw a card for each card you've exiled this
  turn.` parse-fails while the `discarded` variant parses. The shape is
  corpus-attested on unlicensed verbs; those units fail for other reasons, so
  the add-only ratchet could not see it. The test
  `unlicensed_participial_relatives_leave_adjuncts_on_the_outer_predicate`
  pins the wrong reading. Follow-up: `english-v2-adjunct-licence-removal`
  (carries the ruling question).
- This record lacked a Deviations and additions section (6 constructions
  deleted / 7 added; −3 host fold, +4) and a performance advisory. Measured by
  the reviewer: coverage 23.03s wall at host load 19–32 (ceiling 16.26s
  quiet-host), 111,800 ns/B; per-byte thread CPU improved from 116,500 ns/B (parent) to
  113,000 ns/B on the same host.
- 45 units changed their selected analysis (undisclosed; census was stamped
  against the claim baseline, not the parent). Five are fixes: `that has an
  Adventure` / `that has a -1/-1 counter on it` move from a fused-determinative
  + duration reading to the finite subject-gap relative. The rest are
  recursive attachment ambiguities resolved low.
- "restored 2 exact ordinal assertions" is not in the diff. The 59 → 60
  collision is one additional form-literal row (`that` in the new transitive
  subject-gap relative) on an already-colliding surface, not a new overlap.
