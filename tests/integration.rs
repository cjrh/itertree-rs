use rstest::*;
use itertree::TraversalOrder;

#[fixture]
fn tree01() -> Vec<[i32; 3]> {
    [[1, 2, 3],
    [2, 4, 5],
    [3, 6, -1],
    [4, 7, -1],
    [5, -1, -1],
    [6, 8, 9],
    [7, -1, -1],
    [8, -1, -1],
    [9, -1, -1]].into()
}


#[test]
fn test_example() {
    let tree_definition = [[1, 2, 3],
        [2, 4, 5],
        [3, 6, -1],
        [4, 7, -1],
        [5, -1, -1],
        [6, 8, 9],
        [7, -1, -1],
        [8, -1, -1],
        [9, -1, -1]];
    let tree = itertree::Node::new(&tree_definition);
    let order = itertree::TraversalOrder::PreOrder;
    let visited: Vec<u32> = tree.iter(order).map(|n| n.contents).collect();
    println!("{:?}", &visited);
    // [1, 2, 4, 7, 5, 3, 6, 8, 9]
}

#[rstest]
fn test_basic(tree01: Vec<[i32; 3]>) {
    let tree = itertree::Node::new(&tree01);
    println!("{:?}", &tree);
}

#[rstest]
#[case(TraversalOrder::PreOrder, &vec![1, 2, 4, 7, 5, 3, 6, 8, 9])]
#[case(TraversalOrder::InOrder, &vec![7, 4, 2, 5, 1, 8, 6, 9, 3])]
#[case(TraversalOrder::PostOrder, &vec![7, 4, 5, 2, 8, 9, 6, 3, 1])]
fn test_order(tree01: Vec<[i32; 3]>, #[case] order: TraversalOrder, #[case] expected: &[u32]) {
    let tree = itertree::Node::new(&tree01);
    let results: Vec<u32> = tree.iter(order).map(|n| n.contents).collect();
    assert_eq!(results, expected);
}
