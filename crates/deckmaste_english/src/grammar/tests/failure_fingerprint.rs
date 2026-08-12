use super::diagnostic::DiagnosticLimits;
use super::diagnostic::FailureCategory;
use super::diagnostic::FailureClusterKey;
use super::diagnostic::FingerprintStatus;
use super::diagnostic::diagnose_with_registration_order;
use super::rules::RegistrationOrder;
use crate::CatalogKind;
use crate::Catalogs;
use crate::syntax::RecoveryRole;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExpectedSeam {
    BracketDelimiter,
    NegativeModifierList,
    LoyaltyHeader,
    MarkerlessParticiple,
    PredicativeRequirement,
    ContrastiveGerund,
    PassiveCoordination,
    TokenDesignation,
}

#[derive(Debug, Clone, Copy)]
struct RecoveryFixture {
    seam: ExpectedSeam,
    source: &'static str,
}

const RECOVERY_FIXTURES: &[RecoveryFixture] = &[
    RecoveryFixture {
        seam: ExpectedSeam::BracketDelimiter,
        source: "[At the beginning of that turn's end step, you lose the game.]",
    },
    RecoveryFixture {
        seam: ExpectedSeam::BracketDelimiter,
        source: "Counter target spell [that wasn't cast from its owner's hand].",
    },
    RecoveryFixture {
        seam: ExpectedSeam::BracketDelimiter,
        source: "Draw a card for each creature you control [with flying].",
    },
    RecoveryFixture {
        seam: ExpectedSeam::NegativeModifierList,
        source: "You choose a noncreature, nonland card from it.",
    },
    RecoveryFixture {
        seam: ExpectedSeam::NegativeModifierList,
        source: "Destroy target nonartifact, nonblack creature.",
    },
    RecoveryFixture {
        seam: ExpectedSeam::NegativeModifierList,
        source: "You may reveal a noncreature, nonland card from among them and put it into your hand.",
    },
    RecoveryFixture {
        seam: ExpectedSeam::NegativeModifierList,
        source: "For each player, exile up to one target non-Saga, nonland permanent that player controls until this Saga leaves the battlefield.",
    },
    RecoveryFixture {
        seam: ExpectedSeam::NegativeModifierList,
        source: "Return up to one target non-Faerie, nonland permanent you control to its owner's hand.",
    },
    RecoveryFixture {
        seam: ExpectedSeam::LoyaltyHeader,
        source: "∞ — At the beginning of your end step, exile up to one other target nonland permanent you control, then return that card to the battlefield under its owner's control.",
    },
    RecoveryFixture {
        seam: ExpectedSeam::LoyaltyHeader,
        source: "∞ — At the beginning of your upkeep, return target creature card from your graveyard to the battlefield.",
    },
    RecoveryFixture {
        seam: ExpectedSeam::MarkerlessParticiple,
        source: "A creature destroyed this way can't be regenerated.",
    },
    RecoveryFixture {
        seam: ExpectedSeam::MarkerlessParticiple,
        source: "Creatures destroyed this way can't be regenerated.",
    },
    RecoveryFixture {
        seam: ExpectedSeam::MarkerlessParticiple,
        source: "If a spell cast this way would be put into a graveyard, exile it instead.",
    },
    RecoveryFixture {
        seam: ExpectedSeam::MarkerlessParticiple,
        source: "If a spell cast this way would be put into your graveyard, exile it instead.",
    },
    RecoveryFixture {
        seam: ExpectedSeam::MarkerlessParticiple,
        source: "This creature gets +2/+2 for each Aura attached to it.",
    },
    RecoveryFixture {
        seam: ExpectedSeam::MarkerlessParticiple,
        source: "Enchanted creature gets +2/+2 for each Aura and Equipment attached to it.",
    },
    RecoveryFixture {
        seam: ExpectedSeam::MarkerlessParticiple,
        source: "For each 1 damage prevented this way, put a +1/+1 counter on that creature.",
    },
    RecoveryFixture {
        seam: ExpectedSeam::MarkerlessParticiple,
        source: "Put all revealed cards not cast this way on the bottom of your library in a random order.",
    },
    RecoveryFixture {
        seam: ExpectedSeam::PredicativeRequirement,
        source: "The new target must be a player.",
    },
    RecoveryFixture {
        seam: ExpectedSeam::PredicativeRequirement,
        source: "The new target must be a creature.",
    },
    RecoveryFixture {
        seam: ExpectedSeam::ContrastiveGerund,
        source: "If a spell or ability an opponent controls causes you to discard this card, put it onto the battlefield instead of putting it into your graveyard.",
    },
    RecoveryFixture {
        seam: ExpectedSeam::ContrastiveGerund,
        source: "If a spell or ability an opponent controls causes you to discard this card, put it onto the battlefield with two +1/+1 counters on it instead of putting it into your graveyard.",
    },
    RecoveryFixture {
        seam: ExpectedSeam::PassiveCoordination,
        source: "Prevent all combat damage that would be dealt to and dealt by that creature this turn.",
    },
    RecoveryFixture {
        seam: ExpectedSeam::PassiveCoordination,
        source: "Prevent all combat damage that would be dealt to and dealt by this creature this turn.",
    },
    RecoveryFixture {
        seam: ExpectedSeam::PassiveCoordination,
        source: "Until your next turn, prevent all damage that would be dealt to and dealt by target permanent an opponent controls.",
    },
    RecoveryFixture {
        seam: ExpectedSeam::PassiveCoordination,
        source: "Prevent all combat damage that would be dealt to and dealt by creatures you control.",
    },
    RecoveryFixture {
        seam: ExpectedSeam::PassiveCoordination,
        source: "Prevent all combat damage that would be dealt to and dealt by enchanted creature.",
    },
    RecoveryFixture {
        seam: ExpectedSeam::TokenDesignation,
        source: "Create a 0/1 red Kobold creature token named Kobolds of Kher Keep.",
    },
    RecoveryFixture {
        seam: ExpectedSeam::TokenDesignation,
        source: "Create a 1/1 green Wolf creature token named Wolves of the Hunt.",
    },
    RecoveryFixture {
        seam: ExpectedSeam::TokenDesignation,
        source: "Each player creates a colorless artifact token named Banana with \"{T}, Sacrifice this token: Add {R} or {G}. You gain 2 life.\"",
    },
    RecoveryFixture {
        seam: ExpectedSeam::TokenDesignation,
        source: "Create a colorless artifact token named Etherium Cell with \"{T}, Sacrifice this token: Add one mana of any color.\"",
    },
    RecoveryFixture {
        seam: ExpectedSeam::TokenDesignation,
        source: "Create a 2/2 white Cat Soldier creature token named Ajani's Pridemate with \"Whenever you gain life, put a +1/+1 counter on this token.\"",
    },
];

const SAME_SEAM_PAIRS: &[(usize, usize)] = &[
    (0, 1),
    (0, 2),
    (3, 4),
    (8, 9),
    (10, 11),
    (22, 23),
    (22, 25),
    (27, 28),
];

const KNOWN_SAME_SEAM_DECLINED_PAIRS: &[(usize, usize)] = &[(12, 13), (18, 19), (20, 21)];

const NEEDS_REPROBE_PAIRS: &[(usize, usize)] = &[(14, 15)];

const UNRELATED_PAIRS: &[(usize, usize)] = &[
    (0, 3),
    (0, 8),
    (0, 10),
    (3, 8),
    (3, 10),
    (8, 10),
    (10, 18),
    (18, 22),
    (22, 27),
];

const COMPLETE_LIMITS: DiagnosticLimits = DiagnosticLimits {
    max_events: 16_384,
    max_frontier: 2_048,
    max_lattice_states: 1_000_000,
};

fn fixture_catalogs() -> Catalogs {
    Catalogs::default()
        .with_catalog(CatalogKind::ArtifactType, ["Equipment", "Vehicle"])
        .with_catalog(
            CatalogKind::CreatureType,
            [
                "Cat",
                "Centaur",
                "Demon",
                "Faerie",
                "Inkling",
                "Kobold",
                "Phyrexian",
                "Soldier",
                "Spirit",
                "Squirrel",
                "Thrull",
                "Zombie",
                "Wolf",
            ],
        )
        .with_catalog(
            CatalogKind::CardType,
            ["Artifact", "Creature", "Land", "Saga"],
        )
        .with_catalog(CatalogKind::EnchantmentType, ["Aura"])
        .with_catalog(CatalogKind::KeywordAbility, ["Flying"])
}

fn status(source: &str, order: RegistrationOrder, limits: DiagnosticLimits) -> FingerprintStatus {
    diagnose_with_registration_order(
        source,
        &fixture_catalogs(),
        FailureCategory::Sentence,
        limits,
        order,
    )
    .unwrap_or_else(|error| panic!("failed to diagnose {source:?}: {error:?}"))
}

fn complete_key(source: &str, order: RegistrationOrder) -> FailureClusterKey {
    let diagnosed = status(source, order, COMPLETE_LIMITS);
    diagnosed
        .cluster_key()
        .unwrap_or_else(|| panic!("expected a complete cluster key for {source:?}: {diagnosed:#?}"))
        .clone()
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ClusterOutcome {
    Key(FailureClusterKey),
    Declined(&'static str),
}

fn cluster_outcome(source: &str, order: RegistrationOrder) -> ClusterOutcome {
    match status(source, order, COMPLETE_LIMITS) {
        FingerprintStatus::Complete(fingerprint) => {
            ClusterOutcome::Key(fingerprint.cluster_key().clone())
        }
        FingerprintStatus::Declined { reason, .. } => ClusterOutcome::Declined(reason),
        capped @ FingerprintStatus::Capped { .. } => {
            panic!("diagnostic limits were too low for {source:?}: {capped:#?}")
        }
    }
}

#[test]
fn fingerprint_validation_sample_stays_stratified() {
    assert!((25..=50).contains(&RECOVERY_FIXTURES.len()));
    for seam in [
        ExpectedSeam::BracketDelimiter,
        ExpectedSeam::NegativeModifierList,
        ExpectedSeam::LoyaltyHeader,
        ExpectedSeam::MarkerlessParticiple,
        ExpectedSeam::PredicativeRequirement,
        ExpectedSeam::ContrastiveGerund,
        ExpectedSeam::PassiveCoordination,
        ExpectedSeam::TokenDesignation,
    ] {
        assert!(
            RECOVERY_FIXTURES.iter().any(|fixture| fixture.seam == seam),
            "missing {seam:?} coverage",
        );
    }
    for &(left, right) in SAME_SEAM_PAIRS
        .iter()
        .chain(UNRELATED_PAIRS)
        .chain(KNOWN_SAME_SEAM_DECLINED_PAIRS)
        .chain(NEEDS_REPROBE_PAIRS)
    {
        assert!(left < RECOVERY_FIXTURES.len());
        assert!(right < RECOVERY_FIXTURES.len());
    }
}

#[test]
fn known_same_seams_decline_when_the_trace_cannot_localize_them() {
    for &(left, right) in KNOWN_SAME_SEAM_DECLINED_PAIRS {
        let left = RECOVERY_FIXTURES[left];
        let right = RECOVERY_FIXTURES[right];
        assert_eq!(left.seam, right.seam);
        for fixture in [left, right] {
            let outcome = status(fixture.source, RegistrationOrder::Normal, COMPLETE_LIMITS);
            assert!(
                matches!(outcome, FingerprintStatus::Declined { .. }),
                "an unlocalized known seam must decline, not invent a key for {:?}: {outcome:#?}",
                fixture.source,
            );
            assert!(outcome.cluster_key().is_none());
        }
    }
}

#[test]
fn additional_blockers_are_not_auto_clustered() {
    for &(left, right) in NEEDS_REPROBE_PAIRS {
        let left = RECOVERY_FIXTURES[left];
        let right = RECOVERY_FIXTURES[right];
        assert_eq!(left.seam, right.seam);
        let left = status(left.source, RegistrationOrder::Normal, COMPLETE_LIMITS);
        let right = status(right.source, RegistrationOrder::Normal, COMPLETE_LIMITS);
        assert!(
            left.cluster_key().is_none()
                || right.cluster_key().is_none()
                || left.cluster_key() != right.cluster_key(),
            "a dependency-bearing row must be re-probed after its first blocker is fixed",
        );
        assert!(
            matches!(left, FingerprintStatus::Declined { .. })
                || matches!(right, FingerprintStatus::Declined { .. }),
            "the dependency witness must decline automatic clustering",
        );
    }
}

#[test]
fn validation_sample_remains_current_clause_recovery() {
    let catalogs = fixture_catalogs();
    for fixture in RECOVERY_FIXTURES {
        let parsed = crate::parse_with_catalogs(fixture.source, &catalogs);
        assert!(
            parsed.ast().recoveries().iter().any(|recovery| {
                recovery.role == RecoveryRole::Clause && recovery.text == fixture.source
            }),
            "fixture is no longer an exact Clause recovery: {:?}",
            fixture.source,
        );
    }
}

#[test]
fn cluster_keys_are_registration_order_invariant() {
    for index in [0, 3, 8, 10, 18, 20, 22, 27] {
        let source = RECOVERY_FIXTURES[index].source;
        let normal = cluster_outcome(source, RegistrationOrder::Normal);
        for order in [RegistrationOrder::Reversed, RegistrationOrder::FixedShuffle] {
            assert_eq!(
                cluster_outcome(source, order),
                normal,
                "registration order changed the key for {source:?}",
            );
        }
    }
}

#[test]
fn shared_seams_have_shared_cluster_keys() {
    let mut splits = Vec::new();
    for &(left, right) in SAME_SEAM_PAIRS {
        let left = RECOVERY_FIXTURES[left];
        let right = RECOVERY_FIXTURES[right];
        assert_eq!(left.seam, right.seam);
        let left_key = complete_key(left.source, RegistrationOrder::Normal);
        let right_key = complete_key(right.source, RegistrationOrder::Normal);
        if left_key != right_key {
            splits.push((left, left_key, right, right_key));
        }
    }
    assert!(splits.is_empty(), "shared seams split: {splits:#?}");
}

#[test]
fn unrelated_seams_do_not_share_cluster_keys() {
    let mut collisions = Vec::new();
    for &(left, right) in UNRELATED_PAIRS {
        let left = RECOVERY_FIXTURES[left];
        let right = RECOVERY_FIXTURES[right];
        assert_ne!(left.seam, right.seam);
        let left_key = status(left.source, RegistrationOrder::Normal, COMPLETE_LIMITS);
        let right_key = status(right.source, RegistrationOrder::Normal, COMPLETE_LIMITS);
        if left_key.cluster_key().is_some() && left_key.cluster_key() == right_key.cluster_key() {
            collisions.push((left, right, left_key.cluster_key().cloned()));
        }
    }
    assert!(
        collisions.is_empty(),
        "unrelated seams collided: {collisions:#?}"
    );
}

#[test]
fn capped_diagnostics_cannot_supply_cluster_keys() {
    let source = RECOVERY_FIXTURES[0].source;
    let event_capped = status(
        source,
        RegistrationOrder::Normal,
        DiagnosticLimits {
            max_events: 0,
            max_frontier: COMPLETE_LIMITS.max_frontier,
            max_lattice_states: COMPLETE_LIMITS.max_lattice_states,
        },
    );
    assert!(event_capped.cluster_key().is_none());
    assert!(
        matches!(
            event_capped,
            FingerprintStatus::Capped {
                dropped_events: 1..,
                ..
            }
        ),
        "zero event capacity did not report truncation: {event_capped:#?}",
    );

    let frontier_capped = RECOVERY_FIXTURES
        .iter()
        .map(|fixture| {
            status(
                fixture.source,
                RegistrationOrder::Normal,
                DiagnosticLimits {
                    max_events: COMPLETE_LIMITS.max_events,
                    max_frontier: 0,
                    max_lattice_states: COMPLETE_LIMITS.max_lattice_states,
                },
            )
        })
        .find(|status| {
            matches!(
                status,
                FingerprintStatus::Capped {
                    dropped_frontier: 1..,
                    ..
                }
            )
        })
        .expect("stratified sample must exercise frontier truncation");
    assert!(frontier_capped.cluster_key().is_none());
    assert!(
        matches!(
            frontier_capped,
            FingerprintStatus::Capped {
                dropped_frontier: 1..,
                ..
            }
        ),
        "zero frontier capacity did not report truncation: {frontier_capped:#?}",
    );
}
