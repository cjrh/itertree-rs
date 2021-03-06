use rstest::*;
use itertree::TraversalOrder;

#[fixture]
fn tree01() -> Vec<(i32, i32, i32, u32)> {
    [
        (1, 2, 3, 1),
        (2, 4, 5, 2),
        (3, 6, -1, 3),
        (4, 7, -1, 4),
        (5, -1, -1, 5),
        (6, 8, 9, 6),
        (7, -1, -1, 7),
        (8, -1, -1, 8),
        (9, -1, -1, 9)
    ].into()
}


#[test]
fn test_example() {
    let tree_definition = [
        (1, 2, 3, 1),
        (2, 4, 5, 2),
        (3, 6, -1, 3),
        (4, 7, -1, 4),
        (5, -1, -1, 5),
        (6, 8, 9, 6),
        (7, -1, -1, 7),
        (8, -1, -1, 8),
        (9, -1, -1, 9)
    ];
    let tree = itertree::Node::new(&tree_definition);
    let order = itertree::TraversalOrder::PreOrder;
    let visited: Vec<_> = tree.iter(order).map(|n| *n.contents).collect();
    println!("{:?}", &visited);
    // [1, 2, 4, 7, 5, 3, 6, 8, 9]

    let order = itertree::TraversalOrder::InOrder;
    let visited: Vec<_> = tree.iter(order).map(|n| *n.contents).collect();
    println!("{:?}", &visited);
    // [7, 4, 2, 5, 1, 8, 6, 9, 3]

    let order = itertree::TraversalOrder::PostOrder;
    let visited: Vec<_> = tree.iter(order).map(|n| *n.contents).collect();
    println!("{:?}", &visited);
    // [7, 4, 5, 2, 8, 9, 6, 3, 1]
}

#[rstest]
fn test_basic(tree01: Vec<(i32, i32, i32, u32)>) {
    let tree = itertree::Node::new(&tree01);
    println!("{:?}", &tree);
}

#[rstest]
#[case(TraversalOrder::PreOrder, &vec![1, 2, 4, 7, 5, 3, 6, 8, 9])]
#[case(TraversalOrder::InOrder, &vec![7, 4, 2, 5, 1, 8, 6, 9, 3])]
#[case(TraversalOrder::PostOrder, &vec![7, 4, 5, 2, 8, 9, 6, 3, 1])]
fn test_order(tree01: Vec<(i32, i32, i32, u32)>, #[case] order: TraversalOrder, #[case] expected: &[u32]) {
    let tree = itertree::Node::new(&tree01);
    let results: Vec<_> = tree.iter(order).map(|n| *n.contents).collect();
    assert_eq!(results, expected);
}
