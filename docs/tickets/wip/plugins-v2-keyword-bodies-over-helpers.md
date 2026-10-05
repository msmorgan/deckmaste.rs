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
