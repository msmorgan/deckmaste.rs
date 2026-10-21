---
needs: []
---
# A keyword item never ends a line's ability early, and a sentence never follows a keyword it does not belong to

`KeywordFollowedBySentences` (`Ability -> KeywordList Sentences`) is in the
grammar for one Oracle shape: a keyword whose em-dash cost ends in its own
period, followed on the same line by that keyword's associated instruction.
Oracle style guide §3 "Periods" puts it like this: *Equip—Sacrifice another
nonland permanent. Activate only once each turn.* The production accepts any
Keyword List as `first`, though. In 50 Corpus Units the line's top-level
segment boundaries therefore change from one Reading to another (measured on
`loxuknun`). The 3 legitimate witnesses read with one segmentation:
Shredder's Armor, Dark Knight's Greatsword, and Light Up the Night
(*Flashback—{3}{R}, Remove X loyalty counters …. If you cast this spell this
way, X can't be 0.*).

The wrong Readings fall into two shapes.

1. **A DashAbility keyword gives up the rest of its ability (40 units).**
   Power-up, Exhaust, Boast, Max speed, and Solved take an ability after an em
   dash (`DashAbility`). That ability is everything that follows on the line:
   - Power-up, Exhaust, and Boast label an activated ability written
     `[Cost]: [Effect.] [Activation instructions.]` [CR#702.193a,702.177a,702.142a,602.1,602.1b].
   - Max speed: "Max speed — [Ability]" [CR#702.178a].
   - Solved: "followed by ability text" [CR#702.169a].

   A paragraph is one ability [CR#113.2c], and style guide §1 "Paragraphs and
   ability boundaries" agrees. Today the item's ability can stop after any
   sentence, and `KeywordFollowedBySentences` adds the remaining sentences as if
   they followed the keyword. *Thanos, the Mad Titan* has three segmentations
   this way.

   Black Panther, Most Dangerous also reads the whole line as one `Sentence`,
   through `KeywordInstruction` over the Power-up item. It is the only unit
   that does. Find why and remove that Reading too: a keyword-ability line is
   not a Sentence (style guide §14 "Keyword abilities": no terminal period
   unless the keyword's fixed grammar has one).
2. **A word that begins an ordinary sentence is read as a keyword prefix (10
   units):**
   - *Equip abilities you activate … cost {N} less to activate.* gets an Equip
     item plus a bare imperative *cost {N} less to activate.* The keyword
     ability is written "Equip [cost]" [CR#702.6a], and no cost follows
     *Equip* here. The *Equip* is the Keyword Premodifier of *abilities*: a
     keyword mention in running text (style guide §14 "Keyword abilities").
   - *Blitz costs you pay cost {1} less …* gets a Blitz item, although the
     keyword ability is written "Blitz [cost]" [CR#702.152a].
   - *Storm Seeker deals damage …* gets a bare Storm item [CR#702.40a] plus the
     sentence *Seeker deals damage …*.

   Style guide §8 "Keyword abilities on a shared line" says: "Do not join a
   keyword and a nonkeyword sentence … Put the sentence in its own paragraph."

## Change

- Let `KeywordFollowedBySentences` take as `first` only a single keyword item
  whose last Keyword Parameter is an em-dash cost (`DashNonManaCost` or
  `DashCost`). That is the only form whose fixed text ends in a period of its
  own, with an associated instruction able to follow it on the line (style
  guide §3 "Periods").
  - The distinction must come from the catalog's parameter frame, through the
    `Parameters` schema. It must not come from a named keyword. No keyword name
    enters the grammar (`english-v4.md` §2). The ADR's ban on policing flags
    (§4.2) applies: this is a fact about the keyword form's punctuation, not a
    guard against an attachment.
  - A DashAbility item always ends at the end of the line.
  - A bare item, a cost-after-space item, or a quality item is never followed
    by a sentence on its own line.
- Remove the whole-line `KeywordInstruction` Reading of a DashAbility item
  (Black Panther).
- Add Judgments that pin the intended structure: one or more entries per
  shape, in the existing keyword-line judgment files rather than a new topic.
  A Sentence inside a DashAbility item's own ability is legitimate, so do not
  assert its absence there. Instead, assert the wrong analysis that the fix
  removes, and assert a `readings` count where the count is the claim. For
  example:
  - `+ <card=Fighter Class> Equip abilities you activate cost {2} less to activate. => !Sentence "cost {2} less to activate."`
  - `+ <card=Storm Seeker> Storm Seeker deals damage to target player equal to the number of cards in that player's hand. => !Sentence "Seeker deals damage to target player equal to the number of cards in that player's hand."`
  - `+ <card=Black Panther, Most Dangerous> Power-up — … => !Sentence "Power-up — …"`, with the whole line as the surface.

Read before implementing: `docs/decisions/english-v4.md` §2 and §4.2;
`docs/decisions/english-lexical-analysis.md` §2 ("Wrong analyses"). In
`docs/oracle-style-guide.md`: the front matter through "The shortest useful
checklist"; §1 "Paragraphs and ability boundaries"; §3 "Periods", "Colons",
"Em dashes"; §8 "Activated abilities" (all subsections) and "Keyword abilities
on a shared line"; §14 "Keep the five systems separate", "Keyword abilities",
"Ability words and flavor words"; §15 "Cases".

## Done when

- No Corpus Unit's segmentation into top-level segments changes between its
  Readings. Check this with a scratch probe kept outside tracked paths (never
  under `crates/`). It walks each Packed Forest from the root through the outer
  categories and records where each top-level segment starts and ends, ignoring
  structural punctuation (`.`, `•`, `—`, `:`, `|`, `,`, spaces).
  - The outer categories are `Document`, `Paragraphs`, `Ability`, `Sentences`,
    `Effect`, `Mode`, `Modes`, `ModalAbility`, `ModeEffect`, and
    `KeywordList`, plus the keyword item, phrase, and parameters of a
    DashAbility keyword.
  - Every read unit has exactly one segmentation, and its Reading count equals
    the product of its segments' counts. `english-v4-per-sentence-reading-counts`
    describes this walk in full.
  - For this ticket's units, that needs both this change and
    `english-v4-mode-bullet-misanalyses`.
- No Corpus Unit's Reading count goes up and no face is lost.
- Each unit below has the count shown after the arrow (before → after). That is
  its count with only the wrong segmentations removed. If a count stays higher,
  show which Reading remains and why. Light Up the Night, Shredder's Armor, and
  Dark Knight's Greatsword keep their current Readings.

Acceptance set (50 units):

- Thanos, the Mad Titan (9 → 3): *Power-up — {C}{W}{U}{B}{R}{G}: Put two +1/+1 counters on Thanos. Choose odd or even. Destroy each other creature with mana value of the chosen quality.*
- Strong Back (21 → 15): *Equip abilities you activate that target enchanted creature cost {3} less to activate.*
- Unliving Legionnaire (4 → 2): *Power-up — {5}{B}{B}: Return up to one target creature card from your graveyard to your hand. Put two +1/+1 counters on this creature.*
- Baron Helmut Zemo (136 → 68): *Boast — Exile any number of black cards from your graveyard with fifteen or more black mana symbols among their mana costs: Copy those exiled cards. You may cast up to three of the copies without paying their mana costs.*
- Pacesetter Paragon (2 → 1): *Exhaust — {2}{R}: Put a +1/+1 counter on this creature. It gains double strike until end of turn.*
- Éowyn, Lady of Rohan (12 → 10): *Equip abilities you activate cost {1} less to activate.*
- Jeong Jeong, the Deserter (32 → 16): *Exhaust — {3}: Put a +1/+1 counter on Jeong Jeong. When you next cast a Lesson spell this turn, copy it and you may choose new targets for the copy.*
- Rocketeer Boostbuggy (2 → 1): *Exhaust — {3}: This Vehicle becomes an artifact creature. Put a +1/+1 counter on it.*
- Case of the Filched Falcon (10 → 5): *Solved — {2}{U}, Sacrifice this Case: Put four +1/+1 counters on target noncreature artifact. It becomes a 0/0 Bird creature with flying in addition to its other types.*
- Mindspring Merfolk (2 → 1): *Exhaust — {X}{U}{U}, {T}: Draw X cards. Put a +1/+1 counter on each Merfolk creature you control.*
- Tuskeri Firewalker (4 → 2): *Boast — {1}: Exile the top card of your library. You may play that card this turn.*
- Draconautics Engineer (2 → 1): *Exhaust — {R}: Other creatures you control gain haste until end of turn. Put a +1/+1 counter on this creature.*
- Rough Rhino Cavalry (2 → 1): *Exhaust — {8}: Put two +1/+1 counters on this creature. It gains trample until end of turn.*
- Boom Scholar (6 → 3): *Exhaust — {4}{R}{G}: Creatures and Vehicles you control gain trample until end of turn. Put two +1/+1 counters on this creature.*
- Henzie "Toolbox" Torre (66 → 56): *Blitz costs you pay cost {1} less for each time you've cast your commander from the command zone this game.*
- Slick Imitator (4 → 2): *Max speed — {1}, Sacrifice this creature: Copy target spell you control. You may choose new targets for the copy.*
- Case of the Trampled Garden (2 → 1): *Solved — Whenever you attack, put a +1/+1 counter on target attacking creature. It gains trample until end of turn.*
- Iron Fist, Hero for Hire (8 → 4): *Power-up — {7}{R}: Iron Fist deals 5 damage divided as you choose among up to five targets. Put five +1/+1 counters on Iron Fist.*
- Skyserpent Seeker (36 → 12): *Exhaust — {4}: Reveal cards from the top of your library until you reveal two land cards. Put those land cards onto the battlefield tapped and the rest on the bottom of your library in a random order. Put a +1/+1 counter on this creature.*
- Liliana the Repentant (30 → 10): *Exhaust — {5}{B}: Return target creature or planeswalker card from your graveyard to the battlefield. Put a +1/+1 counter on Liliana. Activate only as a sorcery.*
- Marshals' Pathcruiser (2 → 1): *Exhaust — {W}{U}{B}{R}{G}: This Vehicle becomes an artifact creature. Put two +1/+1 counters on it.*
- Dwarven Mauler (13 → 11): *Equip abilities you activate that target this creature cost {2} less to activate.*
- Abomination, Terrifying Titan (4 → 2): *Power-up — {5}{R/G}{R/G}: Put a +1/+1 counter on Abomination. He fights up to one target creature an opponent controls.*
- Stampeding Scurryfoot (2 → 1): *Exhaust — {3}{G}: Put a +1/+1 counter on this creature. Create a 3/3 green Elephant creature token.*
- Black Panther, Most Dangerous (3 → 1): *Power-up — {5}{W}{W}: Put two +1/+1 counters on Black Panther. Other creatures you control get +2/+2 until end of turn.*
- Case of the Burning Masks (6 → 2): *Solved — Sacrifice this Case: Exile the top three cards of your library. Choose one of them. You may play that card this turn.*
- Spire Mechcycle (24 → 12): *Exhaust — Tap another untapped Mount or Vehicle you control: This Vehicle becomes an artifact creature. Put a +1/+1 counter on it for each Mount and/or Vehicle you control other than this Vehicle.*
- Endrider Spikespitter (4 → 2): *Max speed — At the beginning of your upkeep, exile the top card of your library. You may play that card this turn.*
- Cloud, Planet's Champion (16 → 12): *Equip abilities you activate that target Cloud cost {2} less to activate.*
- Fighter Class (12 → 10): *Equip abilities you activate cost {2} less to activate.*
- Fervent Champion (13 → 11): *Equip abilities you activate that target this creature cost {3} less to activate.*
- Rangers' Refueler (2 → 1): *Exhaust — {4}: This Vehicle becomes an artifact creature. Put a +1/+1 counter on it.*
- Invasion Submersible (2 → 1): *Exhaust — Waterbend {3}: This Vehicle becomes an artifact creature. Put three +1/+1 counters on it.*
- Helitrooper (13 → 11): *Equip abilities you activate that target this creature cost {2} less to activate.*
- Extremis Elite (2 → 1): *Power-up — {4}{R}: Put two +1/+1 counters on this creature. It deals 1 damage to any target.*
- Mendicant Core, Guidelight (2 → 1): *Max speed — Whenever you cast an artifact spell, you may pay {1}. If you do, copy it.*
- Camera Launcher (2 → 1): *Exhaust — {3}: Put a +1/+1 counter on this creature. Create a 1/1 colorless Thopter artifact creature token with flying.*
- She-Hulk, Jade Defender (2 → 1): *Power-up — {4}{G}{G}: Destroy up to one target artifact or enchantment. Put a +1/+1 counter on She-Hulk.*
- Bureau Headmaster (12 → 10): *Equip abilities you activate cost {1} less to activate.*
- Ninja of the Hand (2 → 1): *Power-up — {4}{B}: Each opponent discards a card. Put a +1/+1 counter on this creature.*
- Hercules, Prince of Power (2 → 1): *Power-up — {4}{G}: Put a +1/+1 counter on Hercules. He gains vigilance, indestructible, and haste until end of turn.*
- Boommobile (2 → 1): *Exhaust — {X}{2}{R}: This Vehicle deals X damage to any target. Put a +1/+1 counter on this Vehicle.*
- Perilous Snare (4 → 2): *Max speed — {T}: Put a +1/+1 counter on target creature or Vehicle you control. Activate only as a sorcery.*
- Namora, the Sea Queen (2 → 1): *Power-up — {5}{U}: Put a +1/+1 counter on Namora. Create two 1/1 blue Merfolk creature tokens.*
- Axgard Braggart (2 → 1): *Boast — {1}{W}: Untap this creature. Put a +1/+1 counter on it.*
- Kang the Conqueror (12 → 4): *Power-up — {5}{U}{U}{U}: Put a +1/+1 counter on Kang. Take an extra turn after this one. During that turn, power-up abilities can't be activated.*
- Molly Hayes, Runaway (3 → 1): *Power-up — {5}{R}: Put two +1/+1 counters on Molly Hayes. Exile the top card of your library. Until the end of your next turn, you may play that card.*
- Storm Seeker (24 → 12): *Storm Seeker deals damage to target player equal to the number of cards in that player's hand.*
- Bitter Work (2 → 1): *Exhaust — {4}: Earthbend 4. Activate only during your turn.*
- Donald Blake, Guise of Thor (2 → 1): *Power-up — {4}{W}{W}: Put two +1/+1 counters and a flying counter on Donald Blake. He becomes a God Warrior Hero.*

Standard constraints apply.

Census delta: measured at landing.
Judgment set: run at landing.
