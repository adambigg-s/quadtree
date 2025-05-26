use std::collections::HashMap;
use std::ffi::CString;
use std::str::FromStr;

use sokol::gfx;

use crate::compiled_shaders::circ_shader;
use crate::compiled_shaders::line_shader;
use crate::compiled_shaders::tri_shader;
use crate::state::State;

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Primative {
    Tri,
    Circ,
    Line,
}

#[derive(Debug)]
pub struct RenderObject {
    pub pipeline: gfx::Pipeline,
    pub bindings: gfx::Bindings,
    pub draw_elements: usize,
}

#[derive(Debug)]
pub struct Renderer {
    pub targets: HashMap<Primative, RenderObject>,
    pub bindings: gfx::Bindings,
    pub pipeline: gfx::Pipeline,
    pub pass_action: gfx::PassAction,
}

impl Renderer {
    pub fn render(&mut self, world_state: &State) {
        let Some(target) = self.targets.get(&Primative::Circ)
        else {
            panic!("target not initizlied")
        };
        gfx::apply_pipeline(target.pipeline);
        gfx::apply_bindings(&target.bindings);
        let data = circ_shader::VParamsWorld {
            world_dims: [world_state.world_dims.x, world_state.world_dims.y],
            _pad_8: [0; 8],
        };
        gfx::apply_uniforms(circ_shader::UB_V_PARAMS_WORLD, &gfx::value_as_range(&data));
        for particle in &world_state.particles {
            let data = circ_shader::VParams {
                color: [1., 1., 1.],
                _pad_12: [0; 4],
                center: [particle.pos.x, particle.pos.y],
                radius: particle.mass,
                _pad_28: [0; 4],
            };
            gfx::apply_uniforms(circ_shader::UB_V_PARAMS, &gfx::value_as_range(&data));
            gfx::draw(0, target.draw_elements, 1);
        }
    }

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
            Primative::Circ,
            RenderObject { pipeline: self.pipeline, bindings: self.bindings, draw_elements: vertices.len() },
        );
    }

    pub fn init_line(&mut self) {
        self.bindings.vertex_buffers[0] = gfx::make_buffer(&gfx::BufferDesc {
            size: size_of::<f32>() * 2048,
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
        self.targets.insert(
            Primative::Line,
            RenderObject { pipeline: self.pipeline, bindings: self.bindings, draw_elements: 2048 },
        );
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
            Primative::Tri,
            RenderObject { pipeline: self.pipeline, bindings: self.bindings, draw_elements: 3 },
        );
    }
}
