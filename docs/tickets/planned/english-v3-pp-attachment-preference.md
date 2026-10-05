---
needs: []
---
# Audit PP attachment and rank preferred Readings

Use **Avacyn, Guardian Angel** as the primary specimen for examining and
improving PP attachment in `english_v3`. The verified systemic-hosts census
records **33,856 Readings**, a **0.099 s chart** and **150.638 s complete
validation** on tree `ovrpzmymkzkpvquyrypvokmxkzrrzlsl`, lock covered 11,249.
These are historical measurements, not fixed acceptance counts. Reproduce the
specimen on the implementation tree and identify which attachment alternatives
multiply across its two similar prevention abilities.

The [systemic residual audit](english-v3-systemic-residuals.md) already removed
recipient `to`-PP lifting above `dealt`/`be`/`would`: ordinary destination PPs
no longer license independent predicate or Clause Adjuncts. Remaining nominal
and agent attachments need independent analysis. Coordinate with that ticket's
owner; this follow-up does not discharge its broader cutover audit.

## Approach

Keep the distinction between grammatical Admission and optional Preference in
[the accepted decision](../../decisions/english-lexical-analysis.md#chart-admission-and-ambiguity)
and [the glossary](../../contexts/oracle-english/CONTEXT.md). A post-parse step
that ranks already-admitted Readings and chooses a preferred representative
using Construction Cost and attachment heuristics is an acceptable, potentially
more honest solution than forcing a unique parse. Preserve access to every
other grammatical Reading and expose equal-preference alternatives. Existing
Construction Costs currently rank diagnostic samples; inspect and reuse that
machinery rather than mistaking sample retention for a general Preference API.

Parse-stage elimination is also welcome where a syntactic analysis proves a
Reading invalid. Express restrictions through declared lexical properties,
selected frames and general composition constraints. Semantic implausibility,
a higher cost or an unusual attachment is not proof of ungrammaticality. Do not
prune a grammatical Reading to improve Avacyn's count or runtime.

1. Inspect Avacyn's actual Oracle input and representative complete structures.
   Classify PP attachment sites and functions: selected Complement, nominal
   Modifier, agent Complement, predicate/Clause Adjunct, and nominal stacking
   levels. Explain the multiplicative families rather than surveying only the
   cheapest samples. Record intended structures independently of parser output.
2. Apply CGEL diagnostics: head licensing and preposition selection (Ch. 4,
   §1.2, pp. 219–223), coordination/anaphora and stacked nominal modification
   (Ch. 5, §14.2, pp. 446–447), and semantic category/scope (Ch. 8, §1,
   pp. 665–668). Keep attachment level separate from Complement/Modifier
   function. Some structures remain genuinely ambiguous.
3. Compare justified Admission repairs with non-destructive Preference. Candidate
   heuristics include compatibility of PP use with its host, selected-frame
   fit, event versus entity modification, scope, and a weak proximity preference.
   Use grammatical structure and declared features rather than card/word-named
   guards. State which evidence is a grammatical restriction and which merely
   favors a Reading. English remains independent of Game Model semantics.
4. Define how attachment Preference combines with summed Construction Cost:
   priority, weights or ordering, deterministic presentation of ties, and an
   explanation of each preferred result. Do not assume the cheapest structure
   is the intended interpretation. Make the policy inspectable and optional.

## Evidence and completion

Add independent expected-structure tests for Avacyn and contrasting authentic
Oracle constituents, including noun versus verb attachment, passive agents,
selected recipients/destinations, temporal attachment and multiple nominal PP
levels. Retain grammatical alternatives; reject only independently demonstrated
invalid structures. Preference tests must show that ranking changes neither
Admission nor Reading identity and that tied alternatives remain available.

Report Avacyn's before/after complete Reading counts by attachment family,
preferred structures with score explanations, any retired invalid analyses,
chart/forest size, materializations, chart time, full enumeration/validation
cost and preferred-result latency. Ranking alone does not solve the expense of
validating every Reading: investigate whether a preferred-result path can use
the packed forest without eagerly materializing all alternatives. Any partial
search must disclose its bounds and cannot claim a global optimum without
proof; it cannot substitute for the complete census.

Iterate on Avacyn and discriminating witnesses, then verify against the full
supported corpus. Report identity-level coverage changes, retained ambiguity,
Preference effects, both roundtrip laws, lexical ownership/traversal and internal
failures. Record the chosen design and its limits; success requires defensible
attachments and useful Preference, not a predetermined Reading-count reduction
or compulsory uniqueness. Standard constraints apply.
