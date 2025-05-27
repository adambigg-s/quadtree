use glam::Vec2;

pub fn random_vec2() -> Vec2 {
    Vec2::new(fastrand::f32(), fastrand::f32())
}
