---
needs: []
---
Give `Reference::Coalesce` a counted fixture again. The noncanon-wc99
graduation dropped the only canon/testing/builtin occurrence, and the
no-dead-grammar gate now passes it on an allowlist exemption
(no_dead_grammar.rs) whose own text names the fix: author a canon or testing
Rhystic-toll-style card ("unless that player pays …" with a coalesced payer)
that exercises Coalesce end-to-end (parse → render → engine), then drop the
exemption entry.
