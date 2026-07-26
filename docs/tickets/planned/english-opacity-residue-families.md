---
needs: []
---
**Remaining single-word opacity families (post-litaudit census: noun cell
393).** Diagnosed 2026-07-25 (round litaudit, `out/litaudit-plan.md` §1.3/§8,
`litaudit-mechanic-report.md` §11); four distinct classes, each needing its
own treatment — none is a `has_known_word` literal omission:

1. **Zero-exposure literal class, individually unmeasured** — `both`, `half`,
   `rounded`, `rather`, `there's`, `he's`, `she's`, `they're`, `they've`,
   `you've`, `'s` (+ `out`, known-BAD): adding each to
   `OPACITY_RESERVED_LITERALS` needs a per-literal measurement of
   second-order `retain` blast radius (multi-word opaque spans shrinking) and
   existing-test interaction. Known evidence: `out` breaks
   `directional_particle_requires_a_licensed_verb_pair` (legitimate
   opaque-noun fallback for unlicensed verb pairs); `you've` removes two
   out-of-scope dump rows (`surveilled`, `completed`).
2. **Trigger/function words absent from every scanner** — `but` (23), `also`
   (20), `when` (9), `whenever` (5), `longer` (7), `none` (3), `how` (3),
   `yet` (2): honest opacity, the largest single opacity family left (~72
   rows); wants real lexeme/machinery design, not reservation.
3. **Hyphenated compounds** — `face-down` (7), `face-up` (4): single Word
   tokens, related to the queued card_orientation feature-gate debt (wip
   ticket), not to the `face`/`down`/`up` literals.
4. **Numeral opacity** — `four` (1 row): the `Number(Numeral)` path
   (numeral.rs), a different mechanism.
