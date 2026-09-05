---
needs: [english-v2-grammar-migration-design, english-v2-closed-class-single-owner]
---
> **Migration routing (2026-09-05).** This unclaimed ticket waits on
> `english-v2-grammar-migration-design` under the
> [Lean design decision](../../decisions/english-lean-design-workbench.md).
> That design task must reconcile and repin this ticket before it becomes
> executable. The prior body below preserves examples, regression and
> re-coverage obligations, and proposed mechanisms; its old sequence,
> implementation prescriptions, and coverage-ratchet acceptance do not
> override the new design process or the current landing contract.

Adjective lexeme tier (vocab-surface ruling 2026-09-02; homograph-feature
ledger). Adjectives are content words with no legal home but a vocab:
`AttributiveAdjective` / `PredicativeAdjective` are transitional. Mirror
the noun and verb inventories: ONE adjective inventory with provenance as
data — core-declared ordinary adjectives as the seed (additional, other,
main, maximum, six-sided, …), declaration contributions where a game
identity carries an adjective face (keyword-derived participles like
`equipped`/`enchanted` already ride grammar contributions), grammatical
distinctions (attributive/predicative, gradable, participial) as declared
features consumed by constructions; morphology strictly regular with
per-word attested overrides. The homograph licence moves with the members. The card-type modifier feature
this ticket previously waited on is no longer a ticket — it is the `Card type as
a derived feature on nominal modifiers (A8b)` entry in `../fog.md`, and nothing
here depends on it landing first.
Delete the adjective vocabs once every member has a home; the collision
tripwire then covers adjectives without exemption classes. Hard blocker this ticket owns: three
form literals (`additional`, `next`, `other`) collide with adjective vocab
members today and become unconditional load errors once adjectives are
lexemes — re-route them through the inventory first. Coverage must
not drop; standard constraints apply.

2026-09-04: `english-v2-copular-complement-sum` widened every copular complement
site to the whole `PredicativeComplement` sum, so
`The same is true for creature spells you control…` now has a complement site to
reach but still fails for want of `same` and `true` in the adjective inventory —
this ticket owns them.
