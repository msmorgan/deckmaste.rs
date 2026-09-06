# Grammar migration obligations

This is the routing record for the 2026-09-05 migration design, not a corpus
measurement or a second task graph. The implementation tickets own these
outcomes; their `needs:` fields schedule the work. Entries retain source
examples and identity keys from the former tickets/fog without treating old
parser acceptance, census counts or discarded implementation recipes as
linguistic authority. Re-fetch exact supported Oracle text at implementation.

## Grammatical families

| Family | Production owner | Pinned outcome |
|---|---|---|
| Grammatical relations, Case and Person/Number | `english-v2-lexeme-owned-verb-frames` | One NP category across roles; derived finite agreement, including mixed coordination. |
| Lexeme-owned frames and recipient/retained-Object passives (A6) | `english-v2-lexeme-owned-verb-frames` | Ordered declared frames; no passive Object inferred from a following temporal NP. |
| Copular complements (A7) | `english-v2-lexeme-owned-verb-frames` | One copular frame with AdjP/NP/PP; remove game-partitioned complements. |
| Countability, adjective tier, polarity (A8a), type modifiers (A8b) | `english-v2-adjective-inventory` | Feature-constrained nominal stages, lexical adjective ownership and productive modifier morphology; keep the separate noun inventory. |
| Subordination and gerund clauses (A11) | `english-v2-subordinate-clause` | Declared form selection and common clause bodies; preposition consumers use those bodies. |
| Preposition attachment, license sets, of-filter and missing members | `english-v2-remaining-prepositions` | Independent declared license dimensions and general Complement grammar; remove semantic filters. |
| Arithmetic/fractions (A10), comparison and degree | `english-v2-grammar-measure-phrases` | Typed measure expressions and grammatical distributions, without arithmetic evaluation. |
| Relative and content clauses, shared-gap boundary constraints | `english-v2-relative-clause` | That/who/which/whose/zero, supplementary/pied-piped/free-relative and that/wh Complement forms; explicit discharge. |
| Ellipsis (A9), omitted destinations, gapping, shared heads/prepositions | `english-v2-grammar-context-ellipsis` | Contextual omission versus explicit shared gaps; retain the complete permitted scope alternatives. |
| Former/latter and intra-sentence recoverability | `english-v2-grammar-context-ellipsis` | Grammatical anaphoric forms/context only; no game-reference resolution. |
| Ordinary/correlative coordination and retained ambiguity | `english-v2-scope-device-cross-host-gates` | Unique construction ownership per bracketing and exact scope classes, with no arbitrary boundary preference. |
| Costs and document bodies | `english-v2-grammar-document` | Flat textual collections of clauses/notation; ordinary comparative costs belong to measures. |
| Keyword subjects, bound/parameterized/named surfaces and mixed grants | `english-v2-grammar-document` | General nominal hosts, declaration-backed surfaces; fix the vacuous irregular-verb test. |
| Reminder prose, literal quotation and printed layouts | `english-v2-grammar-document` | Typed text/notation and exact boundaries; no game layout validator. |
| Type Line order | `english-v2-type-line-construction` | One open Type authority supplies order to parser and renderer; all declared combinations linearize. |
| Door/half references and remaining lexical/source populations | `english-v2-grammar-lexical-source` | Linguistic referential forms and supplied recipe validation; no dependency on the game-model workbench. |
| Target Verb lexical rivalry | `english-v2-target-verb-subject-selection` | Recover the intended relative use; preserve marker analysis; report a surviving non-scope ambiguity rather than game-filter it. |

## Named identity obligations

The identity keys below are copied from the existing obligation records, not
recomputed measurements. “Re-coverage” means an earlier wrong analysis was
retired; restoring the wrong analysis does not discharge the obligation.

| Identity | Witness | Owner and required structure |
|---|---|---|
| `3a05d72cc35c86f1b6bb8cfb74017af4c6c5b440e9b18475485ad034db4440e6` | Cavalier of Thorns | `english-v2-grammar-context-ellipsis`: re-coverage; land card to battlefield, remaining cards to graveyard. |
| `83b1cc666255a03888d2654849cbdcd552c61d9336ce41cbe163a248c4f887ec` | Animal Magnetism | `english-v2-grammar-context-ellipsis`: re-coverage; chosen card to battlefield, rest to graveyard. |
| `cc39ba9efc13bba3022d314e135707a35dbe17b515db2cbd7ca858a97cff00b2` | Genesis Ultimatum | `english-v2-grammar-context-ellipsis`: re-coverage; chosen permanents to battlefield, rest to hand. |
| `5127c53a4d019cd2ac0cc868f916703b84d38ace52d3d3ee49c6f7d068be242b` | Ashen-Skin Zubera | `english-v2-relative-clause`: `that died this turn`; preserve the temporal adjunct without the old this-way invariant blocking it. |
| `a70c14b1f1abb00e54a5aa550264c5040c79a2e241a2cf5a0a2c725140d94d39` | Boldwyr Heavyweights | `english-v2-relative-clause`: `who searched` with finite preterite. |
| `57bbe75f309b505dd4882064db97811056a148735b94cd43558202587b752c06` | Aggravate | `english-v2-lexeme-owned-verb-frames`: `Each creature dealt damage this way ...` remains reduced passive, not finite preterite. |
| `4c82b6f6b1da0021ddb05732ec64c82a012ca49c9807f0e2d6f835c1843f4741` | Ballista Watcher // Ballista Wielder (Ballista Wielder face) | `english-v2-lexeme-owned-verb-frames`: `A creature dealt damage this way ...` remains reduced passive. |
| `a52b2536e676ecd15985cf16dee330001ed00c2a8c30a6451582e310cc41912a` | Assassin's Blade | `english-v2-lexeme-owned-verb-frames`: `this step` in `if you've been attacked this step` is a temporal Adjunct, never a passive Object. |
| `92be245709b1c7dd6d33fc2b3e4bd385d9120613e84a8339879ebb78d6d933af` | Champion's Victory | `english-v2-lexeme-owned-verb-frames`: `this step` in `if you've been attacked this step` is a temporal Adjunct, never a passive Object. |
| `9c974195f93b89e654ea0152acaabd1c5df3e42e8a87b8f835090b064a24c409` | Defiant Stand | `english-v2-lexeme-owned-verb-frames`: `this step` in `if you've been attacked this step` is a temporal Adjunct, never a passive Object. |
| `cd8a16b89dd6347cc571f3413c667fe443f44f07d62f5509437a52257df47485` | Eightfold Maze | `english-v2-lexeme-owned-verb-frames`: `this step` in `if you've been attacked this step` is a temporal Adjunct, never a passive Object. |
| `7481f5103edbf11abf8bd08ea38ccfc8930d9ca8d27ada55eda758525afc4487` | Just Fate | `english-v2-lexeme-owned-verb-frames`: `this step` in `if you've been attacked this step` is a temporal Adjunct, never a passive Object. |
| `43f14e90b11f20a925d3d3f6cf43167d284bbb0d58e9c58abebe239c6a785680` | Kongming's Contraptions | `english-v2-lexeme-owned-verb-frames`: `this step` in `if you've been attacked this step` is a temporal Adjunct, never a passive Object. |
| `a884c45e1a179a5399b24a3b80eea671122c261def8e0604016a40255a764f1b` | Rally the Troops | `english-v2-lexeme-owned-verb-frames`: `this step` in `if you've been attacked this step` is a temporal Adjunct, never a passive Object. |
| `eedeb6537edb2f7be63bd5a941564e72c587fccc23a27ed75f19b6793f6cbda2` | Remove | `english-v2-lexeme-owned-verb-frames`: `this step` in `if you've been attacked this step` is a temporal Adjunct, never a passive Object. |
| `e310d19a800a04135b8ebd394e11b323d52f39f854e90b12e358ec818d6dd8b7` | Scorching Winds | `english-v2-lexeme-owned-verb-frames`: `this step` in `if you've been attacked this step` is a temporal Adjunct, never a passive Object. |
| `16537b37e03639380fe8b773503a0076ff4ef349be46d69b24c4be0d695b5f3e` | Treetop Defense | `english-v2-lexeme-owned-verb-frames`: `this step` in `if you've been attacked this step` is a temporal Adjunct, never a passive Object. |
| `3f0d55ab5f9a6fc1be0ebeb6b647f38635e324b4a80bc9ee30bf927e45622d32` | Warrior's Stand | `english-v2-lexeme-owned-verb-frames`: `this step` in `if you've been attacked this step` is a temporal Adjunct, never a passive Object. |
| `72b69a69a8a4add9165cd6a8f5b062801ad06247f3c48e126189904cdbac155d` | Absorbing Man and Titania | `english-v2-relative-clause`: re-coverage; `all damage` has an auxiliary object-gap relative: Subject `creature sources you control`, auxiliary `would`, verb `deal`, Object Gap. |
| `5fbcf36b2e6006fefc9c94d13b8c7d165c436d22f6207f5e83dc03956232681c` | Infectious Curse | `english-v2-target-verb-subject-selection`: re-coverage; `Spells you cast` plus `that target enchanted player`, followed by ordinary `cost {1} less to cast`. |

The destination sentences are respectively:

- `Put a land card from among them onto the battlefield and the rest into your graveyard.`
- `Put that card onto the battlefield and the rest into your graveyard.`
- `Put any number of permanent cards from among them onto the battlefield and the rest into your hand.`

The relative/Target re-coverage sentences are `Double all damage that creature
sources you control would deal.` and `Spells you cast that target enchanted
player cost {1} less to cast.` Their previous coverage used an invalid temporal
endpoint reading, recorded by `english-v2-adjunct-licence-removal`.

`Cast this spell next turn.` is the synthetic Object/Adjunct discriminator
owned with the eleven passive-temporal cases. It does not justify making
`next` a Determinative; its lexical classification belongs to the nominal slice.

## Scope and overlap witnesses

Owner: `english-v2-scope-device-cross-host-gates`, with PP admissibility supplied
by `english-v2-remaining-prepositions`. These retain alternatives, not semantic
claims about which attachment a card means.

- The thirteen `with` cases from the [with-preposition landing](tickets/done/english-v2-with-preposition.md):
  Arc Blade, Chronomantic Escape, Cyclical Evolution, Doom's Time Platform,
  Festering March, Inspiring Refrain, Reality Strobe, Suspended Sentence,
  Taigam, Master Opportunist; Curse of Chaos, Curse of Shallow Graves,
  Oath of Kaya, Rigo, Streetwise Mentor. Preserve declared frame-selected `with`
  independently of free attachment.
- Vicious Rivalry (`083bb4922e48a966bdea837696fca2600cb3881eb7819bc57b2092715e170315`):
  `Destroy all artifacts and creatures with mana value X or less`; retain the
  shared qualification reading, with Pernicious Deed as the shared-Determiner
  comparison. Recheck the eighteen affected coordination identities named in
  the with-preposition landing, including the Hurricane/token-list families
  and Eaten by Spiders; no forced scope based on game meaning.
- Class C Adventure Awaits: shared auxiliary scope. Class D Abzan Falconer:
  `each creature you control with a +1/+1 counter on it`, where moving one
  mobile affects the yield of another. Identify the actual old rejecting gate
  before replacing it; test both too-strict and too-loose directions.
- R7's fourteen wrapper-overlap identities (witness Doorman) and the collapse's
  full existing positive/negative fixtures remain requirements. The
  [collapse landing](tickets/done/english-v2-scope-device-collapse.md) preserves
  their provenance and tests. Remove duplicate constructions for a bracketing,
  not grammatical bare-plural coordination; report which old assertions passed
  without establishing their claimed pack.
- Arwen, Mortal Queen (inherited key prefix `f9afecf8…`):
  `Put a +1/+1 counter and a lifelink counter on that creature and a +1/+1 counter and a lifelink counter on Arwen.`
  Both outer-boundary bracketings must survive with their actual anchor tuples.
  Do not assert that the earliest tuple is the unique interpretation. Flat
  n-ary versus nested binary remains the negative fence. The
  [B7a landing](tickets/done/english-v2-frame-complement-coordination.md)
  records 85 gains; test its three gains containing a second connective and
  at least twenty other gains by their independently justified structures.
- `Search your library for a card.` retains its selected role; no site may enter
  an opaque marked role. Later same-marker postmodifiers and non-final conjunct
  PPs remain available; fixed markers stay transparent.
- Four nested mobiles, their quotation embedding, lexical-homograph separation,
  shared heads/determiners and checked frame-pair packing correspond to the
  named Lean interaction modules. Assert exact complete alternatives and
  metadata, never just `raw > packed`.

The missing compiled-consumer `FrameComplementPair` fixture moves to
`english-v2-grammar-frame-compiler`. The old code-generation and predicate
integration tests remain and are re-spelled if their carrier changes.

## Historical frontier, with production owners

Measured 2026-09-05 on `ptoxwkmmrqno`: 19,469 / 32,641 covered, 13,172 parse
failures, 541 unbucketed. These are historical first-failure counts, not current
scope or coverage promises. The original classification is retained to explain
provenance; it does not override the new structural owner. Every named row is
checked by its migration owner before closure. Previously batched/landed rows
are verified, not implemented again.

| Family | Historical units | Historical classification | Production owner |
|---|---:|---|---|
| preterite finite clause (`died`, `attacked`, `entered`, `left`, …) | 929 shaped | STRUCTURAL | `english-v2-lexeme-owned-verb-frames` |
| `as`-clause (`only as a sorcery`, `enter as a copy of`) | 496 | STRUCTURAL | `english-v2-remaining-prepositions` |
| elliptical / re-subjected predicate sequencing (`If you don't, …`, `…, then you may …`) | 350 | STRUCTURAL | `english-v2-grammar-context-ellipsis` |
| `was kicked` | 194 | LEXICAL | `english-v2-grammar-lexical-source` |
| `in addition to its other types` | 170 | STRUCTURAL | `english-v2-remaining-prepositions` |
| `dealt by` / `controlled by` | 154 | STRUCTURAL | `english-v2-remaining-prepositions` |
| landwalk fused surfaces (`Swampwalk`, …) | 127 | LEXICAL | `english-v2-grammar-document` |
| energy counters (`you get {E}{E}`) | 104 | STRUCTURAL | `english-v2-grammar-measure-phrases` |
| copula contraction + `still` | 99 | STRUCTURAL | `english-v2-lexeme-owned-verb-frames` |
| `who` relative clause | 94 | STRUCTURAL | `english-v2-relative-clause` |
| typecycling fused surfaces (`landcycling`, …) | 91 | LEXICAL | `english-v2-grammar-document` |
| `share` / `shares` | 91 | LEXICAL | `english-v2-grammar-lexical-source` |
| bare `X` as card-count quantifier | 141 | STRUCTURAL | `english-v2-grammar-measure-phrases` |
| `until you <verb>` | 84 | STRUCTURAL | `english-v2-subordinate-clause` |
| sentence-initial `Otherwise,` | 84 | STRUCTURAL | `english-v2-subordinate-clause` |
| `amount of X equal to Y` | 83 | STRUCTURAL | `english-v2-grammar-measure-phrases` |
| `win` | 77 | LEXICAL | `english-v2-grammar-lexical-source` |
| `Do this only once each turn.` | 18 | STRUCTURAL | `english-v2-grammar-context-ellipsis` |
| `the greatest X` (superlative measure) | 76 | unresolved | `english-v2-grammar-measure-phrases` |
| `This ability triggers only once` | 74 | STRUCTURAL | `english-v2-lexeme-owned-verb-frames` |
| `Affinity for X` | 73 | STRUCTURAL | `english-v2-grammar-document` |
| `creature without <ability>` (NP postmodifier) | 68 | STRUCTURAL | `english-v2-remaining-prepositions` |
| `attacks alone` | 64 | STRUCTURAL | `english-v2-subordinate-clause` |
| `equal to` as an NP postmodifier | 58 | STRUCTURAL | `english-v2-grammar-measure-phrases` |
| `Multikicker` / `Prototype` / `Change` | 53 | mixed | `english-v2-grammar-document` |
| `devotion` | 52 | STRUCTURAL | `english-v2-grammar-lexical-source` |
| `Partner with <name>` | 52 | STRUCTURAL | `english-v2-grammar-document` |
| storage / finality counter kinds | 45 | LEXICAL, consumer unverified | `english-v2-grammar-lexical-source` |
| `instead of V-ing` | 44 | STRUCTURAL | `english-v2-subordinate-clause` |
| The Ring tempts you | 42 | STRUCTURAL | `english-v2-grammar-lexical-source` |
| `transforms into <name>` | 37 | unverified | `english-v2-grammar-lexical-source` |
| `historic` | 33 | STRUCTURAL | `english-v2-adjective-inventory` |
| `unlock` | 30 | LEXICAL | `english-v2-grammar-lexical-source` |
| level-range line (`1-9 \|`) | 26 | STRUCTURAL | `english-v2-grammar-document` |
| `party` | 26 | STRUCTURAL | `english-v2-grammar-lexical-source` |
| `exert` | 26 | LEXICAL | `english-v2-grammar-lexical-source` |
| `Splice onto <subtype>` | 23 | STRUCTURAL | `english-v2-grammar-document` |
| `exploits` | 23 | LEXICAL | `english-v2-grammar-lexical-source` |
| `target beyond the first` | 22 | STRUCTURAL | `english-v2-grammar-measure-phrases` |
| `at random` (postmodifier position) | 21 | unresolved | `english-v2-remaining-prepositions` |
| `twice that much/many` | 21 | unresolved | `english-v2-grammar-measure-phrases` |

Additional residues have explicit owners:

- `... and only once each turn` (Nature's Chosen and Sawback Manticore):
  `english-v2-scope-device-cross-host-gates` owns coordinated focus; `Do this`
  belongs to recoverability, not the already-landed frequency adjunct.
- Preterite morphology/host discrimination: `english-v2-lexeme-owned-verb-frames`
  owns withheld Deal preterite and unauthored core/plugin paradigms, including
  `If you discarded` (Psionic Snoop), `player who shuffled` (Collision of Realms),
  `opponent who voted` (Erestor of the Council), `a creature that fought`
  (Boxing Ring), `a permanent you controlled explored` (Herald's Reveille),
  and `who investigated` (Wernog, Rider's Chaplain). Relative hosts are exercised
  again by extraction. An underspecified `{Plain, Preterite}` lexical leaf
  must be refined where host constraints decide it. Genuine tense ambiguity
  remains explicit; it is not resolved by choosing a scanner form.
- `When you unlock this door, draw a card.`: lexical/source owns the `door`
  noun and linguistic half reference, using document text-box structure.
  The game-model Room work is not an English dependency.
- Mixed granted keyword/quotation lines belong to documents. The existing
  `english-v2-affinity-quality-surface` result is conformance evidence there;
  legacy `english-keyword-atom-roles`, `english-ability-derived-verb-batch`,
  `macro-keyword-templates`, `macro-ring-emblem` and `engine-party` are not
  required to implement the English shapes in the table.
- The 541 unbucketed units and the generic `.` frontier (660 units over 102
  preceding tokens) go to `english-v2-grammar-migration-close` for representative
  triage. A period frontier is not a grammatical family. A demonstrated missing
  general capability gets a migration owner before closure; genuinely local
  remaining work goes to `english-v2-stage-5-grammar-buildout-14-10`.
- Mid-word first-failure offsets are boundary-suppression artifacts, not proof
  of scanner splitting. Inspect the whole word and its consuming form. A
  post-migration lexical batch adds only inventory members whose consumers
  already accept standalone lexical forms, with no new construction, sum,
  feature or parser facility; verify against an existing-member control.

## Retired ticket routing

The following unclaimed tickets were merged after moving their obligations and
live graph edges. Historical mentions resolve through this table; done records
and other workspaces' WIP tickets were not rewritten.

| Retired ticket | Live replacement |
|---|---|
| `english-v2-grammatical-relations` | `english-v2-lexeme-owned-verb-frames` |
| `english-v2-person-number-agreement` | `english-v2-lexeme-owned-verb-frames` |
| `english-v2-attachment-class-declared` | `english-v2-remaining-prepositions` |
| `english-v2-locative-licence-set` | `english-v2-remaining-prepositions` |
| `english-v2-of-complement-filter-removal` | `english-v2-remaining-prepositions` |
| `english-v2-locative-coordination-arms` | `english-v2-scope-device-cross-host-gates` |
| `english-v2-frame-complement-pair-nesting` | `english-v2-scope-device-cross-host-gates` |
| `english-v2-cost-family-lowering` | `english-v2-grammar-document` |
| `english-v2-keyword-subject-modifiers` | `english-v2-grammar-document` |
| `english-v2-type-line-declaration` | `english-v2-type-line-construction` |
