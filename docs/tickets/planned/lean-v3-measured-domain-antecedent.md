---
needs: [lean-v3-ast-merges-by-rules-meaning]
---
# Is a measured domain an antecedent? (`DetPhrase.bare` after the universal merge)

Standard constraints apply. This is the domain half of
[lean-v3-ast-merges-by-rules-meaning](../wip/lean-v3-ast-merges-by-rules-meaning.md) §2, split out
so that ticket can land. **It waits on an owner ruling. Nothing below is ruled except where marked.**

## Where it stands

- **Ruled and landed (owner, 2026-10-21).** A universal bare plural and "all …" have one rules
  meaning and one plural referent, so the `bare` Template now expands to `.all`.
- **Still to decide.** `DetPhrase.bare` remains as the determiner `countOf`, `aggregate`,
  `exists_` and `thereIsNo` write for the domain they measure: "the number of creatures you
  control", "the greatest power among creatures you control", "if there are cards in your
  graveyard".
- **Today all three publish that domain.** The `countOf` (`Check/Phrase.lean:2653`), `aggregate`
  (`:2646`) and `exists_` (`:2729`) arms return the DRS after stepping it, so the domain stays
  visible to later clauses.
- **The owner's question** (ruling of 2026-10-21 on the first STOP): should a measured domain be
  an antecedent at all?
  - If not, close it like a relative-clause box, and remove the `bare` skips and the domain
    determiner.
  - If so, keep a domain determiner and name the rule that draws the distinction.

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

## Proposed, pending owner ruling

1. Close `countOf` and `aggregate` like a relative-clause box (`leaveRelative`).
2. Drop the `bare` skips in `countGroups` and `countedGroupSize`.
3. Let `exists_` publish its domain and admit a counted subject ("two or more creatures", "four
   or more cards"). Re-spell *Purging Scythe* (and *Loxodon Peacekeeper* and *River Song's Diary*
   when authored) as counted existentials. Pin *Flow State* and *Titania, Voice of Gaea* so the
   existential's domain loses to the nearer or more salient antecedent.
4. Retire `DetPhrase.bare` and the `bare`/`all` split in `countableGroup`, `existentialMention`,
   `perMemberOk`, `partitiveBase` and `playerGroupMention`. Rewrite the pins above to the new
   claims, naming each one rewritten or retired and why.
