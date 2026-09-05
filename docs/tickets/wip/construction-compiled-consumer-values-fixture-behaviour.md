---
needs: [construction-compiled-consumer-values]
---
Consumer-values fixture asserts values, not behaviour (compiled-consumer-values
landing review M1). The xtask emitter fixture's `NounLexeme` declares no
licence/relationality feature members, so the generated aggregate helpers
(`bare_locative_license`/`relationality` accessors over declared members) are
never emitted there; the fixture's assertions read the transported values but
nothing generated ACTS on them, so a regression in the aggregate helper
emission would pass the fixture. Extend the fixture grammar with a noun
lexeme declaring one member per licence/relationality feature and a
construction whose `require` reads the aggregate helper, then assert a
parse OUTCOME that flips with the declared value (mutation-verify: change the
member's value, the assertion must fail). ~20–40 lines. (Review L3, the
missing performance-advisory line in the landing-record rule, is already fixed
in CLAUDE.md.) Zero grammar change; `expand`
byte-identical. Standard constraints apply.
