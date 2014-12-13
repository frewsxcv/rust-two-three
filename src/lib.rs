// Many of my decisions were based off the following document
// https://web.archive.org/web/20140727185616/http://algs4.cs.princeton.edu/33balanced/

use Direction::{Left, Middle, Right, Leaf};
use InsertResult::{Fit, Split};


/// SplitNode
enum SplitNode<V: Ord> {
    Four(V, V, V, Box<Node<V>>, Box<Node<V>>, Box<Node<V>>, Box<Node<V>>),
    LeafFour(V, V, V),
}

impl <V: Ord> SplitNode<V> {
    fn to_two(self) -> Two<V> {
        match self {
            SplitNode::LeafFour(v1, v2, v3) =>
                Two(v2,
                    box Node::LeafTwo(LeafTwo(v1)),
                    box Node::LeafTwo(LeafTwo(v3))),
            SplitNode::Four(v1, v2, v3, n1, n2, n3, n4) =>
                Two(v2,
                    box Node::Two(Two(v1, n1, n2)),
                    box Node::Two(Two(v3, n3, n4))),
        }
    }
}


/// InsertResult
enum InsertResult<V: Ord> {
    Fit(Node<V>),
    Split(SplitNode<V>),
}


/// Direction
enum Direction {
    Left,
    Middle,
    Right,
    Leaf,
}


/// Two
pub struct Two<V: Ord>(pub V, pub Box<Node<V>>, pub Box<Node<V>>);

impl <V: Ord> Two<V> {
    fn as_node(self) -> Node<V> { Node::Two(self) }

    fn to_three(self, other_value: V, other_node: Box<Node<V>>) -> Three<V> {
        let Two(self_value, self_left, self_middle) = self;
        if self_value > other_value {
            Three(other_value, self_value, other_node, self_left, self_middle)
        } else {
            Three(self_value, other_value, self_left, self_middle, other_node)
        }
    }

    fn has_value(&self, value: &V) -> bool {
        let &Two(ref self_value, _, _) = self;
        *self_value == *value
    }
}


/// Three
pub struct Three<V: Ord>(pub V, pub V, pub Box<Node<V>>, pub Box<Node<V>>, pub Box<Node<V>>);

impl <V: Ord> Three<V> {
    fn as_node(self) -> Node<V> { Node::Three(self) }

    fn to_split_node(self, other_value: V, other_node: Box<Node<V>>) -> SplitNode<V> {
        let Three(self_value1, self_value2, self_left, self_middle, self_right) = self;
        if other_value < self_value1 {
            SplitNode::Four(other_value, self_value1, self_value2, other_node, self_left, self_middle, self_right)
        } else {
            SplitNode::Four(self_value1, self_value2, other_value, self_left, self_middle, self_right, other_node)
        }
    }

    fn has_value(&self, value: &V) -> bool {
        let Three(ref self_value1, ref self_value2, _, _, _) = *self;
        *self_value1 == *value || *self_value2 == *value
    }
}


/// LeafTwo
pub struct LeafTwo<V: Ord>(pub V);

impl <V: Ord> LeafTwo<V> {
    fn to_three(self, other_value: V) -> LeafThree<V> {
        let LeafTwo(self_value) = self;
        if self_value > other_value {
            LeafThree(other_value, self_value)
        } else {
            LeafThree(self_value, other_value)
        }
    }

    fn has_value(&self, value: &V) -> bool {
        let &LeafTwo(ref self_value) = self;
        *self_value == *value
    }
}


/// LeafThree
pub struct LeafThree<V: Ord>(pub V, pub V);

impl <V: Ord> LeafThree<V> {
    fn as_node(self) -> Node<V> { Node::LeafThree(self) }

    fn to_split_node(self, value: V) -> SplitNode<V> {
        let LeafThree(value1, value2) = self;
        if value > value2      { SplitNode::LeafFour(value1, value2, value) }
        else if value < value1 { SplitNode::LeafFour(value, value1, value2) }
        else                   { SplitNode::LeafFour(value1, value, value2) }
    }

    fn has_value(&self, value: &V) -> bool {
        let &LeafThree(ref self_value1, ref self_value2) = self;
        *self_value1 == *value || *self_value2 == *value
    }
}


/// Node
pub enum Node<V: Ord> {
    LeafTwo(LeafTwo<V>),
    LeafThree(LeafThree<V>),
    Two(Two<V>),
    Three(Three<V>),
}


impl <V: Ord> Node<V> {
    fn next_direction(&self, to_insert: &V) -> Direction {
        match *self {
            Node::Two(Two(ref value, _, _)) =>
                if to_insert < value { Left }
                else                 { Middle },
            Node::Three(Three(ref value1, ref value2, _, _, _)) =>
                if      to_insert < value1 { Left }
                else if to_insert > value2 { Right }
                else                       { Middle },
            _ => Leaf,
        }
    }

    fn has_value(&self, value: &V) -> bool {
        match *self {
            Node::LeafTwo(ref n)   => n.has_value(value),
            Node::LeafThree(ref n) => n.has_value(value),
            Node::Two(ref n)       => n.has_value(value),
            Node::Three(ref n)     => n.has_value(value),
        }
    }

    fn insert(self, to_insert: V) -> InsertResult<V> {
        // If value is already in this node, we're done (prevent duplicates)
        if self.has_value(&to_insert) { return Fit(self); }

        let next_direction = self.next_direction(&to_insert);

        match self {
            // Insert if leaf TwoNode
            Node::LeafTwo(leaf_two) => {
                let three_node = leaf_two.to_three(to_insert);
                Fit(three_node.as_node())
            },

            // Split if leaf ThreeNode
            Node::LeafThree(leaf_three) => {
                let split_node = leaf_three.to_split_node(to_insert);
                Split(split_node)
            },

            // Recurse down if internal Node and handle results
            Node::Two(Two(value, box left, box middle)) => {

                // Determine which node we'll recurse next
                let (next_node, other_node) = match next_direction {
                    Left => (left, middle),
                    Middle => (middle, left),
                    _ => panic!(""),
                };

                // Recurse, save the result of the insert
                let insert_result = next_node.insert(to_insert);

                // Handle the result of the insert
                let new_node = match insert_result {
                    Fit(returned_node) =>
                        match next_direction {
                            Left =>   Two(value, box returned_node, box other_node),
                            Middle => Two(value, box other_node, box returned_node),
                            _ => panic!(""),
                        }.as_node(),
                    Split(split_node) => split_node.to_two().to_three(value, box other_node).as_node(),
                };

                Fit(new_node)
            },

            // Recurse down if internal Node and handle results
            Node::Three(Three(value1, value2, left, middle, right)) => {

                // Determine which node we'll recurse next
                let (next_node, other_node1, other_node2) = match next_direction {
                    Left => (left, middle, right),
                    Middle => (middle, left, right),
                    Right => (right, left, middle),
                    _ => panic!(""),
                };

                let insert_result = next_node.insert(to_insert);

                match insert_result {
                    Fit(returned_node) => {
                        let three = match next_direction {
                            Left =>   Three(value1, value2, box returned_node, other_node1, other_node2),
                            Middle => Three(value1, value2, other_node1, box returned_node, other_node2),
                            Right =>  Three(value1, value2, other_node1, other_node2, box returned_node),
                            _ => panic!(""),
                        };
                        Fit(three.as_node())
                    },
                    Split(split_node) => {
                        let two = split_node.to_two();
                        let new_node = match next_direction {
                            Left =>   two.to_three(value1, other_node1).to_split_node(value2, other_node2),
                            Middle => two.to_three(value1, other_node1).to_split_node(value2, other_node2),
                            Right =>  two.to_three(value2, other_node2).to_split_node(value1, other_node1),
                            _ => panic!(""),
                        };
                        Split(new_node)
                    }
                }
            }
        }
    }
}


/// TTTree
pub struct TTTree<V: Ord> {
    pub root: Option<Node<V>>,
}

impl <V: Ord> TTTree<V> {
    pub fn new() -> TTTree<V> {
        TTTree { root: None }
    }

    pub fn insert(&mut self, value: V) -> () {
        let root: Option<Node<V>> = self.root.take();
        let new_root: Node<V> = match root {
            None => Node::LeafTwo(LeafTwo(value)),
            Some(root) => {
                let result = root.insert(value);
                match result {
                    Fit(node) => node,
                    Split(split_node) => split_node.to_two().as_node(),
                }
            }
        };
        self.root = Some(new_root);
    }
}
