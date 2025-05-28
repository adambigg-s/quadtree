use std::collections::HashMap;
use std::ffi::CString;
use std::str::FromStr;

use sokol::gfx;

use crate::compiled_shaders::circ_shader;
use crate::compiled_shaders::line_shader;
use crate::compiled_shaders::tri_shader;
use crate::state::State;

#[allow(dead_code)]
#[derive(PartialEq, Eq, Hash, Debug)]
pub enum RenderPrimitive {
    Tri,
    Circ,
    Line,
    Mesh,
    EnumLength,
}

#[repr(C)]
#[derive(Debug)]
pub struct RenderObject {
    pub pipeline: gfx::Pipeline,
    pub bindings: gfx::Bindings,
    pub draw_elements: usize,
}

#[repr(C)]
#[derive(Debug)]
pub struct PrimitiveRenderer {
    pub render_targets: HashMap<RenderPrimitive, RenderObject>,
    pub set_pipeline: gfx::Pipeline,
    pub set_bindings: gfx::Bindings,
    pub set_pass_action: gfx::PassAction,
}

impl PrimitiveRenderer {
    pub fn init_primitives(&mut self) {
        self.init_triangle();
        self.init_circle();
        self.init_line();
    }

    pub fn init_pass_action(&mut self) {
        self.set_pass_action.colors[0] = gfx::ColorAttachmentAction {
            load_action: gfx::LoadAction::Clear,
            clear_value: gfx::Color { r: 0.2, g: 0.2, b: 0.4, a: 1. },
            ..Default::default()
        };
    }

    pub fn render(&mut self, state: &State) {
        let Some(target) = self.render_targets.get(&RenderPrimitive::Circ)
        else {
            panic!("target not initizlied")
        };
        gfx::apply_pipeline(target.pipeline);
        gfx::apply_bindings(&target.bindings);
        gfx::apply_uniforms(
            circ_shader::UB_V_PARAMS_WORLD,
            &gfx::value_as_range(&circ_shader::VParamsWorld {
                world_dims: [state.dimensions.width(), state.dimensions.height()],
                _pad_8: [0; 8],
            }),
        );
        state.particles.iter().for_each(|particle| {
            gfx::apply_uniforms(
                circ_shader::UB_V_PARAMS,
                &gfx::value_as_range(&circ_shader::VParams {
                    color: [1., 1., 1.],
                    _pad_12: [0; 4],
                    center: [particle.position.x, particle.position.y],
                    radius: particle.mass,
                    _pad_28: [0; 4],
                }),
            );
            gfx::draw(0, target.draw_elements, 1);
        });

        let Some(target) = self.render_targets.get(&RenderPrimitive::Line)
        else {
            panic!("target not initialized")
        };
        gfx::apply_pipeline(target.pipeline);
        gfx::apply_bindings(&target.bindings);
        gfx::apply_uniforms(
            line_shader::UB_V_PARAMS_WORLD,
            &gfx::value_as_range(&line_shader::VParamsWorld {
                world_dims: [state.dimensions.width(), state.dimensions.height()],
                _pad_8: [0; 8],
            }),
        );
    }

    fn init_circle(&mut self) {
        #[rustfmt::skip]
        let vertices: [f32; 12] = [
            -1.0, -1.0,
             1.0, -1.0,
             1.0,  1.0,
             1.0,  1.0,
            -1.0,  1.0,
            -1.0, -1.0
        ];
        self.set_bindings.vertex_buffers[0] = gfx::make_buffer(&gfx::BufferDesc {
            size: size_of::<f32>() * vertices.len(),
            usage: gfx::Usage::Immutable,
            data: gfx::slice_as_range(&vertices),
            label: CString::from_str("circle vertices").unwrap().as_ptr(),
            ..Default::default()
        });
        self.set_pipeline = gfx::make_pipeline(&gfx::PipelineDesc {
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
        self.render_targets.insert(
            RenderPrimitive::Circ,
            RenderObject { pipeline: self.set_pipeline, bindings: self.set_bindings, draw_elements: 6 },
        );
    }

    fn init_line(&mut self) {
        self.set_bindings.vertex_buffers[0] = gfx::make_buffer(&gfx::BufferDesc {
            size: size_of::<f32>() * 2048,
            usage: gfx::Usage::Stream,
            label: CString::from_str("line vertices").unwrap().as_ptr(),
            ..Default::default()
        });
        self.set_pipeline = gfx::make_pipeline(&gfx::PipelineDesc {
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
        self.render_targets.insert(
            RenderPrimitive::Line,
            RenderObject { pipeline: self.set_pipeline, bindings: self.set_bindings, draw_elements: 2048 },
        );
    }

    fn init_triangle(&mut self) {
        #[rustfmt::skip]
        let vertices: [f32; 15] = [
            -0.5, -0.5,   1.0, 0.7, 0.0,
             0.5, -0.5,   0.0, 1.0, 0.7,
             0.0,  0.5,   0.7, 0.0, 1.0,
        ];
        self.set_bindings.vertex_buffers[0] = gfx::make_buffer(&gfx::BufferDesc {
            size: size_of::<f32>() * vertices.len(),
            usage: gfx::Usage::Immutable,
            data: gfx::slice_as_range(&vertices),
            label: CString::from_str("triangle vertices").unwrap().as_ptr(),
            ..Default::default()
        });
        self.set_pipeline = gfx::make_pipeline(&gfx::PipelineDesc {
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
        self.render_targets.insert(
            RenderPrimitive::Tri,
            RenderObject { pipeline: self.set_pipeline, bindings: self.set_bindings, draw_elements: 3 },
        );
    }
}
