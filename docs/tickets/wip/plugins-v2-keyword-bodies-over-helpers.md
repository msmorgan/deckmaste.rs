---
needs: []
---
**Every keyword declaration body is written over helper macros, the way a card
is written.** `keyword_actions/amass.ron` is the model. Standard constraints
apply.

Everything below is a WORKING decision from the 2026-10-04 design session, not
a ruling: the semantics_v2 design is in flux and these are expected to move.
Do not promote them into the ADR as dated rulings.

## The constraint

A re-spelling leaves the expanded term identical. Anything that changes a term
is a separate landing (see Follow-ups).

## Working decisions

1. **Scope.** The 136 non-trivial bodies (41 Keyword Actions, 95 Keyword
   Abilities), the 100 empty Keyword Ability bodies, and the counter and type
   declarations that write raw conferrals. Helper-macro bodies are a later
   pass. No new bodies are written for bodyless or STOP-blocked declarations.
2. **The declaration builds its own wrapper.** A Keyword Ability's body is the
   list of abilities; the `Keyword(label, params, body)` node comes from the
   declaration (label from the name, params forwarded from the declared types).
   `champion`, `enchant`, `companion`, `foretell` and `prototype` carry
   `keyword_params: []` and stay as they are. A Keyword Action's body is the
   instruction; the `Enact` node comes from the declaration, with an `agent:`
   field, and `deed: None` on the six that have no wrapper.
3. **Dialect.** Lowercase macros and native values only. Positional calls;
   named only where a middle default must be skipped, and phrasing helpers are
   reordered so defaulted parameters trail. A vocabulary-free ratchet test
   forbids expression constructors in keyword bodies, with an allowlist that
   may only shrink.
4. **Helpers.** Named for the Oracle phrase, in the form the card prints:
   `may`, `enters`, `dies`, `gets`, `insteadOf`, `mayCastFor`, `youControl`,
   `controlledBy`, `defendingPlayer`, `chooseOne`, `returnToHand`,
   `cantBeBlockedBy`, `keywordCostPaid`, `atNext`, `beginningOfYour`, and the
   smaller ones from the survey. Each writes the agent today's term has,
   explicitly and first. Names are proposals until the owner has seen them in
   bodies.
5. **Lean.** RON names are not bound by Lean keywords: the `by_`, `while_`,
   `from_` and `as_` parameters lose their underscores. `Macros.lean` is not
   edited here.
6. **Reader.** `[Block]` reads as `[Core(Block)]`. Bare keyword names
   (`hasKeyword(shadow)`), bare designation names and an `exchange` alias
   wait: a keyword's term has to be built from its label, and `denoted_by`
   today only projects a term already inside the node.
7. **Clean-up.** `plusOnePlusOne` and `minusOneMinusOne` retire in favour of
   the counter declarations. The helper parameters that refuse `None` are
   retyped so they can be omitted.
8. **Comments.** File headers stay verbatim. Inline quotes only above the
   parts of a multi-sentence body, taken from the CR snapshot.

## Proof

`cargo xtask expansions` before and after: `diff -r` may show new helper files
but no changed or missing file, and the skipped list must match. Plus
`cargo xtask lean-check`, `cargo xtask facts check`, and the gate.

## Landings

1. The expansions command, the declaration wrappers, helpers and defaults, the
   two reader changes, the ratchet test.
2. The Keyword Action bodies.
3. The Keyword Ability bodies.

## Follow-ups

- `semantics-v2-action-agents`: which actions take an agent at all.
- `plugins-v2-keyword-body-defects`: what the survey found that is not a
  re-spelling.
- `semantics-v2-counter-kind-is-a-name`.

## Landing record

Measured on change `sksxppymstuk` (2026-10-04). Landing 1 (the expansions
command, declaration wrappers, helper vocabulary, `Deed::Core` injection,
ratchet test) integrated earlier the same day from an ad-hoc workspace; this
record covers the whole ticket.

**Proof.**
- `cargo xtask expansions`: every re-spelling step compared equal to the
  baseline taken before it, including the skipped list. Against the baseline
  after the last term change the final tree differs only by the new
  `predicates/cardIn` declaration.
- `lean/scripts/build` passes; `cargo xtask lean-check`: canon 122/122, testing
  2/2; `cargo xtask facts check`: both files up to date, never regenerated.
- Gate: `cargo test -p deckmaste_construction_core -p deckmaste_lexical_source
  -p deckmaste_semantics_v2 -p deckmaste_construction_v3 -p deckmaste_english_v3
  -p xtask` — 1099 passed, 0 failed, 1 ignored (the pre-existing census test).
- `cargo xtask cite check`: 0 non-compliant, 0 stale.

**Result.** 39 Keyword Action and 95 Keyword Ability bodies re-spelled; the 95
empty ability bodies have no `body:` at all. The two families are 4,368 lines,
from about 8,400. The ratchet allowlist is 8 files, from 135: `search`,
`shuffle`, `vote` (identity bodies), `meld` (a move with riders), `auraSwap`
(no `exchange` alias), `demonstrate`, `transfigure`, `transmute` (the
most-recent-binding pronoun).

**Tests.** Re-spelled: 11 Rust fixtures and 121 Lean pin and bench sites, each
keeping its subject and asserted outcome. Added: 10 Rust tests and 4 canon
cards (Thoughtseize, Hymn to Tourach, Stormbind, Burglar Rat). Removed: 0.
Ignored: 0. Restored: 0.

**Deviations and additions.** The ticket says a term change is a separate
landing. On the owner's direction during the session, two went in here, each
as its own change with a normalised expansions diff showing nothing else moved:
- `Move(subject, from, to, riders)` with a required origin and a `wherever`
  zone expression; 66 declarations gained the field.
- `discard(cards)` with no agent; 7 declarations' Discard deeds lost theirs.
Also beyond the letter: `choose` names its chooser first; `selectAtRandom`;
`move(subject, from, to)`; an origin on `exileBy`, `returnToHand`,
`returnToBattlefield` and `putOntoBattlefieldTappedAttacking`; `youOwn`,
`ownedBy`, `cardIn`; the facts generator reads the built term through the
plugin instead of the source spelling. `plusOnePlusOne` and
`minusOneMinusOne` are gone.

**STOPs.**
- Dropping discard's agent stopped on distribution ("each opponent discards a
  card"). Resolved by the owner's model: an inline indefinite implies its own
  selection [CR#701.9b]; distributed discards are a distributed `choose` then
  one discard, as an interim spelling.
- Letting a cost be a sequence stopped because `badSequentialCost` would flip.
  Dropped; no cost pin changed.
- The converted ability bodies broke `cargo xtask facts`, which read source
  spellings. Fixed in the generator; the generated files did not change.
- Bare keyword names were not built (working decision 6).

**Not done from the agreed scope.** The counter and type declarations that
write raw conferrals: `types/` has no expansions coverage, so a re-spelling
could not be proven identical. Routed with the rest of the residue to
`plugins-v2-keyword-body-defects`; the agent questions are in
`semantics-v2-action-agents`.

**Glossary gaps.** The Game Model glossary defines neither the zone-expression
vocabulary (`wherever`) nor the agent of an action.
