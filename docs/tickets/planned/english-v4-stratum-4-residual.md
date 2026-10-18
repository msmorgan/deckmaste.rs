---
needs: [english-v4-stratum-4-subordinate-clauses-and-triggers]
---
# Stratum 4 residual: twenty deferred units

The Stratum 4 landing read every Stratum 0–4 corpus unit except twenty, which
`crates/deckmaste_english_v4/deferred.txt` defers (owner's choice, 2026-10-18).
Each needs a construction that the straightforward production over-generates.
Close a group by adding a production or lexical idiom that reads its units and
admits no new wrong Reading anywhere in Strata 0–4, then delete its lines from
`deferred.txt` (a deferred unit that reads is stale and fails `strata --check`).
A group may also stay deferred into Stratum 7's long tail.

1. **Predicative-gap bare relatives** (Spider Climb and nine siblings: *the
   controller of the permanent it becomes*; Dead Ringers: *a color the other
   isn't*). CGEL limits relativised predicatives to the subjective type
   (Ch. 12 §3.3(c), p. 1045; §3.5.4(g), p. 1054). Tried and withdrawn: a bare
   relative over any Clause with a `Pred` Gap (*Blue [creatures you control
   can't be __] blocked*; *[creatures [it's __]] blocking*); restricted to a
   lexical verb's finite predicate (*turn*'s complex-transitive frame still
   supplies one: *abilities of Teferi on any player's [turn any time …]*, and
   *the last time [it became __] blocked by that Wall*). Restricting to a bare
   copular verb brings back the contracted *'s* cases.
2. ***returned this way* without a goal** (Arid Archway, Foul Renewal). A
   `Monotransitive` frame for *return* would give every *Return X to Y* a second
   Reading with *to Y* as a Modifier of *X* (*to* + NP hosts Nominal).
3. **Postposed bare relative past *at random*** (Infectious Rage: *choose a
   creature at random this Aura can enchant*; Volatile Chimera: *Choose a card
   at random you exiled with …*). `english-v4-stratum-3-residual` admitted only
   a marked postposed relative; a bare one would give *Draw a card for each
   creature you control* a postposed-relative Reading of *you control* on
   *card*.
4. **Singletons**:
   - Infinite Reflection: *enters attached to a creature*, a depictive
     Adjectival Passive with its Complement. The declared Adjectival Passive
     *attached* is predicative only, since a depictive one gives *gets +1/+1
     for each Aura [attached to it]* a depictive Reading on *gets*.
   - Celeborn the Wise: *each card looked at*, a prepositional passive; the
     engine derives passives only from frames whose first slot is an NP.
   - North Star: *For one spell this turn, …*, two preposed Adjuncts before
     one comma.
   - Critical Hit: *roll a natural 20*, a die result as a determined numeral
     with an adjective.
   - Overpowering Attack: *…, followed by an additional main phase*, a
     comma-set past-participial supplement.

Standard constraints apply.
