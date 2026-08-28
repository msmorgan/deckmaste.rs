---
needs: []
---
# Counted anaphora narrowings: admit the bare-"it" residue without preference

**Ruling (user, 2026-08-27): clause recency is permanently OFF the table.** A
corpus audit (5,153 candidate sites over supported cards) plus two independent
cleanroom consultations converged: every positional/preference rule
(nearest-wins, subject-continuity, mention-recency, clause-age) silently
misbinds real supported lines — a term that typechecks while meaning the wrong
thing, which no gate can catch. The correct program is the house doctrine
generalized: **counted uniqueness over a construction-proven smaller candidate
set; count survivors, never rank; ties refuse.** `ItAt`/`ItVerbed` were the
first two rows of this family; this ticket is the rest.

The measured residue this addresses is the ~886 same-carrier bare-"it"
refusals (docs/tickets/done/workbench-anaphora-a-bare-it.md); after these
rows, the estimated genuinely-refused remainder is double-digit occurrences
(unverified until each family is built and re-measured). A refused bare "it"
never blocks a card — the explicit spelling (`That w`/`TheVerbed`/`This`)
always remains.

## The rows

Each row is a `countBy` fold with the ADR's per-anaphor proof cost
(`countXIsFold`, prefix-resolution lemmas); nothing reorders `bs`; no new
index on `Effect`.

1. **De-pronominalization templates (not reads at all).** Where the printed
   "it" is constructor spelling, no pronoun read exists: copy exceptions
   ([CR#707.9]; `CopyExcept` already does this), "counters on it"
   (`CounterCompare`), "to its owner's [zone]" (`Bare` scope), plus the
   missing templates: a shared-subject coordination form (retiring the
   `itAsPermanent` workaround; "gets +4/+4 and gains trample" is VP
   coordination, not anaphora), a `controllerSacrifices` template ("X's
   controller sacrifices it", 12/12 corpus-consistent), and the subject-bound
   "deals damage equal to its power" (204/204 subject-bound, incl. Legolas,
   Master Archer).
2. **`ItRole` — role-indexed uniqueness.** `countRole r bs = 1` where the
   enclosing construction proves a role identity. First row: the
   damage-replacement family — "If X would deal damage…, IT deals … instead"
   is the REPLACED EVENT'S SOURCE by [CR#614.6] ("A modified event occurs
   instead"; verified verbatim). Census: 60 supported faces, 44 with an
   object-compatible nearer mention, 0 role reversals. The construction marks
   the source binding; stale marks must be cleared before typing the
   replacement (mark leakage across branches/quotes is THE implementation
   risk — refuse on multiple marks, never select).
   Covers: Blind Fury, City on Fire, Inquisitor's Flail.
3. **Co-argument exclusion (Principle B).** A bare "it" in a verb's object
   slot excludes that verb's own subject/co-argument: count uniqueness over
   the prefix minus the co-argument's delta (and minus `SelfD` when the
   co-argument is `This`). Probed: 0 supported lines where a non-reflexive
   object corefers with its co-argument. Applies PER CONSTRUCTOR, not per
   sentence (Bruna's nested description must not be excluded).
   Covers: attach-to-it (63 cards), forced-block ("target creature blocks
   it" — Fighter Class, Feral Contest, Tower Above).
4. **Previous-sibling delta.** In `Sequentially`/coordination, count
   uniqueness over the immediately preceding sibling's OWN delta first; fall
   back to the bare gate when it holds no singular object.
   Covers: "tap target creature and put a stun counter on it" (Stunning
   Shot), "create a token. Put a counter on it".
5. **Producer-label extensions.** `verbFacts` rows for Return, Untap,
   GainControl; a `TokenOrigin` narrowing (`ItToken` over the origin already
   on the payload). Corpus: "create … token. It …" = token 70/70; Return/Put/
   Untap continuations all producer-bound.
6. **Everything else stays refused** and is authored explicitly: the
   genuinely ambiguous pairs (Acolyte of the Inferno vs Basalt Golem, Gaze of
   Pain, the Tamiyo shapes) are resolved in English only by play knowledge —
   refusing the pronoun spelling is correct behavior, recorded per family.

## Pins

- NO preference or ranking anywhere; every admission is a counted gate over a
  provably smaller set; ties refuse. This is the ruling, not a default.
- Exclude a candidate only from a rules- or construction-entailed fact.
- Only the macro owning a construction may emit a role-scoped pronoun.
- ProofsAnaphora's existing lemmas stay; each new gate pays its own fold and
  prefix lemmas ([the ADR's per-anaphor cost]).
- The enters-as-copy constructor obligation (copy-family ticket) stands:
  copy sources are never announced.
- Corpus tallies above from the 2026-08-27 audit + consultations: the CR
  quotes and the damage-family scale are verified; per-family counts are to
  be RE-MEASURED at claim before sizing any row (evidence lives in
  session-local scratch, deliberately not carried — the counts in this
  ticket are the claim to re-verify, not the authority).

## Acceptance

- Each row lands with its measured family re-counted, at least one whole-card
  bench witness, and the refusing shapes still refusing (`badTwoChoosers…`
  siblings and the Principle-B zero probed).
- Blind Fury, Stunning Shot, an attach carrier and Fighter Class bench.
- The residue is re-measured after all rows and recorded with named families.
- `idris/scripts/build` PASS; ProofsAnaphora prefix lemmas intact.

Standard constraints apply.

## As landed — round 1 of 2 (rows 1, 5, 6)

Rows 2 (`ItRole`), 3 (co-argument exclusion) and 4 (previous-sibling
delta) are NOT in this round and nothing here scaffolds for them.
`idris/scripts/build`: **23/23, 0 errors, 0 warnings**; `cite check
--list-noncompliant` empty; `cite check` 0 stale; `cite bless` registered
no new rule (all 32 audited sites read against their rule text).

### Re-measured counts (2026-08-27, `select(.supported)`, 32,568 faces)

| Family | Ticket claimed | Re-measured | Verdict |
|---|---|---|---|
| shared-subject VP coordination | (unstated) | 922 faces — 435 "gets [pt] and gains [ab]", 487 "gets [pt] and has [ab]" | new |
| "[X]'s controller sacrifices it" | 12/12 | **16/16** faces, pronoun = possessor in all 16 | ticket low by 4 |
| "deals damage … equal to its [c]" | 204/204 | **275/275** faces source-bound (270 power, 3 mana value, 2 toughness); the 15 recipient-first lines ("deals damage to itself equal to its power") are source-bound too | ticket low by 71 |
| "Return … to the battlefield. It …" | (unstated) | 93/93 faces producer-bound | new |
| "Untap …. It …" | (unstated) | 99/99 faces producer-bound | new |
| "Gain control of …. Untap it" | (unstated) | 19/19 faces | new |
| "create … token. It …" | 70/70 | **170/170** faces token-bound | ticket low by 100 |

### Row 1 — de-pronominalization templates

Three constructions where the printed "it" is the construction's own
earlier argument, so no gate exists to widen. Each is the ADR's
clause 4, not clause 3; each is witnessed in `ProofsAnaphora` §4's new
template block (`controllerSacrificesReadsNoPrefix`,
`dealDamageOwnReadsNoPrefix`, `ofSubjectReadsNoPrefix`) as writable
whatever else the prefix holds.

- **`StaticEffect.OfSubject`** + `Shared.SubjectVP`/`SubjectVPs` +
  `vpIntro`/`vpsIntro`/`vpOk`/`vpsOk` (`Effect.idr`), macro
  `Macros.sharedSubject`. Two arms, `VPGets` and `VPGains` (the latter
  spells both "gains" and "has"). Parts thread left to right on
  `StaticParts`' model [CR#608.2c]; the per-part obligations are folded
  and asked of the one shared subject, which needed `grantSubjectOk`
  split into the index-free `grantSubjectFits`. §5 gains
  `subjectVPsThreadPrefix`, `vpIntroIsDeltaThenPrefix`,
  `vpGainsMintsNothing`.
  **`itAsPermanent` retired at Ghor-Clan Rampager.** The macro STAYS:
  `Macros.sacrificeIt` is its other call site and genuinely needs the
  carrier-scoped read.
- **`Effect.ControllerSacrifices`** — "[n]'s controller sacrifices it".
  [CR#701.21a] makes the possessive subject and the object one referent
  by the act's own definition. Announces the controller (Arcum Dagsson's
  "That player" reads it back) and stamps/re-zones the patient exactly as
  the labeled move does.
- **`Effect.DealDamageOwn`** — "[src] deals damage equal to its [c] to
  [to]". The possessor is the source slot [CR#120.1]. A sibling row and
  NOT a widening of `DealDamage`: a written amount, an anaphoric one, and
  a characteristic of something other than the source all stay there.
  (Deviation from the obvious design: turning `DealDamage`'s amount slot
  into a `DamageAmount` sum would have touched all 178 existing call
  sites for no semantic gain.)

`CopyExcept` / `CounterCompare` / `Bare` scope untouched, as briefed.

### Row 5 — producer-label extensions

- **`verbFacts` rows** for `"Untap"` ([CR#701.26b]; participle
  "untapped", patient Object, zone Battlefield, no destination — `Tap`'s
  row read the other way), `"Return"` ([CR#701.1] + [CR#400.1]; no
  participle and no zones, on `"Put"`'s ground) and `"GainControl"`
  ([CR#613.1b],[CR#110.2]; battlefield patient, no destination, no
  participle). Macros `Macros.untap`, `Macros.returnTo`,
  `Macros.returnToBattlefield`.
- **`staticIntro (GainsControl who what)` now stamps** —
  `stampIntro (Just "GainControl") what` rather than `selfSubjIntro
  what`. The control change moves nothing, so there is no `Move` for
  `Enact` to label; the constructor names its own act, on `Search`'s
  precedent of a row that writes its own stamp. Only the stamp field
  changes on the binding, so nothing that read it before reads
  differently. `Macros.gainControl`/`gainsControl` had to move their
  duration slot to `staticIntro` (it was written as `selfSubjIntro`).
- **`Noun.ItToken`** + `Words.itTokenReaches`/`countItToken`/
  `provOfItToken`/`zoneOfItToken`/`tyOfItToken`/`payloadOrig`,
  `Phrase.setZoneItToken`, macro `Macros.itAsToken`. The origin-scoped
  narrowing: the create clause already writes `Just TokenOrigin` on its
  own mention [CR#111.1,111.2], and this counts over that. It is the
  OBJECT reading of a create clause where `TokenAsThose` is the reading
  of the DEFINITION [CR#111.3]; different counts, they do not compose.
  Proof cost paid: `countItTokenIsFold` (§2),
  `itTokenReadsOnlyPrefix` + `itTokenResolvesInPrefix` (§3).

### Benches

| Witness | Kind | What it proves |
|---|---|---|
| `ghorClanRampager` | Ability (rewritten) | `OfSubject`; `itAsPermanent` retired |
| `aimHigh` | whole Card | `ItVerbed "Untap"` **and** `OfSubject` |
| `hijack` | whole Card | `ItVerbed "GainControl"` then `ItVerbed "Untap"` |
| `aggressiveInstinct` | whole Card | `DealDamageOwn` |
| `bondOfRevival` | Effect (rewritten) | `ItVerbed "Return"` |
| `arcumDagssonSacrifice` | Effect | `ControllerSacrifices` |
| `harriedDronesmithToken` | Effect | `ItToken` |

### Row 6 — the families that stay refused, and why

Recorded per family. In each, the bare pronoun has two or more
same-carrier candidates and the rules admit both, so a gate that picked
one would be a preference. The explicit spelling (`That w` /
`TheVerbed` / `This`) remains the authoring path in every case.

- **Acolyte of the Inferno vs Basalt Golem.** Identical headers
  ("Whenever this creature becomes blocked by a creature"), opposite
  referents: Acolyte's "it deals 2 damage to that creature" is the
  BLOCKED creature, Basalt Golem's "that creature's controller sacrifices
  it" is the BLOCKER. English resolves this by knowing which of the two
  each effect is about, which is play knowledge and nothing the header
  says. Round 1 removes the pronoun from BOTH clauses structurally
  (Basalt Golem writes `ControllerSacrifices`, Acolyte writes its damage
  source as `This`), but a bare `It` written in either slot still counts
  two and still refuses — correctly.
- **Gaze of Pain.** "…you may choose to have it deal damage equal to its
  power to a target creature. If you do, IT assigns no combat damage this
  turn." The last "it" is the attacker, not the creature just damaged;
  what picks it is knowing that assigning combat damage is the attacker's
  business. `DealDamageOwn` retires the middle "its"; the first and last
  stay bare reads over two battlefield candidates and stay refused.
- **The Tamiyo shapes** (Tamiyo Meets the Story Circle I; Tamiyo,
  Inquisitive Student // Seasoned Scholar [+2]). "Whenever a creature
  attacks you or a planeswalker you control, IT gets -2/-0 until end of
  turn." The header announces both the attacking creature and the
  defending planeswalker; English reads the attacker because a pump is
  aimed at a creature. That is NOT a rules narrowing available here:
  [CR#208.3a] says an effect modifying a noncreature permanent's power or
  toughness "is created even though it doesn't do anything", so the
  planeswalker candidate is rules-meaningful and the pin doctrine forbids
  excluding it. Permanently refused, by design.

### Remainders (round 1's own; not routed as tickets)

- The token self-word: "Sacrifice this token" / "this token gets …" has
  no spelling — `AsType Creature This Nothing` spells "this creature".
  This is why the `ItToken` witness is Harried Dronesmith's line rather
  than Spawning Breath or Make Mischief, both of which would show the
  gate refusing a bare `It` outright.
- `Macros.searchLibraryFor` hardcodes `Search You`, so Arcum Dagsson's
  second sentence ("That player may search THEIR library…") is unbenched
  and its first sentence benches alone.
- `SubjectVP` has two arms. The coordinations the corpus also writes —
  `LosesEveryType` (Nameless Inversion, Ego Erasure) and
  `LosesAllAbilities` — still write `AndAlso` with a pronoun second
  subject. Adding an arm is a row here plus a clause in `vpOk` and
  obliges nothing else.
- The residue re-measurement across ALL rows (ticket acceptance) belongs
  to round 2, after rows 2–4 land.

## As landed — round 2 of 2 (rows 2, 3, 4, and the closing re-measurement)

`idris/scripts/build`: **23/23, 0 errors, 0 warnings** (clean `build/`).
`cite check --list-noncompliant` empty; `cite check` 0 stale over 18,483
citations; `cite bless` registered no new rule; all 11 audited sites read
against their rule text. Every count below re-measured 2026-08-27 over
`data/derived/cards.jsonl` filtered `select(.supported)` = 32,568 faces,
reminder text stripped.

### Row 2 — `ItRole`: the premise is wrong, and nothing lands

**The damage-replacement family carries no pronoun read.** Re-measured:
**60 faces / 62 occurrences** of "If [X] would deal damage …, IT deals …
instead" (the ticket's 60 is exact), of which **42** have an
object-compatible nearer mention in the recipient span and **0** are role
reversals — all 60 distinct sentences read the pronoun as the source, and
[CR#614.6] ("A modified event occurs instead", read verbatim from
`data/rules/cr.txt`) is why.

But the family is already spelled by `StaticEffect.Scales` /
`Redirects` / `RedirectsFrom` / `PreventsFrom`, which write the source
once as `src`/`by` and restate it in the replacement. The printed "it" is
those constructors' own spelling — this is the ticket's ROW 1 shape (a
de-pronominalization), landed long before this ticket, and `ProofsF`'s
`badScaleShiftByThatMuch` already quotes it verbatim. There is no gate to
narrow and no candidate set to count over, so **no `ItRole` constructor
lands**: building one would be a constructor no printed line writes.

Recorded instead as evidence: **Inquisitor's Flail** benches whole, both
of its replacement rows (`Everywhere` scope and a named `AttachHost`
recipient) plus `Equip {2}`, with zero pronoun reads in the term. City on
Fire is the same `Multiplied Tripled` row `fieryEmancipation` already
benches and was not duplicated; Blind Fury's first sentence ("All
creatures lose trample until end of turn") wants a lose-one-named-ability
row that does not exist, and is a remainder below.

### Row 3 — `Noun.ItOtherThan`, the co-argument exclusion

`ItOtherThan : (co : Bindings) -> (rest : Bindings) -> {auto 0 sp : bs =
co ++ rest} -> {auto 0 ok : countOnes Object rest = 1} -> Noun bs Object`.
`It`'s own count asked of a NAMED SEGMENT of the prefix, where the three
existing narrowings ask a per-binding question of the whole of it. The
segment is written as the split equation and never as a position, so
nothing here indexes a slot list; `countBySplit` bounds the narrowed
count by the whole prefix's.

The exclusion is rules-entailed, not linguistic: [CR#509.1a] has the
DEFENDING player choose blockers from among the creatures they control
and, for each, a creature to block that is attacking that player, while
[CR#508.1a] has the ACTIVE player choose attackers from among the
creatures THEY control. A creature therefore never blocks itself.

- **Landing site: the forced-block family**, re-measured at **6 faces**
  (Avalanche Tusker, Feral Contest, Fighter Class, Impetuous Devils,
  Monstrous Step, Tower Above). `Effect.Deontic`'s `DeonticCounterpart`
  is a real constructor and consumes the narrowing directly; the macro
  `Macros.mustBlockIt` owns the segment (`nounDelta n`), on `ItAt`'s
  macro-only ground.
- **Principle-B zero re-probed and holds.** 26 bare-"it" occurrences sit
  in an object slot after a co-argument verb (block / attach / fight /
  deal-damage-to) and in every one the pronoun names the other
  participant, never the co-argument; reflexive coreference is spelled
  "itself" throughout (23 occurrences over 22 faces). **0** supported
  lines corefer a non-reflexive object with its own co-argument.
- **The attach family does NOT land, and the reason is not the anaphor.**
  Re-measured at **65 faces / 66 occurrences** (the ticket's 63 is low by
  2–3; the `bare-it` ticket's 64/61 is the same frame). There is **no
  attach EFFECT constructor in the grammar at all** — `AttachChoice` is
  the as-becomes-attached chooser and `AttachHost` is the noun, and
  neither is "attach [n] to [host]". Per the brief's own instruction the
  narrowing was built where a real constructor can consume it, and the
  missing constructor is a remainder. Note that when it lands, most of
  the family is `ItToken`'s (the "create a token, then attach this
  Equipment to it" subfamily) and only the rest is `ItOtherThan`'s.
- Bruna, Light of Alabaster is the shape the exclusion must NOT reach:
  "attach to IT any number of Auras" nests a description inside the
  object, and that description is a different verb's argument. The
  constructor takes its segment as a written term precisely so that a
  nested description cannot inherit an enclosing clause's exclusion.

### Row 4 — `Noun.ItPrior`, the previous-sibling delta

`ItPrior : (made : Bindings) -> (before : Bindings) -> {auto 0 sp : bs =
made ++ before} -> {auto 0 ok : countOnes Object made = 1} -> Noun bs
Object` — `ItOtherThan`'s twin at the other end of the same split.
`Effects` already hands each member `effIntro e = effDelta e ++ bs`, so
the preceding member's delta is a segment the CONSTRUCTION names
[CR#608.2c] (whose own example, "Destroy target creature. It can't be
regenerated", is this very read). `Macros.itPrior` takes the preceding
clause and reads `effDelta` off it; the clause is written twice and the
two are held together by the type, since the pronoun's context is that
member's `effIntro` and no other clause's. No reordering of `bs`.

It is not clause recency: recency ranks the whole prefix and takes a
winner, where this counts one segment and refuses a tie — two singular
objects in `made` refuse, and a `made` holding none refuses too, leaving
the bare `It` as the spelling. **The macro must be written for a
coordination's own immediate neighbour and nothing else**; writing it
across arbitrary sentence distance would be recency by the back door.

Re-measured families: **38 faces** "tap [X] and put a stun counter on it"
(Stunning Shot's), **31 faces** "create … token, then put a counter on
it", **46 faces** for the tap-and-any-counter shape, and **24
occurrences** of the label-free "[trigger], put [n] counters on target X.
It [verb]s …" shape. The first two are also admitted by row 5's
`ItVerbed "Tap"` and `ItToken`; what `ItPrior` adds is the LABEL-FREE
case, where the producing clause leaves no stamp and no origin.

### Benches

| Witness | Kind | What it proves |
|---|---|---|
| `feralContest` | whole Card | `ItOtherThan` via `Macros.mustBlockIt`; the bare `It` there counts two battlefield creatures, one of them the blocker |
| `thranduilsCompany` | whole Card | `ItPrior` at the LABEL-FREE case — both candidates battlefield, no stamp, no origin; nothing else in the grammar resolves it |
| `stunningShot` | whole Card | `ItPrior` at the ticket's named within-sentence coordination |
| `inquisitorsFlail` | whole Card | the damage-replacement family spelled with zero pronoun reads (row 2's finding) |

### Closing re-measurement — the residue

Re-derived from the method in `done/workbench-anaphora-a-bare-it.md`
(reminder text stripped, scope = the ability paragraph up to the "it",
candidate = a distinct singular object mention announced before it). The
original round's evidence was session-local and deliberately not carried,
so this is a re-derived frame and its raw totals differ from the
original's by ~11%:

| stage | this frame | original |
|---|---|---|
| bare-`it` occurrences | 7,755 | 8,724 |
| ≥2 distinct singular object mentions before it | 2,467 | 3,105 |
| different carrier → `ItAt` resolves | 1,282 | ≈1,656 |
| **same carrier** | **1,185** (968 faces) | ≈886 after hand-verification |

Classifying those 1,185 same-carrier occurrences against every gate the
two rounds landed:

| admitted by | occ |
|---|---|
| row 5 `ItVerbed` (all labels) | 297 |
| row 4 `ItPrior` | 177 |
| row 1 `CopyExcept` / `ControllerSacrifices` | 63 |
| row 2 constructor spelling (`Scales`/`Redirects`/…) | 95 |
| row 5 `ItToken` | 47 |
| row 3 attach — narrowing ready, **constructor missing** | 41 |
| **still REFUSED** | **442** (365 faces) |

**442 same-carrier occurrences over 365 faces is the number this program
leaves refused**, and it is an UPPER bound — the classifier is a regex and
cannot see through a coordination whose last clause names no mention, so
lines `ItVerbed`/`ItPrior` do admit (Alchemax Slayer-Bots is one) are
counted here as refused. Applying the original round's hand-verified
genuine-ambiguity rate (2,541 / 3,105 = 81.8%) puts the genuinely
ambiguous remainder at **≈362 occurrences**, against the ADR's recorded
**≈886**. The program removes roughly 60% of the residue it was sized
against; the double-digit estimate in this ticket's body was optimistic
by an order of magnitude.

Named families inside the 442:

- **185 — self-trigger header + a target in the same clause** (Perrie's
  family). Refused BY DESIGN, row 6: both candidates are battlefield
  creatures and the rules admit both. This is now the single largest
  block and it is the one clause-recency was held in reserve for.
- **44 — attach / equip.** Not an anaphora refusal: the narrowing exists
  and the attach effect constructor does not.
- **50 — counter-placement followed by a second mention**, where the
  preceding clause names two mentions or none, so `ItPrior`'s segment
  does not count to one.
- **19 becomes-a-copy, 17 damage, 16 block relations, 7 exile-then-return,
  2 fight, 102 other.**

### Remainders

- **No attach EFFECT constructor** ("attach [n] to [host]", 65 faces).
  The largest single unwritable family in the residue and the biggest
  single win available; `ItToken` and `ItOtherThan` are both already
  waiting for it.
- **`ItRole` is unbuilt and has no landing site.** The damage-replacement
  family it was sized against is constructor-spelled. If a role read is
  wanted later it needs a construction that announces two participants of
  one event, and `eventIntro` announces exactly one per event today.
- **A bare `It` in `DeonticCounterpart` can still resolve to the Deontic
  subject** when the subject's delta is the only candidate — a residual
  rules-impossible over-generation ("target creature blocks itself"), and
  a pin round of its own. `Macros.mustBlockIt` is the correct spelling;
  nothing on the bench writes the bad term.
- **No lose-one-named-ability row** ("All creatures lose trample until
  end of turn"), which is why Blind Fury is unbenched. `LosesAllAbilities`
  and `LosesEveryType` are the neighbours.
- **`remarkTest` answers `Nothing` for both segment reads.** A per-binding
  test would reach the candidates the segment excludes, so a subject
  written as a segment read tells its following clauses nothing. Widening
  it needs a segment-aware remark, not a test.
- **`Macros.itPrior` writes its neighbour clause twice.** Type-checked, so
  it cannot drift, but a coordination macro would retire the duplication.
