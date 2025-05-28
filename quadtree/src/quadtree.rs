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
            self.insert(particle);
        }
    }

    pub fn insert(&mut self, particle: &Particle) {
        if !self.bounds.contains(particle.position) {
            return;
        }

        if self.points.len() < self.capacity && self.children.is_none() {
            self.points.push(*particle);
            return;
        }

        self.subdivide();

        if let Some(children) = &mut self.children {
            children.iter_mut().for_each(|child| {
                child.insert(particle);
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
                    child.insert(particle);
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

pub struct QuadTreeIndex {
    pub nodes: Vec<QuadTreeNode>,
    pub capacity: usize,
}

impl QuadTreeIndex {
    pub fn build(capacity: usize, bounds: BoundingBox) -> Self {
        let head = QuadTreeNode::build(bounds);
        QuadTreeIndex { nodes: vec![head], capacity }
    }

    pub fn init_tree(&mut self, particles: &[Particle]) {
        self.clear_tree();
        for (idx, particle) in particles.iter().enumerate() {
            self.insert_node(idx, particle);
        }
    }

    fn insert_node(&mut self, idx: usize, particle: &Particle) {
        let mut curr_node = &mut self.nodes[0];
        if !curr_node.bounds.contains(particle.position) {
            return;
        }
        if curr_node.data.len() < self.capacity {
            curr_node.data.push(idx);
        }
    }

    fn subdivide(&mut self) {}

    fn clear_tree(&mut self) {
        self.nodes.truncate(1);
    }
}
