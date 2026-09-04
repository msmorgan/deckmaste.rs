---
needs: [english-v2-adjunct-licence-removal, english-v2-role-preemption-depth, english-v2-attachment-class-declared]
---
DESIGN TICKET — needs a design brief before implementation; do not claim
directly. Authority: rewrite ADR "Ruling: adjunct licences removed; attachment
misselection is a recorded class (2026-09-03)".

Right-peripheral adjunct attachment is not grammar-decidable (Seedborn Muse:
`during each other player's untap step` under `control` vs under `untap`).
Today the ranker picks low attachment and the AST records a single site, so
semantics gets a possibly-wrong tree with no signal. Replace the choice with
underspecification: (1) a ranker collapse — two candidates whose trees differ
ONLY in the attachment site of one right-peripheral adjunct become one packed
candidate; (2) the adjunct node is hoisted to the highest admissible site and
carries the set of admissible lower sites as a value (a derived value like the
Zero determiner, not a category); (3) an attachment-only tie is one candidate
in the selection census, so it is not a STOP, and any other tie remains one;
(4) consumers (semantics workbench, xtask diagnostic display, render) read the
slot; render is unaffected because every site renders the same bytes.

Design questions the brief must pin: the tree-equality-modulo-attachment test
(structural, in the materializer, not a string compare); interaction with the
"selected frame roles preempt postmodifiers" amendment (a declared frame role
is not an admissible site — that tie is already eliminated); nesting (an
adjunct inside a collapsed adjunct); the census line for packed candidates
(report count of packed units so the class stays measured); and the coverage
lock (text-keyed, so unchanged). Coverage must not drop. Standard constraints
apply.

2026-09-04, routed from the `english-v2-with-preposition` landing review:
typing `with` as a `PostmodifierOnly` preposition made two measured instance
families of this class visible on trunk, both to be covered by the
underspecification the brief pins.

- Free `with` that the sentence means as a verb-level adjunct has no adjunct
  site and attaches to the object nominal instead: the suspend template
  "Exile … with three time counters on it" (Arc Blade, Chronomantic Escape,
  Cyclical Evolution, Doom's Time Platform, Festering March, Inspiring
  Refrain, Reality Strobe, Suspended Sentence, Taigam, Master Opportunist) and
  the attack instrument "attacks … with one or more creatures" (Curse of
  Chaos, Curse of Shallow Graves, Oath of Kaya, Rigo, Streetwise Mentor).
  Whether the fix is underspecification here or an adjunct-capable `With` is
  the coordinator's call; the class is the same either way.
- Postmodifier scope over a coordination is the same choice one level down.
  Vicious Rivalry ("Destroy all artifacts and creatures with mana value X or
  less") now selects the member-local reading where the determiner-shared one
  is correct; the same re-bracket is a correction for seventeen other
  identities (the Hurricane family, the token lists, Eaten by Spiders), so the
  ranker cannot be turned back wholesale — the two readings need the packed
  candidate this ticket describes.

2026-09-04, routed from the `english-v2-possessive-nominal-form-collapse` (R10)
landing STOP, by coordinator ruling: the possessive coordination arm is struck
from that ticket and deferred here.

- The arm. `possessed_reference` (a possessive determiner plus a Nominal) has
  no coordination arm, so `your creature and artifact` has no derivation. Its
  sibling `genitive_determiner_coordination_reference` is the shape the arm
  would copy. R10 removed that construction's `coordination.number is Singular`
  requirement, so the genitive side now licenses `target player's creatures and
  artifacts`; the possessive side is what remains open.
- The two sentences. `Destroy your creature and artifact.` is the sentence the
  arm exists to admit. `Whenever you cast your first instant or sorcery spell
  each turn, …` (Aquatic Alchemist // Bubble Up, identity
  `eabbc2128de743c6555fa404e3102827062681275d373db6f3adbed32980abbb`) is the
  sentence the arm breaks.
- The two analyses of the second sentence. Correct, and what trunk selects
  today: `PossessedReference -> ModifiedSingularNominal ->
  AttributiveAdjectiveModifier(first) -> OrSharedHeadModifier(instant or
  sorcery) -> SingularHead(spell)`, i.e. `your first [instant or sorcery]
  spell`. What the probed arm made specificity select instead:
  `PossessedCoordinationReference -> OrNominalCoordination ->
  ModifiedSingularCoordinationMember(first instant) +
  ModifiedSingularCoordinationMember(sorcery spell)`, i.e. `your [first
  instant] or [sorcery spell]`. Both bracketings are English; neither is
  excluded by a Nominal Form or determiner-licence fact at that site, and R10's
  fences forbid a dominance edge, an exception, or a narrowed value list to
  choose between them. The missing piece is the general
  bracketing-preference/attachment device this ticket pins, so the arm lands
  with it, not before it.
