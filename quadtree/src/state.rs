use glam::Vec2;

use crate::{
    quadtree::{QuadTreeWrap, Rectangle, TreeNode},
    utils::random_vec2,
};

#[derive(Debug, Clone, Copy)]
pub struct Particle {
    pub p: Vec2,
    pub v: Vec2,
    pub mass: f32,
}

impl Particle {
    pub fn new(dims: Vec2, mass: f32) -> Self {
        Particle {
            p: {
                let mut vec = random_vec2();
                vec.x *= dims.x;
                vec.y *= dims.y;
                vec
            },
            v: {
                let mut vec = random_vec2() * 2.;
                vec -= Vec2::ONE;
                vec
            },
            mass,
        }
    }

    pub fn update(&mut self) {
        self.p += self.v;
    }
}

#[derive(Debug)]
pub struct State<'d> {
    pub dimensions: Vec2,
    pub particles: Vec<Particle>,
    pub tree: QuadTreeWrap<'d, Particle>,
}

impl<'d> State<'d> {
    pub fn build(width: i32, height: i32) -> Self {
        State {
            dimensions: Vec2::new(width as f32, height as f32),
            particles: Vec::new(),
            tree: QuadTreeWrap::build(
                2,
                Rectangle::build(Vec2::ZERO, Vec2::new(width as f32, height as f32)),
            ),
        }
    }

    pub fn update(&mut self, width: f32, height: f32) {
        self.dimensions = Vec2::new(width, height);

        self.tree.init_tree(&self.particles);

        for particle in &mut self.particles {
            particle.update();
        }

        const G: f32 = 5.;

        for i in 0..self.particles.len() {
            for j in 0..self.particles.len() {
                if i == j {
                    continue;
                }

                let other = self.particles[j];
                let target = &mut self.particles[i];

                let pointing = other.p - target.p;
                let r2 = pointing.length_squared().max(30.);

                let force = G * target.mass * other.mass / r2;

                target.v += pointing.normalize() * force;
            }
        }

        for particle in &mut self.particles {
            if particle.p.x > self.dimensions.x {
                particle.v.x *= -1.;
            }
            if particle.p.x < 0. {
                particle.v.x *= -1.;
            }
            if particle.p.y > self.dimensions.y {
                particle.v.y *= -1.;
            }
            if particle.p.y < 0. {
                particle.v.y *= -1.;
            }
        }
    }

    pub fn add_particle(&mut self, mass: f32) {
        self.particles.push(Particle::new(self.dimensions, mass));
    }
}
