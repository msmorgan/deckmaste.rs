# workbench-attacking-defender-attributive

The attributive combat-role predicate — "attacking creature", "defending
player", "blocked creature" as noun restrictions rather than event headers.
Measured by choice-C (done 2026-08-27) at **94 supported lines**, with real
design questions (role-during-combat is a state read; which roles, which
carriers, interaction with `Attacks`' defender slot). The last blocker on
Beckoning Will-o'-Wisp and Triarch Stalker (whose choosers landed in
choice-C), and a dependency of many combat bundles. Re-verify the count with
`jq 'select(.supported)'` at claim; design before building — this is not a
one-cell round.

Routed from description-2 (close, 2026-08-28): the ATTACKER-voice predicate
("the player who attacked that player" shape — Namor's 4 lines) joins this
family's design; `AttackedBy` (description-1) covers the attacked side.

Routed from damage-prevention (close, 2026-08-28): the `Unblocked` predicate
("[creature] is unblocked" as a description) — Forcefield's last blocker;
combat-role description, this family's design.

## As landed

Four files, +378 / −11. `idris/scripts/build` PASS (23/23 from a deleted
`build/`, 0 errors, 0 warnings). Cites: 0 non-compliant, 20,263 checked / 0
stale, 29 audited sites read against their rule text, 0 rules newly blessed
(every rule the round cites was already registered). No design artifact: the
docstrings plus this section are the record.

### Premise corrections, up front

- **The count is 82, not 94.** Re-measured over `jq 'select(.supported)'`,
  82 distinct supported lines describe one side of an attack by the other,
  and they are three different constructions, not one: 21 relative clauses
  ("creature that's attacking you"), 18 bare participles ("creatures
  attacking you"), 27 the ENTRY RIDER's defender ("tapped and attacking that
  player"), and the rest other shapes. The ticket's headline family — the
  role words used attributively — is a different and much larger population
  again: "defending player" alone is 303 supported lines over 492 cards.
- **`Attacking` has carried the defender slot since the day after choice-C
  wrote that it did not.** `AttackerOf` (description-2, 2026-08-28) is the
  attacker-voice predicate, and the 39 relative-and-participial lines are
  its. The stale note in `beckoningWillOWispChooser`'s docstring is corrected
  in place, and the routed attacker-voice item is closed by inspection plus
  one bench rather than by new structure.
- **Namor, Atlantean King's blocker has MOVED and is not this family's.**
  Its body is "OTHER creatures you control attacking that player";
  `AttackerOf` writes the attacking clause, and `Other` — which asks
  `anyTargeted` of a context an attack trigger does not answer — is what
  holds the card up now. Corrected in place.
- **Beckoning Will-o'-Wisp and Triarch Stalker are still blocked, and the
  combat role is no longer why.** Beyond the flavour word, both spend the
  read inside "the LAST CHOSEN player" — the marked read at the PLAYER sort,
  where `OfLastChosen` is a quality-sorted description of an object and
  `That PlayerW` is the plain demonstrative. That is the chooser family's
  cell. The ticket's claim that this round is their last blocker does not
  hold.

### The design — one vocabulary, at two layers, because the rules are

[CR#506.4] is the whole argument for the object side: a creature removed from
combat "stops being an attacking, blocking, blocked, and/or unblocked
creature". Four roles, one list, one ending event, one carrier — a creature
on the battlefield — and none of them a status ([CR#110.5] closes status at
four categories of two values) or a characteristic. So the vocabulary is four
sibling `Predicate bs Object` rows and nothing else. Two stood already
(`Attacking`, `Blocking`); this round added `Blocked` and `Unblocked`.

The RELATIONAL rows are deliberately not members. `AttackerOf`/`AttackedBy`
and `BlockerOf`/`BlockedBy` each name the other side, which is a second
question [CR#506.4] does not ask; `Blocked` is not `BlockedBy` with the
relatum dropped, because [CR#509.1h] keeps a creature blocked "even if all
the creatures blocking it are removed from combat" and the role therefore
outlives every blocker a relatum could name.

[CR#506.2] gives the PLAYER roles in the same section, and they cannot be
members of the same vocabulary: they are references, not descriptions, so
they live at the noun layer. Two rows, not one indexed row, because the rules
fix them differently — [CR#506.2] makes the attacking player the ACTIVE
player, phase-wide and the same for every creature in the combat, while
[CR#508.5] resolves the defending player per attacking creature and
[CR#508.5a] determines it individually for each creature a sentence could
apply to. A word index over the pair would assert a symmetry the rules do not
have.

The third design question — the interaction with `Attacks`' defender slot —
has a clean answer: none. `TheDefendingPlayer` carries NO slot, on
`TheGrantor`'s precedent, because [CR#508.5] fixes the referent outright from
the sentence's own attacking creature and the printed phrase writes no
anchor. It does not read `AttackDefender`; that slot is a WRITTEN defender
and this word an unwritten one the rule resolves, which is exactly
[CR#508.5]'s "unless otherwise specified". Both may appear on one line and
neither is a binding the other reads — `agateBladeAssassin` benches with
`NoDefender` in the header and the word still resolving.

### Per bucket

**`Blocked` — 11 supported lines, benched.** Attributive and predicative
alike ("destroy target blocked creature", "activate only if this creature is
blocked"). `smite` benches WHOLE. The 12 further lines carrying the word are
NOT this row's and are recorded as such: "as though it weren't blocked" is
[CR#609.4]'s damage-assignment counterfactual, a premise the deed vocabulary
already carries.

**`Unblocked` — 16 supported lines outside reminder text, benched.**
`forcefield` benches WHOLE — the routed damage-prevention item, blocked on
this one word and nothing else: `NextTimeOnly`, `CombatOnly`, `CutAllBut` and
the your-choice damage agent all stood. The other 40 lines are ninjutsu's and
sneak's reminder text.

**The clash gate — `noCombatRoleClash`, one new pin.** [CR#509.1h] hands the
two blocking roles out as alternatives on one attacking creature and lets an
effect change one TO the other, so `And [creature, Blocked, Unblocked]`
describes nothing. Threaded into `contradictionFree` beside `noStatusClash`,
and pinned as `badBlockedAndUnblocked`. `Not Blocked` is deliberately NOT
`Unblocked`: a creature that is not attacking at all is neither, and
[CR#508.4d] mints an unblocked creature with no declaration having passed
over it.

**`TheDefendingPlayer` — 303 supported lines over 492 cards, two benches.**
Three positions: 173 name it as another permanent's controller ("a creature
defending player controls"), 65 make it a clause subject, 16 write the
possessive. `agateBladeAssassin` benches WHOLE at the subject position;
`fiendBinder` benches WHOLE at the controller position, and needed no row
beyond the noun — `ControlledBy` already takes any sole-holder player.
Recorded overgeneration: [CR#508.5] resolves the word only inside an ability
of an attacking creature or a sentence naming one, and no gate asks that,
because it is a context condition the noun cannot see from its own seat.

**`TheAttackingPlayer` — 9 bare supported lines over 9 cards, one fragment.**
`soulsOfTheFaultlessDrain` benches the trigger; the whole card does not,
and the word is not why — the printed line reads the damage twice ("you gain
that much life AND attacking player loses that much life") and `ThatMuch`
asks `countQuantOutcomes … = 1` of a context the first clause has already
added to. The other 8 lines are zeros with their own causes: Goblin Goon,
Mogg Toady and Monstrous Hound write it inside a deontic gated on a
CONDITION, where `Compulsion` offers `GatedBy`'s cost and nothing else;
Defensive Formation and Invasion Plans hand the combat-damage or block
ASSIGNMENT to a player; Contested Game Ball, Karazikar and Norn's Decree need
headers this vocabulary has not reached. The corpus's much commoner "THAT
attacking player" is not this row — it reads a header's announcement, which
is `That PlayerW`.

**The attacker-voice predicate — no new row, one bench.** `blessedReversal`
benches WHOLE and is the round's evidence that `AttackerOf` reaches the bare
participle as well as the relative clause; one row spells both, a reduced
relative stating no fact its full form does not, which is
`ResolvedPermanent`'s recorded principle at another surface. The 7 lines that
first bought the row all sat inside a gate's cost; this is the first carrier
outside one.

### Declined, with counts

- **The entry rider's defender slot — 27 supported lines (14 outside
  myriad's reminder text), declined on LAYERING and routed.** [CR#508.4]
  gives the rider exactly the content `BecomesAttacking`'s `AttackDefender`
  slot has. What blocks it is that `TokenRider` is declared in `Words.idr`,
  below both `Noun` and `AttackDefender`; giving it the slot means moving the
  type up the module order, which is its own round. Recorded in
  `BecomesAttacking`'s docstring.
- **The defender described by the ATTACKING PLAYER — 1 supported line,
  declined.** Tahngarth, First Mate's "a player or planeswalker that opponent
  is attacking" wants `AttackedBy` with a PLAYER relatum where that row takes
  an object. Already recorded in `AttackedBy`'s docstring; one line does not
  pay for the widening.
- **`Attacking`/`Blocking` were NOT collapsed into an indexed row.** The four
  roles are one vocabulary in the rules and four sibling constructors in the
  grammar, which is the house shape for `BlockerOf`/`BlockedBy` and every
  other closed pair here. Collapsing them would have rewritten ~35 existing
  call sites across `Cards.idr` and the pins for no claim the docstrings do
  not already make. `Attacking` gained the docstring that anchors the
  vocabulary; it had none.

### Routed forward

- The entry rider's defender slot, and the `TokenRider` module move it needs.
- The marked read at the PLAYER sort ("the last chosen player") — Beckoning
  Will-o'-Wisp and Triarch Stalker, together with the flavour-word row.
- `Other` outside a targeted context (Namor, Atlantean King).
- The player-kind partitive ("one of your opponents") — Martial Impetus,
  Oviya, Scriv, Seifer.
- A deontic gated on a CONDITION rather than a cost — Goblin Goon, Mogg
  Toady, Monstrous Hound.
- Two reads of one amount in one sentence (Souls of the Faultless).
- The combat-damage and block ASSIGNMENT decision — Defensive Formation,
  Invasion Plans.
