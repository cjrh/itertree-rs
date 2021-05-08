#![warn(dead_code)]
#![warn(unused_imports)]

#[derive(Debug)]
pub struct Node<'a, T> {
    left: Option<Box<Node<'a, T>>>,
    right: Option<Box<Node<'a, T>>>,
    pub contents: &'a T,
}

impl<'a, T> Node<'a, T> {
    pub fn new(arr: &[(i32, i32, i32, T)]) -> Node<T> {
        let (idx, left, right, contents) = &arr[0];
        let l = match left {
            -1 => None,
            i => Some(Box::new(Node::new(&arr[(i - idx) as usize..]))),
        };
        let r = match right {
            -1 => None,
            i => Some(Box::new(Node::new(&arr[(i - idx) as usize..]))),
        };
        Node {
            contents: contents,
            left: l,
            right: r,
        }
    }

    pub fn iter(&self, order: TraversalOrder) -> NodeIterPre<T> {
        NodeIterPre::new(self, order)
    }
}

impl<'a, T> PartialEq for Node<'a, T> {
    fn eq(&self, other: &Node<T>) -> bool {
        self as *const _ == other as *const _
    }
}

#[derive(Debug)]
pub enum TraversalOrder {
    /// Uses: create a copy of the tree. Also prefix expressions like RPN.
    PreOrder,
    /// Uses: for binary search trees, gives ascending order.
    InOrder,
    /// Uses: can be used to delete the tree, or parts of it.
    PostOrder,
}

pub struct NodeIterPre<'a, T> {
    stack: Vec<&'a Node<'a, T>>,
    order: TraversalOrder,
    node: Option<&'a Node<'a, T>>,
    last_visited_node: Option<&'a Node<'a, T>>,
}

impl<'a, T> NodeIterPre<'a, T> {
    fn new(node: &'a Node<'a, T>, order: TraversalOrder) -> NodeIterPre<'a, T> {
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

    fn next_preorder(&mut self) -> Option<&'a Node<'a, T>> {
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

    fn next_inorder(&mut self) -> Option<&'a Node<'a, T>> {
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

    fn next_postorder(&mut self) -> Option<&'a Node<'a, T>> {
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

impl<'a, T> Iterator for NodeIterPre<'a, T> {
    type Item = &'a Node<'a, T>;

    fn next(&mut self) -> Option<&'a Node<'a, T>> {
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
    let arr_tree = [
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
