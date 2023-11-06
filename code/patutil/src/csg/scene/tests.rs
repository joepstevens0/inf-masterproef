#[test]
fn test_empty_scene_save() {
    use crate::csg::CSGScene;

    use std::path::Path;

    use crate::csg::SceneRef;

    // create scene
    let mut saved_scene: SceneRef = CSGScene::default().into();
    let mut loaded_scene: SceneRef = CSGScene::default().into();

    // write brick to file
    {
        saved_scene
            .controller()
            .save(Path::new("testfiles/test_empty_scene_save"));
    }

    // load brick from file
    {
        loaded_scene
            .controller()
            .load(Path::new("testfiles/test_empty_scene_save"));
    }

    let s1 = saved_scene.lock().to_string();
    let s2 = loaded_scene.lock().to_string();
    assert_eq!(
        s1, s2,
        "Scene have different values\n"
    );
}

#[test]
fn test_scene_save() {
    use crate::csg::{Brick, BrickOp, BrickType, CSGScene};

    use std::path::Path;

    use crate::csg::BrickRef;
    use crate::csg::SceneRef;

    // create scene
    let mut saved_scene: SceneRef = CSGScene::default().into();

    let brick: BrickRef = Brick::new(BrickType::Box, BrickOp::Inter, "Test brick 1 name").into();
    brick.controller().set_next_brick(Some(Brick::new(
        BrickType::Sphere,
        BrickOp::Inter,
        "Test brick 2 name",
    )));
    brick.controller().set_size([123., 213., 421., 242.]);
    brick
        .controller()
        .set_pos([2324., 2534513., 4654621., 254754742.]);
    brick.controller().set_color([255, 20, 19, 67]);
    saved_scene.controller().set_first(Some(brick.into()));

    let mut loaded_scene: SceneRef = CSGScene::default().into();

    // write brick to file
    {
        saved_scene
            .controller()
            .save(Path::new("testfiles/test_scene_save"));
    }

    // load brick from file
    {
        loaded_scene
            .controller()
            .load(Path::new("testfiles/test_scene_save"));
    }

    let s1 = saved_scene.lock().to_string();
    let s2 = loaded_scene.lock().to_string();
    assert_eq!(
        s1, s2,
        "Scene have different values\n",
    );
}

#[test]
fn test_scene_save_objects() {
    use crate::csg::{Brick, BrickOp, BrickType, CSGScene};

    use std::path::Path;

    use crate::csg::BrickRef;
    use crate::csg::SceneRef;

    // create scene
    let mut saved_scene: SceneRef = CSGScene::default().into();

    let object: BrickRef = Brick::new(BrickType::Sphere, BrickOp::Union, "object").into();
    let brick2: BrickRef = Brick::new(
        BrickType::Layer,
        BrickOp::Union,
        "Test brick 2 containing object",
    ).into();
    brick2.controller().set_child(Some(object.clone()));
    let brick: BrickRef = Brick::new(BrickType::Box, BrickOp::Inter, "Test brick 1 name").into();
    brick.controller().set_next_brick(Some(brick2));
    brick.controller().set_size([123., 213., 421., 242.]);
    brick
        .controller()
        .set_pos([2324., 2534513., 4654621., 254754742.]);
    brick.controller().set_color([255, 20, 19, 67]);
    saved_scene.controller().set_first(Some(brick.into()));
    saved_scene.controller().set_first_object(Some(object));

    let mut loaded_scene: SceneRef = CSGScene::default().into();

    // write brick to file
    {
        saved_scene
            .controller()
            .save(Path::new("testfiles/test_scene_save_objects"));
    }

    // load brick from file
    {
        loaded_scene
            .controller()
            .load(Path::new("testfiles/test_scene_save_objects"));
    }

    let s1 = saved_scene.lock().to_string();
    let s2 = loaded_scene.lock().to_string();
    assert_eq!(
        s1, s2,
        "Scene have different values\n",
    );
}
