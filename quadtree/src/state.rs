use glam::Vec2;

use sokol::app as sapp;

use crate::quadtree::QuadTreeOwner;
use crate::utils::mouse_to_screen;
use crate::utils::positive_rand_range_vec2;
use crate::utils::wait;
use crate::utils::zero_centered_range_vec2;
use crate::utils::BoundingBox;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub mass: f32,
    pub radius: f32,
}

impl Particle {
    pub fn new(position: Vec2, velocity: Vec2, mass: f32) -> Self {
        Particle { position, velocity, acceleration: Vec2::ZERO, mass, radius: mass.powf(0.333) }
    }

    pub fn update(&mut self, dt: f32) {
        self.position += self.velocity * dt + 0.5 * self.acceleration * dt * dt;
        self.velocity += self.acceleration * dt;
        self.acceleration *= 0.;
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
    pub config: SimulationConfig,
    pub tree: QuadTreeOwner,
}

impl State {
    pub fn build(width: i32, height: i32) -> Self {
        State {
            dimensions: BoundingBox::build(Vec2::ZERO, Vec2::new(width as f32, height as f32)),
            particles: Vec::new(),
            config: SimulationConfig {
                gravity: 1e3,
                epsilon_squared: 50.,
                velocity_rand_max: 50.,
                mass_rand_max: 100.,
                starting_spawn: 30,
                frame_time_dt_mod: 0.1,
                neighbor_distance: 300.,
            },
            tree: QuadTreeOwner::build(
                3,
                BoundingBox::build(Vec2::ZERO, Vec2::new(width as f32, height as f32)),
            ),
        }
    }

    pub fn init(&mut self) {
        (0..self.config.starting_spawn).for_each(|_| {
            self.add_random_particle();
        });
        self.particles.push(Particle::new(self.dimensions.max / 2., Vec2::ZERO, 10000.));
    }

    pub fn handle_event(&mut self, event: sapp::Event) {
        if event.mouse_button == sapp::Mousebutton::Left {
            self.particles.push(Particle::new(
                mouse_to_screen(event.mouse_x, event.mouse_y, &self.dimensions),
                Vec2::ZERO,
                self.config.mass_rand_max,
            ));
            wait(30);
        }
        if event.key_code == sapp::Keycode::R {
            self.particles.clear();
        }
    }

    pub fn update_dimensions(&mut self, width: f32, height: f32) {
        self.dimensions.max = Vec2::new(width, height);
        self.tree.bounds = self.dimensions;
    }

    pub fn add_random_particle(&mut self) {
        self.particles.push(Particle::new(
            positive_rand_range_vec2(self.dimensions.max),
            zero_centered_range_vec2(self.config.velocity_rand_max),
            fastrand::f32() * self.config.mass_rand_max,
        ));
    }

    pub fn init_tree(&mut self) {
        self.tree.init_tree(&self.particles);
    }

    pub fn query_tree(tree: &QuadTreeOwner, pos: Vec2, radius: f32) -> Vec<Particle> {
        tree.query_range(&BoundingBox::build(pos - Vec2::splat(radius), pos + Vec2::splat(radius)))
    }

    pub fn update(&mut self, mut dt: f32) {
        dt *= self.config.frame_time_dt_mod;
        self.init_tree();
        for target in &mut self.particles {
            let neighbors = Self::query_tree(&self.tree, target.position, self.config.neighbor_distance);
            for other in neighbors {
                let pointing = other.position - target.position;

                if pointing.length() < 1e-4 {
                    continue;
                }

                let sq_radius = pointing.length_squared() + self.config.epsilon_squared;
                let force = self.config.gravity * target.mass * other.mass / sq_radius;
                target.acceleration += pointing.normalize() * force / target.mass;
            }
        }

        for particle in &mut self.particles {
            particle.update(dt);
            particle.constrain(&self.dimensions);
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct SimulationConfig {
    pub gravity: f32,
    pub epsilon_squared: f32,
    pub velocity_rand_max: f32,
    pub mass_rand_max: f32,
    pub starting_spawn: usize,
    pub frame_time_dt_mod: f32,
    pub neighbor_distance: f32,
}
