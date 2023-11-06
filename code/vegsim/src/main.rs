mod ui;
mod tree;
mod util;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
mod uirender;
mod controller;
mod treeparameter;
mod parameters;

use glow::*;
use glrender::window::window_state_event;
use glrender::{gl_init, GLRender};
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;
use iced_glow::glow;
use iced_glutin::glutin::{self, ContextWrapper, PossiblyCurrent};
use iced_glutin::winit::event::{ElementState, MouseButton};
use iced_glutin::winit::event_loop::EventLoop;
use iced_glutin::winit::window::Window;
use patutil::window::WindowState;
use patutil::{mesh, points, Rect, Render};

use crate::controller::Controller;
use crate::uirender::UIRender;

const APP_RECT: Rect = Rect::new(0, 0, 1100u32, 800u32);
const SCENE_RECT: Rect = patutil::Rect::new(100, 100, 700, 700);

pub fn main() {

    // create vegsim
    let scene: mesh::SceneRef = mesh::MeshScene::default().into();
    let point_scene: points::SceneRef = points::PointsScene::new(scene.lock().camera()).into();
    let treedata = Arc::new(Mutex::new(tree::TreeApp::new(scene.clone(), point_scene.clone())));

    // create controller
    let controller = Arc::new(Mutex::new(Controller::new(treedata.clone())));

    let (gl, event_loop, windowed_context) = create_window();
    let mut uirender = UIRender::new(gl, &windowed_context, controller.clone());

    let debug_texture = glrender::Texture::new(
        [100,100].into(),
        glrender::gl::RGBA,
        glrender::gl::RGBA,
        glrender::gl::UNSIGNED_BYTE,
        0 as *const _,
    );

    let mut last_update = Instant::now();
    let mut glrender = GLRender::new(APP_RECT.size());
    let mut window_state = WindowState::new(APP_RECT.size());
    const REFRESH_TIME: u64 = 10;

    event_loop.run(move |event, _, control_flow| {
        if last_update.elapsed().as_millis() as u64 > REFRESH_TIME {
            let mut camref = scene.lock().camera().clone();
            let mut cam = camref.lock();
            cam.update(&window_state, SCENE_RECT);
            window_state.controller().advance();
            last_update = Instant::now();
            windowed_context.window().request_redraw();
        }
        *control_flow = ControlFlow::WaitUntil(last_update + Duration::from_millis(REFRESH_TIME));

        window_state_event(&mut window_state, &event);

        match &event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(..) => {
                    glrender.update_screen_size(window_state.size());
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { input, .. } => {
                    if input.state == ElementState::Pressed
                        && input.virtual_keycode
                            == Some(iced_glutin::winit::event::VirtualKeyCode::F)
                    {
                        // prune by picking
                        let picking_id = glrender.mesh_picking(window_state.cursor().pos(),SCENE_RECT, scene.clone());
                        if let Some(id) = picking_id{
                            println!("Prune {}", id);
                            controller.lock().unwrap().prune_id(id);
                        }
                    }
                    if input.state == ElementState::Released
                        && input.virtual_keycode
                            == Some(iced_glutin::winit::event::VirtualKeyCode::E)
                    {
                        controller.lock().unwrap().perform_growth_iteration();
                    } else if input.state == ElementState::Released
                        && input.virtual_keycode
                            == Some(iced_glutin::winit::event::VirtualKeyCode::Escape)
                    {
                        *control_flow = ControlFlow::Exit;
                    } else if input.state == ElementState::Released
                        && input.virtual_keycode
                            == Some(iced_glutin::winit::event::VirtualKeyCode::Q)
                    {
                        println!("Saving image");
                        {
                            let program = uirender.controls();
                            glrender.clear();
                            draw_scene(&glrender, &scene, &point_scene, &debug_texture, program.show_markers());
                            glrender::FrameBuffer::save_viewport("output.png".to_string()).unwrap();
                        }
                        println!("Image saved");
                    }
                }
                WindowEvent::MouseInput { state, button, .. }=>{
                    if *button == MouseButton::Right && *state == ElementState::Pressed{
                        let picking_id = glrender.mesh_picking(window_state.cursor().pos(),patutil::Rect::new(100, 100, 700, 700), scene.clone());
                        controller.lock().unwrap().update_selected(picking_id);
                    }
                }
                _ => (),
            },
            Event::RedrawRequested(_) => {
                // update debug texture
                let program = uirender.controls();
                let index = program.index();
                debug_texture.update_data(treedata.lock().unwrap().debug_texture(index).as_ptr() as *const _);

                // redraw
                glrender.clear();
                draw_scene(&glrender, &scene, &point_scene, &debug_texture, program.show_markers());
            }
            _ => (),
        }

        // vegsim.set_environment_mode(program.mode());
        uirender.event(event, control_flow, &windowed_context);
    });
}

fn create_window() -> (
    Context,
    EventLoop<()>,
    ContextWrapper<PossiblyCurrent, Window>,
) {
    let el = glutin::event_loop::EventLoop::new();

    let wb = glutin::window::WindowBuilder::new()
        .with_title("Window title")
        .with_inner_size(glutin::dpi::LogicalSize::new(APP_RECT.w + 100, APP_RECT.h + 100))
        .with_position(glutin::dpi::LogicalPosition::new(0,0));

    let windowed_context = glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4)
        .build_windowed(wb, &el)
        .unwrap();

    unsafe {
        let windowed_context = windowed_context.make_current().unwrap();
        gl_init(|ptr| windowed_context.get_proc_address(ptr));

        let gl = glow::Context::from_loader_function(|s| {
            windowed_context.get_proc_address(s) as *const _
        });

        // Enable auto-conversion from/to sRGB
        gl.enable(glow::FRAMEBUFFER_SRGB);

        // Enable alpha blending
        gl.enable(glow::BLEND);
        gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);

        // Disable multisampling by default
        gl.disable(glow::MULTISAMPLE);

        return (gl, el, windowed_context);
    }
}

fn draw_scene(
    glrender: &GLRender,
    scene: &mesh::SceneRef,
    point_scene: &points::SceneRef,
    debug_texture: &glrender::Texture,
    draw_points: bool
) {
    glrender.draw_rect(
        ([95, 95].into(), patutil::Size::new(710,710)).into(),
        patutil::Color::new(10, 10, 10, 255),
        false,
    );
    glrender.draw_rect(
        ([100, 100].into(), patutil::Size::new(700,700)).into(),
        patutil::Color::new(255, 255, 255, 255),
        false,
    );
    glrender.draw_mesh(patutil::Rect::new(100, 100, 700, 700), scene.clone());
    if draw_points{
        glrender.draw_points(patutil::Rect::new(100, 100, 700, 700), point_scene.clone());
    }

    let debug_size: patutil::Size = [100,100].into();
    glrender.draw_rect(
        ([0, 0].into(), debug_size + [5,5].into()).into(),
        patutil::Color::new(255, 0, 0, 255),
        false,
    );
    glrender.draw_texture(([0, 0].into(), debug_size).into(), debug_texture, false);
}