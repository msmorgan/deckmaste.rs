//! The term a keyword declaration defines, built from what its file writes.
//!
//! A keyword ability's definition is an `Ability.keyword` term
//! [CR#702.1] — `Keyword(keyword: <label>, params: <arguments>, body:
//! <abilities>)` — and a keyword action's is the deed it names done by the
//! actor [CR#701.1,109.5] — `Enact(verb: Action(<name>), instruction:
//! <instruction>)`, the performer being the actor in context. Every part of
//! both wrappers but the
//! abilities and the instruction follows from the declaration's name and
//! parameter signature, so the file writes only those, and this module builds
//! the rest. A declaration names no performer: whoever performs the action is
//! the actor, and another player performs it only under a handoff
//! (`act(performer, …)`, ADR 7, rulings 2026-10-05). The actor is a player,
//! or, under a handoff to a permanent, that permanent ("target creature
//! explores" [CR#701.44a]); an action a permanent performs written with no
//! such handoff is performed by the source permanent (Lean `enactPerformer`,
//! `enactAgentOk`).
//! The meta-macros (`macros/meta/KeywordAbility.ron`,
//! `KeywordAction.ron`) hand over what the file wrote as a record in the
//! body position:
//!
//! - a keyword ability's `(keyword_params: …, abilities: …)`, where
//!   `keyword_params` is present only when the file overrides the arguments the
//!   signature would forward;
//! - a keyword action's `(deed: …, instruction: …)`, where `deed` is present
//!   only when the file names one: `None` for an action whose definition is not
//!   a single named deed, written unwrapped.
//!
//! Both are named by the declaration's name itself (`firstStrike`,
//! `collectEvidence`), which is also how a card names them: [`label_macro`]
//! registers a keyword ability's name at the `KeywordLabel` position and
//! [`deed_macro`] a keyword action's at the `Deed` position
//! (`docs/decisions/semantics-v2.md` §7, ruling 2026-10-06, "Deeds and
//! keyword labels are declared names").

use std::collections::BTreeMap;
use std::fmt;

use ::ron::value::RawValue;
use macro_ron::Params;
use serde::Deserializer;
use serde::de::DeserializeSeed;
use serde::de::MapAccess;
use serde::de::Visitor;

/// The family kind of a keyword-ability declaration.
pub const KEYWORD_ABILITY: &str = "KeywordAbility";
/// The family kind of a keyword-action declaration.
pub const KEYWORD_ACTION: &str = "KeywordAction";

/// The `KeywordLabel` position a keyword ability's declaration name reads at.
pub const KEYWORD_LABEL: &str = "KeywordLabel";

/// The macro that makes a keyword ability's declaration name read bare where a
/// keyword label is wanted [CR#702.1]: `grants(flying)`, `keyword(haste)`. It
/// takes no arguments and denotes `Named(name: <name>)`, the label the
/// declaration's own `Keyword` term carries; [`deed_macro`]'s twin.
#[must_use]
pub fn label_macro(declaration: &macro_ron::MacroDef) -> macro_ron::MacroDef {
    name_macro(declaration, KEYWORD_LABEL, "Named(name: ")
}

fn name_macro(
    declaration: &macro_ron::MacroDef,
    position: &str,
    constructor: &str,
) -> macro_ron::MacroDef {
    let mut named = declaration
        .clone()
        .with_body(&format!("{constructor}\"{}\")", declaration.name.as_str()));
    named.kinds = vec![macro_ron::Ident::from(position)];
    named.params = Params::Positional(Vec::new());
    named
}

/// The `Deed` position a keyword action's declaration name reads at.
pub const DEED: &str = "Deed";

/// The macro that makes a keyword action's declaration name read bare where a
/// deed is wanted [CR#701.1]: `theVerbed(exile, Many)`, `enact(discard, …)`.
/// It takes no arguments and denotes `Action(label: <name>)`, the deed the
/// declaration's own `Enact` wrapper names. A name no keyword action declares
/// has no such macro, so a card writing it is refused when it loads; the
/// `Action` constructor itself is not author vocabulary at a restricted read
/// (`crate::ron`), so the name is the only way a card writes the deed.
#[must_use]
pub fn deed_macro(declaration: &macro_ron::MacroDef) -> macro_ron::MacroDef {
    name_macro(declaration, DEED, "Action(label: ")
}

/// The definition a declaration of family `family` registers, given the body
/// its meta-macro produced: `Some` for the two keyword families, whose
/// declaration builds the wrapper, and for the designation family, whose
/// meta hands over its definition beside its members
/// ([`crate::designations`]); `None` for every other family, whose body is
/// the definition as written.
///
/// # Errors
/// If the produced body is not the record its meta-macro writes, or a keyword
/// ability's signature names a parameter type that has no keyword argument
/// and the file does not override `keyword_params`.
pub fn definition_body(
    family: &str,
    name: &str,
    params: &Params,
    body: &str,
) -> Result<Option<String>, String> {
    match family {
        KEYWORD_ABILITY => keyword_ability_body(name, params, body).map(Some),
        KEYWORD_ACTION => keyword_action_body(name, params, body).map(Some),
        crate::designations::DESIGNATION => {
            crate::designations::designation_body(name, body).map(Some)
        }
        _ => Ok(None),
    }
}

/// A keyword ability's `Keyword(...)` term from its meta-macro's record.
///
/// # Errors
/// As [`definition_body`].
pub fn keyword_ability_body(name: &str, params: &Params, body: &str) -> Result<String, String> {
    let record = Record::read(body, &["keyword_params", "abilities"])?;
    let abilities = record.required("abilities")?;
    let keyword_params = match record.get("keyword_params") {
        Some(written) => written.to_owned(),
        None => forwarded_keyword_params(name, params)?,
    };
    Ok(format!(
        "Keyword(keyword: Named(name: \"{name}\"), params: {keyword_params}, body: {abilities})"
    ))
}

/// A keyword action's definition from its meta-macro's record: the
/// instruction wrapped as the deed the action names, done by the actor, unless
/// the declaration
/// has no instruction (`()`, the meta's omitted-body default) or names no
/// deed (`deed: None`).
///
/// Inside the deed the body is read in its own Reference Scope
/// (`OwnScope`, `docs/decisions/semantics-v2.md` §7, ruling 2026-10-06): it
/// sees the performer, its parameters and what it introduces, never the
/// calling text's other mentions. Each parameter hole is read in the caller's
/// view (`inCaller`), so an argument such as "it" or "that creature's power"
/// still reads the text that wrote it. The scope opens here, at the keyword
/// action boundary, never at a handoff.
///
/// # Errors
/// As [`definition_body`].
pub fn keyword_action_body(name: &str, params: &Params, body: &str) -> Result<String, String> {
    let record = Record::read(body, &["deed", "instruction"])?;
    let instruction = record.required("instruction")?;
    if instruction == "()" {
        return Ok(instruction.to_owned());
    }
    let deed = match record.get("deed") {
        Some("None") => return Ok(instruction.to_owned()),
        Some(deed) => deed.to_owned(),
        None => format!("Action(\"{name}\")"),
    };
    Ok(format!(
        "Enact(verb: {deed}, instruction: OwnScope(scope: {OWN_SCOPE}, body: {}))",
        in_caller(params, instruction)
    ))
}

/// The scope key the loader writes for every keyword action body's own scope
/// and its parameters' caller views. One key serves every nesting: a caller view
/// masks the frame it opens from, so a parameter forwarded into a nested body
/// reads through to the next scope out.
pub const OWN_SCOPE: u32 = 0;

/// The parameter types whose arguments can refer back into the calling text,
/// and so are read in the caller's view: each has an `InCaller` constructor.
const CALLER_VIEW_TYPES: &[&str] = &[
    "Subject",
    "NounPhrase",
    "Amount",
    "Power",
    "Toughness",
    "Quality",
    "Predicate",
    "Quantity",
    "Condition",
    "ZoneExpr",
    "Instruction",
    "Cost",
    "StaticSpec",
    "GameEvent",
];

/// `instruction` with each hole of a caller-view parameter wrapped
/// `inCaller(OWN_SCOPE, Param(p))`.
fn in_caller(params: &Params, instruction: &str) -> String {
    let viewed: Vec<String> = match params {
        Params::Positional(types) => types
            .iter()
            .enumerate()
            .filter(|(_, ty)| CALLER_VIEW_TYPES.contains(&ty.name.as_str()))
            .map(|(index, _)| index.to_string())
            .collect(),
        Params::Named(named) => named
            .iter()
            .filter(|(_, ty)| CALLER_VIEW_TYPES.contains(&ty.name.as_str()))
            .map(|(param, _)| param.as_str().to_owned())
            .collect(),
    };
    let mut out = String::with_capacity(instruction.len());
    let mut cursor = 0;
    for hole in crate::mentions::param_holes(instruction) {
        if !viewed.contains(&hole.param) {
            continue;
        }
        out.push_str(&instruction[cursor..hole.start]);
        out.push_str("inCaller(");
        out.push_str(&OWN_SCOPE.to_string());
        out.push_str(", ");
        out.push_str(&instruction[hole.start..hole.end]);
        out.push(')');
        cursor = hole.end;
    }
    out.push_str(&instruction[cursor..]);
    out
}

/// The keyword arguments a keyword ability's signature forwards, one per
/// declared parameter in order, each the `KeywordParam` that carries its
/// type: `Cost` → `Cost`, `Amount` → `Number`, `Quality` → `Quality`,
/// `Subject` → `Subject`. An `Ability` parameter is forwarded as nothing —
/// `KeywordParam` has no `Ability` variant — and any other type is an error
/// the declaration answers by writing `keyword_params` itself.
fn forwarded_keyword_params(name: &str, params: &Params) -> Result<String, String> {
    let types: Vec<&str> = match params {
        Params::Positional(types) => types.iter().map(|ty| ty.name.as_str()).collect(),
        Params::Named(_) => {
            return Err(format!(
                "keyword ability `{name}` names its parameters; a keyword's arguments are \
                 positional"
            ));
        }
    };
    let mut forwarded = Vec::new();
    for (index, ty) in types.iter().enumerate() {
        let carrier = match *ty {
            "Cost" => "Cost",
            "Amount" => "Number",
            "Quality" => "Quality",
            "Subject" => "Subject",
            "Ability" => continue,
            other => {
                return Err(format!(
                    "keyword ability `{name}`: a `{other}` parameter has no keyword argument to \
                     forward it as; write `keyword_params` explicitly"
                ));
            }
        };
        forwarded.push(format!("{carrier}(Param({index}))"));
    }
    Ok(format!("[{}]", forwarded.join(", ")))
}

/// A meta-macro's record: its fields' raw texts, by name. A field is absent
/// when the meta's elidable parameter was omitted.
pub(crate) struct Record(BTreeMap<String, String>);

impl Record {
    pub(crate) fn read(body: &str, fields: &'static [&'static str]) -> Result<Self, String> {
        let options = crate::ron::raw_options();
        let mut deserializer = ::ron::Deserializer::from_str_with_options(body, &options)
            .map_err(|error| error.to_string())?;
        let record = RecordSeed { fields }
            .deserialize(&mut deserializer)
            .map_err(|error| deserializer.span_error(error).to_string())?;
        deserializer.end().map_err(|error| error.to_string())?;
        Ok(record)
    }

    fn get(&self, field: &str) -> Option<&str> {
        self.0.get(field).map(String::as_str)
    }

    pub(crate) fn required(&self, field: &str) -> Result<&str, String> {
        self.get(field)
            .ok_or_else(|| format!("the declaration record has no `{field}`"))
    }
}

struct RecordSeed {
    fields: &'static [&'static str],
}

impl<'de> DeserializeSeed<'de> for RecordSeed {
    type Value = Record;

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Record, D::Error> {
        deserializer.deserialize_struct("", self.fields, self)
    }
}

impl<'de> Visitor<'de> for RecordSeed {
    type Value = Record;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a declaration record with fields {:?}", self.fields)
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Record, A::Error> {
        use serde::de::Error;
        let mut out = BTreeMap::new();
        while let Some(key) = map.next_key::<String>()? {
            if !self.fields.contains(&key.as_str()) {
                return Err(A::Error::unknown_field(&key, self.fields));
            }
            let value: Box<RawValue> = map.next_value()?;
            if out
                .insert(key.clone(), value.get_ron().trim().to_owned())
                .is_some()
            {
                return Err(A::Error::duplicate_field(
                    self.fields
                        .iter()
                        .find(|field| **field == key)
                        .expect("the key was checked against the fields"),
                ));
            }
        }
        Ok(Record(out))
    }
}

#[cfg(test)]
mod tests {
    use macro_ron::ParamType;
    use macro_ron::Params;

    use super::keyword_ability_body;
    use super::keyword_action_body;

    fn positional(types: &[&str]) -> Params {
        Params::Positional(types.iter().map(|ty| ParamType::plain(*ty)).collect())
    }

    /// A keyword ability's label is its declaration name, verbatim: the
    /// capitalized label (`FirstStrike`) is retired (ruling 2026-10-06).
    #[test]
    fn a_keyword_abilitys_label_is_its_declaration_name() {
        for name in ["flying", "firstStrike"] {
            assert_eq!(
                keyword_ability_body(name, &positional(&[]), "(abilities: [])").unwrap(),
                format!("Keyword(keyword: Named(name: \"{name}\"), params: [], body: [])")
            );
        }
    }

    /// A keyword action's deed is its declaration name, verbatim: the label
    /// the camelCase name used to be read as ("Collect Evidence") is retired
    /// (ruling 2026-10-06).
    #[test]
    fn a_keyword_actions_deed_is_its_declaration_name() {
        for name in ["destroy", "collectEvidence", "faceAVillainousChoice"] {
            assert_eq!(
                keyword_action_body(name, &positional(&[]), "(instruction: Shuffle)").unwrap(),
                format!(
                    "Enact(verb: Action(\"{name}\"), instruction: OwnScope(scope: 0, body: Shuffle))"
                )
            );
        }
    }

    #[test]
    fn a_keyword_ability_forwards_its_signature() {
        assert_eq!(
            keyword_ability_body(
                "ward",
                &positional(&["Amount", "Ability", "Cost"]),
                "(abilities: [])"
            )
            .unwrap(),
            "Keyword(keyword: Named(name: \"ward\"), params: [Number(Param(0)), Cost(Param(2))], body: [])"
        );
        assert_eq!(
            keyword_ability_body(
                "champion",
                &positional(&["Subject"]),
                "(keyword_params: [], abilities: [])"
            )
            .unwrap(),
            "Keyword(keyword: Named(name: \"champion\"), params: [], body: [])"
        );
        let error =
            keyword_ability_body("companion", &positional(&["Condition"]), "(abilities: [])")
                .unwrap_err();
        assert!(error.contains("keyword_params"), "{error}");
    }

    #[test]
    fn a_keyword_action_enacts_its_deed() {
        assert_eq!(
            keyword_action_body("timeTravel", &positional(&[]), "(instruction: Shuffle)").unwrap(),
            "Enact(verb: Action(\"timeTravel\"), instruction: OwnScope(scope: 0, body: Shuffle))"
        );
        assert_eq!(
            keyword_action_body(
                "shuffle",
                &positional(&[]),
                "(deed: None, instruction: Shuffle)"
            )
            .unwrap(),
            "Shuffle"
        );
        assert_eq!(
            keyword_action_body("scry", &positional(&[]), "(instruction: ())").unwrap(),
            "()"
        );
        // `adapt` is performed by a permanent, and its wrapper names no
        // performer all the same: the actor is that permanent under a handoff
        // to it, and the source permanent by default.
        assert_eq!(
            keyword_action_body(
                "adapt",
                &positional(&["Amount"]),
                "(instruction: Grow(Param(0)))"
            )
            .unwrap(),
            "Enact(verb: Action(\"adapt\"), instruction: OwnScope(scope: 0, body: Grow(inCaller(0, Param(0)))))"
        );
        for agent in ["None", "You"] {
            let error = keyword_action_body(
                "adapt",
                &positional(&[]),
                &format!("(agent: {agent}, instruction: Shuffle)"),
            )
            .unwrap_err();
            assert!(error.contains("agent"), "{error}");
        }
    }
}
