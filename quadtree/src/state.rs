use glam::Vec2;

use crate::utils::random_vec2;

#[derive(Debug, Clone, Copy)]
pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub mass: f32,
}

impl Particle {
    pub fn new(dims: Vec2, mass: f32) -> Self {
        Particle { pos: random_vec2(dims), vel: random_vec2(Vec2::new(2., 2.)) - Vec2::new(1., 1.), mass }
    }

    pub fn update(&mut self) {
        self.pos += self.vel;
    }
}

#[derive(Debug)]
pub struct State {
    pub world_dims: Vec2,
    pub particles: Vec<Particle>,
}

impl State {
    pub fn build(width: i32, height: i32) -> Self {
        State { world_dims: Vec2::new(width as f32, height as f32), particles: Vec::new() }
    }

    pub fn update(&mut self, width: f32, height: f32) {
        self.world_dims = Vec2::new(width, height);

        for particle in &mut self.particles {
            particle.update();
        }

        const G: f32 = 1.;

        for i in 0..self.particles.len() {
            for j in 0..self.particles.len() {
                if i == j {
                    continue;
                }

                let other = self.particles[j];
                let target = &mut self.particles[i];

                let pointing = other.pos - target.pos;
                let r2 = pointing.length_squared().max(5.);

                let force = G * target.mass * other.mass / r2;

                target.vel += pointing.normalize() * force;
            }
        }

        for particle in &mut self.particles {
            if particle.pos.x > self.world_dims.x {
                particle.vel.x *= -1.;
            }
            if particle.pos.x < 0. {
                particle.vel.x *= -1.;
            }
            if particle.pos.y > self.world_dims.y {
                particle.vel.y *= -1.;
            }
            if particle.pos.y < 0. {
                particle.vel.y *= -1.;
            }
        }
    }

    pub fn add_particle(&mut self, mass: f32) {
        self.particles.push(Particle::new(self.world_dims, mass));
    }
}
