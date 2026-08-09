use super::Vocab;
use super::Vocabulary;
use super::noun::consonant_y_stem;
use super::noun::has_sibilant_ending;
use crate::catalog::KeywordAction;
use crate::features::ChartFeature;
use crate::features::ComplementRole;
use crate::features::FeatureKind;
use crate::features::GrammaticalFeature;
use crate::features::Number;
use crate::features::Person;
use crate::syntax::Preposition;
use crate::syntax::VerbParticle;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
pub enum Verb {
    Word(Vocab),
    KeywordAction(KeywordAction),
}

/// Compatibility name for the inherent-realization verb-slot feature.
pub use crate::features::VerbSlot;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
pub struct VerbInstance {
    pub verb: Verb,
    pub slot: VerbSlot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub(crate) enum ArgumentRequirement {
    Forbidden,
    Optional,
    Required,
}

impl ArgumentRequirement {
    pub(crate) const fn accepts(self) -> bool {
        !matches!(self, Self::Forbidden)
    }

    pub(crate) const fn is_satisfied_by(self, present: bool) -> bool {
        match self {
            Self::Forbidden => !present,
            Self::Optional => true,
            Self::Required => present,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub(crate) enum PredicateComplementKind {
    Adjective,
    Infinitive,
    Ability,
    Scalar,
    Statistic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub(crate) enum BareNominalAdjunct {
    Temporal,
    Manner,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
#[allow(
    clippy::struct_excessive_bools,
    reason = "frame model is stable and shared across many call sites"
)]
pub(crate) struct PredicateFrame {
    direct_object: ArgumentRequirement,
    indirect_object: ArgumentRequirement,
    adjective_complement: bool,
    infinitive_complement: bool,
    ability_complement: bool,
    scalar_complement: bool,
    statistic_complement: bool,
    selected_preposition: ArgumentRequirement,
    selected_prepositions: &'static [Preposition],
    prepositional_adjuncts: bool,
    bare_nominal_adjuncts: &'static [BareNominalAdjunct],
    pub(crate) particles: &'static [VerbParticle],
    proform: bool,
    /// Whether the verb takes a causative bare-infinitive complement after its
    /// object (`have <object> <bare VP>`). Set only for the causative `have`;
    /// every other frame leaves it off, so the `VerbPhrase = VerbPhrase
    /// VerbPhrase` production reduces to nothing outside this construction.
    causative_complement: bool,
    /// Whether the passive of this frame promotes the RECIPIENT (indirect
    /// object) rather than the theme, RETAINING the direct object
    /// post-verbally: `X was dealt damage`. Set only on a passive-only frame;
    /// `predicate_arguments_complete` rejects it outright in the active, so
    /// no double-object active reading is ever licensed.
    recipient_passive: bool,
    /// Whether this frame is pending a required `CoinResult` predicate tail
    /// (`come up heads`/`come up tails`). Set only on the narrow `Come` frame
    /// used for the coin-result predicate; `predicate_arguments_complete` and
    /// `predicate_object_gap_complete` both reject a pending frame outright,
    /// so `Come` is never selectable OPEN in this round. Only the generated
    /// `verb_phrase_coin_result` construction discharges the coin-result frame
    /// and can complete it. This keeps `whichever comes first` and `came under
    /// your control` (a general `come` frame, not yet added) out of the blast
    /// radius.
    requires_coin_result: bool,
}

impl GrammaticalFeature for PredicateFrame {
    const KIND: FeatureKind = FeatureKind::LexicalValency;
}

impl ChartFeature for PredicateFrame {}

impl PredicateFrame {
    pub(crate) const OPEN: Self = Self {
        direct_object: ArgumentRequirement::Optional,
        indirect_object: ArgumentRequirement::Forbidden,
        adjective_complement: true,
        infinitive_complement: true,
        ability_complement: true,
        scalar_complement: true,
        statistic_complement: true,
        selected_preposition: ArgumentRequirement::Forbidden,
        selected_prepositions: &[],
        prepositional_adjuncts: true,
        bare_nominal_adjuncts: &[BareNominalAdjunct::Temporal, BareNominalAdjunct::Manner],
        particles: &[],
        proform: false,
        causative_complement: false,
        recipient_passive: false,
        requires_coin_result: false,
    };

    const fn with_direct_object(mut self, requirement: ArgumentRequirement) -> Self {
        self.direct_object = requirement;
        self
    }

    const fn with_indirect_object(mut self, requirement: ArgumentRequirement) -> Self {
        self.indirect_object = requirement;
        self
    }

    const fn with_selected_prepositions(
        mut self,
        requirement: ArgumentRequirement,
        prepositions: &'static [Preposition],
    ) -> Self {
        self.selected_preposition = requirement;
        self.selected_prepositions = prepositions;
        self
    }

    const fn with_prepositional_adjuncts(mut self, allowed: bool) -> Self {
        self.prepositional_adjuncts = allowed;
        self
    }

    const fn with_particles(mut self, particles: &'static [VerbParticle]) -> Self {
        self.particles = particles;
        self
    }

    const fn with_proform(mut self) -> Self {
        self.proform = true;
        self
    }

    const fn with_causative_complement(mut self) -> Self {
        self.causative_complement = true;
        self
    }

    const fn with_required_coin_result(mut self) -> Self {
        self.requires_coin_result = true;
        self
    }

    const fn with_recipient_passive(mut self) -> Self {
        self.recipient_passive = true;
        self
    }

    pub(crate) const fn direct_object(self) -> ArgumentRequirement {
        self.direct_object
    }

    pub(crate) const fn indirect_object(self) -> ArgumentRequirement {
        self.indirect_object
    }

    pub(crate) const fn selected_preposition(self) -> ArgumentRequirement {
        self.selected_preposition
    }

    pub(crate) const fn is_proform(self) -> bool {
        self.proform
    }

    pub(crate) const fn causative_complement(self) -> bool {
        self.causative_complement
    }

    pub(crate) const fn is_recipient_passive(self) -> bool {
        self.recipient_passive
    }

    pub(crate) const fn requires_coin_result(self) -> bool {
        self.requires_coin_result
    }

    /// Returns a copy of this frame with `requires_coin_result` cleared.
    /// Called only from the generated `verb_phrase_coin_result` construction,
    /// the sole frame-discharge owner, once its typed `CoinResult` tail has
    /// attached, so the completed predicate's frame stops rejecting further
    /// completion checks.
    pub(crate) const fn discharge_coin_result(mut self) -> Self {
        self.requires_coin_result = false;
        self
    }

    pub(crate) const fn licenses_complement(self, kind: PredicateComplementKind) -> bool {
        match kind {
            PredicateComplementKind::Adjective => self.adjective_complement,
            PredicateComplementKind::Infinitive => self.infinitive_complement,
            PredicateComplementKind::Ability => {
                self.direct_object.accepts() && self.ability_complement
            }
            PredicateComplementKind::Scalar => {
                self.direct_object.accepts() && self.scalar_complement
            }
            PredicateComplementKind::Statistic => {
                self.direct_object.accepts() && self.statistic_complement
            }
        }
    }

    pub(crate) fn prepositional_role(self, preposition: Preposition) -> Option<ComplementRole> {
        if self.selected_prepositions.contains(&preposition) {
            Some(ComplementRole::SelectedComplement)
        } else if self.prepositional_adjuncts {
            Some(ComplementRole::Adjunct)
        } else {
            None
        }
    }

    pub(crate) fn licenses_bare_nominal_adjunct(self, adjunct: BareNominalAdjunct) -> bool {
        self.bare_nominal_adjuncts.contains(&adjunct)
    }

    pub(crate) fn licenses_particle(self, particle: VerbParticle) -> bool {
        self.particles.contains(&particle)
    }
}

pub(super) const OPEN_PREDICATE_FRAMES: &[PredicateFrame] = &[PredicateFrame::OPEN];
pub(super) const INTRANSITIVE_PREDICATE_FRAME: PredicateFrame =
    PredicateFrame::OPEN.with_direct_object(ArgumentRequirement::Forbidden);
pub(super) const INTRANSITIVE_PREDICATE_FRAMES: &[PredicateFrame] = &[INTRANSITIVE_PREDICATE_FRAME];
pub(super) const REQUIRED_OBJECT_PREDICATE_FRAMES: &[PredicateFrame] =
    &[PredicateFrame::OPEN.with_direct_object(ArgumentRequirement::Required)];
/// The narrow `Come` frame used only by the coin-result predicate
/// (`come up heads`/`come up tails`): no direct object, and pending until a
/// `CoinResult` tail attaches and discharges `requires_coin_result`. `Come`
/// intentionally has no other frame in this round, so `whichever comes
/// first` and `came under your control` stay unaffected (they fail to
/// license any frame and remain residue for a future general `come` round).
pub(super) const COME_PREDICATE_FRAMES: &[PredicateFrame] = &[PredicateFrame::OPEN
    .with_direct_object(ArgumentRequirement::Forbidden)
    .with_required_coin_result()];
/// `have` as a grant/possession verb (required object) plus its causative
/// reading, which additionally takes a bare-infinitive complement after the
/// object (`have this creature enter as a copy of …`).
pub(super) const HAVE_PREDICATE_FRAMES: &[PredicateFrame] = &[
    PredicateFrame::OPEN.with_direct_object(ArgumentRequirement::Required),
    PredicateFrame::OPEN
        .with_direct_object(ArgumentRequirement::Required)
        .with_causative_complement(),
];
pub(crate) const PROFORM_PREDICATE_FRAMES: &[PredicateFrame] = &[
    INTRANSITIVE_PREDICATE_FRAME.with_proform(),
    PredicateFrame::OPEN.with_proform(),
];
pub(super) const ASK_PREDICATE_FRAMES: &[PredicateFrame] = &[
    PredicateFrame::OPEN.with_direct_object(ArgumentRequirement::Required),
    PredicateFrame::OPEN
        .with_direct_object(ArgumentRequirement::Required)
        .with_indirect_object(ArgumentRequirement::Required),
];
/// `deal` is ditransitive in the rules idiom: the theme is the direct object
/// and the recipient surfaces in a `to` phrase in the active voice. Its
/// passive promotes the RECIPIENT and retains the theme (`an opponent was
/// dealt damage this turn`), which no other frame in this grammar does. The
/// retained-object frame is passive-only — the canonical-template domain has
/// no double-object active (`deals target player 2 damage`) — so the active
/// `deals N damage to X` reading remains exactly `PredicateFrame::OPEN` and
/// its trees are unchanged by construction, not by cost.
pub(super) const RECIPIENT_PASSIVE_PREDICATE_FRAMES: &[PredicateFrame] = &[
    PredicateFrame::OPEN,
    PredicateFrame::OPEN
        .with_direct_object(ArgumentRequirement::Required)
        .with_indirect_object(ArgumentRequirement::Required)
        .with_recipient_passive(),
];
pub(super) const ATTACK_PREDICATE_FRAMES: &[PredicateFrame] =
    &[INTRANSITIVE_PREDICATE_FRAME, PredicateFrame::OPEN];
pub(super) const LOOK_PREDICATE_FRAMES: &[PredicateFrame] = &[
    INTRANSITIVE_PREDICATE_FRAME.with_prepositional_adjuncts(false),
    INTRANSITIVE_PREDICATE_FRAME
        .with_selected_prepositions(ArgumentRequirement::Optional, &[Preposition::At]),
];
pub(super) const PHASE_PREDICATE_FRAMES: &[PredicateFrame] =
    &[PredicateFrame::OPEN.with_particles(&[VerbParticle::In, VerbParticle::Out])];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum VerbForm {
    Regular,
    Irregular(IrregularVerbDef),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub struct IrregularVerbDef {
    pub present_second: Option<&'static str>,
    pub present_third_singular: Option<&'static str>,
    pub present_third_plural: Option<&'static str>,
    pub past_second: Option<&'static str>,
    pub past_third_singular: Option<&'static str>,
    pub past_third_plural: Option<&'static str>,
    pub present_participle: Option<&'static str>,
    pub past_participle: Option<&'static str>,
}

impl IrregularVerbDef {
    pub const EMPTY: Self = Self {
        present_second: None,
        present_third_singular: None,
        present_third_plural: None,
        past_second: None,
        past_third_singular: None,
        past_third_plural: None,
        present_participle: None,
        past_participle: None,
    };

    #[must_use]
    pub const fn with_present(
        mut self,
        second: &'static str,
        third_singular: &'static str,
        third_plural: &'static str,
    ) -> Self {
        self.present_second = Some(second);
        self.present_third_singular = Some(third_singular);
        self.present_third_plural = Some(third_plural);
        self
    }

    #[must_use]
    pub const fn with_present_third_singular(mut self, form: &'static str) -> Self {
        self.present_third_singular = Some(form);
        self
    }

    #[must_use]
    pub const fn with_past(mut self, form: &'static str) -> Self {
        self.past_second = Some(form);
        self.past_third_singular = Some(form);
        self.past_third_plural = Some(form);
        self
    }

    #[must_use]
    pub const fn with_past_agreement(
        mut self,
        second: &'static str,
        third_singular: &'static str,
        third_plural: &'static str,
    ) -> Self {
        self.past_second = Some(second);
        self.past_third_singular = Some(third_singular);
        self.past_third_plural = Some(third_plural);
        self
    }

    #[must_use]
    pub const fn with_present_participle(mut self, form: &'static str) -> Self {
        self.present_participle = Some(form);
        self
    }

    #[must_use]
    pub const fn with_past_participle(mut self, form: &'static str) -> Self {
        self.past_participle = Some(form);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
pub struct VerbDefinition {
    pub verb: Verb,
    pub form: VerbForm,
}

impl Verb {
    pub(crate) fn predicate_frames(&self) -> &'static [PredicateFrame] {
        match self {
            Self::Word(vocab) => vocab.predicate_frames(),
            Self::KeywordAction(_) => OPEN_PREDICATE_FRAMES,
        }
    }
}

impl Vocabulary {
    #[must_use]
    pub fn verb_definition(self, vocab: Vocab) -> Option<VerbDefinition> {
        vocab.definition().verb.map(|form| VerbDefinition {
            verb: Verb::Word(vocab),
            form,
        })
    }

    pub(crate) fn irregular_keyword_action_head(surface: &str) -> Option<Vocab> {
        Vocab::ALL.iter().copied().find(|vocab| {
            matches!(vocab.definition().verb, Some(VerbForm::Irregular(_)))
                && vocab.spelling().eq_ignore_ascii_case(surface)
        })
    }

    pub(crate) fn render_regular_verb(lemma: &str, slot: VerbSlot) -> String {
        render_verb_form(lemma, VerbForm::Regular, slot)
    }

    #[must_use]
    pub fn render_verb(self, vocab: Vocab, slot: VerbSlot) -> Option<String> {
        let definition = vocab.definition();
        let form = definition.verb?;
        Some(render_verb_form(definition.spelling, form, slot))
    }

    #[must_use]
    pub fn render_verb_instance(self, verb: &VerbInstance) -> Option<String> {
        self.render_verb_identity(&verb.verb, verb.slot)
    }

    pub(super) fn render_verb_identity(self, verb: &Verb, slot: VerbSlot) -> Option<String> {
        match verb {
            Verb::Word(vocab) => self.render_verb(*vocab, slot),
            Verb::KeywordAction(action) => action.render(slot),
        }
    }
}

pub(crate) const VERB_SLOTS: [VerbSlot; 12] = [
    VerbSlot::Infinitive,
    VerbSlot::Imperative,
    VerbSlot::Present {
        person: Person::Second,
        number: Number::Singular,
    },
    VerbSlot::Present {
        person: Person::Second,
        number: Number::Plural,
    },
    VerbSlot::Present {
        person: Person::Third,
        number: Number::Singular,
    },
    VerbSlot::Present {
        person: Person::Third,
        number: Number::Plural,
    },
    VerbSlot::Past {
        person: Person::Second,
        number: Number::Singular,
    },
    VerbSlot::Past {
        person: Person::Second,
        number: Number::Plural,
    },
    VerbSlot::Past {
        person: Person::Third,
        number: Number::Singular,
    },
    VerbSlot::Past {
        person: Person::Third,
        number: Number::Plural,
    },
    VerbSlot::PresentParticiple,
    VerbSlot::PastParticiple,
];

pub(super) fn render_verb_form(lemma: &str, form: VerbForm, slot: VerbSlot) -> String {
    if let VerbForm::Irregular(irregular) = form {
        let override_form = match slot {
            VerbSlot::Infinitive | VerbSlot::Imperative => None,
            VerbSlot::Present {
                person: Person::Second,
                ..
            } => irregular.present_second,
            VerbSlot::Present {
                person: Person::Third,
                number: Number::Singular,
            } => irregular.present_third_singular,
            VerbSlot::Present {
                person: Person::Third,
                number: Number::Plural,
            } => irregular.present_third_plural,
            VerbSlot::Past {
                person: Person::Second,
                ..
            } => irregular.past_second,
            VerbSlot::Past {
                person: Person::Third,
                number: Number::Singular,
            } => irregular.past_third_singular,
            VerbSlot::Past {
                person: Person::Third,
                number: Number::Plural,
            } => irregular.past_third_plural,
            VerbSlot::PresentParticiple => irregular.present_participle,
            VerbSlot::PastParticiple => irregular.past_participle,
        };
        if let Some(surface) = override_form {
            return surface.to_owned();
        }
    }

    match slot {
        VerbSlot::Present {
            person: Person::Third,
            number: Number::Singular,
        } => regular_third_person_singular(lemma),
        VerbSlot::Infinitive | VerbSlot::Imperative | VerbSlot::Present { .. } => lemma.to_owned(),
        VerbSlot::Past { .. } | VerbSlot::PastParticiple => regular_past(lemma),
        VerbSlot::PresentParticiple => regular_present_participle(lemma),
    }
}

fn regular_third_person_singular(verb: &str) -> String {
    if let Some(stem) = consonant_y_stem(verb) {
        format!("{stem}ies")
    } else if has_sibilant_ending(verb) || verb.ends_with('o') {
        format!("{verb}es")
    } else {
        format!("{verb}s")
    }
}

fn regular_past(verb: &str) -> String {
    if let Some(stem) = consonant_y_stem(verb) {
        format!("{stem}ied")
    } else if verb.ends_with('e') {
        format!("{verb}d")
    } else {
        format!("{verb}ed")
    }
}

fn regular_present_participle(verb: &str) -> String {
    if let Some(stem) = verb.strip_suffix("ie") {
        format!("{stem}ying")
    } else if let Some(stem) = verb.strip_suffix('e')
        && !verb.ends_with("ee")
        && !verb.ends_with("ye")
        && !verb.ends_with("oe")
    {
        format!("{stem}ing")
    } else {
        format!("{verb}ing")
    }
}
