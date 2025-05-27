use crate::state::Particle;
use crate::utils::Rectangle;

#[repr(C)]
#[derive(Debug)]
pub struct QuadTree {
    pub capacity: usize,
    pub bounds: Rectangle,
    pub points: Vec<Particle>,
    pub children: Option<[Box<QuadTree>; 4]>,
}

impl QuadTree {
    pub fn init_tree(&mut self, particles: &[Particle]) {
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
            let quadrants = self.bounds.split_quadrants();
        }
    }
}
