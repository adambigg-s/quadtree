mod compiled_shaders;
mod quadtree;
mod renderer;
mod state;
mod utils;

use std::collections::HashMap;
use std::ffi::c_void;
use std::ffi::CString;
use std::str::FromStr;

use glam::Vec2;
use sokol::app as sapp;
use sokol::gfx;
use sokol::glue as sgl;
use sokol::log as slog;

use renderer::Renderer;
use state::State;
use utils::random_vec2;

extern "C" fn init(ptr: *mut c_void) {
    let state = unsafe { &mut *(ptr as *mut ApplicationState) };

    gfx::setup(&gfx::Desc {
        environment: sgl::environment(),
        logger: gfx::Logger { func: Some(slog::slog_func), user_data: ptr },
        ..Default::default()
    });

    state.renderer.init_triangle();
    state.renderer.init_line();
    state.renderer.init_circle();

    state.renderer.pass_action.colors[0] = gfx::ColorAttachmentAction {
        load_action: gfx::LoadAction::Clear,
        clear_value: gfx::Color { r: 0.2, g: 0.2, b: 0.4, a: 1. },
        ..Default::default()
    };
}

extern "C" fn frame(ptr: *mut c_void) {
    let state = unsafe { &mut *(ptr as *mut ApplicationState) };

    state.update(sapp::widthf(), sapp::heightf());

    gfx::begin_pass(&gfx::Pass {
        action: state.renderer.pass_action,
        swapchain: sgl::swapchain(),
        ..Default::default()
    });
    state.renderer.render(&state.state);
    gfx::end_pass();
    gfx::commit();
}

extern "C" fn event(event: *const sapp::Event, ptr: *mut c_void) {
    let (_state, event) = unsafe { (&mut *(ptr as *mut ApplicationState), *event) };

    if event.key_code == sapp::Keycode::Escape {
        sapp::request_quit();
    }
}

#[allow(unused_must_use)]
extern "C" fn cleanup(ptr: *mut c_void) {
    gfx::shutdown();
    if ptr.is_null() {
        return;
    }
    let state = unsafe { &mut *(ptr as *mut ApplicationState) };
    println!("full state: {:?}", state);
    unsafe { Box::from_raw(&mut *(ptr as *mut ApplicationState)) };
}

#[derive(Debug)]
struct ApplicationState {
    renderer: Renderer,
    state: State,
}

impl ApplicationState {
    fn update(&mut self, width: f32, height: f32) {
        self.state.update(width, height);
    }
}

fn main() {
    let mut state = ApplicationState {
        renderer: Renderer {
            targets: HashMap::new(),
            bindings: gfx::Bindings::new(),
            pipeline: gfx::Pipeline::new(),
            pass_action: gfx::PassAction::new(),
        },
        state: State::build(800, 600),
    };
    for _ in 0..100 {
        state.state.add_particle(3.);
        let random = random_vec2(Vec2::new(20., 20.));
        println!("vec generated: {}", random);
    }
    let state_ptr = Box::into_raw(Box::from(state)) as *mut c_void;

    sapp::run(&sapp::Desc {
        user_data: state_ptr,
        init_userdata_cb: Some(init),
        frame_userdata_cb: Some(frame),
        event_userdata_cb: Some(event),
        cleanup_userdata_cb: Some(cleanup),
        width: 800,
        height: 600,
        high_dpi: true,
        sample_count: 4,
        window_title: CString::from_str("quadtree visualization with Sokol").unwrap().as_ptr(),
        icon: sapp::IconDesc { sokol_default: true, ..Default::default() },
        logger: sapp::Logger { func: Some(slog::slog_func), user_data: state_ptr },
        ..Default::default()
    });
}
