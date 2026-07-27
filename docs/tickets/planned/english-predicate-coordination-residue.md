---
needs: []
---
**Predicate coordination under a shared subject — the two stages round
`predcoord` did not land, and the measured defect in one of them.**
Campaign-internal residue of `english-structural-recovery-zero`, split out so
the diagnosis survives in the tree; fold into the live campaign workspace with
`workflow claim english-predicate-coordination-residue --into
english-structural-recovery-zero`.

Round `predcoord` (2026-07-27) landed **Stage A only** — a shared modal
predicate carried through coordinated-clause lowering, 122 clause spans / 117
faces. The full design is on disk at `recovery-harness/out/predcoord-plan.md`
and remains valid apart from the Stage B defect recorded below. Counts are
unresolved rows measured against the post-`predcoord` census (clause 3501 /
65651, structural total 3637 spans / 3249 faces).

## 1. Stage B — the specified allowance is UNSOUND as written. Do not re-apply it.

The plan's §5 refines the asyndetic special case in `grammar/clause.rs` (near
the `continuation_is_bare_imperative` binding) to admit a second, disjoint
case:

```rust
let continuation_is_shared_finite = first_agreement.is_some()
    && !*has_subject
    && !*standalone
    && (*next_agreement == *first_agreement || next_agreement.is_none());
```

used as an extra `&& !continuation_is_shared_finite` disjunct on the
`ClauseCoordinationAsyndetic` rejection.

**This was implemented, measured, and reverted.** It breaks three pre-existing
anti-misparse gates:

- `grammar::clause::tests::coordinated_donated_agreement_does_not_adopt_a_bare_imperative_tail`
- `grammar::ability::tests::coordinated_trigger_event_is_unchanged`
- `grammar::ability::tests::paragraph_initial_trigger_with_a_coordinated_event_still_recovers`

Those gates are a **previous round's deliberate protection**, and their
docstrings name the exact failure this allowance reintroduces: the allowance
"let unparsed trigger sentences misparse with the trigger word licensed as an
opaque noun subject (`When [you lose]` as a nominal, `control` as its verb) and
the effect clause absorbed as a shared-predicate continuation — **render-identical
but structurally wrong**." The witness that fails is
`"When this creature becomes untapped or you lose control of this creature, exile that creature."`

The allowance is too wide because it tests only the *continuation's* shape
(`!has_subject && !standalone` plus agreement) and never constrains the
**host**. A subordinate trigger host whose subject is an opaque-noun misparse
satisfies `first_agreement.is_some()` just as a genuine `Enchanted creature`
host does, so the gate cannot tell them apart.

Measured cost of reverting: **zero**. With Stage B's allowance in the tree and
its tests failing, the census was byte-identical to Stage A alone (clause 3501
either way) — it contributed no recovery and only risk.

**A redesign must gate the host, not merely the continuation**, and must keep
all three gates above green. Its own test set (six tests, specified in
`predcoord-plan.md` §5) was written and is worth recovering from that document;
it was removed from the tree unimplemented rather than landed red.

Stage B's inventory is **24 spans / 24 faces**
(`gets +N/+N, has <keyword>, and is <type>` asyndetic three-way coordination,
100% failure rate corpus-wide): Ghoulish Impetus, Arachnoform, Nim Deathmantle,
Infectious Bloodlust, Iona's Blessing, the five Vows, Rope, Assault Suit, Aether
Web, and others.

## 2. Stage C — designed, unattempted, no defect known

`and is <NP> in addition to its other types` as a subject-shared copular
continuation. **40 spans / 38 faces**, 90.5% corpus failure rate. Governed by
[CR#205.1b] (such effects retain all prior card types, supertypes and subtypes).

The plan (§6) specifies three narrow productions —
`Clause -> Clause Conjunction Copula NounPhrase PrepositionalPhrase` and its
comma and asyndetic variants — deliberately keyed to an `NP` complement plus a
trailing `in`-headed PP rather than a generic copular remainder. **That
narrowness is load-bearing** and must not be widened: a generic remainder would
also admit the `every/all creature type` rows, which parse only by
manufacturing an opaque `every` noun (see §3).

This is the **only stage that registers productions**, so §A of the invariant
audit applies in full: the categorical host gate belongs at dot 1 in
`accepts_predicate_prefix`, not at reduce.

Stage C was never implemented — the round's mechanic died to an environmental
stall before reaching it. No defect is known or suspected.

## 3. Stage D — dropped on a falsified premise; needs a determiner model first

`and is every/all creature type` (4 spans / 4 faces: Amorphous Axe, Runed
Stalactite, Arachnoform, Stalactite Dagger). A standalone-tail probe found that
Amorphous Axe, Runed Stalactite and Arachnoform each introduce an **opaque
`every` noun** when the tail is parsed at all. Admitting them through a generic
shared copula would convert three structural recoveries into wrong opaque trees
and raise noun opacity — accuracy loss for four rows.

A future round must model the determiner or type-set quantification explicitly
before coordinating it. Do not reach these rows with an `every` spelling check,
an opaque exception, or a TSV row.

## 4. Stage E — not this family; genuinely separate

Floating `each` on a plural target subject
(`Up to two target creatures each get +2/+2 until end of turn.`) — **41 spans /
41 faces, 100% failure**. `each` sits after the plural target NP and before the
predicate, so no clause-coordination production, separator gate, or shared
member reaches that position. This is **quantifier float**, not predicate
coordination, and it stayed 41/41 even when no coordinated predicate follows.

It is a clean, high-rate, self-contained family and a good candidate for its own
round. Aquatic Ingress and Agility Bobblehead are blocked by it despite their
modal material being Stage A work.

## 5. Residue Stage A left behind

Named and measured, all still unresolved:

- **The `as though` family — 12 faces**, blocking otherwise-Stage-A rows: Aether
  Web, Street Savvy, Colossus of Akros, Mobile Fort, Nivix Cyclops, Territorial
  Witchstalker, Walking Wall, Wall of Wonder, Spire Serpent, Arcades the
  Strategist, Walking Bulwark, The Pride of Hull Clade. The right conjunct
  attaches as a clause-level dependent, not the `SimpleClause` slot the
  coordination rule consumes. A design exists at
  `recovery-harness/out/asthough-plan.md`.
- **Two newly-visible one-word lexical gaps**, exposed *because* Stage A
  resolved the clause around them — a precision gain, not a regression. Noun
  opacity 963 → 965.
  - `Garland, Royal Kidnapper` — `"but"`, from the `creatures you control but
    don't own` subject modifier.
  - `Ratonhnhaké꞉ton` — `"yet"`, from the `hasn't dealt damage yet` condition.

  Neither should be pinned by a test until its construction is modelled.
- Independently failing hosts and modal tails, 14 faces: Akawalli (`additional`
  pump host), Hraesvelgr (coordinated trigger head), Anti-Magic Aura (modal
  copular left predicate), Cosmos Charger (foretelling subject), Errantry (`only
  attack alone`), Hundred-Handed One (`additional ninety-nine`), Spectral Shield
  (modal copular NP), the five Archetypes (`have or gain`), and Arcane
  Lighthouse (`lose hexproof and shroud and can't have ...`, a separate
  coordination gap).

## 6. One thing Stage A proved that the row inventory missed

The round was scoped by a regex over `can`/`can't`, but the fix keys on the
**modal auxiliary generally** — `finish_predicate` strips the first modal
whatever it is. So `SharedDeontic` also carries `may`, and 26 faces outside the
scoped inventory cleared as the same construction: Chain Lightning, Chain of
Smog / Acid / Plasma / Silence / Vapor, Chain Stasis, String of
Disappearances, Curse of Echoes, Tempt with Mayhem, Malicious Affliction,
Slayer's Cleaver, Magar of the Magic Strings, Captain Rex Nebula and others.
Verified by tree read on Chain Lightning: `SharedDeontic(Modal { auxiliary:
May, inflection: Base }, Some(Transitive(...)))`, zero opacity leaves.

**Scope future rounds in this area by the modal class, not by a `can't`
regex.**
