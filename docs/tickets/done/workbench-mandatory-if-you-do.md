# workbench-mandatory-if-you-do

"If you do" after a MANDATORY instruction — re-measured by the bucket round
(2026-09-02) at **96 lines / 95 cards**, the board's largest unbuilt item
(the routed version carried it as a Garruk singleton). `Reflexively` keys off
an offer; these clauses key off an instruction with no "may" and no "unless"
(method recorded in the bucket's close: clause-bounded, may/unless-free).
Carriers include Garruk Relentless, Dark Depths, Charnel Troll, Flameskull,
Heart of Yavimaya, Cartographer's Hawk. Re-measure at claim.

## As landed (2026-09-02)

**Re-measurement.** Supported corpus, reminder text stripped, strict "if you
do" (not "don't"), antecedent clause may/unless-free: **94 lines / 93 cards**
(the ticket's 96/95 was a slightly looser clause bound). Buckets: 84 lines
with the did-arm alone, 8 with an "if you don't" arm beside it (the seven
land-cycle members plus Transmute Artifact), 2 with "Otherwise" (Charnel
Troll, Mistbreath Elder). The reflexive TRIGGER over the same mandatory
antecedent ("when you do") is a separate 64 lines / 63 cards and needed
nothing: [CR#603.12] already writes over a clause that "allow[s] or
instruct[s] a player to take an action", and `reflexEncloseUse` already
admits a mandatory body.

**The design.** After a mandatory instruction "if you do" tests whether the
instruction's ACTION HAPPENED, not whether an option was taken.
[CR#608.2c] follows the instructions in the order written and reads the whole
text as English; [CR#609.3] lets an impossible one do only as much as
possible. Official rulings state it: Dark Depths ("you won't be able to
sacrifice it, so you won't create Marit Lage"), Charnel Troll ("You can't
choose not to exile a creature card from your graveyard if you have one to
exile"), Flameskull, Cartographer's Hawk.

**The carrier.** `May`'s `offer` was a `Maybe`, so the offered clause and the
mandatory one shared a term — and Mister Fantastic ("you may draw a card")
had been written as the mandatory row by mistake. `May`'s decider is now
written, always; the mandatory form is the new **`IfDone`** row, gated on
`ReflexEnclosure body` (the pro-verb needs a player to inflect for) and on at
least one arm (with neither it denotes exactly its body). The didn't-arm is
typed at `bs`: it runs because the action did not happen. `Macros.doThen` /
`doElse` repoint to it and `doThenElse` joins them. Not a widening of
`Reflexively`.

**Benched.** `charnelTroll`, `promiseOfBunrei`, `gravePeril`,
`mistbreathElder`, `woeleecherWhole` (all whole), and **`garrukRelentless`
whole** — the transform round's last blocker, both faces. `heartOfYavimaya`
repointed from the raw row to `doThenElse`; `moxDiamond` to `mayThenElse`.

**Pins** (ProofsG): `badIfDoneWithNeitherArm`, `badIfDoneOverAgentlessBody`,
`badIfDoneOverScheduledBody`. `badIfNotReadsMandatoryBody` (ProofsC) reworded
off [CR#118.12] onto [CR#609.3].

## Remainders

- **Dark Depths and Marit Lage's Slumber stay unbenched, on an unrelated
  gap:** `TypeLine` has no supertype slot and `TokenQuality` admits only
  chosen-quality reads and `Named`, so "a legendary 20/20 black Avatar
  creature token" is unwritable. The token SUPERTYPE cell is the ask.
- **Cartographer's Hawk stays unbenched**, likewise unrelated: its header
  needs "a player who controls more lands than you", and `PlayerStat` is
  `LifeTotal | StartingLifeTotal` — there is no player-side count comparison.
- **The COORDINATED antecedent of "if you do"** — Breath of Fury's "sacrifice
  it and attach this Aura to a creature you control. If you do, …", 1 line.
  `ReflexEnclosure` refuses it at `EncNotOneAction`, which is [CR#603.12a]'s
  trigger-counting concern rather than a bar on the pro-verb; if a second
  carrier turns up, the gate wants splitting into an agent half and a
  count half.
- **Flameskull** stays unbenched: its did-arm's "you may play one of those
  cards" reads a batch across two separate exiles.
- **The didn't-arm ALONE over a mandatory instruction** has no whole-card
  witness: the pro-verb "if you don't," lines are the three Pacts, whose
  clause sits under a delayed trigger ("at the beginning of your NEXT
  upkeep"). `doElse` is witnessed by `badIfNotReadsMandatoryBody` only.
