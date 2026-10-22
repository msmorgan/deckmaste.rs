---
needs: [english-v4-unextended-and-closures]
---
# Audit: control flow keyed on child names, and Category fission (Nodules 4 and 5)

**Needs an owner ruling before any implementation.** The owner adopted only
items 1–4 of the round-8 treatment plan (`docs/decisions/english-v4.md` §4.4).
This ticket inventories the audit's Nodules 4 and 5 and proposes a fix for
each finding. It changes no code. Once the owner rules, the ruled fixes become
their own tickets.

It follows `english-v4-unextended-and-closures`, which may delete some of the
name-keyed sites below (`closure.rs`'s `head` test, for one).

## What the audit named

1. **Hidden control flow on child names.** Engine behaviour depends on the
   names authors give children in `english.ron`, and authors rename a head to
   avoid it.
2. **Category fission.** Subcategories (`category: Some(..)`) duplicate an
   existing phrase's Productions to gate a slot, and features tunnel a
   grandchild's values up through a child.

## Evidence

Re-grepped on `qynmxmro`. Line numbers drift.

**Engine sites keyed on a declared child name** (`crates/deckmaste_syntax/src`):

- `grammar.rs:191–193`: "A child named `head` implicitly agrees with the mother
  on every feature both carry". Applied in `compile.rs:1186` (`add`); Valence
  agreement skips `head` at `compile.rs:1500`.
- `step.rs:377–381`: `head` sets the head position, and `head`, `first`,
  `last` and `rest` make the head line, which `head_final`,
  `final_preposition`, `post_adjunct` and coordinator handling follow
  (`step.rs:707–764, 1155`).
- `step.rs:996`: `first` names a first conjunct. `step.rs:1026`: a
  post-dependent host is recognized by every later child being named
  `adjunct`. `step.rs:1158`: `complement` decides `delimited_complement`.
- `reading.rs:671–712` (`attachment_key`): Right Association charges a child by
  its name. `head` is charged per later `adjunct` or `modifier`; `adjunct`,
  `modifier` and `supplement` after a `head` are charged by distance, with
  `supplement` reversed; `last` adds conjunct balance. Also
  `reading.rs:263, 277, 556–558, 1001–1007, 1113`.
- `compile.rs:867` and `closure.rs:146`: a fused head's twin must be followed
  by a child named `head`.

**Head renamings in `english.ron`** that keep a semantic head out of that
behaviour:

- `DistributiveCoordinationDetermined` (`:1101`): the Nominal is `nominal`, with
  `transparent: ["nominal"]` and explicit `Agree`s.
- `TransparentQuantity` (`:1123`) and `TemporalQuantity` (`:1137`): the
  quantificational Nominal is `quantity`. The Noun Phrase takes its Number from
  the oblique, not the head (CGEL, Ch. 5 §3.3, pp. 349–351).
- `PawprintMode` (`:502`): `quantity` beside `head`.

**Category fission:**

- `SubjectInitialClause` (`:121`) restates the Clause Productions
  `SubjectPredicate` (`:540`), `CliticSubjectPredicate` (`:554`),
  `FloatedEach` (`:1019`) and `FloatedAll` (`:1022`) as
  `RelativeSubjectPredicate`, `RelativeCliticSubjectPredicate`,
  `RelativeFloatedEach` and `RelativeFloatedAll` (`:1263–1272`), so that a bare
  relative's Clause has its Subject first (CGEL, Ch. 12 §3.5.5, p. 1055).
  `RelativeAdjunctGap` (`:1308`) is a fifth `SubjectInitialClause` Production.
- `english.ron` declares 61 subcategories with `category: Some(..)`
  (`:115–300`). Some are genuine distributions (`Gerund`, `ToInf`); others may
  be gates. Each needs classifying.
- **Grandchild tunneling.** `ObliqueNumber`, `ObliqueKind`, `ObliqueNounKind`,
  `ObliqueDetermination` and `PrepositionMarking` copy a head's oblique
  Complement's features onto the head's phrase, through the `agree` lists of
  the Noun and Determinative Valences (`:1389`, `:1396`), so a grandparent can
  constrain them. The audit counts six such features; confirm the sixth
  (`ConjunctNumber` and `QuantityNumber` are candidates).

## The audit

For each finding:

1. Say what the name, Category or feature decides, and which Readings or
   Productions depend on it.
2. Classify it:
   - structural and wanted (state it explicitly instead);
   - a gate whose purpose is to suppress a Reading (frozen by §4.4, item 2);
   - or genuine coverage.
3. Propose a fix. Examples: a declared role on the Production instead of a
   reserved child name; one Clause Production with a declared ordering instead
   of a duplicate Category; a declared path to the oblique instead of copied
   features. State what each fix would cost in census and judgments, from a
   measurement where one is cheap.

Write the inventory and proposals into this ticket, then STOP for the owner's
ruling.

## Findings (2026-10-22, on `smnnwupo`)

The Evidence above is from `qynmxmro`; the line numbers below supersede it.
Nodules 2 and 3 have landed, so `closure.rs`, `fused_head`, the `admit_as`
re-parse and the predicate-conjunct hosts are gone. The owner added two
items to this ticket's scope: delete `postdependent`, and triage the load
report.

**Baseline** (`kzvruptx`, the claim):
- faces read 28,800;
- Readings 237,320;
- Surplus 101,963;
- cheapest tier: a singleton in 15,824 of 17,634 ambiguous units, the tiers
  sum to 20,053, and the largest is 8.

Every probe below was measured against this baseline and then reverted.

### The thesis

A child's name is its grammatical function. `Production::names` says so
("The name of each child: its grammatical function in the mother"), and the
judgments assert functions through these names. So an engine that keys on
`adjunct`, `modifier` or `head` is keying on declared functions, not on
arbitrary names. That is structural, and §4.3 is defined that way: the Ranking
Key charges "each unselected dependent".

The audit's defects are narrower:
- **(i) Names that lie.** A semantic head is renamed so that it escapes
  implicit head agreement. It then also escapes everything else the engine
  hangs on `head`, silently.
- **(ii) Two notions of the same thing.** The chart recognizes a coordination
  by `Production::coordinator`. The Ranking Key recognizes one by the names
  `first` and `last`.
- **(iii) An undocumented vocabulary.** The reserved names are nowhere listed
  as the grammar's contract.

### A. Engine sites keyed on child names

"Assigned by" says who writes the name. *Engine* means the engine derives the
name itself (adjunct schemas from `HostKind`, Valence from `SlotRelation`,
Coordination schemas). *Author* means an `english.ron` Production names it.

| # | Site | Names | What it decides | Assigned by | Class | Proposal |
|---|---|---|---|---|---|---|
| A1 | `compile.rs:1253` (`add`); documented at `grammar.rs:174` | `head` | Implicit agreement of the mother with `head` on every feature both carry | author | structural, wanted | Keep. It is the reason for every rename in B. See B for the options. |
| A2 | `compile.rs:1567` (`slot_agreement`) | `head` | Valence `agree` pairs skip the frame head | engine | structural | Keep. It could key on child 0 of a Valence draft. Cosmetic, no ticket. |
| A3 | `compile.rs:1477–1496` (adjunct schemas); `:2358, :2395` (`valence_order`); `:3037` (coordination `first`); `:3060` (`postposed_modifier`); `:3106` (`complement_clusters`) | `head`, `modifier`, `adjunct`, `supplement`, `object`, `first` | The engine names its own derived children and later recognizes them by those names | engine | structural | Keep. It is internally consistent. Comparing `SlotRelation` would be cleaner but is no fix. |
| A4 | `step.rs:367` (`resolve`) → `Production::head`; used at `step.rs:666` (`priority`) | `head` | The phrase inherits the head's preposition, second preposition, extensions and conjunct hosts. This drives selected `PP` slots and same-head complement priority. | author | structural | Keep. A renamed head loses all of this silently (B). |
| A5 | `step.rs:370` → `head_line`; used at `:695–698` (`head_final`), `:723` (`final_preposition`), `:752` (conjunct omissions), `:1141` (`post_adjunct`) | `head`, `first`, `last`, `rest` | The head line: whether a phrase ends in its head, which PP it ends in, and whether a comma-set post-adjunct rises | author and engine | structural | Keep, and document (ticketed). |
| A6 | `step.rs:982` (`comma_scope_excludes`) | `first` | A fronted dependent never sits inside a first conjunct (english-lexical-analysis §2, "fronted dependent outermost") | engine (Coordination) | structural | Keep. It is already gated on `coordinator.is_some()`. |
| A7 | `step.rs:1003–1012` (`post_adjunct_host`) | `adjunct` | An auxiliary Verb Phrase hosts no post-verbal Adjunct (§2, "adjuncts below a non-modal auxiliary"). It holds when every later child is `adjunct`. | author and engine | structural | Keep. Keying it on `host_check` instead is **not** neutral: nine authored Verb Phrase adjunct Productions carry no `host_check` (EndAdverbial `:613`, GappedPurpose `:656`, PositionDepictive `:681`, CoordinatedDepictive `:702`, CoordinatedAdjunct `:706`, DepictiveAdjective `:707`, TemporalAdjunct `:787`, MannerAdjunct `:788`, FrequencyInterval `:789`). The name is the declaration. |
| A8 | `step.rs:1144` | `complement` | A delimited Clause coordination rises only through a Complement or a comma-set dependent | engine (Valence `relation_name`) and author | structural | Keep, and document (ticketed). |
| A9 | `step.rs:984, 994, 1281` (comma-set dependent, `fronted`) | `head` (position) | `fronted` holds when the head is last and is preceded by a comma or a preposed adjunct | author | structural | Keep. |
| A10 | `reading.rs:263, 277` (`edge_key`); `:556–558` (`key_of`); `:675–727` (`attachment_key`) | `head`, `first`, `last`, `adjunct`, `modifier`, `supplement` | The whole Ranking Key: Right Association (host words for each later `adjunct`/`modifier`, distance for each dependent, reversed for `supplement`), conjunct balance and the Attachment Classes | author and engine | structural, as §4.3 defines it | Keep; the `deckmaste_syntax` README documents it. Defect (ii) is at `reading.rs:287`; see E. |
| A11 | `reading.rs:1049–1067` (`locate`) | `object`, `modifier`, `head`, `shared` | The functions a Reading reports: a postposed Modifier also bears its function on the Object, a function Category's head bears the function, and the Right Node Raising gap sites | author and engine | structural (report only) | Keep. |
| A12 | `reading.rs:1162–1168` (`head_word`) | `head` | The head lexeme of every Constituent, which judgments assert | author | structural | Keep. Because of B, *any number of target creatures*, *each creature and planeswalker* and *any number of times* have **no head** in their Readings, so no judgment can assert one. |
| A13 | `reading.rs:633–642` (`distributive_phrase`) | Category `PrepositionPhrase`, `DeterminativeDistribution` `Distributive` | The distributive Attachment Class | none (it names grammar) | line (a) violation | Already ticketed: `english-v4-engine-feature-names`. |
| A14 | `deckmaste_english_v4/src/judgment.rs:629–654` | `head`, `object` | Judgment evaluation reads functions | – | harness | Keep. |

`ProductionDecl::postdependent` and `Weight::PostDependent`, the only
name-weighted admission veto, are deleted (`smnnwupo`). The census and the
cheapest tier are identical.

### B. Head renames

In each case the renamed child is the semantic head. It is renamed for one
reason: implicit head agreement (A1) would equate features that this
construction takes from elsewhere.

| Production | Renamed child | Shared features the rename avoids | Why | Also lost (A4, A5, A12) |
|---|---|---|---|---|
| DistributiveCoordinationDetermined `:1089` | `nominal` | Number, NounKind, Countability, Determination | The Noun Phrase is singular from its distributive determiner, while its *and*-coordination is plural (CGEL, Ch. 5 §18.4, p. 508) | head lexeme; head line |
| TransparentQuantity `:1111` | `quantity` | Number, NounKind (Countability, Determination) | Number and NounKind come from the oblique (`ObliqueNumber`, `ObliqueNounKind`; CGEL, Ch. 5 §3.3, pp. 349–351) | head lexeme; head line |
| TemporalQuantity `:1125` | `quantity` | Number, NounKind | As above, with NounKind `Temporal` from the oblique | head lexeme; head line |

Non-findings:
- **PawprintMode `:500`.** It is not a rename: `head` is the ModeEffect, and
  `quantity` names the pawprint cost. Elsewhere that child is named `cost`
  (PlusCostedMode, LabelledCostedMode). This is cosmetic.
- **The other 60 headless Productions.** They are unary function wrappers
  (`filler`), fused heads (`quantifier`, `determiner`, `possessor`),
  marker-plus-clause relatives and interrogatives (`marker`, `clause`), and
  spines (`cost`/`effect`, `lower`/`upper`). The fused-head ones also avoid
  agreement, for example ComparativeFusedHead `:958`: "The Noun Phrase has
  its standard's Number, not the quantifier's". Their names are defensible
  functions.

**Probe P2: an explicit `Agree` of `$.F` with any other path overrides
implicit head agreement on F; the three children renamed `head`.**
- The first run lost 6 faces (*pay … any number of times*), because
  TemporalQuantity's `Require($.NounKind, Temporal)` met the head's own
  NounKind. With that equation written as `Agree($.NounKind,
  head.ObliqueNounKind)`, no face is lost. One face newly reads (Flow State,
  unverified).
- Readings 237,320 → 237,584, Surplus +58, singleton cheapest tiers
  15,824 → 15,821, 19 units gain Readings.
- All 19 come from the override loosening **other** Productions, not the
  three renamed heads. For example, Partitive's `Agree($.Number,
  head.ObliqueNumber)` drops the head determinative's own Number agreement,
  which admits *another of those counters* and *all but 1 of that damage* a
  second time.
- So implicit head agreement is doing real work in the partitives, and a
  general override rule is not a refactor.

Proposals (ruling R2):
- **B-a (recommended): keep the renames**, each with a comment naming the
  features it keeps out of head agreement, and document what a non-`head`
  name forgoes (ticketed with the reserved names).
- **B-b: a declared per-Production opt-out** (`unagreed: ["Number"]`), so
  that the child can be named `head`. This is honest, but it is a new
  `ProductionDecl` field, which the freeze forbids even though it suppresses
  nothing.
- **B-c: the general override rule.** Measured above. Not recommended.

### C. Category fission: the 61 `category: Some(..)` subcategories

Mechanics: `category: Some(X)` only renames the Category in Readings
(`compile.rs:735–738`) and feeds structural auxiliary detection
(`:753–775`). A subcategory is never X in the chart, so every slot that
should accept it must name it.

Classification, by line:

**F, a genuine distribution or function (keep):**
- Clauses and Verb Phrases: SubjectGapClause 115, Participle 123,
  ExistentialPredicate 128, FiniteClause 273, VerblessClause 278, ToInf 284,
  HollowInf 286, BareInf 287, AuxiliaryComplement 289, Gerund 290,
  GerundClause 293, InterrogativeClause 296.
- Noun Phrases: DummyPronoun 127, FusedNounPhrase 129, TemporalNounPhrase 158,
  ComplementNounPhrase 165, RelativeNounPhrase 261, InterrogativePhrase 298.
- Nominals: QuantifiedNominal 169, ModifiedQuantifiedNominal 170,
  BareNominal 173, NounPremodifier 174, NounPremodifierConjunct 178,
  BareNominalComplement 185.
- Determiner Phrases: NumericalDeterminer 194, FusableDeterminer 200,
  FusedQuantity 201, TargetingMarker 209.
- Function Categories and coordinations of unlike categories: Depictive 222,
  DepictiveCoordination 223, ObjectConjunct 233, ObjectCoordination 234,
  PostmodifierConjunct 237, PostmodifierCoordination 238, Attribute 239,
  AdjunctCoordination 247.
- RelativePrepositionPhrase 263.

**C, coverage (keep):**
- AdjunctGapClause 117, SymbolHead 134, PartitiveHead 143,
  MeasureNounPhrase 146, LocativeHead 147, LocativeNounPhrase 148,
  PrepositionalNominal 179.
- TargetedOblique 183 and TargetedNounPhrase 184.
- GenitiveNominal 192, GenitiveNominals 193, PositionPhrase 199.

**G, a gate.** The table covers each gate, with the parts that duplicate
another phrase's Productions and the proposed fix.

| Line | Subcategory | What it keeps out | Duplicates | Class | Proposal | Ruling |
|---|---|---|---|---|---|---|
| 121 | **SubjectInitialClause** | A preposed dependent before a bare relative's Subject | RelativeSubjectPredicate `:1252`, RelativeCliticSubjectPredicate `:1255`, RelativeFloatedEach `:1258` and RelativeFloatedAll `:1260` restate SubjectPredicate `:540`, CliticSubjectPredicate `:554`, FloatedEach `:1021` and FloatedAll `:1024` (identical but for `$.ClauseMarking`). RelativeAdjunctGap `:1297` restates AdjunctGap `:1299`. | G-ungrammatical: "Bare relatives always have the subject in initial position" (CGEL, Ch. 12 §3.5.5, p. 1055). But it is **over-broad**: it also shuts out coordinated bare relatives (*each creature you control and an opponent owns* has 0 Readings, against 1 with *that*) and gerund subjects. | **C-c (recommended):** move the four base Productions to lhs SubjectInitialClause and add one unary `Clause → SubjectInitialClause` lift. That is −3 Productions and no duplication; the Readings gain a unary node. **C-a:** add `Coordination(category: "SubjectInitialClause", …)` as new coverage (probe P3). **C-b:** retire the Category and key bare relatives on `Summary::fronted`. That needs a new `ProductionDecl` list (frozen), and `fronted` would have to see through unary heads. | R3 |
| 140 | PartitiveNounPhrase | A PP postmodifier on *each/any/all of X* (allowed on *two of X*, PartitiveHead) | DeterminativePartitiveHead parallels PartitiveFusedHead | G-preference: inconsistent, and English admits it | Merge into PartitiveHead and let Right Association rank the inner attachment first. −1 Category, −1 Production. No `!` or count witness involves a PP. Risk low to medium: PartitiveHead's DeterminerPhrase heads would start agreeing on DeterminativeDistribution. | R4 |
| 164 | PeriodNounPhrase (with Determined's two `ExcludeTogether`s, `:1079–1080`) | A deictic or distributive period Noun Phrase as a general Object (*can't block [this turn]*) | DeicticPeriod and DistributivePeriod restate Determined `:1076` | G-preference: semantically anomalous, not ungrammatical (CGEL, Ch. 4 §1.2, p. 224) | Retiring it ranks the anomalous Object first (an Object costs 0, TemporalAdjunct costs 2), and §4.3 forbids lexical plausibility | R1 |
| 186, 189 | Possessor, PossessorNominals | A phrasal genitive (*the King of England's daughter*; CGEL, Ch. 5 §16.6, p. 479) | 7 Productions restating Determined, GenitiveDetermined, MaximumDetermination, BarePlural, ProperName and BareSingularTerm over BareNominal | G-preference | Retiring takes −2 Categories and −7 Productions and adds phrasal-genitive Readings everywhere; the ranking order is unmeasured | R1 |
| 240, 243–246 | AttributeChain, NegativeAttribute, NegativeNounPremodifier, NegativeParticiple, NegativeAdjectivePhrase | Asyndetic premodifier chains other than *non-* chains | The three Negative* Categories are exact restatements of PrefixedNounPremodifier `:1037`, the Premodification `/prefix` draft and NegativeAdjective `:441` | G-preference: register (OSG §6, "Determiners and modifier order") | Retiring breaks the `!Attributes` witnesses in `asyndetic_premodifier_coordination.txt` | R1 |
| 277 | PresentClause | A preterite Clause Complement of *whenever/while/until/where* | – | G-preference, declared honestly in the lexicon (OSG §1, "Voice and tense") | Mapping the slot to FiniteClause puts preterite/present ties into the cheapest tier | R1 |
| 172 | RelativeTemporalNominal | Other postmodifiers of a temporal Noun Phrase Adjunct | RelativeTime ≈ NominalPostmodifier `:1191`; RelativeTimePostmodifier ≈ TemporalPostmodifier `:785` | C plus G-preference | Keep (high risk) | R1 |

Several subcategories exist only to dodge implicit head agreement:
TargetingMarker (it drops `Number`), QuantifiedNominal and
ModifiedQuantifiedNominal (QuantityNumber in place of Number), and probably
PartitiveHead (it omits DeterminativeDistribution). B-b would also let these
go. Most unary-wrapper subcategories exist because a `slot_categories` entry
cannot carry equations.

### D. Grandchild tunneling: the Oblique features

The count is **five** features, through **seven** `agree` pairs:
- the Noun Valence `:1376` has 3: ObliqueNumber←Number,
  ObliqueKind←ComplementKind, ObliqueNounKind←NounKind;
- the Determinative Valence `:1383` has 4: ObliqueNumber, ObliqueKind,
  PrepositionMarking←PrepositionMarking,
  ObliqueDetermination←Determination.

ObliqueNumber is tunneled through both, which may be the audit's "six".
TargetedQuantity `:1117` sets three of them by hand.

The two candidates for a sixth are not tunnels:
- **ConjunctNumber** is a coordination's first conjunct's Number, a child
  (`:1481`).
- **QuantityNumber** is an Amount's own agreement value. It reaches a
  Preposition Phrase through the Preposition Valence's
  `("Number", "QuantityNumber")` (`:1375`), which is again a child.

| Feature | Used for (consumers) | Verdict |
|---|---|---|
| ObliqueNumber | The partitive Noun Phrase's Number from its oblique (DistributivePartitive `:824`, TransparentQuantity, TemporalQuantity, TargetedQuantity); the comparative count determiner's agreement with its head (`:1100`) | Keep: a finite summary must carry what the grandparent constrains (english-lexical-analysis §1; CGEL, Ch. 5 §3.3 and §9.3) |
| ObliqueNounKind | NounKind from the oblique (TransparentQuantity, TemporalQuantity) | Keep |
| ObliqueDetermination | A bare standard after *than* (`:857, :958, :1100, :1102`; CGEL, Ch. 5 §11(d), p. 432) | Keep |
| ObliqueKind | *than* + AdvP (`:939`) against NP and clause (`:857, :876, :958, :1102`); **Unmodified `:1152` excludes `Gerund`** | Keep, except the `Gerund` exclusion, which is a gate (probe P4) |
| PrepositionMarking | *of* against *than* against neither, for a determinative's PP (`:822, :850, :852, :857, :900, :958, :1100`) | A closed-word identity carried as a feature. §3 prefers a direct reference: the consumers would take the determinative and its PP as siblings (`Selected(..,["Of"])`), as SynesisDistributivePartitive `:830` already does. That is a restructure (R5) |

**Probe P4: the Unmodified `Exclude("head.ObliqueKind", "Gerund")` removed.**
About 25 units with *the number of attacking creatures* double. The gerund
Reading (*of [attacking creatures]*, a Verb Phrase) **ties** the Noun Phrase
Reading in the cheapest tier (both 0/-8/36), and
`gerund_preposition_complements.txt:13` fails its `!`. The Ranking Key cannot
absorb this gate. It belongs with R1.

### E. The coordinator-leaf `head_final` smuggling

**The mechanism.**
- `step.rs:697` leaves `State::head_final` unchanged across a coordination's
  coordinator leaf. So the item before the last conjunct records whether the
  first conjunct ended in its head.
- `reading.rs:287` reads it there for the Attachment Class "a dependent after
  a coordination attaches to the whole" (§4.3).
- `Summary::head_final` has had no admission use since `postdependent` went.
  It now exists for this ranking component and for packing.

**Defect (ii).**
- The special case keys on `Production::coordinator`, which only schema
  coordinations set.
- `attachment_key` keys on the names `first` and `last`.
- ComparativeBoundedAlternative `:867` and ComparativeBoundedAmount `:874` are
  authored `first`/`coordinator`/`last` Productions with no
  `Production::coordinator`. For them the forest path reads `false`, while the
  tree path (`reading.rs:582`) computes the first conjunct's real value.
- That is a latent forest/tree key mismatch. No current judgment trips it.

**Proposal (ticketed: `english-v4-head-final-last-head-line`).**
- Make `State::head_final` mean "the last head-line child so far ends in its
  head". Children outside the head line leave it unchanged.
- Make `Summary::head_final` that value, provided the last non-punctuation
  child is in the head line.
- This needs no special case and no new field, and it removes the mismatch.

**Probe P1:** census, Readings, Surplus and the cheapest tier are all
identical, and `ranking_keys` and every judgment hold.

### F. Probe summary

| Probe | Faces | Readings | Surplus | Singleton cheapest | Tier sum | Judgments |
|---|---|---|---|---|---|---|
| `postdependent` deleted (`smnnwupo`) | 28,800 | 237,320 | 101,963 | 15,824 | 20,053 | hold* |
| P1 `head_final` = last head-line child | = | = | = | = | = | hold* |
| P2 explicit `Agree` overrides head agreement; 3 honest heads | 28,801 | 237,584 | 102,021 | 15,821 | 20,070 | hold* |
| P3+P4 `Coordination(SubjectInitialClause)`; Unmodified Gerund exclusion removed | 28,800 | 237,527 | 102,072 | 15,814 | 20,083 | P4 breaks `gerund_preposition_complements.txt:13` |

\* "Hold" means 3,039 enforced and 208 pending, except
`preposed_preposition_pair.txt:3`. That entry fails on the claim base itself,
and trunk's `tztrqzts` rewrites it.

In P3, the coordinated bare relatives add Readings to 6 units (Orzhov Charm:
*you control and all Auras you control attached to it*). Each is wrong, and
each ranks behind the endorsed Reading (Attachment Class 1). No face changes.

### G. Load-report triage (36 findings)

The previous ticket listed about 37. FinalParameter `SetOffCondition` has since
gained a second holder, which leaves 36. Paired findings (ComplementKind and
ObliqueKind, NounKind and ObliqueNounKind) are one value seen through the
Valence tunnel, reported twice.

**J: justified closed-class singleton (21).**
- PronounKind `Existential`: dummy *there* (CGEL, Ch. 16 §6.1, p. 1391). The
  value, not a direct reference, keeps *there* out of the referential
  `Pronoun` Production `:1130`.
- ClauseMarking `To`: the *to*-infinitival marker (CGEL, Ch. 14 §1.1,
  p. 1174).
- NounKind and ObliqueNounKind `Manner`: *way* (CGEL, Ch. 8 §2, p. 671). The
  value is justified, but its three Excludes (`:433, :548, :1253`) are
  exclusion-only (R1).
- NounDistribution `Quantity`: *number*, a number-transparent quantificational
  noun (CGEL, Ch. 5 §3.3, p. 350). The class is open (*rest*, *lot*), but
  Oracle uses only *any number of*.
- NounDistribution `Serial`: already justified in `english.ron`.
- Classes `FusedHeadAdjective` (*other*; CGEL, Ch. 5 §9.3, pp. 415–417),
  `RangeMarker` and `Pawprint` (catalog symbols that cannot be Closed Words).
- AdjunctPosition `Front`, `Central`, `Phrasal` and `Bounding` (4).
- ComplementKind and ObliqueKind `MeasureNP`, `VerblessClause` and
  `QuotedAbility` (6).
- FinalParameter `SetOffMeasure` and `SetOffKeyword` (2).

15 of the 21 are **checker artifacts**:
- An AdjunctPosition value is set by a class-lift Production whose real
  holders are the adverb class's members (Front has 5).
- A Slot Kind value is projected from frames, and no equation reads it. Its
  holders are the lexemes that declare the frame: *if* and *while* for
  `VerblessClause`.
- A FinalParameter value is projected from the catalog parameter kinds, and
  nothing reads it.

These go to `english-v4-holder-report-refinements`, which counts the real
holders rather than exempting anything.

**F: fold candidate (8).**
- ComplementKind and ObliqueKind `InterrogativeClause` (0 holders, 0
  readers): delete from both domains. Ticketed:
  `english-v4-unheld-interrogative-clause-kind`.
- Parameterization `Supplied` and `Omitted`: the only reader,
  CompleteKeywordItem `:447`, can take `KeywordParameters` directly. Ticketed:
  `english-v4-parameterization-fold`.
- VerbKind `Lexical` and AdjectivalPassive `Derived` (0 holders): privative
  features. Every reader excludes the marked value. Dropping the unmarked value
  needs the checker to accept a one-valued privative feature, which changes
  english-lexical-analysis §3's "a feature needs two or more values" (R5).
- NounKind and ObliqueNounKind `Symbol`: **exclusion-only**. Its only reader
  is the Verb Valence `internalised` Exclude `:1374`, which stops *{2}* in
  *reduced by {2}* from being read as a passive agent. The `Quantity` Exclude
  on the same line is the same kind (R1).

**T: needs a ticket or ruling (7).**
- DeterminativeDistribution `SingularOrQuantified` (*another*) and
  `IndefiniteArticle` (*a*): CGEL's class (Ch. 5 §3.4, p. 353) is split
  across SingularOrQuantifiedDetermination `:796` and
  DeterminedModifiedQuantity `:798`. AgreeingModifiedQuantity `:799` excludes
  *a* only to avoid a duplicate tree. A Closed Word fold would lose *another
  day* (TemporalLocation; CGEL, Ch. 8 §6, p. 698).
- DeterminativeDistribution `FreeChoice` (*any*): the Requires could be
  `Word("Any")`. The two Excludes (`:1077, :798`) are probably exclusion-only.
  The fallback distribution would lose *any turn* (CGEL, Ch. 5 §5, p. 361).
- DeterminativeDistribution `Targeting` (*target*): `:1122` could take
  `Word("Target")`, but BareTargetingMarker `:1160` needs "this determiner ends
  in the Marker". This is bound up with TargetingMarker's head-agreement dodge
  (B, C).
- PrepositionMarking `Oblique` and `Comparative`: see D.
- Class `InterrogativeAdverb` (*how*): is interrogative *how* a closed function
  word for english-lexical-analysis §3? If so, it folds to a Closed Word
  (CGEL, Ch. 10 §7, p. 902).

### Proposals ranked by value against risk

1. **`head_final` = last head-line child** (E). Neutral (P1), removes the
   special case and a latent key mismatch. Ticketed.
2. **Holder-report refinements** (G). Removes 15 false findings from the load
   report, so the report lists real findings. Ticketed.
3. **SubjectInitialClause C-c** (C). −3 Productions and no duplicate Clause
   rules, but it adds a unary node to the Readings. Needs R3.
4. **Unheld `InterrogativeClause` and the Parameterization fold** (G).
   Mechanical. Ticketed.
5. **Document the reserved child names as the grammar contract**, and fix
   `english.ron` comment drift (A, B-a). Ticketed:
   `english-v4-reserved-child-names-contract`.
6. **PartitiveNounPhrase into PartitiveHead** (C). An inconsistent gate,
   low-to-medium risk (R4).
7. **The G-preference gates the Ranking Key cannot absorb** (PeriodNounPhrase,
   the Unmodified Gerund exclusion, the Manner, Symbol and Quantity Excludes,
   PresentClause, Possessor, AttributeChain). High value for §4.4 consistency,
   high risk to the cheapest tier. Blocked on R1.

### Owner rulings needed

These rulings are carried open in `english-v4-audit-rulings-pending`.

- **R1. Policy.** §4.4 item 1 says a Reading English admits is kept and
  ranked. For a gate whose retired Readings the structural Ranking Key cannot
  order (they tie, or the anomalous one wins), choose one:
  - (a) keep the gate as an admission rule with a stated justification;
  - (b) retire it and accept cheapest-tier ties;
  - (c) add a structural Ranking Key component.

  It applies to PeriodNounPhrase, the Unmodified Gerund exclusion (P4), the
  Manner Excludes, the `internalised` Symbol and Quantity Excludes,
  PresentClause, Possessor, the *non-* chain, RelativeTemporalNominal and the
  FreeChoice Excludes.
- **R2. Head renames.** B-a (recommended), B-b (a new `ProductionDecl` field)
  or B-c (measured; not recommended).
- **R3. SubjectInitialClause.** C-c (recommended), C-a (coverage only) or C-b
  (a new `ProductionDecl` list).
- **R4. PartitiveNounPhrase merge.** Proceed or not.
- **R5. Load-report T and privative items.** Whether english-lexical-analysis
  §3 admits privative one-valued features. Whether *how* is a closed function
  word. How to restructure `SingularOrQuantified`/`IndefiniteArticle`,
  `FreeChoice`, `Targeting` and `PrepositionMarking`.

### Side findings

- **ParticipialSupplement `:611`** has `transparent: ["head"]`, so a Gap
  passes a comma-set post-dependent. The adjunct schemas close that Gap
  (`compile.rs:1482–1485`; CGEL, Ch. 12 §4, p. 1058). This needs a decision
  before a ticket: what Readings depend on it is unmeasured.
- **FloatedEachSubjectGap `:1268`** lacks SubjectGap's `Exclude Irrealis`.
  This is drift.
- **Remaining line (a) names** in `compile.rs`: AdjectivalPassive, VerbKind,
  AdjectivePosition, Voice, ComplementKind. These are covered by
  `english-v4-engine-feature-names`.

## Done when

- Every finding above is classified and has a proposed fix. The owner has the
  inventory.
- The audit changes no code, except the owner-added deletion of
  `postdependent` (`smnnwupo`). No face is lost, every judgment holds,
  `cargo xtask english-v4 strata --check` passes, the cheapest tier is
  unchanged, and no `Summary`, `State` or `ProductionDecl` field or
  `closure.rs` pass is added.

Standard constraints apply.

Census delta (measured on `<change>` → this landing): faces read, total Readings and Surplus before and after; lost faces by name, or "none".
Judgment set: the result of running it.
