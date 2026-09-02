# workbench-mixed-head-disjunction

The mixed HEAD/ADJECTIVE disjunction — an `Or` of head-bearing arms with an
adjectival one ("artifacts, Sagas, and/or legendary"; "legendary or
enchantment"), refused by `parallelDisjuncts` (`hasHead (HasSupertype _)` is
False). Re-measured by the bucket round (2026-09-02): **16 printed lines /
16 cards independent of keywords** (Acclaimed Contender, Tale's End, Sazh
Katzroy, Banishing Slash) **plus** storied's 5 carriers via [CR#702.195a] —
21 total. Re-measure at claim.

## As landed (2026-09-02)

### The head-borrowing decision

The adjectival arm borrows the **enclosing noun's** head, never a sibling
arm's. [CR#700.6] states such a union in the rules' own words — historic
"refers to an object that has the legendary supertype, the artifact card
type, or the Saga subtype" — three characteristics of ONE object; a
sibling-borrowing read ("legendary" = "legendary artifact") would drop every
legendary creature the term covers. [CR#702.195a] counts the same union as
"permanents that are artifacts, Sagas, and/or legendary", and its printed
reminder substantivises the adjective outright ("three or more artifacts,
legendaries, and/or Sagas"), which only the enclosing head supports.

So `hasHead (HasSupertype _) = False` was already right; headlessness alone
was never the fault. `parallelDisjuncts` now reads `armPresupposes` — a
headless arm presupposes the type it is said of, an arm that writes its own
head asserts a type and presupposes nothing — and demands zone and
presupposition uniform over every arm alike. `headsUniform` and
`zonesUniform` are gone; `seedsUniform` is the single demand. The rewrite is
conservative on both old branches (all-headed → zone only; all-headless →
today's seed pair) and admits only the mixed case.

### Re-measurement

The bucket round's figures were both wrong, and its four named cards are not
this shape:

- **Acclaimed Contender** ("a Knight, Aura, Equipment, or legendary artifact
  card") and **Sazh Katzroy** ("a Bird or basic land card"): the adjectival
  word rides inside an `And` with a type word, and `hasHead (And ps) =
  hasHeadAny ps`, so `parallelDisjuncts` never refused them.
- **Banishing Slash** ("target artifact, enchantment, or tapped creature"):
  every arm heads; refused by the ZONE half (`HasStatus Tapped` seeds
  battlefield against the bare type arms' nothing) — `badCrossZoneDisjunction`'s
  family, not this ticket's.
- **Tale's End** ("target activated ability, triggered ability, or legendary
  spell"): every arm heads; the open question there is the mixed KIND, which
  is `Joined`'s business.

Actual census (supported/vintage-playable only): **9 printed lines / 9 cards
independent of keywords, plus storied's 9 carriers** — 18 cards.

| bucket (what the adjectival arm is) | line | seat | state |
| --- | --- | --- | --- |
| supertype | storied [CR#702.195a], 9 carriers | gate/threshold count | **landed** |
| colour | Tezzeret's Gatebreaker, "a blue or artifact card" | partitive slice | **landed, whole** |
| colour | Soldevi Adnate, "a black or artifact creature" | cost description | **landed, whole** |
| colour | Mechanized Warfare, "a red or artifact source" | replacement subject | declined |
| supertype | Nashi, Searcher in the Dark, "legendary and/or enchantment cards" | partitive slice | declined |
| supertype | Nashi, Moon's Legacy, "target legendary or Rat card" | target restriction | declined |
| supertype | Monument to Perfection, "a basic, Sphere, or Locus land card" | search description | declined |
| supertype | Banish to Another Universe, "each artifact, legendary, and/or Saga permanent you control" | affinity reminder | declined |
| negation | Vision, Spectral Synthezoid, "a noncreature or Robot spell" | castable description | declined |
| negation | Scarlet Witch, Chaotic Avenger, "a Hero or noncreature spell" | castable description | declined |

The declines are all paid for by machinery outside this region (a damage
replacement with an amount rider; an "any number of" partitive over a milled
group; exile-and-copy; a counted search on a card that then becomes a 9/9;
affinity as a cost; "once during each of your turns … without paying its mana
cost"). The disjunction itself writes in every one of them now.

### Landed

- `Phrase.idr`: `armPresupposes` added; `seedsUniform` reads it;
  `headsUniform` and `zonesUniform` deleted; `parallelDisjuncts` and
  `hasHead`'s doc rewritten.
- `ProofsB.idr`: `badHeadlessDisjunct` ("artifact or attacking") holds and
  its prose now names the demand that survives.
- `Cards.idr`: `storiedEnduringStory` is no longer a fragment — the whole of
  [CR#702.195a] including the threshold and the absence check;
  `tezzeretsGatebreaker` and `soldeviAdnate` are whole cards.

### Remainders

- **All-headless arms disagreeing about presupposition** stays refused, and
  three printed lines want it: Sonar Strike ("target attacking, blocking, or
  tapped creature"), Tetsuo Umezawa ("target tapped or blocking creature"),
  Dire Downdraft ("an attacking or tapped creature"). A combat word
  presupposes a creature where a status word presupposes nothing, and
  `seedsUniform` compares `Nothing` against `Just Creature` as a
  disagreement. Loosening that is the Talion precedent's own question and was
  left alone.
- **`historic` has no predicate.** [CR#700.6]'s term is printed on 57
  supported cards; its reminder "(Artifacts, legendaries, and Sagas are
  historic.)" is a substantive list rather than a mixed coordination, and the
  term itself is a separate ticket.
- Storied's 9 carriers are still not whole cards — the gate is, the bodies
  are their own lines.
