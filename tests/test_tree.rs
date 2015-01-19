#![feature(box_syntax)]

extern crate two_three;

use two_three::{Node, TTTree, Two, Three, LeafTwo, LeafThree};

#[test]
fn it_works() {
    let mut tree: TTTree<u64> = TTTree::new();

    tree.insert(50);
    match tree {
        TTTree{root: Some(Node::LeafTwo(LeafTwo(50)))} => (),
        _ => panic!(),
    }

    tree.insert(50);
    match tree {
        TTTree{root: Some(Node::LeafTwo(LeafTwo(50)))} => (),
        _ => panic!(),
    }

    tree.insert(60);
    match tree {
        TTTree{root: Some(Node::LeafThree(LeafThree(50, 60)))} => (),
        _ => panic!(),
    }

    tree.insert(70);
    match tree {
        TTTree{root: Some(
            Node::Two(Two(60,
                box Node::LeafTwo(LeafTwo(50)),
                box Node::LeafTwo(LeafTwo(70))
            ))
        )} => (),
        _ => panic!(),
    }

    tree.insert(100);
    match tree {
        TTTree{root: Some(
            Node::Two(Two(60,
                box Node::LeafTwo(LeafTwo(50)),
                box Node::LeafThree(LeafThree(70, 100))
            ))
        )} => (),
        _ => panic!(),
    }

    tree.insert(55);
    match tree {
        TTTree{root: Some(
            Node::Two(Two(60,
                box Node::LeafThree(LeafThree(50, 55)),
                box Node::LeafThree(LeafThree(70, 100))
            ))
        )} => (),
        _ => panic!(),
    }

    tree.insert(68);
    match tree {
        TTTree{root: Some(
            Node::Three(Three(60, 70,
                box Node::LeafThree(LeafThree(50, 55)),
                box Node::LeafTwo(LeafTwo(68)),
                box Node::LeafTwo(LeafTwo(100))
            ))
        )} => (),
        _ => panic!(),
    }

    tree.insert(75);
    match tree {
        TTTree{root: Some(
            Node::Three(Three(60, 70,
                box Node::LeafThree(LeafThree(50, 55)),
                box Node::LeafTwo(LeafTwo(68)),
                box Node::LeafThree(LeafThree(75, 100))
            ))
        )} => (),
        _ => panic!(),
    }

    tree.insert(52);
    match tree {
        TTTree{root: Some(
            Node::Two(Two(60,
                box Node::Two(Two(52,
                    box Node::LeafTwo(LeafTwo(50)),
                    box Node::LeafTwo(LeafTwo(55)),
                )),
                box Node::Two(Two(70,
                    box Node::LeafTwo(LeafTwo(68)),
                    box Node::LeafThree(LeafThree(75, 100)),
                )),
            ))
        )} => (),
        _ => panic!(),
    }
}
