---
needs: [english-v2-clause-level-duration, english-v2-granted-ability-coordination]
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

2026-09-04: inherits the coverage owed for `Target creature gains trample until
end of turn.` from `english-v2-clause-level-duration`; re-measured on that
landing's tree, the witness is still a parse failure and never regressed.

## Landing record

Work started 2026-09-04 20:15:55 -07:00 and stopped before implementation on
feature change `kwruqqtv`.

### STOP

The ticket's STOP-and-report list says that **any coverage loss** is a STOP.
The governing rewrite ADR amendment, "what a landing proves, discloses, and
reports (2026-09-04)", says that only an **unexplained** loss is a defect and
that a decrease with a complete, classified, and routed identity list is
normal. `CLAUDE.md` repeats the ADR rule. These instructions disagree about
whether a fully explained coverage decrease stops this landing.

The task's stop protocol requires a STOP for a ticket-versus-ruling
contradiction even when the executor could choose a resolution. No grammar,
declaration, test, coverage lock, or citation was changed; the frontier and
landing metrics were not measured, and no implementation gates were run after
the contradiction was found.

### PROVE / DISCLOSE / REPORT

- Coverage and selection changes: not measured; no implementation exists.
- Newly covered, no-longer-covered, or selection-changed identities: none
  produced by this change.
- Construction count and provenance inventories: unchanged by this change,
  but not re-measured because of the STOP.
- Assurance: restored 0; re-spelled 0; ignored with blockers 0; added 0;
  removed 0.
- Deviations and additions: none.
- Glossary gaps: none.
- Decision wanted: amend the ticket so its coverage-loss STOP agrees with the
  2026-09-04 ADR and `CLAUDE.md`, or explicitly supersede that ruling for this
  ticket.
