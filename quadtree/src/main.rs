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

use renderer::PrimitiveRenderer;
use state::Particle;
use state::State;

extern "C" fn init(ptr: *mut c_void) {
    let state = unsafe { &mut *(ptr as *mut ApplicationState) };

    state.state.init();

    gfx::setup(&gfx::Desc {
        environment: sgl::environment(),
        logger: gfx::Logger { func: Some(slog::slog_func), user_data: ptr },
        ..Default::default()
    });

    state.renderer.init_primitives();
    state.renderer.init_pass_action();
}

extern "C" fn frame(ptr: *mut c_void) {
    let state = unsafe { &mut *(ptr as *mut ApplicationState) };

    state.update();

    gfx::begin_pass(&gfx::Pass {
        action: state.renderer.set_pass_action,
        swapchain: sgl::swapchain(),
        ..Default::default()
    });
    state.renderer.render(&state.state);
    gfx::end_pass();
    gfx::commit();
}

extern "C" fn event(event: *const sapp::Event, ptr: *mut c_void) {
    let (state, event) = unsafe { (&mut *(ptr as *mut ApplicationState), *event) };

    if event.key_code == sapp::Keycode::Escape {
        sapp::request_quit();
    }
    if event._type == sapp::EventType::Resized {
        state.state.update_dimensions(sapp::widthf(), sapp::heightf());
    }
    if event.mouse_button == sapp::Mousebutton::Left {
        state.state.particles.push(Particle {
            position: {
                let (mx, my) = (event.mouse_x, sapp::heightf() - event.mouse_y);
                Vec2::new(mx, my)
            },
            velocity: Vec2::ZERO,
            mass: 3.,
        });
    }
    if event.key_code == sapp::Keycode::R {
        state.state.particles.clear();
    }
}

#[allow(unused_must_use)]
extern "C" fn cleanup(ptr: *mut c_void) {
    gfx::shutdown();
    if ptr.is_null() {
        return;
    }
    unsafe { Box::from_raw(&mut *(ptr as *mut ApplicationState)) };
}

#[repr(C)]
#[derive(Debug)]
struct ApplicationState {
    renderer: PrimitiveRenderer,
    state: State,
}

impl ApplicationState {
    fn update(&mut self) {
        self.state.update(0.01);
    }
}

fn main() {
    let state = ApplicationState {
        renderer: PrimitiveRenderer {
            render_targets: HashMap::new(),
            set_bindings: gfx::Bindings::new(),
            set_pipeline: gfx::Pipeline::new(),
            set_pass_action: gfx::PassAction::new(),
        },
        state: State::build(800, 600),
    };

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
