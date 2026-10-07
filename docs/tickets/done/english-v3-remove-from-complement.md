---
needs: []
---
# Read *remove* with its Object and *from* source Complement

## Why

*Remove … from …* never reads. On change `xxknlzypsnwy` (32,828 supported
faces, 17,322 covered, 15,506 unread; recon of 2026-10-07), *remove(s) …
counter(s) from* touches **547** unread faces and is the sole cause on **276**
(recon bucket "remove counters": 608 / 300). Counts are unread faces *touched*
(at least one localised failing unit matches) / *sole* (every failing unit
matches and no other recon STRONG bucket does). They are surface counts, not
gain forecasts.

Probes (admitted roots): "Remove a charge counter from this artifact." 0;
"Remove all counters from target creature." 0; "Remove a charge counter." 0;
"Put a charge counter on this artifact." 1. Remove's only frame in `verbs.ron`
is `Predicate([ObjectNounPhrase, Lex("Preposition", "From"),
Role("FrameComplement")])`; it is the `core-verb:Remove` row of the
unsupported-inventory table in the done `english-v3-generic-frame-consumption`
("unsupported slot category FrameComplement (Complement)"). This ticket takes
that row over from `english-v3-systemic-residuals`.

## Goal

*Remove NP from NP* reads with the theme as Object and the *from* PP as the
source Complement, in imperatives, costs (*Remove a +1/+1 counter from this
creature: …*), finite clauses and under *may*. The legacy `FrameComplement`
slot is replaced by a typed NP Complement of the selected *from*, which the
generic consumer admits. Whether the omissible-source frame (Red Ward: "This
effect doesn't remove this Aura.") is added is decided by attestation and
recorded.

## Analysis

*Remove* takes an Object (theme) and a source PP with *from*: *I removed leaves
from the pool*, which unlike *drain* has no locatum-object alternant (\**I
removed the pool of leaves*) (CGEL, Ch. 4, §8.3.1(c), p. 315, [60ii]). The
source Complement is omissible: *He removed the key from the table* vs *He
removed the key* (Ch. 4, §8.3.3, p. 319). The *from* PP is therefore a selected
Complement, not a free Adjunct.

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket (`xxknlzypsnwy`):

- Spitting Hydra: "{1}{R}, Remove a +1/+1 counter from this creature: It deals
  1 damage to target creature."
- Timberline Ridge: "At the beginning of your upkeep, remove a depletion
  counter from this land."
- Phyrexian Prowler: "Remove a fade counter from this creature: This creature
  gets +1/+1 until end of turn."
- Voracious Hatchling: "Whenever you cast a white spell, remove a -1/-1 counter
  from this creature." (and its black twin)
- Perfect Intimidation: "Remove all counters from target creature." (modal
  bullet)

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Before: `cargo xtask english-v3 --all --workers 12 --samples-per-face 0
   --output target/english-v3/english-v3-remove-from-complement-before.json` on
   the claim parent, stamped with its change id and covered count; after: the
   same command to
   `target/english-v3/english-v3-remove-from-complement-after.json` on the
   final tree. Evidence lives under the workspace's ignored
   `target/english-v3/`, never `/tmp`.
2. Write each witness as a test first. Iterate on `--face-id` selectors; verify
   on `--all` at the end.
3. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
4. Compare per-face Reading counts against the claim-parent census. Any
   decrease on a previously covered face is restored, or named and justified in
   the landing record.
5. Spot-check at least 10 newly covered faces and list them by name with their
   Reading. A *from* PP read as a free Adjunct or as an NP postmodifier of the
   counter NP is a defect. A wrong analysis that parses is a defect, not a
   gain.
6. No guard may name a lexeme, construction, verb, noun, preposition or card;
   read declared features only. Tests are re-spelled, never deleted. No new
   xtask subcommands, flags, fixtures or tooling.
7. May edit `grammar:` frame fields in `plugins_v2` keyword actions and lexicon
   frames in `crates/deckmaste_lexical_source/lexicon/*.ron`; plugin bodies
   untouched. Lexicon additions must be attested in the supported corpus
   (pruning rule).
8. A new frame or construction must not overlap an existing one on the same
   string (two labels for one constituency is a spurious duplicate, not an
   ambiguity); when it supersedes one, retire the old route and re-spell its
   tests.
9. Retire a superseded route on both the lexicon and the grammar side; do not
   leave unreachable declarations.
10. A CGEL citation may back only what the cited passage itself says; a project
    or orchestrator ruling is cited as a ruling, never attributed to CGEL.
11. Timings as integer ns and ns/B, with host load and worker count.
12. Gate: `cargo xtask gate --changed --from <claim> --run`.

## Out of scope

- Other source frames deferred in `english-v3-systemic-residuals` (*from-M*,
  *from-A*: Exile, Discard, Reveal, Choose, Cast, Enter, Put): not reopened
  here. Reuse whatever typed source Complement already exists; do not build a
  second one.
- *counters removed this way* (reduced passive postmodifier) beyond what the
  Remove frame gives for free; *move a counter from … onto …* (Move).

## Landing record

Completed 2026-10-07. Claim: `otowzwsrrrzx`; implementation:
`unnxmsplryxo`. The measured baseline is `ryywzqmtlvnw` / **19,414 covered**
(the claim-parent production tree with only the new tests added); the final
measurement is `yrnluntlxnsk` / **19,682 covered**. All baseline/final corpus
and declaration figures below refer to those trees respectively. Refresh was a
no-op: no trunk changes or trunk-attributed Reading decreases entered the base.

### Implementation and source authority

Remove's sole frame is now
`Predicate([ObjectNounPhrase, MarkedRole("Preposition", "From", "NounPhrase")])`.
The source adapter produces an Object NP slot followed by a From-marked NP
**Complement** slot. This reuses Return's existing typed marked-NP vocabulary
and the generic `SelectedPredicate` consumer; it introduces no parallel
constituency or custom recognition rule.

The old `Lex(From), Role(FrameComplement)` route is removed from Remove's
lexicon entry. There was no Remove-specific v3 grammar declaration to delete:
the old route was an unsupported dynamic frame, not an existing Production.
Its replacement compiles through the existing consumer. The regression checks
that Remove has exactly one frame and no unsupported frame remains. No
unreachable declaration or second label for this constituency is retained.
Other verbs' legacy slots remain with their existing owners.

CGEL, Ch. 4, §8.3.1(c), p. 315, [60ii], supplies the theme-as-Object,
source-as-non-core-Complement pattern and rejects the locative-object alternant
for Remove. Ch. 4, §8.3.3, p. 319, explicitly treats Remove's source Complement
as omissible. Those citations support those linguistic claims only.

**Omissible source decision:** do not add the bare-Object frame in this landing.
Red Ward attests "This effect doesn't remove this Aura." This is an
implementation deferral, not a claim that CGEL requires an overt source.
A temporary diagnostic over `unnxmsplryxo` plus a bare-Object frame admitted
Red Ward's sentence, but changed the exact constituent
"remove a charge counter from this artifact" from one Reading to three:
selected source Complement, free PP Adjunct, and NP postmodifier. The latter two
are wrong for this constituent. The diagnostic frame was removed before final
verification. Narrowing From's Modifier/Adjunct licences globally would
reopen the source-frame prerequisites explicitly excluded by this ticket.
The omission follow-up is routed to the live
[systemic residuals](../planned/english-v3-systemic-residuals.md) ticket alongside
its source-licensing prerequisites. No project exclusion is attributed to CGEL.

### PROVE

No silent loss: **zero lost faces and zero decreases in per-face Reading
counts** on any previously covered face against the refreshed base. Every
previously covered face retains exactly its former Reading count. Both runs
completely enumerate the same 32,828 supported identities and analyzed inputs.
Exact Reading totals rise from 549,892 to 560,894. The comparison, including the
empty loss/decrease lists, is in
[comparison.json](../../../target/english-v3/english-v3-remove-from-complement/comparison.json).

Every counted Reading passes admission, lexical ownership, byte-exact
realization, construction traversal and leaf traversal identity. Both full
runs have zero issues, failed or limited enumerations, and undetermined faces.
The independently authored Sun Droplet constituent additionally checks the
opposite roundtrip law, exact singleton Reading identity, node and Word
traversal, and rejection of a wrong selected marker. Seven other tests check
all admitted Readings of the five ticket witnesses, finite Junk Golem and
modal Sun Droplet constituents. The eight added tests were red before the
frame replacement and green afterward.

No admission guard or licensing policy was added: forbidden word-named guards
added **zero**. Workspace lexical loading succeeds, with zero load errors.
The 87 unmapped source annotations are unchanged; these are not load errors.
There is no `environment.rs` or v3 `coverage`/coverage-lock authority on this
tree; the `english-v3` census is the coverage authority. No permitted legacy
licensing-checker total is emitted by the active tooling.

### DISCLOSE

| Measurement | No Reading | One Reading | Multiple Readings | Exact Readings |
|---|---:|---:|---:|---:|
| `ryywzqmtlvnw` / covered 19,414 | 13,414 | 7,245 | 12,169 | 549,892 |
| `yrnluntlxnsk` / covered 19,682 | 13,146 | 7,294 | 12,388 | 560,894 |

Specificity-resolved selections: zero on both trees; all admitted Readings
remain retained. No destructive selection or ambiguity quotient was introduced.

All **268** new covered identities are listed below. Their new analysis is
`SelectedPredicate` with the theme as Object NP and the source as a marked
From + NP Complement. Existing cost, finite, modal, imperative, quoted-ability
and document hosts compose that predicate. Every gain's input contains Remove.
No gain is claimed merely because an incorrect source attachment parses.

The ten complete spot-check sets contain 99 Readings and 124 Remove
occurrences; all inspected occurrences have that ordered Object/marked
Complement structure, with no From PP incorporated into the theme NP:

| Newly covered face | Readings inspected | Selected analysis / host |
|---|---:|---|
| Divine Intervention | 5 | intervention-counter Object; enchantment source; imperative and finite trigger |
| Trigon of Corruption | 5 | charge-counter Object; artifact source; activation cost |
| Phyrexian Prowler | 2 | fade-counter Object; creature source; activation cost |
| Hapatra's Mark | 2 | all -1/-1 counters Object; pronominal source; imperative |
| Aether Snap | 2 | all counters Object; all permanents source; coordinated imperative |
| Umezawa's Jitte | 4 | charge-counter Object; proper-name source; cost preceding modal bullets |
| Guiding Hydra | 10 | +1/+1-counter Object; creature source; under may |
| Spitting Hydra | 5 | +1/+1-counter Object; creature source; activation cost |
| Timberline Ridge | 44 | depletion-counter Object; land source; upkeep trigger |
| Voracious Hatchling | 20 | -1/-1-counter Object; creature source; both color triggers |

[Complete inspected trees](../../../target/english-v3/english-v3-remove-from-complement/spot-checks.json)
and [inspection results](../../../target/english-v3/english-v3-remove-from-complement/spot-check-analysis.json)
retain the evidence. These subset figures are measured on `yrnluntlxnsk` /
full-corpus covered 19,682, with six workers.

Full-face witness counts: Spitting Hydra 0 → 5; Timberline Ridge 0 → 44;
Phyrexian Prowler 0 → 2; Voracious Hatchling 0 → 20. Perfect Intimidation
remains at zero because its Exile source bullet is outside this ticket;
its Remove bullet passes the structural regression. Sun Droplet's modal
constituent passes, while its full face remains blocked elsewhere. Junk
Golem's full face rises from zero to 15. Other remaining source and host work
stays with systemic residuals.

**Deviations and additions:** eight tests added (the five requested witnesses,
finite and modal hosts, and the independent roundtrip/negative-marker test).
Tests restored: 0; re-spelled: 0; ignored: 0; removed: 0. No test was weakened.
The bare-Object experiment was discarded, with its reason and owner above.
Features added: none. Tables added: none. Policies added: none. Constructions
added: none. The only replacement lexical frame is the required marked-source
frame above. No plugin body or other verb frame changed. No STOP occurred;
the optional-frame defect was resolved within the ticket. Glossary gaps: none.

### REPORT and verification

Grammar economy, unchanged on both stamped production trees: **247** named
Reading constructions (204 ordinary declarations + 43 shared schemas),
315 category instances in 43 instance blocks, and 612 static compiled
Productions before dynamic selected-frame expansion. The claim-parent and
landed `crates/deckmaste_english_v3/src/declarations.rs` each contain **3,227
non-blank lines: net change 0**. No feature/table/policy/construction is added.

The generated-form homograph inventory is unchanged: 586 exact surfaces with
multiple lexical owners, including catalog/vocabulary collisions but excluding
card-name catalog entries and capitalization alternatives. The complete named
surface/owner list is retained in
[inventory-before.json](../../../target/english-v3/english-v3-remove-from-complement/inventory-before.json)
and [inventory-after.json](../../../target/english-v3/english-v3-remove-from-complement/inventory-after.json).
Word-bearing grammar form literals and form-literal/vocabulary overlaps:
**none** (named lists empty). The declaration inventory is unchanged, so these
are advisory reports rather than fitted admission gates.

Performance advisory, six workers in both runs:

| Tree / full covered count | Corpus wall time (ns) | Checked-text thread CPU | Host load (1/5/15 min) |
|---|---:|---:|---|
| `ryywzqmtlvnw` / 19,414 | 115,621,719,472 | 273,940 ns/B | 14.75 / 16.70 / 17.23 |
| `yrnluntlxnsk` / 19,682 | 172,404,067,966 | 374,763 ns/B | 14.12 / 13.14 / 15.01 |

Both exceed the 16,260,000,000 ns quiet-host advisory. These loaded-host
measurements are provenance, not a gate; they do not establish quiet-host
performance. The tractability obligation remains with its existing ticket.

Before command: `cargo xtask english-v3 --all --workers 6 --samples-per-face 0
--output target/english-v3/english-v3-remove-from-complement-before.json`.
Final census used the same built dev executable directly while Cargo's gate
held the build lock: `target/debug/cargo-xtask english-v3 --all --workers 6
--samples-per-face 0 --output
target/english-v3/english-v3-remove-from-complement-after.json`. Rust grammar
code was unchanged and refresh was a no-op. All census, experiment, inventory
and inspection artifacts were authored under the feature workspace's ignored
`target/english-v3/`, never `/tmp`; the named evidence directory preserves them
at landing.

Gate derived by `cargo xtask gate --changed --from otowzwsr --clippy --run`:

```text
cargo test -p deckmaste_lexical_source -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask
cargo clippy -p deckmaste_lexical_source -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask --all-targets -- -D warnings
```

Gate outcome: both commands pass; 802 tests/doc-tests pass, zero failures.
One pre-existing test remains ignored:
`macros::templates::tests::macro_schema_census_count_matches_21`, whose existing
attribute says "cross-checks the live corpus against the census; run on demand".
This landing adds no ignored tests and changes no ignore attribute.
`cargo fmt --all -- --check` passes. Citation checking reports zero
non-compliant citation strings and zero stale citations; the feature's piped
citation diff audit selects zero new Comprehensive Rules citation sites.

### Newly covered identities

Every row was previously unread and now has the selected Remove source
Complement analysis described above. Counts are stamped `yrnluntlxnsk` /
covered 19,682.

| Identity | Face | Readings |
|---|---|---:|
| `011371f0-a7f1-47a5-9883-22a14cfe4de9#card` | Shrewd Hatchling | 40 |
| `0158a440-6573-4f12-958b-a5f0cf190d5b#card` | Divine Intervention | 5 |
| `02765930-e15e-4cc1-ac0d-040334a9c5bc#card` | Chainbreaker | 5 |
| `044be66d-9334-4d4b-b420-b97a18698e95#card` | Junk Golem | 15 |
| `0481adcd-b66d-432e-80ba-3a4718bb3a7d#card` | Tumble Magnet | 5 |
| `071e15b5-6d21-4695-9ad0-69f40eb8f4ef#card` | Sporoloth Ancient | 1 |
| `084e11d9-dd1d-4e8e-afc1-9be4ee04a510#card` | Savage Firecat | 10 |
| `08e95b6c-9f4c-42d9-ad64-2d905d781d49#card` | Sturdy Hatchling | 40 |
| `0951b529-646c-4dfd-88ad-84ee117ce722#card` | Phantom Nantuko | 5 |
| `0da19cd4-7b3f-47bb-bb35-cbcc88ab53cd#card` | Trigon of Corruption | 5 |
| `0e4d2fbc-1b34-4501-9022-8ea800cb2d24#card` | Talon of Pain | 2 |
| `0e7cf14a-6149-4ba9-b3f5-c5faf9f772bc#card` | Quicksilver Fountain | 42 |
| `0f7f841a-1913-4396-9c94-bbd49dfdb9d1#card` | Festercreep | 10 |
| `0fa5803b-3604-46c7-9774-bf70e4a2b925#card` | Quest for the Holy Relic | 14 |
| `10b9bfb3-c478-45c6-b227-9c66b63bc79b#card` | Veldt | 44 |
| `1239d3bf-06e8-4732-b1e2-f6eb145e5894#card` | Thorn Thallid | 1 |
| `128ec451-97f6-4f01-8929-689fd4c45f40#card` | Phyrexian Prowler | 2 |
| `143d3fcf-2152-4911-b95a-d23b91662f33#card` | Hapatra's Mark | 2 |
| `15902a15-090a-44e6-b8d2-59e0f26ed46f#card` | Spike Colony | 5 |
| `159427f7-27fe-490d-ac78-fed092952f51#card` | Spinal Parasite | 1 |
| `162dcfa0-36e4-41cd-8322-08e810ff04d4#card` | Reluctant Dounguard | 15 |
| `16950554-0e39-4705-aab4-7a8734771844#card` | Shape of the Wiitigo | 12 |
| `1755b3d4-0e40-40c4-b913-0960d55d411b#card` | Phantom Tiger | 5 |
| `17b1c870-ef94-49af-8f9b-a8765f1b54db#card` | Trigon of Mending | 5 |
| `1932a19a-0c2b-4d16-a2ba-34ed136d8e70#card` | Spincrusher | 3 |
| `1a1a16e5-ce4f-42b8-a1c6-05dd635d83cc#card` | Aether Snap | 2 |
| `1bf97485-191b-466c-9590-3449466b348c#card` | Floodchaser | 45 |
| `1c0d4f28-2494-45b0-acb8-921fee602c01#card` | Pentavus | 40 |
| `1c55047d-badc-49bb-b791-e9759577e4eb#card` | The Duke, Rebel Sentry | 20 |
| `1cb42cf5-ae69-46c5-b2ff-d2c1cd60099b#card` | Myojin of Towering Might | 640 |
| `1da10d5c-36a8-473f-a5d4-782ad61d8057#card` | Umezawa's Jitte | 4 |
| `20b32052-f66f-4eb8-b56e-00d531907f19#card` | Vivid Marsh | 5 |
| `21f1c6d7-8289-44b2-b88f-c09e202be200#card` | Fertilid | 30 |
| `22e0e822-c4aa-4ec1-b7e4-0c1c869ef71b#card` | Investigator's Journal | 64 |
| `2309436e-3442-46d7-8ffc-9dc4a2314ae9#card` | Haruspex | 1 |
| `2358ffc2-6663-4ef3-b3a4-b036a4733ac6#face:1` | Kaiso, Memory of Loyalty | 5 |
| `24b11ede-26c8-40ed-b874-c0f6b21572c3#card` | Soul Stair Expedition | 14 |
| `24f1445c-16c9-45c0-be56-0266e6c78cdb#card` | Guiding Hydra | 10 |
| `2500a811-2435-4915-ac83-9bfe2887621a#card` | Deathbringer Thoctar | 1 |
| `262bc269-3e70-46ae-b0a3-5f8030a6971a#card` | Thallid Germinator | 2 |
| `26bfc19c-8122-4e95-9a62-2b5418429db2#card` | Midnight Oil | 60 |
| `2bc92c16-cd1e-4c32-9102-4593d77a9a9f#card` | Kappa Tech-Wrecker | 30 |
| `2ce2a091-95b6-45e5-9097-77fea411162e#card` | Fire Nation Turret | 4 |
| `2da7c49f-cc1e-45d9-9cbf-067e92b0daef#card` | Vivid Creek | 5 |
| `2e358f7e-c282-4b92-8255-1436c99cda49#card` | Power Conduit | 1 |
| `2ffb2647-60bb-4916-b919-cbcf18e5e424#card` | Lava Tubes | 44 |
| `3088ee75-3b51-4cbc-8987-76cb93f416b1#card` | Bladed Ambassador | 10 |
| `309a57cb-cc08-4892-bc63-c424ca9c5b43#card` | Savage Thallid | 1 |
| `31b990e3-9bad-4b0a-a9d5-f5b9ed2ad0b0#card` | Fain, the Broker | 7 |
| `3269f589-00fc-4f0b-8693-656f2ab0670d#card` | Argent Dais | 5 |
| `33510024-b1ff-428a-8ccd-2ca2a332d53c#card` | Gremlin Mine | 1 |
| `3361cf88-171a-4ddd-ac56-a31d42613c80#card` | Clockwork Dragon | 15 |
| `36072943-7823-434d-b898-f0a2c4e00af2#card` | Noxious Hatchling | 20 |
| `3641d572-8335-4804-88ad-edf4dc67a8e4#card` | Heartless Act | 9 |
| `36924990-8e3a-434e-bcd1-f03603cb350d#card` | Mabel, Bitter Recluse | 3 |
| `384c61f2-17ed-4c7a-8722-e0a05e9dcaf9#card` | Psychotrope Thallid | 1 |
| `3953fd17-03f4-4431-99b5-87dcac5a279d#card` | Feral Thallid | 1 |
| `3a2a6904-219a-4d16-b11a-1e0e2826814e#card` | Deathspore Thallid | 2 |
| `3a8f382e-0bc6-4e79-87e2-9f99d907d234#card` | Sawtooth Thresher | 2 |
| `3b02dfca-bcaa-4e22-b143-401ed250d016#card` | Titan Forge | 1 |
| `3b172be8-1bac-441e-98cc-f5af8da6005b#card` | Trigon of Rage | 10 |
| `3b30f043-aa79-4262-b13f-6cd794b1920b#card` | Surge Node | 5 |
| `3b71aeee-a4a8-46d3-9973-06db14d8bf6c#card` | Jolting Merfolk | 1 |
| `3d416f83-32b8-4dad-8e4d-3acd182bbe44#face:1` | Azamuki, Treachery Incarnate | 4 |
| `3d4c7f1a-deca-4bae-90c7-68d623da400e#card` | Purestrain Genestealer | 120 |
| `3d794629-9014-460a-8d3b-785eadc1cdb5#card` | Wickerbough Elder | 5 |
| `3eff8f21-9a1b-4afc-8a7b-75a740d0a4ea#card` | Clockwork Condor | 15 |
| `3fd23519-ef40-4c84-9407-2626d0c0d792#card` | Chimeric Egg | 21 |
| `4061572b-b7ff-4d06-9f71-9e66e064593d#card` | Darwin, Adaptive Mutant | 2 |
| `4097fea2-ae34-4b99-ab7e-a276d608b489#card` | Weather Maker | 2 |
| `43a2bfe4-9705-4d0f-901a-21f9803b50f1#card` | Price of Betrayal | 1 |
| `43e0dbaf-3c81-47e2-b0c2-364f7ac7295f#card` | Grim Poppet | 5 |
| `448e0cf2-c97c-4dc0-96ef-91f1eb75d97c#card` | Ion Storm | 1 |
| `44c4839f-c92a-4535-817b-1838c47fcb5e#card` | Risona, Asari Commander | 9 |
| `45bd5e97-f34e-47bc-ad21-da42d3444431#card` | Zektar Shrine Expedition | 28 |
| `46abaabe-7015-4d82-a5aa-2706b1f51bed#card` | Thallid | 1 |
| `46af64c9-2ab5-4e63-98db-cfb885a96109#card` | Conversion Chamber | 4 |
| `481e3a30-fbe9-4ed3-a982-8a73f0dca33b#card` | Thallid Shell-Dweller | 1 |
| `482ec736-7d4a-49c6-a757-e134123329ca#card` | Evolved Spinoderm | 70 |
| `48e3f6dd-68ab-420a-ad4d-fe7ee39486d3#card` | Arcbound Javelineer | 1 |
| `492dafd8-0393-4151-845c-d9ca10bda03e#card` | Utopia Mycon | 1 |
| `4a7c7ecc-df72-4da0-af35-dd0af58ec02c#card` | Mister Hyde, Monster Within | 2 |
| `4b515bb0-f275-4400-8032-3173b799ab40#card` | Walking Ballista | 5 |
| `4c6c064c-da61-4c7a-9607-8fb4490ff9a6#card` | River Delta | 44 |
| `4c9d9d47-1eb6-4981-ba83-d5ec04906e71#card` | Systems Override | 30 |
| `4e81ead0-df8b-4d9d-b1cb-d1d9dc503d4c#card` | Suncrusher | 2 |
| `527e4eec-0983-46a9-a4f7-451080200aea#card` | Carnifex Demon | 5 |
| `52e73390-75e4-45a1-8dbe-96e7cceb0a14#card` | Woeleecher | 1 |
| `56a55f45-2cc6-407d-b212-5619c98456d9#card` | Norn's Wellspring | 3 |
| `571c1923-790f-48b7-86d2-70c3e79fdba3#card` | Shriekhorn | 5 |
| `577eb51d-7979-4f97-a8b1-e31242720611#card` | Thrull Parasite | 1 |
| `592689f2-f17c-40d6-b880-431f4b6a6b0c#card` | Bristlebane Battler | 15 |
| `5972f426-4a51-4130-baf5-25407c1499f1#card` | Sunset Pyramid | 5 |
| `5ac65688-0a74-4dc4-b6cb-12bdf8aed01e#card` | Myojin of Night's Reach | 224 |
| `5b147eab-39dd-4c2a-8f59-d778037788aa#card` | Spike Feeder | 5 |
| `5c22447b-7a7f-4a27-8d3b-1bb664ea94c3#card` | Biting-Palm Ninja | 165 |
| `6025ce2f-d527-457c-866c-cb5c119d0bf4#card` | Karstoderm | 5 |
| `6096b221-7b6e-4528-8b5d-8ad0fcef3fba#card` | Gitaxian Raptor | 10 |
| `62a3f892-11cf-4064-8186-381ae0f9aa23#card` | Serum Tank | 1 |
| `631eb3e9-a3e0-4341-ad2d-e79142ce537d#card` | Loch Mare | 5 |
| `647d7988-9a5d-4700-9238-938d0d5fae12#card` | Karlov of the Ghost Council | 1 |
| `6755a7af-5b55-4245-a777-27e69979479b#card` | Lattice-Blade Mantis | 35 |
| `677c3616-295f-493a-8b60-1e22b0224cfa#card` | Ring of Three Wishes | 35 |
| `67a7bfa4-5004-4838-874d-03fa11f051d0#card` | Spitting Hydra | 5 |
| `686f4a37-b5e4-46d4-9e0e-11794b2d12cd#card` | Golgari Grave-Troll | 32 |
| `698098b8-b8e4-4e88-b13a-6b039579d192#card` | Ray Fillet, Man Ray | 1 |
| `6af2b898-7ba0-4641-8234-5171397cf602#card` | Trigon of Infestation | 40 |
| `6d972501-4174-4bf0-a26b-2edef31b3a0c#card` | Slumbering Walker | 40 |
| `6f498ff7-38c9-44ea-9cb5-994a003825ee#card` | Ingenious Prodigy | 10 |
| `6f571092-7244-4223-aa14-f5133cc000d8#face:1` | Ichiga, Who Topples Oaks | 2 |
| `6fd37ff3-e75d-459a-8717-8f6a9865fb32#card` | Clockwork Hydra | 5 |
| `713bcfb6-e5ea-411e-b4b3-52e5eed5cdf1#card` | Sanctuary Warden | 30 |
| `726fe7a4-49b3-4b67-9d8c-6b334d0fd915#card` | Spike Hatcher | 5 |
| `72e8d67a-41e6-4074-bcbd-b54ead995237#card` | Pentad Prism | 1 |
| `736168bb-0a5c-4b29-9f82-1319cd112873#card` | Quillmane Baku | 8 |
| `74466fa6-34d9-42f9-8590-c7f1fa672bf5#card` | Skullmane Baku | 8 |
| `744f79da-240f-4d14-a98c-e2796645ae88#card` | Shapers of Nature | 1 |
| `74f67dcf-5afb-45aa-8d4b-3cdb23f6f2a1#card` | Triskelion | 5 |
| `7668f7bc-9a43-4024-87c3-995aa0a65aaa#card` | Encumbered Reejerey | 15 |
| `7679942f-fcbc-4750-8e8c-fe0ad8103c11#card` | Blitz Leech | 3 |
| `7776b2dd-5795-458d-81da-54fed22d5ee2#card` | Spike Weaver | 50 |
| `7807122e-917f-4a79-876e-d1c10f7a38ed#card` | Shambling Swarm | 12 |
| `796f1e1f-7fec-429d-82ae-125f541d6cc7#card` | Ulasht, the Hate Seed | 548 |
| `79cf9c52-56e0-4237-a595-ec07824e5eed#card` | Bloodletter Quill | 9 |
| `7a49731b-bd95-4f0a-ac47-1aa318c12fec#card` | Rustvine Cultivator | 1 |
| `7b9ddff4-82f5-4068-8105-116b8a6180d5#card` | Clockwork Beetle | 15 |
| `7bd4e504-541e-4b2c-9f1d-3c67957aef2e#card` | Belligerent Hatchling | 20 |
| `7ceaddba-6301-45f8-86a1-ad4c6e6d3297#face:1` | Jaraku the Interloper | 2 |
| `7ecfa47e-1165-46a6-884c-290b1c14d020#card` | Etched Oracle | 1 |
| `7f5124a4-9ffd-475a-8b77-3ff59a9871a1#card` | Pursuit of Knowledge | 4 |
| `7f5bebfd-faf3-485f-b906-aa96d621d499#card` | Family's Favor | 3 |
| `7f956a21-854e-41cc-96f9-ad774e69fa8e#card` | Wiitigo | 90 |
| `82f3faa8-39fa-450b-843f-d60a4c36d8f7#card` | Mikaeus, the Lunarch | 10 |
| `8443b8c2-d0ab-4fb1-ad35-7e5e4cb345b6#card` | Baku Altar | 4 |
| `8520ee67-439b-4f15-838b-420bacbb8b13#card` | Experiment One | 1 |
| `85488433-3c23-4a29-b063-53dba71f26c4#card` | Furnace Strider | 10 |
| `85d025d9-158d-4bf1-b3b2-361f3fe59663#card` | Magmatic Sprinter | 12 |
| `85eb890e-a5e1-48f8-b813-e8424d18b28a#card` | Migloz, Maze Crusher | 20 |
| `87f746a3-ca6b-404e-99cc-290a0b23f9c4#card` | Myojin of Seeing Winds | 896 |
| `8859a692-d107-4de2-9110-4681fc2761c7#card` | Glissa Sunslayer | 6 |
| `8940f80b-9c8d-48c0-97c0-c18d77797911#card` | Vitaspore Thallid | 2 |
| `8a179d68-c9e6-4f46-99bf-11b2a2708990#card` | Trigon of Thought | 5 |
| `8b3c4a6c-44ac-4718-ac1e-9d503523e89d#card` | Shinewend | 5 |
| `8cac286c-1a17-4ee1-9858-d7a2f8c09037#card` | Sawblade Scamp | 2 |
| `8e1d3722-9bd1-4ab7-b4a5-9265fca3aba9#card` | Ancient Hydra | 1 |
| `8e3eafd7-5823-4cc1-bdb8-c456580e395a#card` | Myojin of Life's Web | 672 |
| `8e96c09f-c064-4dd8-85ee-0f48d86ab670#card` | Golem Foundry | 2 |
| `8fe687aa-d134-4a29-9c93-64086c4a1977#card` | Morselhoarder | 5 |
| `90ec956f-b6de-4214-9b5a-895280e542c1#card` | Spike Rogue | 5 |
| `91303465-4b98-4ecc-82f4-cfd2f9a0f8a7#card` | Gemstone Array | 1 |
| `9148f2cb-5594-463c-a21b-a67706ec46d1#card` | Flitterwing Nuisance | 25 |
| `9188031e-1c24-4981-a3d2-461a2bf917b6#card` | Archfiend of the Dross | 10 |
| `91fe1075-e170-4575-ae28-8d692f051637#card` | Elvish Farmer | 1 |
| `922c6e1b-abcf-40e0-847e-9ebb7d0b6fd4#card` | Grimoire of the Dead | 222 |
| `9398fbb4-a2b9-4ae4-9e0f-2553eb570501#card` | Salt Road Quartermasters | 5 |
| `942216f7-6803-4ab3-9239-463002590f6f#card` | Infused Arrows | 2 |
| `94daa0c2-4790-4193-aaa8-ba492e8c8fe7#card` | Conductor of Cacophony | 5 |
| `9519a4ea-c4a0-4127-b06a-aac9ffaf7a32#card` | Arwen, Mortal Queen | 1360 |
| `97febf6b-302c-4c41-8838-eef7c7353bdc#card` | Glen Elendra Guardian | 5 |
| `9ade7c7b-7d28-4d79-94b8-20c9c5b8369a#card` | Ferropede | 3 |
| `9bb8c54b-1228-4b7c-8651-52cb5b0f6e72#card` | Phantom Centaur | 5 |
| `9c0a8412-af9f-4a22-b462-e9b2d9bd04c6#card` | Medicine Runner | 1 |
| `9e006a4b-8dde-4416-8cb4-8401562d0fd5#card` | Tendo Ice Bridge | 5 |
| `9fceca3c-fc10-4e46-a727-122aafcef349#card` | Cruel Sadist | 1 |
| `a20bb8b3-b724-4fec-b179-ca9e6adc4718#card` | Baton of Courage | 2 |
| `a34d1845-e02f-4795-b445-d73366c2baea#card` | Spike Soldier | 10 |
| `a597d2b7-484b-4cfd-89a6-d166cb1a3420#card` | Timberline Ridge | 44 |
| `a5d803bc-7fd4-4570-8cd6-cb138727a66e#card` | Bewitching Leechcraft | 12 |
| `a7c5318f-e579-447e-9109-7d2e4b3d2c17#card` | Soul Diviner | 2 |
| `a7ee67a6-0ac7-4079-a947-4e52d09ace8e#card` | Axiom Engraver | 5 |
| `a866fb67-6614-45e6-928e-b4b59d95335f#card` | Phantom Wurm | 5 |
| `a98ea782-1c49-4207-a130-41329087fdf6#card` | Channeler Initiate | 1 |
| `a9d72f78-2ab5-4e2e-ab7b-ef875e0a0609#card` | Lost Jitte | 3 |
| `ab4cf0ba-617f-4ef6-a831-eb3b41b4ab34#card` | Heartmender | 1 |
| `aba1d01e-c340-47a3-8432-9d8c4a5cdfe0#card` | Myojin of Infinite Rage | 224 |
| `ac5c2069-0706-4b85-a811-a522db016e3e#card` | Magmaroth | 2 |
| `af0bf083-0624-4ccd-b282-a35937e925fe#card` | Mycologist | 1 |
| `b0d60856-80f9-4c64-9768-28a0a690ecb1#card` | Twilight Drover | 7 |
| `b1f441ce-7619-4ab5-ab36-724ed0a76728#card` | Bloodcrazed Hoplite | 4 |
| `b2354092-7a4c-4e69-9c0a-0a2e146dc00c#card` | Watcher of Hours | 1 |
| `b24d1540-ed8b-4df3-abfe-1b8492a58388#card` | Pestilent Haze | 2 |
| `b47d91a1-f2ac-4604-95a7-c247ac2fab18#card` | Duchess, Wayward Tavernkeep | 3 |
| `b66dd751-fef7-4310-9c09-8d1b1fc78fef#card` | Meldweb Strider | 20 |
| `b7326fd9-7648-4db3-b506-61a7de8a4094#card` | Thallid Devourer | 2 |
| `b7a68899-c0d3-49e0-854b-19268ae9b89d#card` | Vivid Grove | 5 |
| `b7b738e1-298d-489d-9939-efe1c983f0ae#card` | Spike Breeder | 5 |
| `b7d231eb-255d-4505-89a9-0c6824622209#card` | Barkhide Troll | 10 |
| `b8c748ed-7ba2-4f92-a9de-5080382c2fc2#card` | Quillspike | 2 |
| `ba48a1a6-a53f-4e7e-881a-472637a3a007#card` | Hexavus | 5 |
| `bb23221a-f1d0-421d-9769-132e2b26b804#card` | Waxmane Baku | 4 |
| `bb4ab425-3da7-455d-b217-3d585d476113#card` | Workhorse | 5 |
| `bcbb318d-0ebc-48a2-afd7-69cf5351893d#card` | Glistener Seer | 5 |
| `bcded242-6e54-416f-bdc8-093211c50e3f#card` | Phantom Flock | 5 |
| `bd4df73c-bbd5-47fd-a961-19f3a3c59381#card` | Lavabrink Floodgates | 12 |
| `be6155de-c5b2-415c-ad83-142f9926462a#card` | Khalni Heart Expedition | 28 |
| `bf65a7a7-a590-41eb-9844-184e5d63e32a#card` | Phantom Nomad | 5 |
| `bf7c1fa8-0460-4be5-b00d-72c0960c4149#card` | Deity of Scars | 5 |
| `bfb75b28-0e51-4829-b104-f1679e63c126#card` | Myojin of Blooming Dawn | 704 |
| `bfec4d0a-3792-4bc3-bae1-e639da5bb9a6#card` | Land Cap | 44 |
| `c16d79d9-c159-421f-90a2-13313a1de5f6#card` | Monoskelion | 5 |
| `c2c7a9c2-28f8-4cf0-a042-420f6a4ba8bd#card` | Tamiyo's Immobilizer | 5 |
| `c2f42c67-ff88-4248-bfa5-79a18c6473d5#card` | Darigaaz Reincarnated | 560 |
| `c2f6ddc2-86bc-43da-9bdd-0c2af79c62fd#card` | Ammit Eternal | 3 |
| `c2fbdc80-ad5d-4fe0-a860-3e4f452e6302#card` | Arcane Spyglass | 2 |
| `c56e6715-80bc-4f9d-86ef-7c31b1ef0a17#card` | Decimator Beetle | 6 |
| `c5d27469-96e1-44ec-a887-2fc26f9e10d3#card` | Woolly Razorback | 40 |
| `c5f92560-dc82-4bd2-ae6b-4c3942d458b7#card` | Quest for the Gravelord | 1 |
| `c6424a3e-dbc7-403f-867f-a81cccd17ab8#card` | Arcbound Reclaimer | 2 |
| `c762361a-0b72-4db5-8b6f-715ca42d5067#card` | Quest for the Gemblades | 3 |
| `c7b088d6-031b-4e50-aa07-69f4fd26df79#card` | Myojin of Cleansing Fire | 224 |
| `cb0eb84d-b41b-4147-aa7c-089b7fabd835#card` | Undergrowth Champion | 72 |
| `cbbc530f-e51b-423e-b219-23bb580fd834#card` | Druids' Repository | 1 |
| `cc9523da-f48e-49ab-9989-a4031dafb426#card` | Heirloom Auntie | 15 |
| `cff3fba2-9c5d-4e98-9ff0-0cc0b3bc8dbe#card` | Iceberg | 5 |
| `d0653734-9eb7-48c6-9042-65ffdd50fd2a#card` | Woodripper | 1 |
| `d0dd582a-ac35-4a46-beca-6304fdd2d358#card` | Etched Monstrosity | 5 |
| `d52ecb32-26ed-484b-b1a7-ee47b66f0f94#card` | Serum Sovereign | 4 |
| `d5bfb740-8ab2-4160-a8f0-5b70b63c2f3a#card` | Descendant of Masumaro | 806 |
| `d6ded5b0-1309-4f1e-be10-d84ca0a5dd20#card` | Charforger | 24 |
| `d7fcf43a-191f-4f44-aade-0cf89079a0c5#card` | Spike Worker | 5 |
| `d8e2efe0-33a4-4303-9e83-ac42ea5df8cb#card` | Vivid Crag | 5 |
| `da38bfdf-b4c8-4ffe-ad53-b42c317cae5f#card` | Magmasaur | 480 |
| `dbfa5409-5706-4de8-8152-e74b8d93910a#card` | Bolrac-Clan Crusher | 1 |
| `dd6d15c0-c294-42c9-a6cf-57b0b5d40823#card` | Spike Drone | 5 |
| `dd840d84-bc4a-4420-b8ac-99093a5aa253#card` | Sporesower Thallid | 1 |
| `de6b26e4-8700-490f-9df2-dac430463274#card` | Sunstar Chaplain | 1 |
| `dee99df5-628f-4a4e-a203-4dfddc927373#card` | Vivid Meadow | 5 |
| `e1a64bba-eaff-4f59-b85d-7df0b13e2e6b#card` | Clockwork Vorrac | 15 |
| `e1b74ef7-ab6b-4d40-8087-3f3a448b7e0a#card` | Crovax the Cursed | 160 |
| `e3390776-66cc-4e0a-a034-701793b05db8#card` | Shed Weakness | 2 |
| `e38e3723-05f5-4a51-8364-1cda19f9cc49#card` | Steelbane Hydra | 5 |
| `e579a72f-4933-40fe-9e57-96f8d65370bc#card` | Ghave, Guru of Spores | 5 |
| `e6cd9203-e4d3-4d9f-b59f-4e454fc5a477#card` | Unbreathing Horde | 255 |
| `e7441107-57b1-4e88-a9d8-3eb7c338ef07#card` | Mindless Automaton | 5 |
| `e76a1371-76c8-46dd-ba43-87fb0ab940d4#card` | Pallid Mycoderm | 4 |
| `e868028f-4c6f-43d1-9cc6-6f3ce2df2ec3#card` | Stingmoggie | 5 |
| `e958f7bf-3011-4175-b467-ca225a9b7ad2#card` | Triskelavus | 40 |
| `e9e18a5a-d265-456f-a049-fa4b7e223263#card` | Korozda Gorgon | 2 |
| `e9ece3c6-81c2-441a-acce-c109aaa9026b#card` | Render Inert | 1 |
| `ea31c9a4-af4f-4cab-b53c-77ef4241d4a6#face:1` | Scarmaker | 2 |
| `ea53adbe-3f9a-4847-87c7-723ac2789918#card` | Mirrodin's Core | 1 |
| `ea7b0704-7715-4f8a-b32f-d2d2f3b36054#card` | Spawning Pit | 1 |
| `eab5eb29-bed6-456b-8929-5b9b820aa5a3#card` | Voracious Hatchling | 20 |
| `eb414a8f-566f-4635-80a1-77b2e18ac8dc#card` | Runaway Steam-Kin | 4 |
| `ebdf26a4-77ee-4635-8d52-926bdca623f7#card` | Lux Cannon | 1 |
| `ed0fc294-de9b-4c64-b97d-e3a41fd2f46a#card` | O'aka, Traveling Merchant | 2 |
| `edac6c23-cff5-4ee2-9227-756b1dc6ab0c#card` | Firemind's Research | 4 |
| `ee152ffa-9c39-48ed-9aca-e53c8ebb0231#card` | Defiant Greatmaw | 4 |
| `ee9b606a-a1fe-43c8-84d0-9bf7b012cc75#card` | Leatherhead, Swamp Stalker | 120 |
| `f1bd0d32-7513-4d1d-94aa-d06101a940c0#card` | Etched Slith | 3 |
| `f1feac23-8ba3-440c-828c-590f88008c31#card` | Necrogen Censer | 5 |
| `f27fb53f-a983-410a-821b-e48cd8c01f2e#card` | Noosegraf Mob | 5 |
| `f2c10591-4c0c-4286-af91-e8d8d10a4c9d#face:0` | Thing in the Ice | 40 |
| `f2ecb354-8f79-4c39-989e-7aa37ec75154#card` | Bomb Squad | 96 |
| `f328dfb1-0737-478b-bd40-dfcb9a35e5a5#card` | Brigone, Soldier of Meletis | 4 |
| `f352ec62-bf09-45f2-b5b7-75931177b15f#card` | Spore Flower | 10 |
| `f53da056-97be-41ad-8cfc-0d92e17dcd7b#card` | Sage of Fables | 21 |
| `f5aeed95-24f8-4f28-9e7c-1e3502d013cd#card` | Sunspring Expedition | 2 |
| `f732873e-0b34-4ccc-8b4c-9f02463ba57b#card` | Exemplar of Strength | 1 |
| `f73f8eca-55fe-418f-a244-29ef82cfc47c#card` | Petalmane Baku | 4 |
| `f7536d94-4062-4ccc-a086-1c47954b24c7#card` | Anthroplasm | 10 |
| `f91407fe-3c3b-4ffc-81de-cb8c5b0fe526#card` | Chisei, Heart of Oceans | 3 |
| `fa519a46-11ab-4ab8-9286-904ce2746c03#card` | Forgehammer Centurion | 18 |
| `fc338ba2-c3a4-45d6-895f-af7f72bf7a94#card` | Ior Ruin Expedition | 2 |
| `fcb2a95b-cef0-4b29-8b2b-538f380aff2b#card` | Vampire Hexmage | 1 |
| `fcc9f3bc-0bc3-4189-a4d0-9698594b99a8#card` | Enchanted River's Grasp | 24 |
| `fef502af-6e79-4c55-a86a-b45adb3fc64a#card` | Plague Boiler | 4 |
| `ffeb792a-6df0-46c1-badc-4deb857f7d48#card` | Freyalise's Winds | 24 |
