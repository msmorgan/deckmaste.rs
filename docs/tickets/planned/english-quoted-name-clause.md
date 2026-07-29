---
needs: [english-test-structural-assertions]
---
**Parse a quoted token name without treating it as granted rules text.** The
typed-assertion conversion exposed that `Create a token named "A. B" and draw a
card.` currently round-trips only because the whole sentence is recovered. The
old debug-string test merely proved that no `QuotedAbility` appeared anywhere,
so the recovered clause passed while its comment claimed the existing name
machinery handled it.

Lower the quoted text through the existing name-object shape and retain the
ordinary coordinated action outside that name. Do not admit the quoted name as
an embedded or granted ability. Add typed positive and over-fire assertions,
preserve byte-exact rendering, and report the recovery delta. Standard
constraints apply.
