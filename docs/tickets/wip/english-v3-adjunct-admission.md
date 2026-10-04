---
needs: []
---
# Expand lexical coverage, license adjuncts and rank retained readings

Add trigger, remain, resolve, distribute, effect, emblem, devotion and time to
lexical declarations. Keyword-action verbs remain owned by their macros; Explore
keeps both its intransitive and transitive frames. Regular morphology supplies
all four new verb paradigms, including triggered and triggering. Devotion is
mass-only. Time preserves count and mass uses.

Declare FrequencyUnit on count time and attach cardinal frequency phrases to
finite and secondary VPs. Declare AdverbialUse on prepositions and require it at
clause/VP adjunct hosts. Of remains available for nominal attachment and selected
complements, but cannot become a free clause/VP adjunct. Admission uses declared
grammatical features, without word-, verb- or card-named guards.

Construction declarations accept nonnegative `cost N;`, defaulting to 1.
Lexical leaves cost zero; generated readings expose checked summed costs.
Diagnostic samples prefer lower total cost with deterministic fingerprint ties.
All admitted readings remain counted and validated. The generic cost-100
fallback test demonstrates the requested flavor-label weighting; the actual
label constructions belong to the separate keyword-bodies-over-helpers work.

## Landing record

### PROVE

The required `cargo xtask gate --changed --run` passes. Independent constructed
values exercise lexical paradigms, frequency structure, selected complements,
nominal attachment, rejected free-of adjuncts, cost totals and retained
alternatives. Both roundtrip laws, lexical identity and traversal remain checked.
No tests were deleted, disabled or weakened. Corpus enumeration is unlimited;
mechanical issues, internal failures, cycles, duplicates, incomplete enumeration
and undetermined counts are zero. Corpus laws do not alone prove linguistic
correctness; the changed structural sets were reconciled separately.

Across the 54 changed identities, 835 old fingerprint trees survive unchanged,
1,519 removed trees all contain a free-of PP at a prohibited clause/VP host,
and 288 new trees belong to nine newly covered faces. No covered identity is
lost and there are no unexplained removals. The 45 previously covered faces with
removed trees are Donate, Ground Seal, Vedalken Plotter, Solarion, Dennick (Pious
Apprentice), Aethersnatch, Lurker, Bearer of the Heavens, Palace Guard, Switcheroo,
Ideas Unbound, Debt of Loyalty, Kalonian Hydra, Perplexing Chimera, Stigma Lasher,
Silent Gravestone, Spellbane Centaur, Mossborn Hydra, Labyrinth Guardian,
Treacherous Pit-Dweller, Blatant Thievery, Stiltzkin (Moogle Merchant), Wall of
Glare, Ashes of the Abhorrent, Brackwater Elemental, Stormchaser Drake, Dust,
Sky Swallower, Momo's Heist, Ironfist Crusher, Dream Strix, Riches, Thalakos
Deceiver, Cultural Exchange, Suq'Ata Firewalker, Aura Thief, Wrong Turn,
Political Trickery, Empress Galina, Illuminator Virtuoso, Brand, Phyrexian
Infiltrator, Harmless Offering, Dense Foliage and Voracious Hydra.

### DISCLOSE

New covered identities and complete reading counts:

| Face | Oracle identity | Readings | Intended added analysis |
| --- | --- | ---: | --- |
| Jadelight Spelunker | 0a72d6cd-b4a7-4c42-90ba-a14f1733d00e#card | 2 | explores + X times frequency adjunct |
| Blessings of Nature | 1e77cf90-ac51-4ac7-b123-be04aabe1688#card | 14 | distribute, selected among complement |
| Stolen Goodies | 334fba11-e500-40fd-a142-41ff553642b5#face:1 | 86 | distribute, selected among complement |
| Verdurous Gearhulk | 3765e8bb-e70d-4503-b8dc-1e684e434c18#card | 96 | distribute in triggered clause |
| Grove's Bounty | 54175132-2c44-4749-8dfd-d08dcc63e4b3#face:1 | 86 | distribute, selected among complement |
| Follow Him | 8c90b558-de07-4689-9427-da68d823c2e8#face:1 | 1 | Investigate X times |
| Confirm Suspicions | a30d076b-c942-45e4-b943-3749ce955291#card | 1 | Investigate three times |
| Expansion Algorithm | afbc348a-7b27-4577-8c24-cd6d5b894dc1#card | 1 | Proliferate X times |
| Full Flowering | cb3328b5-a759-4c30-8b1f-202980bb6f6d#card | 1 | Populate X times |

The earlier STOP audit correctly identified detached-of trees, but overstated
that every Object reading of times is ungrammatical. Nicanzil, Current Conductor
attests transitive explores a land/nonland card. Count time can independently
form an NP Object; that alternative remains. Jadelight's frequency reading
costs 20 and its retained Object reading costs 21. Nominal attachment/scope
alternatives remain even where their game meanings differ. The initial lexical
only draft had 453 trees across five new faces; the repaired grammar has 284
across those five and four additional frequency faces.

Scope expanded at the user's request from eight lexical entries to three
frequency productions, two grammatical features, compiler cost support and
sample ranking. No custom admission checker was added. The glossary records
Frequency Adjunct, Frequency Phrase, Adverbial Use and Construction Cost; the
lexical ADR explains that ranking is presentation, not pruning or admission.

The 92 source notices classify as 42 counter-kind declarations lacking lexical
grammar, 34 superseded/compositional vocabulary aliases and 16 retired
nonattestation restrictions. Existing-verb morphology has no demonstrated gap
in this audit. Counter-kind nominal modifiers, triggered-ability participial
premodification, Nicanzil's land-card compound and Elven Rite/Throw a Line's
among-one-or-two-target-creatures residuals remain with
`english-v3-systemic-residuals`. Do not reclassify every counter word as an
adjective to fit the current grammar. The prior admission STOP is resolved by
the feature-based repair and complete reconciliation above.

### REPORT

Measured 2026-10-04, change vqksnuwxroutywmwprkslkkmowoovtvx, eight workers,
32,828 Vintage-supported faces. Input SHA-256:
49dc966bda6ef588fc68e8d6972de25df1b660ed2ea654904584698263007ebb.
Lexical inventory SHA-256:
ed3d89e98c063905d7b80a7dee523e91e9528735d169c64e03b61c8777db29ee.

| Text census | Before | After |
| --- | ---: | ---: |
| No reading | 29,063 | 29,054 |
| One reading | 2,629 | 2,644 |
| Multiple readings | 1,136 | 1,130 |
| Covered faces | 3,765 | 3,774 |
| Exact readings | 13,539 | 12,308 |
| Faces with normalized vocabulary gaps | 3,908 | 3,298 |
| Distinct unknown spellings | 1,215 | 1,201 |

Type lines: all 32,828 have exactly one reading, zero issues. Lexical manifest:
34,828 owners, including 118 verbs and 551 nouns; all 43,046 independent values
pass. Eight entries added; 610 faces have their normalized vocabulary gaps
cleared. Raw lexical analysis includes reminders and still has 9,465 unknown
occurrences across 1,289 spellings; this differs from normalized parser input.

Debug text run: 21.338 seconds corpus wall time, checked-text thread CPU
313,177 ns/byte, initial host load [2.222, 1.620, 1.560]. Type-line run:
3.892 seconds, 47,783 ns/byte, load [1.848, 2.627, 2.139]. The text time exceeds
the old 16.26-second advisory; this is not an optimized performance or cutover
claim. The current English-v3 pipeline has no English coverage-lock field;
CR coverage-lock values are unrelated.

Scratch evidence: /tmp/english-v3-adjunct-final-text.json,
/tmp/english-v3-adjunct-final-type-lines.json, /tmp/lexical-adjunct-final.json,
/tmp/adjunct-before-changed-trees.json and /tmp/adjunct-after-changed-trees.json.
Reproduce corpus checks with `cargo xtask english-v3 --all --workers 8` and
lexical checks with `cargo xtask lexical --all --workers 8`. Citation audit has
zero noncompliant entries. Focused changed-file formatting and cost-test clippy
pass; whole-repository formatting and broad strict clippy encounter existing
unrelated violations (macro_ron/set.rs and numeral_features.rs).
