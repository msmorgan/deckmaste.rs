---
needs: []
---
`parse_return_to_hand` (`crates/deckmaste_migrations/src/parsers/effect.rs`,
~line 1358) accepts a determiner-led chosen (non-target) bounce with NO
controller restriction. After stripping " to its owner's hand", the non-`target `
branch checks only that the determiner is `a` / `an` / `another` / `other`, then
calls `object_target_filter(subject)` without requiring "you control". So a
phrase like "Return an artifact to its owner's hand." graduates as
`With(ChooseOne(And([Permanent, Type(Artifact)])), Move(That(Permanent), Hand))`
— a non-targeted chosen bounce over EVERY controller's matching permanents,
dropping all targeting semantics (hexproof / protection / shroud no longer
interact). This is over-permissive vs `done/bounce-followups` §2, which
specified only the "you control" chosen-bounce form.

Currently LATENT: canon has zero bounce cards and every parser test carries the
"you control" restriction, so nothing triggers it today. But it is a correctness
landmine — a future determiner-led bounce card would silently mis-parse to an
over-broad, un-targeted effect.

Fix: require a controller restriction on the determiner-led (non-target) chosen
bounce path — decline (stay `Unparsed`, held for human review) unless the
subject carries "you control", matching the `done/bounce-followups` §2 contract.
A genuine "target"-led bounce still routes to the targeted `Move(Target(0),
Hand)` arm and is unaffected.

Verify: add a parser test that "Return an artifact to its owner's hand."
declines (no controller restriction); the existing "Return a land you control …"
/ "… another creature you control …" tests still parse to the chosen bounce.

(Surfaced by the code review on `engine-block-legality-query`; the finding is in
the bounce-followups parse work, not that feature.)
