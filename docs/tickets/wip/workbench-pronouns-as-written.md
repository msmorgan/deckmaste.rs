---
needs: [workbench-attack-agent-role]
---
**Spell pronouns and demonstratives as printed: `It`/`Them`, `That`/`Those`.**
Ruling 2026-09-04, correcting a misreading of "one macro per lemma, no
agreement or arity pairs": that rule forbids macro pairs that encode forms of
ONE printed word (verb agreement, arities); it never meant that two distinct
printed words share a macro with a `Plurality` argument. References are
written as on the card, so `Macros.It ManyOf` (73 sites) is `Macros.Them`,
`Macros.That w ManyOf` (20 sites) is `Macros.Those w`, and every other macro
whose `Plurality` argument stands for a distinct printed word (`ItVerbed`,
and any the sweep finds) splits the same way; a macro whose plurality is not
a separate printed word (`They`) stays one. The core `Pro`/`Noun` rows keep
their positional `Plurality`; only the macro surface changes, and the RON
re-emitter maps both spellings onto its pronoun macro with number.

Size: S (mechanical sweep, 357 `It`/`That` sites plus pins). Done when: no
macro in `Macros.idr` takes a `Plurality` for a printed-word split; every
site spells the printed word; build at its module count. Standard
constraints apply, including the RON-shaped constraint.

## As landed

- `Macros.It`/`Macros.Them` split: `It` now takes no `Plurality` (fixed
  `OneOf`); `Them` is the new `ManyOf` twin. 73 `Macros.It ManyOf` call sites
  → `Macros.Them`; every `Macros.It OneOf` call site (and the two internal
  unqualified `It OneOf` uses inside `Macros.idr`) → `Macros.It`.
- `Macros.That`/`Macros.Those` split: `That w` fixed to `OneOf`; `Those w` is
  the new `ManyOf` twin. 45 `Macros.That w ManyOf` sites → `Macros.Those w`;
  every `Macros.That w OneOf` site (plus three internal unqualified uses in
  `Macros.idr`) → `Macros.That w`.
- `Macros.ItVerbed`/`Macros.ThemVerbed` split: `ItVerbed v` fixed to `OneOf`;
  `ThemVerbed v` is the new `ManyOf` twin. 5 `Macros.ItVerbed v ManyOf` sites
  → `Macros.ThemVerbed v`; every `Macros.ItVerbed v OneOf` site (plus one
  internal unqualified use) → `Macros.ItVerbed v`.
- `Macros.They` left unchanged (no `Plurality` argument — one printed word).
- Swept every other `Plurality`-carrying declaration in `Macros.idr`
  (`TheVerbed`, `lookedGroup`/`lookedParted`/`lookedSpilled`,
  `agentSelfOrOwn`/`OwnRefOk`) and left them positional: none of them fixes a
  printed word by macro name — their plurality is forwarded to an
  arbitrary noun word or player reference, the same way a core `Pro`/`Noun`
  row stays positional. Nothing undone.
- Core `Pro`/`Noun` constructors: untouched, per the ruling.
- Pin twins in every `Proofs*.idr` module: call spelling updated in place
  (rename only, same obligation, same message); `scripts/check-pin-twins`
  passes; the full build re-validates every pin's impossibility, so no pin
  went vacuous.
- RON re-emitter mapping (Rust side): out of scope for this Idris-only
  ticket; not touched.

## Landing record

Before: 4 `Plurality`-parameterised pronoun/demonstrative macros in
`Macros.idr` (`It`, `That`, `ItVerbed`, plus `They` with no parameter).
After: 8 fixed-plurality macros (`It`/`Them`, `That`/`Those`,
`ItVerbed`/`ThemVerbed`), `They` unchanged. 652 call-site rewrites total (646
via a scripted regex sweep over `idris/src/Experimental/`, covering 30
files, plus 6 manual fixups for unqualified internal calls inside
`Macros.idr` itself). Final tallies: 73 `Macros.Them`, 45 `Macros.Those`, 5
`Macros.ThemVerbed`; the rest of the former 357+ sites spell `Macros.It`,
`Macros.That w`, or `Macros.ItVerbed v`.

Gates:
- `cd idris && ./scripts/build`: 46/46 modules, no Warning lines (unchanged
  module count from before the round).
- `python3 idris/scripts/check-pin-twins`: exit 0.
- `cargo xtask cite check --list-noncompliant`: `0 non-compliant
  citation-looking string(s)`.
- `cargo xtask cite check`: `checked 14436 citations against cr.txt (eff.
  2026-08-07); 0 stale`.
- `cargo xtask cite audit --diff` (piped `jj --no-pager diff --git`):
  `audited 0 citation site(s) — nothing selected` (no citations touched by
  this round).
- `cargo xtask cite bless`: not run — no new citation was introduced.

Assurance counts: restored 0, re-spelled 0, ignored 0, added 0, removed 0.
This round renames call-site spelling only; no pin's obligation, message, or
subject changed, so none of the five categories apply — every existing pin
still typechecks as the same refusal under its new spelling.

Deviations and additions: none beyond the ticket's letter. No witness, pin,
or core row changed meaning; no new pin or twin was added.

STOPs: none.
