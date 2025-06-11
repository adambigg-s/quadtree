use glam::Vec2;

use sokol::time;

pub const EPSILON: f32 = 1e-9;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    pub min: Vec2,
    pub max: Vec2,
}

impl BoundingBox {
    pub fn build(min: Vec2, max: Vec2) -> Self {
        BoundingBox { min, max }
    }

    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }

    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    pub fn max_dimension(&self) -> f32 {
        self.height().max(self.width())
    }

    pub fn contains(&self, point: Vec2) -> bool {
        self.min.x <= point.x && self.min.y <= point.y && self.max.x > point.x && self.max.y > point.y
    }

    pub fn overlaps(&self, other: &BoundingBox) -> bool {
        self.max.x > other.min.x
            && self.min.x <= other.max.x
            && self.max.y > other.min.y
            && self.min.y <= other.max.y
    }

    pub fn center(&self) -> Vec2 {
        (self.min + self.max) / 2.
    }

    pub fn split_quadrants(&self) -> [Self; 4] {
        let center = self.center();
        /* follows the unit circle quadrant conventions, but the origin is in
        the top left so it is a little confusing */
        [
            BoundingBox::build(Vec2::new(center.x, self.min.y), Vec2::new(self.max.x, center.y)), // i
            BoundingBox::build(self.min, center),                                                 // ii
            BoundingBox::build(Vec2::new(self.min.x, center.y), Vec2::new(center.x, self.max.y)), // ii
            BoundingBox::build(center, self.max),                                                 // iv
        ]
    }
}

#[derive(Debug)]
pub struct Clock {
    pub curr_time: u64,
    pub last_time: u64,
    pub frame_time: f32,
}

impl Clock {
    pub fn update(&mut self, now: u64) {
        self.last_time = self.curr_time;
        self.curr_time = now;
        self.frame_time = time::sec(self.curr_time - self.last_time) as f32;
    }
}

pub fn random_vec2() -> Vec2 {
    Vec2::new(fastrand::f32(), fastrand::f32())
}

pub fn positive_rand_range_vec2(scale: Vec2) -> Vec2 {
    let mut vec = random_vec2();
    vec *= scale;
    vec
}

pub fn zero_centered_range_vec2(scale: f32) -> Vec2 {
    let mut vec = random_vec2();
    vec = vec * 2. - Vec2::splat(1.);
    vec *= scale;
    vec
}

pub fn mouse_to_screen(mousex: f32, mousey: f32, dimensions: &BoundingBox) -> Vec2 {
    Vec2::new(mousex, dimensions.height() - mousey)
}

pub fn wait(time_ms: u64) {
    std::thread::sleep(std::time::Duration::from_millis(time_ms));
}
