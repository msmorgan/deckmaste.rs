---
needs: [engine-transform, engine-transform-byindex-reread-hardening, engine-transform-current-face-name-loyalty]
---
Day/night game state, daybound/nightbound transforms (~236 cards). Tracks the
day/night designation and fires the transform event when the condition changes
at end of turn.

This is the first consumer that makes back-up double-faced permanents reachable in
production, so the two latent gaps the `engine-transform` whole-branch review
deferred are hard prerequisites: `engine-transform-byindex-reread-hardening` (a
mid-resolution transform can panic an un-hardened by-index ability re-read) and
`engine-transform-current-face-name-loyalty` (legend-rule name / named-predicate /
loyalty reads still take the front face).
