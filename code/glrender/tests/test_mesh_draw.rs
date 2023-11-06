
use glrender::{window::Window, GLMesh};
use patutil::{Render, Rect, Color, mesh::{self, SceneRef, MeshRef, ModelRef}, Drawable};

const APP_RECT: Rect = Rect::new(0, 0, 800u32,800u32);

#[derive(Clone)]
struct App{
    pos: patutil::Pos,
    scene: SceneRef
}

impl App{
    fn new(scene: SceneRef)->Self{
        App { pos: [0,0].into(), scene }
    }
}

impl Drawable for App{
    fn set_pos(&mut self, pos: patutil::Pos) {
        self.pos = pos;
    }

    fn rect(&self) -> Rect {
        APP_RECT
    }

    fn draw(&self, render: &dyn Render) {
        render.draw_rect(Rect::new(0, 0, 100, 100), Color::new(255, 0, 0, 255), false);
        render.draw_mesh(Rect::new(100, 100, 700, 700), self.scene.clone());
    }
}


#[test]
#[ignore]
fn test_mesh_draw() {
    let mut scene: mesh::SceneRef = mesh::MeshScene::default().into();

    let mesh: MeshRef = GLMesh::new("testfiles/viking_room.obj".into()).into();

    let obj: ModelRef = mesh.into();

    // calc model matrix for mesh
    let mut model = patutil::Matf4::new();
    let mut m: nalgebra_glm::Mat4 = nalgebra_glm::identity();
    m = m.append_translation(&nalgebra_glm::vec3(0., 0., 5.));
    model.data.copy_from_slice(m.data.as_slice());
    obj.lock().set_model_mat(model);

    scene.controller().add_model(obj.clone());
    
    Window::run(APP_RECT.size(), move |window, render| -> bool {
        if let Some(app) = window.content().drawables().get(0){
            let mut camref = scene.lock().camera();
            let mut cam = camref.lock();
            cam.update(window, app.rect());
        } else {
            let drawables: Vec<Box<dyn Drawable>> = vec![Box::new(App::new(scene.clone()))];
            window.controller_content().push_drawables(drawables);
        }
        
        render.clear();
        return false;
    });
}

