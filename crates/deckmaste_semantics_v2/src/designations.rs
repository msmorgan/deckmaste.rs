//! The definitions a designation declaration defines, built from what its
//! meta-macro produced.
//!
//! `macros/meta/Designation.ron` builds the `Designation` definition from the
//! declaration's name and columns, so the label is the declaration's own name
//! (`goaded`) and the author never writes it [CR#701.15b]. It hands the
//! definition over in a record beside the file's `members`:
//! `(members: [...], definition: Designation(label: Named(name: "<name>"),
//! ...))`. With no members the declaration defines that one designation. A
//! declaration whose spelling covers several designations lists them
//! (`dayNight` lists `day` and `night` [CR#731.1]): it then defines one
//! designation per member, each labelled by the member's name with the
//! declaration's columns, and each member is a name of its own that reads
//! bare where a designation is written ([`member_macros`]). The reader
//! (`reader.rs`) and `cargo xtask facts` both read declarations through here.

use macro_ron::Ident;
use macro_ron::MacroDef;

use crate::keywords::Record;

/// The family kind of a designation declaration.
pub const DESIGNATION: &str = "Designation";

/// The kind a designation's name reads at: the `DesignationLabel` position,
/// where the name denotes the label its definition names.
pub const DESIGNATION_LABEL: &str = "DesignationLabel";

/// Every designation the declaration `name` defines, as `(label, definition)`
/// pairs in the order the declaration writes them: the declaration's own name
/// when it lists no members, else one pair per member.
///
/// # Errors
/// If the produced body is not the record the meta-macro writes, the built
/// definition does not carry the declaration's name exactly once, or a member
/// is not a bare name.
pub fn designation_definitions(name: &str, body: &str) -> Result<Vec<(String, String)>, String> {
    let record = Record::read(body, &["members", "definition"])?;
    let definition = record.required("definition")?;
    let members = members(record.required("members")?)?;
    if members.is_empty() {
        return Ok(vec![(name.to_owned(), definition.to_owned())]);
    }
    let own = format!("Named(name: \"{name}\")");
    if definition.matches(&own).count() != 1 {
        return Err(format!(
            "designation `{name}`: the built definition does not carry `{own}` once: {definition}"
        ));
    }
    Ok(members
        .into_iter()
        .map(|member| {
            let relabelled = definition.replace(&own, &format!("Named(name: \"{member}\")"));
            (member, relabelled)
        })
        .collect())
}

/// The definition the declaration `name` registers: its one designation's
/// node, or, where it lists members, the list of theirs.
///
/// # Errors
/// As [`designation_definitions`].
pub fn designation_body(name: &str, body: &str) -> Result<String, String> {
    let definitions = designation_definitions(name, body)?;
    match definitions.as_slice() {
        [(label, definition)] if label == name => Ok(definition.clone()),
        _ => Ok(format!(
            "[{}]",
            definitions
                .iter()
                .map(|(_, definition)| definition.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        )),
    }
}

/// The macros a designation declaration's members register: each member's
/// name, at the `DesignationLabel` position only, whose body is the member's
/// definition. `declaration` is the declaration as registered (its body
/// already built by [`designation_body`]), `produced` the body its meta-macro
/// produced. Empty for a declaration with no members.
///
/// # Errors
/// As [`designation_definitions`].
pub fn member_macros(declaration: &MacroDef, produced: &str) -> Result<Vec<MacroDef>, String> {
    let name = declaration.name.as_str();
    let definitions = designation_definitions(name, produced)?;
    if matches!(definitions.as_slice(), [(label, _)] if label == name) {
        return Ok(Vec::new());
    }
    Ok(definitions
        .into_iter()
        .map(|(member, definition)| {
            let mut member_macro = declaration.clone().with_body(&definition);
            member_macro.name = Ident::from(member.as_str());
            member_macro.kinds = vec![Ident::from(DESIGNATION_LABEL)];
            member_macro
        })
        .collect())
}

/// The bare names a `members` list writes.
fn members(written: &str) -> Result<Vec<String>, String> {
    let inner = written
        .strip_prefix('[')
        .and_then(|rest| rest.strip_suffix(']'))
        .ok_or_else(|| format!("`members` is a list of names, not `{written}`"))?;
    inner
        .split(',')
        .map(str::trim)
        .filter(|member| !member.is_empty())
        .map(|member| {
            let bare = member.starts_with(|c: char| c.is_ascii_lowercase())
                && member.chars().all(|c| c.is_ascii_alphanumeric());
            if bare {
                Ok(member.to_owned())
            } else {
                Err(format!(
                    "a designation member is a bare name, not `{member}`"
                ))
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::designation_body;
    use super::designation_definitions;

    const GOADED: &str = r#"(members: [], definition: Designation(label: Named(name: "goaded"), scope: HeldBy(Object), effectful: true, zone: Battlefield, type: None, half: None))"#;
    const DAY_NIGHT: &str = r#"(members: [day, night], definition: Designation(label: Named(name: "dayNight"), scope: HeldByGame, effectful: true, zone: None, type: None, half: None))"#;

    #[test]
    fn a_declaration_without_members_defines_its_own_name() {
        assert_eq!(
            designation_body("goaded", GOADED).unwrap(),
            r#"Designation(label: Named(name: "goaded"), scope: HeldBy(Object), effectful: true, zone: Battlefield, type: None, half: None)"#
        );
    }

    #[test]
    fn a_declaration_with_members_defines_one_designation_per_member() {
        let definitions = designation_definitions("dayNight", DAY_NIGHT).unwrap();
        let labels: Vec<&str> = definitions
            .iter()
            .map(|(label, _)| label.as_str())
            .collect();
        assert_eq!(labels, ["day", "night"]);
        assert!(
            definitions[1]
                .1
                .starts_with(r#"Designation(label: Named(name: "night"), scope: HeldByGame"#)
        );
        assert!(
            designation_body("dayNight", DAY_NIGHT)
                .unwrap()
                .starts_with('[')
        );
    }

    #[test]
    fn a_member_is_a_bare_name() {
        let quoted = DAY_NIGHT.replace("[day, night]", r#"["day"]"#);
        assert!(designation_definitions("dayNight", &quoted).is_err());
    }
}
