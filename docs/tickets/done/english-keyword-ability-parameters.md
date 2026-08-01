---
needs: []
---
**Keyword-ability parameter shapes the closed `KeywordArgument` vocabulary
still cannot express.** Campaign-internal residue of
`english-structural-recovery-zero`, split out so the diagnosis survives in the
tree.

Round `kwparam` (2026-07-27) typed the tight em-dash `[cost]` surface as a real
`Cost`, added `KeywordArgument::RestrictedCost` for `[quality] [cost]`, and
composed the two. What it deliberately left is below. Counts are unresolved
rows measured against the post-`kwparam` census (clause 3861/70731, unknown
5584); every count is a `clause`-role row unless stated.

1. **`Partner with [name]` — 51 rows.** The parameter is a proper card name
   (`Partner with Silvar, Devourer of the Free`). Needs a genuine name
   nonterminal, which is the same gap the round-trip invariants already record
   for coordinated `named X and Y` members. The largest single named family
   left in the keyword-parameter space and the only one whose blocker is a
   missing nonterminal rather than a missing argument shape.

2. ~~**Bare quality on a single keyword line — `Hexproof from [quality]`, 7
   rows.**~~ **Done in round `kwgrant` (2026-07-27)** by exactly the fix this
   entry prescribed: `parse_predicated`'s single-line gate now reads
   `in_list || carries_from`, where `carries_from` is threaded from the
   matched atom's own catalog surface (`keyword_atom_carries_from`, a
   whole-word `from` suffix test), never a keyword identity. Six of the seven
   rows cleared — Garruk's Harbinger, Knight of Grace, Knight of Malice,
   General Ferrous Rokiric, Sphinx of the Guildpact, Breaker of Creation —
   plus the previously unlisted Niv-Mizzet, Supreme (a `keyword argument`-role
   row) and the grant form on Veil of Summer. `monocolored` needed a new
   `AdjectivePhrase` arm in `parse_predicated_quality`: it is an adjective
   (`regular-vocabulary.tsv:526`), not a noun, so the pre-existing bare
   `NounPhrase` attempt alone would have left Rokiric and Sphinx behind.
   **Nevinyrral, Urborg Tyrant remains** (`Hexproof from artifacts, creatures,
   and enchantments`): its quality is a single Oxford-coordinated noun phrase,
   and it still reports `NoCompleteParse` for the whole line. Not root-caused;
   see `english-keyword-grant-arguments`.

3. **`Champion a [quality]` — 12 rows.** A bare noun-phrase argument with no
   preposition and no cost, so neither `Predicated` (needs `from`/`for` or a
   list) nor `RestrictedCost` (needs a trailing symbol cost) admits it.
   Accepting bare noun phrases on a single keyword line is the over-acceptance
   risk `parse_predicated`'s `in_list` gate was written to avoid; wants a
   real categorical opener, not a widened fallback.

4. **Repeated-preposition quality coordination — 2 rows** (Elite Inquisitor
   `Protection from Vampires, from Werewolves, and from Zombies`; Oversoul of
   Dusk `Protection from blue, from black, and from red`). `PredicatedArgument`
   already models coordinated qualities, but `split_coordinated_predicates`
   handles the ` and `-joined two-member surface, not a comma-plus-Oxford
   three-member one. Run `english-coordination-agrees-audit` alongside.

5. **Variable-count arguments with a `where X is` rider — ~14 rows**
   (`Devour X, where X is the number of creatures devoured this way`,
   `Firebending X, where X is …`, `Mobilize X, …`). `Counted(Quantity)` can
   hold `X`, but the trailing definitional rider has no term. Related:
   **`Reinforce X—[cost]`, 2 rows** (Swell of Courage, Wren's Run Hydra) —
   `parse_counted_cost` requires `TokenKind::Integer` for the count, so a
   variable count never reaches `CountedCost`. `kwparam` kept both Reinforce
   rows as deliberate negative controls for `RestrictedCost`; admitting them
   is a `CountedCost` count-type change, not a new shape.

6. **Headless quantity plus relative clause — 1 row** (Eye of Ojer Taq,
   `Craft with two that share a card type {6}`). `kwparam` gates the general
   `Quantity + that + …` surface out of `RestrictedCost` on purpose: the only
   way the noun-phrase grammar completes it today is by making `two` an
   `Noun::Opaque`, which destroys the numeral's structure. Wants a headless
   quantity that can take a relative complement. The sibling headless case
   `one or more` already parses structurally and recovered in `kwparam`, so
   the gap is the relative clause, not headlessness.

7. **Cost-component splitting treats a coordinated object list as separate
   components — 2 keyword-argument rows.** `parse_cost` splits on every
   top-level comma, so `Ward—Discard an enchantment, instant, or sorcery
   card.` (Saruman of Many Colors) becomes `Clause(Discard an enchantment)`,
   `Noun(instant)`, `Recovered("or sorcery card")` rather than one clause with
   a three-member coordinated object; and `Recover—Pay half your life, rounded
   up.` (Garza's Assassin) leaves `Recovered("rounded up")` instead of
   attaching the participial adjunct to `half your life`. `CostComponent::Noun`
   documents the comma-split-object-continuation model deliberately, so this
   is a known modelling choice rather than a bug — but the `or`-led conjunct
   fits neither `Noun` nor `Alternative`, and both residues are now visible in
   the keyword-argument cell. Revisit when cost components are next touched.

Also still recovering, pre-existing and unrelated to the above:
`Madness—Pay six {C}.` (Emrakul, the World Anew), `Pay eight {E}` (Salvation
Colossus), `Pay {B} and 1 life` (Infernal Darkness), and `Have an opponent
create a 1/1 red Survivor creature token` (Varchild's War-Riders) — four
tight-cost bodies whose cost clauses `parse_cost` cannot type. They recovered
before `kwparam` and still do; only their carrier changed.

Standard constraints apply.

## 2026-07-30: the bare-NP gate needs a keyword→shape table

Attempted entry 3 (`Champion a [quality]`) plus the Enchant misparse below with
a categorical opener — argument parses *exactly* as `Nonterminal::NounPhrase`,
keyword stands alone on its line (`!in_list`). It fixes both targets and moves
recovery 3390 → 3359 spans, but it is **not sound**, on two pre-existing
invariants:

- `Protection creature` becomes a keyword line, which
  `a_bare_noun_argument_is_not_admitted_as_a_keyword_line` forbids by design:
  protection's shape is `Protection from [quality]` [CR#702.16a], never a bare
  noun phrase.
- `Partner with [name]` (entry 1) breaks round-trip on Impetuous Protege and
  Proud Mentor — the noun-phrase arm captures the card *name* as an ordinary
  nominal, the same missing name-nonterminal gap entry 1 records.

Both failures share one cause: **which argument shape a keyword takes is not
recorded anywhere.** `KeywordArgument`'s doc says the argument-shape vocabulary
is closed, but nothing maps a keyword atom to the shape it licenses, so any
gate must either name keywords (banned) or accept every atom (unsound). The
open work is therefore a keyword→argument-shape table derived from the CR, not
another opener heuristic. The CR states a written form explicitly for only
three keywords (`Enchant [object or player]` [CR#702.5a], `Protection from
[quality]` [CR#702.16a], `Gift a [something]` [CR#702.174a]); the rest must
come from the exemplar rules the `kwparam` round already read.

**Not attempted:** the full `Predicated` → `Qualified(Phrase)` collapse.
`Hexproof from` is itself a keyword-ability catalog surface, so `carries_from`
reflects Scryfall data rather than an invention; collapsing needs the argument
re-cut against the shorter `Hexproof` atom (also a catalog member) plus a cost
preference for it when the argument coordinates.

## 2026-07-30: the subject gate is unsound too — same missing data

Also attempted the Enchant misparse from the other end: reject a clause whose
subject is a bare keyword-ability catalog atom (no determiner, modifiers, or
complements) in `lower_simple_clause`. It kills the misparse — `Enchant tapped
creature` recovers honestly instead of deriving a transitive clause whose verb
is a past-tense `tap` — and all 753 tests plus `roundtrip --require-clean`
stay green.

The recovery census refutes it: 3390 → 3405 spans. The premise is false.
`if tribute wasn't paid` (10 spans, Tribute [CR#702.104b]) is a *legitimate*
bare keyword-atom subject, as is the keyword coordination in `Target creature
without first strike, double strike, or vigilance …`. Only `Enchant tapped
creature` was a real misparse.

So both attempted fixes fail on the same missing datum: a per-keyword property
separating **declaration-only** keywords, whose name never heads a nominal
(enchant [CR#702.5a], champion [CR#702.72a]), from **referable** ones, whose
name is an ordinary nominal in running text (tribute, flying, trample). Neither
a structural gate nor an argument-shape opener can stand in for it.

**Method note.** Neither the round-trip gate nor the test suite caught the
over-rejection — recovery preserves spans verbatim, so a rejected derivation
still round-trips clean. The recovery census was the only instrument that saw
it. Pair that with the comma finding above: over-*acceptance* surfaces by not
storing derivable state, over-*rejection* surfaces in the census, and
round-trip catches neither.
