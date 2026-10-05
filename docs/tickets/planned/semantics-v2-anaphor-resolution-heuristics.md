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

## Candidate rules — all UNTESTED

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

## The work: measure, then report

1. **Regenerable corpus extraction.** Build the site lists from `data/`
   (`data/scryfall/oracle-cards.jsonl` for Vintage legality and set type,
   `data/derived/cards.jsonl` for text), not from session files. The
   2026-10-05 research ran from session scratch
   (`/tmp/claude-1000/-home-msmorgan-Projects-deckmaste-rs/d7d9fdb6-2e51-4168-8aa6-b98479d6bbb6/scratchpad/`:
   `corpus.py`, `analyze.py`, `dump.py`, `dumprest.py`, `paras.json`,
   `sites.json`, `pins.py`, `count.py`); those files are temporary and may be
   gone. Use them as a reference if present; the measurement must be
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
