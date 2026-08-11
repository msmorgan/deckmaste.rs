//! `cargo xtask english lint` — sound structural checks over the parsed
//! corpus.
//!
//! These are the adjudicating half of the pair whose discovery half is
//! [`super::shapes`]. Rarity mining produces a *worklist*; a lint here produces
//! a *finding*. The contract is machine-only adjudication: **a lint firing must
//! itself be proof of a defect**, needing no human or model to rule on the
//! card. Recall is explicitly subordinate to soundness — a check that would
//! need triage is either narrowed until it doesn't, or left out.
//!
//! Two of the checks are sound *by construction* (the AST cannot represent the
//! distinction the text carries, so information is lost whatever the card
//! means); one rests on an explicit CR-cited table; one uses the corpus as its
//! own control.

use std::collections::HashMap;

use anyhow::Result;
use clap::Args;
use clap::ValueEnum;
use deckmaste_english::parse_with_identity;
use deckmaste_english::syntax::NominalComplement;
use deckmaste_english::syntax::NounPhraseKind;
use deckmaste_english::syntax::ObjectGapPredicate;
use deckmaste_english::syntax::RelativeBody;
use deckmaste_english::word::NounInstanceKind;
use deckmaste_english::word::Verb;
use deckmaste_english::word::Vocab;

use crate::english::data::OracleDataArgs;
use crate::english::data::map_supported_faces;
use crate::english::shape;
use crate::english::shape::Shape;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum Check {
    /// Every check.
    All,
    /// A flat coordination mixing `and` with `or`.
    MixedConjunction,
    /// A determinerless singular member of a coordination of complete NPs.
    BareSingularConjunct,
    /// An object-gap relative whose verb cannot take its host.
    UncontrollableHost,
    /// Identical ability text parsing two different ways.
    DivergentParse,
}

#[derive(Debug, Args)]
pub(super) struct LintArgs {
    #[command(flatten)]
    data: OracleDataArgs,

    /// Which check to run.
    #[arg(long, value_enum, default_value_t = Check::All)]
    check: Check,

    /// Maximum findings to print per check.
    #[arg(long, default_value_t = 40)]
    limit: usize,

    /// Exit non-zero if any check fires.
    #[arg(long)]
    require_clean: bool,
}

#[derive(Debug, Clone)]
struct Finding {
    check: &'static str,
    card: String,
    detail: String,
}

fn elements<'a>(shape: &'a Shape, key: &str) -> &'a [Shape] {
    shape.field(key).map_or(&[], Shape::elements)
}

/// **Sound by construction.** A single flat member list carrying both `and`
/// and `or` cannot distinguish `(A or B) and C` from `A or (B and C)`: the
/// grouping the text expresses has no representation in the node, so a
/// consumer walking the list has to re-guess it. No semantics needed — the
/// information is simply absent.
fn mixed_conjunction(shape: &Shape, card: &str, found: &mut Vec<Finding>) {
    shape.walk(&mut |node| {
        let Shape::Node { fields, .. } = node else {
            return;
        };
        for (key, value) in fields {
            let mut present: Vec<&'static str> = Vec::new();
            for member in value.elements() {
                if let Some(conjunction) = member.field("conjunction")
                    && let Some(variant) = conjunction.variant()
                    && !present.contains(&variant)
                {
                    present.push(variant);
                }
            }
            if present.contains(&"And") && present.contains(&"Or") {
                found.push(Finding {
                    check: "mixed-conjunction",
                    card: card.to_string(),
                    detail: format!("{}.{key} mixes {}", node.label(), present.join(" + ")),
                });
            }
        }
    });
}

/// **Sound by construction, via asymmetry.** `CoordinatedNounPhrase`
/// coordinates *complete* noun phrases — its sibling
/// `CoordinatedNominalPhrase` is the shape for nominal material sharing one
/// determiner, where every member is determinerless by design.
///
/// The defect is not bareness but **directional, number-compatible
/// inconsistency**: the first member is a determined singular while a later
/// singular sibling has no determiner. English requires a determiner on a
/// singular count noun, so that sibling is not a complete noun phrase and the
/// preceding determiner scopes over it — `another target creature or artifact
/// you control` is one selection, not a determined phrase coordinated with a
/// bare noun. A determiner on a *later* member cannot scope backward, and a
/// plural/mass first head cannot prove that its determiner licenses the later
/// singular; those are different parse defects rather than shared-determiner
/// evidence. Likewise, a prepositional or infinitival complement closes the
/// first nominal before the conjunction, so its determiner cannot scope into a
/// later member.
///
/// A *uniformly* determinerless list is explicitly not a finding: `choose
/// Human, Merfolk, or Goblin` coordinates bare type names, and nothing has
/// been stranded. Measured over the corpus, dropping that case is the
/// difference between 1547 findings and a genuine tail — the earlier version
/// of this check counted every bare type-name list as a defect.
///
/// Restricted to `Singular` deliberately — bare plurals (`creatures you
/// control`) and mass nouns (`damage`) are legitimately determinerless. The
/// fused proform `one` is likewise complete without a determiner; an ellipsis
/// list such as `your hand, one into your graveyard, and one ...` must not be
/// diagnosed as shared possessive scope. A quantity modifier itself determines
/// its nominal (`at least one other Warrior`), while lexical opacity prevents
/// the lint from proving that a bare singular is a common noun rather than a
/// proper name (`Disenchant, Braingeyser, ...`).
fn bare_singular_conjunct(shape: &Shape, card: &str, found: &mut Vec<Finding>) {
    shape.walk(&mut |node| {
        if node.type_name() != Some("CoordinatedNounPhrase") {
            return;
        }
        let mut members: Vec<&Shape> = node.field("first").into_iter().collect();
        for member in elements(node, "rest") {
            if let Some(phrase) = member.field("phrase") {
                members.push(phrase);
            }
        }

        let Some(first) = members.first().map(|member| member.unwrapped()) else {
            return;
        };
        if first.type_name() != Some("NominalPhrase")
            || matches!(first.field("determiner"), Some(Shape::Absent))
            || first.field("head").and_then(Shape::variant) != Some("Singular")
            || !determiner_can_scope_forward(first)
        {
            return;
        }
        let first_is_this = first
            .field("determiner")
            .is_some_and(|determiner| determiner.unwrapped().variant() == Some("This"));

        // One finding per coordination, not per member: `target Shade,
        // Skeleton, … or Zombie` is a single stranded determiner, and counting
        // its six bare members six times would misreport how much is wrong.
        let stranded = members
            .iter()
            .skip(1)
            .map(|member| member.unwrapped())
            .filter(|member| {
                member.type_name() == Some("NominalPhrase")
                    && matches!(member.field("determiner"), Some(Shape::Absent))
                    && determinerless_singular_requires_determiner(member)
                    && !(first_is_this && is_attached_role_nominal(member))
            })
            .count();
        if stranded > 0 {
            found.push(Finding {
                check: "bare-singular-conjunct",
                card: card.to_string(),
                detail: format!(
                    "the first determined conjunct precedes {stranded} determinerless singular one(s)"
                ),
            });
        }
    });
}

fn determiner_can_scope_forward(nominal: &Shape) -> bool {
    elements(nominal, "complements")
        .iter()
        .all(|complement| !matches!(complement.variant(), Some("Prepositional" | "Infinitive")))
}

/// `equipped creature` and `enchanted creature` are contextually definite
/// attachment roles, so `this creature or equipped creature` coordinates two
/// complete noun phrases; `this` does not scope over the second member. Keep
/// the gate on the typed participial verb identities rather than spellings so
/// unrelated participles (`tapped creature`) remain lintable.
fn is_attached_role_nominal(nominal: &Shape) -> bool {
    elements(nominal, "modifiers").iter().any(|modifier| {
        let modifier = modifier.unwrapped();
        if modifier.variant() != Some("Adjective") {
            return false;
        }
        let Some(head) = modifier
            .field("phrase")
            .and_then(|phrase| phrase.field("head"))
        else {
            return false;
        };
        let [_, verb] = head.elements() else {
            return false;
        };
        matches!(verb.unwrapped().variant(), Some("Equip" | "Enchant"))
    })
}

fn determinerless_singular_requires_determiner(nominal: &Shape) -> bool {
    let Some(head) = nominal.field("head") else {
        return false;
    };
    if head.variant() != Some("Singular") || head.unwrapped().variant() == Some("One") {
        return false;
    }
    if elements(nominal, "modifiers")
        .iter()
        .any(|modifier| modifier.variant() == Some("Quantity"))
    {
        return false;
    }
    let mut contains_opacity = false;
    nominal.walk(&mut |node| {
        contains_opacity |= node.variant() == Some("Opaque");
    });
    !contains_opacity
}

/// **Sound by table.** An object-gap relative attaches its gap to the host
/// noun. When the verb filling that gap can only take an object in the rules
/// sense [CR#109.1] and the host is a mass noun such as `damage` — which rule
/// 109.1 does not list, because damage is what objects *deal* [CR#120.1] — the
/// clause cannot belong to this host and has been misattached. Round-trip
/// cannot see this: the tokens render back unchanged.
fn uncontrollable_host(
    ast: &deckmaste_english::syntax::OracleText,
    card: &str,
    found: &mut Vec<Finding>,
) {
    for phrase in ast.noun_phrases() {
        let NounPhraseKind::Nominal(nominal) = phrase.kind() else {
            continue;
        };
        if !matches!(nominal.head().kind(), NounInstanceKind::Mass(_)) {
            continue;
        }
        for complement in nominal.complements() {
            let NominalComplement::Relative(relative) = complement else {
                continue;
            };
            let RelativeBody::ObjectGap { predicate, .. } = relative.body() else {
                continue;
            };
            let Some(verb) = object_taking_verb(predicate) else {
                continue;
            };
            found.push(Finding {
                check: "uncontrollable-host",
                card: card.to_string(),
                detail: format!(
                    "an object-gap relative headed by `{verb}` attaches to a mass-noun host"
                ),
            });
        }
    }
}

/// Verbs whose object must be an object in the rules sense [CR#109.1]. Damage
/// is not among the things rule 109.1 lists; it is what objects *deal*
/// [CR#120.1], so no amount of damage can be the gap of `… that X controls`.
/// The table is deliberately expressed in typed lexical identities rather
/// than serialized enum names.
fn object_taking_verb(predicate: &ObjectGapPredicate) -> Option<&'static str> {
    match predicate.head().verb().verb {
        Verb::Word(Vocab::Control) => Some("Control"),
        Verb::Word(Vocab::Own) => Some("Own"),
        Verb::Word(_) | Verb::KeywordAction(_) => None,
    }
}

/// One ability's text paired with the shape it parsed to.
struct AbilityParse {
    text: String,
    legendary: bool,
    shape: String,
    card: String,
}

/// Pair each ability with its source line, conservatively: only when the line
/// count matches the ability count, so a multi-line ability never mispairs.
fn ability_parses(
    card_name: &str,
    oracle: &str,
    legendary: bool,
    ast: &Shape,
) -> Vec<AbilityParse> {
    let lines: Vec<&str> = oracle
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();
    let abilities = elements(ast, "abilities");
    if lines.len() != abilities.len() {
        return Vec::new();
    }
    lines
        .iter()
        .zip(abilities)
        .filter(|(line, _)| !line.contains(card_name))
        .map(|(line, ability)| AbilityParse {
            text: (*line).to_string(),
            legendary,
            shape: ability.fingerprint(),
            card: card_name.to_string(),
        })
        .collect()
}

/// **Sound by corpus control.** Identical ability text must parse identically:
/// if two cards disagree, at most one can be right, and the divergence alone
/// is the finding — no ground truth needed.
///
/// The guard is what makes it sound. `parse_with_identity` takes the card's
/// name and legendary status, so the *same* text can legitimately parse
/// differently on different cards. Both inputs are therefore neutralised:
/// abilities naming their own card are dropped, and the legendary flag is part
/// of the grouping key rather than something two cards may differ on.
fn divergent_parses(parses: Vec<AbilityParse>, limit: usize) -> Vec<Finding> {
    let mut groups: HashMap<(String, bool), Vec<AbilityParse>> = HashMap::new();
    for parse in parses {
        groups
            .entry((parse.text.clone(), parse.legendary))
            .or_default()
            .push(parse);
    }

    let mut found = Vec::new();
    let mut keys: Vec<_> = groups.keys().cloned().collect();
    keys.sort();
    for key in keys {
        let group = &groups[&key];
        let Some(first) = group.first() else {
            continue;
        };
        let Some(divergent) = group.iter().find(|parse| parse.shape != first.shape) else {
            continue;
        };
        found.push(Finding {
            check: "divergent-parse",
            card: format!("{} vs {}", first.card, divergent.card),
            detail: format!(
                "identical ability text parses two ways: {:?}",
                truncate(&key.0)
            ),
        });
        if found.len() >= limit {
            break;
        }
    }
    found
}

fn truncate(text: &str) -> String {
    const WIDTH: usize = 90;
    if text.chars().count() <= WIDTH {
        return text.to_string();
    }
    let head: String = text.chars().take(WIDTH).collect();
    format!("{head}…")
}

pub(super) fn run(args: &LintArgs) -> Result<()> {
    let data = args.data.load()?;
    let wants = |check: Check| args.check == Check::All || args.check == check;

    let per_face = map_supported_faces(&data.faces, |_, card| {
        let report = parse_with_identity(
            &card.oracle_text,
            &data.catalogs,
            card.printed_name(),
            card.is_legendary,
        );
        let ast = report.ast();
        let ast_shape = shape::of(ast);
        let name = card.printed_name();

        let mut found = Vec::new();
        if wants(Check::MixedConjunction) {
            mixed_conjunction(&ast_shape, name, &mut found);
        }
        if wants(Check::BareSingularConjunct) {
            bare_singular_conjunct(&ast_shape, name, &mut found);
        }
        if wants(Check::UncontrollableHost) {
            uncontrollable_host(ast, name, &mut found);
        }
        let parses = if wants(Check::DivergentParse) {
            ability_parses(name, &card.oracle_text, card.is_legendary, &ast_shape)
        } else {
            Vec::new()
        };
        (found, parses)
    });

    let mut findings = Vec::new();
    let mut parses = Vec::new();
    for (face_findings, face_parses) in per_face {
        findings.extend(face_findings);
        parses.extend(face_parses);
    }
    if wants(Check::DivergentParse) {
        // Coverage, not decoration: this check pairs abilities with source
        // lines only when the counts match, so an unreported check and a
        // clean one look identical without it. Say what was compared.
        let compared = parses.len();
        let shared = {
            let mut counts: HashMap<(&str, bool), usize> = HashMap::new();
            for parse in &parses {
                *counts
                    .entry((parse.text.as_str(), parse.legendary))
                    .or_default() += 1;
            }
            counts.values().filter(|count| **count > 1).count()
        };
        eprintln!(
            "divergent-parse: {compared} abilities paired with their source line, \
             {shared} texts shared by two or more cards"
        );
        findings.extend(divergent_parses(parses, args.limit));
    }

    let mut by_check: HashMap<&'static str, Vec<Finding>> = HashMap::new();
    for finding in findings {
        by_check.entry(finding.check).or_default().push(finding);
    }

    let mut checks: Vec<_> = by_check.keys().copied().collect();
    checks.sort_unstable();

    let mut total = 0;
    println!("check\tcard\tdetail");
    for check in checks {
        let group = &by_check[check];
        total += group.len();
        for finding in group.iter().take(args.limit) {
            println!("{check}\t{}\t{}", finding.card, finding.detail);
        }
        if group.len() > args.limit {
            eprintln!(
                "{check}: {} findings, {} shown (raise --limit)",
                group.len(),
                args.limit
            );
        }
    }

    if total == 0 {
        eprintln!("no findings");
    } else {
        eprintln!("{total} findings");
    }
    if args.require_clean && total > 0 {
        anyhow::bail!("{total} lint findings");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use serde::Serialize;

    use super::*;

    #[derive(Serialize)]
    enum Conjunction {
        And,
        Or,
    }

    #[derive(Serialize)]
    struct Member {
        conjunction: Option<Conjunction>,
    }

    #[derive(Serialize)]
    struct Coordination {
        rest: Vec<Member>,
    }

    fn coordination(conjunctions: Vec<Option<Conjunction>>) -> Coordination {
        Coordination {
            rest: conjunctions
                .into_iter()
                .map(|conjunction| Member { conjunction })
                .collect(),
        }
    }

    fn run_mixed(value: &Coordination) -> Vec<Finding> {
        let mut found = Vec::new();
        mixed_conjunction(&shape::of(value), "Fixture", &mut found);
        found
    }

    #[test]
    fn a_flat_list_mixing_and_with_or_is_unrepresentable() {
        let found = run_mixed(&coordination(vec![
            Some(Conjunction::Or),
            Some(Conjunction::And),
        ]));
        assert_eq!(found.len(), 1, "the grouping has no representation");
    }

    #[test]
    fn a_uniform_list_is_unambiguous() {
        let found = run_mixed(&coordination(vec![
            Some(Conjunction::Or),
            Some(Conjunction::Or),
        ]));
        assert!(found.is_empty(), "one connective throughout groups flatly");
    }

    #[test]
    fn an_asyndetic_member_does_not_count_as_a_connective() {
        let found = run_mixed(&coordination(vec![None, Some(Conjunction::Or)]));
        assert!(
            found.is_empty(),
            "an Oxford list's comma-only member carries no conjunction"
        );
    }

    #[test]
    fn uncontrollable_host_queries_typed_relative_and_predicate_semantics() {
        let report = deckmaste_english::parse_with_identity(
            "Destroy target creature you control.",
            &deckmaste_english::Catalogs::default(),
            "Fixture",
            false,
        );
        let deckmaste_english::syntax::AbilityKind::Paragraph(paragraph) =
            report.ast().abilities[0].kind()
        else {
            panic!("expected a paragraph ability");
        };
        let deckmaste_english::syntax::SentenceBody::Independent(
            deckmaste_english::syntax::IndependentClause::Finite(clause),
        ) = paragraph.sentences[0].body()
        else {
            panic!("expected an imperative transitive clause");
        };
        let None = clause.subject() else {
            panic!("expected an implicit imperative subject");
        };
        let deckmaste_english::syntax::PredicateExpression::Simple(
            deckmaste_english::syntax::Predicate::Transitive(predicate),
        ) = clause.predicate()
        else {
            panic!("expected an imperative transitive predicate");
        };
        let deckmaste_english::syntax::PredicateObject::NounPhrase(noun) = predicate.object()
        else {
            panic!("expected a noun-phrase object");
        };
        let NounPhraseKind::Nominal(nominal) = noun.kind() else {
            panic!("expected a nominal object");
        };
        let [NominalComplement::Relative(relative)] = nominal.complements() else {
            panic!("expected one relative complement");
        };
        let RelativeBody::ObjectGap { predicate, .. } = relative.body() else {
            panic!("expected an object-gap relative");
        };
        assert_eq!(object_taking_verb(predicate), Some("Control"));
        assert!(
            report
                .ast()
                .noun_phrases()
                .iter()
                .any(|candidate| std::ptr::eq(*candidate, noun)),
            "the shared semantic traversal must reach the predicate object"
        );

        let mut found = Vec::new();
        uncontrollable_host(report.ast(), "Fixture", &mut found);
        assert!(found.is_empty(), "a singular creature host is valid");
    }

    #[derive(Serialize)]
    enum TestVocab {
        One,
        Equip,
        Enchant,
        Tap,
    }

    #[derive(Serialize)]
    enum TestDeterminer {
        Target,
        Possessive,
        Quantity,
        Each,
        This,
    }

    #[derive(Serialize)]
    enum TestTense {
        Past,
    }

    #[derive(Serialize)]
    enum TestVerb {
        Word(TestVocab),
    }

    #[derive(Serialize)]
    enum TestAdjective {
        Participle(TestTense, TestVerb),
    }

    #[derive(Serialize)]
    struct TestAdjectivePhrase {
        head: TestAdjective,
    }

    #[derive(Serialize)]
    enum TestNominalModifier {
        Adjective { phrase: TestAdjectivePhrase },
    }

    #[derive(Serialize)]
    enum TestNominalComplement {
        Prepositional,
    }

    #[derive(Serialize)]
    enum TestNoun {
        Text(&'static str),
        Word(TestVocab),
    }

    #[derive(Serialize)]
    enum Head {
        Singular(TestNoun),
        Plural(TestNoun),
    }

    fn singular(text: &'static str) -> Head {
        Head::Singular(TestNoun::Text(text))
    }

    fn plural(text: &'static str) -> Head {
        Head::Plural(TestNoun::Text(text))
    }

    #[derive(Serialize)]
    struct NominalPhrase {
        determiner: Option<TestDeterminer>,
        modifiers: Vec<TestNominalModifier>,
        head: Head,
        complements: Vec<TestNominalComplement>,
    }

    #[derive(Serialize)]
    enum FixtureNounPhrase {
        Nominal(NominalPhrase),
    }

    #[derive(Serialize)]
    struct NounPhraseCoordination {
        phrase: FixtureNounPhrase,
    }

    #[derive(Serialize)]
    struct CoordinatedNounPhrase {
        first: FixtureNounPhrase,
        rest: Vec<NounPhraseCoordination>,
    }

    fn nominal(determiner: Option<TestDeterminer>, head: Head) -> FixtureNounPhrase {
        FixtureNounPhrase::Nominal(NominalPhrase {
            determiner,
            modifiers: Vec::new(),
            head,
            complements: Vec::new(),
        })
    }

    fn participial_role(verb: TestVocab, head: Head) -> FixtureNounPhrase {
        FixtureNounPhrase::Nominal(NominalPhrase {
            determiner: None,
            modifiers: vec![TestNominalModifier::Adjective {
                phrase: TestAdjectivePhrase {
                    head: TestAdjective::Participle(TestTense::Past, TestVerb::Word(verb)),
                },
            }],
            head,
            complements: Vec::new(),
        })
    }

    fn prepositionally_closed(determiner: TestDeterminer, head: Head) -> FixtureNounPhrase {
        FixtureNounPhrase::Nominal(NominalPhrase {
            determiner: Some(determiner),
            modifiers: Vec::new(),
            head,
            complements: vec![TestNominalComplement::Prepositional],
        })
    }

    fn coordinated(members: Vec<FixtureNounPhrase>) -> CoordinatedNounPhrase {
        let mut members = members.into_iter();
        let first = members.next().expect("a coordination has a first member");
        CoordinatedNounPhrase {
            first,
            rest: members
                .map(|phrase| NounPhraseCoordination { phrase })
                .collect(),
        }
    }

    fn run_bare(value: &CoordinatedNounPhrase) -> Vec<Finding> {
        let mut found = Vec::new();
        bare_singular_conjunct(&shape::of(value), "Fixture", &mut found);
        found
    }

    #[test]
    fn a_determined_conjunct_beside_a_bare_singular_strands_its_determiner() {
        // `another target creature or artifact you control`
        let found = run_bare(&coordinated(vec![
            nominal(Some(TestDeterminer::Target), singular("creature")),
            nominal(None, singular("artifact")),
        ]));
        assert_eq!(found.len(), 1);
    }

    #[test]
    fn a_uniformly_bare_list_strands_nothing() {
        // `choose Human, Merfolk, or Goblin` — bare type names throughout.
        let found = run_bare(&coordinated(vec![
            nominal(None, singular("Human")),
            nominal(None, singular("Merfolk")),
            nominal(None, singular("Goblin")),
        ]));
        assert!(
            found.is_empty(),
            "bareness alone is not the defect — inconsistency is"
        );
    }

    #[test]
    fn a_later_determiner_does_not_scope_backward() {
        // `block, and its activated abilities` is malformed for a different
        // reason: `its` cannot determine the preceding `block` nominal.
        let found = run_bare(&coordinated(vec![
            nominal(None, singular("block")),
            nominal(Some(TestDeterminer::Possessive), plural("abilities")),
        ]));
        assert!(found.is_empty());
    }

    #[test]
    fn a_determined_plural_does_not_license_a_later_singular_by_itself() {
        // `two counters ... or suspended card` proves an attachment defect,
        // not that `two` determines the singular `card`.
        let found = run_bare(&coordinated(vec![
            nominal(Some(TestDeterminer::Quantity), plural("counters")),
            nominal(None, singular("card")),
        ]));
        assert!(found.is_empty());
    }

    #[test]
    fn a_closed_first_nominal_does_not_license_a_later_singular() {
        // Spirit Flare currently selects `its power to target attacking or
        // blocking creature`; that is an attachment defect, not evidence that
        // `its` scopes across the coordination.
        let found = run_bare(&coordinated(vec![
            prepositionally_closed(TestDeterminer::Possessive, singular("power")),
            nominal(None, singular("creature")),
        ]));
        assert!(found.is_empty());
    }

    #[test]
    fn a_bare_plural_conjunct_is_legitimate() {
        let found = run_bare(&coordinated(vec![
            nominal(Some(TestDeterminer::Target), singular("creature")),
            nominal(None, plural("lands")),
        ]));
        assert!(found.is_empty(), "bare plurals need no determiner");
    }

    #[test]
    fn a_fused_one_is_not_a_bare_count_nominal() {
        let found = run_bare(&coordinated(vec![
            nominal(Some(TestDeterminer::Possessive), singular("hand")),
            nominal(None, Head::Singular(TestNoun::Word(TestVocab::One))),
        ]));
        assert!(
            found.is_empty(),
            "fused `one` supplies its own quantification"
        );
    }

    #[test]
    fn a_uniformly_determined_list_is_not_a_finding() {
        let found = run_bare(&coordinated(vec![
            nominal(Some(TestDeterminer::Target), singular("creature")),
            nominal(Some(TestDeterminer::Each), singular("player")),
        ]));
        assert!(found.is_empty());
    }

    #[test]
    fn attached_creature_roles_are_independently_definite_after_this_creature() {
        for role in [TestVocab::Equip, TestVocab::Enchant] {
            let found = run_bare(&coordinated(vec![
                nominal(Some(TestDeterminer::This), singular("creature")),
                participial_role(role, singular("creature")),
            ]));
            assert!(found.is_empty());
        }
    }

    #[test]
    fn an_unrelated_participle_does_not_silence_the_lint() {
        let found = run_bare(&coordinated(vec![
            nominal(Some(TestDeterminer::This), singular("creature")),
            participial_role(TestVocab::Tap, singular("creature")),
        ]));
        assert_eq!(found.len(), 1);
    }

    #[test]
    fn identical_text_parsing_identically_is_not_a_finding() {
        let parses = vec![
            AbilityParse {
                text: "Flying".to_string(),
                legendary: false,
                shape: "Keyword".to_string(),
                card: "A".to_string(),
            },
            AbilityParse {
                text: "Flying".to_string(),
                legendary: false,
                shape: "Keyword".to_string(),
                card: "B".to_string(),
            },
        ];
        assert!(divergent_parses(parses, 10).is_empty());
    }

    #[test]
    fn identical_text_parsing_differently_is_a_finding() {
        let parses = vec![
            AbilityParse {
                text: "Flying".to_string(),
                legendary: false,
                shape: "Keyword".to_string(),
                card: "A".to_string(),
            },
            AbilityParse {
                text: "Flying".to_string(),
                legendary: false,
                shape: "Paragraph".to_string(),
                card: "B".to_string(),
            },
        ];
        assert_eq!(divergent_parses(parses, 10).len(), 1);
    }

    #[test]
    fn the_legendary_flag_is_part_of_the_grouping_key() {
        // `parse_with_identity` takes it, so two cards differing only in it may
        // legitimately parse differently — that must not read as a defect.
        let parses = vec![
            AbilityParse {
                text: "Flying".to_string(),
                legendary: false,
                shape: "Keyword".to_string(),
                card: "A".to_string(),
            },
            AbilityParse {
                text: "Flying".to_string(),
                legendary: true,
                shape: "Paragraph".to_string(),
                card: "B".to_string(),
            },
        ];
        assert!(divergent_parses(parses, 10).is_empty());
    }
}
