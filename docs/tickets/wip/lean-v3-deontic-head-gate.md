---
needs: []
---
# Delete the separate deontic head gate: deontic clauses use the event gate

Standard constraints apply. Decided 2026-10-25: delete `Action.deonticHead`; `cant`, `must`, `can`,
`gatedBy` and `asThough` gate their action with the event gate, `Action.eventHeadOk`. Found by the
2026-10-25 triage of an external audit ("engine creep"), verified against the tree at `pmvrnuxw`.
Paths are under `lean/Semantics/` unless they start with `lean/` or `docs/`; line numbers are as of
that change (re-find them with the greps given).

## The defect

*Jared Carthalion, True Heir* prints "You can't become the monarch this turn." *Fioran
Reformist*'s Democracy reminder prints "Players can't become the monarch." Both are Vintage-legal.
The monarch is a designation a player can have, and an effect makes a player the monarch
[CR#725.1,725.3]. A probe on 2026-10-25 of `cant you (gainDesignation you monarch)` (and of `cant
(each isPlayer) …`) returned `[.eventHead]`: `Action.deonticHead` (`Check/Events.lean:50-71`) is the
event gate minus about 25 heads. It omits `gainDesignation`, and its other omissions relative to
`Action.eventHead` (`Check/Events.lean:31-44`) are `markDamage`, `clearDamage`, the
`flipped`/`unflipped`/`faceUp`/`faceDown` statuses, `turnFaceDown`, `combat`, `attackUnblocked`,
`attackWith`, `commitCrime`, `gameGains`, `choose`, `addMana`, `expose`, `shuffle`, `flipCoins`,
`rollDice`, `createObject`, `distribute`, `moveCounters`, `relate`, `merge`, `skipUntap`,
`skipPart`, `addTurnPart` and `exchange`.

The closed list is an observation of which heads cards happened to constrain, not text meaning or
anaphora, and it refuses printed text. A "can't" effect takes precedence over an effect that
allows or directs the thing [CR#101.2], so a prohibition has no reason to admit fewer actions than
the events the checker already knows. Compound and control-flow actions (`sequentially`, `doIf`,
`doForEach`, `chooseModes`, `establish`, …) stay refused, by the event gate.

## The change

Every edit below is required. Do them in this order.

1. `Check/Events.lean`: delete `Action.deonticHead` with its docstring (lines 50-66: the
   docstring "The closed action heads a deontic clause can constrain …" and the `def`), and
   `Action.deonticHeadOk` with its docstring (lines 68-71). Leave `Action.headGate` (`:26`),
   `Action.eventHead` (`:31-44`) and `Action.eventHeadOk` (`:48`) exactly as they are.
2. `Check/Phrase.lean`: replace `Action.deonticHeadOk` with `Action.eventHeadOk` at its four call
   sites. The `Bool` argument after the gate stays `false` at each:
   - `:4579` (`.cant`): `… Action.deonticHeadOk false (between := (stepDeonticModsWith ops · mods))`
   - `:4597` (`.must` / `.can`): `Action.deonticHeadOk false (between := permitting)`
   - `:4609` (`.asThough`): `stepActionClauseWith ops drs subj action .preEvent Action.deonticHeadOk false`
   - `:4627` (`.gatedBy`): `Action.deonticHeadOk false (between := gated)`

   Afterwards `grep -rn deonticHead lean/Semantics` is empty.
3. Docstrings that name the retired gate:
   - `Abilities.lean:1218`: "the event and deontic head gates admit it" becomes "the head gate
     admits it".
   - `Proofs/CheckerCore.lean:144-146`: the docstring "Closed event and deontic head-gating
     (`semantics-v3.md` §3.6–3.7): compound actions (`sequentially`) are refused inside `does`,
     `would`, `did`, and `cant` with `Refusal.eventHead`, while a Definition's `ownScope` header is
     admitted." becomes "Closed head-gating (`semantics-v3.md` §3.6): compound actions
     (`sequentially`) are refused inside `does`, `would`, `did`, and `cant` with
     `Refusal.eventHead`, while a Definition's `ownScope` header is admitted."
   - `Proofs/Deontic.lean:275` ("The prevention is a deontic head; a self-replacement is not.")
     becomes "The prevention is an event head; a self-replacement is not."; `:292` ("`redirectDamage`
     is a deontic head, and the self-replacement `Action.replace` is not") becomes
     "`redirectDamage` is an event head, and the self-replacement `Action.replace` is not".
   - `Proofs/Deontic.lean:683-686` (docstring of `prohibitionAdmitsAnyZoneChange`): drop "(owner
     ruling 2026-10-24)" and read "A deontic clause constrains any Zone Change, as an event clause
     watches one: …".
4. Pins that change (the five existing ones):
   - `headerDeedIsReadByTheGateAndTheStepper` (`Proofs/DefinitionsAndTemplates.lean:77-82`): drop
     the `spell.deonticHeadOk = true` conjunct. The statement becomes
     `spell.eventHeadOk = true ∧ spell.namedDeed = some (.action "cast") ∧ after.referents.map
     (·.payload.prov.map (·.deeds)) = [some [.action "cast"]]`. Its docstring already says "the
     head gate".
   - `headGatesAgreeOnADefinitionHeader` (`:84-88`): retire, with its docstring. With one gate it
     asserts only that the event gate admits `devour`, which `headerDeedIsReadByTheGateAndTheStepper`
     already asserts for `cast` by the same `ownDeed?` clause of `Action.headGate`.
   - `cantBePreventedIsADeonticOverPreventDamage` (`Proofs/Deontic.lean:276-288`): its last
     conjunct becomes `(Action.preventDamage .all none none).eventHeadOk = true`.
   - `whippoorwillCantIsADeonticOverRedirectDamage` (`:294-312`): its two `deonticHeadOk`
     conjuncts become `eventHeadOk`: `(Action.redirectDamage .all .you none).eventHeadOk = true` and
     `Action.eventHeadOk (.replace (.dealDamage .anyAmount .any none) (.instead (.draw (.exactlyOf
     1)))) = false` (a self-replacement is no event head either [CR#614.15]).
   - `prohibitionAdmitsAnyZoneChange` (`:687-690`): drop the third conjunct
     `(returnToHand thisCreature).deonticHeadOk = (returnToHand thisCreature).eventHeadOk`, and the
     `∧` before it, so the second conjunct ends `… exileZone) {}) = [] := by` with one space
     before `:=`.
5. `docs/decisions/semantics-v3.md`, two edits.

   **Line 331** (the shared action-clause spine paragraph). Replace
   "the head gate (§3.6 for events and lookbacks, §3.7 for deontic rules, both reading a
   `Definition` by its `ownScope` header)" with
   "the head gate (§3.6, the event gate `Action.eventHeadOk`, used alike by events, lookbacks and
   deontic rules, which read a `Definition` by its `ownScope` header; the separate closed deontic
   gate of §3.7 was retired 2026-10-25)". Everything else on the line is unchanged.

   **Line 485** (§3.7). Replace the whole bullet with exactly this text:

   > - **Deontic Head-Gating on `cant`, `must`, `can`, `gatedBy`, and `asThough` (`Check/Events.lean`, `Check/Phrase.lean`):** Deontic clauses gate their action with the event gate, `Action.eventHeadOk` (§3.6), and have no head gate of their own (the closed deontic list, narrower than the event heads, was retired 2026-10-25; it refused printed text such as "can't become the monarch"). A "can't" effect takes precedence over an effect that allows or directs the thing `[CR#101.2]`, so a prohibition is gated like an event and admits every action the checker knows as an event: `gainDesignation` ("can't become the monarch" `[CR#725.1]`), `preventDamage` and `redirectDamage` ("can't be prevented", "can't be dealt instead" `[CR#615.12,614.9]`), any `setStatus`, and any `move` ("can't be returned to its owner's hand"; `leave`, `putInto`, `returnTo*` are Templates over the move). Compound and control-flow actions (`sequentially`, `doIf`, `doForEach`, `chooseModes`, `establish`, a `replace`) are refused by the event gate here as there. An Action Definition is admitted by its `ownScope` header, whatever its deed (`Action.headGate`). Copy and vote permissions are required by Display of Power and additional-vote clauses; payment permissions use `pay .anyCost` with the spend-as-though modifier. `Cost.anyCost` is a wildcard restricted to event, lookback, prospective, and deontic action patterns; performed costs require a specified cost. Likewise, `Ballot.anyBallot` is an action-pattern wildcard for additional-vote permissions; a performed vote requires actual labels or a candidate, and an empty label list remains invalid. `changeControl` is a pattern head only ("other players can't gain control of them"); a performed control change is `establish(controlGrant …)` (`[CR#613.1b]`). Pattern entry restores the caller’s mode before checking subsequent effects. The 2026-10-14 bench extension preserves the distinctions between triggering [CR#603.2], crewing and saddling [CR#702.122b,702.171c], phase-in status [CR#702.26c], and fully unlocking [CR#709.5i].

   The 2026-10-24 history the old bullet carried (the battlefield-only `move` rule superseded by
   "any `move`") is kept by the dated sentence above and by `prohibitionAdmitsAnyZoneChange`.
   Read [CR#101.2], [CR#725.1], [CR#615.12], [CR#614.9] and [CR#613.1b] in `data/rules/cr.txt`
   before writing them.

## Pins added

In `Cards/Deontic.lean`, before `end Semantics.Cards`, both checking `[]` (compiled in a scratch
copy of the tree with items 1-4 applied; both are refused `[.eventHead]` on the unchanged tree).
Oracle text is from `data/derived/cards.jsonl` (field `text`):

- `jaredCarthalionTrueHeir`, spelled in full. {R}{G}{W}, Legendary Creature — Human Warrior 3/3.
  "When Jared Carthalion enters, target opponent becomes the monarch. You can't become the monarch
  this turn. / If damage would be dealt to Jared Carthalion while you're the monarch, prevent that
  damage and put that many +1/+1 counters on it."

  ```lean
  /-- Jared Carthalion, True Heir: "You can't become the monarch this turn" is a prohibition over
  `gainDesignation`, an event head [CR#101.2,725.1]. -/
  def jaredCarthalionTrueHeir : Spelled := spelled <| singleFacedCard
    { name := "Jared Carthalion, True Heir", cost := [pip .red, pip .green, pip .white],
      supertypes := [.legendary], types := [.creature], subtypes := [human, warrior],
      text :=
        [ when (does thisCreature enter)
            (sequentially
              [ gainDesignation (target isOpponent) monarch,
                establishThroughout (cant you (gainDesignation you monarch)) thisTurn ]),
          static
            (onlyWhile
              (replace (would any (dealDamage anyAmount thisCreature))
                (instead
                  (preventDamage allDamage
                    (also := putCounters preventedThisWay p1p1Counter thisCreature))))
              (is you (hasDesignation monarch))) ],
      power := stat 3, toughness := stat 3 }
  ```
- `fioranReformistPlayersCantBecomeTheMonarch`, a named fragment, not the card. *Fioran Reformist*
  ({2}{W}, Creature — Human Advisor 3/3) prints "When Fioran Reformist enters the battlefield, if a
  player is the monarch, get rid of the monarchy. / Democracy is in effect until Fioran Reformist
  leaves the battlefield. (At the beginning of each player's end step, if they control more
  creatures than each other player, they draw a card. Players can't become the monarch.)" The
  bench has no declaration for the Democracy designation or for "get rid of the monarchy" (`grep -rni
  'democracy\|monarchy' lean/Semantics` is empty), so the card cannot be spelled in full; the
  docstring says so. The fragment is the reminder's last sentence:

  ```lean
  /-- Fioran Reformist's Democracy reminder, "Players can't become the monarch." The card itself
  is not spelled: the bench declares neither the Democracy designation nor "get rid of the
  monarchy". -/
  def fioranReformistPlayersCantBecomeTheMonarch : Ability :=
    static (cant (.actor (bare isPlayer)) (gainDesignation .actor monarch))
  theorem okFioranReformistPlayersCantBecomeTheMonarch :
      Ability.check (DRS.ofList []) fioranReformistPlayersCantBecomeTheMonarch = [] := by decide
  ```
- No negative pin is added: `badEmptyProhibitedAction` (`Proofs/Deontic.lean:33-36`) already asserts
  that a prohibition over `.sequentially []` is refused `[.eventHead, .nonEmpty]`, and
  `compound_event_head_refused` (`Proofs/CheckerCore.lean`) covers the event side. Both stay
  unchanged.

## What must not change

- The event gate `Action.eventHead`, `Action.eventHeadOk` and `Action.headGate`, including the
  `ownScope` header admission.
- Every other pin in `Proofs/Deontic.lean` and `Cards/Deontic.lean`. If any other pin fails after
  the change, it asserted a refusal of a head the event gate admits: do not edit it silently.
  Rewrite it to assert what is wanted and report it by name with the reason.
- No other file under `crates/` or `plugins*/` changes.

## Commit plan

One commit, `lean: deontic clauses use the event head gate`, with items 1-5 and the pins above,
and no other change.

## Acceptance

- `./lean/scripts/build` (`--wfail`) passes with zero warnings and no `sorry`.
- `cargo xtask lean check`.
- `grep -rn 'deonticHead' lean/ docs/decisions` is empty.
- The CR citation gates: `cargo xtask cite check --list-noncompliant` empty, `cargo xtask cite
  check` 0 stale, `cargo xtask cite bless` for newly cited rules, and `jj --no-pager diff --git |
  cargo xtask cite audit --diff` read rule by rule against its claim.
- Report tests added (`jaredCarthalionTrueHeir` and `fioranReformistPlayersCantBecomeTheMonarch`), rewritten (the four named above) and retired (`headGatesAgreeOnADefinitionHeader`,
  with the reason above).
