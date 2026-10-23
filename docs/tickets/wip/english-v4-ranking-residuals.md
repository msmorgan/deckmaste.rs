---
needs: []
---
# Retired (owner round 9: ranking is a heuristic, not a gate)

**Retired.** Left in `planned/` for the coordinator to close.

## Why

This ticket tracked every Corpus Unit or Judgment whose cheapest Ranking Key
tier held an unendorsed Reading, instead of the endorsed one or tied with it.
Each row was to be resolved by a better Ranking Key or by a grammar fix, and
recorded as a `ranks first` judgment.

The owner's round-9 ruling (`docs/decisions/english-v4.md` §4.4, items 6–8)
retires that work:
- The Ranking Key is a heuristic traversal order, not a correctness gate.
  Where it ranks the endorsed Reading is a reported statistic, never an
  acceptance criterion.
- No new Attachment Class, Production Cost tweak or ranking-residual ticket
  is added to win a #1 rank. Residuals need no tracking.
- The judgment harness no longer has a `ranks first` form.

When this ticket was written, every row's endorsed Reading was in its forest:
without `ranks first`, all 23 identifying entries held. So no row records a
missing analysis. A row whose cheapest tier holds a Reading that no English
grammar admits would be a grammar defect, owned by its own ticket. Row 23 is
owned by `english-v4-agency-outfitter-coordination-ranking`, which is now
recast as a grammatical admission fix: Right Node Raising must take the right
edge of a conjunct.

The identifying-entry block is removed, because its entries used the retired
`ranks first` form. The table and its notes below stay as a record of what
the Ranking Key put first on `tztrqzts`.

## The measured residuals (informational record)

Measured on `tztrqzts` with `cargo xtask english-v4 parse`, with card scope
wherever the text names a card. A key is AttachmentClass/RightAssociationWords/
ProductionCost, as `parse` prints it. "Endorsed key" is the key of the cheapest
Reading that satisfied the row's entry; "ahead" counts the Readings ranked
before it.

| # | Unit | Recorded in | Cheapest key (tier) | Endorsed key | Status |
|---|---|---|---|---|---|
| 1 | Teferi, Master of Time, line 1 (*loyalty abilities of Teferi on any player's turn …*) | `english-v4-unextended-and-closures` | 0/5/34 (1) | 0/15/34 | open; 18 ahead |
| 2 | Ember Island Production, first mode (*except it's not legendary and it's a 4/4 Hero …*) | `english-v4-set-off-first-conjunct` | 0/22/48 (2) | 0/24/48 | open; 2 ahead |
| 3 | Ember Island Production, second mode (*… a 2/2 Coward …*) | `english-v4-set-off-first-conjunct` | 0/24/49 (2) | 0/25/49 | open; 2 ahead |
| 4 | Mishra, Eminent One (*except its name is Mishra's Warform and …*) | `english-v4-set-off-first-conjunct` | 0/34/87 (1) | 0/35/87 | open; 1 ahead |
| 5 | Nightmare Shepherd (*except it's 1/1 and it's a Nightmare …*) | `english-v4-set-off-first-conjunct` | 0/18/65 (1) | 0/22/65 | open; 1 ahead |
| 6 | Ratadrabik of Urborg (*except it's not legendary and it's a 2/2 black Zombie …*) | `english-v4-set-off-first-conjunct` | 0/18/57 (2) | 0/28/57 | open; 4 ahead |
| 7 | Spark Double (`enter_as_copy_except_clause.txt:23`) | `english-v4-set-off-first-conjunct` | 0/51/65 (3) | 0/51/65 | tied; 1 of 3 endorsed |
| 8 | Ray of Ruin (*target creature, Vehicle, or nonbasic land*) | `english-v4-attributive-participle-licence` | 0/0/20 (1) | 0/1/17 | open; 1 ahead |
| 9 | Rob the Hoard (*target artifact, enchantment, or legendary card*) | `english-v4-attributive-participle-licence` | 0/1/21 (1) | 0/5/24 | open; 1 ahead |
| 10 | Abuelo's Awakening (`noun_premodifiers.txt:23`) | `english-v4-compound-noun-premodifier-coordination` | 0/16/56 (1) | 0/20/56 | open; 3 ahead (Noun Phrase alone: 0/0/9 against 0/1/9) |
| 11 | Excava, the Risen Past (`noun_premodifiers.txt:24`) | `english-v4-compound-noun-premodifier-coordination` | 0/20/71 (1) | 0/25/71 | open; 3 ahead (Noun Phrase alone: 0/2/22 against 0/7/22) |
| 12 | Nameless Race, *plus* (`comparative_quantity_than.txt:16`) | `english-v4-fused-comparative-predicative` | 0/12/61 (1) | 0/16/61 | open; 2 ahead |
| 13 | `enter_as_copy_complement.txt:11` (*with … equal to this creature's power and as a Mutant*) | `english-v4-right-association-key` | 0/37/43 (1) | 0/43/40 | open; 5 ahead |
| 14 | `remove_source_attachment.txt:6` (*you may untap it and remove it from combat*) | `english-v4-right-association-key` | 0/0/22 (1) | 0/2/21 | open; 3 ahead |
| 15 | `gapped_coordination_shared_adjunct.txt:11` (*in any order*) | `english-v4-right-association-key` | 0/1/32 (1) | 0/8/31 | open; 2 ahead |
| 16 | `kind_of_counter.txt:9` (*counter on each Spider and legendary creature you control*) | `english-v4-right-association-key` | 0/-7/28 (1) | 0/-7/28 | resolved by `english-v4-attachment-classes` |
| 17 | Seal of the Guildpact (`predicative_gap_relatives.txt:5`) | `english-v4-attachment-classes` | 0/-2/35 (2) | 0/-2/35 | resolved by `usvxoorq` (the partitive Noun Phrase carries `DeterminativeDistribution`) |
| 18 | Xyris, the Writhing Storm (`pro_form_one.txt:4`) | partitive *each of* loss after `usvxoorq` | 0/-6/48 (1) | 0/2/48 | open; 5 ahead |
| 19 | Birgi, God of Storytelling (`clause_supplements_and_adjuncts.txt:5`) | partitive *each of* loss after `usvxoorq` | 0/7/27 (4) | 0/7/27 | tied; 1 of 4 endorsed |
| 20 | Temple of Cyclical Time, line 2 (`coordinated_focused_adjuncts.txt:4`) | `english-v4-excise-predicate-conjunct-hosts` | 0/7/37 (2) | 0/8/37 | open; 2 ahead |
| 21 | Hedron Alignment, line 2 (*in exile, in your hand, in your graveyard, and on the battlefield*) | `english-v4-excise-predicate-conjunct-hosts` | 0/7/58 (2) | 0/8/58 | open; 2 ahead |
| 22 | `no_more_than.txt:13` (*Activate no more than three times each turn.*) | `english-v4-frequency-np-direct-object` | 0/1/16 (1) | 0/6/14 | open; 1 ahead |
| 23 | Agency Outfitter (`coordination.txt:73`) | `english-v4-agency-outfitter-coordination-ranking` | 0/7/58 (1) | 0/18/58 | open; owned by that ticket |

## What the cheapest tier holds instead

- **Row 1.** The garden path *loyalty [abilities of Teferi on any player's]
  turn any time …*: a bare relative whose verb is *turn*. The endorsed Reading
  uses the face's Shortened Name. The same tree with the subtype *Teferi* costs
  0/15/32.
- **Rows 2–6.** [[C, except A] and B] ranks ahead of C, except [A and B]. The
  owner's round-5 ruling (item 7a) asked the Ranking Key to rank C, except
  [A and B] first. The round-6 ruling left these units un-tuned.
- **Row 7.** The tier also holds *if* [*it's a creature, it enters … , and it
  isn't legendary*] and [[C, except A], B, and C].
- **Rows 8–11.** These are coordinations of type words before a carrier noun
  (`docs/oracle-style-guide.md` §7, "Card types are common nouns and
  attributive modifiers"; the shortest useful checklist, item 6). Ray of Ruin
  names permanents. Its endorsed Reading is the Nominal coordination
  [*creature*], [*Vehicle*], or [*nonbasic land*], but the Attributes
  coordination [*creature, Vehicle, or nonbasic*] *land* ranks first. Rob the
  Hoard, Abuelo's Awakening and Excava name graveyard cards. Their endorsed
  Reading lets *card* distribute over the coordination, but the Nominal split
  with a bare *artifact* conjunct ranks first. The two directions conflict:
  whichever structure a change ranks first fixes one side and breaks the other.
- **Row 12.** *plus …* attaches inside the *of* Complement of the first
  *number* (*number of [white nontoken permanents your opponents control plus
  the total number of …]*). The endorsed Reading adds two Noun Phrases.
- **Row 13.** Conjunct balance coordinates *equal to this creature's power*
  with *as a Mutant* rather than the *with* phrase with the *as* phrase. The
  endorsed Reading also has *equal to …* postmodify *number*.
- **Row 14.** *from combat* becomes an Adjunct of the coordination [*untap it*]
  *and* [*remove it*] under *may*: the coordination host is free and the
  conjuncts balance. The endorsed Reading has *from combat* as *remove*'s
  Complement, and *may* over both conjuncts.
- **Row 15.** *in any order* postmodifies *library* rather than modifying
  *put*.
- **Rows 18–19.** Since the partitive *each of* is distributive, the
  distributive Attachment Class attaches *in each of their draw steps* to the
  matrix *draws*, outside the relative clause *they draw …*. It also attaches
  *during each of your turns* to *can* rather than *boast*.
- **Row 20.** The tier holds *only* [[*if …*] *and* [*only as a sorcery*]] and
  *has no time counters* [[*on it*] *and* [*only as a sorcery*]]. The judgment
  endorses [*only if …*] *and* [*only as a sorcery*]. It does not settle whether
  *on it* is on *counters* (0/8/37) or on *has* (0/10/37).
- **Row 21.** The tier puts the locative coordination on *named*, or closes an
  *if*-clause conjunct after *in exile*. Endorsed: the coordination on *card*
  (0/8/58) or on *own* (0/10/58).
- **Row 22.** *activate* takes *no more than three times each turn* as its
  Object. `english-v4-frequency-np-direct-object` records why Right
  Association prefers it, and the measured cost of reordering the components.
- **Row 23.** See `english-v4-agency-outfitter-coordination-ranking`.

Closed: retired by owner round 9; no work remains, integration moves it to done.

Census delta (measured on `okprvpoz` -> this landing): documentation and comment changes only; faces read 28,801, total Readings 245,825 and Surplus 106,859 before and after; lost faces: none.
Judgment set (measured on `okprvpoz`): not affected; `cargo xtask english-v4 strata --check` passes and the crate tests pass.
