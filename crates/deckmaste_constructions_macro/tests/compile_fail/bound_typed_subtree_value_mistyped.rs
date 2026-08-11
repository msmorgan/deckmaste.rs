pub struct StoredClause;
pub struct DeclaredClause;

pub struct BoundRecord {
    pub clause: StoredClause,
}

deckmaste_constructions_macro::constructions! {
    group typed_subtree_mistyped;
    element record bind BoundRecord {
        clause: hole DeclaredClause via Clause,
    }
}

fn main() {}
