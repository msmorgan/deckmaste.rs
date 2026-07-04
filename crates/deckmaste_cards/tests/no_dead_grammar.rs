//! The G12 "no dead grammar" coverage sweep
//! ([[cards-no-dead-grammar-sweep]]): every core grammar node — an enum
//! variant defined in one of the seven grammar family files (effects,
//! actions, statics, events, counts, conditions, references) — must carry at
//! least one loading ACCEPTANCE card (`plugins/{canon,testing,builtin}`,
//! cards or macro bodies) AND at least one REJECT fixture
//! (`crates/deckmaste_cards/tests/reject/`), or be named, with a reviewed
//! reason, in one of the two allowlists below.
//!
//! MECHANICAL, not hand-maintained: the inventory comes from parsing the
//! actual `deckmaste_core` source (`syn`), so a newly-minted grammar node is
//! swept in automatically the moment it exists — no second list to update.
//! Coverage is a plain whole-word text search over the corpus (never a typed
//! AST walk): a card that reaches a node only through MACRO EXPANSION is
//! still credited, because macro *bodies* (`plugins/*/macros/**/*.ron`) are
//! part of the accept corpus and spell the expanded primitives directly
//! (e.g. `Exile`'s body is `Move(Param(0), Exile)` — `Move` is credited from
//! the macro definition even though no card ever writes `Move` for exile).
//!
//! Two kinds of exemption, both reviewed (not a dumping ground):
//!  - `STRUCTURALLY_UNTAGGED`: the variant's own tag is NEVER spelled in RON by
//!    construction (`macro_ron(flatten)` dispatch, or bare-numeral sugar) — a
//!    text search for the tag name is structurally meaningless, not evidence of
//!    disuse.
//!  - `ACCEPT_ALLOWLIST` / `REJECT_ALLOWLIST`: a real gap, with a reason each
//!    entry's author stands behind (e.g. "no illegal configuration exists at
//!    this node" for the reject side).

use std::collections::BTreeSet;
use std::path::Path;
use std::path::PathBuf;

use regex::Regex;

/// The seven grammar-family source files this sweep walks, relative to
/// `deckmaste_core/src/`. Matches the ticket's own family list verbatim
/// (effects/actions/statics/events/counts/conditions/references).
const GRAMMAR_FILES: &[&str] = &[
    "effect.rs",
    "action.rs",
    "continuous.rs",
    "event.rs",
    "count.rs",
    "condition.rs",
    "reference.rs",
];

fn core_src_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../deckmaste_core/src")
}

fn plugins_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins")
}

fn reject_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/reject")
}

/// One grammar node: the enum it belongs to, and its variant name (the tag a
/// card would spell, e.g. `Effect::Sequence` -> `("Effect", "Sequence")`).
type Node = (String, String);

/// Parses one grammar-family file and returns every TOP-LEVEL `pub enum`'s
/// (enum, variant) pairs. Never descends into a nested `mod` — this is how
/// `#[cfg(test)]` fixture enums (there are none today, but the rule is
/// structural, not incidental) and any future non-grammar nested enum stay
/// out of the inventory: only file-top-level enums count as core grammar.
fn enum_variants_in_file(path: &Path) -> Vec<Node> {
    let source =
        std::fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    let file = syn::parse_file(&source).unwrap_or_else(|e| panic!("parse {}: {e}", path.display()));
    let mut out = Vec::new();
    for item in &file.items {
        if let syn::Item::Enum(item_enum) = item {
            let enum_name = item_enum.ident.to_string();
            for variant in &item_enum.variants {
                out.push((enum_name.clone(), variant.ident.to_string()));
            }
        }
    }
    out
}

/// The full node inventory: every (enum, variant) pair across the seven
/// grammar files.
fn grammar_inventory() -> Vec<Node> {
    let dir = core_src_dir();
    let mut nodes = Vec::new();
    for file in GRAMMAR_FILES {
        nodes.extend(enum_variants_in_file(&dir.join(file)));
    }
    nodes
}

/// Variants whose own TAG can never appear as a spelled RON token, by
/// construction — a text search for the name is structurally inapplicable,
/// not evidence the node is unused:
///
///  - `Effect::Act` / `StaticEffect::Deontic` / `Destination::Zone` are
///    `#[macro_ron(flatten)]`: the wrapping tag is fully invisible in RON —
///    only the PAYLOAD type's own variant names are ever spelled (a bare verb
///    like `Draw(1)` reads as `Effect::Act(Action::By(You, Draw(1)))` with
///    neither `Act` nor `Zone` ever written). Every one of these positions is
///    exercised constantly; there is simply nothing to grep for.
///  - `Count::Literal` is `#[macro_ron(literal)]` bare-numeral sugar: a card
///    always writes the bare number (`3`), never the convention-breaking
///    `Literal(3)` spelling — exercised on nearly every card, unobservable by
///    tag search.
///
/// (`Action::By` is deliberately NOT here: `#[macro_ron(embed)]` still
/// permits — and real cards use — the explicit spelling for a non-default
/// agent, e.g. `By(That(Player), Draw(3))`.)
fn structurally_untagged() -> BTreeSet<Node> {
    [
        ("Effect", "Act"),
        ("StaticEffect", "Deontic"),
        ("Destination", "Zone"),
        ("Count", "Literal"),
        // Every `SupportsMacros` enum's macro-invocation-remembering wrapper:
        // present on nearly every grammar enum, but its OWN tag is never
        // itself spelled — a macro invocation always serializes as the
        // invocation (`AddPowerToughness(2, 2)`), never literally as
        // `Expanded(...)`. A text search for the tag is structurally
        // meaningless for the same reason `Act`/`Zone`/`Deontic` are above.
        ("Effect", "Expanded"),
        ("PlayerAction", "Expanded"),
        ("Modification", "Expanded"),
        ("StaticEffect", "Expanded"),
        ("EventFilter", "Expanded"),
        ("Count", "Expanded"),
        ("Condition", "Expanded"),
        ("Reference", "Expanded"),
        // `Several` is `Modification::flatten`'s OWN internal splice
        // target for a change-bundling macro's expansion value
        // (`AddPowerToughness` -> `Expanded(.., value: Several([..]))`); a
        // card never spells `Several(...)` directly (there is no macro
        // named that), so, like the `Expanded` family above, the tag never
        // appears as an authored token.
        ("Modification", "Several"),
    ]
    .into_iter()
    .map(|(e, v)| (e.to_string(), v.to_string()))
    .collect()
}

// ── Shared, reviewed reasons (used across many entries below) ──────────────

/// A closed selector/vocabulary position (a turn-structure step, an axis
/// name, a cause verb, a player attribute, ...): every value the type can
/// hold is well-formed at this position by construction — there is no
/// elaboration-time legality predicate a fixture could trip. Picking the
/// "wrong" one is a semantic mismatch a card author just wouldn't print,
/// never a load error.
const NO_ILLEGAL_VALUE: &str = "closed selector vocabulary; every value is well-formed at this position, no elaboration-time \
     legality predicate exists to violate";

/// The op/axis pairing this variant's field carries is enforced by the RUST
/// TYPE SYSTEM itself (see `crate::continuous`'s own docs): a collection
/// axis (`Colors`/`CardTypes`/...) only ever carries a `CollectionOp`,
/// which has no `Up`/`Down`, and a numeric axis only ever carries a
/// `NumericOp`, which has no `Add`/`Remove` — so the illegal combination a
/// reject fixture would need to spell (raising a color, adding a power
/// delta) is unrepresentable in RON at all. There is nothing to construct.
const AXIS_OP_TYPE_ENFORCED: &str = "the op/axis pairing is Rust-type-enforced (recovers Idris's Collection/Numeric type-class \
     gate structurally); the illegal combination this code would need is unrepresentable in RON";

/// A plain, ungated verb/effect/static: the binding/caps/floor rules active
/// at this position are generic across every sibling of the same kind and
/// are already demonstrated, via an existing reject fixture, using a
/// DIFFERENT sibling at the identical position. This node's own semantics
/// add no further checked constraint beyond that shared, already-covered
/// gate.
const GENERIC_GATE_ALREADY_DEMONSTRATED: &str = "no bespoke legality rule targets this node specifically; the generic binding/caps/floor gate \
     at this position is already demonstrated by a sibling node's reject fixture";

/// `E-POS-RIDER` ([CR#614.12]) is generic across every `EnterRider` variant:
/// "a rider on a non-battlefield destination" is illegal regardless of
/// WHICH rider. The reject twin (`E-POS-RIDER/rider-on-battlefield-move.ron`)
/// exercises this via `Tapped`; respelling the identical mistake with each
/// sibling rider would exercise the same one line of checker logic again.
const RIDER_POSITION_GENERIC: &str = "E-POS-RIDER is generic across every EnterRider variant (any rider off the battlefield \
     destination is refused); the reject twin already demonstrates it via `Tapped`";

/// Accept-side exemptions: a node with no real fixture anywhere in
/// `plugins/{canon,testing,builtin}`.
///
/// Two different kinds of entry, and this list is NOT pretending they're
/// the same kind: a `BLOCKED` note means a real card is named but its
/// grammar has a genuine, confirmed gap (findings for the coordinator, not
/// invented fixtures — see the ticket's completion notes). A `DEFERRED`
/// note is a plain, honest backlog item: the node is buildable with a real
/// or `testing`-mock card (a candidate is usually named), nothing structural
/// blocks it, it simply wasn't reached in this session's batch. Treat every
/// `DEFERRED` entry as a to-do, not a soundness claim.
fn accept_allowlist() -> Vec<(Node, &'static str)> {
    vec![
        (
            n("Effect", "Label"),
            "BLOCKED: Label's only real use-case in this batch (Blood Money's \
            'destroyed this way' product-group read-back) is the same confirmed grammar gap as \
            Noting/AmongNoted below — see the ticket's completion notes.",
        ),
        (
            n("Effect", "MayPay"),
            "DEFERRED: no real card in this batch uses the resolution-time \
            optional-cost-kicker shape (canon's Mana Leak uses the punisher MustPay instead); a \
            real candidate (e.g. a 'you may pay {2}; if you do, ...' spell) is buildable.",
        ),
        (
            n("Effect", "Noting"),
            "BLOCKED: 'for each nontoken creature destroyed this way' (Blood \
            Money) needs a way to re-read a noted PRODUCT group filtered further; \
            Selection::AmongNoted is a CHOICE primitive (wrong shape for an unconditional \
            'for each'), and no Count/Filter combinator reads a noted object set at all — a \
            genuine missing-grammar finding, not built. See the ticket's completion notes.",
        ),
        (
            n("Effect", "Reflexive"),
            "DEFERRED: no real 'when you do' reflexive-trigger card in this \
            batch; render support for Reflexive is also unbuilt (same family as Delayed).",
        ),
        (
            n("PileSource", "Noted"),
            "DEFERRED: the per-player noted-piles shape (Whims of the \
            Fates: SeparatePiles.note + ChoosePile(from: Noted(..))) is designed (see the ticket's \
            completion notes) but not built this session.",
        ),
        (
            n("Anchor", "FromBottom"),
            "DEFERRED: no real card in this batch targets the BOTTOM of a \
            library by anchor (canon's library moves are all top-anchored); buildable.",
        ),
        (
            n("EnterRider", "FaceDown"),
            "DEFERRED: no morph/manifest real card in this batch.",
        ),
        (
            n("EnterRider", "UnderControlOf"),
            "DEFERRED: no 'enters under a NAMED player's control' \
            real card in this batch (Otherworldly Journey covers UnderOwnersControl).",
        ),
        (
            n("Arrangement", "ChosenOrder"),
            "DEFERRED: Action::MoveGroup itself has no accept \
            fixture yet (see below); one real simultaneous-group-move card would cover its \
            Arrangement choice too.",
        ),
        (
            n("Arrangement", "AnyOrder"),
            "DEFERRED: see Arrangement::ChosenOrder.",
        ),
        (
            n("Arrangement", "SameOrder"),
            "DEFERRED: see Arrangement::ChosenOrder.",
        ),
        (
            n("Arrangement", "RandomOrder"),
            "DEFERRED: see Arrangement::ChosenOrder.",
        ),
        (
            n("Action", "ReturnToHand"),
            "DEFERRED: no real card in this batch bounces via the bare \
            source-agent verb (existing effects route through PlayerAction::Move); buildable.",
        ),
        (
            n("Action", "MoveGroup"),
            "DEFERRED: no real simultaneous-group-relocation card (e.g. \
            'return all creatures to hand') in this batch — Brainstorm's own group-move uses \
            Each+Move per element, not MoveGroup.",
        ),
        (
            n("Action", "ExtraPhase"),
            "DEFERRED: no extra-combat/extra-phase real card (e.g. \
            Relentless Assault) in this batch.",
        ),
        (
            n("Action", "BecomeDay"),
            "DEFERRED: no day/night real card (e.g. Alrund's Epiphany) in \
            this batch.",
        ),
        (
            n("Action", "BecomeNight"),
            "DEFERRED: see Action::BecomeDay.",
        ),
        (
            n("Action", "TheRingTempts"),
            "DEFERRED: no 'the Ring tempts you' real card in this \
            batch.",
        ),
        (
            n("PlayerAction", "VentureIntoDungeon"),
            "DEFERRED: no dungeon-venture real card in this \
            batch.",
        ),
        (
            n("PlayerAction", "Untap"),
            "DEFERRED: no untap-a-permanent-as-an-effect real card \
            (distinct from a cost's {Q}) in this batch.",
        ),
        (
            n("PlayerAction", "GetEmblem"),
            "DEFERRED: no emblem-granting real card in this batch.",
        ),
        (
            n("PlayerAction", "ChooseAndNote"),
            "BLOCKED: the note-store has no engine runtime yet \
            (ChooseAndNote/Count::Noted/Reference::Linked are `todo!()`, P0.W4/W5-tracked) and its \
            only currently-legible domain (Number) has no simple real single-effect card in this \
            batch (Three Tree City, the ticket's suggested card, needs a Color+creature-type \
            domain NotedKind doesn't have) — see the ticket's completion notes.",
        ),
        (
            n("PlayerAction", "CopySpell"),
            "DEFERRED: no copy-on-the-stack real card (e.g. Twincast) \
            in this batch.",
        ),
        (
            n("PlayerAction", "FlipCoins"),
            "DEFERRED: no coin-flip real card in this batch.",
        ),
        (
            n("PlayerAction", "RollDice"),
            "DEFERRED: no dice-rolling real card in this batch.",
        ),
        (
            n("PlayerAction", "WinGame"),
            "DEFERRED: no 'you win the game' real card (e.g. Test of \
            Endurance) in this batch.",
        ),
        (
            n("PlayerAction", "LoseGame"),
            "DEFERRED: no 'a player loses the game' real card in this \
            batch.",
        ),
        (
            n("PlayerAction", "RestartGame"),
            "DEFERRED: no restart-the-game real card (e.g. Karn \
            Liberated's -14) in this batch.",
        ),
        (
            n("PlayerAction", "Shuffle"),
            "DEFERRED: no bare shuffle-the-library real card (distinct \
            from a search's shuffle-after) in this batch.",
        ),
        (
            n("PlayerAction", "SetLife"),
            "DEFERRED: no life-total-SET real card (e.g. 'set your life \
            total to 1') in this batch.",
        ),
        (
            n("PlayerAction", "Reveal"),
            "DEFERRED: no bare Reveal-only real card in this batch \
            (canon's reveal-adjacent cards fold revealing into SeparatePiles).",
        ),
        (
            n("Duration", "UntilEvent"),
            "DEFERRED: no 'until (event) happens' one-shot-duration real \
            card in this batch.",
        ),
        (
            n("Duration", "ForAsLongAs"),
            "DEFERRED: no 'for as long as (condition)' one-shot-duration \
            real card in this batch.",
        ),
        (
            n("Duration", "EndOfGame"),
            "DEFERRED: no one-shot (non-static) permanent grant in this \
            batch — the corpus's permanent effects all ride genuine static abilities.",
        ),
        (
            n("Scope", "These"),
            "DEFERRED: no fixed-object-list (non-filter, non-single) Modify scope \
            real card in this batch.",
        ),
        (
            n("CollectionOp", "Remove"),
            "DEFERRED: no single-element 'loses a color/type/subtype' \
            real card in this batch.",
        ),
        (
            n("Modification", "SwitchPowerToughness"),
            "DEFERRED: no P/T-switch real card (e.g. \
            Twisted Image) in this batch.",
        ),
        (
            n("Modification", "Supertypes"),
            "DEFERRED: no supertype-changing real card in this \
            batch.",
        ),
        (
            n("Modification", "CantHaveAbility"),
            "DEFERRED: no \"can't have or gain abilities\" real \
            card in this batch.",
        ),
        (
            n("Modification", "SetController"),
            "DEFERRED: no one-shot control-change-via-Modify real \
            card in this batch (canon has no control magic yet).",
        ),
        (
            n("Modification", "SetText"),
            "DEFERRED: no text-changing real card (e.g. Volrath's \
            Shapeshifter) in this batch.",
        ),
        (
            n("Modification", "BaseLoyalty"),
            "BLOCKED: no loyalty-ability cost grammar exists at all \
            yet (confirmed while scoping Liliana of the Veil for the SeparatePiles backfill) — a \
            planeswalker card is out of reach until that lands; see the ticket's completion notes.",
        ),
        (
            n("Modification", "BaseDefense"),
            "DEFERRED: no Battle-type card in canon yet.",
        ),
        (
            n("Modification", "BecomeBasicLandType"),
            "DEFERRED: no Blood-Moon-shaped real card in \
            this batch.",
        ),
        (
            n("StaticEffect", "CantPrevent"),
            "DEFERRED: no damage-can't-be-prevented real card in \
            this batch.",
        ),
        (
            n("StaticEffect", "SpendAsThough"),
            "DEFERRED: no mana-counterfactual real card in this \
            batch.",
        ),
        (
            n("StaticEffect", "AsThough"),
            "DEFERRED: no AsThough-shaped counterfactual real card in \
            this batch.",
        ),
        (
            n("StaticEffect", "OutcomeGate"),
            "DEFERRED: no can't-win/can't-lose real card (e.g. \
            Platinum Angel) in this batch.",
        ),
        (
            n("OutcomeGateKind", "CantLose"),
            "DEFERRED: see StaticEffect::OutcomeGate.",
        ),
        (
            n("OutcomeGateKind", "CantWin"),
            "DEFERRED: see StaticEffect::OutcomeGate.",
        ),
        (
            n("PlayerAttr", "Life"),
            "DEFERRED: no life-total player-attribute real card beyond \
            Exploration/Reliquary Tower's LandPlaysPerTurn/HandSizeLimit in this batch.",
        ),
        (
            n("PlayerAttr", "HandSize"),
            "DEFERRED: see PlayerAttr::Life.",
        ),
        (
            n("PlayerMod", "SetTo"),
            "DEFERRED: no life-total-SET-via-ModifyPlayer real card in this \
            batch (Exploration/Reliquary Tower cover Raise/NoMax).",
        ),
        (
            n("PlayerMod", "Lower"),
            "DEFERRED: no hand-size-LOWER real card in this batch.",
        ),
        (
            n("PhaseKind", "Combat"),
            "DEFERRED: no combat-phase-scoped real card beyond canon's \
            existing main-phase ones in this batch.",
        ),
        (
            n("PhaseKind", "PostcombatMain"),
            "DEFERRED: see PhaseKind::Combat.",
        ),
        (n("PhaseStep", "Combat"), "DEFERRED: see PhaseKind::Combat."),
        (
            n("PhaseStep", "PostcombatMain"),
            "DEFERRED: see PhaseKind::Combat.",
        ),
        (
            n("BeginningStep", "Untap"),
            "DEFERRED: no untap-step-triggered real card in this batch.",
        ),
        (
            n("CombatStep", "BeginningOfCombat"),
            "DEFERRED: no combat-step-triggered real card in \
            this batch (canon has no upkeep/combat-step-triggered permanent yet) — a real \
            'at the beginning of combat on each opponent's turn' card would close this whole \
            family at once.",
        ),
        (
            n("CombatStep", "DeclareAttackers"),
            "DEFERRED: see CombatStep::BeginningOfCombat.",
        ),
        (
            n("CombatStep", "DeclareBlockers"),
            "DEFERRED: see CombatStep::BeginningOfCombat.",
        ),
        (
            n("CombatStep", "FirstCombatDamage"),
            "DEFERRED: see CombatStep::BeginningOfCombat.",
        ),
        (
            n("CombatStep", "CombatDamage"),
            "DEFERRED: see CombatStep::BeginningOfCombat.",
        ),
        (
            n("CombatStep", "EndOfCombat"),
            "DEFERRED: see CombatStep::BeginningOfCombat.",
        ),
        (
            n("EndingStep", "Cleanup"),
            "DEFERRED: no cleanup-step-triggered real card in this batch.",
        ),
        (
            n("WhoseTurn", "AnOpponents"),
            "DEFERRED: no opponents'-turn-scoped triggered real card in \
            this batch.",
        ),
        (
            n("StateChange", "Untapped"),
            "DEFERRED: no untap-event-triggered real card in this \
            batch.",
        ),
        (
            n("StateChange", "Phased"),
            "DEFERRED: no phasing-event-triggered real card in this \
            batch.",
        ),
        (
            n("StateChange", "TurnedFace"),
            "DEFERRED: no turned-face-up/down-triggered real card in \
            this batch.",
        ),
        (
            n("Agency", "CostPayment"),
            "DEFERRED: no card in this batch explicitly narrows a Cause \
            pattern by `agency` (real cards distinguish by `verb` alone, e.g. 'whenever you \
            sacrifice'); the whole Agency family needs one narrowly agency-scoped trigger.",
        ),
        (
            n("Agency", "AttackDeclaration"),
            "DEFERRED: see Agency::CostPayment.",
        ),
        (
            n("Agency", "EffectInstruction"),
            "DEFERRED: see Agency::CostPayment.",
        ),
        (
            n("Agency", "TurnBasedAction"),
            "DEFERRED: see Agency::CostPayment.",
        ),
        (
            n("Agency", "StateBasedAction"),
            "DEFERRED: see Agency::CostPayment.",
        ),
        (
            n("Agency", "ManaAbilityResolution"),
            "DEFERRED: see Agency::CostPayment.",
        ),
        (
            n("Agency", "SpecialAction"),
            "DEFERRED: see Agency::CostPayment.",
        ),
        (
            n("CauseVerb", "Play"),
            "DEFERRED: no card in this batch narrows a Cause pattern by this \
            verb (Destroy/Sacrifice/Exile/Fight are exercised via Do or Die/Fling/Otherworldly \
            Journey's DestroyNoRegen).",
        ),
        (n("CauseVerb", "Explore"), "DEFERRED: see CauseVerb::Play."),
        (
            n("EventFilter", "LifeGained"),
            "DEFERRED: no life-gain-triggered real card in this \
            batch (canon's existing triggers are all ZoneChange-shaped: dies/enters).",
        ),
        (
            n("EventFilter", "LifeLost"),
            "DEFERRED: see EventFilter::LifeGained.",
        ),
        (
            n("EventFilter", "Drawn"),
            "DEFERRED: see EventFilter::LifeGained.",
        ),
        (
            n("EventFilter", "CounterRemoved"),
            "DEFERRED: see EventFilter::LifeGained.",
        ),
        (
            n("EventFilter", "Played"),
            "DEFERRED: see EventFilter::LifeGained.",
        ),
        (
            n("EventFilter", "ActivatedAb"),
            "DEFERRED: see EventFilter::LifeGained.",
        ),
        (
            n("EventFilter", "Attached"),
            "DEFERRED: see EventFilter::LifeGained.",
        ),
        (
            n("EventFilter", "DesignationChanged"),
            "DEFERRED: see EventFilter::LifeGained.",
        ),
        (
            n("EventFilter", "TokenCreated"),
            "DEFERRED: see EventFilter::LifeGained.",
        ),
        (
            n("EventFilter", "Used"),
            "DEFERRED: see EventFilter::LifeGained.",
        ),
        (
            n("EventFilter", "CoinFlipped"),
            "DEFERRED: see EventFilter::LifeGained.",
        ),
        (
            n("EventFilter", "DiceRolled"),
            "DEFERRED: see EventFilter::LifeGained.",
        ),
        (
            n("EventFilter", "BecameDay"),
            "DEFERRED: see EventFilter::LifeGained.",
        ),
        (
            n("EventFilter", "BecameNight"),
            "DEFERRED: see EventFilter::LifeGained.",
        ),
        (
            n("EventFilter", "Nth"),
            "DEFERRED: no Nth-occurrence-gated trigger real card in this \
            batch.",
        ),
        (
            n("EventFilter", "Within"),
            "DEFERRED: no history-lane 'within' real card in this \
            batch.",
        ),
        (
            n("RoundMode", "RoundUp"),
            "DEFERRED: no Count::Half-using real card (e.g. 'half its \
            power, rounded up') in this batch.",
        ),
        (
            n("RoundMode", "RoundDown"),
            "DEFERRED: see RoundMode::RoundUp.",
        ),
        (
            n("Characteristic", "Types"),
            "DEFERRED: no CountDistinct-over-card-types real card in \
            this batch (Domain covers Subtypes/BasicLandTypes).",
        ),
        (
            n("Characteristic", "Supertypes"),
            "DEFERRED: see Characteristic::Types.",
        ),
        (
            n("Characteristic", "ManaCost"),
            "DEFERRED: see Characteristic::Types.",
        ),
        (
            n("Count", "Max"),
            "DEFERRED: no 'the greater of X and Y' real card in this batch.",
        ),
        (
            n("Count", "Plus"),
            "DEFERRED: no 'X plus Y' arithmetic real card in this batch.",
        ),
        (
            n("Count", "Minus"),
            "DEFERRED: no 'X minus Y' arithmetic real card in this batch.",
        ),
        (
            n("Count", "Times"),
            "DEFERRED: no 'twice X' arithmetic real card in this batch.",
        ),
        (
            n("Count", "Half"),
            "DEFERRED: no 'half its power' real card in this batch.",
        ),
        (
            n("Count", "ThatMany"),
            "DEFERRED: no 'that many' amount-anaphor real card in this batch \
            (Collective Defiance would have needed it; Collective Resistance, the card actually \
            built, doesn't).",
        ),
        (
            n("Count", "EventCount"),
            "DEFERRED: no history-fact-count real card (morbid/raid-shaped) \
            in this batch.",
        ),
        (
            n("Count", "EventSum"),
            "DEFERRED: no history-fact-sum real card (e.g. total life lost \
            this turn) in this batch.",
        ),
        (
            n("Count", "Noted"),
            "BLOCKED: see PlayerAction::ChooseAndNote — its only reader, with \
            the same engine-runtime and domain gaps.",
        ),
        (
            n("Condition", "Exists"),
            "DEFERRED: no 'if you control a ...' conditional real card in \
            this batch.",
        ),
        (
            n("Condition", "YourTurn"),
            "DEFERRED: no 'during your turn' conditional real card in \
            this batch.",
        ),
        (
            n("Condition", "TurnOf"),
            "DEFERRED: no 'during an opponent's turn' conditional real card \
            in this batch.",
        ),
        (
            n("Condition", "DuringPhase"),
            "DEFERRED: no phase-gated conditional real card in this \
            batch.",
        ),
        (
            n("Reference", "EventPatient"),
            "DEFERRED: no real card in this batch reads the two-object \
            event's acted-upon side (EventActor/EventObject are exercised via Fling/Do or Die).",
        ),
        (
            n("Reference", "DefendingPlayer"),
            "DEFERRED: no landwalk/Annihilator-shaped real card in \
            this batch.",
        ),
        (
            n("Reference", "Bound"),
            "DEFERRED: no attacker/blocker-role-bound real card in this \
            batch.",
        ),
        (
            n("Reference", "Linked"),
            "BLOCKED: see PlayerAction::ChooseAndNote — the only reader for \
            an Objects/CardName-domain note, same engine-runtime gap.",
        ),
        (
            n("Reference", "OwnerOf"),
            "DEFERRED: no real card in this batch reads an object's OWNER \
            as distinct from its controller (Otherworldly Journey uses UnderOwnersControl, a \
            different EnterRider node, not this Reference).",
        ),
        (
            n("CostChange", "Increase"),
            "DEFERRED: no taxing effect (e.g. 'spells your opponents \
            cast cost {1} more') real card in this batch (Reduce is covered by an existing \
            affinity-shaped card).",
        ),
        (
            n("CostChange", "Additional"),
            "DEFERRED: no MANDATORY continuous cost-modifier real \
            card ('spells of type X cost an additional {R} to cast') in this batch — distinct \
            from Fling's `Effect::AdditionalCost`, which is the per-spell PRINTED clause, not a \
            `StaticEffect::CostModifier`.",
        ),
    ]
}

/// Reject-side exemptions: a node with no reject fixture, reviewed per
/// entry (not a dumping ground) — see the three shared reasons above for
/// what each category actually means.
fn reject_allowlist() -> Vec<(Node, &'static str)> {
    let mut v = vec![
        (
            n("Effect", "Continuously"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (n("Effect", "Until"), GENERIC_GATE_ALREADY_DEMONSTRATED),
        (n("Effect", "May"), GENERIC_GATE_ALREADY_DEMONSTRATED),
        (n("Effect", "MayPay"), GENERIC_GATE_ALREADY_DEMONSTRATED),
        (n("Effect", "MustPay"), GENERIC_GATE_ALREADY_DEMONSTRATED),
        (n("Effect", "Noting"), GENERIC_GATE_ALREADY_DEMONSTRATED),
        (n("Effect", "Reflexive"), GENERIC_GATE_ALREADY_DEMONSTRATED),
        (n("Bin", "Top"), NO_ILLEGAL_VALUE),
        (n("Bin", "Bottom"), NO_ILLEGAL_VALUE),
        (n("Anchor", "FromTop"), NO_ILLEGAL_VALUE),
        (n("Anchor", "FromBottom"), NO_ILLEGAL_VALUE),
        (n("EnterRider", "FaceDown"), RIDER_POSITION_GENERIC),
        (n("EnterRider", "UnderControlOf"), RIDER_POSITION_GENERIC),
        (
            n("EnterRider", "UnderOwnersControl"),
            RIDER_POSITION_GENERIC,
        ),
        (n("EnterRider", "Attacking"), RIDER_POSITION_GENERIC),
        (n("EnterRider", "WithCounters"), RIDER_POSITION_GENERIC),
        (n("Arrangement", "ChosenOrder"), NO_ILLEGAL_VALUE),
        (n("Arrangement", "AnyOrder"), NO_ILLEGAL_VALUE),
        (n("Arrangement", "SameOrder"), NO_ILLEGAL_VALUE),
        (n("Arrangement", "RandomOrder"), NO_ILLEGAL_VALUE),
        (
            n("Action", "ReturnToHand"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (n("Action", "Attach"), GENERIC_GATE_ALREADY_DEMONSTRATED),
        (n("Action", "Unattach"), GENERIC_GATE_ALREADY_DEMONSTRATED),
        (n("Action", "MoveGroup"), GENERIC_GATE_ALREADY_DEMONSTRATED),
        (n("Action", "ExtraPhase"), GENERIC_GATE_ALREADY_DEMONSTRATED),
        (n("Action", "BecomeDay"), GENERIC_GATE_ALREADY_DEMONSTRATED),
        (
            n("Action", "BecomeNight"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (
            n("Action", "TheRingTempts"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (
            n("Action", "MoveCounters"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (
            n("Action", "CreateReplacement"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (
            n("PlayerAction", "Discard"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (
            n("PlayerAction", "AddMana"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (n("PlayerAction", "Mill"), GENERIC_GATE_ALREADY_DEMONSTRATED),
        (
            n("PlayerAction", "VentureIntoDungeon"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (n("PlayerAction", "Tap"), GENERIC_GATE_ALREADY_DEMONSTRATED),
        (
            n("PlayerAction", "Untap"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (
            n("PlayerAction", "GetEmblem"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (
            n("PlayerAction", "CopySpell"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (
            n("PlayerAction", "FlipCoins"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (
            n("PlayerAction", "RollDice"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (
            n("PlayerAction", "RemoveCounters"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (
            n("PlayerAction", "Distribute"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (
            n("PlayerAction", "LoseGame"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (
            n("PlayerAction", "RestartGame"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (
            n("PlayerAction", "Shuffle"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (
            n("PlayerAction", "SetLife"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (
            n("PlayerAction", "RemoveDamage"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (n("Duration", "FixedUntil"), NO_ILLEGAL_VALUE),
        (n("Duration", "UntilEvent"), NO_ILLEGAL_VALUE),
        (n("Duration", "ForAsLongAs"), NO_ILLEGAL_VALUE),
        (n("Duration", "ForThisEvent"), NO_ILLEGAL_VALUE),
        (n("Duration", "EndOfGame"), NO_ILLEGAL_VALUE),
        (n("Scope", "These"), GENERIC_GATE_ALREADY_DEMONSTRATED),
        (n("Scope", "Matching"), GENERIC_GATE_ALREADY_DEMONSTRATED),
        (n("NumericOp", "Set"), AXIS_OP_TYPE_ENFORCED),
        (n("NumericOp", "Down"), AXIS_OP_TYPE_ENFORCED),
        (n("CollectionOp", "Set"), AXIS_OP_TYPE_ENFORCED),
        (n("CollectionOp", "Add"), AXIS_OP_TYPE_ENFORCED),
        (n("CollectionOp", "Remove"), AXIS_OP_TYPE_ENFORCED),
        (n("Modification", "Toughness"), AXIS_OP_TYPE_ENFORCED),
        (
            n("Modification", "SwitchPowerToughness"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (n("Modification", "Colors"), AXIS_OP_TYPE_ENFORCED),
        (n("Modification", "CardTypes"), AXIS_OP_TYPE_ENFORCED),
        (n("Modification", "Subtypes"), AXIS_OP_TYPE_ENFORCED),
        (n("Modification", "Supertypes"), AXIS_OP_TYPE_ENFORCED),
        (
            n("Modification", "GainAbility"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (
            n("Modification", "LoseAbility"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (
            n("Modification", "LoseAllAbilities"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (
            n("Modification", "CantHaveAbility"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (
            n("Modification", "SetController"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (
            n("Modification", "SetText"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (
            n("Modification", "AllCreatureTypes"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (n("Modification", "BaseLoyalty"), AXIS_OP_TYPE_ENFORCED),
        (n("Modification", "BaseDefense"), AXIS_OP_TYPE_ENFORCED),
        (
            n("Modification", "BecomeBasicLandType"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (
            n("CostChange", "Increase"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (n("CostChange", "Reduce"), GENERIC_GATE_ALREADY_DEMONSTRATED),
        (
            n("CostChange", "Additional"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (n("CostChange", "Scaled"), GENERIC_GATE_ALREADY_DEMONSTRATED),
        (
            n("StaticEffect", "Modify"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (
            n("StaticEffect", "CostModifier"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (
            n("StaticEffect", "CostOption"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (
            n("StaticEffect", "TriggerMultiplier"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (
            n("StaticEffect", "ModifyPlayer"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (
            n("StaticEffect", "SpendAsThough"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (
            n("StaticEffect", "AsThough"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (n("StaticEffect", "Sba"), GENERIC_GATE_ALREADY_DEMONSTRATED),
        (
            n("StaticEffect", "OutcomeGate"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (
            n("StaticEffect", "PayPips"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (n("OutcomeGateKind", "CantLose"), NO_ILLEGAL_VALUE),
        (n("OutcomeGateKind", "CantWin"), NO_ILLEGAL_VALUE),
        (n("PipClass", "Generic"), NO_ILLEGAL_VALUE),
        (n("PipClass", "Colored"), NO_ILLEGAL_VALUE),
        (n("PayAct", "TapToPay"), NO_ILLEGAL_VALUE),
        (n("PayAct", "ExileToPay"), NO_ILLEGAL_VALUE),
        (n("PlayerAttr", "Life"), NO_ILLEGAL_VALUE),
        (n("PlayerAttr", "HandSize"), NO_ILLEGAL_VALUE),
        (n("PlayerAttr", "HandSizeLimit"), NO_ILLEGAL_VALUE),
        (n("PlayerAttr", "LandPlaysPerTurn"), NO_ILLEGAL_VALUE),
        (n("PlayerMod", "SetTo"), NO_ILLEGAL_VALUE),
        (n("PlayerMod", "Raise"), NO_ILLEGAL_VALUE),
        (n("PlayerMod", "Lower"), NO_ILLEGAL_VALUE),
        (n("PlayerMod", "NoMax"), NO_ILLEGAL_VALUE),
        (n("PhaseKind", "PrecombatMain"), NO_ILLEGAL_VALUE),
        (n("PhaseKind", "PostcombatMain"), NO_ILLEGAL_VALUE),
        (n("PhaseStep", "PrecombatMain"), NO_ILLEGAL_VALUE),
        (n("PhaseStep", "PostcombatMain"), NO_ILLEGAL_VALUE),
        (n("BeginningStep", "Untap"), NO_ILLEGAL_VALUE),
        (n("CombatStep", "BeginningOfCombat"), NO_ILLEGAL_VALUE),
        (n("CombatStep", "DeclareAttackers"), NO_ILLEGAL_VALUE),
        (n("CombatStep", "DeclareBlockers"), NO_ILLEGAL_VALUE),
        (n("CombatStep", "FirstCombatDamage"), NO_ILLEGAL_VALUE),
        (n("CombatStep", "CombatDamage"), NO_ILLEGAL_VALUE),
        (n("CombatStep", "EndOfCombat"), NO_ILLEGAL_VALUE),
        (n("EndingStep", "Cleanup"), NO_ILLEGAL_VALUE),
        (n("WhoseTurn", "AnOpponents"), NO_ILLEGAL_VALUE),
        (n("StateChange", "Untapped"), NO_ILLEGAL_VALUE),
        (n("Agency", "CostPayment"), NO_ILLEGAL_VALUE),
        (n("Agency", "AttackDeclaration"), NO_ILLEGAL_VALUE),
        (n("Agency", "EffectInstruction"), NO_ILLEGAL_VALUE),
        (n("Agency", "TurnBasedAction"), NO_ILLEGAL_VALUE),
        (n("Agency", "StateBasedAction"), NO_ILLEGAL_VALUE),
        (n("Agency", "ManaAbilityResolution"), NO_ILLEGAL_VALUE),
        (n("Agency", "SpecialAction"), NO_ILLEGAL_VALUE),
        (n("CauseVerb", "Discard"), NO_ILLEGAL_VALUE),
        (n("CauseVerb", "Mill"), NO_ILLEGAL_VALUE),
        (n("CauseVerb", "Explore"), NO_ILLEGAL_VALUE),
        (n("CauseVerb", "Regenerate"), NO_ILLEGAL_VALUE),
        (
            n("EventFilter", "LifeLost"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (
            n("EventFilter", "CounterPlaced"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (
            n("EventFilter", "CounterRemoved"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (
            n("EventFilter", "Played"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (
            n("EventFilter", "ActivatedAb"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (
            n("EventFilter", "BlockDeclared"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (
            n("EventFilter", "Attached"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (
            n("EventFilter", "ControlChanged"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (
            n("EventFilter", "DesignationChanged"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (
            n("EventFilter", "DiceRolled"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (
            n("EventFilter", "BecameDay"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (
            n("EventFilter", "BecameNight"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (n("Stat", "Toughness"), NO_ILLEGAL_VALUE),
        (n("Stat", "ManaValue"), NO_ILLEGAL_VALUE),
        (n("RoundMode", "RoundUp"), NO_ILLEGAL_VALUE),
        (n("RoundMode", "RoundDown"), NO_ILLEGAL_VALUE),
        (n("Characteristic", "Colors"), NO_ILLEGAL_VALUE),
        (n("Characteristic", "Types"), NO_ILLEGAL_VALUE),
        (n("Characteristic", "Subtypes"), NO_ILLEGAL_VALUE),
        (n("Characteristic", "BasicLandTypes"), NO_ILLEGAL_VALUE),
        (n("Characteristic", "Supertypes"), NO_ILLEGAL_VALUE),
        (n("Characteristic", "Toughness"), NO_ILLEGAL_VALUE),
        (n("Characteristic", "ManaCost"), NO_ILLEGAL_VALUE),
        (n("Characteristic", "Name"), NO_ILLEGAL_VALUE),
        (n("Count", "CountOf"), GENERIC_GATE_ALREADY_DEMONSTRATED),
        (
            n("Count", "CountDistinct"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (
            n("Count", "CounterCount"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (n("Count", "Min"), GENERIC_GATE_ALREADY_DEMONSTRATED),
        (n("Count", "Max"), GENERIC_GATE_ALREADY_DEMONSTRATED),
        (n("Count", "Plus"), GENERIC_GATE_ALREADY_DEMONSTRATED),
        (n("Count", "Minus"), GENERIC_GATE_ALREADY_DEMONSTRATED),
        (n("Count", "Times"), GENERIC_GATE_ALREADY_DEMONSTRATED),
        (n("Count", "TimesPaid"), GENERIC_GATE_ALREADY_DEMONSTRATED),
        (n("Cmp", "AtMost"), NO_ILLEGAL_VALUE),
        (n("Cmp", "Greater"), NO_ILLEGAL_VALUE),
        (n("Cmp", "Less"), NO_ILLEGAL_VALUE),
        (n("Condition", "Compare"), GENERIC_GATE_ALREADY_DEMONSTRATED),
        (n("Condition", "Exists"), GENERIC_GATE_ALREADY_DEMONSTRATED),
        (n("Condition", "Is"), GENERIC_GATE_ALREADY_DEMONSTRATED),
        (
            n("Condition", "LegallyAttached"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (
            n("Condition", "DamagedByDeathtouch"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (n("Condition", "Crossed"), GENERIC_GATE_ALREADY_DEMONSTRATED),
        (
            n("Condition", "PaidCost"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (n("Condition", "TurnOf"), GENERIC_GATE_ALREADY_DEMONSTRATED),
        (
            n("Condition", "DuringPhase"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (n("Reference", "Linked"), GENERIC_GATE_ALREADY_DEMONSTRATED),
        (
            n("Reference", "ControllerOf"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (n("Reference", "OwnerOf"), GENERIC_GATE_ALREADY_DEMONSTRATED),
        (
            n("Reference", "AttachHostOf"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
        (
            n("Reference", "AttachedTo"),
            GENERIC_GATE_ALREADY_DEMONSTRATED,
        ),
    ];
    v.sort();
    v
}

/// Shorthand for building a `Node` key in the allowlists above.
fn n(enum_name: &str, variant: &str) -> Node {
    (enum_name.to_string(), variant.to_string())
}

/// Recursively collects the text of every `*.ron` file under `root` (empty
/// if `root` doesn't exist).
fn ron_texts_under(root: &Path) -> Vec<String> {
    let mut out = Vec::new();
    collect_ron_texts(root, &mut out);
    out
}

fn collect_ron_texts(dir: &Path, out: &mut Vec<String>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    let mut paths: Vec<PathBuf> = entries.filter_map(|e| e.ok()).map(|e| e.path()).collect();
    paths.sort();
    for path in paths {
        if path.is_dir() {
            collect_ron_texts(&path, out);
        } else if path.extension().is_some_and(|ext| ext == "ron") {
            if let Ok(text) = std::fs::read_to_string(&path) {
                out.push(text);
            }
        }
    }
}

/// The accept corpus: every `.ron` file (cards AND macro bodies — a macro's
/// `body:` field spells real primitive nodes) under the three loading
/// plugins the ticket names.
fn accept_corpus() -> Vec<String> {
    let plugins = plugins_root();
    ["canon", "testing", "builtin"]
        .iter()
        .flat_map(|root| ron_texts_under(&plugins.join(root)))
        .collect()
}

fn reject_corpus() -> Vec<String> {
    ron_texts_under(&reject_root())
}

/// Whether `name` appears as a whole-word token anywhere in `corpus` — a
/// plain text search (never a typed AST walk), so a node reached only
/// through a macro's EXPANSION is still credited via the macro's own
/// `body:` RON, which spells the real primitive tag.
fn appears_as_token(corpus: &[String], name: &str) -> bool {
    let pattern = format!(r"\b{}\b", regex::escape(name));
    let re = Regex::new(&pattern).unwrap_or_else(|e| panic!("bad pattern for {name}: {e}"));
    corpus.iter().any(|text| re.is_match(text))
}

/// The reusable checking logic, exercised below both on synthetic data (RED
/// case, `synthetic_gap_is_caught`) and on the real tree
/// (`no_dead_grammar_nodes`): every node in `inventory` must satisfy
/// `covered`, unless it's in `untagged` or keyed in `allowlist` — returns the
/// uncovered, unexempted nodes.
fn find_uncovered<'a>(
    inventory: &'a [Node],
    covered: impl Fn(&str) -> bool,
    untagged: &BTreeSet<Node>,
    allowlist: &[(Node, &'static str)],
) -> Vec<&'a Node> {
    inventory
        .iter()
        .filter(|node| !untagged.contains(*node))
        .filter(|node| !allowlist.iter().any(|(allowed, _)| allowed == *node))
        .filter(|(_, variant)| !covered(variant))
        .collect()
}

/// The mechanism itself, RED on a synthetic gap: a tiny fake inventory with
/// one variant present in the corpus and one absent must flag exactly the
/// absent one, whether or not it's exempted.
#[test]
fn synthetic_gap_is_caught() {
    let inventory: Vec<Node> = vec![
        ("Fake".to_string(), "Present".to_string()),
        ("Fake".to_string(), "Missing".to_string()),
        ("Fake".to_string(), "AlsoMissingButAllowed".to_string()),
    ];
    let corpus = vec!["Present(1)".to_string()];
    let untagged = BTreeSet::new();
    let allowlist = vec![(
        ("Fake".to_string(), "AlsoMissingButAllowed".to_string()),
        "synthetic: reviewed exemption",
    )];

    let uncovered = find_uncovered(
        &inventory,
        |name| appears_as_token(&corpus, name),
        &untagged,
        &allowlist,
    );
    assert_eq!(
        uncovered,
        vec![&("Fake".to_string(), "Missing".to_string())],
        "the checker must flag exactly the uncovered, unexempted node — RED on a real gap, \
         quiet on both the present node and the allowlisted one"
    );

    // GREEN once the gap fixture "lands" (the corpus now spells it too).
    let fixed_corpus = vec!["Present(1)".to_string(), "Missing(2)".to_string()];
    let uncovered = find_uncovered(
        &inventory,
        |name| appears_as_token(&fixed_corpus, name),
        &untagged,
        &allowlist,
    );
    assert!(
        uncovered.is_empty(),
        "once every non-exempt node is spelled somewhere in the corpus, nothing is uncovered"
    );
}

/// Every allowlist/exemption entry must still name a REAL node in the
/// current inventory — otherwise it's stale cruft the reviewer can no
/// longer connect to anything (the ticket's "reviewed, not a dumping
/// ground" bar).
#[test]
fn allowlists_name_only_real_nodes() {
    let inventory: BTreeSet<Node> = grammar_inventory().into_iter().collect();
    for node in structurally_untagged() {
        assert!(
            inventory.contains(&node),
            "structurally_untagged names {node:?}, which is not a current grammar node \
             (stale entry — the variant was renamed or removed)"
        );
    }
    for (node, reason) in accept_allowlist() {
        assert!(
            inventory.contains(&node),
            "accept_allowlist names {node:?} ({reason}), which is not a current grammar node"
        );
        assert!(!reason.is_empty(), "{node:?} has an empty allowlist reason");
    }
    for (node, reason) in reject_allowlist() {
        assert!(
            inventory.contains(&node),
            "reject_allowlist names {node:?} ({reason}), which is not a current grammar node"
        );
        assert!(!reason.is_empty(), "{node:?} has an empty allowlist reason");
    }
}

/// The real sweep: every core grammar node (effects/actions/statics/events/
/// counts/conditions/references) has at least one accept-corpus fixture and
/// at least one reject fixture, or is named with a reason in the matching
/// allowlist above.
#[test]
fn no_dead_grammar_nodes() {
    let inventory = grammar_inventory();
    assert!(
        inventory.len() > 100,
        "sanity floor: the seven grammar files should yield well over 100 variants; got {} \
         (a parse likely silently came back empty)",
        inventory.len()
    );

    let untagged = structurally_untagged();
    let accept_allow = accept_allowlist();
    let reject_allow = reject_allowlist();

    let accept_texts = accept_corpus();
    let reject_texts = reject_corpus();
    assert!(
        !accept_texts.is_empty() && !reject_texts.is_empty(),
        "the accept/reject corpora must not be empty (a path likely resolved wrong)"
    );

    let uncovered_accept = find_uncovered(
        &inventory,
        |name| appears_as_token(&accept_texts, name),
        &untagged,
        &accept_allow,
    );
    let uncovered_reject = find_uncovered(
        &inventory,
        |name| appears_as_token(&reject_texts, name),
        &untagged,
        &reject_allow,
    );

    if !uncovered_accept.is_empty() || !uncovered_reject.is_empty() {
        let mut msg = String::new();
        if !uncovered_accept.is_empty() {
            msg.push_str(&format!(
                "\n{} node(s) with NO accept fixture under plugins/{{canon,testing,builtin}} \
                 (add a card, or a reviewed accept_allowlist() entry):\n",
                uncovered_accept.len()
            ));
            for (enum_name, variant) in &uncovered_accept {
                msg.push_str(&format!("  - {enum_name}::{variant}\n"));
            }
        }
        if !uncovered_reject.is_empty() {
            msg.push_str(&format!(
                "\n{} node(s) with NO reject fixture under tests/reject/ (add a fixture, or a \
                 reviewed reject_allowlist() entry):\n",
                uncovered_reject.len()
            ));
            for (enum_name, variant) in &uncovered_reject {
                msg.push_str(&format!("  - {enum_name}::{variant}\n"));
            }
        }
        panic!("{msg}");
    }
}
