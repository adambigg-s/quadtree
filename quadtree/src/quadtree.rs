use crate::state::Particle;
use crate::utils::BoundingBox;

#[repr(C)]
#[derive(Debug)]
pub struct QuadTree {
    pub points: Vec<Particle>,
    pub children: Option<[Box<QuadTree>; 4]>,
    pub capacity: usize,
    pub bounds: BoundingBox,
}

impl QuadTree {
    pub fn build(capacity: usize, bounds: BoundingBox) -> Self {
        QuadTree { points: Vec::new(), children: None, capacity, bounds }
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

        if self.points.len() < self.capacity {
            self.points.push(*particle);
            return;
        }

        if self.children.is_none() {
            self.subdivide();
        }

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
            Box::new(QuadTree::build(self.capacity, quads[0])),
            Box::new(QuadTree::build(self.capacity, quads[1])),
            Box::new(QuadTree::build(self.capacity, quads[2])),
            Box::new(QuadTree::build(self.capacity, quads[3])),
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
