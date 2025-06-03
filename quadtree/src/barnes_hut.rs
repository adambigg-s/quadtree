use glam::Vec2;

use crate::quadtree::QuadTree;
use crate::state::Particle;

pub struct BarnesHutNodeData {
    pub mass: f32,
    pub mass_center: Vec2,
}

impl BarnesHutNodeData {
    pub fn build(mass: f32, mass_center: Vec2) -> Self {
        BarnesHutNodeData { mass, mass_center }
    }
}

pub struct BarnesHutWrapper<'d> {
    pub inner_tree: Option<&'d QuadTree>,
    pub particles: Option<&'d [Particle]>,
    pub barnes_hut_data: Vec<Option<BarnesHutNodeData>>,
}

impl<'d> BarnesHutWrapper<'d> {
    pub fn new() -> BarnesHutWrapper<'d> {
        BarnesHutWrapper { inner_tree: None, particles: None, barnes_hut_data: Vec::new() }
    }

    pub fn plant_tree(&mut self, tree: &'d QuadTree, particles: &'d [Particle]) {
        self.inner_tree = Some(tree);
        self.particles = Some(particles);
        self.barnes_hut_data.clear();
        self.barnes_hut_data.resize_with(tree.nodes.len(), || None);
    }

    pub fn build_hierarchy(&mut self) {
        let Some(tree) = self.inner_tree
        else {
            println!("no tree initialized into barnes hut");
            return;
        };
        let Some(particles) = self.particles
        else {
            println!("no particles initialized into barnes hut");
            return;
        };

        self.recursive_builder(QuadTree::ROOT_INDEX, tree, particles);
    }

    fn recursive_builder(
        &mut self, target_node_index: usize, tree: &QuadTree, particles: &[Particle],
    ) -> BarnesHutNodeData {
        let mut mass = 0.;
        let mut position = Vec2::ZERO;

        if let Some(leaf_start) = tree.nodes[target_node_index].leaves {
            for leaf in leaf_start..(leaf_start + QuadTree::STEM_LEAF_COUNT) {
                let data = self.recursive_builder(leaf, tree, particles);
                mass += data.mass;
                position += data.mass_center;
            }
            return BarnesHutNodeData::build(mass, position);
        }
        // if we are dealing with a stem, EXIT
        {
            debug_assert!(tree.nodes[target_node_index].leaves.is_none());
        }

        // guarenteed to be a leaf here
        if let Some(list_node_index) = tree.nodes[target_node_index].data_head {
            for &particle_index in &tree.node_pointers[list_node_index] {
                mass += particles[particle_index].mass;
                position += particles[particle_index].position;
            }
        }
        self.barnes_hut_data[target_node_index] = Some(BarnesHutNodeData::build(mass, position));

        BarnesHutNodeData::build(mass, position)
    }
}
