//! The whole `plugins_v2` corpus through the reader, with no Lean involved.
//!
//! `cargo xtask lean-check` proves the cards; it is the only thing that read
//! the corpus, and the Rust gate did not. A dialect change that broke a card
//! therefore surfaced only in an xtask run — twice, in the first landing of
//! `plugins-v2-dialect`. This test is the Rust half: every declaration body
//! in `plugins_v2/builtin/macros/<family>/`, every card and token and rules
//! row in `plugins_v2/canon` and `plugins_v2/testing`, read through the same
//! `MacroSet` a real load uses. Zero refusals, and the counts printed so a
//! silently emptied corpus is visible.

use std::path::Path;
use std::path::PathBuf;

use deckmaste_semantics_v2::reader::Plugin;

fn plugins_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v2")
}

/// `plugins_v2/builtin` — every family's declaration bodies.
#[test]
fn every_builtin_declaration_reads() {
    let root = plugins_root().join("builtin");
    let builtin = Plugin::load(&root).expect("the builtin declarations read");
    let families: std::collections::BTreeSet<&str> = builtin
        .declarations
        .keys()
        .map(|(kind, _)| kind.as_str())
        .collect();
    println!(
        "plugins_v2/builtin: {} declaration(s) across {} kind(s)",
        builtin.declarations.len(),
        families.len()
    );
    assert!(
        builtin.declarations.len() >= 400,
        "only {} declarations read; the scan lost the corpus it reads",
        builtin.declarations.len()
    );
}

/// Every card, token and rules row of the two card plugins, over `builtin`.
#[test]
fn every_card_reads() {
    let root = plugins_root();
    let builtin = Plugin::load(root.join("builtin")).expect("the prelude reads");
    let mut total = 0;
    for name in ["canon", "testing"] {
        let plugin = Plugin::load_with_prelude(&builtin, root.join(name))
            .unwrap_or_else(|error| panic!("plugins_v2/{name} reads: {error}"));
        println!(
            "plugins_v2/{name}: {} card(s), {} token(s), {} sba / {} conferral / {} damage row(s)",
            plugin.cards.len(),
            plugin.tokens.len(),
            plugin.rules.sba.len(),
            plugin.rules.conferral.len(),
            plugin.rules.damage_result.len(),
        );
        total += plugin.cards.len();
    }
    assert!(
        total >= 120,
        "only {total} cards read; the scan lost the corpus it reads"
    );
}

/// Every helper declaration that takes no arguments, invoked by its bare name
/// at its own kind.
///
/// Reading a declaration file only checks that its `body` is well-formed RON;
/// the body is opaque text until something expands it. This is the expansion
/// half: each nullary helper is written where a card would write it and read
/// as the type its kind names, so a body that names a constructor the type
/// does not have — or a macro that did not port — fails here rather than on
/// the first card to write it.
#[test]
fn every_nullary_helper_expands() {
    let builtin = Plugin::load(plugins_root().join("builtin")).expect("the builtin reads");
    let mut expanded = 0;
    let mut untested = 0;
    for ((kind, name), declaration) in &builtin.declarations {
        let nullary = match &declaration.definition.params {
            macro_ron::Params::Positional(types) => types.is_empty(),
            macro_ron::Params::Named(signature) => signature.is_empty(),
        };
        // A BODYLESS declaration — a meta-macro's omitted `body` argument,
        // which several keyword families still are — has nothing to expand.
        if !nullary || declaration.definition.body_head(&builtin.macros).is_none() {
            continue;
        }
        macro_rules! read {
            ($($position:literal => $ty:ty),* $(,)?) => {
                match kind.as_str() {
                    $($position => builtin
                        .macros
                        .read_str::<$ty>(name.as_str())
                        .map(drop)
                        .unwrap_or_else(|error| {
                            panic!("{kind}/{name} ({}): {error}", declaration.path.display())
                        }),)*
                    _ => {
                        untested += 1;
                        continue;
                    }
                }
            };
        }
        read! {
            "NounPhrase" => deckmaste_semantics_v2::phrase::NounPhrase,
            "Predicate" => deckmaste_semantics_v2::phrase::Predicate,
            "Amount" => deckmaste_semantics_v2::phrase::Amount,
            "Quantity" => deckmaste_semantics_v2::phrase::Quantity,
            "ZoneExpr" => deckmaste_semantics_v2::phrase::ZoneExpr,
            "Condition" => deckmaste_semantics_v2::phrase::Condition,
            "GameEvent" => deckmaste_semantics_v2::phrase::GameEvent,
            "Duration" => deckmaste_semantics_v2::triggers::Duration,
            "Ability" => deckmaste_semantics_v2::abilities::Ability,
            "Instruction" => deckmaste_semantics_v2::abilities::Instruction,
            "StaticSpec" => deckmaste_semantics_v2::abilities::StaticSpec,
            "Cost" => deckmaste_semantics_v2::abilities::Cost,
            "ManaSymbol" => deckmaste_semantics_v2::words::ManaSymbol,
            "ColorTerm" => deckmaste_semantics_v2::phrase::ColorTerm,
            // A subtype declaration's body is its `Definition` node, and a
            // definition's name denotes its term, so reading the macro at a
            // `Subtype` position exercises both the derived body and the
            // projection on all 462 declarations
            // (`plugins-v2-subtypes-macro-only`).
            "Subtype" => deckmaste_semantics_v2::words::Subtype,
        }
        expanded += 1;
    }
    println!("{expanded} nullary declaration(s) expand; {untested} at untested kinds");
    assert!(
        expanded >= 200,
        "only {expanded} expanded; the scan lost the declarations it reads"
    );
}

/// Every helper ported from a `Primitives.*` alias, invoked with arguments at
/// its own kind.
///
/// [`every_nullary_helper_expands`] covers the argument-less half; these take
/// arguments, so nothing else in the corpus reaches them until a card writes
/// one. A macro that loads but cannot be invoked is the defect shape
/// `semantics-v2-macro-bodies-keyword-actions` recorded, so each is written
/// here exactly as a card would write it.
#[test]
fn every_ported_alias_expands() {
    use deckmaste_semantics_v2::abilities::{Ability, Instruction};
    use deckmaste_semantics_v2::phrase::{Amount, Condition, GameEvent, NounPhrase, Predicate};
    use deckmaste_semantics_v2::words::ManaSymbol;
    let builtin = Plugin::load(plugins_root().join("builtin")).expect("the builtin reads");
    macro_rules! reads {
        ($($ty:ty : $source:literal),* $(,)?) => {
            $(builtin
                .macros
                .read_str::<$ty>($source)
                .map(drop)
                .unwrap_or_else(|error| panic!("{}: {error}", $source));)*
        };
    }
    reads! {
        Amount: "countOf(p: AnyPlayer)",
        Amount: "aggregate(op: Sum, axis: Stat(stat: Power), p: AnyPlayer)",
        Predicate: "ofChosen(sort: Color)",
        Predicate: "castBy(player: You)",
        NounPhrase: "theRest(kind: Object)",
        NounPhrase: "someOf(quantity: Range(low: 1, high: 1), group: You)",
        ManaSymbol: "generic(amount: 2)",
        Instruction: "move(subject: You, destination: graveyard)",
        Instruction: "draw(amount: Lit(value: 1))",
        Instruction: "doIf(condition: Exists(subject: You), instruction: Shuffle(agent: You))",
        Instruction: "choose(subject: You)",
        Instruction: "rollDice(count: 1, sides: 20)",
        Instruction: "flipCoins(count: 1)",
        GameEvent: "flipsCoin(player: You)",
        Condition: "happened(event: FlipsCoin(player: You, call: None), who: You, lookback: ThisTurn)",
        Predicate: "happenedTo(event: FlipsCoin(player: You, call: None), lookback: ThisTurn)",
        Instruction: "shiftResult(amount: Lit(value: 1))",
        Instruction: "delay(event: FlipsCoin(player: You, call: None), instruction: Shuffle(agent: You))",
        Instruction: "removeCounters(quantity: Range(low: 1, high: 1), kind: Printed(kind: Named(label: \"charge\")), from_: You)",
        GameEvent: "counterEvent(move: Put, kind: Named(label: \"charge\"), batch: One, subject: You)",
        GameEvent: "tokensCreated(tokens: You)",
        Ability: "keyword(label: \"Flying\")",
        Ability: "triggered(event: FlipsCoin(player: You, call: None), instruction: Shuffle(agent: You))",
        Ability: "activated(cost: TapSymbol, instruction: Shuffle(agent: You))",
    }
}

/// The dialect's WRITER, over the whole card corpus: every card reads, writes,
/// and reads back to the same value, and every injection and numeral leaf is
/// written bare.
///
/// `crate::ron::raw_options`' `UNWRAP_NEWTYPES` is the writer
/// (`docs/decisions/semantics-v2.md` §11.1): an `#[macro_ron(embed)]` struct
/// variant serializes its payload through a newtype struct named
/// `Type.Variant`, which the extension drops, so `ManaSymbol::Simple { … }`
/// writes `White` and `Amount::Lit { … }` writes `3`. Nothing exercised that
/// direction over the corpus before; the substring assertions below are what
/// makes the elision a fact rather than a claim, and the round trip is what
/// makes it lossless.
///
/// The way back is the UNRESTRICTED read on purpose. A written value is the
/// constructor basis, not author vocabulary: expansion is name-erasing
/// (`ron::tests::no_kind_remembers_its_invocation`), so a card's macro
/// invocations are gone by the time it is a value, and §11.1's macro-only rule
/// refuses what comes back at a card entry. That is the gap between this pin
/// and a file-level conversion, and it is why `plugins-v2-cosmetic-conversion`
/// stopped.
#[test]
fn every_card_writes_and_reads_back_to_the_same_value() {
    let root = plugins_root();
    let builtin = Plugin::load(root.join("builtin")).expect("the prelude reads");
    let options = deckmaste_semantics_v2::ron::raw_options();
    // Each pair is an injection or a numeral leaf together with the binder it
    // would keep if it were written out; the writer must elide every one.
    let never_written = [
        "Simple(symbol:",
        "Specific(color:",
        "Of(color:",
        "Lit(color:",
        "Lit(value:",
        "Generic(amount:",
    ];
    let mut written = 0;
    for name in ["canon", "testing"] {
        let plugin = Plugin::load_with_prelude(&builtin, root.join(name))
            .unwrap_or_else(|error| panic!("plugins_v2/{name} reads: {error}"));
        for (stem, card) in &plugin.cards {
            let text = options
                .to_string(card)
                .unwrap_or_else(|error| panic!("{name}/{stem} writes: {error}"));
            for elided in never_written {
                assert!(
                    !text.contains(elided),
                    "{name}/{stem} was written with `{elided}`, which the dialect elides"
                );
            }
            let read_back = plugin
                .macros
                .read_str::<deckmaste_semantics_v2::card::Card>(&text)
                .unwrap_or_else(|error| panic!("{name}/{stem} reads back: {error}"));
            assert_eq!(
                &read_back, card,
                "{name}/{stem} did not survive the round trip"
            );
            written += 1;
        }
    }
    println!("{written} card(s) read, wrote, and read back to the same value");
    assert!(
        written >= 120,
        "only {written} cards written; the scan lost the corpus it reads"
    );
}

/// The corpus stays converted: no card or declaration body writes out an
/// injection or a numeral leaf the dialect elides.
///
/// `plugins-v2-cosmetic-conversion` reserialised every macro-free sub-value
/// through the writer, so the corpus now spells `[2, White, White, White]`
/// where it spelled `[Simple(symbol: Generic(amount: 2)), …]`. Written-out
/// forms stay LEGAL — §11.1's carve-out keeps a literal leaf spellable, and an
/// injection's constructor is never refused — so this is a ratchet on the
/// corpus, not a rule about the dialect: a new card authored the long way is
/// caught here rather than drifting back one file at a time.
///
/// The four spellings are the ones a single `(constructor, binder)` pair fixes
/// unambiguously across the whole mirror. `ColorOrColorless::Of` and
/// `ColorTerm::Lit` are deliberately absent: `Of(color:` is also
/// `ManaMatch::Of`'s spelling, so the text alone does not say which
/// constructor is written, and a guard that cannot tell them apart would
/// refuse a legitimate write.
#[test]
fn no_source_file_writes_out_an_elided_constructor() {
    let root = plugins_root();
    let elided = [
        "Simple(symbol:",
        "Specific(color:",
        "Generic(amount:",
        "Lit(value:",
    ];
    let mut checked = 0;
    for dir in [
        root.join("canon/cards"),
        root.join("testing/cards"),
        root.join("testing/macros"),
        root.join("builtin/macros"),
    ] {
        for path in deckmaste_semantics_v2::reader::ron_files_recursive(&dir)
            .expect("the corpus directory is readable")
        {
            let source = std::fs::read_to_string(&path).expect("a corpus file is readable UTF-8");
            for spelling in elided {
                assert!(
                    !source.contains(spelling),
                    "{} writes `{spelling}`, which the dialect elides",
                    path.display()
                );
            }
            checked += 1;
        }
    }
    println!("{checked} source file(s) hold no elided constructor");
    assert!(
        checked >= 1_500,
        "only {checked} files checked; the scan lost the corpus it reads"
    );
}
