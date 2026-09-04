---
needs: []
---
**Keyword-ability grants are a verb frame, not a bespoke predicate.** `Target
creature gains flying until end of turn.` fails while `Target creature gets
+1/+1 until end of turn.` selects. Sizing from the 2026-09-03 failure census
(measured on change `lkznywvp`, first-failure attribution, re-measure at claim):
1,522 units, the largest family in the corpus — about 1,295 bare keyword plus
duration, 118 with a keyword argument (`protection from black`, `ward {2}`),
99 coordinated (`gains trample and haste`).

Root cause: `have_keyword_ability: VerbPhrase` (`form have_keyword_ability =
verb(head) lex(ability)`) is outside `abstract sum LexicalVerbPhrase`, and the
shared adjunct host `predicate_adjunct_predicate` takes only
`LexicalVerbPhrase`, so no
duration or other adjunct can ever attach; `ability: lex KeywordAbility` admits
neither coordination nor an argument.

Pinned shape: delete the bespoke construction and declare the grant as a new
`LexicalVerbPhrase` member (sibling of `TransitiveLexicalVerbPhrase`) whose object is the
existing `abstract sum GrantedAbility { Keyword: KeywordLineItem, Quoted:
QuotedAbility }`, the same value the nominal grant takes in `reference "with"
granted`. `KeywordLineItem` already carries costed, amounted, qualified and
subject forms, so `ward {2}` and `protection from black` need no new
vocabulary. Coordination goes through the general coordination machinery, never
a per-grant coordination family. Durations and every other adjunct come from
`predicate_adjunct_predicate` alone. Fold `quoted_ability_predicate` into the
same frame if the two verb codecs' tails unify without a new sealed atom;
otherwise leave it and say so in the landing record.

2026-09-04: superseded by ADR "Amendment: Verb Frame and execution-context
vocabulary (2026-09-04)" — was: "abstract sum BaseVerbFrame" / "a new
`BaseVerbFrame` member (sibling of the transitive frame)". The sum is
`LexicalVerbPhrase` and its members are named `*LexicalVerbPhrase`.

Ruled against: a `duration: opt DurationPhrase` slot on the grant (the
adjunct-class landing ruled per-X slots are replaced, not inherited; the
surviving `get_power_toughness` slot is residue and out of scope here);
flipping adjunct-licence flags on the grant verb rows.

STOP-and-report: any `checked by` or `require` naming a keyword lexeme, a verb
identity, or a card; a closed list of duration-taking or argument-taking
keywords; any dominance edge or exception entry added to break the `gains
protection from black` keyword-argument versus generic nominal-PP rivalry; any
coverage loss.

Acceptance: the standard landing record (coverage, selection census, lock
state, construction count, all before/after and stamped with the measured
tree's change id and lock `covered` count; every newly covered identity with
its selected analysis; performance advisory), with the construction count
showing the bespoke construction deleted rather than kept beside the frame, and
both byte-exact laws green with total ownership. Standard constraints apply.
