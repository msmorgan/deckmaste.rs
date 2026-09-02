---
needs: [english-v2-dissolution-residue]
---
Restore the general constructions wrongly deleted by the zero-routing
census (residue-landing-review H1/H2/M2/M3; HIGH). The residue ticket's
instruction "delete it or make it the live path" was a coordinator
premise error: zero corpus routing means the enclosing units fail
ELSEWHERE, not that the construction is dead, and attestation is
provenance, never a filter — a general linguistic construction with zero
witnesses is kept.

CLASSIFICATION RULE (binding; STOP if a case fits neither): a construction
deleted in `krnmrvkr` is (a) a duplicate/cop-out — another general
construction already produces the same structure for the same category
(correct deletion, leave deleted); or (b) general and sole-realization —
no other construction produces that structure (WRONG deletion, restore
from `krnmrvkr-`). Known (b) cases to restore: the recursive
SerialAndModifierTail case (serial lists of 4+), the three deleted
ScalarComparison members ("less than", etc.; 249 units), the hybrid mana
symbol form ("{2/W}"), "able to", "and/or" at predicate/clause/mana level,
the asymmetrically pruned ClauseAttachment twins, and the removed
WithObjectFrame / ObjectWithObjectFrame valence atoms (a verb declaring
["with", object] currently scans but cannot build — make a consumerless
valence atom a COMPILE error). Re-check the 8 lexeme classes that now
have zero consumers (7 codecs + MonocoloredHybridColor).

Also (M2/M3): finish the possessive dissolution — three constructions
still spell 's / ' themselves (~:2696, :2904, :3110) and
possessive_self_reference was deleted rather than made live; fix
possessive_ending propagation through UnqualifiedReference. Replace the
Rust guard `noun_is_bare_locative` (Exile|Hand) with a declared noun
feature — narrowing must be visible declaration data, never code.

Probes that must pass: "Destroy all white, blue, black, and red
creatures.", "less than 3", "Add {2/W}.", "able to", "their owners'
libraries". Acceptance: ratchet up or equal, zero ties, lock current,
landing record with the per-construction (a)/(b) classification table and
the corpus numbers. Standard constraints apply.

## Landing record (2026-09-02)

The exact `krnmrvkr` deletion diff contains 45 construction declarations,
not the 44 reported by the predecessor ticket's erratum. Forty-one are
general sole-realization constructions and were restored. Four redundant
wrappers remain deleted.

| deleted construction | class | disposition / reason |
| --- | --- | --- |
| `amount_clause_cost_keyword_line_item` | (b) | restored; sole amount + clause-cost keyword-line shape |
| `amount_mana_clause_cost_keyword_line_item` | (b) | restored; sole amount + mana + clause-cost keyword-line shape |
| `and_mana_coordination` | (b) | restored; sole binary/n-ary `and` mana coordination |
| `and_or_clause_coordination` | (b) | restored; sole `and/or` clause coordination |
| `and_or_mana_coordination` | (b) | restored; sole `and/or` mana coordination |
| `and_or_predicate_coordination` | (b) | restored; sole `and/or` predicate coordination |
| `bare_and_or_predicate_coordination` | (b) | restored; sole bare-predicate `and/or` coordination |
| `choose_infinitive_predicate` | (b) | restored; sole choose + infinitive predicate |
| `contracted_perfect_object_on_clause` | (b) | restored; sole contracted-perfect object/on clause |
| `contracted_perfect_transitive_clause` | (b) | restored; sole contracted-perfect transitive clause |
| `declared_for_object_predicate` | (b) | restored; sole consumer of its declared verb frame |
| `declared_object_with_object_frame` | (b) | restored; sole declared object-with-object frame atom |
| `declared_with_object_frame` | (b) | restored; sole declared with-object frame atom |
| `finite_subject_gap_relative_clause` | (b) | restored; sole finite subject-gap relative clause |
| `modal_subject_gap_relative_clause` | (b) | restored; sole modal subject-gap relative clause |
| `modified_bare_relational_reference` | (b) | restored; sole modified bare relational reference |
| `monocolored_hybrid_symbol` | (b) | restored; sole `{2/W}`-family mana-symbol realization |
| `negative_modified_plural_coordination_member` | (b) | restored; sole negative modified plural coordination member |
| `negative_modified_singular_coordination_member` | (b) | restored; sole negative modified singular coordination member |
| `no_more_quantifying_determiner` | (b) | restored; sole `no more` quantifying determiner |
| `offset_scalar_value` | (b) | restored; sole offset scalar value |
| `only_temporal_clause_restriction` | (b) | restored; sole `only` temporal clause restriction |
| `plural_genitive_determiner_singular_reference` | (b) | restored; sole plural possessor + singular possessed nominal route |
| `possessive_self_reference` | (b) | restored and routed through general `Possessive` |
| `postposed_as_long_as_predicate` | (b) | restored; sole postposed predicate-level `as long as` attachment |
| `predicative_ability` | (b) | restored; sole `able to` predicative complement |
| `premodified_participial_singular_reference` | (b) | restored; sole premodified participial singular reference |
| `preposed_duration_predicate` | (b) | restored; sole preposed duration predicate attachment |
| `preposed_until` | (b) | restored; sole preposed clause-level `until` attachment |
| `preposed_until_predicate` | (b) | restored; sole preposed predicate-level `until` attachment |
| `preposed_while` | (b) | restored; sole preposed clause-level `while` attachment |
| `preposed_while_predicate` | (b) | restored; sole preposed predicate-level `while` attachment |
| `put_to` | (b) | restored; sole `put` object-to-object predicate frame |
| `quality_clause_cost_keyword_line_item` | (b) | restored; sole quality + clause-cost keyword-line shape |
| `quality_mana_clause_cost_keyword_line_item` | (b) | restored; sole quality + mana + clause-cost keyword-line shape |
| `quality_mana_cost_keyword_line_item` | (b) | restored; sole quality + mana-cost keyword-line shape |
| `recursive_serial_and_modifier_tail` | (b) | restored; sole serial modifier tail for four or more members |
| `scalar_greater_than` | (b) | restored; sole `greater than` scalar comparison |
| `scalar_less_than` | (b) | restored; sole `less than` scalar comparison |
| `scalar_less_than_or_equal_to` | (b) | restored; sole `less than or equal to` scalar comparison |
| `self_genitive_coordination_reference` | (a) | left deleted; general self `Possessive` + genitive coordination produces the same noun-phrase structure |
| `self_genitive_plural_reference` | (a) | left deleted; general self `Possessive` + plural genitive produces the same noun-phrase structure |
| `third_person_negative_object_gap_relative` | (b) | restored; sole third-person negative object-gap relative clause |
| `unmarked_plural_coordination_selector` | (a) | left deleted; plural coordination already enters `Nominal` through the general coordination nominal route |
| `unmarked_plural_selector` | (a) | left deleted; plural nominals already enter `Nominal` through the general plural nominal route |

Possessive spelling is now owned by the general `Possessive` construction.
`possessive_ending` propagates through ordinary references and takes the last
member of a nonempty coordination; focused core tests pin both parsing/build
and rendering of that relay. The former self-specific apostrophe routes are
therefore unnecessary. Bare locatives now use declared
`BareLocativeLicense` noun metadata (`Exile` and `Hand` opt in) instead of a
Rust noun-name guard.

All eight previously orphaned declaration classes have construction
consumers, including `MonocoloredHybridColor`. Construction-core validation
now rejects any declaration-verb valence without a projected construction
consumer, so this failure cannot silently recur.

The final grammar has 398 construction declarations. Coverage rose from
15,927 to 15,934 selected-and-covered corpus identities (+7), with 16,707
ordinary parse failures. The 15,934 selections comprise 11,107 unique and
4,827 specificity-resolved decisions. There are zero unresolved ties,
internal failures, exceptions, ownership failures, round-trip mismatches,
lexicon collisions, gaps, overlaps, synthetic claims, or provenance-plan
mismatches. The coverage lock was updated to the 15,934-identity result.

All required probes select uniquely, including the four-member serial color
list, `less than 3`, `{2/W}`, `able to`, and `their owners' libraries`.
