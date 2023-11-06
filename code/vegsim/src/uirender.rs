use std::sync::{Arc, Mutex};

use glow::*;
use glrender::gl;
use glutin::dpi::PhysicalPosition;
use glutin::event::{Event, ModifiersState, WindowEvent};
use glutin::event_loop::ControlFlow;
use iced_glow::glow;
use iced_glow::{Backend, Renderer, Settings, Viewport};
use iced_glutin::conversion;
use iced_glutin::glutin::{self, ContextWrapper, PossiblyCurrent};
use iced_glutin::renderer;
use iced_glutin::winit::window::Window;
use iced_glutin::{program, Clipboard, Color, Debug, Size};

use crate::controller::Controller;
use crate::ui::Controls;

pub struct UIRender {
    cursor_position: PhysicalPosition<f64>,
    modifiers: ModifiersState,
    viewport: Viewport,
    renderer: iced_glow::Renderer<iced_glow::Theme>,
    debug: Debug,
    gl: Context,
    clipboard: Clipboard,
    state: program::State<Controls>,
}

impl UIRender {
    pub fn controls(&self) -> &Controls {
        return self.state.program();
    }

    pub fn new(
        gl: Context,
        windowed_context: &ContextWrapper<PossiblyCurrent, Window>,
        controller: Arc<Mutex<Controller>>,
    ) -> Self {
        let physical_size = windowed_context.window().inner_size();
        let viewport = Viewport::with_physical_size(
            Size::new(physical_size.width, physical_size.height),
            windowed_context.window().scale_factor(),
        );

        let cursor_position = PhysicalPosition::new(-1.0, -1.0);
        let modifiers = ModifiersState::default();
        let clipboard = Clipboard::connect(windowed_context.window());

        let mut renderer = Renderer::new(Backend::new(&gl, Settings::default()));

        let mut debug = Debug::new();

        let controls = Controls::new(controller);
        let state =
            program::State::new(controls, viewport.logical_size(), &mut renderer, &mut debug);

        Self {
            cursor_position,
            modifiers,
            viewport,
            renderer,
            debug,
            gl,
            clipboard,
            state,
        }
    }

    pub fn event(
        &mut self,
        event: Event<()>,
        control_flow: &mut ControlFlow,
        windowed_context: &ContextWrapper<PossiblyCurrent, Window>,
    ) {
        match event {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CursorMoved { position, .. } => {
                        self.cursor_position = position;
                    }
                    WindowEvent::ModifiersChanged(new_modifiers) => {
                        self.modifiers = new_modifiers;
                    }
                    WindowEvent::Resized(physical_size) => {
                        self.viewport = Viewport::with_physical_size(
                            Size::new(physical_size.width, physical_size.height),
                            windowed_context.window().scale_factor(),
                        );
                    }
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => (),
                }

                // Map window event to iced event
                if let Some(event) = iced_winit::conversion::window_event(
                    &event,
                    windowed_context.window().scale_factor(),
                    self.modifiers,
                ) {
                    self.state.queue_event(event);
                }
            }
            Event::MainEventsCleared => {
                // If there are events pending
                if !self.state.is_queue_empty() {
                    // We update iced
                    let _ = self.state.update(
                        self.viewport.logical_size(),
                        conversion::cursor_position(
                            self.cursor_position,
                            self.viewport.scale_factor(),
                        ),
                        &mut self.renderer,
                        &iced_glow::Theme::Dark,
                        &renderer::Style {
                            text_color: Color::WHITE,
                        },
                        &mut self.clipboard,
                        &mut self.debug,
                    );

                    // and request a redraw
                    windowed_context.window().request_redraw();
                }
            }
            Event::RedrawRequested(_) => {
                unsafe {
                    gl::Enable(gl::BLEND);
                    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
                    gl::Disable(gl::DEPTH_TEST);
                }
                // And then iced on top
                self.renderer.with_primitives(|backend, primitive| {
                    backend.present(&self.gl, primitive, &self.viewport, &self.debug.overlay());
                });

                // Update the mouse cursor
                windowed_context.window().set_cursor_icon(
                    iced_winit::conversion::mouse_interaction(self.state.mouse_interaction()),
                );

                windowed_context.swap_buffers().unwrap();
            }
            _ => (),
        }
    }
}
