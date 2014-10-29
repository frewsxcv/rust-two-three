extern crate rust_two_three;

use rust_two_three::{ThreeNode, LeafThreeNode, TwoNode, LeafTwoNode, TTTree};


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
