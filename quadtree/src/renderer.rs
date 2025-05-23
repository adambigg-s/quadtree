use std::collections::HashMap;
use std::ffi::CString;
use std::str::FromStr;

use sokol::gfx;

use crate::compiled_shaders::circ_shader;
use crate::compiled_shaders::line_shader;
use crate::compiled_shaders::tri_shader;

#[derive(PartialEq, Eq, Hash)]
pub enum RenderPrimitive {
    Triangle,
    Circle,
    Line,
}

pub struct RenderObject {
    pub pipeline: gfx::Pipeline,
    pub bindings: gfx::Bindings,
}

pub struct Renderer {
    pub targets: HashMap<RenderPrimitive, RenderObject>,
    pub bindings: gfx::Bindings,
    pub pipeline: gfx::Pipeline,
    pub pass_action: gfx::PassAction,
}

impl Renderer {
    pub fn init_circle(&mut self) {
        #[rustfmt::skip]
        let vertices: [f32; 12] = [
            -1.0, -1.0,
             1.0, -1.0,
             1.0,  1.0,
             1.0,  1.0,
            -1.0,  1.0,
            -1.0, -1.0
        ];
        self.bindings.vertex_buffers[0] = gfx::make_buffer(&gfx::BufferDesc {
            size: size_of::<f32>() * vertices.len(),
            usage: gfx::Usage::Immutable,
            data: gfx::slice_as_range(&vertices),
            label: CString::from_str("circle vertices").unwrap().as_ptr(),
            ..Default::default()
        });
        self.pipeline = gfx::make_pipeline(&gfx::PipelineDesc {
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
        self.targets.insert(
            RenderPrimitive::Circle,
            RenderObject { pipeline: self.pipeline, bindings: self.bindings },
        );
    }

    pub fn init_line(&mut self) {
        self.bindings.vertex_buffers[0] = gfx::make_buffer(&gfx::BufferDesc {
            size: size_of::<f32>() * 2048, //can only hold 2048 lines per draw call
            usage: gfx::Usage::Stream,
            label: CString::from_str("line vertices").unwrap().as_ptr(),
            ..Default::default()
        });
        self.pipeline = gfx::make_pipeline(&gfx::PipelineDesc {
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
        self.targets
            .insert(RenderPrimitive::Line, RenderObject { pipeline: self.pipeline, bindings: self.bindings });
    }

    pub fn init_triangle(&mut self) {
        #[rustfmt::skip]
        let vertices: [f32; 15] = [
            -0.5, -0.5,   1.0, 0.7, 0.0,
             0.5, -0.5,   0.0, 1.0, 0.7,
             0.0,  0.5,   0.7, 0.0, 1.0,
        ];
        self.bindings.vertex_buffers[0] = gfx::make_buffer(&gfx::BufferDesc {
            size: size_of::<f32>() * vertices.len(),
            usage: gfx::Usage::Immutable,
            data: gfx::slice_as_range(&vertices),
            label: CString::from_str("triangle vertices").unwrap().as_ptr(),
            ..Default::default()
        });
        self.pipeline = gfx::make_pipeline(&gfx::PipelineDesc {
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
        self.targets.insert(
            RenderPrimitive::Triangle,
            RenderObject { pipeline: self.pipeline, bindings: self.bindings },
        );
    }
}
