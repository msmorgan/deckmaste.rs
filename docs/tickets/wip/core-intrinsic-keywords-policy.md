---
needs: []
design: true
---
Policy decision on which keywords graduate from plugin macros to intrinsic
`KeywordAbility` variants, and the template-param story for parameterized
keywords (ward, protection, typed cycling).

**DECIDED — the normative policy lives in `docs/keyword-policy.md`** (the
durable, discoverable home; the prescriptive companion to
`docs/rules-taxonomy.md §10`, cross-linked from the `keyword.rs` module doc).
This ticket changes no code beyond authoring that doc and its cross-links.

Decisions, in brief (full text and CR citations in the policy doc):

- **Graduation rule** — intrinsic iff a keyword needs a native opcode no
  composition reproduces. The intrinsic set is closed at **9 abilities**
  (first/double strike, deathtouch, trample, vigilance, banding, phasing,
  mutate, companion); enum membership follows *implementation* (5 present, 4
  reserved-but-absent).
- **Composite-given never graduate** — lifelink/wither/infect/toxic etc. stay
  macros; they decompose into their body once the shared primitive lands.
  Nothing moves macro → enum.
- **Typed cycling mirrors Landwalk** — one `Cycling` macro + a defaulted
  type param; Typecycling's library-search body is a separate follow-up.
- **Prune `ParamShape`/`KeywordDecl`** — soundness is post-expansion
  (`idris2 --check` by name), so the closed-shape check and its unconsumed
  registry are unmotivated; a cleanup ticket removes them.

Keyword **actions** (`[CR#701]`, Part II of the policy doc) follow the same
spine under a **pure-irreducibility** criterion:

- **Native only if irreducible** — Counter, Create, Tap, Untap, Reveal,
  Shuffle, Search, Transform, Attach/Unattach (+ the Cast/Play/Activate pipeline
  processes). Everything decomposable is a `Composite` macro over the atoms, as
  Fight became a macro over DealDamage.
- **No "interceptable" escape hatch** — destroy/sacrifice/discard/mill/return
  decompose to `Move` (name-tagged); interceptors (indestructible, regeneration,
  madness) retarget to the named composite. Those demotions + building the
  missing `Transform`/`Search` verbs are follow-ups.

Follow-up tickets are enumerated in the policy doc's §15.
