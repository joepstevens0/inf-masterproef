// #[test]
// fn test_brick_save() {
//     use crate::csg::{Brick, BrickOp, BrickType};
//     use patfile::pscan;
//     use patfile::pwrite;
//     use std::fs::File;
//     use std::io::Read;

//     let mut saved_brick = Brick::new(BrickType::Box, BrickOp::Inter, "Test brick name");
//     saved_brick.controller().set_next_brick(Some(Brick::new(BrickType::Sphere, BrickOp::Inter, "Test brick 2 name")));
//     saved_brick.controller().set_size([123., 213., 421., 242.]);
//     saved_brick
//         .controller()
//         .set_pos([2324., 2534513., 4654621., 254754742.]);
//     saved_brick.controller().set_color([255, 20, 19, 67]);

//     let mut loaded_brick = Brick::default();

//     // write brick to file
//     {
//         let mut file = File::create("testfiles/test_brick_save").unwrap();
//         pwrite!(&mut file, "brick: {}", saved_brick.clone()).unwrap();
//     }

//     // load brick from file
//     {
//         let file = File::open("testfiles/test_brick_save").unwrap();
//         let it: &mut dyn Iterator<Item = u8> = &mut file.bytes().map(std::result::Result::unwrap);
//         pscan!(it => "brick: {}", loaded_brick).unwrap();
//     }

//     assert_eq!(saved_brick, loaded_brick);
// }

#[test]
fn test_brick_simple() {
    use crate::csg::{Brick, BrickOp, BrickType};
    use patfile::pscan;
    use patfile::pwrite;
    use std::fs::File;
    use std::io::Read;
    use crate::csg::BrickRef;

    let saved_brick: BrickRef = Brick::new(BrickType::Box, BrickOp::Inter, "Test brick name").into();
    saved_brick.controller().set_size([123., 213., 421., 242.]);
    saved_brick
        .controller()
        .set_pos([2324., 2534513., 4654621., 254754742.]);
    saved_brick.controller().set_color([255, 20, 19, 67]);

    let mut loaded_brick = Brick::default();

    // write brick to file
    {
        let mut file = File::create("testfiles/test_brick_save").unwrap();
        pwrite!(&mut file, "brick: {}", saved_brick.get()).unwrap();
    }

    // load brick from file
    {
        let file = File::open("testfiles/test_brick_save").unwrap();
        let it: &mut dyn Iterator<Item = u8> = &mut file.bytes().map(std::result::Result::unwrap);
        pscan!(it => "brick: {}", loaded_brick).unwrap();
    }

    assert_eq!(saved_brick.get(), loaded_brick);
}

// #[test]
// fn test_brick_save_multiple() {
//     use crate::csg::{Brick, BrickOp, BrickType};
//     use patfile::pscan;
//     use patfile::pwrite;
//     use std::fs::File;
//     use std::io::Read;

//     let mut saved_brick1 = Brick::new(BrickType::Box, BrickOp::Inter, "Test brick name");
//     saved_brick1.controller().set_next_brick(Some(Brick::new(BrickType::Sphere, BrickOp::Inter, "Test brick 2 name")));
//     saved_brick1
//         .controller()
//         .set_size([1253., 2153., 5421., 2542.]);
//     saved_brick1
//         .controller()
//         .set_pos([2324., 2534513., 4654621., 254754742.]);
//     saved_brick1.controller().set_color([255, 20, 19, 67]);

//     let mut saved_brick2 = Brick::new(BrickType::Sphere, BrickOp::Diff, "Test2 brick name");
//     saved_brick2.controller().set_next_brick(Some(Brick::new(BrickType::Sphere, BrickOp::Inter, "Test2 brick 2 name")));
//     saved_brick2
//         .controller()
//         .set_size([123., 213., 421., 2432.]);
//     saved_brick2
//         .controller()
//         .set_pos([2324., 253513., 654621., 4754742.]);
//     saved_brick2.controller().set_color([7, 20, 255, 67]);

//     let mut saved_brick3 = Brick::new(BrickType::Sphere, BrickOp::Union, "Test3 brick name");
//     saved_brick3.controller().set_next_brick(Some(Brick::new(BrickType::Sphere, BrickOp::Inter, "Test3 brick 2 name")));
//     saved_brick3
//         .controller()
//         .set_size([123., 2213., 421., 242.]);
//     saved_brick3
//         .controller()
//         .set_pos([234., 253453., 4654621., 254754742.]);
//     saved_brick3.controller().set_color([25, 255, 19, 67]);

//     // write brick to file
//     {
//         let mut file = File::create("testfiles/test_brick_save_multiple").unwrap();
//         pwrite!(&mut file, "brick: {}\n", saved_brick1.clone()).unwrap();
//         pwrite!(&mut file, "brick: {}\n", saved_brick2.clone()).unwrap();
//         pwrite!(&mut file, "brick: {}\n", saved_brick3.clone()).unwrap();
//     }

//     // load brick from file
//     {
//         let mut loaded_brick = Brick::default();
//         let file = File::open("testfiles/test_brick_save_multiple").unwrap();
//         let it: &mut dyn Iterator<Item = u8> = &mut file.bytes().map(std::result::Result::unwrap);

//         pscan!(it => "brick: {}\n", loaded_brick).unwrap();
//         assert_eq!(saved_brick1, loaded_brick);
//         pscan!(it => "brick: {}\n", loaded_brick).unwrap();
//         assert_eq!(saved_brick2, loaded_brick);
//         pscan!(it => "brick: {}\n", loaded_brick).unwrap();
//         assert_eq!(saved_brick3, loaded_brick);
//     }
// }
