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
