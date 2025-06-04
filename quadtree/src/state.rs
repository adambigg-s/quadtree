use glam::Vec2;

use sokol::app as sapp;

use crate::barnes_hut::BarnesHutWrapper;
use crate::quadtree::PositionPlanar;
use crate::quadtree::QuadTree;
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

impl PositionPlanar for Particle {
    fn position(&self) -> Vec2 {
        self.position
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct State {
    pub dimensions: BoundingBox,
    pub particles: Vec<Particle>,
    pub config: SimulationConfig,
    pub quadtree: QuadTree,
}

impl State {
    pub fn build(width: i32, height: i32) -> Self {
        State {
            dimensions: BoundingBox::build(Vec2::ZERO, Vec2::new(width as f32, height as f32)),
            particles: Vec::new(),
            config: SimulationConfig {
                starting_spawn: 50,
                gravity: 1e3,
                epsilon_squared: 50.,
                velocity_rand_max: 50.,
                mass_rand_max: 100.,
                frame_time_dt_mod: 0.1,
                neighbor_distance: 300.,
            },
            quadtree: QuadTree::build(
                1,
                BoundingBox::build(Vec2::ZERO, Vec2::new(width as f32, height as f32)),
            ),
        }
    }

    pub fn init(&mut self) {
        (0..self.config.starting_spawn).for_each(|_| {
            self.add_random_particle();
        });
        // spawns one big particle in the middle
        self.particles.push(Particle::new(self.dimensions.max / 2., Vec2::ZERO, 10000.));
    }

    pub fn update(&mut self, mut dt: f32) {
        dt *= self.config.frame_time_dt_mod;

        self.init_tree();
        let mut bh = BarnesHutWrapper::new();
        bh.build_hierarchy(&self.quadtree, &self.particles);
        for target_index in 0..self.particles.len() {
            let neighbors = Self::query_tree(
                &self.quadtree,
                self.particles[target_index].position,
                self.config.neighbor_distance,
            );

            for other_index in neighbors {
                if target_index == other_index {
                    continue;
                }

                let other = self.particles[other_index];
                let target = &mut self.particles[target_index];

                let pointing = other.position - target.position;

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

    pub fn handle_event(&mut self, event: sapp::Event) {
        if event.mouse_button == sapp::Mousebutton::Left && event._type == sapp::EventType::MouseDown {
            self.particles.push(Particle::new(
                mouse_to_screen(event.mouse_x, event.mouse_y, &self.dimensions),
                Vec2::ZERO,
                self.config.mass_rand_max,
            ));
            wait(3);
        }
        if event.key_code == sapp::Keycode::R {
            self.particles.clear();
        }
    }

    pub fn update_dimensions(&mut self, width: f32, height: f32) {
        self.dimensions.max = Vec2::new(width, height);
        self.quadtree.root().boundary = self.dimensions;
    }

    fn add_random_particle(&mut self) {
        self.particles.push(Particle::new(
            positive_rand_range_vec2(self.dimensions.max),
            zero_centered_range_vec2(self.config.velocity_rand_max),
            fastrand::f32() * self.config.mass_rand_max,
        ));
    }

    fn init_tree(&mut self) {
        self.quadtree.construct_tree(&self.particles);
    }

    fn query_tree(tree: &QuadTree, pos: Vec2, radius: f32) -> Vec<usize> {
        tree.query_range(&BoundingBox::build(pos - Vec2::splat(radius), pos + Vec2::splat(radius)))
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct SimulationConfig {
    pub starting_spawn: usize,
    pub gravity: f32,
    pub epsilon_squared: f32,
    pub velocity_rand_max: f32,
    pub mass_rand_max: f32,
    pub frame_time_dt_mod: f32,
    pub neighbor_distance: f32,
}
