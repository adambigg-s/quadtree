use crate::state::Particle;
use crate::utils::BoundingBox;

#[repr(C)]
#[derive(Debug)]
pub struct QuadTreeOwner {
    pub points: Vec<Particle>,
    pub children: Option<[Box<QuadTreeOwner>; 4]>,
    pub capacity: usize,
    pub bounds: BoundingBox,
}

impl QuadTreeOwner {
    pub fn build(capacity: usize, bounds: BoundingBox) -> Self {
        QuadTreeOwner { points: Vec::new(), children: None, capacity, bounds }
    }

    pub fn init_tree(&mut self, particles: &[Particle]) {
        self.clear_tree();
        for particle in particles {
            self.insert_recursive(particle);
        }
    }

    pub fn insert_recursive(&mut self, particle: &Particle) {
        if !self.bounds.contains(particle.position) {
            return;
        }

        if self.points.len() < self.capacity && self.children.is_none() {
            self.points.push(*particle);
            return;
        }

        if self.children.is_none() {
            self.subdivide();
        }

        if let Some(children) = &mut self.children {
            children.iter_mut().for_each(|child| {
                child.insert_recursive(particle);
            });
        }
    }

    pub fn query_range(&self, range: &BoundingBox) -> Vec<Particle> {
        let mut output = Vec::new();
        self.recursive_search(range, &mut output);
        output
    }

    fn recursive_search(&self, range: &BoundingBox, outputs: &mut Vec<Particle>) {
        if !self.bounds.overlaps(range) {
            return;
        }

        for particle in &self.points {
            if range.contains(particle.position) {
                outputs.push(*particle);
            }
        }

        if let Some(children) = &self.children {
            for child in children.iter() {
                child.recursive_search(range, outputs);
            }
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
        for particle in &self.points {
            if let Some(children) = &mut self.children {
                for child in children.iter_mut() {
                    child.insert_recursive(particle);
                }
            }
        }

        self.points.clear();
    }

    fn clear_tree(&mut self) {
        self.points.clear();
        self.children = None;
    }
}

#[derive(Debug)]
pub struct QuadTreeNode {
    pub bounds: BoundingBox,
    pub children: Option<[usize; 4]>,
    pub data: Vec<usize>,
}

impl QuadTreeNode {
    pub fn build(bounds: BoundingBox) -> Self {
        QuadTreeNode { bounds, children: None, data: Vec::with_capacity(5) }
    }
}

#[derive(Debug)]
pub struct QuadTreeIndex {
    pub nodes: Vec<QuadTreeNode>,
    pub capacity: usize,
}

impl QuadTreeIndex {
    const HEAD_INDEX: usize = 0;

    pub fn build(capacity: usize, bounds: BoundingBox) -> Self {
        let head = QuadTreeNode::build(bounds);
        QuadTreeIndex { nodes: vec![head], capacity }
    }

    pub fn init_tree(&mut self, particles: &[Particle]) {
        self.clear_tree();
        for (idx, particle) in particles.iter().enumerate() {
            self.insert_node(Self::HEAD_INDEX, idx, particle);
        }
    }

    fn insert_node(&mut self, node_index: usize, idx: usize, particle: &Particle) {
        let curr_node = &mut self.nodes[node_index];
        if !curr_node.bounds.contains(particle.position) {
            return;
        }
        if curr_node.data.len() < self.capacity && curr_node.children.is_none() {
            curr_node.data.push(idx);
            return;
        }

        self.subdivide(node_index);

        if let Some(children) = self.nodes[node_index].children {
            for child in children {
                self.insert_node(child, idx, particle);
            }
        }
    }

    fn subdivide(&mut self, node_index: usize) {
        let quads = self.nodes[node_index].bounds.split_quadrants();
        self.nodes[node_index].children =
            Some([node_index + 1, node_index + 2, node_index + 3, node_index + 4]);
        self.nodes.push(QuadTreeNode::build(quads[0]));
        self.nodes.push(QuadTreeNode::build(quads[1]));
        self.nodes.push(QuadTreeNode::build(quads[2]));
        self.nodes.push(QuadTreeNode::build(quads[3]));
    }

    fn clear_tree(&mut self) {
        self.nodes.truncate(1);
    }
}
