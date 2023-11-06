
use glrender::{window::Window, GLPointsList};
use patutil::{Render, Rect, Color, points::{self, SceneRef}, Drawable, Vecf3};

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
        render.draw_points(Rect::new(100, 100, 700, 700), self.scene.clone());
    }
}


#[test]
#[ignore]
fn test_mesh_draw() {
    let mut scene: points::SceneRef = points::PointsScene::default().into();

    let mut points = vec![];
    for x in 0..100{
        for y in 0..100{
            points.push(points::Point{
                pos: Vecf3::new(x as f32, y as f32, 3.), 
                color: Color::new(100+x, 100 + y, 0, 255),
                size: 10.
            });
        }
    }
    let pointslist: points::PointsListRef = GLPointsList::new(points).into();
    scene.controller().add_list(pointslist);
    
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

