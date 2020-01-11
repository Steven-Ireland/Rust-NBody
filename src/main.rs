extern crate glutin;
extern crate gl;
extern crate nanovg;

use std::time::Instant;
use glutin::{GlContext, EventsLoop, GlWindow};
use nanovg::{Color, Context};

const INIT_WINDOW_SIZE: (u32, u32) = (1024, 720);

struct Body {
    name: String,
    x: f64,
    y: f64,
    mass: f64
}

fn unsafe_clear(width: i32, height: i32) {
    unsafe {
        gl::Viewport(0, 0, width, height);
        gl::Clear(
            gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT,
        );
    }
}

fn draw_frame(context: &Context, gl_window: &GlWindow) {
    let (width, height) = gl_window.get_inner_size().unwrap();

    unsafe_clear(width as i32, height as i32);
    context.frame((width as f32, height as f32), gl_window.hidpi_factor(), |frame| {
        // Draw red-filled rectangle.
        frame.path(
            |path| {
                path.rect((100.0, 100.0), (300.0, 300.0));
                path.fill(
                    Color::from_rgb(255, 0, 0),
                    Default::default()
                );
            },
            Default::default(),
        );
    });

    gl_window.swap_buffers().unwrap();
}

fn create_window(events_loop: &EventsLoop) -> GlWindow {
    let window = glutin::WindowBuilder::new()
        .with_title("Glutin NanoVG")
        .with_dimensions(INIT_WINDOW_SIZE.0, INIT_WINDOW_SIZE.1);
    let context = glutin::ContextBuilder::new();
    let gl_window = GlWindow::new(window, context, &events_loop).unwrap();

    unsafe {
        gl_window.make_current().unwrap();
        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    }

    gl_window
}

fn main() {
    let mut running = true;
    let mut events_loop = EventsLoop::new();
    let gl_window = createWindow(&events_loop);
    let nano_context = nanovg::ContextBuilder::new().stencil_strokes().build().expect("Initialization of NanoVG failed!");

    while running {
        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => {
                match event {
                    glutin::WindowEvent::Closed => running = false,
                    glutin::WindowEvent::Resized(w, h) => gl_window.resize(w, h),
                    _ => {}
                }
            }
            _ => {}
        });

        drawFrame(&nano_context, &gl_window);
    }
}