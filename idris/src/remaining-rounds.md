# Remaining rounds

The workbench's round queue. The chapter log (its §8 ledger and Representability
Frontier record what each chapter MET) is archived in the campaign's shared
local memory (2026-08-20; earlier revisions in jj history as
`idris/src/experiment-log.md`); this file records what is left to DO, in one
place, so a round can be picked without re-walking the log.

**Maintenance protocol.** When a round lands, delete its entry from this file and
squash that deletion into this file's own commit below `@` — the queue carries no
history of its own, only its current contents. The chapter that closed the entry
is the record; a struck-through line here would be a second copy that can drift.

Entries are grouped by what they cost, not by priority. Counts and witnesses come
from the log's own measurements and from round 4's recon; re-measure before
building, since several are a prior session's figures.

---

## Structural rounds — a new axis, and the reason Idris is worth paying for

**Bundle by shared packet.** A round's dominant cost is exposing the designer to
the packet, so entries whose declarations sit in the same region should be minted
in ONE round rather than several that reload the same context. Clusters are
marked below. Splitting a cluster needs a positive structural reason — differing
verb shapes, or a member blocked on machinery that does not exist — not merely
that the round looks large. Not everything clusters; the unmarked entries are
genuinely separate.

### Combat cluster

- **Combat-assignment surgery** — a could-block condition (resolution-time,
  reading restrictions and tapped-ness but not requirements or costs) and a
  blocks/stops-blocking write, which must express the remove-then-write versus
  stops-blocking difference ([CR#509.3a]). Same provenance and same caveat.
### Unclustered

- **The MARKER OBJECT's self-ascription** — one gap, two carriers, and the
  biggest single unlock the quotation rounds left. `TokenChars.abilities` and
  `GetsEmblem` both hold whole abilities now, and both families keep most of
  their spans behind a word `AsType` cannot give: it ascribes a card type or
  one of nine subtypes ([CR#109.2]'s "card type or subtype"), while [CR#111.1]
  makes "token" the marker word and [CR#114.3] leaves an emblem no types at
  all. THIS TOKEN is 193 of the 239 token-creation spans; THIS EMBLEM is 10 of
  the 90 emblem payloads (Chandra ×4, Karn, Koth, Narset, Ral). A THIRD axis on
  `AsType` rather than a row on an existing one, which is why chapters
  eighty-seven and eighty-eight both recorded it instead of buying it. Whoever
  takes it inherits the whole creation carrier by name: Anax, Hardened in the
  Forge, Mesmerizing Benthid, Harried Spearguard, Wolf's Quarry, Nesting Dragon
  and Reef Worm wait on this word and on nothing else in the payload.
- **The granted ability's subject in a non-battlefield zone** — FOUR supported
  spans, not the 40 chapter eighty-seven reported, and the correction is
  finding 657's: the earlier sweep read a whole clause segment where it should
  have read the subject NP, so every "Threshold — As long as there are seven or
  more cards in your graveyard, THIS CREATURE has '…'" and every "{2}, Exile
  this card from your hand: TARGET LAND gains '…'" was counted as a
  non-battlefield subject when the zone word is in the condition or the cost.
  The real four: Case of the Uneaten Feast, Kethis the Hidden Hand and The Grim
  Captain's Locker grant to cards in a GRAVEYARD, Lukka to cards EXILED this
  way; hand and library are zero. Under finding 85's bar, so this waits for a
  fifth card rather than for a design. Recorded because three of the four
  payloads are play permissions `MayPlay` already spells — it is the SUBJECT
  that refuses — and because [CR#113.6b] is what those payloads say about
  themselves.
- **Ability-as-a-value: the GRANTOR named from inside the quotation** — the
  last of the axis, and [CR#201.5a] is its rule. 47 quoted payloads name a card
  ("This token's power and toughness are each equal to the number of slime
  counters on Gutter Grime"; every Equipment that names itself in the ability
  it grants — Leonin Bola, Heartseeker, Blazing Torch, Hankyu), where the rule
  fixes the name to "the specific object which is that first ability's source"
  and `Named` describes a CLASS. A denotation gap, not a spelling one, so it
  wants a reference row and not a predicate. Gutter Grime is the cheapest whole
  card. Everything else on this axis closed in chapter eighty-seven.
- **The flip verb** — [CR#710]'s keyword action. LARGELY CLOSED by chapter
  seventy-two: the shape question is answered, because the merged `SetStatus`
  row takes the patient the fifteen-line count always wanted and
  `statusEffectOk Flipped` is open at 16 measured occurrences
  ([CR#710.1a]); Jushi Apprentice's whole ability is on the bench. One-way
  ([CR#710.4]), so no unflip row ever, and `badUnflipInstruction` now says so
  as a cell. What is LEFT here is the EVENT half and the conditions: no
  trigger header in the corpus watches a flip (`badFlipEvent`), and most of
  the fifteen lines remain blocked on condition machinery — Kitsune Mystic's
  "enchanted by two or more Auras", Bushi Tenderfoot's by-source lookback,
  Akki Lavarunner's non-combat damage-DEALING event (the vocabulary has the
  combat one and the patient side, and no general source-side row). Homura's "return it to the battlefield
  flipped" is NOT this verb's: it is the arrival-rider family's Flipped cell,
  the construction of the "…tapped" and "…face down" riders. Design note for
  whichever round opens the riders: the status-valued riders should index over
  `StatusVal`, making the rider vocabulary the third reader of [CR#110.5]'s
  product beside `HasStatus` and the transition verbs (`EntersAttacking` stays
  apart — a combat designation, not a status).
- **Standing triggered abilities** — the ability SHAPE beyond delayed queries.
- ~~**Dependent iteration**~~ and ~~**The fronted for-each binder**~~ —
  **BOTH CLOSED by chapter one hundred thirty-eight, and they were ONE ROW.**
  The entries counted 30 per-member lines and 25 fronted ones as separate
  gaps (five cards in both); measured together they are 49 distinct lines
  over 49 cards differing in nothing but which noun fills the domain slot,
  and finding 275 run over the six domain forms against the reader sets
  their bodies use found no covariance at all. `Effect.ForEachOf` takes the
  group as an argument and mints the element as the partitive's binding with
  the number fixed at one, which is all either entry needed: the "derived
  per-member agent" is `ControllerOf` over the element, unchanged and
  already written, its own `nounPlur n = OneOf` demand being exactly what a
  group could not satisfy. [CR#608.2f] is the rule and its own two examples
  are the two domain forms. `PerMember` and `EachOf` — the machinery both
  entries named — were untouched. Killing Wave, Fade Away, Martyr's Cry,
  Descent of the Dragons and Tidal Flats benched whole. THE RESIDUES ARE THE
  THREE BELOW; the repeat-process loop below them was built separately by
  chapter one hundred thirty-nine and shared no machinery with any of them,
  as predicted — though its until-condition cell turned out to want this
  round's own ledgered union-over-iterations mention.
- **The union-over-iterations mention** — three lines, and the one thing the
  binder deliberately does not contribute outward: "Those tokens gain haste"
  (Hate Mirage), "Exile those tokens at the beginning of the next end step"
  (Twinflame), "Those tokens have enchant creature and …" (Smoke Spirits'
  Aid). Each reads the tokens the BODY created, and reads them PLURAL, where
  the body creates one per iteration — so passing `effIntro body` outward
  would offer a singular mention no card writes and still not spell these.
  What they want is a mention denoting the union over iterations, which
  nothing here has. Small, well-bounded, and it is the only thing standing
  between these three cards and the bench (all three probed green up to
  their last sentence). CHAPTER ONE HUNDRED THIRTY-NINE FOUND THE SECOND
  CONSUMER, and it is three times the size: the repeat-process loop's
  until-condition cell is nine cards whose termination reads the
  iterations' own doing, not the board ("until no one pays life", "until
  all cards exiled this way have been chosen"), so whoever builds this
  mention should scope it against BOTH readers — twelve cards, one
  missing mention, and the entry below spells the nine out.
- **The PLAYER element** — the same construction one kind over, and a
  genuinely large cell the fronted entry's own "of [group]" narrowing hid:
  53 supported sentences over 50 cards front "For each opponent, …" / "For
  each player, …", and 32 of them read the element back as "that player"
  (Blatant Thievery's "For each opponent, gain control of target permanent
  that player controls" — which is [CR#608.2f]'s FIRST worked example, the
  rule naming this construction). Chapter one hundred thirty-eight typed
  `ForEachOf` at `Object` on its own corpus's evidence and routed this
  rather than folding it in unmeasured. The design pointer is one line:
  `elemIntro` generalises over `Kind` the way `bindFor` does, minting
  `PlayerP` where it now mints `ObjectP`, and the Kind index goes on the
  row. Note the boundary before scoping it — the wider "For each <fresh
  description>, …" surface is 237 sentences over 220 cards and most of it is
  the ordinary multiplier `CountOf` already spells; this entry is the
  read-back subset at the player sort and nothing else.
- **The permanent word after a zone change** — not a gap but a recorded
  TENSION, found while benching the binder. `wordReaches PermanentW` demands
  `onFieldZone`, correctly: [CR#110.1] makes a permanent a card or token on
  the battlefield and it "stops being a permanent as it's moved to another
  zone". The corpus writes the word anyway — Soul of Emancipation's "destroy
  up to three other target nonland permanents. For each of those permanents,
  its controller creates a 3/3 white Angel creature token with flying" reads
  "those permanents" of things that are in graveyards by then. Spelled with
  "them" the whole trigger elaborates (probed green), so the card is one
  demonstrative away from the bench. Whoever owns that cell decides whether
  the word's zone demand tolerates the loose reading; do not patch it from
  the binder's side.
- **The repeat-process loop — CLOSED by chapter one hundred thirty-nine**,
  which measured it at 44 lines over 44 cards (the entry's "five termination
  shapes" was four plus a construction that is not a termination) and landed
  `Effect.Repeat` with the three cells the corpus writes: nothing at all (17,
  every one of them gated by the conditional or may it sits inside), a
  written count (7, under the joined `RepeatCount` demand), and the unbounded
  offer (4, always under a `May` whose decider the node already carries). The
  clause is a LEAF and not a container because 20 of the 44 write it inside a
  conditional consequent, where a body-carrying loop would have to restate
  the gate as a termination phrase. Ad Nauseam, Primal Surge, Cultivator
  Colossus benched whole; Zimone and Dina and Another Round at their
  abilities. What is LEFT is the three entries below.
- **The loop's UNTIL-CONDITION**, 9 cards and the second-largest cell of the
  repeat family: Eureka, Hypergenesis ("until no one puts a card onto the
  battlefield"), Plague of Vermin, Mana Clash, Struggle for Sanity, Tainted
  Pact, Thieves' Auction, Timesifter, Helm of Obedience. NOT an ordinary
  board condition and that is why chapter one hundred thirty-nine minted no
  slot for it: all nine read the ITERATIONS' own doing, which is the
  union-over-iterations mention entry above (its second consumer, and its
  larger one). Two want a disjunctive termination besides ("whichever comes
  first" — Tainted Pact, Helm of Obedience), which is round ninety-nine's
  routed OR cell. Timesifter additionally waits on the tie condition already
  queued under the extremal-selection entry. Build the mention first; the
  cell is a one-constructor round afterwards.
- **The repeated SCHEMA**, 6 cards — "Repeat this process FOR …", where the
  domain does not bound the repetition but FILLS A HOLE in it once per item:
  Equipoise ("for artifacts and creatures", the earlier sentence having said
  lands), Firemind's Foresight ("for instant cards with mana values 2 and
  1"), Invoke Despair, Kathril (ten keywords), Linessa, and Protection Racket
  ("for each opponent in turn order"). What is missing is a parameterized
  clause: nothing here abstracts a slot out of a written process and refills
  it. PROTECTION RACKET DOES NOT COMPOSE WITH `ForEachOf` (checked, not
  assumed): that row takes a `Noun bs Object` and this domain is the player
  element the entry above routes, the iteration is ORDERED ("in turn order"),
  and the process is stated forward across the four sentences that follow.
  The rulebook writes this shape itself ([CR#701.44d], explore) — worth
  reading before scoping it.
- **The loop's FORWARD announcement**, 2 cards — "Repeat the following
  process X times" (Torment of Hailfire) and Protection Racket's. A cataphor
  where the other 42 write an anaphor: it ANNOUNCES a process and then states
  it, which is a different sentence and possibly a body-carrying node after
  all, for these two alone. Torment of Hailfire is blocked on a disjunctive
  alternative payment as well ("unless that player sacrifices a nonland
  permanent of their choice OR discards a card"), which is also Remorseless
  Punishment's one gap and the largest single blocker in the counted cell.
- **The counterfactual "as though" — the CAST-TIMING cell CLOSED by chapter one
  hundred fourteen**, which also measured the whole surface for the first time:
  264 supported sentences over 261 cards, in eight cells, and [CR#609.4]
  bipartitions them in its own words ("a player MAY do something 'as though' …
  or A CREATURE CAN do something 'as though' …"). A naive line grep returns 314;
  the 50-card difference is phasing REMINDER text ("treated as though it doesn't
  exist", [CR#702.26b]) and no card writes that phrase as its own line. What
  landed is `PlayAsThough = HadFlash` on `StaticEffect.MayPlay` (88 sentences;
  Vedalken Orrery, Shimmer Myr, Quick Sliver, Vernal Equinox, Borne Upon a Wind
  whole). What is left:
  - **The CREATURE-CAN half, 118 sentences in six cells, ONE BLOCKER FOR ALL OF
    THEM** — and it is not the counterfactual. `Compulsion` has `Forbid`,
    `Require` and `GatedBy` and no PERMISSION row, so "this creature can attack"
    is unwritable before the "as though" is reached. The permissive twin of the
    deed restriction is the round this half wants, and the counterfactual is a
    slot on it afterwards. The cells: attack despite defender 52 (the largest,
    and it wants nothing else — 45 of the 52 are a bare "This creature can
    attack as though it didn't have defender" under a condition or an activated
    cost), assign combat damage as though unblocked 19 (Thorn Elemental's
    family, always "You may have this creature …" — a permission whose subject
    is the PLAYER and whose actor is the creature, so it may want the other
    carrier), crew or saddle at a greater power 18 (all one sentence, "as though
    its power were 2 greater", 14 of them inside a quoted token ability),
    blocking despite evasion 17 (landwalk 11, shadow 4, "those abilities" 2,
    reach 1 and "as though they were untapped" 1 — a NEGATED-KEYWORD payload,
    which is a second counterfactual vocabulary and the one place this family
    wants a general one), attack
    despite summoning sickness 7, be targeted despite hexproof or shroud 5.
  - **The MANA half, 52 sentences, and it is NOT `ManaRider`/`SpendOnly`** —
    checked rather than assumed. That row is a RESTRICTION attached to mana as
    it is produced ("Spend this mana only to cast an Assassin spell", 164
    sentences); this is a PERMISSION widening what already-made mana may pay
    for, on a different carrier. [CR#609.4b] states its semantics separately
    ("this affects only how the player may pay a cost. It doesn't change that
    cost, and it doesn't change what mana was actually spent") and [CR#118.14]
    gives the sibling phrasing "mana of any type can be spent" its own rule —
    two surfaces, **52 "as though" sentences and 33 "can be spent" ones**
    (VIZIER OF THE MENAGERIE is one of the latter away from whole: chapter one
    hundred sixteen landed its first line and its second already wrote),
    which the round that takes this should measure against each other before
    minting either. The as-though half names a PURPOSE in 46 of its 52 ("to
    cast that spell" and its kin 39, "to activate" 7) and writes none in 6,
    which is `SpendPurpose`'s own axis and the strongest hint that the two
    families share a vocabulary even though they do not share a carrier.
  - **The ZONE counterfactual, 1 sentence** — Shaman's Trance's "You may play
    lands and cast spells from other players' graveyards this turn as though
    those cards were in your graveyard". It is the second row of
    `PlayAsThough`, deliberately unminted (finding 246), and it is also the ONE
    attested clause writing a place and an "as though" together — which is why
    `badZonedFlashPermission` is row-sensitive rather than slot-sensitive. Its
    card additionally wants a play-from-OTHER-PLAYERS'-graveyards permission and
    a coordinated "play lands and cast spells", so the row alone lands nothing.
  - **The six unclustered sentences**, each its own thing: Everlasting Torment
    and Phyrexian Unlife counterfactualise the damage SOURCE's keyword ("as
    though its source had wither/infect"), The Chain Veil and Elvish Refueler
    counterfactualise an ACTIVATION history ("as though none of its loyalty
    abilities have been activated"), Biomancer's Familiar an object's COUNTERS,
    and Shaman's Trance is above.
- ~~**The cast permission's SPELL-SORTED COMPLEMENT**~~ — **CLOSED by chapter
  one hundred fifteen**, and the answer to the question this entry posed was
  the OVERRIDE. A written source phrase says where the card is; the complement
  word says what playing it will make it. The rule derives from the table that
  was already there rather than adding a second one —
  `complementLocates z = playableFrom z`, a complement's zone locates its
  object exactly when that zone could have been the source — because the two
  zones `playableFrom` refuses are refused for being places nothing can be
  played FROM, which is exactly what makes them impossible as locative claims
  in a clause that names a source. It opened the LAND arm as well as the spell
  arm (finding 907), which a stack-only patch would have missed: 233 spell-worded
  and 17 land-worded sentences over a source phrase, 250 in all. The four
  permission pins all still refuse. What is left of the permission surface:
  - ~~**The TOP-OF-LIBRARY arm's two visibility riders**~~ — **CLOSED by
    chapter one hundred sixteen**, and the answer to the question this entry
    posed was ONE ROW. The axis was not minted: `ExposeVerb = LookAt | Reveal`
    has carried [CR#701.20a]/[CR#701.20e]'s "one operation, two audiences"
    since chapter twenty-two, and [CR#401.5] names the two surfaces in one
    sentence and then gives them one timing regime, which is [CR#604.6]'s move
    at the play permission the riders gate. The "may" and the possessive both
    fell out as construction-owned spelling (findings 916, 917), so the
    complement could be a closed two-row tag (`VisibleThing = TopOfLibrary |
    WholeHand`) rather than the exposure clause's general `Exposed`. ELEVEN
    WHOLE CARDS, all first attempt: Goblin Spy, Garruk's Horde, Future Sight,
    Magus of the Future, Courser of Kruphix, Telepathy, Wandering Eye,
    Korlessa Scale Singer, Assemble the Players, Precognition Field, Mystic
    Forge. What the riders did NOT unblock, each with its own gap:
    - **The GROUP POSSESSOR**, 3 reveal cards and 5 lines in all — Field of
      Dreams, Lantern of Insight and Wizened Snitches write "the top card of
      their libraries", the plural zone word `badSliceOfGroupPossessor`
      refuses on the positioned slice's own possessor question. It is a
      LibrarySlice question and not a visibility one; the riders themselves
      take those cards' subjects happily. Field of Dreams and Revelation
      additionally want the World supertype (2 cards).
    - **`Casts` has no SOURCE-ZONE slot**, and it is Melek, Izzet Paragon's
      only blocker — its copy trigger probed green in full without the phrase,
      so what is missing is "whenever you cast … FROM YOUR LIBRARY". Worth
      counting against the source-phrase family the permission round already
      opened.
    - ~~**The ADDITIONAL LAND PLAY**~~ — **CLOSED by chapter one hundred
      seventeen**, and ORACLE OF MUL DAYA IS WHOLE: three consecutive rounds
      each named that card and could not finish it, and its three lines are
      now one term apiece.
    - Routed elsewhere with the entry that owns them: Vizier of the
      Menagerie (chapter one hundred fourteen's mana half), Conspicuous Snoop
      and Skill Borrower (ability borrowing), Vampire Nocturnus (the
      coordinated subject — its top-card condition PROBED GREEN this
      chapter), Crown of Convergence and Mul Daya Channelers, Bolas's Citadel
      and Experimental Frenzy and Augur of Autumn and Cemetery Illuminator
      (an alternative cost, a source-phrase prohibition, a different-powers
      count, a shares-a-card-type relation).
  - **The FACE-DOWN look-at rider**, 3 lines plus one coordinated —
    "You may look at face-down creatures you don't control any time" (Keeper
    of the Lens, Found Footage, Lumbering Laundry) and Lens of Clarity's
    coordination of it with the library form. It is the same construction over
    an OBJECT DESCRIPTION rather than a place, overriding [CR#708.5] where the
    two landed rows override [CR#401.2] and [CR#402.3], and it wants a
    face-down predicate this grammar has no word for. Keeper of the Lens is a
    one-line whole card the day it lands.
  - **The NON-AGREEING look-at possessive**, 1 sentence — Xanathar, Guild
    Kingpin's "you may look at the top card of THEIR library any time", the
    only one of the family's 77 sentences whose possessive does not agree with
    its subject (finding 917). Its card additionally wants a play-the-top-card
    permission over another player's library and the mana half, so the
    possessor slot alone lands nothing.
  - **The bare WINDOW on a permission**, 2 sentences — "During each of your
    turns, you may play a land and cast a permanent spell of each permanent type
    from your graveyard" (Muldrotha, the Gravetide) and Coram, the Undertaker.
    These permit REPEATEDLY inside a window where `PlayLimit` permits once, so
    they are a different axis and were deliberately kept out of it (finding 910).
    Muldrotha additionally wants "of each permanent type", a distributive
    quantifier over card types.
  - **"without paying its mana cost"**, and it is the single largest companion
    blocker in the whole family — it appears on a clear majority of the
    permission cards whose other lines are otherwise writable (Omniscience is
    one line and blocked on nothing else; Omnispell Adept, Maelstrom Archangel,
    Jace's Mindseeker, Etali, Villainous Wealth, Epic Experiment and dozens
    more). It is the alternative-cost family and has its own entry; recorded
    here because it is what stands between this construction and most of its
    cards.
  - **The SUBTYPE-narrowed spell complement**, blocked one level down and NOT
    by anything this chapter touched: `And [spell, Or [HasSubtype Aura,
    HasSubtype Equipment]]` is refused by `ZoneCoherent`, because a
    permanent-only subtype word answers the battlefield inside a conjunction
    whose head answers the stack. DANITHA, NEW BENALIA'S LIGHT is otherwise
    whole (three keyword lines and a limited graveyard permission) and this is
    its only blocker. It is finding 908's question asked INSIDE the predicate
    vocabulary rather than at the permission, and it should be answered there
    the same way or explicitly differently — a creature-type subtype does not
    trigger it (Gisa and Geralf's "Zombie creature spell" elaborates), so the
    cell is narrow and well-defined.
  - **The self-permission's EXCLUSION clause**, 1 sentence — Haakon, Stromgald
    Scourge's "You may cast this card from your graveyard, but not from anywhere
    else". Its second line (the conditional over a subtype-narrowed graveyard
    cast) was probed green this chapter; the exclusion is the only blocker, and
    it is a negative co-ordinate on a permission that nothing else writes.
- **The rest of the "ADDITIONAL" surface**, 669 supported sentences in six
  cells, inventoried by chapter one hundred seventeen (finding 923) and every
  one of them a different construction from the land allowance it landed:
  - **The BLOCK ALLOWANCE, 31 sentences — the OBJECT-SORTED SIBLING and the
    nearest thing to a free round in this list.** "This creature can block an
    additional creature each combat" is 13 lines by itself, plus "each
    creature you control can block an additional creature each combat" (2),
    activated "this turn" forms (5), and the quantity axis writes the same
    four shapes ("an additional seven creatures", "an additional ninety-nine
    creatures", "up to two additional creatures"). It has the SAME window
    covariance the land cell has — "each combat" on a static line, "this
    turn" on a resolving one — so the derived-spelling argument transfers
    whole. What does NOT transfer is the constructor: the subject is an
    object, not a player, which is `PlayerCant`/`ObjectCant`'s situation
    exactly (two rows, siblings), and the rule is [CR#509.1a]'s blocker
    declaration rather than [CR#305.2]'s land count.
  - **The ADDITIONAL COST, 328** — already the cost machinery's territory;
    listed here only so the next inventory does not re-count it.
  - **The ADDITIONAL COUNTER AT ENTRY — CLOSED, and this entry's account
    of it was WRONG.** It read "a modifier on `EntersWithCounters`' amount
    rather than a construction of its own", and chapter one hundred
    twenty-one's probe found the amount was never the blocker and neither
    was the subject: that row has taken a general `Noun bs Object` since
    chapter ninety-one, so the class subject, the anchored complement, the
    counted class, the colorless class and the chosen-type class all
    elaborated already (finding 962). The only purchase was the WORD, an
    `EntryCounterMark` defaulted implicit at zero churn. RECOUNTED: 70
    occurrences over 70 sentences, not 96, inside a whole entry-counter
    surface of 578 over 352. GRUMGULLY, DRAGONSTORM GLOBE, SAGE OF FABLES
    and METALLIC MIMIC are benched whole. Residues:
    - **The COUNTER-BEARING DESCRIPTION — CLOSED by chapter one hundred
      twenty-nine, with this entry's count low and its blocker claim
      right.** "119 occurrences" is **221 frames over 219 sentences and
      212 cards** once the arrival-rider family is separated by whether
      the phrase hangs on a noun or on a placement verb (743 of the 964
      frames of these words are the rider or its kin). THE SAME CELL IS
      CARRIED TWICE: "Counter cluster" below counted its kind-blind slice
      alone at 26, itself stale against 46 — both entries closed on one
      account. The "SINGLE blocker" claim held and was PROVED rather than
      assumed: both cards were elaborated whole with a stand-in in the
      description slot before the row was designed (finding 1048), and
      BRAMBLEWOOD PARAGON and OONA'S BLACKGUARD are benched whole, the
      latter costing only `Subtype.Faerie`. Landed as
      `Predicate.HasCounters (kind : Maybe CounterKind)` at Object with
      `CounterKindNamed` — the kind a `Maybe` because finding 275 came
      back negative (175 named / 46 not, same environments), the scope
      fixed because the corpus's player-scoped description is corrupted's
      six THRESHOLD lines and the bare player existence is zero, the zone
      `zoneAdmit [Battlefield, Exile]` on 212/9/0. Negation rode in free
      (4 lines, `Not`). Residue: the BOUND read (below).
    - The COPY EXCEPTION (4) and the GRANTED QUOTATION (1) — two carriers
      of the same clause, neither this row's.
    - Cards blocked on one named other line apiece: MASTER BIOMANCER
      (counters and a type addition coordinated in one entry clause),
      RENATA, CALLED TO THE HUNT (devotion in a power definition), OATH OF
      GIDEON (a `Loyalty` counter kind — [CR#306.5b] gives every
      planeswalker the intrinsic "enters with a number of loyalty
      counters" ability, so the row has a rule behind it — plus a `Kor`
      subtype), ARLINN, VOICE OF THE PACK (Wolf, Werewolf, Arlinn rows),
      TAYAM (a three-counter removal cost over a described group), TROMELL
      (proliferate X), SLINZA (a cost reduction and a fight trigger),
      CURATOR BEASTIE (manifest dread), DEARLY DEPARTED (a
      graveyard-scoped conditional static), VAMPIRE SOCIALITE (the same
      with a life-loss lookback).
  - **The COUNTER-SCALING replacement, 17 sentences / 21 occurrences** —
    "If one or more [X] counters would be put on [subject], that many plus
    one (14) / twice that many (6) are put on it instead" (Hardened
    Scales, Winding Constrictor, CONCLAVE MENTOR — which is this family
    and NOT the entry-counter one — Doubling Season, Corpsejack Menace).
    Measured by chapter one hundred twenty-one and deliberately not built,
    with the transferable half named: chapter one hundred's `DamageScale`
    already carries `Multiplied` and `Shifted ShiftUp`, exactly the two
    values this family writes and no third, so the SCALE VOCABULARY is
    ready. What is not is the carrier — `Scales` is keyed by `DamageKind`
    and takes a damage source, where this family intercepts a counter
    placement on a described permanent, which is `Intercepts`' shape. Hardened
    Scales ({G}, one line) and Corpsejack Menace are the cheapest wholes.
    Its union-headed neighbours (Vorinclex, Innkeeper's Talent, Lae'zel)
    are NOT this family's — they belong to the distributive counter-kind
    anaphor below, whose own note records that chapter one hundred
    twenty-eight cleared their recipient read.
  - **The ADDITIONAL TIME, 46** — "that ability triggers an additional time"
    (a trigger multiplier, ~15 lines over a shared shape) and "while voting,
    you may vote an additional time" (3). Two allowances again, at two more
    subject sorts.
  - **The ADDITIONAL PHASE OR STEP** — RECOUNTED by chapter one hundred
    nineteen and moved: the construction is 27 sentences over 51
    occurrences, not 44 (finding 941 records the three patterns tried and
    that none reproduces the earlier figure). An EXISTENTIAL frame ("there
    is") over the turn schedule with [CR#500.10a]'s "you get" beside it;
    the full entry with its blockers now lives under the turn-schedule
    heading below, and not here. LANDED in chapter one hundred twenty for
    the existential frame (48 of the 51 occurrences); four residues there.
  - **The ADDITIONAL CARD, 31** — a draw inside a draw-step trigger (finding
    924), where the word is an adverbial on an event the turn already
    performs. RITES OF FLOURISHING is one line from whole behind it, its
    other line being the each-player land allowance this chapter landed.
  - Fenced from the landed row with counts: Nahiri's Lithoforming's "You may
    play X additional lands this turn" (1 — `Quantity` is Nat-based and the
    variable count is a magnitude), and FASTBOND's "You may play any number
    of lands on each of your turns" (1 — the unbounded allowance, which drops
    the word "additional" and states the whole number rather than increasing
    it; `badAnyNumberOfAdditionalLands` is the pin, and Fastbond's second
    line is a played-a-land trigger with an intervening if).
- **Ability borrowing** — and chapter one hundred sixteen puts two cards
  one line away from whole behind it: CONSPICUOUS SNOOP ("as long as the top
  card of your library is a Goblin card, this creature has all activated
  abilities of that card") and SKILL BORROWER (the same sentence over an
  artifact or creature card), both of whose OTHER lines now write — the
  visibility rider and a top-of-library permission apiece. The top-card
  condition itself probed green, so the gap is exactly "has all activated
  abilities of [that card]".
- **The COST GATE's remaining gaps** — what is left of chapter forty-nine's
  entry after chapter one hundred thirty-six landed the scaled payment
  (`Cost.ScaledMana`) and found that the scaled cost was never the gate
  family's only blocker. The family is 30 lines over 29 cards today (chapter
  forty-nine measured 26), 25 of them scaled, and NONE is benched but
  Hipparion. Every one of the 25 carries a second gap, and they are four
  distinct pieces of work, counted here so nobody scopes a round that
  unblocks nothing:
  - the DEFENDING-PLAYER noun, 15 lines ("Creatures can't attack you unless
    their controller pays…" — Propaganda, Ghostly Prison, Windborn Muse,
    Koskun Falls, Elephant Grass, Baird, Archon of Absolution, Dáin,
    Forbidding Spirit, Norn's Annex, Onakke Oathkeeper, Sphere of Safety,
    Summon: Yojimbo, Collective Restraint, Archangel of Tithes). The patient
    slot is `Noun bs Object` and `deonticPatientOk GateT Attack Agent` is
    `PatientRefused`, both with this case named in their own comments since
    chapter forty. THE MARQUEES ARE HERE, and this is the piece to take
    first: it is what Propaganda and Ghostly Prison wait on.
  - the ANAPHORIC COUNT, 9 lines ("pays {1} for each of THOSE creatures").
    `CountOf` takes a `Predicate`; "those creatures" is a plural mention and
    `Those (TypeW Creature)` is a `Noun`, so the count has nothing to run
    over. Overlaps the set above; a round that closes both closes 15+ lines
    at once.
  - the COORDINATED DEED, 3 lines ("can't attack or block" — Myr Prototype,
    Cowed by Wisdom, Whipgrass Entangler), chapter thirty-eight's
    coordinated-event gap at this site, unchanged.
  - the PAYER NOUN INSIDE AN ACTION COST, 2 lines (Heat Wave, Sivitri,
    Dragon Master). The gate DERIVES its payer and spells it; a life payment
    is a clause that writes its own payer noun, and "their controller" is not
    a noun the cost's context can see. New in chapter one hundred
    thirty-six; the scaled LIFE payment itself needs nothing (it composes
    today).
  Also still here: the Aura carrier (Brainwash, Oppressive Rays, Awesome
  Presence, Cowed by Wisdom) and Awesome Presence's "defending player", which
  is the same noun as the first piece read at the block role.
- **The scaled payment's three ledgered units** — the counted residues of
  chapter one hundred thirty-six, each too small for its own round and all
  three the same shape of purchase (a unit or a factor the magnitude cannot
  hold): the COLORED scaled payment, 3 lines (Cyclone's "{G} for each wind
  counter on it", Thelon's Curse's {U}, Norn's Annex's {W/P}) — `ScaledMana`
  carries an `Amount` and no symbol, exactly as `CostShift` does, and the two
  ledgers should be closed together if either is; the {X} PER-UNIT, 4 lines
  (Collective Restraint, Sphere of Safety, War Cadence, War Tax), which wants
  a product of two amounts where `Times` takes a written numeral
  ([CR#118.4]); and the "PLUS AN ADDITIONAL" compound, 3 lines (Rune Snag,
  Spell Stutter, Concerted Defense), a fixed base beside a scaled one under a
  coordinator `Compound`'s comma does not spell. Zero lines write a scaled
  ALTERNATIVE cost (the row reaches `AltCost` structurally and no card asks),
  and zero write a scaled non-mana ACTION cost ("as an additional cost …
  for each" returns nothing), so neither is owed anything.
- **The conditional's two orientations** — one design across both containers,
  and it has grown a second payer since chapter fifty-eight. For the STATIC:
  about 70 lines pronominalise the statement's subject inside the condition ("X
  has hexproof as long as IT's untapped") where chapter fifty's fronted flow
  pronominalises the condition's subject in the body. For the ONE-SHOT:
  `Effect.If` types the trailing orientation only (its condition sits at
  `preIntro e`, which is what `badTrailingPostStateZone` refuses a post-state
  zone at) while SPELLING both, so a fronted condition cannot contribute
  anything to its own consequent — which is what costs Balance of Power,
  Vraska's [−9] and Iymrith their gap read (chapter fifty-eight; Balance of
  Power's condition half compiles today, so the orientation is the whole
  blocker). One constructor cannot type both arguments in each other's context
  (finding 362), so both halves are second orientations — carried by the marking
  or by the order, whichever the corpus supports — and not a second `condIntro`.
  The independent-condition trailing forms are writable today and are NOT part
  of this.
- **The counted-condition "unless"** — a boundary found inside a settled family
  (finding 354): the conditional static's `Unless` marking demands a NEGATABLE
  condition, and `condNegatable` answers False for `CompareAmt`, so "unless you
  control an artifact" composes and "unless you control four or more artifacts"
  (Gadrak) does not. It belongs to the comparison's negation gap, not the
  deontic's, and it is small — measure before scheduling.
- **The coordination's per-part duration** — what chapter eighty-three left of
  the static coordination. The carrier LANDED (`AndAlso` over `StaticParts`,
  1,433 supported sentences over 1,407 cards, 36 signatures, findings
  609–612), and one envelope covers every part. Forty-five lines need two:
  41 of them are "<grant> until end of turn and can't be blocked this turn"
  (Distortion Strike, Taigam's Strike, Teleportal, Marchesa's Smuggler…) and
  six write the same word twice (the Mimic cycle — "has base power and
  toughness 4/2 until end of turn and gains first strike until end of turn").
  These are NOT a counterexample to the envelope, they are the span table
  refusing to let a grant and a restriction share a current-turn word
  (finding 613, `badCoordinatedSpanDisagree`), so the shape they want is a
  span slot per part rather than a second carrier. Measure the SAME-word-twice
  six first: if those six are a spelling variant of the shared envelope, the
  family shrinks to one coherent grant-plus-restriction class and the design
  is a per-part `Maybe Duration` with the envelope kept as the elided form.
- **The multi-sentence static line** — chapter eighty-four's own residue
  (finding 620). Eleven supported lines write two sentences on one static
  ability and read the first one's subject back in the second: "Equipped
  creature gets +2/+0. It gets an additional +0/+2 and has first strike as
  long as …" (Bride's Gown, Groom's Finery), "Enchanted creature can't attack
  or block. It loses all abilities and has \"{T}: …\"" (Heliod's Punishment),
  Retro-Mutation, Spider-Man No More, Intercessor's Arrest, Lost in Thought,
  Volrath's Curse, Shuriken. `Static : StaticEffect [] -> Ability` holds ONE
  statement, so this is a carrier question and not a mention one — the
  announcement is already there since chapter eighty-four, and what is missing
  is a place for the second sentence to stand. Its shape is `Effects`' and
  `StaticParts`' a third time (a telescope of statements typed in what their
  predecessors announced), the difference from `AndAlso` being the SPELLING —
  a full stop and a written subject where the coordination writes "and" and
  elides. Measure whether the eleven are one class before choosing between a
  second carrier and a marking on this one.
- **The statement's own subject read from INSIDE it** — the fourth container,
  and two positions with two different costs (finding 624). The pump's AMOUNT
  reads the subject on 9 lines ("Enchanted creature gets +2/+2 for each Aura
  attached to it", Auramancer's Guise; Golem-Skin Gauntlets, Luxior, Mantle of
  the Ancients, Strong Back, Thran Power Suit, Alpha Status, Stoneforge
  Masterwork), and that half is cheap: `Gets`' slots are typed at `nomIntro n`
  and would move to `selfSubjIntro n` exactly as `staticIntro` did, the same
  one-line change at a second site. The "as long as" CONDITION reads it on 16
  ("Enchanted creature gets +2/+2 as long as it's a Human", Bonds of Faith;
  Armament of Nyx, Burden of Proof, Clutch of Undeath, Favorable Destiny, Gift
  of Fangs …), and that half is not cheap: `Conditionally : (c : Condition bs)
  -> (se : StaticEffect (condIntro c))` threads condition INTO statement, so
  the adverbial reading its own statement's subject is an argument-order
  question. Cheapest witnesses: Auramancer's Guise for the amount (its other
  conjunct is a plain vigilance grant), Bonds of Faith for the condition.
- **The prenominal attachment predicate** — "equipped creatures you control
  have flying" (Dalakos, Crafter of Wonders; Blacksmith's Talent; Kemba, Kha
  Enduring; Stone Haven Outfitter; Greater Auramancy; A Tale for the Ages; Syr
  Armont; Zamriel; Firion; Resistance Reunited), 18 occurrences over 18 cards.
  Chapter fifty-one's finding 369 minted the inverse direction as a PREDICATE
  and recorded that the corpus writes it "predicatively after the copula and
  never prenominally"; these lines are the prenominal writing, so the record is
  half wrong and the reader is `attachedCheckOk`'s, not `AttachHost`'s. Check
  whether the participle is simply the predicate in attributive position
  (finding 275 on the copula) before minting anything.
- **`attachHeadOk Equipped PermanentW`, one attested line against a False
  cell** — Luxior, Giada's Gift writes "Equipped permanent isn't a planeswalker
  and is a creature in addition to its other types", where the cell says the
  equipped participle takes no permanent head (finding 625). The cell is
  rules-backed ([CR#301.5]: an Equipment "can't legally be attached to anything
  that isn't a creature") and so is the exception ([CR#702.6e]: "equip
  planeswalker" attaches "as though that planeswalker were a creature"), which
  is why the card cannot write "equipped creature". No pin stands on the cell.
  Flipping it buys nothing until the line's OTHER half exists — a negated type
  setting, "isn't a planeswalker" — so schedule the two together or not at all.
- **Titania's Song, and the self-reading definition** — the last of chapter
  eighty-two's four compounds, and the only one the coordination did not
  free. "Each noncreature artifact loses all abilities and becomes an artifact
  creature with power and toughness each equal to its mana value": the first
  two parts compose today, and the third is `DefinesPt` reading the SUBJECT's
  own mana value from inside a coordinated part. It is also finding 604's one
  reverse crossing — the sole static ability line in the corpus writing the
  inchoative "becomes" — so landing it benches that crossing too. Small: this
  card and Karn, Silver Golem's animation are most of the family.
- **The compound subject** — "Gogo and that creature each get +2/+0 and gain
  haste until end of turn", "this creature and those creatures get +1/+1 and
  have vigilance", "the chosen creatures get +X/+X and gain trample". Three
  lines, found while measuring the coordination's subject elision (finding
  611): the "and" is inside the NOUN PHRASE and the distributive "each" marks
  it. Not a coordination question at all — a noun question, and it belongs
  with the group vocabulary rather than here. Recorded so the next reader of
  finding 611's 1,432-of-1,433 does not mistake these for crossings.
- ~~**The arrival description**~~ — **CLOSED by chapter one hundred
  thirty-seven, and its central claim REFUTED rather than met.** The entry
  said the cell "needs not a verb slot but a SUBJECT: an anaphor to an object
  the previous clause moved, read at the destination zone". It needs no such
  thing and never did: `Sequentially` threads `effIntro`, a `Move` announces
  its patient through `moveIntro` retagged at the destination, and "it",
  "they" and "that creature" have read a battlefield placement in the
  preceding clause since Bond of Revival — while the cross-sentence anaphoric
  ASCRIPTION itself has been benched since Ashnod's Transmogrant, the same
  two sentences with a counter where this family has a placement (finding
  1116, three probes green before anything was designed). The family
  remeasured at 59 lines over 58 cards (finding 1117; the entry's 63 is a
  counting-method delta, its shape claim card-for-card right). What the
  family was actually blocked on was the ADDITION's payload, and chapter one
  hundred thirty-seven landed it: `BecomesAlso` now takes `SetsType`'s
  `TokenChars` bundle, so the colour, the P/T glyph and the with-clause
  ability are writable at both readings (finding 1121). Six whole cards
  benched, marquee Rise from the Grave. THE RESIDUES ARE THE THREE BELOW.
- **The face-down arrival rider** — 7 of the arrival ascription's 59 lines
  place their object FACE DOWN before ascribing to it (Yedora Grave Gardener,
  Magar of the Magic Strings, Tezzeret Cruel Machinist, Missy, Cybership,
  Death in Heaven, The Cyber-Controller), and `TokenRider` is
  `EntersTapped | EntersAttacking` and nothing else. This is the flip verb's
  entry above, counted from this side: its design note (index the rider
  vocabulary over `StatusVal`) is what would close both at once. Four of the
  seven want a `Cyberman` catalog word as well, and Yedora is the cheapest —
  one line, one rider, and a type line the catalog already spells.
- **The colour-only type addition** — one line, and the smallest thing this
  corner still wants: "Target permanent becomes blue in addition to its other
  colors" (Indigo Faerie). `BecomesAlso` now carries the colour but every one
  of its gates asks the TYPE LINE, so a bundle with a colour and no type word
  cannot pass `LineNonEmpty`. The other line of the shape is the CHOSEN
  colour `AddsChosenQuality` already spells (Painter's Servant), so the cell
  is one card wide; what it costs is a bundle-level "says something" gate
  where three line-level ones stand (finding 1119).
- **The type addition's remaining SURFACE demands** — two cells the payload
  widening deliberately did not settle, both recorded with their reasons in
  `BecomesAlso`'s docstring (finding 1123). The type-ORDERING half of
  `tokenCanonical` is not asked of this row and never has been, so an
  unordered addition line is writable where the setting's is not; whether
  that is a real free variation or an unmeasured hole is one sweep's work.
  And the "base power and toughness 4/3" RIDER phrase — 78 lines at the
  setting, 19 more inside addition clauses — is `HasBasePt`'s and still wants
  the coordination of two statics inside one sentence that no construction
  spells, the same ledger entry `SetsType` carries. The 19 are worth their
  own count: most of them coordinate the rider with the addition in one
  sentence ("Equipped creature has base power and toughness 5/5, has menace,
  and is a black Demon in addition to its other colors and types", Blade of
  the Oni), so closing the coordination closes them at both rows at once.
- **The COULD-TARGET multiplication** — [CR#707.10d], and chapter
  ninety-three's largest residue from the stack copy it fenced against. It is
  TWO constructions and not one: an AMOUNT that counts the players or objects
  a named spell "could target" (19 supported for-each copy counts —
  "copy that spell for each other Golem that spell could target"), and a
  DISTRIBUTION sentence that hands the copies out one target apiece (7 —
  "Each copy targets a different one of those Golems"). The second reads the
  copy mention chapter ninety-three minted, distributively ("each copy" is 8
  of the 261 mentions), so half the machinery is already standing. Precursor
  Golem, Radiate, Ink-Treader Nephilim, Mirrorwing Dragon, Zada Hedron
  Grinder, Agrus Kos and Radiant Performer are the whole family; Radiate is
  the cheapest — its first sentence is an ordinary choice and it has no other
  lines. The rule also supplies the failure case a gate would want: "if that
  player or object isn't a legal target for each instance of the word
  'target', a copy isn't created for that player or object".
- **The copy's SPECIFIED target** — [CR#707.10e], 6 supported sentences and
  the smallest thing this corner still wants: "The copy targets Ivy", "The
  copy targets that token", "The copy targets the chosen creature". A
  statement whose SUBJECT is the copy mention (landed, chapter ninety-three)
  and whose predicate is a targeting relation nothing in the grammar states —
  `AnyTarget` and the target quantities describe a phrase's own targeting,
  never one object targeting another after the fact. Beamsplitter Mage,
  Exterminator Magmarch, Feather Radiant Arbiter, Frontline Heroism, Ivy
  Gleeful Spellthief. [CR#707.10e]'s own worked example is Frontline Heroism.
  Small, and it would pay for itself only beside the could-target entry
  above, which writes the same relation in the plural.
- **The copy-a-CARD family** — [CR#707.12], 22 copy verbs over 20 cards plus
  45 "you may cast the copy" readbacks over 43, and the single largest thing
  chapter ninety-three measured and did not touch. "Exile the top card of
  your library. You may copy that card and cast the copy without paying its
  mana cost" is the shape: the copy is created IN THE ZONE THE CARD IS IN and
  then cast while another spell is resolving, which is a different rule from
  [CR#707.10]'s stack copy and a different verb. It SHARES chapter
  ninety-three's mention word ("the copy", "the copies") and nothing else, so
  the noun is already there and what is missing is the verb, the
  cast-the-copy permission and the without-paying rider (which `MayPlay`'s
  neighbourhood may already carry — measure before designing). Arcane Proxy,
  Elite Arcanist, Isochron Scepter, Wizard's Spellbook, Mnemonic Deluge,
  Spelltwine, Reversal of Fortune. [CR#707.12a] handles the plural's
  per-object choice; [CR#707.13] (Garth One-Eye) and [CR#707.14] (Magar of
  the Magic Strings) are one-card rules riding at the edge.
- ~~**The stack-verb PROHIBITION**~~ — **CLOSED by chapter one hundred
  twelve** (findings 875–882). `StaticEffect.ObjectCant` over the closed
  `ObjectAct` catalog (`Countered` 123 lines/113 cards, `Cast` 9/8, `Copied`
  3, `Played` 2 — 138 lines over 127 cards), with a per-verb subject demand
  (`ActSubject`: the three stack verbs want a stack-fitting subject
  [CR#701.6a], playing wants a LAND [CR#305.1]). It is `PlayerCant`'s
  SIBLING and not a `Deed` extension, and the grid is what decided it: "must
  be countered" and "can't be countered unless [payment]" are zero lines
  apiece, so two of `Deontic`'s three polarities have nothing to spell here,
  and no verb has a second voice. The card frame's [CR#113.6g] cell was
  opened row-sensitively to admit the 87-line self-reference on spell cards
  (`staticOnSpellCardOk`). Benched: MEDDLING MAGE (whole after five
  chapters), NEVERMORE, ABRUPT DECAY, DOVIN'S VETO, CONJURER'S BAN.
  Residues, each counted and each a different construction:
  - **The SPANLESS clause**, about nine lines and NOT this row's question:
    `absentOk DeedRestriction` is False, and the neighbouring `Prevention`
    refuses the identical cell — Banefire writes both halves in one sentence
    ("this spell can't be countered and the damage can't be prevented"). The
    round that answers it should re-measure `absentOk` across the whole
    prohibition neighbourhood at once. It costs: VEXING SHUSHER's second
    line ("{R/G}: Target spell can't be countered" — its first line
    elaborates today), the four mana-permission riders (Boseiju, Cavern of
    Souls, Delighted Halfling, Savage Summoning, which want the
    spend-only permission as well), and the "next spell you cast this turn"
    clauses (Insist, Overmaster, Mistrise Village), which additionally want
    a NEXT-object noun.
  - **The ability-class subject — CLOSED by chapter one hundred
    twenty-two**, which minted `Kind.Ability` and widened `ObjectCant` and
    `CostsToCast` over the kind. THIS ENTRY'S NUMBERS WERE ALL WRONG and
    finding 970 says where each came from: the "74" was the raw "can't be
    activated" grep including 12 detain REMINDER lines, and the "50 / 15"
    split is 43 / 19 (the 19 short by four because a literal "activated
    abilities" substring misses Kang the Conqueror's power-up class,
    Eidolon of Obstruction's loyalty one, and Kopala, Warden of Waves and
    Tithe Taker, which name no class at all). The whole family is 98 lines
    and this entry never counted the 28 cost-REDUCTION ones. Its
    parenthetical examples for the "15 the class" bucket were also drawn
    from the named-source bucket, which the other entry treated as
    separate. RESIDUE, counted: the triggered-ability subject (2 — The
    Master, Multiplied's "can't cause", Nowhere to Run's "don't trigger",
    both wanting a verb neither carrier has, the referent now built); the
    targeted ability (35 — "counter target activated ability", which is an
    ability ON THE STACK and therefore an object by [CR#109.1], wanting an
    object-sorted ability head this chapter deliberately did not mint); the
    floor rider (6 — "This effect can't reduce the mana in that cost to
    less than one mana", the second sentence on Training Grounds,
    Heartstone, Biomancer's Familiar, Forensic Gadgeteer, Convergence of
    Dominion, Power Artifact); the "that target …" restrictor on an ability
    class (7 — Bladegraft Aspirant, Cloud, Planet's Champion, Dwarven
    Mauler, Fervent Champion, Helitrooper, Kopala, Strong Back); and five
    KEYWORD CATALOG rows (Boast, Cycling, Ninjutsu, Exhaust, Power-up — 6
    lines between them, data rather than grammar, since the class word
    takes the `Keyword` parameter).
  - **STALE PROSE ABOUT THE PLURAL POSSESSOR, everywhere it still
    appears.** Chapter one hundred twenty-two wrote "activated abilities of
    creatures your opponents control" expecting a refusal and got a green
    build (finding 980): `possessorOk` has admitted `PlayerGroup
    YourOpponents` since it was widened for the relative clause, so the
    `PlayerGroup` docstring's "creatures your opponents control is 386
    lines and stays unwritable" and every queue line repeating it are out
    of date. LINVALA, KEEPER OF SILENCE and EIDOLON OF OBSTRUCTION landed
    whole on the correction. Whoever next reads a "plural possessor blocks
    this" note should probe it first. What the ability class still owes on
    this axis: KOPALA, WARDEN OF WAVES (the "that target a Merfolk you
    control" restrictor), TITHE TAKER (Afterlife, and a during-your-turn
    condition over a coordination of two cost statements at two sorts),
    SHARKEY, TYRANT OF THE SHIRE and DRANA AND LINVALA and KARN, THE GREAT
    CREATOR (each blocked on an ability-grant or loyalty line, not on this
    one), and CARTH THE LION ("planeswalkers' loyalty abilities you
    activate cost an additional [+1] to activate" — an ADDITIONAL cost
    with a loyalty-symbol payload, which `CostsToCast` excludes by
    measurement).
  - **The `Phyrexian` name-straddle**, 1 bench line: the creature type
    cannot enter the subtype catalog because the word is already this
    module's Phyrexian MANA SYMBOL constructor ([CR#107.4f]), so PHYREXIAN
    REVOKER's type line is benched as "Horror" alone. It costs nothing
    grammatical and is recorded so the elision is not read as a gap
    (finding 978). Any fix is a naming decision about one of the two
    catalogs, not a construction.
  - **GADDOCK TEEG's second line**, 1 line: "Noncreature spells with {X} in
    their mana costs can't be cast" wants a predicate over a mana cost's
    SYMBOLS. Its first line elaborates today. AETHER STORM's second line
    wants an any-player activation and the regeneration rider. Chapter one
    hundred ten named both cards as free payers for this row and both were
    wrong (finding 881).
  - **SANCTUM PRELATE**, 1 line: has this row now and still wants an
    `Amount` that reads a chosen NUMBER ("mana value equal to the chosen
    number") — the number sort's own reader, which `chosenQualityReadOk`
    refuses at the quality readers and which no `Amount` row spells either.
    ONE OF ITS TWO BLOCKERS IS GONE: chapter one hundred twenty-four landed
    the equality this line's comparison needs, so the missing READ is all
    that is left. The chosen number is wanted by 8 supported lines across
    four unrelated constructions (Sanctum Prelate and Talion, the Kindly
    Lord at this description frame; Expel the Interlopers at a ranged one;
    Squall, Gunblade Duelist in a condition; Liquid Fire in a letter
    definition; Haktos the Unscarred, Mindblaze and Scrying Glass
    elsewhere), which is the population a round should take, not this card
    alone.
  - **The conditional carriers**, 4 lines (Banefire, Demonfire, Dragonlord's
    Prerogative, Exquisite Firecraft): "If X is 5 or more, this spell can't
    be countered" is this row under `Conditionally`, and each of the four
    trails a second conjunct or an ability word of its own.

- **The STANDING PROHIBITION — the player half CLOSED by chapter one
  hundred three**, and the entry kept for the rest, which is most of it.
  The whole population is 2,683 supported sentences over 2,528 cards
  (finding 787's table); attacking and blocking (1,818), damage prevention
  (31), the untap cap (9) and the game outcomes (10, `OutcomeGate`, built
  years of chapters before the brief listed it as a gap) were already
  written, and `StaticEffect.PlayerCant` over `PlayerAct` adds 41 more —
  gain life 21, play lands 7, cast spells 7, search libraries 4, draw cards
  2, all five bare, one row, no new `StaticKind` and no span-table cell.
  What is left, each measured and each a different construction:
  - ~~**Being REGENERATED**~~ — **CLOSED by chapter one hundred
    twenty-five**, together with the whole regeneration family (435
    sentences, 2.8x what this entry's 156 names — the 156 is the
    prohibition alone and reproduces exactly, three passes running). The
    entry's SHAPE call was right and its numbers were half right: the
    anaphoric subject is 127, not 123, and the no-span count is 138, which
    is also the ATTACHED count — the two axes coincide, and the corpus
    writes neither a spanned rider nor a spanless standing clause even
    once. Its reading of [CR#701.19c] was the one thing overturned
    (finding 1008): the rule is why the MECHANISM is not a deed denial and
    is not a reason the WORD cannot sit in a word catalog, so
    `ObjectAct` took a sixth row read by two carriers —
    `StaticEffect.ObjectCant` for the spanned 18 and the new
    `Effect.CantBe` for the 138. RESIDUE, counted: the **demonstrative
    subject** (4 lines — Nekrataal, Scorching Lava and kin), blocked
    because `That` cannot read a destroyed target at all, probed bare and
    refused the same way without this row; the **participle read**
    ("Creatures destroyed this way can't be regenerated", 10 lines),
    blocked because the verb stamp lives in the post-context while the
    rider's subject is typed in the pre; and PLAGUE SPORES, the CR's own
    worked example, which needs two separate targets under one plural
    anaphor.
  - **THE ANAPHOR'S VERB PROVENANCE**, 4 lines and a named
    overgeneration (finding 1010): the grammar cannot see WHICH verb
    stamped the mention a bare `It` reads, so chapter one hundred
    twenty-five's rider is spellable after a DAMAGE clause although the
    corpus spans those instead ("Engulfing Flames deals 1 damage to target
    creature. It can't be regenerated this turn."; Rage of Purphoros, and
    Carbonize and Disintegrate with their own conditional). `TheVerbed`
    already carries `stampedBy` and `It` carries nothing; a round that
    gives the bare anaphor a provenance query would close this and would
    also unblock the participle-read cell above. Measure `It`'s other
    readers before widening — it is the most-read noun in the grammar.
  - **The QUALIFIED cast lines** — 74 sentences, and the reason
    `PlayerAct.CastsSpells` spells only the bare form. Five qualifier
    families, each its own subsystem: a spell TYPE ("noncreature spells",
    "creature spells", "spells of the chosen type"), a NAME ("with the
    chosen name", "with the same name as the exiled card"), a ZONE ("from
    graveyards", "from anywhere other than their hands"), a TIMING
    ("during combat", "during your turn") and a COUNT CAP ("more than one
    spell each turn", 9 lines, which is `CantUntapMoreThan`'s shape at a
    second verb). The count cap is the cheapest of the five and the only
    one with a row already shaped like it. Chapter one hundred twelve landed
    the OBJECT-subject prohibition beside this one (`ObjectCant`), which
    settles what these 74 are not: a player prohibition with a qualified
    complement is a different row from an object prohibition with a
    described subject, and the corpus writes both about the same sentences
    ("Your opponents can't cast spells with the chosen name" against "Spells
    with the chosen name can't be cast", one card apart). The object half is
    paid; this half still wants an act that carries a description. **The NAME qualifier's
    DESCRIPTION is landed** (chapter one hundred ten, `Named ChosenName`),
    so its five sentences — Council of the Absolute, Gideon's Intervention,
    Alhammarret, Failure // Comply, Academic Probation — now want only what
    all 74 want: a prohibition whose act carries an object description
    instead of a bare `PlayerAct` value. For those five the qualifier is no
    longer the blocker; the catalog row's shape is.
  - **The TARGETING prohibition** — 39 real sentences, and the count is the
    correction: a naive sweep returns 114 and 75 of those are the reminder
    text printed under hexproof and shroud, which is not a card's own line.
    It is this row's OBJECT-subject sibling ("this creature can't be the
    target of spells or abilities your opponents control") and it carries a
    complement the player prohibitions never do — WHOSE spells and
    abilities — so it is a round of its own and not a value in this
    catalog.
  - **The MANA restriction** (48, "can't be spent to cast spells with
    mana value 3 or greater"), **the ACTIVATION prohibition** (35, "can't
    be activated" / "can't activate abilities"), **the amount FLOOR** (18,
    "X can't be 0", "this effect can't reduce the mana in that cost to
    less than one mana") and the long tail of sacrifice (6), destruction
    (5), drawing an object's cards (5), copying (4) and exile (1). Each is
    its own question; the floor is the odd one, since it prohibits nothing
    an agent does and belongs with the amount vocabulary.
  - **Inside the landed family, measured and not minted**: "can't lose
    life" (2, [CR#119.8]'s sentence, and both are CONJUNCTS of a larger
    coordination rather than sentences of their own — Courageous Resolve,
    Everybody Lives!), and "can't pay life" (3, all three qualified by a
    purpose clause, "to cast spells or to activate abilities"). Either
    becomes a catalog value the day a witness writes it alone.
  - **The GAME-SPANNING duration** — "for the rest of the game", 55 lines
    of which 41 are the reminder text of Ascend (26), Storied (9) and Epic
    (5). The 14 that remain are the no-maximum-hand-size clause (7), the
    goad rider (4), Cyclopean Tomb's delayed trigger, and this family's two
    (Screaming Nemesis, Stigma Lasher). ALL FOURTEEN ARE RESOLUTION CLAUSES
    and no printed static line writes the phrase, [CR#611.3b] already
    making a line last as long as its source — so finding 275's covariance
    holds perfectly and the phrase is a SPELLING, not a `Duration` row.
    Acting on that means flipping `absentOk` for whichever kind, and
    chapter one hundred three declined because neither carrier can be
    benched: Screaming Nemesis is blocked on the union-narrowing container
    below and Stigma Lasher on the active damage event (chapter one hundred
    two's 43-line fence). Whichever round takes the hand-size or goad
    riders should settle it for all four constructions at once.
  - **The union-narrowing CONTAINER** — 1 sentence and the last thing
    between Screaming Nemesis and a whole card, recorded because two
    chapters have now walked up to it. "If a player is dealt damage this
    way, they can't gain life" is NOT chapter ninety-six's unchecked
    linearization side condition: that container merely named an anaphor's
    antecedent, where this one SELECTS — the damage went to "any other
    target", a phrase that spans both kinds (`nounSpansPlayers`), and the
    container picks out the player case and binds it for `They` to read.
    A container that narrows a union mention to one of its kinds has no row
    here. Count it with the other conditional containers before scheduling.
- **The ABILITY on the stack** — 60 supported sentences over two verbs, and
  the noun `CounterSpell` has ledgered since chapter twenty-eight: "counter
  target activated or triggered ability" is 29 and "copy target activated or
  triggered ability you control" is 31. An ability on the stack is an object
  ([CR#109.1], [CR#113.7a]) that this vocabulary has no head word for — it is
  not a card, so `CardW` does not reach it, and it is not a spell, so the
  stack's carrier noun [CR#112.1] does not either. Both verbs' complement
  slots are already the ordinary stack noun, so nothing about either row
  changes when the word lands; this is a NOUN round and not a verb one. The
  descriptions the corpus writes are narrow ("activated or triggered",
  "activated", "triggered", plus source restrictions — "from an artifact
  source", "from a creature source"), so measure the description vocabulary
  before minting anything.
- **The STACK copy — CLOSED** by chapter ninety-three (findings 693–701), which
  landed the verb (`CopyStack`, 294 sentences), the mention (`Origin.CopyOrigin`
  + `NounWord.CopyW`, 261 reads), the retarget verb (`ChooseNewTargets`, 202
  sentences), `CopyExcept.ExceptColor` and the `Subtype.Bird` row, with Twincast,
  Fork, Meletis Charlatan and Tawnos the Toymaker whole and Echo Mage's level-4
  ability. Its measured residues are the five entries above plus these small
  ones, recorded rather than queued: "the copy they control" (2, a possessive on
  the mention — Curse of Echoes, Tempt with Mayhem); "copy that spell an
  additional time" (2, an increment on a count some other clause set — Howl of
  the Horde, Tempt with Mayhem); the CONDITIONAL exception (1, Double Major's
  "except it isn't legendary if the spell is legendary"); the starting-loyalty
  readback (1, Ob Nixilis the Adversary — finding 639's unmodeled box a second
  time); and the participial read, which is a measured ZERO ("the copied spell",
  "copied this way" are unwritten, and copying mints no `VerbName` stamp).
  TWO CARDS ARE BLOCKED ON ONE MISSING RIDER and it is not a copy question:
  Iron Man Bleeding Edge and Donal Herald of Wings both end "Do this only once
  each turn", which [CR#603.2h] defines as a restriction on the ACTION and which
  `Triggered`'s `limit` slot explicitly is not. That rider is 32 lines by the
  slot's own comment and is the cheapest way to buy those two cards.
- **ENTERS as a copy** — 64 supported sentences over 63 cards, and chapter
  ninety-two fenced it on a PROBE rather than on a missing carrier. The
  composition `Intercepts (Enters …)` over a `Continuously (BecomesCopy …)`
  compiles today (probed, verified, removed), and it spells the sentence
  [CR#707.5] exists to distinguish: an object that enters "as a copy" "becomes
  a copy AS it enters. It doesn't enter the battlefield, and then become a
  copy". The corpus writes "enter as a copy" 64 times and the
  would-enter-instead form zero times. So the ask is a copy-on-entry row, and
  the rule hands it two riders the composition could not carry either —
  [CR#707.5]'s "enters with"/"as … enters" abilities of the copied text take
  effect, and [CR#707.6] gives the copy's controller the as-enters choices
  fresh. THIS ENTRY OWES `ExceptPt` ITS WITNESS: that row landed in chapter
  ninety-two with finding 481's bar declared unmet, its 10 bare-P/T lines all
  multi-clause or behind this fence, and the cheapest payer is Quicksilver
  Gargantuan — [CR#707.9d]'s own worked example. Clone, Sculpting Steel and
  Sakashima's Student are the bare, the artifact and the type-adding
  witnesses. Chapter ninety-three CHECKED the stack copy for a payer and found
  none — Donal, Herald of Wings writes its number inside a characteristics
  bundle — so the debt is still this entry's.
- **The exception kinds chapter ninety-two refused**, each at its count and
  none of them scheduled alone. The NAME exception is 17 sentences ("except
  his name is Absorbing Man", "except its name is Mishra") and wants a
  name-setting the grammar has nowhere; it is the same missing piece
  chapter eighty-nine's pairwise-distinctness entry and Garth One-Eye's
  chosen-name clause want, so measure the three together. CHAPTER ONE
  HUNDRED FIVE CHECKED WHETHER ITS MACHINERY REACHED THE NAME CELL AND IT
  DOES NOT, in either direction: `chosenQualityReadOk CardName = False`
  still holds and the new `OfYourChoice` row inherits the refusal on its
  own measurement (`badYourChoiceCardName`, a shared-P pair with
  `badChosenCardNameRead`) — "of the chosen name" and "of the card name of
  your choice" are zero supported lines apiece, because a chosen NAME is
  matched by name and never read as a quality of a description. What the
  three entries are actually waiting on is now counted: "with the chosen
  name" is 72 lines over 65 cards (Meddling Mage, Phyrexian Revoker,
  Council of the Absolute, Disruptor Flute), and it is a MATCH surface with
  no row, not a quality read that needs unlocking. Nothing here was bought;
  the count is the delta. CHAPTER ONE HUNDRED SEVEN PAID THE OTHER HALF:
  the container is indexed and `EntersChoice` takes a bare `QualitySort`
  (finding 830), so the CHOOSER line of all 65 cards elaborates today and
  what remains is the match predicate and its gate alone — one row, not an
  architecture. CHAPTER ONE HUNDRED TEN LANDED THAT ROW (`NameSource`, 70
  lines over 63 cards re-derived) AND THE NAME EXCEPTION STILL GETS
  NOTHING, which is now pinned rather than merely absent: `badAscribedName`
  refuses a name in the ascription rows' payload, because a name is MATCHED
  and never ascribed — "are/becomes/is the chosen name" is zero lines
  apiece. What these 16 sentences want (the count re-derived from chapter
  ninety-two's 17) is a name SETTING inside a copy clause, which is a rider
  on `CopyExcept` and shares nothing with the match but the word "name".
  Measure it with the copy exception's other kinds, not with the chosen
  name. GARTH ONE-EYE, the third of the three this entry told successors to
  measure together, gets HALF: "the card with the chosen name" is writable
  today, so what is left of that line is a chooser narrowed to a printed
  LIST ("a card name that hasn't been chosen from among Disenchant,
  Braingeyser, Terror, Shivan Dragon, Regrowth, and Black Lotus" — the
  domain slot again, at its most extreme cell) and the copy-a-CARD verb
  above. The three were never one purchase, and this is the chapter that
  says which of them the name machinery could reach. The CHARACTERISTICS
  BUNDLE is 36 sentences writing a number inside a whole bundle ("except it's
  a 4/4 black Zombie", the Scarab God family). CHAPTER NINETY-THREE CORRECTS
  THIS ENTRY'S DIAGNOSIS: the missing cell is NOT the colour. `ExceptColor`
  landed there (one bare sentence, Fork, [CR#707.10]'s own worked example) and
  the bundle is still refused, because the bundle is ONE NOUN PHRASE setting
  three characteristics at once and not three exceptions joined — writing it
  as `[ExceptPt, ExceptColor, ExceptTypes]` would spell "except it's 4/4, it's
  black and it's a Zombie in addition to its other types", which is not the
  sentence. What it wants is a bundle-shaped exception carrying a
  `TokenChars`-like value; measure that against `SetsType`'s payload, which is
  the same phrase in the statement position. The bundle's own tail divides and
  the split should be re-measured before design: of the 27 tails matching a
  bare "except it's a N/M …", 16 end "in addition to its other types" and 11
  do not, so the bundle is not uniformly a setting. The remaining
  four are small and recorded rather than queued: the enters-with exception (5,
  [CR#707.9e]'s "additional effect rather than a modification"), the type
  SETTING exception (4, "and it loses all other card types"), starting loyalty
  (1) and RETAIN (1, [CR#707.9c], Vesuvan Doppelganger).
- **The other-marking anchored to the SOURCE — CLOSED by chapter one hundred
  one, and it was never open.** `OtherThan This` has elaborated since chapter
  twenty-two, which minted the anchored complement with the source as its
  intended anchor (finding 113); chapter ninety-two probed `Other` — the
  TARGETING complement, the other row of the same word — got the refusal that
  row is supposed to give, and recorded the family as unwritable. Unstable
  Shapeshifter is benched whole and `Shapeshifter` is re-minted. The
  inventory the entry asked for is finding 770: 3,143 occurrences over 2,955
  cards, of which 1,897 over 1,832 are this cell. What the sweep left behind,
  each measured and each a different question:
  - **The anaphoric "another of those"** — 4 occurrences over 4 cards
    (Clockspinning's "put another of those counters on it"), a PARTITIVE over
    a group mention rather than a complement over a description. No row, and
    under finding 85's bar on its own; it belongs with `SomeOf`'s family if
    anything ever schedules that.
  - **"Another card named [X]"** — 12 over 12, the Grandeur cycle's discard
    cost. The complement composes; what these want is the cost shape around
    it, so they are a cost-line question and not an otherness one.
  - **"The other [X]"** — 104 over 102, finding 111's subset complement,
    still needing a cardinality no binding records. Unchanged and restated
    here only because a sweep of the word will keep finding it.
  - **The source-anchored CLASS WORD's witnesses — PAID by chapter one
    hundred two, as far as the corpus allows one to be paid.** The trigger
    event's amount readback is built (`eventAfter` announces
    `outcomeB DamageDealt` at both damage events), and
    `screamingNemesisTrigger` is benched — a FRAGMENT, because Screaming
    Nemesis's second sentence wants three unbuilt things (a "dealt damage
    this way" container over the trigger's own event, a standing
    prohibition on gaining life, a rest-of-the-game duration `Duration` has
    no row for). The entry's list of five was measured wider and each
    carrier's real blocker named: of the eleven supported cards writing
    this reading, the other ten stop on an Adventure face, two Sieges, a
    reflexive trigger, [CR#615.13]'s prevention trigger, a power-up cost or
    a granted quoted ability — none of them on otherness and none on the
    magnitude any more. Pain for All's SECOND line ("Whenever enchanted
    creature is dealt damage, it deals that much damage to each opponent")
    is `spitefulShadows`' term with a distributive recipient, both halves
    of which are now benched; its FIRST line is what the card still waits
    on.
  - **The distributive MILL** — found while pricing Altar of the Brood as a
    witness ("Whenever another permanent you control enters, each opponent
    mills a card"). Every benched mill has a singular subject whose library
    `They` can read; a distributive subject has none, so the clause cannot be
    written. Not measured beyond the one card; a successor should count the
    family before scheduling it.
- ~~**The chosen basic land type**~~ — CLOSED by chapter one hundred four.
  The USER-SETTLED DESIGN LEAN was HONORED unchanged: `BasicLandType :
  Subtype -> Type` with `PlainsBasic | IslandBasic | SwampBasic |
  MountainBasic | ForestBasic` is in `Words.idr` beside `subtypeType`, and
  the naming reason held on re-reading ([CR#205.4c] keeps Basic-the-supertype
  independent of the subtype, Wastes proving the axes come apart). Two
  departures from the entry, both measurements: the family's arity is
  [CR#305.7]'s "ONE OR MORE", so a list witness `BasicLandTypes` rides beside
  it (Lush Growth writes three types in one line and is benched); and
  `KeywordCounterEligible`'s Bool table was NOT copied — that idiom exists
  because [CR#122.1b]'s list is open at an open catalog's end, where
  [CR#305.6] closes this set from outside the catalog, so five constructors
  say what a table would only ask.
  THE BUNDLE HALF NEEDED NOTHING (finding 798): this entry's claim that it
  waits on a supertype predicate was false — `HasSupertype` landed with
  Winter Moon's backfill, and seven probes of the whole half (Blood Moon,
  Conversion, Celestial Dawn, Kavu Recluse, Contaminated Ground, Lush Growth,
  Swampbenders) elaborated first attempt on `SetsType`/`BecomesAlso`. THE
  CHOICE HALF is 12 lines, not 19 (finding 797), and got one row,
  `SetsChosenBasicType`, with no domain slot and no chooser slot (both
  measured zeros, finding 802). Residues:
  - **The chosen type ADDED** — "becomes the basic land type of your choice
    in addition to its other types" (Navigator's Compass, 1 line).
    [CR#305.7]'s last sentence at the chosen subtype. `BecomesAlso`'s payload
    is a bare `TypeLine`, bs-free like the token bundle, so the choice cannot
    ride it; one line does not buy a second row. Wait for a second carrier.
  - ~~**The choose-and-read family**~~ — TAKEN UP by chapter one hundred
    five and the "fifth `QualitySort` row" guess REFUTED there: `CreatureType`
    has been a `QualitySort` since the quality mentions landed and Kindred
    Dominance benched the linked pair the whole time. What that round found
    instead was the phrase that chooses in its OWN noun phrase
    (`OfYourChoice`, 15 lines) and, behind the rest of the family, the
    architectural wall at finding 815 — WHICH CHAPTER ONE HUNDRED SEVEN
    TOOK DOWN: the container is indexed and the as-enters chooser is a row.
    See the standing-choice entry below for what is left.
  - ~~**The whole domain as a phrase**~~ — **CLOSED by chapter one hundred
    eighteen**, and DRYAD OF THE ILYSIAN GROVE IS WHOLE: the retraction of
    finding 930a is reversed, the term now quantifying where the card
    quantifies. `TypeSpace` is the third payload shape at the ascription
    position (the enumerated line, the chosen quality, the space) and a
    sibling row for `SetsChosenBasicType`'s stated reason — no `Subtype`
    value denotes a space. It landed the GRANT frame with the ascription
    (finding 935: the verb and the quantifier word are both the frame's), so
    22 of the family's 33 sentences write. What is left of the quantifier:
    - **The LOSS cell, 6 sentences** — "loses all creature types" (Curse of
      Conformity, Ego Erasure, Nameless Inversion, Amoeboid Changeling) and
      "loses all land types and abilities" (Alpine Moon, Lithoform Blight,
      Ultima, Origin of Oblivion). The negative pole of `AddsEveryType`, and
      the three land-type lines COORDINATE the type loss with an ability loss
      inside one phrase, which `LosesAllAbilities` writes as its own
      statement today — so the round that takes this should decide whether
      the coordination is one row or two before minting either.
    - **The becomes-a-creature-WITH cell, 3** — "This land becomes a 2/2
      creature with all creature types until end of turn" (Mutavault,
      Faceless Haven, Soulstone Sanctuary). The quantifier rides `SetsType`'s
      `TokenChars` there rather than the ascription position, so it is a
      payload question inside the token characteristics and not this row's.
    - **"Is all colors", 1** (Leyline of the Guildpact) — a COLOUR payload at
      the same position, quantifying a different space with its own carrier.
      Measure against the colour-setting family before scheduling; that card
      additionally wants the opening-hand permission (chapter one hundred
      fourteen's residue).
    - **The LAND space's other carriers** — Omo, Queen of Vesuva writes the
      only "is every land type" ascription and wants an everything counter;
      the space's row is minted and witnessed by nothing yet.
    - One card away: ENERGYBENDING (the basic-land grant cell) needs only the
      Lesson subtype; MASKWOOD NEXUS needs "the same is true for creature
      spells you control and creature cards you own that aren't on the
      battlefield", an `AlsoOffBattlefield` whose subject is TWO FURTHER
      NOUNS rather than the same one restated.
    Still open from the enumerated side: the choose-two lines ("As this
    enchantment enters, choose two basic land types"; "Choose two basic land
    types. Create the appropriate Shockland token").
  - **The named "becomes" settings** (17 lines: Kavu Recluse, Streambed
    Aquitects, Nightcreep, Thelonite Monk, Cyclopean Giant) — WRITABLE TODAY
    and deliberately unbenched, since Blood Moon's copular sentence and these
    inchoative ones are the same term under a different frame and both frames
    are already on the bench. Free witnesses for any future round that wants
    a cheap land line.
  - **A NON-BASIC land type** — the `Subtype` catalog holds only the five,
    so `basicLandLine`'s gate currently refuses nothing the card-type check
    would not (finding 801, recorded as a refusal by vocabulary and not
    pinned). The first card to bench Desert, Gate, Cave, Locus or Urza's
    turns that gate live and earns the pin.
  - **Song of the Dryads** ("Enchanted permanent is a colorless Forest
    land", 1 line) — writable on `SetsType` and unbenched. It is the reason
    NO gate was added to `SetsType` against subtype-only basic land lines:
    a basic land type beside a color and a card type is a sentence no
    basic-only payload could hold, so the general row must keep the case.
    The two rows do not overlap: `SetsChosenBasicType` carries no subtype at
    all, so the named sentence has exactly one term and the chosen one has
    exactly one.
- **The standing choice — CLOSED by chapter one hundred seven**, and the
  entry is kept for the residues it left. The container is indexed:
  `AbilityAt : Bindings -> Type` with `Text.AbilitySeq` threading `abIntro`,
  `Card.text : AbilitySeq []`, `Ability = AbilityAt []` keeping every call
  site edit-free, and ONE grammar row, `StaticEffect.EntersChoice`
  ([CR#603.6d] files the surface as a static ability; [CR#614.12a] times the
  choice; [CR#607.2d] links the reader). Seven whole cards, Etchings of the
  Chosen the marquee — its chooser read by a static pump AND by an
  activation cost, which is why `Activated`'s cost had to index. Cold gate
  17/17 with every existing term unchanged.
  Chapter one hundred six's open question (finding 822) was answered by
  MEASUREMENT before the surgery: three cards in the whole corpus write two
  same-sort choosers and none of them then writes the plain read, because
  English marks a re-choice with a different word ("the LAST chosen color",
  Chromatic Armor) that [CR#607.2d] names in the same sentence. No append,
  no re-reading of `countQuality` (finding 826, pinned as
  `badTwoChoosersOneSortRead`).
  What this did NOT buy, each checked rather than assumed:
  - ~~**The keyword PARAMETER**~~ — **CLOSED by chapter one hundred eight**
    (findings 837–838). `KeywordParam` is indexed and exactly one of its four
    rows uses the index; the other three record their zeros (ward cost,
    enchant restriction, renown count: 0 chosen reads apiece). The blocker
    moved on measurement: all 21 supported lines are GRANTS and none is a
    bare printed keyword line, so `Gains`' payload indexed with the parameter
    ([CR#607.1a] makes a granted ability count as printed text). VOICE OF ALL
    and WARD SLIVER are benched whole. Residues: the seven Aura carriers
    (Cho-Manno's Blessing, Flickering Ward, Pentarch Ward, Ward of Lights,
    Benevolent Blessing, Floating Shield) each trail "This effect doesn't
    remove this Aura", a rider with no row — the cheapest six-card win in the
    area, and chapter one hundred nine measured it as its OWN construction
    rather than the extension's: 15 supported lines in four spellings, all
    of them same-line trailers like the extension but pointing the opposite
    way, carving an object OUT of a statement's scope where the extension
    widens it (12 write "This effect doesn't remove this Aura", the other
    three name Auras and Equipment already attached). Its own entry's worth
    of work; protection from a PLAYER (True-Name Nemesis, Guardian Archon) wants
    a `Predicate bs Player` payload, which `ParamQuality` does not take;
    protection from a CARD TYPE (Serra's Emissary) and from a CARD NAME
    (Runed Halo) want `QualitySort` values that do not exist. And ONE emblem
    reads a chosen value through `GetsEmblem`'s bs-free list (Oko, Lorwyn
    Liege) — one card, one index, unscheduled.
  - ~~**The type-line ascription**~~ — **CLOSED by chapter one hundred
    eight** (findings 839–842), and the diagnosis three chapters carried was
    WRONG: it was never waiting for a context. `TypeLine.subs` holds
    `Subtype` WORDS, and a chosen value is not a word at any index, so
    indexing the record — which is shared with the card frame, where a
    printed type line genuinely has no discourse — would not have helped.
    The payload is the quality-denoting PREDICATE the grammar already had
    (`OfChosen q`, `OfYourChoice q`), gated by `QualityRead`. TWO rows,
    because [CR#205.1a] and [CR#205.1b] tell setting from adding exactly as
    they do for `SetsType`/`BecomesAlso`; ONE row per operation, because the
    payload carries the source axis. XENOGRAFT, ADAPTIVE AUTOMATON (both
    readers of its choice) and MISTFORM DREAMER are benched whole. **The
    bs-free `TypeLine` is untouched and no longer blocks anything named
    here.** Residues, each checked not assumed:
    - ~~**ARCANE ADAPTATION, CONSPIRACY, LEYLINE OF TRANSFORMATION and
      RUKARUMEL**~~ — **CLOSED by chapter one hundred nine** (findings
      846–852). `AlsoOffBattlefield` is a transparent WRAPPER, which nine
      carriers decided: none writes the sentence on a line of its own and
      all nine write it as the second sentence of the line it extends, so
      the card-level telescope was the wrong mechanism. It carries NO scope
      slot — the extension's noun is the wrapped statement's own subject
      transposed to the other zones, and Celestial Dawn against Encroaching
      Mycosynth is the crossing pair that proves the leftover qualifiers are
      free variation (finding 849). ARCANE ADAPTATION (whole at last, named
      as blocked in four consecutive chapters), CONSPIRACY and ENCROACHING
      MYCOSYNTH are benched. Residues: LEYLINE OF TRANSFORMATION wants the
      Leyline play permission ("If this card is in your opening hand, you
      may begin the game with it on the battlefield") and RUKARUMEL a
      COORDINATED subject ("Slivers you control and nontoken creatures you
      control"), one construction each; BIOTRANSFERENCE, ROSHAN and
      CELESTIAL DAWN are blocked on unrelated second lines; MASKWOOD NEXUS
      wants "every creature type" and CELESTIAL DAWN the literal colour
      setting, both of which are ascriptions the extension would accept the
      day their rows exist.
    - **ASHES OF THE FALLEN** ascribes to creature cards in a GRAVEYARD,
      where both rows demand a battlefield subject (`BecomesAlso`'s demand,
      kept rather than carved out). One line.
    - **The chosen BASIC LAND type**, 5 lines (Realmwright, Thran Portal,
      Convincing Mirage, Phantasmal Terrain, Multiversal Passage). Not
      reachable and not absorbable: `QualitySort` has no land-type value and
      chapter one hundred four's `SetsChosenBasicType` covers only the
      your-choice SETTING case at that sort, so the two rows are disjoint in
      fact (finding 842). The day a land-type `QualitySort` value is minted,
      these fall out of the rows landed here.
    - **The colour halves are already spelled** — Painter's Servant and
      Shifting Sky elaborate on these rows (finding 841), 20 lines. What is
      still unspelled is the LITERAL colour change ("becomes blue until end
      of turn", 42 lines), which is the separate layer-5 entry below.
  - ~~**The match surface**~~ — **CLOSED by chapter one hundred ten**
    (findings 853–864). `Predicate.Named` now takes a `NameSource bs`
    (`PrintedName` / `ChosenName`), one row over the source with the
    binding demand on the payload and the spelling derived from it —
    finding 275's positive direction with zero crossings measured in both
    directions, and finding 246's rule on eight-of-nine wildcard readers.
    The population is **70 supported lines over 63 cards** in three frames
    (the count corrects "72 over 65"), and **43 of the 70 make the choice
    and read it in the SAME line**, so the container was necessary for a
    MINORITY of this family and not for it (finding 854). Benched:
    DECLARATION OF NAUGHT (the whole cross-ability card), CURSED SCROLL,
    MAGUS OF THE SCROLL. Residues, each counted:
    - ~~**The NARROWED chooser**~~ — **CLOSED by chapter one hundred
      eleven** (findings 865–870). `ChoiceDomain : QualitySort -> Type` with
      the domain slot on both `QualityNoun` (20 effect-level lines) and
      `EntersChoice` (10 as-enters); the narrowing turned out to be
      cross-sort, not a card-name gadget (46 lines over four sorts).
      SILVERQUILL SILENCER is the card-name cell's only whole witness and
      it is benched. The count is corrected here: the card-name chooser is
      66 lines, **26 prenominal narrowings + 4 postnominal exceptions**, not
      28 (finding 868 — the earlier sweep dropped the narrowings containing
      a comma). What the narrowing did NOT buy, stated exactly: **MEDDLING
      MAGE IS STILL NOT WHOLE** (finding 869). Its chooser line elaborates
      today; its second line is "Spells with the chosen name can't be cast",
      the object-subject stack-verb prohibition, and that is its ONLY
      remaining blocker — as it is for NEVERMORE and CONJURER'S BAN, and for
      SANCTUM PRELATE beside the number match. **PAID BY CHAPTER ONE HUNDRED
      TWELVE**: `ObjectCant` landed and MEDDLING MAGE, NEVERMORE and
      CONJURER'S BAN are benched whole. VOIDSTONE GARGOYLE was listed here in
      error and is corrected there (finding 881): its fourth line is an
      ability-class subject, so it waits with Phyrexian Revoker and not with
      these.
    - **The postnominal exception**, 4 lines ("a card name other than a
      basic land card name" — Booby Trap, Desperate Research, Necromentia,
      Null Chamber). Its content is `NameOfCard (Not (And [HasSupertype
      Basic, HasType Land]))` and what refuses it is `negatable (And _) =
      False`, a standing refusal with nothing to do with names or choosers.
      Garth One-Eye's "a card name that hasn't been chosen from among [six
      printed names]" is the same slot's most extreme cell, one line, and
      wants a literal-name list plus a not-yet-chosen memory.
    - **The Memoricide cluster** (Ancient Vendetta, Unmoored Ego, The Stone
      Brain, Cranial Extraction, Memoricide, Slaughter Games, Lost Legacy,
      Stain the Mind, Necromentia, Dispossess, Infinite Obliteration) —
      blocked on a SEARCH ACROSS THREE ZONES in one clause ("Search target
      opponent's graveyard, hand, and library for …"), which is nothing to
      do with names; `Search` takes one `ZoneExpr`. Eleven cards behind one
      construction, and most of them behind the narrowed chooser as well.
    - **The plural read**, **5 cards, not 1 line** (finding 1028 —
      ENLARGED, and it splits by CHOOSER shape rather than by surface).
      One plural surface over three different chooser shapes: **two
      choosers made by one distributive clause** (Null Chamber, "you and
      an opponent each choose a card name" … "the chosen names"); **one
      chooser making a PLURAL choice** (Seal of the Guildpact, Tablet of
      the Guilds — "choose two colors" … "for each of the chosen colors it
      is"); and **three choosers written as three clauses with a union
      read** (Paliano, the High City and Regicide — "The player to your
      right chooses a color, you choose another color, then the player to
      your left chooses a third color" … "one or more of the colors chosen
      as you drafted cards named Regicide", which also wants draft-time
      choosers and a card-name-scoped memory). Measure which shape a round
      takes before assuming one construction: the recency family, which
      shares the two-chooser configuration, writes a SINGULAR marked read
      instead and is now landed. The reason `badNameMatchWrongSort`'s
      sibling refusal at two bindings is not a pin about English still
      stands.
    - **Runed Halo**, 1 line: its phrase `ParamQuality (Named ChosenName)`
      elaborates today (probed), so what blocks the card is that its
      subject is a PLAYER and `Gains` takes an object — True-Name Nemesis's
      blocker at the other end of the same keyword.
  - ~~**The RECENCY reader**~~ — **CLOSED by chapter one hundred
    twenty-seven**, which landed `Predicate.OfLastChosenColor`. The entry's
    count was RIGHT (14 cards / 12 supported reproduces exactly, by two
    independent patterns) and its design was right in both halves the brief
    leaned on: the representation needed nothing (`Bindings` is
    nearest-first, the latest choice is the head) and the demand really is
    "at least one, read the nearest" against `OfChosen`'s "exactly one",
    with the two kept apart so that `badTwoChoosersOneSortRead` stands
    untouched. Two things it got wrong, both recorded as findings: its
    **five sorts are seven** (1022 — "card" was one word for a chosen card
    NAME and a chosen card OBJECT, and creature type was missed; the 12
    read back colour, card name, creature type, number, card object, player
    and direction, and unsupported Polis the Planeshifter adds plane), and
    **the chooser side is three shapes, not two** (1023). The landed row is
    COLOUR-FIXED with the refusal recorded: at the reader's own position —
    a description's modifier and the bare read after a preposition —
    "the last chosen color" is 2 lines and every other sort is 0, the other
    sorts' reads living in carriers this row is not. What remains of the
    family is below, each piece counted rather than left as "the rest".
    - **The ATTACH-triggered chooser**, 3 cards (Sanctuary Blade, Dinosaur
      Headdress, Psychic Paper): "As this Equipment becomes attached to a
      creature, choose …". `EntersChoice`'s `ZoneFits (nounZone n) (Just
      Battlefield)` gate is keyed to entering the battlefield and the
      attachment happens after the permanent is already there, so this is a
      different entry-replacement timing and not a widening of that row.
      It is the marked read's largest single blocker, and the price is
      PROBED not guessed: Sanctuary Blade written with a stand-in
      as-enters chooser elaborates whole — its second line is `Gains …
      (KeywordAbility Protection {param = Just (ParamQuality
      OfLastChosenColor)})`, Voice of All's shape with the marked word, and
      its Equip line writes too — so the attach trigger is that card's ONLY
      blocker.
    - **The chooser that chooses an OBJECT**, 4 cards (Koh the Face
      Stealer, Dinosaur Headdress, Forgotten Lore, Shrouded Lore): "Choose
      a creature card exiled with Koh", "Target opponent chooses a card in
      your graveyard". The read is "the last chosen card" and the mention
      is an OBJECT, not a `Quality` — `countQuality`/`qualityB` do not
      reach it, and the two Lores add an opponent as chooser and a repeated
      process ("repeat this process except that opponent can't choose a
      card already chosen for Forgotten Lore") on top.
    - **The non-entry chooser**, 5 cards (Beckoning Will-o'-Wisp and
      Triarch Stalker at a combat trigger, Teyo at a loyalty ability, Koh
      at an activated ability, Mystic Barrier at a FUSED "when this
      enchantment enters and at the beginning of your upkeep" header): the
      chooser sentence is not an entry rider at all. Chromatic Armor's and
      Shapeshifter's second choosers belong here too — the effect-level
      `Choose` row exists, so what these want is the CONTAINER, not the
      choice clause.
    - **The marked read's other carriers**, 2 lines: Psychic Paper's name-
      and-type SETTING clause ("its name and creature type are the last
      chosen name and creature type" — two qualities read in one clause,
      and the sorts `ChosenQualityRead` calls unreadable for the unmarked
      read), and Shapeshifter's AMOUNT ("power is equal to the last chosen
      number"), which is the chosen-number Amount finding 1000 already owes
      Sanctum Prelate — one construction serving two entries.
    - **The sorts with no representation**: DIRECTION has no `Kind`
      constructor at all (Mystic Barrier, Teyo — "the nearest opponent in
      the last chosen direction"), and PLAYER has a `Kind` and a payload
      but is unreachable through `countQuality`, which is hard-wired to
      `Quality QualitySort` (Beckoning Will-o'-Wisp, Triarch Stalker —
      note the read stands in `Attacks`' defender slot, landed round 40).
      Polis the Planeshifter's PLANE is unsupported and needs no row.
    - **The overgeneration the row carries** (finding 1030): every one of
      the 12 carriers has a chooser that can fire more than once, and no
      card writes the marked read against a once-only chooser — but
      nothing here represents a chooser's repeatability, so
      `OfLastChosenColor` admits a marked read after a single
      non-repeating chooser. Closing it wants the same kind of provenance
      chapter one hundred twenty-five queued for anaphors, and it is worth
      nothing on its own.
    - **Chromatic Armor's residue**, 2 lines: the `{X}` activation cost
      (`SimpleManaSymbol` is `Generic Nat | Specific ColorOrColorless` —
      there is no variable symbol) with the further wrinkle that its
      defining rider is read by the ABILITY'S COST rather than by its body,
      and the `sleight` counter kind, deliberately not bought while the
      card cannot land whole. Everything else on that ability already
      writes.
  - ~~**The mana read**~~ — **CLOSED by chapter one hundred twenty-six**,
    which landed `ProducedMana.OfChosenColor`. THIS ENTRY'S COUNT WAS THE
    QUEUE'S LARGEST MISCOUNT TO DATE (finding 1012): the family is **33
    lines over 32 cards**, not 101/91, and 101/91 is within a line and two
    cards of the SIBLING "of the chosen type" family (102/89 here), an
    unrelated tribal construction discussed in the adjacent paragraph — a
    copy between neighbours, not a count. Its shortlist was also one card
    short: MIRAGE MESA is the ninth clean two-line carrier and is benched.
    NOTE FOR FUTURE READERS: this cell had a SECOND entry further down this
    file ("The chosen color as produced mana"), whose count was exactly
    right and whose blocker was wrong; the two are reconciled there and
    both are now closed.
  - **The effect-level chooser** — a DIFFERENT family the container does not
    touch and never did (finding 836): 27 lines write "Choose a color." as a
    spell's own instruction and 21 write one after a colon, both binding for
    the rest of their own ability, which `Choose` plus the quality mentions
    have spelled since chapter twenty-two. Nothing owed.
  - **The narrowed choice** — "choose a color other than black", 16 supported
    lines (the five Thriving lands, Chromatic Sphere's kin). A printed
    restriction on the chooser's range; `EntersChoice` takes no domain slot
    and chapter one hundred four declined the same slot for the same reason.
    Sixteen lines — **CLOSED by chapter one hundred eleven together with the
    card-name half** (findings 865–867). The narrowing was measured at all
    four sorts and landed as one indexed vocabulary, `ChoiceDomain`, read at
    two positions: `ColorOtherThan` 10 lines, `TypeOtherThan` 4,
    `NumberAbove` 2, `NameOfCard` 26. The creature-type cell carried that
    round (Imagecrafter, Unnatural Selection, Standardize benched).
    Re-derived: the colour narrowing is 10 supported lines, not 16.
    **THE COLOUR CELL'S RESIDUE IS CLOSED BY CHAPTER ONE HUNDRED
    TWENTY-SIX** (finding 1018): it had no whole witness because all ten of
    its lines trail "add one mana of the chosen color", and that production
    row now exists. The five Thriving lands are benched whole — one land per
    colour, the domain's entire attested colour range — so `ColorOtherThan`
    has its first whole witnesses fifteen chapters after it was minted. The
    other five (the Gate cycle) carry a third, unrelated ability apiece and
    stay two-line-shortlist residue rather than gaps.
  - **The compound chooser** — "choose a color and a creature type" (Riptide
    Replicator, Volrath's Laboratory) and "choose a color and an opponent"
    (Call to Arms), 3 supported lines. The COMPOSITION is already free —
    two `EntersChoice` lines of different sorts thread two mentions and both
    reads elaborate (probed) — so what is missing is only the SPELLING of
    two choices in one sentence. A coordination question, not a discourse
    one.
  - **The each-player chooser** — "a type chosen this way", 3 lines (Harsh
    Mercy, Patriarch's Bidding, Lydari Druid). Unchanged: a distributive
    choice with as many values as there are players and no single binding to
    read. Not this shape.
  - **The CARD TYPE and LAND TYPE sorts** — 11 card-type chooser lines and 20
    land-type ones, against one card-type reader (Pippin, Guard of the
    Citadel). `QualitySort` has neither value; chapter one hundred four
    routed the land type to its own vocabulary. The chooser row will take
    them the day the sort exists, without further container work.
  - **Bloodline Pretender** is blocked on `Changeling`, a missing `Keyword`
    catalog entry, and on nothing else — chapter one hundred six probed its
    triggered header green and this round benched the same shape (Prism
    Ring). It falls out of the next keyword round for free.

- ~~**The CO-REFERENTIAL name — "with the same name as [X]"**~~ — **CLOSED
  by chapter one hundred eleven** (findings 871–874). `NameSource.SameNameAs`
  takes an ordinary `Noun bs Object`, which is what the relatum inventory
  asked for: 61 anaphors, 18 bare indefinites, 10 definites, 9 "this [word]",
  6 complements, 4 partitives, 3 targets, 2 "it". One demand, the relatum is
  singular ([CR#201.2a] compares an object to an object), satisfied by 111 of
  the 113 lines and pinned for the other shape (`badGroupNameRelatum`).
  Benched: CANDLES OF LENG (indefinite relatum, condition frame), AVEN SHRINE
  (anaphor inside a counted amount) and BIFURCATE (a target relatum inside a
  search description). Residues, each counted:
  - **The COORDINATED subject**, and it is the family's commonest frame —
    "Target creature and all other creatures with the same name as that
    creature" (Echoing Decay, Echoing Truth, Echoing Ruin, Echoing Calm,
    Echoing Return, Bile Blight, Declaration in Stone, Deputy of Detention,
    Banishment, Cylian Sunsinger). The name half is spelled; what these want
    is a subject that coordinates two nouns, which is RUKARUMEL's
    construction from chapter one hundred nine. Landing that one buys this
    whole cycle at once.
  - **The three-zone search**, again (Eradicate, Counterbore, Crumble to
    Dust, Deicide, Bloodbond March's per-player return): "Search its
    controller's graveyard, hand, and library for …" — the same construction
    the Memoricide cluster waits on, so the two families should be counted
    together when it is scheduled.
  - **The SAME-NAME group constraint**, 6 lines, moved to chapter
    eighty-nine's entry where it belongs: "with the same name as one
    another" (Chrome Replicator, Mechanized Production) and the four
    elliptical lines that write no relatum at all (Endless Atlas, Sceptre of
    Eternal Glory, Sphinx of the Chimes, Tainted Pact). [CR#201.2b] is the
    negative pole's rule and states it in the same shape.

- **Colour-only "becomes [color]"** — layer 5 ([CR#613.1e]), 42 supported
  lines under the "becomes" verb alone ("becomes blue until end of turn",
  "All creatures become black"). No row and no neighbour: `SetsType`'s payload
  carries colors, but only as part of a type line, and a colour change that
  names no type is a different layer and a different sentence. Small and
  self-contained.
  IT NOW HAS A NEIGHBOUR (chapter one hundred eight, finding 841):
  `SetsChosenQuality`/`AddsChosenQuality` spell the colour change at this
  same layer when the colour is a READ ("All nonland permanents are the
  chosen color", Shifting Sky; Painter's Servant's adding form), 20 lines.
  What is left here is exactly the LITERAL half — a colour WORD where those
  write a read — so the entry is unchanged in size and now has a row to be
  modelled on rather than designed from nothing.
- **The planeswalker row's WITNESS DEBT — PAID (chapter eighty-nine), and what
  the payment left behind.** The loyalty frame landed (`LoyaltySymbol`, four
  symbol rows, seven reader cells, eight pins) and with it two whole
  planeswalkers: JACE BELEREN pays the card type's debt (finding 638) and
  ELSPETH, SUN'S CHAMPION pays chapter eighty-eight's emblem debt in the same
  card. The Talent cycle's third debt is paid as far as a fragment pays it —
  `elspethsTalentGrant`, all seven quoted-loyalty carriers checked and blocked
  elsewhere. Nothing is owed on the card type or on the frame. What the round
  did NOT close, each its own entry below: the activation event, the
  ability-sharing static, the loyalty counter, the starting-loyalty box, and
  the two non-cost positions of the symbol itself.
  GIDEON JURA, the entry's old named target, is now a ONE-gap card, and
  chapter ninety-four corrected the second gap's diagnosis rather than closing
  it (finding 706). The [0]'s second sentence was said to want "the SOURCE as
  a damage recipient (`DamageRecipient` refuses `This` bare and ascribed
  alike)"; the ascribed half of that was FALSE — `AsType Creature This` was
  always a legal recipient and the corpus writes it 30 times. What refused was
  the PLANESWALKER ascription, at `DamageableTy`, which had one row; the row
  landed and the [0] is written whole on Gideon, Ally of Zendikar. What is
  left is the [+2]: "during target opponent's next turn" is a duration
  `Duration` has no row for (probed, refused), on top of the `DeonticPatient`
  cell for a forced attack aimed at a named permanent. The prevention
  machinery was never the blocker, as this entry already said.
- **The SOURCE head word — CLOSED by chapter ninety-seven**, and the entry is
  kept for the three residues it left. `Predicate.IsSource` is a HEAD
  PREDICATE, not the `NounWord` finding 707 fenced — the corpus wants a
  description head in 268 of the word's 281 occurrences and a READ in the
  other 13. Its whole gate is a projection (`headIsPlaceless`, generalised
  from `AnyTarget`): [CR#120.7] exempts a source from being in any zone, so
  every verb that demands one refuses it and "destroy target source" and its
  kin are unwritable rather than merely unwritten. `ChoiceMode.YourChoice`
  came in beside it. Urza's Armor, Deflecting Palm, Circle of Protection: Red,
  Healing Grace, Reverse Damage and Sphere of Law are all benched whole.
- **The source READ** — 13 sentences, and the lowest-value entry this family
  has left, recorded so that is visible rather than rediscovered. "That
  source" and "the source's controller" (Deflecting Palm, New Way Forward,
  Honorable Passage, Comeuppance, Archfiend of Spite, Belltower Sphinx,
  Phyrexian Obliterator, Crag Saurian, Reaper of Sheoldred, Rona, Elesh Norn,
  Flameblade Angel, Bitter Feud) want `SourceW` in `NounWord`, which is a
  dozen tables wide — `wordReaches`, `verbedWordOk`, `kindOfW`, `zoneOfThat`,
  `tyOfThat` and the rest, most of which would answer the narrow way `CopyW`'s
  do. EVERY ONE OF THE 13 IS BENCHABLE TODAY through `It` with the
  demonstrative re-sort elided, and Deflecting Palm is benched that way, so
  this buys a SPELLING and not a sentence. Take it only when a card needs the
  re-sort to disambiguate two object mentions, which none of the 13 does.
- **The protection-shaped ABILITY source** — 12 sentences, and the word is no
  longer what blocks them: "can't be the target of nongreen spells or
  abilities from nongreen sources" (Gaea's Revenge, Thrun, Spellbane Centaur,
  Mercenary Informer, Raiding Party, Rebel Informer, Suq'Ata Firewalker,
  Artifact Ward and kin). What these want is the negated colour distributed
  over BOTH conjuncts and a targeting restriction that reaches two phrases at
  once — one statement, two descriptions, and the conjunction is the ask.
  [CR#113.7] is the relation and chapter ninety-seven's `IsSource` is the
  head; nothing else about the source is missing.
- ~~**The NAMED source**~~ — **CLOSED by chapter one hundred twenty-two**,
  all 8 sentences. PHYREXIAN REVOKER and VOIDSTONE GARGOYLE are benched
  WHOLE (the Gargoyle writing the prohibition at both of its subject sorts
  in two consecutive sentences), and so are PITHING NEEDLE and DAMPING
  MATRIX with the mana carve-out. This entry's own claims, corrected:
  "sources with the chosen name" was indeed writable and the round verified
  it by probe rather than trusting the entry (finding 977); "the verb
  catalog (`ObjectAct`) is already waiting for it" was true only of the
  table's SHAPE — the `Activated` row did not exist and the round minted it;
  and the "74 supported lines, of which these eight are the smaller half —
  50 more" is 98 lines with 43 possessor-anchored (finding 970). `Deontic`
  turned out not to be the territory at all — its deed catalog is
  `Attack | Block` and its participant gate demands a card type — and
  `CostsToCast` was, but reached by an index over the kind rather than from
  a new side. The mana carve-out did ride along, and chapter ninety-one's
  no-mana-ability-tag ruling was re-read and HELD: `IsManaAbility` is a
  word the sentence writes about a class, not a tag on an ability (finding
  975). REMAINING from the eight: SORCEROUS SPYGLASS and ANOINTED
  PEACEKEEPER want "look at an opponent's hand" inside the entry choice;
  DISRUPTOR FLUTE wants Flash beside its two writable lines; PETRIFIED
  HAMLET wants a quoted mana ability granted to a class; SKYSEER'S CHARIOT
  wants Vehicle and Crew. Each is one named blocker away and none of them
  is this construction.
- **The complement AMOUNT** — "all but [n]", 9 supported sentences and two
  constructions away from each other, which is why it is one entry and not
  two. Four are the SHIELD's determiner ("prevent all but 1 of the damage
  that would be dealt to you this turn") and five the event-shaped row's cut
  ("the next time an unblocked creature of your choice would deal combat
  damage to you this turn, prevent all but 1 of that damage" — Forcefield;
  Ajani Steadfast's emblem, Hyperion, Temple Altisaur). Both `Shield` and
  `PreventCut` refuse it today and both would take the same value, so build it
  once. Beside it, and smaller: "half that damage, rounded up/down" (2,
  Gisela, Blade of Goldnight), which wants a halving amount the magnitude
  vocabulary has no row for and which carries the rounding word with it.
- **The DAMAGE EVENT as a mention** — counted FOUR times now and wanted by
  four families. Ten can't-be-prevented lines write "the damage"/"that damage
  can't be prevented" reading a damage event their own previous clause
  described (Combust, Banefire, Demonfire, Urza's Rage, Flames of the Blood
  Hand, Volcano Hellion, Pinpoint Avalanche, Arrow Storm, Lightning Surge,
  Lava Burst), and chapter ninety-five's three redirection residues want the
  same thing from three more sides (the excess-damage redirect's
  portion-reading subject, the redirect-immunity pair, and Flaming Gambit's
  and Megatron's anaphoric bodies). A damage clause leaves an `Outcome`
  behind, which is a MAGNITUDE read by "that much" and never an event to
  point at, so the ask is an event mention with an amount projection — which
  is exactly chapter twenty-five's deferral (findings 141/143) stated as a
  noun question. If it lands, `CantPrevent`'s anaphoric subject and all three
  residues fall out of it. THE REDIRECTION BODY DID NOT NEED IT IN THE END,
  and that is worth knowing before this is scoped: chapter ninety-five fixed
  the body to a redirection instead of leaving it a general `Effect`, so
  "that damage" became the row's own constant rather than an amount anything
  computes.
  CHAPTER ONE HUNDRED TWO DID NOT TOUCH THIS and the distinction is worth
  keeping sharp: a damage EVENT now leaves an `Outcome` behind too
  (`eventAfter`), so the MAGNITUDE is readable from both mouths, and this
  entry is still asking for the event as a NOUN. The excess-damage redirect
  was re-measured against the new announcement and stays fenced — its
  subject reads a PORTION of a damage event ([CR#120.10] computes it), not
  the event's whole magnitude, and no `Amount` row spells it.
- **The TRIGGER EVENT's magnitude — the two damage cells CLOSED by chapter
  one hundred two**, and the entry is kept for the rest of its own table.
  `eventAfter` announces `outcomeB DamageDealt` at `IsDealtDamage` (58
  lines / 57 cards) and at `DealsCombatDamage` (77 / 77), and the passive
  row mints its subject's mention besides, which is what lets a body say
  "it" or "that creature" after the event. No sort was minted and no
  vocabulary; nine witnesses. What the same 326-line sweep left, each
  measured and each blocked on a `GameEvent` row rather than on the
  anaphor:
  - **The ACTIVE damage event, with its source written** — 43 lines over 42
    cards ("Whenever a source you control deals damage to you, put that many
    +1/+1 counters on this creature"; "Whenever enchanted creature deals
    damage, you gain that much life"), plus Soul-Scar Mage on the
    replacement side. The biggest and the nearest: it is the damage event
    with `DamageAgent`'s axis written, which the redirection and prevention
    rows both carry already, so the vocabulary exists and only the event row
    does not. Whichever round takes it inherits this chapter's announcement
    for free.
  - **The replacement side of both damage events** — 10 lines ("If damage
    would be dealt to this creature, put that many +1/+1 counters on it
    instead": Phytohydra, Lichenthrope, Delaying Shield, Force Bubble,
    Crumbling Sanctuary, Nefarious Lich, Dralnu, Szadek, Undead Alchemist,
    Soul-Scar Mage). Blocked on the TABLE CELL and not on the announcement:
    `eventUse DamageTaken` and `eventUse CombatDamage` are both
    `TriggeredOnly`, so `Intercepts` cannot take either event, which is why
    chapter one hundred two announced on `eventAfter` alone. Flipping the
    cell is the whole ask, and the `eventIntro` half follows it. The 22
    "deals that much damage plus N instead" lines are NOT this: chapter one
    hundred made that phrase `DamageScale`'s own spelling and they need
    nothing.
  - **The LIFE events** — 14 gains and 8 losses or payments ("Whenever you
    gain life, target opponent loses that much life", Sanguine Bond;
    "Whenever an opponent loses life, you gain that much life", Exquisite
    Blood; Ageless Entity, Kavu Predator, Vilis, Mindcrank). No `GameEvent`
    row watches a life change and [CR#119.3] is all the rules supply. The
    sorts they would announce already exist (`LifeGained`, `LifeLost`), so
    this is an event row and nothing else.
  - **The BATCH-COUNT events** — twelve attack declarations reading the
    ATTACKING GROUP's size ("Whenever one or more Dragons you control
    attack, draw that many cards", The Ur-Dragon), six discards, two exiles,
    two token entries, and one apiece of energy, mill and draw. Each is the
    counter batch's shape at a different verb and each is under finding 85's
    bar alone; the attack cell is the only one worth its own round, and it
    wants a plural-subject group size where `eventSubjectPlur` records
    singularity.
  - **The damage KIND on the event** — 5 lines ("is dealt combat damage",
    Pious Warrior, Wall of Essence, Wall of Souls, Souls of the Faultless;
    "is dealt noncombat damage", Smaug). `IsDealtDamage` has one slot and no
    `DamageKind` axis, which is the shield row's adjective at an event
    position. Small, and it would land beside the active-damage row.
  - **The DOUBLE READ** — 3 lines, and recorded as a PRICE rather than a
    gap: "Whenever this creature is dealt combat damage, you gain that much
    life and attacking player loses that much life" (Souls of the Faultless;
    Kain, Neheb) reads one announcement twice, and the second read is
    refused because the first clause's own outcome joins the context.
    `ThatMuch`'s obligation is a uniqueness one and this is what it costs.
    Attested English, so not pinnable, and not fixable without a second
    reader.
  - **The BODY-INTERNAL announcement** — 60 lines over 59 cards, NOT fenced
    and recorded so no successor re-buys it as part of this family: "discard
    any number of cards, then draw that many cards" reads a quantity the
    BODY wrote, not the header's event, and it belongs with `GroupSize`'s
    family. It is the largest false friend in the sweep.
- **The [CR#615.5] consequence rider — CLOSED by chapter ninety-six**, and
  the entry is kept only for the four residues it split into. The rider is a
  FIELD on `Prevents` and `PreventsFrom` (the rule's own framing, and the only
  shape that reaches a standing static ability); `OutcomeSort.DamagePrevented`
  is announced in the field's own typing and nowhere else; `PreventedThisWay`
  reads it at the named surfaces and `ThatMuch` at the anaphoric one. 35 of
  the 36 sentences are readbacks and the thirty-sixth (Judgment of Alexander)
  only looks like one. THE PREVENT-AND-DEAL PAIR IS DONE: chapter
  ninety-seven landed the "source" head word its first sentence wanted, and
  Deflecting Palm is benched WHOLE — the campaign's longest-standing marquee,
  blocked in three consecutive rounds on three different things.
- **The source-QUALIFIED prevention container** — 6 sentences, and the half of
  the rider's container clause that carries meaning rather than spelling: "If
  damage from a RED SOURCE is prevented this way" (Honorable Passage), "from a
  creature source" and "from a noncreature source" (Comeuppance's two), "from
  a black source" (Shadowbane), "from a creature" (Judgment of Alexander),
  "from a black or red source" (Samite Ministration). It restricts WHICH
  prevented damage feeds the rider, which no spelling supplies, and four of
  the six also want the headless "source" head word — so this entry and that
  one should be taken together or this one waits.
- **[CR#615.13]'s prevention TRIGGER** — 4 sentences, and a different
  construction from the rider beside it, which the rule states outright:
  "some TRIGGERED ABILITIES trigger when damage that would be dealt is
  prevented. Such an ability triggers each time a prevention effect is applied
  to one or more simultaneous damage events and prevents some or all of that
  damage." A trigger uses the stack where [CR#615.5]'s rider does not. "When
  damage is prevented this way" (New Way Forward, Phyrexian Vindicator),
  "Whenever …" (Judgment of Alexander, Samite Ministration). No `GameEvent`
  row watches a prevention and four lines is under finding 85's bar for
  minting one; Phyrexian Vindicator is the cheapest whole card if it is ever
  taken, and its "any OTHER target" is a second small ask.
- **The `{X}` MANA SYMBOL — CLOSED by chapter ninety-eight.** `ManaSymbol`
  gained a nullary `Variable` row, sibling to `Simple`/`Hybrid`/`Phyrexian`
  ([CR#107.4,107.4b]); the shared multi-X value is [CR#107.3i], recorded as
  an unchecked side condition rather than built. Re-measured at 531 supported
  cards (house method, unchanged from chapter ninety-six) and all 531 are
  now cost-writable — no other symbol in any of those costs was ever the
  blocker. Zero reader cells: nothing in the grammar pattern-matches
  `ManaSymbol`'s constructors for a semantic read. Witnesses: `temper`
  (beside `testOfFaith`, Test of Faith's minimal pair) and `prosperity`
  (a fragment retired into the card, its cost line the only thing it was
  missing). QUEUE CORRECTION: Divine Deflection was NOT "blocked on exactly
  this and on nothing else" — it was blocked on the CONJOINED RECIPIENT gap
  ("you and/or permanents you control"), independent of `{X}`. Chapter
  ninety-nine landed that gap and the card is now benched whole, which is
  this correction paying out rather than being reversed.
- **REDIRECTION — CLOSED by chapter ninety-five.** Two rows landed:
  `Redirects` (the shield-shaped single clause, 44 sentences over 44 cards)
  and `RedirectsFrom` (the event-shaped one, 15/15), with `DamageAgent`
  beneath both and `SingleRecipient` on the destination. The recon's
  "single-clause redirect 64/63" was one bucket over FOUR constructions; the
  re-derived total across all four is 66. What is left is the four entries
  below plus the amount rider above.
- **The amount-SCALING replacement — CLOSED by chapter one hundred.**
  `StaticEffect.Scales` over the `DamageScale` vocabulary, 62 supported
  sentences over 60 cards (round fifty-nine's figure re-derived unchanged).
  Two things this entry expected it to need turned out not to be needed:
  the amount readback (0 lines write a literal base amount, so the operation
  slot absorbs it — `PreventCut`'s trade at a third site) and the union
  head's anaphor (the closing "to that permanent or player" is a SPELLING,
  finding 760's crossed grid). Seven whole cards benched, five of them
  writing `KindJoin`'s permanent cell that chapter ninety-nine landed
  witnessless. What is left of it is one line: **the where-X rider on a
  shift amount** — Hawkeye, Young Avenger's "it deals that much damage plus
  X, WHERE X IS HAWKEYE'S POWER", the only one of the family's three
  variable shifts that `Amount` cannot already spell, and blocked on
  `WhereLetterStatic` reaching into an operation's amount rather than on
  anything of this family's. One sentence, under finding 85's bar on its
  own; it belongs to whichever round takes the where-rider's remaining
  positions (`PreventCut` ledgers three more of them). RE-MEASURED against
  chapter one hundred two and it does NOT fall in: Hawkeye's line is an
  `Intercepts` body and its "that much damage" is `DamageScale`'s spelling,
  so the trigger event's announcement reaches neither half of it. The
  where-rider is still the whole of what it wants.
- **`PreventCut`'s HALVING cut** — 2 sentences ("prevent half that damage,
  rounded down", Dark Sphere; "rounded up", Gisela, Blade of Goldnight),
  ledgered by chapter ninety-four as needing "a halving amount the magnitude
  vocabulary has no row for". Half of that is now built: chapter one
  hundred's `RoundMode` is the vocabulary, and [CR#107.1a] is why it is
  obligatory. What is left is a `PreventCut` row that halves rather than
  cutting a written count — small, and the two cards write the two rounding
  directions between them, so it would populate both cells at once. Under
  finding 85's bar alone; schedule it beside anything else touching the
  shield's cut.
- **The NOUN COORDINATIONS chapter ninety-nine did NOT land** — that chapter
  closed the conjoined damage recipient (`Noun.YouAnd`, 35 sentences, Harm's
  Way and Divine Deflection and Endure benched) and the cross-Kind union head
  (`Predicate.KindJoin`, 326 occurrences, Lava Axe and Searing Flesh and
  Onakke Javelineer benched). Three coordinations are left, each measured and
  each a different mechanism:
  - **The player-plus-player DISTRIBUTIVE**, 39 supported sentences over 36
    cards ("you and target opponent each draw a card", "each opponent and you
    each create a Treasure token"). The trailing "each" is present in ALL 39
    and bare unmarked joint player-plus-player is 0, so the marking IS the
    construction rather than decoration on `YouAnd`'s shape — it needs a
    distribution over two referents where `YouAnd` has a joint reading and an
    invariant player half. Two more fold the distributivity into the second
    conjunct (Model of Unity, Juxtapose). Do not fold this into the group
    referent; it is the opposite reading.
  - **The heterogeneous DOUBLE TARGET**, 12 sentences over 12 cards (Churning
    Eddy, Fumarole, Goblin Grenadiers, Grip of Desolation, Hull Breach,
    Legerdemain, Necron Deathmark, Plague Spores, Spiteful Blow, Stomp and
    Howl, Sudden Substitution, Cruel Entertainment). TWO MENTIONS, not one
    filter, and [CR#601.2c]'s own example says so: "a spell that says 'Destroy
    target artifact and target land' … can target the same artifact land twice
    because it uses the word 'target' in multiple places". Neither `KindJoin`
    nor `Or` can reach it — the word "target" is written once in a union
    phrase and twice here. Ballroom Brawlers is a thirteenth of the same
    shape with a floating "both" over it. None benched.
  - ~~**The union head's ANAPHOR**~~ — **CLOSED by chapter one hundred
    twenty-eight**, which landed `Payload.UnionP` and `NounWord.JoinW`.
    THE ENTRY WAS ACCURATE IN EVERY FIGURE (finding 1031): 326 occurrences,
    exactly 48 anaphoric, exactly 21 of them inside the scaling family and
    closed there — all three reproduce against fresh measurement, the first
    queue cell in five rounds that needed no correction. The mechanism was
    as described and one layer lower than "no union word": `wordReaches`
    asks the binding's PAYLOAD, and a union-headed mention was leaving the
    same `ObjectP` an ordinary object leaves, so the fix marks the mention
    (`UnionP`, carrying nothing) and adds the one word that reads it. The
    echo's word pair is DERIVED, not carried (1032 — 33 of 33 copy their
    antecedent's pair, 15 of 15 with no pair write the generic one). What
    remains of the cell is below.
    - **Its own residue is 26 occurrences, not 27** (finding 1036): PLATED
      PEGASUS needed nothing and is benched whole — its closing "to that
      permanent or player" is one sentence's spelling of its own recipient,
      finding 760's boundary one card wider than chapter one hundred drew
      it. The two-sentence prevention riders (Guardian Angel, Orim's Touch)
      do want the anaphor and are blocked on `{X}` in a cost and on the
      kicked condition respectively.
    - **The FIXED-DAMAGE repeats**, 7 occurrences, each blocked on its own
      gating clause and none on the anaphor: Raid (Arrow Storm), Threshold
      (Lightning Surge) and kicker (Urza's Rage) are unminted ability
      words/keywords; DIVINE PRESENCE and EQUAL TREATMENT want a damage
      replacement conditioned on the AMOUNT ("if a source would deal 4 or
      more damage to …"), which `IsDealtDamage` cannot express — the event
      row carries a recipient and neither an agent nor an amount, where
      `Scales` carries both but only scales; GHYRSON STARN, KELERMORPH wants
      a damage-DEALT trigger with an agent ("whenever another source you
      control deals exactly 1 damage to …"), the same slot from the trigger
      side. Two rows would take all five.
    - **The counter reads**, 8 occurrences, ALREADY FILED TWICE AND NOT
      REFILED (finding 1040): Vorinclex (×2), Innkeeper's Talent and
      Lae'zel are four of the ten lines of "the DISTRIBUTIVE counter-kind
      anaphor, and the kind-blind event"; Animation Module, Maulfist
      Revolutionary, Powerful Broker and Skyship Plunderer write "another
      counter of that kind", the chosen-kind family named at that entry's
      foot, plus a for-each-KIND quantifier. Both entries now have ONE
      BLOCKER FEWER — the union recipient they all read is landed — and
      neither is this cell's to re-price.
    - **TAHNGARTH, FIRST MATE**, 1 occurrence: "Tahngarth is attacking that
      player or planeswalker" wants `Attacks`' defender slot to admit a
      union, and the slot is `Noun bs Player` (probed, refused — the
      anaphor is Object-kinded). The defender slot landed in chapter
      seventy-six with a player-only `AttackDefender`; this is its second
      value.
    - **The two antecedent shapes with no head**, 5 occurrences (finding
      1033): the two-phrase coordination ("an opponent or a permanent an
      opponent controls" — Bitter Feud, Gisela Blade of Goldnight, Solphim
      Mayhem Dominus) and the three-way union ("creatures, planeswalkers,
      and/or players" — Soulfire Eruption; "a creature or planeswalker you
      control or … yourself" — Lae'zel). Both write the generic echo, so
      the READ is landed and what is missing is the antecedent's own
      construction. `KindJoin`'s grid is 2×4 and neither of these fits in
      it.
  - **The SPLIT-DETERMINER union read**, 25 occurrences over 24 cards — the
    sibling spelling the entry above never counted (finding 1037). "That
    player or that planeswalker's controller" (Blightning, Searing Blaze,
    Rakdos's Return, Chandra Nalaar, Bonfire of the Damned, Lavalanche, Soul
    of Shandalar ×2, and sixteen more) names each half of the union instead
    of echoing it whole, and TWO CARDS WRITE BOTH SPELLINGS IN ONE TEXT
    (Searing Blaze, Quenchable Fire), which is what says it is one
    phenomenon. It is NOT closed by chapter one hundred twenty-eight and it
    needs two things: (a) the halves' own word access — `That PlayerW` and
    `That (TypeW Planeswalker)` reaching a union mention, which the landed
    `UnionP` deliberately refuses for want of evidence (no card reads a half
    alone), and which would want the CLASS half recorded on the payload to
    keep "that planeswalker's controller" from following a permanent-headed
    union; and (b) a same-Kind NOUN DISJUNCTION of two player nouns, which
    is a FOURTH coordination beside the three above. Measured shape: the
    class half echoes its antecedent (planeswalker 21, permanent 4) and the
    player half does not (24 "player", 1 "opponent", and Rakdos's Return
    writes "that player" over an antecedent that said "target opponent"), so
    the two halves take different gates. `ControllerOf` is not a blocker —
    it takes an ordinary object noun and would compose the moment (a)
    lands.
  MEASURED ZEROS recorded so no successor re-buys them: fresh "either … or"
  as a coordinator 0 (all 9 are anaphoric "either of them"), fresh "both X and
  Y" as a two-referent coordinator 0, "neither … nor" 12 of which 10 are the
  "neither day nor night" state idiom. None is pinnable — the refusal is by
  VOCABULARY, there being no such word in the grammar to be `impossible`
  about.
  ALSO OUT OF SCOPE and named so no successor folds them in: the and/or
  type-word bucket, "instant and sorcery cards" (a spelling of disjunction on
  ONE mention), `AndAlso`/`StaticParts` (statement-level, chapter
  eighty-three), the "when … and whenever …" header join (queued separately)
  and modal "choose one or both".
  LAYERING RULING (user-settled, 2026-08-15): this stays a NOUN COORDINATION
  question. The prior core iteration minted players as zoneless non-card
  objects, and that unification is a core/lowering concern, NOT a grammar
  refactor — the surface always MARKS its player/object union sites (a minted
  word "any target", explicit disjunction, explicit coordination), so the
  grammar keeps the Kind split and represents each union site as the
  construction the cards write. Do not brief a players-as-objects grammar
  round; the full ruling with the lowering wrinkle (zone-proxy gates like
  `ObjectTakes` cannot key on zone for a zoneless player) lives in the
  session ledger. CHAPTER NINETY-NINE IS THE RULING APPLIED: `KindJoin` and
  `YouAnd` are marked union sites written down as constructions, the `Kind`
  split is untouched, and the three phrases that may denote a player now
  share one table (`nounSpansPlayers`).
- ~~**The ALTERNATIVE COST sentence**~~ — **CLOSED by chapter one hundred
  twenty-three**, which landed `StaticEffect.AltCost` over 123 supported
  lines. TWO OF THIS ENTRY'S CLAIMS WERE WRONG and finding 989 says how.
  (i) "Fall of the Titans' Surge line is the same gap (chapter ninety-eight
  recorded it), so the two want one round" — it is NOT the same gap. Fall
  of the Titans' line is Surge's REMINDER TEXT, verbatim-shared with ten
  other Surge cards and part of a 956-line, 25-keyword reminder population;
  Shining Shoal's is a card's own printed sentence in a family with zero
  reminder text. This round does not unblock it. Chapter ninety-eight is
  not the source of the error — finding 743 says only "its Surge
  alternative cost still unbuilt", which is accurate; the equivalence was
  this entry's own addition. (ii) The entry's second example, "if you've
  cast another spell this turn, you may pay {1}{U} rather than pay this
  spell's mana cost", IS NOT A CORPUS LINE — zero supported lines gate a
  "rather than pay" offer on having cast another spell. It reads as Surge's
  reminder condition, or Patrician's Scorn's gate, moved across the
  phrasing boundary. RESIDUE, counted: the 14 GENERIC GRANTS (As Foretold,
  Fist of Suns, Dream Halls — "the mana cost FOR spells you cast", a
  subject slot this row has none of and `CostsToCast`'s shape); the 11
  NON-MANA declined costs (equip 3, cycling 2, echo, crew, power-up, and
  two single-mana ones — these want the declined cost to be a VALUE, which
  is `Pay`'s standing decline); the 23 pronoun-tail lines (Worldheart
  Phoenix's "by paying … rather than paying its mana cost" — play
  permissions with an alternative-cost rider, `MayPlay`'s); INVIGORATE
  alone (its payment is a life GAIN and `costActionOk` admits only the
  loss — one line, one cell, table deliberately not widened); and the 5
  COMMANDER-gated free spells (Deflecting Swat, Fierce Guardianship, Deadly
  Rollick, Flawless Maneuver, Obscuring Haze), blocked because
  `CommanderD`'s scope is `HeldByCard` and `HasDesignation` reads only the
  object- and player-held rows.
- ~~**The EXACT-VALUE description**~~ — **CLOSED by chapter one hundred
  twenty-four**, which landed `Comparator.Eq` as a fifth row on the shared
  table with no per-reader gate. THE ENTRY'S COUNTS DO NOT REPRODUCE and
  finding 991 says so: the description frame is 116 exact lines against
  1,003 ranged (18 literal, 63 X, 35 read), not 78 against 416, and no
  pattern tried reproduces either figure. THE ENTRY'S DIAGNOSIS WAS ALSO
  HALF WRONG (finding 992): the condition frame is not the reader that
  refuses the cell — it WRITES it, 27 occurrences against 914, in the bare
  copula ("Counter target spell if its mana value is X") and in the have-,
  control- and there-are clauses with the emphatic ("if you have exactly 13
  life", 18 occurrences over 16 cards). The old decline had measured the
  PHRASE "is equal to", which really is four unwritable lines, and
  concluded about the RELATION. Both readers agree, so one row serves both.
  ALL FIVE SHOALS ARE BENCHED WHOLE (Shining, Disrupting, Blazing,
  Sickening, Nourishing), Disrupting Shoal writing the cell at both readers
  in one card. RESIDUE, counted: the **11 SUMMED BOUNDS** (below, its own
  entry); the **CHOSEN-NUMBER read** (Sanctum Prelate, Talion — a missing
  `Amount`, not this relation; see that entry, whose second blocker this
  chapter cleared); and the two exiled-with-X carriers this round did not
  probe (Ashiok, Nightmare Weaver — a loyalty ability and a type-addition
  rider; Bronzebeak Foragers — an until-leaves exile and a per-opponent
  distributive), both routed forward untested rather than claimed.
- **The SUMMED BOUND**, 12 supported lines and a corrected measurement
  (finding 996): `comparableBound`'s docstring listed the sum among the
  things that "appear as bounds ZERO times", and it is written eleven
  times in the description frame ("search your library for a creature card
  with mana value equal to 1 plus the sacrificed creature's mana value" —
  Birthing Pod, Neoform, Prime Speaker Vannifar, Oswald Fiddlebender,
  Enigmatic Incarnation, Iron Man Titan of Innovation, Repurposing Bay,
  Synthesis Pod, Vivien on the Hunt, Pyre of Heroes, In Search of
  Greatness) and once in the condition frame (Obscura Ascendancy). All
  twelve stand at the EQUALITY and none at any other relation. Chapter one
  hundred twenty-four left the refusal standing rather than widening the
  bound column inside a round about the relation column; a round that takes
  it should read the other four relations against a sum before deciding
  whether `writtenBound` grows or the sum gets its own admission. The
  SCALED product and the outcome read are still measured zeros and
  `badScaledBound`/`badScaledConditionBound` still refuse them.
- **X ON A COMPARISON'S LEFT**, 4 supported lines, and a pin retired to
  open it (finding 997): MULTIPLE CHOICE writes "If X is 1", "If X is 2",
  "If X is 3" and "If X is 4 or more", which is `CompareAmt XVal` against a
  written bound at both the equality and the ranged relation. `readAmount`
  refuses `XVal` on the left, so none of the four is writable;
  `badCompareXSubject` had pinned the fourth as a measured zero and is
  gone. `badCompareLiteralSubject` ("if 3 is 4 or greater") is untouched
  and still correct. Whoever takes this should measure `ReadAmount`'s other
  readers before widening it — the test is shared, and the reason it exists
  is that a numeral on the left states arithmetic rather than a fact about
  the game, which is true of a literal and NOT true of an announced X.
- **THE CONDITIONED CLAUSE'S OWN TARGET**, read back by the next sentence:
  SAVAGE SWIPE writes "Target creature you control gets +2/+2 until end of
  turn if its power is 2. Then it fights target creature you don't
  control." Its first sentence is benched (chapter one hundred
  twenty-four); the second is refused because `If` makes its clause a HOLE
  outward (`effIntro`) and `It` finds nothing. The refusal is right for
  `badConditionAntecedent`'s case, where the CONDITION introduces the
  phrase and may have been false — but a TARGET announced by the clause is
  chosen at cast and survives the conditional either way. One card found;
  the population is unmeasured and a round that takes it should measure it
  first.
- **The FREE-CAST family**, 296 non-reminder lines and measured here for
  the first time: "you may cast that card without paying its mana cost"
  (Aetherworks Marvel, Chancellor of the Spires, Breaching Dragonstorm) —
  a licence to cast a DIFFERENT or later card for nothing, where the
  alternative cost substitutes a payment for the spell being cast. The
  whole "without paying … mana cost" surface is 547 lines, of which 235 are
  keyword reminder text (Suspend 64, Cascade 46, Plot 37, Rebound 37,
  Discover 25, Cipher 15, Paradigm 5) and 16 are [CR#118.9]'s own second
  phrasing on the SELF, landed this chapter. The remainder is this entry.
- **The KEYWORD ALTERNATIVE COST**, 956 lines across 25 keywords and 955 of
  them reminder text — "you may cast this spell for its [keyword] cost [if
  …]" (Flashback 213, Morph 150, Madness 61, "for its mana cost" 56,
  Foretell 51, Bestow 41, Disguise 38, Escape 34, Mutate 34, Warp 32,
  Megamorph 31, Evoke 28, Overload 27, Disturb 24, Dash 22, Miracle 20,
  Blitz 17, Freerunning 13, Cleave 12, Harmonize 12, Prowl 11, Spectacle
  11, SURGE 11, Impending 5, Mayhem and Web-slinging 1 each). Reminder text
  is not a card's own line, so none of it is bench-payable on its own — but
  the KEYWORDS are catalog rows and their costs are what FALL OF THE TITANS
  and its ten Surge siblings actually want. Surge, Prowl and Freerunning
  share one template ("for its <keyword> cost if <you-did-X-this-turn>");
  the rest carry their own condition and zone templates. Nothing here is
  scheduled until a keyword-cost round is.
- **The excess-damage redirect and the redirect-immunity pair** — recorded at
  their counts, both small and both blocked on the damage event as a mention.
  Excess: 5 sentences over 5 cards ("Excess damage is dealt to that creature's
  controller instead" — Flame Spill, Gandalf's Sanction, Pigment Storm,
  Ravenous Tyrannosaurus, Ram Through), whose destination is `Redirects`'
  destination and whose SUBJECT reads a PORTION of a damage event the card's
  own previous sentence described. Immunity: 2 sentences over 2 cards,
  VERIFIED (Lava Burst and Whippoorwill — not Arrow Storm, which the recon
  named and which writes the plain can't-be-prevented), a conjoined negation
  over `CantPrevent` and `Redirects` together. Under finding 85's bar apiece.
- **The other "instead" bodies** — 16 sentences that replace damage with
  something that is not damage: "put that many delay counters on this
  enchantment instead" (Force Bubble, Delaying Shield), "that player exiles
  that many cards from the top of their library" (Crumbling Sanctuary), "put
  that many -1/-1 counters on that creature" (Soul-Scar Mage), "sacrifice
  that many permanents" (Dralnu, Lich Lord). These ARE `Intercepts` — a
  general `Effect` body is exactly right for them — and every one is blocked
  on the same readback, the body's "that many" measuring the replaced damage.
  Listed so the amount round knows its full audience.
- **The loyalty ACTIVATION as an event** — "Whenever you activate a loyalty
  ability of enchanted planeswalker", and the Talent cycle's remaining
  blocker. SIX supported cards, not the two the chapter-eighty-nine brief
  carried (Chandra's Regulator, Keral Keep Disciples, Elspeth's Talent,
  Rowan's Talent, Repeated Reverberation, and The Chain Veil's negative form
  "if you didn't activate a loyalty ability of a planeswalker this turn"),
  which puts it over finding 85's bar. No `GameEvent` row watches an
  ACTIVATION at all — the event vocabulary watches things happening to
  objects and players, and this watches a player using an ability — so it is
  a row and not a cell. Elspeth's and Rowan's Talents are its whole-card
  witnesses and both are one gap from landing.
- **The ability-sharing static** — "Each other planeswalker you control has the
  loyalty abilities of Kasmina" (Kasmina, Enigma Sage) and "Nicol Bolas has all
  loyalty abilities of all other planeswalkers on the battlefield" (Nicol
  Bolas, Dragon-God). Two cards, and a `Gains` whose payload is named BY
  DESCRIPTION rather than quoted — which is chapter eighty-seven's ledgered
  by-name direction ([CR#201.5a], "the grantor named from inside the
  quotation") meeting the loyalty frame from the other side. Whichever round
  takes ability-as-a-value should take both together; neither is writable
  alone.
- **The battle's second half** — the Battle row landed and Invasion of
  Dominaria pays for it, but only its FRONT FACE and only because the Siege
  reminder is not a line. What no battle card can write yet is the DEFENSE box
  (a printed part the container carries no field for, finding 639's deliberate
  non-purchase, and the loyalty box's twin) and the transform back face. All 36
  supported battle cards are transforming Sieges, so the two questions arrive
  together whenever either is scheduled — and the defense number is the same
  shape as loyalty, which now means the same shape as a printed box chapter
  eighty-nine also declined to buy (the planeswalker's; finding 664). Neither
  box is language and neither is blocked by anything but that; the
  starting-loyalty entry below is where the loyalty half turns into language,
  and the battle's has no such line.
- **The mana exception, all that is left of the ability loss's variants** —
  "lose all abilities except mana abilities" is Blood Sun alone, and it is one
  line with no neighbours: an exception NAMES A CLASS where chapter
  eighty-five's "other" named a position (finding 630, the scope word being a
  derived spelling and no slot at all). One line is under finding 85's bar for
  minting a row, so this waits for a second card rather than for a design.
  Nothing else is owed here — the eleven "all other" lines are spelled by the
  coordination's part order and Darksteel Mutation is benched whole.
- **The LOYALTY SYMBOL outside a cost** — the two positions chapter
  eighty-nine's frame does not reach, two cards between them and both under
  finding 85's bar on their own. Carth the Lion writes the symbol as an
  ADDITIONAL COST a static confers ("Planeswalkers' loyalty abilities you
  activate cost an additional [+1] to activate") — [CR#606.5]'s own example
  card, and a cost MODIFICATION rather than a cost, which is the
  total-cost pipeline [CR#601.2f] this vocabulary ledgers whole. Comet,
  Stellar Pup writes it as an EFFECT, four times, inside die-roll outcome
  clauses ("1 or 2 — [+2], then create two 1/1 green Squirrel creature
  tokens"). The two shapes want different things — a cost-shift vocabulary
  and a counter clause — and neither has a second card, so this waits for
  company rather than for a design.
- **PAIRWISE DISTINCTNESS — "with different names"** — 24 supported lines and
  no row, turned up by chapter eighty-nine's witness sweep and unlogged
  anywhere before it ("Search your library for up to three artifact cards with
  different names", Saheeli Rai; "Return any number of permanent cards with
  different names from your graveyard to the battlefield", Eerie Ultimatum;
  Deathbellow War Cry; Behold the Sinister Six!; Begin the Invasion;
  Ecological Appreciation). It is NOT a `Predicate`, which is why nothing here
  spells it: every predicate this vocabulary has is a test on one member, and
  this is a constraint on the GROUP — no member may share a name with
  another — so it rides the counted mention rather than its description.
  `DistinctDisjuncts` is the nearest existing shape and asks a different
  question (two written alternatives, not n chosen members). Saheeli Rai is
  its named witness and the only reason that card is not the bench's second
  planeswalker. CHAPTER ONE HUNDRED TEN CHECKED WHETHER THE NAME MATCH
  REACHED IT AND IT DOES NOT, in either direction: the count is re-derived
  unchanged at 24, and `NameSource`'s three sources all answer "does THIS
  object have that name", where this asks whether no two of n share one.
  The rule says the same thing in the same shape — [CR#201.2b], "those
  objects have different names only if each of them has at least one name
  and no two objects in that group have a name in common" — a predicate
  over a group, stated as one. Nothing was bought; the rule is the delta.
  CHAPTER ONE HUNDRED ELEVEN ADDS THE ENTRY'S POSITIVE POLE, found while
  measuring the co-referential name and belonging here rather than there
  (finding 872): the SAME-name group constraint is **6 supported lines** —
  "two or more nonland, nontoken permanents with the same name as one
  another" (Chrome Replicator), "eight or more artifacts with the same name
  as one another" (Mechanized Production), and four that write no relatum at
  all ("three or more lands with the same name", Endless Atlas and Sceptre of
  Eternal Glory; "two nonland cards with the same name", Sphinx of the
  Chimes; Tainted Pact's "you exile two cards with the same name"). Same
  shape, opposite polarity, and the same reason neither is a `Predicate`: the
  constraint is on the group. Thirty lines between the two poles, one
  construction, and `badGroupNameRelatum` is the pin that keeps the
  co-referential row from being mistaken for it.
- **The STARTING-LOYALTY box** — finding 639's deliberate non-purchase,
  measured where it actually surfaces as language. The printed box is a card
  part with no field, as the defense box is; what makes this an entry is the
  two supported lines that SET it on a token copy — "except it's not legendary
  and its starting loyalty is 1" (Jace, Mirror Mage) and "The copy isn't
  legendary and has starting loyalty X" (Ob Nixilis, the Adversary). Both ride
  a copy clause, and chapter ninety-two confirmed both at one line apiece
  while landing the exception list they would ride — `CopyExcept` has the
  legendary refusal and not the loyalty one. So the entry now belongs with the
  exception kinds above; recorded here so neither the loyalty frame's chapter
  nor the copy's is searched for it.
- **The one-time boon** — "You get a one-time boon with '…'", the family the
  chapter-eighty-seven sweep turned up unlogged anywhere. MEASURED AT ITS ZERO
  before anything else: 27 cards write it and NONE of them is supported. Every
  one is Alchemy ("perpetually", "seek a nonland card"), which is the
  `supported` flag doing exactly what chapter eighty-six's six refused card
  types showed it doing. Recorded so it is not re-discovered, and DECLINED
  until the flag says otherwise — not work. Its shapes, for whoever inherits a
  changed flag: one-time, two-time, three-time and bare (Merfolk Tunnel-Guide,
  Tasteful Offering, Swiftspear's Teachings, Flaming Fist Duskguard).
- **The static's turn window** — "During your turn, this creature has first
  strike", the gap that took RESTLESS SPIRE off chapter eighty-seven's witness
  list after the widening had made its carrier writable. A statement carries no
  window slot: `windowOk`'s grid is the activated ability's timing and
  `TriggerWindow`'s is the trigger's, and the third reader has never been
  asked. Restless Spire is its named witness (its mana ability elides as every
  animated land's does); "During your turn, outlaws you control have first
  strike" (At Knifepoint) is the same word on a non-animated line.
- **The asymmetric definition** — "Tarmogoyf's power is equal to the number of
  card types among cards in all graveyards and its toughness is equal to that
  number plus 1" (12 supported lines, the whole Lhurgoyf family plus Souls of
  the Lost and Consuming Blob). Two definitions in one sentence, and the
  second reads the FIRST's computed value back: "that number" is an anaphor
  over a sibling slot's amount, which is not `ThatMuch` (no clause did
  anything) and not `TheDifference` (no comparison held). Chapter
  sixty-seven's row writes each half separately and the corpus never does.
  Its printed box is the OFFSET star at the toughness slot ("*/1+*"), which
  `PrintedStat` already spells; what is missing is the anaphor and, for
  Cephalopod Sentry, the subtracted star ("7-*"). Most of the family is
  double-blocked besides: eight of the twelve count CARD TYPES among cards
  rather than cards ("the number of card types among cards in all
  graveyards"), which is the domain-style distinct-kind count `CountOf` does
  not spell, and five of those write the all-graveyards possessor as well.
- **Partial-cause outcome immunity** — "You don't lose the game for having 0 or
  less life" (9 lines, Phyrexian Unlife's family): carves out ONE state-based
  cause ([CR#104.3b,104.3c,104.3d]) and leaves [CR#104.3e] intact, so it is not
  chapter fifty-two's gate but a narrower thing. Wants an SBA-cause vocabulary
  the grammar has none of.
- **The rider's blocked right sides** — chapter fifty-nine's binder takes any
  amount that is not already written, so what its 1,118 cards still cannot
  define X as is exactly what the `Amount` vocabulary lacks, with counts:
  the superlative 69 (chapter sixty-two landed the AMOUNT half of these and
  chapter sixty-eight the modifier half; what is left of them rides the missing
  AXES entry below), the "amount of"
  magnitude read 58 (core's `EventSum`; the unnumbered form landed in chapter
  forty-two and this is its numeric twin), devotion 17 (its own entry), die
  results 12, half 6, excess damage 5. Nothing here is a rider problem; each
  belongs to the row it names. "Half … rounded down/up" is now the ONLY thing
  standing between Aspect of Wolf and the second letter, both of its halves
  being that phrase — and with the static frame built (chapter sixty-one),
  Aspect of Wolf is the letter Y's nearest witness of the three.
- **The cost statement's residues** — chapter sixty-three landed the row
  (`CostsToCast`, `StaticKind.CostModification`) and with it the 334 self lines
  and the 18 unqualified class lines. Four pieces stayed out, each small and
  each its own thing:
  - The CONDITIONAL self-reduction, 127 lines ("This spell costs {1} less to
    cast **if** you control a Wizard", Academy Journeymage). The wrapper
    already composes — the 3 "as long as" lines land free (probed) — so this
    is a MARKING WORD, and chapter forty-nine measured `CondMarking` as the
    closed pair "as long as"/"unless". Before adding a third, settle whether
    it is the same construction at all: [CR#601.2f] determines a total cost
    once and locks it in, where [CR#611.3a] applies a static ability's effect
    "at any given moment", so the "if" may be a one-time test rather than a
    standing one.
  - The COLORED payload, ~25 lines ("costs {2}{R} more to cast"; Strive's
    "costs {R} more to cast for each target beyond the first"). The magnitude
    slot holds an `Amount` — the generic component — because the X-rider
    family needs a letter to read there (finding 445); a colored run is a
    second payload shape, not a widening of this one.
  - The ACTIVATION variant, 45 self lines + the class lines. [CR#602.2b]
    extends the whole cost machine to activation costs in one sentence, so
    the rules cost nothing and the blocker is the SUBJECT: "This ability
    costs {1} less to activate" needs a noun for an ABILITY, and every `Noun`
    in this grammar is an object or a player. A referent sort, not an
    extension. Agatha of the Vile Cauldron additionally writes a floor rider
    ("This effect can't reduce the mana in that cost to less than one mana").
  - The DURATIONAL cost line, 1 ("Spells with the chosen name cost {1} less
    to cast this turn", Cheering Fanatic — its chosen-name subject is
    spelled since chapter one hundred ten, so the span cell below is now
    the card's ONLY blocker). `admitsSpan CostModification` is False in all ten cells;
    flipping the cell would rename a `SpanUse` row.
  Abaddon the Despoiler's neighbour line ("spells you cast … have cascade") is
  not this entry's: chapter sixty-six landed the grant row, and what that line
  still waits on is cascade — one of the nine words core cannot resolve (see
  the keyword grant's residues below) — plus the from-zone qualifier above and
  a duration.
- **Cross-ability memory** — "the exiled creature card's power" (Phyrexian
  Ingester's static line reading the card its ETB imprint trigger exiled;
  Drach'Nyen wants the same). The pieces for a SAME-ability version exist —
  `ExiledWith` links a card to its source, `TheVerbed Exile` reads a verbed
  mention back definitely — and both are discourse-local by construction,
  the discourse being one ability wide. Core has nothing either: no imprint
  primitive, and `Count::Noted(Ident)` is a number read back from a slot
  ([CR#607.2] linked values) rather than an object reference. A both-layer
  gap, and the last thing standing between Phyrexian Ingester and the letter
  Y. Neighbour to the pronoun-preference entry below (both are about what a
  later sentence may point at), but a different mechanism: that one is
  ambiguity inside one ability, this one is reference across two.
- **LETTER-VALUED keyword parameters** — "Ulamog has annihilator X, where X
  is the number of +1/+1 counters on it", "mobilize X" (Avenger of the Fallen,
  Infantry Shield), "Monstrosity X" (2 cards). What this entry was about is
  now DONE twice over: chapter seventy-three built the parameter slot and
  chapter seventy-four landed the NUMBER shape on Renown, whose payload is an
  ordinary `Amount`. So a letter-valued parameter is `ParamNumber
  (DefinedLetter LetterX)` and the only question left is the WHERE-clause
  binding it — the static rider is ready for it, and [CR#701.37c] is the rule
  that makes the monstrosity case a linked value rather than a fresh read.
- **Subject preference for the bare pronoun** — chapter forty-eight minted the
  self-subject mention and finding 349 recorded that English resolves a bare
  "it" by preferring the subject where this grammar refuses two object
  mentions outright. That was an over-generation then; chapter sixty made it a
  BLOCKER: Bioplasm's "exile the top card of your library. If it's a creature
  card, this creature gets +X/+Y, where X is the exiled creature card's power
  and Y is its toughness" has the trigger's attacking subject and the exiled
  card both in scope. Wants a measured preference rule; the definite
  description it would otherwise use exists (`TheVerbed Exile`, chapter
  sixty-one's finding 433), so the two halves of that card sit in two
  different entries — this one and cross-ability memory above.
- **The "difference between" surface** — "the difference between that
  creature's power and its toughness" (Jaws of Defeat, Lady Loki; 11 lines, 6
  supported, the other 5 dice Contraptions). NOT the row chapter fifty-eight
  landed: plain English "the difference between 2 and 5" is three, so a
  directional floored `Minus` mis-models it whenever the stated order puts the
  smaller first, and all six supported lines read symmetrically. Wants an
  absolute difference of its own (or `Max` over the two directional ones).
  Two of the six are blocked elsewhere, on the life exchange; the other four
  wait on this surface alone now that the layer words have landed.
  Two of those were mis-filed here for two rounds and the correction is worth
  keeping: "Doran, Besieged by Time and Spry and Mighty" is TWO cards and not
  three, and neither sets a base power and toughness — Doran, Besieged by Time
  writes a ±pump under a rider ("it gets +X/+X until end of turn, where X is
  the difference between its power and toughness") and Spry and Mighty writes
  the same shape over a chosen pair, so the difference IS their blocker rather
  than a neighbour of it.
- **The gap off the other licensors** — chapter fifty-eight's anaphor reads a
  comparison in the containers that type the leading order. Three lines want it
  under `Effect.If` (Balance of Power, Vraska's [−9], Iymrith's second
  sentence) and wait on the fronted-conditional orientation below; three want
  it off a CHOICE clause ("choose an opponent who controls more lands than
  you …", Boreas Charger, Sandstone Oracle, Slithermuse) and two off a
  superlative one (Tales of the Ancestors, Scholarship Sponsor — chapter
  sixty-eight landed the superlative and neither card moved: both fold cards in
  hand, an axis this catalog does not carry).
- **The devotion read** — "your devotion to black" (61 supported lines: 24 the
  strict comparison "as long as your devotion to black is less than five" that
  gates a God's creature-ness, 5 a P/T definition, and 32 the read in an
  ordinary amount slot). A count over mana symbols among the permanents a
  player controls; the comparison frame around it has been writable since
  chapter fifty-seven and the definition frame since chapter sixty-seven — the
  five that write it there (Anax, Callaphe, Daxos, Renata, Tymaret) are single-
  slot definitions whose printed box is already spellable — so the read is the
  whole of what is missing at three frames now. [CR#700.5] defines it as a count of MANA SYMBOLS among the mana
  costs of the permanents a player controls, which is not `CountOf`'s shape —
  the domain is symbols inside costs and not a set of objects.
- **The equality comparison** — `Comparator` has four relations and not core's
  five, because "is equal to" as a COMPARISON is four supported lines and every
  one is blocked twice over (a die result, unspent mana, a sum). Small, and it
  should ride whichever round makes one of those four writable rather than
  being scheduled for itself.
- **The announcing comparison operand** — "power greater than target creature's
  power" (Fell the Mighty). The postnominal frame threads its bound's
  announcement since chapter fifty-seven (`amtDelta`), but the CONDITION frame
  introduces nothing, so an announcing subject or bound written there is
  dropped silently — a hole the comparison's SUBJECT has carried since chapter
  forty-four and the bound now shares. The fix is one bindingless test over
  amounts at both slots, and both slots have to be answered together.
- **The player-headed comparison predicate** — "target opponent who has more
  life than you do" (Keeper of the Flame, Keeper of the Light, Oath of Mages,
  Namor). Sharpened by chapter sixty-eight, which is a player-kind qualifier and
  is NOT this one: `Superlative` bounds a player's number against a fold of the
  player's own SET, and these lines bound it against an independent standard
  written beside it, which is `Compare`'s question and `Compare` is
  `Predicate bs Object` over a `Characteristic`. So the gap is exactly the
  player-sorted half of that row — the condition frame (`CompareAmt` over
  `PlayerStatOf`) says the same thing about a player and the DESCRIPTION frame
  still cannot. The Keepers additionally want the as-you-activate targeting
  restriction, which no vocabulary here has.
- **The negated existential over players** — "if no opponent has more life than
  that player" (5 supported lines, the Commander-creature grant family). A
  quantified player SUBJECT under a negation, which is neither `PlayerGroup`'s
  plural read nor `NotCond`'s negation of a whole condition.
- **The point-in-time SNAPSHOT rider** — "life equal to that player's life
  total AS THE TURN BEGAN" (Sengir, the Dark Baron), and the shape of any
  read taken at a past moment rather than now. Absent from BOTH layers (core's
  `Noted` and `EventSum` are different mechanisms), and it is the only thing
  between Sengir and a bench line — the loss trigger and the life read both
  landed in chapters fifty-five and fifty-six.
- **The life EXCHANGE** — "exchange life totals with target opponent" (13
  lines, [CR#701.12c,701.12g] defining). A two-player effect whose operand is
  the SET this grammar already has (`LifeOp.Set`; finding 121 read the rule
  that way), so what is missing is the two-participant clause and not the
  arithmetic.
- **The player-set count** — "for each player who has lost the game" (Rampant
  Frogantua) and "if two or more players have lost the game" (Hot Pursuit): a
  count over PLAYERS in a standing state, which chapter fifty-five measured and
  found is not the lookback query it looks like. Two supported lines; small,
  and it is the only reader those lines want.
- **Coordination under one subject** — "players can't lose the game or win the
  game this turn" (Everybody Lives!, and Platinum Persecutor without the span):
  two gate KINDS sharing one subject, which the one-clause-one-gate row
  deliberately cannot say (finding 374). It is the same shape as chapter
  thirty-eight's coordinated EVENT and chapter forty-nine's coordinated DEED
  ("can't attack or block", 6 lines) at a third site, so it is worth designing
  once across all three rather than per family.
- **The CONDITION-FIRST conditional** — all that is left of the explicit TIE
  sentence, and chapter seventy-seven narrowed it to one mechanism. The
  family's condition has been writable since chapter sixty-eight and its draw
  consequent landed with Celestial Convergence in chapter sixty-nine; its
  chooser slot landed in chapter seventy-seven. What blocks the seven "you
  choose one of them" cards (Drop of Honey, Porphyry Nodes, Purging Scythe,
  Desecrator Hag, Loxodon Peacekeeper, Juxtapose, Tariff) is that `Effect.If`
  types its condition at `preIntro e` — the condition reads the EFFECT's
  phrase, never the reverse — so a condition cannot announce the group its own
  consequent reads back. The two carriers that DO thread `condIntro` forward
  are the static `Conditionally` and a trigger's intervening clause, and
  neither is this sentence. DESIGN ALREADY DONE, do not re-derive it: the
  group the tie condition announces is the counted domain's own denotation,
  minted exactly when that domain `uniquifies` (picks an extreme), which is
  measured 7/7 — every line that reads "them" back off a count comparison is
  a tie sentence and no ordinary threshold does. Chapter seventy-seven wrote
  that `condIntro` clause, compiled it, and backed it out for want of a
  consumer; it goes back in on the round that mints the carrier. The two
  remaining consequents are Psychic Battle's (the targets are unchanged) and
  Timesifter's (the tied players repeat the process), each one card.
- **The COORDINATED-DESTINATION move** — "Put that card into your graveyard
  and the rest into your hand" (Murmurs from Beyond, the one elision that
  card's bench line names). One verb, two phrases, two destinations, joined by
  "and" rather than by a sequence word. The grammar writes it as two `Move`
  clauses under `Sequentially`, which says the same thing in the vocabulary it
  has and spells "then" where the card spells "and". Worth a measurement
  before a row: the coordination may be general across verbs rather than the
  move's own.
- **The quantity-sensitive CHOICE cell** — `choosable (CountedGroup _ _)` is
  False and quantity-blind, and chapter seventy-six set it that way on "choose
  one or more X" being zero (which it still is). Two cards show the cell
  over-refusing: "Choose up to one creature. Destroy the rest." (Duneblast)
  and "Choose any number of creatures. They block this turn if able."
  (Berserker's Frenzy). The fix reads the quantity — the "N or more" form is
  the unwritten one — but it wants the full quantity-by-attestation grid
  across BOTH choice tables (`choosable` and chapter seventy-seven's
  `agentChoosable`), which is the measurement the round owes.
  `badChooseCountedGroup` is unaffected: its own line writes `atLeast 1`.
- **The DEFINITE under a control test** — "draw a card if you control the
  creature with the greatest power or tied for the greatest power" (6 supported
  lines: Abzan Beastmaster, Padeem, Consul of Innovation, Summon: Fenrir,
  Thickest in the Thicket, Triumph of Cruelty, Triumph of Ferocity). Chapter
  sixty-eight's `Definite` announces its referent, and `Matches` demands a
  `Bindingless` subject because a condition introduces nothing — so the phrase
  can only be written there by losing the mention silently, and the refusal is
  the honest form of that (probed). `Exists` is the wrong frame in the other
  direction: it IS the indefinite. What the corpus writes is a control test over
  a definite description, and neither condition row says it.
- **The distributive subject read back** — "Each opponent sacrifices a creature
  with the greatest power among creatures THAT PLAYER controls" (16 supported
  modifier lines, plus whatever else writes the shape). `ControlledBy (That
  PlayerW)` under an `Each` agent does not elaborate, and the superlative is not
  why (checked with the modifier removed) — the demonstrative reads a
  DISTRIBUTED mention, which is a different question from the singular reads
  `That` was built for. It is what costs Crackling Doom, Soul Shatter's family
  and Summon: Valefor; the singular twin ("target player sacrifices … creatures
  THEY control") is writable and benched (Consume).
- **DETHRONE's remaining two cards** — chapter seventy-seven CLOSED this
  entry's premise and landed one of its three cards. The phrase was never the
  explicit tie sentence: "the player with the most life or tied for most life"
  is chapter sixty-eight's FOLDED clause, which states the superlative's
  denotation, and with chapter seventy-six's defender slot Undercover Butler
  is benched whole. Seraphic Greatsword, a fourth card the entry never named,
  writes the same phrase off an equipped creature. The two that remain are
  blocked on things of their own: PREACHER OF THE SCHISM on its third line's
  header-internal "while" clause (the ledgered family), and SCOURGE OF THE
  THRONE on three at once — the dethrone KEYWORD as a printed line, the
  "for the first time each turn" rider and the additional combat phase.
- **The extremal modifier's missing AXES** — 10 supported modifier lines fold an
  axis `ProjAxis` does not carry: votes 4 ("each permanent with the most votes",
  the will-of-the-council family), the colour share 3 ("shares a color with the
  most common color among all permanents" — a fold over a QUALITY, which is a
  third sort beside the object and the player), cards in hand 2 (Adamaro, Tales
  of the Ancestors — declined as a `PlayerStat` in chapter fifty-six on the
  measurement that the corpus counts cards rather than reading a hand size) and
  a noted number 1 (Menacing Ogre, chapter fifty-nine's cross-ability memory).
  Each rides its own family; none is an extremal problem.
- **The fold over a MENTION** — chapter sixty-two's fold takes a DESCRIPTION,
  `CountOf`'s slot and core's shared `Countable`. The corpus's other complement
  is a group already mentioned: "the total power of the sacrificed creatures"
  (Soulblast, Corpse Cobble), "the greatest power among them", "the total mana
  value of those cards", and the possessive "their total toughness"
  (Dracoplasm, Sutured Ghoul) — 26 lines, 4 extremal and 15 sum and 7
  possessive. `ThoseVerbed`/`Them`/`Those` all exist as nouns, so this is a
  second slot shape rather than new machinery, and it is what would finally pay
  `StatOf`'s own comment (which has named Soulblast since the singular gate was
  written).
- **The relativized per-element count** — "the greatest number of creatures a
  player controls" (10 supported cards: Investigator's Journal, Jace's
  Archivist, Cavern-Hoard Dragon, Thought Sponge, Windfall's cycle). A fold
  whose per-element read is a COUNT relativized to the member, spelled with a
  relative clause instead of "among". This is the general projection both prior
  arts have (core's `Projection { of, by }` over a bound `It`; the legacy
  module's `Project`/`bindIt`) and it is what chapter sixty-two's closed
  `ProjAxis` deliberately does not buy — because the element these lines bind
  sits INSIDE the domain's own description, so a general axis slot alone lands
  none of them. The real ask is an element binder reaching into a predicate.
- **The counter axis on the fold** — "the number of counters among creatures
  you control" (6 lines). Four write the KIND-BLIND quantifier (its own entry)
  and one names a kind outside the catalog ("time counters"), so a `ProjAxis`
  row for `CountersOn` would land one line (Vault 12's rad counters, a Saga
  chapter). Rides the kind-blind counter entry rather than the fold.
- **The cast relation's residues** — chapter sixty-four landed the row
  (`CastBy`, a predicate beside `ControlledBy` rather than a verb on it,
  seeding the stack) and the controller relation's two-zone domain
  (`zoneAdmit`, the second admissibility table), and chapter sixty-five took
  the union head off this list — it was never a construction, only `Or`'s
  mis-documented coordinator, so Goblin Electromancer, Arcane Melee, Mana
  Matrix and Aura of Silence all landed there. 97 of the 99 plain
  cost-modification lines are structurally writable now (the 2 face-down
  heads being the remainder). Three pieces stayed out, each its own thing:
  - The TARGETING relative clause, 19 lines ("Spells your opponents cast that
    target this creature cost {2} more to cast", Icefall Regent and 8
    siblings; 4 more write "spells you cast that target …"). No targeting
    predicate exists AT ALL — nothing in `Predicate` describes an object by
    what it targets — so this is a vocabulary row and not a composition. Its
    complement is an ordinary noun; the relation is what is missing.
  - ~~The CAST-FROM-ZONE origin qualifier~~ — **CLOSED by chapter one hundred
    thirty-three** (`Predicate.CastFrom`), which hung it exactly where this
    entry said to. The count here was RIGHT for what it scoped: 24 was this
    frame — the cost-modification and keyword-grant statics — and the frame
    is 29 today, grown by recent prints (Aven Interrupter, Emet-Selch,
    Zhulodok, Quandrix). It was never the family, which is 185 lines over
    five frames; a recon that read the 24 as a family-wide undercount had it
    backwards. Doc Aurlock still does not land, on the zone DISJUNCTION
    ("from your graveyard or from exile") — a coordination of two origin
    clauses, which is the standing coordination gap and not this row's;
    PATRICIAN GEIST is benched as the frame's single-zone witness instead.
  - The ORDINAL cast, 5 lines by the phrase "the first spell you cast each
    turn" (Maelstrom Nexus, Rain of Riches, Wild-Magic Sorcerer, The Twelfth
    Doctor, Zimone) plus 20 more writing "your first spell" in a trigger
    header. An ELEMENT BINDER over a turn's casts — the same shape the
    relativized per-element count wants — and the per-turn reset is a second
    unbuilt thing beside it.
  Measurement note, recorded rather than entered: the AGENTIVE negated cast
  ("a spell you didn't cast") is ZERO lines, so `negatable (CastBy _)` is
  False — but what the phrase would mean is attested through an agentless
  passive one card over ("target spell you control that wasn't cast", Errant,
  Street Artist), which is why the cell is refused and not pinned. A round
  that mints the passive should re-measure the cell rather than assume it.
- **The coordinator's remaining forms** — chapter sixty-five derived the
  word this row already writes ("or" under a singular determiner or negative
  polarity, "and" under an affirmative plural class head) and left three
  coordinations it does NOT cover:
  - **"and/or" is a THIRD coordinator**, not a spelling of this one: either
    category alone or both together qualify, which is a different truth
    condition and not an environment. 365 supported lines write it outside
    reminder text ("the number of tapped artifacts and/or creatures you
    control", "search your graveyard, hand and/or library"), and the parser
    keeps `Conjunction::AndOr` beside `And` and `Or` already. The big
    surfaces are a counted domain and a multi-zone search, so it wants the
    rows it coordinates as much as a constructor.
  - **The KIND-crossing disjunction** — the kind-union entry wearing a
    coordinator; the kind index forces alternatives to describe one sort and
    that is the gate, not a spelling. NOT the player/object crossing, which
    chapter ninety-nine landed as `Predicate.KindJoin`: these cross a card
    TYPE with a SUBTYPE at one `Kind`, so the union head's closed two-half
    vocabulary has nothing to say about them. SUGAR COAT is its precisely-known whole
    card and has been down to this one thing since chapter eighty-seven: its
    flash line is a keyword row, its "loses all other card types and
    abilities" is a spelling (finding 630), and its quoted `{2}, {T},
    Sacrifice this artifact:` payload became writable when `TokenChars`
    started holding abilities. What is left is "Enchant creature or Food",
    which crosses a card TYPE with an artifact SUBTYPE — and which brings the
    `Food` subtype row with it (finding 631). IN TOO DEEP and MINIMUS
    CONTAINMENT are the same sentence with the same one blocker apiece
    ("creature, planeswalker, or Clue"; a mana ability inside the quotation).
  - **The CROSS-ZONE disjunction**, 17 lines under a shared preposition ("an
    Equipment card from your hand or graveyard"), which the single-valued
    zone projection refuses outright rather than mis-place
    (`badCrossZoneDisjunction`). A zone-SET projection is the same shape
    chapter sixty-four's `zoneAdmit` took for a relation's domain, so these
    two are neighbours if either is scheduled.
  The SUBTYPE union is not on this list: "Other Ninja and Rogue creatures you
  control get +1/+1" (Silver-Fur Master, the one supported line) composes
  today as `Or [HasSubtype …, HasSubtype …]` under the same derived "and",
  and what it waits on is two catalog words. Recorded as the head
  generalizing past card types.
- **The keyword grant's residues** — chapter sixty-six landed the row
  (`GrantSubject` coupling the granted keyword to the subject's place, and
  `keywordStackRegime` deciding which relation may describe a spell class)
  and with it convoke, improvise, storm and lifelink. What stayed out is one
  parameter gap and one CORE gap:
  - **AFFINITY's typed parameter**, 5 lines and every one of them
    parameterised — "affinity for artifacts" (Mycosynth Golem, Sami, Wildcat
    Captain, Tezzeret), "for Auras" (Pearl-Ear), "for creatures"
    (Witherbloom). The `Keyword` row has no slot, and the parameter is a
    described CLASS rather than a number, so this is a different shape from
    the numeric keyword parameters (annihilator X, mobilize X) that entry
    already carries — schedule them together, mint the slot once.
  - **The nine keywords core cannot resolve** — cascade, replicate, emerge,
    conspire, rebound, prowl, freerunning, demonstrate, sticker kicker. No
    macro under `plugins/builtin/macros/keyword/` and zero built-card use, so
    a row here would name a word the layer below has nothing for. This is a
    CORE-side entry that the queue carries honestly rather than a grammar
    round: The First Sliver's own printed cascade resolves to nothing today,
    so the card is unbuildable whatever this file says about its second line.
    Cast Through Time (rebound) and Flamekin Herald (cascade) wait with it.
  - **Flash**, 1 line and that line unsupported — deferred on evidence, not
    on cost; it is a cheap word whenever a supported line wants it. Chapter
    eighty-four's re-measurement narrows that to the GRANT: as a printed
    keyword LINE flash is 500 supported cards, and one of them is now a
    migrated witness waiting on the word alone (Amphibian Downpour, the
    Frogify sentence under Flash and Storm).
  The NEXT-SPELL and FIRST-SPELL grants are not this entry's: "the next spell
  you cast this turn has cascade" is [CR#611.2f]'s regime, a continuous effect
  that begins to apply when the player next puts an appropriate spell on the
  stack, and it wants the ordinal binder the cast residues entry names. The
  as-though-flash family is a play PERMISSION and not a grant at all.
- **The ownership relation as a description** — "cards your opponents own" (6
  supported lines, all of them naming cards in EXILE, where [CR#109.4] leaves
  no controller and [CR#108.3]'s owner is the only possessor left). This is the
  descriptive twin of chapter fifty-four's possessive-zone reader: that one
  widened `OwnedBy`, which writes "your opponents' graveyards", where this one
  needs an `OwnedBy` PREDICATE beside `ControlledBy`. Merge with the
  object-ownership entry below when either is scheduled — they are one row.
- **The plural RELATIONAL possessor** — "their owners' hands", "its controller's
  graveyard" pluralised: a possessive over a relational noun derived from a
  GROUP, which `ControllerOf`/`OwnerOf` cannot mint because they derive one
  referent from one object (`badGroupOwner`). Distinct from both entries above:
  the possessor is not a named set but a relation applied member-wise.
- **The conditional threshold over a possessor set** — "unless your opponents
  control eight or more lands" and its family (~9 lines, 2.3% of the "your
  opponents control" mass): an INDEPENDENT clause, not a relative one, so it
  needs a counted condition over a described set rather than a possessor. Rides
  the counted-condition entry above.
- **Life-gain suppression** — "Players can't gain life" (10 lines), "Your
  opponents can't gain life" (9), and three singular-subject lines: 22
  supported lines and no channel at all. Chapter fifty-three declined it as a
  witness rather than mint a suppression subsystem to carry one. It is not
  the outcome gate's shape (that gate is [CR#101.2] precedence over an
  outcome) and not `Prevents` (damage only); measure whether it generalises
  with the other "can't" statics — "Players can't draw cards", "Players can't
  search libraries" — before scheduling, since the family may be one row over
  a suppressed EVENT rather than several.
- **Object ownership as a relation** — a possessor axis distinct from
  `ControlledBy`, which exile makes necessary ([CR#109.4] leaves an exiled card
  no controller; [CR#108.3]'s owner is the only possessor left).
### Counter cluster

- **The COUNTER COUNT AS A BOUND, 11 lines** — chapter one hundred
  twenty-nine's one residue, and one cell over two scopes. At OBJECT scope
  five frames bound the number instead of testing existence ("Creatures you
  control with three or more +1/+1 counters on them have haste", RUNADI,
  BEHEMOTH CALLER; CHAMPION'S DRAKE, NILS DISCIPLINE ENFORCER, SHADOW
  URCHIN, SPARK RUPTURE); at PLAYER scope the corrupted keyword's six are
  the WHOLE of the player-scoped counter description ("each opponent who
  has three or more poison counters" — FEED THE INFECTION, GETH'S SUMMONS,
  GLISSA'S RETRIEVER, IXHEL SCION OF ATRAXA, PHYREXIAN ATLAS, WURMQUAKE),
  which is why `HasCounters` is fixed at Object: the bare player existence
  it could have spelled is zero lines, and [CR#122.1f] states the
  player-side test in the ranged shape too ("a player is 'poisoned' if they
  have one or more poison counters"). ONE BLOCKER, named: `Predicate.Compare`
  bounds a `Characteristic`, whose whole vocabulary is Power, Toughness and
  ManaValue, so a counter READ cannot stand on a comparison's left at this
  position — the description-side twin of chapter one hundred twenty-four's
  queued "X on a comparison's left" (4 lines), and probably one round with
  it. `CountersOn` already exists as the amount; what is missing is the
  comparison that takes one. Deliberately not reached by giving
  `HasCounters` a quantity slot, which would duplicate `Comparator` for
  eleven lines.
- **Kind-blind counter quantifiers, and moving a counter** — what chapter forty
  left of the counter cluster, its other two entries having landed there (the
  player holder and its two verbs, the count read, the last-removal trigger).
  Three families, all wanting one thing the `Amount` vocabulary has no term for,
  a quantity over KINDS rather than over a number: bare "a counter" (82 lines),
  ~~the presence read "with a counter on it" (26)~~ — **CLOSED by chapter one
  hundred twenty-nine, and this entry was the SECOND description of that cell:
  it is the kind-blind slice of the counter-bearing description filed above,
  46 frames and not 26, and the whole family 221. The kind-blind cell is
  `HasCounters Nothing` and needed no kind-quantifier at all, the `Maybe` of
  finding 1043 saying whether the sentence names a kind rather than how many
  kinds it means** — and "move a counter from … to
  …" (36), which additionally wants a two-holder transfer verb. The always-all
  player removal is NOT here — it landed as `LosesAllCounters`' unnamed cell,
  because that verb takes no number and its `Maybe` says whether a kind is
  named, not how many are taken.
- **The catalog's remaining tail** — chapter sixty-nine minted `Charge`, `Omen`
  and `Intervention` and restated the minting doctrine as two tests (finding
  481): the corpus must write the kind as a one-shot put or remove, and this
  grammar must be able to write that line. Eight kinds clear the FIRST half
  and wait only on a landable line — oil (28 puts / 29 removes), quest (33/10),
  level, spore, storage, ki, verse, depletion — so each enters the catalog on
  the round that benches one of its lines, and none needs a design decision
  first. LORE IS NO LONGER ONE OF THE CROSS-LINKED-OUT KINDS, and this entry
  was wrong about it in both halves: chapter one hundred thirteen measured
  twelve supported lines that write a lore counter as an ordinary one-shot put
  or remove with no chapter symbol near them (Keldon Warcaller, Myth Realized
  twice, Scroll of the Masters, Satsuki, Mind Unbound, Chong and Lily,
  Storyweave, both Clash of the Eikons modes, Sigurd's boast, Garnet — four of
  them removes, and two of the carriers not Sagas at all), so the kind cleared
  the first test on its own lines and `Lore` minted with Keldon Warcaller as a
  whole card. Charge's history exactly (finding 85's premise did not hold there
  either). ~~Two named kinds clear NEITHER half and stay cross-linked out: age
  (cumulative upkeep — all 85 of its lines are that keyword's reminder
  text)~~ **AGE MINTED, chapter one hundred thirty, and this line was wrong
  in lore's own half**: the corpus writes "age counter" 187 times and
  FIFTEEN are a card's own non-reminder line (all on keyword carriers), of
  which Cover of Winter's "{S}: Put an age counter on this enchantment" is a
  one-shot put — the first test, cleared, and benched. That leaves ONE kind
  cross-linked out: energy (the symbol family, finding 85's Ticket
  refusal). Loyalty and shield are [CR#122.1e]'s
  and [CR#122.1c]'s own mechanics and are not flat kinds at all.
- **The ascription's remaining SUBTYPE WORDS** — chapter seventy widened
  `AsType` to [CR#109.2]'s whole phrase ("a card type or subtype") and minted
  the rows two cards demanded, `Aura` and `Curse`, alongside the cell for the
  already-rowed `Equipment`. Seven measured words are still unrowed, and each
  grows its row on the round that benches a card writing it: Vehicle 166
  occurrences / 123 cards, Saga 92/80, Siege 37/36, Class 27/22, Spacecraft
  20/16, Case 13/9, Room 4/4. No design question remains — `ascribesAsSubtype`
  gets a True cell and `subtypeType` a card type, and the ascription itself is
  already general. SIEGE IS NO LONGER ONE OF THEM: chapter eighty-six minted
  the `Battle` card type its cross-link was waiting on ([CR#205.3q] making it
  the only battle type) and the word landed with it, paid for by Invasion of
  Dominaria — and SAGA IS NO LONGER ONE OF THEM EITHER: chapter one hundred
  thirteen landed the chapter-ability construction its cross-link was waiting
  on and the word came with it (92 own-line occurrences over 80 cards, against
  the 2 lines on a Saga that write "this enchantment" instead — Day of the
  Moon, Death in Heaven). So five words remain (Vehicle 166/123, Class 27/22,
  Spacecraft 20/16, Case 13/9, Room 4/4), NO CROSS-LINKS LEFT, and each is a
  `ascribesAsSubtype` cell plus a `subtypeType` cell on the round that benches
  a card writing it. Vehicle is the cheapest of the five and its cards want
  crew.
- **The self-antecedent pronoun after an ascription** — Soul Ransom's "This
  Aura's controller sacrifices IT, then draws two cards" does not land, and the
  block is exactly one thing: an unmoved ascription announces no mention, so
  `It` has no antecedent to read. The possessive itself was probed green in
  chapter seventy and needs nothing (`ControllerOf` over the ascription); the
  whole possessive family is five occurrences over four cards (Aura 1,
  Equipment 2, Vehicle 2). The question is whether a self-reference should
  announce a readable mention at all, which is chapter thirty's split between
  "this card" and "this creature" asked from the pronoun's side.
- **Linkage under a subtype word** — `SortedSelfLinked` covers the card-type
  ascription only, and that is measured: "cards exiled with this Saga" is six
  supported lines, "this Vehicle" three, "this Class" one, while the two rowed
  subtypes write it zero times apiece. The cell opens with those rows, not
  before them — AND THE SAGA ROW NOW EXISTS (chapter one hundred thirteen), so
  this is the first of the three whose word is no longer the blocker. Its six
  lines are the delta.

### Unclustered (continued)

- **CUMULATIVE UPKEEP — the cell's FIRST account, and CLOSED in it**
  (chapter one hundred thirty). This file had never carried an entry for
  the keyword: it mentioned it twice, once inside the counter-kind catalog
  tail and once inside the spend-restriction entry, and the round that
  landed it was briefed with a "probe first" flag and a duplicate-cell
  claim that exist nowhere in the repo (finding 1051). MEASURED: 80 cards
  print the line, 5 grant it in a quotation, 1 names it in a spend
  restriction. LANDED: `Keyword.CumulativeUpkeep` at `CostParam` —
  chapter seventy-three's mechanism's third consumer, [CR#702.24a] writing
  "Cumulative upkeep [cost]" as [CR#702.21a] writes ward — with the
  escalation left on the rules' side of chapter one hundred
  twenty-three's boundary; `CounterKind.Age`; GLACIAL CHASM whole (its
  third and fourth lines benched as fragments since chapters seventeen and
  twenty-five), ABOROTH, SHELTERING ANCIENT, POLAR KRAKEN, YAVIMAYA ANTS,
  ILLUSIONARY FORCES, VEXING SPHINX, MANA CHAINS. Residues, each counted:
  - **The four `costActionOk` cells this line disagrees with, 4 cards** —
    Braid of Fire ("Add {R}", `AddMana`), Psychic Vortex ("Draw a card",
    `Draw`), Varchild's War-Riders ("Have an opponent create a 1/1 red
    Survivor creature token", `Create`) and Wall of Shards ("An opponent
    gains 1 life", `ChangeLife Up`). The table's zeros were measured AT THE
    COLON and hold there; these four are the same table read by a second
    carrier, so closing them wants a SECOND table over `Cost`, not a
    widened cell (round eighty-seven's posture at `Pay`'s life-gain cell,
    generalised — finding 1055). None is pinnable: every one is attested.
  - **The mana-OR-mana cost, 4 cards** — Arctic Nishoba {G} or {W}, Earthen
    Goo {R} or {G}, Jötun Owl Keeper {W} or {U}, Krovikan Whispers {U} or
    {B}. English "or" between two RUNS, not a hybrid symbol; `ManaCost` is a
    list and `Compound` an AND-join. A `Cost`-side disjunction serving every
    cost carrier, and [CR#702.24a]'s own example spells out what it means
    per age counter.
  - **Four one-gap-each blocks** (finding 1060): Karplusan Minotaur (no
    coin-flip verb in the vocabulary at all), Herald of Leshrac (a one-shot
    control change, where `GainsControl` is a `StaticEffect`), Jötun Grunt
    ("a single graveyard", a uniqueness phrase over the zone's possessor —
    the move itself clears the cost gate), Balduvian Shaman (describes a
    permanent by NOT having the keyword; `HasKeyword` demands
    `KeywordParamless`).
  - **Phyrexian Soulgorger** — the sacrifice cell's other card, blocked on
    the `Phyrexian` name-straddle in its type line, not on this row.
  - **Cover of Winter** — its second line prevents X combat damage where X
    reads the age tally; a prevention size read off a count. Its third line
    is benched.
  - **The longhand mirror without the keyword, 2 cards** — Cyclone and
    Phantasmal Sphere write the whole escalating procedure out ("put a wind
    counter…, then sacrifice it unless you pay {G} for each wind counter on
    it"). A boundary, not this row's scope: three more (Myr Prototype,
    Primordial Ooze, Rogue Skycaptain) share the escalation and swap the
    consequence.

- **The Saga frame's residues** — chapter one hundred thirteen landed the
  frame itself (`GameEvent.ChapterMark` under the ordinary `Triggered`,
  `ChapterDefaults`, `CardChapters`, `Subtype.Saga`, `CounterKind.Lore`;
  History of Benalia, Vault 75: Middle School, Founding of Omashu, Keldon
  Warcaller whole), so what is left here is a list of unbuilt neighbours and
  not a design question. Each is counted.
  - **READ AHEAD**, 10 supported cards ([CR#702.155a,714.3b]). The keyword is
    one `Keyword` catalog row and its usual six cells, and the catalog's
    discipline mints it on the round a witness prints it — but no witness is
    cheap: every one of the ten needs machinery from an unbuilt family for at
    least one chapter (The Elder Dragon War wants a cross-kind coordinated
    damage recipient AND a "that many" readback over a discard; Founding the
    Third Path wants cast-without-paying and a copy-a-card verb; The
    Weatherseed Treaty wants domain; Urza Assembles the Titans wants the
    loyalty-activation permission; Braids's Frightful Return and The Cruelty of
    Gix want reveal-and-choose; The Phasing of Zhalfir wants phasing; The World
    Spell wants look-at-N-and-take; Yotia Declares War wants the reflexive
    "When you do"; Love Song of Night and Day wants a coordinated
    you-and-opponent subject). ALSO WANTED BY Barbara Wright, whose "Sagas you
    control have read ahead" is a grant of the same keyword. The word is
    cheap; the card that prints it is not.
  - **The FINAL CHAPTER ABILITY as an event**, 2 supported lines and
    [CR#714.2e]'s own term: "Whenever the final chapter ability of a Saga you
    control triggers" (Historian's Boon) and "… resolves" (Narci, Fable
    Singer). This is chapter one hundred twelve's ABILITY-CLASS SUBJECT gap at
    a third site — `Kind` has no ability sort — so it belongs to that entry
    and not to this one; recorded here so the cross-link is visible from the
    Saga side. Note the two write DIFFERENT verbs about the same ability
    (triggers / resolves), which the ability-class round will have to tell
    apart.
  - **The NAMED CHAPTER**, 38 supported lines over 24 cards. The Final Fantasy
    Sagas write a name inside the striation after the dash — "I, II, III —
    Pain — You draw a card and you lose 1 life" (Summon: Anima), and 36 more
    across the Summon cycle and the Dominant double-faced cards — while the
    unsupported Five Stages of Grief writes its names BEFORE the dash ("I
    Denial —") and Summon: Magus Sisters writes them inside modal BULLETS
    ("• Defense! — Put a shield counter on target creature"), three placements
    for one thing. The 38th is the giveaway: The Weatherseed Treaty's "III —
    Domain — Target creature you control gets +X/+X", where the name is a real
    ABILITY WORD. So this is the ability-word idiom ("Boast — ", "History
    Teacher — ") at a new carrier, and measuring the two together is the
    better-shaped round: this grammar spells no ability word anywhere yet.
  - **The transform chapter**, 29 supported lines writing one sentence — "Exile
    this Saga, then return it to the battlefield transformed under your
    control" — which is the double-faced Sagas' whole mechanism (43 supported
    transform Saga faces, 29 fronts and 14 backs). It is an ordinary chapter
    EFFECT and needs nothing of the frame; what it wants is the transform verb
    and the exile-and-return-under-your-control compound. Its cluster is the
    largest single unwritten chapter sentence there is.
  - **The Saga CREATURE's second text box**, [CR#714.1a], 18 bare keyword
    lines over the Summon cycle (Flying 6, Menace 2, "Flying, lifelink" 2,
    "Ward {2}", "Reach, trample", "Trample, haste", "Flying, haste", First
    strike, Indestructible, Haste, Vigilance). The frame gate already admits
    them (probed green in chapter one hundred thirteen), so this costs only
    whatever `Keyword` rows the chosen witness prints — but the cycle's chapter
    effects are mana abilities and modal lists, so no Summon card is close.
    The other 6 non-chapter lines on supported Sagas are modal bullets under a
    chapter, not second-box abilities.
  - **The MODAL chapter**, 2 cards (Life of Toshiro Umezawa's "I, II — Choose
    one —", Summon: Magus Sisters' "I, II, III — Choose one at random —"). The
    mode list is `chooseOne`'s and needs nothing new; both cards are blocked on
    their other lines.
  - **The Saga's own tally as a READ**, writable today and unclaimed:
    `CountersOn Lore (AsType Enchantment This {sub = Just Saga})` probed green,
    so "the number of lore counters on this Saga" costs nothing. The lines that
    write it (The Triumph of Anax, Genesis of the Daleks, Summon: Esper
    Valigarmanda, Mind Unbound, Scroll of the Masters, Tom Bombadil, Chong and
    Lily) each want their own consuming effect, which is where their blockers
    are.
  - **Two card-level laws MEASURED AND DELIBERATELY NOT GATED** (finding 888),
    recorded so no successor re-proposes them. Chapter COMPLETENESS (a Saga's
    numerals cover 1..N exactly once) holds on all 223 supported cards and is
    still declined, because [CR#714.2d] and [CR#714.4] both provide for the
    Saga it would forbid. Printed ORDER is refused by an attested card: BLINK
    prints "I, III —" above "II, IV —". Do not gate either.
- **The ADDITIONAL PHASE — CLOSED for the existential frame, with four
  counted residues.** Chapter one hundred twenty landed
  `Effect.AdditionalPart` over the frame's 24 sentences / 48 occurrences,
  with the part, the anchor, the follower and the count as slots and the
  anchor's sort word derived; AGGRAVATED ASSAULT, RELENTLESS ASSAULT (the
  entry's oldest name) and HELLKITE CHARGER are benched whole, and Full
  Throttle's, Raphael's and Y'shtola's clauses as fragments. What is left:
  - The "YOU GET" FRAME, 3 — Obeka, Splitter of Seconds, Paradox Haze and
    The Ninth Doctor. [CR#500.10a] makes this a difference of MEANING and
    not of wording (a "you get" addition on another player's turn adds
    nothing), so it is a second row and not a slot. Obeka's variable count
    ("that many additional upkeep steps") rides with it, reading a damage
    amount back.
  - The added BEGINNING PHASE, 3 — Cyclonus, Shadow of the Second Sun,
    Sphinx of the Second Sun. The one part the corpus adds that `TurnPart`
    has no row for. SPHINX OF THE SECOND SUN is the only whole card behind
    it and its header cell probed green in chapter one hundred twenty
    (`BeginningOf PostcombatMain (Just EachYours)`), so the row plus a
    `Sphinx` subtype is the entire cost.
  - The ORDINAL ANCHOR, 1 — World at War's "After the second main phase
    this turn". Its second sentence also reads the added combat back ("At
    the beginning of that combat"), which is the deictic entry's question
    at a PART rather than at a turn; Moraug, Fury of Akoum writes the same
    read.
  - The carriers still blocked and why, each checked: FURY OF THE HORDE
    (alternative cost), WAVES OF AGGRESSION (retrace), SEIZE THE DAY
    (flashback — its two sentences write today), AKKI BATTLE SQUAD
    (modified-permanent predicate), RAPHAEL, TAG TEAM TOUGH (menace, and
    the "for the first time each turn" trigger rider), MORAUG and
    OVERPOWERING ATTACK (attack-count reads), LAST NIGHT TOGETHER and
    ALL-OUT ASSAULT (their other lines), ZARIEL (an emblem holding the
    clause), GREAT TRAIN HEIST (spree), BALTHIER AND FRAN / LIGHTNING
    RUNNER / COMBAT CELEBRANT / BREATH OF FURY / BRUCE BANNER (their own
    mechanics).

- **The turn schedule's OTHER residues**, each with its count, left by
  chapter one hundred nineteen.
  - The WOULD-WORDED SKIP, 4 sentences — "If you would begin your draw
    step, you may skip that step instead" (Fasting), "If you would begin
    your turn while this artifact is tapped, you may skip that turn
    instead" (Time Vault), and Gerrard's Hourglass Pendant / Stranglehold's
    "If a player would begin an extra turn, that player skips that turn
    instead" (2). These want a BEGINS-A-STEP/TURN event for `Intercepts`
    to watch, which no `GameEvent` row supplies, and the last two want the
    extra turn as a DESCRIPTION rather than as a creation.
  - The DRAW-EVENT skip, 6 — Island Sanctuary, Living Conundrum, Notion
    Thief, Obstinate Familiar, Plagiarize, Possessed Portal. This skips an
    EVENT and not a turn part, so it is the interception's and is ledgered
    there; the landed `SkipsNext`/`Skips` rows take a `TurnPart` and reach
    none of them.
  - The QUANTIFIED PART, 2 — "skips all combat phases of their next turn"
    (Empty City Ruse, False Peace), a quantifier over the parts of a named
    turn and a second surface. Both are one-line white sorceries, so this
    is two whole cards for one construction.
  - The WINDOWED scheduled skip, 3 — Elfhame Sanctuary's "your draw step
    this turn", Moment of Silence's "their next combat phase this turn"
    (which writes the window AND "next"), Fatespinner's "each instance of
    the chosen step or phase this turn".
  - The VARIABLE skip count, 1 — Ral Zarek, Guest Lecturer's "their next X
    turns", which `SkipCount`'s closed table cannot carry and which is the
    land allowance's Nahiri situation exactly.
  - The extra turn's FOR-EACH multiplier, 3 — Ral Zarek, Expropriate, Sage
    of Hours. A scaling adverbial over a count of one, not a third count
    cell.
  - The extra turn as a CLASS OF TURNS, 1 — Medomai the Ageless's "Medomai
    can't attack during extra turns", which reads extra turns rather than
    creating one and is a `TriggerWindow`/restriction question.
  - EMRAKUL, THE PROMISED END's fronted anchor, 1 — "After that turn, that
    player takes an extra turn", the one sentence of 34 that does not
    write "after this one".
- **The DEICTIC TURN — CLOSED for the possessive reader, with four
  counted residues.** Chapter one hundred twenty landed the mention
  (`Kind.TurnRef`, prepended by `ExtraTurn`) and the reader
  (`Owner.ThatTurns`, gated by `TurnDeixis`), and benched Final Fortune,
  Last Chance and Chance for Glory whole; Warrior's Oath prints Last
  Chance's text word for word and is not benched separately. What is left,
  each with its count:
  - The SECOND INTRODUCER, 2 — "during that player's next turn" (Oracle
    en-Vec, Emrakul, the Promised End), which is GIDEON JURA's adverbial
    and would prepend the same mention from a second site. Oracle en-Vec
    then reads it with the possessive this chapter built, so the two cards
    are one introducer away from the reader they already have; Emrakul
    additionally wants the fronted "After that turn" anchor.
  - "DURING THAT TURN", 2 (Alchemist's Gambit, Kang the Conqueror) — a
    demonstrative on the part itself rather than a possessive determiner
    before it, so `Owner` is the wrong slot and `Timing`/`Duration` is
    where it would sit.
  - SAVOR THE MOMENT's "the untap step of that turn", 1 — the skip derives
    its possessive from the SUBJECT by design (chapter one hundred
    nineteen, finding 947) and has no slot for a turn. Giving it one is a
    second surface, not a cell.
  - EMRAKUL, THE PROMISED END's "After that turn, that player takes an
    extra turn", 1 — the extra turn's fronted deictic anchor.
  - NOT this entry's: "at this turn's next end of combat", 3 (Gaze of the
    Gorgon, Glyph of Doom, Triton Tactics), which names the CURRENT turn
    with no introducer at all and is the delayed trigger's endpoint
    question.

- **RETURN is not a VerbName row — do not re-propose it** (chapter
  seventy-three's finding 514, recorded here because the recon proposed it
  twice). Measured: "return" is absent from [CR#701]'s seventy keyword actions
  and the CR defines it nowhere; "would be returned" 0 supported lines, "was
  returned to" 0, "the returned card" 0, against mill's 2, 12 and 19. The five
  "is returned to [a] hand" triggers are zone-change triggers ([CR#603.6])
  keyed on the destination, and "returned this way" (10) is the verb-general
  deictic idiom — it attaches to "removed", "prevented", "moved", "died" and
  "enchanted" too. What IS open here is smaller and different: a return's
  hand-destination `Move` writes no stamp, so nothing reads a returned card
  back by participle, and if a witness ever wants "the returned card" it wants
  a stamp without a tag.
- **The plural-GROUP library slice** — the half of chapter seventy-three's
  retired pin that survives as `badSliceOfGroupPossessor`. Five supported
  lines write a slice over a group possessor and every one of them pluralises
  the ZONE word ("the top card of their libraries", Field of Dreams and
  Lantern of Insight; "each of those opponents' libraries", Breeches; "one of
  their opponents' libraries", Shared Fate). `LibrarySlice` spells one library
  and pluralises only the card word, so the phrase is a second surface, not a
  cell — and two of the five put a partitive over the possessor besides.
- **The mill's AMONG-restriction** — all that chapter seventy-three left of
  the mill's read forms. The participle halves landed with the tag ("the
  milled card" 19 lines, "milled this way" 43, both through
  `TheVerbed`/`ThoseVerbed`), but "from among the milled cards" / "from among
  them" is a partitive over a verbed mention and a surface of its own; it is
  not the tag's and no VerbName row will produce it. Neighbours `SomeOf`,
  which is the partitive this grammar has, and the question is whether the
  "from among" frame is that determiner under a different spelling or a second
  construction. Twenty-plus lines across mill, reveal and exile.
- **The remaining parameterized [CR#702] keywords** — chapter seventy-three
  minted the slot (`KeywordParamShape`/`KeywordParam`, findings 517–518) and
  two rows into it, ward (cost) and protection (quality). What is left is rows
  and their shapes, measured — and after chapter seventy-four there is NO
  SHAPE LEFT TO MINT: cost, quality, subject and number are all built and
  carry rows. ANNIHILATOR ([CR#702.86a] "Annihilator N", 21 supported lines)
  is now a `keywordParamShape` cell of `NumberParam` and four attestation
  cells, and waits on a consumer alone; AFFINITY ("affinity for artifacts", 5
  lines) is a quality and likewise waits only on a witness. Equip, ascend,
  storied and renown LANDED in chapter seventy-four and are struck from this
  entry. The mechanism does not move again: a row is a `Keyword` constructor,
  a `keywordParamShape` cell, and its four attestation cells.
- **The status event's SECOND READER** — `statusEventOk` is one table asked by
  the trigger header and, once, by a duration endpoint: Vesuvan Shapeshifter's
  "until this creature is turned face down" is the clause's only supported
  occurrence anywhere and the FaceDown cell refuses it along with the header
  it correctly refuses (0 headers, against the up cell's 125). One table, two
  readers, and they disagree on exactly one cell — the split is cheap and is
  deferred only because chapter seventy-two had two jobs already. It travels
  with `eventSpan`, whose Unattested row for `StatusChange` is the same
  question asked of the tap pair.
- **Recurring untap grants** — the untap cap's mirror: "Untap [set] you control
  during each other player's untap step" (Seedborn Muse's family, twelve lines,
  measured in chapter thirty-nine's recon). Grants where the cap denies. The
  plural player axis this entry was said to be the sole consumer of is GONE:
  chapter fifty-three minted it on the outcome gate's evidence rather than on
  this family's and retired `CapDomain` with it (findings 379–382). What is
  left here is the GRANT construction itself, plus one possessor cell —
  "each other player's untap step" writes a quantifier possessor with an
  other-marking over it, and chapter forty-five's `Owner` has `EachPlayers`
  with no other-marked row.
- **The referential productions** — what chapter ninety-one measured and did
  not build, each at its count and its named blocker. `ProducedByEvent`,
  "add one mana of any type that land produced", is 17 add-sentences over 17
  cards and wants a TAP-FOR-MANA event plus the capability gate the frozen
  module carries (`producesMana (eventCaps b)`), which is why it is filed
  with the tapped-for-mana family below rather than with the production
  vocabulary. The COULD-PRODUCE read ([CR#106.7]) is 18 lines over 18 cards,
  15 of them add-payloads over 15 cards (Exotic Orchard, Fellwar Stone,
  Reflecting Pool). Its blocker is a hypothetical query over another
  permanent's abilities that nothing here spells — [CR#106.7] defines it by
  what an ability "would produce if the ability were to resolve at that time".
  Keep the two rows apart when re-measuring: the past tense and the modal
  share the phrase "mana of any type that", and a regex over it merges them
  (which is where a first pass put `ProducedByEvent` at 24). `AmongColorsOf` — "one
  mana of any of the exiled card's colors" — is 4 sentences (Chrome Mox and
  its neighbours) and wants a color-set read off a mentioned object.
- **The tapped-for-mana trigger family, DEFINITIONS RECONCILED** — the recon's
  34 and chapter thirty-six's 22 are two narrower readings of one family, and
  the numbers are: 23 lines write the passive "Whenever [X] is tapped for
  mana" (chapter thirty-six's figure), 33 write the active "Whenever a player
  taps [X] for mana", and the union is 56 lines over 55 cards, of which 42
  have an effect that adds mana. [CR#106.12] defines the phrase ("to tap a
  permanent for mana is to activate a mana ability of that permanent that
  includes the {T} symbol"), [CR#106.12a] makes the trigger fire on the
  resolution, and [CR#605.1b] makes the resulting ability a mana ability
  itself. This entry owns `ProducedByEvent`'s 17 lines, since ALL SEVENTEEN
  sit inside one of these headers — a total covariance, and the warrant for
  the frozen module's capability gate.
- ~~**The chosen color as produced mana**~~ — **CLOSED by chapter one
  hundred twenty-six**. THE COUNT HERE WAS EXACTLY RIGHT (33/32, reproduced
  first pass) AND THE BLOCKER WAS WRONG, which is the mirror of the other
  entry for this same cell in the standing-prohibition list, where the count
  was wrong and the blocker right (finding 1011 — one cell, two entries,
  each carrying the other's error). "A chosen quality does not cross the
  ability line" is false and was refuted by probe before the design was
  written (finding 1013): `abIntro (Static se) = staticChoiceIntro se`
  threads an `EntersChoice` binding to every ability printed after it, and a
  two-line card whose SECOND, ACTIVATED ability reads `OfChosen Color`
  elaborates with nothing changed. The gap was one constructor, exactly as
  this entry's own parenthesis guessed and then talked itself out of. Its
  alternative-list observation was right and is what shaped the row: the
  literal alternative is a `Maybe` of ONE unit (23 lines print none, 10
  print exactly one symbol, a second literal and a multi-symbol run are
  zeros apiece). RESIDUE, counted and routed: the 4 OTHER-OBJECT readers
  (Caged Sun, Gauntlet of Power, Utopia Sprawl, Shimmerwilds Growth), which
  need this row PLUS the `ProducedByEvent` machinery `AddMana`'s docstring
  already disclaims — they belong to that entry, not this one, and Caged
  Sun's middle line ("Creatures you control of the chosen color get +1/+1")
  was buildable all along, Hall of Triumph having benched that shape since
  the chooser landed. RADIANT LOTUS is routed to the "that color" family:
  its chooser is in the same ability, so the word "chosen" is the corpus's
  own wording inconsistency and not a second construction. The two
  neighbours named here ("of that color" in a per-color iteration, 7; the
  commander-identity phrase, 6) are untouched and keep their own homes.
- **The mana the sentence keeps talking about** — [CR#106.6]'s other two
  strings and the persistence sentence, all three blocked on the same missing
  thing and worth landing together. The EFFECT ON THE PAID-FOR OBJECT is 11
  cells: 9 sentences write "If that mana is spent on a creature spell, it
  gains haste" and 2 write the restriction and the effect in one sentence
  (Cavern of Souls, Delighted Halfling — "and that spell can't be countered").
  The DELAYED TRIGGER ([CR#603.7a]) is 3 (Path of Ancestry, Primal Amulet,
  Pyromancer's Goggles). Both bind the spell the mana was spent on, and the
  frozen module names the shape this grammar has no sort for
  (`bindIt PaidSpellAnte`). PERSISTENCE is 25 lines — "Until end of turn, you
  don't lose this mana as steps and phases end" — which core carries as a
  fourth rider (`Persistent(TurnMarker)`) and [CR#106.4] is the rule it
  overrides; it is a separate sentence with a duration, so it may want the
  `Continuously` envelope rather than a rider slot, and that is the design
  question. Chapter ninety-one measured 219 "this mana"/"that mana" lines in
  all and every one belongs to one of these three families or to the spend
  restriction it landed.
- **The unspent-mana pool** — 14 lines over 13 cards read a player's unspent
  mana (Doubling Cube, Kruphix, Omnath Locus of All, Mana Short, Upwelling,
  Yurlok), and ZERO write "mana pool", which is [CR#106.4]'s own errata note
  visible in the corpus. Drain Power is [CR#106.13]'s named single card and
  rides here. The pool is engine state chapter ninety-one deliberately did not
  put in the grammar; these lines are what would make it a sentence.
- **The storage counter and the Mana Batteries** — "storage counter" is 36
  lines over 18 cards, and the five Mana Batteries write the family's shape
  twice over: "{T}, Remove any number of charge counters from this artifact:
  Add {B}, then add an additional {B} for each charge counter removed this
  way". Two things are missing, an ANY-NUMBER cost quantity and a for-each
  over what a cost removed. (The word "an additional" is NOT one of them:
  chapter ninety-one measured its 36 lines and they are all the second add in
  a chain or a trigger off another production, so it is a discourse spelling
  and no slot — finding 682.)
- **The spend restrictions that name a cost** — 9 of the 164, refused with
  their shapes: 6 restrict to a cost's contents ("on costs that contain {X}",
  Rosheen Meanderer, Nexos, Elementalist's Palette; "to pay cumulative upkeep
  costs" — Adarkar Unicorn, and chapter one hundred thirty's keyword row does
  NOT unblock it: the restriction names a cost SHAPE, which is this entry's
  own gap and not the keyword's), 2 disjoin a cost arm with a cast arm (Qarsi Deceiver, Unblinking
  Observer) and 1 names a special action ("to turn permanents face up",
  Overgrown Zealot). `SpendPurpose` takes a spell or an ability's source
  because those are objects; a cost is not, and a cost SHAPE is a third thing
  again.
- **Mana named by a cost** — 2 lines, and they are the surface [CR#106.8..106.11]
  exist for: "Add mana equal to enchanted permanent's mana cost" and Ice
  Cauldron's "Add this artifact's last noted type and amount of mana". A
  produced run is `ColorOrColorless` by [CR#106.1b]; these two name a printed
  COST and let the rules translate it, which is a different row and not a
  widening of `ProducedRun`.
- **The remaining freedom cells** — three small ones measured in chapter
  ninety-one and left: "add [N] mana in any combination of {R} and/or {G}" (12
  sentences — a combination over a WRITTEN color set where `EachColor` ranges
  over all five), "add two mana of different colors" (4 — a third value on the
  freedom axis, all units distinct), and the five-way alternative list, which
  is written ZERO times and is why a canonicality gate against
  `Runs [[W],[U],[B],[R],[G]]` was recorded rather than minted (it would
  refuse a term denoting attested English).
- ~~**The event-history COMPLEMENT**~~ — **CLOSED by chapter one hundred
  thirty-two**, which landed the third slot on all three readers as a defaulted
  `EventComplement`, gated by `lookbackComplementOk` (event x subject sort x
  complement sort — THREE axes, the attack declaration forcing the third as it
  forced the subject table's second) and `bareLookbackOk` (whether the slot may
  stand empty; two measured False cells, both pinned). This entry's own sizing
  was an order of magnitude short and the correction is the round's headline:
  the cast history read alone is 141 lines of which only TWO write the bare
  form the landed rows spelled, so the complement was the construction and not
  its residue. The by-source damage family is 48 and not "~20+"; the block
  relation's by-complement is 1 line and not "~9" (Joven's Ferrets — the other
  eight the entry was counting are present-tense `BlockerOf`/`BlockedBy`
  descriptions, a different row, or the "blocked or was blocked by X"
  coordination). Three `lookbackSubjectOk` cells reopened with it
  (`CombatDamage Object`, `TokenCreation Player`, `BlockedDeclaration Object`).
  What still rides here, each with its own remaining blocker:
  - **The four delayed end-of-combat forms** (finding 196) — unchanged and
    confirmed: their independent second blocker is the "At this turn's next end
    of combat" delayed shell, which no current machinery reaches.
  - **The coordinated history read** — "blocked or was blocked by a Zombie this
    turn" (Time to Reflect, You Cannot Pass!, Venomous Breath, Sea Troll): 4
    lines sharing ONE complement across two events, which is the coordinated
    EVENT filed with the other coordinations and not this slot.
  - **The dealer-subject read of damage IN GENERAL** — 17 lines, "target
    creature that dealt damage to you this turn" (Reciprocate, Retaliate, Spear
    of Heliod, Giltspire Avenger, Brine Hag, Giant Albatross, Aegar, Hawkeye,
    Wolverine, Dunerider Outlaw, Whirling Dervish...). It has NO `EventName`:
    `DamageTaken` is the victim's side of the happening and `CombatDamage` is
    the combat dealer's, and the general dealer's side is neither. Chapter
    forty-two's comment counted two of these as combat-damage lines and they
    say "damage", not "combat damage" — the correction is what exposed the gap.
  - **The SECOND complement sort** — the placement's ZONE (19 lines, "if a
    creature card was put into your graveyard from anywhere this turn", and
    most name a source zone as well as a destination) and the counter pair's
    KIND (3 lines). This slot's axis is `Kind`; neither is a noun of any sort,
    so `Placement`, `CounterPlacement` and `CounterRemoval` keep their False
    cells and would want a complement over `ZoneExpr` and one over
    `CounterKind`. Whether that is one mechanism with a sorted payload or three
    tables is the design question.
  - **The MAGNITUDE read** (`EventSum`) — "if you gained 3 or more life this
    turn" (15 lines) and Gollum, Obsessed Stalker's "life equal to the amount of
    life you gained this turn". A complement is a participant and this is a
    quantity; it stays ledgered on `EventCount`'s own row.
  - Continue?, whose zone-change surface is a different spelling from the past
    verb these rows write (chapter forty-three records it as the seeding's
    proof); and Bill Ferny, still blocked on the predefined-token catalog
    ([CR#111.10]) alone.

- ~~**The CAST-ZONE PROVENANCE**~~ — **CLOSED by chapter one hundred
  thirty-three**, and this entry's own scoping was the thing the round had to
  correct. Written last chapter from the history-read slice alone, it counted
  31 lines; the family is 185 over FIVE frames (trigger header 80,
  self-condition 37, history lookback 32, description 29, mana-spend purpose
  7), beside 148 permission lines that are a different axis. One row landed
  and reaches three of the five with no constructor changed:
  `Predicate.CastFrom (z : ZoneExpr)`, gated by `playableFrom` (SHARED with
  the play permission, every cell re-measured, none moved) and `WholeZone`,
  seeding no zone so that `badCastInGraveyard` still stands. The negation
  needed nothing — ordinary `Not`, two attested spellings, one frame writing
  both.
  What is left, each with its own blocker:
  - **The history LOOKBACK frame, 32 lines.** The origin inside chapter one
    hundred thirty-two's event-history read. 21 of the 32 are the commander
    family and want a COMMAND ZONE row: `data Zone = Battlefield | Graveyard
    | Exile | Hand | Library | Stack` has no seventh, and adding one moves
    ten tables (`sameZone`, `searchableZone`, `putSourceZoneOk`,
    `putDestZoneOk`, `publicZone`, `exposableZone`, `destTypeOk`,
    `visibilityOk`, `playableFrom`, `zoneFits`) whose cells this round did
    not measure. The other 11 (the six "if you haven't cast a spell from your
    hand this turn" cards, Laboratory Drudge, Impending Flux, Surge of
    Brilliance, Spider-Man 2099, Approach) want the origin as a rider on
    `Happened`/`EventCount` and nothing else — the cheaper half, and it can
    land without the zone row.
  - **The zone DISJUNCTION**, "from your graveyard or from exile" (Doc
    Aurlock; Aven Interrupter; Soulless Jailer's "from graveyards or exile").
    Two origin clauses coordinated, which is the standing coordination gap
    at a new site.
  - **The mana-SPEND purpose, 7 lines** ("Spend this mana only to cast
    spells from your graveyard", Rootcoil Creeper, Lord of the Forsaken,
    Interdimensional Web Watch, Altar of the Lost, Mm'menon; and the two
    negative "This mana can't be spent to cast spells from your hand",
    Karolina Dean, Vhal). `SpendPurpose` takes a spell or an ability's
    source; a zone-qualified purpose is a third shape, and it belongs with
    chapter ninety-one's spend-restriction entry rather than here.
  - **The PROHIBITION family, 8 lines** ("Players can't cast spells from
    graveyards or libraries", Grafdigger's Cage, Weathered Runestone,
    Kunoros, Soulless Jailer, Ashes of the Abhorrent, Drannith Magistrate,
    Avatar's Wrath, Experimental Frenzy). This is the PERMISSION's negation
    and belongs to `MayPlay`/`PlaySource`, not to the provenance row — the
    boundary the round states in finding 1082.
  - **The LIBRARY cell has no cheap whole carrier**: the whole family writes
    it twice — Melek, Izzet Paragon's trigger header and Fblthp, the Lost's
    self-condition — and both cards carry a second unbuilt thing besides
    (Melek a play permission over the top of a library, Fblthp a coordinated
    entered-or-was-cast condition). The cell is open and probed; the witness
    is owed.

- **The CONDITION DISJUNCTION — "if X or if Y"** — minted by chapter one
  hundred thirty-five, which built the conjunction beside it and deliberately
  left this unbuilt because the number is not known. What IS known: 11
  supported lines mark both halves ("Activate only if this land entered this
  turn or if you control a basic land" — Dark Fortress, Gathering Place,
  Gleaming Bastion, Hidden Lair and Training Compound are one reprint cycle;
  plus Armored Kincaller, Bonecache Overseer, Dragon's Disciple, Mythos of
  Nethroi, Quakebringer, Reptilian Recruiter), and those 11 are a LOWER BOUND
  because a singly-marked disjunction is indistinguishable by sweep from the
  coordinations that are already spelled. The raw `if…or…` population is
  1,274, still 360 after the comparator idiom ("two or more", "4 or greater")
  is stripped, and it is dominated by the same one-condition coordinations
  (`Predicate.Or`) that made up the conjunction round's largest
  false-positive bucket. THE ROUND THAT TAKES THIS OWES THE HAND COUNT
  FIRST — 360 lines, sorted the way chapter one hundred thirty-five sorted
  its 182 — and only then the row. Three things it can take as settled:
  the row would sit beside `AndCond` on `Condition` and reach the same five
  carriers for free; the arity and flatness demands are already written
  (`atLeastTwoCs`, `flatConjuncts`) and would want a decision about whether a
  disjunct may be a conjunction (Quakebringer's "if Quakebringer is on the
  battlefield or if Quakebringer is in your graveyard and you control a
  Giant" is an OR of an AND, so the answer is probably yes and
  `FlatConjuncts` would need a companion rather than a copy); and the
  ACTIVATION GUARD is this family's biggest carrier where it was the
  conjunction's empty one (6 of the 11).
- **The OWNERSHIP predicate, and the meld idiom above it** — five supported
  cards write "you both own and control [X] and a creature named [Y]"
  (Gisela, the Broken Blade; Graf Rats; Hanweir Battlements; Vanille,
  Cheerful l'Cie; Titania, Voice of Gaea), and the grammar has `ControlledBy`
  with no owner counterpart, so the conjunction chapter one hundred
  thirty-five landed cannot reach them: the clause "both own and control X"
  is itself two conditions and one of them has no row. [CR#108.3] is the
  ownership rule. Note the second blocker before scoping a round: all five
  lines end in "exile them, then meld them into [Z]", and MELD has no
  vocabulary anywhere in this file — a round that buys ownership alone
  unblocks none of these five cards and should say so, or find the ownership
  carriers that do not meld.
- **Per-game memory, what is left after chapter one hundred thirty-two.**
  `Lookback.ThisGame` is landed and ungated; 28 supported lines write "this
  game" and none was benched when this entry was written, every carrier
  having a second blocker. APPROACH OF THE SECOND SUN is off the list —
  chapter one hundred thirty-five benched it whole, and it is this row's
  first witness. The rest:
  - the 21 commander-cast counts — cast-zone provenance, above;
  - **Gollum, Obsessed Stalker** — "each opponent dealt combat damage this game
    by a creature named Gollum, Obsessed Stalker loses life equal to the amount
    of life you gained this turn": the subject, complement, window and name are
    all writable now (its shape is benched at WICKED AKUBA in miniature), and
    what refuses it is the `EventSum` magnitude in the body;
  - **Yidaro, Wandering Monster** — "if you've cycled a card named Yidaro,
    Wandering Monster four or more times this game": there is no `EventName` for
    a CYCLING, and the keyword-action event name is the same gap the surveil
    entry's per-turn state read records ("as long as you've surveilled this
    turn", Darkblade Agent, Eye of Duskmantle);
  - **The Fallen** — "each opponent and planeswalker it has dealt damage to this
    game": a relative clause with BOTH an overt subject ("it") and its head in
    the complement position, which is neither `Happened` (no head) nor
    `HappenedTo` (no subject) — a third reader shape — over a union head;
  - **Once Upon a Time** — "if this spell is the first spell you've cast this
    game": an ORDINAL identity read over the history, not a count and not an
    occurrence; and the rest of the card (look at five, reveal, bottom in a
    random order) is unbuilt besides;
  - **Frodo, Adventurous Hobbit** and **Frodo, Sauron's Bane** — "if the Ring
    has tempted you two/four or more times this game": no Ring machinery.
  Also measured and NOT families: "chosen/noted this game" is zero lines, and
  every "first/second time" in the corpus is turn-scoped.
- **The header-internal "while" clause** — measured in chapter forty-one and
  deliberately not designed there: 60 header-internal lines, none comma-marked,
  attaching the clause to the EVENT rather than sitting between two commas after
  it. [CR#603.4] explicitly does not reach it ("the word 'if' has only its
  normal English meaning anywhere else"), so it is not a marking variant of the
  intervening slot. At least twelve condition shapes, and three of them name an
  action IN PROGRESS rather than a state ("while you're activating a craft
  ability", "while casting a spell with emerge", "while scrying") — which is why
  it cannot be a marking word on a state condition and needs its own design.
  The saddle family alone is about twenty-five of the sixty. Named payoff:
  Veiling Oddity, whose body, zone check and intervening slot ALL elaborate as
  of chapter forty-one — the last-removal event's only reachable bench line.
  Its second blocker MOVED in chapter forty-eight: the self-PRONOUN half is
  fixed (`It` now resolves off the event subject) and what remains beside this
  word is the self's ZONE EVIDENCE — the sorted self projects the battlefield
  by [CR#109.2] and the card asks about exile (findings 346 and 351).
- **Source phrases** — ledger L1/OPEN-2, waiting on the [CR#609.7] by-source
  prevention rider.
- **Granted quoted replacement** — Bewitching Leechcraft's family, down to ONE
  blocker as of chapter eighty-seven, and the surviving blocker is NOT the one
  this entry recorded. The quotation is gone (`Grantable` takes a nonkeyword
  static now) and the attachment subject went in chapter fifty-one. The
  would-untap EVENT is not missing either: `StatusEvent … Untapped` is a row.
  What refuses is that `eventUse StatusChange` is `TriggeredOnly` — the status
  change is admitted as a trigger header and not as a replacement's antecedent
  — and the "during your untap step" window has nowhere to sit on `Intercepts`
  besides, which is the static-window entry above at a second site. So the ask
  is one cell plus a window, both on the interception side.
- **The attachment's own direction** — "as long as this Equipment is attached to
  a creature", 12 lines: the THIRD direction, naming the relation from the
  attachment's side where chapter fifty-one minted the host's participle
  (attachment→host) and the presence reads (host→has-attachment). Small, and it
  is the one place an attach relation would actually be written.
- **The RESTRICTED EQUIP compound** — all that chapter seventy-four left of
  the equip line. 21 supported lines carry BOTH a quality and a cost ("Equip
  legendary creature {1}" 2, "Equip commander {3}", "Equip Knight {1}",
  "Equip Soldier {W}", … 20 distinct shapes over 21 lines), which
  [CR#702.6c] describes as an additional restriction on the ability's TARGET
  and not on what the Equipment may be attached to — a fact worth keeping,
  since it means the quality does not compose with the attachment. `CostParam`
  cannot hold them and neither shape alone can; core has no compound
  precedent for equip either, only `Reinforce.ron`'s `[Count, Cost]` proving
  multi-parameter possible at all. Deliberately not minted in chapter
  seventy-four: no witness demanded it, and a shape with no consumer is the
  one thing the open-catalog ruling still refuses. Neighbours [CR#702.6e]'s
  "Equip planeswalker", which is a variant of the same shape.
- **The EQUIP COST-REDUCTION statics** — Ghostfire Blade's line and its
  family, and NOT part of the equip keyword line: 9 supported lines write
  "Equip abilities you activate … cost {N} less to activate" plus 7 more that
  append a reduction sentence to a bare equip line ("Equip {4}. This ability
  costs {3} less to activate if you're the monarch."). They are cost
  modification over an ability class, which is a different axis from the
  keyword row entirely — the row landed in chapter seventy-four and Ghostfire
  Blade still does not, on this line alone.
- **MONSTROSITY's expansion body** — chapter seventy-four measured it and
  refused the tag (finding 526): the machinery keys on the DESIGNATION, which
  has had its row since chapter seventy-one, not on the action's name
  ("monstrosity this way" 0 lines, "would become monstrous" 0), so no
  `VerbName` row is earned. What it needs is the ability to WRITE
  [CR#701.37a]'s expansion, and the probe named the block precisely:
  `GainsDesignation This Monstrous` is refused because `designationGiven
  Monstrous` is False. That cell is right about bare sentences — no card
  writes "this creature becomes monstrous" as an instruction — and wrong
  about an expansion body, and telling the two apart is the mechanism
  question. It is the same question the other four keyword-conferred givings
  ask (city's blessing, enduring story, renowned, saddled), and chapter
  seventy-four confirmed all four cells stay False by benching three cards
  that print the keyword line and check the designation without ever
  instructing the conferral. 37 monstrosity lines, all "{cost}: Monstrosity
  N."; Chillerpillar is the card it makes whole.
- **The MILL event, and the verbed event generally** — the put-into-a-zone
  event landed in chapter seventy-eight and this did not fall out of it,
  which is the point of the entry: [CR#701.17a] makes milling "puts that
  many cards from the top of their library into their graveyard", so the
  transition IS `PutInto`'s with a library source and a graveyard
  destination, and the corpus still spells it with the VERB. Four supported
  lines write the header ("Whenever one or more nonland cards are milled",
  Mirelurk Queen, Screeching Scorchbeast, The Wise Mothman; Saruman writes
  the reflexive "when one or more cards are milled this way"), and two write
  the replacement ("If an opponent would mill one or more cards, they mill
  twice that many instead", Bruvac, The Water Crystal). `VerbName` has had
  `Mill` since chapter seventy-three and `verbedMarkingOk Mill` is open, so
  what is missing is the EVENT reading of a verb — an event named by the
  keyword action rather than by the zone change it entails, which is
  core's `EventFilter::Act { verb, … }` and has no row here. Screeching
  Scorchbeast is the witness and it also wants the counted group's size as
  an amount ("that many"), so it is two gaps until that lands.
- **The DEFINITION channel — what `InsteadOf` withholds** — all that chapter
  eighty-one left of the token anaphor, and a sharp one. Sixteen supported
  lines write an anaphoric create specification inside a SELF-replacement:
  "Create six 1/1 white Kor Soldier creature tokens. If this spell was kicked,
  create twelve of those tokens instead" (Conqueror's Pledge), and the same
  shape on Saproling Migration, Increasing Devotion, Gather the Townsfolk,
  Prismari Pianist, Anax, Skeletal Swarming, Throne of Empires, Starnheim
  Unleashed, Rite of Replication, Adipose Offspring, From Under the
  Floorboards, Safana, Andúril, Runo Stromkirk and The Final Days. The
  composition is fine — `TokenSpec.TokenAsThose` in an ordinary sequence and
  under arrival riders both probed green — and what refuses is `InsteadOf`,
  whose replacement arm is typed at `annIntro replaced` and so cannot see the
  batch the replaced clause made.
  THAT REFUSAL IS CORRECT AS FAR AS IT GOES and must not simply be relaxed:
  [CR#614.6] says the replaced event never happened, so the tokens do not
  exist and `annIntro` is right to withhold the OBJECT mention
  (`badInsteadReadsReplacedSequenceOutcome` is the same discipline one step
  over). What the anaphor actually points at is the CHARACTERISTIC DEFINITION
  the replaced clause wrote — [CR#111.3]'s "this becomes the token's 'text'" —
  which is written whether or not the creation happens. So the ask is a
  channel for DEFINITIONS beside the announcement channel for phrases, and the
  binding vocabulary today has one mention doing both jobs. Whoever takes it
  should decide whether `Payload`'s object arm grows a definition flag or
  whether the announcement channel gains a second row, and should note that
  the five NON-instead ordinary lines (Brood Birthing, Delina, Swarming
  Goblins ×2, Wurmquake) are blocked on dice, quoted abilities and mana
  instead, so this entry lands no card by itself — Prismari Pianist is the
  cheapest one behind it, single-blocked on exactly this.
- **The DISTRIBUTIVE counter-kind anaphor, and the kind-blind event** — one
  entry rather than two, which is chapter eighty-one's correction of chapter
  eighty's pair. "That many plus one of each of THOSE KINDS of counters" is 10
  supported lines (Doc Samson, Innkeeper's Talent, Lae'zel, Loading Zone, Pir,
  Vorinclex ×2, Winding Constrictor ×2, and Aragorn at the trigger position),
  and every one of them PRESUPPOSES a kind-blind announcement: "each of those
  kinds" can only range over a batch that may hold several kinds, which is
  exactly what `CounterEvent`'s required `kind` refuses. So the anaphor cannot
  land before the kind-blind event and the two are one round — finding 285's
  territory, with Stalwart Successor's header the trigger-side carrier.
  Doubling Season's second line is the marquee behind it: its causer half
  composes green as of chapter eighty-one and the required kind is the only
  refusal left, so the card goes whole the moment this lands.
  ONE BLOCKER FEWER as of chapter one hundred twenty-eight: four of the ten
  lines (Vorinclex ×2, Innkeeper's Talent, Lae'zel) put their counters on
  "that permanent or player", and that read is landed (`NounWord.JoinW`), so
  what is left on those four is this entry's own kind-blind event and
  nothing else. Recorded here rather than refiled under the union anaphor —
  the union cell routes to this entry, not the other way round.
  NOT this family, recorded so it is not swept in: "those counters" is 16
  lines but 15 of them move or remove a set of counters already ON an object
  ("if it had counters on it, put those counters on target creature", The
  Ozolith, Iron Apprentice, Resourceful Defense and four more) — a different
  anaphor over a different thing, and its own entry when someone wants it.
  Beside it, "counter of that kind" is 14 lines of a CHOSEN kind ("choose a
  counter on target permanent … put a counter of that kind on"), a
  chosen-quality axis and a third family again.
- **The DESCRIBED-ability causer** — `Causer` is one row (`AnEffect`,
  [CR#614.16]'s bare word, 4 lines) and three supported lines write a
  described ability in the same position: "if a spell or ability would cause
  its controller to gain life" (Rain of Gore), "if a cycling ability of
  another nonland card would cause you to draw a card" (Unpredictable
  Cyclone), "if a modular triggered ability would put one or more +1/+1
  counters on a creature you control" (Zabaz, the Glimmerwasp). One line
  apiece, each carrying its own restriction, and two of the three write the
  periphrastic verb "cause … to" rather than the event's own — which is a
  second unbuilt thing and the reason these are not `Causer` rows. Zabaz is
  ALSO the sole crossing of finding 582's voice covariance, so the line that
  lands it is the line that turns `Intercepts`' derived body voice into a
  slot; the two facts belong to one round.
  The SINGULAR anaphor rides here too: "instead create that token and a
  Treasure token" (Mr. House, President and CEO) is the one line writing a
  singular create specification, and it is attested so `TokenAsThose`'s plural
  demand is not pinned (finding 596). It wants a coordination of two
  specifications and [CR#111.10]'s predefined name besides.
- **The creation clause's remaining subject shapes** — NARROWED by chapter
  eighty-one, which landed what was (a): the effect subject is `Causer` and
  [CR#614.16]'s word is a row. The worry recorded there did not survive
  measurement — the body's "it creates" needed no mentionable thing at all,
  because a replacement body over this event writes no subject phrase of its
  own in any of its three shapes (finding 595), so the pronoun is the
  container's derivation exactly as the passive voice is. What remains of
  the entry, both single lines: (b) THE
  ABILITY SUBJECT, Zabaz, the Glimmerwasp's "if a modular triggered ability
  would put one or more +1/+1 counters on a creature you control" — one
  line, and it is the SOLE crossing of finding 582's voice covariance (an
  active event clause with a passive body), so the line that lands it is
  also the line that turns `Intercepts`' derived voice into a slot (see the
  described-ability causer entry, which is where it is now filed). (c) THE
  RELATIVE-CLAUSE determiner, Crafty Cutpurse's "each token that would be
  created under an opponent's control this turn is created under your
  control instead" — one line, a determiner `TokenPhrase` has no
  constructor for and a body that re-states the control phrase.
- **The "FIRST time … each turn" replacement word** — `ReplUse` is
  [CR#614.3]'s two endings, "used up" and "duration expired", and three
  supported lines write a third thing: "The first time you would create one
  or more tokens during each of your turns" (Esix, Fractal Bloom), "the
  first time you would create one or more tokens each turn" (Mirrormind
  Crown, Moonlit Meditation). A once-per-turn CAP is neither ending, and it
  is the replacement-side twin of the once-each-turn trigger rider chapter
  seventy-nine landed (`UsageLimit`) — which is the argument for reading
  that vocabulary here rather than minting a third value. All three bodies
  also want the anaphoric token spec above, so the word alone lands no
  card.
- **The ORDINAL counter event** — "When the fourth plan counter is put on
  this enchantment", 11 supported headers (7 plan, 1 hour, 1 +1/+1, plus
  siblings) that chapter seventy-eight's `CounterEvent` deliberately does
  not spell: `CounterBatch` is two determiners, "a" and "one or more", and
  an ordinal is a third thing entirely — it names WHICH counter in a
  sequence rather than how many. It is the same ordinal vocabulary
  `LibPos`'s comment ledgered ("second from the top", [CR#401.7]) at a
  second site, which is the argument for doing them together. Note that
  every one of the 11 writes `When` and not `Whenever`, which is what a
  once-per-object event looks like; the family is Sagas' plan counters and
  Midnight Clock.
- **Bioessence Hydra's dependency chain** — recorded whole so it is not
  re-derived. Its second line ("Whenever one or more loyalty counters are
  put on planeswalkers you control, put that many +1/+1 counters on this
  creature") needed THREE things; chapter eighty paid the first (the batch
  size, now `ThatMuch` over `OutcomeSort.CountersPut`) and TWO remain: a
  `CounterKind` row for
  loyalty, which finding 567 refused on a sharpened reason — loyalty
  counters are a CHARACTERISTIC's current value ([CR#122.1e,306.5c,109.3])
  where stun, poison and rad are markers; and a holder the grammar can
  describe, planeswalkers not being a `CardType` row. Its FIRST line wants
  loyalty as a readable count as well. Chandra, Fire Artisan is the same
  chain plus a fourth: "target opponent or planeswalker" is a cross-kind
  disjunction over that missing type word.
  UPDATED BY CHAPTERS EIGHTY-SIX AND EIGHTY-NINE: the holder is no longer
  missing — `Planeswalker` is a `CardType` row and two planeswalkers are
  benched whole — so what is left of the chain is the counter kind alone, and
  chapter eighty-nine measured what that refusal now costs: 62 supported lines
  write a loyalty counter, and two of the five Talents are blocked on nothing
  else (Vivien's "put a loyalty counter on enchanted planeswalker", Teferi's
  the same). Finding 567's reason still holds; the price is recorded so
  whichever round revisits it is revisiting a measured question and not a
  remembered one.
- **SHIELD counters, and the waiting-kind list corrected** — finding 567
  found `CounterKind`'s comment claiming shield "clears neither" half of
  finding 85's test, and the count refutes it: 14 supported lines write a
  one-shot put ("Put a shield counter on target creature", Boon of Safety;
  Brokers Veteran; Aegis of the Legion) and one writes a remove. Shield is
  therefore an ordinary WAITING kind beside oil, quest, level, spore,
  storage, ki, verse and depletion — a row the moment a line writing it
  lands — and not a refusal. [CR#122.1c]'s replacement-and-prevention pair
  is machinery the engine owns, exactly as stun's is.
- **The COMMAND ZONE** — `Zone` has five of [CR#400.1]'s zones and core's
  `Command` row "is still not ported" (the catalog's own comment). Chapter
  seventy-eight is the second construction to want it: two put-into headers
  name it ("Whenever your commander is put into the command zone from
  anywhere", Myth Unbound; "Whenever a creature you control dies or is put
  into the command zone", Reyhan), and neither can be refused by
  `putDestZoneOk` because there is no value to refuse. The designations
  census already treats commander as card-held and zone-surviving, so the
  zone and the designation are one round's worth of work.
- **The coordinated SOURCE, and the coordinated DESTINATION** — the
  coordinated EVENT landed in chapter seventy-nine and this half did not,
  because it is a disjunction of ZONE PHRASES rather than of clauses:
  "from your hand or library" (Desert Warfare), "from graveyards and/or
  the battlefield" (Ketramose), "from your library and/or your graveyard"
  (Laelia), and on the destination side Kaya, Spirits' Justice's "one or
  more creatures you control and/or creature cards in your graveyard".
  The `Or` predicate's shape may already reach it — a `ZoneExpr`
  disjunction is a smaller thing than an event one — but nothing has
  measured which of `Or`'s four gates a zone phrase would have to answer.
  Five lines in all, so this is a row on a rainy day rather than a round.
- **The "and whenever" TWO-HEADER JOIN** — 58 supported lines, and NOT the
  coordination chapter seventy-nine built: "When this creature enters and
  whenever it attacks while saddled, create a 3/3 green Elephant creature
  token" (Autarch Mammoth) writes TWO trigger words and joins two whole
  headers over one effect, where `AltEvent` coordinates two events under
  one word. The measured difference is exactly the word — "Whenever X or
  Y" never writes a second header word and "When X and whenever Y" always
  does — so the two families are told apart by the surface and this one
  wants a second slot or a second row. The cards are Autarch Mammoth,
  Brinelin, Case File Auditor, Balemurk Leech, MACH-1, Exuberant Fuseling,
  Cryptid Inspector, Inspired Skypainter and fifty more; several are
  otherwise-writable whole cards.
- **The coordinated header's READBACK** — the tail chapter seventy-nine
  measured and did not pay: 144 of the 364 coordinated headers read a
  mention back, and `headerCtx` answers the empty discourse for every
  coordination, so all 144 stay unwritable. The reason is honest — the two
  disjuncts announce different things and the sentence gives no way to say
  which happened — but it is not the whole truth for one pair: the 46
  "blocks or becomes blocked" lines announce the SAME phrase on both
  sides, `eventAfter (Blocks n (Just m))` and `eventAfter (BecomesBlocked
  n (Just m))` both being `nomIntro m`, and "that creature becomes green"
  (Aisling Leprechaun) reads it. What would reach them is a COMMON
  ANNOUNCEMENT — the two after-discourses where they agree — which needs a
  decidable comparison on `Bindings` this vocabulary does not have.
  Whoever takes it should start there and not at the carrier, which is
  already right.
- **The ORDINAL cast restriction** — "Whenever you cast your FIRST spell
  during each opponent's turn" (Alela, Arena Trickster, Blightwing Bandit,
  Dreamstalker Manticore, Mischievous Chimera, Stinging Lionfish,
  Wavebreak Hippocamp and more; 8 headers write the each-opponent's-turn
  form and Rashmi and Ragavan writes the each-of-your-turns one), plus
  "your second card"/"their second spell" (The Council of Four, Moon Girl
  and Devil Dinosaur) and Geralf's "other than your first spell that turn".
  The window landed in chapter seventy-nine and these did not: what they
  restrict is WHICH occurrence counts, an ordinal over the event within
  the window, which is the same ordinal vocabulary the counter event's
  "the fourth plan counter" wants and the library position's "second from
  the top" ([CR#401.7]). Three constructions, one missing word — that is
  the argument for doing them together.
- **"Do this only once each turn"** — 32 supported lines and [CR#603.2h]'s
  own sentence, kept apart from the rider chapter seventy-nine landed and
  worth the entry so the two are never merged. The rule is explicit about
  the difference: "a triggered ability may have an instruction followed by
  'Do this only once each turn'. This ability triggers only if its source's
  controller has not yet taken the indicated action that turn" — a
  restriction on the ACTION, checked against what the controller has done,
  where "This ability triggers only once each turn" restricts the
  TRIGGERING. Screeching Scorchbeast writes it, which is why that card is
  still two gaps and not one. The slot would be a third value on the
  header or a rider on the effect; nothing has measured which.
- **The BARE CARD HEAD** — chapter seventy-eight found that no
  `Predicate` heads a phrase with the word "card" on its own (finding 561)
  and chapter seventy-nine found the card it costs: STONEBINDER'S FAMILIAR
  writes "Whenever one or more cards are put into exile during your turn",
  and both of the pieces chapter seventy-nine landed — the window and the
  limit — are on that card, so it is now blocked on this alone. The
  workaround the zone clause offers does not reach it: `InZone` heads with
  the word ([CR#109.2] naming a zone and the word "card" in one breath),
  and Rakshasa Vizier could carry the source's zone unspelled because its
  event WROTE one, where this card writes no source at all. What is wanted
  is either a bare card head or a reading that lets the DESTINATION
  license the word; measured carriers are Stonebinder's Familiar, Ennis,
  Ketramose, Laelia, The War Doctor, Dutiful Knowledge Seeker and Wan Shi
  Tong, plus the 10 "one or more cards" and 6 "a card" put-into subjects
  chapter seventy-eight counted.
- **The header windows with no `TurnPart` row** — the eight lines chapter
  seventy-nine's grid could not reach, each with its measurement.
  "During your main phase" 3 (Dovin's Acuity, Canyon Vaulter, Reckless
  Velocitaur) names the phase without saying which, where [CR#505.1] gives
  a turn two and this vocabulary has a row for each specific one and none
  for the pair (`badHeaderMainPhaseWindow`) — and chapter one hundred
  twenty MINTED that row (`TurnPart.MainPhase`, on [CR#505.1]'s own
  sentence: the two main phases "are individually and collectively known
  as the main phase") and set `headerWindowOk MainPhase (Just Yours)`
  True, so all three lines are writable. The PIN IS NOT RETIRED: it
  refuses the SPECIFIC row, which is still the wrong one for a card that
  does not say which, and its own docstring's argument is what the new row
  vindicates. Also landed with the row: `EachYours` triggers on it (Carpet
  of Flowers, Frontier Siege), and six delayed headers over it are
  `PartUnclaimed` — "at the beginning of your next main phase" (Mana
  Drain, Mana Sculpt, Scattering Stroke, Pygmy Hippo, Conduit of Storms
  twice) and "at the beginning of the next main phase this turn" (Vivien's
  Stampede), all six waiting on the ORDINAL and not on the part, which is
  a residue of its own. "During the declare attackers
  step" 4 (Misleading Signpost, Portal Mage, Portal Manipulator,
  Windshaper Planetar) wants a `TurnPart` row that does not exist. "During
  each of their turns" 1 wants a possessor `Owner` does not have. All are
  one row apiece and none has a whole card behind it yet.
- **The event rows the REPLACEMENT lines still want** — NARROWED by chapter
  eighty, which minted the token creation and corrected the entry, and by
  chapter seventy-eight, which minted the counter placement and the
  put-into-a-zone arrival. What is left in this entry is the energy family:
  "If you would get one or more {E}, you get twice that many {E} instead"
  (Aether Refinery) routes with the symbol round, energy being a counter a
  PLAYER holds ([CR#107.14]) with no event row and no verb here. The
  remaining non-interceptable cells of `eventUse`, each with its own count
  and its own entry elsewhere: the DESTRUCTION (3 standing "would be
  destroyed" lines, `EventUnclaimed` because regeneration's four-part
  instruction is [CR#614.8] and unwritable), the DAMAGE event (chapter
  ninety-five took [CR#614.9]'s redirection out of this cell entirely, into
  two dedicated rows whose body is fixed rather than general; what is left
  here is the SCALING family and the other "instead" bodies, both blocked on
  the amount readback rather than on the event), the LIFE-CHANGE pair (`EventUnclaimed`, no producer), the
  departure inside a granted quotation, the dice, and what is LEFT of the
  skip family ([CR#614.1b] — its own word, not "instead") now that chapter
  one hundred nineteen has taken the bare skip out of it: the four
  would-worded skips and the six draw-event skips, both counted in the
  turn-schedule entry. None of those is this entry's; they are listed so
  the partition is on the record once.
- **The ENTRY interception's owed bench** — chapter eighty corrected
  `eventUse Entry` to `InterceptedAndTriggered` on [CR#614.1d] and 15
  supported lines, probed the composition green, and benched none of them,
  because each sub-family carries an independent blocker. (a) THE LAND
  CYCLE, 9 cards plus Mox Diamond: "If this land would enter, sacrifice a
  Forest instead. If you do, put this land onto the battlefield. If you
  don't, put it into its owner's graveyard." Two conditionals over ONE
  antecedent, where `Reflexively` writes "if you do" and nothing writes
  "if you don't" — that pair is the whole ask, and every land in the cycle
  also needs mana production, which chapter ninety-one landed. Sheltered
  Valley was recorded here as the exception and the cheapest target —
  "instead sacrifice … , then put this land onto the battlefield" with no
  conditional pair at all, so that mana production alone was said to stand
  between it and the bench. CORRECTED at that landing: its mana line writes
  today, and the card still refuses, because "sacrifice each other permanent
  named Sheltered Valley you control" wants a SACRIFICE clause (no `Effect`
  row spells one; The Golden Throne is blocked on the same thing at a cost
  position) and a name-matching predicate besides. (b) THE CAST-HISTORY CONJUNCT, 4 cards (Containment Priest,
  Hallowed Moonlight, Mistcaller, Primeval Spawn): "and it wasn't cast",
  agentless, over a permanent that is entering — where `CastBy` demands a
  possessor and seeds the stack. It is the same passive the cast-relation
  entry already ledgers from the description side (Errant, Street Artist),
  and Hallowed Moonlight is a two-line instant the moment it lands. (c)
  Gather Specimens wants an ENTRY as a replacement BODY ("it enters under
  your control instead"), which no `Effect` row writes; Don't Blink wants a
  source zone on the entry event plus a disjunction over it.
- **The `idris_emit.rs` OneOrMore arm** — NOT A ROUND: a Rust-side delegate
  task, one match arm at `crates/deckmaste_plugin/src/idris_emit.rs`'s
  `emit_event_filter`, currently `EventFilter::OneOrMore(_) => return
  Err(gap(…))`. It was unmappable while the grammar's counted group and its
  universal left the same mark; `CountD` is the mark that tells them apart.
  Whoever takes it should read finding 534 first — the tag exists precisely
  for this reader, and the arm is its first live consumer.
- **Blood Spatter Analysis** — one card, one row: the bloodstain counter. Its
  header writes as of chapter seventy-five ("Whenever one or more creatures
  die") and its mill writes as of chapter seventy-three, so what remains is a
  flat `CounterKind` row and the card's third and fourth clauses ("Then
  sacrifice it if it has five or more bloodstain counters on it. When you do,
  …" — the reflexive trigger). Lands whenever a round wants it, on finding
  481's bar exactly as Rev did.
- **The PEEK family — CLOSED (chapter ninety), and what it left behind.**
  `Scry` and `Surveil` are `VerbName` rows over `ScryB`/`SurveilB`, whose
  bodies are [CR#701.22a]/[CR#701.25a]'s LOOK with the partition recorded and
  not spelled; the written-out look-and-partition frame the entry treated as
  the family's other half was never a gap at all — chapter forty-one built it
  and Impulse, Anticipate and their neighbours have been benched on it since.
  Both debts the entry carried are paid whole: Nefarious Imp and Saheeli,
  Filigree Master. The entry's twice-repeated warning served its purpose and
  is retired with it.
  FATESEAL IS DECLINED, not deferred: [CR#701.29] is a single paragraph with
  no zero clause, no trigger clause and no neighbour, where its two siblings
  have four sub-rules each, and the corpus matches — two lines, zero triggers,
  zero replacements, zero reads. It is the family's opponent-directed cell and
  it waits for machinery keyed on its name, which is the catalog's admission
  test. Not work; recorded so it is not re-discovered.
  FOUR RESIDUES, each measured (finding 674): the SCRYING SUBJECT (the body
  fixes `You` per [CR#701.22a]'s "your library"; "Target player scries 3" and
  "…scries X" are the only two lines in 440, and zero of 219 surveil lines);
  the PROCESS READ ("the number of cards looked at while scrying this way",
  5 cards — a manner adverbial naming the tag over an inner clause's
  participle); the PER-TURN STATE READ ("as long as you've surveilled this
  turn", Darkblade Agent; Eye of Duskmantle — 2 cards, `Happened` over a
  keyword-action event name); and the COORDINATED EVENT ("whenever you scry or
  surveil", Matoya, Archon Elder and Planetarium of Wan Shi Tong — `AltEvent`'s
  shape over two keyword actions rather than two `GameEvent` rows).
- **The AGENTIVE PLACEMENT CLAUSE — "[player] puts [it] into/onto [zone]"** —
  146 supported lines, measured by chapter one hundred thirty-four when the
  ordinal position's own family split on it. `Move` carries a patient and a
  destination and NO subject; the declarative clause `Does` needs a `VerbName`
  tag, and that catalog's seven rows (Destroy, Sacrifice, Exile, Discard,
  Mill, Scry, Surveil) have no "put". So every line whose placement names its
  performer is unwritable: "the owner of target nonland permanent puts it into
  their library second from the top" (Deem Inferior and 5 more), "that player
  puts that card into their library third from the top" (Lost Hours), "its
  owner puts it on their choice of the top or bottom of their library" (Aether
  Gust and 30 more), "that player puts it onto the battlefield" (11 lines),
  "puts it into their hand" (6). The question the round must answer first is
  WHICH of the two shapes it is — a `Put` row in `VerbName` with a `TagBody`
  over `Move` (which is how mill and the three keyword actions reach their
  placements), or a subject slot on `Move` itself — and the evidence to weigh
  is that the imperative and the agentive spell the SAME event, where
  Destroy's two frames already ride one `TagBody`. Note the boundary: the
  destination's own possessive is rendering's business ([CR#400.3], finding
  34), so "their library" costs nothing here.
- **The LIBRARY POSITION DISJUNCTION — "the top or bottom"** — 38 supported
  lines over two spellings, measured by chapter one hundred thirty-four and
  deliberately NOT built into the ordinal offset it half-touches. The
  dominant spelling names its chooser: "its owner puts it on their choice of
  the top or bottom of their library" (32 lines — Aether Gust, Aetherspouts,
  Desynchronize, Dire Downdraft, Diver Skaab and 27 more). The other six write
  the disjunction against an OFFSET: "puts it into their library second from
  the top or on the bottom" (Deem Inferior, Happy Hogan, Lost Days, Temporal
  Cleansing, Trickster's Stratagem, Wan Shi Tong). Three facts the round
  should start from. The chooser phrase is SEPARABLE — Write into Being writes
  the bare disjunction ("put the other on the top or bottom of your library")
  — so "on their choice of" is its own slot and not part of the coordination.
  The disjunction is of POSITIONS and not of destinations: the only other
  two-destination lines in the corpus are Illuna's and Green Sun's Twilight's
  "onto the battlefield or into your hand", 2 lines, which is a different and
  much smaller cell. And 34 of the 38 carry the agentive placement clause of
  the entry above, so most of this entry is blocked behind that one — the
  four imperatives (Arashin Sovereign, Hinder, Not Forgotten, Write into
  Being) are what a round taking this one alone could land, and three of the
  four name their chooser with "on your choice of".
- **The double counted group** — Hostile Investigator's "Whenever one or more
  players discard one or more cards", where BOTH slots are counted groups and
  the first is a PLAYER. The noun row is kind-general and takes the player
  slot already, so this is not a grammar question; the block is `investigate`,
  a keyword action that creates a named token. Recorded so the composition is
  not re-derived: two `CountedGroup`s in one event is ordinary and needs
  nothing.
- **"One of your opponents": spelling or row** — NARROWED to one question by
  chapter seventy-six, which refused the partitive widening. The phrase is 32
  lines and its covariance with the clause's subject is nearly total (finding
  547: 15/15 and 8/8 one way, 3/3 and 80/83 the other), so it is very probably
  a SPELLING of `Indefinite Opponent` — "an opponent" where the subject is
  you-anchored, "one of your opponents" where it is not. What stops the round
  from settling it is three lines: Fiendish Duo, Gisela, Blade of Goldnight and
  Rem Karolus, Stalwart Slayer all write "an opponent" under a bare "a
  source"/"a spell" in a REPLACEMENT static, which is the wrong side of the
  covariance. Whoever takes it should decide whether the replacement frame is a
  second environment with its own rule (in which case the spelling stands and
  the renderer owes it) or a genuine crossing (in which case the determiner is
  a row after all). `groupMention (PlayerGroup _)` stays False either way — its
  own reader measured 8 lines against `Each Opponent`'s mass, and chapter
  seventy-six did not disturb that. WITNESSES: Frenzied Gorespawn is now ONE
  gap from whole (its defender slot exists; it needs Menace, an open-catalog
  data row, plus this decision), and Frontier Warmonger needs a cross-kind
  disjunction besides ("one of your opponents OR a planeswalker they control").
- **The planeswalker and battle defenders** — [CR#506.3] admits three things as
  attackable and chapter seventy-six rowed one. The CARD TYPES are no longer
  the blocker: chapter eighty-six gave `deedType`'s `Attack Patient` row its
  first two True cells, so the deed table admits both and what remains is the
  EVENT header alone. The planeswalker is ONE header (Oath of Kaya) and it
  writes the two coordinated, so the header and the cross-kind disjunction
  arrive together or not at all; battles are zero headers. Recorded so the
  narrowing is not re-derived as an oversight.
- **The designation catalog's remaining rows** — chapter seventy-one minted
  fourteen rows covering thirteen of `docs/designations.md`'s twenty-one
  entries and left eight, each with its own measurement. THREE have no
  supported card text at all: protector, planar controller, archenemy. ONE is
  reminder-only — the three sector designations appear inside space sculptor's
  reminder on Space Beleren and nowhere else, so like Mount they are neither
  rowed nor pinnable. FOUR are written through a keyword action or an ability
  container rather than through either reader, and so route with the keyword
  entry above: harnessed ("Harness The Mind Stone", two cards, [CR#701.64a]),
  solved (a Case's "Solved —" label, 13 cards), level (a Class's level bars, 68
  occurrences over 34 cards) and the unlocked pair (lock and unlock
  instructions, 110 over 79, [CR#709.5c]). None needs a design decision: the
  catalog is open, so each is a row and a handful of cells on the round that
  benches one.
- **The CARD scope's reader** — chapter seventy-one gave the commander a scope
  cell ([CR#903.3]: the designation "is an attribute of the card itself", which
  is why it survives a zone change) and no reader, because neither of the two
  forms the corpus writes is one. "Your commander" is 84 supported occurrences
  over 81 cards written as a possessed NOUN in subject position, which is the
  biggest single designation family in the corpus and wants a noun row, not a
  predicate; "is your commander" is 3 lines and all three sit inside a
  before-the-game static ability finding 188 keeps unread. Two entries' worth
  of work behind one scope cell.
- **The designation ABSENCE check** — "there is no monarch", five lines, which
  asks whether ANY player holds a designation rather than describing one that
  does. An existential over the holder, sibling to the other absence checks,
  and the one negative the designation family writes at all (every "isn't the
  monarch"-shaped negation measures zero; one "isn't saddled" line is the
  exception and rides here).
- **Game restart** — MINT QUEUED. Chapter thirty-one declined on the honest
  count ([CR#727.1], one corpus line) and that decline is overridden: a small
  late round.

## Protocol bundles — must be designed whole, misleading to cost row by row

- The draft.
- The secret-choice and voting pair.
- Pile partitions ([CR#700.3a,700.3c,700.3d]; Death or Glory partitions the
  graveyard, so this is not the library gap).
- Bidding.
- End-the-turn.

## Spelling residues — semantics exist, only a surface is missing

- Clause ellipsis, including Galvanic Blast's replacement clause writing no
  recipient.
- Effect disjunction.
- The bare generic plural (finding 223).
- The definite determiner over a DESCRIPTION — what "the card exiled with this
  artifact" needs, and what makes finding 206's second surface unwritable.

## Vocabulary rows — census, cheap, and candidates for the Rust side

- **The nominal possessor tier** — what chapter forty-five's possessor round
  left: "enchanted player's upkeep", "its controller's end step", and the Curse
  and Aura mass, which write a possessive over a NOUN where that round minted
  quantifier WORDS (`Owner`). Wants an anaphoric-noun possessor, which is also
  what would let a quantifier possessor be an antecedent — the gap that costs
  `EachOpponents` its bench positive despite 33 corpus lines (finding 330).
- **The activation window's boundary-relative forms** — "during your turn,
  before attackers are declared" (19 lines) and the bare "before attackers are
  declared" (24). Chapter forty-five's `DuringPart` names a part to be inside;
  these name a point to be before, which is a different slot.
- Participle verb tags beyond the four.
- Entry and exile counter NAME tails — measured and NOT taken (finding 229);
  the flat names are left to the bar that governs them.
- Card-level creation check (finding 210) and the named-source linkage
  ([CR#607.2n], three lines).

## Not rounds — delegate tasks

- **Keyword classification audit.** `docs/keyword-policy.md` declares 9 intrinsic
  (5 implemented), 19 composite-given, 215 composite, 1 marker. The user
  estimates ~90% confidence and suspects stragglers that should be composite or
  one or two missed. The rubric is written down, so each entry is checkable.
  Note the descriptive classification is pinned to mtg-rules skill v1.7.0 —
  confirm that pin is current first, since a stale pin is a plausible source of
  the misses.
- **Ability words.** Settled by user ruling: `AbilityWord Name Ability`, word
  retained because it prints, shape correlation authored as a macro. Needs the
  attested inventory measured, then transcription — no designer dispatch.

## Out of scope until the verifier transition

Adding keywords that will eventually become macros. The new Idris model is
intended to mirror the macro shape more accurately (string labels for
composites); Flying and Haste are deliberate existing passes. Already-implemented
keywords may be USED where they unlock better `Cards.idr` witnesses.
- **The KEYWORD-LIST extension — LANDED, chapter one hundred thirty-one, and
  this entry counted one of its own neighbours twice.** SIXTEEN IS FIFTEEN:
  Ward of Bones carries no keyword list at all, its only "same is true"
  clause being the card-type extension that the entry immediately below files
  correctly — one card in two families inside one section. (Finding 846's
  total of 25 lines was right; its family sizes 9 + 16 + 1 were not.) The
  wrapper reading held and the "0 on their own line" measurement reproduced
  (14 of 15 lines are two sentences with the trailer second; Indominus Rex,
  Alpha is the one three-sentence line). THE CARRIER MOVED: the row sits on
  `AbilityAt`, not on the statement, because this family repeats a STATIC
  line on nine cards and a TRIGGERED one on five and the unit the trailer
  never crosses is the line. THE BASE WAS NEVER THIS ROW'S — the
  keyword-conditional grant elaborated before the round began, both carriers,
  and the corpus writes it with no trailer at all (Mutual Destruction, Manor
  Gargoyle), so the row carries only the list. Landed as
  `AlsoForKeywords ab ks` with the list measured nonempty (3..12), distinct,
  paramless and disjoint from the base word (15/15 — ODRIC, whose base is
  first strike and whose list holds flying, is what makes the last one
  visible). Catalog: `Hexproof`, `Menace`, `Skulk` bought, each spent by a
  benched card. ODRIC LUNARCH MARSHAL, BLEEDING EFFECT and URBORG SCAVENGERS
  benched whole. Residues:
  - **The keyword-CLASS entry, 7 cards** — "protection" bare (Cairn Wanderer,
    Concerted Effort, Death-Mask Duplicant, Eater of Virtue, Rayami, Wretched
    Bonemass), "protection from any color" (Escaped Shapeshifter) and
    "landwalk" (Cairn Wanderer, Concerted Effort, Death-Mask Duplicant). Not a
    word but a QUANTIFIER over words — every landwalk keyword, every
    protection quality — which the list's paramless demand refuses and which
    `HasKeyword` refuses one position over, so the base sentence cannot be
    written for them either. Attested, so counted rather than pinned; it wants
    a keyword-class term this grammar has at no site. Eight of the fifteen
    lists are clean, seven are not.
  - **Indominus Rex, Alpha** — the list quantifies over which COUNTER to
    place, not which keyword to grant; a different base, routed with it. Also
    the family's only three-sentence line.
  - **`fear` (3 lines) and `shroud` (1)** — paramless and mintable, deliberately
    not bought: their only carriers are the class-blocked seven.
  - **The unbenched clean carriers** — Animus of Predation (a draft-time
    read), Soulflayer (delve's linkage phrase, "exiled with this creature's
    delve ability"), Majestic Myriarch (a doubling CDA), Thunderous Orator
    (wants `Subtype.Kor`) — each one line away, none blocked on this row.
- **Ward of Bones' card-type extension** — "Each opponent who controls more
  creatures than you can't cast creature spells. The same is true for
  artifacts and enchantments." ONE card, and the third construction sharing
  the anaphor (finding 846): it extends a PROHIBITION across card types where
  one extends across zones and the other across keywords. One line; nothing to
  design from until a second carrier appears. Chapter one hundred thirty-one
  confirms the count is one and that the entry above had been carrying this
  same card as a keyword-list carrier as well; the two entries now agree.
