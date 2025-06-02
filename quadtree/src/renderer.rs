use std::collections::HashMap;
use std::ffi::CString;
use std::str::FromStr;

use glam::Vec2;
use glam::Vec3;

use sokol::gfx;

use crate::compiled_shaders::circ_shader;
use crate::compiled_shaders::line_shader;
use crate::compiled_shaders::tri_shader;
use crate::quadtree::QuadTree;
use crate::state::State;

#[allow(dead_code)]
#[repr(C)]
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
    pub instance_size: Option<usize>,
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
            clear_value: gfx::Color { r: 0., g: 0., b: 0., ..Default::default() },
            ..Default::default()
        };
    }

    /* everything else is basically generic and can be reused for all
    primitives. this function tho is specific to n-body and should honestly be
    put inside state or pulled into another renderer but it keeps this specific
    application more organized */
    pub fn render(&mut self, state: &State) {
        'lines: {
            let Some(target) = self.render_targets.get(&RenderPrimitive::Line)
            else {
                panic!("triangle primitive not initialized")
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
            let mut instances = Vec::new();
            quad_centers(&state.quadtree, &mut instances);
            if instances.is_empty() {
                break 'lines;
            }
            gfx::update_buffer(target.bindings.vertex_buffers[0], &gfx::slice_as_range(&instances));
            gfx::draw(0, instances.len() / target.draw_elements, 1);

            fn quad_centers(quadtree: &QuadTree, data: &mut Vec<f32>) {
                traverse_recursive(quadtree, QuadTree::ROOT_INDEX, data);

                fn traverse_recursive(quadtree: &QuadTree, target_node_index: usize, data: &mut Vec<f32>) {
                    if let Some(leaf_start) = quadtree.nodes[target_node_index].leaves {
                        for leaf in leaf_start..(leaf_start + QuadTree::STEM_LEAF_COUNT) {
                            traverse_recursive(quadtree, leaf, data);
                        }

                        let node = quadtree.nodes[target_node_index];
                        let center = node.boundary.center();
                        let (min, max) = (node.boundary.min, node.boundary.max);
                        #[rustfmt::skip]
                        data.extend_from_slice(&[
                            center.x, min.y, 1., 0.7, 0.7,
                            center.x, max.y, 1., 0.7, 0.7,
                            min.x, center.y, 1., 0.7, 0.7,
                            max.x, center.y, 1., 0.7, 0.7,
                        ]);
                    }
                }
            }
        }

        'circles: {
            let Some(target) = self.render_targets.get(&RenderPrimitive::Circ)
            else {
                panic!("circle target not initizlied")
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
            let color = [0.7, 0.7, 0.7];
            let mut instances = Vec::with_capacity(state.particles.len() * 6);
            state.particles.iter().for_each(|particle| {
                instances.extend_from_slice(&[particle.position.x, particle.position.y, particle.radius]);
                instances.extend_from_slice(&color);
            });
            if instances.is_empty() {
                break 'circles;
            }
            gfx::update_buffer(target.bindings.vertex_buffers[1], &gfx::slice_as_range(&instances));
            gfx::draw(0, target.draw_elements, instances.len() / target.draw_elements);
        }

        'polygons: {
            break 'polygons;
        }
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
        let instance_size = size_of::<Vec2>() + size_of::<f32>() + size_of::<Vec3>();
        self.set_bindings.vertex_buffers[0] = gfx::make_buffer(&gfx::BufferDesc {
            size: size_of::<f32>() * vertices.len(),
            usage: gfx::Usage::Immutable,
            data: gfx::slice_as_range(&vertices),
            label: CString::from_str("circle vertices").unwrap().as_ptr(),
            ..Default::default()
        });
        self.set_bindings.vertex_buffers[1] = gfx::make_buffer(&gfx::BufferDesc {
            size: instance_size * 1_000_001, // can draw one million circles per call
            usage: gfx::Usage::Stream,
            label: CString::from_str("circle instances").unwrap().as_ptr(),
            ..Default::default()
        });
        self.set_pipeline = gfx::make_pipeline(&gfx::PipelineDesc {
            shader: gfx::make_shader(&circ_shader::circle_shader_desc(gfx::query_backend())),
            layout: {
                let mut layout = gfx::VertexLayoutState::new();

                layout.attrs[circ_shader::ATTR_CIRCLE_V_POS].format = gfx::VertexFormat::Float2;
                layout.attrs[circ_shader::ATTR_CIRCLE_V_POS].buffer_index = 0;

                layout.attrs[circ_shader::ATTR_CIRCLE_I_CENTER].format = gfx::VertexFormat::Float2;
                layout.attrs[circ_shader::ATTR_CIRCLE_I_CENTER].buffer_index = 1;
                layout.buffers[circ_shader::ATTR_CIRCLE_I_CENTER].step_func = gfx::VertexStep::PerInstance;

                layout.attrs[circ_shader::ATTR_CIRCLE_I_RADIUS].format = gfx::VertexFormat::Float;
                layout.attrs[circ_shader::ATTR_CIRCLE_I_RADIUS].buffer_index = 1;
                layout.buffers[circ_shader::ATTR_CIRCLE_I_RADIUS].step_func = gfx::VertexStep::PerInstance;

                layout.attrs[circ_shader::ATTR_CIRCLE_I_COLOR].format = gfx::VertexFormat::Float3;
                layout.attrs[circ_shader::ATTR_CIRCLE_I_COLOR].buffer_index = 1;
                layout.buffers[circ_shader::ATTR_CIRCLE_I_COLOR].step_func = gfx::VertexStep::PerInstance;

                layout
            },
            primitive_type: gfx::PrimitiveType::Triangles,
            label: CString::from_str("circle pipeline").unwrap().as_ptr(),
            ..Default::default()
        });
        self.render_targets.insert(
            RenderPrimitive::Circ,
            RenderObject {
                pipeline: self.set_pipeline,
                bindings: self.set_bindings,
                draw_elements: vertices.len() / 2,
                instance_size: Some(instance_size),
            },
        );
    }

    fn init_line(&mut self) {
        let instance_size = (size_of::<Vec2>() * 2 + size_of::<Vec3>()) * 2;
        self.set_bindings.vertex_buffers[0] = gfx::make_buffer(&gfx::BufferDesc {
            size: instance_size * 1_000_001, // can draw about 1 million lines per call
            usage: gfx::Usage::Stream,
            label: CString::from_str("line instances").unwrap().as_ptr(),
            ..Default::default()
        });
        self.set_pipeline = gfx::make_pipeline(&gfx::PipelineDesc {
            shader: gfx::make_shader(&line_shader::line_shader_desc(gfx::query_backend())),
            layout: {
                let mut layout = gfx::VertexLayoutState::new();

                layout.attrs[line_shader::ATTR_LINE_I_POS].format = gfx::VertexFormat::Float2;
                layout.attrs[line_shader::ATTR_LINE_I_POS].buffer_index = 0;
                layout.buffers[line_shader::ATTR_LINE_I_POS].step_func = gfx::VertexStep::PerVertex;

                layout.attrs[line_shader::ATTR_LINE_I_COLOR].format = gfx::VertexFormat::Float3;
                layout.attrs[line_shader::ATTR_LINE_I_COLOR].buffer_index = 0;
                layout.buffers[line_shader::ATTR_LINE_I_COLOR].step_func = gfx::VertexStep::PerVertex;

                layout
            },
            primitive_type: gfx::PrimitiveType::Lines,
            label: CString::from_str("line pipeline").unwrap().as_ptr(),
            ..Default::default()
        });
        self.render_targets.insert(
            RenderPrimitive::Line,
            RenderObject {
                pipeline: self.set_pipeline,
                bindings: self.set_bindings,
                draw_elements: 5,
                instance_size: None,
            },
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
            RenderObject {
                pipeline: self.set_pipeline,
                bindings: self.set_bindings,
                draw_elements: vertices.len() / 5,
                instance_size: None,
            },
        );
    }
}
