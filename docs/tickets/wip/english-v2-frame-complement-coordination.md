---
needs: [english-v2-role-preemption-depth]
---
# Coordination of a Verb Frame's complement cluster

**B7a — minted 2026-09-04 from the `english-v2-attachment-class-declared` (R1)
landing review, H1, by coordinator ruling.** Authority: rewrite ADR "Amendment:
one scope device, principles before packing (2026-09-04)", section D of
`docs/memory/scratch/plan09-postmortem/b7-scope-device-design.md`.

Defect. A Verb Frame with two or more Complements has no Coordination for the
Complement cluster, so a sentence that coordinates two full clusters under one
verb has only an incoherent derivation: the Coordinator joins the innermost
Nominal of the first cluster with the first Nominal of the second, and the
second cluster's marked Complement becomes a Postmodifier of that pair. Forge
Devil (`07b0724f…`) is the witness — the tree does not say the second quantity
is dealt to the second recipient — and the shape recurs across the movement
frames. 62 of R1's 219 gains are that bracketing.

This is a Construction, not a scope-device case. The two readings differ in
which strings are Conjuncts, not in the attachment height of one right-peripheral
Constituent, so they are not scope variants and the packing device cannot mint
the missing one. Principle 2 of the scope device (frame-role preemption, reaching
the whole complement the frame governs) eliminates the incoherent reading; this
ticket supplies the coherent one, and the two must land together or those 62
identities lose coverage.

Pin. One general Construction: a Coordination whose Conjuncts are the Complement
cluster of one Verb Frame — the frame's non-head material realized once per
Conjunct, the head realized once before the first. Declared from English
(argument-cluster Coordination), not from the attested frames: the Construction
is one rule over the frame's declared roles, never one arm per frame and never a
list of frames. The three Coordinators get their three arms as every other
family does. No Construction, requirement or checker names a preposition, verb,
noun or card.

Acceptance. The witness sentence selects the cluster Coordination, uniquely or
as the single survivor; the incoherent bracketing has no derivation; R1's 62
identities keep coverage with a correct selected analysis, each listed. Report
the census for every frame the Construction reaches.

Standard constraints apply. Gate scope: `cargo test --workspace`.

## Identities routed here by the `english-v2-role-preemption-depth` landing (2026-09-04)

These keep coverage on the parent's bracketing after B6 — the right-periphery
rule does not reach an incoherent Coordination whose final Conjunct carries no
role preposition — so each still needs the coherent cluster Coordination this
ticket mints, and each must be listed in this ticket's acceptance:

- Trygon Prime (`0ac2cee1...`) — `... put a +1/+1 counter on it and a +1/+1
  counter on up to one other target attacking creature.`
- Sumala Sentry (`31b2e614...`) — `... put a +1/+1 counter on it and a +1/+1
  counter on this creature.`
- Evolutionary Escalation; X-23, Deadly Weapon; Ajani, the Greathearted; Stand
  Together; Serrated Biskelion; Filigree Vector; Juniper Order Ranger; Brokers
  Ascendancy; River Heralds' Boon — all
  `put <quantity> on <recipient> and <quantity> on <recipient>`, all selecting
  `put [<quantity> on <recipient> and <quantity>] on [<recipient>]`, whose
  Coordination joins a permanent with a counter.

## Landing record

### Outcome

Implemented one declaration-driven `FrameComplementPair` member category and
the ordinary `and`, `or`, and `and/or` positional Coordination arms over it.
Each member contains the Object followed by the frame-selected marker and the
Frame Complement. The selected declaration frame supplies both roles and the
marker through the generated frame-role accessor; `Marked` and `OptionalLex`
are interpreted structurally. No arm names a verb, preposition, lexeme, or
card, and the construction does not add a dominance edge, exception, narrowed
form, or census gate.

The generated checked-sequence compiler path and its compact structural-role
index are the necessary declaration-runtime seam. The latter preserves the
scanner carrier's existing 72-byte size contract. The environment keeps the
matched frame marker in its scan identity, so members with different declared
markers cannot be collapsed before the structural checker compares them.

The construction reaches these eight declared Predicate frames (a census,
never a gate):

- `Deal`: Object, `Lex(To)`, Object.
- `Put`: Object role, optional `Lex(From)`, `Lex(Onto)`, Frame Complement,
  optional Predicative Complement, optional Object marked by `Under`.
- `Put`: Object role, optional `Lex(From)`, `Lex(On)`, Frame Complement.
- `Put`: Object role, `Lex(To)`, Frame Complement.
- `Put`: Object, `Lex(Into)`, Object.
- `Put`: Object, `Lex(On)`, Object.
- `Remove`: Object, `Lex(From)`, Frame Complement.
- `Return`: Object role, optional `Lex(From)`, `Lex(To)`, Frame Complement,
  optional Predicative Complement, optional Object marked by `Under`.

All three coordinator arms are declared irrespective of corpus counts. The
selected corpus census is `and` 99, `or` 0, `and/or` 0; positive tests cover
all three arms. A mismatched-marker pair and a declaration with required
material after the complement are negative witnesses.

### Routed acceptance identities

All eleven identities routed from B6, plus the original Forge Devil witness,
select the coherent `VerbPhraseAndFrameComplementPairCoordination` analysis:

| Identity | Card | Selected analysis |
| --- | --- | --- |
| `07b0724f…` | Forge Devil | `deals [1 damage to target creature] and [1 damage to you]` |
| `0ac2cee1…` | Trygon Prime | `put [a +1/+1 counter on it] and [a +1/+1 counter on up to one other target attacking creature]` |
| `31b2e614…` | Sumala Sentry | `put [a +1/+1 counter on it] and [a +1/+1 counter on this creature]` |
| `09d932ee…` | Evolutionary Escalation | `put [three +1/+1 counters on target creature you control] and [three +1/+1 counters on target creature an opponent controls]` |
| `18ce9faa…` | X-23, Deadly Weapon | `put [a +1/+1 counter on that creature] and [a +1/+1 counter on X-23]` |
| `3dec1f46…` | Ajani, the Greathearted | `put [a +1/+1 counter on each creature you control] and [a loyalty counter on each other planeswalker you control]` |
| `510d7713…` | Stand Together | `put [two +1/+1 counters on target creature] and [two +1/+1 counters on another target creature]` |
| `5850b56e…` | Serrated Biskelion | `put [a -1/-1 counter on this creature] and [a -1/-1 counter on target creature]` |
| `c99ec7be…` | Filigree Vector | `put [a +1/+1 counter on each of any number of target creatures] and [a charge counter on each of any number of target artifacts]` |
| `d7d2e157…` | Juniper Order Ranger | `put [a +1/+1 counter on that creature] and [a +1/+1 counter on this creature]` |
| `f3e03d23…` | Brokers Ascendancy | `put [a +1/+1 counter on each creature you control] and [a loyalty counter on each planeswalker you control]` |
| `f7c6a215…` | River Heralds' Boon | `put [a +1/+1 counter on target creature] and [a +1/+1 counter on up to one target Merfolk]` |

The other two already-covered selections whose paths changed are also coherent:

| Identity | Card | Selected analysis |
| --- | --- | --- |
| `f9afecf8…` | Arwen, Mortal Queen | `put [a +1/+1 counter and a lifelink counter on that creature] and [a +1/+1 counter and a lifelink counter on Arwen]` |
| `9fd0a1ad…` | Captain America, Team Leader | `put [a +1/+1 counter on that Hero] and [a +1/+1 counter on Captain America]` |

For all thirteen path changes, the parent selected
`VerbPhrasePutOn > LocativeNounPhraseCoordination`; the landing selects
`VerbPhraseAndFrameComplementPairCoordination`. There are no other selected-path
changes.

### Coverage-lock delta and gained analyses

Report-mode coverage changed from 18,917 to 19,002 selected and covered units:
`+85 / -0`. Every gain selects
`VerbPhraseAndFrameComplementPairCoordination`; none is a negative oracle.

| Identity | Card | Selected analysis and oracle text |
| --- | --- | --- |
| `c9e023ed…` | Ancestral Memories | `VerbPhraseAndFrameComplementPairCoordination` — Look at the top seven cards of your library. Put two of them into your hand and the rest into your graveyard. |
| `ab20a1fb…` | Arc Trail | `VerbPhraseAndFrameComplementPairCoordination` — Arc Trail deals 2 damage to any target and 1 damage to any other target. |
| `497198a3…` | Assembled Alphas | `VerbPhraseAndFrameComplementPairCoordination` — Whenever this creature blocks or becomes blocked by a creature, this creature deals 3 damage to that creature and 3 damage to that creature's controller. |
| `2e050bee…` | Beast Hunt | `VerbPhraseAndFrameComplementPairCoordination` — Reveal the top three cards of your library. Put all creature cards revealed this way into your hand and the rest into your graveyard. |
| `3bcbe154…` | Bellowing Fiend | `VerbPhraseAndFrameComplementPairCoordination` — Flying<br>Whenever this creature deals damage to a creature, this creature deals 3 damage to that creature's controller and 3 damage to you. |
| `ee790cf0…` | Bitter Revelation | `VerbPhraseAndFrameComplementPairCoordination` — Look at the top four cards of your library. Put two of them into your hand and the rest into your graveyard. You lose 2 life. |
| `1a3fc9d3…` | Borborygmos Enraged | `VerbPhraseAndFrameComplementPairCoordination` — Trample<br>Whenever Borborygmos Enraged deals combat damage to a player, reveal the top three cards of your library. Put all land cards revealed this way into your hand and the rest into your graveyard.<br>Discard a land card: Borborygmos Enraged deals 3 damage to any target. |
| `b3b66d84…` | Boulder Dash | `VerbPhraseAndFrameComplementPairCoordination` — Boulder Dash deals 2 damage to any target and 1 damage to any other target. |
| `13e68337…` | Brothers of Fire | `VerbPhraseAndFrameComplementPairCoordination` — {1}{R}{R}: This creature deals 1 damage to any target and 1 damage to you. |
| `de8dbb85…` | Burn the Accursed | `VerbPhraseAndFrameComplementPairCoordination` — Burn the Accursed deals 5 damage to target creature and 2 damage to that creature's controller. If that creature would die this turn, exile it instead. |
| `967620d2…` | Burning Sun's Avatar | `VerbPhraseAndFrameComplementPairCoordination` — When this creature enters, it deals 3 damage to target opponent or planeswalker and 3 damage to up to one target creature. |
| `6e999065…` | Carnival // Carnage | `VerbPhraseAndFrameComplementPairCoordination` — Carnival deals 1 damage to target creature or planeswalker and 1 damage to that permanent's controller. |
| `5f2ed94c…` | Chandra's Fury | `VerbPhraseAndFrameComplementPairCoordination` — Chandra's Fury deals 4 damage to target player or planeswalker and 1 damage to each creature that player or that planeswalker's controller controls. |
| `ebad1c36…` | Chandra's Outrage | `VerbPhraseAndFrameComplementPairCoordination` — Chandra's Outrage deals 4 damage to target creature and 2 damage to that creature's controller. |
| `4cf2baa6…` | Chandra, Flame's Fury | `VerbPhraseAndFrameComplementPairCoordination` — [+1]: Chandra deals 2 damage to any target.<br>[−2]: Chandra deals 4 damage to target creature and 2 damage to that creature's controller.<br>[−8]: Chandra deals 10 damage to target player and each creature that player controls. |
| `1e37be80…` | Char | `VerbPhraseAndFrameComplementPairCoordination` — Char deals 4 damage to any target and 2 damage to you. |
| `6db18548…` | Commune with Evil | `VerbPhraseAndFrameComplementPairCoordination` — Look at the top four cards of your library. Put one of them into your hand and the rest into your graveyard. You gain 3 life. |
| `76815000…` | Conduct Electricity | `VerbPhraseAndFrameComplementPairCoordination` — Conduct Electricity deals 6 damage to target creature and 2 damage to up to one target creature token. |
| `2b00c0ae…` | Confounding Riddle | `VerbPhraseAndFrameComplementPairCoordination` — Choose one —<br>• Look at the top four cards of your library. Put one of them into your hand and the rest into your graveyard.<br>• Counter target spell unless its controller pays {4}. |
| `7edefe40…` | Cunning Strike | `VerbPhraseAndFrameComplementPairCoordination` — Cunning Strike deals 2 damage to target creature and 2 damage to target player or planeswalker.<br>Draw a card. |
| `cc112b20…` | Cuombajj Witches | `VerbPhraseAndFrameComplementPairCoordination` — {T}: This creature deals 1 damage to any target and 1 damage to any target of an opponent's choice. |
| `409f4f40…` | Dagger Caster | `VerbPhraseAndFrameComplementPairCoordination` — When this creature enters, it deals 1 damage to each opponent and 1 damage to each creature your opponents control. |
| `621a9093…` | Daredevil's Billy Club | `VerbPhraseAndFrameComplementPairCoordination` — When this Equipment enters, it deals 2 damage to any target and 1 damage to you.<br>Equipped creature gets +1/+0 and has menace.<br>Equip {2} |
| `4ec3a9b7…` | Discerning Taste | `VerbPhraseAndFrameComplementPairCoordination` — Look at the top four cards of your library. Put one of them into your hand and the rest into your graveyard. You gain life equal to the greatest power among creature cards put into your graveyard this way. |
| `4d7b7602…` | Drakuseth, Maw of Flames | `VerbPhraseAndFrameComplementPairCoordination` — Flying<br>Whenever Drakuseth attacks, it deals 4 damage to any target and 3 damage to each of up to two other targets. |
| `2b0f9c4f…` | Explosive Welcome | `VerbPhraseAndFrameComplementPairCoordination` — Explosive Welcome deals 5 damage to any target and 3 damage to any other target. Add {R}{R}{R}. |
| `6c3f208e…` | Fear, Fire, Foes! | `VerbPhraseAndFrameComplementPairCoordination` — Damage can't be prevented this turn. Fear, Fire, Foes! deals X damage to target creature and 1 damage to each other creature with the same controller. |
| `c9caf165…` | Fireslinger | `VerbPhraseAndFrameComplementPairCoordination` — {T}: This creature deals 1 damage to any target and 1 damage to you. |
| `2af89064…` | Firja, Judge of Valor | `VerbPhraseAndFrameComplementPairCoordination` — Flying, lifelink<br>Whenever you cast your second spell each turn, look at the top three cards of your library. Put one of them into your hand and the rest into your graveyard. |
| `90a4b2f2…` | First Volley | `VerbPhraseAndFrameComplementPairCoordination` — First Volley deals 1 damage to target creature and 1 damage to that creature's controller. |
| `9bccfc1a…` | Forbidden Alchemy | `VerbPhraseAndFrameComplementPairCoordination` — Look at the top four cards of your library. Put one of them into your hand and the rest into your graveyard.<br>Flashback {6}{B} |
| `07b0724f…` | Forge Devil | `VerbPhraseAndFrameComplementPairCoordination` — When this creature enters, it deals 1 damage to target creature and 1 damage to you. |
| `a10aabc6…` | Glimpse the Future | `VerbPhraseAndFrameComplementPairCoordination` — Look at the top three cards of your library. Put one of them into your hand and the rest into your graveyard. |
| `29b87236…` | Goblin Artillery | `VerbPhraseAndFrameComplementPairCoordination` — {T}: This creature deals 2 damage to any target and 3 damage to you. |
| `b492d2e1…` | Granger Guildmage | `VerbPhraseAndFrameComplementPairCoordination` — {R}, {T}: This creature deals 1 damage to any target and 1 damage to you.<br>{W}, {T}: Target creature gains first strike until end of turn. |
| `593f0c06…` | Hungry Flames | `VerbPhraseAndFrameComplementPairCoordination` — Hungry Flames deals 3 damage to target creature and 2 damage to target player or planeswalker. |
| `b2f8dde5…` | Insult // Injury | `VerbPhraseAndFrameComplementPairCoordination` — Aftermath<br>Injury deals 2 damage to target creature and 2 damage to target player or planeswalker. |
| `273bd10f…` | Invasion of Regatha // Disciples of the Inferno | `VerbPhraseAndFrameComplementPairCoordination` — When this Siege enters, it deals 4 damage to another target battle or opponent and 1 damage to up to one target creature. |
| `18c19cb6…` | Judgment Bolt | `VerbPhraseAndFrameComplementPairCoordination` — Judgment Bolt deals 5 damage to target creature and X damage to that creature's controller, where X is the number of Equipment you control. |
| `af8dc759…` | Kruphix's Insight | `VerbPhraseAndFrameComplementPairCoordination` — Reveal the top six cards of your library. Put up to three enchantment cards from among them into your hand and the rest of the revealed cards into your graveyard. |
| `6b4bb950…` | Lunge | `VerbPhraseAndFrameComplementPairCoordination` — Lunge deals 2 damage to target creature and 2 damage to target player or planeswalker. |
| `7f5302ab…` | Maestros Charm | `VerbPhraseAndFrameComplementPairCoordination` — Choose one —<br>• Look at the top five cards of your library. Put one of those cards into your hand and the rest into your graveyard.<br>• Each opponent loses 3 life and you gain 3 life.<br>• Maestros Charm deals 5 damage to target creature or planeswalker. |
| `f1d5eec7…` | Mulch | `VerbPhraseAndFrameComplementPairCoordination` — Reveal the top four cards of your library. Put all land cards revealed this way into your hand and the rest into your graveyard. |
| `f707e8d1…` | Murmurs from Beyond | `VerbPhraseAndFrameComplementPairCoordination` — Reveal the top three cards of your library. An opponent chooses one of them. Put that card into your graveyard and the rest into your hand. |
| `eeaf6423…` | Neonate's Rush | `VerbPhraseAndFrameComplementPairCoordination` — This spell costs {1} less to cast if you control a Vampire.<br>Neonate's Rush deals 1 damage to target creature and 1 damage to its controller. Draw a card. |
| `32142ab1…` | Orcish Artillery | `VerbPhraseAndFrameComplementPairCoordination` — {T}: This creature deals 2 damage to any target and 3 damage to you. |
| `1a183bfb…` | Orcish Cannonade | `VerbPhraseAndFrameComplementPairCoordination` — Orcish Cannonade deals 2 damage to any target and 3 damage to you.<br>Draw a card. |
| `ca282295…` | Orcish Cannoneers | `VerbPhraseAndFrameComplementPairCoordination` — {T}: This creature deals 2 damage to any target and 3 damage to you. |
| `a93d2135…` | Organ Hoarder | `VerbPhraseAndFrameComplementPairCoordination` — When this creature enters, look at the top three cards of your library, then put one of them into your hand and the rest into your graveyard. |
| `88c3dc52…` | Pieces of the Puzzle | `VerbPhraseAndFrameComplementPairCoordination` — Reveal the top five cards of your library. Put up to two instant and/or sorcery cards from among them into your hand and the rest into your graveyard. |
| `e519feb9…` | Psionic Blast | `VerbPhraseAndFrameComplementPairCoordination` — Psionic Blast deals 4 damage to any target and 2 damage to you. |
| `608cbf39…` | Psionic Entity | `VerbPhraseAndFrameComplementPairCoordination` — {T}: This creature deals 2 damage to any target and 3 damage to itself. |
| `6932c2bb…` | Psionic Sliver | `VerbPhraseAndFrameComplementPairCoordination` — All Sliver creatures have "{T}: This creature deals 2 damage to any target and 3 damage to itself." |
| `d987633d…` | Punish the Enemy | `VerbPhraseAndFrameComplementPairCoordination` — Punish the Enemy deals 3 damage to target player or planeswalker and 3 damage to target creature. |
| `463c4131…` | Radiating Lightning | `VerbPhraseAndFrameComplementPairCoordination` — Radiating Lightning deals 3 damage to target player and 1 damage to each creature that player controls. |
| `5318002d…` | Rakdos Firewheeler | `VerbPhraseAndFrameComplementPairCoordination` — When this creature enters, it deals 2 damage to target opponent and 2 damage to up to one target creature or planeswalker. |
| `0c85e1f1…` | Rakshasa's Bargain | `VerbPhraseAndFrameComplementPairCoordination` — Look at the top four cards of your library. Put two of them into your hand and the rest into your graveyard. |
| `a5bd8054…` | Ransack the Lab | `VerbPhraseAndFrameComplementPairCoordination` — Look at the top three cards of your library. Put one of them into your hand and the rest into your graveyard. |
| `30d18f75…` | Reckless Embermage | `VerbPhraseAndFrameComplementPairCoordination` — {1}{R}: This creature deals 1 damage to any target and 1 damage to itself. |
| `3bfa5066…` | Reckless Rage | `VerbPhraseAndFrameComplementPairCoordination` — Reckless Rage deals 4 damage to target creature you don't control and 2 damage to target creature you control. |
| `3c92e3f1…` | Resentful Revelation | `VerbPhraseAndFrameComplementPairCoordination` — Look at the top three cards of your library. Put one of them into your hand and the rest into your graveyard.<br>Flashback {6}{B} |
| `efe0cfc4…` | Scattered Thoughts | `VerbPhraseAndFrameComplementPairCoordination` — Look at the top four cards of your library. Put two of those cards into your hand and the rest into your graveyard. |
| `87152346…` | Seismic Wave | `VerbPhraseAndFrameComplementPairCoordination` — Seismic Wave deals 2 damage to any target and 1 damage to each nonartifact creature target opponent controls. |
| `e0c71519…` | Self-Destruct | `VerbPhraseAndFrameComplementPairCoordination` — Target creature you control deals X damage to any other target and X damage to itself, where X is its power. |
| `4765a8e3…` | Shadow Guildmage | `VerbPhraseAndFrameComplementPairCoordination` — {U}, {T}: Put target creature you control on top of its owner's library.<br>{R}, {T}: This creature deals 1 damage to any target and 1 damage to you. |
| `9bcd735a…` | Shocker, Unshakable | `VerbPhraseAndFrameComplementPairCoordination` — During your turn, Shocker has first strike.<br>Vibro-Shock Gauntlets — When Shocker enters, he deals 2 damage to target creature and 2 damage to that creature's controller. |
| `8f9c849d…` | Shower of Sparks | `VerbPhraseAndFrameComplementPairCoordination` — Shower of Sparks deals 1 damage to target creature and 1 damage to target player or planeswalker. |
| `f0f1a07f…` | Soul of Shandalar | `VerbPhraseAndFrameComplementPairCoordination` — First strike<br>{3}{R}{R}: This creature deals 3 damage to target player or planeswalker and 3 damage to up to one target creature that player or that planeswalker's controller controls.<br>{3}{R}{R}, Exile this card from your graveyard: It deals 3 damage to target player or planeswalker and 3 damage to up to one target creature that player or that planeswalker's controller controls. |
| `e6a2a9f9…` | Sparksmith | `VerbPhraseAndFrameComplementPairCoordination` — {T}: This creature deals X damage to target creature and X damage to you, where X is the number of Goblins on the battlefield. |
| `009f02b5…` | Spicy Oatmeal Pizza | `VerbPhraseAndFrameComplementPairCoordination` — When this artifact enters, it deals 4 damage to any target and 3 damage to you.<br>{2}, {T}, Sacrifice this artifact: You gain 3 life. |
| `e9577dd1…` | Strategic Planning | `VerbPhraseAndFrameComplementPairCoordination` — Look at the top three cards of your library. Put one of them into your hand and the rest into your graveyard. |
| `1cc54dfa…` | Sultai Soothsayer | `VerbPhraseAndFrameComplementPairCoordination` — When this creature enters, look at the top four cards of your library. Put one of them into your hand and the rest into your graveyard. |
| `483a0b64…` | Sunflare Shaman | `VerbPhraseAndFrameComplementPairCoordination` — {1}{R}, {T}: This creature deals X damage to any target and X damage to itself, where X is the number of Elemental cards in your graveyard. |
| `4017913c…` | Taigam, Sidisi's Hand | `VerbPhraseAndFrameComplementPairCoordination` — Skip your draw step.<br>At the beginning of your upkeep, look at the top three cards of your library. Put one of them into your hand and the rest into your graveyard.<br>{B}, {T}, Exile X cards from your graveyard: Target creature gets -X/-X until end of turn. |
| `50d5e68f…` | Tamiyo, Collector of Tales | `VerbPhraseAndFrameComplementPairCoordination` — Spells and abilities your opponents control can't cause you to discard cards or sacrifice permanents.<br>[+1]: Choose a nonland card name, then reveal the top four cards of your library. Put all cards with the chosen name from among them into your hand and the rest into your graveyard.<br>[−3]: Return target card from your graveyard to your hand. |
| `8daf0859…` | Testament Bearer | `VerbPhraseAndFrameComplementPairCoordination` — When this creature dies, look at the top three cards of your library. Put one of them into your hand and the rest into your graveyard. |
| `acddc54f…` | The Brothers' War | `VerbPhraseAndFrameComplementPairCoordination` — I — Create two tapped Powerstone tokens.<br>II — Choose two target players. Until your next turn, each creature they control attacks the other chosen player each combat if able.<br>III — This Saga deals X damage to any target and X damage to any other target, where X is the number of artifacts you control. |
| `df4bff06…` | The Fall of Kroog | `VerbPhraseAndFrameComplementPairCoordination` — Choose target opponent. Destroy target land that player controls. The Fall of Kroog deals 3 damage to that player and 1 damage to each creature they control. |
| `ba182804…` | Tracker's Instincts | `VerbPhraseAndFrameComplementPairCoordination` — Reveal the top four cards of your library. Put a creature card from among them into your hand and the rest into your graveyard.<br>Flashback {2}{U} |
| `07fdd70d…` | Trick Shot | `VerbPhraseAndFrameComplementPairCoordination` — Trick Shot deals 6 damage to target creature and 2 damage to up to one other target creature token. |
| `c1a86020…` | Tropical Storm | `VerbPhraseAndFrameComplementPairCoordination` — Tropical Storm deals X damage to each creature with flying and 1 additional damage to each blue creature. |
| `71c17e3f…` | Unleash Shell | `VerbPhraseAndFrameComplementPairCoordination` — Unleash Shell deals 5 damage to target creature or planeswalker and 2 damage to that permanent's controller. |
| `e9cad880…` | Vigean Intuition | `VerbPhraseAndFrameComplementPairCoordination` — Choose a card type, then reveal the top four cards of your library. Put all cards of the chosen type revealed this way into your hand and the rest into your graveyard. |
| `56af1ad1…` | Volcanic Offering | `VerbPhraseAndFrameComplementPairCoordination` — Destroy target nonbasic land you don't control and target nonbasic land of an opponent's choice you don't control.<br>Volcanic Offering deals 7 damage to target creature you don't control and 7 damage to target creature of an opponent's choice you don't control. |
| `e37fa1af…` | Winding Way | `VerbPhraseAndFrameComplementPairCoordination` — Choose creature or land. Reveal the top four cards of your library. Put all cards of the chosen type revealed this way into your hand and the rest into your graveyard. |

### Verification and inventories

- Refreshed feature basis: parent tip `ykrsttlu`, claim change `uxxyspmv`;
  gates were run after that refresh.
- `cargo fmt --all`: exit 0.
- Strict clippy for the three touched crates and all targets: exit 0,
  `Finished` in 7.76s.
- `cargo test --workspace`: exit 0. Salient summaries include 43 compiled
  consumer tests, 154 compiler tests, 110 predicate-grammar tests, 76 TUI
  tests, 757 engine tests with 1 ignored, and 459 xtask tests with 1 ignored;
  determinism and all doctests passed.
- Coverage `--check`: 19,002 selected/covered; 13,639 parse failures; zero
  unresolved ties, internal failures, roundtrip mismatches, ownership failures,
  traversal failures, leaf-traversal failures, gaps, overlaps, synthetic
  claims, or provenance-plan mismatches. Report-mode lock delta: `+85 / -0`.
- Coverage `--bless`: exact same 19,002-unit accepted set and summary.
- Ambiguity `--require-resolved`: 32,641 total; 19,002 selected; 14,943
  unique; 4,059 specificity-resolved; 13,639 parse failures; zero unresolved
  ties, internal failures, exception resolutions, or exception uses.
- Parent-to-feature ambiguity diff: 98 units total: 85 parse failure to
  selected, and the 13 selected-path changes listed above.
- Roundtrip `--require-clean`: 19,002 accepted, 19,002 clean, zero
  mismatches, 13,639 not accepted.
- Construction count: 393 to 397 (`+4`: the member and three coordinator
  arms). Licensed homographs remain 2; form-literal/vocabulary overlaps remain
  9; permitted licensing checkers are 20 to 21; forbidden checkers remain 0.
  The new structural checker is the generically named
  `members_fill_declared_role`.
- Citations were not changed, so no cite gate was required.

Performance advisory (all corpus commands used 8 workers; the 16.26s ceiling
was exceeded under load and is reported, not fitted):

| Gate | Wall time | Integer ns/B | Host load (1m/5m/15m) |
| --- | ---: | ---: | --- |
| coverage check | 101.371298586s | 119,587 | 9.58 / 8.97 / 8.55 |
| coverage bless | 102.634668391s | 121,293 | 8.52 / 8.92 / 8.59 |
| ambiguity | 109.382890062s | 129,249 | 10.76 / 9.96 / 9.02 |
| roundtrip | 104.074313723s | 117,053 | 5.94 / 8.72 / 8.69 |

Concurrent-process count is unavailable because the sandbox cannot observe
sibling executors; the reviewer must stamp contention.

### Assurance accounting

- Restored: 0.
- Re-spelled: 1 existing production-census assertion, 20 to 21 permitted
  checkers.
- Ignored: 0.
- Added: 4 test functions and 5 additional invalid declaration cases inside
  an existing validation test. The functions cover generated checked
  structural roles, structural-pattern parsing, environment matching including
  optional lexical material and a required-trailer negative, and all three
  coordinator arms plus a mismatched-marker negative.
- Removed: 0.

### Deviations, additions, and disclosures

- Necessary supporting additions within the ticket's seam: generated
  checked-sequence lowering, the compact frame-role accessor index, preservation
  of the matched marker in environment scan identity, and a diagnostic category
  for frame-complement pairs. No unrelated construction or test was added or
  removed.
- During fence review, the generic structural checker was renamed from a name
  that repeated this construction's label to `members_fill_declared_role`.
  The full affected gate set was rerun after the correction.
- STOP: none. No selection tie, contradictory authority, unexplained loss,
  word-naming guard, wrong newly covered analysis, negative oracle, or
  roundtrip mismatch was found.
- Glossary gap: none.
- Decision wanted: none.
