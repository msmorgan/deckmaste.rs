---
needs: []
---
[design] **Two sentences in the semantics-v2 contract are narrower than the
direction that has since been settled. This ticket proposes edits; it does not
prescribe them.** Claiming it means opening the conversation with the user, who
decides whether either edit lands and in what words.

Authority for the direction:
[The kind index joins; union marking is spelling](../../decisions/kind-index-joins-union-marking-is-spelling.md).
Target: [Semantics v2](../../decisions/semantics-v2.md), which is a draft that
"moves freely with the design discussion" — so these are in-scope amendments, not
a re-litigation.

## Proposal 1 — §6, "The constructor/macro boundary"

**What it says now.** The section fixes top-level constructors as the engine
primitive basis, then names exactly one class of thing that lives above it:
"Keyword actions and keyword abilities are phrase-shaped macros over the basis,
mirroring the real definitions' name, params, and body."

**Why it is now narrow.** The settled direction says the same thing about
**common phrasings generally**, not only about rules-minted keyword words: "any
target" [CR#115.4] is a function expanding to a joined-kind term, and phrasing
knowledge that would otherwise become a core constructor goes into the card
language or the spelling declarations instead. A reader applying §6 as written
would conclude that a phrasing which is not a keyword has no home above the
basis, which is the error the direction was written to correct.

**Proposed shape of the edit** (wording is the user's): widen the sentence from
"keyword actions and keyword abilities" to "keyword actions, keyword abilities,
and common phrasings generally", and add the rank rule the direction states — a
core shape is judged by lowerability and by its algebra, never by surface
affinity; a meta construct is legitimate in the core; phrasing facts belong to
the card language or the spelling declarations. The existing sentences about
mirroring the real definitions' name/params/body, the `Composite` tag, and the
whittling-candidate rule are unaffected.

## Proposal 2 — §3, "Context data: surface projections and fold-state only"

**What it says now.** "The record is kind-indexed: a binding can record only the
data its kind can have (an object's head type and zone; a player or quality has
neither), so an ill-sorted binding — 'a player in your hand' — is unrepresentable
rather than merely unused."

**Why it is now incomplete.** Under the settled direction there is a third case
the sentence does not cover: a **joined** kind. The sentence enumerates object
and player-or-quality and stops, so it does not say what a binding at a joined
kind may record — and that is exactly the question the direction leaves open for
execution (whether the join is flat or pair-carrying, and whether the binding
carries the antecedent's kind pair). The measured fact in play is that the
demonstrative echo copies its antecedent's kind pair 33 of 33 times, which is
representable only if the record can hold it.

**Proposed shape of the edit** (wording is the user's, and the content depends on
a decision not yet taken): extend the sentence with the joined-kind case — a
binding at a joined kind records the projections its kind admits, which are the
meet of its members' (no zone and no type where the members disagree), plus
whatever the join itself carries. If the join's shape is still undecided when
this is taken, the honest edit is one sentence saying the joined case is open and
naming where it will be settled, rather than a guess that reads as contract.

## Consumption boundary

`docs/decisions/semantics-v2.md` only. Do not edit
`kind-index-joins-union-marking-is-spelling.md` — it is settled, and its own
reference list is maintained separately.

## Acceptance

- The user has seen both proposals and each has landed, been reworded, or been
  declined — with the decline recorded if that is the outcome.
- No other section of the contract is touched in passing.

Standard constraints apply.

## Confirmed decisions (2026-09-06)

The design interview is complete and the user confirmed the recommended edits:
common phrasings belong above the primitive basis alongside keyword macros;
the joined-kind binding payload stays explicitly open. No new execution
representation is selected.

## Landing record

Change `vnzynvvxykypvtmunknpntuyvtxukosw`, lock `covered` 20,254 (2026-09-06).

### PROVE

The ADR diff touches only the two authorized sections: §6 widens the macro
vocabulary and states the lowerability/algebra criterion for core shapes; §3
explicitly leaves joined-kind payloads open and points to the existing
execution-shape discussion. The settled kind-index ADR is unchanged.

This is documentation only. No card, construction, test, or licensing guard
changed; tests restored 0, re-spelled 0, ignored 0, added 0, removed 0. No
coverage identity was added or lost, and no English structural law changed.
The existing target of the new Markdown link was checked. Citation checks
report zero noncompliant strings and zero stale references; the diff introduces
no citation sites. The ticket graph has no duplicates, cycles, or dangling
dependencies.

### DISCLOSE

The user approved the content during the design interview; there is no pending
design approval or implicit decision about the shape of a join. No glossary
term was introduced.

Addition beyond the ADR ticket's consumption boundary: the planned
`lean-card-soundness-gate` ticket records the user's explicit deferral until
`semantics_v2` exists. This is a scheduling correction requested in the same
interview, not emitter implementation or completion. Its status and dependency
edges are unchanged. No Lean/Rust adapter or crosswalk was added.

### REPORT

Provenance: `vnzynvvxykypvtmunknpntuyvtxukosw`, lock `covered` 20,254.
The coverage lock and declaration data are unchanged. English selection,
construction, licensing, homograph, and form-literal/vocabulary inventories
were not remeasured. No coverage performance run, host-load/worker telemetry,
or ns/B measurement is claimed for this documentation change.
