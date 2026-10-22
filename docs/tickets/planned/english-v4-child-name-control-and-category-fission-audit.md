---
needs: [english-v4-unextended-and-closures]
---
# Audit: control flow keyed on child names, and Category fission (Nodules 4 and 5)

**Needs an owner ruling before any implementation.** The owner adopted only
items 1–4 of the round-8 treatment plan (`docs/decisions/english-v4.md` §4.4).
This ticket inventories the audit's Nodules 4 and 5 and proposes a fix for
each finding. It changes no code. Once the owner rules, the ruled fixes become
their own tickets.

It follows `english-v4-unextended-and-closures`, which may delete some of the
name-keyed sites below (`closure.rs`'s `head` test, for one).

## What the audit named

1. **Hidden control flow on child names.** Engine behaviour depends on the
   names authors give children in `english.ron`, and authors rename a head to
   avoid it.
2. **Category fission.** Subcategories (`category: Some(..)`) duplicate an
   existing phrase's Productions to gate a slot, and features tunnel a
   grandchild's values up through a child.

## Evidence

Re-grepped on `qynmxmro`. Line numbers drift.

**Engine sites keyed on a declared child name** (`crates/deckmaste_syntax/src`):

- `grammar.rs:191–193`: "A child named `head` implicitly agrees with the mother
  on every feature both carry". Applied in `compile.rs:1186` (`add`); Valence
  agreement skips `head` at `compile.rs:1500`.
- `step.rs:377–381`: `head` sets the head position, and `head`, `first`,
  `last` and `rest` make the head line, which `head_final`,
  `final_preposition`, `post_adjunct` and coordinator handling follow
  (`step.rs:707–764, 1155`).
- `step.rs:996`: `first` names a first conjunct. `step.rs:1026`: a
  post-dependent host is recognized by every later child being named
  `adjunct`. `step.rs:1158`: `complement` decides `delimited_complement`.
- `reading.rs:671–712` (`attachment_key`): Right Association charges a child by
  its name. `head` is charged per later `adjunct` or `modifier`; `adjunct`,
  `modifier` and `supplement` after a `head` are charged by distance, with
  `supplement` reversed; `last` adds conjunct balance. Also
  `reading.rs:263, 277, 556–558, 1001–1007, 1113`.
- `compile.rs:867` and `closure.rs:146`: a fused head's twin must be followed
  by a child named `head`.

**Head renamings in `english.ron`** that keep a semantic head out of that
behaviour:

- `DistributiveCoordinationDetermined` (`:1101`): the Nominal is `nominal`, with
  `transparent: ["nominal"]` and explicit `Agree`s.
- `TransparentQuantity` (`:1123`) and `TemporalQuantity` (`:1137`): the
  quantificational Nominal is `quantity`. The Noun Phrase takes its Number from
  the oblique, not the head (CGEL, Ch. 5 §3.3, pp. 349–351).
- `PawprintMode` (`:502`): `quantity` beside `head`.

**Category fission:**

- `SubjectInitialClause` (`:121`) restates the Clause Productions
  `SubjectPredicate` (`:540`), `CliticSubjectPredicate` (`:554`),
  `FloatedEach` (`:1019`) and `FloatedAll` (`:1022`) as
  `RelativeSubjectPredicate`, `RelativeCliticSubjectPredicate`,
  `RelativeFloatedEach` and `RelativeFloatedAll` (`:1263–1272`), so that a bare
  relative's Clause has its Subject first (CGEL, Ch. 12 §3.5.5, p. 1055).
  `RelativeAdjunctGap` (`:1308`) is a fifth `SubjectInitialClause` Production.
- `english.ron` declares 61 subcategories with `category: Some(..)`
  (`:115–300`). Some are genuine distributions (`Gerund`, `ToInf`); others may
  be gates. Each needs classifying.
- **Grandchild tunneling.** `ObliqueNumber`, `ObliqueKind`, `ObliqueNounKind`,
  `ObliqueDetermination` and `PrepositionMarking` copy a head's oblique
  Complement's features onto the head's phrase, through the `agree` lists of
  the Noun and Determinative Valences (`:1389`, `:1396`), so a grandparent can
  constrain them. The audit counts six such features; confirm the sixth
  (`ConjunctNumber` and `QuantityNumber` are candidates).

## The audit

For each finding:

1. Say what the name, Category or feature decides, and which Readings or
   Productions depend on it.
2. Classify it:
   - structural and wanted (state it explicitly instead);
   - a gate whose purpose is to suppress a Reading (frozen by §4.4, item 2);
   - or genuine coverage.
3. Propose a fix. Examples: a declared role on the Production instead of a
   reserved child name; one Clause Production with a declared ordering instead
   of a duplicate Category; a declared path to the oblique instead of copied
   features. State what each fix would cost in census and judgments, from a
   measurement where one is cheap.

Write the inventory and proposals into this ticket, then STOP for the owner's
ruling.

## Done when

- Every finding above is classified and has a proposed fix. The owner has the
  inventory.
- The audit changes no code. No face is lost, every judgment holds,
  `cargo xtask english-v4 strata --check` passes, the cheapest tier is
  unchanged, and no `Summary`, `State` or `ProductionDecl` field or
  `closure.rs` pass is added.

Standard constraints apply.

Census delta (measured on `<change>` → this landing): faces read, total Readings and Surplus before and after; lost faces by name, or "none".
Judgment set: the result of running it.
