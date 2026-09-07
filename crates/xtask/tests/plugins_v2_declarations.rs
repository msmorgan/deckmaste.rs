//! The declaration file is the shared contract: `deckmaste_construction_core`
//! reads its spelling and grammar under a typed metadata, and
//! `deckmaste_semantics_v2` reads its params and body with that metadata
//! opaque. Neither crate depends on the other for the file, so nothing but
//! this test holds the two readings together.
//!
//! It fails on any file only one side accepts, and on any identity only one
//! side reports (`docs/decisions/semantics-v2.md` §11).

use std::collections::BTreeSet;
use std::path::Path;
use std::path::PathBuf;

use deckmaste_construction_core::macro_def::ParameterType;
use deckmaste_construction_core::macro_def::read_builtin_v2;
use deckmaste_semantics_v2::abilities::Ability;
use deckmaste_semantics_v2::card::Card;
use deckmaste_semantics_v2::reader::Plugin;
use deckmaste_semantics_v2::ron::param_types;
use deckmaste_semantics_v2::triggers::Timing;
use deckmaste_semantics_v2::words::ItalicWord;
use deckmaste_semantics_v2::words::TurnPart;

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("the workspace root resolves")
}

fn builtin() -> PathBuf {
    workspace_root().join("plugins_v2/builtin")
}

/// Every stub declaration reads on both sides, and both sides name the same
/// declarations.
#[test]
fn both_readers_accept_every_builtin_declaration() {
    let root = builtin();
    let typed = read_builtin_v2(&root).expect("construction_core reads the builtin declarations");
    let opaque = Plugin::load(&root).expect("semantics_v2 reads the builtin declarations");

    let typed_names: BTreeSet<String> = typed
        .iter()
        .map(|declaration| declaration.identity().name().to_string())
        .collect();
    // `macros/meta/` holds the declaration meta-macros themselves, which the
    // typed reader consumes rather than reports; every other definition is a
    // declaration both sides see.
    let opaque_names: BTreeSet<String> = opaque
        .declarations
        .values()
        .filter(|declaration| {
            !declaration
                .path
                .starts_with(root.join("macros").join("meta"))
        })
        .map(|declaration| declaration.definition.name.to_string())
        .collect();

    let only_typed: Vec<&String> = typed_names.difference(&opaque_names).collect();
    let only_opaque: Vec<&String> = opaque_names.difference(&typed_names).collect();
    assert!(
        only_typed.is_empty() && only_opaque.is_empty(),
        "the two readings of plugins_v2/builtin disagree\n  \
         construction_core only: {only_typed:?}\n  semantics_v2 only: {only_opaque:?}"
    );
    assert!(
        !typed_names.is_empty(),
        "neither reader found a declaration; the tree moved or the test is looking in the \
         wrong place"
    );
}

/// Every param type name `deckmaste_semantics_v2::ron::param_types()`
/// registers also parses as a `deckmaste_construction_core::macro_def::
/// ParameterType`: the declaration file is the shared contract
/// (`docs/decisions/semantics-v2.md` §11), so a name the semantics side
/// accepts must not be unregistrable on the typed side. `ParameterType` is
/// open (a name outside its eight typed variants normalizes to `Other`
/// rather than failing), so this never fails for a real registration — it
/// exists to catch a future closing-back-up of the enum.
#[test]
fn every_semantics_v2_param_type_parses_in_construction_core() {
    let types = param_types();
    let names: Vec<&str> = types.names().collect();
    assert!(
        !names.is_empty(),
        "param_types() registered nothing; the test is looking at the wrong set"
    );
    for name in names {
        assert!(
            ParameterType::new(name).is_ok(),
            "`{name}` is a `deckmaste_semantics_v2` param type construction_core cannot parse"
        );
    }
}

/// A plugin whose only content is the given `macros/` and `cards/` files,
/// under a temporary root.
///
/// The builtin turn-part and keyword-action stubs are another ticket's to
/// grow bodies for, so the declarations that need one to exercise an
/// invocation are written here instead of edited in place.
fn temp_plugin(files: &[(&str, &str)]) -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("a temporary directory");
    for (path, contents) in files {
        let path = dir.path().join(path);
        std::fs::create_dir_all(path.parent().expect("a file has a parent"))
            .expect("the plugin's directories are creatable");
        std::fs::write(&path, contents).expect("the plugin's files are writable");
    }
    dir
}

/// An ability word invoked at the `Ability` position its body occupies
/// expands to the `ItalicHead` that body builds.
///
/// This is the shape the three ability-word canon cards used to spell by
/// hand, because the declaration registered under its family kind alone and
/// so was unreachable at an `Ability` position.
#[test]
fn an_ability_word_invocation_expands_at_an_ability_position() {
    let plugin = Plugin::load(builtin()).expect("semantics_v2 reads the builtin declarations");
    let ability = r#"Keyword(keyword: "Flying", params: [], body: [])"#;

    let invoked: Ability = plugin
        .macros
        .read_str(&format!("Threshold({ability})"))
        .expect("`Threshold(<ability>)` resolves at an `Ability` position");
    let written_out: Ability = plugin
        .macros
        .read_str(&format!(
            r#"ItalicHead(word: AbilityWord(label: "Threshold"), ability: {ability})"#
        ))
        .expect("the shape the declaration's body builds reads directly");
    assert_eq!(invoked, written_out);

    let Ability::ItalicHead { word, .. } = invoked else {
        panic!("`Threshold(<ability>)` must expand to an italic head");
    };
    assert_eq!(
        word,
        ItalicWord::AbilityWord {
            label: "Threshold".to_owned()
        }
    );
}

/// A keyword action invoked at the `Instruction` position its body occupies
/// expands to that body, read from a card.
///
/// The declaration is a temporary one because the builtin keyword-action
/// stubs are still bodyless; what it exercises is the family's kinds
/// (`[KeywordAction, Instruction]`), which come from the shared meta-macro.
#[test]
fn a_keyword_action_invocation_expands_at_an_instruction_position() {
    let dir = temp_plugin(&[
        (
            "macros/keyword_actions/DrawFor.ron",
            r#"KeywordAction(
                name: "DrawFor",
                params: [Amount],
                spelling: "draw for <Param(0)>",
                body: Draw(amount: Param(0), agent: You),
            )"#,
        ),
        (
            "cards/Drawing Spell.ron",
            r#"SingleFaced(face: (
                characteristics: (
                    name: "Drawing Spell",
                    cost: [Simple(symbol: Specific(color: Of(color: Blue)))],
                    types: [Instant],
                    text: [Spell(timing: None, instruction: DrawFor(Lit(value: 2)))],
                ),
            ))"#,
        ),
    ]);

    let prelude = Plugin::load(builtin()).expect("semantics_v2 reads the builtin declarations");
    let plugin = Plugin::load_with_prelude(&prelude, dir.path())
        .expect("the keyword action registers at `Instruction` and the card reads");

    let declaration = plugin
        .declarations
        .get(&(
            macro_ron::Ident::from("Instruction"),
            macro_ron::Ident::from("DrawFor"),
        ))
        .expect("the declaration registers under its semantic kind as well as its family");
    assert_eq!(
        declaration.definition.kinds,
        [
            macro_ron::Ident::from("KeywordAction"),
            macro_ron::Ident::from("Instruction"),
        ]
    );

    let card = plugin.cards.get("Drawing Spell").expect("the card loaded");
    let written_out: Card = prelude
        .macros
        .read_str(
            r#"SingleFaced(face: (
                characteristics: (
                    name: "Drawing Spell",
                    cost: [Simple(symbol: Specific(color: Of(color: Blue)))],
                    types: [Instant],
                    text: [Spell(timing: None, instruction: Draw(amount: Lit(value: 2), agent: You))],
                ),
            ))"#,
        )
        .expect("the expansion's own shape reads directly");
    assert_eq!(card, &written_out);
}

/// A turn-part declaration whose body is the constructor of the same name
/// registers, and one whose body is a *different* turn part expands from a
/// card.
///
/// The identity case is the blocker this ticket lifts: `macro_ron`'s cycle
/// check exempts a body that names a native variant of one of the macro's own
/// kinds, and the exemption reads the kind's dispatch set — which a
/// hand-built `Kind::new("TurnPart")` does not carry, so `Upkeep` → `Upkeep`
/// was refused as a self-reference at register time. The bodies are written
/// here rather than in `plugins_v2/builtin/macros/turn_parts/`, which is
/// another ticket's to fill in.
#[test]
fn a_turn_part_declaration_may_name_its_own_constructor() {
    let dir = temp_plugin(&[
        (
            "macros/turn_parts/Upkeep.ron",
            r#"TurnPart(
                name: "Upkeep",
                spelling: "upkeep",
                grammar: Noun(singular: "upkeep"),
                params: [],
                body: Upkeep,
            )"#,
        ),
        (
            "macros/turn_parts/UpkeepStep.ron",
            r#"TurnPart(
                name: "UpkeepStep",
                spelling: "upkeep step",
                grammar: Noun(singular: "upkeep step"),
                params: [],
                body: Upkeep,
            )"#,
        ),
        (
            "cards/Upkeep Spell.ron",
            r#"SingleFaced(face: (
                characteristics: (
                    name: "Upkeep Spell",
                    cost: [Simple(symbol: Specific(color: Of(color: Blue)))],
                    types: [Instant],
                    text: [Spell(
                        timing: DuringPart(part: UpkeepStep, whose: None),
                        instruction: Draw(amount: Lit(value: 1), agent: You),
                    )],
                ),
            ))"#,
        ),
    ]);

    let prelude = Plugin::load(builtin()).expect("semantics_v2 reads the builtin declarations");
    let plugin = Plugin::load_with_prelude(&prelude, dir.path())
        .expect("an identity turn-part declaration registers");
    assert!(
        plugin.declarations.contains_key(&(
            macro_ron::Ident::from("TurnPart"),
            macro_ron::Ident::from("Upkeep"),
        )),
        "the identity declaration must be registered, not merely tolerated"
    );

    let card = plugin.cards.get("Upkeep Spell").expect("the card loaded");
    let Card::SingleFaced { face } = card else {
        panic!("the card is single-faced as written");
    };
    let [Ability::Spell { timing, .. }] = face.characteristics.text.as_slice() else {
        panic!("the card's only ability is the spell ability as written");
    };
    assert_eq!(
        timing.as_ref(),
        Some(&Timing::DuringPart {
            part: TurnPart::Upkeep,
            whose: None,
        }),
        "`UpkeepStep` must expand to the turn part its body names"
    );
}
