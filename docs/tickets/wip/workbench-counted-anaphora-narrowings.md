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
