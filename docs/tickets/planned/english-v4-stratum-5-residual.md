---
needs: [english-v4-stratum-5-comparatives-measures-and-partitives]
---
# Stratum 5 residual: thirty-three deferred units

Close the thirty-three Stratum 5 units listed under `english-v4-stratum-5-residual`
in `crates/deckmaste_english_v4/deferred.txt`. Groups 1 and 2 wait on a ruling;
the rest need grammar or lexicon work.

## Groups

1. **Predicative relative under affirmative *be*** (Mana Cannons, Crystal,
   Inhuman Princess, Tablet of the Guilds, Seal of the Guildpact, Bound, Jared
   Carthalion: *the number of colors that spell is*, *for each of the chosen
   colors it is*). The Stratum 4 residual ruling excludes affirmative
   auxiliary *be* from `BarePredicativeRelative` so that *the player it's
   attacking* keeps one Reading. **Needs a ruling** refining that exclusion
   (for instance, admitting it when the relative clause ends at the gapped
   copula).
2. ***any number of other target X*** (Appa, Steadfast Guardian; Broodlord;
   Guardian of Faith; Lagrella, the Magpie; Lathiel, the Bounteous Dawn; Silver
   Surfer, Cosmic Voyager). `adj:target` requires overt determination so that
   it never duplicates determinative *target*, but the oblique of *any number
   of* has no determiner. **Needs a ruling** on adjectival *target* in a
   determinerless oblique. The judgment `phase_in_out.txt:19` is pending while
   Guardian of Faith stays deferred.
3. **Pro-form *one*** (Ezuri's Predation: *a different one of those
   creatures*; Coveted Falcon: *each one they gained control of*; Xyris, the
   Writhing Storm and Leela, Sevateem Warrior: *the first one they draw*).
4. ***divided evenly, rounded down, among*** (Fireball, Dwarven Catapult): a
   participial Supplement interrupting a participial postmodifier.
5. ***N {P} worth of modes*** (the five Bloomburrow Seasons).
6. **Single units:**
   - *pay {1}{G} any number of times*: a transparent quantity as a frequency
     Adjunct (Taste of Paradise);
   - *for each card fewer than three in their hand*: a postpositive comparative
     determinative (Stabwhisker the Odious);
   - *can boast*: the keyword as a verb (Birgi, God of Storytelling);
   - *to a maximum of 10* (Diligent Zookeeper);
   - *Each of X target creatures you control connive*: a plural verb after a
     distributive partitive, and *any number of them* (Change of Plans);
   - *divide that creature's combat damage as you choose among any of the
     creatures blocking it* (Defensive Formation);
   - *dealt to you so far this turn by artifacts* (Reverse Polarity);
   - *if you rolled 6 exactly seven times* (Luck Bobblehead);
   - *up to twice X target cards* (Erebos's Intervention);
   - *more creatures than you do as you activate this ability*: a comparative
     clause; the ruling for the Stratum 5 ticket removed *than*'s finite-clause
     frame until comparative clauses (obligatorily reduced, CGEL, Ch. 13 §2)
     land (Keeper of the Beasts).

## Gate

- `crates/deckmaste_english_v4/deferred.txt` lists no `english-v4-stratum-5-residual`
  unit.
- 100% of corpus units in Strata 0–5 parse as **read** (`cargo xtask english-v4
  strata --check`), with 0 `known-wrong-only`, 0 `unread`, and 0
  `law-failures`.
- All judgment suites pass with no entry pending for a deferral.

Standard constraints apply.
