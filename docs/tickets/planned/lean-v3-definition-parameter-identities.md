---
needs: []
---
# Generate the Binding Identities of a Definition's NounPhrase parameters

Standard constraints apply. Design ticket, design proposed below. Found by the 2026-10-25 triage
of an external audit ("Atraxa"), verified against `wmyykqnv`. Paths are under `lean/Semantics/`.

## Decided

ADR `docs/decisions/semantics-v3.md` §2 (`:292`) enables a Binding Identity for every
`NounPhrase` parameter of a Definition (`firstMention "<decl>.<param>" (inCaller p)` at the
first textual occurrence, `laterMention …` after; §1.4 `:149`; `lean/CONTRACTS.md:1255-1268`).
Authors write these 89 calls by hand. The ADR prescribes the shape, not who writes it; the
elaborator writes them for Definition `NounPhrase` parameters. Templates keep theirs by hand.

## Open item (does not block parts 1-3)

`search.for_` (`Definitions/Actions.lean:715-717`) and `searchZonesOf.for_` (`:176-183`) carry
only `.inCaller for_`, with no identity, against the §2 table. Under this design they keep that
spelling through the opt-out (an author-written `.inCaller p` is left alone), and the elaborated
terms are unchanged. Whether they should be bound is for the owner; until then add a doc comment
on each saying the parameter is deliberately unbound and why it is unknown.

## Design

1. **Where.** In the `semantic_definition` branch of `elab_rules` (`Authoring.lean:266-269`),
   wrap the body syntax before `elabAuthoredDecl` (`Authoring.lean:177`) emits the `def`:
   `body ↦ bind_parameters% "<last name component>" body`. The identity prefix is the
   declaration's last name component (`Action.devour` → `"devour.subject"`, matching today's id at
   `Definitions/Keywords.lean:548`). `semantic_template` is not wrapped.
2. **Algorithm** (a working prototype is
   `/tmp/claude-1000/-home-msmorgan-Projects-deckmaste-rs/4e5532ee-6640-4aad-9769-888a72f918d1/scratchpad/q13/Elab.lean`):
   elaborate the body against the expected type, `synthesizeSyntheticMVarsNoPostponing`,
   `instantiateMVars`. Collect the local context's non-implementation-detail locals whose `whnf`
   type is `Semantics.NounPhrase` (the declared parameters; binders inside the body are not in
   the local context and are untouched). Walk the `Expr` left to right (`.app f a` visits `f`
   then `a`; lambda, let, mdata and proj pass through), which is textual order (ADR §0.1 item 2).
   At a bare `fvar p`, emit `NounPhrase.firstMention "<decl>.<p>" (NounPhrase.inCaller p)` the
   first time and `NounPhrase.laterMention "<decl>.<p>" (NounPhrase.inCaller p)` each later time;
   the identity is outermost (`lean/CONTRACTS.md:1266`). An occurrence already written
   `NounPhrase.inCaller p` is left alone: that is the opt-out.
3. **Guards**, new elaborator errors:
   - (a) a hand-written `firstMention`/`laterMention` whose id is `"<decl>.<q>"` for a
     `NounPhrase` parameter `q` is refused ("parameter identity is generated");
   - (b) an occurrence of a parameter as an argument of a function that is neither a semantic
     constructor nor an authored declaration (for example `p.plur`) is refused, because wrapping
     it would change what the function reads. No Definition has one today.
4. **Hand-written identities that stay:** the performer and non-parameter identities
   `fight.source` (`Definitions/Actions.lean:189,191,195`), `explore.permanent` (`:406,413`),
   `connive.permanent` (`:420,427`), `endure.permanent` (`:667,669`); their ids name no parameter,
   so guard (a) does not fire. Template identities stay: `duringNextTurnOf`
   (`Templates.lean:836-837`), `lookAndSortSlice` (`:1404-1405`), `delay.trigger` (`:1334`),
   `controllerSacrifices` (`Definitions/Actions.lean:543-544`).
5. **Moved referents** need no handling. Every later occurrence becomes a `laterMention`; one
   after a move is refused by the checker's identity clearing (ADR §0.1 item 2, last bullet;
   `movedParameterCannotBeReused`, `Proofs/DefinitionsAndTemplates.lean:66`), and the author
   writes the anaphor, as now. No current Definition mentions a parameter after moving it.
6. **Scope:** `NounPhrase` parameters only. No Definition has an `Option NounPhrase` or
   `List NounPhrase` parameter today. The other `X.inCaller` wrappers stay hand-written.

A probe of this design (`scratchpad/q13/Probe.lean`) stripped and rebuilt the 51 Definitions
with `NounPhrase` parameters: all 51 kernel terms came out identical.

## Change

Part 1: implement §Design 1-3 in `Authoring.lean`.

Part 2: at each site below, delete the hand-written wrapper and leave the bare parameter
(`Definitions/Actions.lean` unless noted; f = `firstMention`, l = `laterMention`):

- `:21` attachTo.subject f; `:22` attachTo.host f
- `:32` counter f; `:43` cast f; `:50` play f; `:57` activate f; `:74` resolve f; `:79` destroy f
- `:84` exile f; `:89` exileFaceDown f; `:96` exileWithCounters f; `:102` sacrifice f;
  `:108` transform f
- `:112` meldInto f; `:117` manifestPlacement f; `:122` tap f
- regeneration: `:130` f, `:132` l, `:133` l, `:136` l, `:138` l, `:140` l
- `:152` discard f; `:161` untap f; `:172` millCards.cards f
- searchZonesOf.whose: `:180` f, `:181` l, `:182` l
- fight.other: `:188` f, `:192` l, `:193` l, `:194` l
- fightEachOther.pair: `:207` f, `:208` l
- `:214` reveal.cards f; `:220` shuffleInto f; `:234` voteStartingWith.first f
- doubleCounters.on: `:285` f, `:287` l, `:288` l
- `:292`/`:293` exchangeCards.left/right f; `:297`/`:298` exchangeControl.left/right f;
  `:301` exchangeLifeTotals.parties f; `:305`/`:306` exchangeTextBoxes.left/right f
- `:316` exileFrom f; `:323` exileFaceDownFrom f; `:340` castWithoutPaying f
- manifest: `:378` f, `:379` l; cloak: `:385` f, `:386` l
- `:434` airbend f
- doublePower `:448` f, `:449` l; doublePowerAndToughness `:456` f, `:457` l, `:458` l;
  doubleToughness `:465` f, `:466` l
- triplePowerAndToughness `:472` f, `:475` l, `:478` l; triplePower `:485` f, `:486` l;
  tripleToughness `:493` f, `:496` l
- regenerate: `:592` f, `:593` l
- detain: `:643` f, `:644` l, `:649` l
- `:673` exert f; `:686` goad f; harness `:692` f, `:694` l; `:698` heal f; `:723` suspect f
- `Definitions/Keywords.lean:548` devour.subject f

Totals: 50 Definitions, 57 `firstMention` and 32 `laterMention` calls. If other tickets have
moved these lines, find each by its id string; the ids are the authority, not the line numbers.

Part 3: docs. `lean/CONTRACTS.md:1255-1268`: identities are generated for Definition
`NounPhrase` parameters, a written `.inCaller p` opts out, performer identities are written.
ADR `:149` (§1.4) and `:292` (§2 table): the same authoring note. `Templates.lean:9-10` module
doc and `CallerScope.lean:8`: align their wording.

## What must not change

Every elaborated Definition value (the `rfl` pins below prove it); Template bodies; performer
identities; the `Authoring.Form` of cards.

## Pins

- Added, in `Proofs/DefinitionsAndTemplates.lean` (each probed `rfl` on today's tree; the right
  side is today's hand-written body):
  ```lean
  theorem destroyBindsItsSubject (subject : NounPhrase) :
      destroy subject =
        .ownScope (some (.action "destroy"))
          (.move (.firstMention "destroy.subject" (.inCaller subject)) (.zone .battlefield none)
            graveyard .departure []) := rfl
  theorem harnessBindsItsSubject (subject : NounPhrase) :
      harness subject =
        .ownScope (some (.action "harness"))
          (doIf (not (is (.firstMention "harness.subject" (.inCaller subject)) (hasDesignation harnessed)))
            (gainDesignation (.laterMention "harness.subject" (.inCaller subject)) harnessed) none) := rfl
  theorem fightBindsOtherAndKeepsTheSource (other : NounPhrase) :
      fight other =
        .ownScope (some (.action "fight" .performs))
          (doIf (and [
            is (.firstMention "fight.other" (.inCaller other)) isCreature,
            is (.firstMention "fight.source" .actor) isCreature])
            (.simultaneously [
              .dealDamage (.statOf (.stat .power) (.laterMention "fight.source" .actor))
                (.laterMention "fight.other" (.inCaller other)),
              instruct (.laterMention "fight.other" (.inCaller other))
                (.dealDamage (.statOf (.stat .power) (.laterMention "fight.other" (.inCaller other)))
                  (.laterMention "fight.source" .actor))])
            none) := rfl
  ```
  If a right side no longer matches today's body when this lands, update it to the body as it
  stands immediately before Part 2, and say so.
- Added: two `#guard_msgs` pins, one per guard in §Design 3, each over a scratch
  `semantic_definition` declared inside the proof file.
- Must stay green, unedited: `templateKeepsSingleTarget` (`:56`), `parameterIdentitySurvivesTap`
  (`:61`), `movedParameterCannotBeReused` (`:66`), all of `Proofs/ControllerSacrifice.lean` and
  `Proofs/CallerScope.lean`, every `ok…` and `spelled` pin.
- Retired: none.

## Acceptance

- `./lean/scripts/build` (`--wfail`), `cargo xtask lean check`, the CR citation gates.
- `grep -c '\.firstMention "' lean/Semantics/Definitions/*.lean` counts only the performer
  identities (`fight.source`, `explore.permanent`, `connive.permanent`, `endure.permanent`)
  plus `controllerSacrifices`'s Template identity.
