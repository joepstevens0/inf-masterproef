use std::{
    time::{Duration, Instant},
};

use crate::GLRender;
use glutin::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{Event, VirtualKeyCode, WindowEvent},
};

use iced_glutin::{glutin, winit::event_loop::EventLoopBuilder};
use patutil::{Render, Size};

use patutil::window::{CursorIcon, WindowState};

type WindowContext = glutin::WindowedContext<glutin::PossiblyCurrent>;
type EventLoop = glutin::event_loop::EventLoop<()>;
type ControlFlow = glutin::event_loop::ControlFlow;

fn build_window(screen_size: Size) -> (WindowContext, EventLoop) {
    let event_loop = EventLoopBuilder::with_user_event().build();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Hello world!")
        .with_inner_size(PhysicalSize::new(screen_size.w, screen_size.h))
        .with_position( PhysicalPosition::new(0i32, 0i32));
    let window_context = glutin::ContextBuilder::new()
        .build_windowed(wb, &event_loop)
        .unwrap();

    let window_context = unsafe { window_context.make_current().unwrap() };
    println!(
        "Pixel format of the window's GL context: {:?}",
        window_context.get_pixel_format()
    );

    crate::gl_init(|ptr| window_context.get_proc_address(ptr));

    return (window_context, event_loop);
}

pub fn window_state_event(window_state: &mut WindowState, event: &Event<()>){
    match &event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::KeyboardInput {
                device_id: _,
                input,
                is_synthetic: _,
            } => {
                // exit on escape
                if let Some(key) = input.virtual_keycode {
                    match input.state {
                        glutin::event::ElementState::Pressed => {
                            window_state.controller_keyboard().press(key);
                        }
                        glutin::event::ElementState::Released => {
                            window_state.controller_keyboard().release(key);
                        }
                    }
                }
            }
            WindowEvent::Moved(pos) => {
                window_state.controller().set_window_pos([pos.x, pos.y].into());
            }
            WindowEvent::Resized(size) => {
                window_state
                    .controller()
                    .set_window_size([size.width, size.height].into());
            }
            WindowEvent::CursorMoved { position, .. } => {
                window_state
                    .controller_cursor()
                    .pos_update([position.x as i32, position.y as i32].into());
            }
            WindowEvent::MouseInput { state, button, .. } => match state {
                glutin::event::ElementState::Pressed => {
                    window_state.controller_cursor().press(*button);
                }
                glutin::event::ElementState::Released => {
                    window_state.controller_cursor().release(*button);
                }
            },
            _ => (),
        }
        _ => (),
    }
}

pub struct Window {}

impl Window {
    pub fn run(size: impl Into<Size>, mut builder: impl FnMut(&mut WindowState, &mut dyn Render) -> bool + 'static) -> ! {
        let size = size.into();

        let mut window = WindowState::new(size);
        let (window_context, event_loop): (WindowContext, EventLoop) = build_window(size);
        let mut render = GLRender::new(size);

        const REFRESH_TIME: u64 = 10;

        let mut last_cursor_icon = CursorIcon::default();

        let mut last_redraw = Instant::now();
        event_loop.run(move |event, _, control_flow| {
            if last_redraw.elapsed().as_millis() as u64 > REFRESH_TIME{
                while builder(&mut window, &mut render) {}
                // advance state
                window.controller().advance();
                
                window_context.window().request_redraw();
                last_redraw = Instant::now();
            }

            *control_flow = ControlFlow::WaitUntil(last_redraw + Duration::from_millis(REFRESH_TIME));

            window_state_event(&mut window, &event);

            match &event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput {
                        device_id: _,
                        input,
                        is_synthetic: _,
                    } => {
                        // exit on escape
                        if let Some(key) = input.virtual_keycode {
                            if key == VirtualKeyCode::Escape {
                                *control_flow = ControlFlow::Exit;
                            }
                        }
                    }
                    WindowEvent::Resized(size) => {
                        window_context.resize(*size);
                        render.update_screen_size([size.width, size.height].into());
                    }
                    _ => (),
                },
                Event::RedrawRequested(_) => {
                    Window::draw_content(&window, &render);
                    window_context.swap_buffers().unwrap();
                }
                Event::MainEventsCleared => {
                    // set cursor icon if changed
                    if last_cursor_icon != window.cursor().icon() {
                        window_context
                            .window()
                            .set_cursor_icon(window.cursor().icon());
                        last_cursor_icon = window.cursor().icon();
                    }

                    // set window position if not maximized
                    if !window_context.window().is_maximized() {
                        window_context
                            .window()
                            .set_outer_position(PhysicalPosition::new(
                                window.pos().x,
                                window.pos().y,
                            ));
                    } else{
                        window.controller().set_window_pos([0,0].into());
                    }
                }
                _ => (),
            }
        });
    }

    fn draw_content(state: &WindowState, render: &dyn Render) {
        // clear screen
        render.clear();

        for d in state.content().drawables() {
            d.draw(render);
        }
    }
}
