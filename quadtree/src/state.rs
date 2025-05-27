use glam::Vec2;

use crate::quadtree::QuadTree;
use crate::utils::random_vec2;
use crate::utils::Rectangle;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub mass: f32,
}

impl Particle {
    pub fn update(&mut self) {
        self.position += self.velocity;
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct State {
    pub dimensions: Vec2,
    pub particles: Vec<Particle>,
    pub tree: QuadTree,
}

impl State {
    pub fn build(width: i32, height: i32) -> Self {
        State {
            dimensions: Vec2::new(width as f32, height as f32),
            particles: Vec::new(),
            tree: QuadTree {
                capacity: 2,
                bounds: Rectangle::build(Vec2::ZERO, Vec2::new(width as f32, height as f32)),
                points: Vec::new(),
                children: None,
            },
        }
    }

    pub fn init(&mut self) {
        (0..500).for_each(|_| {
            self.add_random_particle();
        });
    }

    pub fn update_dimensions(&mut self, width: f32, height: f32) {
        self.dimensions = Vec2::new(width, height);
    }

    pub fn add_random_particle(&mut self) {
        self.particles.push(Particle {
            position: {
                let mut vec = random_vec2();
                vec *= self.dimensions;
                vec
            },
            velocity: {
                let mut vec = random_vec2();
                vec = vec * 2. - Vec2::splat(1.);
                vec * 5.
            },
            mass: fastrand::f32() * 3.,
        });
    }

    pub fn rebuild_tree(&mut self) {
        self.tree.init_tree(&self.particles);
    }

    pub fn update(&mut self) {
        self.rebuild_tree();

        for particle in &mut self.particles {
            particle.update();
        }

        const G: f32 = 30.;

        for i in 0..self.particles.len() {
            for j in 0..self.particles.len() {
                if i == j {
                    continue;
                }

                let other = self.particles[j];
                let target = &mut self.particles[i];

                let pointing = other.position - target.position;
                let r2 = pointing.length_squared().max(30.);

                let force = G * target.mass * other.mass / r2;

                target.velocity += pointing.normalize() * force;
            }

            // let neighbors = self.query_nearby(self.particles[i].position, 300.);
            // for &j in neighbors.iter() {
            //     if i == j {
            //         continue;
            //     }
            // }
        }

        for particle in &mut self.particles {
            if particle.position.x > self.dimensions.x {
                particle.velocity.x *= -1.;
            }
            if particle.position.x < 0. {
                particle.velocity.x *= -1.;
            }
            if particle.position.y > self.dimensions.y {
                particle.velocity.y *= -1.;
            }
            if particle.position.y < 0. {
                particle.velocity.y *= -1.;
            }
        }
    }
}
