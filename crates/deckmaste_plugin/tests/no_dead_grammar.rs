//! The "no dead grammar" coverage sweep: every semantic grammar node — an enum
//! variant defined in one of the seven grammar family files (effects,
//! actions, statics, events, counts, conditions, references) — must carry at
//! least one loading ACCEPTANCE card (`plugins/{canon,testing,builtin}`,
//! cards or macro bodies), or be named, with a reviewed reason, in the
//! allowlist below.
//!
//! MECHANICAL, not hand-maintained: the inventory comes from parsing the
//! actual `deckmaste_semantics` source (`syn`), so a newly-minted grammar node is
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
//! (a CR-tagged binding-context walk — `deckmaste_plugin::elaborate`) was
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
/// `deckmaste_semantics/src/`.
/// (effects/actions/statics/events/counts/conditions/references).
const GRAMMAR_FILES: &[&str] = &[
    "effect.rs",
    "action.rs",
    "continuous.rs",
    "event.rs",
    "count.rs",
    "condition.rs",
    "reference.rs",
    "selection.rs",
    "filter.rs",
    "mana.rs",
];

fn semantics_src_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../deckmaste_semantics/src")
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
/// out of the inventory: only file-top-level enums count as semantic grammar.
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
    let dir = semantics_src_dir();
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
        // invocation (`PowerAndToughnessUp(2, 2)`), never literally as
        // `Expanded(...)`. A text search for the tag is structurally
        // meaningless for the same reason `Act`/`Zone`/`Deontic` are above.
        ("OneShotEffect", "Expanded"),
        ("Action", "Expanded"),
        ("Modification", "Expanded"),
        ("StaticEffect", "Expanded"),
        ("EventFilter", "Expanded"),
        ("Count", "Expanded"),
        ("Condition", "Expanded"),
        ("Reference", "Expanded"),
        // `Predicate`'s three compartments are `#[macro_ron(flatten)]` (the
        // same mechanism as `Destination::Zone` above): the compartment tag
        // NEVER appears in RON — only the inner atom's own variant name is
        // spelled (`Type(Creature)`, never `Characteristic(Type(Creature))`;
        // `InZone(Battlefield)`, never `State(InZone(...))`). A text search for
        // the wrapper tag is structurally meaningless; the atoms themselves are
        // exercised constantly.
        ("Predicate", "Characteristic"),
        ("Predicate", "State"),
        ("Predicate", "Relation"),
        // The mana enums' `#[serde(untagged)]` fall-through variants: the tag is
        // never a spelled token because the untagged arm serializes its payload
        // transparently. `ManaSpec::Specific` writes `White` (not
        // `Specific(White)`), `SimpleManaSymbol::Specific` and `ManaSymbol::Simple`
        // likewise lift the inner symbol, and `ManaProduction::Bare` writes the
        // bare spec (`AddMana(Literal(1), AnyColor)`, no `Bare(...)`). Each is
        // exercised on every mana card; there is simply nothing to grep for.
        ("ManaSpec", "Specific"),
        ("SimpleManaSymbol", "Specific"),
        ("ManaSymbol", "Simple"),
        ("ManaProduction", "Bare"),
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
/// grammar has a genuine, confirmed gap. A `DEFERRED`
/// note is a plain, honest backlog item: the node is buildable with a real
/// or `testing`-mock card (a candidate is usually named), nothing structural
/// blocks it, but it does not yet have fixture coverage. Treat every
/// `DEFERRED` entry as a to-do, not a soundness claim.
#[expect(
    clippy::too_many_lines,
    reason = "a flat data table of reviewed allowlist entries with per-entry justification comments; it is one cohesive literal, not logic to decompose"
)]
fn accept_allowlist() -> Vec<(Node, &'static str)> {
    vec![
        (
            n("OneShotEffect", "Label"),
            "BLOCKED: Label's real use-case (Blood Money's \
            'destroyed this way' product-group read-back) is the same confirmed grammar gap as \
            Noting/AmongNoted below.",
        ),
        (
            n("OneShotEffect", "Noting"),
            "BLOCKED: 'for each nontoken creature destroyed this way' (Blood \
            Money) needs a way to re-read a noted PRODUCT group filtered further; \
            Selection::AmongNoted is a CHOICE primitive (wrong shape for an unconditional \
            'for each'), and no Count/Predicate combinator reads a noted object set at all — a \
            genuine missing-grammar finding.",
        ),
        (
            n("OneShotEffect", "Reflexive"),
            "DEFERRED: no covered real 'when you do' reflexive-trigger card; \
            render support for Reflexive is also unbuilt (same family as Delayed).",
        ),
        (
            n("OneShotEffect", "Batch"),
            "SHELL: `Batch` is a deliberate \
            scaffold, semantically identical to `Repeat` for now — no card needs it yet \
            because there is no observable difference from `Repeat` to author toward. Its \
            true aggregate-count tier ([CR#616.1g] 'twice that many' replacements and aggregate \
            triggers) remains unimplemented.",
        ),
        (
            n("PileSource", "Noted"),
            "DEFERRED: the per-player noted-piles shape (Whims of the \
            Fates: SeparatePiles.note + ChoosePile(from: Noted(..))) is designed but not built.",
        ),
        (
            n("Anchor", "FromBottom"),
            "DEFERRED: no covered real card targets the BOTTOM of a \
            library by anchor (canon's library moves are all top-anchored); buildable.",
        ),
        (
            n("EnterRider", "FaceDown"),
            "DEFERRED: no covered morph/manifest real card.",
        ),
        (
            n("EnterRider", "AsCopy"),
            "DEFERRED: the grammar is present (an \
            EnterRider carrying the shared CopySpec, [CR#707.5]) and a documented never-panic \
            fizzle seam at every rider-consuming site; the layer-1a APPLICATION that would make \
            a real Clone-style card graduatable is engine-layers-1-copy-facedown-text's, not \
            built yet.",
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
            n("Action", "MoveGroup"),
            "DEFERRED: no real simultaneous-group-relocation card (e.g. \
            'return all creatures to hand') has coverage — Brainstorm's own group-move uses \
            Each+Move per element, not MoveGroup.",
        ),
        (
            n("Action", "ExtraPhase"),
            "DEFERRED: no extra-combat/extra-phase real card (e.g. \
            Relentless Assault) has coverage.",
        ),
        (
            n("Action", "Cease"),
            "ENGINE-INTERNAL: the cease-to-exist verb the copy-cease SBA \
            ([CR#704.5d,707.10a], sba.rs) speaks through — no card will ever spell it \
            directly (it has no Idris counterpart either, same gap() shape as ExtraPhase); \
            it exists purely so the SBA's removal routes through the \
            data-usable Action grammar (unit-testable, reusable) instead of building its \
            GameEvent inline. Not a DEFERRED buildable-someday gap — see the \
            copy-grammar report.",
        ),
        (
            n("Action", "Untap"),
            "DEFERRED: no untap-a-permanent-as-an-effect real card \
            (distinct from a cost's {Q}) has coverage.",
        ),
        (
            n("Action", "GetEmblem"),
            "DEFERRED: the engine (command-zone mint + static/triggered sourcing) and the \
            parse/render grammar arm now exist and are covered by unit tests, but no \
            emblem-granting real card is graduatable: every canon emblem-granter \
            (Garruk Cursed Huntsman, Daretti, Kiora, Ob Nixilis of the Black Oath, Dovin Baan, \
            Teferi's Talent, The Capitoline Triad, Professor Dellian Fel) is a fully-Unparsed \
            multi-ability planeswalker/permanent whose OTHER abilities need unimplemented \
            machinery (loyalty-ability activation, animation, prevention, token creation, \
            variable exile costs) — graduating one would drag in all of it. See the \
            the documented emblem limitations.",
        ),
        (
            n("Action", "SetGameDesignation"),
            "DEFERRED: the generic game-scope designation transition is wired, but no \
            day/night macro or real card uses it until engine-day-night lands.",
        ),
        (
            n("Action", "ChooseValue"),
            "BLOCKED: reader-gated engine runtime now exists for the note kinds that HAVE a reader \
            (Number → a declared register; Objects → the noted group / AmongNoted; Color/CardName/Piles \
            stay loud — no reader grammar), but no covered simple real single-effect card \
            spells ChooseAndNote (Three Tree City needs a Color+creature-type domain \
            NotedKind doesn't have).",
        ),
        (
            n("Action", "FlipCoins"),
            "DEFERRED: every real coin-flip card that PERFORMS a flip \
            branches on its own win/lose result ('If you win the flip, ...', Boompile/Mana \
            Screw/Chaotic Goo/Karplusan Minotaur) — that branch needs a Condition reading the \
            just-emitted flip's outcome. `CoinFlipped.won` is readable only via \
            `EventFilter::CoinFlipped`, a TRIGGER match against the already-recorded fact (e.g. \
            Chance Encounter's 'whenever you win a coin flip'); there is no in-body Condition \
            that reads the flip action's own result within the same effect chain, so this \
            branch shape remains unbuilt. Chance Encounter/ \
            Tavern Scoundrel-style 'whenever you WIN a coin flip' TRIGGER cards don't need this \
            action at all (the flip happens off-card) — see EventFilter::CoinFlipped's own fixture \
            (Chance Encounter) for that half.",
        ),
        (
            n("Action", "RollDice"),
            "DEFERRED: every real 'roll a d20' card in the corpus pairs the \
            roll with either a 3-way modal range-table (Loathsome Troll, Cone of Cold, Contact \
            Other Plane, Myrkul's Edict, Recruitment Drive, Thunderwave, Herald of Hadar) or a \
            'roll and add N' modifier (Diviner's Portent, Song of Inspiration, Wyll's Reversal) — \
            both unbuilt (branch-by-numeric-range, and a roll-then-add primitive); none is a bare \
            unmodified roll. Buildable once either lands.",
        ),
        (
            n("Action", "RollPlanarDie"),
            "BLOCKED: see EventFilter::RollPlanarDie — no Plane-card \
            support exists to wire this action to either; a documented absent-subsystem no-op in \
            resolve.rs, never a panic.",
        ),
        (
            n("Action", "LoseGame"),
            "DEFERRED: no 'a player loses the game' real card in this \
            batch.",
        ),
        (
            n("Action", "RestartGame"),
            "DEFERRED: no restart-the-game real card (e.g. Karn \
            Liberated's -14) has coverage.",
        ),
        (
            n("Action", "CastCopy"),
            "DEFERRED: the grammar is present \
            ([CR#707.12], sibling to CopySpell) with a documented never-panic fizzle resolve arm \
            (`// execution: engine-copy-permanent-spells`); the [CR#601.2] cast-a-copy pipeline that \
            would make a real 'cast a copy of [source]' card graduatable is \
            engine-copy-permanent-spells's, not built yet.",
        ),
        // The [CR#115.7] retarget discriminant. Canon's one retarget card
        // exercises ChooseNew ([CR#115.7d], the leave-any-even-if-illegal
        // shape); the other three modes are grammar landed with the
        // discriminant that replaced `ChooseNewTargets`' one-size-fits-all
        // shape — which mis-lumped Bolt Bend with Redirect — and fizzle as
        // documented seams until a witness. The modes are distinguished by
        // PRINTED WORDING, so each needs a card that says it. Semantics are
        // core-pay-player-action T7's.
        (
            n("RetargetMode", "ChangeAll"),
            "DEFERRED: [CR#115.7a] \"change the target(s)\" — all-or-none (\"if all the targets \
            aren't changed to other legal targets, none of them are changed\"). No covered real \
            card prints that wording; canon's retarget card is the ChooseNew shape.",
        ),
        (
            n("RetargetMode", "ChangeOne"),
            "DEFERRED: [CR#115.7b] \"change a target\" — [CR#115.7a]'s process except only ONE \
            target may be changed. No covered real card prints that wording.",
        ),
        (
            n("RetargetMode", "ChangeAny"),
            "DEFERRED: [CR#115.7c] \"change any targets\" — [CR#115.7a]'s process except ANY \
            number may be changed. No covered real card prints that wording.",
        ),
        (
            n("CopyRetarget", "TargetsThat"),
            "DEFERRED: [CR#707.10e]'s all-slots-to-one-object copy mode (every one of the copy's \
            targets must be that player or object; the copy ISN'T CREATED if it is an illegal \
            target for any instance of the word \"target\"). This is the creation-time half of \
            the for-each-could-target family that Selection::ValidTargetsFor and InChosenOrder \
            compose, and no covered real card spells that family — see those two entries. \
            Semantics are core-pay-player-action T7's.",
        ),
        (
            n("Duration", "UntilEvent"),
            "DEFERRED: no 'until (event) happens' one-shot-duration real \
            covered card.",
        ),
        (
            n("Duration", "ForAsLongAs"),
            "DEFERRED: no 'for as long as (condition)' one-shot-duration \
            covered real card.",
        ),
        (
            n("CollectionOp", "Remove"),
            "DEFERRED: no single-element 'loses a color/type/subtype' \
            covered real card.",
        ),
        (
            n("Modification", "SwitchPowerToughness"),
            "DEFERRED: no P/T-switch real card (e.g. \
            Twisted Image) has coverage.",
        ),
        (
            n("Modification", "Supertypes"),
            "DEFERRED: no supertype-changing real card in this \
            batch.",
        ),
        (
            n("Modification", "CantHaveAbility"),
            "DEFERRED: no \"can't have or gain abilities\" real \
            covered card.",
        ),
        (
            n("Modification", "SetText"),
            "DEFERRED: no text-changing real card (e.g. Volrath's \
            Shapeshifter) has coverage.",
        ),
        (
            n("Modification", "BaseLoyalty"),
            "BLOCKED: no loyalty-ability cost grammar exists at all \
            yet (confirmed while scoping Liliana of the Veil for the SeparatePiles backfill) — a \
            planeswalker card is out of reach until that capability exists.",
        ),
        (
            n("Modification", "BaseDefense"),
            "DEFERRED: no Battle-type card in canon yet.",
        ),
        (
            n("Modification", "BecomeBasicLandType"),
            "DEFERRED: no Blood-Moon-shaped real card in \
            covered.",
        ),
        (
            n("StaticEffect", "Conditionally"),
            "DEFERRED: the \"as long as [condition], [effect]\" wrapper \
            ([CR#611.3a]) is elaborated and rendered, but the engine gather is a \
            documented unwired seam (`static_effect_scope` skips it) and no covered real card \
            needs the qualifier over the graveyard/hand `from`-zone shape it replaces \
            (see `renders_graveyard_static_from_zone` for a synthetic exercise); buildable once \
            the gather seam is wired.",
        ),
        (
            n("StaticEffect", "CantPrevent"),
            "DEFERRED: no damage-can't-be-prevented real card in \
            covered.",
        ),
        (
            n("StaticEffect", "SpendAsThough"),
            "DEFERRED: no mana-counterfactual real card in this \
            batch.",
        ),
        (
            n("StaticEffect", "AsThough"),
            "DEFERRED: no AsThough-shaped counterfactual real card in \
            covered.",
        ),
        (
            n("StaticEffect", "BecomesCopy"),
            "DEFERRED: the grammar is present \
            ([CR#707.4] — a continuous layer-1a copy effect, carrying the shared CopySpec like \
            Modify carries a Modification) with a documented never-panic fizzle seam citing \
            engine-layers-1-copy-facedown-text (the same downstream owner as EnterRider::AsCopy \
            above); the layer-1a application that would make a real 'becomes a copy of' card \
            graduatable is not built yet.",
        ),
        (
            n("PlayerAttr", "Life"),
            "DEFERRED: no life-total player-attribute real card beyond \
            Exploration/Reliquary Tower's LandPlaysPerTurn/HandSizeLimit.",
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
            "DEFERRED: no covered hand-size-LOWER real card.",
        ),
        (
            n("PhaseKind", "Combat"),
            "DEFERRED: no combat-phase-scoped real card beyond canon's \
            existing main-phase ones.",
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
            "DEFERRED: no covered untap-step-triggered real card.",
        ),
        (
            n("CombatStep", "BeginningOfCombat"),
            "DEFERRED: no combat-step-triggered real card in \
            coverage (canon has no upkeep/combat-step-triggered permanent yet) — a real \
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
            "DEFERRED: no covered cleanup-step-triggered real card.",
        ),
        (
            n("WhoseTurn", "AnOpponents"),
            "DEFERRED: no opponents'-turn-scoped triggered real card in \
            coverage.",
        ),
        (
            n("StateChange", "Phased"),
            "DEFERRED: no phasing-event-triggered real card in this \
            batch.",
        ),
        (
            n("StateChange", "TurnedFace"),
            "DEFERRED: no turned-face-up/down-triggered real card in \
            coverage.",
        ),
        (
            n("StateChange", "Transformed"),
            "DEFERRED: the Transformed fact is emitted by the transform resolve \
            arm + apply, and Delver of Secrets now performs a transform \
            ([CR#701.27a]) — but this node is the transform TRIGGER-EVENT \
            pattern ('whenever ~ transforms'), and no canon card OBSERVES a \
            transform yet, so nothing spells the `Transformed` token in a \
            trigger. Covered once a transform-triggered real card lands.",
        ),
        (
            n("Agency", "CostPayment"),
            "DEFERRED: no covered card explicitly narrows a Cause \
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
            n("EventFilter", "LifeLost"),
            "DEFERRED: no covered life-LOSS-triggered real card \
            (life GAIN — EventFilter::LifeGained, the `GainsLife` macro — is covered by \
            the misc-event-trigger wave's 'you gain life'/'an opponent gains life' \
            productions; `Drawn`, the `Draws` macro's expansion, is covered by the same \
            wave's draw-a-card productions).",
        ),
        (
            n("EventFilter", "CounterRemoved"),
            "DEFERRED: see EventFilter::LifeLost.",
        ),
        (
            n("EventFilter", "Played"),
            "DEFERRED: see EventFilter::LifeLost.",
        ),
        (
            n("EventFilter", "ActivatedAb"),
            "DEFERRED: see EventFilter::LifeLost.",
        ),
        (
            n("EventFilter", "Attached"),
            "DEFERRED: see EventFilter::LifeLost.",
        ),
        (
            n("EventFilter", "DesignationChanged"),
            "DEFERRED: see EventFilter::LifeLost.",
        ),
        (
            n("EventFilter", "TokenCreated"),
            "DEFERRED: see EventFilter::LifeLost.",
        ),
        (
            n("EventFilter", "Used"),
            "DEFERRED: see EventFilter::LifeLost.",
        ),
        (
            n("EventFilter", "DiceRolled"),
            "DEFERRED: see EventFilter::LifeLost.",
        ),
        (
            n("EventFilter", "RollPlanarDie"),
            "BLOCKED: the (Planechase) planar die's whole surrounding \
            subsystem — Plane cards, a different game-object type, and the chaos/planeswalking \
            abilities a roll triggers — isn't modeled by this engine at all; no real permanent/\
            spell card rolls the planar die (only Plane cards do, out of scope). Never matches at \
            runtime (see eval.rs's documented fizzle), matching Action::RollPlanarDie below.",
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
            n("EventFilter", "Shuffled"),
            "DEFERRED: no covered shuffle-triggered real card (Psychic \
            Surgery) semantic yet — the T5 exposure-row master form + FactView atom exist and \
            are exercised by engine unit tests; authoring the card is optional per that round's \
            scope.",
        ),
        (
            n("EventFilter", "Revealed"),
            "DEFERRED: see EventFilter::Shuffled — the reveal-keyed master \
            form is exercised by engine unit tests; no covered real card spells it yet.",
        ),
        (
            n("RoundMode", "RoundUp"),
            "DEFERRED: no Count::Half-using real card (e.g. 'half its \
            power, rounded up') has coverage.",
        ),
        (
            n("RoundMode", "RoundDown"),
            "DEFERRED: see RoundMode::RoundUp.",
        ),
        (
            n("Characteristic", "Types"),
            "DEFERRED: no CountDistinct-over-card-types real card in \
            coverage (Domain covers Subtypes/BasicLandTypes).",
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
            n("Characteristic", "Name"),
            "DEFERRED: no CountDistinct-over-card-names real card in coverage.",
        ),
        (
            n("Count", "Max"),
            "DEFERRED: no covered 'the greater of X and Y' real card.",
        ),
        (
            n("Count", "Plus"),
            "DEFERRED: no covered 'X plus Y' arithmetic real card.",
        ),
        (
            n("Count", "Minus"),
            "DEFERRED: no covered 'X minus Y' arithmetic real card.",
        ),
        (
            n("Count", "Times"),
            "DEFERRED: no covered 'twice X' arithmetic real card.",
        ),
        (
            n("Count", "Half"),
            "DEFERRED: no covered 'half its power' real card.",
        ),
        (
            n("Count", "ThatMany"),
            "DEFERRED: no covered 'that many' amount-anaphor real card \
            (Collective Defiance would have needed it; Collective Resistance, the card actually \
            built, doesn't).",
        ),
        (
            n("Count", "EventCount"),
            "DEFERRED: no history-fact-count real card (morbid/raid-shaped) \
            has coverage.",
        ),
        (
            n("Count", "EventSum"),
            "DEFERRED: no history-fact-sum real card (e.g. total life lost \
            this turn) has coverage.",
        ),
        (
            n("Count", "Noted"),
            "BLOCKED: the noted-number read is engine-wired (it reads the resolution note store), \
            but its writer PlayerAction::ChooseAndNote(Number) has no simple real single-effect \
            covered card — see that node.",
        ),
        (
            n("Count", "ManaAvailable"),
            "STRATEGY-ONLY: the floated-mana-pool reader senses a player's \
            unspent mana for data-driven strategy ramp gates; card text never reads it, so it has \
            no card fixture (and no Idris counterpart).",
        ),
        (
            n("AggregateOp", "MinOf"),
            "DEFERRED: Count::Aggregate + SumOf + Countable::ManaSymbols are now \
            covered by the Devotion Creature fixture, but no semantic card yet spells an EXTREMAL \
            fold or an extremal-AggregateOp Pick — MinOf/MaxOf stay deferred until one does.",
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
            coverage.",
        ),
        (
            n("Condition", "YourTurn"),
            "DEFERRED: no 'during your turn' conditional real card in \
            coverage.",
        ),
        (
            n("Condition", "TurnOf"),
            "DEFERRED: no 'during an opponent's turn' conditional real card \
            has coverage.",
        ),
        (
            n("Condition", "DuringPhase"),
            "DEFERRED: no phase-gated conditional real card in this \
            batch.",
        ),
        (
            n("Reference", "EventPatient"),
            "DEFERRED: no covered real card reads the two-object \
            event's acted-upon side (EventActor/EventObject are exercised via Fling/Do or Die).",
        ),
        (
            n("Reference", "DefendingPlayer"),
            "DEFERRED: no landwalk/Annihilator-shaped real card in \
            coverage.",
        ),
        (
            n("Reference", "Bound"),
            "DEFERRED: no attacker/blocker-role-bound real card in this \
            batch.",
        ),
        (
            n("Reference", "Linked"),
            "BLOCKED: the linked-ability value read ([CR#607]) stays an unbound-ref fizzle — it \
            needs a per-(ObjectId, Ident) association store (engine-linked-abilities), distinct \
            from activation registers used by scalar choices; and no \
            covered real card spells it regardless.",
        ),
        (
            n("Reference", "Source"),
            "RETIRED AUTHORING: retained in semantics only so legacy Matches(Source, predicate) \
            values lower through the condition-level compatibility rewrite; new data spells the \
            explicit DealtDamageBy condition, and core has no damage-source reference.",
        ),
        (
            n("Reference", "OwnerOf"),
            "DEFERRED: no covered real card reads an object's OWNER \
            as distinct from its controller (Otherworldly Journey uses UnderOwnersControl, a \
            different EnterRider node, not this Reference).",
        ),
        (
            n("CostChange", "Increase"),
            "DEFERRED: no taxing effect (e.g. 'spells your opponents \
            cast cost {1} more') covered real card (Reduce is covered by an existing \
            affinity-shaped card).",
        ),
        (
            n("CostChange", "Additional"),
            "DEFERRED: no MANDATORY continuous cost-modifier real \
            covered card ('spells of type X cost an additional {R} to cast') — distinct \
            from Fling's `OneShotEffect::AdditionalCost`, which is the per-spell PRINTED clause, not a \
            `StaticEffect::CostModifier`.",
        ),
        (
            n("Count", "Divide"),
            "DEFERRED: no general integer-division real card (`Half`'s \
            dedicated /2 twin).",
        ),
        (
            n("Count", "Mod"),
            "DEFERRED: no remainder/parity-check real card ('if X is even') \
            has coverage.",
        ),
        (
            n("Count", "TargetsOf"),
            "DEFERRED: no Strive-style 'for each target beyond the first' \
            covered real card.",
        ),
        (
            n("Countable", "ManaSpentMatching"),
            "DEFERRED: no Adamant-style filtered-mana-spent real card in \
            coverage (the plain, unfiltered mana-spent domain has no Rust \
            constructor yet either).",
        ),
        (
            n("IgnoreRule", "IgnoreLowest"),
            "DEFERRED: Krark's Thumb (StaticEffect::ReplaceRoll's only \
            fixture) uses IgnoreChosen(1) — the flipper's choice, per [CR#706.6] — not an \
            automatic ignore-the-lower rule; no covered real card needs the forced-lowest \
            reading. Buildable once one does.",
        ),
        // --- selection.rs (Selection) ---
        (
            n("Selection", "Union"),
            "DEFERRED: no covered real card groups two selections as ONE \
            set ('each X and each Y' — the Idris Union); canon's multi-group effects iterate each \
            group separately. Buildable.",
        ),
        (
            n("Selection", "Random"),
            "DEFERRED: no random-selection real card ('a creature at random') \
            has coverage. Buildable.",
        ),
        (
            n("Selection", "AmongNoted"),
            "BLOCKED: the among-a-noted-set choice is engine-wired (both the full-group read and \
            the constrained-quantity chooser over the noted group's live members), but no real \
            covered card spells a bare AmongNoted; its 'destroyed this way' anaphor use-case \
            (Blood Money) needs the FURTHER-filtered product read — the OneShotEffect::Noting / \
            Label grammar gap above — not built.",
        ),
        (
            n("Selection", "BottomOfLibrary"),
            "DEFERRED: no covered real card reads the BOTTOM \
            of a library as an ordered set (canon's library reads are all top-anchored — \
            TopOfLibrary is covered), mirroring Anchor::FromBottom above. Buildable.",
        ),
        (
            n("Selection", "PilesOf"),
            "DEFERRED: the labeled per-player noted-piles read (Whims of the \
            Fates: SeparatePiles.note + PilesOf) is the same unbuilt shape as PileSource::Noted \
            above — designed, not built.",
        ),
        (
            n("Selection", "Pick"),
            "DEFERRED: the extremal-element selection ('the creature with the \
            greatest power') shares AggregateOp::MinOf/MaxOf, which stay deferred until a real card \
            spells an extremal fold or Pick — see AggregateOp::MinOf above.",
        ),
        (
            n("Selection", "LibraryOf"),
            "SHELL: the whole-zone library group is grammar landed one task AHEAD of its \
            consumer — [CR#701.24a] names a library as one of shuffle's two own objects, and \
            the Shuffle(Selection) reshape that spells it is the next task of the same \
            core-pay-player-action effort. Retire this entry when that lands.",
        ),
        (
            n("Selection", "InChosenOrder"),
            "DEFERRED: the ordered-selection combinator ('in the order of their controller's \
            choice', [CR#707.10d]) exists so the for-each-could-target copy family stays \
            COMPOSED rather than becoming a bespoke copy mode; no covered real card spells that \
            family. The chooser-driven sequence is itself a recorded engine seam — membership is \
            exact, the order degrades to the inner group's.",
        ),
        (
            n("Selection", "ValidTargetsFor"),
            "DEFERRED: [CR#707.10d]'s could-target read is fully engine-wired (per-slot legal \
            sets folded by the same-object INTERSECTION, unit-tested against a two-slot spell), \
            but no covered real card spells the for-each-could-target family it composes — the \
            same family as InChosenOrder above.",
        ),
        // --- filter.rs ---
        (
            n("ObjectKind", "CardCopy"),
            "DEFERRED: the card-copy object kind ([CR#707.12]) is grammar \
            footing for copy-creating grammar; no covered real card produces \
            or filters a non-stack card copy. Buildable once copy grammar exists.",
        ),
        (
            n("ObjectKind", "Emblem"),
            "DEFERRED: no covered real card FILTERS over emblems. The emblem object kind \
            is now live in the engine (`object_kind` reports `Emblem` for a command-zone emblem) \
            and the emblem-granting verb PlayerAction::GetEmblem is implemented + grammar-covered, \
            but no canon card targets/counts emblems, and no emblem-granting card is graduatable \
            yet (see PlayerAction::GetEmblem above).",
        ),
        (
            n("CharacteristicPredicate", "Named"),
            "DEFERRED: no covered real card filters by object \
            NAME ('a creature named ~'); buildable.",
        ),
        (
            n("CharacteristicPredicate", "Multicolored"),
            "DEFERRED: no multicolored-matters real card ('a \
            multicolored creature') has coverage (Colorless is covered); buildable.",
        ),
        (
            n("StatePredicate", "Status"),
            "DEFERRED: no covered real card filters by object STATUS \
            ('a tapped creature' — Status(Tapped)); canon's tapped-matters effects are all \
            costs/actions, not filters. Buildable.",
        ),
        (
            n("StatePredicate", "RelatedBy"),
            "DEFERRED: no soulbond/paired real card ('the creature ~ is \
            paired with') has coverage; buildable.",
        ),
        (
            n("StatePredicate", "Blocking"),
            "DEFERRED: no covered 'a blocking creature' real card \
            (Attacking is covered); buildable.",
        ),
        (
            n("StatePredicate", "Unblocked"),
            "DEFERRED: no 'an unblocked attacker' real card in this \
            batch; buildable.",
        ),
        (
            n("StatePredicate", "TargetCount"),
            "DEFERRED: no 'a spell with a single target' real card \
            ([CR#115.9a]) has coverage; buildable.",
        ),
        (
            n("StatePredicate", "WasCastWith"),
            "DEFERRED: no covered real card filters by an \
            alternative-base-cost tag ('a spell cast with flashback/for its overload cost' — the \
            filter-language twin of the now-covered WasPaidWith); the alt-cost cast linkage \
            (Cast.tag / WasCastWith) landed with core-alt-costs but no canon card yet reads it as \
            a filter. Buildable.",
        ),
        (
            n("StatePredicate", "WasPutFrom"),
            "DEFERRED: the move-provenance filter ('milled' = \
            WasPutFrom(Library), 'discarded' = WasPutFrom(Hand), [CR#701.17a,701.9a]) is emit-wired \
            and unit-tested (filter::tests::new_atoms_read_flat) but no covered canon card \
            filters on it — the engine fizzles gracefully absent turn-scoped provenance. Buildable \
            with a milled/discarded-matters card.",
        ),
        (
            n("RelationPredicate", "TeammateOf"),
            "DEFERRED: no Two-Headed-Giant / multiplayer real card \
            ('a creature a teammate controls', [CR#102.3,810.1]) has coverage; unit-tested \
            (filter::tests::teammate_of_reads_and_round_trips) but no canon fixture. Buildable.",
        ),
        (
            n("RelationPredicate", "Attachment"),
            "DEFERRED: no 'a creature with an Aura/Equipment attached \
            to it' covered real card (the inverse, AttachedTo, is covered); buildable.",
        ),
        (
            n("Adjacency", "Below"),
            "DEFERRED: Death Spark ('a creature card directly ABOVE it', \
            Predicate::Adjacent) covers Adjacency::Above; no covered real card reads directly \
            BELOW. Buildable.",
        ),
        (
            n("Predicate", "FromSource"),
            "DEFERRED: no covered real card lifts a quality to a \
            stack ability's SOURCE ('abilities from red sources', the hexproof-from-red agent \
            shape, [CR#702.11d]); unit-tested (filter::tests::from_source_reads_and_round_trips) \
            but no canon fixture. Buildable.",
        ),
        (
            n("Predicate", "PlayerStatCmp"),
            "DEFERRED: migrations parser emits this for regenerated wizards cards; \
            Core variant, RON emitter, and engine live-matcher landed, unit-/engine-tested \
            (filter::tests::player_stat_cmp_reads_and_round_trips, \
            resolve::tests::players_countable_counts_by_life_threshold), but no \
            canon/testing/builtin fixture contains it yet. Gate scans committed corpora only. Buildable.",
        ),
        // --- mana.rs ---
        (
            n("PlanarFace", "Blank"),
            "BLOCKED: the (Planechase) planar die's faces ride \
            EventFilter::RollPlanarDie, whose whole surrounding subsystem — Plane cards, a distinct \
            game-object type — isn't modeled by this engine; no real permanent/spell card reads a \
            planar-die face (only Plane cards do, out of scope). Matches PlanarFace::Chaos and \
            EventFilter::RollPlanarDie / PlayerAction::RollPlanarDie above.",
        ),
        (
            n("PlanarFace", "Chaos"),
            "BLOCKED: see PlanarFace::Blank — same absent Planechase subsystem.",
        ),
        (
            n("ManaSpec", "OneOfRuns"),
            "DEFERRED: no filterland real card ('{W}{W}, {W}{U}, or {U}{U}', \
            [CR#106.1b]) has no fixture coverage (the single-mana OneOf is covered); buildable.",
        ),
        (
            n("ManaSpec", "AmongColorsOf"),
            "BLOCKED: 'add one mana of any of the exiled card's colors' \
            (Chrome Mox's imprint, [CR#105.2]) needs the imprint/exile-linked-mana subsystem, which \
            is blocked; no covered real card reaches it. (ManaSpec::ProducedByEvent, the \
            sibling, IS now covered by Dictate of Karametra.)",
        ),
        (
            n("SymbolPred", "AnyType"),
            "DEFERRED: no 'as though it were mana of any type' \
            spend-as-though / any-type-devotion covered real card (AnyColor is covered); \
            buildable.",
        ),
        (
            n("SymbolPred", "IsGeneric"),
            "DEFERRED: no covered real card matches a GENERIC pip via \
            SymbolPred (devotion/spend filters in canon count colored pips, CountsAs); buildable.",
        ),
        (
            n("ManaSymbol", "Snow"),
            "DEFERRED: no snow real card — none with a {S} symbol in a cost \
            ([CR#107.4h]); buildable once a snow card lands.",
        ),
        (
            n("ManaRider", "GrantOnSpend"),
            "DEFERRED: no 'if that mana is spent on a creature spell, it \
            gains X' covered real card ([CR#106.6]) (ManaRider::SpendOnly, the restriction \
            rider, is covered); buildable.",
        ),
        (
            n("ManaRider", "TriggerOnSpend"),
            "DEFERRED: no 'when that mana is spent to cast …, …' \
            delayed-trigger mana rider ([CR#603.7a]) covered real card; buildable.",
        ),
        (
            n("ManaRider", "Persistent"),
            "DEFERRED: no 'you don't lose this mana as steps and phases \
            end' persistence-override rider ([CR#106.4]) covered real card; buildable.",
        ),
        (
            n("ManaRider", "Snow"),
            "DEFERRED: the snow-provenance rider is set at the production emit \
            site from the source's supertypes, so it only appears via a snow permanent — none in \
            fixture coverage (see ManaSymbol::Snow); buildable once a snow source lands.",
        ),
        (
            n("StatePredicate", "SummoningSick"),
            "PERMANENT (not dead grammar): `SummoningSick` appears only in the \
            Idris-invisible `Creature` type confer (`Conditionally(Matches(This, SummoningSick), \
            Cant(...))` on `Creature.ron`), never in any card's oracle text — so it has no \
            card-text accept fixture BY DESIGN, like the `idris_emit` gap. It is exercised by \
            the engine's combat/tap gating and the layer-confer tests ([CR#302.6]).",
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
/// plugins under test.
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
/// longer connect to anything.
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

/// The cause-verb typo gate: with
/// the closed `CauseVerb` enum retired into the bareword [`VerbName`] newtype,
/// the type no longer rejects an unknown verb (any bareword parses) — so this
/// gate does, by MEMBERSHIP against the closed vocabulary the Idris model emits
/// (`crates/deckmaste_plugin/tables/entailments.ron`). Every `Cause(verb: X)`
/// spelled anywhere in the corpus must name a verb the entailment table
/// carries; a typo (`Desroy`) or an out-of-vocab verb fails here, recovering
/// the safety the enum used to give at compile time.
#[test]
fn cause_verbs_are_entailment_rows() {
    // The closed cause-verb vocabulary: the `verb: "…"` column of the emitted
    // entailment table.
    let table = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tables/entailments.ron"),
    )
    .expect("read entailments.ron");
    let row_verb = Regex::new(r#"verb:\s*"([A-Za-z]+)""#).unwrap();
    let known: BTreeSet<String> = row_verb
        .captures_iter(&table)
        .map(|c| c[1].to_string())
        .collect();
    assert!(
        known.len() >= 9,
        "entailments table should carry the closed cause-verb vocab; got {known:?}"
    );

    // The `Act` master form's `verb:` slot shares this field name but carries an
    // ACT-FACT verb name, a vocabulary that supersets the cause verbs: the
    // reorder keyword actions (scry/surveil/fateseal, [CR#701.22a]) and draw
    // ([CR#121.1]) entail no cause-narrowed fact view, so they carry NO
    // entailments.ron row (the would-lane shape guard is simply vacuous for a
    // rowless verb) — yet remain valid `verb:` spellings. Admit them alongside
    // the cause-verb rows; the typo/dead-verb check still bites every
    // genuinely-unknown spelling.
    //
    // NB `Draw` is in this set as an act-fact NAME, not as a keyword action:
    // [CR#701] enumerates the keyword actions and drawing is not among them (it
    // is [CR#121]). The name is historical; the set means "act-fact verb with no
    // entailments row".
    let act_fact_only: BTreeSet<&str> = BTreeSet::from(["Draw", "Fateseal", "Scry", "Surveil"]);

    // Every corpus cause/act-struct `verb:` field must be a known row or a
    // keyword-action name. RON is lenient about the spellings this must
    // survive: the struct name is optional (`(verb: Destroy)` parses as a
    // `Cause`/`CausePattern` too), named fields are ORDER-FREE
    // (`Cause(agency: CostPayment, verb: X)`), and a quoted `verb: "X"`
    // deserializes into the open `VerbName` newtype just as silently as a
    // bareword — so anchor on the field inside any paren group (simple
    // ident-valued fields may precede), not on a first-position `Cause(verb:`
    // spelling.
    let use_verb =
        Regex::new(r#"\(\s*(?:[A-Za-z_]+\s*:\s*[A-Za-z_]+\s*,\s*)*verb\s*:\s*"?([A-Za-z]+)"?"#)
            .unwrap();
    let mut bad: BTreeSet<String> = BTreeSet::new();
    for text in accept_corpus() {
        for c in use_verb.captures_iter(&text) {
            let m = c.get(1).unwrap();
            // A macro's own def forwards its argument as `verb: Param(name)`
            // (an identity macro over a `verb:`-carrying struct is the
            // clearest case) — that's a hole for the CALL SITE's spelling,
            // not itself a verb spelling, and the call site's own literal
            // text is what this scan actually needs to catch. Narrowly
            // skip only the literal `Param(` forwarding shape (not "any
            // captured word followed by `(`", which would also blind the
            // scan to a real typo'd verb that happens to precede a
            // parenthesized fragment elsewhere in the match).
            // `EventFilter::Act`'s identity scaffold is the live instance.
            if m.as_str() == "Param" && text.as_bytes().get(m.end()) == Some(&b'(') {
                continue;
            }
            let verb = m.as_str().to_string();
            if !known.contains(&verb) && !act_fact_only.contains(verb.as_str()) {
                bad.insert(verb);
            }
        }
    }
    assert!(
        bad.is_empty(),
        "cause verb(s) outside the closed entailments.ron vocabulary (typo, or a verb needing \
         a new emitted row): {bad:?}\nknown: {known:?}"
    );
}
