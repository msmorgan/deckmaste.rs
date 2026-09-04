---
needs: [english-v2-stage-5-grammar-buildout-13-10]
---
A quoted granted ability that ends its enclosing sentence: decide who owns
the sentence-final period. Today `quoted_ability` wraps a `DocumentBlock`
whose interior owns its own period (style guide: the final period sits
inside the quotes), so `Create a 1/1 red Goblin creature token with "This
token can't block."` dies at end of input — the outer sentence still wants
a terminator. `has "…"` escapes only because the quoted-ability predicate
is sentence-final as a verb phrase. 312 corpus units carry ` with "` (149
`token with "`, 86 `emblem with "`), all uncovered.

This is a per-boundary byte-ownership ruling, the same class as the ADR's
"Trigger → following ASCII space" row: either the quoted block's interior
terminator also discharges the enclosing sentence's terminator, or the
enclosing sentence takes a terminator-less form when its last constituent
is a quoted block. Record the row in the ADR's ownership table, implement
it generally (any nominal position, not the `with` postmodifier alone),
and prove it with `ambiguity --require-resolved` and the byte-exact gates.
Standard constraints apply.

Coordinator amendment (modal landing review F2/F3): no fresh ADR ruling is
needed — `quote_terminated_statement` (constructions.rs ~:4606) already
implements the rule (the quoted interior's period discharges the enclosing
sentence's terminator), narrowed by `require predicate is
QuotedAbilityPredicate`, a construction-naming guard of the same defect
class as `require possessor is Their`. Generalize that construction to any
sentence whose final constituent is a quoted block and delete the guard;
record the ownership row in the ADR as the now-general rule. Sizing: 312
is a string count; ~215 units fail only at the terminator, the rest fail
mid-text for unrelated reasons — expect ~215 returned.

## Landing record

Measured on change `pnvonrnpqtpnrlnsnxsprnmwzzqxqyss` with 16,555 covered
lock identities.

| gate | before | after |
| --- | ---: | ---: |
| selected and covered units | 16,337 | 16,555 |
| ordinary parse failures | 16,304 | 16,086 |
| unique selections | 10,843 | 11,063 |
| specificity-resolved selections | 5,494 | 5,492 |
| unresolved ties | 0 | 0 |
| construction declarations | 384 | 394 |
| coverage-lock identities | 16,337 | 16,555 |

- Coverage and lock state: this feature's refreshed-parent lock diff is
  +154/-0 rows and no retirement manifest was created or used. Across the
  requested baseline and the refreshed measured tree, the lock moved from
  48,987 lines and SHA-256
  `c73055d0af4c46077cc580bbc0185baaba1188184b3391806dfe22be5cc9069b`
  to 49,205 lines and SHA-256
  `074a2a5ed71e5b131a9d3ade9727cab974215df98b9f913cbd6ac1590a4993c0`.
  The required refresh contributed 64 independently covered rows and nine
  constructions from the concurrent adjunct and keyword-line landings; the
  quoted-terminator feature itself remains +154 covered rows and one
  construction.
  Selected-uncovered units, internal failures, exception resolutions,
  exception uses, round-trip mismatches, ownership failures, gaps, overlaps,
  synthetic claims, and provenance-plan mismatches are all zero.
- Ownership: the ADR now records that the nested quoted sentence owns the
  period, the quoted-ability closing affix owns the closing quote, and the
  interior period also discharges the enclosing sentence terminator. The
  byte-exact witness ends at the quote with no synthetic outer period.
- Assurance: restored 0; re-spelled 1 existing quote-boundary test around the
  generalized AST envelope; ignored with blockers 0; added 0 test functions
  and 3 positive/negative witness shapes inside that test; removed 0. The
  ordinary periodless-sentence reciprocal remains rejected.
- Positive artifacts: `cargo test -p deckmaste_english_v2` passed 409 tests;
  focused quote-boundary tests passed; `cargo clippy -p
  deckmaste_english_v2 --all-targets -- -D warnings` passed; `cargo fmt --all
  -- --check` passed; `cargo xtask english_v2 ambiguity --require-resolved`
  reported the census above with zero unresolved or internal failures;
  `cargo xtask english_v2 coverage --check` passed with exact rendering and
  byte ownership; `cargo xtask cite check` checked 17,795 citations with 0
  stale. The corpus gates emitted their non-failing common-path performance
  warning while host load was above the quiet-host criterion.
- Deviations and additions: added one `quote_terminated_sentences`
  construction so the same boundary rule applies when the quoted sentence is
  final in a multi-sentence ability body; no construction was removed. The
  coordinator's ~215 sizing estimate measured 154 returned corpus units for
  this feature before and after refresh. The remainder fail before the quote
  boundary for unrelated grammar, so none was admitted by weakening the
  checked final-constituent condition. The structural recursion also covers
  attachments, clause and predicate coordination, auxiliaries,
  infinitive/alternative tails, and every final nominal route that can contain
  `QuotedAbility`. The required refresh used jj-sensei harmony to resolve
  three adjacent generated-lock additions by regenerating the lock from the
  merged grammar; the source merge itself had no textual conflict.
- STOPs: none.
