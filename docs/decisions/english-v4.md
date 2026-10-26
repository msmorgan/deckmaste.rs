# ADR: English v4 — Data-Driven Syntax Engine, Domain Catalogs, and Stratified Buildout

- **Status:** Accepted (2026-10-14)
- **Companion Contract:** [`docs/decisions/english-lexical-analysis.md`](./english-lexical-analysis.md)
- **Supersedes (archived in `docs/archive/decisions/`):**
  - `docs/archive/decisions/builtin-v2-macro-spelling-and-grammar.md`

---

## 1. Charter, Crate Split, and Separation of Concerns

`english-v4` is a clean-sheet rebuild of the English parser and realizer across three focused crates, built atop `deckmaste_lexical` under the engine and Reading contract in [`docs/decisions/english-lexical-analysis.md`](./english-lexical-analysis.md):

1. **`deckmaste_syntax` (Unversioned Syntax Engine Crate):**
   - Replaces the former proc-macro codegen pair (`deckmaste_construction_v3` + `deckmaste_construction_v3_core`). Because `deckmaste_syntax` is a new crate name, it requires no `_v4` suffix.
   - Loads a declarative grammar data file of **Schemas** and **Productions** at runtime (eliminating proc-macro Rust codegen and multi-hundred-variant `enum Reading` recompilations).
   - Compiles lexical subcategorization frames into derived valence productions (Active, Passive, Slash/Gap across all verb forms, and Right-Node Raising) and runs the table-driven Earley chart, packed parse forest, Reading normal-form / decomposition checks, and bidirectional surface realization.
   - Note on terminology: **Construction** is reserved for the Construction Grammar (CxG) **spelling** layer that pairs syntactic form with `semantics_v3` meaning. Purely syntactic grammar declarations use **Schema** (a high-level template that expands into productions) and **Production** (an individual grammar rule).
2. **`deckmaste_lexical_source_v4` (Core Lexicon & Catalogs Crate):**
   - Owns clean RON core English vocabulary and MTG catalogs, each with its subcategorization frames, reading only `data/gen/catalogs/card-names.txt` from `data/` and decoupling completely from `plugins_v2/builtin`.
3. **`deckmaste_english_v4` (Oracle English Grammar & Corpus Crate):**
   - Owns the Oracle English grammar data file, binds it with `deckmaste_lexical_source_v4` into `deckmaste_syntax`, and owns Oracle-English-specific logic: attachment-choosing heuristics (the structural Ranking Key, §4.3), reminder-text stripping, card-face and unit splitting, corpus census execution, and complexity-stratum profiling (moving heavy corpus analysis out of `crates/xtask` so `xtask` is a thin CLI wrapper).

The system enforces a strict three-layer boundary:

| Layer | Authority | What Belongs Here | What Is Forbidden Here |
| :--- | :--- | :--- | :--- |
| **1. Syntax Engine & English Grammar Data** (`deckmaste_syntax`, `deckmaste_english_v4`) | Pure English morphosyntax, Oracle text-box/symbolic syntax, valence compilation, and attachment heuristics | • Core CGEL syntactic categories (`Document`, `Clause`, `Predicate`, `NounPhrase`, `Nominal`, `PrepositionPhrase`, `AdjectivePhrase`, `AdverbPhrase`, `DeterminerPhrase`)<br>• Genuine morphosyntactic features (`Person`, `Number`, `Case`, `VerbForm`, `Voice`, `Countability`, `Definiteness`, `Gap`)<br>• Engine-derived valence productions from frames (Active, Passive, Slash/Gap extraction, Right-Node Raising)<br>• General adjunct/modifier attachment schemas & the non-destructive Ranking Key (§4.3) | • Any CR-cataloged word or open-class lexeme<br>• Word- or phrase-specific boolean policing flags (`AtRandom...`, `StartingWith...`, `Instead...`)<br>• MTG rules semantics (`CounterShareableVerb`, `ManaPassive...`)<br>• Guards that suppress syntactically valid attachment ambiguities |
| **2. Lexical & Frame Catalogs** (`deckmaste_lexical_source_v4`, later `plugins/`) | Lexical inventory, morphology, and complement subcategorization | • Closed-class English function words<br>• Open-class English & MTG vocabulary (`Nouns`, `Verbs`, `Adjectives`, `Prepositions`, `Adverbs`)<br>• Subcategorization frames for **complements** (`NP`, `NP + PP[into]`, `PP[for]`, `Clause[Finite]`, `ToInf`)<br>• Preposition host-category distribution (`Nominal` postmodifier, `Predicate`/`Clause` adjunct, or complement-only)<br>• CR-enumerated domain catalogs | • Corpus-attestation filters that withhold grammatical inflections<br>• Verb- or noun-specific flags restricting which open-class heads may host general adjuncts |
| **3. CxG Spelling & Semantic Checker** (`deckmaste_semantics_v3`, `lean/Semantics/`) | Form–meaning constructions, game-rules meaning, anaphora, and disambiguation | • Consuming the **packed parse forest** (never blindly enumerating all $C_n$ attachment trees)<br>• Pairing syntactic patterns with semantic meaning (Constructions)<br>• Resolving discourse referents and attachment ambiguities | • Re-parsing surface strings |

---

## 2. Hard Catalog Constraint: No CR-Enumerated Vocabulary in the Grammar

**No word or phrase enumerated by the Comprehensive Rules as a catalog/registry item may appear directly in the grammar data file (`deckmaste_english_v4/grammar/`).**

This prohibition is absolute and enforced by `deckmaste_syntax` at grammar load time:

1. **Keyword Actions (`[CR#701.1]`):** Every Vintage-legal keyword action—including common English verbs that Section 701 defines as keyword actions (`destroy`, `exile`, `sacrifice`, `tap`, `untap`, `counter`, `attach`, `unattach`, `create`, `reveal`, `search`, `shuffle`, `regenerate`, `fight`, `scry`, `mill`, `goad`, `proliferate`, `populate`, `investigate`, `explore`, `connive`, `amass`, `surveil`, `adapt`, `bolster`, `detain`, `exert`, `fateseal`, `forage`, `incubate`, `learn`, `manifest`, `monstrosity`, `suspect`, `support`, `time travel`, `venture`, `vote`, `waterbend`, `airbend`, `earthbend`, `firebend`, etc.)—is strictly a catalog entry in `keyword_actions.ron`. The grammar never names a keyword action or defines a keyword-action-specific feature, schema, or production; it parses them solely through their lexical category (`Verb` or fixed clause) and declared subcategorization frames.
2. **Keyword Abilities (`[CR#702.1]`):** All keyword abilities live in `keyword_abilities.ron` and are parsed via general keyword-line and keyword-parameter schemas driven by their catalog-declared parameter frames (cost, amount, quality, dash cost, bound stem class).
3. **Ability Words (`[CR#207.2c]`):** All ability words live in `ability_words.ron`. In the lexical layer, a registered ability word or keyword is never simultaneously emitted as an open flavor-word label.
4. **Card Types, Supertypes, and Subtypes (`[CR#205.2a,205.3,205.4a]`):** Card types (`artifact`, `battle`, `creature`, `enchantment`, `instant`, `kindred`, `land`, `planeswalker`, `sorcery`), supertypes (`basic`, `legendary`, `ongoing`, `snow`, `world`), and all subtype families (artifact, battle, creature/kindred, enchantment, land, planeswalker, spell) live exclusively in `types.ron` and `subtypes.ron`.
5. **Counters (`[CR#122.1]`), Turn Parts (`[CR#500.1]`), Designations, and Names (`[CR#201.5,201.5c]`):** Named counter kinds (`counters.ron`), phases and steps (`turn_parts.ron`), designations (`designations.ron`), predefined tokens (`tokens.ron`), and card names (`data/gen/catalogs/card-names.txt` plus shortened self-reference nicknames) enter exclusively through lexical catalogs.

Only closed-class English grammatical function words (articles, demonstratives, core quantifiers, personal/relative/reflexive pronouns, auxiliary/modal verbs, coordinators, core prepositions, and comparative markers) may be bound as `closed_word` terminals in the grammar data file.

Consequently, a custom set designer can introduce a new keyword action, keyword ability, subtype, counter kind, or verb phrasing solely by adding a catalog entry with its subcategorization frame—without touching the grammar data file or recompiling Rust code.

---

## 3. Syntactic Scope: Oracle Text Box, Symbolic, and Punctuation Conventions

While decoupled from MTG rules semantics, the English grammar data file directly models the **syntactic conventions** of Oracle card text:

1. **Document and Line Structure:**
   - A card face's rules text is a sequence of paragraph lines (`Document`), each parsed as an independent ability or instruction unit (with parenthetical reminder text stripped or checked separately by `deckmaste_english_v4`).
   - **Activated Abilities (`[CR#602.1]`):** `<Cost> : <Effect>` (plus optional trailing activation instructions), where `<Cost>` is a coordinated or comma-separated sequence of mana/tap/untap/energy symbols, loyalty cost brackets (`[+N]`, `[-N]`, `[0]`), or imperative cost clauses.
   - **Triggered Abilities (`[CR#603.1]`):** Clauses introduced by `When`, `Whenever`, or `At`.
   - **Labelled Abilities (`[CR#207.2c]`):** `<AbilityWord | FlavorWord> — <Ability>`.
   - **Modal Headers and Mode Lists (`[CR#700.2]`):** `<ModalHeader> —` followed by bulleted `• <Mode>` lines (including costed modes `• <Cost> — <Mode>`).
   - **Structured Headers:** Level headers (`Level N–M`, `Level N+`), Saga chapter headers (`I, II —`), and Class level headers.
2. **Symbols and Numerals:**
   - Braced symbols (`{W}`, `{2/B}`, `{W/P}`, `{X}`, `{T}`, `{Q}`, `{E}`, `{P}`) in cost, mana-production, and inline nominal positions.
   - Slash measures (`+2/+2`, `-X/-X`, `2/2`), die notations (`d6`, `d20`), Arabic numerals (`1`, `2`, `3`), spelled cardinals (`one`, `two`, `three`), ordinals (`first`, `second`, `third`), multipliers (`once`, `twice`, `double`, `triple`), and fractions (`half`).
3. **Punctuation and Oxford-Comma Coordination:**
   - Binary coordination uses `A and B`, `A or B`, `A and/or B`, or correlative `both A and B`, `either A or B`, `neither A nor B`.
   - Coordinations of 3 or more members in Oracle text strictly use the **Oxford comma** (`A, B, and C` or `A, B, or C`), plus comma-coordinated cost sequences and clause sequences where licensed by punctuation rules.

---

## 4. Subcategorization Frames vs. Free Adjuncts in the Controlled Oracle Register

Oracle text is a **controlled technical register** (`docs/oracle-style-guide.md`), not unrestricted literary English. While *The Cambridge Grammar of the English Language* (CGEL) provides our standard terminology and constituent structure (`Complement` vs. `Adjunct`/`Modifier`, `Clause`, `Predicate`, `NounPhrase`, `Nominal`, `PrepositionPhrase`), **the controlled Oracle register—not open-domain literary English—governs which unselected attachment patterns exist**:

### 4.1 Complements Are Driven by Lexical Frames (Proven First via Prototype)
A **Complement** is a dependent licensed by the head's lexical subcategorization frame:
- **Unified Frame Engine (First Deliverable):** Before scaling out the catalogs or strata, `deckmaste_syntax` and `deckmaste_english_v4` prove the unified frame compiler end-to-end on a ~15-word seed lexicon across `Verb`, `Preposition`, `Noun`, `Adjective`, and `Determinative`, with correctness and chart-size measurements.
- **Prepositions from Frames:** Instead of 30 bespoke `PrepositionPhrase` productions and 25 licence tables, a `PrepositionPhrase` is built from a `Preposition` head plus the complement selected by its frame in `prepositions.ron` (`NP`, `Clause[Finite]`, `Clause[Verbless]`, `PP[with]`, bare location `Nominal`, or `Intransitive`).
- **Engine-Derived Valence Alternations:** Verb complement frames are declared **once** (in active voice) in `verbs.ron` / `keyword_actions.ron`. From those active frames and the valence schemas in the grammar data file, `deckmaste_syntax` automatically derives productions for:
  1. **Active** complementation (`put [the card] [into your hand]`).
  2. **Passive** complementation (`[the card] was put [into your hand]`, `each card put [into your hand]`, plus optional `by <NP>` internalised complement).
  3. **Slashed / Gapped** complementation across **all** verb forms—finite, plain/bare, progressive (`be + V-ing`), perfect (`have + V-en`), and passive (`be + V-en`)—eliminating manual cross-product gap categories (`"the card you put / are putting / have put _ into your hand"`).
  4. **Right-Node Raising / Shared Complements** across coordinated heads (`enters or leaves [the battlefield]`, `put into your hand or exile [_]`).

### 4.2 Adjuncts, Preposition Frame Host Categories, and Packed-Forest Consumption
An **Adjunct** (on a clause/VP) or **Modifier** (on a nominal/NP) is not subcategorized by a specific open-class verb or noun lexeme:
- **Per-frame host categories:** Each `(preposition, complement_frame)` entry in `prepositions.ron` declares which **syntactic host categories** that PP may modify when unselected (`Nominal` postmodifier, `Predicate`/`Clause` adjunct, both, or `ComplementOnly`):
  - Goal prepositions (`into`, `onto`) are `ComplementOnly`.
  - Unselected `for + NP` is a `Predicate`/`Clause` adjunct (distributive `for each ...`, duration `for the rest of the turn`), not a `Nominal` postmodifier. A noun that takes a `for`-complement (`cost for ...`, `substitute for ...`) declares `PP[for]` in its own **noun subcategorization frame** in `nouns.ron`.
  - `with + KeywordPhrase` (`with flying`) and `with + QuotedAbility` have `Nominal` postmodifier as host category, whereas `with + NP` (`with a +1/+1 counter on it`) can modify both `Nominal` and `Predicate`. A verb may still select a `with`-PP as a complement through its own frame: *escapes with a +1/+1 counter on it* and *escapes with "..."* (Skyway Robber) take the `with`-PP as a selected Complement of *escape* (`Prepositional(["with"])` in the catalog), so a quoted ability is not only a nominal postmodifier.
- **Same-head complement priority versus cross-head attachment:**
  - On the **same head**, a frame-selected PP complement (e.g., `for a card` selected by `search`'s `NP + PP[for]` frame in `"Search your library for a card"`) pre-empts attaching that same PP to that same verb instance as an unselected adjunct.
  - Across **different heads** along the right spine, when a PP or clause can syntactically attach at more than one site, the parser **retains all syntactically grammatical attachments** in the packed forest (`english-lexical-analysis.md` §2), ranked by the Ranking Key (§4.3) and disambiguated downstream by `deckmaste_semantics_v3`. In **Seedborn Muse** (`"Untap all permanents you control during each other player's untap step."`), `during each other player's untap step` can modify either the matrix imperative `Untap` or the embedded relative-clause verb `control`; both attachments are retained.
- **Packed-forest consumption downstream:** Neither the census gate nor downstream semantic translation (`deckmaste_semantics_v3`) may blindly enumerate all $C_n$ attachment trees of a multi-PP chain. Admission and deferred validation run over the packed forest (or the bounded best Readings by Ranking Key for display).

### 4.3 The Ranking Key

The **Ranking Key** is the presentation and traversal order of the Readings retained in the packed forest: it decides which Readings a report or a consumer sees first, and which ones the bounded best-*k* extraction visits. It is a heuristic, not a correctness gate (§4.4, principle 4): where it ranks the endorsed Reading is reported, never required. It is a lexicographic tuple compared component by component, lower first. It replaces the former scalar sum of `Production Cost`, which left most attachment alternatives tied (Mockingbird's 36,709 Readings had 6,354 tied at the minimum) and ranked every Predicate-hosted PP Adjunct behind every Nominal-hosted one. The Ranking Key is purely structural. The components, in order:

1. **AttachmentClass**: the number of attachments that violate a structural attachment class, keyed on host kind, dependent function and existing grammatical features, never on words:
   - a comma-set Supplement (`host_kind: Supplement`, e.g. *, where X is …*) anchors to the outermost eligible clause (CGEL, Ch. 15 §5.1, p. 1351), and so does a distributive adjunct, whose complement has a `Distributive` determinative (*for each …*);
   - a dependent following a coordination of nominals or predicates attaches to the whole coordination rather than to its last conjunct (`docs/oracle-style-guide.md` §6, Determiners and modifier order: "Put ownership and control after the full noun phrase");
   - of two dependents preceding their head, the second attaches inside the first where the first can hold it, rather than to the head beside it: a temporal bound belongs to the trigger, header or relative clause before it (*At the beginning of each combat this turn, …*; *For each creature that entered this turn, …*; `docs/oracle-style-guide.md` §11: "Whenever you cast your second spell each turn" in This, next, and each, and "the next time [event] this turn" in Continuous and bounded durations), and a locative to the phrase before it (*at the beginning of combat on your turn*, §11, Turn-structure names). The second stays a Reading beside it, and the only one where the first cannot hold it (*For one spell this turn, …*).
2. **RightAssociationWords**: for each unselected dependent, the length in words of the host constituent it attaches to (word distance from the host's head to the dependent), computed by the engine from spans for every adjunct and modifier schema. A dependent that attaches high by its class counts it reversed; a coordination host is not charged; coordinated conjuncts add their difference in length (conjunct balance).
3. **ProductionCost**: the sum of the `Production Cost` of the Reading's Productions, as declared in the grammar data file. A Nominal-hosted and a Predicate-hosted PP Adjunct have the same cost: no Production Cost encodes a blanket preference between those hosts. A Production's `scoped` lexical child costs one more where analysis did not admit its leaf through the scope of the face whose text is parsed (the face's own name, or a card-scoped spelling such as its Shortened Name [CR#201.5c]): another card's full name ranks after the face's own name spelled alike. That is a property of the parse context, never of a word.
4. **FinalTiebreak**: a deterministic total order on Readings, so that the minimum is a single Reading.

**Lexical and domain plausibility are not part of ranking.** No component is declared on lexemes, frames or catalog entries, and no component prefers a host because of what a word means: not that *cast* takes an origin, not that *spend* takes a purpose, and not that a zone noun dislikes postmodifiers. Such preferences are semantic plausibility judgments made one word at a time. They are policing by another name: they re-enter the grammar through the lexicon what §4.2 bans as boolean features. They also bought little. In the cost study, the structural key alone made 217 of the 308 single-group judgment entries all-endorsed at the top, against 92 under the scalar sum; adding such catalog preferences gained 6 more. Where a preference seemed needed, the cause was a lexicon declaration that was too broad, such as a preposition frame's host categories, and the fix is to correct that declaration. Disambiguation by meaning belongs to `deckmaste_semantics_v3`.

Every component is a sum over the Reading's attachments or Productions, and lexicographic order is preserved under componentwise addition. So the cheapest *k* Readings are still extracted by dynamic programming over the packed forest, compositionally and without enumerating trees. The Ranking Key is a Preference: it never decides Admission and never removes a Reading, and no Reading is right or wrong because of where it ranks. Spurious analyses are removed by grammar fixes that tighten or delete the rule that admits them, never by the Ranking Key, which assumes they are gone. A spurious analysis is one no English grammar admits (§4.4, principle 1).

### 4.4 Principles

1. **Admit English.** The grammar admits what English grammar admits, in Oracle register ([`docs/oracle-style-guide.md`](../oracle-style-guide.md)), and rejects only what no English grammar admits. A Reading that is grammatical but wrong for the rules is semantics' to reject (`deckmaste_semantics_v3`), not the grammar's. A Corpus Unit whose Readings include the endorsed one is read, whatever else is admitted beside it.
2. **One tree per analysis.** The structural canonicalizations of [`english-lexical-analysis.md`](./english-lexical-analysis.md) §2 apply. Two trees for the same analysis is a defect.
3. **Structure over suppression.** A missing or wrong analysis is fixed by a general construction that cites CGEL, OSG or the CR, adding the features, values, Categories, Productions or engine capabilities that construction needs, in any Stratum. It is not fixed by an exclusion aimed at one witness.
4. **Ranking is traversal order.** The Ranking Key (§4.3) orders the forest for presentation and bounded traversal and accepts or rejects nothing. A judgment asserts grammaticality only: a positive entry, that its endorsed structure is admitted; a negative entry, that a genuinely ungrammatical form is not. No judgment asserts a rank. Cheapest-tier figures and judgment tallies are reported, never gated, and no ranking work is done to move a Reading up.
5. **Measure.** Every change to the grammar, its compiler, its lexicon or the census is measured by the census before and after: no face is lost, and the units that change are reported.
6. **Boundaries.** `deckmaste_syntax` never names a grammar feature, value or Category, except in the lexicon feature-bundle mapping table, the one declared lexicon-to-syntax interface; purely structural `Summary` bits computed from the engine's own host check, coordinators, extensions and punctuation are allowed. Vocabulary lives in the catalogs and lexicon, and no word the CR enumerates appears in a grammar declaration (§2).

**Scope.** Every card not on the Reserved List must parse. A Reserved List card may be declined when its construction is unique to it and costly to support. Declined cards are listed in `crates/deckmaste_english_v4/declined.txt`, each with its reason, and the census reports them separately from unread. A declined card is scope, not an accepted wrong reading.

### 4.5 Gates in the grammar

An admission gate (an `Exclude` or `Require`, a Category that restates Productions) is justified only when it enforces a form English, or the Oracle register, never builds. A form that is merely dispreferred is ranked (§4.3), not gated. A gate whose removal would explode forests on real cards also stays, with the measurement that shows it. This is an index of the gates the grammar has now, not a closed list: adding or retiring a gate follows the principles above and needs no ruling.

- **AttributeChain and its `Polarity: Negative` requirement**: an asyndetic chain of premodifiers holds only independent *non-* premodifiers, set off by commas; any other comma before a premodifier is a serial coordination's (`docs/oracle-style-guide.md` §6, "Determiners and modifier order"; §3, "Commas and the serial comma"). AsyndeticAttributes, AttributeSequence and FinalAttribute require each Attribute in the chain to carry `Polarity: Negative`, as the Adjunct of a later premodifier conjunct requires its preposed premodifier to (*artifact or non-Aura enchantment*); the prefixed premodifier Productions (PrefixedNounPremodifier, PrefixedParticiple and NegativeAdjective) value it.
- **RelativeTemporalNominal**, with RelativeTemporalLocation, RelativeTime and RelativeTimePostmodifier: a Noun Phrase is a temporal Adjunct only in restricted forms (CGEL, Ch. 8 §6.3, p. 698), and a temporal nominal in one takes a *when*-less adverbial relative clause, then at most a temporal bound (`docs/oracle-style-guide.md` §11, "Continuous and bounded durations"; "This, next, and each").
- **Possessor and PossessorNominals**, with their Productions and the PossessorNominals Coordination: Oracle never postmodifies a possessor; it fronts the player noun "to avoid a dangling possessive" (`docs/oracle-style-guide.md` §6, "Controller and owner"), though English has the phrasal genitive (CGEL, Ch. 5 §16.6, p. 479). Without it the forest grows by 8,926 Readings on 779 units, the largest unit ×3.8.
- **The FreeChoice Excludes on Determined and DeterminedModifiedQuantity**: Determined's routes *any* to the Productions that carry number transparency (CGEL, Ch. 5 §3.3, pp. 349–351), which keeps out *any number of target creatures becomes* (`number_transparency.txt`); with DeterminedModifiedQuantity's, no analysis gets a second tree (principle 2). Their value, `DeterminativeDistribution` `FreeChoice`, has the one holder *any*, which is the whole class, as `NounDistribution` `Serial` is *time*.
- **TemporalLocation's IndefiniteArticle Exclude**: a period noun forms a temporal location Noun Phrase with *another* (*another day*), never with the indefinite article (*\*a day*; CGEL, Ch. 8 §6.3, p. 698). Its value, `DeterminativeDistribution` `IndefiniteArticle`, has the one holder *a*, the whole of its class (CGEL, Ch. 5 §3.4, p. 353, set (a) [67]); `SingularOrQuantified` likewise has the one holder *another* (set (c) [69–70]).
- **SubjectInitialClause**, with RelativeSubjectPredicate, RelativeCliticSubjectPredicate, RelativeFloatedEach, RelativeFloatedAll and RelativeAdjunctGap: a bare relative clause always has its Subject in initial position, so no preposed adjunct comes before it ("Bare relatives always have the subject in initial position"; CGEL, Ch. 12 §3.5.5, p. 1055). The Category restates the Clause Productions that open with the Subject and omits those that front an adjunct; no `ProductionDecl` field carries the rule.
- **The alternative Noun Phrase Coordination's `Exclude("first.NounKind","Numeric")`**: a conjunct must fill the coordination's function alone (CGEL, Ch. 15 §3.1, p. 1323, rule [1]), and a number fills no Noun Phrase Complement slot, so never *is dealt [3] or [more damage]*. An `ExcludeTogether` on both conjuncts instead admits 175 more Readings on 95 units, every one that wrong analysis.
- **The depictives' `Amount` Excludes** (AdjectivalDepictive, DepictiveAdjunct, DepictiveAdjective): a number bounded by a comparative (*10 or greater*, ComparativeBoundedAlternative) is no depictive, since a number is none (*\*She left 10*) and a coordination fills only a function its conjuncts could each fill (CGEL, Ch. 15 §3.1, pp. 1323–1324, rule [1]). A comparative alone is a depictive (*She left richer than she arrived*). Without them the forest grows by 16,477 Readings, Mayael's Aria to 4,608 and Whiskervale Forerunner to 4,750.
- **FiniteComplementClause's `unlinked` head**: a sole connective links instructions and main clauses or Verb Phrases, never subordinate clauses (CGEL, Ch. 15 §2.10, p. 1320, on *so*; `docs/oracle-style-guide.md` §10, Sequence: "`Then` orders instructions"), so a Clause Complement of a preposition is not, nor is headed by, a Linked Coordination: *until [they exile a nonland card], then you may cast it*, never *until [they exile a nonland card, then you may cast it]*. Without it the forest grows by 279 Readings on 12 units, every one that analysis, and two units (Collective Voyage, Kaboom!) read only through it.
- **The `Numeric` Excludes on NonNumericComplement and NounPhraseArithmetic**: a number is an Amount, so a Numeric Noun Phrase over it would build a second tree of the same analysis ([`english-lexical-analysis.md`](./english-lexical-analysis.md) §2). Without them *equal to 2*, *Scry 2* and *X plus 1* double.
- **FixedFrameBareNoun's `FixedFrame` Require**: a singular count noun stands bare only in a preposition's Fixed Frame that admits it, and the nouns that do "are severely restricted" (CGEL, Ch. 5 §8.5(b), pp. 409–410): *from outside the game*, never the noun bare as a Predicative (*if it's outside the game* has the preposition). The noun's lexical `NounDistribution` `FixedFrame` is the restriction, and *from*'s `FixedFrameNP` frame the frame. Without it, as a bare mass noun, *if it's outside the game* reads twice.
- **Valence `interleaved_internalised`**: a manner adjunct is admitted only before a passive's Internalised Complement (*chosen this way by any player*), never elsewhere between a verb and its complements. Oracle register has it in that slot (`docs/oracle-style-guide.md` OSG §5, "This way"), and English allows an adjunct before a complement there (CGEL, p. 241). Added by change `oslntzlv`.
- **Heavy Object shift** (Valence `heavy_shift` with `shifted_object`, and the engine's `Postposable` weight): an Object postposed past a predicative Complement, or past an interleaved Adjunct alone, is heavy relative to what it crosses (CGEL, Ch. 16 §4, pp. 1382–1384, [7]): it ends in a dependent or is a coordination, and it carries `Determination: Overt`, as every Object Oracle postposes does (*chooses from among the permanents they control [an artifact, a creature, an enchantment, and a land]*; *turns face up [all cards they own exiled with this artifact]*). With the weight condition alone the forest grows by 1,057 Readings on 413 units, most of them a bare nominal split from the determinative or premodifier before it (*enters [with two] [+1/+1 counters on it]*); with both, by 144 on 41. Past selected Preposition Phrase Complements alone the shift stays ungated (*Put into your hand the card*). Added by change `spkrozto`.
- **`elliptical` ellipsis sites** (`Summary::ellipsis`): before any punctuation, a phrase ending in an ellipsis site is followed only by a coordinator or by the head it is a dependent of, and not where a phrase right after the site could be the omitted Complement, which it then is (CGEL, Ch. 17 §7.1, p. 1519).
- **Open comma-set post-dependents** (`Summary::open_comma`): a phrase that ends in a comma-set post-dependent is followed by a mark, which closes the comma, or ends its Production, since a comma that sets off a dependent is closed by the next mark or runs to the end of its sentence (CGEL, Ch. 20 §3.2.5, p. 1745 [25]): *from black if [you control a Swamp, from red] if …* is no Cluster. Without it Dominaria's Judgment has 88 Readings, not 32.
- **`unclosed` dash phrases**: a phrase opened by a single dash that only its sentence's end closes is followed by nothing but a period (CGEL, Ch. 20 §5, p. 1751).
- **Fronted-dependent scope** (`Summary::fronted`, `Fronting`): a fronted dependent scopes over a comma-set post-dependent of its Clause and, unless it is a Predicate Adjunct preposed without a comma, over a whole coordination it would begin, never its initial conjunct ([`english-lexical-analysis.md`](./english-lexical-analysis.md) §2; CGEL, Ch. 15 §1.3.4, pp. 1285–1286).
- **Complement Cluster shape**: Clusters coordinate only with Clusters of one frame shape (CGEL, Ch. 15 §4.3, p. 1341, [17ii]: *blame X for Y* and *blame Y on X* do not coordinate).
- **Adjunct Cluster parallelism** (Coordination `parallel`): Clusters of Adjuncts alone coordinate only with Clusters of the initial one's prepositions, its first overt or elided in a remnant (CGEL, Ch. 15 §4.3, p. 1341, the structural parallelism requirement; p. 1343, note 65). Without it the forest grows by 10 Readings on 3 units, each a non-parallel Cluster (*[a +1/+1 counter] [on it]* beside *[with a +1/+1 counter on it] [for each …]*). Added by change `swknnvqx`.
- **`uncoordinated` and `coordinated` children**: an `uncoordinated` child is no coordination where a coordination one level up is the same analysis (principle 2); a `coordinated` child is one, for a dependent that scopes over a whole coordination where a single conjunct has a site of its own (CGEL, Ch. 15 §1.3.4, pp. 1285–1286).
- **Flavor Word title casing** (`deckmaste_lexical` analysis): a label before an em dash is a Flavor Word only when every word of it that begins in lowercase is a function word (`docs/oracle-style-guide.md` §14, "Ability words and flavor words": "natural title casing"), so a clause before a dash is none (*Each opponent faces a villainous choice — …*).
- **A sentence's one unwritten period** (`Summary::unwritten_period`): a quotation's final period is moved outside its closing mark (`chart::quotations`), a sentence's period after a word whose spelling ends in terminal punctuation is not written (*Exile Blood for the Blood God!*, *Agents of S.H.I.E.L.D.*), and no two Constituents both end in the unwritten one. A keyword's verbal cost ends in its own period (`docs/oracle-style-guide.md` §3, "Periods": *Ward—Pay 3 life.*), so without it *have "Ward—Pay 2 life."* reads as *Ward—Pay 2 life..*, and six faces stop round-tripping.

---

## 5. Self-Contained Core Lexicon and Catalogs (`deckmaste_lexical_source_v4`)

To unblock the retirement of `plugins_v2/builtin` and avoid `deckmaste_lexical_source`'s 4-layer salvage pipeline, `deckmaste_lexical_source_v4` owns clean, uniform RON files. Core English vocabulary is not a Catalog: it is declared with Core provenance, which is what lets the grammar name its closed-class words; Catalog entries carry Catalog provenance and never appear in the grammar.

- **Core English Lexicon (`lexicon/english/`):**
  - `closed_class.ron` (pronouns, determinatives, coordinators, auxiliaries/modals, clitics, symbols)
  - `prepositions.ron` (prepositions, their complement frames, and their per-frame unselected host-category distribution in Oracle English)
  - `verbs.ron` (non-keyword English verbs and their active complement frames)
  - `nouns.ron` (common nouns, countability, and optional complement frames)
  - `adjectives.ron` (adjectives and optional complement frames)
  - `adverbs.ron` (adverbs and clause-position class)
- **MTG Catalogs (`catalogs/`):**
  - `types.ron` (`[CR#205.2a,205.4a]` card types and supertypes)
  - `subtypes.ron` (`[CR#205.3]` artifact, battle, creature/kindred, enchantment, land, planeswalker, and spell subtypes)
  - `keyword_actions.ron` (`[CR#701.1]` keyword-action verbs/clauses and their complement frames)
  - `keyword_abilities.ron` (`[CR#702.1]` keyword abilities and their parameter frames)
  - `ability_words.ron` (`[CR#207.2c]`, including Vintage-legal printed ability words omitted by the CR list such as `Corrupted`)
  - `counters.ron` (`[CR#122.1]`), `designations.ron`, `turn_parts.ron` (`[CR#500.1]`), `tokens.ron`
- **External Data (`data/`):**
  - Reads **only** `data/gen/catalogs/card-names.txt` (and shortened self-reference nicknames `[CR#201.5,201.5c]`).
  - Zero dependency on `plugins_v2/builtin`, `deckmaste_macro_def`, or other `data/gen/catalogs/*.txt` files at runtime.
- **Full Morphological Paradigms (Attestation Is Provenance, Never a Filter):**
  - Every declared verb and noun receives its full grammatical inflectional paradigm (present 3sg, preterite, gerund-participle, past participle, plain; singular, plural) unless the word is genuinely defective in English. Absence of an inflected form in the current printed card corpus is never grounds to mark an inflection `Unavailable`.
- **Future `plugins/` Migration:**
  - Once `english_v4` and `semantics-v1-cutover` are complete, the `catalogs/*.ron` declarations move 1-to-1 into the metadata of the corresponding `plugins/` (`v3`) DSL declarations.

---

## 6. Stratified Complexity Roadmap, Verification Gates, and Size Signal

### 6.1 The 8 Complexity Strata (Strata 0–7) and Composition Rule
Rather than reactive blocker recon across 32,828 cards, the entire Vintage-legal Scryfall Oracle corpus (both the 93.8% currently parsed by `v3` and the 6.2% unparsed residual) is partitioned up front into **8 cumulative Complexity Strata** by abstracting open-class catalog items into frequency-ranked **Syntactic Skeletons** inside `deckmaste_english_v4`.

**Max-Stratum Composition Rule:** Every complete corpus unit (line/ability) is assigned to the **highest stratum required by any constituent inside it** ($\max_i \text{stratum}(c_i)$). When an outer spine production (such as Stratum 0's `Cost : Effect` or `<Label> — <Ability>`) lands before higher clausal strata exist, its ticket tests the spine over Stratum 0/seed payloads; corpus units combining that spine with a Stratum $k$ clause belong to Stratum $k$.

- **Stratum 0 — Card Frame, Headers & Non-Clausal Lines:** Type lines, standalone keyword-ability lists & non-clausal parameters, ability/flavor-word prefixes, modal headers & bullets, level/saga/class headers, and `Cost : Effect` spines (tested over symbol/seed payloads in Stratum 0; clausal costs/effects unlock at their clause stratum).
- **Stratum 1 — Core Imperative/Declarative Clauses & NP/PP Structure:** Layered `Nominal` and `NounPhrase`, frame-driven `PrepositionPhrase`, single-clause active verbs (including `[CR#701.1]` keyword actions), and simple adjuncts.
- **Stratum 2 — Auxiliaries, Copulas, Passives & Non-Finite Complements:** Modals/auxiliaries, negation, predicative complements (`gets +2/+2`, `is a ...`), engine-derived passives, and infinitival/participial catenatives.
- **Stratum 3 — Relative Clauses & Frame-Derived Gaps:** Subject relatives, reduced participial postmodifiers, and engine-derived object-gap and stranded-preposition relatives across all verb forms, plus adverbial relatives (`the next time ...`).
- **Stratum 4 — Subordinate Clauses, Triggers & Conditionals:** `When`/`Whenever`/`At` triggers, clausal prepositions (`if`, `unless`, `while`, `until`, `as long as`, `as`, `except`, `rather than`), verbless clauses (`if able`), content clauses (`that ...`), and `where X is ...`.
- **Stratum 5 — Comparatives, Measures & Partitives:** Partitives (`all of`, `one of`, `any number of`), scalar comparisons (`equal to`, `greater than`, `at least`, `up to`), and arithmetic modifiers (`plus`, `minus`, `twice`, `rounded up/down`).
- **Stratum 6 — Coordination & Shared-Complement Ellipsis (RNR):** Binary and Oxford-comma serial coordination across categories, correlatives, and engine-derived Right-Node Raising.
- **Stratum 7 — Discourse Sequencing, Replacement & Long Tail:** Replacement effects (`instead`), discourse anaphoric clauses (`When you do`, `Then ...`, `Otherwise`, `Repeat this process`), `with`-supplements, quoted ability grants, and the remaining corpus tail.

**Stratum 7 Long-Tail Rule:** If a long-tail card's wording cannot be parsed by a clean general schema/production or a catalog-declared lexical idiom without introducing a policing flag (`...Use`, `...Licence`, `...Present`), the implementer **must STOP and surface the witness to the user** to decide between a catalog-level idiom, a clean grammatical production, or an explicit documented deferral (`unread`). Tail outliers may stay explicitly deferred in `v4` rather than polluting the grammar with policing flags or blocking cutover.

### 6.2 Verification Gates and Design Health Signals
1. **Cumulative Stratum Coverage & Both Round-Trip Laws (`english-lexical-analysis.md` §§1–2):**
   - Completing Stratum $k$ requires covering 100% of non-deferred corpus units assigned to Strata $0..=k$, with the census reporting **read**, **known-wrong-only**, and **unread** separately (a unit whose Readings are all known-wrong is never counted as covered).
   - Both round-trip laws from [`docs/decisions/english-lexical-analysis.md`](./english-lexical-analysis.md) §1 must hold (`v is an admitted Reading of s => render(v) = s` and `v is independently constructed and grammatical => v is a Reading of parse(render(v))`, preserving lexical identity, correlated alternatives, and deferred validation).
2. **Curated Linguistic Judgment Suite (`english-lexical-analysis.md` §5):**
   - Hand-written, one line per entry, no tree dumps.
   - Asserts constituent category, surface span, head identity, grammatical function (`Subject`, `Object`, `Complement`, `Adjunct`, `Modifier`), gap-filler binding, negative controls of inflectional-form selection (agreement, case and verb form, which generation needs; never a word combination no card contains), and documented attachment-ambiguity witnesses—without freezing internal `Reading` constructor names or form indices.
   - Adjudicated against the controlled Oracle English register (`docs/oracle-style-guide.md` and this ADR, using CGEL for standard terminology and constituent structure) by two independent judges, with disagreements surfaced to the owner.
3. **Size as a Review Signal, Not a Mechanical Target (`english-lexical-analysis.md` §6):**
   - Around ~2,500 lines in the English grammar data file (~40 features, ~60 categories) at full coverage is the expected scale and serves as a review smell test across the grammar data file, `deckmaste_syntax`, and `deckmaste_lexical_source_v4`—never a mechanical line cap that pushes hacks into the syntax engine or catalogs.

---

## 7. Two-Stage Cutover and Crate Fates

1. **Immediate Glossary Pruning:** [`docs/contexts/oracle-english/CONTEXT.md`](../contexts/oracle-english/CONTEXT.md) is pruned immediately of all `english_v3` wave 5–9 policing features (`...Use`, `...Licence`, `...Present`, `...Restriction`) and implementation-detail rationalizations, retaining clean CGEL and Oracle-English domain terminology (`Schema`, `Production`, `Production Cost`, `Ranking Key`, `Admission`, `Packing`, `Derivation`, etc.) and updated inline as `v4` lands.
2. **Side-by-Side Buildout (Prototype First):** First build `deckmaste_syntax` and `deckmaste_english_v4` and prove the unified frame engine on the seed lexicon (`english-v4-compiler-and-runtime`), then populate `deckmaste_lexical_source_v4` (`english-v4-lexical-catalogs`), stratify the corpus and migrate `v3`'s genuine behavioral test obligations into the `v4` judgment set (`english-v4-corpus-stratification-and-judgment-harness`), and execute Strata 0–7.
3. **Cutover 1 (`english-v4-cutover-v3`):** Once Stratum 7 completes and passes the judgment suite:
   - Repoint `xtask` and workspace consumers at `deckmaste_english_v4`.
   - Delete `deckmaste_english_v3` (recording the migrated obligations and named retirement reasons for the wave test suite), `deckmaste_construction_v3*`, and `deckmaste_lexical_source`. `plugins_v2/builtin/` and its reader `deckmaste_macro_def` are not deleted here: they stay until `semantics-v3-dsl-and-plugins`, because `xtask` still reads them.
4. **Cutover 2 (`english-v4-final-suffix-cutover`):** After `semantics-v1-cutover` retires the legacy v1 consumers and renames `plugins_v3/` → `plugins/`:
   - Delete legacy `deckmaste_features`. (Legacy suffixless `deckmaste_english`, `deckmaste_construction_compiler` and `deckmaste_constructions_macro` were deleted earlier by `retire-v1-english-and-spelling`.)
   - Drop all `_v4` suffixes (`deckmaste_english_v4` → `deckmaste_english`, `deckmaste_lexical_source_v4` → `deckmaste_lexical_source`; `deckmaste_syntax` already has its final unversioned name).
   - Move the `catalogs/*.ron` lexical declarations into `plugins/` declaration metadata.
