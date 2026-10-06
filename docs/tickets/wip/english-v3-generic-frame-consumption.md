---
needs: [english-v3-reading-test-helper]
---
# Consume ordered lexical Verb Frames compositionally

Replace whole-frame matching against a handwritten grammar alias catalogue with
shared composition over declared ordered Frame Slots. Retain lexical syntactic
selection, slot relations, Categories, marker ownership and chosen frame identity.
This is not permission for every verb to take arbitrary Complements, nor a request
to add an alias and named predicate variant for every newly encountered shape.

Pinned witness: Orcish Bowmasters' “Then amass Orcs 1.” The exact constituent
“amass Orcs 1” has zero Secondary Verb Phrase Readings, while “Orcs” as a Noun
Phrase and “1” as Amount each have one. The lexical NP-plus-Amount frame already
exists; its whole shape has no active grammar alias. Implement this through the
keyword action's existing macro-owned declaration, not duplicate vocabulary.

The exploration tree qymzymor (parent sqnmlrnz; grammar baseline vwtnryos,
covered 13,226) loads 120 verbs, 192 frame assignments and 51 distinct shapes.
Only 24 shapes / 160 assignments match active aliases; 27 shapes / 32 assignments
lack matches. These counts are an interface inventory, not promised coverage gains.
Audit every unmatched shape: legacy FrameComplement and ReplacementMarker payloads
need typed reconciliation before consumption. “Remove a +1/+1 counter from this
creature” from Triskelion also fails, but its legacy placeholder makes it a less
clean witness. Reins of the Vinesteed's “shares a creature type with that creature”
already parses through transitive-plus-Adjunct routes; restore the selected With
Complement analysis rather than counting mere recognition as success.

Design the shared consumer at the declaration/compiler and packed-admission layers:
consume typed ordered slots without constructing eager AST products, and generate
checked construction, realization and traversal from the same declarations.
Inspect exact-equality projection in deckmaste_construction_v3_core/src/emit.rs.
Preserve correlations needed by selected Complements and frame coordination;
coordinate with english-v3-frame-coordination without absorbing its sharing work.
CGEL Ch. 4 sections 1.1–1.2, pp. 216–228 distinguishes syntactic licensing from
semantic plausibility. Ordinary noun/verb meaning is not an admission guard.

Acceptance: independently construct the attested Amass Reading and prove both
roundtrip laws; test typed slot order and marker mismatches meaningfully; restore
selected roles for contrasting authentic constituents. Reconcile the unmatched
inventory and report remaining owners, identity-level corpus changes, all retained
Readings, internal failures and construction/declaration economy. Old scratch
reports in /tmp/english-v3-frame-probes are optional; regenerate evidence from the
implementation tree. Standard constraints apply.

## Scope bound

Three deliverables: the shared ordered-slot consumer, the Amass witness with the
contrasting selected-With constituent, and a report of every frame shape still
unmatched afterwards with the ticket that owns it. "Reconcile the unmatched
inventory" above means that report. Typed reconciliation of the legacy
`FrameComplement` and `ReplacementMarker` payloads is routed to an owner, not
done here; a shape that depends on them stays an explicit unsupported
diagnostic.

## Sequencing

This is the one queued ticket that changes how existing Readings are
represented, so it goes first among the grammar tickets and after the shared
test helper: write the pinned witnesses with the constituent assertion before
implementing, and use it for any existing test that has to be re-spelled.
The landing compares corpus Reading identities before and after on its own tree.

## Prior attempt

The bookmark `archive-english-v3-compact-grammar` contains an unfinished attempt
at this ticket, made in one change together with six others and never verified
against the corpus. It replaced thirteen ordinary predicate shape schemas with a single
`SelectedPredicate` schema, re-spelled the existing structural tests against it,
and reported 12 unsupported shapes / 13 assignments remaining; it ran no
full-corpus identity comparison and no gate. Read it for ideas if useful. Do not rebase onto
it, and treat every claim in its ticket notes as unverified.

## Landing record

Implementation and final corpus measurements: change `ykrqsqyr`, recognized
covered count 13,277. Baseline: parent `rmxmzuss`, covered 13,226.
Both use the same supported 32,828 face identities, input digest
`49dc966bda6ef588fc68e8d6972de25df1b660ed2ea654904584698263007ebb` and reminder-stripping policy.
No v3 coverage lock or `coverage` subcommand exists on this tree: the covered
figures are the current `english-v3` census authority, not a legacy lock claim.

### Proof and retained Readings

The shared `selected_frame(head, Predicate)` consumer compiles ordered typed
Arguments, declared Markers, Marked Arguments and Optional slots. Packed
admission checks canonical selected Frame and declared marker class before
complements; full child AST products are materialized only on request. One
declaration drives recognition, checked construction, realization and traversal.
Lexical owner, local frame choice and ordered complement values remain in each
Reading. Auxiliary, passive, depictive and shared-head consumers retain their
own grammatical constraints.

No silent loss: zero previously covered face identities disappear, and all
148,053 baseline Reading identities survive after the explicit representation
normalization. It replaces retired ordinary predicate variants with
`SelectedPredicate`, retains every Word/owner/form/frame choice/marker and
ordered child, and erases only the scalar-only category projection wrapper.
No spelling-based equivalence or preferred-Reading pruning is used.
The final export contains 149,172 Readings: 1,119 added
across 132 faces (51 previously uncovered, 81 already covered), zero removed
baseline identities and zero duplicate normalized identities.

Every counted corpus Reading passes lexical ownership and declaration
admission, byte-exact realization, construction traversal and leaf traversal
comparison against materialization traces. Issues, duplicate materializations,
cyclic derivations and internal failures are all zero. Independent authored
values test the opposite roundtrip law for Amass, selected With, ordered and
optional slots, marker homographs and the repaired relative clause.

Forbidden word/card/lexeme-named admission guards: zero. Head and marker guards
read declared Frame and `FrameMarker` features. Source reconciliation derives
marker eligibility from declaration references, never grammatical word spelling.
Lexical environment loading succeeds with zero load errors; the former
`environment.rs` authority is now `deckmaste_lexical_source::load_workspace`.

### Census and newly covered identities

| Tree / covered | No Reading | One Reading | Multiple Readings | Exact Readings |
|---|---:|---:|---:|---:|
| `rmxmzuss` / 13,226 | 19,602 | 6,552 | 6,674 | 148,053 |
| `ykrqsqyr` / 13,277 | 19,551 | 6,576 | 6,701 | 149,172 |

Specificity-resolved count is zero on both trees: all grammatical Readings are
retained. Optional cheapest samples do not decide admission. Selected With
Complements coexist with the existing NP-plus-PP Adjunct analyses; the named
construction contrast is `SelectedPredicate` versus `PrepositionPredicate`.
Current corpus tooling emits no legacy licensing-checker total. The new
consumer adds two declared-feature guard forms (Frame and marker class), and
adds zero word-named licensing checkers.

All 51 newly covered face identities follow. These analyses were inspected
across the exported Reading sets. Amass selects an Accusative Phrase followed
by Amount; finite Choose selects Cardinal; negative copulas select the overt
adjectival or nominal Predicative Complement inside their Clause.

| Identity | Card face | New selected analysis | Readings |
|---|---|---|---:|
| `080f504f-20c8-4975-837c-d704e879d7f5#card` | Great Goblin, Foul-Hearted | Amass: NP + Amount | 1 |
| `08a64b22-49da-415e-98ce-e1a61d3dfff3#card` | Neurok Transmuter | negative copula: nominal PC | 20 |
| `0c77d37e-791c-415e-b369-55e93147ec34#card` | Skizzik | negative copula: adjectival PC | 1 |
| `149887fa-6b2d-495e-b3e0-b2ac7d3dc838#card` | Easterling Vanguard | Amass: NP + Amount | 1 |
| `1b415806-a3e3-4d87-b4c1-ccd11a8ee589#card` | Saruman's Trickery | Amass: NP + Amount | 1 |
| `25597913-aa17-4e3b-a4e9-371ce1459160#card` | Sphinx of Lost Truths | negative copula: adjectival PC | 1 |
| `27e17542-549b-4c05-8091-c10a245c916b#face:1` | Clap! Snap! | Amass: NP + Amount | 1 |
| `324a30cc-bddd-463c-9f28-c94c93a26780#card` | Eternal Skylord | Amass: NP + Amount | 2 |
| `3a7c2e32-8585-4f7f-8635-199e3c6cd8a9#card` | Toll of the Invasion | Amass: NP + Amount | 4 |
| `3c9faba7-f2d3-4978-be94-020dc8003dc0#card` | Intuition | finite Choose: Cardinal | 28 |
| `4598c221-b111-4bde-b4dc-673186c4efe6#card` | Book of Mazarbul | Amass: NP + Amount | 6 |
| `492108b7-f831-4739-925f-c3a419859c52#card` | Mordor Muster | Amass: NP + Amount | 1 |
| `4fbb8e03-22f1-4171-88e9-877244e25613#card` | Dreadhorde Twins | Amass: NP + Amount | 2 |
| `61897e1b-dea0-44c2-9e38-6e2627647e17#card` | Vizier of the Scorpion | Amass: NP + Amount | 2 |
| `6e4bf4ee-317e-4c31-b121-6efbc6ae4fcf#card` | Invading Manticore | Amass: NP + Amount | 1 |
| `7b238254-51b4-4e68-a8c6-93ab1180012e#card` | Answered Prayers | negative copula: nominal PC | 184 |
| `85643f9b-49ea-434b-9f27-c1431064345e#card` | Lazotep Reaver | Amass: NP + Amount | 1 |
| `85ceb4ac-7a1c-4c7e-95a3-515a5b5d6116#card` | Goblin-town Flunkies | Amass: NP + Amount | 1 |
| `8919c8dd-d3b3-406f-a387-44cc1b6d92f3#card` | Lifecraft Awakening | negative copula: nominal PC | 1 |
| `8b70aad1-bcb1-4359-b27e-ba8b40ef2752#card` | The Torment of Gollum | Amass: NP + Amount | 4 |
| `8ba24644-8dff-4f1e-a96a-7361baf46b5a#card` | Saruman the White | Amass: NP + Amount | 4 |
| `8ba2f559-f9bd-48ef-9fef-6dffe768f4ed#card` | Fa'adiyah Seer | negative copula: nominal PC | 2 |
| `8c7641b6-3a6c-4441-a98c-3b5d61d91f76#card` | Rage into the Valley | Amass: NP + Amount | 2 |
| `90fe98df-4f87-49ff-b562-0ed51d1d3a1c#card` | Thermal Flux | finite Choose: Cardinal; negative copula: adjectival PC | 36 |
| `9219b7d5-642d-4470-a7c4-f3e56bfae8c7#card` | Callous Dismissal | Amass: NP + Amount | 1 |
| `9259e918-ace5-4f13-9035-d8bc8c820d25#card` | March from the Black Gate | Amass: NP + Amount | 1 |
| `92cc43e5-782f-4540-bc60-3d18f357caf3#card` | Iridian Maelstrom | negative copula: nominal PC | 1 |
| `96264dde-c0b6-4912-afc4-18757f938a4d#card` | Grim Initiate | Amass: NP + Amount | 1 |
| `978e07fd-8b8c-4eab-850e-e4c0e712251e#card` | Bothersome Noisemaker | Amass: NP + Amount | 2 |
| `978e0d87-3ff2-4a73-916c-ff0dc0ab2797#card` | Urza's Ruinous Blast | negative copula: adjectival PC | 2 |
| `9cdbe247-9dd3-4894-8e65-43d879c19321#card` | Herald of the Dreadhorde | Amass: NP + Amount | 1 |
| `9dd755a5-0bef-4bde-92fa-14ef2a1a1410#card` | Dunland Crebain | Amass: NP + Amount | 1 |
| `a6fd90dc-0ec3-4dce-a77a-4d04f5e254bf#card` | Winds of Rath | negative copula: adjectival PC | 1 |
| `a9720545-37f4-42f1-932f-13ef88c9ea72#card` | Deceive the Messenger | Amass: NP + Amount | 2 |
| `b38d1811-64d7-4a35-a86c-3291eecd8391#card` | Summon: Leviathan | negative copula: nominal PC | 2 |
| `ba0082fb-2d4c-489e-8140-93a6fa693fd0#card` | Lazotep Plating | Amass: NP + Amount | 2 |
| `ba7aad10-6897-4e85-a976-1bf9282de5de#card` | Aven Eternal | Amass: NP + Amount | 1 |
| `bd101a9b-4c1e-44c8-b9ef-cd79c4d28f36#card` | Tawnos's Tinkering | negative copula: nominal PC | 16 |
| `bf65e547-9d81-4f4b-8adb-46aee9088906#card` | Mindless Conscription | Amass: NP + Amount | 1 |
| `c394b767-620c-4310-b78f-ece312259e46#card` | Misty Mountains Raider | Amass: NP + Amount | 1 |
| `c96402dc-1509-4136-bbfd-c86a85e3ee6f#card` | Swarming of Moria | Amass: NP + Amount | 1 |
| `dc81069b-b2cf-44b3-98fc-45bb24b815cb#card` | Sindbad | negative copula: nominal PC | 2 |
| `dd464a76-5f96-4e21-b7c6-4a476e7f9ce6#card` | Brainspoil | negative copula: adjectival PC | 1 |
| `e01860cd-0aa1-435a-9ef9-ee412bc458cd#card` | Down, Down to Goblin-town | Amass: NP + Amount | 4 |
| `e282eb5b-f9e6-4fb3-9edf-44e23359357d#card` | Misfortune | finite Choose: Cardinal | 8 |
| `edbd3d4e-78a2-4c03-936d-dcb24cf7c87c#card` | Fearsome Goblin Pair | Amass: NP + Amount | 1 |
| `f3817f63-7bde-46c5-93cb-b4756f5f821e#card` | Gothmog, Morgul Lieutenant | Amass: NP + Amount | 2 |
| `f4437cb3-85f7-49b5-bb41-6a948549d987#card` | Along the Crooked Way | Amass: NP + Amount | 12 |
| `f54573d1-9950-46c3-b8f1-cd6f081d62ad#card` | Relentless Advance | Amass: NP + Amount | 1 |
| `f7716dff-5de7-4b52-966e-72ee3adc8d9a#card` | Gleaming Overseer | Amass: NP + Amount | 2 |
| `f852bc33-e2f8-43be-9e6e-7badd1f894ef#card` | Dose of Dawnglow | negative copula: nominal PC | 4 |

### Unsupported inventory and owners

On `ykrqsqyr`, covered 13,277, the lexical Verb inventory contains 120 owners,
192 Frame assignments and 51 distinct shapes. Of 177 Predicate assignments /
47 shapes, the generic consumer supports 157 assignments / 28 shapes. The
remaining 20 assignments / 19 shapes produce explicit diagnostics. Each row
is routed to live `english-v3-systemic-residuals` for its cause audit and
bounded-owner routing; coordination sharing stays with
`english-v3-frame-coordination`. No legacy payload is silently treated as NP.

| Lexical owners | Ordered shape | Diagnostic |
|---|---|---|
| `core-verb:Cause` | `Complement(Object), InfinitivalMarker(To), Complement(VerbPhrase)` | unsupported slot category Object (Complement) |
| `core-verb:Choose` | `Preposition(Among), Complement(Object)` | unsupported slot category Object (Complement) |
| `core-verb:Cost` | `Complement(ManaAmount), Complement(ComparisonDirection), Complement(ControlledCostAction)` | unsupported slot category ManaAmount (Complement) |
| `core-verb:Deal` | `Complement(Amount), Complement(MassNoun), Complement(DistributionPhrase), [Complement(ReplacementMarker)]` | unsupported slot category MassNoun (Complement) |
| `core-verb:Deal` | `Object(NounPhrase), Complement(ScalarEquality), Preposition(To), Complement(Object)` | unsupported slot category Object (Complement) |
| `core-verb:Deal` | `Object(NounPhrase), Preposition(To), Complement(Object), Complement(ScalarEquality)` | unsupported slot category Object (Complement) |
| `core-verb:Enter` | `Preposition(With), Object(NounPhrase), Preposition(On), Complement(FrameComplement)` | unsupported slot category FrameComplement (Complement) |
| `core-verb:Enter` | `Complement(Object), [Preposition(Under) Complement(Object)]` | unsupported slot category Object (Complement) |
| `core-verb:Enter` | `Preposition(Under) Complement(Object)` | unsupported slot category Object (Complement) |
| `core-verb:Gain`, `core-verb:Have` | `Complement(GrantedAbility)` | unsupported slot category GrantedAbility (Complement) |
| `core-verb:Have` | `Complement(Object), Complement(VerbPhrase)` | unsupported slot category Object (Complement) |
| `core-verb:Look` | `Preposition(At), Complement(Object)` | unsupported slot category Object (Complement) |
| `core-verb:Put` | `Complement(Object), [Preposition(From)], Preposition(Onto), Complement(FrameComplement), [Preposition(Under) Complement(Object)]` | unsupported slot category Object (Complement) |
| `core-verb:Put` | `Complement(Object), [Preposition(From)], Preposition(On), Complement(FrameComplement)` | unsupported slot category Object (Complement) |
| `core-verb:Put` | `Complement(Object), Preposition(To), Complement(FrameComplement)` | unsupported slot category Object (Complement) |
| `core-verb:Put` | `Complement(Object), [Preposition(From)], Preposition(On), Complement(FrameComplement), Preposition(In), Complement(ArbitraryDeterminer), CommonNoun(Order)` | unsupported slot category Object (Complement) |
| `core-verb:Remove` | `Object(NounPhrase), Preposition(From), Complement(FrameComplement)` | unsupported slot category FrameComplement (Complement) |
| `core-verb:Return` | `Complement(Object), [Preposition(From)], Preposition(To), Complement(FrameComplement), [Preposition(Under) Complement(Object)]` | unsupported slot category Object (Complement) |
| `lexeme:keyword_action/exile` | `Object(NounPhrase), Complement(ResultativeComplement)` | unsupported slot category ResultativeComplement (Complement) |

### Construction economy and lexical inventories

Counts on the measured baseline/current trees (covered 13,226 / 13,277):

| Quantity | `rmxmzuss` | `ykrqsqyr` |
|---|---:|---:|
| Shared schemas | 55 | 43 |
| Ordinary Constructions | 168 | 168 |
| Named Reading constructors (schema + ordinary) | 223 | 211 |
| Category instances | 332 | 306 |
| Static Productions | 552 | 526 |
| Compiled Productions | 552 | 582 |
| Grammar declaration lines | 2,674 | 2,537 |
| Constructions used in corpus | 167 | 158 |

The 582 compiled Productions include two inert selected-frame templates and
56 concrete ordered-layout expansions; templates are never predicted.
The inventory is unchanged in lexical spellings: 119 exact declared-form
homographs across lexical owners (catalog names and notation excluded; their
own inventories are independent). The named list follows. Grammar form-literal
/ vocabulary overlaps: none on either tree; grammatical words use lexical
leaves rather than word literals.

| Declared surface | Lexical owners |
|---|---|
| 'd | `core-verb:HaveContracted`, `core-verb:WouldContracted` |
| 's | `core-verb:BeContracted`, `core-verb:HaveContracted`, `vocab:Genitive/Default` |
| Adamant | `catalog:ability-words.txt/Adamant`, `lexeme:ability_word/adamant` |
| Addendum | `catalog:ability-words.txt/Addendum`, `lexeme:ability_word/addendum` |
| Alliance | `catalog:ability-words.txt/Alliance`, `lexeme:ability_word/alliance` |
| Battalion | `catalog:ability-words.txt/Battalion`, `lexeme:ability_word/battalion` |
| Bloodrush | `catalog:ability-words.txt/Bloodrush`, `lexeme:ability_word/bloodrush` |
| Celebration | `catalog:ability-words.txt/Celebration`, `lexeme:ability_word/celebration` |
| Channel | `catalog:ability-words.txt/Channel`, `lexeme:ability_word/channel` |
| Chroma | `catalog:ability-words.txt/Chroma`, `lexeme:ability_word/chroma` |
| Cohort | `catalog:ability-words.txt/Cohort`, `lexeme:ability_word/cohort` |
| Constellation | `catalog:ability-words.txt/Constellation`, `lexeme:ability_word/constellation` |
| Converge | `catalog:ability-words.txt/Converge`, `lexeme:ability_word/converge` |
| Council's dilemma | `catalog:ability-words.txt/Council's dilemma`, `lexeme:ability_word/councilsDilemma` |
| Coven | `catalog:ability-words.txt/Coven`, `lexeme:ability_word/coven` |
| Delirium | `catalog:ability-words.txt/Delirium`, `lexeme:ability_word/delirium` |
| Descend 4 | `catalog:ability-words.txt/Descend 4`, `lexeme:ability_word/descend4` |
| Descend 8 | `catalog:ability-words.txt/Descend 8`, `lexeme:ability_word/descend8` |
| Disappear | `catalog:ability-words.txt/Disappear`, `lexeme:ability_word/disappear` |
| Domain | `catalog:ability-words.txt/Domain`, `lexeme:ability_word/domain` |
| Eerie | `catalog:ability-words.txt/Eerie`, `lexeme:ability_word/eerie` |
| Eminence | `catalog:ability-words.txt/Eminence`, `lexeme:ability_word/eminence` |
| Enrage | `catalog:ability-words.txt/Enrage`, `lexeme:ability_word/enrage` |
| Fateful hour | `catalog:ability-words.txt/Fateful hour`, `lexeme:ability_word/fatefulHour` |
| Fathomless descent | `catalog:ability-words.txt/Fathomless descent`, `lexeme:ability_word/fathomlessDescent` |
| Ferocious | `catalog:ability-words.txt/Ferocious`, `lexeme:ability_word/ferocious` |
| Flurry | `catalog:ability-words.txt/Flurry`, `lexeme:ability_word/flurry` |
| Formidable | `catalog:ability-words.txt/Formidable`, `lexeme:ability_word/formidable` |
| Grandeur | `catalog:ability-words.txt/Grandeur`, `lexeme:ability_word/grandeur` |
| Hellbent | `catalog:ability-words.txt/Hellbent`, `lexeme:ability_word/hellbent` |
| Heroic | `catalog:ability-words.txt/Heroic`, `lexeme:ability_word/heroic` |
| I | `vocab:ChapterNumeral/One`, `vocab:SubjectPronoun/I` |
| Imprint | `catalog:ability-words.txt/Imprint`, `lexeme:ability_word/imprint` |
| Infusion | `catalog:ability-words.txt/Infusion`, `lexeme:ability_word/infusion` |
| Inspired | `catalog:ability-words.txt/Inspired`, `lexeme:ability_word/inspired` |
| Join forces | `catalog:ability-words.txt/Join forces`, `lexeme:ability_word/joinForces` |
| Kinship | `catalog:ability-words.txt/Kinship`, `lexeme:ability_word/kinship` |
| Landfall | `catalog:ability-words.txt/Landfall`, `lexeme:ability_word/landfall` |
| Lieutenant | `catalog:ability-words.txt/Lieutenant`, `lexeme:ability_word/lieutenant` |
| Magecraft | `catalog:ability-words.txt/Magecraft`, `lexeme:ability_word/magecraft` |
| Metalcraft | `catalog:ability-words.txt/Metalcraft`, `lexeme:ability_word/metalcraft` |
| Morbid | `catalog:ability-words.txt/Morbid`, `lexeme:ability_word/morbid` |
| More Than Meets the Eye | `catalog:keyword-abilities.txt/More Than Meets the Eye`, `lexeme:keyword_ability/moreThanMeetsTheEye` |
| Opus | `catalog:ability-words.txt/Opus`, `lexeme:ability_word/opus` |
| Pack tactics | `catalog:ability-words.txt/Pack tactics`, `lexeme:ability_word/packTactics` |
| Paradox | `catalog:ability-words.txt/Paradox`, `lexeme:ability_word/paradox` |
| Parley | `catalog:ability-words.txt/Parley`, `lexeme:ability_word/parley` |
| Radiance | `catalog:ability-words.txt/Radiance`, `lexeme:ability_word/radiance` |
| Raid | `catalog:ability-words.txt/Raid`, `lexeme:ability_word/raid` |
| Rally | `catalog:ability-words.txt/Rally`, `lexeme:ability_word/rally` |
| Renew | `catalog:ability-words.txt/Renew`, `lexeme:ability_word/renew` |
| Repartee | `catalog:ability-words.txt/Repartee`, `lexeme:ability_word/repartee` |
| Revolt | `catalog:ability-words.txt/Revolt`, `lexeme:ability_word/revolt` |
| Secret council | `catalog:ability-words.txt/Secret council`, `lexeme:ability_word/secretCouncil` |
| Spell mastery | `catalog:ability-words.txt/Spell mastery`, `lexeme:ability_word/spellMastery` |
| Strive | `catalog:ability-words.txt/Strive`, `lexeme:ability_word/strive` |
| Survival | `catalog:ability-words.txt/Survival`, `lexeme:ability_word/survival` |
| Sweep | `catalog:ability-words.txt/Sweep`, `lexeme:ability_word/sweep` |
| Tempting offer | `catalog:ability-words.txt/Tempting offer`, `lexeme:ability_word/temptingOffer` |
| Threshold | `catalog:ability-words.txt/Threshold`, `lexeme:ability_word/threshold` |
| Undergrowth | `catalog:ability-words.txt/Undergrowth`, `lexeme:ability_word/undergrowth` |
| Valiant | `catalog:ability-words.txt/Valiant`, `lexeme:ability_word/valiant` |
| Vivid | `catalog:ability-words.txt/Vivid`, `lexeme:ability_word/vivid` |
| Void | `catalog:ability-words.txt/Void`, `lexeme:ability_word/void` |
| Will of the council | `catalog:ability-words.txt/Will of the council`, `lexeme:ability_word/willOfTheCouncil` |
| control | `core-verb:Control`, `lexeme:CommonNoun/Control` |
| copies | `core-verb:Copy`, `lexeme:CommonNoun/Copy` |
| copy | `core-verb:Copy`, `lexeme:CommonNoun/Copy` |
| cost | `core-verb:Cost`, `lexeme:CommonNoun/Cost` |
| costs | `core-verb:Cost`, `lexeme:CommonNoun/Cost` |
| counter | `lexeme:CommonNoun/Counter`, `lexeme:keyword_action/counter` |
| counters | `lexeme:CommonNoun/Counter`, `lexeme:keyword_action/counter` |
| cycling | `core-verb:Cycle`, `lexeme:keyword_ability/cycling` |
| deathtouch | `lexeme:counter_kind/deathtouchCounter`, `lexeme:keyword_ability/deathtouch` |
| decayed | `lexeme:counter_kind/decayedCounter`, `lexeme:keyword_ability/decayed` |
| die | `core-verb:Die`, `lexeme:CommonNoun/Die` |
| double strike | `lexeme:counter_kind/doubleStrikeCounter`, `lexeme:keyword_ability/doubleStrike` |
| draw | `core-verb:Draw`, `lexeme:CommonNoun/Draw` |
| draws | `core-verb:Draw`, `lexeme:CommonNoun/Draw` |
| exalted | `lexeme:counter_kind/exaltedCounter`, `lexeme:keyword_ability/exalted` |
| exile | `lexeme:CommonNoun/Exile`, `lexeme:keyword_action/exile` |
| exiles | `lexeme:CommonNoun/Exile`, `lexeme:keyword_action/exile` |
| first strike | `lexeme:counter_kind/firstStrikeCounter`, `lexeme:keyword_ability/firstStrike` |
| flying | `lexeme:counter_kind/flyingCounter`, `lexeme:keyword_ability/flying` |
| goaded | `lexeme:designation/goaded`, `lexeme:keyword_action/goad` |
| harnessed | `lexeme:designation/harnessed`, `lexeme:keyword_action/harness` |
| haste | `lexeme:counter_kind/hasteCounter`, `lexeme:keyword_ability/haste` |
| her | `vocab:ObjectPronoun/Her`, `vocab:PossessiveDeterminerPronoun/Her` |
| hexproof | `lexeme:counter_kind/hexproofCounter`, `lexeme:keyword_ability/hexproof` |
| his | `vocab:PossessiveAbsolutePronoun/His`, `vocab:PossessiveDeterminerPronoun/His` |
| if | `vocab:Preposition/If`, `vocab:Subordinator/If` |
| indestructible | `lexeme:counter_kind/indestructibleCounter`, `lexeme:keyword_ability/indestructible` |
| it | `vocab:ObjectPronoun/It`, `vocab:SubjectPronoun/It` |
| less | `vocab:Adjective/Less`, `vocab:Determinative/Less` |
| lifelink | `lexeme:counter_kind/lifelinkCounter`, `lexeme:keyword_ability/lifelink` |
| menace | `lexeme:counter_kind/menaceCounter`, `lexeme:keyword_ability/menace` |
| name | `lexeme:CommonNoun/Name`, `lexeme:Verb/Name` |
| names | `lexeme:CommonNoun/Name`, `lexeme:Verb/Name` |
| one | `lexeme:CommonNoun/One`, `vocab:CardinalDeterminative/One` |
| reach | `lexeme:counter_kind/reachCounter`, `lexeme:keyword_ability/reach` |
| shadow | `lexeme:counter_kind/shadowCounter`, `lexeme:keyword_ability/shadow` |
| solved | `lexeme:designation/solved`, `lexeme:keyword_ability/solved` |
| suspected | `lexeme:designation/suspected`, `lexeme:keyword_action/suspect` |
| tapped | `lexeme:keyword_action/tap`, `lexeme:keyword_action/tap/adjective` |
| target | `lexeme:CommonNoun/Target`, `lexeme:Verb/Target`, `vocab:TargetingMarker/Target` |
| targets | `lexeme:CommonNoun/Target`, `lexeme:Verb/Target` |
| that | `vocab:SingularDemonstrative/That`, `vocab:Subordinator/That` |
| time | `lexeme:CommonNoun/Time`, `lexeme:counter_kind/timeCounter` |
| to | `vocab:InfinitivalMarker/To`, `vocab:Preposition/To` |
| trample | `lexeme:counter_kind/trampleCounter`, `lexeme:keyword_ability/trample` |
| turn | `core-verb:Turn`, `lexeme:CommonNoun/Turn` |
| turns | `core-verb:Turn`, `lexeme:CommonNoun/Turn` |
| untap | `lexeme:keyword_action/untap`, `vocab:AttributiveAdjective/Untap` |
| untapped | `lexeme:keyword_action/untap`, `lexeme:keyword_action/untap/adjective` |
| vigilance | `lexeme:counter_kind/vigilanceCounter`, `lexeme:keyword_ability/vigilance` |
| you | `vocab:ObjectPronoun/You`, `vocab:SubjectPronoun/You` |
| ’d | `core-verb:HaveContracted`, `core-verb:WouldContracted` |
| ’s | `core-verb:BeContracted`, `core-verb:HaveContracted`, `vocab:Genitive/Default` |
| ∞ | `catalog:keyword-abilities.txt/∞`, `lexeme:keyword_ability/infinity` |

### Deviations, assurance and STOP resolutions

- Added `SelectedPredicate` and the scalar-only `ScalarMeasurePhrase`
  projection; retired 13 handwritten ordinary predicate schemas and
  `SecondaryCardinal`, and moved only the ordinary selected-PP instances to
  the shared consumer. The scalar projection preserves the old Scalar feature
  restriction. Auxiliary/passive/depictive/sharing distinctions remain.
- Tests: restored 0; re-spelled 36 (35 English tests directly or through four
  shared local helpers, plus the exact macro-owned resultative-frame test);
  added 8 (three compiler, four grammar, one lexical-source); removed 0.
  No ignore was added. Two pre-existing ignores remain: grammar
  `seedborn_muse_retains_relative_clause_attachment` is blocked on
  `english-v3-relative-clause-adjuncts`; xtask
  `macro_schema_census_count_matches_21` cross-checks the live corpus census
  on demand. Existing exact outcomes remain.
- STOP 1: an Object-role-to-NP alias admitted an extra Enter Reading in the
  Grafdigger's Cage exact witness. User-authorized repair removes that alias;
  all unreconciled Object role payloads remain unsupported. The original exact
  one-Reading contract passes.
- STOP 2: Urza's Ruinous Blast newly admitted four wrong Readings with an
  auxiliary-stranding gap at `aren't` and `legendary` selected by exile.
  Copular negative/contracted be now has a Predicate frame kind. The intended
  relative owns its adjective in both exact full-card Readings.
- STOP 3: attempted deletion of exile's resultative frame conflicted with the
  recorded 2026-08-26 declaration-language amendment and its exact test.
  The frame is preserved, re-spelled with existing `Role("ResultativeComplement")`
  metadata so its function is explicit. The declaration test still compares the
  complete two-frame value. Its unreconciled typed slot is diagnosed and routed
  above. General copular PredicativeComplement conversion remains intact for
  all other source declarations; no global conversion or named guard ships.
- CGEL Ch. 4 §§5, 5.3 (pp. 251–263) distinguishes resultatives from depictives
  and requires verb licensing for resultatives; Ch. 17 §7.1 (p. 1523) explains
  recoverability for auxiliary ellipsis. This evidence motivated the typed
  resultative reconciliation. The missing Resultative Complement term was
  added to the Oracle English glossary with those linguistic citations; no
  Game Model concept or rules citation was introduced.
- Test-authoring repairs select third-person singular `shares`; the isolated
  `aren't legendary` witness retains all four licensed agreement bundles, while
  the relative-Clause Subject constrains the full card to third-person plural.
  The archive-only additional `pay {1}` assertion assumed an unlanded lexical
  frame and was not imported; existing payment tests/declarations are retained.
- Corpus reports from temporary attempted repairs and the stale executable are
  discarded. Final measurement rebuilds with `cargo xtask english-v3`; its
  Reading totals agree with the independently rebuilt export. Scratch reports
  and normalization/verifier code remain in `/tmp`, never tracked in crates.

### Verification and performance advisory

`cargo xtask gate --changed --clippy --run` passes the reverse-dependency
closure, including consumed plugin declarations: 1,209 passed, 0 failed,
2 pre-existing ignored tests across 92 test/doc-test suites.

```text
cargo test -p deckmaste_construction_core -p deckmaste_construction_v3_core -p deckmaste_lexical_source -p deckmaste_semantics_v2 -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask
cargo clippy -p deckmaste_construction_core -p deckmaste_construction_v3_core -p deckmaste_lexical_source -p deckmaste_semantics_v2 -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask --all-targets -- -D warnings
```

Final corpus command: `cargo xtask english-v3 --all --workers 8
--samples-per-face 0 --output /tmp/generic-frame-landing.json`; zero issues.
Formatting passes. Citation checks report no noncompliant strings and zero stale
citations. No new Comprehensive Rules citation needs blessing.

| Measurement tree / covered | Workers | Host load (1/5/15 min) | Corpus wall ns | Checked-text thread CPU ns/B |
|---|---:|---|---:|---:|
| `rmxmzuss` / 13,226 | 8 | 6.42/6.33/5.98 | 208,360,683,904 | 717,468 ns/B |
| `ykrqsqyr` / 13,277 | 8 | 9.47/15.0/14.8 | 330,012,609,089 | 1,031,290 ns/B |

Both busy-host measurements exceed the inherited 16.26s quiet-host ceiling.
This is a performance advisory, not a gate or a fitted target. The baseline
report's late tree stamp is superseded by its measured parent `rmxmzuss`.
Scratch provenance: `/tmp/generic-frame-before.json`,
`/tmp/generic-frame-landing.json`, `/tmp/generic-frame-verified-identities.json`
and `/tmp/generic-frame-verified-identity-delta.json`.
