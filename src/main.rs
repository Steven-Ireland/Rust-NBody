mod world;

extern crate glutin;
extern crate gl;
extern crate nanovg;

use glutin::{GlContext, EventsLoop, GlWindow};
use nanovg::{Color, Context};
use world::{Body, Point, Vector};
use std::env;

const INIT_WINDOW_SIZE: (u32, u32) = (1024, 720);

fn clear_screen(width: i32, height: i32) {
    unsafe {
        gl::Viewport(0, 0, width, height);
        gl::Clear(
            gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT,
        );
    }
}

fn draw_frame(context: &Context, gl_window: &GlWindow, world_state: &Vec<Body>) {
    let (width, height) = gl_window.get_inner_size().unwrap();

    clear_screen(width as i32, height as i32);
    context.frame((width as f32, height as f32), gl_window.hidpi_factor(), |frame| {
        // Draw red-filled rectangle.
        for body in world_state {
            frame.path(
                |path| {
                    path.circle(((body.position.x + (width as f64 / 2.0)) as f32, (-1.0 * body.position.y + (height as f64 / 2.0)) as f32), (body.mass * 2.0 + 5.0) as f32);
                    path.fill(
                        Color::from_rgb(255, 0, 0),
                        Default::default()
                    );
                },
                Default::default(),
            );
        }
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

fn handle_window_events(events_loop: &mut EventsLoop, gl_window: &GlWindow) -> bool {
    let mut still_running = true;

    events_loop.poll_events(|event| match event {
        glutin::Event::WindowEvent { event, .. } => {
            match event {
                glutin::WindowEvent::Closed => still_running = false,
                glutin::WindowEvent::Resized(w, h) => gl_window.resize(w, h),
                _ => {}
            }
        }
        _ => {}
    });

    still_running
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let time_step: f64 = args[1].parse::<f64>().unwrap();


    let body2_x_coord: f64 = args[2].parse::<f64>().unwrap();
    let body2_y_coord: f64 = args[3].parse::<f64>().unwrap();
    let body2_x_velocity: f64 = args[4].parse::<f64>().unwrap();
    let body2_y_velocity: f64 = args[5].parse::<f64>().unwrap();

    let mut running = true;
    let mut events_loop = EventsLoop::new();
    let gl_window = create_window(&events_loop);
    let nano_context = nanovg::ContextBuilder::new().stencil_strokes().build().expect("Initialization of NanoVG failed!");

    let mut world_state = vec![ 
        Body {
            name: "Body A",
            position: world::ORIGIN,
            velocity: world::RESTING,
            mass: 5.0
        }, 
        Body {
            name: "Body B",
            position: Point { x: body2_x_coord, y: body2_y_coord},
            velocity: Vector { x: body2_x_velocity, y: body2_y_velocity },
            mass: 1.0
        },
        Body {
            name: "Body C",
            position: Point { x: -200.0, y: -100.0 },
            velocity: Vector { x: 0.2, y: -0.2 },
            mass: 1.0
        },
        Body {
            name: "Body D",
            position: Point { x: 100.0, y: -200.0 },
            velocity: Vector { x: 0.2, y: 0.2 },
            mass: 1.0
        }

    ];

    while running {
        running = handle_window_events(&mut events_loop, &gl_window);
        world_state = world::update_bodies(world_state, time_step);
        draw_frame(&nano_context, &gl_window, &world_state);
    }
}