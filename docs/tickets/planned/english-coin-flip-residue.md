---
needs: []
---
**Coin-flip shapes `[CR#705.1..705.3]` the round `chance` deliberately left.**
Campaign-internal residue of `english-structural-recovery-zero`, split out so
the diagnosis survives in the tree; fold into the live campaign workspace with
`workflow claim english-coin-flip-residue --into
english-structural-recovery-zero`.

Round `chance` (2026-07-27) added `flip` as a count noun (95 clause rows) and a
typed `come up heads`/`come up tails` result predicate — `CoinSide` plus
`PredicateElement::CoinResult`, licensed only by a narrow `Come` frame that is
incomplete until the tail attaches (9 of 13 targeted rows). Counts below are
unresolved `clause` rows against the post-`chance` census (clause 3744/69039,
unknown 5468).

1. **The other `come` frames — deliberately not added.** `Vocab::Come` was
   given ONLY the coin-result frame, with no general OPEN frame, precisely to
   keep `whichever comes first` (Squee's Revenge) and `came under your control`
   out of the blast radius. Those are valid `come` frames and need their own
   subcategorization work. Whoever widens `Come` must re-run the coin-result
   negative gates, because a general frame re-licenses the tail everywhere.

2. **Possessive-relative subject `each player whose coin comes up tails` — 3
   rows** (Goblin Assassin, Mana Clash, Rakdos the Showstopper). The
   coin-result predicate itself is correct on these; the host has no
   possessive-relative production. Blocked on that gap, not on the coin
   procedure.

3. **`The first time you flip one or more coins each turn, ...` — 1 row**
   (Edgar, King of Figaro). Missing a `the first time` fronting connective.
   Same note: the coin-result predicate is not the blocker.

4. **Flip-as-object-gap and quantified-flip rows — 3 rows.** Goblin
   Traprunner's `For each flip you win, ...`, Squee's Revenge's `If you win all
   the flips, draw two cards for each flip.` and `Flip a coin that many times or
   until you lose a flip, whichever comes first.` The noun `flip` now exists;
   what is missing is the object-gap relative, the `all the N` predeterminer,
   and the `whichever comes first` correlative respectively — three unrelated
   host gaps, listed here only so a `flip` grep finds them.

5. **`Repeat this process until ...`, `You and target opponent each flip a
   coin.`, and the remaining coin-adjacent rows** fail on coordinated subjects
   with post-subject `each` and on other host shapes outside the coin
   procedure. Attributed in `recovery-harness/out/chance-plan.md` Appendix A;
   none of them are coin-procedure work.

**Standing caution for anything touching this area:** `tail` is an ordinary
count noun in `regular-vocabulary.tsv`, and that entry is NOT the designated
coin side. `heads` and the designated `tails` are carried only by the closed
two-word `up heads`/`up tails` scanner licensed by `[CR#705.1,705.2]`; do not
add either as a noun, an adjective, or a general opaque-reserved literal.
