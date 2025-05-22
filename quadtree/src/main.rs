use std::ffi::c_void;
use std::ffi::CString;
use std::str::FromStr;

use sokol::app as sapp;
use sokol::gfx;
use sokol::log as slog;

struct State {
    pipeline: gfx::Pipeline,
    bindings: gfx::Bindings,
    pass_action: gfx::PassAction,
}

extern "C" fn init(ptr: *mut c_void) {
    let state = unsafe { &mut *(ptr as *mut State) };

    gfx::setup(&gfx::Desc {
        logger: gfx::Logger { func: Some(slog::slog_func), user_data: ptr },
        ..Default::default()
    });

    #[rustfmt::skip]
    let vertices = [
        -0.5, -0.5,
        0.5, -0.5,
        0.0, 0.5,
    ];

    state.bindings.vertex_buffers[0] = gfx::make_buffer(&gfx::BufferDesc {
        size: vertices.len(),
        data: gfx::value_as_range(&vertices),
        ..Default::default()
    });
}

extern "C" fn frame(ptr: *mut c_void) {
    let state = unsafe { &mut *(ptr as *mut State) };
}

extern "C" fn event(event: *const sapp::Event, ptr: *mut c_void) {
    let _state = unsafe { &mut *(ptr as *mut State) };
    let event = unsafe { *event };

    if event.key_code == sapp::Keycode::Escape {
        sapp::request_quit();
    }
}

extern "C" fn cleanup(ptr: *mut c_void) {
    unsafe {
        if !ptr.is_null() {
            let _ = Box::from_raw(&mut *(ptr as *mut State));
        }
    }
    gfx::shutdown();
}

fn main() {
    let state = State {
        pipeline: gfx::Pipeline::new(),
        bindings: gfx::Bindings::new(),
        pass_action: gfx::PassAction::new(),
    };
    let state_ptr = Box::into_raw(Box::from(state)) as *mut c_void;

    sapp::run(&sapp::Desc {
        user_data: state_ptr,
        init_userdata_cb: Some(init),
        frame_userdata_cb: Some(frame),
        event_userdata_cb: Some(event),
        cleanup_userdata_cb: Some(cleanup),
        width: 1920,
        height: 1080,
        high_dpi: true,
        sample_count: 4,
        window_title: CString::from_str("quadtree visualization with Sokol").unwrap().as_ptr(),
        icon: sapp::IconDesc { sokol_default: true, ..Default::default() },
        logger: sapp::Logger { func: Some(slog::slog_func), user_data: state_ptr },
        ..Default::default()
    });
}
