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
    "selection.rs",
    "filter.rs",
    "mana.rs",
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
        // invocation (`PowerAndToughnessUp(2, 2)`), never literally as
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
            n("Reference", "Coalesce"),
            "DEFERRED: Coalesce IS live — it is the Rhystic payer in Rhystic \
            Lightning (`MayPay(actor: Coalesce([ControllerOf(Target(0)), Target(0)]))` — the \
            target's controller pays, or the target itself if it's a player). But that card \
            lives in plugins/wizards, and this gate counts only canon/testing/builtin; the \
            noncanon-wc99 graduation dropped the burn-targeting Rust override that had kept a \
            counted-dir fixture. Fix = add a canon/testing Rhystic-toll card; unrelated to \
            keyword-action work.",
        ),
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
            n("OneShotEffect", "Batch"),
            "SHELL: `Batch` (engine-act-facet-contract Task 2) is a deliberate \
            scaffold, semantically identical to `Repeat` for now — no card needs it yet \
            because there is no observable difference from `Repeat` to author toward. A \
            later task in the same plan upgrades it to the true aggregate-count tier \
            ([CR#616.1g] 'twice that many' replacements, aggregate triggers) and wires the \
            first real card that needs the distinction.",
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
            "DEFERRED: the engine (command-zone mint + static/triggered sourcing) and the \
            parse/render grammar arm now exist and are covered by unit tests, but no \
            emblem-granting real card is graduatable in this batch: every canon emblem-granter \
            (Garruk Cursed Huntsman, Daretti, Kiora, Ob Nixilis of the Black Oath, Dovin Baan, \
            Teferi's Talent, The Capitoline Triad, Professor Dellian Fel) is a fully-Unparsed \
            multi-ability planeswalker/permanent whose OTHER abilities need unimplemented \
            machinery (loyalty-ability activation, animation, prevention, token creation, \
            variable exile costs) — graduating one would drag in all of it. See the \
            core-emblems ticket's completion notes.",
        ),
        (
            n("PlayerAction", "ChooseAndNote"),
            "BLOCKED: reader-gated engine runtime now exists for the note kinds that HAVE a reader \
            (Number → Count::Noted; Objects → the noted group / AmongNoted; Color/CardName/Piles \
            stay loud — no reader grammar), but no simple real single-effect card in this batch \
            spells ChooseAndNote (Three Tree City, the ticket's suggested card, needs a \
            Color+creature-type domain NotedKind doesn't have) — see the ticket's completion notes.",
        ),
        (
            n("PlayerAction", "FlipCoins"),
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
            n("PlayerAction", "RollDice"),
            "DEFERRED: every real 'roll a d20' card in the corpus pairs the \
            roll with either a 3-way modal range-table (Loathsome Troll, Cone of Cold, Contact \
            Other Plane, Myrkul's Edict, Recruitment Drive, Thunderwave, Herald of Hadar) or a \
            'roll and add N' modifier (Diviner's Portent, Song of Inspiration, Wyll's Reversal) — \
            both unbuilt (branch-by-numeric-range, and a roll-then-add primitive); none is a bare \
            unmodified roll. Buildable once either lands.",
        ),
        (
            n("PlayerAction", "RollPlanarDie"),
            "BLOCKED: see EventFilter::RollPlanarDie — no Plane-card \
            support exists to wire this action to either; a documented absent-subsystem no-op in \
            resolve.rs, never a panic.",
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
            n("EventFilter", "LifeLost"),
            "DEFERRED: no life-LOSS-triggered real card in this batch \
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
            runtime (see eval.rs's documented fizzle), matching PlayerAction::RollPlanarDie below.",
        ),
        (
            n("EventFilter", "BecameDay"),
            "DEFERRED: see EventFilter::LifeLost.",
        ),
        (
            n("EventFilter", "BecameNight"),
            "DEFERRED: see EventFilter::LifeLost.",
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
            "BLOCKED: the noted-number read is engine-wired (it reads the resolution note store), \
            but its writer PlayerAction::ChooseAndNote(Number) has no simple real single-effect \
            card in this batch — see that node.",
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
            covered by the Devotion Creature fixture, but no authored card yet spells an EXTREMAL \
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
            "BLOCKED: the linked-ability value read ([CR#607]) stays an unbound-ref fizzle — it \
            needs a per-(ObjectId, Ident) association store (engine-linked-abilities), distinct \
            from the resolution note store that now serves ChooseAndNote/Count::Noted; and no \
            real card in this batch spells it regardless.",
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
        (
            n("Count", "Divide"),
            "DEFERRED: no general integer-division real card (`Half`'s \
            dedicated /2 twin) in this batch.",
        ),
        (
            n("Count", "Mod"),
            "DEFERRED: no remainder/parity-check real card ('if X is even') \
            in this batch.",
        ),
        (
            n("Count", "TargetsOf"),
            "DEFERRED: no Strive-style 'for each target beyond the first' \
            real card in this batch.",
        ),
        (
            n("Countable", "ManaSpentMatching"),
            "DEFERRED: no Adamant-style filtered-mana-spent real card in \
            this batch (the plain, unfiltered mana-spent domain has no Rust \
            constructor yet either).",
        ),
        (
            n("IgnoreRule", "IgnoreLowest"),
            "DEFERRED: Krark's Thumb (StaticEffect::ReplaceRoll's only \
            fixture) uses IgnoreChosen(1) — the flipper's choice, per [CR#706.6] — not an \
            automatic ignore-the-lower rule; no real card in this batch needs the forced-lowest \
            reading. Buildable once one does.",
        ),
        // --- selection.rs (Selection) ---
        (
            n("Selection", "Union"),
            "DEFERRED: no real card in this batch groups two selections as ONE \
            set ('each X and each Y' — the Idris Union); canon's multi-group effects iterate each \
            group separately. Buildable.",
        ),
        (
            n("Selection", "Random"),
            "DEFERRED: no random-selection real card ('a creature at random') \
            in this batch. Buildable.",
        ),
        (
            n("Selection", "AmongNoted"),
            "BLOCKED: the among-a-noted-set choice is engine-wired (both the full-group read and \
            the constrained-quantity chooser over the noted group's live members), but no real \
            card in this batch spells a bare AmongNoted; its 'destroyed this way' anaphor use-case \
            (Blood Money) needs the FURTHER-filtered product read — the OneShotEffect::Noting / \
            Label grammar gap above — not built.",
        ),
        (
            n("Selection", "BottomOfLibrary"),
            "DEFERRED: no real card in this batch reads the BOTTOM \
            of a library as an ordered set (canon's library reads are all top-anchored — \
            TopOfLibrary is covered), mirroring Anchor::FromBottom above. Buildable.",
        ),
        (
            n("Selection", "PilesOf"),
            "DEFERRED: the labeled per-player noted-piles read (Whims of the \
            Fates: SeparatePiles.note + PilesOf) is the same unbuilt shape as PileSource::Noted \
            above — designed, not built this session.",
        ),
        (
            n("Selection", "Pick"),
            "DEFERRED: the extremal-element selection ('the creature with the \
            greatest power') shares AggregateOp::MinOf/MaxOf, which stay deferred until a real card \
            spells an extremal fold or Pick — see AggregateOp::MinOf above.",
        ),
        // --- filter.rs ---
        (
            n("ObjectKind", "CardCopy"),
            "DEFERRED: the card-copy object kind ([CR#707.12]) is grammar \
            footing for when the copy-creating grammar lands; no real card in this batch produces \
            or filters a non-stack card copy. Buildable once copy grammar exists.",
        ),
        (
            n("ObjectKind", "Emblem"),
            "DEFERRED: no real card in this batch FILTERS over emblems. The emblem object kind \
            is now live in the engine (`object_kind` reports `Emblem` for a command-zone emblem) \
            and the emblem-granting verb PlayerAction::GetEmblem is implemented + grammar-covered, \
            but no canon card targets/counts emblems, and no emblem-granting card is graduatable \
            yet (see PlayerAction::GetEmblem above).",
        ),
        (
            n("CharacteristicPredicate", "Named"),
            "DEFERRED: no real card in this batch filters by object \
            NAME ('a creature named ~'); buildable.",
        ),
        (
            n("CharacteristicPredicate", "Multicolored"),
            "DEFERRED: no multicolored-matters real card ('a \
            multicolored creature') in this batch (Colorless is covered); buildable.",
        ),
        (
            n("StatePredicate", "Status"),
            "DEFERRED: no real card in this batch filters by object STATUS \
            ('a tapped creature' — Status(Tapped)); canon's tapped-matters effects are all \
            costs/actions, not filters. Buildable.",
        ),
        (
            n("StatePredicate", "RelatedBy"),
            "DEFERRED: no soulbond/paired real card ('the creature ~ is \
            paired with') in this batch; buildable.",
        ),
        (
            n("StatePredicate", "Blocking"),
            "DEFERRED: no 'a blocking creature' real card in this batch \
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
            ([CR#115.9a]) in this batch; buildable.",
        ),
        (
            n("StatePredicate", "WasCastWith"),
            "DEFERRED: no real card in this batch filters by an \
            alternative-base-cost tag ('a spell cast with flashback/for its overload cost' — the \
            filter-language twin of the now-covered WasPaidWith); the alt-cost cast linkage \
            (Cast.tag / WasCastWith) landed with core-alt-costs but no canon card yet reads it as \
            a filter. Buildable.",
        ),
        (
            n("StatePredicate", "WasPutFrom"),
            "DEFERRED: the move-provenance filter ('milled' = \
            WasPutFrom(Library), 'discarded' = WasPutFrom(Hand), [CR#701.17a,701.9a]) is emit-wired \
            and unit-tested (filter::tests::new_atoms_read_flat) but no canon card in this batch \
            filters on it — the engine fizzles gracefully absent turn-scoped provenance. Buildable \
            with a milled/discarded-matters card.",
        ),
        (
            n("RelationPredicate", "TeammateOf"),
            "DEFERRED: no Two-Headed-Giant / multiplayer real card \
            ('a creature a teammate controls', [CR#102.3,810.1]) in this batch; unit-tested \
            (filter::tests::teammate_of_reads_and_round_trips) but no canon fixture. Buildable.",
        ),
        (
            n("RelationPredicate", "Attachment"),
            "DEFERRED: no 'a creature with an Aura/Equipment attached \
            to it' real card in this batch (the inverse, AttachedTo, is covered); buildable.",
        ),
        (
            n("Adjacency", "Below"),
            "DEFERRED: Death Spark ('a creature card directly ABOVE it', \
            Predicate::Adjacent) covers Adjacency::Above; no real card in this batch reads directly \
            BELOW. Buildable.",
        ),
        (
            n("Predicate", "FromSource"),
            "DEFERRED: no real card in this batch lifts a quality to a \
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
            [CR#106.1b]) in this batch (the single-mana OneOf is covered); buildable.",
        ),
        (
            n("ManaSpec", "AmongColorsOf"),
            "BLOCKED: 'add one mana of any of the exiled card's colors' \
            (Chrome Mox's imprint, [CR#105.2]) needs the imprint/exile-linked-mana subsystem, which \
            is blocked; no real card in this batch reaches it. (ManaSpec::ProducedByEvent, the \
            sibling, IS now covered by Dictate of Karametra.)",
        ),
        (
            n("SymbolPred", "AnyType"),
            "DEFERRED: no 'as though it were mana of any type' \
            spend-as-though / any-type-devotion real card in this batch (AnyColor is covered); \
            buildable.",
        ),
        (
            n("SymbolPred", "IsGeneric"),
            "DEFERRED: no real card in this batch matches a GENERIC pip via \
            SymbolPred (devotion/spend filters in canon count colored pips, CountsAs); buildable.",
        ),
        (
            n("ManaSymbol", "Snow"),
            "DEFERRED: no snow real card — none with a {S} symbol in a cost \
            ([CR#107.4h]) — in this batch; buildable once a snow card lands.",
        ),
        (
            n("ManaRider", "GrantOnSpend"),
            "DEFERRED: no 'if that mana is spent on a creature spell, it \
            gains X' real card ([CR#106.6]) in this batch (ManaRider::SpendOnly, the restriction \
            rider, is covered); buildable.",
        ),
        (
            n("ManaRider", "TriggerOnSpend"),
            "DEFERRED: no 'when that mana is spent to cast …, …' \
            delayed-trigger mana rider ([CR#603.7a]) real card in this batch; buildable.",
        ),
        (
            n("ManaRider", "Persistent"),
            "DEFERRED: no 'you don't lose this mana as steps and phases \
            end' persistence-override rider ([CR#106.4]) real card in this batch; buildable.",
        ),
        (
            n("ManaRider", "Snow"),
            "DEFERRED: the snow-provenance rider is set at the production emit \
            site from the source's supertypes, so it only appears via a snow permanent — none in \
            this batch (see ManaSymbol::Snow); buildable once a snow source lands.",
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

/// The cause-verb TYPO GATE ([[engine-keyword-action-intent]] Stage 2a): with
/// the closed `CauseVerb` enum retired into the bareword [`VerbName`] newtype,
/// the type no longer rejects an unknown verb (any bareword parses) — so this
/// gate does, by MEMBERSHIP against the closed vocabulary the Idris model emits
/// (`crates/deckmaste_cards/tables/entailments.ron`). Every `Cause(verb: X)`
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

    // The `Act` master form's `verb:` slot ([CR#701]) shares this field name
    // but carries a keyword-action NAME, a vocabulary that supersets the cause
    // verbs: the reorder actions (scry/surveil/fateseal) and draw entail no
    // cause-narrowed fact view, so they carry NO entailments.ron row (the
    // would-lane shape guard is simply vacuous for a rowless verb) — yet remain
    // valid `verb:` spellings. Admit them alongside the cause-verb rows; the
    // typo/dead-verb check still bites every genuinely-unknown spelling.
    let keyword_action_only: BTreeSet<&str> =
        BTreeSet::from(["Draw", "Fateseal", "Scry", "Surveil"]);

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
            let verb = c[1].to_string();
            if !known.contains(&verb) && !keyword_action_only.contains(verb.as_str()) {
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
