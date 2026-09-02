---
needs: [english-v2-frame-key-reduction]
---
Dissolution residue (dissolution-landing-review F3/F5/F6), small and
first in the chain so the general-constituent work starts clean.

- F5, the named gains that did not materialize: `Gain` lacks
  Role("KeywordAbility") (`Have` has it) — "gains flying" is 290 corpus /
  0 selected; "as though it were" 67/0; "mana of any type" 61/0. Land the
  general fixes (no per-phrase constructions) and probe each.
- F6: delete the two vocabs duplicating CommonNoun surfaces —
  BareLocativeNoun{exile,hand}, ControllerNoun{opponent,player} — routing
  through the noun inventory.
- F3: re-run the zero-routing construction census (53 remain of the
  audit's 71); for each, delete it or make it the live path. The general
  Possessive root must become the live possessive path (the possessive
  literals it should own carry ~750 units).
- Correct english-v2-pp-construction's count: the per-preposition family
  is ~15 (at_phrase joined it), not 13.

Acceptance: ratchet up or equal, zero ties, no new literals for content
words, the two vocabs gone, census table in the landing record. Standard
constraints apply.

## Landing record (2026-09-02)

| census point | constructions | zero-routed |
| --- | ---: | ---: |
| first exact pass after the initial possessive cleanup | 396 | 45 |
| landed grammar | 356 | 0 |

- Added `KeywordAbility` to `Gain`'s declaration-driven valence. `Target
  creature gains flying.` now selects uniquely through
  `VerbPhraseHaveKeywordAbility`.
- Probed `You may play it as though it were a land.` and `Add one mana of
  any type.`. Both already select through the general clause and noun/PP
  trees, so no phrase-specific constructions or content-word literals were
  added.
- Removed the `BareLocativeNoun` and `ControllerNoun` vocabs. Bare `exile`
  and `hand` locatives now use a checked `Noun` constituent backed by the
  common noun inventory; controller references already use the general noun
  path.
- Routed singular and plural genitives through the general `Possessive`
  root. The corpus proves the singular, plural-reference, plural-nominal, and
  plural-mass routes live, including `their owners' libraries` and `their
  owners' control`.
- Deleted every remaining construction with no selected corpus route and
  removed the corresponding generated API/test residue. The final census has
  no zero-routed construction.
- Confirmed `english-v2-pp-construction` already records the corrected
  per-preposition family count as approximately 15.
- Coverage rose from 15,886 to 15,927 selected-and-covered identities, with
  zero unresolved ties, ownership failures, or round-trip mismatches.
