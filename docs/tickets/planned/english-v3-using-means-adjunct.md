---
needs: []
---
# Read a *using* phrase as a means Adjunct: You may cast this card from your graveyard using its blitz ability. / If this spell was cast using teamwork, …

## Why

No verb *use* exists in the lexicon, and no subjectless gerund-participial
reads as a bare Adjunct after a verb's Complements. "You may cast this card
using its blitz ability." 0, "This creature crews Vehicles using its
toughness rather than its power." 0, "If this spell was cast using teamwork,
draw a card." 0, and with *by* in front, "You may cast this card by using its
blitz ability." 0. The same sentences with *with* read ("… crews Vehicles
with its toughness rather than its power." 3; "If this spell was cast with
teamwork, draw a card." 2), as does *by* + another verb ("You may cast this
card from your graveyard by paying its mana cost." 5). Blocker report at
trunk `ymqwzour` (`target/english-v3/blockers-wave8.txt`): `word:using` 25
sole, 32 touched; also `word:used` 7. `cargo xtask english-v3 grep -i
'\busing\b'` finds **32** faces, all unread; **26** have that line as their
only failing line. By Complement of *using*: a bare keyword, `\bcast using
(teamwork|web-slinging)`, 19 faces (14 sole) plus *cast a spell using
teamwork* 1; a determined NP, `\busing (its|their) `, 9 (8): *its blitz /
bestow / mutate / warp ability*, *their sneak abilities*, *its toughness
rather than its power*; *using mana produced by* a name, 3 (3).

## Goal

*using* + NP reads as a means Adjunct after the verb and its Complements, in
active and passive clauses: *cast this card from your graveyard using its
blitz ability*, *was cast using teamwork*, *crews Vehicles using its
toughness rather than its power*, *cast a permanent spell using mana produced
by Tecutlan*, with byte-exact realisation.

## Analysis

CGEL Ch. 8 §2.2, pp. 673–674: "Means and instrument adjuncts are
characteristically realised by PPs headed by *by* … and *with*"; "the verb in
the gerund-participial complement of *by* can be *use*, which can give rise
to a means adjunct effectively equivalent to an instrument: *They got in by
using the master-key* / *with the master-key*." That backs the function
(means Adjunct) and *use* as the verb of the equivalent *by* phrase. The
*by*-less *using* phrase is not analysed there, and CGEL's list of deverbal
prepositions (Ch. 7 §2.3, p. 611, [24]: *according*, *barring*,
*concerning*, *including*, *regarding* …) does not contain *using*. Its
analysis here is a project ruling.

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`):
`ParticipialDepictive` (`[phrase: SecondaryVerbPhrase]`, `phrase.form =
GerundParticiple`, `phrase.ParticipialUse = Ordinary`), the one place a
subjectless gerund-participial VP functions as an Adjunct, consumed by the
depictive predicate schemas (`SelectedSourceDepictivePredicate`,
`DepictiveSelectedPredicativePredicate`); `GerundComplementPreposition`
(`[head: lexical(Preposition), " ", complement: SecondaryVerbPhrase]`, *by
paying …*); the VP-final Adjunct admission of
`docs/tickets/done/english-v3-adjunct-admission.md`. A bare keyword as NP
(*teamwork*, *web-slinging*; both `FixedKeyword` in `plugins_v2/`) reads
after *with* through `KeywordComplementPreposition`.

Orchestrator rulings 2026-10-10:

- UM1. *using* is the gerund-participle of a verb, not a Preposition: declare
  `core-verb:Use` in `lexicon/verbs.ron` with a transitive frame. No
  `vocab:Preposition/Using`. The same lexeme supplies *used* (9 faces, *an
  exiled card used to craft it*), which this ticket does not otherwise chase.
- UM2. The phrase enters through the existing gerund-participial Adjunct
  route (`ParticipialDepictive` and the VP-final Adjunct admission), refined
  so that a subjectless gerund-participial VP that declares the means use can
  follow the verb's Complements with no comma, in active and passive clauses.
  The licence is a declared feature of the verb (its participial Adjunct
  use), read by the route; no Construction names the verb.
- UM3. The Object of *use* may be a bare keyword name (*using teamwork*). If
  the transitive frame's Object does not admit a `KeywordPhrase`, add the
  frame slot the *with* route already uses (the keyword Complement class),
  as data; do not mint noun homographs of keywords.
- UM4. One analysis per string. The *using* phrase attaches to the verb
  phrase once; it must not also read as a postmodifier of the Object (*a
  spell using teamwork*: a reduced relative is ungrammatical here only by
  meaning, so if the existing `ParticipialPostmodifiedNominal` admits it,
  report the count and shape rather than guarding the word) or of the source
  NP (*your graveyard using …*).
- UM5. Expected shape of increases: none on covered faces from data alone
  (no covered face contains *using* / *used*). If UM2 widens the bare
  gerund-participial Adjunct for every verb, covered faces with *-ing* forms
  after an Object may gain Readings: bound it by the declared feature so
  they do not, and report any that do with its shape before landing.
- UM6. Newly covered faces are gains. If UM2 needs more than a refinement of
  the existing route, land UM1 and return with the evidence.

## Witnesses

Each is the only failing line of its face in `blockers-wave8.json` (census
`ymqwzour`, 0 Readings). Fragment probes ran 2026-10-10 on the `ymqwzour`
binary (`--readings 60`); trunk then moved to `pxmykwnl` and the prebuilt
probe no longer loads plugin data, so (census) controls give the covered
line's `reading_count`. Re-probe on the claim parent first.

- Tenacious Underdog: "You may cast this card from your graveyard using its
  blitz ability." (control "You may cast this card from your graveyard." 3)
- Giant Ox: "This creature crews Vehicles using its toughness rather than its
  power." (control with *with* 3; "This creature crews Vehicles." 1)
- Heroic Teamwork: "… If this spell was cast using teamwork, draw a card."
  (whole line in the census; fragment control with *with* 2)
- Virtual Assistant: "Whenever you cast a spell using teamwork, create a 1/1
  colorless Robot Hero artifact creature token."
- The Myriad Pools: "Whenever you cast a permanent spell using mana produced
  by The Myriad Pools, up to one other …" (whole line in the census; control
  "Whenever you cast a spell with mana produced by this land, draw a card."
  18)

Controls that must not change: Scourge of Nel Toth "You may cast this
creature from your graveyard by paying {B}{B} and sacrificing two creatures
rather than paying its mana cost." 10 (census); Alien Symbiosis "You may cast
this card from your graveyard by discarding a card in addition to paying its
other costs." 22 (census); Plunder the Trollshaws "Draw a card. If this spell
was cast from a graveyard, draw two cards instead." 2 (census); Hero of
Bladehold "Whenever this creature attacks, create two 1/1 white Soldier
creature tokens that are tapped and attacking." 5 (census).

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching

    (?i)\busing\b|\bused\b|\bby [a-z]+ing\b|\bcast (with|from|for|without)\b|\b(tapped and )?attacking\.

plus 300 random covered controls, `--workers 6 --samples-per-face 0`,
compared with `cargo xtask english-v3 diff` against the claim parent (size
the selector with `cargo xtask english-v3 grep --count` and record it; the
first four alternatives are 316 faces). Because UM2 touches a route other
verbs share, the selector deliberately includes the gerund-participial
control classes, not only *using*. Zero lost; UM5 for increases. Full `cargo
test -p` and `cargo clippy -p … --tests -- -D warnings` for
`deckmaste_lexical_source` and `deckmaste_english_v3`, once, at the end; a
rebuild after a grammar edit takes about 2–3 minutes, the six test binaries
about 75 seconds; lexicon data loads at runtime. Tests assert the role (the
gerund-participial VP as means Adjunct of the matrix verb, its Object; the
UM4 negatives), never an incidental count or a Debug-printed tree.
Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- *used to craft* NP (9 faces, 7 sole): past-participial postmodifier with a
  purpose infinitival; it gains only if the existing routes read it once the
  verb exists. Report the count; do not build it here.
- *Choose one. If this spell was cast using teamwork, choose both instead.*
  (5 faces with three failing lines each): the modal header is the other
  blocker.
- Lines with another blocker: Tapestry Warden (*stations permanents*), Ninja
  Teen, Scarlet Spider and Spiders-Man (*he* / *they* enter with …).

## Claim-parent evidence and STOP (2026-10-11)

Measured on `rrprmvmlsnms` (claim parent `wxwrlxzu`), before changing data:
Tenacious Underdog 0 / control 3; Giant Ox 0 / plain control 1 / *with*
control 3; Heroic Teamwork 0 / *with* control 2; Virtual Assistant 0 /
construction-free control 2; The Myriad Pools whole witness 0 / the ticket's
*with mana* control 18. The *by using* witness is 0. Scourge of Nel Toth,
Alien Symbiosis, Plunder the Trollshaws, and Hero of Bladehold controls are
10, 22, 2, and 5. No already-readable target witness was dropped.

The Method grep matches 432 faces. The built-in census diff selected 300
covered controls with seed 20261008 from `baseline-wave8.json`; two overlap
the pattern, giving 730 identities in the union. The BEFORE census was rebuilt
on the claim parent, not taken from that older report. Evidence is under
`target/english-v3/english-v3-using-means-adjunct/`.

A runtime-only UM1 experiment adds `core-verb:Use` with one transitive Object
frame. It leaves every control unchanged, but Tenacious's witness now has 4
Readings, Giant Ox 2, and the Myriad Pools predicate fragment 34. They enter
through `ParticipialPostmodifiedNominal`: *graveyard using its blitz ability*,
*Vehicles using its toughness rather than its power*, and *a permanent spell
using mana …*. Heroic Teamwork and Virtual Assistant remain 0 because the
ordinary Object frame does not select a Keyword Phrase. *By using* reads twice
through `GerundComplementPreposition`, with a selected Object. No means
Adjunct route is supplied by this experiment.

`cargo xtask english-v3 diff before.json after-um1.json --json diff-um1.json
--lines` compares the same 730 faces: covered 607 → 617; Readings 21554 →
25676; lost/decreases/increases/digest-only all 0. All ten apparent gains use
the unintended nominal attachment, so **none is counted as a correct gain**:
Barracks of the Thousand (3949), Brokkos, Apex of Forever (6), Detective's
Phoenix (7), Giant Ox (2), Interface Ace (3), Sabin, Master Monk (6), Tecutlan,
the Searing Rift (35), Tenacious Underdog (5), The Myriad Pools (103), and
Timeline Culler (6). These are whole-face counts, including unaffected lines.
The nine *used* faces do not gain coverage. The bounded AFTER experiment took
its timing from `after-um1-time.log`, with six workers and zero samples.

**Earlier STOP, resolved by UM7: UM1 alone is not a safe fallback.** UM4 explicitly anticipates nominal
attachments and requests their count/shape rather than a word guard. The
standing landing contract says an unintended analysis is a defect, even when
it supplies coverage. That same defect is already present before any UM2
class-level widening. The existing depictive attachment sites are restricted
to intransitive lexical heads and the selected-source/object-preposition host
Categories; the generic final Adjunct route accepts Adverb Phrases and
Preposition Phrases, not gerund-participial Verb Phrases. A new means-use
licence can admit the intended attachment but cannot itself retire these
existing nominal attachments. Clarification is needed on whether UM4 permits
retaining them as grammatical semantic nonsense, or authorizes a declared
participial-function restriction (and its natural class). No class licence,
keyword frame, grammar guard, or test has been added; the data experiment is
uncommitted and is not a completed landing. No refresh or integration ran.

## UM7 ruling and return to planned (2026-10-11)

UM7 (orchestrator ruling): retain ordinary gerund-participial nominal
attachment ambiguity without a restriction aimed only at *use*. UM1 alone
must not land: the ten faces count as gains only when at least one Reading
also contains the intended VP means Adjunct. Preserved nominal attachments
are reported beside another participial-postmodifier control. A covered-face
increase must be a correct means-Adjunct Reading with shared control
ambiguity. If UM2 needs a new Construction or produces incorrect increases,
restore code, record the findings, return this ticket to planned, and land
only that ticket change. Otherwise finish and stop before integration.

Continuation on `rrprmvmlsnms` tried the existing schema rather than adding
a named Construction. The experimental lexical category default declared
`ParticipialMeansUse` for Verb, with stative exceptions on *be*, *have*,
*control*, *own*, *share*, *cost*, and *remain*; auxiliary projections denied
that use. Ordinary and coordinated secondary projections retained the
permission, and `ParticipialDepictive` projected it into its existing phrase.
The experiment preserved all ordinary nominal and depictive routes. It also
tried UM3's Object frame selecting `KeywordPhrase`, without keyword noun
homographs. These were exploratory declarations, not a landed class ruling.

The proposed refinement added these rows to `ComplementedDepictivePredicate`:

```
(FinitePredicate, Self, PredicateHostAgreement, MeansAdjunctAdmission)
(SecondaryVerbPhrase, Self, SecondaryAdjunctProjection, MeansAdjunctAdmission)
```

The existing rows use `FiniteDepictiveHost` and `SecondaryDepictiveHost`.
The generic rows require the modifier's declared means permission. This is
the missing after-Complements, no-comma attachment site, distinct from the
intransitive lexical host of `DepictivePredicate`. `cargo build -p xtask`
rejected it with **"schema requires unique result category per instance"**
for `ComplementedDepictivePredicate`. The generated Reading variant cannot
select two different Head Categories for the same Result Category under this
schema. Keeping the old selected depictive hosts while adding the completed
predicate host needs a distinct attachment Construction (or a host-union
conversion Construction). Reusing `AdverbPredicate` has the same repeated
Result restriction and does not make a gerund-participial VP an Adverb Phrase.
Replacing the old depictive rows would retire existing analyses, contrary to
the preservation requirement. No compiler change or new Construction was made.

Consequently UM2 does not build, its witnesses have no measured intended
means-Adjunct Reading, and there is no UM2 census or claimed gain. The UM1
probe/census findings above remain the evidence for what reads through the
existing nominal route and *by* Complement route; *using teamwork* remains
unread. All grammar and lexical edits, including UM1 and UM3, were restored.
No tests were added, removed, ignored, restored, or re-spelled; no full tests,
clippy, Lean checks, or full corpus census ran. The 730-face before/UM1 census
and built-in diff remain under target for review. The return change is
**ticket-only**; this is not a completed implementation or a Landing record.
A revised ticket needs authority for the distinct means/manner attachment
Construction or a concrete refinement that preserves the existing schema's
single-Head-Category constraint.

The ticket-only findings were committed as `rrprmvmlsnms` and refreshed.
Kata's bundled Kanban driver completes WIP items into done during integrate;
its supported return transition is `kata drop english-v3-using-means-adjunct
--return-items` from default. That transition preserves this ticket's edits,
returns it to its claim-parent planned folder, and retires the workspace.
Code/data restoration was verified by an empty jj diff for both crates.
The restored `cargo xtask` build succeeded. Citation checks reported zero
non-compliant strings and zero stale citations; diff audit selected no new
citation sites. Refresh brought only ticket claim changes, not code or data.

## Landing record

To be written by the claimant (six items, `docs/tickets/README.md`).
