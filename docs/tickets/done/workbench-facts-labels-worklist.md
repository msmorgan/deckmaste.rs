---
needs: []
---
**Make `cargo xtask facts labels` green by closing the stub/table label gaps
and re-scoping the direction the check cannot mean.** Residue of
`workbench-facts-from-ron` (2026-09-04), which landed the generator
(`FactsGen.idr` from the keyword-ability stubs, per-row shape lists, named
membership witnesses) and the two-way label check, which exits 1 today:

- 102 keyword-ability stubs have no `keywordFacts` row (rowless keywords are
  refused by `KnownKeyword` until rowed); 3 rows have no stub
  (BandsWithOther, Multikicker, PartnerWith).
- `actFacts`: 17 rows have no stub — the wider turn-and-game deed vocabulary
  (draw, untap, …) that the keyword-action stubs never cover; that direction
  of the check needs a scope rule (a deed row is not obliged to have a stub),
  not silencing.
- 42 counter rows have no stub; designations are not covered because stub
  names are not the Idris constructor names.
- The gate columns (`bodied`, `paidCost`, `regime`) stay hand-kept until the
  stub schema (`meta/KeywordAbility.ron`) carries them.

Size: M. Done when: `cargo xtask facts labels` exits 0 under a recorded scope
rule per table; every rowed keyword has a stub or a recorded reason; build at
its module count. Standard constraints apply.

## As landed

**Keyword abilities: rowed, both directions clean.** `overlay()` in
`crates/xtask/src/facts.rs` gained 100 rows — every keyword-ability stub that
had none except `Prototype` — so `keywordFacts` is 97 → 197 rows and the
generated `FactsGen.idr` is regenerated from the stubs. Each row's parameter
shapes still come from the stub's `params`; the gate columns were authored
from the keyword's own CR entry under four rules, printed above and recorded
here:

- `bodied` where the entry defines **one** triggered ability with a quoted
  expansion (afflict, battle cry, persist, …; 33 new rows). A keyword the
  entry splits into a static *and* a triggered ability (graft, modular,
  enlist, ravenous, soulbond, champion, fading, vanishing) is not bodied, on
  the decayed precedent [CR#702.147a].
- `paidCost` where the CR names a "[keyword] cost" that is paid [CR#702.1a] —
  exactly the rows whose stub signature carries a `Cost`, plus conspire, whose
  expansion reads "if its conspire cost was paid" with no cost parameter
  [CR#702.78a] (18 new rows).
- `regime` `Just AtCasting` where the entry's ability functions while the
  spell is on the stack, or is a "when you cast this spell" trigger (16 new
  rows); `Just AtResolution` where the keyword changes the damage its source
  deals [CR#120.3] — wither alone [CR#702.80a], the deathtouch/lifelink/infect
  family. Extort and increment take `AtCasting` on the prowess precedent
  [CR#702.108a], because `Effect.keywordBodyFits` pairs a `SpellCast` body
  with that regime.
- `onPermanentCard := False` only where the entry itself names instants and
  sorceries or split-card halves — aftermath [CR#702.127a], cipher
  [CR#702.99a], jump-start [CR#702.133a], rebound [CR#702.88a]; refusing a
  card class needs the rule, not the corpus. `onSpellCard := True` where the
  entry places the ability on a spell or the printed corpus attests the
  keyword on an instant or sorcery card (24 new rows).
- No new row is `counterEligible`: the keyword-counter list [CR#122.1b] is
  exactly the fifteen rows already eligible.

**`Prototype` is the one stub left unrowed, under a recorded reason.** Its
`[Cost, Power, Toughness]` signature wants a `KeywordParamShape` carrying a
second power/toughness pair beside the cost [CR#702.160a] — a new sort, which
the ticket routes away from a row.

**The three rowless-stub rows keep their rows under recorded reasons**, not
new stubs: multikicker is a variant of kicker [CR#702.33c], "bands with other"
a special form of banding [CR#702.22b], and "partner with [name]" one of the
partner abilities [CR#702.124j]. Each is a CR variant of a keyword whose own
stub exists, so the RON vocabulary spells it through that stub rather than
through a macro of its own.

**`actFacts`: the scope rule, and the 47 stubs it did not excuse.** The
row-without-a-stub direction is now informational and printed as such: a deed
row is not obliged to have a stub, because `actFacts` also carries the
turn-and-game deed vocabulary (attack, block, draw, …) which is wider than
[CR#701]. The other direction still binds, so the 47 keyword-action stubs with
no row got one — every [CR#701] section from abandon to recruit — each at
every column's default, on the `Venture Into The Dungeon` precedent. `actFacts`
is 40 → 87 rows.

**Counter kinds: a scope rule, no row change.** Every counter-kind stub
already had a row; the reverse direction is now informational, because
`counterFacts` carries the counter kinds the CR names [CR#122.1] and the
printed ones the corpus attests, which is wider than the RON counter-kind
macros.

**Designations: the mapping is meaningful, so it is implemented.** `facts
labels` gained a fourth table. Both directions bind through a stub-name →
`Designation` constructor mapping — `Commander → CommanderD`, `Initiative →
TheInitiative`, `DayNight → Day, Night`; every other stub name is the
constructor name. All 16 constructors are covered by a stub. Four stubs have
no constructor and carry recorded reasons: `Harnessed` [CR#701.64b], `Level`
[CR#716.2b], `Sector` [CR#702.158b] and `Solved` [CR#719.3b] — each wants a
`Designation` constructor plus a `designationFacts` row whose scope,
persistence and carrier columns are hand-authored.

**The gate columns stay hand-kept, and the command says so.** `facts labels`
ends with a line naming all seven and the reason:
`plugins/builtin_v2/macros/meta/KeywordAbility.ron` declares no field that
could carry them.

**A recorded reason is itself checked.** `report` refuses a reason whose label
turns out to be present on both sides ("stale recorded reason"), so a scope
rule cannot quietly stop meaning anything once the gap it excuses is closed.

## Landing record

Numbers before → after: `keywordFacts` 97 → 197 rows; `actFacts` 40 → 87 rows;
`counterFacts` 71 label-check rows (unchanged); `Designation` constructors 16
(unchanged); Idris modules 46 → 46. `facts labels` keyword abilities 101 stubs
without a row → 0 (1 recorded reason) and 3 rows without a stub → 0 (3
recorded reasons); keyword actions 47 stubs without a row → 0, 17 rows without
a stub (informational); counter kinds 0 / 42 (informational); designations
newly covered, 0 / 0 with 4 recorded reasons. Coverage lock untouched; no
construction added or retired.

Gates (all foreground):

- `cd idris && ./scripts/build` — `46/46: Building Cards (src/Cards.idr)`,
  exit 0, 0 `Error`/`Warning` lines, `real 1m25.015s` (1m11.6s before this
  round; the keyword table doubled and `keywordWordsDistinct` is quadratic in
  it)
- `cargo xtask facts check` — `…/idris/src/Experimental/FactsGen.idr is up to
  date`
- `cargo xtask facts labels` — exit 0, four tables each with its scope rule
- `cargo test -p xtask` — `test result: ok. 444 passed; 0 failed; 1 ignored`
  (441 before; 3 added)
- `cargo fmt --check` — clean; `cargo clippy -p xtask --all-targets` — 0
  warnings
- `cargo xtask cite check --list-noncompliant` — `0 non-compliant
  citation-looking string(s)`
- `cargo xtask cite check` — `checked 14198 citations against cr.txt
  (eff. 2026-08-07); 0 stale`

Pin probes (foreground, `Experimental.ProbeTmp` typechecked and removed):

- Positive: `KnownKeyword "Wither"`, `KnownAct "Investigate"`,
  `keywordStackRegime "Wither" = Just AtResolution`, `So (keywordBodied
  "Persist")` and `So (paidCostNamed (ByKeyword "Splice"))` all elaborate — the
  new rows and their gate columns are live, not inert.
- Mis-stated once each, all four refuse: `keywordStackRegime "Toxic" = Just
  AtResolution` (`Mismatch between: Just AtResolution and Nothing`), `So
  (keywordBodied "Toxic")` and `So (paidCostNamed (ByKeyword "Persist"))`
  (`Mismatch between: True and False`), and `KnownAct "Zorp"` (`Can't find an
  implementation for isJust (…) = True`).
- The pre-existing unknown-label pins (`badUnknownKeywordClass`,
  `badUnknownCounterLabel`, `badUnknownVerbLabel`) re-elaborated in the clean
  46/46 build.

Assurance counts: restored 0; re-spelled 0; ignored with blockers 0; added 3
(`every_stub_has_a_row_or_a_recorded_reason`,
`recorded_reasons_name_labels_that_are_really_there`,
`the_designation_map_reaches_the_idris_constructors`); removed 0.

Deviations and additions:

- **47 `actFacts` rows, which the ticket's own bullet list did not count.**
  The ticket names only the 17 rows without a stub for that table, but the
  check also failed on 47 stubs without a row, and "exits 0" cannot be reached
  by scoping that direction out without contradicting the ruling ("every stub
  has a row"). The rows are the [CR#701] sections the workbench had not
  reached; each is at every column's default, so the row admits the label and
  asserts nothing about the deed's roles.
- **A fourth table.** `facts labels` now reports designations, which it did not
  before; the ticket made that conditional on a meaningful mapping, and 16 of
  the 19 stub names map one-to-one or by rename.
- **Recorded reasons are data with a staleness check**, rather than a
  hard-coded skip list. The extra `stale recorded reason` failure mode and the
  `recorded_reasons_name_labels_that_are_really_there` test are beyond the
  ticket's letter; without them a reason outlives the gap it excuses.
- **`normalize`, `table_labels` and `report` were rewritten, not extended.**
  `report` now takes a `Table` carrying the scope rule, the two exemption
  lists and whether the row→stub direction binds; the old three-argument form
  could not express an informational direction.
- **Extort and increment take `AtCasting`.** Their expansions are "whenever
  you cast a spell" triggers on a permanent, so the regime reads oddly as a
  stack regime; it is what `Effect.keywordBodyFits` requires to admit their
  bodies, and it follows prowess [CR#702.108a], which the previous round
  already rowed that way. The consequence — `Effect.grantSubjectFits` treats
  an `AtCasting` keyword as ungrantable to a battlefield permanent — is
  pre-existing with prowess and is not re-opened here.
- **Column derivations are recorded rules, not per-keyword prose.** The
  overlay carries one nine-line comment naming the four rules and their
  anchor citations rather than a citation per row; a per-row cite would be 100
  citations for one derivation.

STOP taken: none. The nearest call was the 47 `actFacts` rows above: the
ticket's bullet list and its "done when" disagree about that direction, and
the ruling in `docs/decisions/workbench-ron-shaped-and-label-rulings.md` §2
("every stub has a row") settles it in favour of rowing them, so this is a gap
in the ticket's own inventory rather than a contradiction with a recorded
ruling.

Residue (routed as future work, none in scope here):

- `Prototype` wants a `KeywordParamShape` for `[Cost, Power, Toughness]`
  [CR#702.160a].
- `Harnessed`, `Level`, `Sector` and `Solved` want `Designation` constructors
  and `designationFacts` rows.
- The seven gate columns want a home in `meta/KeywordAbility.ron` before they
  can stop being hand-kept.
- The 47 new `actFacts` rows carry default roles; each wants its `agentRole`,
  `patientRole` and destination authored as the deed gets spelled.
