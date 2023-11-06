use glrender::GLRender;
use iced_glutin::{glutin, winit::event_loop::EventLoopBuilder};
use glutin::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{Event, VirtualKeyCode, ElementState, MouseButton},
    event_loop::ControlFlow,
};
use patutil::{Render, Rect, Color, csg::{CSGScene, SceneRef, Brick, BrickRef}, Pos};

#[test]
#[ignore]
fn test_csg_picking() {
    let event_loop = EventLoopBuilder::with_user_event().build();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Hello world!")
        .with_inner_size(PhysicalSize::new(900, 900))
        .with_position(PhysicalPosition::new(0i32, 0i32));
    let window_context = glutin::ContextBuilder::new()
        .build_windowed(wb, &event_loop)
        .unwrap();

    let window_context = unsafe { window_context.make_current().unwrap() };
    println!(
        "Pixel format of the window's GL context: {:?}",
        window_context.get_pixel_format()
    );

    glrender::gl_init(|ptr| window_context.get_proc_address(ptr));

    let render = GLRender::new([900,900].into());

    let mut scene: SceneRef = CSGScene::default().into();
    let layer: BrickRef = Brick::new(patutil::csg::BrickType::Layer, patutil::csg::BrickOp::Union, "layer").into();
    let first_brick: BrickRef = Brick::new(patutil::csg::BrickType::Box, patutil::csg::BrickOp::Union, "box").into();
    let second_brick: BrickRef = Brick::new(patutil::csg::BrickType::Sphere, patutil::csg::BrickOp::Union, "sphere").into();
    second_brick.controller().move_pos([1.,0.,0.,0.]);
    first_brick.controller().set_next_brick(Some(second_brick));
    layer.controller().set_child(Some(first_brick));
    scene.controller().set_first(Some(layer));


    let mut mouse_pos = Pos::default();

    event_loop.run(move |event: Event<()>, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match &event {
            Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                    if input.virtual_keycode == Some(VirtualKeyCode::Escape) {
                        *control_flow = ControlFlow::Exit;
                    }
                }
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                glutin::event::WindowEvent::MouseInput { state, button, .. } =>{
                    if button == &MouseButton::Left && state == &ElementState::Pressed{
                        println!("Picking on pos: {:?}", mouse_pos);
                        let id = render.picking_csg(Rect::new(100, 100, 700, 700), scene.clone(), mouse_pos);
                        println!("Picking result: {:?}", id);
                        window_context.window().request_redraw();
                    }
                }
                glutin::event::WindowEvent::CursorMoved {  position, .. } =>{
                    mouse_pos = [position.x as i32, position.y as i32].into();
                }
                _ => {}
            },
            Event::RedrawRequested(_) => {
                render.clear();
                render.draw_rect(Rect::new(0, 0, 100, 100), Color::new(255, 0, 0, 255), false);
                render.draw_rect(Rect::new(95, 95, 710, 710), Color::new(255, 0, 0, 255), false);
                render.draw_csg(Rect::new(100, 100, 700, 700), scene.clone());
                window_context.swap_buffers().unwrap();
            }
            _ => {}
        }
    });
}
