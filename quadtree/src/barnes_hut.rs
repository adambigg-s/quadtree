use glam::Vec2;

use crate::quadtree::QuadTree;
use crate::state::Particle;
use crate::utils::EPSILON;

#[derive(Clone, Copy)]
pub struct BarnesHutNode {
    pub mass: f32,
    pub mass_center: Vec2,
}

impl BarnesHutNode {
    pub fn build(mass: f32, mass_center: Vec2) -> Self {
        BarnesHutNode { mass, mass_center }
    }
}

pub struct BarnesHutWrapper {
    pub barnes_hut_data: Vec<Option<BarnesHutNode>>,
}

impl BarnesHutWrapper {
    pub fn new() -> BarnesHutWrapper {
        BarnesHutWrapper { barnes_hut_data: Vec::new() }
    }

    pub fn build_hierarchy(&mut self, tree: &QuadTree, particles: &[Particle]) {
        self.init_vector(tree);
        self.builder(QuadTree::ROOT_INDEX, tree, particles);
    }

    fn builder(&mut self, target_index: usize, tree: &QuadTree, particles: &[Particle]) -> BarnesHutNode {
        let mut mass = 0.;
        let mut mass_averaged_position = Vec2::ZERO;

        if let Some(leaf_start) = tree.nodes[target_index].leaves {
            (leaf_start..(leaf_start + QuadTree::STEM_LEAF_COUNT)).for_each(|leaf| {
                let leaf_data = self.builder(leaf, tree, particles);
                mass += leaf_data.mass;
                mass_averaged_position += leaf_data.mass_center * leaf_data.mass;
            });
        }
        else if let Some(list_node_index) = tree.nodes[target_index].data_head {
            tree.node_pointers[list_node_index].iter().for_each(|&particle_index| {
                let particle = &particles[particle_index];
                mass += particle.mass;
                mass_averaged_position += particle.position * particle.mass;
            });
        }

        self.update_node(target_index, mass, mass_averaged_position)
    }

    fn init_vector(&mut self, tree: &QuadTree) {
        self.barnes_hut_data.resize_with(tree.nodes.len(), || None);
    }

    fn update_node(&mut self, target_index: usize, mass: f32, mut mass_center: Vec2) -> BarnesHutNode {
        if mass > EPSILON {
            mass_center /= mass;
        }

        let node = BarnesHutNode::build(mass, mass_center);
        self.barnes_hut_data[target_index] = Some(node);

        node
    }
}
