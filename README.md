# itertree-rs
Hobby project exploring tree traversal in Rust using iterators

## Demo

```rust
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
```

## Credits

My resources were 
[this page on Rosetta Code](https://www.rosettacode.org/wiki/Tree_traversal#Rust),
[this page on Wikipedia](https://en.wikipedia.org/wiki/Tree_traversal),
and a great deal of head bashing against the Rust compiler.