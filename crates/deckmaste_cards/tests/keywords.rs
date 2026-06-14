//! Force-expansion proof for every builtin keyword macro: bodies parse
//! LAZILY, so a load-passing plugin proves only the declaration shells —
//! this test invokes each macro (sample args for the parameterized ones)
//! and requires the body to deserialize as a real `KeywordAbility`.

use std::path::Path;

use deckmaste_cards::plugin::Plugin;
use deckmaste_core::KeywordAbility;

fn builtin() -> Plugin {
    Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin")).unwrap()
}

#[test]
fn every_builtin_keyword_macro_expands() {
    // (invocation, expected carried name) — args chosen to satisfy each
    // body's typed positions ([Tap] where a cost-component list lands,
    // a filter where a quality lands; unused params take anything).
    let cases = [
        ("Flying", "Flying"),
        ("Lifelink", "Lifelink"),
        ("Reach", "Reach"),
        ("Flash", "Flash"),
        ("Defender", "Defender"),
        ("Menace", "Menace"),
        ("Haste", "Haste"),
        ("Indestructible", "Indestructible"),
        ("Changeling", "Changeling"),
        ("Prowess", "Prowess"),
        ("Hexproof()", "Hexproof"),
        ("Hexproof(from: ColorIs(Blue))", "Hexproof"),
        ("Ward([Tap])", "Ward"),
        ("Kicker([Tap])", "Kicker"),
        ("Flashback([Tap])", "Flashback"),
        ("Equip([Tap])", "Equip"),
        ("Enchant(Type(Creature))", "Enchant"),
        ("Protection(ColorIs(Red))", "Protection"),
        ("Crew(2)", "Crew"),
        ("Affinity(Type(Artifact))", "Affinity"),
    ];
    let plugin = builtin();
    for (invocation, name) in cases {
        let kw: KeywordAbility = plugin
            .macros
            .read_str(invocation)
            .unwrap_or_else(|e| panic!("expanding {invocation}: {e}"));
        let KeywordAbility::Expanded(expanded) = &kw else {
            panic!("expected Expanded for {invocation}, got {kw:?}");
        };
        assert_eq!(
            expanded.name.as_str(),
            name,
            "carried name for {invocation}"
        );
        assert!(
            matches!(&*expanded.value, KeywordAbility::Composite { name: n, .. } if n.as_str() == name),
            "body of {invocation} is a name-carrying Composite"
        );
    }
}

/// [CR#702.5a,303.4a,303.4f]: the **Enchant** keyword confers THREE abilities,
/// not just the legal-host restriction: (1) a targeting `Spell` (target spec
/// only, no-op effect) so cast targeting stays on the live `spell_targets`
/// path; (2) the removable `Cant(Attach(what: Ref(This), to: Not(Param(0))))`
/// host bound; (3) `AsEnters(Attach(Ref(This), Param(0)))` so the Aura enters
/// attached. (The conferral map in the spec.)
#[test]
fn enchant_confers_spell_cant_attach_and_as_enters() {
    use deckmaste_core::Ability;
    use deckmaste_core::DeonticAction;
    use deckmaste_core::StaticEffect;

    let plugin = builtin();
    let kw: KeywordAbility = plugin
        .macros
        .read_str("Enchant(Type(Creature))")
        .expect("Enchant expands");
    let KeywordAbility::Expanded(expanded) = &kw else {
        panic!("expected Expanded, got {kw:?}");
    };
    let KeywordAbility::Composite { abilities, .. } = &*expanded.value else {
        panic!("Enchant body is a Composite");
    };

    // (1) a targeting Spell ability.
    assert!(
        abilities
            .iter()
            .any(|a| matches!(a, Ability::Spell(s) if !s.targets.is_empty())),
        "Enchant confers a targeting Spell ability ([CR#303.4a]); got {abilities:?}"
    );

    // Walk every Static effect (peel Expanded) looking for the two static rows.
    fn statics(a: &Ability, out: &mut Vec<StaticEffect>) {
        match a {
            Ability::Static(s) => out.extend(s.effects.iter().cloned()),
            Ability::Expanded(e) => statics(&e.value, out),
            _ => {}
        }
    }
    let mut effs = Vec::new();
    for a in abilities {
        statics(a, &mut effs);
    }
    fn peel(e: &StaticEffect) -> &StaticEffect {
        match e {
            StaticEffect::Expanded(x) => peel(&x.value),
            other => other,
        }
    }
    // (2) the host-restriction Cant(Attach) row.
    assert!(
        effs.iter().any(|e| matches!(peel(e),
            StaticEffect::Deontic(d) if matches!(deontic_inner(d), Some(DeonticAction::Attach { .. })))),
        "Enchant confers Cant(Attach(... to Not(Param(0)))) ([CR#702.5a]); got {effs:?}"
    );
    // (3) the AsEnters self-replacement (enters attached).
    assert!(
        effs.iter()
            .any(|e| matches!(peel(e), StaticEffect::Replacement(r) if is_also(r))),
        "Enchant confers AsEnters(Attach(...)) ([CR#303.4f]); got {effs:?}"
    );
}

fn deontic_inner(d: &deckmaste_core::Deontic) -> Option<&deckmaste_core::DeonticAction> {
    use deckmaste_core::Deontic;
    match d {
        Deontic::Cant(a) | Deontic::May(a) | Deontic::Must(a) | Deontic::Gate(a, _) => Some(a),
        Deontic::Expanded(e) => deontic_inner(&e.value),
    }
}

fn is_also(r: &deckmaste_core::Replacement) -> bool {
    use deckmaste_core::Replacement;
    match r {
        Replacement::Also { .. } => true,
        Replacement::Expanded(e) => is_also(&e.value),
        _ => false,
    }
}
