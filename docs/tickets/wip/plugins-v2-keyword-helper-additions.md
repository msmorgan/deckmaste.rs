---
needs: [plugins-v2-implicit-actor-spelling]
---
**New helpers the keyword bodies asked for, and the small fixes found beside
them.** Split from `plugins-v2-keyword-body-defects` on 2026-10-05. The
helpers that write a performer are spelled with the implicit actor, so the
ticket as a whole follows `plugins-v2-implicit-actor-spelling`; the items
marked *independent* below do not depend on it and may land earlier, alone or
together. Standard constraints apply.

## New helpers

The owner accepted these names on 2026-10-05 ("getsBoth is a strange name to
me but sure I guess"). Re-spell each caller listed onto the helper.

- `mayOrElse(body, otherwise)` — "you may …; if you don't, …". Fabricate,
  fading and madness fall to named calls today because the "if you do" slot
  comes first. The 2026-10-04 proposal took a player first; under the actor
  it does not. Needs B.
- `getsBoth(subject, delta, duration)` — one delta to both power and
  toughness, "+X/+X" (melee, rampage). *Independent* of B. If
  `semantics-v2-gets-both-deltas` has landed, write it over the
  one-modification form that ticket introduces; otherwise over `gets`, and
  that ticket re-spells it.
  *Done, 2026-10-05, in `semantics-v2-gets-both-deltas`:* written over
  `ptModification`; melee and rampage call it, and so do bushido, exalted,
  flanking and prowess, whose two deltas were textually identical.
- `createTokenCopy(source, riders)` — myriad's tapped-and-attacking token copy.
  Writes a creation, so needs B.
- `artifactCreatureToken(power, toughness, colors, subtypes, abilities = [])` —
  a token spec with the extra card type (fabricate's Servo). *Independent*
  (a spec, no performer).
- `mayCastFrom(what, zone, paying)` — a general cast permission whose permitted
  player is the actor (`airbend`, aftermath). Airbend's "its owner may cast
  it" is `act(ownerOf(it), establish(mayCastFrom(it, exileZone, mana([2]))))`;
  this relies on `semantics-v2-actor-handoff`'s prototype finding that `actor`
  reads as the controller inside a static ability. Needs A and B.
  (2026-10-05: confirmed by the Lean pins `okStaticActorPermission` and
  `okHandedCastPermission` in `Proofs/Actor.lean`, over Lean's
  `Actor.mayCastFrom (what) (zone)`, which has no `paying` parameter.)
- `cantBeBlockedByFewerThan(subject, n)` — the bound in menace. *Independent*.
- Event helper `discards(player, card)`, and **Megrim** joins canon to prove
  "whenever … discards": "Whenever an opponent discards a card, this
  enchantment deals 2 damage to that player." The event is already pinned in
  Lean ("Whenever you discard a card", `Proofs/Anaphora.lean`
  `okDiscardFromHand`). *Independent* of B, but it is the proof the
  actor-handoff design names for discard triggers, so land it no earlier than
  `semantics-v2-actor-handoff`.

## Small fixes

- `anotherSpell` becomes `and([spell, otherThan(thisSpell)])`, matching Lean
  `stormExpansion` (`Macros.lean` ~L1146); storm, split second and surge call
  it. Today it is `And([InZone(stack), Not(AbilityHead(AnyOnStack)),
  OtherThan(thisSpell)])`. *Independent*.
- Delete the RON helper `returnToBattlefieldWithCounters`
  (`macros/instructions/returnToBattlefieldWithCounters.ron`): it has no
  callers, and persist and undying already go through `returnToBattlefield`.
  The old ticket's "one of the two is wrong" was mistaken. The Lean macro of
  the same name is used by the bench and stays. *Independent*.
- Add `isCard` to the Lean bench where RON writes `cardIn`: the bench writes
  "a card in hand" without it, and the conjunct is meaningful because a token
  can sit in a hand, graveyard or exile until state-based actions are checked
  [CR#111.7,704.5d]. *Independent*.
- Fix the `amass` comment to the rule's wording, "Put N +1/+1 counters on that
  creature" [CR#701.47a], not "on it" (`semantics-v2-actor-handoff` re-spells
  `amass` and may do this first). *Independent*. (2026-10-05: done by
  `semantics-v2-actor-handoff`; nothing left here.)
- Reorder helper parameters freely so required ones precede defaulted ones,
  removing the named calls forced by order today: `returnToBattlefield`
  (riders before agent and origin), `activated` (limit before guard),
  `deonticRule`, `damage`, `verbedEvent`, `addMana`. List every reorder in the
  landing record. `returnToBattlefield` and `addMana` lose their agent in B, so
  reorder them after it; the other four are *independent*.
- `transfigure` and `transmute` write `Pro(Bare, One, top(1))` for the card
  their search found; use the existing `foundCard` helper if the Lean check
  accepts it, and strike them from `ALLOWED_RAW`
  (`crates/deckmaste_semantics_v2/tests/keyword_bodies.rs`). If the check
  refuses, they stay and the record says why. *Independent*.

## Proof

- Each helper has at least one caller re-spelled onto it, and Megrim checks in
  `cargo xtask lean-check`.
- A re-spelling onto a helper leaves `cargo xtask expansions` byte-identical;
  `anotherSpell`, the `isCard` conjuncts and Megrim are term changes and are
  listed as such.

## Out of scope

"Gets" writing its subject twice (`semantics-v2-gets-both-deltas`).

## Landing record

The series, oldest first, on the claim `uxztwtzpvsxv`:

- S1 `zxptumutykrq`: `anotherSpell` is `and([spell, otherThan(thisSpell)])`.
- S2 `nxzwmxnxsoss`: `returnToBattlefieldWithCounters` deleted.
- S3 `kvmlxynxvrrq`: `artifactCreatureToken`; fabricate's Servo uses it.
- S4 `ntsurszpnrsm`: `mayOrElse`; fabricate and madness use it.
- S5 `upvnzryyqqyy`: `createTokenCopy`; myriad and populate use it.
- S6 `ryyzkrvpsunt`: `discards`; Megrim joins canon; madness and mayhem use it.
- S7 `utzryuvqsryk`: transfigure and transmute read `foundCard`; both off
  `ALLOWED_RAW`.
- S8 `utvrxrwqtsyo`: `mayCastFrom`; aftermath and madness use it.
- S9 `mxvlptuqymrq`: helper parameter reorders.
- S10 `pyvkwmtnyxww`: Lean `okThoughtseize` says `isCard`.
- S11 `mkywwonwonry`: `docs/decisions/semantics-v2.md` §11 and §12.1.

This record and four finding tickets are written on top of S11 and change
only `docs/tickets/`. Each stage was gated before its commit; nothing was
squashed or rebased. `cantBeBlockedByFewerThan` (item 4) did not land (STOP 1),
and airbend's re-spelling did not land (STOP 2).

**Proof.**

- Per-stage gate. `cargo xtask gate --changed` derived the same command at
  every stage: `cargo test -p deckmaste_construction_core -p
  deckmaste_lexical_source -p deckmaste_semantics_v2 -p
  deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask`, run with
  `--no-fail-fast`: 87 binaries, 1169 passed, 0 failed, 1 ignored (ignored
  before this landing), at S1 through S10.
- `cargo xtask lean-check`: canon 126/126 and testing 5/5 at the claim and
  S1; canon 127/127 (Megrim) and testing 5/5 from S6 on, run at S6, S7, S8
  and S10. At S2–S5 and S9 every card term was byte-identical to the stage
  before (card dumps, below), so the generated Lean was unchanged.
- `cd lean && ./scripts/build` (S10): builds with `--wfail`, every pin at its
  old outcome; `okThoughtseize` still `= []`.
- `cargo xtask facts check`: up to date (S1, S6). Cite checks at every stage
  that cites: 0 non-compliant, 0 stale; every new site read through
  `cite audit --diff` (`[CR#112.1]` anotherSpell, `[CR#701.9a]` discards,
  `[CR#111.7,704.5d]` okThoughtseize, `[CR#701.65a]`, `[CR#702.111b]` and
  `[CR#111.7,704.5d]` in the finding tickets). Nothing was blessed: every rule
  was already in `cr-citations.lock`.
- Term comparison: `cargo xtask expansions` and a per-card dump of every
  canon and testing card (Debug and emitted Lean; scratch dumper outside the
  repo), taken at the claim and after each item, compared with `diff -r`.
  Claim → S11: 1505 → 1509 declarations, 1457 → 1461 printed, 48 skipped both
  times, 0 failed; added `artifactCreatureToken`, `createTokenCopy`,
  `discards`, `mayCastFrom`, `mayOrElse`; removed
  `returnToBattlefieldWithCounters`. Every other difference is listed per
  item below; there is none unexplained.

Per item:

| item | stage | terms | proving card |
|---|---|---|---|
| 1 `anotherSpell` | S1 | changed: `And([InZone(stack), Not(AbilityHead(AnyOnStack)), OtherThan(thisSpell)])` → `And([And([InZone(stack), Not(AbilityHead(AnyOnStack))]), OtherThan(thisSpell)])`, Lean `stormExpansion`'s `.and [spell, .otherThan thisSpell]`; in `anotherSpell`, storm, splitSecond, surge | Empty the Warrens (storm), Krosan Grip (split second), Jwar Isle Avenger (surge) |
| 2 delete `returnToBattlefieldWithCounters` | S2 | its expansion gone; every other term byte-identical | none needed; no caller, no test names it |
| 3 `artifactCreatureToken` | S3 | byte-identical (fabricate) | Accomplished Automaton (unchanged) |
| 4 `cantBeBlockedByFewerThan` | — | not landed, STOP 1 | — |
| 5 `mayOrElse` | S4 | byte-identical (fabricate, madness) | Accomplished Automaton, Arrogant Wurm |
| 6 `createTokenCopy` | S5 | byte-identical (myriad, populate) | Wyrm's Crossing Patrol, Wake the Reflections |
| 7 `mayCastFrom` | S8 | changed, see below | Farm-Market (aftermath), Arrogant Wurm (madness) |
| 8 `discards` + Megrim | S6 | byte-identical (madness, mayhem); Megrim added | Megrim (canon, new) |
| 9 `foundCard` | S7 | changed: `Pro(Bare, One, Top(1))` → `Pro(Stamped(Action("Search")), One, Whole)` in transfigure (once) and transmute (twice) | Fleshwrither, Drift of Phantasms |
| 10 reorders | S9 | every expansion and card byte-identical (`diff -r` empty, index included) | — |
| 11 bench `isCard` | S10 | Lean pin term only: `okThoughtseize`'s choice gains `.and [.isCard, …]` | pin outcome unchanged |

- Item 7 term changes, in the implicit-actor landing's classes: aftermath,
  the permitted player `You` → `Actor` (identity class; a static ability with
  no handoff, so the actor is the controller, Lean
  `okStaticActorPermission`). Madness, the permitted player `PossessorOf(Owner,
  This)` → `Actor` inside the existing `act(ownerOf(this), …)`: identity class
  in the sense "`Actor` where the same player was written", the handoff
  already present (Lean `okHandedCastPermission` is the same shape). The
  counterpart `This` is unchanged in both.
- Item 9: `Pro(Bare, One, top(1))` reads "the top one", a position in a
  zone; the search finds a card anywhere in the library [CR#701.23a], so the
  old term did not name it. `foundCard` reads the search's own result, which
  is what "put it onto the battlefield" and "reveal that card" mean. Checked in this workspace
  with `lake env lean`: `putOntoBattlefield foundCard` alone is refused
  `[anaphor (stamped (action "Search")) one 0]`, and after
  `searchLibraryFor (exactly 1) creature` it checks `[]`, so the pronoun can
  only be the found card.
- Item 8: Megrim's term is `triggered(verbedEvent(some (a opponent), Discard,
  some (a isCard)), dealDamage(this enchantment, 2, Pro(Word(Player), One,
  Whole)))`. "That player" resolves to the discarding opponent: with the
  event's agent removed, the same ability is refused `[anaphor (word player)
  one 0]` (`lake env lean`, this workspace), so the event's agent is the only
  player the pronoun can read.

**The helpers.**

- `artifactCreatureToken(power: Nat, toughness: Nat, colors, subtypes,
  abilities = [])` (`instructions/`): the characteristics record of
  `creatureTokenOf` with `types: [Artifact, Creature]`.
- `mayOrElse(body, otherwise)` (`instructions/`): `may(body, None,
  otherwise)`. The actor decides and performs both branches; madness keeps
  its one `act(ownerOf(this), …)` around it, so the owner decides, casts and
  moves the card.
- `createTokenCopy(source, riders = [])` (`instructions/`):
  `createObject(1, Token(copyOf(source), riders))`. Riders stay a list of
  rider terms; one caller writes riders, so no rider helper.
- `mayCastFrom(what, zone, paying = ItsOwnCost, exclusive = false)`
  (`abilities/`): `mayPlayDeed(Action("Cast"), actor, what, Play(zone, None,
  None, exclusive, paying))`. Lean's `Actor.mayCastFrom (what) (zone)` writes
  `.itsOwnCost` and `false`; RON takes both as defaulted parameters, so the
  two-argument call is Lean's. `paying` is a `PlayPayment`
  (`PayingInstead(cost)`, not a bare cost) so that aftermath's own-cost
  permission and madness's alternative cost share the helper. `exclusive`
  is aftermath's "can't be cast from any zone other than a graveyard"
  [CR#702.127a].
- `discards(player, card)` (`events/`): `verbedEvent(Action("Discard"), card,
  player)` [CR#701.9a], the form Lean pins in `okDiscardFromHand`.

**Reorders (S9),** every expansion byte-identical:

- `activated(cost, instruction, timing?, limit?, guard?, activator?)` →
  `(cost, instruction, timing?, guard?, limit?, activator?)`; reconfigure's
  second ability goes positional.
- `damage(kind, source?, patient?)` (event) → `(kind, patient?, source?)`;
  bloodthirst goes positional.
- `deonticRule(subject, compulsion, deeds, role, bound?, patient,
  as_though?, rider)` → `(subject, compulsion, deeds, role, patient, rider,
  bound?, as_though?)`; menace goes positional (Ravener keeps its wholly
  named call).
- `verbedEvent(agent?, verb, patient?, becomes?, locus?)` → `(verb,
  patient?, agent?, becomes?, locus?)`; madness's `causes` event and
  umbraArmor go positional; `discards`'s body is written in the new order.
- predicate `attachment(side, word?, counterpart?)` → `(side, counterpart?,
  word?)`; reconfigure's guard goes positional.
- Not reordered: `returnToBattlefield` and `addMana` are already
  required-first; no other order lets unearth (agent, origin) and
  persist/undying/earthbend (riders) all call positionally, so unearth keeps
  its named call. The other helpers with a required binder after a defaulted
  one are routed (below).

**Tests.** Restored: 0. Re-spelled: 1, `ALLOWED_RAW` in
`crates/deckmaste_semantics_v2/tests/keyword_bodies.rs` lost
`keyword_abilities/transfigure` and `keyword_abilities/transmute` (7 → 5).
Added: one canon card (Megrim), a `lean-check` obligation. Ignored: 0.
Removed: 0. No test enumerates or counts the helper declarations or the
canon cards, so no count was re-spelled.

**Deviations and additions.**
1. `mayCastFrom` gained `exclusive` beyond the ticket's signature; aftermath
   needs it, and its default keeps Lean's two-argument form.
2. `paying` takes the `PlayPayment` term (`PayingInstead(mana([2]))`), not
   the brief's bare `mana([2])`, so the own-cost default is expressible.
3. Madness's permission was re-spelled over `mayCastFrom` though the brief
   named only airbend and aftermath: its term is in the identity class and
   proves (Arrogant Wurm), and with airbend stopped it is the one proof of
   the handed-off permission. Left on `mayCastFor`: escape, evoke,
   flashback, impending, mayhem, retrace, spectacle, surge, webSlinging
   (all `mayCastFor(you, …)`; four of them cast from any zone, which Lean's
   `mayCastFrom` cannot say, since its zone is required).
4. `mayOrElse` is written over `may`, not the constructor, and fading is
   left on `doIfDone`: "if you can't" is a different shape.
5. Item 9 is a deliberate term change, the one the ticket allowed.
6. Item 11 changed one Lean pin (`okThoughtseize`, `Proofs/Actor.lean`), not
   a bench card: of the five canon cards that write `cardIn` (Dangerous
   Wager, Farm-Market, Thoughtseize, Stormbind, Deadeye Tracker) only
   Thoughtseize has a Lean counterpart, and it is that pin. The 139 bench
   sites without a RON counterpart are routed.
7. Megrim's "a card" is written `a(card)`; Lean's `okDiscardFromHand` writes
   `a (.inZone hand)`. The discard verb supplies its zone (§7, ruling
   2026-10-05).
8. Four finding tickets minted (Routed).

**STOPs.**
1. *Item 4, `cantBeBlockedByFewerThan(subject, n)`, not landed.* Menace's
   term carries `MoreThan(1)`; the accepted name takes the printed number,
   2, and a template cannot write `n - 1`, so no body is byte-identical.
   Menace keeps `deonticRule` (positional since S9).
   `plugins-v2-menace-bound-helper`.
2. *Item 7, airbend not re-spelled.* `act(ownerOf(themVerbed(Action("Airbend"))),
   establish(mayCastFrom(themVerbed(Action("Airbend")), exileZone,
   PayingInstead(mana([2]))), forAsLongAs(…)))` is refused under a probe
   ("Airbend target creature.") with `[anaphor (stamped (action "Airbend"))
   many 0]` three times. The OLD body is refused identically under the same
   probe: the body reads the stamp of the wrapper that encloses it, and no
   card had ever airbent. Airbend keeps its old spelling; the probe was not
   committed. `plugins-v2-airbend-reads-its-own-stamp`.

**Names for the owner.** `exclusive` (a `mayCastFrom` parameter); `paying`
taking a `PlayPayment`; the parameter names `otherwise` (`mayOrElse`),
`source`/`riders` (`createTokenCopy`), `player`/`card` (`discards`), `what`/
`zone` (`mayCastFrom`, Lean's); the probe name "Airbend Handoff Probe"
(uncommitted).

**Glossary.** No new term.

**Not applicable.** Coverage lock, selection census, licensing-checker
totals, homograph and form-literal inventories and the performance advisory:
no English grammar, lexicon or corpus input changed, so `coverage` was not
run.

**Routed.**
- `plugins-v2-menace-bound-helper` (new): STOP 1.
- `plugins-v2-airbend-reads-its-own-stamp` (new): STOP 2, then airbend over
  `mayCastFrom` with its handoff.
- `lean-bench-card-in-zone-is-card` (new): the 139 bench `inZone` sites.
- `plugins-v2-helper-parameter-order-residue` (new): the fifteen helpers
  still declaring a required binder after a defaulted one, unearth's named
  `returnToBattlefield`, fading's named `doIfDone`.
