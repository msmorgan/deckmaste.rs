---
needs: []
---
**Residue of round `kwgrant`: parameterized keyword abilities in grant
position.** Campaign-internal residue of `english-structural-recovery-zero`,
split out so the diagnosis survives in the tree; fold into the live campaign
workspace with `workflow claim english-keyword-grant-arguments --into
english-structural-recovery-zero`.

Round `kwgrant` (2026-07-27) carried the keyword-line argument machinery into
grant position. A keyword ability granted by `has`/`have`/`gains`/`gain`/
`loses`, or attached by a token's postnominal `with`, is now a keyword-headed
`NominalPhrase` (`NounInstance::Mass(Noun::Catalog(atom))`) whose new
`NominalComplement::KeywordArgument` carries the structured argument —
`KeywordArgument::Costed(KeywordCost::Symbols(..))` for `ward {2}`
[CR#702.21a], `KeywordArgument::Predicated(..)` for `protection from black`
[CR#702.16a,702.16g]. 124 spans / 1429 source tokens recovered, 0 rows added.
What it deliberately left is below. Counts are unresolved rows measured against
the post-`kwgrant` census (clause 3623/67633, unknown 5344); every count is a
`clause`-role row unless stated.

1. **Disjunctive predicated arguments — 5 rows** (Jeweled Spirit, Razor
   Barrier, Giver of Runes, Angelic Intervention, Apostle's Blessing;
   `protection from artifacts or from the color of your choice`).
   `PredicatedArgument` is a bare `Vec<PredicatedQuality>` that **records no
   connective**, and `Renderer::keyword_argument`'s inverse hardcodes `" and "`
   between qualities, so an `or`-joined argument is not merely unparsed — it is
   *unrepresentable*, and admitting it would silently render as `and`. The
   round's `accepts_keyword_grant_prefix` dot-2 gate rejects `or` at predict
   time and a duplicate check guards the reduce, with
   `keyword_grant_predicated_or_coordination_stays_unresolved` pinning it. The
   fix is a connective-bearing coordination carrier on `PredicatedArgument`
   plus its renderer inverse; it must not overload the current `Vec` whose
   inverse is fixed to `and`.

2. **A single bare `<keyword-noun> from <NounPhrase>` quality is genuinely
   ambiguous against the generic `Nominal -> Nominal PrepositionalPhrase`
   reduction, and the generic reading currently wins.** Both trees render
   byte-identically, so this is invisible to round-trip. The new
   `NominalKeywordPredicatedArgument` rule and the pre-existing
   `NominalPrepositional` rule can each complete the same span (a bare
   plural/mass quality needs no determiner); costs tie, and ties resolve by
   registration order, which `add_keyword_grant_rules` loses by construction —
   it is called last precisely to preserve earlier `RuleId`s. **No corpus row
   currently takes the wrong path**: every row that cleared has either a
   `ColorWord` quality (no noun phrase for the generic PP rule to consume) or a
   coordinated `from X and from Y` tail (which the single-PP rule cannot span),
   and every single-bare-quality row in the corpus fails for an unrelated
   reason (entry 4). The latent defect is nonetheless real and will surface the
   moment one of those hosts is repaired. `grammar/clause.rs`'s
   `keyword_grant_predicated_noun_phrase_quality_is_a_known_ambiguity`
   deliberately **asserts the currently-observed generic tree** so a fix is
   visible as a change there; flip that assertion when fixing. The only
   available fix found was giving `Features::Noun`/`Features::Nominal` a field
   carrying whether the head is a keyword-ability catalog atom — Features carry
   no atom identity at all today, by design — which touches every match arm
   over those features file-wide. Wildly out of proportion to `kwgrant`'s
   scope; wants its own round.

3. **Conditioned quality runs — 1 row** (Dominaria's Judgment, `gain
   protection from white if you control a Plains, from blue if you control an
   Island, …`). Needs a per-quality condition on the quality itself plus the
   elided repeated keyword surface, not merely an argument in grant position.

4. **Seven rows whose keyword argument is no longer the blocker.** Each was
   predicted to clear and did not; each was read with `cargo xtask english
   inspect -a` and reports `NoCompleteParse` for the **whole** clause, so none
   is a silently-wrong tree. None was root-caused — the surviving hypothesis is
   named for each, and all seven look unrelated to the keyword-argument
   mechanism:
   - Perch Protection (`protection from everything`) — the surrounding
     `If the gift was promised, … phase out, and … your life total can't
     change` chain;
   - Katilda's Rising Dawn (`protection from Vampires`) — the trailing
     `where X is the number of permanents you control that are Spirits and/or
     enchantments`, an `and/or`-coordinated relative;
   - Aven Warcraft (`protection from the chosen color`) — a bare single
     `NounPhrase` quality, but the observed failure is a total parse failure,
     not entry 2 manifesting;
   - Commander's Plate (`protection from each color that's not in your
     commander's color identity`) — a restrictive relative on the quality;
   - Guardian Archon (`protection from the chosen player` [CR#702.16k]) — the
     coordinated distributive subject `You and target permanent you control
     each gain`;
   - Escaped Shapeshifter — `not named Escaped Shapeshifter` self-reference
     negation and the `The same is true for …` ellipsis;
   - Nevinyrral, Urborg Tyrant (`Hexproof from artifacts, creatures, and
     enchantments`) — a single Oxford-coordinated noun-phrase quality on the
     atom-carried path.

5. **`Equipment you control have equip {1}.` — 3 rows** (Astor, Puresteel
   Paladin, and Syr Gwyn's `equip Knight {0}`, which is a `RestrictedCost`
   shape besides). **The obvious root cause is false and was checked:** `Equip`
   is a `CatalogKind::KeywordAbility` atom (`data/gen/catalogs/
   keyword-abilities.txt:63`, loaded by `CATALOG_FILES` in
   `crates/xtask/src/english/data.rs`), `keyword-actions.txt` does not contain
   it, and `Catalogs::matches` maps `CatalogSlot::KeywordAbilityNoun` straight
   onto that kind — so the round's new lexical slot does reach `equip`. The
   corpus holds exactly three `Equipment you control have …` sentences and all
   three are these rows, so there is no positive control; every other
   `Equipment you control …` sentence in the corpus takes a **singular** verb
   (`enters`, `gains`, `is`). The surviving hypothesis is a number-agreement
   gap on the plural-invariant catalog noun `Equipment` as a subject of plural
   `have` — a subject-side gap with nothing to do with keyword arguments.

The round also generalized by shape rather than identity, as intended: the
symbol-cost slot gates only on keyword-atom catalog membership, so 26 grants
outside the ward ledger recovered with it (unearth, cycling, flashback,
encore, evoke, outlast, warp, prowl, sneak, ninjutsu, miracle, echo,
offspring, slivercycling, web-slinging, freerunning). One accepted
over-generation follows from the same choice: a bare keyword followed by a
symbol cost (`has flying {2}`) will parse. That surface does not occur in the
corpus, and a closed list of cost-taking keywords was rejected as an identity
gate. An ordinary noun still cannot acquire either complement —
`keyword_grant_ordinary_noun_never_acquires_a_symbol_complement` and
`..._never_acquires_a_predicated_complement` pin that.

Standard constraints apply.
