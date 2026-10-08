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

## Decided 2026-10-07

The owner answered all four decisions "yes"; the ruling is in
[the spelling/grammar ADR](../../decisions/builtin-v2-macro-spelling-and-grammar.md),
under the 2026-10-07 amendment.

1. Amount stays an arabic numeral by default. `:word` is the second and
   last hint, valid on an Amount only, for counts that print as words
   (create, mill).
2. The subject is never a slot: a template is the predicate only and the
   clause supplies the subject. Fight's first creature is a leftover from
   before the actor handoff (2026-10-05: the performer is `actor`, never a
   parameter). Fight becomes `params: [Subject]` with `actor` as the first
   creature, and its callers become `act(<first>, fight(<second>))`.
3. Literal words that no grammar frame has are free in a template. The
   consistency test checks only the head rule and the slots against the
   params. Owner remark, no action: "some words are core lexicon words, but
   now that there are wrapping macros for everything, there's a natural
   place to put things like `creature`. pretty sure that it's in the core
   lexicon at this time though; no need to fix yet."
4. One slot per parameter, in card order, with literals between. The
   realizer carries number from a count slot into the noun slot after it.
   There is no combined-slot syntax. For TokenSpec, Instruction, Disclosure,
   Ballot, SearchScope, Quantity and String the default is the argument's
   own realization, as for Subject and Ability.

## Templates

Each template is quoted against its target card's Oracle text.

| Action | Template | Card text |
|---|---|---|
| behold | `behold a <Param(0)>` | Champion of the Weird: "behold a Goblin and exile it" |
| create | `create <Param(0):word> <Param(1)>` | S.H.I.E.L.D. Helicarrier: "create two 1/1 white Soldier creature tokens" |
| faceAVillainousChoice | `face a villainous choice — <Param(0)>, or <Param(1)>` | The Dalek Emperor: "each opponent faces a villainous choice — That player sacrifices a creature of their choice, or you create a 3/3 black Dalek artifact creature token with menace" |
| meld | `meld <Param(0)> into <Param(1)>` | Hanweir Battlements: "meld them into Hanweir, the Writhing Township" |
| mill | `mill <Param(0):word> cards` | Devoted Grafkeeper: "mill two cards" |
| search | `search <Param(0)> for <Param(1)> <Param(2)>` | Screaming Seahawk: "search your library for a card named Screaming Seahawk" (scope, quantity "a", quality "card named …") |
| vote | `<Param(0)> vote for <Param(1)>` | Magister of Worth: "each player votes for grace or condemnation" (open disclosure, nothing printed); Elrond of the White Council: "each player secretly votes for fellowship or aid" |
| fight | not written (STOP below) | Hunt the Hunter: "It fights target green creature an opponent controls" |

## Landing record

Measured on the refreshed tree, change `zvvuzywz` (claim `twmnvwkl`). This landing changes no
English grammar or corpus parse, so no lock count was measured.

- **Stages.** `nmunomuo` reader (`:word`); `qrzuzkqw` seven templates;
  `zvvuzywz` ADR, ticket and this record. Stage (c), fight, did not land
  (STOP below).
- **Hint vocabulary:** `plural` (3 uses) and `word` (2 uses: create's and
  mill's count). `word` on a kind other than Amount is
  `ValidationError::SpellingHintKindMismatch`.
- **STOP, open: fight.** The trial (fight `params: [Subject]`, spelling
  `fight <Param(0)>`, body `dealDamage(actor, powerOf(actor), Param(0))`
  and `dealDamage(Param(0), powerOf(Param(0)), actor)`; Prey Upon as
  `act(target(creatureYouControl), fight(target(creatureYouDontControl)))`)
  made `cargo xtask lean check` canon 133/134: Prey Upon refused with
  `[Refusal.kindMismatch player object, Refusal.enactAgentOk]`. The Lean
  checker's `handedOne` hands an enacted deed the actor, and fight's facts
  row (`lean/Semantics/Check/Words.lean`, `("fight", {})`) names no
  performer. This is the recorded ruling in the semantics-v2 ADR §7,
  amendment 2026-10-06: "A deed whose facts row names no performer ("fight"
  [CR#701.14a]) records none, and is refused when a handoff names one."
  That ruling contradicts D2, and the fix is in lean/, which this landing
  may not edit. The trial was reverted. Fight keeps the bare head and stays
  in `PENDING_RULING`, which now holds only `"fight"`.
- **Deviations and additions.** (1) Stage (b) wrote seven templates, not
  eight, and `PENDING_RULING` holds fight (STOP). (2) Vote's template opens
  with a slot: the disclosure prints before the verb ("secretly votes").
  It is a predicate modifier, not the subject (D2). The reader's head rule
  already allowed a leading hole ("<Param(0)> offering"). The keyword-action
  head assertion in `builtin_v2_keyword_actions.rs` read only the first
  part, so it was re-spelled to read the first literal and to require a
  space between that literal and a leading hole. (3) The consistency test
  keeps its every-param-exactly-once check (D4 makes every template
  satisfy it), and it checks no literal against a frame (D3).
- **Proofs** (after `kata refresh`, exit 0). `cargo test -p … --no-fail-fast`:
  deckmaste_construction_core 464 passed; deckmaste_lexical_source 56;
  deckmaste_semantics_v2 96; xtask 389 passed, 1 ignored (an existing
  ignore that this landing did not touch); 0 failed in each. `cargo clippy
  -p <crate> --tests -- -D warnings` is clean for all four.
  `cargo xtask lean check`: canon 134/134, testing 7/7, no lean/ edits.
  `cargo xtask expansions` on the claim and on the stack: `diff -r` shows
  no difference. Fight did not change, so no expansion is expected to.
  `cargo xtask lexical --card-name "Black Lotus" --export`: the export
  RON is byte-identical before and after (sha256 prefix
  `cc171fe4a39e8115`). `cite check --list-noncompliant`: 0. `cite check`:
  0 stale. `cite audit --diff` from the claim: 3 sites (701.17a, and
  701.14a twice), each read against its claim. `kata kanban check`: OK.
- **Counts.** Restored 0. Re-spelled 1: the keyword-action head assertion
  (`builtin_v2_keyword_actions.rs`). Ignored 0. Added 1: the reader unit
  test for `:word`. Removed 0. Updated in place: seven template pins, and
  `PENDING_RULING`, which went from eight names to one.
- **Glossary gap:** none found.
