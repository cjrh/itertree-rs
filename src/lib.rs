#![warn(dead_code)]
#![warn(unused_imports)]

#[derive(Debug)]
pub struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    pub contents: u32,
}

impl Node {
    pub fn new(arr: &[[i32; 3]]) -> Node {
        let [idx, left, right] = arr[0];
        let l = match left {
            -1 => None,
            i => Some(Box::new(Node::new(&arr[(i - idx) as usize..]))),
        };
        let r = match right {
            -1 => None,
            i => Some(Box::new(Node::new(&arr[(i - idx) as usize..]))),
        };
        Node {
            contents: idx as u32,
            left: l,
            right: r,
        }
    }

    pub fn iter(&self, order: TraversalOrder) -> NodeIterPre {
        NodeIterPre::new(self, order)
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self as *const _ == other as *const _
    }
}

#[derive(Debug)]
pub enum TraversalOrder {
    PreOrder,
    InOrder,
    PostOrder,
}

pub struct NodeIterPre<'a> {
    stack: Vec<&'a Node>,
    order: TraversalOrder,
    node: Option<&'a Node>,
    last_visited_node: Option<&'a Node>,
}

impl<'a> NodeIterPre<'a> {
    fn new(node: &Node, order: TraversalOrder) -> NodeIterPre {
        match order {
            TraversalOrder::PreOrder =>
                NodeIterPre {
                    stack: vec![node],
                    order,
                    node: None,
                    last_visited_node: None,
                },
            TraversalOrder::InOrder =>
                NodeIterPre {
                    stack: vec![],
                    order,
                    node: Some(node),
                    last_visited_node: None,
                },
            TraversalOrder::PostOrder =>
                NodeIterPre {
                    stack: vec![],
                    order,
                    node: Some(node),
                    last_visited_node: None,
                }
        }
    }

    fn next_preorder(&mut self) -> Option<&'a Node> {
        let stack = &mut self.stack;
        match stack.pop() {
            None => None,
            Some(n) => {
                if let Some(bn) = &n.right {
                    stack.push(bn.as_ref());
                };
                if let Some(bn) = &n.left {
                    stack.push(bn.as_ref());
                };
                Some(n)
            }
        }
    }

    fn next_inorder(&mut self) -> Option<&'a Node> {
        let stack = &mut self.stack;
        let r = loop {
            if self.node.is_some() {
                let n = self.node.unwrap();
                stack.push(n);
                self.node = match &n.left {
                    None => None,
                    Some(left) => Some(left.as_ref()),
                };
            } else {
                let result = stack.pop();
                match result {
                    None => {},
                    Some(r) => {
                        self.node = match &r.right {
                            None => None,
                            Some(rr) => Some(rr.as_ref()),
                        }
                    }
                }
                break result;
            }
        };
        r
    }

    fn next_postorder(&mut self) -> Option<&'a Node> {
        let r= loop {
            if self.node.is_some() {
                let n = self.node.unwrap();
                self.stack.push(n);
                self.node = match &n.left {
                    None => None,
                    Some(left) => Some(left.as_ref()),
                };
            } else {
                let peek_node = self.stack.last();
                // if right child exists and traversing node from
                // left child, then move right.
                if let Some(pk) = peek_node {
                    let right = match &pk.right {
                        None => None,
                        Some(r) => Some(r.as_ref()),
                    };
                    if right.is_some() && self.last_visited_node.is_some() && *self.last_visited_node.unwrap() != *right.unwrap() {
                        self.node = right;
                    } else {
                        self.last_visited_node = self.stack.pop();
                        // break Some(peek_node);
                        break self.last_visited_node;
                    }
                } else {
                    break None;
                }
            }
        };
        r
    }

}

impl<'a> Iterator for NodeIterPre<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<&'a Node> {
        let r = match self.order {
            TraversalOrder::PreOrder => self.next_preorder(),
            TraversalOrder::InOrder => self.next_inorder(),
            TraversalOrder::PostOrder => self.next_postorder(),
        };
        r
    }
}

#[allow(dead_code)]
fn make_tree() {
    let arr_tree = [[1, 2, 3],
        [2, 4, 5],
        [3, 6, -1],
        [4, 7, -1],
        [5, -1, -1],
        [6, 8, 9],
        [7, -1, -1],
        [8, -1, -1],
        [9, -1, -1]];

    let tree = Node::new(&arr_tree);
    println!("{:?}", &tree);

    let orders = vec![
        TraversalOrder::PreOrder,
        TraversalOrder::InOrder,
        TraversalOrder::PostOrder,
    ];
    for order in orders {
        println!();
        println!("{:?}", &order);
        tree.iter(order).for_each(
            |n| print!("{:?} ", &n.contents)
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base() {
        println!("{}", 123);
        make_tree();
        assert_eq!(1, 1);
    }

}
