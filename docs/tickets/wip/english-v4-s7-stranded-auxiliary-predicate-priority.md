---
needs: []
---
# Stratum 7: a Stranded Auxiliary's Subject before a Complement-like predicate

`english-v4-s7-stranded-auxiliaries` reads a Stranded Auxiliary through the
engine's `elliptical` Production flag: before any punctuation, a Constituent
that ends in the ellipsis site is followed only by a coordinator or by the head
it depends on. The head exception keeps *Each opponent who can't discards a
card* (a Subject whose relative clause strands *can't*, then its predicate).

The same exception admits spurious Readings where the head after the Subject
could itself be the omitted Complement, so English takes it as that
Complement and no ellipsis is there (CGEL, Ch. 17 §7.1, p. 1519):

- *Creatures with power less than this creature's power can't block it.*:
  Subject *creatures with power less than [this creature's power can't]*,
  predicate *block it* (plain *block* is *can't*'s Complement).
- *all creature cards in your graveyard that were put there from the
  battlefield*: bare relative clause with Subject *... that were* and
  predicate *put there ...* (past participle *put* is *were*'s Complement).

The count when this ticket was minted: about 2,200 Readings on about 45
units, Muse Vortex 1,776 → 3,552 alone (*the exiled instant and sorcery cards
that weren't cast this way*), then Myojin of Grim Betrayal 76 → 156,
Activated Sleeper 190 → 250 and the *can't block it* family 2 → 4 each.

The fix is structural: a head right after an ellipsis site is admitted only
when no form of its first word could fill the stranded auxiliary's Complement
(*discards* and *loses* cannot; *block* and *put* can). It needs the engine to
know, at the site, which word forms the omitted Complement takes, without
naming grammar features (`docs/decisions/english-v4.md` §4.4, principle 6).
Witness each fix with a `+` entry and a `!VerbPhrase "can't"` assertion in
`judgments/stranded_auxiliaries.txt`.
