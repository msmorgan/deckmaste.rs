---
needs: []
---
# The load report counts real holders of class-lifted and projected values

From `english-v4-child-name-control-and-category-fission-audit`, finding G.

`CompiledGrammar::bind`'s single-holder report (`report_holders`,
`crates/deckmaste_syntax/src/compile.rs`, about line 1131) lists 15 values
whose holders it miscounts. The fix is to count the actual holders, not to
exempt anything: a value that really has one holder stays reported.

1. **A value set by a class-lift Production.** FrontAdverb, CentralAdverb,
   PhrasalAdverb and BoundingAdverb each set one `AdjunctPosition` value on a
   phrase whose `head` is a `Class` child. That value's holders are the class's
   members (Front has 5), not the one Production. `End` escapes the report
   today only because ComparativeAdverb also sets it.
2. **A value projected from a Frame Slot Kind.** The Preposition and Noun
   Valence write the frame's Slot Kind as `ComplementKind`, which reaches
   `ObliqueKind` through the `agree` lists. The holders are the lexemes that
   declare such a frame (`VerblessClause`: *if*, *while*), not the one
   derived Valence draft per frame shape. `MeasureNP` (*of*) and
   `QuotedAbility` (*with*) really have one holder, and stay reported.
3. **A value projected from a catalog parameter kind.** FinalParameter
   `SetOffMeasure` and `SetOffKeyword` are held by the keywords whose
   parameter frame ends in that kind. Count those keywords as holders. Each
   has one, so they stay reported unless the census of keywords says
   otherwise.
4. **Linked features.** A value of two linked features (ComplementKind and
   ObliqueKind, NounKind and ObliqueNounKind) is reported once, naming both.

Tests in `crates/deckmaste_syntax/tests/load_report.rs` cover each rule. The
landing lists the load report before and after.

Standard constraints apply.

Census delta (measured on `<change>` → this landing): faces read, total Readings and Surplus before and after; lost faces by name, or "none".
Judgment set: the result of running it.
