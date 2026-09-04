---
needs: [english-v2-np-postmodifiers]
---
Ruling recorded: `docs/decisions/english-v2-rewrite.md`, "Ruling: adjunct
licences removed; attachment misselection is a recorded class (2026-09-03)".
Low attachment is the INTERIM selection; the underspecified-attachment design
(`english-v2-underspecified-adjunct-attachment`) supersedes it later.

Remove the adjunct-licence dimension from verb valence rows (np-postmodifiers
landing review HIGH). `AdjunctLicensed` / `NonprepositionalAdjunctLicensed`
default every verb to "no adjunct may attach inside my object's reduced or
finite relative", with six verbs opted in. That rejects well-formed Oracle
English (`Draw a card for each card you've exiled this turn.` parse-fails; the
`discarded` variant parses), and it encodes no English fact: it is the
Seedborn Muse `Control` blacklist re-expressed as a whitelist in data, kept to
prefer `Untap all permanents you control during each other player's untap
step` with `during` on Untap rather than on `control`.

Pin (ruled): a temporal/manner/locative adjunct attaches to
any verb clause, matrix or embedded — no per-verb licence — and the recorded
derived-attachment rule (low attachment) selects among the survivors. Seedborn
Muse then selects the `during`-under-`control` reading; the grammar admits
both, and preferring the matrix reading is semantics, which the grammar does
not do. Consequences: delete both licence variants and their valence-row uses
(core_verbs.ron, the three keyword_actions stubs, `environment.rs` defaults,
the compiled-consumer fixture's two derived queries); re-spell
`unlicensed_participial_relatives_leave_adjuncts_on_the_outer_predicate` to
assert the low-attachment reading on the same sentence; add the `exiled` /
`revealed` minimal pair as positives. Coverage must not drop; a genuine tie
surviving low attachment is a STOP. Report the winner-change census against
the PARENT tip (the review found 45 undisclosed winner changes on the
previous landing). Standard constraints apply.

Ruled against: keeping a licence with an inverted default (opt-out by class);
no English fact stands behind an opt-out.
