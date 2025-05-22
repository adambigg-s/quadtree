mod quadtree;
mod shader;
mod state;

use std::ffi::c_void;
use std::ffi::CString;
use std::str::FromStr;

use sokol::app as sapp;
use sokol::gfx;
use sokol::glue as sgl;
use sokol::log as slog;

use state::State;

struct ApplicationState {
    pipeline: gfx::Pipeline,
    bindings: gfx::Bindings,
    pass_action: gfx::PassAction,

    state: State,
}

impl ApplicationState {
    fn update(&mut self) {
        self.state.update();
    }
}

extern "C" fn init(ptr: *mut c_void) {
    let state = unsafe { &mut *(ptr as *mut ApplicationState) };

    gfx::setup(&gfx::Desc {
        environment: sgl::environment(),
        logger: gfx::Logger { func: Some(slog::slog_func), user_data: ptr },
        ..Default::default()
    });

    #[rustfmt::skip]
    let vertices: [f32; 15] = [
        -0.5, -0.5,   1.0, 0.7, 0.0,
         0.5, -0.5,   0.0, 1.0, 0.7,
         0.0,  0.5,   0.7, 0.0, 1.0,
    ];
    state.bindings.vertex_buffers[0] = gfx::make_buffer(&gfx::BufferDesc {
        size: size_of_val(&vertices),
        data: gfx::slice_as_range(&vertices),
        label: CString::from_str("triangle vertices").unwrap().as_ptr(),
        ..Default::default()
    });

    state.pipeline = gfx::make_pipeline(&gfx::PipelineDesc {
        shader: gfx::make_shader(&shader::simple_shader_desc(gfx::query_backend())),
        layout: {
            let mut layout = gfx::VertexLayoutState::new();
            layout.attrs[shader::ATTR_SIMPLE_V_POS].format = gfx::VertexFormat::Float2;
            layout.attrs[shader::ATTR_SIMPLE_V_COLOR].format = gfx::VertexFormat::Float3;
            layout
        },
        label: CString::from_str("triangle pipeline").unwrap().as_ptr(),
        ..Default::default()
    });

    state.pass_action.colors[0] = gfx::ColorAttachmentAction {
        load_action: gfx::LoadAction::Clear,
        clear_value: gfx::Color { r: 0.2, g: 0.2, b: 0.4, a: 1. },
        ..Default::default()
    };
}

extern "C" fn frame(ptr: *mut c_void) {
    let state = unsafe { &mut *(ptr as *mut ApplicationState) };

    state.update();

    gfx::begin_pass(&gfx::Pass {
        action: state.pass_action,
        swapchain: sgl::swapchain(),
        ..Default::default()
    });
    gfx::apply_pipeline(state.pipeline);
    gfx::apply_bindings(&state.bindings);
    gfx::draw(0, 3, 1);
    gfx::end_pass();
    gfx::commit();
}

extern "C" fn event(event: *const sapp::Event, ptr: *mut c_void) {
    let _state = unsafe { &mut *(ptr as *mut ApplicationState) };
    let event = unsafe { *event };

    if event.key_code == sapp::Keycode::Escape {
        sapp::request_quit();
    }
}

#[allow(unused_must_use)]
extern "C" fn cleanup(ptr: *mut c_void) {
    if !ptr.is_null() {
        unsafe { Box::from_raw(&mut *(ptr as *mut ApplicationState)) };
    }
    gfx::shutdown();
}

fn main() {
    let state = ApplicationState {
        pipeline: gfx::Pipeline::new(),
        bindings: gfx::Bindings::new(),
        pass_action: gfx::PassAction::new(),
        state: State {},
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
