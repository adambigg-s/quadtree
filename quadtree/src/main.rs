mod compiled_shaders;
mod quadtree;
mod state;

use std::collections::HashMap;
use std::ffi::c_void;
use std::ffi::CString;
use std::str::FromStr;

use compiled_shaders::circ_shader;
use compiled_shaders::line_shader;
use compiled_shaders::tri_shader;
use sokol::app as sapp;
use sokol::gfx;
use sokol::glue as sgl;
use sokol::log as slog;

use state::State;

#[derive(PartialEq, Eq, Hash)]
enum RenderPrimitive {
    Triangle,
    Circle,
    Line,
}

struct RenderObject {
    pipeline: gfx::Pipeline,
    bindings: gfx::Bindings,
}

struct Renderer {
    targets: HashMap<RenderPrimitive, RenderObject>,
    bindings: gfx::Bindings,
    pipeline: gfx::Pipeline,
    pass_action: gfx::PassAction,
}

struct ApplicationState {
    renderer: Renderer,
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

    // triangle primitive pipeline
    #[rustfmt::skip]
    let vertices: [f32; 15] = [
        -0.5, -0.5,   1.0, 0.7, 0.0,
         0.5, -0.5,   0.0, 1.0, 0.7,
         0.0,  0.5,   0.7, 0.0, 1.0,
    ];
    state.renderer.bindings.vertex_buffers[0] = gfx::make_buffer(&gfx::BufferDesc {
        size: size_of::<f32>() * vertices.len(),
        usage: gfx::Usage::Immutable,
        data: gfx::slice_as_range(&vertices),
        label: CString::from_str("triangle vertices").unwrap().as_ptr(),
        ..Default::default()
    });
    state.renderer.pipeline = gfx::make_pipeline(&gfx::PipelineDesc {
        shader: gfx::make_shader(&tri_shader::simple_shader_desc(gfx::query_backend())),
        layout: {
            let mut layout = gfx::VertexLayoutState::new();
            layout.attrs[tri_shader::ATTR_SIMPLE_V_POS].format = gfx::VertexFormat::Float2;
            layout.attrs[tri_shader::ATTR_SIMPLE_V_COLOR].format = gfx::VertexFormat::Float3;
            layout
        },
        primitive_type: gfx::PrimitiveType::Triangles,
        label: CString::from_str("triangle pipeline").unwrap().as_ptr(),
        ..Default::default()
    });
    state.renderer.targets.insert(
        RenderPrimitive::Triangle,
        RenderObject { pipeline: state.renderer.pipeline, bindings: state.renderer.bindings },
    );

    // line primitive pipeline
    state.renderer.bindings.vertex_buffers[0] = gfx::make_buffer(&gfx::BufferDesc {
        size: size_of::<f32>() * 2048, //can only hold 2048 lines per draw call
        usage: gfx::Usage::Stream,
        label: CString::from_str("line vertices").unwrap().as_ptr(),
        ..Default::default()
    });
    state.renderer.pipeline = gfx::make_pipeline(&gfx::PipelineDesc {
        shader: gfx::make_shader(&line_shader::line_shader_desc(gfx::query_backend())),
        layout: {
            let mut layout = gfx::VertexLayoutState::new();
            layout.attrs[line_shader::ATTR_LINE_V_POS].format = gfx::VertexFormat::Float2;
            layout.attrs[line_shader::ATTR_LINE_V_COLOR].format = gfx::VertexFormat::Float3;
            layout
        },
        primitive_type: gfx::PrimitiveType::Lines,
        label: CString::from_str("line pipeline").unwrap().as_ptr(),
        ..Default::default()
    });
    state.renderer.targets.insert(
        RenderPrimitive::Line,
        RenderObject { pipeline: state.renderer.pipeline, bindings: state.renderer.bindings },
    );

    // circle render primitive pipeline
    #[rustfmt::skip]
    let vertices: [f32; 12] = [
        -1.0, -1.0,
         1.0, -1.0,
         1.0,  1.0,
         1.0,  1.0,
        -1.0,  1.0,
        -1.0, -1.0
    ];
    state.renderer.bindings.vertex_buffers[0] = gfx::make_buffer(&gfx::BufferDesc {
        size: size_of::<f32>() * vertices.len(),
        usage: gfx::Usage::Immutable,
        data: gfx::slice_as_range(&vertices),
        label: CString::from_str("circle vertices").unwrap().as_ptr(),
        ..Default::default()
    });
    state.renderer.pipeline = gfx::make_pipeline(&gfx::PipelineDesc {
        shader: gfx::make_shader(&circ_shader::circle_shader_desc(gfx::query_backend())),
        layout: {
            let mut layout = gfx::VertexLayoutState::new();
            layout.attrs[circ_shader::ATTR_CIRCLE_V_POS].format = gfx::VertexFormat::Float2;
            layout
        },
        primitive_type: gfx::PrimitiveType::Triangles,
        label: CString::from_str("circle pipeline").unwrap().as_ptr(),
        ..Default::default()
    });
    state.renderer.targets.insert(
        RenderPrimitive::Circle,
        RenderObject { pipeline: state.renderer.pipeline, bindings: state.renderer.bindings },
    );

    // pass action - can be just static for now
    state.renderer.pass_action.colors[0] = gfx::ColorAttachmentAction {
        load_action: gfx::LoadAction::Clear,
        clear_value: gfx::Color { r: 0.2, g: 0.2, b: 0.4, a: 1. },
        ..Default::default()
    };
}

extern "C" fn frame(ptr: *mut c_void) {
    let state = unsafe { &mut *(ptr as *mut ApplicationState) };

    state.update();

    gfx::begin_pass(&gfx::Pass {
        action: state.renderer.pass_action,
        swapchain: sgl::swapchain(),
        ..Default::default()
    });

    let target = state.renderer.targets.get(&RenderPrimitive::Circle).unwrap();

    let params = circ_shader::VParams {
        color: [1., 1., 1.],
        _pad_12: [0; 4],
        center: [0., 0.],
        radius: 0.5,
        _pad_28: [0; 4],
    };

    gfx::apply_pipeline(target.pipeline);
    gfx::apply_bindings(&target.bindings);
    gfx::apply_uniforms(circ_shader::UB_V_PARAMS, &gfx::value_as_range(&params));
    gfx::draw(0, 6, 1);

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
    unsafe { Box::from_raw(&mut *(ptr as *mut ApplicationState)) };
}

fn main() {
    let state = ApplicationState {
        renderer: Renderer {
            targets: HashMap::new(),
            bindings: gfx::Bindings::new(),
            pipeline: gfx::Pipeline::new(),
            pass_action: gfx::PassAction::new(),
        },
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
