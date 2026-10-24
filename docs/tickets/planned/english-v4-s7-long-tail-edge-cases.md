---
needs: []
---
# Stratum 7: long-tail edge cases

Part of `english-v4-stratum-7-discourse-and-long-tail`. Implement in `crates/deckmaste_english_v4` per [`docs/decisions/english-v4.md`](../../decisions/english-v4.md) (§6, Stratum 7).

## Scope

Faces that fit no cluster. The coordination faces with existing owners (the unlike-cluster, cleave-transparent-brackets and gapping-remnants tickets) are not repeated here.

- **Finale of Promise**: *cast [up to one target instant card and/or up to one target sorcery card] [from your graveyard] [each with mana value X or less]*. Hypothesis: floated *each* + postmodifier ordering.
- **Marvo, Deep Operative**: *cast a spell from your hand with mana value 8 or less without paying its mana cost* (unverified). Second triage: noun *clash* missing (*win a clash*).
- **Long-Term Plans**: *put that card third from the top*. Hypothesis: *put* requires a locative PP, which the position phrase does not satisfy (unverified).
- **Wheel of Misfortune**: *secretly chooses a number 0 or greater … determine the highest and lowest numbers revealed this way* (unverified). Second triage: *a number 0 or greater*, a postpositive numeric bound.
- **Barroom Brawl**: *target creature the opponent to your left controls*. Hypothesis: a PP-postmodified subject inside a bare relative. Second triage: noun *left* missing (*the opponent to your left*).
- **Widespread Brutality**: *then the Army you amassed deals damage …*. Hypothesis: the keyword action *amass* as a finite verb in a bare relative.
- **Azlask, the Swelling Scourge**: *Scions and Spawns you control gain [indestructible and annihilator 1]*. Hypothesis: plural subtypes as bare plurals, and a bare keyword coordinated with a parametric one.
- **Avatar of Woe**: *ten or more creature cards total*. Hypothesis: postpositive *total*.
- **Collective Voyage**: a *, where X is …,* supplement between two VP conjuncts.
- **Blood for the Blood God!**: *Exile Blood for the Blood God!*. The name's *!* doubles as the sentence's end mark. Design: a general lexer rule. A sentence-final token whose spelling already ends in terminal punctuation absorbs the sentence's period. That covers `.`, `!` and `?`, including an abbreviation point (*S.H.I.E.L.D.*, *Esq.*, *3 a.m.*) and a closing quote with a period inside. The name catalog knows each name's exact spelling. This rule would also replace the engine's `Summary.unwritten_period` for quoted abilities; coordinate with the re-scan's finding on it.
- **Strixhaven Stadium, Sway of the Stars**: second triage, unverified. Strixhaven Stadium: *remove them all*, postpositive *all* after an accusative pronoun; FloatedAll only licenses nominative subjects. Sway of the Stars: *their hand, graveyard, and all permanents they own*, a shared determiner over bare singular nominals plus a final conjunct with its own determiner.

## Witness cards

- Finale of Promise
- Marvo, Deep Operative
- Long-Term Plans
- Wheel of Misfortune
- Barroom Brawl
- Widespread Brutality
- Azlask, the Swelling Scourge
- Avatar of Woe
- Collective Voyage
- Blood for the Blood God!
- Strixhaven Stadium
- Sway of the Stars

## Estimated faces

12 faces. One further unread face is owned by `crates/deckmaste_english_v4/corrections.txt` (an Oracle agreement correction).

## Hypotheses, not findings

Every diagnosis above comes from a triage of the unread faces and is a hypothesis. Before changing anything, confirm it against the code and a parse of the card, and drop or replace any diagnosis that does not hold. Measure each construction before and after (§4.4, principle 5), and add a positive judgment per witness asserting its intended analysis (`crates/deckmaste_english_v4/judgments/`).

## Constraints

Governance is `docs/decisions/english-v4.md` §4.4, Principles: admit English, suppress nothing grammatical, add the general construction the text needs (any features, values, Categories, Productions or engine capabilities, each justified by a CGEL, OSG or CR citation), and keep the engine free of grammar names. Standard constraints apply.

## Landing note

Per `docs/tickets/README.md`, end the ticket with these two lines, stamped with the change id they were measured on:

1. **Census delta**: faces read and total Readings before and after; lost faces by name, or "none".
2. **Judgment set**: the result of running it.
