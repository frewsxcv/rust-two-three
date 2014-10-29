use std::cmp::Ord;
use std::option::{None, Option, Some};
use std::to_string::ToString;


/// FourNode
struct FourNode<V: ToString+Ord>(V, V, V, Option<(Box<Node<V>>, Box<Node<V>>, Box<Node<V>>, Box<Node<V>>)>);

impl <V: ToString+Ord> FourNode<V> {

    fn from_leaf_3node_and_value(node: Node<V>, value: V) -> FourNode<V> {
        match node {
            LeafThreeNode(value1, value2) =>
                if value > value2      { FourNode(value1, value2, value, None) }
                else if value < value1 { FourNode(value, value1, value2, None) }
                else                   { FourNode(value1, value, value2, None) },
            _ => fail!(""),
        }
    }

    /// Convert a Split into a TwoNode with the middle value as the value of the new parent node
    fn to_two_node(self) -> Node<V> {
        match self {
            FourNode(v1, v2, v3, None) =>
                TwoNode(v2,
                    box LeafTwoNode(v1),
                    box LeafTwoNode(v3)),
            FourNode(v1, v2, v3, Some((n1, n2, n3, n4))) =>
                TwoNode(v2,
                    box TwoNode(v1, n1, n2),
                    box TwoNode(v3, n3, n4)),
        }
    }
}

/// InsertResult
enum InsertResult<V: ToString+Ord> {
    Fit(Node<V>),  // box this node?
    Split(FourNode<V>),  // split should always contain a ThreeNode/LeafThreeNode. box this?
}

/// Direction
enum Direction {
    Left,
    Middle,
    Right,
    Leaf,
}

/// Node
enum Node<V: ToString+Ord> {
    LeafTwoNode(V),
    LeafThreeNode(V, V),
    TwoNode(V, Box<Node<V>>, Box<Node<V>>),
    ThreeNode(V, V, Box<Node<V>>, Box<Node<V>>, Box<Node<V>>),
}

impl <V: ToString+Ord> ToString for Node<V> {
    fn to_string(&self) -> String {
        match *self {
            LeafTwoNode(ref v) => format!("LeafTwoNode({:s})", v.to_string()),
            LeafThreeNode(ref v1, ref v2) => format!("LeafThreeNode({:s}, {:s})", v1.to_string(), v2.to_string()),
            TwoNode(ref v, ref left, ref middle) => format!("TwoNode({:s}, {:s}, {:s})", v.to_string(), left.to_string(), middle.to_string()),
            ThreeNode(_, _, ref left, ref middle, ref right) => format!("ThreeNode({:s}, {:s}, {:s})", left.to_string(), middle.to_string(), right.to_string()),
        }
    }
}

impl <V: ToString+Ord> Node<V> {
    // PRIVATE
    /// Extend this TwoNode with the provided value and child node to make it a ThreeNode
    fn extend(self, other_value: V, other_node: Box<Node<V>>) -> Node<V> {
        match self {
            TwoNode(self_value, self_left, self_middle) => {
                if self_value > other_value {
                    ThreeNode(other_value, self_value, other_node, self_left, self_middle)
                } else {
                    ThreeNode(self_value, other_value, self_left, self_middle, other_node)
                }
            }
            _ => fail!(""),
        }
    }

    fn extend4(self, other_value: V, other_node: Box<Node<V>>) -> FourNode<V> {
        match self {
            ThreeNode(self_value1, self_value2, self_left, self_middle, self_right) => {
                if other_value < self_value1 {
                    FourNode(other_value, self_value1, self_value2, Some((other_node, self_left, self_middle, self_right)))
                } else {
                    FourNode(self_value1, self_value2, other_value, Some((self_left, self_middle, self_right, other_node)))
                }
            }
            _ => fail!(""),
        }
    }

    fn next_direction(&self, to_insert: &V) -> Direction {
        match self {
            &TwoNode(ref value, _, _) =>
                if to_insert < value { Left }
                else                 { Middle },
            &ThreeNode(ref value1, ref value2, _, _, _) =>
                if      to_insert < value1 { Left }
                else if to_insert > value2 { Right }
                else                       { Middle },
            _ => Leaf,
        }
    }

    fn contains_node(&self, to_check: &V) -> bool {
        match self {
            &TwoNode(ref v, _, _) |
                &LeafTwoNode(ref v) if to_check == v => true,

            &ThreeNode(ref v1, ref v2, _, _, _) |
                &LeafThreeNode(ref v1, ref v2) if to_check == v1 || to_check == v2 => true,

            _ => false
        }
    }

    fn insert(self, to_insert: V) -> InsertResult<V> {
        // If value is already in this node, we're done (prevent duplicates)
        if self.contains_node(&to_insert) { return Fit(self); }

        let next_direction = self.next_direction(&to_insert);

        match self {
            // Insert if leaf TwoNode
            LeafTwoNode(value) => {
                if to_insert > value { Fit(LeafThreeNode(value, to_insert)) }
                else                 { Fit(LeafThreeNode(to_insert, value)) }
            },

            // Split if leaf ThreeNode
            node @ LeafThreeNode(..) => {
                let four_node = FourNode::from_leaf_3node_and_value(node, to_insert);
                Split(four_node)
            },

            // Recurse down if internal Node and handle results
            TwoNode(value, box left, box middle) => {

                // Determine which node we'll recurse next
                let (next_node, other_node) = match next_direction {
                    Left => (left, middle),
                    Middle => (middle, left),
                    _ => fail!(""),
                };

                // Recurse, save the result of the insert
                let insert_result = next_node.insert(to_insert);

                // Handle the result of the insert
                let new_node = match insert_result {
                    Fit(returned_node) =>
                        match next_direction {
                            Left => TwoNode(value, box returned_node, box other_node),
                            Middle => TwoNode(value, box other_node, box returned_node),
                            _ => fail!(""),
                        },
                    Split(four_node) => four_node.to_two_node().extend(value, box other_node),
                };

                Fit(new_node)
            },

            // Recurse down if internal Node and handle results
            ThreeNode(value1, value2, left, middle, right) => {

                // Determine which node we'll recurse next
                let (next_node, other_node1, other_node2) = match next_direction {
                    Left => (left, middle, right),
                    Middle => (middle, left, right),
                    Right => (right, left, middle),
                    _ => fail!(""),
                };

                let insert_result = next_node.insert(to_insert);

                match insert_result {
                    Fit(returned_node) => {
                        let new_node = match next_direction {
                            Left => ThreeNode(value1, value2, box returned_node, other_node1, other_node2),
                            Middle => ThreeNode(value1, value2, other_node1, box returned_node, other_node2),
                            Right => ThreeNode(value1, value2, other_node1, other_node2, box returned_node),
                            _ => fail!(""),
                        };
                        Fit(new_node)
                    },
                    Split(four_node) => {
                        let new_node: FourNode<V> = match next_direction {
                            Left => four_node.to_two_node().extend(value1, other_node1).extend4(value2, other_node2),
                            Middle => four_node.to_two_node().extend(value1, other_node1).extend4(value2, other_node2),
                            Right => four_node.to_two_node().extend(value2, other_node2).extend4(value1, other_node1),
                            _ => fail!(""),
                        };
                        Split(new_node)
                    }
                }
            }
        }
    }
}


/// TTTree
pub struct TTTree<V: ToString+Ord> {
    root: Option<Node<V>>,
}

impl <V: ToString+Ord> ToString for TTTree<V> {
    fn to_string(&self) -> String {
        match self.root {
            Some(ref node) => node.to_string(),
            None => "TTTree(<empty>)".to_string(),
        }
    }
}

impl <V: ToString+Ord> TTTree<V> {
    pub fn new() -> TTTree<V> {
        TTTree { root: None }
    }

    pub fn insert(&mut self, value: V) -> () {
        let root: Option<Node<V>> = self.root.take();
        let new_root: Node<V> = match root {
            None => LeafTwoNode(value),
            Some(root) => {
                let result = root.insert(value);
                match result {
                    Fit(node) => node,
                    Split(node @ FourNode(..)) => node.to_two_node(),
                }
            }
        };
        self.root = Some(new_root);
    }
}



// TODO:
// Collection trait
// convert this to Key/Value K/V
// remove ToString debugging stuffs


#[test]
fn it_works() {
    let mut tree: TTTree<uint> = TTTree::new();

    tree.insert(50);
    match tree {
        TTTree{root: Some(LeafTwoNode(50))} => (),
        _ => fail!(),
    }

    tree.insert(50);
    match tree {
        TTTree{root: Some(LeafTwoNode(50))} => (),
        _ => fail!(),
    }

    tree.insert(60);
    match tree {
        TTTree{root: Some(LeafThreeNode(50, 60))} => (),
        _ => fail!(),
    }

    tree.insert(70);
    match tree {
        TTTree{root: Some(
            TwoNode(60,
                box LeafTwoNode(50),
                box LeafTwoNode(70)
            )
        )} => (),
        _ => fail!(),
    }

    tree.insert(100);
    match tree {
        TTTree{root: Some(
            TwoNode(60,
                box LeafTwoNode(50),
                box LeafThreeNode(70, 100)
            )
        )} => (),
        _ => fail!(),
    }

    tree.insert(55);
    match tree {
        TTTree{root: Some(
            TwoNode(60,
                box LeafThreeNode(50, 55),
                box LeafThreeNode(70, 100)
            )
        )} => (),
        _ => fail!(),
    }

    tree.insert(68);
    match tree {
        TTTree{root: Some(
            ThreeNode(60, 70,
                box LeafThreeNode(50, 55),
                box LeafTwoNode(68),
                box LeafTwoNode(100)
            )
        )} => (),
        _ => fail!(),
    }

    tree.insert(75);
    match tree {
        TTTree{root: Some(
            ThreeNode(60, 70,
                box LeafThreeNode(50, 55),
                box LeafTwoNode(68),
                box LeafThreeNode(75, 100)
            )
        )} => (),
        _ => fail!(),
    }

    tree.insert(52);
    match tree {
        TTTree{root: Some(
            TwoNode(60,
                box TwoNode(52,
                    box LeafTwoNode(50),
                    box LeafTwoNode(55),
                ),
                box TwoNode(70,
                    box LeafTwoNode(68),
                    box LeafThreeNode(75, 100),
                ),
            )
        )} => (),
        _ => fail!(),
    }
}
