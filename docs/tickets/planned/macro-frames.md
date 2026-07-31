---
needs: []
design: true
---
**Macro frames: one bidirectional synchronous grammar between core RON and
Oracle English.** Every lexicon entry — macro defs *and* renderable core
constructors (cards are not macros all the way down; Bathe in Dragonfire's
spine is raw `DealDamage`/`Target` with only the `Creature` filter macro) —
gains guarded English **frames**: surface Magic English with typed holes
(`~` self-reference, `<Param(i)>` params; both sigils verified absent from
all 34,690 oracle-text fields), parsed by `deckmaste_english` into
AST-with-holes. Rendering = hole substitution + english's byte-exact
renderer; matching = tree unification recovering RON (a guarded imperative
frame recovers its pre-bound `You` subject); discovery = residual census
over the supported corpus emitting ranked draft frames with exemplars.
Guards are per-entry data because imperativization is lexical-editorial,
not derivable: sentence-initial `Gain N life` 0 vs `You gain N life` 236,
imperative `Draw …` 641 — Revitalize prints both styles on adjacent lines.

Design: `docs/superpowers/specs/2026-07-30-macro-frames-design.md` (local
working doc, with the full evidence base and D1–D13). Supersedes
`english-semantic-ir` (collapse-quotient premise refuted: ≤18.5% ceiling ⇒
the english AST is already near-canonical ⇒ no IR layer); that ticket's wip
workspace stack is to be dropped un-integrated. End-state deletions once
parity is reached: `deckmaste_migrations` bespoke parsers (~16.5k lines),
`deckmaste_cards/src/render/` (~12.7k), `src/template/` (~1.1k). Interim:
`template:` coexists and is *generated* from the frame by lexical
projection (`<Param(i)>` → `${i}`), lint-enforced, so there is never a
second truth.

Round 1 (pilot, in a fresh workspace claiming this ticket): frame field +
english template mode (hole lexing, per-category entry points, feature
variables) + projection lint + pilot unifier over ~10 macros and 2–3
constructor entries; five gates, keystone G4 ground-truth recovery — parse
canon oracle text, unify, and the recovered RON must equal the authored
RON. Zero consumer cutovers; all existing gates stay green. Execution
staffing: an Opus orchestrator juggling Sonnet/Opus subagents per subtask —
complex reasoning throughout warrants it. Four exploration reports
(2026-07-30, `docs/superpowers/research/2026-07-30-macro-frames/`) feed
the plan: english parser architecture, per-verb imperativization table,
macro schema census, hole-constituency audit.
