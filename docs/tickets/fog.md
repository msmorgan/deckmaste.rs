# Fog

Work that is **in scope but not yet sharp enough to be a ticket**. A fog entry
records the substance — the family, the shape it should take, the identities and
probes it owns, and the open decision it waits on — so that nothing is lost while
the shape is still unsettled.

This file is **not a graph source**. The Kanban driver reads only the status
folders (`critical/`, `planned/`, `maybe/`, `wip/`, `done/`); like `census.md`,
nothing here is machine-read and no `needs:` may name a fog entry.

**Graduation rule.** An entry becomes a ticket only when it can carry a
`Pinned shape:` — a named method a claimant can execute without re-deciding the
design. Write the ticket, then **delete the entry**; the two never coexist. An
entry's `Hangs on:` line names the live ticket(s) whose landing is expected to
sharpen it.

## english-v2

**Routing changed 2026-09-05:** the [Lean design workbench and migration](../english-grammar-wayfinder.md)
now owns grammar design and consolidation. The entries below retain examples,
re-coverage identities, and historical measurements. Their old `Hangs on:`
lines and corpus-first graduation recipes describe the previous schedule;
the wayfinder maps their formal owners, and
`english-v2-grammar-migration-design` must assign every production obligation.
Do not mint new structural tail tickets from these counts during the design
phase. Existing linguistic rulings still apply; proposed old mechanisms must
be reconciled against the formal model.

### Gerund clauses and modal ellipsis (A9, A11)

Two nonfinite/reduced clause shapes v2 has no path for. **A11:** a gerund clause
category (`instead of putting it…`, `by replacing…`, `rather than paying…`) —
CGEL lists gerund-participials alongside infinitivals as a nonfinite clause type;
v1 had the category with few productions, and its replacement-effect corpus rows
were still recovered through it. v2 has zero gerund constructions. **A9:** a modal
with an optional inner predicate, licensing post-auxiliary VP-ellipsis (`If you
can't, …`); v1 carried it, v2 was not found to — verify with a probe before
pinning.

Re-coverage targets routed here from `english-v2-control-role-declared-valence`,
which retires them because their sole surviving reading is wrong: **Cavalier of
Thorns, Animal Magnetism, Genesis Ultimatum** — `Put X onto the battlefield and
the rest into your graveyard`, a gapped coordinated destination that is not
expressible today. They are re-covered when this entry graduates and lands.
The Cavalier of Thorns sentence is `Put a land card from among them onto the
battlefield and the rest into your graveyard.`
(3a05d72cc35c86f1b6bb8cfb74017af4c6c5b440e9b18475485ad034db4440e6).
The Animal Magnetism sentence is `Put that card onto the battlefield and the
rest into your graveyard.`
(83b1cc666255a03888d2654849cbdcd552c61d9336ce41cbe163a248c4f887ec).
The Genesis Ultimatum sentence is `Put any number of permanent cards from among
them onto the battlefield and the rest into your hand.`
(cc39ba9efc13bba3022d314e135707a35dbe17b515db2cbd7ca858a97cff00b2).

Hangs on: `english-v2-relative-clause` (and the general clause machinery under it).

### Predicative complement as one copular frame (A7)

Predicative complement is a **function filled by AdjP | NP | PP**, not a family of
game-carved subtypes: color, designation, status and face orientation are lexical
classes of adjective or noun. v2 partitions 8 of its 9 predicative-complement
subtypes by game meaning. Adopt one copular frame and delete the partition — the
frame-literal ruling's spirit applied to the copula. No ruling blocks it.

`english-v2-cost-family-lowering` names the predicative block as owned elsewhere;
that pointer is this entry until it graduates.

Hangs on: `english-v2-subordinate-clause` (the general clause and frame shape it
would be spelled against).

### Recipient / retained-object passive as frame data (A6)

The passive of `deal X damage to Y` promotes the recipient and retains the theme
(`were dealt damage`), plus its reduced form (`a creature dealt damage this way`).
That is a **valence fact of the verb** — frame data on the declared valence, on the
`Custom`/`Participle` axis — never a construction per verb. v1 carried it as a
`recipient_passive` verb property with a reduced-passive phrase; v2 fails the probe
at `dealt`. The audit expects it to become natural once `damage` is an ordinary NP.

Hangs on: `english-v2-frame-selected-prepositions` (same seam — lexical facts moved
into declared Verb Frame data).

### Arithmetic and fraction values (A10)

One general quantity grammar for arithmetic value expressions: `half their life,
rounded up`, `twice X`, `X plus Y`, `X minus Y`. v1 had it; v2 fails the probe at
`half`. No ruling and no mechanism blocks it.

Hangs on: **nothing structural.** This one is fogged only because its inventory has
never been measured — it needs a corpus census of the arithmetic surfaces and their
rounding forms to pin a shape, not another ticket's landing. It can graduate as soon
as someone does that census.

### Polarity as a derived feature (A8a)

`non-` is productive prefixation and `not`/`no` negation is a feature — both belong
on the general modifier/nominal/predicate constructions, not in per-polarity
construction families. v1 carried a `Polarity` axis and a negated-modifier lexical
slot; v2 spends 12 constructions on `non-`. Collapsing them deletes those families.
Mechanism note from the audit: v2's per-family `declaration_noun` codecs would need
one codec parameterised by family — a mechanism change, not a ruling.

Hangs on: the noun-feature work; `english-v2-number-feature-unification` is the
precedent for a feature collapse (categories duplicating a feature axis get deleted
in favour of the declared feature) and the shape this should be spelled against.

### Card type as a derived feature on nominal modifiers (A8b)

24 of v2's 34 nominal modifiers are per-card-type constructions (8 types × 3
layers); card type is a lexical subclass, so collapse them to **one modifier
construction with the type as a declared feature**. Scope fence, still binding: the
ADR-blessed 34-construction card-kind **noun** partition (`english-v2-rewrite.md`,
~:547-551) is a separate kept decision and stays out of it — this is the modifier
side only. Same `declaration_noun` codec-parameterisation note as A8a.

`english-v2-adjective-inventory` previously waited on this as a ticket; it no longer
does. 2026-09-04: its three form-literal re-routes
(`AttributiveAdjective::{Additional,Next,Other}`) are NOT handled by
`english-v2-closed-class-single-owner` (done; it did not touch them) — the
adjective ticket's own "Hard blocker this ticket owns" is accurate and owns
them.

Hangs on: the noun-feature work; `english-v2-number-feature-unification` as the
feature-collapse precedent.

### Tail families (the frontier register for `english-v2-stage-5-grammar-buildout-14-10`)

**Historical frontier; scheduling suspended until `english-v2-grammar-migration-close`.**
The umbrella remains `english-v2-stage-5-grammar-buildout-14-10`. Its resumed
loop re-measures these buckets and routes general omissions back to design.
First-failure offsets hide later missing capabilities; the table is evidence
for the whole-grammar scope review, not the design agenda.

Measured 2026-09-05 on change `ptoxwkmmrqno` (19,469 / 32,641 covered, 13,172
parse failures; 541 rows unbucketed). Key: the first-failure byte offset in each
`parse_failure` row's `message`, bucketed by the token at that offset and the
token before it. A unit is a whole card face, so a family's count is the units it
is the FIRST failure of.

**Reading a mid-word offset.** An offset inside an orthographic word
(`color|less`, `Other|wise`, `a|ddition`, `land|cycling`) is an ARTIFACT of
boundary suppression, not a scanner split: `has_lexical_boundary`
(`crates/deckmaste_english_v2/src/parser/scan.rs:1184`) does enforce a right word
boundary, and the prefix reading survives only under an adjacency-marked
continuation, then dies. Bucket such rows by the WHOLE WORD. Never mint a scanner
ticket on this evidence.

**Classify before minting.** A family is LEXICAL iff (i) the fix is a member
added to an existing inventory, (ii) every consumer it reaches is already a
standalone `lex(...)`/`verb(head)` form, and (iii) the landing adds zero
constructions, sums, seams or features. LEXICAL families are BATCHED one ticket
per round (`english-v2-lexical-inventory-<date>`); only STRUCTURAL families get a
one-family ticket. Criterion (ii) is decided by reading the consumer's `form`
line, never by the category name appearing in a derivation — and every row is
verified against a CONTROL sentence whose lexeme already exists. Six of this
round's classifications flipped on that check; see below.

Families at >= 15 units:

| family | units | class | owner |
|---|---:|---|---|
| preterite finite clause (`died`, `attacked`, `entered`, `left`, …) | 929 shaped | STRUCTURAL | minted `english-v2-tail-preterite-finite-clause` |
| `as`-clause (`only as a sorcery`, `enter as a copy of`) | 496 | STRUCTURAL | `english-v2-remaining-prepositions` |
| elliptical / re-subjected predicate sequencing (`If you don't, …`, `…, then you may …`) | 350 | STRUCTURAL | fogged only, under "Gerund clauses and modal ellipsis (A9, A11)" |
| `was kicked` | 194 | LEXICAL | batched |
| `in addition to its other types` | 170 | STRUCTURAL | `english-v2-locative-licence-set` (double-blocked; the noun alone will not land it) |
| `dealt by` / `controlled by` | 154 | STRUCTURAL | `english-v2-remaining-prepositions` — fails at `by`, not at the participle |
| landwalk fused surfaces (`Swampwalk`, …) | 127 | LEXICAL | batched |
| energy counters (`you get {E}{E}`) | 104 | STRUCTURAL | UNOWNED |
| copula contraction + `still` | 99 | STRUCTURAL | UNOWNED |
| `who` relative clause | 94 | STRUCTURAL | `english-v2-relative-clause` |
| typecycling fused surfaces (`landcycling`, …) | 91 | LEXICAL | `macro-keyword-templates` (design-gated there) |
| `share` / `shares` | 91 | LEXICAL | batched |
| bare `X` as card-count quantifier | 141 | STRUCTURAL | UNOWNED |
| `until you <verb>` | 84 | STRUCTURAL | `english-v2-subordinate-clause` |
| sentence-initial `Otherwise,` | 84 | STRUCTURAL | UNOWNED |
| `amount of X equal to Y` | 83 | STRUCTURAL | UNOWNED |
| `win` | 77 | LEXICAL | batched |
| `Do this only once each turn.` | 18 | STRUCTURAL | UNOWNED — residue of `english-v2-frequency-adverbial-family` (landed 2026-09-05); blocked by the pro-verb predicate, not the frequency adverbial: `Do this.` fails on its own |
| `the greatest X` (superlative measure) | 76 | unresolved | UNOWNED |
| `This ability triggers only once` | 74 | STRUCTURAL | UNOWNED (the frequency ticket does not quote this shape) |
| `Affinity for X` | 73 | STRUCTURAL | `english-v2-affinity-quality-surface` |
| `creature without <ability>` (NP postmodifier) | 68 | STRUCTURAL | UNOWNED |
| `attacks alone` | 64 | STRUCTURAL | UNOWNED |
| `equal to` as an NP postmodifier | 58 | STRUCTURAL | UNOWNED |
| `Multikicker` / `Prototype` / `Change` | 53 | mixed | UNOWNED |
| `devotion` | 52 | STRUCTURAL | UNOWNED |
| `Partner with <name>` | 52 | STRUCTURAL | `english-keyword-atom-roles` |
| storage / finality counter kinds | 45 | LEXICAL, consumer unverified | UNOWNED — verify criterion (ii) before batching |
| `instead of V-ing` | 44 | STRUCTURAL | UNOWNED |
| The Ring tempts you | 42 | STRUCTURAL | `macro-ring-emblem` |
| `transforms into <name>` | 37 | unverified | UNOWNED |
| `historic` | 33 | STRUCTURAL | UNOWNED |
| `unlock` | 30 | LEXICAL | batched |
| level-range line (`1-9 \|`) | 26 | STRUCTURAL | UNOWNED |
| `party` | 26 | STRUCTURAL | `engine-party` (design-gated) |
| `exert` | 26 | LEXICAL | batched |
| `Splice onto <subtype>` | 23 | STRUCTURAL | UNOWNED |
| `exploits` | 23 | LEXICAL | `english-ability-derived-verb-batch` |
| `target beyond the first` | 22 | STRUCTURAL | UNOWNED |
| `at random` (postmodifier position) | 21 | unresolved | UNOWNED |
| `twice that much/many` | 21 | unresolved | UNOWNED |

Relative-clause residue routed from
`english-v2-tail-preterite-finite-clause` on 2026-09-05 is owned by
`english-v2-relative-clause`:

- `5127c53a4d019cd2ac0cc868f916703b84d38ace52d3d3ee49c6f7d068be242b`
  — Ashen-Skin Zubera — `that died this turn` — blocked by the
  `MannerReference.this_way` invariant before the preterite distinction can
  decide selection.
- `a70c14b1f1abb00e54a5aa550264c5040c79a2e241a2cf5a0a2c725140d94d39`
  — Boldwyr Heavyweights — `who searched` — stops at `who`, before the
  preterite distinction can decide selection.

Frontier residue routed from the same landing: preterite/past-participle
homograph host discrimination (`dealt`). `Deal::Preterite` must remain withheld
until the host distinguishes it from the selected past-participle reduced
passive; otherwise these established selections move to a wrong finite
analysis:

- `57bbe75f309b505dd4882064db97811056a148735b94cd43558202587b752c06`
  — Aggravate — `Each creature dealt damage this way ...`.
- `4c82b6f6b1da0021ddb05732ec64c82a012ca49c9807f0e2d6f835c1843f4741`
  — Ballista Watcher // Ballista Wielder (Ballista Wielder face) — `A creature
  dealt damage this way ...`.

Preterite morphology residue routed from the same landing (review, 2026-09-05),
same family as the `dealt` item above and UNOWNED:

- Only `Cast` and `Search` of the 67 verb declarations under
  `plugins/builtin_v2/macros/` carry an authored `preterite:` surface, and the
  core inventory leaves `Have`, `Share`, `Unlock` and `Win` without one. Every
  unauthored lexeme's preterite is homographic with its past participle, so
  authoring them reopens the host-discrimination question above. Attested
  preterite surfaces waiting on it include `If you discarded` (Psionic Snoop),
  `player who shuffled` (Collision of Realms), `opponent who voted` (Erestor of
  the Council), `a creature that fought` (Boxing Ring), `a permanent you
  controlled explored` (Herald's Reveille) and `who investigated` (Wernog,
  Rider's Chaplain).
- A plain/preterite homograph selected under a build-time Concord Class derive
  keeps the underspecified `{Plain, Preterite}` Inflectional Form on its leaf:
  the finite host licenses the set rather than rewriting it, so a
  third-person-singular subject does not stamp `Preterite` on the selected
  analysis. Byte-identical surfaces make this unobservable in rendering,
  ownership and provenance today; it becomes real the moment a consumer reads
  tense off the tree.

Not a family, do not mint: the bare `.` fragment, 660 units over 102 distinct
preceding tokens — the generic end-of-sentence position.

Classifications that flipped on the control check (record, so they are not
re-proposed): `died` and the other `-ed` forms are NOT lexeme gaps — `Attack`
has its frames and `attacked` still fails, so they are the preterite family;
`dealt by` fails at `by`, not at `dealt`, so it is a preposition gap; `shares`
and `win` ARE lexeme gaps, proved by `that controls a land` and `If you control
a Goblin` selecting; `was kicked` IS a lexeme gap, proved by `was exiled`
selecting; `named` had zero units under this key and was dropped.

Routed residue: the vocabulary is renamed `ColorWord` ([CR#105.4]) by
`english-v2-rename-color-vocabulary`, rather than folded into the lexical batch.
The `only … each turn` family landed 2026-09-05 as
`english-v2-frequency-adverbial-family` (95 units gained, measured on change
`mkvwwvzuwytw`, lock `covered` 19,564); what remains of it is the 18-unit
`Do this only once each turn.` row above and a 2-unit coordinated focus tail
(`… and only once each turn`, Nature's Chosen and Sawback Manticore), both
UNOWNED.
Coordination families generally: v1's coordination modules are a phenomenon
checklist, never code or vocabulary to import.

Hangs on: `english-v2-grammar-migration-close` before frontier scheduling resumes.

### Bare temporal adjunct inside a passive predicate

Routed here from `english-v2-bare-duration-adjunct-licence` (2026-09-05). That
landing made a marker-less Duration Phrase read the determinative's declared
`bare_duration_license`, which correctly retired the zero-determined `step`
reading of `if you've been attacked this step`. The surviving selected reading
is still wrong: `this step` is analysed as the passive predicate's object
(`ObjectObjectNominal -> NounPhraseQualifiedNounPhrase ->
UnqualifiedReferenceDeterminedNominal`) rather than as a temporal adjunct of the
passive. Eleven identities select it, all with the same clause, all still
covered:

- `a52b2536e676ecd15985cf16dee330001ed00c2a8c30a6451582e310cc41912a` — Assassin's Blade
- `92be245709b1c7dd6d33fc2b3e4bd385d9120613e84a8339879ebb78d6d933af` — Champion's Victory
- `9c974195f93b89e654ea0152acaabd1c5df3e42e8a87b8f835090b064a24c409` — Defiant Stand
- `cd8a16b89dd6347cc571f3413c667fe443f44f07d62f5509437a52257df47485` — Eightfold Maze
- `7481f5103edbf11abf8bd08ea38ccfc8930d9ca8d27ada55eda758525afc4487` — Just Fate
- `43f14e90b11f20a925d3d3f6cf43167d284bbb0d58e9c58abebe239c6a785680` — Kongming's Contraptions
- `a884c45e1a179a5399b24a3b80eea671122c261def8e0604016a40255a764f1b` — Rally the Troops
- `eedeb6537edb2f7be63bd5a941564e72c587fccc23a27ed75f19b6793f6cbda2` — Remove
- `e310d19a800a04135b8ebd394e11b323d52f39f854e90b12e358ec818d6dd8b7` — Scorching Winds
- `16537b37e03639380fe8b773503a0076ff4ef349be46d69b24c4be0d695b5f3e` — Treetop Defense
- `3f0d55ab5f9a6fc1be0ebeb6b647f38635e324b4a80bc9ee30bf927e45622d32` — Warrior's Stand

The same landing's synthetic probe `Cast this spell next turn.` shows the
general shape: a nominal-object candidate outranks the adjunct candidate on
specificity, so a temporal phrase after a predicate is preferentially swallowed
as an object. No corpus identity turns on the `next` case — every corpus
occurrence of `next` is preceded by a determiner or possessive — so the
prospective-deictic determinative the landing had added was removed at review as
unreachable vocabulary; whether `next` should be a Determinative at all belongs
to the lexical-inventory family, not here.

Two decisions it waits on: whether the passive predicate declares an adjunct
position of its own, and how the object/adjunct preference is expressed once it
does.

Hangs on: `english-v2-grammatical-relations` (the object/adjunct distinction it
needs) and the attachment device parked under
`english-v2-attachment-class-declared`.

### Door half-level trigger (`When you unlock this door, …`)

Residue routed from `english-v2-lexical-inventory-2026-09-05`. The `unlock` verb
identity landed in that batch and gained **zero** corpus identities: on the
witness `When you unlock this door, draw a card.` the parse now advances through
`unlock` and stops at bytes 21..25, on `door`. So the missing pieces are the
`door` noun and the half-level referent a Room's two halves need, not the verb.

`workbench-room-halves` owns the game-model side and is **done**; nothing live
owns the English-v2 parity, which is why this entry exists rather than a
`Hangs on:` pointer to a closed ticket.

Shape it should take: the `door` noun declaration plus whatever reference form
addresses one half of a two-halved Room, so the trigger clause composes out of
existing trigger grammar. Count the units with `jq` before graduating — the
lexical-inventory batch measured the row at 30 units by surface, of which none
was reachable.

Hangs on: nothing live. Graduate on a corpus count plus a pinned shape for the
half-level referent.

### Shared gaps (neither v1 nor v2 covers)

Recorded for the record, attributed to neither grammar — no ticket owns these, and
`english-v2-relative-clause` explicitly does not require the relative ones:

- `which` / `whose`
- pied-piping
- non-restrictive relatives
- finite `that`-complement clauses (and wh-complement clauses)
- correlative coordination (`both…and`, `neither…nor`, `either…or`), and `but`/`nor`
- gapping / right-node raising
- `the former` / `the latter`
- reminder text
- AdvP with degree modification

These gaps are now inputs to `english-lean-grammar-model`'s whole-grammar scope
map. Decide representation or an explicit scope exclusion from grammatical and
editorial evidence; a corpus count is not a prerequisite for modeling a general
capability. The migration design owns subsequent implementation routing.
