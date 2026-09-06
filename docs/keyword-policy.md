# Keyword primitives policy

The normative policy on **which keyword abilities and keyword actions are
engine primitives versus plugin macros**, and the **template-param
story for parameterized keywords**. It is the prescriptive companion to
`docs/rules-taxonomy.md §5,§10`, which derive the descriptive classification
(pinned to the mtg-rules skill v1.10.0, CR effective 2026-08-07); this file
states the rules the engine commits to and where the code embodies them.
Decided by the `core-intrinsic-keywords-policy` ticket. The project term for
this class is **primitive** (glossary: Primitive Keyword Ability); the
descriptive catalog this file derives from spells it *intrinsic*. Intrinsic
Ability is reserved for the CR sense — the mana ability a basic land type
supplies ([CR#305.6]).

**Part I** governs keyword *abilities* (`[CR#702]` — the `KeywordAbility`
enum). **Part II** governs keyword *actions* (`[CR#701]` — the `Action` /
`PlayerAction` verbs). The two share one spine: a keyword is primitive only if
it is *irreducible*; everything else is a `Composite` macro, name-carried so
triggers can match it, with soundness measured post-expansion.

---

# Part I — Keyword abilities

## 1. The graduation rule

A keyword is a **primitive `KeywordAbility` enum variant** iff it owns
*prospective* machinery no composition of statics, triggers, replacements,
permissions/restrictions, and action-sequences can reproduce — i.e. it needs a
native engine opcode. Every other keyword is a `Composite` plugin macro invoked
inside the `Keyword(...)` wrapper, so a card always spells keyword-ness
explicitly (`Keyword(Flying)`).

The rubric (taxonomy §10):

- **Decompose first** — reduce to the kinds above before declaring anything
  primitive.
- **A shared `given`-primitive keeps a keyword composite.** If a hypothetical
  primitive `P` would let ≥ 2 keywords decompose, `P` is the primitive thing,
  not the keywords — they stay composite and reference `P` once it exists.
- **A sole-dependent primitive collapses into its keyword** as primitive.

Primitive-ness is a property of the **keyword**, not the bearing ability: a
primitive can still be parameterized (mutate carries a cost).

## 2. The three-class map, and enum membership

| Class | Abilities | Actions | Representation | Matched in engine |
|---|---:|---:|---|---|
| **Primitive** | 9 | 16 | `KeywordAbility` enum variant (abilities); native verb (actions) | by-variant (`has_keyword`) |
| **Composite-given(P)** | 14 | 5 | `Composite` macro | by-name (`has_keyword_named`) |
| **Composite** | 171 | 49 | `Composite` macro | by-name |
| **Marker** | 1 (reach) | 0 | empty `Composite` macro | by-name |

The columns are the two catalogs, not one population: 195 keyword abilities
(`data/gen/catalogs/keyword-abilities.txt`) and 70 keyword actions
(`keyword-actions.txt`). The 16 action primitives are `Action` / `PlayerAction`
verbs governed by Part II §10 — they never become `KeywordAbility` variants, and
Part I's "primitive set is closed at 9" is a statement about the ability column
alone.

**The primitive set (9 abilities) is closed and exhaustive:** first strike
`[CR#702.7]`, double strike `[CR#702.4]`, deathtouch `[CR#702.2,704.5h]`,
trample `[CR#702.19]`, vigilance `[CR#702.20a..702.20b,508.1f]` (combat-damage /
declare-attackers machinery, `[CR#510.1]`), plus banding
`[CR#702.22c,702.22e]`, phasing `[CR#702.26b..702.26e]`, mutate `[CR#702.140]`,
and companion `[CR#702.139a,116.2g]`. No other keyword may ever become a
variant.

**Enum membership follows implementation, not classification.** The enum
(`crates/deckmaste_core/src/keyword.rs`) holds only the *implemented* primitives
— today the five combat variants. Banding, phasing, mutate, and companion are
**reserved but absent**: each gets a variant when its mechanics land, never as
an unroutable placeholder. The invariant that `KeywordAbility::ALL`, `as_str`,
and `FromStr` list the same variants (enforced by the `keyword.rs` tests) keeps
"implemented" and "in the enum" the same set.

Keyword **names** stay an open, data-driven set (Warp, Firebending, Station are
never Rust variants). The primitive set is the only closed keyword vocabulary.

## 3. Composite-given keywords never graduate

The 19 composite-given(P) keywords — lifelink/wither/infect/toxic (a
damage-result-rewrite stage, `[CR#120.3b..120.3g]`), daybound/nightbound and
the speed keywords (a progress track), attach, the delve/convoke/improvise/
assist cost-modifiers, the morph/manifest family — carry only their printed name
today and match by string via the `has_keyword_named` look-through seam.

**Policy: they stay composite.** "Composite-given(P)" means the keyword
*decomposes* once its shared primitive exists — its macro body then expands into
honest abilities referencing `P`. It does **not** move into the enum. So the
near-term case (lifelink + wither/infect/toxic, pending the damage-result-
rewrite stage) is a macro-layer change, not a graduation: when the primitive
lands, their `Composite` bodies gain a real expansion and the combat hook stops
matching by bare name — but no `KeywordAbility` variant appears.

**Nothing graduates macro → enum.** The only future enum additions are the four
already-classified true primitives, each arriving with its own mechanics.

## 4. Template-param conventions

Parameterized keywords live entirely in the `Composite` macro layer:

- **Positional `params: [T]`** for a single obvious arg — Protection `[Predicate]`
  `[CR#702.16b..702.16f]`, Cycling `[Cost]` `[CR#702.29a]`.
- **Named `params: { "field": T, ... }`** for multiple/optional args — Ward
  `{ "cost": Cost, "where_x": Default(Count, 0) }` `[CR#702.21a,702.21b]`.
- **`Default(T, v)`** for an optional/defaulted arg — Hexproof
  `{ "from": Default(Predicate, Any) }` `[CR#702.11]`, Landwalk
  `{ "quality": Default(Predicate, Type(Land)) }`.
- **`template:` owns display and reverse-parse** — typed slots (`${cost}`),
  conditional fragments (`${ from #from#}`), sign/plural modifiers — each with a
  render codec and a matching parse codec (the `TemplateIndex`), so one authored
  template both prints the keyword and recognizes it in oracle text.

**Card soundness is measured post-expansion.** A card's meaning is the
expanded semantic data: a named composite keeps its keyword identity and
expanded abilities. The [Lean checker](../lean/Semantics/Check/Card.lean)
and its declared-feature laws are the workbench reference for that data;
macro parameterization describes how the invocation supplies it. The existing
Rust-to-Idris emitter is legacy tooling, pending the Lean card gate described
in §14.

## 5. Typed cycling — mirror Landwalk

Typed cycling (Slivercycling, Wizardcycling, Basic landcycling, generic
Typecycling `[CR#702.29e]`) is the one named parameterized case explicitly out
of scope in today's `Cycling` macro. Resolve it exactly as Landwalk handles its
typed variants:

- **One `Cycling` macro gains a defaulted quality/type parameter**, as Landwalk
  carries `quality: Default(Predicate, Type(Land))`. A typed variant passes a
  subtype predicate: `Slivercycling` → `Cycling(type: Subtype("Sliver"), cost:
  ...)`; `Basic landcycling` → a basic-land predicate. Base cycling reads the
  default, unchanged.
- **Typecycling proper (`[CR#702.29e]`) is a distinct body variant** — a
  *library search* for a matching card, not a plain draw — and is a separate
  follow-up rather than folded into the base body.

This is a prescription; the macro edits belong to a downstream
`macro-keyword-templates` / `kw-cycling` ticket.

## 6. Prune the shape registry (`ParamShape` / `KeywordDecl`)

`keyword.rs` defines a closed vocabulary of param shapes — `ParamShape` (`None`,
`Counted`, `Costed`, `CountedCost`, `Predicated`, `PredicatedCosted`, `Named`) —
and a registry row `KeywordDecl { name, shape }`. At plugin load (`plugin.rs`)
each `KeywordAbility`-kind macro's param-type multiset is matched against
exactly seven signatures; anything else (`keyword_shape` → `None`) is a load
error, and the derived `KeywordDecl` is inserted into a `keywords: HashMap`
registry.

**Both the check and the registry rest on a defunct justification and should be
removed.** Their original rationale was that the Idris model indexed keyword
*uses* by `ParamShape`, making a mis-shaped use unrepresentable and rejected at a
re-emit gate. That is no longer how soundness works (§4): the gate is
post-expansion, by name over desugared abilities, and no code consumes
`KeywordDecl` — the only reads are one test and a clone-forward. Two facts
confirm the vocabulary carries no weight:

- **It is lossy past the point of validating anything** — the shape collapses
  named-vs-positional, drops arity and order, and erases `Default`-ness (Ward's
  optional defaulted `where_x:Count` and Suspend's required leading `Count` both
  project to `CountedCost`). A consumer could not faithfully validate a use from
  the shape alone.
- **The closed seven-signature cap is an arbitrary authoring restriction** — a
  keyword whose honest signature is `[Predicate, Predicate]`, or three params, or
  a type outside `{Count, Cost, Predicate, String}`, fails to load for no
  downstream reason.

**Policy: a cleanup ticket removes `KeywordDecl`, the `ParamShape` enum, the
`keyword_shape` load-time check, and the `keywords` registry field.** Keyword
macros may then declare any param signature; the expanded card is judged
against the Lean checker and its declared-feature laws (§14). The stale
`keyword.rs` re-emit-gate comment is removed with them.

## 7. Invariants the ability policy commits to

- The primitive set is closed at 9 abilities; enum membership equals the
  *implemented* subset, kept in sync across `ALL` / `as_str` / `FromStr`.
- The `Keyword(...)` wrapper is always explicit on a card.
- Every keyword — primitive, composite-given, or composite — carries its printed
  name, so `LoseAbility` / `Has(KeywordRef)` name paths behave uniformly.
- Keyword names are open and data-driven; the primitive set is the only closed
  keyword vocabulary (`ParamShape` is retired, not preserved).
- Card soundness is post-expansion, against the Lean checker and its
  declared-feature laws; no closed keyword-signature registry is reintroduced.

---

# Part II — Keyword actions

## 8. The atomicity criterion (pure irreducibility)

A `[CR#701]` keyword action is a **native atomic verb** (an `Action` /
`PlayerAction` variant) iff it is **irreducible** — it cannot be expressed as a
composition of smaller game actions. If it *can* be decomposed, it is
**composite**: an `Instruction`-kind macro whose body is built from the atoms,
exactly as `fight` became a macro over `DealDamage`
(`docs/tickets/done/core-fight-primitive-to-macro.md`).

This is stricter than the mtg-rules skill's "event-basis" cut, and deliberately
so. **There is no "interceptable-as-itself" escape hatch.** An action that is
merely a *named flavor* of a more primitive verb is composite even when
replacement / prevention / restriction rules key off it: the CR defines destroy
as "move it to its owner's graveyard" `[CR#701.8a]`, so destroy is
`Move(→graveyard)` plus a name-tag, and the interceptors (indestructible,
regeneration) target the **named composite**, not a bespoke `Destroy` opcode.

**The `Composite` name-tag.** A decomposed action is wrapped in
`Action::Composite(KeywordAction, Box<Instruction>)` (pairing the keyword-action
atom with its body and emitting the present-tense `Act` event) iff a trigger or
interceptor must **name** the event ("whenever you scry…", "can't be
destroyed"). Otherwise it desugars straight to
the atoms with no wrapper — `investigate` and `amass` have no name-tag because
nothing names them. The emit is gated on the body actually acting (scry 0 does
nothing, `[CR#701.22b]`).

## 9. Scope

This part governs `[CR#701]` keyword actions and the primitive effect-verbs they
decompose into. It does **not** classify every verb in the effect language:
set-life `[CR#119.5]`, get-an-emblem `[CR#114]`, and similar are effect
primitives, **not** keyword actions, and are out of scope here — the effect
language owns them.

## 10. The irreducible keyword actions (native)

These `[CR#701]` actions cannot be reduced to smaller game actions and so are
native verbs:

- **Counter** `[CR#701.6]` — remove a spell/ability from the stack.
- **Create** `[CR#701.7]` — instantiate a token (its own ETB-ordering rules).
- **Tap** / **Untap** `[CR#701.26]` — status flips.
- **Reveal** `[CR#701.20]` — show, no zone change; an information event.
- **Shuffle** `[CR#701.24]` — randomize a library; an information event.
- **Search** `[CR#701.23]` — look through a (possibly hidden) zone.
- **Transform** `[CR#701.27]` — DFC face-flip. **Convert** `[CR#701.28]` folds
  in — its CR text simply defers to Transform's rules, the same op, not a second
  atom.
- **Attach** / **Unattach** `[CR#701.3]` — the attachment *relation*, engine
  machinery no game action reproduces (a shared `given`-primitive: equip, aura,
  fortify, and reconfigure all depend on it).

Plus the **process** actions **Cast** `[CR#701.5]`, **Play** `[CR#701.18]`, and
**Activate** `[CR#701.2]` — irreducible, but they live in the casting/priority
pipeline (`[CR#601,602,116]`), not as one-shot effect verbs.

## 11. Everything else is composite

Every other `[CR#701]` action decomposes. Grouped by its primary decomposition
target (⊕ = also carries a name-tag):

| Decomposes over | Actions |
|---|---|
| **Move** (zone change) | destroy⊕ `[CR#701.8]`, sacrifice⊕ `[CR#701.21]`, discard⊕ `[CR#701.9]`, exile `[CR#701.13]`, mill⊕ `[CR#701.17]`, return-to-hand, forage `[CR#701.61]`, collect evidence `[CR#701.59]` |
| **DealDamage** | fight `[CR#701.14]` (simultaneous — see §13) |
| **Create** | investigate `[CR#701.16]`, populate `[CR#701.36]`, incubate `[CR#701.53]`, amass `[CR#701.47]`, endure `[CR#701.63]` |
| **PutCounters / RemoveCounters** | bolster `[CR#701.39]`, support `[CR#701.41]`, adapt `[CR#701.46]`, proliferate `[CR#701.34]`, blight `[CR#701.68]`, time travel `[CR#701.56]`, earthbend `[CR#701.66]` |
| **GetDesignation** | goad `[CR#701.15]`, detain `[CR#701.35]`, suspect `[CR#701.60]`, harness `[CR#701.64]`, monstrosity `[CR#701.37]`, become day/night `[CR#731.1]`, the Ring tempts you `[CR#701.54]` |
| **CreateReplacement** | regenerate `[CR#701.19]` |
| **Reveal + Move** (peek-and-sort) | scry⊕ `[CR#701.22]`, surveil⊕ `[CR#701.25]`, fateseal⊕ `[CR#701.29]`, clash `[CR#701.30]`, explore `[CR#701.44]`, discover `[CR#701.57]` |
| **Draw / Discard combinations** | connive `[CR#701.50]`, learn `[CR#701.48]`, recruit `[CR#701.70]` (draw, discard, then a conditional token) |
| **RemoveDamage** | heal `[CR#701.69]` |
| **continuous restriction** | exert `[CR#701.43]` (a next-untap-step restriction; the attack rider is an optional cost plus a linked trigger, `[CR#701.43d]`) |
| **continuous P/T** | double `[CR#701.10]`, triple `[CR#701.11]` |
| **GainControl / life swap** | exchange `[CR#701.12]` |
| **Reveal / choice** | behold `[CR#701.4]` (reveal-from-hand or choose), vote `[CR#701.38]`, face a villainous choice `[CR#701.55]` |
| **composite-given (deferred primitive)** | manifest/cloak/manifest dread `[CR#701.40,701.58,701.62]` (face-down objects), meld `[CR#701.42]` (merged objects), waterbend `[CR#701.67]` (cost-modification hook), airbend `[CR#701.65]` |
| **variant-format machinery** | venture `[CR#701.49]`, planeswalk `[CR#701.31]`, set in motion `[CR#701.32]`, abandon `[CR#701.33]`, open an attraction `[CR#701.51]`, roll to visit `[CR#701.52]`, assemble `[CR#701.45]` (Unstable Contraptions — outside these rules, `[CR#701.45a]`) |

## 12. The primitive-verb inventory (the atoms)

The effect-language leaves that composites decompose into: **Move**,
**DealDamage** `[CR#120.1]`, **GainLife**/**LoseLife** `[CR#119.3]`, **AddMana**
`[CR#106.4]`, **PutCounters**/**RemoveCounters** `[CR#122.1]`, **MoveCounters**
`[CR#122]`, **GainControl** `[CR#701.12b]`, **CopySpell** `[CR#707.10]`,
**FlipCoins** `[CR#705.1]`, **RollDice** `[CR#706.1]`, **CreateReplacement**
`[CR#614.3]`, **GetDesignation**, **ChooseAndNote** `[CR#608.2d]`, **Draw**
`[CR#121.1]`, **WinGame**/**LoseGame** `[CR#104.2b,104.3e]`, **RestartGame**
`[CR#727.1]`, **ExtraPhase** `[CR#500.8]`, **RemoveDamage** `[CR#614.8]` — plus
the irreducible keyword actions of §10.

Note the criterion recurses: some of these primitives are themselves reducible
in principle — **Draw** `[CR#121.1]` is `Move(top-of-library → hand)` with a
name-tag (draw-from-empty loss, draw replacements). Whether to keep such verbs
as convenience primitives or decompose them further is an **effect-language**
decision, flagged here but not settled by the keyword-action policy. This policy
commits only that keyword actions *above* the `Move`/`DealDamage`/counter level
decompose to them.

## 13. Shared sub-primitives (the ≥2-dependents rule)

The action-side echo of Part I's shared-`given`-primitive rule: a not-quite-
keyword shape that recurs across ≥ 2 composites may earn a native building block
used only in composition.

- **Justified now:** `CreateReplacement` (regenerate + one-shot prevention), the
  **Attach relation** (equip/aura/fortify/reconfigure), and the `Simultaneous`
  **combinator** — which is mid-generalization precisely because both the
  exchange family *and* fight need one batch/timestamp with per-member
  replacement and post-batch SBAs `[CR#603.2c,704.3]`
  (`docs/tickets/done/core-fight-primitive-to-macro.md`).
- **Watched, not minted:** the "peek the top N and sort to top/bottom/graveyard"
  shape recurs across scry/surveil/fateseal/clash/explore/discover, but it is
  expressible via `Move` + a top-of-library binder + `Modal` with no interceptor
  — so **no shared verb yet**. Mint one only if a further consumer makes the
  desugaring painful.

## 14. Lean checker and soundness (actions)

Keyword actions have open labels and expanded instruction bodies. Macros
construct those bodies; the label records which named action occurred.
The Lean checker reads declared features for roles, scopes and conferrals,
rather than requiring a new syntax constructor for each action name.
`Semantics/Check/Facts.lean` is generated from builtin_v2 declarations and
checker-column overlays. A missing declaration or required overlay is a
visible generation gap; `cargo xtask facts check` detects stale generated data.

`lean/scripts/build` checks the syntax, laws, card bench and exact-result proof
pins. It is the active workbench gate. The separate
[lean-card-soundness-gate](tickets/planned/lean-card-soundness-gate.md) ticket
replaces the legacy Rust-to-Idris card emitter with Rust-to-Lean re-emission.
Until that lands, the workbench build does not claim to validate every loaded
Rust card. [Lean is the workbench](decisions/lean-is-the-workbench.md) records
the succession; the handwritten Idris model is reference only.

---

## 15. Prescribed follow-up tickets

This policy authorizes, but does not itself perform:

**Abilities (Part I):**

1. **`macro-keyword-templates` / `kw-cycling`** — extend `Cycling` with the
   defaulted type parameter (§5); add the Typecycling library-search body variant
   as its own item.
2. **A shape-registry cleanup ticket** — remove `ParamShape`, `KeywordDecl`,
   `keyword_shape`, and the `keywords` registry field; drop the stale re-emit
   comment in `keyword.rs` (§6).
3. **The composite-given expansion work** (existing damage-result-rewrite and
   related engine tickets) — as each shared primitive lands, replace the affected
   keywords' bare-name matching with honest `Composite` expansions; no enum
   change (§3).
4. **The four reserved primitives** (banding, phasing, mutate, companion) — each
   gets its `KeywordAbility` variant when implemented (§2).

**Actions (Part II):**

5. **Demote the reducible native verbs** — Destroy, Sacrifice, Discard →
   `Move(→graveyard)` name-tagged; Mill → `Move(top-N→graveyard)` name-tagged;
   ReturnToHand → `Move(→hand)`; Convert → Transform. Each requires the
   interceptors (indestructible, regeneration, madness) to key off the named
   composite; sequence them behind the `Simultaneous` generalization the Fight
   demotion already needs (§13).
6. **Build the remaining primitive verb** — `Transform` `[CR#701.27]` has no
   `Action`/`PlayerAction` variant yet (§10). Search `[CR#701.23]` is now
   represented by `Binder::{Search, SearchOne}`; its grammar is present, while
   runtime search/choice consumption remains an engine seam.
7. **Complete the Lean card gate** — re-emit expanded semantic data into the
   active workbench, reporting translation gaps explicitly (§14). New action
   names are declarations and macros, not workbench enum constructors.
8. **Build the staged composite actions** — Explore, Connive, Bolster, Search-
   dependent actions, and the rest of §11 as card pressure demands.

---

## 16. Classification audit — 2026-08-21

Re-pinned from mtg-rules **v1.10.0** (CR effective 2026-08-07,
`keywords-classified.json` sha256 `6b1ab6ae89a9…`, 265 records = 195 abilities +
70 actions), replacing the v1.7.0 baseline (260 records). Every catalog entry was
re-checked against §1's rubric; the result is a count refresh, not a
reclassification.

- **265 checked, 0 reclassified.** No entry changed class between the two pins.
  The primitive (9 / 16), composite-given (19), and marker (1) populations are
  identical; only the composite population grew, 215 → 220.
- **5 additions since the pin.** Ability composites Storied `[CR#702.195]`,
  Power-up `[CR#702.193]`, Teamwork `[CR#702.194]` (absorbed by §2's count — the
  ability composites are not enumerated here); action composites heal
  `[CR#701.69]` and recruit `[CR#701.70]`, now filed in §11. Storied and recruit
  are confirmed 1.10.0 additions; the other three are inferred from the
  1.7.0 → 1.8.0 count delta, the v1.7.0 catalog not being recoverable locally.
- **4 misses closed.** §11 lacked assemble `[CR#701.45]`, exert `[CR#701.43]`,
  heal `[CR#701.69]`, and recruit `[CR#701.70]`; each is now under its
  decomposition target.
- **5 knowing divergences, all retained.** §8's stricter criterion keeps destroy
  `[CR#701.8]`, sacrifice `[CR#701.21]`, discard `[CR#701.9]`, and exile
  `[CR#701.13]` composite where the descriptive catalog files them as intrinsic
  event-basis actions; §10 keeps attach `[CR#701.3]` native where the catalog
  files it composite-given(attachment-relation) — the same primitive, minted
  here (§13) rather than deferred.
- **Non-catalog entries in §11 stay.** "Return-to-hand" and "become day/night"
  `[CR#731.1]` are effect-language verbs, not `[CR#701]` keyword actions; they
  are decomposition examples, not classified entries.
- §2's rows are now labeled by catalog, which discharges the count-reconciliation
  item of `docs-keyword-policy-refresh`. That ticket keeps the §6
  `ParamShape` / `KeywordDecl` decision.
