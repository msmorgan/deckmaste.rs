Replace the emit-layer if-let lookup cascades with resolve-once terminal
classification. render.rs (80 if-lets, 12 else-if-let chains), validate.rs
(82), and build.rs (35) each re-discover which table owns a terminal name
by probing find_vocab / find_signed_decimal / find_unsigned_number /
find_declaration_determinative / find_binding in sequence, with
near-duplicate arm bodies. Classify every terminal into one TerminalKind
sum at plan-validation time; emit sites become exhaustive matches so a new
declaration kind is a missing-arm compile error instead of a silent
fall-through (the mechanism that let the CounterKind/Designation consumer
gap hide). Pure internal refactor: generated output byte-identical
(pin via the expansion determinism test), gates unchanged. Standard
constraints apply.

## Landing record (coordinator, from terminal-kind-landing-review.md, 2026-09-02)

Executor wrote none. render.rs migrated onto the pre-existing AtomTerminal
sum (gained a Lexeme payload; two dead lookups removed): `find_*` probes
63 -> 0, `else if let` 13 -> 2 (neither on terminals), `TerminalPlan::`
15 -> 1; all 27 terminal matches exhaustive; 360 tests before/after;
gates green at 15,934. Coordinator premise correction: the ticket's
validate.rs (82) / build.rs (35) figures were raw `if let` counts, not
lookup cascades — both files had zero `find_*` probes; the executor
correctly left them alone but did not flag the premise. Byte-identical
output was asserted via SHA but emission is not reproducible
(construction-core-deterministic-emission); equivalence was established
by the review via stable-arm comparison instead.
