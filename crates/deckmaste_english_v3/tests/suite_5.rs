// Register new tests/suite modules in exactly one binary root; see tests/README.
#[allow(dead_code, reason = "Each binary uses a subset of the shared test fixtures.")]
mod common;

#[path = "suite/as_clause_np_licensing.rs"]
mod as_clause_np_licensing;

#[path = "suite/as_though_content_clauses.rs"]
mod as_though_content_clauses;

#[path = "suite/clitic_have_object_relative.rs"]
mod clitic_have_object_relative;

#[path = "suite/comparative_quantity_than.rs"]
mod comparative_quantity_than;

#[path = "suite/counter_kind_nouns.rs"]
mod counter_kind_nouns;

#[path = "suite/degree_quantifiers.rs"]
mod degree_quantifiers;

#[path = "suite/except_matrix_complements.rs"]
mod except_matrix_complements;

#[path = "suite/keyword_dash_cost_line.rs"]
mod keyword_dash_cost_line;

#[path = "suite/keyword_payloads.rs"]
mod keyword_payloads;

#[path = "suite/keyword_verbs.rs"]
mod keyword_verbs;

#[path = "suite/mana.rs"]
mod mana;

#[path = "suite/measured_damage_nominal.rs"]
mod measured_damage_nominal;

#[path = "suite/noun_premodifiers.rs"]
mod noun_premodifiers;

#[path = "suite/number_transparency.rs"]
mod number_transparency;

#[path = "suite/participial_uses.rs"]
mod participial_uses;

#[path = "suite/perfect_relative_adverbial_gap.rs"]
mod perfect_relative_adverbial_gap;

#[path = "suite/pp_admission.rs"]
mod pp_admission;

#[path = "suite/produce_verb.rs"]
mod produce_verb;

#[path = "suite/proper_names.rs"]
mod proper_names;

#[path = "suite/quoted_ability_coordination.rs"]
mod quoted_ability_coordination;

#[path = "suite/trailing_adjunct_height.rs"]
mod trailing_adjunct_height;

#[path = "suite/preposed_frequency.rs"]
mod preposed_frequency;

#[path = "suite/keyword_mention_without_parameter.rs"]
mod keyword_mention_without_parameter;

#[path = "suite/ring_tempts_you.rs"]
mod ring_tempts_you;

#[path = "suite/keyword_state_adjectives.rs"]
mod keyword_state_adjectives;
