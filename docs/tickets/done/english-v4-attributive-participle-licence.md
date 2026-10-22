---
needs: []
---
# [design] Attributive participles only where Oracle uses them

Owner ruling (cost-audit smaller defect S9): mint this as a defect. A verbal
participle read as an attributive Premodifier of a bare noun yields spurious
Readings.

## The defect

The `Premodification(head: Verb, …, forms: [GerundParticiple, PastParticiple])`
Schema makes every verb's participle an attributive Premodifier of a bare
Nominal. In Oracle text this produces:

- *You may play [lands and cast spells] from that player's hand this turn*:
  *cast spells* is a Noun Phrase ("spells that were cast"), coordinated with
  *lands* as the Object of *play*. The same happens on two other sampled
  Segments:
  - *Until end of turn, you may play lands and cast spells from your
    graveyard*;
  - *You may play lands and cast spells from among cards you own in exile with
    croak counters on them*.
- *instead of [declaring blockers]*: *declaring blockers* is a Noun Phrase
  ("blockers that are declaring") instead of a Gerund-participial Clause.

## Authority

- CGEL, Ch. 5 §14.2, p. 444 admits attributive VPs headed by participles in
  English generally (*the defeated army*, *the gleaming showroom*).
  - This reading is therefore not ungrammatical in open-domain English.
  - It is wrong in the Oracle register.
- `docs/decisions/english-lexical-analysis.md` §2 ("Morphological and valence
  completeness vs controlled-register syntax"): syntactic schemas model the
  controlled register, not open-domain English.
- Corpus facts:
  - No Vintage-playable Oracle text uses attributive *cast* or *declaring*
    after a determinative: 0 matches of *the/a/each/that/those/any/no* +
    *cast*/*declaring* + noun in the Oracle Cards snapshot.
  - Oracle does use many participles attributively. In the evaluation data the
    following appear in every Reading of their Segment, so they must keep
    reading:
    - *chosen* (13 Segments), *attacking* (9), *exiled* (7);
    - *triggered* (4), *activated* (4);
    - *blocking* (3);
    - *tapped*, *starting*, *revealed*, *sacrificed*, *locked* (2 each);
    - *enchanted*, *warped* (1 each).

## Owner ruling (round 5, item 7b; decisions of 2026-10-21)

The licence list proposed at minting is rejected: a per-verb list of
participles that may premodify is policing by another name.

1. Gate the gerund-participle form of `Premodification` on the verb having an
   Intransitive frame (`Test::FreeOptional`). An attributive VP takes no
   post-head dependent (CGEL, Ch. 5 §14.2, p. 444), so an active *-ing*
   premodifier needs a verb whose Complement is optional. This rejects
   *declaring blockers* as "blockers that declare". Every attested *-ing*
   premodifier must keep reading.
2. No past-participle licence. *play [lands and [cast spells]]* beside
   *[play lands] and [cast spells]* is genuine structural ambiguity and stays
   in the forest for the Ranking Key.
3. Deduplicate *tapped*/*untapped*: one source supplies each attributive word.

Decisions on the findings:

- (a) Serial coordination of unlike attributes is allowed. It keeps
  *attacking, blocking, or tapped creature* (Sonar Strike) and newly reads
  *the basic, Sphere, and Locus lands* (Monument to Perfection). It causes
  three rises: Acclaimed Contender, Rob the Hoard, Ray of Ruin. Where the
  cheapest Reading is not the endorsed one (Rob the Hoard, Ray of Ruin), that
  is tracked in `english-v4-ranking-residuals`.
- (b) *starting* is an attributive adjective, not a participle of *start*
  (CGEL, Ch. 6 §2.4.3, p. 541: semantic divergence from the verb, as in *the
  winning team*; [CR#103.2a,103.4]). The three judgments that asserted
  VerbPhrase *starting* are rewritten.

## The fix

- `deckmaste_syntax` `premodification`: a gerund-participle premodifier is
  `Test::FreeOptional`; a past participle whose verb carries the existing
  `AdjectivalPassive: Lexical` (its homograph is a declared Adjective) does not
  premodify, as `SingleParticipialPostmodifier` already excludes it. The
  declared adjective (*tapped*, *untapped* [CR#110.5], *goaded*, *modified*,
  ...) is the one attributive source.
- `english.ron`: `ParticipialAttribute` is `ComplementKind: Gerund`, as
  `GerundDepictive` is, so the status adjective and a participle coordinate as
  unlike attributes (*tapped and attacking token*); the `Attributes`
  coordination is serial.
- `adjectives.ron`: attributive-only *starting*.
- No new feature, no word named in the grammar.

## Acceptance set

- *instead of declaring blockers* has only the Gerund-participial Clause
  (Camouflage 220 → 110 Readings).
- *play lands and cast spells* keeps both analyses; the Ranking Key puts
  *[play lands] and [cast spells]* first (key 0/0/19 against 0/1/18 for the
  Noun Phrase *lands and cast spells*).
- Every attested *-ing* premodifier keeps reading: *attacking*, *blocking*
  (also with *non-*), *defending* (the noun *defending player*), *starting*
  (the adjective); *dying* and *entering* occur only as postmodifiers and
  subjects, and their verbs are Intransitive.
- Units losing duplicate copies: about 495 with attributive *tapped* or
  *untapped* (1,350 Readings), 54 with *goaded*, *modified* and other
  declared-adjective homographs (117), and the *cycling ability/cost* units
  (no participle *cycling*: *cycle* takes its Object).
- No face is lost; only the three units in (a) rise.

## Done when

- *instead of declaring blockers* has no Noun Phrase *declaring blockers*.
- *play lands and cast spells* has both Readings, witnessed by a `+`
  Judgment.
- *target tapped creature* has exactly one Reading, the adjective.
- No face is lost, and no unit beyond the three in (a) gains a Reading.

## Constraint: no new features in `english.ron`

Owner's ruling (round 2): this defect comes from a rule that is too broad, and
it is fixed by tightening or deleting that rule. The fix adds **zero new
features to `english.ron`**. It uses existing features, lexicon or catalog
frames, or an engine mechanism, and names no word in the grammar. If it
cannot be done that way, STOP and return to the owner.

Caveat 5 (owner-approved): this ticket needs a structural fix or goes back to
the owner. The round-5 ruling supplies it: existing frames (Intransitive) and
an existing feature (`AdjectivalPassive`), no licence list.

Standard constraints apply.

1. **Census delta** (measured on `wortskox` → `trurusvy`): faces read 28,798 →
   28,798; total Readings 219,746 → 217,705; Surplus 92,028 → 90,481; cheapest
   tier singleton in 14,682 of 17,255 ambiguous units (summed 20,540) →
   14,918 of 16,866 (summed 19,451); lost faces: none.
2. **Judgment set**: 3,022 enforced at `COMPLETED_STRATUM` 6, all hold; 208
   pending.
