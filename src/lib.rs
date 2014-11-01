// Many of my decisions were based off the following document
// https://web.archive.org/web/20140727185616/http://algs4.cs.princeton.edu/33balanced/

use std::cmp::Ord;
use std::option::{None, Option, Some};
use std::to_string::ToString;


/// Four
struct Four<V: ToString+Ord>(V, V, V, Option<(Box<Node<V>>, Box<Node<V>>, Box<Node<V>>, Box<Node<V>>)>);

impl <V: ToString+Ord> Four<V> {

    /// Convert a Split into a TwoNode with the middle value as the value of the new parent node
    fn to_two(self) -> Two<V> {
        match self {
            Four(v1, v2, v3, None) =>
                Two(v2,
                    box LeafTwoNode(LeafTwo(v1)),
                    box LeafTwoNode(LeafTwo(v3))),
            Four(v1, v2, v3, Some((n1, n2, n3, n4))) =>
                Two(v2,
                    box TwoNode(Two(v1, n1, n2)),
                    box TwoNode(Two(v3, n3, n4))),
        }
    }
}


/// InsertResult
enum InsertResult<V: ToString+Ord> {
    Fit(Node<V>),  // box this node?
    Split(Four<V>),  // split should always contain a ThreeNode/LeafThreeNode. box this?
}


/// Direction
enum Direction {
    Left,
    Middle,
    Right,
    Leaf,
}


/// Two
pub struct Two<V: ToString+Ord>(pub V, pub Box<Node<V>>, pub Box<Node<V>>);

impl <V: ToString+Ord> Two<V> {
    fn to_node(self) -> Node<V> { TwoNode(self) }

    fn to_three(self, other_value: V, other_node: Box<Node<V>>) -> Three<V> {
        let Two(self_value, self_left, self_middle) = self;
        if self_value > other_value {
            Three(other_value, self_value, other_node, self_left, self_middle)
        } else {
            Three(self_value, other_value, self_left, self_middle, other_node)
        }
    }
}


/// Three
pub struct Three<V: ToString+Ord>(pub V, pub V, pub Box<Node<V>>, pub Box<Node<V>>, pub Box<Node<V>>);

impl <V: ToString+Ord> Three<V> {
    fn to_node(self) -> Node<V> { ThreeNode(self) }

    fn to_four(self, other_value: V, other_node: Box<Node<V>>) -> Four<V> {
        let Three(self_value1, self_value2, self_left, self_middle, self_right) = self;
        if other_value < self_value1 {
            Four(other_value, self_value1, self_value2, Some((other_node, self_left, self_middle, self_right)))
        } else {
            Four(self_value1, self_value2, other_value, Some((self_left, self_middle, self_right, other_node)))
        }
    }
}


/// LeafTwo
pub struct LeafTwo<V: ToString+Ord>(pub V);


/// LeafThree
pub struct LeafThree<V: ToString+Ord>(pub V, pub V);

impl <V: ToString+Ord> LeafThree<V> {
    fn to_four(self, value: V) -> Four<V> {
        let LeafThree(value1, value2) = self;
        if value > value2      { Four(value1, value2, value, None) }
        else if value < value1 { Four(value, value1, value2, None) }
        else                   { Four(value1, value, value2, None) }
    }
}


/// Node
pub enum Node<V: ToString+Ord> {
    LeafTwoNode(LeafTwo<V>),
    LeafThreeNode(LeafThree<V>),
    TwoNode(Two<V>),
    ThreeNode(Three<V>),
}

impl <V: ToString+Ord> ToString for Node<V> {
    fn to_string(&self) -> String {
        match *self {
            LeafTwoNode(LeafTwo(ref v)) => format!("LeafTwoNode({:s})", v.to_string()),
            LeafThreeNode(LeafThree(ref v1, ref v2)) => format!("LeafThreeNode({:s}, {:s})", v1.to_string(), v2.to_string()),
            TwoNode(Two(ref v, ref left, ref middle)) => format!("TwoNode({:s}, {:s}, {:s})", v.to_string(), left.to_string(), middle.to_string()),
            ThreeNode(Three(_, _, ref left, ref middle, ref right)) => format!("ThreeNode({:s}, {:s}, {:s})", left.to_string(), middle.to_string(), right.to_string()),
        }
    }
}



impl <V: ToString+Ord> Node<V> {
    // PRIVATE

    fn next_direction(&self, to_insert: &V) -> Direction {
        match self {
            &TwoNode(Two(ref value, _, _)) =>
                if to_insert < value { Left }
                else                 { Middle },
            &ThreeNode(Three(ref value1, ref value2, _, _, _)) =>
                if      to_insert < value1 { Left }
                else if to_insert > value2 { Right }
                else                       { Middle },
            _ => Leaf,
        }
    }

    fn contains_node(&self, to_check: &V) -> bool {
        match self {
            &TwoNode(Two(ref v, _, _)) |
                &LeafTwoNode(LeafTwo(ref v)) if to_check == v => true,

            &ThreeNode(Three(ref v1, ref v2, _, _, _)) |
                &LeafThreeNode(LeafThree(ref v1, ref v2)) if to_check == v1 || to_check == v2 => true,

            _ => false
        }
    }

    fn insert(self, to_insert: V) -> InsertResult<V> {
        // If value is already in this node, we're done (prevent duplicates)
        if self.contains_node(&to_insert) { return Fit(self); }

        let next_direction = self.next_direction(&to_insert);

        match self {
            // Insert if leaf TwoNode
            LeafTwoNode(LeafTwo(value)) => {
                let (smaller, larger) =
                    if value < to_insert { (value, to_insert) }
                    else                 { (to_insert, value) };
                let new_node = LeafThreeNode(LeafThree(smaller, larger));
                Fit(new_node)
            },

            // Split if leaf ThreeNode
            LeafThreeNode(leaf_three) => {
                let four_node = leaf_three.to_four(to_insert);
                Split(four_node)
            },

            // Recurse down if internal Node and handle results
            TwoNode(Two(value, box left, box middle)) => {

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
                            Left => Two(value, box returned_node, box other_node).to_node(),
                            Middle => Two(value, box other_node, box returned_node).to_node(),
                            _ => fail!(""),
                        },
                    Split(four_node) => four_node.to_two().to_three(value, box other_node).to_node(),
                };

                Fit(new_node)
            },

            // Recurse down if internal Node and handle results
            ThreeNode(Three(value1, value2, left, middle, right)) => {

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
                        let three = match next_direction {
                            Left =>   Three(value1, value2, box returned_node, other_node1, other_node2),
                            Middle => Three(value1, value2, other_node1, box returned_node, other_node2),
                            Right =>  Three(value1, value2, other_node1, other_node2, box returned_node),
                            _ => fail!(""),
                        };
                        Fit(three.to_node())
                    },
                    Split(four_node) => {
                        let new_node: Four<V> = match next_direction {
                            Left =>   four_node.to_two().to_three(value1, other_node1).to_four(value2, other_node2),
                            Middle => four_node.to_two().to_three(value1, other_node1).to_four(value2, other_node2),
                            Right =>  four_node.to_two().to_three(value2, other_node2).to_four(value1, other_node1),
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
    pub root: Option<Node<V>>,
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
            None => LeafTwoNode(LeafTwo(value)),
            Some(root) => {
                let result = root.insert(value);
                match result {
                    Fit(node) => node,
                    Split(node @ Four(..)) => node.to_two().to_node(),
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
