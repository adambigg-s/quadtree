use glam::Vec2;

use crate::state::Particle;
use crate::utils::BoundingBox;

pub trait PositionPlanar {
    fn position(&self) -> Vec2;
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct QuadTreeNode {
    // the boundary for the specific node
    pub boundary: BoundingBox,
    // the *first* index for the leaves (there are always 4)
    pub leaves: Option<usize>,
    // index in QuadTree->node_pointers where actual node data lives
    pub data_head: Option<usize>,
}

impl QuadTreeNode {
    pub fn build(boundary: BoundingBox) -> Self {
        QuadTreeNode { boundary, leaves: None, data_head: None }
    }
}

/// non-tree style quadtree which only uses indices - should be way faster but
/// is really confusing
#[repr(C)]
#[derive(Debug)]
pub struct QuadTree {
    // all nodes for the tree
    pub nodes: Vec<QuadTreeNode>,
    // each vec holds indices in main for its data, one vec for each node
    pub node_pointers: Vec<Vec<usize>>,
    // max number of items in each leaf
    pub leaf_capacity: usize,
}

impl QuadTree {
    pub const ROOT_INDEX: usize = 0;
    pub const STEM_LEAF_COUNT: usize = 4; // because it is a 'quad'-tree

    pub fn build(leaf_capacity: usize, boundary: BoundingBox) -> Self {
        QuadTree { nodes: vec![QuadTreeNode::build(boundary)], node_pointers: Vec::new(), leaf_capacity }
    }

    pub fn construct_tree<T>(&mut self, items: &[T])
    where
        T: PositionPlanar,
    {
        self.clear_tree();
        (0..items.len()).for_each(|index| {
            self.insert_recursive(Self::ROOT_INDEX, index, items);
        });
    }

    pub fn query_range(&self, boundary: &BoundingBox) -> Vec<usize> {
        let mut output = Vec::new();
        self.search_recursive(Self::ROOT_INDEX, boundary, &mut output);

        output
    }

    pub fn clear_tree(&mut self) {
        self.nodes[Self::ROOT_INDEX].leaves = None;
        self.nodes[Self::ROOT_INDEX].data_head = None;
        self.nodes.truncate(1);
        self.node_pointers.clear();
    }

    pub fn root(&mut self) -> &mut QuadTreeNode {
        &mut self.nodes[Self::ROOT_INDEX]
    }

    fn insert_recursive<T>(&mut self, target_node_index: usize, item_index: usize, items: &[T])
    where
        T: PositionPlanar,
    {
        if !self.nodes[target_node_index].boundary.contains(items[item_index].position()) {
            return;
        }
        // if we don't contain the item, EXIT
        {
            debug_assert!(self.nodes[target_node_index].boundary.contains(items[item_index].position()));
        }

        if let Some(leaf_start) = self.nodes[target_node_index].leaves {
            (leaf_start..(leaf_start + Self::STEM_LEAF_COUNT)).for_each(|leaf| {
                self.insert_recursive(leaf, item_index, items);
            });
            return;
        }
        // if the node is a stem (has leaves to insert), EXIT
        {
            debug_assert!(self.nodes[target_node_index].leaves.is_none());
        }

        if let Some(node_list_index) = self.nodes[target_node_index].data_head {
            self.node_pointers[node_list_index].push(item_index);

            if self.node_pointers[node_list_index].len() > self.leaf_capacity {
                self.subdivide_stem_to_leaf(target_node_index, items);
            }

            return;
        }
        // if the node is a leaf and has existing data, push new data and EXIT
        {
            debug_assert!(self.nodes[target_node_index].data_head.is_none());
        }

        // yikes this is really confusing
        let node_index = self.node_pointers.len();
        self.node_pointers.push(Vec::with_capacity(self.leaf_capacity));
        self.node_pointers[node_index].push(item_index);
        self.nodes[target_node_index].data_head = Some(node_index);
    }

    fn search_recursive(&self, target_node_index: usize, boundary: &BoundingBox, outputs: &mut Vec<usize>) {
        if !self.nodes[target_node_index].boundary.overlaps(boundary) {
            return;
        }

        if let Some(leaf_start) = self.nodes[target_node_index].leaves {
            (leaf_start..(leaf_start + Self::STEM_LEAF_COUNT)).for_each(|leaf| {
                self.search_recursive(leaf, boundary, outputs);
            });
            return;
        }

        if let Some(node_list_index) = self.nodes[target_node_index].data_head {
            // iteratively clone data from leaf into output vec
            outputs.extend_from_slice(&self.node_pointers[node_list_index]);
        }
    }

    fn subdivide_stem_to_leaf<T>(&mut self, target_node_index: usize, items: &[T])
    where
        T: PositionPlanar,
    {
        self.nodes[target_node_index].leaves = Some(self.nodes.len());

        let [i, ii, iii, iv] = self.nodes[target_node_index].boundary.split_quadrants();
        self.nodes.extend([
            QuadTreeNode::build(i),
            QuadTreeNode::build(ii),
            QuadTreeNode::build(iii),
            QuadTreeNode::build(iv),
        ]);

        if let Some(node_list_index) = self.nodes[target_node_index].data_head {
            // drains the (now stem) node's data to pour into new leaves
            let stored_item_indices: Vec<usize> = self.node_pointers[node_list_index].drain(..).collect();
            {
                debug_assert!(stored_item_indices.len() == self.leaf_capacity + 1);
            }

            ((self.nodes.len() - Self::STEM_LEAF_COUNT)..self.nodes.len()).for_each(|leaf| {
                stored_item_indices.iter().for_each(|&item_index| {
                    self.insert_recursive(leaf, item_index, items);
                });
            });
        }
    }
}

/// typical tree structure, actually takes ownership of the data. pretty
/// readable and safe but slow
#[allow(dead_code)]
#[repr(C)]
#[derive(Debug)]
pub struct QuadTreeOwner {
    pub points: Vec<Particle>,
    pub children: Option<[Box<QuadTreeOwner>; 4]>,
    pub capacity: usize,
    pub bounds: BoundingBox,
}

#[allow(dead_code)]
impl QuadTreeOwner {
    pub fn build(capacity: usize, bounds: BoundingBox) -> Self {
        QuadTreeOwner { points: Vec::new(), children: None, capacity, bounds }
    }

    pub fn init_tree(&mut self, particles: &[Particle]) {
        self.clear_tree();
        particles.iter().for_each(|particle| {
            self.insert_recursive(particle);
        });
    }

    pub fn query_range(&self, range: &BoundingBox) -> Vec<Particle> {
        let mut output = Vec::new();
        self.recursive_search(range, &mut output);
        output
    }

    fn insert_recursive(&mut self, particle: &Particle) {
        if !self.bounds.contains(particle.position) {
            return;
        }

        if let Some(children) = &mut self.children {
            children.iter_mut().for_each(|child| {
                child.insert_recursive(particle);
            });
            return;
        }

        self.points.push(*particle);
        if self.points.len() > self.capacity {
            self.subdivide();
        }
    }

    fn recursive_search(&self, range: &BoundingBox, outputs: &mut Vec<Particle>) {
        if !self.bounds.overlaps(range) {
            return;
        }

        self.points.iter().for_each(|particle| {
            if range.contains(particle.position) {
                outputs.push(*particle);
            }
        });

        if let Some(children) = &self.children {
            children.iter().for_each(|child| {
                child.recursive_search(range, outputs);
            });
        }
    }

    fn subdivide(&mut self) {
        let quads = self.bounds.split_quadrants();
        self.children = Some([
            Box::new(QuadTreeOwner::build(self.capacity, quads[0])),
            Box::new(QuadTreeOwner::build(self.capacity, quads[1])),
            Box::new(QuadTreeOwner::build(self.capacity, quads[2])),
            Box::new(QuadTreeOwner::build(self.capacity, quads[3])),
        ]);

        self.points.iter().for_each(|particle| {
            if let Some(children) = &mut self.children {
                children.iter_mut().for_each(|child| {
                    child.insert_recursive(particle);
                });
            }
        });

        self.points.clear();
    }

    fn clear_tree(&mut self) {
        self.points.clear();
        self.children = None;
    }
}
