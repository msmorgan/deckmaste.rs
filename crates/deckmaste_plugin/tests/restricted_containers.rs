//! The per-container restriction matrix (spec §4) on the real loader.
//!
//! Restriction is opt-in at the read entry, so "which containers are
//! restricted" is not a property of any file's contents — it is a property of
//! the function that read it. This suite pins one fixture per row of §4's
//! container table by sending the SAME author text down each container's own
//! loader and asserting which way it goes:
//!
//! | container | mode | fixture |
//! |---|---|---|
//! | `cards/` | restricted | [`a_card_refuses_a_variant_with_no_macro`] |
//! | `tokens/` | restricted | [`a_token_refuses_a_variant_with_no_macro`] |
//! | `.ron.todo` graduation | restricted | [`a_graduation_candidate_is_restricted`] |
//! | `rules/sba` | free | [`sba_rules_read_free_vocabulary`] |
//! | `rules/grant` | free | [`grant_rules_read_free_vocabulary`] |
//! | `rules/damage` | free | [`damage_rules_read_free_vocabulary`] |
//! | macro definitions, and `frames:` on them | free | [`a_macro_body_and_its_frames_read_free_vocabulary`] |
//! | test fixtures | per fixture | [`the_entry_decides_not_the_text`] |
//!
//! Engine strategy RON is outside the program (spec §3/§4) and has no row.
//!
//! Two probes carry the suite, both real variants of registered kinds with no
//! macro of that name anywhere:
//!
//! - [`BANNED_KEYWORD`] — `KeywordAbility::Composite`, the one `UNCOVERABLE`
//!   row (`deckmaste_semantics::authoring::UNCOVERABLE`): a keyword's expanded
//!   form, which only a macro expansion may produce.
//! - [`BANNED_EXPANDED`] — `Predicate::Expanded`, synthesized invocation
//!   provenance. Never authored, so never author vocabulary.

use std::fs;
use std::path::Path;
use std::path::PathBuf;

use deckmaste_plugin::plugin::Plugin;

fn plugins() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins")
}

fn builtin() -> Plugin {
    Plugin::load(plugins().join("builtin")).expect("builtin loads")
}

/// A `KeywordAbility::Composite` spelling. `Composite` is a real variant of the
/// registered kind `KeywordAbility` with no macro of that name in any plugin —
/// it is what a keyword macro EXPANDS TO, never what an author writes.
const BANNED_KEYWORD: &str = r#"Keyword(Composite(name: "Ward", abilities: []))"#;

/// A `Predicate::Expanded` spelling: synthesized invocation provenance
/// (spec §5). The reader writes it during expansion; an author never may.
const BANNED_EXPANDED: &str = r#"Expanded(name: "Probe", value: Type(Creature))"#;

/// The message the restricted reader produces for a spelling that IS in the
/// grammar but is not author vocabulary — distinct from the typo message.
const NOT_VOCABULARY: &str = "not author vocabulary";

fn card_with(ability: &str) -> String {
    format!(r#"Normal(name: "Probe", types: [Creature], abilities: [{ability}])"#)
}

fn token_with(ability: &str) -> String {
    format!("Token(types: [Artifact], abilities: [{ability}])")
}

/// A plugin rooted at a fresh temp dir with `builtin` as its prelude, holding
/// exactly the files in `files` (path relative to the root, then contents).
/// The real loader, the real container directories — only the corpus differs.
fn plugin_with(files: &[(&str, &str)]) -> (tempfile::TempDir, anyhow::Result<Plugin>) {
    let root = tempfile::tempdir().expect("temp dir");
    for (path, contents) in files {
        let path = root.path().join(path);
        fs::create_dir_all(path.parent().expect("a parent")).expect("mkdir");
        fs::write(&path, contents).expect("write");
    }
    let loaded = Plugin::load_with_prelude(&builtin(), root.path());
    (root, loaded)
}

// ---------------------------------------------------------------------------
// Restricted containers
// ---------------------------------------------------------------------------

/// `cards/` is restricted: a variant spelling with no macro behind it is
/// refused, and the error says WHY rather than reporting a typo.
#[test]
fn a_card_refuses_a_variant_with_no_macro() {
    let err = builtin()
        .card_from_str(&card_with(BANNED_KEYWORD))
        .expect_err("`Composite` is not author vocabulary");
    let msg = format!("{err:#}");
    assert!(msg.contains(NOT_VOCABULARY), "{msg}");
    assert!(msg.contains("Composite"), "{msg}");
    assert!(msg.contains("KeywordAbility"), "{msg}");
}

/// The same ban reached through the file entry rather than the string entry —
/// `Plugin::card` delegates, so `cards/` on disk is restricted too.
#[test]
fn a_card_file_refuses_a_variant_with_no_macro() {
    let (_root, plugin) = plugin_with(&[("cards/Probe.ron", &card_with(BANNED_KEYWORD))]);
    let err = plugin
        .expect("the plugin loads; cards are read on demand")
        .card("Probe")
        .expect_err("`Composite` is not author vocabulary");
    assert!(format!("{err:#}").contains(NOT_VOCABULARY), "{err:#}");
}

/// A bare primitive at the card ROOT fails with the vocabulary error, not a
/// serde shape error: with no macros in scope at all, `Normal` itself is a
/// suppressed `Card` variant. This is the "useful error" obligation — the
/// reader names the ident and its kind instead of reporting a parse failure.
#[test]
fn a_bare_primitive_card_fails_usefully() {
    let empty = tempfile::tempdir().expect("temp dir");
    let plugin = Plugin::load(empty.path()).expect("an empty plugin loads");
    let err = plugin
        .card_from_str(&card_with(BANNED_KEYWORD))
        .expect_err("no macros in scope: even the root is not author vocabulary");
    let msg = format!("{err:#}");
    assert!(msg.contains(NOT_VOCABULARY), "{msg}");
    assert!(msg.contains("Normal"), "{msg}");
    assert!(msg.contains("Card"), "{msg}");
}

/// The vocabulary error is a DIFFERENT error from the typo error: a name that
/// is neither variant nor macro still reports as unknown, so the two mistakes
/// stay distinguishable.
#[test]
fn an_unknown_name_is_not_reported_as_banned_vocabulary() {
    let err = builtin()
        .card_from_str(&card_with("Keyword(Nonexistent)"))
        .expect_err("`Nonexistent` names nothing");
    let msg = format!("{err:#}");
    assert!(!msg.contains(NOT_VOCABULARY), "{msg}");
    assert!(msg.contains("neither a variant"), "{msg}");
}

/// `tokens/` is restricted, on the same terms as `cards/`.
#[test]
fn a_token_refuses_a_variant_with_no_macro() {
    let err = builtin()
        .token_from_str(&token_with(BANNED_KEYWORD))
        .expect_err("`Composite` is not author vocabulary");
    assert!(format!("{err:#}").contains(NOT_VOCABULARY), "{err:#}");
}

/// Restriction inherits through a NESTED DEFINITION by textual provenance: a
/// token bundle defined inline inside a card file is card-authored text, so it
/// is restricted even though nothing read it through the token entry.
#[test]
fn a_token_defined_inline_in_a_card_is_restricted() {
    let inline_token = format!(
        "Create(agent: You, count: 1, token: Token(types: [Artifact], \
         abilities: [{BANNED_KEYWORD}]))"
    );
    let source = card_with(&format!(
        "Triggered(event: ThisEnters, effect: {inline_token})"
    ));
    let err = builtin()
        .card_from_str(&source)
        .expect_err("the inline token bundle inherits the card's restriction");
    assert!(format!("{err:#}").contains(NOT_VOCABULARY), "{err:#}");
}

/// `tokens/` on disk, through the file entry — [`Plugin::token`] delegates the
/// same way [`Plugin::card`] does. The plugin still LOADS: a token that fails
/// to read is skipped at index time, so the refusal has to be demanded of the
/// entry rather than of the load.
#[test]
fn a_token_file_refuses_a_variant_with_no_macro() {
    let (_root, plugin) = plugin_with(&[("tokens/Probe.ron", &token_with(BANNED_KEYWORD))]);
    let err = plugin
        .expect("the plugin loads; the unreadable token is skipped at index time")
        .token("Probe")
        .expect_err("`Composite` is not author vocabulary");
    assert!(format!("{err:#}").contains(NOT_VOCABULARY), "{err:#}");
}

/// A `.ron.todo` graduation candidate is card-authored text, and reads through
/// [`Plugin::card_from_str`] — so it is restricted by construction.
///
/// This fixture pins the ROW, not the driver: it is the `cards/` string-entry
/// fixture with a `.ron.todo` on disk, and nothing here consults the extension.
/// The driver itself — that `graduate` really routes candidates through that
/// entry, and that a banned spelling therefore does not graduate — is
/// `deckmaste_migrations::graduate::tests::a_candidate_spelling_banned_vocabulary_does_not_graduate`,
/// which lives there because `deckmaste_plugin` cannot depend on the crate that
/// depends on it.
#[test]
fn a_graduation_candidate_is_restricted() {
    let (root, plugin) = plugin_with(&[]);
    let candidate = root.path().join("cards/Probe.ron.todo");
    fs::create_dir_all(candidate.parent().expect("a parent")).expect("mkdir");
    fs::write(&candidate, card_with(BANNED_KEYWORD)).expect("write");
    let source = fs::read_to_string(&candidate).expect("read");
    let err = plugin
        .expect("the plugin loads")
        .card_from_str(&source)
        .expect_err("a graduation candidate is card-authored text");
    assert!(format!("{err:#}").contains(NOT_VOCABULARY), "{err:#}");
}

// ---------------------------------------------------------------------------
// Free containers
// ---------------------------------------------------------------------------

/// The probe used by the `rules/` rows is genuinely banned at a restricted
/// entry — otherwise those rows would pass vacuously.
#[test]
fn the_predicate_probe_is_banned_author_vocabulary() {
    let err = builtin()
        .macros
        .read_str_restricted::<deckmaste_semantics::Predicate>(BANNED_EXPANDED)
        .expect_err("`Expanded` is synthesized provenance, not author vocabulary");
    assert!(err.to_string().contains(NOT_VOCABULARY), "{err}");
}

/// `rules/sba/` is an engine table, not author vocabulary: it reads free.
#[test]
fn sba_rules_read_free_vocabulary() {
    let (_root, plugin) = plugin_with(&[(
        "rules/sba/probe.ron",
        &format!(
            "[SbaRule(scope: {BANNED_EXPANDED}, when: YourTurn, then: Move(This, Graveyard))]"
        ),
    )]);
    let plugin = plugin.expect("an sba table reads free vocabulary");
    assert_eq!(plugin.sba_rules.len(), 1);
}

/// `rules/grant/` reads free — including the exact `Ability` text that a card
/// is refused for. Same bytes, opposite verdict: the entry decides.
#[test]
fn grant_rules_read_free_vocabulary() {
    let (_root, plugin) = plugin_with(&[(
        "rules/grant/probe.ron",
        &format!("[ConferralRule(scope: Type(Creature), confer: Ability({BANNED_KEYWORD}))]"),
    )]);
    let plugin = plugin.expect("a grant table reads free vocabulary");
    assert_eq!(plugin.conferral_rules.len(), 1);
}

/// `rules/damage/` reads free.
#[test]
fn damage_rules_read_free_vocabulary() {
    let (_root, plugin) = plugin_with(&[(
        "rules/damage/probe.ron",
        &format!("[DamageResultRule(recipient: {BANNED_EXPANDED}, remove: LoyaltyCounter)]"),
    )]);
    let plugin = plugin.expect("a damage table reads free vocabulary");
    assert_eq!(plugin.damage_result_rules.len(), 1);
}

/// A macro BODY is definition text, free however it is reached: the def loads,
/// and a card may invoke it even though the card could not spell the body
/// itself. This is the whole point of the layer — the macro is the author
/// surface, its expansion is not.
///
/// The `frames:` row of the container table rides along, because frame data has
/// no restrictable payload of its own — it is a list of strings on the same
/// def, and its whole claim is that carrying it does not make the def's body
/// any less free. Asserted here rather than as a separate fixture, which would
/// have taken all of its force from this one's banned body.
#[test]
fn a_macro_body_and_its_frames_read_free_vocabulary() {
    let (_root, plugin) = plugin_with(&[(
        "macros/probe.ron",
        &format!(
            r#"(name: "ProbeWard", kinds: [Ability], params: [], frames: ["ward"], body: {BANNED_KEYWORD})"#
        ),
    )]);
    let plugin = plugin.expect("a macro body reads free vocabulary");
    plugin
        .card_from_str(&card_with("ProbeWard"))
        .expect("a card may invoke the macro whose body it could not spell");
    let def = plugin
        .macros
        .get("Ability", &macro_ron::Ident::from("ProbeWard"))
        .expect("the def registered");
    assert_eq!(def.frames().len(), 1, "frames data survived the load");
}

// ---------------------------------------------------------------------------
// Argument provenance
// ---------------------------------------------------------------------------

/// A banned spelling written as a macro ARGUMENT is blamed on the argument.
///
/// Argument validation reads each captured argument as its declared type; it
/// used to read free, so a banned spelling passed validation and was rejected
/// later at the `param` re-read — the ban fired, but the error named the macro
/// body rather than the call site that wrote the argument. Validation now
/// reads the argument under the argument text's own restriction, so the
/// rejection lands where the author can act on it.
#[test]
fn a_banned_argument_is_blamed_on_the_argument() {
    let (_root, plugin) = plugin_with(&[(
        "macros/probe.ron",
        r#"(name: "ProbeWrap", kinds: [Ability], params: [Ability], body: Param(0))"#,
    )]);
    let err = plugin
        .expect("the plugin loads")
        .card_from_str(&card_with(&format!("ProbeWrap({BANNED_KEYWORD})")))
        .expect_err("a card-written argument stays restricted");
    let msg = format!("{err:#}");
    assert!(msg.contains(NOT_VOCABULARY), "{msg}");
    assert!(msg.contains("`ProbeWrap` argument 1"), "{msg}");
}

/// The other half of the same rule: validation uses the ARGUMENT's provenance,
/// not the reader's. An argument written inside a macro body is definition
/// text, so a body may hand a banned spelling to a macro it invokes.
#[test]
fn an_argument_written_in_a_body_validates_free() {
    let (_root, plugin) = plugin_with(&[
        (
            "macros/wrap.ron",
            r#"(name: "ProbeWrap", kinds: [Ability], params: [Ability], body: Param(0))"#,
        ),
        (
            "macros/ward.ron",
            &format!(
                r#"(name: "ProbeWard", kinds: [Ability], params: [], body: ProbeWrap({BANNED_KEYWORD}))"#
            ),
        ),
    ]);
    plugin
        .expect("the plugin loads")
        .card_from_str(&card_with("ProbeWard"))
        .expect("a body's own text is free vocabulary, argument position included");
}

// ---------------------------------------------------------------------------
// The wrapper interaction (spec §5)
// ---------------------------------------------------------------------------

/// **Spec §5's wrapper interaction, asserted directly.** A previously-bare
/// spelling at a `remembers_expansion` kind now routes through its identity
/// macro and comes back wrapped in invocation provenance — `This` at a
/// `Reference` position is the smallest real instance. Free, the same text
/// still reads as the bare variant, because a native variant wins there and
/// the identity macro is never consulted.
///
/// That difference is the whole reason [`Plugin::rendering_card_from_str`]
/// exists, and it was proved only indirectly (by that entry existing) until
/// this fixture. §5's claim is that the wrapper is invisible at stored-byte
/// round-trip and lowered core — NOT that it is absent, and a reader who
/// assumes absence will write a structural match that silently stops firing.
#[test]
fn an_identity_macro_wraps_a_bare_spelling_at_a_remembering_kind() {
    use deckmaste_semantics::Reference;

    let macros = &builtin().macros;
    let restricted: Reference = macros
        .read_str_restricted("This")
        .expect("`This` has an identity macro");
    let Reference::Expanded(expansion) = &restricted else {
        panic!("expected invocation provenance at a remembering kind, got {restricted:?}");
    };
    assert_eq!(expansion.name.as_str(), "This");
    assert_eq!(*expansion.value, Reference::This);

    let free: Reference = macros.read_str("This").expect("`This` is a native variant");
    assert_eq!(
        free,
        Reference::This,
        "a free read takes the native variant"
    );
}

// ---------------------------------------------------------------------------
// Test fixtures: per fixture
// ---------------------------------------------------------------------------

/// The container table's last row. A fixture is restricted or free according to
/// the entry it chooses, not according to where it lives — the same bytes read
/// both ways from one test.
#[test]
fn the_entry_decides_not_the_text() {
    let macros = &builtin().macros;
    macros
        .read_str::<deckmaste_semantics::Predicate>(BANNED_EXPANDED)
        .expect("the free entry reads it");
    macros
        .read_str_restricted::<deckmaste_semantics::Predicate>(BANNED_EXPANDED)
        .expect_err("the restricted entry refuses it");
}
