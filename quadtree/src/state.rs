use glam::Vec2;

use crate::quadtree::QuadTree;
use crate::utils::random_vec2;
use crate::utils::BoundingBox;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub mass: f32,
}

impl Particle {
    pub fn update(&mut self, dt: f32) {
        self.position += self.velocity * dt;
    }

    pub fn constrain(&mut self, bounds: &BoundingBox) {
        if self.position.x > bounds.max.x {
            self.position.x = bounds.min.x;
        }
        if self.position.x < bounds.min.x {
            self.position.x = bounds.max.x;
        }
        if self.position.y > bounds.max.y {
            self.position.y = bounds.min.y;
        }
        if self.position.y < bounds.min.y {
            self.position.y = bounds.max.y;
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct State {
    pub dimensions: BoundingBox,
    pub particles: Vec<Particle>,

    pub tree: QuadTree,
}

impl State {
    pub fn build(width: i32, height: i32) -> Self {
        State {
            dimensions: BoundingBox::build(Vec2::ZERO, Vec2::new(width as f32, height as f32)),
            particles: Vec::new(),
            tree: QuadTree::build(1, BoundingBox::build(Vec2::ZERO, Vec2::new(width as f32, height as f32))),
        }
    }

    pub fn init(&mut self) {
        (0..1).for_each(|_| {
            self.add_random_particle();
        });
    }

    pub fn update_dimensions(&mut self, width: f32, height: f32) {
        self.dimensions.max = Vec2::new(width, height);
        self.tree.bounds = self.dimensions;
    }

    pub fn add_random_particle(&mut self) {
        self.particles.push(Particle {
            position: {
                let mut vec = random_vec2();
                vec *= self.dimensions.max;
                vec
            },
            velocity: {
                let mut vec = random_vec2();
                vec = vec * 2. - Vec2::splat(1.);
                vec *= 300.;
                vec
            },
            mass: fastrand::f32() * 10.,
        });
    }

    pub fn init_tree(&mut self) {
        self.tree.init_tree(&self.particles);
    }

    pub fn query_tree(tree: &QuadTree, pos: Vec2, radius: f32) -> Vec<Particle> {
        tree.query_range(&BoundingBox::build(pos - Vec2::splat(radius), pos + Vec2::splat(radius)))
    }

    pub fn update(&mut self, dt: f32) {
        /* terrible n-body algorithm - this will be changed to barnes-hut soon
        this is just to test and make sure the quadtree is working */
        let grav: f32 = 30000.;
        self.init_tree();
        for target in &mut self.particles {
            let neighbors = Self::query_tree(&self.tree, target.position, 150.);
            for other in neighbors {
                let pointing = other.position - target.position;

                if pointing.length() < 1e-4 {
                    continue;
                }

                let sq_radius = pointing.length_squared().max(50.);
                let force = grav * other.mass / sq_radius;
                target.velocity += pointing.normalize() * force * dt;
            }
        }

        for particle in &mut self.particles {
            particle.update(dt);
            particle.constrain(&self.dimensions);
        }
    }
}
