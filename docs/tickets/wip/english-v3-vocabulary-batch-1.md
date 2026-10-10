---
needs: []
---
# Vocabulary batch 1: declare the commonest unknown words that existing routes already read

Priority: high. Data only. This ticket is deliberately a batch: the
one-construction-per-ticket rule is about grammar changes, and a declaration
an existing Construction already reads is not one. Owner decision 2026-10-10.

## Why

At the per-line baseline (`sskyxnyvmruk`; file
`/home/msmorgan/Projects/deckmaste.rs/target/english-v3/baseline-per-line.json`)
2,027 of 6,292 unread faces contain at least one word with no lexical
reading. The commonest, by unread faces (surface counts, not gain
forecasts; *divided* and *produce(d)* have since landed):

*starting* 84, *half* 83, *rounded* 77, *historic* 56, *tied* 51, *declared*
48, *modified* 47, *exactly* 45, *party* 40, *take* 40, *kind* 35, *pile(s)*
34 + 27, *door* 33, *difference* 32, *move* 32, *mutates* 32, *using* 32,
*excess* 30, *deck* 28, *crime* 27, *opening* 27, *paired* 27, *result* 26,
*begin* 25, *crews* 25, *switch* 25, *true* 25, *draft* 24, *promised* 24,
*exploits* 23, *bargained* 20, *unspent* 20, *commit* 19, *empower* 19,
*multikicker* 19, *completed* 17.

Each has so far cost a whole ticket (*produce*: 1 h 43 min for 43 faces).

## Goal

Declare every candidate word that is **vocabulary-only**: a word whose
witness reads once the word is known, through Constructions and frames that
exist today. Words that need a new Construction or frame kind are not
implemented; they are listed for follow-up tickets.

## Analysis

Orchestrator rulings 2026-10-10:

- V1. **Candidates.** Take the unknown words of the unread faces in the
  baseline file above (`.faces[] | select(.census == "no") | .unknown_words`),
  lower-cased and grouped by lemma, and work down from the most frequent
  until 60 words have been declared or the next word touches fewer than 8
  unread faces. Skip any word already known on the claim parent.
- V2. **The vocabulary-only test.** For each candidate pick two attested
  units (`cargo xtask english-v3 grep`, `--as-card` probes). Substitute a
  declared word of the category and frame you intend to give it (the
  *produce* ticket's control was *add*) and probe. If the substituted unit
  reads at least once with the intended analysis, the candidate is
  vocabulary-only: declare it. If no substitution reads, it is not: record
  the word, its face count and the missing piece in the follow-up list and
  move on. Do not build a Construction, a frame kind, a Role or a feature in
  this ticket.
- V3. **Where the declaration goes.** A keyword action, keyword ability or
  other game term owned by a `plugins_v2` definition (*mutate*, *crew*,
  *exploit*, *bargain*, *multikicker*, *historic*, *party*, *modified*, …)
  is declared through that definition's `grammar:` field or the macro that
  declares the term, never as a free-standing core word. Ordinary English
  words go in `lexicon/verbs.ron` or `lexicon/core.ron`, in the shortest
  form the loader accepts; forms the loader derives are exempt from the
  pruning rule (ruling R16e, `english-v3-produce-verb`).
- V4. **Attested frames and categories only.** A verb gets the frames its
  corpus uses need, copied from an existing verb with the same
  complementation; a noun its attested countability; an adjective its
  attested positions. Categories follow CGEL, cited only where the passage
  says it (for example *exactly* as a focusing modifier), otherwise cited as
  this ruling.
- V5. **One lexical reading per use.** A new declaration must not give an
  already-covered face a second Reading by homography with an existing
  lexeme. Any Reading-count increase on a previously covered face is a
  defect: narrow the declaration or drop the word to the follow-up list.
- V6. *half* / *rounded up/down*, *exactly N*, *tied for*, *the difference
  between* and similar are expected to fail V2 because they need a
  Construction. That is the intended outcome for them: list, do not build.

- V7. **Unavailable declaration paths.** Orchestrator ruling 2026-10-10:
  any game term whose owning plugin definition has no supported declaration
  path today is deferred under **passes V2, blocked on declaration path**,
  with owner, loader exclusion, face count and substitution evidence. Do not
  declare it in core or change the loader. List and skip any individual word
  that cannot be declared cleanly under V1–V7. STOP only if fewer than ten
  words are declarable at all.

- V8. **Unblocked ownership exception.** Orchestrator ruling 2026-10-10: *unblocked* remains in core by the existing colorless / monocolored / multicolored precedent, although the predicate family owns those terms. This is a V3 deviation pending a follow-up that gives the predicate family a declaration path.

- V9. **Predicative restriction deferred.** Orchestrator ruling 2026-10-10: keep starting, opening, excess, highest and lowest in AttributiveAdjective; defer the predicative restriction and its two zero-Reading negatives. No grammar changes.

## Witnesses

Chosen during the work under V2 and recorded per word: two units per declared
word, each 0 before and at least 1 after with the intended analysis.

## Method

Standard constraints apply. Deltas:

1. Verification budget: one bounded census after all declarations, on every
   supported face containing any declared word (`cargo xtask english-v3 grep
   -i --names`, one pattern per word, unioned) plus 300 random covered
   controls, `--workers 6 --samples-per-face 0` under `/usr/bin/time -v`,
   compared with `cargo xtask english-v3 diff` against the claim parent on
   the same manifest. Zero lost faces; zero increases on covered faces (V5).
2. Tests: one table-driven test over the declared words (each row: card,
   unit, category, expected Reading count, one asserted constituent), plus
   the lexical-source test for each new declaration class. No test pins a
   sample cost or scans a Debug-printed tree.
3. Full `cargo test -p` and clippy for `deckmaste_lexical_source`,
   `deckmaste_english_v3` and any plugin-consuming crate whose data changed;
   `cargo xtask lean check` only if a plugin definition's non-`grammar:`
   content changed (it must not). `cargo xtask facts check` if a keyword
   definition gained a `grammar:` field.
4. No guard may name a lexeme, construction, verb, noun, preposition or
   card. No grammar edits at all; a needed one sends the word to the
   follow-up list.

## Out of scope

- Any Construction, frame kind, Role or feature.
- Words below the V1 cut-off; they are batch 2, chosen from
  `english-v3-blocker-report` output once it exists.

## Landing record

1. **Census delta** — measured on `nouqyyouuops` against the data of claim parent `tutzxorzmwws`: 3,702 faces, covered 2,207 → 2,628, Readings 122,516 → 164,117; 421 gained, no lost faces, decreases, covered-face increases, digest-only changes or source changes. Forty-one words/lemma groups declared. The selector is the union of the 41 case-insensitive attestation patterns (all declared inflections and compound surfaces) in `target/english-v3/english-v3-vocabulary-batch-1/selectors.json`: 3,417 surface faces plus 300 covered controls chosen by built-in `diff --manifest --random 300 --seed 20261008`, 3,702 distinct identities in `target/english-v3/english-v3-vocabulary-batch-1/manifest.json`. Full evidence: `before.json`, `after.json`, `diff.json` and `diff.log` there. Counts below are distinct unread faces with an unknown form of the lemma in the supplied V1 baseline; gains are intersections with the built-in diff, and can overlap between words. Categories are attributed to project ruling V4, not to an uncited CGEL passage. Post-refresh measurement `oxtqzporvvmw` on the same manifest adds seven trunk gains: covered 2,628 → 2,635 and Readings 164,117 → 165,511, with zero losses, decreases, covered-face increases, digest-only or source changes (`diff-refresh.json`). Against the original parent data the final totals are covered 2,207 → 2,635 and Readings 122,516 → 165,511, 428 gained (`diff-final.json`); 421 are this batch’s gains and seven are trunk’s.

   | Word / lemma group | Category / frame or feature | Declaration owner / source | V1 unread faces | Gained faces containing a declared surface |
   |---|---|---|---:|---:|
   | declared | Verb / transitive | `core-verb:Declare` in `verbs.ron` | 49 | 7 |
   | take | Verb / transitive | `core-verb:Take` in `verbs.ron` | 48 | 55 |
   | pile | Count noun | `lexeme:CommonNoun/Pile` in `core.ron` | 38 | 12 |
   | door | Count noun | `lexeme:CommonNoun/Door` in `core.ron` | 36 | 29 |
   | move | Verb / ObjectNP + Onto + ObjectNP | `core-verb:Move` in `verbs.ron` | 34 | 28 |
   | excess | adjective | `vocab:AttributiveAdjective/Excess` in `lexicon/vocabulary.rs` | 30 | 25 |
   | deck | Count noun | `lexeme:CommonNoun/Deck` in `core.ron` | 28 | 14 |
   | opening | adjective | `vocab:AttributiveAdjective/Opening` in `lexicon/vocabulary.rs` | 27 | 20 |
   | result | Count noun | `lexeme:CommonNoun/Result` in `core.ron` | 28 | 6 |
   | begin | Verb / transitive, intransitive / intransitive: finite present third singular only | `core-verb:Begin + core-verb:BeginIntransitive` in `verbs.ron + core.ron (finite intransitive) + overrides.ron (beginning)` | 30 | 20 |
   | secretly | adverb / VPFinalAdjunct=No, ClauseInitialAdjunct=No, CentralAdjunctLicence=Imperative | `vocab:Adverb/Secretly` in `core.ron` | 17 | 1 |
   | completed | Verb / transitive | `core-verb:Complete` in `verbs.ron` | 21 | 15 |
   | unblocked | adjective | `vocab:Adjective/Unblocked` in `core.ron` | 17 | 13 |
   | fully | adverb / VPFinalAdjunct=No, ClauseInitialAdjunct=No, CentralAdjunctLicence=Preverbal | `vocab:Adverb/Fully` in `core.ron` | 16 | 14 |
   | text | Mass noun | `lexeme:CommonNoun/Text` in `core.ron` | 15 | 0 |
   | unspent | adjective | `vocab:Adjective/Unspent` in `core.ron` | 20 | 6 |
   | note | Verb / transitive | `core-verb:Note` in `verbs.ron` | 16 | 1 |
   | instances | Count noun | `lexeme:CommonNoun/Instance` in `core.ron` | 14 | 0 |
   | life total | Count noun / compound life total / ScalarDenotation=Yes | `lexeme:CommonNoun/LifeTotal` in `core.ron` | 8 | 42 |
   | team | Count noun | `lexeme:CommonNoun/Team` in `core.ron` | 14 | 10 |
   | activation | Count noun / NounPremodifier=Yes | `lexeme:CommonNoun/Activation` in `core.ron` | 13 | 3 |
   | followed | Verb / transitive | `core-verb:Follow` in `verbs.ron` | 11 | 6 |
   | odd | adjective | `vocab:Adjective/Odd` in `core.ron` | 15 | 4 |
   | even | adjective | `vocab:Adjective/Even` in `core.ron` | 17 | 5 |
   | lowest | adjective | `vocab:AttributiveAdjective/Lowest` in `lexicon/vocabulary.rs` | 8 | 3 |
   | suspended | participial adjective | `lexeme:keyword_ability/suspend/adjective` in `plugins_v2/builtin/macros/keyword_abilities/suspend.ron` | 16 | 12 |
   | bargained | participial adjective | `lexeme:keyword_ability/bargain/adjective` in `plugins_v2/builtin/macros/keyword_abilities/bargain.ron` | 20 | 16 |
   | warped | participial adjective | `lexeme:keyword_ability/warp/adjective` in `plugins_v2/builtin/macros/keyword_abilities/warp.ron` | 15 | 14 |
   | colored | adjective | `vocab:Adjective/Colored` in `core.ron` | 9 | 4 |
   | hasn't/haven't | finite present negative perfect auxiliary / PastParticiplePredicate | `core-verb:HaveNegative` in `core.ron` | 33 | 14 |
   | identity | Count noun / compound color identity | `lexeme:CommonNoun/ColorIdentity` in `core.ron` | 9 | 5 |
   | symbols | Count noun / compound mana symbol | `lexeme:CommonNoun/ManaSymbol` in `core.ron` | 15 | 12 |
   | kind | Count noun | `lexeme:CommonNoun/Kind` in `core.ron` | 45 | 5 |
   | highest | adjective | `vocab:AttributiveAdjective/Highest` in `lexicon/vocabulary.rs` | 14 | 3 |
   | separate | Verb / into | `core-verb:Separate` in `verbs.ron` | 26 | 8 |
   | cadet | Catalog / IdentityUse=Name, exact / NameNumber=Singular | `lexeme:Name/Cadet` in `core.ron` | 13 | 8 |
   | starting | adjective | `vocab:AttributiveAdjective/Starting` in `lexicon/vocabulary.rs` | 84 | 10 |
   | word | Count noun / compound color word | `lexeme:CommonNoun/ColorWord` in `core.ron` | 10 | 0 |
   | right | Count noun | `lexeme:CommonNoun/Right` in `core.ron` | 13 | 1 |
   | switch | Verb / transitive | `core-verb:Switch` in `verbs.ron` | 25 | 25 |
   | friends | Count noun | `lexeme:CommonNoun/Friend` in `core.ron` | 14 | 0 |

2. **Witnesses** — measured on `nouqyyouuops`. Every listed unit failed before declaration (0 Readings), and its declared-word substitution read with the intended category/frame. The before/control/after JSON paths, categories and exact units are in `target/english-v3/english-v3-vocabulary-batch-1/declared-after.json`. Each table row is also in the shared integration fixture.

   | Word | Attested card / unit | After Readings |
   |---|---|---:|
   | declared | Rapid Fire: `Cast this spell only before blockers are declared.` (document) | 2 |
   | declared | Master Warcraft: `Cast this spell only before attackers are declared.` (document) | 2 |
   | take | Time Walk: `Take an extra turn after this one.` (document) | 2 |
   | take | Time Warp: `Target player takes an extra turn after this one.` (document) | 2 |
   | pile | Fact or Fiction: `one pile` (noun-phrase) | 2 |
   | pile | Mangara's Tome: `the exiled pile` (noun-phrase) | 1 |
   | door | Underwater Tunnel: `When you unlock this door, surveil 2.` (document) | 1 |
   | door | Ticket Booth: `When you unlock this door, manifest dread.` (document) | 1 |
   | move | Fate Transfer: `Move all counters from target creature onto another target creature.` (document) | 1 |
   | move | Leech Bonder: `Move a counter from target creature onto a second target creature.` (document) | 1 |
   | excess | Flame Spill: `Excess damage is dealt to that creature's controller instead.` (document) | 3 |
   | excess | Pigment Storm: `Excess damage is dealt to that creature's controller instead.` (document) | 3 |
   | deck | Rat Colony: `A deck can have any number of cards named Rat Colony.` (document) | 5 |
   | deck | Seven Dwarves: `A deck can have up to seven cards named Seven Dwarves.` (document) | 2 |
   | opening | Leyline of Lightning: `your opening hand` (noun-phrase) | 1 |
   | opening | Leyline of Hope: `your opening hand` (noun-phrase) | 1 |
   | result | Clay Golem: `the result` (noun-phrase) | 1 |
   | result | Netherese Puzzle-Ward: `the result` (noun-phrase) | 1 |
   | begin | Ugin's Nexus: `If a player would begin an extra turn, that player skips that turn instead.` (document) | 3 |
   | begin | Gerrard's Hourglass Pendant: `If a player would begin an extra turn, that player skips that turn instead.` (document) | 3 |
   | begin | Clara Oswald: `the game begins` (finite-clause) | 1 |
   | begin | The Prismatic Piper: `the game begins` (finite-clause) | 1 |
   | secretly | Guardian Archon: `As this creature enters, secretly choose an opponent.` (document) | 1 |
   | secretly | Stalking Leonin: `When this creature enters, secretly choose an opponent.` (document) | 1 |
   | completed | Gloom Stalker: `As long as you've completed a dungeon, this creature has double strike.` (document) | 1 |
   | completed | Nadaar, Selfless Paladin: `Other creatures you control get +1/+1 as long as you've completed a dungeon.` (document) | 4 |
   | unblocked | Curtain of Light: `Target unblocked attacking creature becomes blocked.` (document) | 1 |
   | unblocked | Dazzling Beauty: `Target unblocked attacking creature becomes blocked.` (document) | 1 |
   | fully | Entity Tracker: `Eerie — Whenever an enchantment you control enters and whenever you fully unlock a Room, draw a card.` (document) | 2 |
   | fully | Skullsnap Nuisance: `Eerie — Whenever an enchantment you control enters and whenever you fully unlock a Room, surveil 1.` (document) | 2 |
   | text | Alter Reality: `the text of target spell or permanent` (noun-phrase) | 1 |
   | text | Glamerdye: `the text of target spell or permanent` (noun-phrase) | 1 |
   | unspent | Omnath, Locus of Mana: `unspent green mana you have` (noun-phrase) | 3 |
   | unspent | Electro, Assaulting Battery: `unspent red mana` (noun-phrase) | 1 |
   | note | Bell Borca, Spectral Sergeant: `the greatest number noted for it this turn` (noun-phrase) | 2 |
   | note | Bell Borca, Spectral Sergeant: `Note the mana value of each card as it's put into exile.` (document) | 2 |
   | instances | Alter Reality: `all instances` (noun-phrase) | 1 |
   | instances | Swirl the Mists: `All instances` (noun-phrase) | 1 |
   | life total | Soul Conduit: `Two target players exchange life totals.` (document) | 1 |
   | life total | Axis of Mortality: `At the beginning of your upkeep, you may have two target players exchange life totals.` (document) | 1 |
   | team | Rushblade Commander: `Warriors your team controls have haste.` (document) | 1 |
   | team | Sylvia Brightspear: `Dragons your team controls have double strike.` (document) | 1 |
   | activation | Quicksilver Elemental: `the activation costs of this creature's abilities` (noun-phrase) | 2 |
   | activation | Manascape Refractor: `the activation costs of this artifact's abilities` (noun-phrase) | 2 |
   | followed | Seize the Day: `an additional combat phase followed by an additional main phase` (noun-phrase) | 20 |
   | followed | Relentless Assault: `an additional combat phase followed by an additional main phase` (noun-phrase) | 20 |
   | odd | Obosh, the Preypiercer: `odd mana values` (noun-phrase) | 1 |
   | odd | Obosh, the Preypiercer: `cards with odd mana values and land cards` (noun-phrase) | 5 |
   | even | Gyruda, Doom of Depths: `even mana values` (noun-phrase) | 1 |
   | even | Gyruda, Doom of Depths: `an even mana value` (noun-phrase) | 1 |
   | lowest | Culling Scales: `the lowest mana value` (noun-phrase) | 1 |
   | lowest | Culling Scales: `At the beginning of your upkeep, destroy target nonland permanent with the lowest mana value.` (document) | 5 |
   | suspended | Dust of Moments: `each suspended card` (noun-phrase) | 1 |
   | suspended | Timecrafting: `suspended card` (nominal) | 1 |
   | bargained | Ice Out: `This spell costs {1} less to cast if it's bargained.` (document) | 3 |
   | bargained | Hamlet Glutton: `This spell costs {2} less to cast if it's bargained.` (document) | 3 |
   | warped | Close Encounter: `a warped creature card you own in exile` (noun-phrase) | 9 |
   | warped | Temporal Intervention: `a spell was warped this turn` (finite-clause) | 1 |
   | colored | Void Mirror: `Whenever a player casts a spell, if no colored mana was spent to cast it, counter that spell.` (document) | 2 |
   | colored | Emblazoned Golem: `colored mana` (noun-phrase) | 1 |
   | hasn't/haven't | Stoic Sphinx: `This creature has hexproof as long as you haven't cast a spell this turn.` (document) | 5 |
   | hasn't/haven't | Jem Lightfoote, Sky Explorer: `At the beginning of your end step, if you haven't cast a spell from your hand this turn, draw a card.` (document) | 5 |
   | identity | Command Tower: `your commander's color identity` (noun-phrase) | 1 |
   | identity | Commander's Sphere: `your commander's color identity` (noun-phrase) | 1 |
   | symbols | Primalcrux: `green mana symbols` (noun-phrase) | 1 |
   | symbols | Umbra Stalker: `black mana symbols` (noun-phrase) | 1 |
   | kind | Hundred-Battle Veteran: `different kinds of counters` (noun-phrase) | 2 |
   | kind | Aragorn, Company Leader: `each of those kinds of counters` (noun-phrase) | 2 |
   | highest | Arbiter of Knollridge: `the highest life total among all players` (noun-phrase) | 2 |
   | highest | Arbiter of Knollridge: `When this creature enters, each player's life total becomes the highest life total among all players.` (document) | 2 |
   | separate | Liliana of the Veil: `Separate all permanents target player controls into two piles.` (document) | 1 |
   | separate | Do or Die: `Separate all creatures target player controls into two piles.` (document) | 1 |
   | cadet | Craftwork Crusher: `Create a 2/2 colorless Wizard Soldier creature token named Cadet.` (document) | 6 |
   | cadet | Peer Review: `Create a 2/2 colorless Wizard Soldier creature token named Cadet.` (document) | 6 |
   | starting | Angel of Destiny: `your starting life total` (noun-phrase) | 1 |
   | starting | Resolute Archangel: `your starting life total` (noun-phrase) | 1 |
   | word | Alter Reality: `one color word` (noun-phrase) | 2 |
   | word | Glamerdye: `one color word` (noun-phrase) | 2 |
   | right | Hithlain Rope: `your right` (noun-phrase) | 1 |
   | right | Bucknard's Everfull Purse: `your right` (noun-phrase) | 1 |
   | switch | Dwarven Thaumaturgist: `{T}: Switch target creature's power and toughness until end of turn.` (document) | 2 |
   | switch | Valakut Fireboar: `Whenever this creature attacks, switch its power and toughness until end of turn.` (document) | 3 |
   | friends | Master of Ceremonies: `friends` (noun-phrase) | 1 |
   | friends | Master of Ceremonies: `For each player who chose friends, you and that player each create a 1/1 green and white Citizen creature token.` (document) | 1 |

3. **Spot checks** — measured on `nouqyyouuops`: fifteen gained faces across thirteen words. Each full analysed face was probed for a retained Reading and exact roundtrip; selected grammatical analyses were inspected (`target/english-v3/english-v3-vocabulary-batch-1/spot-checks.json`, `spot-0.json` through `spot-14.json`).

   - Time Walk (2 census Readings): imperative Take with turn ObjectNP.
   - Time Warp (2 census Readings): finite transitive Take, target-player subject.
   - Underwater Tunnel (1 census Readings): Door noun ObjectNP of unlock in a When trigger.
   - Ticket Booth (1 census Readings): Door noun ObjectNP of unlock, with manifest-dread consequent.
   - Fate Transfer (1 census Readings): Move with counter ObjectNP and onto-marked destination.
   - Flame Spill (3 census Readings): adjective premodifier of damage, passive deal.
   - Rat Colony (9 census Readings): count noun deck as subject of can-have.
   - Leyline of Lightning (17 census Readings): adjective premodifier of hand in initial conditional.
   - Ugin's Nexus (71 census Readings): passive Begin with extra-turn subject inside if clause.
   - Guardian Archon (4 census Readings): central imperative adverb before choose.
   - Gloom Stalker (1 census Readings): perfect auxiliary plus transitive Complete dungeon.
   - Curtain of Light (5 census Readings): adjective premodifier of attacking creature.
   - Entity Tracker (3 census Readings): central preverbal adverb before unlock.
   - Soul Conduit (1 census Readings): compound plural life totals ObjectNP of exchange.
   - Rushblade Commander (1 census Readings): team noun possessor of the controlling subject relative.

4. **Deviations and STOPs** — measured on `nouqyyouuops`. V7 resolves the declaration-path STOP raised for historic; V9 resolves the review STOP about predicative use of the attributive class. Both rulings are recorded above. No Construction, frame kind, Role, feature, grammar implementation, loader, non-grammar plugin content or glossary term changed; the reviewed, unattested intransitive Move route was retired from lexicon data and its fixtures; it introduced no grammar route to retire. The finite negative auxiliary has only the six attested present-concord cells, no invented negative plain/nonfinite/past forms. Starting is an adjective; no Start verb was added. The finite Begin intransitive use remains. Review retired Move’s unframed Predicate declaration and its two synthetic mid-clause fragment fixtures (Skullbriar / Me, the Immortal: it moves), because the attested move-to-zone clauses remain unread. Move now selects ObjectNP + Onto + ObjectNP only; the lexical-source frame test matches that declaration. An unrestricted Begin intransitive frame caused V5 increases on Cyclonus, Cybertronian Fighter (14→26), Shadow of the Second Sun (7→13), and Sphinx of the Second Sun (7→13): verbal beginning competed with the existing beginning-phase term. It was narrowed to a native finite present/third/singular BeginIntransitive declaration, with no gerund/plain/nonfinite/past cells. The ordinary transitive Begin keeps its full paradigm. The bounded recheck restores the three faces to their parent counts; both new finite witnesses remain at 1. Compound life total, color identity, mana symbol and color word have no lexical-owning plugin declaration; semantic amount/instruction accessors are not lexical declarations. The ordinary friend noun is distinct from the unavailable Friends-forever keyword.

   The supplied V1 file actually stamps `yntwupzymxxl`, despite the old ticket label `sskyxnyvmruk`, and predates the claim parent’s Produce/Divided landings. Therefore the parent was replayed on the bounded manifest with the prebuilt feature binary, temporarily restoring byte-verified `jj file show -r @-` parent data and restoring feature data in `finally`. The census metadata stamps the feature change id; `before.json` contains claim-parent data, not the feature declarations. A provisional bounded result was superseded after the final friend declaration spelling corrections (beginning, switches), and the Move/Begin intransitive frames; only bounded rechecks were used. No full-corpus run, custom comparison script or raw-Scryfall attestation search was used. Evidence stays under target. Kata refresh succeeded after the initial commit, incorporating equality-property grammar `wlstpkpuulvt` and linked-readback changes `vmupwtsqwxlm` / `xsqzrryrlups`. The seven refresh gains are Aether Vial (43), Enigmatic Incarnation (49), In Search of Greatness (999), Instrument of the Bards (222), Necroplasm (25), Powder Keg (32), and Wave of Terror (24); all have mana-value-equal-to phrases and are attributable to `wlstpkpuulvt`, not vocabulary declarations. The built-in line diff reports thirteen newly readable lines. No trunk decreases need attribution. Shared claim transitions during verification changed only other ticket locations, not dependency content.

   Review on `qmqmopsronzl`: begin’s gain column counts begin/begins surfaces only (20, excluding beginning, from `review/begin-surfaces.json` intersected with the original built-in gain list). Life total is named as a compound. Secretly is Imperative-only. Was warped this turn is a predicative state Adjective, the only path V3 permits today, not a verbal event passive. V8 permits unblocked in core as a V3 deviation until the predicate family has a declaration path.

   The five requested adjective redeclarations retain all ten original witness counts. However, the AttributiveAdjective class maps to ordinary Category::Adjective (`legacy/core.rs::inventory_category`), and generic Adjective → AdjectivalComplement still admits Your hand is opening and This damage is excess (1 each). Its existing HomographLicense / ModifierLicense defaults do not constrain predicative use. V9 resolves the STOP by deferring the predicative restriction and both zero-Reading adjective negatives while keeping the correct AttributiveAdjective declarations.

   **Follow-up: missing constructions or other vocabulary (distinct V1 unread-face counts).**

   | Candidate | Faces | Missing piece / evidence |
   |---|---:|---|
   | move to zone | 2 | Zone noun + to-goal Complement (Skullbriar / Me, the Immortal); the full attested move-to-zone clauses stay unread. |
   | secretly, finite uses | 10 | Imperative-only declaration; finite secretly-chooses/votes stays unread (`review/secretly-finite.json`), requiring a separately attested central-adverb licence. |
   | half | 83 | Quantitative half + possessive/partitive complement; both known-noun controls fail (`followup-probes-before.json`), as expected by V6. |
   | round / rounded | 87 | Rounded up/down requires particle/result direction and supplement licensing; turned-up controls fail (`remaining-probes-before.json`), V6. |
   | tied | 51 | Tied-for selected comparison; even vote-is-legendary controls fail because the vote nominal is not mapped (`tied-before.json`); V6 construction remains. |
   | exactly | 45 | Focusing cardinal modifier; another is an additional cardinal determiner and does not witness this analysis. No intended-control Reading (`followup-probes-before.json`), V6. |
   | difference | 32 | Scalar relational difference-between / paired property values; number-between control fails (`remaining-probes-before.json`), V6. |
   | use / using / used | 41 | Purpose attachment in cards-used-to-craft and gerund adjunct use; chosen/choosing controls fail (`pass1-before.json`). |
   | true | 25 | Fused-head the-same subject and for-complement; legendary controls fail (`pass1-before.json`). |
   | promised | 24 | Gift/possessive context still lacks a readable control, chosen controls 0/0 (`pass1-before.json`). |
   | else | 16 | Anywhere-else postmodifier of a fused locative; instead controls fail (`followup-probes-before.json`). |
   | common | 12 | Most-common degree/superlative phrase; legendary controls 0/0 (`followup-probes-before.json`). |
   | replace / replacing | 12 | Another as a fused-head object in replacing-X-with-another; sharing controls fail with all other batch vocabulary present (`replace-final-before.json`). |
   | full | 12 | Party remains unavailable; legendary substitution alone leaves party unknown, 0/0 (`followup-probes-before.json`). |
   | ignore | 12 | Choose-one control uses Cardinal complementation, not the proposed ObjectNP. Provisional ObjectNP Ignore read 0/0 and was removed before final verification; roll nominal/fused cardinal object needed (provisional probe output; `repairs2-before.json`). |
   | how | 11 | Embedded degree/manner interrogative complements: how-many cards / how-those-creatures-block. No declared word has the required interrogative class/frame. |
   | come / comes | 15 | Come-up heads/tails particle-result and comes-first comparative/supplement use; no existing copied frame for the attested uses. |
   | legend / rule / apply | 10 each | Quoted legend-rule nominal plus stative apply-to complementation; mana-value/resolve full-clause control fails (`quoted-rule-before.json`). |
   | differently | 10 | Adverb inside named participial premodifier; already controls 0/0 (`followup-probes-before.json`). |
   | longer | 9 | No-longer negative/degree central modifier; already controls 0/0 (`followup-probes-before.json`). |
   | whichever | 8 | Supplementary fused relative whichever-comes-first and its priority predicate; no declared substitute of that category. |
   | contain / contains | 12 | Focusing only over the cards ObjectNP; has-only-cards controls fail even after other declarations (`contain-before.json`). |
   | starting; kind; odd/even; hasn’t | 84; 45; 15/17; 17 | Starting-with-you supplement, each-kind-of-counter bare singular, choose-odd/even complements and one-that-hasn’t-been-chosen fused-head relatives remain unimplemented; witness NPs/adjective uses and haven’t-perfect clauses do read. These residual uses do not claim new constructions. |

   ### grammar hygiene

   V9: `AttributiveAdjective` members read predicatively through the generic AdjectivalComplement (`declarations.rs:3781`), e.g. “Your hand is opening.” and presumably “This cost is additional.”; the class needs a declared attributive-only feature read by AdjectivalComplement; affects every existing member, zero attested predicative uses to confirm by grep. A supported-corpus grep over all 20 member surfaces after copular verbs (including contractions, become/remain and optional not) found zero matches on `qmqmopsronzl`; the exact pattern and JSON are `review/attributive-predicative-pattern.txt` and `review/attributive-predicative-hits.json`. This attestation search does not repair the grammar licence. The two proposed adjective negatives are deferred, not ignored tests.

   ### passes V2, blocked on declaration path

   `read_builtin_v2` reads only the eight `BUILTIN_FAMILIES` in `deckmaste_construction_core/src/macro_def.rs`; lexical `legacy/plugins.rs::load` consumes only that normalized set. Predicate/event helpers are excluded. `FixedKeyword` supports the added adjective supplement but no secondary verb export; `FixedTerm` supplies one surface, not additional independent noun heads. The following terms are deferred under V7, never declared in core. Evidence below gives both substitution counts where available; joint-term substitutions are explicitly identified.

   | Candidate | Faces | Owning definition / unsupported path | Substitution evidence |
   |---|---:|---|---|
   | historic | 56 | `predicates/historic.ron`, excluded helper family | a-historic-spell → a-legendary-spell, 1/1 (`followup-probes-before.json`); original full-card controls also retained. |
   | modified | 47 | No Modified declaration exists in the supported families | legendary adjective in two attested creature NPs, 2/2 (`followup-probes-before.json`). |
   | party | 40 | No Party lexical definition exists in supported families | your-party → your-hand, 1/1 (`followup-probes-before.json`). |
   | mutate | 33 | `keyword_abilities/mutate.ron`, FixedKeyword has no secondary intransitive verb | this-creature-attacks, 1/1 (`path-probes-before.json`). |
   | crew | 36 | `keyword_abilities/crew.ron`, FixedKeyword has no secondary verb | blocks-Vehicles predicate, 1/1 (`followup-probes-before.json`); crewed shares this owner/path block. |
   | exploit | 24 | `keyword_abilities/exploit.ron`, FixedKeyword has no secondary verb | blocks-a-creature trigger clauses, 1/1 (`followup-probes-before.json`). |
   | paired | 27 | `keyword_abilities/soulbond.ron`, cannot export Pair verb selecting with-NP; unframed adjective supplement cannot encode it | shared-with-creature clauses, 2/2 (`followup-probes-before.json`). |
   | crime; commit | 27 each | `events/commitsCrime.ron`, event helper family excluded; no separate crime noun export | a-spell NPs 1/1; joint you-cast-a-spell controls 2/2 (`followup-probes-before.json`, `remaining-probes-before.json`). |
   | multikicker | 19 | No Multikicker definition; `keyword_abilities/kicker.ron` owns only kicker | Kicker cost keyword phrases 1/1 (`remaining-probes-before.json`). |
   | day; night | 10; 16 | `designations/dayNight.ron`, FixedTerm owns day-or-night only | noun controls it’s-life and damage-becomes-life, 1/1 each (`path-probes-before.json`), joint day/night substitution identified. |
   | expend | 12 | No Expend keyword-action definition | draw-4 trigger clauses 3/3 (`followup-probes-before.json`). |
   | descend / descended | 12 | `ability_words/descend4.ron`, `descend8.ron` declare ability labels, no Descend action/verb owner | you-attacked-this-turn clauses 2/2 (`followup-probes-before.json`). |
   | saddle | 16 | `keyword_abilities/saddle.ron`, FixedKeyword has no secondary verb; `designations/saddled.ron` is a different designation | blocks-Mounts predicates 1/1 (`followup-probes-before.json`). |
   | escape | 16 | `keyword_abilities/escape.ron`, FixedKeyword has no secondary verb | enters-with-counter sentences 5/5 (`followup-probes-before.json`). |
   | haunt | 9 | `keyword_abilities/haunt.ron`, FixedKeyword has no secondary transitive verb | creature-this-card/it-targets NPs 1/1 (`path-probes-before.json`). |
   | outlaw | 20 | `predicates/outlaw.ron`, excluded helper family | artifact-spells/sources NPs 1/1 (`path-probes-before.json`). |
   | Wicked; Monster | 10; 8 | `subtypes/enchantment/role.ron` owns Role only; individual Role token definitions/exports absent | Whole Wicked/Monster-Role-token term → Treasure-token NPs 1/1 each (`remaining-probes-before.json`). |
   | bands | 8 | `keyword_abilities/banding.ron` has no bands-with-other definition/export | Protection-from-legendary-creatures keyword phrases 2/2 (`remaining-probes-before.json`), whole construction substitution. |
   | Friends forever | 7 keyword faces (friend lemma group 14; ordinary noun implemented) | No Friends-forever keyword definition/export | Separate from the ordinary count noun; no single-word keyword control of the same payload shape. Deferred ownership, not an ordinary-noun declaration. |
   | empower | 19 | No Empower keyword-action definition; Name + measure payload also lacks a supported frame | V2 not established: no declared verb with that Name+measure frame; deferred rather than manufactured in core. |
   | evidence / collected | 8 | `keyword_actions/collectEvidence.ron` owns collect-evidence with MeasureComplement only; no evidence noun + bare Collect export | Joint damage-was-dealt controls 1/1 (`remaining-probes-before.json`); a single-word control cannot repair both heads. |

   Produce/Divided were already declared on the claim parent and skipped. Draft/drafted/drafting (27 faces), booster (9), pack (8) and planeswalk (8) are permanent scope exclusions, not future model reservations. All remaining lemma groups fall below eight distinct unread faces (including guess 7, count 7, quality 7, select 6 and give 5); they belong to the next cutoff, not unprocessed eligible work.

5. **Tests** — on `nouqyyouuops`: `cargo test -p deckmaste_english_v3` passed (713); `cargo test -p deckmaste_lexical_source` passed (71); `cargo test -p deckmaste_construction_core` passed (468); `cargo test -p deckmaste_semantics_v2` passed (101). `cargo clippy -p deckmaste_lexical_source -p deckmaste_english_v3 -p deckmaste_construction_core -p deckmaste_semantics_v2 --tests -- -D warnings` passed. Full suites were rerun after the initial new-fixture failures and the deliberately superseded lexical-gap assertion were corrected. Eight tests added (seven lexical-source class tests and one shared integration table with 84 positive witness cases plus three beginning-phase V5 guards and one negative case (This creature moves)); restored 0, re-spelled 1 existing focusing-only lexical-gap case (original text now also asserted positively; incomplete by-agent complement remains negative), ignored 0, removed 0. New-fixture lifetime, Keyword provenance, keyword adjective feature, and central-adverb constituent expectations were corrected while keeping exact value assertions; moving the witness table to a constant resolved clippy’s function-length finding. Citation checks passed (no noncompliant or stale citations; diff audit found no new citation sites). After the grammar-bearing refresh, on `oxtqzporvvmw`, the same four exact full-suite commands passed again: English v3 724, lexical source 71, construction core 468, semantics v2 104. The same four-package clippy command passed again. Refreshed logs are `refresh-test-deckmaste_english_v3.log`, `refresh-test-deckmaste_lexical_source.log`, `refresh-test-deckmaste_construction_core.log`, `refresh-test-deckmaste_semantics_v2.log` and `refresh-clippy.log` under the evidence directory. The declaration passes were complete before the three-hour time box; only required refresh verification continued afterward.

   Review gates on `qmqmopsronzl`: `cargo xtask facts check` passed; both generated fact files are up to date (`review/facts-cargo.log`). Two fragment cases were removed by the explicit review direction; 55 whole-root checks were replaced with constituent roles, with all positive expected counts retained. Test functions removed / ignored remain 0. An initial fixture serialization emitted invalid Rust Unicode escapes; these were corrected to the original em dash before rerunning tests. Full lexical-source, construction-core and semantics-v2 suites passed after the review edits. The English suite’s initial compile failed on fixture serialization and that syntax is corrected; V9 defers its two zero-Reading adjective assertions; `cargo test -p deckmaste_english_v3` passed on the final review head (724 tests, `review/test-english-final.log`), including all constituent-role assertions and the remaining Move negative. The exact four-package clippy command passed after the review edits (`review/clippy.log`). The review bounded census uses `review/manifest.json`: 198 matching move/adjective surface faces plus 300 seeded covered controls, overlap 12, 486 distinct faces. Built-in comparison with the accepted post-refresh state has lost 0, gained 0, decreases 0, increases 0 and 61 digest-only owner changes; comparison with claim-parent data has 129 gains and zero loss, decreases, increases or digest-only changes. The other 3,216 identities are outside this recheck, not losses. All 61 owner changes are on this batch’s newly covered faces.

6. **Timing** — on `nouqyyouuops`: final bounded census 62.44 seconds, peak RSS 4,098,356 KiB (`after.log`, `/usr/bin/time -v`); prebuilt binary, six workers, zero samples per face. Post-refresh bounded recheck on `oxtqzporvvmw`: 100.20 seconds, peak RSS 4,352,664 KiB, six workers, zero samples (`after-refresh.log`). Review bounded recheck on `qmqmopsronzl`: 12.11 seconds, six workers, peak RSS 2,504,112 KiB (`review/after.log`). No full-corpus census.
