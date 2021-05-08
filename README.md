# itertree-rs
Hobby project exploring tree traversal in Rust using iterators

## Demo

```rust
// Each row is 
// [node idx, <left child idx>, <right child idx>, <contents>]
// In the example below, the `contents` is simply the node index
// itself, for simplicity.
// The visual representation of this tree:
// 
//          1
//         / \
//        /   \
//       /     \
//      2       3
//     / \     /
//    4   5   6
//   /       / \
//  7       8   9
//
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
```

## Credits

My resources were
[this page on Rosetta Code](https://www.rosettacode.org/wiki/Tree_traversal#Rust),
[this page on Wikipedia](https://en.wikipedia.org/wiki/Tree_traversal),
and a great deal of head bashing against the Rust compiler.