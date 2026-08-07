---
needs: []
---
**[design]** Gated on an external event: if and when oracle templating extends
named-character personal pronouns to `they`/`them`/`their`. Nothing to build
until then — this records the analysis so it isn't re-derived.

Today `they`/`them`/`their` is unambiguous in oracle text: plural objects, or a
player. `docs/oracle-style-guide.md` §5 (Pronouns) permits personal pronouns
only for an established named character, and lists exactly `he, him, his, she,
her`. A named character who uses they/them has no licensed form under the guide
as written and falls through to `it`.

Two consequences if the inventory gains `they`:

**Anaphora.** `Whenever [named character] attacks, they gain hexproof until end
of turn.` Both readings — the character, and the controller — are rules-legal,
so resolution has to know the antecedent is a named character taking personal
pronouns. That is precisely the clause in §5 that is not machine-checkable
today: "reserve that treatment for a specific character whose identity supports
it."

**Lint.** A rule flagging gendered pronouns as anachronism (the `he or she` /
`his or her` ban in §1) would false-positive on the named-character usage §5
explicitly allows. The anachronism check and the personal-pronoun check are
different rules over different scopes and must stay separate.

## Decisions already made

- **Do not pre-build a character-identity slot.** Rules-legality already prunes
  the overwhelming majority of `they`: most predicates sort cleanly into
  player-only (draw, discard, lose life) or object-only (attack, be sacrificed),
  and lowering rejects the illegal reading without consulting identity at all.
  Identity is load-bearing only where the predicate ranges over *both* players
  and permanents — hexproof and protection are both defined on "a permanent or
  player" [CR#702.11c,702.16b], and damage is the other obvious member. That set
  is small and enumerable from the CR, so the eventual change is local.
  Speculative generality now buys nothing.

- **This is a second instance of the style-layer epoch requirement, not a new
  one.** `his or her` is well-formed, unambiguous, correct-for-its-era text;
  flagging it needs a notion of *current* templating distinct from *parseable*.
  Same machinery.

## When it triggers

1. Enumerate the player-or-permanent predicate set from the CR.
2. Add a character-identity attribute to canon legendary and planeswalker names.
   Narrative identity — not derivable from the type line or the supertype,
   though the two mostly coincide.
3. Resolution rule: consult identity only when a `they` antecedent is a named
   character *and* the predicate is in that set. Everywhere else lowering
   already decides.
4. Style guide: extend §5's pronoun inventory; leave §1's anachronism ban scoped
   to `he or she` / `his or her`.
