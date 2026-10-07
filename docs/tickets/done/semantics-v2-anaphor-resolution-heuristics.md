---
needs: []
---
**Find the rule that resolves a pronoun or demonstrative with two candidates
the way Oracle text means it, and measure it before changing anything.** The
owner asked for this ticket on 2026-10-05: "make sure the anaphor resolution
question is ticketed because I think there must be some heuristic to get it to
resolve correctly that we haven't discovered/implemented yet". Research first;
a rule change is a separate, later decision. Standard constraints apply.

## Decided 2026-10-05

The strict rule stays, for both `it` and `that(N)`, until a measured
alternative is put to the owner.

## Owner direction, later on 2026-10-05

Direction for the measurement, not a rule change:

- `it` and `that(N)` need not share one rule. The demonstrative is the form
  Oracle text uses to disambiguate where a bare pronoun would be ambiguous,
  and treating the two alike is "sorta not correct".
- A ranking is a rule to measure alongside the filters. The owner's view is
  that Oracle English is uniform enough for a good heuristic to be good: what
  the corpus bears out is the editors' convention.

## Decided 2026-10-06

- **The measurement runs now.** It is read-only work for an agent; its results
  are written into this ticket. It changes no checker code and no pin (see
  "Out of scope").
- **Frame it on the discourse-representation table** in
  `docs/tickets/maybe/lean-drt-anaphora-refactor.md`: bindings are discourse
  referents, windows and masking are accessibility, a unique reach is a
  presupposition bound to its antecedent. The missing piece, and the subject of
  this measurement, is the choice among accessible antecedents.
- **`lean-drt-anaphora-refactor` is promoted to `planned/` when the owner
  accepts a measured rule**, and not before.
- Owner: "the last quarter mile here may be trickier than we realize."

### Inventory: windows built by hand

The Lean macros that build a pronoun window themselves are the piecemeal
resolution the refactor names. Each is evidence for which rule the corpus
needs; the measurement says, for each, whether the candidate rule reproduces
it. They are not conveniences and the capture landing does not touch them.
Lines in `lean/Semantics/Macros.lean`, 2026-10-06:

| Macro | Line | Window |
|---|---|---|
| `itPrior` | 37 | `.introduced` over what the previous instruction announced |
| `itCondSubject` | 44 | `.introduced` over what the condition introduced |
| `attachToIt` | 670 | `.outsideIntroduced` (671) |
| `requireBlockIt` | 696 | `.outsideIntroduced` (699) |
| `agentRef` | 727 | `.introduced` over the agent's own bindings |
| `itsOther` | 747 | `sameWindow` over `selfSubjIntro` |
| `ownSubject` | 761 | `.introduced` over what the subject announced |
| `controllerSacrifices` | 766 | `.introduced [.player, …]`, or `ownSubject` |
| `lookedCards` | 776 | `.introduced` over the slice |

### The reciprocal gap

Fight's rule text, "Each of those creatures deals damage equal to its power
to the other creature" [CR#701.14a], is a reciprocal ("each … the other"):
each member of a pair reads the remaining member as its antecedent. The model
has no such reference form. The capture landing writes fight with the
first-mention binding rule; the measurement classifies the reciprocal with the
other forms and says what the context would need to carry for it.

## The current mechanism

- A pronoun (`it`, `that(N)`, `they`) is refused unless exactly one visible
  binding matches: `lean/Semantics/Check/PhraseRules.lean` `.pro` case, ~L310,
  `refuse (n == 1) (.anaphor r pl n)`.
- Once admitted, its denotation is the first, i.e. most recent, match:
  `lean/Semantics/Check/Phrase.lean` ~L1323, `NounPhrase.result`'s `.pro` arm,
  `(windowAddresses window bs).find? …`.
- Windows narrow the candidates (`lean/Semantics/Words.lean` `Window`,
  ~L299-306): `whole`, `parameter`, `top n`, `below n`, `introduced`, `outsideIntroduced`. Only
  `outsideIntroduced` (macros `attachToIt` ~L678 and `requireBlockIt` ~L704 in
  `Macros.lean`) selects a non-newest antecedent today.

## The recorded rationale

- `docs/decisions/oracle-text-is-forward-anaphoric.md` L60-82: carrier-scoping
  is "emphatically NOT a preference: there is no find-first, no nearest-wins,
  no determiner tiebreak" (L62-63). Of ~2,541 corpus occurrences where a bare
  "it" has two or more singular object antecedents, carrier-scoping resolves
  ~1,656 and ~886 have both candidates in the same carrier and stay refused
  (L76-78). Clause-recency, the rejected alternative, "is recorded as held in
  reserve against that number" (L79-82): it would replace the count with a
  find-first and cost the `countBy` witness lemmas.
- `docs/decisions/semantics-v2.md` L101-105: wildcard pronouns "demand a unique
  compatible antecedent"; "There is no nearest-wins tiebreak at the semantics
  layer; the guide's own editorial rule — repeat a noun rather than stack
  ambiguous pronouns — is the type discipline."

## What the style guide says

`docs/oracle-style-guide.md` is this project's reconstruction from the corpus,
not a Wizards document, so its silence is a gap in the reconstruction and not
evidence that the editors have no rule.

- L577: `it` and `its` are for an object "after a clear singular antecedent".
  The closing checklist (L2495-2496) wants each pronoun to have "one
  unambiguous antecedent".
- L583-585: "Repeat a noun or name instead of stacking ambiguous pronouns.
  After a sentence names a source, target, controller, and owner, replace an
  ambiguous `it` with `that creature`, `that player`, `the exiled card`, or the
  source's short name." The demonstrative is the disambiguating form.
- L590-591: "Use `that` or `those` for an antecedent established by the
  preceding instruction".
- It states no rule for which of several candidates `it` takes: no subject,
  recency or first-target rule and no tiebreak (pronoun and object sections
  read, the rest searched, 2026-10-05).

The checker reads "clear" as "exactly one compatible binding". The ~886
same-carrier occurrences above show the editors count some two-candidate
antecedents as clear. Their working definition of "clear" is what this ticket
is looking for.

## Evidence gathered 2026-10-05 (Vintage-legal, non-funny text)

**Bare "it" does not follow recency.**
- Feral Contest: "Another target creature blocks it this turn if able".
- Cloud, Ex-SOLDIER: "attach up to one target Equipment you control to it".
- Monstrous Step: "Up to one other target creature blocks it this turn if
  able".
- Saheeli, Sublime Artificer: "Target artifact you control becomes a copy of
  another target artifact or creature you control until end of turn, except
  it's an artifact in addition to its other types."
- The template "becomes a copy of another target creature, except it has this
  ability".

**"that N": 464 sites with two or more candidates were hand-checked; 77 refer
to an OLDER candidate.** By kind:
- (a) 29: the demonstrative sits inside the newer noun phrase ("search for a
  creature card with the same name as that creature") — harmless to the model,
  since the outer phrase is not yet bound.
- (b) 22: the newer candidate is the subject of the demonstrative's own clause
  ("Each creature you control deals 1 damage to that creature"; Time to Feed
  "Target creature you control fights that creature").
- (c) 11: the newer mention is inside the referent's own modifier.
- (d) 4: a token created in between is skipped (Kalitas, Bloodchief of Ghet:
  "create a black Vampire creature token. Its power is equal to that
  creature's power").
- (e) 2: "than each other player".
- (f) 9: no structural excuse (Zur's Weirding, Ram Through, Mangara's Equity,
  Tragic Banshee, Alpha Brawl, Reincarnation, Noetic Scales, Sokrates, Guard
  Dogs).

**Same syntax, opposite results.** Vampiric Embrace: "Whenever a creature
dealt damage by enchanted creature this turn dies, put a +1/+1 counter on that
creature" (the enchanted creature), versus Scythe of the Wretched: "… dies,
return that card to the battlefield … Attach this Equipment to that creature"
(the one that died), and Runesword.

**Editorial habit.** Of 78 two-creature fight/bite templates, 65 never refer
back, 9 use definite descriptions ("the creature you control"), and exactly
one uses "that creature" (Ram Through). One-target templates put the
demonstrative before the second creature appears ("Then that creature fights
target creature you don't control").

**Pins.** 19 Lean pins assert an ambiguity refusal (n >= 2). Under
most-recent, 6 would pick the intended referent, 2 the wrong one, and 11 are
undeterminable.

## Candidate rules (measured 2026-10-06; see below)

Discourse representation theory (Kamp and Reyle 1993) settles which
antecedents are accessible, which the checker already models with bindings and
windows. It does not choose among accessible antecedents. The candidates below
come from the work that does. Nothing here has been measured; do not implement
any of it on this account.

**A. Disjoint reference among the arguments of one clause** (binding theory's
Principle B; this was the orchestrator's hypothesis). A plain pronoun or
demonstrative does not refer to another argument of its own clause (that would
be "itself"): in "Another target creature blocks it", "it" cannot be the
blocker; in "Each creature you control deals 1 damage to that creature", "that
creature" cannot be the dealer; in "Its power is equal to that creature's
power", the two cannot be the same creature. Excluding clause-mates and then
requiring uniqueness may resolve kinds (b) and (d) and the bare-"it" cases;
kinds (c) and (f) would remain. Two macros already hand-code this for one verb
each through `outsideIntroduced`: `attachToIt` and `requireBlockIt` exclude
what the clause's other argument introduced. A general rule should reproduce
both.

**B. A different antecedent status for `it` and `that N`** (the Givenness
Hierarchy, Gundel, Hedberg and Zacharski 1993). A bare pronoun needs an
antecedent at the centre of attention; a demonstrative noun phrase needs only
one already mentioned. So `it` would count its candidates in a narrower set
than `that N` does. The measurement has to find an operational definition of
the narrower set. Definitions to try: the subject of the preceding
instruction, the object the preceding instruction acted on, the first target.
The literature calls this status "in focus", which is not the glossary's
**Focus**; a landing needs its own term.

**C. The demonstrative takes what the pronoun left.** When a pronoun and a
demonstrative occur in one clause they do not corefer, and the demonstrative
takes the less prominent candidate (reported for German demonstratives by
Bosch and Umbach 2007). Kalitas: "Its power is equal to that creature's
power".

**D. A ranking.** Order the candidates by grammatical role (Centering, Grosz,
Joshi and Weinstein 1995: subject before object before the rest) and take the
top one, refusing only a true tie. Plain recency is the baseline to beat; the
evidence above already has it wrong for 77 of 464 `that N` sites. A to C
narrow the candidates before the existing count. D replaces the count with a
pick, so a wrong answer checks cleanly where a wrong filter would refuse.

Expected residue under all four: the same-syntax pairs above (Vampiric Embrace
against Scythe of the Wretched), unless the measurement finds what tells them
apart.

## Measured 2026-10-06

The measurement asked for under "The work" below. It is research only and
recommends no implementation. The report, scripts, answer keys and outputs
are in
`docs/evidence/anaphora-measurement-2026-10-06/` (an ignored local evidence directory in the `default` checkout, shared into feature workspaces by provisioning; not version-controlled)
(report `measurement-2026-10-06.md`; keys `key_that.txt`, `key_it.txt`,
`key_pins.txt`; scripts `corpus.py`, `analyze.py`, `dump_that.py`,
`extract_it.py`, `pins.py`, `rules.py`, `score_*.py`, `breakdown.py`,
`reciprocals.py`; outputs `*.out`).

**Framing.** As in `lean-drt-anaphora-refactor`: bindings are discourse
referents, windows and masking are accessibility. The key excludes a
candidate only as inaccessible: `i` inside the anaphor's own unbound NP; `g`
non-referring (predicate nominal, plural, quoted ability, the "card" of "card
name"); `s` closed scope (`each`/`no`/`any`, the `Otherwise` arm reading the
condition, a mention outside a quoted ability); `c` the consuming verb cannot
apply in the candidate's zone [CR#109.2]. Mentions of one referent are merged
into one chain. The rules choose among what remains.

### Reproduction

- 31,981 Vintage-legal, non-funny oracle ids (32,828 faces, 60,106
  paragraphs); 4,517 "that N" occurrences.
- 464 with two or more regex candidates; `sites.json` is byte-identical to the
  2026-10-05 list.
- 121 are not the newest regex mention: the 77 in kinds a–f, split a 33 /
  b 22 / c 8 / d 2 / e 3 / f 9 (2026-10-05: 29 / 22 / 11 / 4 / 2 / 9; the f
  cards are the same nine), plus a new **kind g**, 44 sites: the newest regex
  mention does not refer at all ("If it's a creature card, … that card"). The
  2026-10-05 hand check did not count g. The a/c/d/e differences are boundary
  judgments: Mythos of Illuna and Arna Kennerüd moved from c to a; d counts
  only where the token is the newest regex mention (Kalitas, Broken Visage);
  Battle Angels of Tyr's two "than each other player" sites are e.

### "that N": why the strict count admits or refuses each site

- 239 are one coreference chain ("target creature … that creature … that
  creature").
- 138 have one accessible referent: 58 after `g`, 40 after `s`, 32 after `i`,
  8 after `c`.
- **87 are genuinely ambiguous**: 84 with two accessible referents, 3 with
  three.

The 377 single-referent sites resolve correctly under strict only if the
checker's minting and masking match the key's accessibility, which is
unverified.

### Rules measured

All apply after accessibility. strict: exactly one, else refuse. R0/R1: plain
recency over raw regex mentions / accessible referents (R1 is today's `find?`).
A: remove clause-mates, then require uniqueness; Aarg: Principle B proper,
exempting possessive anaphors and possessor-only clause-mates. B (`it` only):
B1 subject, B2 object of the preceding or superordinate clause, B3 first
target; for `that N`, A+B = A. C: remove what a pronoun in the demonstrative's
own clause picks up; ACwB1/ACwB2 also shun the B1/B2 referent. D: rank role
(S > O > X), then nearer clause, refuse a true tie. D′: nearer clause, then
role (the ADR's reserved clause-recency). X>Y: filter X, then pick by Y. A pick
rule turns a wrong answer into a clean check; a filter can only refuse.

### "that N": the 87 ambiguous sites

| Rule | Correct | Wrong (silent) | Refused | Correct − wrong |
|---|---|---|---|---|
| strict | 0 | 0 | 87 | 0 |
| R1 recency | 53 | 34 | 0 | 19 |
| A | 28 | 0 | 59 | 28 |
| A+B (= A) | 28 | 0 | 59 | 28 |
| Aarg | 23 | 0 | 64 | 23 |
| C alone | 14 | 0 | 73 | 14 |
| A+B+C (AC) | 29 | 0 | 58 | 29 |
| ACwB1 | 40 | 8 | 39 | 32 |
| ACwB2 | 33 | 27 | 27 | 6 |
| D alone | 36 | 47 | 4 | −11 |
| D′ alone | 44 | 39 | 4 | 5 |
| AC>D | 61 | 24 | 2 | 37 |
| AC>D′ | 72 | 13 | 2 | 59 |
| **AC>R1** | **76** | **11** | **0** | **65** |

Over all 464: R0 343 correct / 121 wrong; AC>R1 453 / 11; strict 377 correct,
87 refused. No site is undeterminable.

By kind (correct/wrong/refused):

| Rule | a 33 | b 22 | c 8 | d 2 | e 3 | f 9 | g 44 |
|---|---|---|---|---|---|---|---|
| strict | 31/0/2 | 5/0/17 | 0/0/8 | 0/0/2 | 3/0/0 | 2/0/7 | 44/0/0 |
| R1 | 33/0/0 | 5/17/0 | 0/8/0 | 0/2/0 | 3/0/0 | 2/7/0 | 44/0/0 |
| A (= A+B) | 31/0/2 | 21/0/1 | 0/0/8 | 2/0/0 | 3/0/0 | 6/0/3 | 44/0/0 |
| AC | 31/0/2 | 21/0/1 | 0/0/8 | 2/0/0 | 3/0/0 | 6/0/3 | 44/0/0 |
| D | 32/1/0 | 6/16/0 | 5/1/2 | 2/0/0 | 3/0/0 | 5/4/0 | 44/0/0 |
| AC>D | 32/1/0 | 22/0/0 | 5/1/2 | 2/0/0 | 3/0/0 | 7/2/0 | 44/0/0 |
| AC>D′ | 32/1/0 | 22/0/0 | 5/1/2 | 2/0/0 | 3/0/0 | 7/2/0 | 44/0/0 |
| AC>R1 | 33/0/0 | 22/0/0 | 0/8/0 | 2/0/0 | 3/0/0 | 6/3/0 | 44/0/0 |

A clears kinds b and d as predicted; a, e and g are accessibility matters.
Kind c is where recency and role disagree: AC>R1 gets all 8 wrong, AC>D′ 5 of
8 right.

**The 11 wrong under AC>R1** (0 refused, 0 undeterminable):

- Kind c, a newer mention inside the intended referent's modifier: Tahngarth,
  First Mate; Unpredictable Cyclone; Eriette, the Beguiler; Runesword; Scythe
  of the Wretched; Bronze Bombshell.
- "an opponent or a permanent an opponent controls", the newer opponent inside
  the sibling disjunct: Gisela, Blade of Goldnight; Solphim, Mayhem Dominus.
- Kind f: Ram Through; Mangara's Equity; Noetic Scales.

The 13 wrong under AC>D′: Joint Assault, Wall of Caltrops, Sword of Kaldra,
Mythos of Illuna, Ram Through, Guile, Shield of the Righteous, Eriette, Hazel
of the Rootbloom, Barrow-Blade, Aegis of the Legion, Neko-Te, Noetic Scales;
mostly "Whenever X deals damage to/blocks a creature, … that creature", where
role ranking picks the subject X.

Five of the nine f-cards resolve under A: Guard Dogs ("if it shares a color
with that permanent"), Alpha Brawl ("each of those creatures deals damage … to
that creature"), Reincarnation (the returned card is the object of "return"),
Sokrates ("This creature's controller and that player"). Zur's Weirding
resolves under strict: "If a player does" is not accessible from the
`Otherwise` arm (DRT, and the checker's own `badOtherwiseReadsIfArm`).

### Bare `it`: a random sample

`extract_it.py` takes every `it`/`it's` (not `its`) and counts every earlier
singular object mention in the paragraph, `CARDNAME` and "this N" included:
9,151 occurrences, **3,759 (on 2,846 cards) with two or more candidates**. This
frame is broader than the ADR's ~2,541, so the counts are not directly
comparable. Sample: **150**, random, seed 20261006. One is non-anaphoric (Grim
Reaper's Sprint, "If it's your main phase"); 20 are coreference chains; 68
have one accessible referent (25 by carrier alone); **61 are ambiguous**
(40.7%, Wilson 95% CI 33.1–48.7%), **projected ~1,529 of 3,759 (95%
1,246–1,829)**, above the ADR's ~886 (710–1,061) because of the broader frame
and because carrier was applied only where the verb cannot apply at all.

| Rule | Correct | Wrong (silent) | Refused | Correct − wrong | Wrong in population |
|---|---|---|---|---|---|
| strict | 0 | 0 | 61 | 0 | 0 |
| R1 recency | 46 | 15 | 0 | 31 | ~376 (231–596) |
| A | 3 | 1 | 57 | 2 | ~25 |
| Aarg | 3 | 0 | 58 | 3 | 0 |
| B1 | 19 | 2 | 40 | 17 | ~50 |
| B2 | 30 | 1 | 30 | 29 | ~25 (4–138) |
| B3 | 12 | 7 | 42 | 5 | ~175 |
| A+B2 | 30 | 1 | 30 | 29 | ~25 |
| D alone | 34 | 25 | 2 | 9 | ~626 (434–881) |
| A>D | 35 | 24 | 2 | 11 | ~601 |
| **D′ alone** | **60** | **0** | **1** | **60** | **0 (0–94)** |
| A>D′ | 59 | 1 | 1 | 58 | ~25 |
| A>R1 / Aarg>R1 | 47 / 48 | 14 / 13 | 0 | 33 / 35 | ~350 |

D′'s one refusal is Cocoon ("if this Aura has a pupa counter on it", two
clause subjects). Of D's 25 errors, 20 are an earlier (trigger) subject
outranking a searched, revealed or targeted object.

**The copy template.** 12 of R1's 15 errors are copy templates: in "create a
token that's a copy of target creature, except it has haste" the newest
mention is the copied creature but "it" is the token (Callidus Assassin, Jolly
Balloon Man, Mocking Doppelganger, Splinter Twin, Multiversal Recruitment,
Vesuvan Shapeshifter, Imposter Mech, Twinflame, Offspring's Revenge, Shuri,
Sakashima, Orthion). The other 3: Frostwielder (kind c); Grip of Phyresis and
Anthem of Rakdos (clause-mates).

### The ambiguity pins

`pins.py` finds **21** theorems in `lean/Semantics/Proofs/` asserting
`.anaphor … n` with n >= 2, not 19 (14 `it`/`them`/`its`, 7 `that N`/`they`/
`that turn`):

| Pin | Location | Theorem | Key |
|---|---|---|---|
| P01 | Actor.lean:82 | `badAmassBareItAfterDestroy` | intended |
| P02 | Anaphora.lean:465 | `badSingularSpellReadAfterCopy` | ? |
| P03 | Anaphora.lean:486 | `badSingularAbilityReadAfterCopy` | ? |
| P04 | Anaphora.lean:782 | `badIndefiniteChosenPlayerRead` | intended |
| P05 | Anaphora.lean:848 | `badItAcrossOwnSlot` | intended |
| P06 | Anaphora.lean:886 | `badCondUnwindowedRead` | intended |
| P07 | Anaphora.lean:914 | `badSharedSubjectTwoInDelta` | conjunction |
| P08 | ControllerSacrifice.lean:106 | `wholeOwnedScopeWouldBeAmbiguous` | intended |
| P09 | Counters.lean:178 | `badBatchTwoCreatesThenIt` | ? |
| P10 | Damage.lean:53 | `badIt` | ? |
| P11 | Damage.lean:81 | `badThemAmbig` | ? |
| P12 | Damage.lean:89 | `badInnerAmbig` | ? |
| P13 | Faces.lean:886 | `badUntapNextAmbiguousIt` | ? |
| P14 | GetsBothDeltas.lean:315 | `badSharedSubjectTwoInDeltaTwoHalves` | conjunction |
| P15 | ReferenceScopes.lean:106 | `attachKeepsOuterAmbiguity` | ? |
| P16 | ReferenceScopes.lean:28 | `ordinaryItRemainsAmbiguous` | ? |
| P17 | ReferenceScopes.lean:33 | `formerDepthReachesOuterArtifact` | intended |
| P18 | Turn.lean:120 | `badVerbedAmbig` | ? |
| P19 | Turn.lean:128 | `badBareCardRead` | intended |
| P20 | Turn.lean:499 | `badThatTurnAfterTwoTurns` | ? |
| P21 | Turn.lean:68 | `badTwoCostMentions` | ? |

`key_pins.txt` codes 7 pins with a determinate intended referent (`I=B`, the
newer binding in each): P01, P04, P05, P06, P08, P17, P19. The report's prose
lists five of them as "intended" and puts P08 and P19 under "wrong if
resolved", meaning a wrong pick is possible there (A picks the wrong one for
P08); the key itself counts all seven as intended. P07 and P14 (`I=X`, the
shared subject "both … get +1/+1") intend the conjunction, so any single pick
is wrong. The other 12 (`I=?`) are constructed ambiguities with no intended
referent ("Target creature fights target creature. Tap it."). For P08 the key
assumes the creature binding is minted after the artifact embedded in it.

Flips (no longer refused) / correct / wrong / undeterminable:

| Rule | 14 `it`-form pins | 7 `that`-form pins |
|---|---|---|
| strict | 0/0/0/0 | 0/0/0/0 |
| R1 | 14/5/2/7 | 7/2/0/5 |
| A | 2/0/2/0 (P05, P08 go wrong) | 0 |
| Aarg | 0 | 0 |
| B1 / B2 / B3 | 4/3/0/1 · 6/1/1/4 · 10/1/6/3 | n/a |
| D | 8/4/0/4 | 6/2/0/4 |
| D′ | 8/4/0/4 | 6/2/0/4 |
| AC>R1 | n/a | 7/2/0/5 |

- **"`it`: D′, `that N`: AC>R1" (clause recency for `it`, A–C then recency for
  `that N`)**: 15 of 21 flip, 6 intended, 0 wrong, 9 undeterminable; the 6
  that stay refused are true ties (P07, P08, P09, P12, P15, P16).
- **Plain recency (R1) for both**: 21 flip, 7 intended, 2 wrong (P07, P14), 12
  undeterminable; the 2026-10-05 6/2/11 plus the two extra pins.
- **Rule A must not exclude possessives.** P05 `badItAcrossOwnSlot` ("Target
  creature deals damage equal to its power to any target"): a possessive `its`
  may take its own clause's subject, as `itsOther`, `ownSubject` and
  `dealDamageOwnPower` encode. Weeping Angel ("that creature's owner shuffles
  it") is the corpus case: the possessor of the subject is the antecedent, so
  an A that excludes possessors is wrong and Aarg is right.

### The same-syntax pair is settled by zone

Recency picks the creature inside the modifier ("by enchanted creature"); role
ranking picks the head ("a creature … dies"). Vampiric Embrace means the
embedded creature; Scythe of the Wretched and Runesword mean the head. The
separating feature is fold-state, not syntax: in Vampiric Embrace the head
creature has died, so "put a +1/+1 counter on that creature" cannot apply to
it [CR#109.2], and with that `c` exclusion Vampiric Embrace resolves under
strict. Scythe ("return that card … Attach this Equipment to that creature")
is resolved only by D′, since it re-mentions the creature that died; Runesword
by D′ and AC>D; AC>R1 gets both wrong. Zone settles **8 sites, which the zone
exclusion removes from the 95 otherwise ambiguous, leaving 87**: Survival of
the Fittest, Takklemaggot, Next of Kin, Tragic Banshee, Necrotic Plague,
Devour Intellect, Vampiric Embrace, Infectious Rage. This assumes the checker
applies carrier and zone to `that N` as it does for `ItAt`, which is
unverified.

### The reciprocal gap row

Not a candidate rule: not a choice among antecedents, and none of A–D
addresses it.

| Construction | Paragraphs / cards | What the model needs |
|---|---|---|
| "fight(s)", reciprocal by rule: "Each of those creatures deals damage equal to its power to the other creature" [CR#701.14a] | 145 / 142 | A binder over a pair that reads "the other member" |
| "fight each other" | 11 / 11 | Same |
| "one another" (Chrome Replicator, Mechanized Production) | 2 / 2 | Same |

### What each rule reads (step 6)

Mechanism today: the gate is `lean/Semantics/Check/PhraseRules.lean` L309–311
(`.pro`, `refuse (n == 1) (.anaphor r pl n)` at L311); the denotation is
`lean/Semantics/Check/Phrase.lean` L1343–1346 (`NounPhrase.result`'s `.pro`
arm, `(windowAddresses window bs).find?` at L1344, already R1 within the
window); windows are `lean/Semantics/Words.lean` L304–311; the hand-built
windows are the macros inventoried above (same lines).

| Rule | What the context must carry | Existing hand-coding it would generalise |
|---|---|---|
| R1 / AC>R1 | Nothing new: `bs` order is recency; the gate becomes a pick. | `find?` (Phrase.lean L1344) |
| A / Aarg | **Clause membership**: the bindings introduced or read by the sibling argument slots of the predicate being checked (`NounResult` already returns a read's `address`). Aarg also needs the anaphor's **position** (possessive or argument) and whether the clause-mate link runs through a possessor. | `attachToIt`, `requireBlockIt` (`outsideIntroduced` of one sibling); `itsOther`, `ownSubject`, `controllerSacrifices` encode the opposite, so a general A must exempt possessives (P05, Weeping Angel). |
| C | The addresses that pronouns in the same clause resolved to. | None |
| B1 / B2 | **Antecedent status**: each binding stamped with its grammatical role (S/O/X) and the instruction or clause it came from, relative to the reading clause. | `itCondSubject`, `agentRef` (B1); `itPrior`, `lookedCards` (B2-like) |
| B3 | A first-target marker (`Binding.det = .target` plus order). | None |
| D / D′ | Role and clause distance for every binding (as B), plus a ranked pick with tie refusal. | `Window.top` users (`comparesOwnStat`, `dealDamageOwnPower`) |

### Caveats

1. **One annotator, unblinded.** The same agent judged the intended referents
   and coded clause distance and role, knowing the intended referent. D′'s
   near-perfect `it` score depends on those distance codes. The 87 + 61
   ambiguous sites need a blind second coding before any rule is accepted.
2. **Accessibility is the key's DRT judgment, not the checker's behaviour.** Of
   the 377 sites strict resolves, 40 depend on `s`, 32 on `i`, 8 on `c`.
3. **The candidate frame is regex head nouns**; hands and missed names were
   added by hand (7 `it` sites).
4. **The `it` result is a sample** (150 sites, 61 ambiguous); the `that N`
   result is a census.

## Blind second coding and agreement (2026-10-07)

Answers caveat 1 above and open judgment 6 below. Research only; it recommends
no implementation. Every number comes from the agreement scripts; the coder 1
columns reproduce `score_that.out` and `score_it.out` byte for byte.

**Artifacts.** `docs/evidence/anaphora-measurement-2026-10-06/` (coder 1,
above); `docs/evidence/anaphora-blind-2026-10-07/` (coder 2: keys
`key2_that.txt`, `key2_it.txt`, hard-site notes `judgments.md`);
`docs/evidence/anaphora-agreement-2026-10-07/` (report
`agreement-2026-10-07.md`; scripts `agree.py`, `lists.py`, `dcheck.py`,
`c2only.py`, each with its `*.out`). All three are ignored local evidence
directories, not version-controlled. The scripts import coder 1's `rules.py`
unchanged.

**Design.** A fresh coder saw the site texts only: not coder 1's keys and not
the 2026-10-06 report. Coder 2 used different referent labels and a different
clause convention, so the keys are aligned through the candidate-index set of
each coder's intended referent: exact (same set), merge (one set contains the
other: same referent, one coder merged a further mention into its chain),
non-object (coder 1's `+` hand against coder 2's `X`), both-`+`,
undeterminable (`?` on one side), and disagree (disjoint or partial overlap).

### Referent agreement

**No site falls in the disagree class.**

- **"that N"**: 94 of 94 determinable sites agree (92 exact, 2 merges:
  Deadly Cover-Up and Green Slime, which coder 2 therefore finds
  unambiguous). One is undeterminable to coder 2: T426 Analyze the Pollen,
  where "that card" is whichever search happened, a disjunctive antecedent.
- **Bare `it`**: 150 of 150 agree: 128 exact, 12 merges, 6 hand sites
  (coder 1 `+`, coder 2 `X`), 3 both-`+`, 1 expletive (Grim Reaper's Sprint).
- **Cohen's kappa**, over two site-independent labels (L1: how many regex
  mentions follow the referent's latest mention; L2: whether the referent
  holds the newest mention): 1.0 on both for "that N". For `it`, 0.81 (L1)
  and 0.96 (L2) over all 150. The whole shortfall is the 12 merges: the
  referent is the same, but the coders disagree on whether a predicative,
  copy-target ("a token that's a copy of …") or quoted "this creature"
  mention belongs to its chain, which moves the referent's latest index. On
  referent identity alone, agreement is 100% and kappa 1.0.

### Clause distance and role

- **Clause distance** matches exactly on 81% of matched referent pairs for
  "that N" (149/183) and 66% for `it` (89/134).
- **The cause is systematic.** Coder 2 counts trigger conditions and costs as
  clauses (Nissa, Abundance, Divining Witch, River's Grasp, the bare-`it` self
  mentions), and puts relative and "except" clauses inside their host, so
  coder 1's minimal d0 becomes coder 2's d1 (all copy templates, Gurzigost).
  Coder 2's distance is larger in 63 of 79 mismatches. **The offset is not
  constant**: it varies within 24 "that N" sites and 23 `it` sites, so it does
  not cancel out of the within-site ranking there.
- **Role** matches on 91% (kappa 0.86 for both forms).

### Re-scores, three ways

Correct / wrong / refused, `u` undeterminable: coder 1's labels on coder 1's
ambiguous set; coder 2's labels on coder 2's set; and the agreed subset (83
"that N", 61 `it`) under each coder's features.

**"that N"**

| Rule | c1 on c1's 87 | c2 on c2's 92 | Agreed 83, c1 | Agreed 83, c2 |
|---|---|---|---|---|
| AC | 29/0/58 | 27/0/65 | 29/0/54 | 27/0/56 |
| AC>D | 61/24/2 | 54/33/4 u1 | 58/23/2 | 51/28/4 |
| AC>D′ | 72/13/2 | 71/16/4 u1 | 69/12/2 | 65/14/4 |
| **AC>R1** | **76/11/0** | **78/11/2 u1** | **73/10/0** | **71/10/2** |

- AC>R1 errs on the **same ten cards** under both keys: Tahngarth, Bronze
  Bombshell, Ram Through, Unpredictable Cyclone, Gisela, Eriette, Solphim,
  Runesword, Scythe of the Wretched, Mangara's Equity. The eleventh differs:
  Noetic Scales for coder 1 (coder 2 codes "its owner" inaccessible, `dep`),
  Tragic Banshee for coder 2.
- AC keeps **zero wrong** under both keys.
- AC>D′ goes from 13 to 16 wrong; AC>D from 24 to 33. With coder 1's labels
  and accessibility but coder 2's distance and role alone, AC>D′ on the 87 is
  68/15/4 and AC>D 54/29/4: any rule that reads raw distance across clause
  types is fragile.

**Bare `it`**

| Rule | c1 on c1's 61 | c2 on c2's 96 | Agreed 61, c1 | Agreed 61, c2 |
|---|---|---|---|---|
| R1 | 46/15/0 | 72/24/0 | 46/15/0 | 46/15/0 |
| **D′** | **60/0/1** | **88/7/1** | **60/0/1** | **60/1/0** |
| A>D′ | 59/1/1 | 88/8/0 | 59/1/1 | 60/1/0 |

- **D′ on coder 1's 61 with coder 2's features is 60/1/0.** Grip of Phyresis
  is the one flip to wrong (coder 2 codes the Equipment as d0); Cocoon goes
  from refused to correct. D′'s score on the shared set does not hinge on
  coder 1's distance codes.
- **The ambiguous count rises from 61 to 96 of 150**: about 2,406 of 3,759
  occurrences (Wilson 2,107–2,678), against coder 1's ~1,529. Of the 35 extra
  sites, 28 are carrier self-mentions that coder 1's `c` had removed; the rest
  are 2 hands (Brain Maggot, Elite Spellbinder), 2 merge or for-each recodings
  (Mizzix's Mastery, Wedding Announcement), Serene Master (`i`) and Alaundo
  the Seer (`s`).
- **D′ on coder 2's 96 is 88/7/1.** Two wrong are the `X` hand sites (a format
  artifact); five are real: Grip of Phyresis, and four carrier sites where the
  self mention outranks the intended referent (Bloodtracker, Dusk Urchins,
  Vogar, Engulfing Flames). That scales to about 125 wrong in the population
  (54–284), against coder 1's 0 (0–94).

### Carrier and fold-state: the single driver

The keys differ on almost nothing except coder 1's carrier and fold-state
exclusion (`n=c`, [CR#109.2]), which coder 2 did not apply:

- It accounts for 28 of the 35 extra ambiguous `it` sites and 4 of the 5 real
  D′ errors.
- The 8 fold-state "that N" sites (Survival of the Fittest, Takklemaggot,
  Next of Kin, Tragic Banshee, Necrotic Plague, Devour Intellect, Vampiric
  Embrace, Infectious Rage) are ambiguous only to coder 2. Tragic Banshee
  becomes an AC>R1 error.
- Loki and Reincarnation become AC>R1 refusals, because coder 2 marks both
  referents as clause-mates (`m`) where coder 1 did not (judgment 2 below).

### Format limits

- **Parallel disjuncts** ("that player or permanent" against "an opponent or
  a permanent an opponent controls"): no kind for them; 2 sites (Gisela,
  Solphim), coded kind c loosely by coder 2 and in neither coder 1's kind c.
  Both are AC>R1 errors under both keys.
- **`m` in small clauses** ("counters on it"): 6 sites. If the intended
  referent were coded `m` there, A>D′ and A>R1 would each go correct to wrong
  on 5 and correct to refused on 1; A and Aarg refused to wrong on 4. D′ and
  R1 ignore `m`. The consequence: whether AC really is zero-wrong depends on
  this convention.
- **`X` is scored as wrong**: 6 hand sites, of which 2 (Brain Maggot, Elite
  Spellbinder) are ambiguous to coder 2 and count as wrong against every pick
  rule.

**Pins.** Coder 2 did not code them. The pin result above (15 of 21 flip: 6
intended, 0 wrong, 9 undeterminable) stands as coder 1's alone.

### Put to the owner (2026-10-07)

The two judgments the agreement report raises, as it states them:

1. **Is the carrier and fold-state exclusion part of accessibility?** That
   is, does the checker resolve against "the candidates the verb can apply
   to"? Or is it a later filter whose errors count against the resolution
   rule? The two keys differ on almost nothing else, and the D′ zero-error
   claim and the ambiguity estimate depend on it.
2. **Do possessor and small-clause clause-mates count as `m`?** This covers
   Loki, Reincarnation, the six "counters on it" sites and Noetic Scales'
   `dep`. It moves AC and the A-family rules by several sites, and so it
   decides whether AC really is zero-wrong.

Both were put to the owner on 2026-10-07 with the orchestrator's
recommendation (A) for each: (1) accessibility, since bindings already carry
zone and kind; (2) co-arguments only, per the pin `badItAcrossOwnSlot` and
the Weeping Angel corpus case.

Owner, 2026-10-07: both accepted, **provisionally**, in the owner's words:
"ok to both I guess. we'll try this way for now anyhow".

1. **The "cannot apply" exclusion is part of accessibility**, applied before
   any resolution rule runs. A mention the verb cannot apply to by zone or
   kind is not a candidate: a creature that died cannot take a counter, a
   non-permanent cannot be sacrificed [CR#109.2,701.21a]. This is coder 1's
   carrier and fold-state exclusion. The checker's bindings already carry zone
   and kind, so it is a condition on the referent in the DRT sense.
2. **Clause-mates are the co-arguments of the verb only.** Possessors and
   small-clause objects ("counters on it") are not clause-mates. This keeps
   the pin `badItAcrossOwnSlot` and the Weeping Angel corpus case.

## Decided 2026-10-07

The owner accepted the measured rules as the target ("ok"). Recorded as the
§7 ruling of 2026-10-07 in `docs/decisions/semantics-v2.md`.

- **Bare `it`: clause recency, with grammatical role as the tie-break** (D′
  above). 60 correct on the 61 ambiguous sample sites under both coders'
  features: 60 / 1 / 0 with coder 2's (Grip of Phyresis the one wrong),
  60 / 0 / 1 with coder 1's (Cocoon refused).
- **"that N": clause-mate exclusion, then most recent** (AC>R1). 76 / 11 / 0
  on the 87 ambiguous sites, wrong on the same cards under both coders once
  judgment 1 settles Tragic Banshee: Tahngarth, Unpredictable Cyclone,
  Eriette, Runesword, Scythe of the Wretched, Bronze Bombshell, Gisela,
  Solphim, Ram Through, Mangara's Equity, Noetic Scales.
- **Those 11 are KNOWN-WRONG sites.** Each resolves silently to the wrong
  referent; the rendered English is identical, so the round-trip cannot catch
  it. Each needs an explicit selector when its card is written, and they
  become a pin list in the refactor.
- **`it` and "that N" keep different rules.**
- **`lean-drt-anaphora-refactor` is promoted from `maybe/` to `planned/`** to
  build what the rules read.

## The work: measure, then report (done 2026-10-06)

Done; the results are "Measured 2026-10-06" above and the decision is
"Put to the owner" below.


1. **Regenerable corpus extraction.** Build the site lists from `data/`
   (`data/scryfall/oracle-cards.jsonl` for Vintage legality and set type,
   `data/derived/cards.jsonl` for text), not from session files. The
   2026-10-05 research ran from session scratch (`corpus.py`, `analyze.py`,
   `dump.py`, `dumprest.py`, `paras.json`, `sites.json`, `pins.py`,
   `count.py`). Of these, `corpus.py`, `analyze.py`, `paras.json`,
   `sites.json` and `pins.py` are retained, in their 2026-10-06 form, in
   `docs/evidence/anaphora-measurement-2026-10-06/`; `dump.py`, `dumprest.py`
   and `count.py` were session-scoped and not retained. The measurement must be
   reproducible without them, and per CLAUDE.md the scripts and their output
   stay out of `crates/` and out of version control.
2. **Classify the 464 "that N" sites** under each candidate rule: whether it
   leaves exactly one candidate (for D, a single top candidate), and whether
   that candidate is the intended referent (the hand classification above is
   the answer key).
3. **Classify a sample of the ~886 same-carrier "it" sites** the same way;
   state the sample size and how it was drawn, with an interval, as the ADR
   did for its numbers.
4. **Run the 19 ambiguity pins** under each rule: how many become the
   intended referent, the wrong one, or stay refused.
5. **Report** to the owner: per kind, resolved correctly / resolved wrongly /
   still refused, for A alone, A with B, A with B and C, D alone, and D applied
   after A to C. A wrong resolution counts against a rule more than a refusal
   does, and under D it is silent; say so with numbers.
6. **State what each rule reads.** For each rule, name what the checker
   context would have to carry to apply it: clause membership, grammatical
   role, antecedent status. `lean-drt-anaphora-refactor` builds whatever the
   accepted rule needs.

## Proof

A report with the tables above, regenerable from `data/`. Any proposed rule
change goes to the owner as a decision, recorded in
`oracle-text-is-forward-anaphoric.md` as a dated ruling only if accepted.

With it, a proposed amendment to the Pronouns section of
`docs/oracle-style-guide.md` stating the rule the corpus bears out, with the
counts as its evidence. The amendment lands only with an accepted rule.

## Out of scope

Changing `PhraseRules.lean`, `Phrase.lean` or any pin.

## Put to the owner (2026-10-06)

From the measurement above. No rule is recommended for implementation; the
strict rule stands ("Decided 2026-10-05") until the owner accepts one.

**`that N`** (87 ambiguous sites), ranked by correct − wrong:

| Rank | Rule | Correct / wrong / refused | Score | Trade-off |
|---|---|---|---|---|
| 1 | AC>R1 | 76 / 11 / 0 | 65 | The existing find-first behind the A and C filters; reads clause membership only. Silent errors concentrate in kind c (Tahngarth, Unpredictable Cyclone, Eriette, Runesword, Scythe, Bronze Bombshell, plus the disjunct cases Gisela, Solphim) and kind f (Ram Through, Mangara's Equity, Noetic Scales). Never refuses. |
| 2 | AC>D′ | 72 / 13 / 2 | 59 | Gets 5 of 8 kind-c sites right; fails "Whenever X deals damage to a creature, … that creature" by picking the subject. Needs role and clause stamps. |
| 3 | AC>D | 61 / 24 / 2 | 37 | Silent errors are high. |
| 4 | ACwB1 | 40 / 8 / 39 | 32 | The Givenness reading (the demonstrative avoids the subject in focus). Refuses many. |
| 5 | AC (= A+B+C) | 29 / 0 / 58 | 29 | **Zero wrong**: a pure narrowing of the count that keeps the `countBy` gate shape. Leaves 58 refused. |

Strict scores 0; pure D −11.

**Bare `it`** (61 ambiguous sampled sites, ≈1,529 occurrences):

| Rank | Rule | Correct / wrong / refused | Score | Trade-off |
|---|---|---|---|---|
| 1 | D′ (clause recency, then role, refusing ties) | 60 / 0 / 1 | 60 | 0 wrong (upper bound ~94 occurrences). Needs every binding stamped with role and clause; replaces the count with a pick and the `countBy` witnesses with ranking lemmas. |
| 2 | A>D′ | 59 / 1 / 1 | 58 | Adds Weeping Angel as an error unless A exempts possessors (Aarg>D′ = D′). |
| 3 | R1 | 46 / 15 / 0 | 31 | Existing mechanism; 12 silent errors on copy templates. |
| 4 | B2 / A+B2 | 30 / 1 / 30 | 29 | Still a filter, so the gate shape survives; half stay refused. |

Pure D scores 9: role-first ranking lets trigger subjects beat searched cards.

**Pins.** Under "`it`: D′, `that N`: AC>R1", 15 of 21 flip: 6 intended, 0
wrong, 9 undeterminable. Under strict, none flip.

**Open judgments:**

1. **Pick or filter.** Buy a pick with silent errors (`that N` 11 of 87; `it`
   0 of 61 in the sample), or keep a filter (AC, B2) at 0–1 wrong with roughly
   half still refused?
2. **Separate rules.** The data supports different rules for the two forms:
   `it` by clause recency with role ranking, `that N` by disjoint reference
   then recency. The demonstrative's kind-c failures are where they differ
   most.
3. **What counts as a clause-mate.** Including possessors (the Kalitas reading)
   breaks Weeping Angel and P05/P08; Principle B proper (Aarg) costs Kalitas
   and Broken Visage.
4. **Fold-state for `that N`.** Give `that N` the carrier and fold-state
   scoping of `ItAt`? That alone settles 8 sites, Vampiric Embrace among them.
5. **Accessibility.** The key's accessibility (quantifier scope, if/otherwise,
   quoted abilities) is the premise of every number; confirm it matches the
   checker before trusting the strict baseline.
6. **Blind recoding** of the ambiguous sites.
7. **Reciprocals.** "fight" [CR#701.14a] and "each other" need a pair binder
   that no rule here supplies.

## Landing record

Research ticket: its work (measure, then report) is complete and the owner
has decided. Stamped on change `nzzrkstw` (commit "tickets: anaphor
measurement done, promote DRT refactor"), whose parent `qvlstlsv` carries the
ruling. No coverage command ran, so no lock `covered` count is stamped.

### PROVE

- **Reproduction.** The 2026-10-06 measurement regenerated the site list from
  `data/`: 464 "that N" sites with two or more candidates, `sites.json`
  byte-identical to the 2026-10-05 list. `pins.py` found 21 ambiguity pins,
  not the 19 this ticket listed; the two extra are reconciled in "The
  ambiguity pins" (14 `it`-form, 7 `that`-form).
- **Blind second coding.** A fresh coder saw only the site texts. Referent
  agreement: "that N" 94 of 94 determinable sites, Cohen's kappa 1.0 on both
  labels; bare `it` 150 of 150 on referent identity (kappa 1.0 on identity;
  0.81 and 0.96 on the index labels, the shortfall being 12 chain merges). No
  site falls in the disagree class.
- **No code touched; no tests touched.** Restored 0, re-spelled 0, ignored 0,
  added 0, removed 0. No checker file, pin or Rust crate changed; the landing
  edits documentation only.

### DISCLOSE

- **The two judgments** (decided above, provisional): accessibility includes
  the cannot-apply exclusion; clause-mates are co-arguments only. Judgment 1
  decides the ambiguity estimate and D′'s zero-error claim; judgment 2
  decides whether AC is zero-wrong (Loki, Reincarnation, six "counters on it"
  sites, Noetic Scales).
- **The 11 known-wrong "that N" sites** under the accepted rule, listed under
  "Decided 2026-10-07"; routed to `lean-drt-anaphora-refactor` as a required
  pin list.
- **Ambiguity estimate under each judgment 1 reading.** With the exclusion
  (accepted): 61 of 150 sampled `it` sites ambiguous, ~1,529 of 3,759
  occurrences. Without it: 96 of 150, ~2,406 of 3,759.
- **Format limits** (see "Format limits"): no kind for parallel disjuncts
  (Gisela, Solphim); the `m` convention in small clauses, now settled by
  judgment 2; `X` hand sites scored as wrong.
- **Deviations.** Coder 2 coded 95 "that N" and all 150 `it` sites rather
  than the 87 and 61 ambiguous ones, because it applied no carrier exclusion
  and judged ambiguity itself. The 2026-10-06 kind split differs from
  2026-10-05: a 33 / b 22 / c 8 / d 2 / e 3 / f 9 (was 29 / 22 / 11 / 4 / 2 /
  9) plus a new kind g, 44 sites, which 2026-10-05 did not count.
- **Not done here, routed.** "Proof" above asks for the Pronouns amendment to
  `docs/oracle-style-guide.md` with an accepted rule; it is routed to
  `lean-drt-anaphora-refactor`. The ruling is recorded in `semantics-v2.md`
  §7 rather than in `oracle-text-is-forward-anaphoric.md`, which gets a
  dated note pointing at it.
- **Glossary.** Added **Clause-mate** to
  `docs/contexts/oracle-english/CONTEXT.md` (no CR rule fits). "In focus"
  was not added (it collides with **Focus**). The Game Model **Reference
  Scope** entry still states the strict uniqueness rule as current behaviour;
  routed to the refactor.
- **STOPs:** none.

### REPORT

- Evidence (ignored local directories, not version-controlled):
  `docs/evidence/anaphora-measurement-2026-10-06/` (coder 1: report, keys,
  `corpus.py`, `analyze.py`, `dump_that.py`, `extract_it.py`, `pins.py`,
  `rules.py`, `score_*.py`, `breakdown.py`, `reciprocals.py`, `*.out`);
  `docs/evidence/anaphora-blind-2026-10-07/` (coder 2: `key2_that.txt`,
  `key2_it.txt`, `judgments.md`);
  `docs/evidence/anaphora-agreement-2026-10-07/` (`agreement-2026-10-07.md`;
  `agree.py`, `lists.py`, `dcheck.py`, `c2only.py`, each with `*.out`).
- Commits that carried the measurement into this ticket: `nqzpvrlo`
  (candidate rules from prior art, 2026-10-05), `mwwtqtxr` (measurement,
  2026-10-06), `xqlsnruk` (blind coding and agreement, 2026-10-07).
- **Performance advisory: not applicable.** No coverage command ran; the
  landing changes no code or consumed data.
