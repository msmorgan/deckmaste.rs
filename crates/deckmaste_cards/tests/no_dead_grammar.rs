//! The G12 "no dead grammar" coverage sweep
//! ([[cards-no-dead-grammar-sweep]]): every core grammar node — an enum
//! variant defined in one of the seven grammar family files (effects,
//! actions, statics, events, counts, conditions, references) — must carry at
//! least one loading ACCEPTANCE card (`plugins/{canon,testing,builtin}`,
//! cards or macro bodies), or be named, with a reviewed reason, in the
//! allowlist below.
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
//!  - `ACCEPT_ALLOWLIST`: a real gap, with a reason each entry's author stands
//!    behind.
//!
//! The load-time elaborator this sweep's reject-fixture half depended on
//! (a CR-tagged binding-context walk — `deckmaste_cards::elaborate`) was
//! deleted; anaphora/binding resolution now happens purely at engine eval
//! time, so there is no load-time gate left to demonstrate REJECT fixtures
//! against. The reject-side requirement and `tests/reject/` corpus are gone
//! with it — this sweep now only checks the accept side.

use std::collections::BTreeSet;
use std::fmt::Write as _;
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

/// One grammar node: the enum it belongs to, and its variant name (the tag a
/// card would spell, e.g. `OneShotEffect::Sequentially` -> `("OneShotEffect",
/// "Sequentially")`).
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
///  - `OneShotEffect::Act` / `StaticEffect::Deontic` / `Destination::Zone` are
///    `#[macro_ron(flatten)]`: the wrapping tag is fully invisible in RON —
///    only the PAYLOAD type's own variant names are ever spelled (a bare verb
///    like `Draw(1)` reads as `OneShotEffect::Act(Action::By(You, Draw(1)))`
///    with neither `Act` nor `Zone` ever written). Every one of these positions
///    is exercised constantly; there is simply nothing to grep for.
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
        ("OneShotEffect", "Act"),
        ("StaticEffect", "Deontic"),
        ("Destination", "Zone"),
        ("Count", "Literal"),
        // Every `SupportsMacros` enum's macro-invocation-remembering wrapper:
        // present on nearly every grammar enum, but its OWN tag is never
        // itself spelled — a macro invocation always serializes as the
        // invocation (`AddPowerToughness(2, 2)`), never literally as
        // `Expanded(...)`. A text search for the tag is structurally
        // meaningless for the same reason `Act`/`Zone`/`Deontic` are above.
        ("OneShotEffect", "Expanded"),
        ("PlayerAction", "Expanded"),
        ("Modification", "Expanded"),
        ("StaticEffect", "Expanded"),
        ("EventFilter", "Expanded"),
        ("Count", "Expanded"),
        ("Condition", "Expanded"),
        ("Reference", "Expanded"),
    ]
    .into_iter()
    .map(|(e, v)| (e.to_string(), v.to_string()))
    .collect()
}

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
#[expect(
    clippy::too_many_lines,
    reason = "a flat data table of reviewed allowlist entries with per-entry justification comments; it is one cohesive literal, not logic to decompose"
)]
fn accept_allowlist() -> Vec<(Node, &'static str)> {
    vec![
        (
            n("OneShotEffect", "Label"),
            "BLOCKED: Label's only real use-case in this batch (Blood Money's \
            'destroyed this way' product-group read-back) is the same confirmed grammar gap as \
            Noting/AmongNoted below — see the ticket's completion notes.",
        ),
        (
            n("OneShotEffect", "MayPay"),
            "DEFERRED: no real card in this batch uses the resolution-time \
            optional-cost-kicker shape (canon's Mana Leak uses the punisher MustPay instead); a \
            real candidate (e.g. a 'you may pay {2}; if you do, ...' spell) is buildable.",
        ),
        (
            n("OneShotEffect", "Noting"),
            "BLOCKED: 'for each nontoken creature destroyed this way' (Blood \
            Money) needs a way to re-read a noted PRODUCT group filtered further; \
            Selection::AmongNoted is a CHOICE primitive (wrong shape for an unconditional \
            'for each'), and no Count/Predicate combinator reads a noted object set at all — a \
            genuine missing-grammar finding, not built. See the ticket's completion notes.",
        ),
        (
            n("OneShotEffect", "Reflexive"),
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
            n("StaticEffect", "Conditionally"),
            "DEFERRED: the \"as long as [condition], [effect]\" wrapper \
            ([CR#611.3a]) is elaborated and rendered this session, but the engine gather is a \
            documented unwired seam (`static_effect_scope` skips it) and no real card in this \
            batch needs the qualifier over the graveyard/hand `from`-zone shape it replaces \
            (see `renders_graveyard_static_from_zone` for a synthetic exercise); buildable once \
            the gather seam is wired.",
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
            n("Count", "ManaAvailable"),
            "STRATEGY-ONLY: the floated-mana-pool reader senses a player's \
            unspent mana for data-driven strategy ramp gates; card text never reads it, so it has \
            no card fixture (and no Idris counterpart).",
        ),
        (
            n("Countable", "ManaSymbols"),
            "DEFERRED: the pip-count (devotion, [CR#700.5]) source is engine-\
            eval'd and unit-tested (deckmaste_engine::resolve::tests), but the card-facing \
            devotion fixture + render phrasing ride Count::Aggregate — a separate ticket task, not \
            yet built.",
        ),
        (
            n("Count", "Aggregate"),
            "DEFERRED: the fold (SumOf/MinOf/MaxOf/AverageOf over a \
            Projection, [CR#107.1]) is engine-eval'd and unit-tested \
            (deckmaste_engine::resolve::tests::aggregate_folds_a_projection_over_a_selection) and \
            round-trips (deckmaste_core::count::tests::aggregate_round_trips), but the card-facing \
            devotion fixture that would exercise it end-to-end (Count::Aggregate + \
            Countable::ManaSymbols) is a separate ticket task, not yet built.",
        ),
        (
            n("AggregateOp", "SumOf"),
            "DEFERRED: see Count::Aggregate — no accept fixture until the \
            devotion fixture card lands.",
        ),
        (
            n("AggregateOp", "MinOf"),
            "DEFERRED: see Count::Aggregate — no accept fixture until a real \
            card uses the extremal fold (Selection::Pick already covers the extremal-ELEMENT \
            shape; Pick unifying onto AggregateOp is a later ticket task).",
        ),
        (
            n("AggregateOp", "MaxOf"),
            "DEFERRED: see AggregateOp::MinOf.",
        ),
        (
            n("AggregateOp", "AverageOf"),
            "DEFERRED: see Count::Aggregate — no accept fixture until a real \
            card uses the average fold.",
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
            from Fling's `OneShotEffect::AdditionalCost`, which is the per-spell PRINTED clause, not a \
            `StaticEffect::CostModifier`.",
        ),
    ]
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
    let mut paths: Vec<PathBuf> = entries
        .filter_map(std::result::Result::ok)
        .map(|e| e.path())
        .collect();
    paths.sort();
    for path in paths {
        if path.is_dir() {
            collect_ron_texts(&path, out);
        } else if path.extension().is_some_and(|ext| ext == "ron")
            && let Ok(text) = std::fs::read_to_string(&path)
        {
            out.push(text);
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
}

/// The real sweep: every core grammar node (effects/actions/statics/events/
/// counts/conditions/references) has at least one accept-corpus fixture, or
/// is named with a reason in `accept_allowlist`.
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

    let accept_texts = accept_corpus();
    assert!(
        !accept_texts.is_empty(),
        "the accept corpus must not be empty (a path likely resolved wrong)"
    );

    let uncovered_accept = find_uncovered(
        &inventory,
        |name| appears_as_token(&accept_texts, name),
        &untagged,
        &accept_allow,
    );

    if !uncovered_accept.is_empty() {
        let mut msg = String::new();
        let _ = write!(
            msg,
            "\n{} node(s) with NO accept fixture under plugins/{{canon,testing,builtin}} \
             (add a card, or a reviewed accept_allowlist() entry):\n",
            uncovered_accept.len()
        );
        for (enum_name, variant) in &uncovered_accept {
            let _ = writeln!(msg, "  - {enum_name}::{variant}");
        }
        panic!("{msg}");
    }
}
