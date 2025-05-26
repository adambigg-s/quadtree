use glam::Vec2;

pub fn random_vec2(constraits: Vec2) -> Vec2 {
    Vec2::new(fastrand::f32() * constraits.x, fastrand::f32() * constraits.y)
}
