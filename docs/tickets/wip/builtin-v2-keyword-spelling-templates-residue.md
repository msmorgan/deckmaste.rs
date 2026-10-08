---
needs: []
---
**Eight keyword actions still spell their bare head, because no frame of
their grammar takes their params one slot each.** This is what is left of
`builtin-v2-keyword-spelling-templates`, which made every other keyword
spelling a card-usage template. The ruling is the 2026-10-07 amendment to
[the spelling/grammar ADR](../../decisions/builtin-v2-macro-spelling-and-grammar.md),
and its default table is there too. The test
`crates/deckmaste_construction_core/tests/builtin_v2_keyword_spelling_templates.rs`
lists these eight in `PENDING_RULING`. It fails as soon as one of them gets
a template, so take the name out when you write it. Standard constraints
apply.

## The eight

Each entry gives the action's params, its grammar frames today, one
Vintage-legal card's Oracle text as the target, and why slot-for-slot
derivation fails.

- **behold** [Quality]. Frame: Transitive. Target: Champion of the Weird,
  "behold a Goblin and exile it". Fails because the article "a" is in no
  frame.
- **create** [Amount, TokenSpec]. Frame: Transitive. Target:
  S.H.I.E.L.D. Helicarrier, "create two 1/1 white Soldier creature
  tokens". Fails because two params share the one object slot, and the
  count prints as a number word.
- **faceAVillainousChoice** [Instruction, Instruction]. Frame:
  Intransitive. Target: The Dalek Emperor, "each opponent faces a
  villainous choice — That player sacrifices a creature of their choice, or
  you create a 3/3 black Dalek artifact creature token with menace." Fails
  because neither option has a slot, and " — " and ", or" are in no frame.
- **fight** [Subject, Subject]. Frame: Transitive. Target: Hunt the
  Hunter, "It fights target green creature an opponent controls." Fails
  because the first creature is the clause subject, not a frame slot.
- **meld** [Subject, String]. Frame: Transitive. Target: Hanweir
  Battlements, "meld them into Hanweir, the Writhing Township". Fails
  because there is no "into" slot for the name.
- **mill** [Amount]. Frame: Transitive. Target: Devoted Grafkeeper,
  "mill two cards". Fails because "cards" is in no frame, and the count
  prints as a number word.
- **search** [SearchScope, Quantity, Quality]. Frames: [NP], [for NP],
  [NP for NP]. Target: Screaming Seahawk, "search your library for a card
  named Screaming Seahawk". Fails because there are three params and only
  two slots.
- **vote** [Disclosure, Ballot]. Frames: [], [NP], [for NP]. Target:
  Magister of Worth, "each player votes for grace or condemnation". Fails
  because the disclosure (open, or "secretly") has no slot.

## Decisions owed

1. **A number-word hint** (`:word`) for counts that print as words ("two
   cards", "two … tokens") rather than as the Amount default, an arabic
   numeral. Needed by create and mill. The plural noun that agrees with
   the count is part of the same question.
2. **A subject slot** for fight: whether a template may open with a slot
   for the clause subject ("<Param(0)> fight <Param(1)>"), and how the head
   rule reads that.
3. **Literal words in the template that are in no frame**: "a" for behold,
   "cards" for mill, "into" for meld, "for" in search. These are cases
   where the template says more than the grammar does.
4. **Two params sharing one slot**: create's count and token spec in one
   noun phrase, and search's quantity and quality in one "for" phrase
   alongside its scope.

The defaults for TokenSpec, Instruction, Disclosure, Ballot, SearchScope,
Quantity and String are decided along with these templates.
