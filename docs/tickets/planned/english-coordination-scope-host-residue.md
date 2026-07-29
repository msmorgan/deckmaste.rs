---
needs: [english-coordination-agrees-audit]
design: true
---
**Model operator scope and the current coordination member's host.** Tightening
the generic agreement fallbacks exposed 13 render-identical but structurally
unsound trees. Do not restore them with a broader agreement tuple or surface
string gate.

- Shared auxiliary scope: Fog on the Barrow-Downs, Observed Stasis, Immovable
  Rod, and Boros Battleshaper flatten `can't attack or block`; Flaming Gambit
  flattens `may choose ... and have ...`; Osseous Sticktwister flattens
  `didn't sacrifice ... or discard ...`. Decide whether a modal/auxiliary owns
  a coordinated predicate expression or whether member metadata records
  ellipsis. Audit the already-supported bare witness `Enchanted creature can't
  attack or block`, whose present tree likewise makes only `attack` deontic and
  leaves `block` as an imperative member through `hosted_imperative`.
- Current-member host state: Krydle of Baldur's Gate and Thunderfoot Baloth
  need a subjectless member to attach to the immediately preceding
  subject-bearing member, not inherit the first clause's agreement. Burning
  Prophet and Consuming Ferocity instead switch from a third-person statement
  to a controller-directed instruction after `then`. Titania, Voice of Gaea
  combines an existential with a subject-bearing condition before that switch.
- Attachment rather than coordination: Visions uses preverbal `then` inside
  `may then have`; Vazi, Keen Negotiator coordinates infinitives under one `to`
  marker (`to cast it or activate it`).

The design must make scope and host choice explicit in the AST, preserve the
surface distinction between repeated and elided auxiliaries, and add typed
assertions rather than render-only tests.
