---
needs: [english-v2-open-grammar-environment]
---
**Sweep the Plan 03 landing-review findings that need no new design.**
Each item as cited by the 2026-08-20 independent review; if an item turns
out to require design (new codegen shapes, contract changes), STOP and
report rather than improvising — it may belong to the lexeme-tier ticket
instead.

1. Close the two handwritten terminal mirror switches: the exhaustive
   `Lexical` match in the scanner's bound-terminal dispatch and the xtask
   diagnostic `TerminalClass` DTO match. Target property: adding a
   terminal declaration requires no handwritten edit outside the
   declaration. If either closure demands new emitter design, STOP.
2. Drop the stale `#[allow(dead_code, reason = "render claim collection
   lands in this task")]` — the task landed; the reason is false.
3. Tighten the owner-size assertions from inequalities to equalities on
   the actual measured sizes, so a size regression fails instead of
   passing silently under the slack.
4. Rename the parser-environment synthetic fixtures to identifiers that
   are genuinely absent from the official catalogs AND the stub registries
   (not Scry/Sprite/Charge), give the subtype and counter-kind fixtures
   scan/render round-trip depth rather than existence-only assertions, and
   run the open-declaration verb round-trip tests on synthetic rows rather
   than production rows — the novelty requirement exists to prove the
   environment reads only its rows.
5. Replace the owner-template `unreachable!` fallback with the typed
   internal failure the ownership contract requires for impossible
   states.
6. Move the Parse and Materialize one-run counters inside the functions
   they measure (Specificity and Ranking already are), so the one-run
   invariant is enforced at the callee, not trusted at one call site.

From the 2026-08-20 lexeme-morphology landing review (same
no-new-design class):

7. Extend the plan-derived audit cross-check beyond the lexeme tier: the
   `expected_lexeme_terminal_items` filter covers `TerminalKind::Lexeme`
   only, though the sealed projection also carries the Vocab branch —
   every terminal kind should flow plan → audit.
8. Derive the closed-lexeme guard patterns from the declaration set
   instead of the `ends_with("Lexeme")` name convention (both the
   identifier finder and the two-segment owner check) — a lexeme declared
   under another name currently escapes both silently.
9. Replace the positional `index == 3` emission of the
   morphology-irregulars block in the text report with a keyed lookup.
10. Drop the stale `#[allow(dead_code)]` on `LexemePlan::morphology`
    (consumed by emit/terminal.rs) and fix the policy comment that says
    "recipes stay unavailable" where it means the plurals.

Acceptance: all suites green; corpus gates byte-identical (32,285 / 48 /
0 / 0); clippy `-D warnings` clean; no public API changes. Standard
constraints apply.
