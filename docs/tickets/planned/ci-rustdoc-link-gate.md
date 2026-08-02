---
needs: []
---
**Make `cargo doc` usable as a gate: the `[CR#…]` citation format collides with
rustdoc's intra-doc link syntax.** `cargo doc --workspace --no-deps` currently
emits **1601** unresolved-link warnings, of which **~1532** are `[CR#…]`
citations in doc comments being parsed by rustdoc as intra-doc links to a
nonexistent item named `CR`. The repo's mandated citation spelling and
rustdoc's `[Item]` link syntax are the same characters.

**Why this matters — a defect class no gate catches.** A broken rustdoc
intra-doc link is invisible to `cargo clippy` and to the test suite; only
rustdoc reports it, and CI runs neither. Deleting a public type therefore
leaves a silent trail of dead doc links behind a fully green battery. This is
not hypothetical: `core-pay-player-action` deleted `PlayerAction`, `By`,
`MayPay`, `MustPay`, and the `GainLife`/`LoseLife`/`SetLife` family, and three
broken links to `PlayerAction` (`continuous.rs`, `effect.rs` ×2) survived
clippy, the full workspace suite, `idris-check`, `fidelity`, and `cite check`.
They were found only by grepping for them by hand. The reason a real check
could not be run instead is exactly this ticket: **the ~1532 false positives
bury the real ones**, so the warning count carries no signal.

**Constraint.** The bracket format is not negotiable by itself — `CLAUDE.md`
mandates `[CR#704.5g]` / `[CR#601.2g,106.4]` / `[CR#601.2a..601.2b]`, and
`cargo xtask cite check --list-noncompliant` flags both the unbracketed
`CR`-prefixed spelling and a loose bare rule number in prose. (This ticket
deliberately does not spell those two counter-example forms out: doing so trips
the checker, as the first draft of this file discovered.) Whatever fix lands
must leave `cite check`,
`cite check --list-noncompliant`, `cite bless`, and `cite audit --diff` all
working on the new spelling, and must not silently drop citations from the
10625 the checker currently tracks.

**Approaches, roughly in order of preference:**

1. **Backslash-escape the brackets in doc comments** — a `\` before each
   bracket makes rustdoc render literal brackets rather than parse a link.
   Requires teaching the `cite` tooling to accept the escaped form and a
   mechanical sweep over every doc-comment citation. Two traps this file hit
   while being written: the escaped spelling cannot be quoted verbatim in
   prose (the checker reads the trailing backslash as part of the rule number
   and reports MALFORMED), and the rendered output must still read as an
   ordinary bracketed citation.
2. **A narrower rustdoc lint scope** — do NOT reach for a blanket
   `#![allow(rustdoc::broken_intra_doc_links)]`; that suppresses the genuine
   breakage this ticket exists to expose.
3. **A different in-doc-comment delimiter** for citations, with the checker
   taught both spellings. Least attractive — it splits the convention.

**Then triage what is left.** The residual ~69 warnings are a mix of
English-word placeholders inside quoted example sentences (`"unless [who] pays
[cost]"`, `"If [event] would happen, [effect] instead"` — the same
bracket-prose convention) and a genuine tail in unrelated subsystems:
`Fragment`, `Display`, `FaceLayout`, `Self::Adjective`/`Self::Noun`,
`crate::render::template`, `OneShotEffect::Batch`, `DeonticAction::Cast`,
`crate::Selection::That`, `SubtypeRef`, `crate::layer`, `Predicate::r#type`,
`deckmaste_plugin::Strategy`, `VerbName`. The quoted-placeholder cases want the
same escaping treatment as the citations; the rest are real dead links to fix.

**Acceptance:** `cargo doc --workspace --no-deps` reaches a known-zero (or
pinned-baseline) warning count, `cite check` still reports 0 stale and 0
non-compliant over the full citation set, and the doc build joins the gate
battery so that deleting a type can never again leave dead links behind a green
run.

Related: [[ci-idris-gate]] (the other structural gate this repo runs),
[[comment-discipline-sweep]] (doc-comment hygiene at scale),
[[core-pay-player-action]] (the effort that surfaced the three silent breaks).
