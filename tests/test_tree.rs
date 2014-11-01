extern crate rust_two_three;

use rust_two_three::{ThreeNode, LeafThreeNode, TwoNode, LeafTwoNode, TTTree, LeafTwo, LeafThree, Two, Three};


#[test]
fn it_works() {
    let mut tree: TTTree<uint> = TTTree::new();

    tree.insert(50);
    match tree {
        TTTree{root: Some(LeafTwoNode(LeafTwo(50)))} => (),
        _ => fail!(),
    }

    tree.insert(50);
    match tree {
        TTTree{root: Some(LeafTwoNode(LeafTwo(50)))} => (),
        _ => fail!(),
    }

    tree.insert(60);
    match tree {
        TTTree{root: Some(LeafThreeNode(LeafThree(50, 60)))} => (),
        _ => fail!(),
    }

    tree.insert(70);
    match tree {
        TTTree{root: Some(
            TwoNode(Two(60,
                box LeafTwoNode(LeafTwo(50)),
                box LeafTwoNode(LeafTwo(70))
            ))
        )} => (),
        _ => fail!(),
    }

    tree.insert(100);
    match tree {
        TTTree{root: Some(
            TwoNode(Two(60,
                box LeafTwoNode(LeafTwo(50)),
                box LeafThreeNode(LeafThree(70, 100))
            ))
        )} => (),
        _ => fail!(),
    }

    tree.insert(55);
    match tree {
        TTTree{root: Some(
            TwoNode(Two(60,
                box LeafThreeNode(LeafThree(50, 55)),
                box LeafThreeNode(LeafThree(70, 100))
            ))
        )} => (),
        _ => fail!(),
    }

    tree.insert(68);
    match tree {
        TTTree{root: Some(
            ThreeNode(Three(60, 70,
                box LeafThreeNode(LeafThree(50, 55)),
                box LeafTwoNode(LeafTwo(68)),
                box LeafTwoNode(LeafTwo(100))
            ))
        )} => (),
        _ => fail!(),
    }

    tree.insert(75);
    match tree {
        TTTree{root: Some(
            ThreeNode(Three(60, 70,
                box LeafThreeNode(LeafThree(50, 55)),
                box LeafTwoNode(LeafTwo(68)),
                box LeafThreeNode(LeafThree(75, 100))
            ))
        )} => (),
        _ => fail!(),
    }

    tree.insert(52);
    match tree {
        TTTree{root: Some(
            TwoNode(Two(60,
                box TwoNode(Two(52,
                    box LeafTwoNode(LeafTwo(50)),
                    box LeafTwoNode(LeafTwo(55)),
                )),
                box TwoNode(Two(70,
                    box LeafTwoNode(LeafTwo(68)),
                    box LeafThreeNode(LeafThree(75, 100)),
                )),
            ))
        )} => (),
        _ => fail!(),
    }
}
