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

## Orchestrator's hypothesis — UNTESTED

Most exceptions look like disjoint reference among the arguments of one
clause. A plain pronoun or demonstrative does not refer to another argument of
its own clause (that would be "itself"): in "Another target creature blocks
it", "it" cannot be the blocker; in "Each creature you control deals 1 damage
to that creature", "that creature" cannot be the dealer; in "Its power is
equal to that creature's power", the two cannot be the same creature.
Excluding clause-mates and then requiring uniqueness (or taking the most
recent) may resolve kinds (b) and (d) and the bare-"it" cases; kinds (c) and
(f) would remain. Nothing here has been measured; do not implement it on this
account.

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
2. **Classify the 464 "that N" sites** by whether excluding the
   demonstrative's clause-mates leaves exactly one candidate, and whether that
   candidate is the intended referent (the hand classification above is the
   answer key).
3. **Classify a sample of the ~886 same-carrier "it" sites** the same way;
   state the sample size and how it was drawn, with an interval, as the ADR
   did for its numbers.
4. **Run the 19 ambiguity pins** under the same rule: how many become the
   intended referent, the wrong one, or stay refused.
5. **Report** to the owner: per kind, resolved correctly / resolved wrongly /
   still refused, for (i) clause-mate exclusion plus uniqueness and (ii)
   clause-mate exclusion plus most recent. A wrong resolution counts against a
   rule more than a refusal does; say so with numbers.

## Proof

A report with the tables above, regenerable from `data/`. Any proposed rule
change goes to the owner as a decision, recorded in
`oracle-text-is-forward-anaphoric.md` as a dated ruling only if accepted.

## Out of scope

Changing `PhraseRules.lean`, `Phrase.lean` or any pin.
