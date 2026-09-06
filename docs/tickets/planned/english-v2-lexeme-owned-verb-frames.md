---
needs: [english-v2-grammar-migration-design, english-v2-grammar-frame-compiler]
---
# Replace lexical predicates, grammatical relations and agreement together

Activate the checked frame family from the compiler ticket. Replace the named
`declaration_verb` tail codecs, core/plugin tail matching and their lexical
predicate consumers together. Include `ObjectEqualityToVerb` versus
`ObjectToEqualityVerb`, `EnterWithCountersVerb`, `OrderedVerb`, `LookAtVerb`,
`ProVerbHead`, `HaveKeywordAbilityVerb` and `GetPowerToughnessVerb`; the final
replacement inventory is the codec declarations present at claim, not an old
count. No compatibility category aliases or per-verb checks.

Use the same NP category for Subject, Object and prepositional Complement.
Remove `Subject`/`Object` constituent-category wrappers and relation-named
pronoun vocabularies. Declare pronoun Case and Person/Number, propagate NP
agreement, and derive Concord Class separately from Finiteness, tense,
Inflectional Form and Voice. Constrain ambiguous lexical forms at their host;
where its host determines a form, record that licensed form rather than the
unrefined scanner set. If two tenses remain grammatically possible, retain
the competing analyses for the normal selection/ambiguity check; do not guess. Update existing relative, document,
coordination, AST-export and diagnostic consumers to the new shared types in
this landing; later tickets add capabilities, not compilation adapters.

Use one copular frame whose predicative Complement is AdjP/NP/PP; delete the
game-partitioned copular complement members. Declare recipient/retained-Object
passives as frame data. Bare temporal NP adjuncts require declared Temporal
features and add no Object to a passive frame. Preserve grammatical pro-verb,
auxiliary, negative and contracted forms through their lexical declarations.
Style-guide evidence: §1 “Write rules instructions, not conversational prose”
and §7 “Types, subtypes, and supertypes as nouns and modifiers”.

Formal correspondence: `JudgeFrameIn`, finite agreement and
`AgreementInteractions`, with `FeatureInteractions` temporal exclusions. Extend
witnesses for multiple declared roles, optional roles and retained Objects
before relying on the extensions. Share the form/voice carriers with later
subordination and extraction; do not manufacture a category per form.

Acceptance: `you cast`, `a player casts`, `players/they cast`, mixed and/or
Subjects, `was/were`, and wrong-case/wrong-agreement exclusions. Preserve reduced
passive `dealt damage this way` versus finite preterite and make remaining
preterite paradigms available without misclassifying either host. Cover both
orders of a multi-Complement frame, `turn <NP> face up/down` through ordinary
phrase structure and declared lexical forms, and transitive `exile/return ... with ... on
...` from the old frame ticket. All eleven passive-temporal identities, the
`Cast this spell next turn.` probe, copular consolidation and preterite residues
in [the register](../../english-grammar-migration-obligations.md) belong here.

Standard constraints apply. Production correspondence and the applicable
[obligations](../../english-grammar-migration-obligations.md) are part of this
ticket; re-spell existing tests by their independently justified outcomes.
