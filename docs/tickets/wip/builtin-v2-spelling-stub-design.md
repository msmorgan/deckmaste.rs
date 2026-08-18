---
needs: []
---
**Design ticket: settle the `plugins/builtin_v2/macros/stubs` spelling-record
design (keyword actions, keyword abilities, creature subtypes), record the
ruling, and plan the tree.** The user ratified the split `spelling` / optional
`grammar` design on 2026-08-17. The durable ruling is
[`docs/decisions/builtin-v2-macro-spelling-and-grammar.md`](../../decisions/builtin-v2-macro-spelling-and-grammar.md);
it supersedes the pre-ruling sketches and conflicting candidate fences kept
below as dialogue history. Runs independently of the stage-5 grammar buildout;
after it lands the user relays a refresh to the buildout claim. This design
ticket lands the ADR and three inventory follow-up tickets only; it does not
land stubs, a generator, a validator, or parser wiring.

Why the tree exists (settled): CR-enumerated keyword actions and keyword
abilities, and subtype spellings, are NOT core-grammar vocabulary. The
english_v2 slice currently hardcodes `Destroy`/`Connive` as lexemes; that is
acknowledged scaffolding until the frame seam. Spellings of these
inventories belong to the macro layer, and this tree is its nursery: each
stub graduates in place into the real builtin_v2 macro (single source of
truth, and the one surface a custom-set author ever touches). Non-keyword
game-action verbs (deal, gain, draw) are a closed set and STAY core
grammar.

Pre-ruling fences and candidate assumptions (the linked decision is now
authoritative where these conflict):

- Nursery ontology: stubs grow in place; no parallel registry ever.
- `name:` is the authoring address and equals the filename minus extension
  (PascalCase identifier). `spelling:` is how it's written on cards.
  Identifier-hostile catalog entries ("For Mirrodin!", "Start Your
  Engines!", "∞" — all three CR-verified real keyword abilities) therefore
  never touch filenames.
- Every orthographic fact lives in ONE structured value under `spelling:`.
  Records never accrue top-level how-it's-written fields.
- Grammatical form is the spelling value's enum variant, not a sibling
  mark; illegal combinations must be unrepresentable.
- Compound verbs record whole inflected surfaces ("collect evidence" /
  "collects evidence"), never head-verb position marks — position-into-
  string mechanisms are the v1 text-offset disease.
- Frames and spelling are UNIFIED. The spelling enum is compositional: a
  frame is a sequence of spelling atoms as structured data, holes are typed
  atoms, and today's stub values must embed unchanged as atoms of
  tomorrow's frames. A template string with embedded meta-syntax (the v1
  `template:` mini-language) is BANNED, as is the v1 `kinds:` registry —
  nothing in builtin_v2 inherits a v1 shape.

Current sketch (candidate only; every choice below is open to revision):

~~~ron
KeywordAction(name: "Destroy", spelling: Verb(bare: "destroy"))
KeywordAction(
    name: "CollectEvidence",
    spelling: Verb(bare: "collect evidence", third_person: "collects evidence"),
)
KeywordAction(
    name: "TheRingTemptsYou",
    spelling: Clause(present: "the Ring tempts you", perfect: "the Ring has tempted you"),
)
KeywordAbility(name: "ForMirrodin", spelling: Term("For Mirrodin!"))
CreatureType(name: "TimeLord", spelling: Noun(singular: "Time Lord", plural: "Time Lords"))
CreatureType(name: "Goblin", spelling: Noun(singular: "Goblin"))
~~~

Graduated form, illustrative only:
`spelling: Frame([Verb(bare: "amass"), Param(0, number: Plural), Param(1)])`.

Questions closed by the ratified decision:

1. The spelling enum's exact vocabulary and composition rules: the variant
   set (Verb/Clause/Term/Noun/…), how Frame atoms and holes are typed, and
   how this enum relates to the construction DSL's form atoms and the
   stage-5 lexeme surface tables (same value space, a projection, or
   deliberately distinct).
2. Field policy: omit-derivable (a present field always means a
   non-derivable fact; invariants like Sheep explicit) vs always-explicit.
3. One shared spelling enum across the three categories vs per-category.
4. `Term` naming; whether keyword abilities need grammatical kinds of
   their own.
5. Catalog integrity: the catalog-line ↔ stub mapping (mechanical
   PascalCase plus a counted exception map, e.g. "∞" ↔ `Infinity`?) and
   the direction and strictness of the eventual check gate.
6. Policy for evidence-less facts (the undetermined plurals below):
   best-call value with a marker comment, omitted field, or another shape.
7. Where the ruling is recorded — a section of
   `docs/decisions/english-v2-rewrite.md` or its own decision doc — and it
   is recorded BEFORE generation.

Evidence basis (re-derivable by grep against `data/mtgjson/AtomicCards.json`
and `data/rules/cr.txt`; inventory sizes from `data/gen/catalogs`:
keyword-actions 70, keyword-abilities 195, creature-types 324):

- Clause-form actions: exactly one. "the Ring tempts you" (present) with
  corpus-attested perfect "the Ring has tempted you" (Frodo, Adventurous
  Hobbit). All other multiword actions are verb-form with the conjugating
  verb as the first word (including the discontinuous "set … in motion").
- Casing exceptions (not the blind lowercase of the catalog name): "open
  an Attraction", "roll to visit your Attractions", "the Ring tempts you";
  ability "More Than Meets the Eye" stays Title Case mid-sentence.
- Corpus-confirmed irregular plurals: Ally→Allies, Army→Armies,
  Child→Children, Dwarf→Dwarves, Elf→Elves, Fox→Foxes, Fungus→Fungi,
  Hero→Heroes, Homunculus→Homunculi, Leech→Leeches, Lhurgoyf→Lhurgoyfs
  (NOT -ves), Mercenary→Mercenaries, Mouse→Mice, Nautilus→Nautiluses,
  Octopus→Octopuses (not Octopi), Ox→Oxen, Pegasus→Pegasi,
  Sphinx→Sphinxes, Spy→Spies, Werewolf→Werewolves, Wolf→Wolves; invariant:
  Starfish, Aurochs.
- No corpus evidence, needs the question-6 policy plus a per-entry call:
  Kor, Kree, C'tan, Astartes, Custodes, Cyclops (Cyclopes vs Cyclopses),
  Drix, Qu, Thalakos, Cyberman (franchise plural Cybermen, weak corpus
  support); also unverified-lowercase abilities "aura swap" and
  "power-up" (appear only ability-line-initial in the corpus).

Acceptance: every open question closed by explicit user ratification; the
ruling recorded, including the scope statement (keyword actions/abilities/
subtype spellings are macro-sourced; deal/gain/draw stay core; the slice's
`Destroy`/`Connive` are scaffolding until the frame seam); three follow-up
tickets minted for the committed keyword-action, keyword-ability, and
creature-type inventories; the catalog-integrity relationship stated in the
ruling; no stubs, generator, validator, loader wiring, grammar change, or
english_v2 edit landed here; the stage-5 relay (no per-keyword core
constructions in the buildout plans) handed to the user. Standard constraints
apply.
