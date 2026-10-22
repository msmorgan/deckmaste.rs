---
needs: []
---
# Rank a face's own name before another card's name spelled alike

`english-v4-look-alike-duplicate-readings` left 57 units whose Readings
differ only in which card a name leaf names. In most, a face's own Shortened
Name is spelled like another card's full name: *Nicol Bolas* on Nicol Bolas,
the Ravager, *Ayesha Tanaka* on Ayesha Tanaka, Armorer. Both are Readings,
since full names are in scope everywhere (`english-lexical-analysis.md` §1).

Owner ruling (round 5, item 10):

- Keep other cards' full names in scope.
- Rank the face's own name, its Full Name or Shortened Name, cheaper. Do it as
  Production Cost on the name, never by suppressing Admission.
- The extra Readings are accepted; they at most double a unit's count.

The witnesses are 57 units on 45 faces: Abomination, Irradiated Brute;
Abomination, Terrifying Titan; Avatar Kyoshi, Earthbender; Ayesha Tanaka,
Armorer (2); Boomerang, Blade Flinger; Brawn, Amadeus Cho (2); Chaos, the
Endless; Chromium, the Mutable; Cloak and Dagger, Entwined; Colossus, Steel
Stalwart; Genesis of the Daleks; Ghost-Spider, Gwen Stacy; Hornet Cannon;
Human Torch, Johnny Storm (2); Jared Carthalion, True Heir (2); Jaya Ballard,
Task Mage (2); Jedit Ojanen of Efrava; Jedit Ojanen, Mercenary; Justice, Vance
Astrovik (2); Madame Hydra, Reanimated (2); Massacre Girl, Most Wanted (2);
Memnarch, the Warden (2); Mockingbird, Ace Agent; Mockingbird, Bobbi Morse; Mu
Yanling, Wind Rider; Nicol Bolas, Dragon-God; Nicol Bolas, God-Pharaoh; Nicol
Bolas, the Arisen; Nicol Bolas, the Deceiver; Nicol Bolas, the Ravager;
Palladia-Mors, the Ruiner; Pestilence Rats; Phenomenon Investigators; Ramirez
DePietro, Pillager; Rescue, Pepper Potts; Shocker, Unshakable (2); Sidar
Jabari of Zhalfir; Splinter, Aging Champion (2); Splinter, the Mentor; Squall,
Gunblade Duelist (2); Squall, SeeD Mercenary; The Thing, Ben Grimm; Tor Wauki
the Younger; Venom, Eddie Brock; Wirefly Hive.

## Change

- Whether a name leaf is the face's own is a property of how analysis admitted
  it: through the face's scope (the face's own name, or a card-scoped spelling)
  or not. It is never a property of the word.
- The Production that takes the name costs one more where its leaf did not
  come in through the scope: another card's full name.

## Done when

- Readings, Surplus, verdicts and faces are unchanged.
- In each witness unit that has another card's name, every cheapest-tier
  Reading uses the face's own name.
- The judgment-set ranking (S/A/P/F) does not regress.

Standard constraints apply.

## Record

- Analysis records the face's scope on the analyzed text, and the chart marks
  each leaf it admitted through that scope: one of the face's own names, or a
  card-scoped spelling (a Shortened Name, an Introduced Name, an Anchor Word).
  A Production's new `scoped` child costs one more where its leaf is not so
  marked. `NamedParameter`, the Production that takes a name, declares its
  head `scoped`. The tree's key reads the same scope, so the forest and tree
  keys agree.
- Of the 57 witness units, 55 have another card's full name. In all 55, every
  cheapest-tier Reading now uses the face's own name; before, 53 had the other
  card's name in the tier. In the other two, Hornet Cannon and Wirefly Hive,
  *named Hornet* and *named Wirefly* read as the token's Introduced Name or as
  the face's Shortened Name. Both are admitted through the scope, so they stay
  tied, and the Introduced Name is the intended one.
- The cost is on the other card's name, not a discount on the face's. A
  discount was measured first. It ranked Assembly-Worker's own name over the
  subtype in *Target Assembly-Worker creature* on its own text, and a split
  of *Pestilence Rats* into its Shortened Name beside its full name. The cost
  on another card's name changes neither. It also ranks a subtype first where
  a card shares its name (*Blood*, *Forest*, *Shapeshifter*, *Assembly-Worker*
  on Mishra's Factory).
- The cheapest tier changes in 209 units, and it shrinks in every one.
- Judgment-set ranking, over single-group entries with 2 to 4,000 Readings and
  an unendorsed one: S 211, A 19, P 10, F 76 before; S 212, A 19, P 9, F 76
  after. `noun_premodifiers.txt:7` moves from P to S. No entry moves down.
- `english-lexical-analysis.md` §1 now scopes a Shortened Name to its face
  (owner ruling, round 5, item 9) and ranks the face's own name first.
  `english-v4.md` §4.3 records the cost.

Census delta (measured on murlqoum → pwrtmrzl): faces read 28,798 → 28,798; total Readings 213,193 → 213,193; Surplus 88,433 → 88,433; cheapest tier singleton in 14,604 of 16,560 → 14,801 of 16,560; no verdict changed; lost faces: none.
Judgment set: 3,029 enforced at `COMPLETED_STRATUM` 6, all hold; 208 pending.
