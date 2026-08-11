use deckmaste_construction_compiler::runtime::FieldKindData;
use deckmaste_construction_compiler::runtime::LinearizationVisitor;
use deckmaste_construction_compiler::runtime::NonEmpty;
use deckmaste_construction_compiler::runtime::Separated;
use deckmaste_construction_compiler::runtime::SeparatedNonEmpty;

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
enum Node {
    Unit,
    Leaf(String),
    Pair(Box<Node>, Box<Node>),
    Named { label: String, child: Box<Node> },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
enum Separator {
    Comma,
    Semicolon,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
struct ConcreteClause(String);

#[derive(Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
struct Wrapped {
    node: Node,
    clause: Box<ConcreteClause>,
    marked: bool,
    item: Item,
}

deckmaste_constructions_macro::constructions! {
    group sum_nonempty;

    element node bind Node {
        variant Unit {},
        variant Leaf: lex String,
        variant Pair(left: sum box node, right: sum box node),
        variant Named { label: lex String, child: sum box node },
    }
    element item {
        value: lex String,
    }
    element wrapped bind Wrapped {
        node: sum node,
        clause: hole box ConcreteClause via ClauseCategory,
        marked: lex bool,
        item: product item,
    }

    construction bundle: BundleCategory {
        own Bundle {
            root: sum node,
            wrapped: product wrapped,
            boxed_wrapped: product box wrapped,
            clause: hole ConcreteClause via ClauseCategory,
            boxed_clause: hole box ConcreteClause via ClauseCategory,
            nodes: nonempty seq node,
            items: nonempty seq item separated by lex Separator,
        }
        form unit @ 0 when root.variant in [Unit] =
            root wrapped boxed_wrapped clause boxed_clause nodes items;
        form other @ 1 otherwise =
            root wrapped boxed_wrapped clause boxed_clause nodes items;
        selection unique;
        serialize;
        deserialize;
    }
}

#[derive(Default)]
struct Events(Vec<String>);

impl LinearizationVisitor for Events {
    type Error = std::convert::Infallible;

    fn literal(&mut self, literal: &'static str) -> Result<(), Self::Error> {
        self.0.push(format!("literal:{literal}"));
        Ok(())
    }

    fn subtree<T: std::any::Any>(
        &mut self,
        category: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error> {
        self.0.push(format!("subtree:{category}"));
        Ok(())
    }

    fn scalar<T: std::any::Any>(
        &mut self,
        codec: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error> {
        self.0.push(format!("scalar:{codec}"));
        Ok(())
    }

    fn begin_sum_variant(
        &mut self,
        role: &'static str,
        element: &'static str,
        variant: &'static str,
    ) -> Result<(), Self::Error> {
        self.0.push(format!("sum:{role}:{element}:{variant}"));
        Ok(())
    }

    fn begin_product(
        &mut self,
        role: &'static str,
        element: &'static str,
    ) -> Result<(), Self::Error> {
        self.0.push(format!("product:{role}:{element}"));
        Ok(())
    }

    fn begin_sequence(&mut self, role: &'static str, len: usize) -> Result<(), Self::Error> {
        self.0.push(format!("sequence:{role}:{len}"));
        Ok(())
    }
}

fn leaf(value: &str) -> Node {
    build_node_leaf(value.to_owned())
}

#[test]
fn typed_sum_views_builders_and_selection_are_structural() {
    let root = build_node_pair(
        Box::new(build_node_unit()),
        Box::new(build_node_named(
            "outer".to_owned(),
            Box::new(leaf("inner")),
        )),
    );
    let NodeVariantRef::Pair { left, right } = parts_node(&root) else {
        panic!("the typed view must preserve the product variant")
    };
    assert!(matches!(left.as_ref(), Node::Unit));
    assert!(matches!(right.as_ref(), Node::Named { .. }));

    let nodes = NonEmpty::try_from(vec![leaf("member")]).unwrap();
    let items = SeparatedNonEmpty::new(
        Item {
            value: "first".to_owned(),
        },
        vec![Separated::new(
            Separator::Semicolon,
            Item {
                value: "second".to_owned(),
            },
        )],
    );
    let wrapped = Wrapped {
        node: leaf("direct-product"),
        clause: Box::new(ConcreteClause("direct nested clause".to_owned())),
        marked: true,
        item: Item {
            value: "nested owned product".to_owned(),
        },
    };
    let boxed_wrapped = Box::new(Wrapped {
        node: build_node_unit(),
        clause: Box::new(ConcreteClause("boxed nested clause".to_owned())),
        marked: false,
        item: Item {
            value: "boxed nested owned product".to_owned(),
        },
    });
    let bundle = Bundle::try_new(
        root,
        wrapped,
        boxed_wrapped,
        ConcreteClause("direct clause".to_owned()),
        Box::new(ConcreteClause("boxed clause".to_owned())),
        nodes,
        items,
    )
    .unwrap();
    let selected = selected_bundle_form(&bundle).unwrap();
    assert_eq!((selected.name, selected.ordinal), ("other", 1));

    let mut events = Events::default();
    linearize_bundle_with(&bundle, &mut events).unwrap();
    assert!(events.0.contains(&"sum:root:node:Pair".to_owned()));
    assert!(events.0.contains(&"sum:left:node:Unit".to_owned()));
    assert!(events.0.contains(&"sum:right:node:Named".to_owned()));
    assert!(events.0.contains(&"sum:child:node:Leaf".to_owned()));
    assert!(events.0.contains(&"product:wrapped:wrapped".to_owned()));
    assert!(
        events
            .0
            .contains(&"product:boxed_wrapped:wrapped".to_owned())
    );
    assert!(events.0.contains(&"sum:node:node:Leaf".to_owned()));
    assert!(events.0.contains(&"subtree:ClauseCategory".to_owned()));
    assert!(events.0.contains(&"product:item:item".to_owned()));
    assert!(events.0.contains(&"sequence:nodes:1".to_owned()));
    assert!(events.0.contains(&"sequence:items:2".to_owned()));
}

#[test]
fn checked_sequence_metadata_builders_reject_empty_and_preserve_separators() {
    let pair_builder = SUM_NONEMPTY_DECLARATION.element_data[0].erased_builders[2];
    let pair = pair_builder(vec![
        Box::new(Box::new(build_node_unit())),
        Box::new(Box::new(leaf("right"))),
    ])
    .unwrap()
    .downcast::<Node>()
    .unwrap();
    assert!(matches!(*pair, Node::Pair(_, right) if matches!(*right, Node::Leaf(_))));

    let wrapped_builder = SUM_NONEMPTY_DECLARATION.element_data[2].erased_builders[0];
    let wrapped = wrapped_builder(vec![
        Box::new(leaf("wrapped")),
        Box::new(Box::new(ConcreteClause("clause".to_owned()))),
        Box::new(true),
        Box::new(Item {
            value: "item".to_owned(),
        }),
    ])
    .unwrap()
    .downcast::<Wrapped>()
    .unwrap();
    assert!(wrapped.marked);
    assert_eq!(wrapped.clause.0, "clause");

    let fields = SUM_NONEMPTY_DECLARATION.constructions[0].fields;
    let FieldKindData::Product {
        element: "wrapped",
        boxed: false,
    } = fields[1].kind
    else {
        panic!("wrapped must retain its structural product metadata")
    };
    let FieldKindData::Product {
        element: "wrapped",
        boxed: true,
    } = fields[2].kind
    else {
        panic!("boxed_wrapped must retain its boxed product metadata")
    };
    let FieldKindData::TypedSubtree {
        value_type: "ConcreteClause",
        category: "ClauseCategory",
        boxed: false,
    } = fields[3].kind
    else {
        panic!("clause must retain both its Rust type and grammar category")
    };
    let FieldKindData::Product {
        element: "item",
        boxed: false,
    } = SUM_NONEMPTY_DECLARATION.element_data[2].fields[3].kind
    else {
        panic!("the nested item must retain its owned product metadata")
    };
    let FieldKindData::NonEmptySequence { erased_builder, .. } = fields[5].kind else {
        panic!("nodes must carry its checked erased builder")
    };
    assert!(matches!(
        erased_builder(Vec::new()),
        Err(
            deckmaste_construction_compiler::runtime::ErasedBuildError::EmptySequence {
                owner: "bundle",
                field: "nodes",
            }
        )
    ));
    let nodes = erased_builder(vec![Box::new(leaf("one"))])
        .unwrap()
        .downcast::<NonEmpty<Node>>()
        .unwrap();
    assert_eq!(nodes.len(), 1);

    let FieldKindData::SeparatedNonEmptySequence { erased_builder, .. } = fields[6].kind else {
        panic!("items must carry its checked separated builder")
    };
    assert!(matches!(
        erased_builder(Vec::new()),
        Err(
            deckmaste_construction_compiler::runtime::ErasedBuildError::EmptySequence {
                owner: "bundle",
                field: "items",
            }
        )
    ));
    assert!(matches!(
        erased_builder(vec![
            Box::new(Item {
                value: "one".to_owned(),
            }),
            Box::new(Separator::Comma),
        ]),
        Err(
            deckmaste_construction_compiler::runtime::ErasedBuildError::MissingField {
                owner: "bundle",
                field: "items",
            }
        )
    ));
    let items = erased_builder(vec![
        Box::new(Item {
            value: "one".to_owned(),
        }),
        Box::new(Separator::Comma),
        Box::new(Item {
            value: "two".to_owned(),
        }),
    ])
    .unwrap()
    .downcast::<SeparatedNonEmpty<Item, Separator>>()
    .unwrap();
    assert_eq!(items.len(), 2);
    assert_eq!(items.rest()[0].separator(), &Separator::Comma);
    assert_eq!(items.iter().len(), 2);
}
