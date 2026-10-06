//! The term a keyword declaration defines, built from what its file writes.
//!
//! A keyword ability's definition is an `Ability.keyword` term
//! [CR#702.1] — `Keyword(keyword: <label>, params: <arguments>, body:
//! <abilities>)` — and a keyword action's is the deed it names done by the
//! actor [CR#701.1,109.5] — `Enact(verb: Action(<label>), instruction:
//! <instruction>, agent: Some(Actor))`. Every part of both wrappers but the
//! abilities and the instruction follows from the declaration's name and
//! parameter signature, so the file writes only those, and this module builds
//! the rest. A declaration names no performer: whoever performs the action is
//! the actor, and another player performs it only under a handoff
//! (`act(performer, …)`, ADR 7, rulings 2026-10-05). The actor is a player,
//! or, under a handoff to a permanent, that permanent ("target creature
//! explores" [CR#701.44a]); an action a permanent performs written with no
//! such handoff is performed by the source permanent (Lean `enactAgentOk`).
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
//! The labels are the two boundaries at which a declaration's name is read as
//! a label rather than as an identity: a keyword ability's is the name
//! capitalized (`firstStrike` → `FirstStrike`), a keyword action's the words
//! the camelCase name spells, each capitalized (`collectEvidence` → `Collect
//! Evidence`). `cargo xtask facts` reads both through here.

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

/// A keyword ability's label: its declaration name with the first letter
/// capitalized, so `flying` is `Flying` and `firstStrike` is `FirstStrike`.
#[must_use]
pub fn keyword_ability_label(name: &str) -> String {
    capitalized(name)
}

/// A keyword action's label [CR#701.1]: its camelCase declaration name read
/// as the words it spells, each capitalized, so `collectEvidence` is "Collect
/// Evidence" and `theRingTemptsYou` is "The Ring Tempts You".
#[must_use]
pub fn keyword_action_label(name: &str) -> String {
    let mut label = String::new();
    for (index, ch) in capitalized(name).chars().enumerate() {
        if index > 0 && ch.is_uppercase() {
            label.push(' ');
        }
        label.push(ch);
    }
    label
}

fn capitalized(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().chain(chars).collect(),
        None => String::new(),
    }
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
        KEYWORD_ACTION => keyword_action_body(name, body).map(Some),
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
        "Keyword(keyword: \"{}\", params: {keyword_params}, body: {abilities})",
        keyword_ability_label(name)
    ))
}

/// A keyword action's definition from its meta-macro's record: the
/// instruction wrapped as the deed the action names, done by the actor, unless
/// the declaration
/// has no instruction (`()`, the meta's omitted-body default) or names no
/// deed (`deed: None`).
///
/// # Errors
/// As [`definition_body`].
pub fn keyword_action_body(name: &str, body: &str) -> Result<String, String> {
    let record = Record::read(body, &["deed", "instruction"])?;
    let instruction = record.required("instruction")?;
    if instruction == "()" {
        return Ok(instruction.to_owned());
    }
    let deed = match record.get("deed") {
        Some("None") => return Ok(instruction.to_owned()),
        Some(deed) => deed.to_owned(),
        None => format!("Action(\"{}\")", keyword_action_label(name)),
    };
    Ok(format!(
        "Enact(verb: {deed}, instruction: {instruction}, agent: Some(Actor))"
    ))
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
    use super::keyword_ability_label;
    use super::keyword_action_body;
    use super::keyword_action_label;

    fn positional(types: &[&str]) -> Params {
        Params::Positional(types.iter().map(|ty| ParamType::plain(*ty)).collect())
    }

    #[test]
    fn labels_capitalize_the_name() {
        assert_eq!(keyword_ability_label("flying"), "Flying");
        assert_eq!(keyword_ability_label("firstStrike"), "FirstStrike");
        assert_eq!(keyword_action_label("destroy"), "Destroy");
        assert_eq!(keyword_action_label("collectEvidence"), "Collect Evidence");
        assert_eq!(
            keyword_action_label("faceAVillainousChoice"),
            "Face A Villainous Choice"
        );
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
            "Keyword(keyword: \"Ward\", params: [Number(Param(0)), Cost(Param(2))], body: [])"
        );
        assert_eq!(
            keyword_ability_body(
                "champion",
                &positional(&["Subject"]),
                "(keyword_params: [], abilities: [])"
            )
            .unwrap(),
            "Keyword(keyword: \"Champion\", params: [], body: [])"
        );
        let error =
            keyword_ability_body("companion", &positional(&["Condition"]), "(abilities: [])")
                .unwrap_err();
        assert!(error.contains("keyword_params"), "{error}");
    }

    #[test]
    fn a_keyword_action_enacts_its_deed() {
        assert_eq!(
            keyword_action_body("timeTravel", "(instruction: Shuffle(agent: Actor))").unwrap(),
            "Enact(verb: Action(\"Time Travel\"), instruction: Shuffle(agent: Actor), agent: \
             Some(Actor))"
        );
        assert_eq!(
            keyword_action_body(
                "shuffle",
                "(deed: None, instruction: Shuffle(agent: Actor))"
            )
            .unwrap(),
            "Shuffle(agent: Actor)"
        );
        assert_eq!(
            keyword_action_body("scry", "(instruction: ())").unwrap(),
            "()"
        );
        // `adapt` is performed by a permanent, and records the actor all the
        // same: the actor is that permanent under a handoff to it, and the
        // source permanent by default.
        assert_eq!(
            keyword_action_body("adapt", "(instruction: Shuffle(agent: Actor))").unwrap(),
            "Enact(verb: Action(\"Adapt\"), instruction: Shuffle(agent: Actor), agent: \
             Some(Actor))"
        );
        for agent in ["None", "You"] {
            let error = keyword_action_body(
                "adapt",
                &format!("(agent: {agent}, instruction: Shuffle(agent: Actor))"),
            )
            .unwrap_err();
            assert!(error.contains("agent"), "{error}");
        }
    }
}
