---
needs: [lean-v3-ast-merges-by-rules-meaning]
---
# Is a measured domain an antecedent? (`DetPhrase.bare` after the universal merge)

Standard constraints apply. This is the domain half of
[lean-v3-ast-merges-by-rules-meaning](../done/lean-v3-ast-merges-by-rules-meaning.md) §2, split out
so that ticket could land. **Ruled by the owner on 2026-10-22 (option A, below). The
implementation is this ticket.**

## Where it stands

- **Ruled and landed (owner, 2026-10-21).** A universal bare plural and "all …" have one rules
  meaning and one plural referent, so the `bare` Template now expands to `.all`.
- **Ruled (owner, 2026-10-22), not yet implemented.** `DetPhrase.bare` is the determiner
  `countOf`, `aggregate`, `exists_` and `thereIsNo` write for the domain they measure: "the number
  of creatures you control", "the greatest power among creatures you control", "if there are cards
  in your graveyard". The ruling retires it; see "Ruling" at the end.
- **Until it lands, all three publish that domain.** The `countOf` (`Check/Phrase.lean:2653`),
  `aggregate` (`:2646`) and `exists_` (`:2729`) arms return the DRS after stepping it, so the
  domain stays visible to later clauses.
- **The question that was put** (owner, 2026-10-21, on the first STOP): should a measured domain
  be an antecedent at all? The audit below answered "it splits by construct", and the owner took
  that answer on 2026-10-22.

## Audit result (2026-10-21): it splits by construct

**"The number of …" and "the greatest … among …" (`countOf`, `aggregate`) are not antecedents.**
- No Vintage-legal Oracle text that follows "where X is / equal to the number of <X>" with a
  pronoun reads the counted domain back.
- **Tears of Rage:** "Attacking creatures you control get +X/+0 until end of turn, where X is the
  number of attacking creatures. Sacrifice those creatures …"
  - "Those creatures" reads the subject.
  - A published domain would win `AC>R1` here, because it is the more recent mention.
- **Mishra, Claimed by Gix:** "…where X is the number of attacking creatures. If Mishra, Claimed
  by Gix and a creature named Phyrexian Dragon Engine are attacking, and you both own and control
  them, exile them …"
  - "Them" reads the two named permanents, not the attacking creatures counted.
- **Highcliff Felidar:** "for each opponent, choose a creature with the greatest power among
  creatures that player controls. Destroy those creatures."
  - "Those creatures" reads the chosen creatures, not the domain the superlative ranges over.
- **Experiment (reverted).** I closed the `countOf` and `aggregate` arms with `leaveRelative` and
  dropped both `bare` skips. The whole bench stayed green, including
  `bareCountDomainDoesNotCompeteWithTheRemainderGroup`, with one exception: *Purging Scythe*
  (below).
- **The skips are needed only while `countOf` publishes.** Dropping the skips alone, with
  `countOf` still publishing, broke two pins: `theRestStandsWhereTheOtherRefuses`
  (`Cards/Anaphora.lean:607`) and `okBoreasChargerSpell` (`Cards/Choice.lean:1059`). In both, the
  counted domain competes with the group "the rest" is taken from.

**Existentials (`exists_`), counted ones included, are antecedents.**
- In each card below, the consequent reads the existential's domain. This is the way an
  indefinite in a conditional's antecedent is read in its consequent (`semantics-v3.md` §1.3,
  box 2).
  - **Evercoat Ursine:** "if there are cards exiled with it, you may play one of them".
  - **Brothers Yamazaki:** "If there are exactly two permanents named Brothers Yamazaki on the
    battlefield, the 'legend rule' doesn't apply to them".
  - **River Song's Diary:** "if there are four or more cards exiled with this artifact, choose one
    of them".
  - **Treasure Map:** "Then if there are three or more landmark counters on it, remove those
    counters".
  - **Cursed Recording:** "Then if there are seven or more time counters on it, remove those
    counters".
  - **Purging Scythe:** "If two or more creatures are tied for least toughness, you choose one of
    them".
  - **Loxodon Peacekeeper:** "If two or more players are tied for lowest life total, you choose
    one of them".
- **Rikala, Homarid King and Neck Tangle read the same way, but are not Vintage-legal.** They
  corroborate the pattern but are out of scope as cards.
  - **Rikala:** "Then if there are four or more tide counters on Rikala, remove them".
  - **Neck Tangle:** "If there are five or more Heads on the battlefield, tap two of them".

**Why this needs a ruling.**
- **Purging Scythe** is written today with `countOf`, but its text is a counted existential.
  - Rewriting it as one is refused by `existentialMention`, which admits only the bare domain
    (or `agree`).
  - So closing `countOf` means changing one of the checks below.
- **Loxodon Peacekeeper, River Song's Diary and Treasure Map** need the same counted
  existential.
- **The existential's bare-only determiner** is an English (definiteness) restriction, not one the
  CR draws.

## The seven checks that tell `.bare` from `.all`

The ruling retires the split in all seven (the two `Check/Words.lean` rows lose their `.bare` skip).

| Check | Where | What it does |
|---|---|---|
| `NounPhrase.countableGroup` | `Check/Phrase.lean:384` | Admits `.bare`, not `.all`, as what "the number of …" counts. |
| `NounPhrase.existentialMention` | `Check/Phrase.lean:387` | Admits only `.bare` (or `.agree` over a count or target) as the subject of `exists_`. |
| `countGroups` | `Check/Words.lean:762` | Read by `theRestOk`/`theOtherOk`. Skips `.bare` referents. |
| `countedGroupSize` | `Check/Words.lean:805` | Skips `.bare` referents. |
| `NounPhrase.perMemberOk` | `Check/Phrase.lean:358` | Admits `.all` and `.each`, not `.bare`, as a damage or counter recipient. |
| `NounPhrase.partitiveBase` | `Check/Phrase.lean:403` | Admits `.all`, not `.bare`, as the group "N of …" is taken from. |
| `NounPhrase.playerGroupMention` | `Check/Phrase.lean:413` | Reads `.described .all` of players as a group each member holds alone. Read by `soleHolderOk` and `slicePossessorOk`. |

`DetPhrase.participialNumber?` and `DetPhrase.plur` already treat the two alike.

## Pins that depend on them

| Pin | Where | Claim |
|---|---|---|
| `countRequiresACountableGroup` | `Proofs/RefusalReachability.lean:9` | `countOf` over `.bare` checks; over `.all` it is refused `countableGroup`. |
| `existentialRequiresAnExistentialMention` | `Proofs/RefusalReachability.lean:17` | `exists_` over `.bare` checks; over "you" it is refused `existentialMention`. |
| `bareCountDomainDoesNotCompeteWithTheRemainderGroup` | `Check/Queries.lean:152` | A `countOf` domain inside a slice's amount is not a group "the rest" is taken from. |
| `measuredDomainIsNotAPerMemberRecipient` | `Proofs/Damage.lean:345` | A raw `.bare` damage recipient is refused `perMember`. |
| `okBarePluralPlayerDamageRecipient` | `Proofs/Damage.lean:337` | The `bare` Template, now `.all`, checks as a recipient. Stays under any ruling. |
| `theRestStandsWhereTheOtherRefuses` | `Cards/Anaphora.lean:607` | Card-bench pins that read through the skips or the published domain (see the audit). |
| `okBoreasChargerSpell` | `Cards/Choice.lean:1059` | Same as above. |
| `okPurgingScythe` | `Cards/Damage.lean:235` | Same as above. |

## Corpus cases to pin whatever the ruling (reviewer, 2026-10-21)

Both cases bound the existential's publication, because the pronoun must not read its domain.

- **Flow State:** "Look at the top three cards of your library. Put one of them into your hand and
  the rest on the bottom of your library in any order. If there is an instant card and a sorcery
  card in your graveyard, instead put two of them into your hand and the rest on the bottom …"
  - "Them" and "the rest" read the three cards looked at.
  - The existential's domain must not take "them" or compete with that group for "the rest".
  - Its domain is a coordination, which publishes a group referent.
- **Titania, Voice of Gaea:** "if there are four or more land cards in your graveyard and you
  both own and control Titania, Voice of Gaea and a land named Argoth, Sanctum of Nature, exile
  them".
  - "Them" reads Titania and Argoth, not the land cards counted.

## Ruling (owner, 2026-10-22, option A)

The audit's split stands. The measured domain is an antecedent only where the consequent reads it.

1. **"The number of …" (`countOf`) and "the greatest … among …" (`aggregate`) are closed like a
   relative-clause box (`leaveRelative`) and are not antecedents.** No later pronoun or
   demonstrative reads the counted domain (Tears of Rage, Mishra, Claimed by Gix, Highcliff
   Felidar above).
2. **"There are …" (`exists_`) publishes its domain, which the consequent can read back, and
   admits counted subjects** ("two or more creatures", "four or more cards"). This is the way an
   indefinite in a conditional's antecedent is read in its consequent (`semantics-v3.md` §1.3,
   box 2). *Purging Scythe* and *Loxodon Peacekeeper* are the witnesses; *River Song's Diary* and
   *Treasure Map* take the same form when authored.
3. **The `bare` skips go.** Drop them in `countGroups` and `countedGroupSize`
   (`Check/Words.lean`). They were needed only while `countOf` published, and it no longer does.
4. **`DetPhrase.bare` is retired**, with the bare/all split in the checks of the table above that
   draw it: `countableGroup`, `existentialMention`, `perMemberOk`, `partitiveBase` and
   `playerGroupMention`. `countableGroup` admits the plural universal (`.all`) as what "the number
   of …" counts; `existentialMention` admits a counted or plain indefinite subject as well as
   `.agree`. The English definiteness restriction on the existential's determiner was never one
   the CR draws.
5. **Pin *Flow State* and *Titania, Voice of Gaea* to the readings in "Corpus cases" above.**
   They bound the existential's publication: "them" and "the rest" in *Flow State* read the three
   cards looked at, and "them" in *Titania* reads Titania and Argoth.

### Work this ticket carries

- Implement 1 to 4 in `Check/Phrase.lean`, `Check/Words.lean`, `Abilities.lean` (where `DetPhrase` is
  declared) and `Templates.lean`. `thereIsNo` also writes `.bare` today; its
  domain sits in a negated box (`semantics-v3.md` §1.3, box 3), so it is closed, and the
  implementer picks the surviving determiner for it. That choice is an implementation detail, not
  part of the ruling.
- **STOP condition for *Flow State*.** If the rules of `D′` and `AC>R1` as they stand cannot keep
  "them" and "the rest" on the three cards once the existential publishes its domain, do not
  weaken the pin and do not carve a skip back in. Report the rule that is missing and ask. The
  owner has ruled the reading, not the mechanism.
- Update `semantics-v3.md` §1.3 (the sentence on `DetPhrase.bare`) to the landed state, and drop
  `lean-v3-measured-domain-antecedent` from its "awaits" wording.

### Pins: rewritten, retired, added

Name each in the landing report, with the reason.

| Pin | Action |
|---|---|
| `countRequiresACountableGroup` | Rewritten. Same claim on the surviving determiner: `countOf` over a plural universal checks; over a non-group it is refused `countableGroup`. |
| `existentialRequiresAnExistentialMention` | Rewritten. `exists_` over a plain or counted indefinite checks; over "you" it is refused `existentialMention`. |
| `bareCountDomainDoesNotCompeteWithTheRemainderGroup` | Rewritten. Its claim now holds by closure: a `countOf` leaves no referent, so "the rest" takes the slice's group. Pin that no referent survives. |
| `measuredDomainIsNotAPerMemberRecipient` | Retired. It pins a raw `.bare` recipient, which no longer exists. The claim it guarded (a measured domain is not a recipient) is carried by the closure pin above. |
| `okBarePluralPlayerDamageRecipient` | Stays. |
| `theRestStandsWhereTheOtherRefuses`, `okBoreasChargerSpell` | Stay as written. They must pass with the skips dropped and `countOf` closed; that is the evidence the skips are no longer needed. |
| `okPurgingScythe` | Rewritten as a counted existential ("two or more creatures are tied for least toughness"), no longer `countOf`. |

New pins:

- *Loxodon Peacekeeper*, with its consequent reading "one of them" from the existential's domain.
- *Flow State* and *Titania, Voice of Gaea* as in "Corpus cases", plus a refusal-reachability pin
  that a pronoun after a closed `countOf` does not resolve to its domain (*Tears of Rage*,
  *Mishra, Claimed by Gix*, *Highcliff Felidar*).
- One card from each of *Evercoat Ursine*, *Brothers Yamazaki*, *Treasure Map*, *Cursed
  Recording* and *River Song's Diary* that reads its existential's domain back, when its card is
  authored.

## Owner rulings on the implementation STOPs (2026-10-22)

- **Q1, Flow State: option B.** A referent published by an existential condition
  (`existentialDomain`) ranks below ordinary earlier mentions for plural pronouns (D′) and for the
  "the rest" / "the other" group count. When an earlier group is in scope (Flow State's three
  cards looked at), "them" and "the rest" stay on it. When none is (Evercoat Ursine, Purging Scythe,
  Loxodon Peacekeeper), "them" reads the existential domain. Pin Flow State to the cards looked at.
- **Q1, follow-up ticket:** the self-replacement `replace … (instead …)` form refuses "the rest"
  even with no existential (Fblthp; Flow State's "instead" form). The proposed cause, to verify:
  `Action.replace` steps the replacement from the replaced action's post-state, after the
  partitive's remainder was already consumed. Mint it as its own ticket.
- **Q2, Titania: option A.** In a property test (`Condition.matches`), a coordinated subject may
  contain an indefinite conjunct. It reads existentially and publishes the coordinated group, so
  "exile them, then meld them" resolves. This covers the meld family: Titania, Mishra, Urza Lord
  Protector, Graf Rats, Hanweir Battlements, Midnight Scavengers. Pin Titania in this ticket.
