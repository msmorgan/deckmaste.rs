//! Review surface for generic sum/product and checked-sequence emission.

use deckmaste_construction_compiler::emit::emit_group;
use deckmaste_construction_compiler::parse::parse_group;
use deckmaste_construction_compiler::validate::validate;

fn formatted_emission() -> String {
    let group = parse_group(quote::quote! {
        group fixture_sum backend ability;

        element node bind Node {
            variant Unit {},
            variant Leaf: lex String,
            variant Pair(left: sum box node, right: sum box node),
            variant Named { label: lex String, child: sum box node },
        }
        element item { value: lex String, }
        element wrapped bind Wrapped {
            node: sum node,
            clause: hole box ConcreteClause via ClauseCategory,
            marked: lex bool,
            item: product item,
        }

        construction root: Root {
            own RootNode {
                node: sum node,
                wrapped: product wrapped,
                boxed_wrapped: product box wrapped,
                clause: hole ConcreteClause via ClauseCategory,
                boxed_clause: hole box ConcreteClause via ClauseCategory,
                nodes: nonempty seq node,
                items: nonempty seq item separated by lex Separator,
            }
            form unit @ 0 when node.variant in [Unit] =
                node wrapped boxed_wrapped clause boxed_clause nodes items;
            form other @ 1 otherwise =
                node wrapped boxed_wrapped clause boxed_clause nodes items;
            selection unique;
        }
    })
    .expect("sum fixture parses");
    let validated = validate(&group).expect("sum fixture validates");
    let file: syn::File = syn::parse2(emit_group(&validated)).expect("sum emission parses as Rust");
    prettyplease::unparse(&file)
}

#[test]
fn sum_and_checked_sequence_emission_matches_committed_golden() {
    assert!(
        !(std::env::var_os("CI").is_some() && std::env::var_os("UPDATE_GOLDENS").is_some()),
        "UPDATE_GOLDENS must never run in CI"
    );
    let formatted = formatted_emission();
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/goldens/fixture_sum_nonempty.golden"
    );
    if std::env::var_os("UPDATE_GOLDENS").is_some() {
        std::fs::write(path, &formatted).expect("write sum golden");
    }
    let golden = std::fs::read_to_string(path)
        .expect("golden missing: rerun with UPDATE_GOLDENS=1 and review the diff");
    assert_eq!(formatted, golden, "regenerate with UPDATE_GOLDENS=1");
}

#[test]
fn sum_emission_is_byte_stable() {
    assert_eq!(formatted_emission(), formatted_emission());
}
