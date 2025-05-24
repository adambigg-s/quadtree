use glam::Vec2;

struct Particle {
    pos: Vec2,
    vel: Vec2,
}

impl Particle {
    pub fn new() -> Self {
        Particle { pos: random_vec2(), vel: random_vec2() }
    }
}

pub struct State {
    world_dims: Vec2,
    particles: Vec<Particle>,
}

impl State {
    pub fn build(width: i32, height: i32) -> Self {
        State { world_dims: Vec2::new(width as f32, height as f32), particles: Vec::new() }
    }

    pub fn update(&mut self, width: f32, height: f32) {
        self.world_dims.x = width;
        self.world_dims.y = height;
    }

    pub fn add_particle(&mut self) {
        self.particles.push(Particle::new());
    }
}

pub fn random_vec2() -> Vec2 {
    Vec2::new(fastrand::f32(), fastrand::f32())
}
